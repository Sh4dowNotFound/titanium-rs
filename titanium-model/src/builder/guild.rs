use crate::TitanString;

/// Payload for modifying a guild.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ModifyGuild<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<TitanString<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<crate::Snowflake>,
}

/// Builder for modifying a Guild.
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct ModifyGuildBuilder<'a> {
    params: ModifyGuild<'a>,
}

impl<'a> ModifyGuildBuilder<'a> {
    /// Create a new `ModifyGuildBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name.
    pub fn name(mut self, name: impl Into<TitanString<'a>>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    /// Set region (deprecated).
    pub fn region(mut self, region: impl Into<TitanString<'a>>) -> Self {
        self.params.region = Some(region.into());
        self
    }

    /// Set verification level.
    pub fn verification_level(mut self, level: u8) -> Self {
        self.params.verification_level = Some(level);
        self
    }

    /// Set default message notifications.
    pub fn default_message_notifications(mut self, level: u8) -> Self {
        self.params.default_message_notifications = Some(level);
        self
    }

    /// Set explicit content filter.
    pub fn explicit_content_filter(mut self, level: u8) -> Self {
        self.params.explicit_content_filter = Some(level);
        self
    }

    /// Set AFK channel ID.
    pub fn afk_channel_id(mut self, id: impl Into<crate::Snowflake>) -> Self {
        self.params.afk_channel_id = Some(id.into());
        self
    }

    /// Set AFK timeout.
    pub fn afk_timeout(mut self, timeout: u32) -> Self {
        self.params.afk_timeout = Some(timeout);
        self
    }

    /// Set icon (base64).
    pub fn icon(mut self, icon: impl Into<TitanString<'a>>) -> Self {
        self.params.icon = Some(icon.into());
        self
    }

    /// Set system channel ID.
    pub fn system_channel_id(mut self, id: impl Into<crate::Snowflake>) -> Self {
        self.params.system_channel_id = Some(id.into());
        self
    }

    /// Build the `ModifyGuild` payload.
    #[must_use]
    pub fn build(self) -> ModifyGuild<'a> {
        self.params
    }
}

/// Payload for creating a Guild.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateGuild {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<u8>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<crate::json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<crate::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<u64>,
}

/// Builder for creating a Guild.
#[derive(Debug, Clone)]
pub struct CreateGuildBuilder {
    params: CreateGuild,
}

impl CreateGuildBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            params: CreateGuild {
                name: name.into(),
                ..Default::default()
            },
        }
    }

    #[inline]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.params.icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn verification_level(mut self, level: u8) -> Self {
        self.params.verification_level = Some(level);
        self
    }

    #[must_use]
    pub fn build(self) -> CreateGuild {
        self.params
    }
}
