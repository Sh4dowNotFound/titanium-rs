use crate::snowflake::Snowflake;
use crate::thread::{DefaultReaction, ForumTag, ThreadMember, ThreadMetadata};
use crate::user::User;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// Discord Channel representation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel<'a> {
    /// Channel ID.
    pub id: Snowflake,
    /// Channel type.
    #[serde(rename = "type")]
    pub channel_type: u8,
    /// Guild ID (if in a guild).
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// Sorting position.
    #[serde(default)]
    pub position: Option<i32>,
    /// Permission overwrites.
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    /// Channel name (1-100 characters).
    #[serde(default)]
    pub name: Option<TitanString<'a>>,
    /// Channel topic (0-4096 characters for forum, 0-1024 for others).
    #[serde(default)]
    pub topic: Option<TitanString<'a>>,
    /// Whether NSFW.
    #[serde(default)]
    pub nsfw: bool,
    /// ID of the last message sent.
    #[serde(default)]
    pub last_message_id: Option<Snowflake>,
    /// Bitrate (for voice).
    #[serde(default)]
    pub bitrate: Option<u32>,
    /// User limit (for voice).
    #[serde(default)]
    pub user_limit: Option<u32>,
    /// Rate limit per user in seconds.
    #[serde(default)]
    pub rate_limit_per_user: Option<u32>,
    /// Recipients of the DM.
    #[serde(default)]
    pub recipients: Vec<User<'a>>,
    /// Icon hash (for group DM).
    #[serde(default)]
    pub icon: Option<TitanString<'a>>,
    /// ID of the DM creator.
    #[serde(default)]
    pub owner_id: Option<Snowflake>,
    /// Application ID of the group DM creator if bot-created.
    #[serde(default)]
    pub application_id: Option<Snowflake>,
    /// Whether the channel is managed by an application.
    #[serde(default)]
    pub managed: bool,
    /// Parent category ID.
    #[serde(default)]
    pub parent_id: Option<Snowflake>,
    /// Last pin timestamp.
    #[serde(default)]
    pub last_pin_timestamp: Option<TitanString<'a>>,
    /// Voice region ID.
    #[serde(default)]
    pub rtc_region: Option<TitanString<'a>>,
    /// Video quality mode.
    #[serde(default)]
    pub video_quality_mode: Option<u8>,
    /// Approximate message count (threads).
    #[serde(default)]
    pub message_count: Option<u32>,
    /// Approximate member count (threads).
    #[serde(default)]
    pub member_count: Option<u32>,
    /// Thread metadata.
    #[serde(default)]
    pub thread_metadata: Option<ThreadMetadata<'a>>,
    /// Thread member object for the current user.
    #[serde(default)]
    pub member: Option<ThreadMember<'a>>,
    /// Default auto-archive duration.
    #[serde(default)]
    pub default_auto_archive_duration: Option<u32>,
    /// Computed permissions for the user.
    #[serde(default)]
    pub permissions: Option<crate::permissions::Permissions>,
    /// Channel flags as a bitfield.
    #[serde(default)]
    pub flags: Option<u64>,
    /// Total messages ever sent (threads).
    #[serde(default)]
    pub total_message_sent: Option<u32>,
    /// Tags available in a forum channel.
    #[serde(default)]
    pub available_tags: Vec<ForumTag<'a>>,
    /// IDs of tags applied to a forum thread.
    #[serde(default)]
    pub applied_tags: Vec<Snowflake>,
    /// Default reaction emoji.
    #[serde(default)]
    pub default_reaction_emoji: Option<DefaultReaction<'a>>,
    /// Default slowmode for threads.
    #[serde(default)]
    pub default_thread_rate_limit_per_user: Option<u32>,
    /// Default sort order for forum posts.
    #[serde(default)]
    pub default_sort_order: Option<u8>,
    /// Default forum layout.
    #[serde(default)]
    pub default_forum_layout: Option<u8>,
}

impl crate::Mention for Channel<'_> {
    fn mention(&self) -> String {
        format!("<#{}>", self.id.0)
    }
}

/// Permission overwrite for a channel.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PermissionOverwrite {
    /// Role or user ID.
    pub id: Snowflake,
    /// Type (0 = role, 1 = member).
    #[serde(rename = "type")]
    pub overwrite_type: u8,
    /// Permission bit set for allowed permissions.
    pub allow: crate::permissions::Permissions,
    /// Permission bit set for denied permissions.
    pub deny: crate::permissions::Permissions,
}

impl PermissionOverwrite {
    /// Create an overwrite for a role.
    #[inline]
    pub fn role(role_id: impl Into<Snowflake>) -> Self {
        Self {
            id: role_id.into(),
            overwrite_type: 0,
            allow: crate::permissions::Permissions::empty(),
            deny: crate::permissions::Permissions::empty(),
        }
    }

    /// Create an overwrite for a user/member.
    #[inline]
    pub fn member(user_id: impl Into<Snowflake>) -> Self {
        Self {
            id: user_id.into(),
            overwrite_type: 1,
            allow: crate::permissions::Permissions::empty(),
            deny: crate::permissions::Permissions::empty(),
        }
    }

    /// Set allowed permissions.
    #[inline]
    #[must_use]
    pub fn allow(mut self, permissions: crate::permissions::Permissions) -> Self {
        self.allow = permissions;
        self
    }

    /// Set denied permissions.
    #[inline]
    #[must_use]
    pub fn deny(mut self, permissions: crate::permissions::Permissions) -> Self {
        self.deny = permissions;
        self
    }
}

/// A channel mention.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelMention {
    /// Channel ID.
    pub id: Snowflake,
    /// Guild ID.
    pub guild_id: Snowflake,
    /// Channel type.
    #[serde(rename = "type")]
    pub channel_type: u8,
    /// Channel name.
    pub name: String,
}

/// Channel pins update event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelPinsUpdateEvent {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub last_pin_timestamp: Option<String>,
}
