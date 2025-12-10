//! Guild member types.
//!
//! Represents Discord guild members with roles, permissions, and presence.

use crate::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// A member of a Discord guild.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildMember<'a> {
    /// The user this guild member represents.
    #[serde(default)]
    pub user: Option<super::User<'a>>,

    /// This user's guild nickname.
    #[serde(default)]
    pub nick: Option<TitanString<'a>>,

    /// The member's guild avatar hash.
    #[serde(default)]
    pub avatar: Option<TitanString<'a>>,

    /// Array of role object IDs.
    #[serde(default)]
    pub roles: smallvec::SmallVec<[Snowflake; 5]>,

    /// When the user joined the guild (ISO8601 timestamp).
    pub joined_at: TitanString<'a>,

    /// When the user started boosting the guild.
    #[serde(default)]
    pub premium_since: Option<TitanString<'a>>,

    /// Whether the user is deafened in voice channels.
    #[serde(default)]
    pub deaf: bool,

    /// Whether the user is muted in voice channels.
    #[serde(default)]
    pub mute: bool,

    /// Guild member flags as a bitfield.
    #[serde(default)]
    pub flags: u64,

    /// Whether the user has not yet passed the guild's Membership Screening.
    #[serde(default)]
    pub pending: Option<bool>,

    /// Total permissions of the member in the channel (including overwrites).
    #[serde(default)]
    pub permissions: Option<crate::permissions::Permissions>,

    /// When the user's timeout will expire (ISO8601 timestamp).
    #[serde(default)]
    pub communication_disabled_until: Option<TitanString<'a>>,
}

/// A Discord role.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Role<'a> {
    /// Role ID.
    pub id: Snowflake,

    /// Role name.
    pub name: TitanString<'a>,

    /// Integer representation of hex color code.
    pub color: u32,

    /// If this role is pinned in the user listing.
    pub hoist: bool,

    /// Role icon hash.
    #[serde(default)]
    pub icon: Option<TitanString<'a>>,

    /// Role unicode emoji.
    #[serde(default)]
    pub unicode_emoji: Option<TitanString<'a>>,

    /// Position of this role.
    pub position: i32,

    /// Permission bit set.
    pub permissions: crate::permissions::Permissions,

    /// Whether this role is managed by an integration.
    pub managed: bool,

    /// Whether this role is mentionable.
    pub mentionable: bool,

    /// The tags this role has.
    #[serde(default)]
    pub tags: Option<RoleTags>,

    /// Role flags as a bitfield.
    #[serde(default)]
    pub flags: u64,
}

/// Tags for a role.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleTags {
    /// The ID of the bot this role belongs to.
    #[serde(default)]
    pub bot_id: Option<Snowflake>,

    /// The ID of the integration this role belongs to.
    #[serde(default)]
    pub integration_id: Option<Snowflake>,

    /// Whether this is the guild's premium subscriber role.
    #[serde(default)]
    pub premium_subscriber: Option<()>,

    /// The ID of the subscription listing for this role.
    #[serde(default)]
    pub subscription_listing_id: Option<Snowflake>,

    /// Whether this role is available for purchase.
    #[serde(default)]
    pub available_for_purchase: Option<()>,

    /// Whether this role is a guild's linked role.
    #[serde(default)]
    pub guild_connections: Option<()>,
}

/// An emoji in a guild.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Emoji<'a> {
    /// Emoji ID.
    #[serde(default)]
    pub id: Option<Snowflake>,

    /// Emoji name (can be null for reaction emoji objects).
    #[serde(default)]
    pub name: Option<TitanString<'a>>,

    /// Roles allowed to use this emoji.
    #[serde(default)]
    pub roles: smallvec::SmallVec<[Snowflake; 5]>,

    /// User that created this emoji.
    #[serde(default)]
    pub user: Option<super::User<'a>>,

    /// Whether this emoji must be wrapped in colons.
    #[serde(default)]
    pub require_colons: bool,

    /// Whether this emoji is managed.
    #[serde(default)]
    pub managed: bool,

    /// Whether this emoji is animated.
    #[serde(default)]
    pub animated: bool,

    /// Whether this emoji can be used.
    #[serde(default)]
    pub available: bool,
}

impl<'a> Emoji<'a> {
    /// Returns the URL of the emoji.
    pub fn url(&self) -> Option<String> {
        self.id.map(|id| {
            let ext = if self.animated { "gif" } else { "png" };
            format!("https://cdn.discordapp.com/emojis/{}.{}", id, ext)
        })
    }
}

/// A sticker in a guild.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sticker<'a> {
    /// ID of the sticker.
    pub id: Snowflake,

    /// ID of the pack the sticker is from.
    #[serde(default)]
    pub pack_id: Option<Snowflake>,

    /// Name of the sticker.
    pub name: TitanString<'a>,

    /// Description of the sticker.
    #[serde(default)]
    pub description: Option<TitanString<'a>>,

    /// Autocomplete/suggestion tags for the sticker.
    #[serde(default)]
    pub tags: TitanString<'a>,

    /// Type of sticker.
    #[serde(rename = "type")]
    pub sticker_type: u8,

    /// Format type of the sticker (1=PNG, 2=APNG, 3=LOTTIE, 4=GIF).
    #[serde(rename = "format_type")]
    pub format_type: u8,

    // ...
    /// The user that uploaded the guild sticker.
    #[serde(default)]
    pub user: Option<super::User<'a>>,

    /// The standard sticker's sort order within its pack.
    #[serde(default)]
    pub sort_value: Option<u32>,
}

impl<'a> Sticker<'a> {
    /// Returns the URL of the sticker.
    pub fn url(&self) -> String {
        // Sticker formats: 1 (PNG), 2 (APNG), 3 (LOTTIE), 4 (GIF)
        // CDN: https://cdn.discordapp.com/stickers/{sticker_id}.{png|json|gif}
        let ext = match self.format_type {
            1 => "png",  // PNG
            2 => "png",  // APNG -> png
            3 => "json", // Lottie
            4 => "gif",  // GIF
            _ => "png",
        };
        format!("https://cdn.discordapp.com/stickers/{}.{}", self.id, ext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guild_member_deserialize() {
        let json = r#"{
            "nick": "Test Nick",
            "roles": ["123456789"],
            "joined_at": "2021-01-01T00:00:00.000Z",
            "deaf": false,
            "mute": false,
            "flags": 0
        }"#;

        let member: GuildMember = crate::json::from_str(json).unwrap();
        assert_eq!(member.nick, Some(TitanString::Borrowed("Test Nick")));
    }

    #[test]
    fn test_role_deserialize() {
        let json = r#"{
            "id": "123",
            "name": "Admin",
            "color": 16711680,
            "hoist": true,
            "position": 10,
            "permissions": "8",
            "managed": false,
            "mentionable": true,
            "flags": 0
        }"#;

        let role: Role = crate::json::from_str(json).unwrap();
        assert_eq!(role.name, "Admin");
        assert_eq!(role.color, 16711680);
    }
}
