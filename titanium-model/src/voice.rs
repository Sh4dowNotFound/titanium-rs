//! Voice-related types for Discord.
//!
//! This module contains voice state, voice server, stage instance,
//! and voice channel effect types.

use crate::member::GuildMember;
use crate::reaction::ReactionEmoji;
use crate::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// ============================================================================
// Voice State (merged from voice_state.rs)
// ============================================================================

/// Partial voice state from `GUILD_CREATE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartialVoiceState<'a> {
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,

    pub session_id: TitanString<'a>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    #[serde(default)]
    pub self_video: bool,
    pub suppress: bool,
    #[serde(default)]
    pub request_to_speak_timestamp: Option<TitanString<'a>>,
}

// ============================================================================
// Voice Events
// ============================================================================

/// Voice state update event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceStateUpdateEvent<'a> {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    #[serde(default)]
    pub member: Option<GuildMember<'a>>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    #[serde(default)]
    pub self_stream: bool,
    pub self_video: bool,
    pub suppress: bool,
    #[serde(default)]
    pub request_to_speak_timestamp: Option<String>,
}

/// Voice server update event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceServerUpdateEvent {
    pub token: String,
    pub guild_id: Snowflake,
    #[serde(default)]
    pub endpoint: Option<String>,
}

/// Voice channel effect send event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceChannelEffectSendEvent<'a> {
    pub channel_id: Snowflake,
    pub guild_id: Snowflake,
    pub user_id: Snowflake,
    #[serde(default)]
    pub emoji: Option<ReactionEmoji<'a>>,
    #[serde(default)]
    pub animation_type: Option<u8>,
    #[serde(default)]
    pub animation_id: Option<u64>,
    #[serde(default)]
    pub sound_id: Option<Snowflake>,
    #[serde(default)]
    pub sound_volume: Option<f64>,
}

// ============================================================================
// Stage Instance (merged from stage.rs)
// ============================================================================

/// A stage instance holds information about a live stage.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StageInstance {
    /// The ID of this stage instance.
    pub id: Snowflake,

    /// The guild ID of the associated stage channel.
    pub guild_id: Snowflake,

    /// The ID of the associated stage channel.
    pub channel_id: Snowflake,

    /// The topic of the stage instance (1-120 characters).
    pub topic: String,

    /// The privacy level of the stage instance.
    pub privacy_level: StagePrivacyLevel,

    /// Whether or not stage discovery is disabled (deprecated).
    #[serde(default)]
    pub discoverable_disabled: bool,

    /// The ID of the scheduled event for this stage instance.
    #[serde(default)]
    pub guild_scheduled_event_id: Option<Snowflake>,
}

/// Privacy level of a stage instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
pub enum StagePrivacyLevel {
    /// The stage instance is visible publicly (deprecated).
    Public = 1,
    /// The stage instance is visible to only guild members.
    #[default]
    GuildOnly = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_instance() {
        let json = r#"{
            "id": "123",
            "guild_id": "456",
            "channel_id": "789",
            "topic": "Welcome to the stage!",
            "privacy_level": 2
        }"#;

        let stage: StageInstance = crate::json::from_str(json).unwrap();
        assert_eq!(stage.topic, "Welcome to the stage!");
        assert_eq!(stage.privacy_level, StagePrivacyLevel::GuildOnly);
    }
}
