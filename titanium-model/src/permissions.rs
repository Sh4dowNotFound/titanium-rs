//! Discord Permissions
//!
//! Permissions in Discord are a way to limit and grant certain abilities to users.
//! They are represented by a 64-bit integer, serialized as a string.

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

bitflags! {
    /// Permissions that can be assigned to a Role or User.
    ///
    /// See: https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Permissions: u64 {
        /// Allows creation of instant invites.
        const CREATE_INSTANT_INVITE = 1 << 0;
        /// Allows kicking members.
        const KICK_MEMBERS = 1 << 1;
        /// Allows banning members.
        const BAN_MEMBERS = 1 << 2;
        /// Allows all permissions and bypasses channel permission overwrites.
        const ADMINISTRATOR = 1 << 3;
        /// Allows management and editing of channels.
        const MANAGE_CHANNELS = 1 << 4;
        /// Allows management and editing of the guild.
        const MANAGE_GUILD = 1 << 5;
        /// Allows adding reactions to messages.
        const ADD_REACTIONS = 1 << 6;
        /// Allows viewing the audit log.
        const VIEW_AUDIT_LOG = 1 << 7;
        /// Allows using priority speaker in a voice channel.
        const PRIORITY_SPEAKER = 1 << 8;
        /// Allows the user to go live.
        const STREAM = 1 << 9;
        /// Allows viewing guild content (channels, members, etc).
        const VIEW_CHANNEL = 1 << 10;
        /// Allows sending messages in a channel.
        const SEND_MESSAGES = 1 << 11;
        /// Allows sending TTS messages.
        const SEND_TTS_MESSAGES = 1 << 12;
        /// Allows managing messages of others.
        const MANAGE_MESSAGES = 1 << 13;
        /// Allows embedding content in messages (e.g. links).
        const EMBED_LINKS = 1 << 14;
        /// Allows attaching files.
        const ATTACH_FILES = 1 << 15;
        /// Allows reading of message history.
        const READ_MESSAGE_HISTORY = 1 << 16;
        /// Allows mentioning @everyone, @here, and all roles.
        const MENTION_EVERYONE = 1 << 17;
        /// Allows using external emojis.
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        /// Allows viewing guild insights.
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        /// Allows connecting to a voice channel.
        const CONNECT = 1 << 20;
        /// Allows speaking in a voice channel.
        const SPEAK = 1 << 21;
        /// Allows muting members in a voice channel.
        const MUTE_MEMBERS = 1 << 22;
        /// Allows deafening members in a voice channel.
        const DEAFEN_MEMBERS = 1 << 23;
        /// Allows moving members between voice channels.
        const MOVE_MEMBERS = 1 << 24;
        /// Allows using voice-activity-detection.
        const USE_VAD = 1 << 25;
        /// Allows changing own nickname.
        const CHANGE_NICKNAME = 1 << 26;
        /// Allows managing nicknames of others.
        const MANAGE_NICKNAMES = 1 << 27;
        /// Allows managing roles.
        const MANAGE_ROLES = 1 << 28;
        /// Allows managing webhooks.
        const MANAGE_WEBHOOKS = 1 << 29;
        /// Allows managing emojis and stickers.
        const MANAGE_EMOJIS_AND_STICKERS = 1 << 30;
        /// Allows using application commands.
        const USE_APPLICATION_COMMANDS = 1 << 31;
        /// Allows requesting to speak in stage channels.
        const REQUEST_TO_SPEAK = 1 << 32;
        /// Allows creating, editing, and deleting events.
        const MANAGE_EVENTS = 1 << 33;
        /// Allows managing threads.
        const MANAGE_THREADS = 1 << 34;
        /// Allows creating public threads.
        const CREATE_PUBLIC_THREADS = 1 << 35;
        /// Allows creating private threads.
        const CREATE_PRIVATE_THREADS = 1 << 36;
        /// Allows using external stickers.
        const USE_EXTERNAL_STICKERS = 1 << 37;
        /// Allows sending messages in threads.
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        /// Allows using embedded activities.
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        /// Allows timing out users.
        const MODERATE_MEMBERS = 1 << 40;
        /// Allows viewing creator monetization analytics.
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        /// Allows using soundboard.
        const USE_SOUNDBOARD = 1 << 42;
        /// Allows using external sounds.
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        /// Allows sending voice messages.
        const SEND_VOICE_MESSAGES = 1 << 46;
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Permissions are serialized as strings
        serializer.serialize_str(&self.bits().to_string())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermissionsVisitor;

        impl<'de> serde::de::Visitor<'de> for PermissionsVisitor {
            type Value = Permissions;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or integer representing permissions")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Permissions::from_bits_truncate(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value
                    .parse::<u64>()
                    .map(Permissions::from_bits_truncate)
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(PermissionsVisitor)
    }
}
