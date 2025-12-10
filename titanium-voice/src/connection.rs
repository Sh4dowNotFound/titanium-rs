//! Voice connection state machine.
//!
//! Orchestrates WebSocket and UDP connections for Discord voice.

use crate::error::VoiceError;
use crate::payload::{EncryptionMode, SpeakingFlags};
use crate::udp::VoiceUdp;
use crate::ws::{VoiceEvent, VoiceWebSocket};
use parking_lot::RwLock;
use std::sync::atomic::{AtomicU8, Ordering};

use std::sync::Arc;
use tokio::sync::{mpsc, RwLock as AsyncRwLock};
use tracing::{debug, error, info, warn};

/// Voice connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VoiceState {
    /// Not connected.
    Disconnected = 0,
    /// Connecting to WebSocket.
    Connecting = 1,
    /// Performing IP discovery.
    Discovering = 2,
    /// Selecting protocol.
    SelectingProtocol = 3,
    /// Ready to send/receive audio.
    Ready = 4,
    /// Disconnecting.
    Disconnecting = 5,
}

impl From<u8> for VoiceState {
    fn from(value: u8) -> Self {
        match value {
            0 => VoiceState::Disconnected,
            1 => VoiceState::Connecting,
            2 => VoiceState::Discovering,
            3 => VoiceState::SelectingProtocol,
            4 => VoiceState::Ready,
            5 => VoiceState::Disconnecting,
            _ => VoiceState::Disconnected,
        }
    }
}

/// Voice connection configuration.
#[derive(Debug, Clone)]
pub struct VoiceConfig {
    /// Guild ID.
    pub guild_id: u64,
    /// Channel ID.
    pub channel_id: u64,
    /// User ID.
    pub user_id: u64,
    /// Session ID from VOICE_STATE_UPDATE.
    pub session_id: String,
    /// Voice server endpoint.
    pub endpoint: String,
    /// Voice token.
    pub token: String,
}

/// A Discord voice connection.
pub struct VoiceConnection {
    /// Configuration.
    config: VoiceConfig,
    /// Current state.
    state: AtomicU8,
    /// UDP transport.
    udp: AsyncRwLock<Option<VoiceUdp>>,
    /// SSRC assigned by server.
    ssrc: RwLock<Option<u32>>,
    /// Currently speaking.
    speaking: RwLock<bool>,
    /// Command sender.
    command_tx: RwLock<Option<mpsc::Sender<crate::ws::VoiceCommand>>>,
}

impl VoiceConnection {
    /// Create a new voice connection.
    pub fn new(config: VoiceConfig) -> Self {
        Self {
            config,
            state: AtomicU8::new(VoiceState::Disconnected as u8),
            udp: AsyncRwLock::new(None),
            ssrc: RwLock::new(None),
            speaking: RwLock::new(false),
            command_tx: RwLock::new(None),
        }
    }

    /// Get the current state.
    pub fn state(&self) -> VoiceState {
        VoiceState::from(self.state.load(Ordering::SeqCst))
    }

    /// Check if ready to send audio.
    pub fn is_ready(&self) -> bool {
        self.state() == VoiceState::Ready
    }

    /// Connect to the voice server.
    pub async fn connect(self: &Arc<Self>) -> Result<(), VoiceError> {
        self.state
            .store(VoiceState::Connecting as u8, Ordering::SeqCst);

        // Create WebSocket handler
        let ws = VoiceWebSocket::new(&self.config.endpoint);

        // Create event channel
        let (event_tx, mut event_rx) = mpsc::channel(100);

        // Create command channel
        let (command_tx, command_rx) = mpsc::channel(100);
        *self.command_tx.write() = Some(command_tx);

        // Spawn WebSocket task
        let session_id = self.config.session_id.clone();
        let guild_id = self.config.guild_id;
        let user_id = self.config.user_id;
        let token = self.config.token.clone();

        let ws = Arc::new(ws);
        let ws_clone = Arc::clone(&ws);

        info!(guild_id = guild_id, "Connecting to voice");

        tokio::spawn(async move {
            if let Err(e) = ws_clone
                .connect_and_run(session_id, guild_id, user_id, token, event_tx, command_rx)
                .await
            {
                error!("Voice WebSocket error: {:?}", e);
            }
        });

        // Spawn event listener
        let this = Arc::clone(self);
        tokio::spawn(async move {
            while let Some(event) = event_rx.recv().await {
                match event {
                    VoiceEvent::Ready(info) => {
                        if let Err(e) = this.handle_ready(info).await {
                            error!("Failed to handle Ready: {:?}", e);
                        }
                    }
                    VoiceEvent::SessionDescription(desc) => {
                        if let Err(e) = this.handle_session_description(desc).await {
                            error!("Failed to handle SessionDescription: {:?}", e);
                        }
                    }
                    VoiceEvent::Closed { code, reason } => {
                        info!(code = code, reason = %reason, "Voice connection closed");
                        this.state
                            .store(VoiceState::Disconnected as u8, Ordering::SeqCst);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Process a Ready event.
    async fn handle_ready(&self, info: crate::ws::VoiceReadyInfo) -> Result<(), VoiceError> {
        *self.ssrc.write() = Some(info.ssrc);
        self.state
            .store(VoiceState::Discovering as u8, Ordering::SeqCst);

        // Connect UDP
        let udp = VoiceUdp::connect(&info.ip, info.port, info.ssrc).await?;

        // Perform IP discovery
        let (external_ip, external_port) = udp.discover_ip().await?;
        info!(ip = %external_ip, port = external_port, "IP discovery complete");

        // Store UDP handler
        *self.udp.write().await = Some(udp);

        // Select encryption mode
        let mode = EncryptionMode::select_preferred(&info.modes)
            .ok_or_else(|| VoiceError::Encryption("No supported encryption mode".to_string()))?;

        self.state
            .store(VoiceState::SelectingProtocol as u8, Ordering::SeqCst);

        // Send SelectProtocol
        let tx = { self.command_tx.read().clone() };
        if let Some(tx) = tx {
            tx.send(crate::ws::VoiceCommand::SelectProtocol {
                protocol: "udp".to_string(),
                ip: external_ip,
                port: external_port,
                mode: mode.to_string(),
            })
            .await
            .map_err(|_| VoiceError::NotConnected)?;
        }

        info!(mode = ?mode, "Selected encryption mode");

        Ok(())
    }

    /// Process a SessionDescription event.
    async fn handle_session_description(
        &self,
        desc: crate::ws::VoiceSessionDescription,
    ) -> Result<(), VoiceError> {
        let mode = EncryptionMode::parse_mode(&desc.mode)
            .ok_or_else(|| VoiceError::Encryption(format!("Unknown mode: {}", desc.mode)))?;

        // Configure encryption on UDP handler
        if let Some(udp) = self.udp.write().await.as_mut() {
            udp.set_encryption(&desc.secret_key, mode)?;
        }

        self.state.store(VoiceState::Ready as u8, Ordering::SeqCst);
        info!("Voice connection ready");

        Ok(())
    }

    /// Send an Opus audio frame.
    ///
    /// # Arguments
    /// * `opus_data` - Opus-encoded audio frame (typically 20ms @ 48kHz).
    pub async fn send_audio(&self, opus_data: &[u8]) -> Result<(), VoiceError> {
        if !self.is_ready() {
            return Err(VoiceError::NotConnected);
        }

        let mut udp_guard = self.udp.write().await;
        let udp = udp_guard.as_mut().ok_or(VoiceError::NotConnected)?;
        udp.send_audio(opus_data).await
    }

    /// Set speaking state.
    pub async fn set_speaking(&self, speaking: bool) -> Result<(), VoiceError> {
        *self.speaking.write() = speaking;

        let flags = if speaking {
            SpeakingFlags::MICROPHONE
        } else {
            SpeakingFlags::NONE
        };

        let tx = { self.command_tx.read().clone() };
        if let Some(tx) = tx {
            let ssrc_opt = *self.ssrc.read();
            if let Some(ssrc) = ssrc_opt {
                tx.send(crate::ws::VoiceCommand::Speaking {
                    speaking: flags,
                    delay: 0,
                    ssrc,
                })
                .await
                .map_err(|_| VoiceError::NotConnected)?;
            }
        }

        debug!(speaking = speaking, "Set speaking state");

        Ok(())
    }

    /// Disconnect from voice.
    pub async fn disconnect(&self) -> Result<(), VoiceError> {
        self.state
            .store(VoiceState::Disconnecting as u8, Ordering::SeqCst);

        // Clear UDP handler
        *self.udp.write().await = None;
        *self.ssrc.write() = None;

        self.state
            .store(VoiceState::Disconnected as u8, Ordering::SeqCst);
        info!("Disconnected from voice");

        Ok(())
    }

    /// Get the assigned SSRC.
    pub fn ssrc(&self) -> Option<u32> {
        *self.ssrc.read()
    }

    /// Get the guild ID.
    pub fn guild_id(&self) -> u64 {
        self.config.guild_id
    }

    /// Get the channel ID.
    pub fn channel_id(&self) -> u64 {
        self.config.channel_id
    }

    /// Get the UDP transport.
    pub fn udp(&self) -> &AsyncRwLock<Option<VoiceUdp>> {
        &self.udp
    }
}

impl Drop for VoiceConnection {
    fn drop(&mut self) {
        if self.state() != VoiceState::Disconnected {
            warn!("VoiceConnection dropped without disconnecting");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_state_conversion() {
        assert_eq!(VoiceState::from(0), VoiceState::Disconnected);
        assert_eq!(VoiceState::from(4), VoiceState::Ready);
        assert_eq!(VoiceState::from(255), VoiceState::Disconnected);
    }

    #[test]
    fn test_voice_config() {
        let config = VoiceConfig {
            guild_id: 123,
            channel_id: 456,
            user_id: 789,
            session_id: "session".to_string(),
            endpoint: "wss://voice.discord.gg".to_string(),
            token: "token".to_string(),
        };

        assert_eq!(config.guild_id, 123);
    }
}
