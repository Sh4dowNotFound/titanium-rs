//! Gateway event parsing and dispatch.
//!
//! This module provides zero-copy event parsing where possible,
//! using boxed event data to minimize memory overhead for large events.
//!
//! # Event Types
//!
//! All Discord Gateway events from API v10/11 are supported:
//! - Connection events (Ready, Resumed)
//! - Guild events (Create, Update, Delete, Member changes, etc.)
//! - Channel events (Create, Update, Delete, Threads)
//! - Message events (Create, Update, Delete, Reactions)
//! - Interaction events (Slash commands, Buttons, Modals)
//! - Voice events (State, Server updates)
//! - AutoMod events (Rules and actions)
//! - Monetization events (Entitlements, Subscriptions)
//! - And many more...

use crate::error::GatewayError;
use titanium_model::voice::{
    VoiceChannelEffectSendEvent, VoiceServerUpdateEvent, VoiceStateUpdateEvent,
};
pub use titanium_model::*;

/// Parsed Gateway event ready for handler dispatch.
///
/// Events are parsed from Dispatch payloads (opcode 0). Each variant
/// corresponds to a Discord Gateway event type.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event<'a> {
    // =========================================================================
    // Connection Events
    // =========================================================================
    /// Received after successful Identify.
    Ready(Box<ReadyEventData<'a>>),

    /// Received after successful Resume.
    Resumed,

    // =========================================================================
    // Guild Events
    // =========================================================================
    /// Lazy-load for unavailable guild, or guild joined.
    GuildCreate(Box<Guild<'a>>),

    /// Guild was updated.
    GuildUpdate(Box<Guild<'a>>),

    /// Guild was deleted or became unavailable.
    GuildDelete(UnavailableGuild),

    /// User was banned from guild.
    GuildBanAdd(Box<GuildBanEvent<'a>>),

    /// User was unbanned from guild.
    GuildBanRemove(Box<GuildBanEvent<'a>>),

    /// Guild emojis were updated.
    GuildEmojisUpdate(Box<GuildEmojisUpdateEvent<'a>>),

    /// Guild stickers were updated.
    GuildStickersUpdate(Box<GuildStickersUpdateEvent<'a>>),

    /// Guild integrations were updated.
    GuildIntegrationsUpdate(Box<GuildIntegrationsUpdateEvent>),

    /// Audit log entry was created.
    GuildAuditLogEntryCreate(Box<AuditLogEntry>),

    // =========================================================================
    // Guild Member Events
    // =========================================================================
    /// New member joined guild.
    GuildMemberAdd(Box<GuildMemberAddEvent<'a>>),

    /// Member was removed from guild.
    GuildMemberRemove(Box<GuildMemberRemoveEvent<'a>>),

    /// Member was updated.
    GuildMemberUpdate(Box<GuildMemberUpdateEvent<'a>>),

    /// Response to Request Guild Members.
    GuildMembersChunk(Box<GuildMembersChunkEvent<'a>>),

    // =========================================================================
    // Role Events
    // =========================================================================
    /// Role was created.
    GuildRoleCreate(Box<GuildRoleEvent<'a>>),

    /// Role was updated.
    GuildRoleUpdate(Box<GuildRoleEvent<'a>>),

    /// Role was deleted.
    GuildRoleDelete(Box<GuildRoleDeleteEvent>),

    // =========================================================================
    // Channel Events
    // =========================================================================
    /// Channel was created.
    ChannelCreate(Box<Channel<'a>>),

    /// Channel was updated.
    ChannelUpdate(Box<Channel<'a>>),

    /// Channel was deleted.
    ChannelDelete(Box<Channel<'a>>),

    /// Message was pinned/unpinned.
    ChannelPinsUpdate(Box<ChannelPinsUpdateEvent>),

    // =========================================================================
    // Thread Events
    // =========================================================================
    /// Thread was created.
    ThreadCreate(Box<Channel<'a>>),

    /// Thread was updated.
    ThreadUpdate(Box<Channel<'a>>),

    /// Thread was deleted.
    ThreadDelete(Box<ThreadDeleteEvent>),

    /// Sent when gaining access to a channel with threads.
    ThreadListSync(Box<ThreadListSyncEvent<'a>>),

    /// Current user's thread member was updated.
    ThreadMemberUpdate(Box<ThreadMemberUpdateEvent<'a>>),

    /// Users were added/removed from a thread.
    ThreadMembersUpdate(Box<ThreadMembersUpdateEvent<'a>>),

    // =========================================================================
    // Message Events
    // =========================================================================
    /// Message was created.
    MessageCreate(Box<Message<'a>>),

    /// Message was updated.
    MessageUpdate(Box<MessageUpdateEvent<'a>>),

    /// Message was deleted.
    MessageDelete(MessageDeleteEvent),

    /// Multiple messages were deleted (bulk).
    MessageDeleteBulk(MessageDeleteBulkEvent),

    // =========================================================================
    // Reaction Events
    // =========================================================================
    /// Reaction was added to a message.
    MessageReactionAdd(Box<MessageReactionAddEvent<'a>>),

    /// Reaction was removed from a message.
    MessageReactionRemove(Box<MessageReactionRemoveEvent<'a>>),

    /// All reactions were removed from a message.
    MessageReactionRemoveAll(Box<MessageReactionRemoveAllEvent>),

    /// All reactions for an emoji were removed.
    MessageReactionRemoveEmoji(Box<MessageReactionRemoveEmojiEvent<'a>>),

    // =========================================================================
    // Interaction Events
    // =========================================================================
    /// An interaction was created (slash command, button, modal, etc.).
    InteractionCreate(Box<Interaction<'a>>),

    // =========================================================================
    // Invite Events
    // =========================================================================
    /// Invite was created.
    InviteCreate(Box<InviteCreateEvent<'a>>),

    /// Invite was deleted.
    InviteDelete(Box<InviteDeleteEvent<'a>>),

    // =========================================================================
    // Stage Instance Events
    // =========================================================================
    /// Stage instance was created.
    StageInstanceCreate(Box<StageInstance>),

    /// Stage instance was updated.
    StageInstanceUpdate(Box<StageInstance>),

    /// Stage instance was deleted.
    StageInstanceDelete(Box<StageInstance>),

    // =========================================================================
    // Scheduled Event Events
    // =========================================================================
    /// Scheduled event was created.
    GuildScheduledEventCreate(Box<ScheduledEvent<'a>>),

    /// Scheduled event was updated.
    GuildScheduledEventUpdate(Box<ScheduledEvent<'a>>),

    /// Scheduled event was deleted.
    GuildScheduledEventDelete(Box<ScheduledEvent<'a>>),

    /// User subscribed to scheduled event.
    GuildScheduledEventUserAdd(Box<ScheduledEventUserEvent>),

    /// User unsubscribed from scheduled event.
    GuildScheduledEventUserRemove(Box<ScheduledEventUserEvent>),

    // =========================================================================
    // AutoMod Events
    // =========================================================================
    /// AutoMod rule was created.
    AutoModerationRuleCreate(Box<AutoModRule>),

    /// AutoMod rule was updated.
    AutoModerationRuleUpdate(Box<AutoModRule>),

    /// AutoMod rule was deleted.
    AutoModerationRuleDelete(Box<AutoModRule>),

    /// AutoMod action was executed.
    AutoModerationActionExecution(Box<AutoModActionExecution>),

    // =========================================================================
    // Integration Events
    // =========================================================================
    /// Integration was created.
    IntegrationCreate(Box<Integration<'a>>),

    /// Integration was updated.
    IntegrationUpdate(Box<Integration<'a>>),

    /// Integration was deleted.
    IntegrationDelete(Box<IntegrationDeleteEvent>),

    // =========================================================================
    // Webhook Events
    // =========================================================================
    /// Webhooks were updated in a channel.
    WebhooksUpdate(Box<WebhooksUpdateEvent>),

    // =========================================================================
    // Monetization Events
    // =========================================================================
    /// Entitlement was created.
    EntitlementCreate(Box<Entitlement>),

    /// Entitlement was updated.
    EntitlementUpdate(Box<Entitlement>),

    /// Entitlement was deleted.
    EntitlementDelete(Box<Entitlement>),

    /// Subscription was created.
    SubscriptionCreate(Box<Subscription>),

    /// Subscription was updated.
    SubscriptionUpdate(Box<Subscription>),

    /// Subscription was deleted.
    SubscriptionDelete(Box<Subscription>),

    // =========================================================================
    // Soundboard Events
    // =========================================================================
    /// Soundboard sound was created in a guild.
    SoundboardSoundCreate(Box<SoundboardSound<'a>>),

    /// Soundboard sound was updated in a guild.
    SoundboardSoundUpdate(Box<SoundboardSound<'a>>),

    /// Soundboard sound was deleted in a guild.
    SoundboardSoundDelete(Box<SoundboardSoundDeleteEvent>),

    /// Multiple soundboard sounds were updated.
    SoundboardSoundsUpdate(Box<SoundboardSoundsUpdateEvent<'a>>),

    /// Guild soundboard sounds were updated.
    GuildSoundboardSoundsUpdate(Box<GuildSoundboardSoundsUpdateEvent<'a>>),

    // =========================================================================
    // Presence & Typing Events
    // =========================================================================
    /// User started typing.
    TypingStart(Box<TypingStartEvent<'a>>),

    /// User's presence was updated.
    PresenceUpdate(Box<PresenceUpdateEvent>),

    /// Current user was updated.
    UserUpdate(Box<User<'a>>),

    // =========================================================================
    // Voice Events
    // =========================================================================
    /// Voice state was updated.
    VoiceStateUpdate(Box<VoiceStateUpdateEvent<'a>>),

    /// Voice server information received.
    VoiceServerUpdate(VoiceServerUpdateEvent),

    /// Voice channel effect was sent.
    VoiceChannelEffectSend(Box<VoiceChannelEffectSendEvent<'a>>),

    // =========================================================================
    // Unknown / Raw Event
    // =========================================================================
    /// Unknown event type - contains raw JSON for custom handling.
    Unknown {
        /// The event name.
        name: String,
        /// Raw JSON data.
        data: titanium_model::json::Value,
    },
}

// ============================================================================
// Event Data Structures
// ============================================================================

// Note: Most event data structures are now imported from titan-model.

// ============================================================================
// Event Parsing
// ============================================================================

/// Parse an event from its name and raw JSON data.
///
/// This function uses a match block for O(1) event dispatch without
/// unnecessary allocations. Events are boxed to minimize enum size.
#[cfg(not(feature = "simd"))]
pub fn parse_event(
    event_name: &str,
    data: &serde_json::value::RawValue,
) -> Result<Event<'static>, GatewayError> {
    let json_str = data.get();

    match event_name {
        // Connection Events
        "READY" => {
            let ready: ReadyEventData = serde_json::from_str(json_str)?;
            Ok(Event::Ready(Box::new(ready)))
        }
        "RESUMED" => Ok(Event::Resumed),

        // Guild Events
        "GUILD_CREATE" => {
            let guild: Guild = serde_json::from_str(json_str)?;
            Ok(Event::GuildCreate(Box::new(guild)))
        }
        "GUILD_UPDATE" => {
            let guild: Guild = serde_json::from_str(json_str)?;
            Ok(Event::GuildUpdate(Box::new(guild)))
        }
        "GUILD_DELETE" => {
            let guild: UnavailableGuild = serde_json::from_str(json_str)?;
            Ok(Event::GuildDelete(guild))
        }
        "GUILD_BAN_ADD" => {
            let ban: GuildBanEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildBanAdd(Box::new(ban)))
        }
        "GUILD_BAN_REMOVE" => {
            let ban: GuildBanEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildBanRemove(Box::new(ban)))
        }
        "GUILD_EMOJIS_UPDATE" => {
            let emojis: GuildEmojisUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildEmojisUpdate(Box::new(emojis)))
        }
        "GUILD_STICKERS_UPDATE" => {
            let stickers: GuildStickersUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildStickersUpdate(Box::new(stickers)))
        }
        "GUILD_INTEGRATIONS_UPDATE" => {
            let integrations: GuildIntegrationsUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildIntegrationsUpdate(Box::new(integrations)))
        }
        "GUILD_AUDIT_LOG_ENTRY_CREATE" => {
            let entry: AuditLogEntry = serde_json::from_str(json_str)?;
            Ok(Event::GuildAuditLogEntryCreate(Box::new(entry)))
        }

        // Guild Member Events
        "GUILD_MEMBER_ADD" => {
            let member: GuildMemberAddEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildMemberAdd(Box::new(member)))
        }
        "GUILD_MEMBER_REMOVE" => {
            let member: GuildMemberRemoveEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildMemberRemove(Box::new(member)))
        }
        "GUILD_MEMBER_UPDATE" => {
            let member: GuildMemberUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildMemberUpdate(Box::new(member)))
        }
        "GUILD_MEMBERS_CHUNK" => {
            let chunk: GuildMembersChunkEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildMembersChunk(Box::new(chunk)))
        }

        // Role Events
        "GUILD_ROLE_CREATE" => {
            let role: GuildRoleEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildRoleCreate(Box::new(role)))
        }
        "GUILD_ROLE_UPDATE" => {
            let role: GuildRoleEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildRoleUpdate(Box::new(role)))
        }
        "GUILD_ROLE_DELETE" => {
            let role: GuildRoleDeleteEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildRoleDelete(Box::new(role)))
        }

        // Channel Events
        "CHANNEL_CREATE" => {
            let channel: Channel = serde_json::from_str(json_str)?;
            Ok(Event::ChannelCreate(Box::new(channel)))
        }
        "CHANNEL_UPDATE" => {
            let channel: Channel = serde_json::from_str(json_str)?;
            Ok(Event::ChannelUpdate(Box::new(channel)))
        }
        "CHANNEL_DELETE" => {
            let channel: Channel = serde_json::from_str(json_str)?;
            Ok(Event::ChannelDelete(Box::new(channel)))
        }
        "CHANNEL_PINS_UPDATE" => {
            let pins: ChannelPinsUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::ChannelPinsUpdate(Box::new(pins)))
        }

        // Thread Events
        "THREAD_CREATE" => {
            let thread: Channel = serde_json::from_str(json_str)?;
            Ok(Event::ThreadCreate(Box::new(thread)))
        }
        "THREAD_UPDATE" => {
            let thread: Channel = serde_json::from_str(json_str)?;
            Ok(Event::ThreadUpdate(Box::new(thread)))
        }
        "THREAD_DELETE" => {
            let thread: ThreadDeleteEvent = serde_json::from_str(json_str)?;
            Ok(Event::ThreadDelete(Box::new(thread)))
        }
        "THREAD_LIST_SYNC" => {
            let sync: ThreadListSyncEvent = serde_json::from_str(json_str)?;
            Ok(Event::ThreadListSync(Box::new(sync)))
        }
        "THREAD_MEMBER_UPDATE" => {
            let member: ThreadMemberUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::ThreadMemberUpdate(Box::new(member)))
        }
        "THREAD_MEMBERS_UPDATE" => {
            let members: ThreadMembersUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::ThreadMembersUpdate(Box::new(members)))
        }

        // Message Events
        "MESSAGE_CREATE" => {
            let message: Message = serde_json::from_str(json_str)?;
            Ok(Event::MessageCreate(Box::new(message)))
        }
        "MESSAGE_UPDATE" => {
            let update: MessageUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::MessageUpdate(Box::new(update)))
        }
        "MESSAGE_DELETE" => {
            let delete: MessageDeleteEvent = serde_json::from_str(json_str)?;
            Ok(Event::MessageDelete(delete))
        }
        "MESSAGE_DELETE_BULK" => {
            let delete_bulk: MessageDeleteBulkEvent = serde_json::from_str(json_str)?;
            Ok(Event::MessageDeleteBulk(delete_bulk))
        }

        // Reaction Events
        "MESSAGE_REACTION_ADD" => {
            let reaction: MessageReactionAddEvent = serde_json::from_str(json_str)?;
            Ok(Event::MessageReactionAdd(Box::new(reaction)))
        }
        "MESSAGE_REACTION_REMOVE" => {
            let reaction: MessageReactionRemoveEvent = serde_json::from_str(json_str)?;
            Ok(Event::MessageReactionRemove(Box::new(reaction)))
        }
        "MESSAGE_REACTION_REMOVE_ALL" => {
            let reaction: MessageReactionRemoveAllEvent = serde_json::from_str(json_str)?;
            Ok(Event::MessageReactionRemoveAll(Box::new(reaction)))
        }
        "MESSAGE_REACTION_REMOVE_EMOJI" => {
            let reaction: MessageReactionRemoveEmojiEvent = serde_json::from_str(json_str)?;
            Ok(Event::MessageReactionRemoveEmoji(Box::new(reaction)))
        }

        // Interaction Events
        "INTERACTION_CREATE" => {
            let interaction: Interaction = serde_json::from_str(json_str)?;
            Ok(Event::InteractionCreate(Box::new(interaction)))
        }

        // Invite Events
        "INVITE_CREATE" => {
            let invite: InviteCreateEvent = serde_json::from_str(json_str)?;
            Ok(Event::InviteCreate(Box::new(invite)))
        }
        "INVITE_DELETE" => {
            let invite: InviteDeleteEvent = serde_json::from_str(json_str)?;
            Ok(Event::InviteDelete(Box::new(invite)))
        }

        // Stage Instance Events
        "STAGE_INSTANCE_CREATE" => {
            let stage: StageInstance = serde_json::from_str(json_str)?;
            Ok(Event::StageInstanceCreate(Box::new(stage)))
        }
        "STAGE_INSTANCE_UPDATE" => {
            let stage: StageInstance = serde_json::from_str(json_str)?;
            Ok(Event::StageInstanceUpdate(Box::new(stage)))
        }
        "STAGE_INSTANCE_DELETE" => {
            let stage: StageInstance = serde_json::from_str(json_str)?;
            Ok(Event::StageInstanceDelete(Box::new(stage)))
        }

        // Scheduled Event Events
        "GUILD_SCHEDULED_EVENT_CREATE" => {
            let event: ScheduledEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildScheduledEventCreate(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_UPDATE" => {
            let event: ScheduledEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildScheduledEventUpdate(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_DELETE" => {
            let event: ScheduledEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildScheduledEventDelete(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_USER_ADD" => {
            let event: ScheduledEventUserEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildScheduledEventUserAdd(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
            let event: ScheduledEventUserEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildScheduledEventUserRemove(Box::new(event)))
        }

        // AutoMod Events
        "AUTO_MODERATION_RULE_CREATE" => {
            let rule: AutoModRule = serde_json::from_str(json_str)?;
            Ok(Event::AutoModerationRuleCreate(Box::new(rule)))
        }
        "AUTO_MODERATION_RULE_UPDATE" => {
            let rule: AutoModRule = serde_json::from_str(json_str)?;
            Ok(Event::AutoModerationRuleUpdate(Box::new(rule)))
        }
        "AUTO_MODERATION_RULE_DELETE" => {
            let rule: AutoModRule = serde_json::from_str(json_str)?;
            Ok(Event::AutoModerationRuleDelete(Box::new(rule)))
        }
        "AUTO_MODERATION_ACTION_EXECUTION" => {
            let action: AutoModActionExecution = serde_json::from_str(json_str)?;
            Ok(Event::AutoModerationActionExecution(Box::new(action)))
        }

        // Integration Events
        "INTEGRATION_CREATE" => {
            let integration: Integration = serde_json::from_str(json_str)?;
            Ok(Event::IntegrationCreate(Box::new(integration)))
        }
        "INTEGRATION_UPDATE" => {
            let integration: Integration = serde_json::from_str(json_str)?;
            Ok(Event::IntegrationUpdate(Box::new(integration)))
        }
        "INTEGRATION_DELETE" => {
            let integration: IntegrationDeleteEvent = serde_json::from_str(json_str)?;
            Ok(Event::IntegrationDelete(Box::new(integration)))
        }

        // Webhook Events
        "WEBHOOKS_UPDATE" => {
            let webhooks: WebhooksUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::WebhooksUpdate(Box::new(webhooks)))
        }

        // Monetization Events
        "ENTITLEMENT_CREATE" => {
            let entitlement: Entitlement = serde_json::from_str(json_str)?;
            Ok(Event::EntitlementCreate(Box::new(entitlement)))
        }
        "ENTITLEMENT_UPDATE" => {
            let entitlement: Entitlement = serde_json::from_str(json_str)?;
            Ok(Event::EntitlementUpdate(Box::new(entitlement)))
        }
        "ENTITLEMENT_DELETE" => {
            let entitlement: Entitlement = serde_json::from_str(json_str)?;
            Ok(Event::EntitlementDelete(Box::new(entitlement)))
        }
        "SUBSCRIPTION_CREATE" => {
            let subscription: Subscription = serde_json::from_str(json_str)?;
            Ok(Event::SubscriptionCreate(Box::new(subscription)))
        }
        "SUBSCRIPTION_UPDATE" => {
            let subscription: Subscription = serde_json::from_str(json_str)?;
            Ok(Event::SubscriptionUpdate(Box::new(subscription)))
        }
        "SUBSCRIPTION_DELETE" => {
            let subscription: Subscription = serde_json::from_str(json_str)?;
            Ok(Event::SubscriptionDelete(Box::new(subscription)))
        }

        // Soundboard Events
        "SOUNDBOARD_SOUND_CREATE" => {
            let sound: SoundboardSound = serde_json::from_str(json_str)?;
            Ok(Event::SoundboardSoundCreate(Box::new(sound)))
        }
        "SOUNDBOARD_SOUND_UPDATE" => {
            let sound: SoundboardSound = serde_json::from_str(json_str)?;
            Ok(Event::SoundboardSoundUpdate(Box::new(sound)))
        }
        "SOUNDBOARD_SOUND_DELETE" => {
            let sound: SoundboardSoundDeleteEvent = serde_json::from_str(json_str)?;
            Ok(Event::SoundboardSoundDelete(Box::new(sound)))
        }
        "SOUNDBOARD_SOUNDS_UPDATE" => {
            let sounds: SoundboardSoundsUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::SoundboardSoundsUpdate(Box::new(sounds)))
        }
        "GUILD_SOUNDBOARD_SOUNDS_UPDATE" => {
            let sounds: GuildSoundboardSoundsUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::GuildSoundboardSoundsUpdate(Box::new(sounds)))
        }

        // Presence & Typing Events
        "TYPING_START" => {
            let typing: TypingStartEvent = serde_json::from_str(json_str)?;
            Ok(Event::TypingStart(typing))
        }
        "PRESENCE_UPDATE" => {
            let presence: PresenceUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::PresenceUpdate(Box::new(presence)))
        }
        "USER_UPDATE" => {
            let user: User = serde_json::from_str(json_str)?;
            Ok(Event::UserUpdate(Box::new(user)))
        }

        // Voice Events
        "VOICE_STATE_UPDATE" => {
            let voice_state: VoiceStateUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::VoiceStateUpdate(Box::new(voice_state)))
        }
        "VOICE_SERVER_UPDATE" => {
            let voice_server: VoiceServerUpdateEvent = serde_json::from_str(json_str)?;
            Ok(Event::VoiceServerUpdate(voice_server))
        }
        "VOICE_CHANNEL_EFFECT_SEND" => {
            let effect: VoiceChannelEffectSendEvent = serde_json::from_str(json_str)?;
            Ok(Event::VoiceChannelEffectSend(Box::new(effect)))
        }

        // Unknown event - parse as generic JSON Value
        _ => {
            let value: serde_json::Value = serde_json::from_str(json_str)?;
            Ok(Event::Unknown {
                name: event_name.to_owned(),
                data: value,
            })
        }
    }
}

/// Parse an event from its name and parsed JSON Value (SIMD).
#[cfg(feature = "simd")]
pub fn parse_event<'a>(
    event_name: &str,
    data: titanium_model::json::BorrowedValue<'a>,
) -> Result<Event<'static>, GatewayError> {
    match event_name {
        // Connection Events
        "READY" => {
            let ready: ReadyEventData = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::Ready(Box::new(ready)))
        }
        "RESUMED" => Ok(Event::Resumed),

        // Guild Events
        "GUILD_CREATE" => {
            let guild: Guild = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildCreate(Box::new(guild)))
        }
        "GUILD_UPDATE" => {
            let guild: Guild = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildUpdate(Box::new(guild)))
        }
        "GUILD_DELETE" => {
            let guild: UnavailableGuild = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildDelete(guild))
        }
        "GUILD_BAN_ADD" => {
            let ban: GuildBanEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildBanAdd(Box::new(ban)))
        }
        "GUILD_BAN_REMOVE" => {
            let ban: GuildBanEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildBanRemove(Box::new(ban)))
        }
        "GUILD_EMOJIS_UPDATE" => {
            let emojis: GuildEmojisUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildEmojisUpdate(Box::new(emojis)))
        }
        "GUILD_STICKERS_UPDATE" => {
            let stickers: GuildStickersUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildStickersUpdate(Box::new(stickers)))
        }
        "GUILD_INTEGRATIONS_UPDATE" => {
            let integrations: GuildIntegrationsUpdateEvent =
                titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildIntegrationsUpdate(Box::new(integrations)))
        }
        "GUILD_AUDIT_LOG_ENTRY_CREATE" => {
            let entry: AuditLogEntry = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildAuditLogEntryCreate(Box::new(entry)))
        }

        // Guild Member Events
        "GUILD_MEMBER_ADD" => {
            let member: GuildMemberAddEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildMemberAdd(Box::new(member)))
        }
        "GUILD_MEMBER_REMOVE" => {
            let member: GuildMemberRemoveEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildMemberRemove(Box::new(member)))
        }
        "GUILD_MEMBER_UPDATE" => {
            let member: GuildMemberUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildMemberUpdate(Box::new(member)))
        }
        "GUILD_MEMBERS_CHUNK" => {
            let chunk: GuildMembersChunkEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildMembersChunk(Box::new(chunk)))
        }

        // Role Events
        "GUILD_ROLE_CREATE" => {
            let role: GuildRoleEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildRoleCreate(Box::new(role)))
        }
        "GUILD_ROLE_UPDATE" => {
            let role: GuildRoleEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildRoleUpdate(Box::new(role)))
        }
        "GUILD_ROLE_DELETE" => {
            let role: GuildRoleDeleteEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildRoleDelete(Box::new(role)))
        }

        // Channel Events
        "CHANNEL_CREATE" => {
            let channel: Channel = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ChannelCreate(Box::new(channel)))
        }
        "CHANNEL_UPDATE" => {
            let channel: Channel = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ChannelUpdate(Box::new(channel)))
        }
        "CHANNEL_DELETE" => {
            let channel: Channel = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ChannelDelete(Box::new(channel)))
        }
        "CHANNEL_PINS_UPDATE" => {
            let pins: ChannelPinsUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ChannelPinsUpdate(Box::new(pins)))
        }

        // Thread Events
        "THREAD_CREATE" => {
            let thread: Channel = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ThreadCreate(Box::new(thread)))
        }
        "THREAD_UPDATE" => {
            let thread: Channel = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ThreadUpdate(Box::new(thread)))
        }
        "THREAD_DELETE" => {
            let thread: ThreadDeleteEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ThreadDelete(Box::new(thread)))
        }
        "THREAD_LIST_SYNC" => {
            let sync: ThreadListSyncEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ThreadListSync(Box::new(sync)))
        }
        "THREAD_MEMBER_UPDATE" => {
            let member: ThreadMemberUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ThreadMemberUpdate(Box::new(member)))
        }
        "THREAD_MEMBERS_UPDATE" => {
            let members: ThreadMembersUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::ThreadMembersUpdate(Box::new(members)))
        }

        // Message Events
        "MESSAGE_CREATE" => {
            let message: Message = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageCreate(Box::new(message)))
        }
        "MESSAGE_UPDATE" => {
            let update: MessageUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageUpdate(Box::new(update)))
        }
        "MESSAGE_DELETE" => {
            let delete: MessageDeleteEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageDelete(delete))
        }
        "MESSAGE_DELETE_BULK" => {
            let delete_bulk: MessageDeleteBulkEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageDeleteBulk(delete_bulk))
        }

        // Reaction Events
        "MESSAGE_REACTION_ADD" => {
            let reaction: MessageReactionAddEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageReactionAdd(Box::new(reaction)))
        }
        "MESSAGE_REACTION_REMOVE" => {
            let reaction: MessageReactionRemoveEvent =
                titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageReactionRemove(Box::new(reaction)))
        }
        "MESSAGE_REACTION_REMOVE_ALL" => {
            let reaction: MessageReactionRemoveAllEvent =
                titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageReactionRemoveAll(Box::new(reaction)))
        }
        "MESSAGE_REACTION_REMOVE_EMOJI" => {
            let reaction: MessageReactionRemoveEmojiEvent =
                titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::MessageReactionRemoveEmoji(Box::new(reaction)))
        }

        // Interaction Events
        "INTERACTION_CREATE" => {
            let interaction: Interaction = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::InteractionCreate(Box::new(interaction)))
        }

        // Invite Events
        "INVITE_CREATE" => {
            let invite: InviteCreateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::InviteCreate(Box::new(invite)))
        }
        "INVITE_DELETE" => {
            let invite: InviteDeleteEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::InviteDelete(Box::new(invite)))
        }

        // Stage Instance Events
        "STAGE_INSTANCE_CREATE" => {
            let stage: StageInstance = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::StageInstanceCreate(Box::new(stage)))
        }
        "STAGE_INSTANCE_UPDATE" => {
            let stage: StageInstance = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::StageInstanceUpdate(Box::new(stage)))
        }
        "STAGE_INSTANCE_DELETE" => {
            let stage: StageInstance = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::StageInstanceDelete(Box::new(stage)))
        }

        // Scheduled Event Events
        "GUILD_SCHEDULED_EVENT_CREATE" => {
            let event: ScheduledEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildScheduledEventCreate(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_UPDATE" => {
            let event: ScheduledEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildScheduledEventUpdate(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_DELETE" => {
            let event: ScheduledEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildScheduledEventDelete(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_USER_ADD" => {
            let event: ScheduledEventUserEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildScheduledEventUserAdd(Box::new(event)))
        }
        "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
            let event: ScheduledEventUserEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildScheduledEventUserRemove(Box::new(event)))
        }

        // AutoMod Events
        "AUTO_MODERATION_RULE_CREATE" => {
            let rule: AutoModRule = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::AutoModerationRuleCreate(Box::new(rule)))
        }
        "AUTO_MODERATION_RULE_UPDATE" => {
            let rule: AutoModRule = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::AutoModerationRuleUpdate(Box::new(rule)))
        }
        "AUTO_MODERATION_RULE_DELETE" => {
            let rule: AutoModRule = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::AutoModerationRuleDelete(Box::new(rule)))
        }
        "AUTO_MODERATION_ACTION_EXECUTION" => {
            let action: AutoModActionExecution = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::AutoModerationActionExecution(Box::new(action)))
        }

        // Integration Events
        "INTEGRATION_CREATE" => {
            let integration: Integration = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::IntegrationCreate(Box::new(integration)))
        }
        "INTEGRATION_UPDATE" => {
            let integration: Integration = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::IntegrationUpdate(Box::new(integration)))
        }
        "INTEGRATION_DELETE" => {
            let integration: IntegrationDeleteEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::IntegrationDelete(Box::new(integration)))
        }

        // Webhook Events
        "WEBHOOKS_UPDATE" => {
            let webhooks: WebhooksUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::WebhooksUpdate(Box::new(webhooks)))
        }

        // Monetization Events
        "ENTITLEMENT_CREATE" => {
            let entitlement: Entitlement = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::EntitlementCreate(Box::new(entitlement)))
        }
        "ENTITLEMENT_UPDATE" => {
            let entitlement: Entitlement = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::EntitlementUpdate(Box::new(entitlement)))
        }
        "ENTITLEMENT_DELETE" => {
            let entitlement: Entitlement = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::EntitlementDelete(Box::new(entitlement)))
        }
        "SUBSCRIPTION_CREATE" => {
            let subscription: Subscription = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::SubscriptionCreate(Box::new(subscription)))
        }
        "SUBSCRIPTION_UPDATE" => {
            let subscription: Subscription = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::SubscriptionUpdate(Box::new(subscription)))
        }
        "SUBSCRIPTION_DELETE" => {
            let subscription: Subscription = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::SubscriptionDelete(Box::new(subscription)))
        }

        // Soundboard Events
        "SOUNDBOARD_SOUND_CREATE" => {
            let sound: SoundboardSound = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::SoundboardSoundCreate(Box::new(sound)))
        }
        "SOUNDBOARD_SOUND_UPDATE" => {
            let sound: SoundboardSound = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::SoundboardSoundUpdate(Box::new(sound)))
        }
        "SOUNDBOARD_SOUND_DELETE" => {
            let sound: SoundboardSoundDeleteEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::SoundboardSoundDelete(Box::new(sound)))
        }
        "SOUNDBOARD_SOUNDS_UPDATE" => {
            let sounds: SoundboardSoundsUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::SoundboardSoundsUpdate(Box::new(sounds)))
        }
        "GUILD_SOUNDBOARD_SOUNDS_UPDATE" => {
            let sounds: GuildSoundboardSoundsUpdateEvent =
                titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::GuildSoundboardSoundsUpdate(Box::new(sounds)))
        }

        // Presence & Typing Events
        "TYPING_START" => {
            let typing: TypingStartEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::TypingStart(Box::new(typing)))
        }
        "PRESENCE_UPDATE" => {
            let presence: PresenceUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::PresenceUpdate(Box::new(presence)))
        }
        "USER_UPDATE" => {
            let user: User = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::UserUpdate(Box::new(user)))
        }

        // Voice Events
        "VOICE_STATE_UPDATE" => {
            let voice_state: VoiceStateUpdateEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::VoiceStateUpdate(Box::new(voice_state)))
        }
        "VOICE_SERVER_UPDATE" => {
            let voice_server: VoiceServerUpdateEvent =
                titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::VoiceServerUpdate(voice_server))
        }
        "VOICE_CHANNEL_EFFECT_SEND" => {
            let effect: VoiceChannelEffectSendEvent = titanium_model::json::from_borrowed_value(data)?;
            Ok(Event::VoiceChannelEffectSend(Box::new(effect)))
        }

        // Unknown event
        _ => Ok(Event::Unknown {
            name: event_name.to_owned(),
            data: data.into(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ready_event() {
        let json = r#"{
            "v": 10,
            "user": {"id": "123", "username": "test", "discriminator": "0001"},
            "guilds": [],
            "session_id": "abc123",
            "resume_gateway_url": "wss://gateway.discord.gg"
        }"#;

        let ready: ReadyEventData = serde_json::from_str(json).unwrap();
        assert_eq!(ready.v, 10);
        assert_eq!(ready.session_id, "abc123");
    }

    #[test]
    fn test_parse_message_delete() {
        let json = r#"{"id": "123", "channel_id": "456", "guild_id": "789"}"#;
        let event: MessageDeleteEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.id.get(), 123);
        assert_eq!(event.channel_id.get(), 456);
    }

    #[test]
    fn test_interaction_parse() {
        let json = r#"{
            "id": "123",
            "application_id": "456",
            "type": 2,
            "token": "abc",
            "version": 1
        }"#;

        let interaction: Interaction = serde_json::from_str(json).unwrap();
        assert_eq!(
            interaction.interaction_type,
            InteractionType::ApplicationCommand
        );
    }
}
