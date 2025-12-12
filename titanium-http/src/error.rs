//! HTTP error types.

use thiserror::Error;

/// Errors that can occur during HTTP operations.
#[derive(Debug, Error)]
pub enum HttpError {
    /// Request failed.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Rate limited by Discord.
    #[error("Rate limited, retry after {retry_after_ms}ms")]
    RateLimited {
        /// Milliseconds until rate limit expires.
        retry_after_ms: u64,
        /// Whether this is a global rate limit.
        global: bool,
    },

    /// Discord API returned an error.
    #[error("Discord API error {code}: {message}")]
    Discord {
        /// Error code.
        code: u32,
        /// Error message.
        message: String,
    },

    /// Unauthorized (invalid token).
    #[error("Unauthorized: Invalid token")]
    Unauthorized,

    /// Forbidden (missing permissions).
    #[error("Forbidden: Missing permissions")]
    Forbidden,

    /// Resource not found.
    #[error("Not found")]
    NotFound,

    /// Server error.
    #[error("Discord server error: {0}")]
    ServerError(u16),

    /// Invalid header value.
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// Client internal error.
    #[error("Client error: {0}")]
    ClientError(String),
}

/// Discord API error response.
#[derive(Debug, serde::Deserialize)]
pub struct DiscordError {
    pub code: u32,
    pub message: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub errors: Option<serde_json::Value>,
}
