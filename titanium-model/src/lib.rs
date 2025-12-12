//! Titan Model - Core types and models for Discord API
#![deny(unsafe_code)]
#![allow(clippy::struct_excessive_bools)]
//!
//! This crate provides zero-copy deserialization support for Discord API entities.
//! All types follow Discord API v10/11 specifications.
//!
//!
//! # Modules
//!

//! - [`automod`] - `AutoMod` rules and actions
//! - [`audit`] - Audit log entries
//! - [`integration`] - Integrations and webhooks
//! - [`invite`] - Guild invites
//! - [`member`] - Guild members, roles, emoji, stickers
//! - [`monetization`] - Entitlements, subscriptions, SKUs
//! - [`reaction`] - Message reactions
//! - [`scheduled`] - Scheduled events
//! - [`soundboard`] - Soundboard sounds
//! - [`stage`] - Stage instances
//! - [`thread`] - Thread channels and members

pub mod audit;
pub mod automod;
pub mod builder;
pub mod cdn_tests;
pub mod command;
pub mod component;
pub mod create_message;
pub mod integration;
pub mod intents;
pub mod interaction;
pub mod invite;
pub mod json;
pub mod member;
pub mod monetization;
pub mod permissions;
pub mod reaction;
pub mod scheduled;
pub mod snowflake;
pub mod soundboard;
pub mod string;
pub mod thread;
pub mod ui;
pub mod voice;

// New modules
pub mod channel;
pub mod guild;
pub mod message;
pub mod user;

// Re-exports from submodules
pub use audit::{AuditLogChange, AuditLogEntry, AuditLogEvent, AuditLogOptions};
pub use automod::{
    AutoModAction, AutoModActionExecution, AutoModActionMetadata, AutoModActionType,
    AutoModEventType, AutoModKeywordPresetType, AutoModRule, AutoModTriggerMetadata,
    AutoModTriggerType,
};
pub use builder::{
    ActionRowBuilder, AutoModRuleBuilder, ButtonBuilder, CommandBuilder, CreateChannelBuilder,
    CreateEmojiBuilder, CreateGuildBuilder, CreateInviteBuilder, CreateRoleBuilder,
    CreateStickerBuilder, EmbedBuilder, InteractionResponseBuilder, MessageBuilder,
    ModifyEmojiBuilder, ModifyGuildBuilder, ModifyMemberBuilder, PollBuilder,
    ScheduledEventBuilder, SelectMenuBuilder, StageInstanceBuilder, StartThreadBuilder,
    WebhookExecuteBuilder,
};
pub use command::{ApplicationCommand, CommandOption, CommandType};
pub use component::{ActionRow, Button, Component, ComponentType, SelectMenu};
pub use create_message::CreateMessage;
pub use create_message::FileUpload;
pub use integration::{
    GuildIntegrationsUpdateEvent, Integration, IntegrationAccount, IntegrationApplication,
    IntegrationDeleteEvent, Webhook, WebhooksUpdateEvent,
};
pub use intents::Intents;
pub use interaction::{
    Interaction, InteractionCallbackData, InteractionCallbackType, InteractionResponse,
    InteractionType,
};
pub use invite::{InviteCreateEvent, InviteDeleteEvent};
pub use member::{Emoji, GuildMember, Role, RoleTags, Sticker};
pub use message::{Poll, PollAnswer, PollMedia, PollResults};
pub use monetization::{
    Entitlement, EntitlementType, Sku, SkuType, Subscription, SubscriptionStatus,
};
pub use permissions::Permissions;
pub use reaction::{
    MessageReactionAddEvent, MessageReactionRemoveAllEvent, MessageReactionRemoveEmojiEvent,
    MessageReactionRemoveEvent, ReactionEmoji,
};
pub use scheduled::{
    ScheduledEvent, ScheduledEventEntityMetadata, ScheduledEventEntityType,
    ScheduledEventPrivacyLevel, ScheduledEventStatus, ScheduledEventUserEvent,
};
pub use snowflake::Snowflake;
pub use soundboard::{
    GuildSoundboardSoundsUpdateEvent, SoundboardSound, SoundboardSoundDeleteEvent,
    SoundboardSoundsUpdateEvent,
};
pub use string::TitanString;
pub use thread::{
    DefaultReaction, ForumTag, ThreadDeleteEvent, ThreadListSyncEvent, ThreadMember,
    ThreadMemberUpdateEvent, ThreadMembersUpdateEvent, ThreadMetadata,
};
pub use voice::PartialVoiceState;
pub use voice::{StageInstance, StagePrivacyLevel};

// Re-exports from new modules
pub use channel::{Channel, ChannelMention, ChannelPinsUpdateEvent, PermissionOverwrite};
pub use guild::{
    Application, Guild, GuildBanEvent, GuildEmojisUpdateEvent, GuildMemberAddEvent,
    GuildMemberRemoveEvent, GuildMemberUpdateEvent, GuildMembersChunkEvent, GuildRoleDeleteEvent,
    GuildRoleEvent, GuildStickersUpdateEvent, ReadyEventData, UnavailableGuild,
};
pub use message::{
    Attachment, Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedMedia, EmbedProvider, Message,
    MessageDeleteBulkEvent, MessageDeleteEvent, MessageReference, MessageUpdateEvent, Reaction,
    ReactionCountDetails, StickerItem, TypingStartEvent,
};
pub use user::{ClientStatus, PartialUser, PresenceUpdateEvent, User};

// Builder From impls are now in builder.rs

/// Trait for items that can be mentioned in Discord.
pub trait Mention {
    /// Returns the mention string for this item (e.g., <@123>).
    fn mention(&self) -> String;
}

impl Mention for GuildMember<'_> {
    fn mention(&self) -> String {
        if let Some(user) = &self.user {
            format!("<@{}>", user.id.0)
        } else {
            String::new() // Should not happen for valid members
        }
    }
}

impl Mention for Role<'_> {
    fn mention(&self) -> String {
        format!("<@&{}>", self.id.0)
    }
}
