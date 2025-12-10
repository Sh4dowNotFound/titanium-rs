use crate::snowflake::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// Discord User representation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User<'a> {
    /// User ID.
    pub id: Snowflake,
    /// Username (not unique per se post-pomelo).
    pub username: TitanString<'a>,
    /// User's 4-digit Discord tag (deprecated, "0" for pomelo users).
    pub discriminator: TitanString<'a>,
    /// User's display name.
    #[serde(default)]
    pub global_name: Option<TitanString<'a>>,
    /// Avatar hash.
    #[serde(default)]
    pub avatar: Option<TitanString<'a>>,
    /// Whether the user is a bot.
    #[serde(default)]
    pub bot: bool,
    /// Whether the user is a system user.
    #[serde(default)]
    pub system: bool,
    /// Whether the user has MFA enabled.
    #[serde(default)]
    pub mfa_enabled: Option<bool>,
    /// Banner hash.
    #[serde(default)]
    pub banner: Option<TitanString<'a>>,
    /// Banner color as integer.
    #[serde(default)]
    pub accent_color: Option<u32>,
    /// User's locale.
    #[serde(default)]
    pub locale: Option<TitanString<'a>>,
    /// Whether email is verified.
    #[serde(default)]
    pub verified: Option<bool>,
    /// User's email (requires email scope).
    #[serde(default)]
    pub email: Option<TitanString<'a>>,
    /// User flags.
    #[serde(default)]
    pub flags: Option<u64>,
    /// Nitro subscription type.
    #[serde(default)]
    pub premium_type: Option<u8>,
    /// Public flags on the user.
    #[serde(default)]
    pub public_flags: Option<u64>,
    /// Avatar decoration data.
    #[serde(default)]
    pub avatar_decoration_data: Option<crate::json::Value>,
}

impl<'a> User<'a> {
    /// Returns the URL of the user's avatar.
    pub fn avatar_url(&self) -> Option<String> {
        self.avatar.as_ref().map(|hash| {
            let ext = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}",
                self.id, hash, ext
            )
        })
    }

    /// Returns the URL of the user's default avatar.
    pub fn default_avatar_url(&self) -> String {
        let index = if self.discriminator == "0" {
            (self.id.0 >> 22) % 6
        } else {
            self.discriminator.parse::<u64>().unwrap_or(0) % 5
        };
        format!("https://cdn.discordapp.com/embed/avatars/{}.png", index)
    }

    /// Returns the user's displayed avatar URL (avatar or default).
    pub fn face(&self) -> String {
        self.avatar_url()
            .unwrap_or_else(|| self.default_avatar_url())
    }
}

impl<'a> crate::Mention for User<'a> {
    fn mention(&self) -> String {
        format!("<@{}>", self.id.0)
    }
}

/// Partial user for presence updates.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartialUser {
    pub id: Snowflake,
}

/// Client status for presence.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientStatus {
    #[serde(default)]
    pub desktop: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub web: Option<String>,
}

/// Presence update event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PresenceUpdateEvent {
    pub user: PartialUser,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub status: String,
    #[serde(default)]
    pub activities: Vec<crate::json::Value>,
    #[serde(default)]
    pub client_status: Option<ClientStatus>,
}
