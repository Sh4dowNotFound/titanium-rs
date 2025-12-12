use crate::TitanString;

/// Payload for modifying a guild member.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ModifyMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::Snowflake>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<crate::Snowflake>, // Move to voice channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<TitanString<'a>>, // Timeout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
}

/// Builder for modifying a `GuildMember`.
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct ModifyMemberBuilder<'a> {
    params: ModifyMember<'a>,
}

impl<'a> ModifyMemberBuilder<'a> {
    /// Create a new `ModifyMemberBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set nickname.
    pub fn nick(mut self, nick: impl Into<TitanString<'a>>) -> Self {
        self.params.nick = Some(nick.into());
        self
    }

    /// Set roles (replaces all roles).
    pub fn roles(mut self, roles: Vec<crate::Snowflake>) -> Self {
        self.params.roles = Some(roles);
        self
    }

    /// Mute or unmute.
    pub fn mute(mut self, mute: bool) -> Self {
        self.params.mute = Some(mute);
        self
    }

    /// Deafen or undeafen.
    pub fn deaf(mut self, deaf: bool) -> Self {
        self.params.deaf = Some(deaf);
        self
    }

    /// Move to voice channel (or disconnect if null, but we use strict type here).
    pub fn move_to_channel(mut self, channel_id: impl Into<crate::Snowflake>) -> Self {
        self.params.channel_id = Some(channel_id.into());
        self
    }

    /// Timeout user until timestamp (ISO8601).
    pub fn timeout_until(mut self, timestamp: impl Into<TitanString<'a>>) -> Self {
        self.params.communication_disabled_until = Some(timestamp.into());
        self
    }

    /// Build the `ModifyMember` payload.
    #[must_use]
    pub fn build(self) -> ModifyMember<'a> {
        self.params
    }
}
