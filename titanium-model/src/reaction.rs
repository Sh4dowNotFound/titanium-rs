//! Reaction types for Discord message reactions.
//!
//! Reactions are emoji added to messages by users.

use crate::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// Event data for `MESSAGE_REACTION_ADD`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageReactionAddEvent<'a> {
    /// ID of the user.
    pub user_id: Snowflake,

    /// ID of the channel.
    pub channel_id: Snowflake,

    /// ID of the message.
    pub message_id: Snowflake,

    /// ID of the guild.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// Member who reacted if in a guild.
    #[serde(default)]
    pub member: Option<super::member::GuildMember<'a>>,

    /// The emoji used to react.
    pub emoji: ReactionEmoji<'a>,

    /// ID of the user who authored the message.
    #[serde(default)]
    pub message_author_id: Option<Snowflake>,

    /// Whether this is a super-reaction.
    #[serde(default)]
    pub burst: bool,

    /// Colors used for super-reaction animation (hex format).
    #[serde(default)]
    pub burst_colors: Vec<String>,

    /// The type of reaction.
    #[serde(default, rename = "type")]
    pub reaction_type: u8,
}

/// Event data for `MESSAGE_REACTION_REMOVE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageReactionRemoveEvent<'a> {
    /// ID of the user.
    pub user_id: Snowflake,

    /// ID of the channel.
    pub channel_id: Snowflake,

    /// ID of the message.
    pub message_id: Snowflake,

    /// ID of the guild.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// The emoji used to react.
    pub emoji: ReactionEmoji<'a>,

    /// Whether this was a super-reaction.
    #[serde(default)]
    pub burst: bool,

    /// The type of reaction.
    #[serde(default, rename = "type")]
    pub reaction_type: u8,
}

/// Event data for `MESSAGE_REACTION_REMOVE_ALL`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageReactionRemoveAllEvent {
    /// ID of the channel.
    pub channel_id: Snowflake,

    /// ID of the message.
    pub message_id: Snowflake,

    /// ID of the guild.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// Event data for `MESSAGE_REACTION_REMOVE_EMOJI`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageReactionRemoveEmojiEvent<'a> {
    /// ID of the channel.
    pub channel_id: Snowflake,

    /// ID of the message.
    pub message_id: Snowflake,

    /// ID of the guild.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// The emoji that was removed.
    pub emoji: ReactionEmoji<'a>,
}

/// Emoji used in a reaction.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ReactionEmoji<'a> {
    /// Emoji ID (null for standard emoji).
    #[serde(default)]
    pub id: Option<Snowflake>,

    /// Emoji name.
    #[serde(default)]
    pub name: Option<TitanString<'a>>,

    /// Whether this emoji is animated.
    #[serde(default)]
    pub animated: bool,
}

impl<'a> ReactionEmoji<'a> {
    /// Create a unicode emoji reaction (e.g., "👍").
    #[inline]
    pub fn unicode(name: impl Into<TitanString<'a>>) -> Self {
        Self {
            id: None,
            name: Some(name.into()),
            animated: false,
        }
    }

    /// Create a custom emoji reaction.
    #[inline]
    pub fn custom(id: impl Into<Snowflake>, name: impl Into<TitanString<'a>>) -> Self {
        Self {
            id: Some(id.into()),
            name: Some(name.into()),
            animated: false,
        }
    }

    /// Create an animated custom emoji reaction.
    #[inline]
    pub fn animated(id: impl Into<Snowflake>, name: impl Into<TitanString<'a>>) -> Self {
        Self {
            id: Some(id.into()),
            name: Some(name.into()),
            animated: true,
        }
    }

    /// Format this emoji for use in Discord text (e.g., "<:name:id>").
    #[must_use]
    pub fn format(&self) -> String {
        match (&self.id, &self.name) {
            (Some(id), Some(name)) if self.animated => format!("<a:{}:{}>", name, id.0),
            (Some(id), Some(name)) => format!("<:{}:{}>", name, id.0),
            (None, Some(name)) => name.to_string(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_add_event() {
        let json = r#"{
            "user_id": "123",
            "channel_id": "456",
            "message_id": "789",
            "emoji": {"name": "đź‘Ť"}
        }"#;

        let event: MessageReactionAddEvent = crate::json::from_str(json).unwrap();
        assert_eq!(event.emoji.name, Some(TitanString::Borrowed("đź‘Ť")));
    }

    #[test]
    fn test_custom_emoji() {
        let json = r#"{
            "id": "123456789",
            "name": "custom_emoji",
            "animated": true
        }"#;

        let emoji: ReactionEmoji = crate::json::from_str(json).unwrap();
        assert!(emoji.animated);
        assert!(emoji.id.is_some());
    }
}
