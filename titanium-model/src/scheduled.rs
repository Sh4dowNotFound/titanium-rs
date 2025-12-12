//! Scheduled event types for Discord guild events.
//!
//! Scheduled events allow guilds to plan activities.

use crate::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A scheduled event in a guild.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduledEvent<'a> {
    /// The ID of the scheduled event.
    pub id: Snowflake,

    /// The guild ID which the scheduled event belongs to.
    pub guild_id: Snowflake,

    /// The channel ID in which the event will be hosted (if applicable).
    #[serde(default)]
    pub channel_id: Option<Snowflake>,

    /// The ID of the user that created the scheduled event.
    #[serde(default)]
    pub creator_id: Option<Snowflake>,

    /// The name of the scheduled event (1-100 characters).
    pub name: TitanString<'a>,

    /// The description of the scheduled event (1-1000 characters).
    #[serde(default)]
    pub description: Option<TitanString<'a>>,

    /// The time the scheduled event will start (ISO8601 timestamp).
    pub scheduled_start_time: TitanString<'a>,

    /// The time the scheduled event will end (ISO8601 timestamp).
    #[serde(default)]
    pub scheduled_end_time: Option<TitanString<'a>>,

    /// The privacy level of the scheduled event.
    pub privacy_level: ScheduledEventPrivacyLevel,

    /// The status of the scheduled event.
    pub status: ScheduledEventStatus,

    /// The type of the scheduled event.
    pub entity_type: ScheduledEventEntityType,

    /// The ID of an entity associated with a guild scheduled event.
    #[serde(default)]
    pub entity_id: Option<Snowflake>,

    /// Additional metadata for the guild scheduled event.
    #[serde(default)]
    pub entity_metadata: Option<ScheduledEventEntityMetadata<'a>>,

    /// The user that created the scheduled event.
    #[serde(default)]
    pub creator: Option<super::User<'a>>,

    /// The number of users subscribed to the scheduled event.
    #[serde(default)]
    pub user_count: Option<u32>,

    /// The cover image hash of the scheduled event.
    #[serde(default)]
    pub image: Option<TitanString<'a>>,
}

/// Privacy level of a scheduled event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
pub enum ScheduledEventPrivacyLevel {
    /// The scheduled event is only accessible to guild members.
    #[default]
    GuildOnly = 2,
}

/// Status of a scheduled event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ScheduledEventStatus {
    /// The event is scheduled.
    Scheduled = 1,
    /// The event is active/started.
    Active = 2,
    /// The event has completed.
    Completed = 3,
    /// The event was cancelled.
    Cancelled = 4,
}

/// Entity type of a scheduled event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
pub enum ScheduledEventEntityType {
    /// A stage instance.
    StageInstance = 1,
    /// A voice channel.
    #[default]
    Voice = 2,
    /// An external location.
    External = 3,
}

/// Metadata for scheduled event entities.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledEventEntityMetadata<'a> {
    /// Location of the event (required for External events).
    #[serde(default)]
    pub location: Option<TitanString<'a>>,
}

/// Event data for scheduled event user add/remove.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduledEventUserEvent {
    /// The ID of the scheduled event.
    pub guild_scheduled_event_id: Snowflake,

    /// The ID of the user.
    pub user_id: Snowflake,

    /// The ID of the guild.
    pub guild_id: Snowflake,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduled_event() {
        let json = r#"{
            "id": "123",
            "guild_id": "456",
            "name": "Community Event",
            "scheduled_start_time": "2024-01-01T00:00:00.000Z",
            "privacy_level": 2,
            "status": 1,
            "entity_type": 2
        }"#;

        let event: ScheduledEvent = crate::json::from_str(json).unwrap();
        assert_eq!(event.name, TitanString::Borrowed("Community Event"));
        assert_eq!(event.status, ScheduledEventStatus::Scheduled);
    }
}
