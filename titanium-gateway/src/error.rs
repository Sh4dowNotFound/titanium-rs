//! Gateway error types using thiserror.
//!
//! All errors in titanium-gateway are represented by the [`GatewayError`] enum.
//! No `.unwrap()` calls are used throughout the codebase.

use thiserror::Error;

/// Errors that can occur during Gateway operations.
#[derive(Debug, Error)]
pub enum GatewayError {
    /// WebSocket connection or protocol error.
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// Failed to parse JSON payload.
    #[error("JSON decode error: {0}")]
    JsonDecode(String),

    /// Session was invalidated by Discord.
    /// The boolean indicates if the session is resumable.
    #[error("Session invalidated, resumable: {resumable}")]
    InvalidSession {
        /// Whether the session can be resumed.
        resumable: bool,
    },

    /// Connection was closed by Discord.
    #[error("Connection closed: code={code}, reason={reason}")]
    Closed {
        /// WebSocket close code.
        code: u16,
        /// Close reason.
        reason: String,
    },

    /// Heartbeat acknowledgment was not received in time.
    #[error("Heartbeat acknowledgment timeout")]
    HeartbeatTimeout,

    /// Authentication failed (invalid token, missing intents, etc.).
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Unknown or unhandled event type.
    #[error("Unknown event type: {0}")]
    UnknownEvent(String),

    /// Failed to send message through channel.
    #[error("Channel send error: {0}")]
    ChannelSend(String),

    /// URL parsing error.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Shard is not connected.
    #[error("Shard not connected")]
    NotConnected,

    /// Rate limited.
    #[error("Rate limited, retry after {retry_after_ms}ms")]
    RateLimited {
        /// Milliseconds until rate limit expires.
        retry_after_ms: u64,
    },
}

impl From<serde_json::Error> for GatewayError {
    fn from(err: serde_json::Error) -> Self {
        GatewayError::JsonDecode(err.to_string())
    }
}

#[cfg(feature = "simd")]
impl From<simd_json::Error> for GatewayError {
    fn from(err: simd_json::Error) -> Self {
        GatewayError::JsonDecode(err.to_string())
    }
}

impl<T> From<flume::SendError<T>> for GatewayError {
    fn from(err: flume::SendError<T>) -> Self {
        GatewayError::ChannelSend(err.to_string())
    }
}

/// Discord Gateway close codes.
///
/// See: <https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CloseCode {
    /// Unknown error occurred.
    UnknownError = 4000,
    /// Invalid opcode sent.
    UnknownOpcode = 4001,
    /// Invalid payload (decode error).
    DecodeError = 4002,
    /// Payload sent before identifying.
    NotAuthenticated = 4003,
    /// Invalid token.
    AuthenticationFailed = 4004,
    /// Already authenticated.
    AlreadyAuthenticated = 4005,
    /// Invalid sequence number for resume.
    InvalidSeq = 4007,
    /// Rate limited.
    RateLimited = 4008,
    /// Session timed out.
    SessionTimedOut = 4009,
    /// Invalid shard configuration.
    InvalidShard = 4010,
    /// Too many guilds (sharding required).
    ShardingRequired = 4011,
    /// Invalid API version.
    InvalidApiVersion = 4012,
    /// Invalid intents.
    InvalidIntents = 4013,
    /// Disallowed intents (privileged intent not enabled).
    DisallowedIntents = 4014,
}

impl CloseCode {
    /// Returns whether this close code is recoverable by resuming.
    #[allow(dead_code)]
    pub const fn is_resumable(self) -> bool {
        matches!(
            self,
            CloseCode::UnknownError | CloseCode::InvalidSeq | CloseCode::SessionTimedOut
        )
    }

    /// Returns whether reconnection is possible after this close code.
    pub const fn can_reconnect(self) -> bool {
        !matches!(
            self,
            CloseCode::AuthenticationFailed
                | CloseCode::InvalidShard
                | CloseCode::ShardingRequired
                | CloseCode::InvalidApiVersion
                | CloseCode::InvalidIntents
                | CloseCode::DisallowedIntents
        )
    }

    /// Try to convert a u16 close code to this enum.
    pub fn from_code(code: u16) -> Option<Self> {
        match code {
            4000 => Some(CloseCode::UnknownError),
            4001 => Some(CloseCode::UnknownOpcode),
            4002 => Some(CloseCode::DecodeError),
            4003 => Some(CloseCode::NotAuthenticated),
            4004 => Some(CloseCode::AuthenticationFailed),
            4005 => Some(CloseCode::AlreadyAuthenticated),
            4007 => Some(CloseCode::InvalidSeq),
            4008 => Some(CloseCode::RateLimited),
            4009 => Some(CloseCode::SessionTimedOut),
            4010 => Some(CloseCode::InvalidShard),
            4011 => Some(CloseCode::ShardingRequired),
            4012 => Some(CloseCode::InvalidApiVersion),
            4013 => Some(CloseCode::InvalidIntents),
            4014 => Some(CloseCode::DisallowedIntents),
            _ => None,
        }
    }
}
