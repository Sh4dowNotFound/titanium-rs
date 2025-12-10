//! Voice Gateway payloads.

use crate::opcode::VoiceOpCode;
use serde::{Deserialize, Serialize};
use std::fmt;
use titanium_model::Snowflake;

/// A Voice Gateway payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicePayload<D> {
    /// Opcode.
    pub op: VoiceOpCode,
    /// Payload data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<D>,
}

impl<D: Serialize> VoicePayload<D> {
    /// Create a new payload.
    pub fn new(op: VoiceOpCode, data: D) -> Self {
        Self { op, d: Some(data) }
    }
}

/// Raw payload for initial parsing.
#[derive(Debug, Clone, Deserialize)]
pub struct RawVoicePayload {
    /// Opcode.
    pub op: VoiceOpCode,
    /// Raw payload data.
    pub d: Option<serde_json::Value>,
}

/// Hello payload (op 8).
#[derive(Debug, Clone, Deserialize)]
pub struct HelloPayload {
    /// Heartbeat interval in milliseconds.
    pub heartbeat_interval: f64,
}

/// Identify payload (op 0).
#[derive(Debug, Clone, Serialize)]
pub struct IdentifyPayload {
    /// Server ID (guild ID).
    pub server_id: String,
    /// User ID.
    pub user_id: String,
    /// Session ID from VOICE_STATE_UPDATE.
    pub session_id: String,
    /// Token from VOICE_SERVER_UPDATE.
    pub token: String,
}

/// Ready payload (op 2).
#[derive(Debug, Clone, Deserialize)]
pub struct ReadyPayload {
    /// SSRC for this connection.
    pub ssrc: u32,
    /// IP address of the voice server.
    pub ip: String,
    /// UDP port of the voice server.
    pub port: u16,
    /// Available encryption modes.
    pub modes: Vec<String>,
}

/// Select Protocol payload (op 1).
#[derive(Debug, Clone, Serialize)]
pub struct SelectProtocolPayload {
    /// Protocol to use (always "udp").
    pub protocol: String,
    /// Protocol data.
    pub data: SelectProtocolData,
}

/// Data for Select Protocol.
#[derive(Debug, Clone, Serialize)]
pub struct SelectProtocolData {
    /// Our external IP address.
    pub address: String,
    /// Our external port.
    pub port: u16,
    /// Encryption mode.
    pub mode: String,
}

impl SelectProtocolPayload {
    /// Create a new Select Protocol payload.
    pub fn new(address: String, port: u16, mode: EncryptionMode) -> Self {
        Self {
            protocol: "udp".to_string(),
            data: SelectProtocolData {
                address,
                port,
                mode: mode.to_string(),
            },
        }
    }
}

/// Session Description payload (op 4).
#[derive(Debug, Clone, Deserialize)]
pub struct SessionDescriptionPayload {
    /// Encryption mode.
    pub mode: String,
    /// Secret key for encryption (32 bytes).
    pub secret_key: Vec<u8>,
}

/// Speaking payload (op 5).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakingPayload {
    /// Speaking flags.
    pub speaking: u8,
    /// Delay (always 0).
    pub delay: u32,
    /// SSRC.
    pub ssrc: u32,
}

/// Speaking flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeakingFlags(u8);

impl SpeakingFlags {
    /// Not speaking.
    pub const NONE: SpeakingFlags = SpeakingFlags(0);
    /// Normal voice speaking.
    pub const MICROPHONE: SpeakingFlags = SpeakingFlags(1 << 0);
    /// Soundshare/application audio.
    pub const SOUNDSHARE: SpeakingFlags = SpeakingFlags(1 << 1);
    /// Priority speaker.
    pub const PRIORITY: SpeakingFlags = SpeakingFlags(1 << 2);

    /// Get the raw value.
    pub fn bits(self) -> u8 {
        self.0
    }

    /// Check if a flag is set.
    pub fn contains(self, other: SpeakingFlags) -> bool {
        self.0 & other.0 == other.0
    }
}

impl std::ops::BitOr for SpeakingFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        SpeakingFlags(self.0 | rhs.0)
    }
}

/// Resume payload (op 7).
#[derive(Debug, Clone, Serialize)]
pub struct ResumePayload {
    /// Server ID.
    pub server_id: String,
    /// Session ID.
    pub session_id: String,
    /// Token.
    pub token: String,
}

/// Heartbeat payload (op 3).
#[derive(Debug, Clone, Serialize)]
pub struct HeartbeatPayload {
    /// Nonce (usually current timestamp).
    #[serde(rename = "t")]
    pub nonce: u64,
    /// Sequence number (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_ack: Option<u64>,
}

/// Client Connect payload (op 12).
#[derive(Debug, Clone, Deserialize)]
pub struct ClientConnectPayload {
    /// User ID of connected client.
    pub user_id: Snowflake,
    /// Audio SSRC.
    #[serde(default)]
    pub audio_ssrc: Option<u32>,
    /// Video SSRC.
    #[serde(default)]
    pub video_ssrc: Option<u32>,
}

/// Client Disconnect payload (op 13).
#[derive(Debug, Clone, Deserialize)]
pub struct ClientDisconnectPayload {
    /// User ID of disconnected client.
    pub user_id: Snowflake,
}

/// Supported encryption modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncryptionMode {
    /// XSalsa20 with Poly1305 (lite variant - smaller nonce in packet).
    #[default]
    XSalsa20Poly1305Lite,
    /// XSalsa20 with Poly1305 (suffix variant).
    XSalsa20Poly1305Suffix,
    /// XSalsa20 with Poly1305 (original).
    XSalsa20Poly1305,
    /// AEAD AES256 GCM (for Dave E2EE when available).
    AeadAes256Gcm,
    /// AEAD XChaCha20 Poly1305 RTPSIZE.
    AeadXChaCha20Poly1305Rtpsize,
}

impl EncryptionMode {
    /// Parse from string.
    pub fn parse_mode(s: &str) -> Option<Self> {
        match s {
            "xsalsa20_poly1305_lite" => Some(EncryptionMode::XSalsa20Poly1305Lite),
            "xsalsa20_poly1305_suffix" => Some(EncryptionMode::XSalsa20Poly1305Suffix),
            "xsalsa20_poly1305" => Some(EncryptionMode::XSalsa20Poly1305),
            "aead_aes256_gcm" => Some(EncryptionMode::AeadAes256Gcm),
            "aead_xchacha20_poly1305_rtpsize" => Some(EncryptionMode::AeadXChaCha20Poly1305Rtpsize),
            _ => None,
        }
    }

    /// Get preferred mode from available modes.
    pub fn select_preferred(modes: &[String]) -> Option<Self> {
        // Preference order: lite > suffix > original
        for mode_str in [
            "xsalsa20_poly1305_lite",
            "xsalsa20_poly1305_suffix",
            "xsalsa20_poly1305",
        ] {
            if modes.iter().any(|m| m == mode_str) {
                return Self::parse_mode(mode_str);
            }
        }
        None
    }
}

impl fmt::Display for EncryptionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::XSalsa20Poly1305 => write!(f, "xsalsa20_poly1305"),
            Self::XSalsa20Poly1305Suffix => write!(f, "xsalsa20_poly1305_suffix"),
            Self::XSalsa20Poly1305Lite => write!(f, "xsalsa20_poly1305_lite"),
            Self::AeadAes256Gcm => write!(f, "aead_aes256_gcm"),
            Self::AeadXChaCha20Poly1305Rtpsize => write!(f, "aead_xchacha20_poly1305_rtpsize"),
        }
    }
}


