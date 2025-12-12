//! Discord API route types and responses.

use serde::Deserialize;
use titanium_model::Snowflake;

/// Response from GET /gateway/bot.
#[derive(Debug, Clone, Deserialize)]
pub struct GatewayBotResponse {
    /// Gateway WebSocket URL.
    pub url: String,

    /// Recommended number of shards.
    pub shards: u16,

    /// Session start limit information.
    pub session_start_limit: SessionStartLimit,
}

/// Session start limit from /gateway/bot.
#[derive(Debug, Clone, Deserialize)]
pub struct SessionStartLimit {
    /// Total number of session starts allowed.
    pub total: u32,

    /// Remaining session starts.
    pub remaining: u32,

    /// Milliseconds until the limit resets.
    pub reset_after: u64,

    /// Maximum number of concurrent identify operations.
    pub max_concurrency: u32,
}

/// Response from GET /users/@me.
#[derive(Debug, Clone, Deserialize)]
pub struct CurrentUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub global_name: Option<String>,
    #[serde(default)]
    pub system: bool,
    #[serde(default)]
    pub mfa_enabled: Option<bool>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub accent_color: Option<u32>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub flags: Option<u64>,
    #[serde(default)]
    pub premium_type: Option<u8>,
    #[serde(default)]
    pub public_flags: Option<u64>,
}

/// Response from GET /applications/@me.
#[derive(Debug, Clone, Deserialize)]
pub struct CurrentApplication {
    pub id: Snowflake,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub bot_public: bool,
    #[serde(default)]
    pub bot_require_code_grant: bool,
    #[serde(default)]
    pub flags: Option<u64>,
}
