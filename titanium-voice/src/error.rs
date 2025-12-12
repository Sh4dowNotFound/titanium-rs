//! Voice Gateway error types.

use thiserror::Error;

/// Errors that can occur during voice operations.
#[derive(Debug, Error)]
pub enum VoiceError {
    /// WebSocket error.
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Connection was closed.
    #[error("Connection closed: code={code}, reason={reason}")]
    Closed {
        /// Close code.
        code: u16,
        /// Close reason.
        reason: String,
    },

    /// Timeout waiting for response.
    #[error("Timeout waiting for {0}")]
    Timeout(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// UDP socket error.
    #[error("UDP error: {0}")]
    Udp(String),

    /// IP discovery failed.
    #[error("IP discovery failed: {0}")]
    IpDiscovery(String),

    /// Encryption error.
    #[error("Encryption error: {0}")]
    Encryption(String),

    /// Decoder error.
    #[error("Decoder error: {0}")]
    Decoder(String),

    /// Not connected.
    #[error("Not connected to voice server")]
    NotConnected,

    /// Invalid state.
    #[error("Invalid state: expected {expected}, got {actual}")]
    InvalidState {
        /// Expected state.
        expected: String,
        /// Actual state.
        actual: String,
    },

    /// URL parse error.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Channel send error.
    #[error("Channel error: {0}")]
    ChannelError(String),
}

impl<T> From<flume::SendError<T>> for VoiceError {
    fn from(err: flume::SendError<T>) -> Self {
        VoiceError::ChannelError(err.to_string())
    }
}

/// Discord Voice close codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum VoiceCloseCode {
    /// Unknown error.
    UnknownError = 4000,
    /// Unknown opcode.
    UnknownOpcode = 4001,
    /// Failed to decode payload.
    FailedToDecodePayload = 4002,
    /// Not authenticated.
    NotAuthenticated = 4003,
    /// Authentication failed.
    AuthenticationFailed = 4004,
    /// Already authenticated.
    AlreadyAuthenticated = 4005,
    /// Session is no longer valid.
    SessionNoLongerValid = 4006,
    /// Session timed out.
    SessionTimeout = 4009,
    /// Server not found.
    VoiceServerNotFound = 4011,
    /// Unknown protocol.
    UnknownProtocol = 4012,
    /// Disconnected.
    Disconnected = 4014,
    /// Voice server crashed.
    VoiceServerCrashed = 4015,
    /// Unknown encryption mode.
    UnknownEncryptionMode = 4016,
}

impl VoiceCloseCode {
    /// Whether reconnection is possible.
    pub fn can_reconnect(self) -> bool {
        matches!(
            self,
            VoiceCloseCode::UnknownError
                | VoiceCloseCode::SessionTimeout
                | VoiceCloseCode::VoiceServerCrashed
        )
    }

    /// Try to parse a close code.
    pub fn from_code(code: u16) -> Option<Self> {
        match code {
            4000 => Some(VoiceCloseCode::UnknownError),
            4001 => Some(VoiceCloseCode::UnknownOpcode),
            4002 => Some(VoiceCloseCode::FailedToDecodePayload),
            4003 => Some(VoiceCloseCode::NotAuthenticated),
            4004 => Some(VoiceCloseCode::AuthenticationFailed),
            4005 => Some(VoiceCloseCode::AlreadyAuthenticated),
            4006 => Some(VoiceCloseCode::SessionNoLongerValid),
            4009 => Some(VoiceCloseCode::SessionTimeout),
            4011 => Some(VoiceCloseCode::VoiceServerNotFound),
            4012 => Some(VoiceCloseCode::UnknownProtocol),
            4014 => Some(VoiceCloseCode::Disconnected),
            4015 => Some(VoiceCloseCode::VoiceServerCrashed),
            4016 => Some(VoiceCloseCode::UnknownEncryptionMode),
            _ => None,
        }
    }
}
