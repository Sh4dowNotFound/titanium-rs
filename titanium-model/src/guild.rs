use crate::member::{Emoji, Role, Sticker};
use crate::snowflake::Snowflake;
use crate::PartialVoiceState;
use crate::TitanString;
use crate::User;
use serde::{Deserialize, Serialize};

/// Discord Guild (Server) representation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild<'a> {
    /// Guild ID.
    pub id: Snowflake,
    /// Guild name (2-100 characters).
    pub name: TitanString<'a>,
    /// Icon hash.
    #[serde(default)]
    pub icon: Option<TitanString<'a>>,
    /// Icon hash for animated icons.
    #[serde(default)]
    pub icon_hash: Option<TitanString<'a>>,
    /// Splash hash.
    #[serde(default)]
    pub splash: Option<TitanString<'a>>,
    /// Discovery splash hash.
    #[serde(default)]
    pub discovery_splash: Option<TitanString<'a>>,
    /// ID of owner.
    #[serde(default)]
    pub owner_id: Option<Snowflake>,
    /// Total permissions for the user in the guild.
    #[serde(default)]
    pub permissions: Option<crate::permissions::Permissions>,
    /// Voice region ID (deprecated).
    #[serde(default)]
    pub region: Option<TitanString<'a>>,
    /// ID of AFK channel.
    #[serde(default)]
    pub afk_channel_id: Option<Snowflake>,
    /// AFK timeout in seconds.
    #[serde(default)]
    pub afk_timeout: Option<u32>,
    /// Verification level required.
    #[serde(default)]
    pub verification_level: Option<u8>,
    /// Default message notification level.
    #[serde(default)]
    pub default_message_notifications: Option<u8>,
    /// Explicit content filter level.
    #[serde(default)]
    pub explicit_content_filter: Option<u8>,
    /// Roles in the guild.
    #[serde(default)]
    pub roles: Vec<Role<'a>>,
    /// Custom guild emojis.
    #[serde(default)]
    pub emojis: Vec<Emoji<'a>>,
    /// Enabled guild features.
    #[serde(default)]
    pub features: Vec<TitanString<'a>>,
    /// Required MFA level.
    #[serde(default)]
    pub mfa_level: Option<u8>,
    /// Application ID of guild creator (if bot-created).
    #[serde(default)]
    pub application_id: Option<Snowflake>,
    /// The ID of the channel for system messages.
    #[serde(default)]
    pub system_channel_id: Option<Snowflake>,
    /// System channel flags.
    #[serde(default)]
    pub system_channel_flags: Option<u64>,
    /// The ID of the channel for rules.
    #[serde(default)]
    pub rules_channel_id: Option<Snowflake>,
    /// Max number of presences (null for large guilds).
    #[serde(default)]
    pub max_presences: Option<u32>,
    /// Max number of members.
    #[serde(default)]
    pub max_members: Option<u32>,
    /// Vanity URL code.
    #[serde(default)]
    pub vanity_url_code: Option<TitanString<'a>>,
    /// Guild description.
    #[serde(default)]
    pub description: Option<TitanString<'a>>,
    /// Banner hash.
    #[serde(default)]
    pub banner: Option<TitanString<'a>>,
    /// Premium tier (boost level).
    #[serde(default)]
    pub premium_tier: Option<u8>,
    /// Number of boosts.
    #[serde(default)]
    pub premium_subscription_count: Option<u32>,
    /// Preferred locale.
    #[serde(default)]
    pub preferred_locale: Option<TitanString<'a>>,
    /// The ID of the channel for public updates.
    #[serde(default)]
    pub public_updates_channel_id: Option<Snowflake>,
    /// Max video channel users.
    #[serde(default)]
    pub max_video_channel_users: Option<u32>,
    /// Max stage video channel users.
    #[serde(default)]
    pub max_stage_video_channel_users: Option<u32>,
    /// Approximate member count.
    #[serde(default)]
    pub approximate_member_count: Option<u32>,
    /// Approximate presence count.
    #[serde(default)]
    pub approximate_presence_count: Option<u32>,
    /// Member count (only in `GUILD_CREATE`).
    #[serde(default)]
    pub member_count: Option<u64>,
    /// Guild NSFW level.
    #[serde(default)]
    pub nsfw_level: Option<u8>,
    /// Custom guild stickers.
    #[serde(default)]
    pub stickers: Vec<Sticker<'a>>,
    /// Whether premium progress bar is enabled.
    #[serde(default)]
    pub premium_progress_bar_enabled: Option<bool>,
    /// The ID of the channel for safety alerts.
    #[serde(default)]
    pub safety_alerts_channel_id: Option<Snowflake>,
    /// Voice states (only in `GUILD_CREATE`).
    #[serde(default)]
    pub voice_states: Vec<PartialVoiceState<'a>>,
}

impl Guild<'_> {
    /// Returns the URL of the guild's icon.
    #[must_use]
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            let ext = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/icons/{}/{}.{}",
                self.id, hash, ext
            )
        })
    }

    /// Returns the URL of the guild's splash.
    #[must_use]
    pub fn splash_url(&self) -> Option<String> {
        self.splash.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/splashes/{}/{}.png",
                self.id, hash
            )
        })
    }

    /// Returns the URL of the guild's discovery splash.
    #[must_use]
    pub fn discovery_splash_url(&self) -> Option<String> {
        self.discovery_splash.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/discovery-splashes/{}/{}.png",
                self.id, hash
            )
        })
    }

    /// Returns the URL of the guild's banner.
    #[must_use]
    pub fn banner_url(&self) -> Option<String> {
        self.banner.as_ref().map(|hash| {
            let ext = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/banners/{}/{}.{}",
                self.id, hash, ext
            )
        })
    }
}

/// Unavailable Guild (during outages).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnavailableGuild {
    /// Guild ID.
    pub id: Snowflake,
    /// Whether unavailable.
    #[serde(default)]
    pub unavailable: bool,
}

// ============================================================================
// Guild-related Events
// ============================================================================

/// Event data for `GUILD_MEMBER_UPDATE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildMemberUpdateEvent<'a> {
    /// The ID of the guild.
    pub guild_id: Snowflake,
    /// User role IDs.
    pub roles: Vec<Snowflake>,
    /// The user.
    pub user: User<'a>,
    /// Nickname of the user.
    #[serde(default)]
    pub nick: Option<String>,
    /// Member's guild avatar hash.
    #[serde(default)]
    pub avatar: Option<String>,
    /// When the user joined the guild.
    #[serde(default)]
    pub joined_at: Option<String>,
    /// When the user started boosting.
    #[serde(default)]
    pub premium_since: Option<String>,
    /// Whether the user is deafened.
    #[serde(default)]
    pub deaf: Option<bool>,
    /// Whether the user is muted.
    #[serde(default)]
    pub mute: Option<bool>,
    /// Whether the user has not yet passed screening.
    #[serde(default)]
    pub pending: Option<bool>,
    /// When the user's timeout will expire.
    #[serde(default)]
    pub communication_disabled_until: Option<String>,
    /// Guild member flags.
    #[serde(default)]
    pub flags: Option<u64>,
}

/// Event data for `GUILD_MEMBER_REMOVE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildMemberRemoveEvent<'a> {
    /// The ID of the guild.
    pub guild_id: Snowflake,
    /// The user who was removed.
    pub user: User<'a>,
}

/// Event data for `GUILD_MEMBERS_CHUNK`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildMembersChunkEvent<'a> {
    /// The ID of the guild.
    pub guild_id: Snowflake,
    /// Set of guild members.
    pub members: Vec<crate::member::GuildMember<'a>>,
    /// Chunk index (starting from 0).
    pub chunk_index: u32,
    /// Total number of expected chunks.
    pub chunk_count: u32,
    /// If passing an invalid ID, it will be returned here.
    #[serde(default)]
    pub not_found: Vec<Snowflake>,
    /// Presences (if requested).
    #[serde(default)]
    pub presences: Vec<crate::json::Value>,
    /// Nonce used in the request.
    #[serde(default)]
    pub nonce: Option<String>,
}

/// Event data for `GUILD_BAN_ADD` / `GUILD_BAN_REMOVE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildBanEvent<'a> {
    /// Guild ID.
    pub guild_id: Snowflake,
    /// The banned user.
    pub user: User<'a>,
}

/// Event data for `GUILD_ROLE_CREATE` / `GUILD_ROLE_UPDATE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildRoleEvent<'a> {
    /// Guild ID.
    pub guild_id: Snowflake,
    /// The role created or updated.
    pub role: Role<'a>,
}

/// Event data for `GUILD_ROLE_DELETE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildRoleDeleteEvent {
    /// Guild ID.
    pub guild_id: Snowflake,
    /// ID of the role.
    pub role_id: Snowflake,
}

/// Event data for `GUILD_EMOJIS_UPDATE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildEmojisUpdateEvent<'a> {
    /// Guild ID.
    pub guild_id: Snowflake,
    /// Array of emojis.
    pub emojis: Vec<Emoji<'a>>,
}

/// Event data for `GUILD_STICKERS_UPDATE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildStickersUpdateEvent<'a> {
    /// Guild ID.
    pub guild_id: Snowflake,
    /// Array of stickers.
    pub stickers: Vec<Sticker<'a>>,
}

/// Event data for `GUILD_MEMBER_ADD`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildMemberAddEvent<'a> {
    /// The ID of the guild.
    pub guild_id: Snowflake,
    /// The user this member represents.
    #[serde(default)]
    pub user: Option<User<'a>>,
    /// This user's guild nickname.
    #[serde(default)]
    pub nick: Option<String>,
    /// The member's guild avatar hash.
    #[serde(default)]
    pub avatar: Option<String>,
    /// Array of role object IDs.
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// When the user joined the guild.
    pub joined_at: String,
    /// Whether the user is deafened.
    #[serde(default)]
    pub deaf: bool,
    /// Whether the user is muted.
    #[serde(default)]
    pub mute: bool,
    /// Guild member flags.
    #[serde(default)]
    pub flags: u64,
    /// Whether the user has not yet passed screening.
    #[serde(default)]
    pub pending: Option<bool>,
}

/// Ready event data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadyEventData<'a> {
    /// Gateway protocol version.
    pub v: u8,
    /// Current user.
    pub user: User<'a>,
    /// Guilds (unavailable at first).
    pub guilds: Vec<UnavailableGuild>,
    /// Session ID for resuming.
    pub session_id: String,
    /// Resume URL.
    pub resume_gateway_url: String,
    /// Shard info.
    #[serde(default)]
    pub shard: Option<[u16; 2]>,
    /// Application information.
    #[serde(default)]
    pub application: Option<Application>,
}

/// Application information for Ready event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    /// Application ID.
    pub id: Snowflake,
    /// Application flags.
    #[serde(default)]
    pub flags: Option<u64>,
}
