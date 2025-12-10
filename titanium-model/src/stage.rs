//! Stage instance types for Discord voice stages.
//!
//! Stage channels are special voice channels for hosting events.

use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
