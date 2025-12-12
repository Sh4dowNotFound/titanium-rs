//! Discord Gateway Intents
//!
//! Intents are a bitfield that controls which events the gateway sends.
//! Some intents are "privileged" and require approval in the Discord Developer Portal.

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Gateway Intents control which events Discord sends to your bot.
    ///
    /// See: https://discord.com/developers/docs/topics/gateway#gateway-intents
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Intents: u64 {
        /// Includes events for guild creation, update, delete, role changes, etc.
        const GUILDS = 1 << 0;

        /// Includes events for member joins, updates, removes.
        /// **PRIVILEGED INTENT** - Requires verification for 100+ servers.
        const GUILD_MEMBERS = 1 << 1;

        /// Includes events for guild bans.
        const GUILD_MODERATION = 1 << 2;

        /// Includes events for emoji and sticker updates.
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;

        /// Includes events for integration updates.
        const GUILD_INTEGRATIONS = 1 << 4;

        /// Includes events for webhook updates.
        const GUILD_WEBHOOKS = 1 << 5;

        /// Includes events for invite creation/deletion.
        const GUILD_INVITES = 1 << 6;

        /// Includes events for voice state updates.
        const GUILD_VOICE_STATES = 1 << 7;

        /// Includes events for user presence updates.
        /// **PRIVILEGED INTENT** - Requires verification for 100+ servers.
        const GUILD_PRESENCES = 1 << 8;

        /// Includes events for messages in guilds (not content without MESSAGE_CONTENT).
        const GUILD_MESSAGES = 1 << 9;

        /// Includes events for message reactions in guilds.
        const GUILD_MESSAGE_REACTIONS = 1 << 10;

        /// Includes events for typing indicators in guilds.
        const GUILD_MESSAGE_TYPING = 1 << 11;

        /// Includes events for direct messages.
        const DIRECT_MESSAGES = 1 << 12;

        /// Includes events for DM reactions.
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;

        /// Includes events for DM typing indicators.
        const DIRECT_MESSAGE_TYPING = 1 << 14;

        /// Enables receiving message content in MESSAGE_CREATE events.
        /// **PRIVILEGED INTENT** - Requires verification for 100+ servers.
        const MESSAGE_CONTENT = 1 << 15;

        /// Includes events for scheduled events.
        const GUILD_SCHEDULED_EVENTS = 1 << 16;

        /// Includes events for AutoMod configuration changes.
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;

        /// Includes events for AutoMod action execution.
        const AUTO_MODERATION_EXECUTION = 1 << 21;

        /// Includes events for poll votes in guilds.
        const GUILD_MESSAGE_POLLS = 1 << 24;

        /// Includes events for poll votes in DMs.
        const DIRECT_MESSAGE_POLLS = 1 << 25;

        // ===== Convenience Combinations =====

        /// All non-privileged intents.
        const NON_PRIVILEGED = Self::GUILDS.bits()
            | Self::GUILD_MODERATION.bits()
            | Self::GUILD_EMOJIS_AND_STICKERS.bits()
            | Self::GUILD_INTEGRATIONS.bits()
            | Self::GUILD_WEBHOOKS.bits()
            | Self::GUILD_INVITES.bits()
            | Self::GUILD_VOICE_STATES.bits()
            | Self::GUILD_MESSAGES.bits()
            | Self::GUILD_MESSAGE_REACTIONS.bits()
            | Self::GUILD_MESSAGE_TYPING.bits()
            | Self::DIRECT_MESSAGES.bits()
            | Self::DIRECT_MESSAGE_REACTIONS.bits()
            | Self::DIRECT_MESSAGE_TYPING.bits()
            | Self::GUILD_SCHEDULED_EVENTS.bits()
            | Self::AUTO_MODERATION_CONFIGURATION.bits()
            | Self::AUTO_MODERATION_EXECUTION.bits()
            | Self::GUILD_MESSAGE_POLLS.bits()
            | Self::DIRECT_MESSAGE_POLLS.bits();

        /// All privileged intents (require approval).
        const PRIVILEGED = Self::GUILD_MEMBERS.bits()
            | Self::GUILD_PRESENCES.bits()
            | Self::MESSAGE_CONTENT.bits();

        /// All intents (use with caution, requires all privileged intents approved).
        const ALL = Self::NON_PRIVILEGED.bits() | Self::PRIVILEGED.bits();
    }
}

impl Default for Intents {
    fn default() -> Self {
        Self::NON_PRIVILEGED
    }
}

impl Serialize for Intents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for Intents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;
        Ok(Intents::from_bits_truncate(bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_flags() {
        let intents = Intents::GUILDS | Intents::GUILD_MESSAGES;
        assert!(intents.contains(Intents::GUILDS));
        assert!(intents.contains(Intents::GUILD_MESSAGES));
        assert!(!intents.contains(Intents::GUILD_MEMBERS));
    }

    #[test]
    fn test_intent_serialization() {
        let intents = Intents::GUILDS | Intents::GUILD_MESSAGES;
        let bits = intents.bits();
        assert_eq!(bits, (1 << 0) | (1 << 9)); // 513
    }
}
