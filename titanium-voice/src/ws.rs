//! Voice WebSocket connection handler.
//!
//! Manages the WebSocket connection to Discord's voice servers.

use crate::error::VoiceError;
use crate::opcode::VoiceOpCode;
use crate::payload::*;
use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{interval, sleep};
use tokio_tungstenite::{
    connect_async, tungstenite::Message as WsMessage, MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, trace, warn};

/// Voice WebSocket connection.
pub struct VoiceWebSocket {
    /// WebSocket URL.
    url: String,
    /// Whether we're connected.
    connected: AtomicBool,
    /// Last heartbeat nonce.
    last_heartbeat_nonce: AtomicU64,
    /// Heartbeat acknowledged.
    heartbeat_acked: AtomicBool,
    /// Heartbeat latency in milliseconds.
    heartbeat_latency_ms: RwLock<Option<u64>>,
}

/// Voice connection info received from Ready.
#[derive(Debug, Clone)]
pub struct VoiceReadyInfo {
    /// SSRC for this connection.
    pub ssrc: u32,
    /// Voice server IP.
    pub ip: String,
    /// Voice server UDP port.
    pub port: u16,
    /// Available encryption modes.
    pub modes: Vec<String>,
}

/// Session description received after protocol selection.
#[derive(Debug, Clone)]
pub struct VoiceSessionDescription {
    /// Selected encryption mode.
    pub mode: String,
    /// Secret key for encryption.
    pub secret_key: Vec<u8>,
}

/// Events from the voice WebSocket.
#[derive(Debug, Clone)]
pub enum VoiceEvent {
    /// Ready event with connection info.
    Ready(VoiceReadyInfo),
    /// Session description with encryption key.
    SessionDescription(VoiceSessionDescription),
    /// Client connected to voice.
    ClientConnect { user_id: u64, ssrc: Option<u32> },
    /// Client disconnected from voice.
    ClientDisconnect { user_id: u64 },
    /// Resumed successfully.
    Resumed,
    /// Connection closed.
    Closed { code: u16, reason: String },
}

/// Commands to control the Voice WebSocket.
#[derive(Debug)]
pub enum VoiceCommand {
    /// Send Select Protocol payload.
    SelectProtocol {
        protocol: String,
        ip: String,
        port: u16,
        mode: String,
    },
    /// Send Speaking payload.
    Speaking {
        speaking: SpeakingFlags,
        delay: u32,
        ssrc: u32,
    },
}

impl VoiceWebSocket {
    /// Create a new voice WebSocket handler.
    ///
    /// # Arguments
    /// * `endpoint` - Voice server endpoint from VOICE_SERVER_UPDATE.
    pub fn new(endpoint: &str) -> Self {
        // Build WebSocket URL
        let url = if endpoint.starts_with("wss://") {
            format!("{}/?v=4", endpoint)
        } else {
            format!("wss://{}/?v=4", endpoint)
        };

        Self {
            url,
            connected: AtomicBool::new(false),
            last_heartbeat_nonce: AtomicU64::new(0),
            heartbeat_acked: AtomicBool::new(true),
            heartbeat_latency_ms: RwLock::new(None),
        }
    }

    /// Connect and run the voice WebSocket.
    ///
    /// # Arguments
    /// * `session_id` - Session ID from VOICE_STATE_UPDATE.
    /// * `guild_id` - Guild ID.
    /// * `user_id` - User ID.
    /// * `token` - Voice token from VOICE_SERVER_UPDATE.
    /// * `event_tx` - Channel to send voice events.
    /// * `command_rx` - Channel to receive voice commands.
    pub async fn connect_and_run(
        &self,
        session_id: String,
        guild_id: u64,
        user_id: u64,
        token: String,
        event_tx: mpsc::Sender<VoiceEvent>,
        mut command_rx: mpsc::Receiver<VoiceCommand>,
    ) -> Result<(), VoiceError> {
        info!(url = %self.url, "Connecting to Voice Gateway");

        // Connect to WebSocket
        let (ws_stream, _response) = connect_async(&self.url).await?;
        let (mut sink, mut stream) = ws_stream.split();

        self.connected.store(true, Ordering::SeqCst);

        // Wait for Hello
        let hello = self.wait_for_hello(&mut stream).await?;
        let heartbeat_interval = Duration::from_millis(hello.heartbeat_interval as u64);
        debug!(interval_ms = hello.heartbeat_interval, "Received Hello");

        // Send Identify
        let identify = IdentifyPayload {
            server_id: guild_id.to_string(),
            user_id: user_id.to_string(),
            session_id: session_id.clone(),
            token: token.clone(),
        };
        self.send_payload(&mut sink, VoiceOpCode::Identify, &identify)
            .await?;
        info!("Sent Voice Identify");

        // Wait for Ready
        let ready = self.wait_for_ready(&mut stream).await?;
        info!(ssrc = ready.ssrc, ip = %ready.ip, port = ready.port, "Received Voice Ready");

        // Send Ready event
        let _ = event_tx.send(VoiceEvent::Ready(ready.clone())).await;

        // Start heartbeat task
        self.heartbeat_acked.store(true, Ordering::SeqCst);
        let _heartbeat_interval_ms = heartbeat_interval.as_millis() as u64;

        // Main event loop
        let mut heartbeat_timer = interval(heartbeat_interval);
        let mut first_heartbeat = true;

        loop {
            tokio::select! {
                // Command channel
                command = command_rx.recv() => {
                    match command {
                        Some(VoiceCommand::SelectProtocol { protocol, ip, port, mode }) => {
                            if let Err(e) = self.send_select_protocol(&mut sink, &protocol, &ip, port, &mode).await {
                                error!(?e, "Failed to send Select Protocol");
                                break;
                            }
                        }
                        Some(VoiceCommand::Speaking { speaking, delay, ssrc }) => {
                            if let Err(e) = self.send_speaking(&mut sink, speaking, delay, ssrc).await {
                                error!(?e, "Failed to send Speaking");
                                break;
                            }
                        }
                        None => {
                            info!("Voice command channel closed");
                            break;
                        }
                    }
                }

                // Heartbeat timer
                _ = heartbeat_timer.tick() => {
                    // First heartbeat should be after a jitter delay
                    if first_heartbeat {
                        first_heartbeat = false;
                        let jitter = rand::random::<f64>() * heartbeat_interval.as_secs_f64();
                        sleep(Duration::from_secs_f64(jitter)).await;
                    }

                    // Check if last heartbeat was acknowledged
                    if !self.heartbeat_acked.load(Ordering::SeqCst) {
                        warn!("No heartbeat ACK received, connection may be dead");
                        return Err(VoiceError::Timeout("Heartbeat ACK".to_string()));
                    }

                    // Send heartbeat
                    let nonce = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64;

                    self.last_heartbeat_nonce.store(nonce, Ordering::SeqCst);
                    self.heartbeat_acked.store(false, Ordering::SeqCst);

                    let heartbeat = HeartbeatPayload { nonce, seq_ack: None };
                    if let Err(e) = self.send_payload(&mut sink, VoiceOpCode::Heartbeat, &heartbeat).await {
                        error!(?e, "Failed to send heartbeat");
                        break;
                    }
                    trace!(nonce = nonce, "Sent heartbeat");
                }

                // WebSocket message
                message = stream.next() => {
                    match message {
                        Some(Ok(WsMessage::Text(text))) => {
                            if let Err(e) = self.handle_message(&text, &mut sink, &event_tx).await {
                                error!(?e, "Failed to handle message");
                            }
                        }
                        Some(Ok(WsMessage::Close(frame))) => {
                            let (code, reason) = frame
                                .map(|f| (f.code.into(), f.reason.to_string()))
                                .unwrap_or((0, "No reason".to_string()));
                            info!(code = code, reason = %reason, "Voice WebSocket closed");
                            let _ = event_tx.send(VoiceEvent::Closed { code, reason }).await;
                            break;
                        }
                        Some(Ok(_)) => {} // Ignore other message types
                        Some(Err(e)) => {
                            error!(?e, "WebSocket error");
                            break;
                        }
                        None => {
                            info!("Voice WebSocket stream ended");
                            break;
                        }
                    }
                }
            }
        }

        self.connected.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Wait for Hello payload.
    async fn wait_for_hello(
        &self,
        stream: &mut futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) -> Result<HelloPayload, VoiceError> {
        let timeout = Duration::from_secs(10);

        match tokio::time::timeout(timeout, async {
            while let Some(msg) = stream.next().await {
                if let WsMessage::Text(text) = msg? {
                    let payload: RawVoicePayload = serde_json::from_str(&text)?;
                    if payload.op == VoiceOpCode::Hello {
                        if let Some(data) = payload.d {
                            return Ok(serde_json::from_value(data)?);
                        }
                    }
                }
            }
            Err(VoiceError::Closed {
                code: 0,
                reason: "Stream ended before Hello".to_string(),
            })
        })
        .await
        {
            Ok(result) => result,
            Err(_) => Err(VoiceError::Timeout("Hello".to_string())),
        }
    }

    /// Wait for Ready payload.
    async fn wait_for_ready(
        &self,
        stream: &mut futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) -> Result<VoiceReadyInfo, VoiceError> {
        let timeout = Duration::from_secs(10);

        match tokio::time::timeout(timeout, async {
            while let Some(msg) = stream.next().await {
                if let WsMessage::Text(text) = msg? {
                    let payload: RawVoicePayload = serde_json::from_str(&text)?;
                    if payload.op == VoiceOpCode::Ready {
                        if let Some(data) = payload.d {
                            let ready: ReadyPayload = serde_json::from_value(data)?;
                            return Ok(VoiceReadyInfo {
                                ssrc: ready.ssrc,
                                ip: ready.ip,
                                port: ready.port,
                                modes: ready.modes,
                            });
                        }
                    }
                }
            }
            Err(VoiceError::Closed {
                code: 0,
                reason: "Stream ended before Ready".to_string(),
            })
        })
        .await
        {
            Ok(result) => result,
            Err(_) => Err(VoiceError::Timeout("Ready".to_string())),
        }
    }

    /// Handle an incoming message.
    async fn handle_message(
        &self,
        text: &str,
        _sink: &mut futures_util::stream::SplitSink<
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            WsMessage,
        >,
        event_tx: &mpsc::Sender<VoiceEvent>,
    ) -> Result<(), VoiceError> {
        let payload: RawVoicePayload = serde_json::from_str(text)?;

        match payload.op {
            VoiceOpCode::HeartbeatAck => {
                self.heartbeat_acked.store(true, Ordering::SeqCst);

                // Calculate latency
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64;
                let sent = self.last_heartbeat_nonce.load(Ordering::SeqCst);
                let latency = now.saturating_sub(sent);
                *self.heartbeat_latency_ms.write() = Some(latency);
                trace!(latency_ms = latency, "Heartbeat ACK");
            }

            VoiceOpCode::SessionDescription => {
                if let Some(data) = payload.d {
                    let desc: SessionDescriptionPayload = serde_json::from_value(data)?;
                    info!(mode = %desc.mode, "Received session description");
                    let _ = event_tx
                        .send(VoiceEvent::SessionDescription(VoiceSessionDescription {
                            mode: desc.mode,
                            secret_key: desc.secret_key,
                        }))
                        .await;
                }
            }

            VoiceOpCode::ClientConnect => {
                if let Some(data) = payload.d {
                    let client: ClientConnectPayload = serde_json::from_value(data)?;
                    debug!(user_id = client.user_id.get(), "Client connected to voice");
                    let _ = event_tx
                        .send(VoiceEvent::ClientConnect {
                            user_id: client.user_id.get(),
                            ssrc: client.audio_ssrc,
                        })
                        .await;
                }
            }

            VoiceOpCode::ClientDisconnect => {
                if let Some(data) = payload.d {
                    let client: ClientDisconnectPayload = serde_json::from_value(data)?;
                    debug!(
                        user_id = client.user_id.get(),
                        "Client disconnected from voice"
                    );
                    let _ = event_tx
                        .send(VoiceEvent::ClientDisconnect {
                            user_id: client.user_id.get(),
                        })
                        .await;
                }
            }

            VoiceOpCode::Resumed => {
                info!("Voice session resumed");
                let _ = event_tx.send(VoiceEvent::Resumed).await;
            }

            VoiceOpCode::Speaking => {
                // We don't need to process incoming Speaking events
                trace!("Received Speaking event");
            }

            _ => {
                debug!(?payload.op, "Unhandled voice opcode");
            }
        }

        Ok(())
    }

    /// Send Select Protocol payload.
    pub async fn send_select_protocol(
        &self,
        sink: &mut futures_util::stream::SplitSink<
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            WsMessage,
        >,
        protocol: &str,
        ip: &str,
        port: u16,
        mode: &str,
    ) -> Result<(), VoiceError> {
        let payload = SelectProtocolPayload {
            protocol: protocol.to_string(),
            data: SelectProtocolData {
                address: ip.to_string(),
                port,
                mode: mode.to_string(),
            },
        };
        self.send_payload(sink, VoiceOpCode::SelectProtocol, &payload)
            .await
    }

    /// Send Speaking payload.
    pub async fn send_speaking(
        &self,
        sink: &mut futures_util::stream::SplitSink<
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            WsMessage,
        >,
        speaking: SpeakingFlags,
        delay: u32,
        ssrc: u32,
    ) -> Result<(), VoiceError> {
        let payload = SpeakingPayload {
            speaking: speaking.bits(),
            delay,
            ssrc,
        };
        self.send_payload(sink, VoiceOpCode::Speaking, &payload)
            .await
    }

    /// Send a payload.
    async fn send_payload<D: serde::Serialize>(
        &self,
        sink: &mut futures_util::stream::SplitSink<
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            WsMessage,
        >,
        op: VoiceOpCode,
        data: &D,
    ) -> Result<(), VoiceError> {
        let payload = VoicePayload { op, d: Some(data) };
        let json = serde_json::to_string(&payload)?;
        sink.send(WsMessage::Text(json.into())).await?;
        Ok(())
    }

    /// Check if connected.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    /// Get heartbeat latency.
    pub fn heartbeat_latency_ms(&self) -> Option<u64> {
        *self.heartbeat_latency_ms.read()
    }
}
