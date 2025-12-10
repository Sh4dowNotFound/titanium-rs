//! Shard implementation for Discord Gateway connection.
//!
//! A Shard represents a single WebSocket connection to Discord's Gateway.
//! For large bots, multiple shards are used to distribute guild events.

use crate::compression::ZlibDecompressor;
use crate::error::{CloseCode, GatewayError};
use crate::event::{parse_event, Event, ReadyEventData};
use crate::heartbeat::HeartbeatHandler;
use crate::opcode::OpCode;
use crate::payload::{
    create_heartbeat_payload, GatewayPayload, HelloPayload, IdentifyPayload, RawGatewayPayload,
    ResumePayload,
};
use crate::ratelimit::{exponential_backoff, with_jitter, IdentifyRateLimiter};
use crate::{DEFAULT_GATEWAY_URL, GATEWAY_VERSION};

use flume::Sender;
use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
#[cfg(feature = "simd")]
use simd_json::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, trace, warn};
use url::Url;

/// Command sent to the shard from the application.
#[derive(Debug)]
enum ShardCommand {
    /// Send a raw JSON payload.
    Send(String),
}

/// Internal action to take after parsing a frame.
enum GatewayAction {
    Dispatch(Event<'static>),
    Heartbeat,
    Reconnect,
    InvalidSession(bool),
    None,
}

/// Type alias for the WebSocket stream.
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Shard connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShardState {
    /// Disconnected, not running.
    Disconnected,
    /// Attempting to connect.
    Connecting,
    /// Connected, waiting for Hello.
    Handshaking,
    /// Sending Identify.
    Identifying,
    /// Resuming a previous session.
    Resuming,
    /// Fully connected and receiving events.
    Connected,
    /// Reconnecting after disconnect.
    Reconnecting,
    /// Shutting down.
    Disconnecting,
}

/// Configuration for a shard.
#[derive(Debug, Clone)]
pub struct ShardConfig {
    /// Bot token.
    pub token: String,

    /// Gateway intents.
    pub intents: titanium_model::Intents,

    /// Gateway URL (usually from /gateway/bot).
    pub gateway_url: String,

    /// Large guild threshold (50-250).
    pub large_threshold: u8,

    /// Enable zlib compression.
    pub compress: bool,

    /// Maximum reconnection attempts before giving up.
    pub max_reconnect_attempts: u32,

    /// Base reconnect delay in milliseconds.
    pub reconnect_base_delay_ms: u64,

    /// Maximum reconnect delay in milliseconds.
    pub reconnect_max_delay_ms: u64,
}

impl ShardConfig {
    /// Create a new shard configuration with required fields.
    pub fn new(token: impl Into<String>, intents: titanium_model::Intents) -> Self {
        Self {
            token: token.into(),
            intents,
            gateway_url: DEFAULT_GATEWAY_URL.to_string(),
            large_threshold: 250,
            compress: false,
            max_reconnect_attempts: 10,
            reconnect_base_delay_ms: 1000,
            reconnect_max_delay_ms: 60000,
        }
    }

    /// Set a custom gateway URL.
    pub fn with_gateway_url(mut self, url: impl Into<String>) -> Self {
        self.gateway_url = url.into();
        self
    }
}

/// Session data for resuming connections.
#[derive(Debug, Clone)]
struct SessionData {
    /// Session ID from Ready event.
    session_id: String,
    /// Resume URL from Ready event.
    resume_url: String,
}

/// A Discord Gateway shard.
///
/// Handles WebSocket connection, heartbeating, event dispatch,
/// and automatic reconnection with session resumption.
pub struct Shard {
    // =========================================================================
    // Identity
    // =========================================================================
    /// This shard's ID.
    shard_id: u16,

    /// Total number of shards.
    total_shards: u16,

    // =========================================================================
    // Configuration
    // =========================================================================
    /// Shard configuration.
    config: ShardConfig,

    /// Rate limiter for identify (shared across cluster).
    rate_limiter: Arc<IdentifyRateLimiter>,

    // =========================================================================
    // State
    // =========================================================================
    /// Current connection state.
    state: RwLock<ShardState>,

    /// Session data for resuming.
    session: RwLock<Option<SessionData>>,

    /// Last sequence number received.
    sequence: AtomicU64,

    /// Heartbeat handler.
    heartbeat: HeartbeatHandler,

    /// Zlib-stream decompressor.
    decompressor: RwLock<ZlibDecompressor>,

    /// Whether shutdown has been requested.
    shutdown: AtomicBool,

    /// Channel for sending commands to the shard loop.
    command_tx: Sender<ShardCommand>,

    /// Channel for receiving commands in the shard loop.
    command_rx: flume::Receiver<ShardCommand>,
}

impl Shard {
    /// Create a new shard.
    ///
    /// # Arguments
    /// * `shard_id` - This shard's ID (0-indexed).
    /// * `total_shards` - Total number of shards.
    /// * `config` - Shard configuration.
    pub fn new(shard_id: u16, total_shards: u16, config: ShardConfig) -> Self {
        Self::with_rate_limiter(
            shard_id,
            total_shards,
            config,
            Arc::new(IdentifyRateLimiter::default()),
        )
    }

    /// Create a new shard with a shared rate limiter.
    pub fn with_rate_limiter(
        shard_id: u16,
        total_shards: u16,
        config: ShardConfig,
        rate_limiter: Arc<IdentifyRateLimiter>,
    ) -> Self {
        let (tx, rx) = flume::unbounded();

        Self {
            shard_id,
            total_shards,
            config,
            rate_limiter,
            state: RwLock::new(ShardState::Disconnected),
            session: RwLock::new(None),
            sequence: AtomicU64::new(0),
            heartbeat: HeartbeatHandler::new(),
            decompressor: RwLock::new(ZlibDecompressor::new()),
            shutdown: AtomicBool::new(false),
            command_tx: tx,
            command_rx: rx,
        }
    }

    /// Get the shard ID.
    pub fn shard_id(&self) -> u16 {
        self.shard_id
    }

    /// Get the total number of shards.
    pub fn total_shards(&self) -> u16 {
        self.total_shards
    }

    /// Get the current shard state.
    pub fn state(&self) -> ShardState {
        *self.state.read()
    }

    /// Get the last sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence.load(Ordering::SeqCst)
    }

    /// Request a graceful shutdown.
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }

    /// Get the last measured latency.
    pub fn latency(&self) -> Option<Duration> {
        self.heartbeat.latency()
    }

    /// Send a raw payload to the gateway.
    ///
    /// This is useful for sending voice state updates (Op 4) or presence updates (Op 3).
    /// Accepts any type that implements `serde::Serialize`.
    pub fn send_payload<T: serde::Serialize>(&self, payload: &T) -> Result<(), GatewayError> {
        #[cfg(feature = "simd")]
        let json = simd_json::to_string(payload).map_err(|e| GatewayError::Closed {
            code: 0,
            reason: format!("Serialization error: {}", e),
        })?;

        #[cfg(not(feature = "simd"))]
        let json = serde_json::to_string(payload)?;

        self.command_tx
            .send(ShardCommand::Send(json))
            .map_err(|_| GatewayError::Closed {
                code: 0,
                reason: "Shard command channel closed".to_string(),
            })
    }

    /// Run the shard event loop.
    ///
    /// This will connect to Discord, handle events, and automatically reconnect
    /// on disconnection. Events are sent to the provided channel.
    ///
    /// # Arguments
    /// * `event_tx` - Channel to send parsed events to.
    ///
    /// # Returns
    /// Returns `Ok(())` on graceful shutdown, or an error if unrecoverable.
    pub async fn run(&self, event_tx: Sender<Event<'static>>) -> Result<(), GatewayError> {
        let mut reconnect_attempts = 0u32;
        let mut read_buffer = Vec::with_capacity(32 * 1024);

        loop {
            // Check for shutdown
            if self.shutdown.load(Ordering::SeqCst) {
                info!(shard_id = self.shard_id, "Shard shutdown requested");
                *self.state.write() = ShardState::Disconnecting;
                return Ok(());
            }

            // Connect and run
            match self.connect_and_run(&event_tx, &mut read_buffer).await {
                Ok(()) => {
                    // Graceful disconnect (shutdown requested)
                    return Ok(());
                }
                Err(GatewayError::HeartbeatTimeout) => {
                    warn!(
                        shard_id = self.shard_id,
                        "Heartbeat timeout, reconnecting..."
                    );
                    reconnect_attempts += 1;
                }
                Err(GatewayError::InvalidSession { resumable }) => {
                    if !resumable {
                        // Clear session to force new identify
                        *self.session.write() = None;
                        self.sequence.store(0, Ordering::SeqCst);
                    }
                    warn!(
                        shard_id = self.shard_id,
                        resumable = resumable,
                        "Session invalidated, reconnecting..."
                    );
                    reconnect_attempts += 1;
                }
                Err(GatewayError::Closed { code, reason }) => {
                    let close_code = CloseCode::from_code(code);

                    if let Some(cc) = close_code {
                        if !cc.can_reconnect() {
                            error!(
                                shard_id = self.shard_id,
                                code = code,
                                reason = %reason,
                                "Fatal close code, cannot reconnect"
                            );
                            return Err(GatewayError::Closed { code, reason });
                        }
                    }

                    warn!(
                        shard_id = self.shard_id,
                        code = code,
                        reason = %reason,
                        "Connection closed, reconnecting..."
                    );
                    reconnect_attempts += 1;
                }
                Err(e) => {
                    error!(shard_id = self.shard_id, error = %e, "Shard error");
                    reconnect_attempts += 1;
                }
            }

            // Check reconnect limit
            if reconnect_attempts > self.config.max_reconnect_attempts {
                error!(
                    shard_id = self.shard_id,
                    attempts = reconnect_attempts,
                    "Max reconnect attempts exceeded"
                );
                return Err(GatewayError::Closed {
                    code: 0,
                    reason: "Max reconnect attempts exceeded".to_string(),
                });
            }

            // Calculate backoff
            let backoff = exponential_backoff(
                reconnect_attempts - 1,
                self.config.reconnect_base_delay_ms,
                self.config.reconnect_max_delay_ms,
            );
            let backoff_with_jitter = with_jitter(backoff, 0.25);

            info!(
                shard_id = self.shard_id,
                attempt = reconnect_attempts,
                backoff_ms = backoff_with_jitter.as_millis(),
                "Waiting before reconnect"
            );

            *self.state.write() = ShardState::Reconnecting;
            sleep(backoff_with_jitter).await;
        }
    }

    /// Connect and run the event loop once.
    async fn connect_and_run(
        &self,
        event_tx: &Sender<Event<'static>>,
        buffer: &mut Vec<u8>,
    ) -> Result<(), GatewayError> {
        // Build connection URL
        let gateway_url = self.build_gateway_url()?;

        info!(shard_id = self.shard_id, url = %gateway_url, "Connecting to Gateway");
        *self.state.write() = ShardState::Connecting;

        // Connect WebSocket
        let (ws_stream, _response) = connect_async(gateway_url.as_str()).await?;
        let (mut sink, mut stream) = ws_stream.split();

        info!(shard_id = self.shard_id, "WebSocket connected");
        *self.state.write() = ShardState::Handshaking;

        // Wait for Hello
        let hello = self.wait_for_hello(&mut stream).await?;
        let heartbeat_interval = Duration::from_millis(hello.heartbeat_interval);
        self.heartbeat.set_interval(heartbeat_interval);

        debug!(
            shard_id = self.shard_id,
            interval_ms = hello.heartbeat_interval,
            "Received Hello"
        );

        // Send Identify or Resume
        self.rate_limiter.acquire().await;

        let session = self.session.read().clone();
        if let Some(ref session_data) = session {
            // Try to resume
            *self.state.write() = ShardState::Resuming;
            info!(
                shard_id = self.shard_id,
                session_id = %session_data.session_id,
                "Resuming session"
            );
            self.send_resume(&mut sink, session_data).await?;
        } else {
            // Fresh identify
            *self.state.write() = ShardState::Identifying;
            info!(shard_id = self.shard_id, "Sending Identify");
            self.send_identify(&mut sink).await?;
        }

        // Reset heartbeat ACK
        self.heartbeat.reset();

        // Send IMMEDIATE heartbeat to get latency measured right away
        self.send_heartbeat(&mut sink).await?;
        self.heartbeat.mark_sent();

        // Schedule next heartbeat at normal interval
        let mut next_heartbeat = Instant::now() + heartbeat_interval;

        // Main event loop
        loop {
            // Check shutdown
            if self.shutdown.load(Ordering::SeqCst) {
                // Send close frame
                let _ = sink.close().await;
                return Ok(());
            }

            tokio::select! {
                // WebSocket message received
                message = stream.next() => {
                    match message {
                        Some(Ok(msg)) => {
                            self.handle_message(msg, event_tx, &mut sink, buffer).await?;
                        }
                        Some(Err(e)) => {
                            return Err(GatewayError::WebSocket(e));
                        }
                        None => {
                            // Stream closed
                            return Err(GatewayError::Closed {
                                code: 0,
                                reason: "WebSocket stream ended".to_string(),
                            });
                        }
                    }
                }

                // Heartbeat timer
                _ = sleep(next_heartbeat.saturating_duration_since(Instant::now())) => {
                    // Check if we received ACK for last heartbeat
                    if !self.heartbeat.is_acked() {
                        error!(shard_id = self.shard_id, "No heartbeat ACK received, assuming zombie connection");
                        return Err(GatewayError::HeartbeatTimeout);
                    }

                    // Send heartbeat
                    self.send_heartbeat(&mut sink).await?;
                    self.heartbeat.mark_sent();

                    // Schedule next heartbeat
                    next_heartbeat = Instant::now() + self.heartbeat.interval();
                }

                // Command channel
                command = self.command_rx.recv_async() => {
                    match command {
                        Ok(ShardCommand::Send(json)) => {
                            trace!(shard_id = self.shard_id, "Sending custom payload");
                            sink.send(WsMessage::Text(json.into())).await?;
                        }
                        Err(_) => {
                            // Channel closed, connection likely dropping
                            return Err(GatewayError::Closed {
                                code: 0,
                                reason: "Command channel closed".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    /// Build the gateway URL with query parameters.
    fn build_gateway_url(&self) -> Result<Url, GatewayError> {
        // Use resume URL if available, otherwise default
        let base_url = self
            .session
            .read()
            .as_ref()
            .map(|s| s.resume_url.clone())
            .unwrap_or_else(|| self.config.gateway_url.clone());

        let mut url = Url::parse(&base_url).map_err(|e| GatewayError::Closed {
            code: 0,
            reason: format!("Invalid URL: {}", e),
        })?;

        // Add query parameters
        url.query_pairs_mut()
            .append_pair("v", &GATEWAY_VERSION.to_string())
            .append_pair("encoding", "json");

        if self.config.compress {
            url.query_pairs_mut().append_pair("compress", "zlib-stream");
        }

        Ok(url)
    }

    /// Wait for the Hello payload after connecting.
    async fn wait_for_hello(
        &self,
        stream: &mut futures_util::stream::SplitStream<WsStream>,
    ) -> Result<HelloPayload, GatewayError> {
        // Timeout for Hello (10 seconds)
        let hello_timeout = Duration::from_secs(10);

        let message = timeout(hello_timeout, stream.next())
            .await
            .map_err(|_| GatewayError::Closed {
                code: 0,
                reason: "Timeout waiting for Hello".to_string(),
            })?
            .ok_or_else(|| GatewayError::Closed {
                code: 0,
                reason: "Connection closed before Hello".to_string(),
            })??;

        if let WsMessage::Text(text) = message {
            let payload: RawGatewayPayload = serde_json::from_str(&text)?;

            if payload.op == OpCode::Hello {
                if let Some(data) = payload.d {
                    #[cfg(feature = "simd")]
                    let hello: HelloPayload = titanium_model::json::from_value(data)?;
                    #[cfg(not(feature = "simd"))]
                    let hello: HelloPayload = serde_json::from_str(data.get())?;
                    return Ok(hello);
                }
            }
        }

        Err(GatewayError::Closed {
            code: 0,
            reason: "Expected Hello payload".to_string(),
        })
    }

    /// Send an Identify payload.
    async fn send_identify(
        &self,
        sink: &mut futures_util::stream::SplitSink<WsStream, WsMessage>,
    ) -> Result<(), GatewayError> {
        let identify = IdentifyPayload::new(
            std::borrow::Cow::Borrowed(self.config.token.as_str()),
            self.config.intents,
        )
        .with_shard(self.shard_id, self.total_shards);

        let payload = GatewayPayload::new(OpCode::Identify, identify);

        #[cfg(feature = "simd")]
        let json = simd_json::to_string(&payload).map_err(|e| GatewayError::Closed {
            code: 0,
            reason: e.to_string(),
        })?;

        #[cfg(not(feature = "simd"))]
        let json = serde_json::to_string(&payload)?;

        trace!(shard_id = self.shard_id, "Sending Identify payload");
        sink.send(WsMessage::Text(json.into())).await?;

        Ok(())
    }

    /// Send a Resume payload.
    async fn send_resume(
        &self,
        sink: &mut futures_util::stream::SplitSink<WsStream, WsMessage>,
        session: &SessionData,
    ) -> Result<(), GatewayError> {
        let resume = ResumePayload {
            token: std::borrow::Cow::Borrowed(self.config.token.as_str()),
            session_id: std::borrow::Cow::Borrowed(session.session_id.as_str()),
            seq: self.sequence.load(Ordering::SeqCst),
        };

        let payload = GatewayPayload::new(OpCode::Resume, resume);

        #[cfg(feature = "simd")]
        let json = simd_json::to_string(&payload).map_err(|e| GatewayError::Closed {
            code: 0,
            reason: e.to_string(),
        })?;

        #[cfg(not(feature = "simd"))]
        let json = serde_json::to_string(&payload)?;

        trace!(shard_id = self.shard_id, "Sending Resume payload");
        sink.send(WsMessage::Text(json.into())).await?;

        Ok(())
    }

    /// Send a Heartbeat payload.
    async fn send_heartbeat(
        &self,
        sink: &mut futures_util::stream::SplitSink<WsStream, WsMessage>,
    ) -> Result<(), GatewayError> {
        let seq = self.sequence.load(Ordering::SeqCst);
        let seq_opt = if seq > 0 { Some(seq) } else { None };

        let json = create_heartbeat_payload(seq_opt);

        trace!(shard_id = self.shard_id, seq = seq, "Sending Heartbeat");
        sink.send(WsMessage::Text(json.into())).await?;

        Ok(())
    }

    /// Handle a received WebSocket message.
    async fn handle_message(
        &self,
        message: WsMessage,
        event_tx: &Sender<Event<'static>>,
        sink: &mut futures_util::stream::SplitSink<WsStream, WsMessage>,
        buffer: &mut Vec<u8>,
    ) -> Result<(), GatewayError> {
        let action = match message {
            WsMessage::Text(text) => {
                // Reuse scratch buffer to avoid allocation
                buffer.clear();
                buffer.extend_from_slice(text.as_str().as_bytes());
                self.process_frame(buffer)?
            }
            WsMessage::Binary(data) => {
                // Binary messages are zlib-compressed
                // We use scopes to drop locks quickly
                let mut decompressor = self.decompressor.write();
                match decompressor.push(&data) {
                    Ok(Some(msg)) => self.process_frame(msg)?,
                    Ok(None) => GatewayAction::None, // Incomplete
                    Err(e) => {
                        return Err(GatewayError::JsonDecode(format!(
                            "Decompression error: {}",
                            e
                        )))
                    }
                }
            }
            WsMessage::Close(frame) => {
                let (code, reason) = frame
                    .map(|f: CloseFrame| (f.code.into(), f.reason.to_string()))
                    .unwrap_or((0, String::new()));

                return Err(GatewayError::Closed { code, reason });
            }
            WsMessage::Ping(data) => {
                sink.send(WsMessage::Pong(data)).await?;
                return Ok(());
            }
            WsMessage::Pong(_) => return Ok(()),
            WsMessage::Frame(_) => return Ok(()),
        };

        match action {
            GatewayAction::Dispatch(event) => {
                event_tx.send_async(event).await?;
            }
            GatewayAction::Heartbeat => {
                debug!(shard_id = self.shard_id, "Received Heartbeat request");
                self.send_heartbeat(sink).await?;
            }
            GatewayAction::Reconnect => {
                info!(shard_id = self.shard_id, "Received Reconnect request");
                return Err(GatewayError::Closed {
                    code: 0,
                    reason: "Server requested reconnect".to_string(),
                });
            }
            GatewayAction::InvalidSession(resumable) => {
                warn!(
                    shard_id = self.shard_id,
                    resumable = resumable,
                    "Session invalidated"
                );
                return Err(GatewayError::InvalidSession { resumable });
            }
            GatewayAction::None => {}
        }

        Ok(())
    }

    /// Process a text frame (JSON) and determine the action.
    ///
    /// # optimization
    /// Accepts `&mut [u8]` to allow in-place SIMD parsing.
    /// This function is synchronous and does NOT hold locks across awaits.
    fn process_frame(&self, text: &mut [u8]) -> Result<GatewayAction, GatewayError> {
        #[cfg(feature = "simd")]
        {
            // Zero-Copy parse whole buffer
            let json = titanium_model::json::to_borrowed_value(text)
                .map_err(|e| GatewayError::JsonDecode(e.to_string()))?;

            // Update sequence number
            if let Some(seq) = json["s"].as_u64() {
                self.sequence.store(seq, Ordering::SeqCst);
            }

            // Parse OpCode
            let op_val = json["op"].clone();
            let op: OpCode = titanium_model::json::from_borrowed_value(op_val)
                .map_err(|e| GatewayError::JsonDecode(e.to_string()))?;

            match op {
                OpCode::Dispatch => {
                    let d_val = json["d"].clone();
                    if let Some(event_name) = json["t"].as_str() {
                        let event_result = parse_event(event_name, d_val)?;

                        if let Event::Ready(ref ready) = event_result {
                            self.handle_ready(ready);
                        }

                        return Ok(GatewayAction::Dispatch(event_result));
                    }
                }

                OpCode::Heartbeat => return Ok(GatewayAction::Heartbeat),
                OpCode::Reconnect => return Ok(GatewayAction::Reconnect),

                OpCode::InvalidSession => {
                    let resumable = json["d"].as_bool().unwrap_or(false);
                    return Ok(GatewayAction::InvalidSession(resumable));
                }

                OpCode::HeartbeatAck => {
                    self.heartbeat.mark_acked();
                    let rtt = self.heartbeat.latency().unwrap_or_default();
                    trace!(
                        shard_id = self.shard_id,
                        rtt_ms = rtt.as_millis(),
                        "Heartbeat ACK received"
                    );
                }

                _ => {
                    trace!(
                        shard_id = self.shard_id,
                        opcode = ?op,
                        "Ignoring opcode"
                    );
                }
            }
        }

        #[cfg(not(feature = "simd"))]
        {
            let payload: RawGatewayPayload = titanium_model::json::from_slice_mut(text)
                .map_err(|e| GatewayError::JsonDecode(e.to_string()))?;

            if let Some(seq) = payload.s {
                self.sequence.store(seq, Ordering::SeqCst);
            }

            match payload.op {
                OpCode::Dispatch => {
                    if let (Some(event_name), Some(data)) = (payload.t.as_deref(), payload.d) {
                        let event_name = event_name.to_string();
                        // For non-simd, we clone data to avoid holding the buffer, effectively similar logic
                        // but here we just process synchronously.
                        let json_string = data.get().to_string();

                        let raw_value = serde_json::value::RawValue::from_string(json_string)
                                .map_err(GatewayError::from)?;
                        let event_result = parse_event(&event_name, &raw_value)?;

                         if let Event::Ready(ref ready) = event_result {
                            self.handle_ready(ready);
                        }
                        return Ok(GatewayAction::Dispatch(event_result));
                    }
                }
                 OpCode::Heartbeat => return Ok(GatewayAction::Heartbeat),
                 OpCode::Reconnect => return Ok(GatewayAction::Reconnect),
                 OpCode::InvalidSession => {
                      // Need to parse 'd' as bool manually or assume structure
                      // RawGatewayPayload d is Box<RawValue>.
                      // A bit hacky but for standard compliance:
                      let resumable = payload.d.map(|d| d.get() == "true").unwrap_or(false);
                      return Ok(GatewayAction::InvalidSession(resumable));
                 }
                OpCode::HeartbeatAck => {
                    self.heartbeat.mark_acked();
                }
                _ => {}
            }
        }

        Ok(GatewayAction::None)
    }


    /// Handle the Ready event to store session data.
    fn handle_ready(&self, ready: &ReadyEventData) {
        *self.session.write() = Some(SessionData {
            session_id: ready.session_id.clone(),
            resume_url: ready.resume_gateway_url.clone(),
        });
        *self.state.write() = ShardState::Connected;

        info!(
            shard_id = self.shard_id,
            session_id = %ready.session_id,
            guilds = ready.guilds.len(),
            "Shard connected"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use titanium_model::Intents;

    #[test]
    fn test_shard_config() {
        let config = ShardConfig::new("test_token", Intents::GUILDS | Intents::GUILD_MESSAGES);
        assert_eq!(config.token, "test_token");
        assert!(config.intents.contains(Intents::GUILDS));
    }

    #[test]
    fn test_shard_creation() {
        let config = ShardConfig::new("test_token", Intents::default());
        let shard = Shard::new(0, 1, config);

        assert_eq!(shard.shard_id(), 0);
        assert_eq!(shard.total_shards(), 1);
        assert_eq!(shard.state(), ShardState::Disconnected);
    }

    #[test]
    fn test_gateway_url_building() {
        let config = ShardConfig::new("test", Intents::default());
        let shard = Shard::new(0, 1, config);

        let url = shard.build_gateway_url().expect("Failed to build URL");
        assert!(url.as_str().contains("v=10"));
        assert!(url.as_str().contains("encoding=json"));
    }
}
