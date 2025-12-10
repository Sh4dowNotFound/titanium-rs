//! Thread types for Discord's thread channels.
//!
//! Threads are sub-channels within text or forum channels.

use crate::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// Thread metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMetadata<'a> {
    /// Whether the thread is archived.
    pub archived: bool,

    /// The thread will stop showing in the channel list after `auto_archive_duration` minutes of inactivity.
    pub auto_archive_duration: u32,

    /// Timestamp when the thread's archive status was last changed (ISO8601).
    pub archive_timestamp: TitanString<'a>,

    /// Whether the thread is locked.
    #[serde(default)]
    pub locked: bool,

    /// Whether non-moderators can add other non-moderators to the thread.
    #[serde(default)]
    pub invitable: Option<bool>,
}

/// A member of a thread.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMember<'a> {
    /// The ID of the thread.
    #[serde(default)]
    pub id: Option<Snowflake>,

    /// The ID of the user.
    #[serde(default)]
    pub user_id: Option<Snowflake>,

    /// Time the user last joined the thread.
    pub join_timestamp: TitanString<'a>,

    /// Any user-thread flags.
    pub flags: u64,

    /// Additional member information (if requested).
    #[serde(default)]
    pub member: Option<super::member::GuildMember<'a>>,
}

/// A tag in a forum channel.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForumTag<'a> {
    /// The ID of the tag.
    pub id: Snowflake,

    /// The name of the tag (0-20 characters).
    pub name: TitanString<'a>,

    /// Whether this tag can only be added/removed by moderators.
    pub moderated: bool,

    /// The ID of a guild's custom emoji.
    pub emoji_id: Option<Snowflake>,

    /// The unicode character of the emoji.
    #[serde(default)]
    pub emoji_name: Option<TitanString<'a>>,
}

/// Default reaction for a forum channel.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultReaction<'a> {
    /// The ID of a guild's custom emoji.
    pub emoji_id: Option<Snowflake>,

    /// The unicode character of the emoji.
    #[serde(default)]
    pub emoji_name: Option<TitanString<'a>>,
}

/// Event data for THREAD_DELETE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadDeleteEvent {
    /// The ID of the thread.
    pub id: Snowflake,

    /// The ID of the guild.
    pub guild_id: Snowflake,

    /// The ID of the parent channel.
    pub parent_id: Option<Snowflake>,

    /// The type of the thread.
    #[serde(rename = "type")]
    pub channel_type: u8,
}

/// Event data for THREAD_LIST_SYNC.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadListSyncEvent<'a> {
    /// The ID of the guild.
    pub guild_id: Snowflake,

    /// The parent channel IDs whose threads are being synced.
    #[serde(default)]
    pub channel_ids: Option<Vec<Snowflake>>,

    /// All active threads in the given channels.
    pub threads: Vec<super::Channel<'a>>,

    /// All thread members for the current user in each of the threads.
    pub members: Vec<ThreadMember<'a>>,
}

/// Event data for THREAD_MEMBER_UPDATE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMemberUpdateEvent<'a> {
    #[serde(flatten)]
    pub member: ThreadMember<'a>,
    pub guild_id: Snowflake,
}

/// Event data for THREAD_MEMBERS_UPDATE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMembersUpdateEvent<'a> {
    /// The ID of the thread.
    pub id: Snowflake,

    /// The ID of the guild.
    pub guild_id: Snowflake,

    /// The approximate number of members in the thread.
    pub member_count: u32,

    /// The users who were added to the thread.
    #[serde(default)]
    pub added_members: Option<Vec<ThreadMember<'a>>>,

    /// The ID of the users who were removed from the thread.
    #[serde(default)]
    pub removed_member_ids: Option<Vec<Snowflake>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_metadata() {
        let json = r#"{
            "archived": false,
            "auto_archive_duration": 1440,
            "archive_timestamp": "2021-01-01T00:00:00.000Z",
            "locked": false
        }"#;

        let metadata: ThreadMetadata = crate::json::from_str(json).unwrap();
        assert!(!metadata.archived);
        assert_eq!(metadata.auto_archive_duration, 1440);
    }
}
