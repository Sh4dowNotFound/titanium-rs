use crate::error::GatewayError;
use std::sync::Arc;
use titanium_model::voice::{
    VoiceChannelEffectSendEvent, VoiceServerUpdateEvent, VoiceStateUpdateEvent,
};
pub use titanium_model::*;

/// Parsed Gateway event ready for handler dispatch.
///
/// Events are parsed from Dispatch payloads (opcode 0). Each variant
/// corresponds to a Discord Gateway event type.
///
/// # Performance
///
/// Events are wrapped in `Arc` to allow cheap cloning when dispatching to multiple handlers
/// or storing in cache.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event<'a> {
    // =========================================================================
    // Connection Events
    // =========================================================================
    /// Received after successful Identify.
    Ready(Arc<ReadyEventData<'a>>),

    /// Received after successful Resume.
    Resumed,

    // =========================================================================
    // Guild Events
    // =========================================================================
    /// Lazy-load for unavailable guild, or guild joined.
    GuildCreate(Arc<Guild<'a>>),

    /// Guild was updated.
    GuildUpdate(Arc<Guild<'a>>),

    /// Guild was deleted or became unavailable.
    GuildDelete(UnavailableGuild),

    /// User was banned from guild.
    GuildBanAdd(Arc<GuildBanEvent<'a>>),

    /// User was unbanned from guild.
    GuildBanRemove(Arc<GuildBanEvent<'a>>),

    /// Guild emojis were updated.
    GuildEmojisUpdate(Arc<GuildEmojisUpdateEvent<'a>>),

    /// Guild stickers were updated.
    GuildStickersUpdate(Arc<GuildStickersUpdateEvent<'a>>),

    /// Guild integrations were updated.
    GuildIntegrationsUpdate(Arc<GuildIntegrationsUpdateEvent>),

    /// Audit log entry was created.
    GuildAuditLogEntryCreate(Arc<AuditLogEntry>),

    // =========================================================================
    // Guild Member Events
    // =========================================================================
    /// New member joined guild.
    GuildMemberAdd(Arc<GuildMemberAddEvent<'a>>),

    /// Member was removed from guild.
    GuildMemberRemove(Arc<GuildMemberRemoveEvent<'a>>),

    /// Member was updated.
    GuildMemberUpdate(Arc<GuildMemberUpdateEvent<'a>>),

    /// Response to Request Guild Members.
    GuildMembersChunk(Arc<GuildMembersChunkEvent<'a>>),

    // =========================================================================
    // Role Events
    // =========================================================================
    /// Role was created.
    GuildRoleCreate(Arc<GuildRoleEvent<'a>>),

    /// Role was updated.
    GuildRoleUpdate(Arc<GuildRoleEvent<'a>>),

    /// Role was deleted.
    GuildRoleDelete(Arc<GuildRoleDeleteEvent>),

    // =========================================================================
    // Channel Events
    // =========================================================================
    /// Channel was created.
    ChannelCreate(Arc<Channel<'a>>),

    /// Channel was updated.
    ChannelUpdate(Arc<Channel<'a>>),

    /// Channel was deleted.
    ChannelDelete(Arc<Channel<'a>>),

    /// Message was pinned/unpinned.
    ChannelPinsUpdate(Arc<ChannelPinsUpdateEvent>),

    // =========================================================================
    // Thread Events
    // =========================================================================
    /// Thread was created.
    ThreadCreate(Arc<Channel<'a>>),

    /// Thread was updated.
    ThreadUpdate(Arc<Channel<'a>>),

    /// Thread was deleted.
    ThreadDelete(Arc<ThreadDeleteEvent>),

    /// Sent when gaining access to a channel with threads.
    ThreadListSync(Arc<ThreadListSyncEvent<'a>>),

    /// Current user's thread member was updated.
    ThreadMemberUpdate(Arc<ThreadMemberUpdateEvent<'a>>),

    /// Users were added/removed from a thread.
    ThreadMembersUpdate(Arc<ThreadMembersUpdateEvent<'a>>),

    // =========================================================================
    // Message Events
    // =========================================================================
    /// Message was created.
    MessageCreate(Arc<Message<'a>>),

    /// Message was updated.
    MessageUpdate(Arc<MessageUpdateEvent<'a>>),

    /// Message was deleted.
    MessageDelete(MessageDeleteEvent),

    /// Multiple messages were deleted (bulk).
    MessageDeleteBulk(MessageDeleteBulkEvent),

    // =========================================================================
    // Reaction Events
    // =========================================================================
    /// Reaction was added to a message.
    MessageReactionAdd(Arc<MessageReactionAddEvent<'a>>),

    /// Reaction was removed from a message.
    MessageReactionRemove(Arc<MessageReactionRemoveEvent<'a>>),

    /// All reactions were removed from a message.
    MessageReactionRemoveAll(Arc<MessageReactionRemoveAllEvent>),

    /// All reactions for an emoji were removed.
    MessageReactionRemoveEmoji(Arc<MessageReactionRemoveEmojiEvent<'a>>),

    // =========================================================================
    // Interaction Events
    // =========================================================================
    /// An interaction was created (slash command, button, modal, etc.).
    InteractionCreate(Arc<Interaction<'a>>),

    // =========================================================================
    // Invite Events
    // =========================================================================
    /// Invite was created.
    InviteCreate(Arc<InviteCreateEvent<'a>>),

    /// Invite was deleted.
    InviteDelete(Arc<InviteDeleteEvent<'a>>),

    // =========================================================================
    // Stage Instance Events
    // =========================================================================
    /// Stage instance was created.
    StageInstanceCreate(Arc<StageInstance>),

    /// Stage instance was updated.
    StageInstanceUpdate(Arc<StageInstance>),

    /// Stage instance was deleted.
    StageInstanceDelete(Arc<StageInstance>),

    // =========================================================================
    // Scheduled Event Events
    // =========================================================================
    /// Scheduled event was created.
    GuildScheduledEventCreate(Arc<ScheduledEvent<'a>>),

    /// Scheduled event was updated.
    GuildScheduledEventUpdate(Arc<ScheduledEvent<'a>>),

    /// Scheduled event was deleted.
    GuildScheduledEventDelete(Arc<ScheduledEvent<'a>>),

    /// User subscribed to scheduled event.
    GuildScheduledEventUserAdd(Arc<ScheduledEventUserEvent>),

    /// User unsubscribed from scheduled event.
    GuildScheduledEventUserRemove(Arc<ScheduledEventUserEvent>),

    // =========================================================================
    // AutoMod Events
    // =========================================================================
    /// AutoMod rule was created.
    AutoModerationRuleCreate(Arc<AutoModRule>),

    /// AutoMod rule was updated.
    AutoModerationRuleUpdate(Arc<AutoModRule>),

    /// AutoMod rule was deleted.
    AutoModerationRuleDelete(Arc<AutoModRule>),

    /// AutoMod action was executed.
    AutoModerationActionExecution(Arc<AutoModActionExecution>),

    // =========================================================================
    // Integration Events
    // =========================================================================
    /// Integration was created.
    IntegrationCreate(Arc<Integration<'a>>),

    /// Integration was updated.
    IntegrationUpdate(Arc<Integration<'a>>),

    /// Integration was deleted.
    IntegrationDelete(Arc<IntegrationDeleteEvent>),

    // =========================================================================
    // Webhook Events
    // =========================================================================
    /// Webhooks were updated in a channel.
    WebhooksUpdate(Arc<WebhooksUpdateEvent>),

    // =========================================================================
    // Monetization Events
    // =========================================================================
    /// Entitlement was created.
    EntitlementCreate(Arc<Entitlement>),

    /// Entitlement was updated.
    EntitlementUpdate(Arc<Entitlement>),

    /// Entitlement was deleted.
    EntitlementDelete(Arc<Entitlement>),

    /// Subscription was created.
    SubscriptionCreate(Arc<Subscription>),

    /// Subscription was updated.
    SubscriptionUpdate(Arc<Subscription>),

    /// Subscription was deleted.
    SubscriptionDelete(Arc<Subscription>),

    // =========================================================================
    // Soundboard Events
    // =========================================================================
    /// Soundboard sound was created in a guild.
    SoundboardSoundCreate(Arc<SoundboardSound<'a>>),

    /// Soundboard sound was updated in a guild.
    SoundboardSoundUpdate(Arc<SoundboardSound<'a>>),

    /// Soundboard sound was deleted in a guild.
    SoundboardSoundDelete(Arc<SoundboardSoundDeleteEvent>),

    /// Multiple soundboard sounds were updated.
    SoundboardSoundsUpdate(Arc<SoundboardSoundsUpdateEvent<'a>>),

    /// Guild soundboard sounds were updated.
    GuildSoundboardSoundsUpdate(Arc<GuildSoundboardSoundsUpdateEvent<'a>>),

    // =========================================================================
    // Presence & Typing Events
    // =========================================================================
    /// User started typing.
    TypingStart(Arc<TypingStartEvent<'a>>),

    /// User's presence was updated.
    PresenceUpdate(Arc<PresenceUpdateEvent>),

    /// Current user was updated.
    UserUpdate(Arc<User<'a>>),

    // =========================================================================
    // Voice Events
    // =========================================================================
    /// Voice state was updated.
    VoiceStateUpdate(Arc<VoiceStateUpdateEvent<'a>>),

    /// Voice server information received.
    VoiceServerUpdate(VoiceServerUpdateEvent),

    /// Voice channel effect was sent.
    VoiceChannelEffectSend(Arc<VoiceChannelEffectSendEvent<'a>>),

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
/// unnecessary allocations. Events are wrapped in Arc.
/// Macro to implement the massive match statement for event parsing.
///
/// This deduplicates the logic between SIMD and non-SIMD builds.
/// $event_name: The variable holding the event name string.
/// $deser: A macro/callback to invoked to deserialize a specific type.
macro_rules! impl_parse_event {
    ($event_name:expr, $deser:ident) => {
        match $event_name {
            // Connection Events
            "READY" => {
                let ready: ReadyEventData = $deser!(ReadyEventData);
                Ok(Event::Ready(Arc::new(ready)))
            }
            "RESUMED" => Ok(Event::Resumed),

            // Guild Events
            "GUILD_CREATE" => {
                let guild: Guild = $deser!(Guild);
                Ok(Event::GuildCreate(Arc::new(guild)))
            }
            "GUILD_UPDATE" => {
                let guild: Guild = $deser!(Guild);
                Ok(Event::GuildUpdate(Arc::new(guild)))
            }
            "GUILD_DELETE" => {
                let guild: UnavailableGuild = $deser!(UnavailableGuild);
                Ok(Event::GuildDelete(guild))
            }
            "GUILD_BAN_ADD" => {
                let ban: GuildBanEvent = $deser!(GuildBanEvent);
                Ok(Event::GuildBanAdd(Arc::new(ban)))
            }
            "GUILD_BAN_REMOVE" => {
                let ban: GuildBanEvent = $deser!(GuildBanEvent);
                Ok(Event::GuildBanRemove(Arc::new(ban)))
            }
            "GUILD_EMOJIS_UPDATE" => {
                let emojis: GuildEmojisUpdateEvent = $deser!(GuildEmojisUpdateEvent);
                Ok(Event::GuildEmojisUpdate(Arc::new(emojis)))
            }
            "GUILD_STICKERS_UPDATE" => {
                let stickers: GuildStickersUpdateEvent = $deser!(GuildStickersUpdateEvent);
                Ok(Event::GuildStickersUpdate(Arc::new(stickers)))
            }
            "GUILD_INTEGRATIONS_UPDATE" => {
                let integrations: GuildIntegrationsUpdateEvent =
                    $deser!(GuildIntegrationsUpdateEvent);
                Ok(Event::GuildIntegrationsUpdate(Arc::new(integrations)))
            }
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => {
                let entry: AuditLogEntry = $deser!(AuditLogEntry);
                Ok(Event::GuildAuditLogEntryCreate(Arc::new(entry)))
            }

            // Guild Member Events
            "GUILD_MEMBER_ADD" => {
                let member: GuildMemberAddEvent = $deser!(GuildMemberAddEvent);
                Ok(Event::GuildMemberAdd(Arc::new(member)))
            }
            "GUILD_MEMBER_REMOVE" => {
                let member: GuildMemberRemoveEvent = $deser!(GuildMemberRemoveEvent);
                Ok(Event::GuildMemberRemove(Arc::new(member)))
            }
            "GUILD_MEMBER_UPDATE" => {
                let member: GuildMemberUpdateEvent = $deser!(GuildMemberUpdateEvent);
                Ok(Event::GuildMemberUpdate(Arc::new(member)))
            }
            "GUILD_MEMBERS_CHUNK" => {
                let chunk: GuildMembersChunkEvent = $deser!(GuildMembersChunkEvent);
                Ok(Event::GuildMembersChunk(Arc::new(chunk)))
            }

            // Role Events
            "GUILD_ROLE_CREATE" => {
                let role: GuildRoleEvent = $deser!(GuildRoleEvent);
                Ok(Event::GuildRoleCreate(Arc::new(role)))
            }
            "GUILD_ROLE_UPDATE" => {
                let role: GuildRoleEvent = $deser!(GuildRoleEvent);
                Ok(Event::GuildRoleUpdate(Arc::new(role)))
            }
            "GUILD_ROLE_DELETE" => {
                let role: GuildRoleDeleteEvent = $deser!(GuildRoleDeleteEvent);
                Ok(Event::GuildRoleDelete(Arc::new(role)))
            }

            // Channel Events
            "CHANNEL_CREATE" => {
                let channel: Channel = $deser!(Channel);
                Ok(Event::ChannelCreate(Arc::new(channel)))
            }
            "CHANNEL_UPDATE" => {
                let channel: Channel = $deser!(Channel);
                Ok(Event::ChannelUpdate(Arc::new(channel)))
            }
            "CHANNEL_DELETE" => {
                let channel: Channel = $deser!(Channel);
                Ok(Event::ChannelDelete(Arc::new(channel)))
            }
            "CHANNEL_PINS_UPDATE" => {
                let pins: ChannelPinsUpdateEvent = $deser!(ChannelPinsUpdateEvent);
                Ok(Event::ChannelPinsUpdate(Arc::new(pins)))
            }

            // Thread Events
            "THREAD_CREATE" => {
                let thread: Channel = $deser!(Channel);
                Ok(Event::ThreadCreate(Arc::new(thread)))
            }
            "THREAD_UPDATE" => {
                let thread: Channel = $deser!(Channel);
                Ok(Event::ThreadUpdate(Arc::new(thread)))
            }
            "THREAD_DELETE" => {
                let thread: ThreadDeleteEvent = $deser!(ThreadDeleteEvent);
                Ok(Event::ThreadDelete(Arc::new(thread)))
            }
            "THREAD_LIST_SYNC" => {
                let sync: ThreadListSyncEvent = $deser!(ThreadListSyncEvent);
                Ok(Event::ThreadListSync(Arc::new(sync)))
            }
            "THREAD_MEMBER_UPDATE" => {
                let member: ThreadMemberUpdateEvent = $deser!(ThreadMemberUpdateEvent);
                Ok(Event::ThreadMemberUpdate(Arc::new(member)))
            }
            "THREAD_MEMBERS_UPDATE" => {
                let members: ThreadMembersUpdateEvent = $deser!(ThreadMembersUpdateEvent);
                Ok(Event::ThreadMembersUpdate(Arc::new(members)))
            }

            // Message Events
            "MESSAGE_CREATE" => {
                let message: Message = $deser!(Message);
                Ok(Event::MessageCreate(Arc::new(message)))
            }
            "MESSAGE_UPDATE" => {
                let update: MessageUpdateEvent = $deser!(MessageUpdateEvent);
                Ok(Event::MessageUpdate(Arc::new(update)))
            }
            "MESSAGE_DELETE" => {
                let delete: MessageDeleteEvent = $deser!(MessageDeleteEvent);
                Ok(Event::MessageDelete(delete))
            }
            "MESSAGE_DELETE_BULK" => {
                let delete_bulk: MessageDeleteBulkEvent = $deser!(MessageDeleteBulkEvent);
                Ok(Event::MessageDeleteBulk(delete_bulk))
            }

            // Reaction Events
            "MESSAGE_REACTION_ADD" => {
                let reaction: MessageReactionAddEvent = $deser!(MessageReactionAddEvent);
                Ok(Event::MessageReactionAdd(Arc::new(reaction)))
            }
            "MESSAGE_REACTION_REMOVE" => {
                let reaction: MessageReactionRemoveEvent = $deser!(MessageReactionRemoveEvent);
                Ok(Event::MessageReactionRemove(Arc::new(reaction)))
            }
            "MESSAGE_REACTION_REMOVE_ALL" => {
                let reaction: MessageReactionRemoveAllEvent =
                    $deser!(MessageReactionRemoveAllEvent);
                Ok(Event::MessageReactionRemoveAll(Arc::new(reaction)))
            }
            "MESSAGE_REACTION_REMOVE_EMOJI" => {
                let reaction: MessageReactionRemoveEmojiEvent =
                    $deser!(MessageReactionRemoveEmojiEvent);
                Ok(Event::MessageReactionRemoveEmoji(Arc::new(reaction)))
            }

            // Interaction Events
            "INTERACTION_CREATE" => {
                let interaction: Interaction = $deser!(Interaction);
                Ok(Event::InteractionCreate(Arc::new(interaction)))
            }

            // Invite Events
            "INVITE_CREATE" => {
                let invite: InviteCreateEvent = $deser!(InviteCreateEvent);
                Ok(Event::InviteCreate(Arc::new(invite)))
            }
            "INVITE_DELETE" => {
                let invite: InviteDeleteEvent = $deser!(InviteDeleteEvent);
                Ok(Event::InviteDelete(Arc::new(invite)))
            }

            // Stage Instance Events
            "STAGE_INSTANCE_CREATE" => {
                let stage: StageInstance = $deser!(StageInstance);
                Ok(Event::StageInstanceCreate(Arc::new(stage)))
            }
            "STAGE_INSTANCE_UPDATE" => {
                let stage: StageInstance = $deser!(StageInstance);
                Ok(Event::StageInstanceUpdate(Arc::new(stage)))
            }
            "STAGE_INSTANCE_DELETE" => {
                let stage: StageInstance = $deser!(StageInstance);
                Ok(Event::StageInstanceDelete(Arc::new(stage)))
            }

            // Scheduled Event Events
            "GUILD_SCHEDULED_EVENT_CREATE" => {
                let event: ScheduledEvent = $deser!(ScheduledEvent);
                Ok(Event::GuildScheduledEventCreate(Arc::new(event)))
            }
            "GUILD_SCHEDULED_EVENT_UPDATE" => {
                let event: ScheduledEvent = $deser!(ScheduledEvent);
                Ok(Event::GuildScheduledEventUpdate(Arc::new(event)))
            }
            "GUILD_SCHEDULED_EVENT_DELETE" => {
                let event: ScheduledEvent = $deser!(ScheduledEvent);
                Ok(Event::GuildScheduledEventDelete(Arc::new(event)))
            }
            "GUILD_SCHEDULED_EVENT_USER_ADD" => {
                let event: ScheduledEventUserEvent = $deser!(ScheduledEventUserEvent);
                Ok(Event::GuildScheduledEventUserAdd(Arc::new(event)))
            }
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
                let event: ScheduledEventUserEvent = $deser!(ScheduledEventUserEvent);
                Ok(Event::GuildScheduledEventUserRemove(Arc::new(event)))
            }

            // AutoMod Events
            "AUTO_MODERATION_RULE_CREATE" => {
                let rule: AutoModRule = $deser!(AutoModRule);
                Ok(Event::AutoModerationRuleCreate(Arc::new(rule)))
            }
            "AUTO_MODERATION_RULE_UPDATE" => {
                let rule: AutoModRule = $deser!(AutoModRule);
                Ok(Event::AutoModerationRuleUpdate(Arc::new(rule)))
            }
            "AUTO_MODERATION_RULE_DELETE" => {
                let rule: AutoModRule = $deser!(AutoModRule);
                Ok(Event::AutoModerationRuleDelete(Arc::new(rule)))
            }
            "AUTO_MODERATION_ACTION_EXECUTION" => {
                let action: AutoModActionExecution = $deser!(AutoModActionExecution);
                Ok(Event::AutoModerationActionExecution(Arc::new(action)))
            }

            // Integration Events
            "INTEGRATION_CREATE" => {
                let integration: Integration = $deser!(Integration);
                Ok(Event::IntegrationCreate(Arc::new(integration)))
            }
            "INTEGRATION_UPDATE" => {
                let integration: Integration = $deser!(Integration);
                Ok(Event::IntegrationUpdate(Arc::new(integration)))
            }
            "INTEGRATION_DELETE" => {
                let integration: IntegrationDeleteEvent = $deser!(IntegrationDeleteEvent);
                Ok(Event::IntegrationDelete(Arc::new(integration)))
            }

            // Webhook Events
            "WEBHOOKS_UPDATE" => {
                let webhooks: WebhooksUpdateEvent = $deser!(WebhooksUpdateEvent);
                Ok(Event::WebhooksUpdate(Arc::new(webhooks)))
            }

            // Monetization Events
            "ENTITLEMENT_CREATE" => {
                let entitlement: Entitlement = $deser!(Entitlement);
                Ok(Event::EntitlementCreate(Arc::new(entitlement)))
            }
            "ENTITLEMENT_UPDATE" => {
                let entitlement: Entitlement = $deser!(Entitlement);
                Ok(Event::EntitlementUpdate(Arc::new(entitlement)))
            }
            "ENTITLEMENT_DELETE" => {
                let entitlement: Entitlement = $deser!(Entitlement);
                Ok(Event::EntitlementDelete(Arc::new(entitlement)))
            }
            "SUBSCRIPTION_CREATE" => {
                let subscription: Subscription = $deser!(Subscription);
                Ok(Event::SubscriptionCreate(Arc::new(subscription)))
            }
            "SUBSCRIPTION_UPDATE" => {
                let subscription: Subscription = $deser!(Subscription);
                Ok(Event::SubscriptionUpdate(Arc::new(subscription)))
            }
            "SUBSCRIPTION_DELETE" => {
                let subscription: Subscription = $deser!(Subscription);
                Ok(Event::SubscriptionDelete(Arc::new(subscription)))
            }

            // Soundboard Events
            "SOUNDBOARD_SOUND_CREATE" => {
                let sound: SoundboardSound = $deser!(SoundboardSound);
                Ok(Event::SoundboardSoundCreate(Arc::new(sound)))
            }
            "SOUNDBOARD_SOUND_UPDATE" => {
                let sound: SoundboardSound = $deser!(SoundboardSound);
                Ok(Event::SoundboardSoundUpdate(Arc::new(sound)))
            }
            "SOUNDBOARD_SOUND_DELETE" => {
                let sound: SoundboardSoundDeleteEvent = $deser!(SoundboardSoundDeleteEvent);
                Ok(Event::SoundboardSoundDelete(Arc::new(sound)))
            }
            "SOUNDBOARD_SOUNDS_UPDATE" => {
                let sounds: SoundboardSoundsUpdateEvent = $deser!(SoundboardSoundsUpdateEvent);
                Ok(Event::SoundboardSoundsUpdate(Arc::new(sounds)))
            }
            "GUILD_SOUNDBOARD_SOUNDS_UPDATE" => {
                let sounds: GuildSoundboardSoundsUpdateEvent =
                    $deser!(GuildSoundboardSoundsUpdateEvent);
                Ok(Event::GuildSoundboardSoundsUpdate(Arc::new(sounds)))
            }

            // Presence & Typing Events
            "TYPING_START" => {
                let typing: TypingStartEvent = $deser!(TypingStartEvent);
                Ok(Event::TypingStart(Arc::new(typing)))
            }
            "PRESENCE_UPDATE" => {
                let presence: PresenceUpdateEvent = $deser!(PresenceUpdateEvent);
                Ok(Event::PresenceUpdate(Arc::new(presence)))
            }
            "USER_UPDATE" => {
                let user: User = $deser!(User);
                Ok(Event::UserUpdate(Arc::new(user)))
            }

            // Voice Events
            "VOICE_STATE_UPDATE" => {
                let voice_state: VoiceStateUpdateEvent = $deser!(VoiceStateUpdateEvent);
                Ok(Event::VoiceStateUpdate(Arc::new(voice_state)))
            }
            "VOICE_SERVER_UPDATE" => {
                let voice_server: VoiceServerUpdateEvent = $deser!(VoiceServerUpdateEvent);
                Ok(Event::VoiceServerUpdate(voice_server))
            }
            "VOICE_CHANNEL_EFFECT_SEND" => {
                let effect: VoiceChannelEffectSendEvent = $deser!(VoiceChannelEffectSendEvent);
                Ok(Event::VoiceChannelEffectSend(Arc::new(effect)))
            }

            // Unknown event will be handled by the specific parser implementation
            // because they treat the data field differently (RawValue vs BorrowedValue)
            _ => {
                $deser!(UNKNOWN_VARIANT)
            }
        }
    };
}

/// Parse an event from its name and raw JSON data.
///
/// This function uses a match block for O(1) event dispatch without
/// unnecessary allocations. Events are wrapped in Arc.
#[cfg(not(feature = "simd"))]
pub fn parse_event(
    event_name: &str,
    data: &serde_json::value::RawValue,
) -> Result<Event<'static>, GatewayError> {
    let json_str = data.get();

    macro_rules! deser_standard {
        (UNKNOWN_VARIANT) => {{
            let value: serde_json::Value = serde_json::from_str(json_str)?;
            Ok(Event::Unknown {
                name: event_name.to_owned(),
                data: value,
            })
        }};
        ($T:ty) => {
            serde_json::from_str::<$T>(json_str)?
        };
    }

    impl_parse_event!(event_name, deser_standard)
}

/// Parse an event from its name and parsed JSON Value (SIMD).
#[cfg(feature = "simd")]
pub fn parse_event<'a>(
    event_name: &str,
    data: titanium_model::json::BorrowedValue<'a>,
) -> Result<Event<'static>, GatewayError> {
    macro_rules! deser_simd {
        (UNKNOWN_VARIANT) => {
            Ok(Event::Unknown {
                name: event_name.to_owned(),
                data: data.into(),
            })
        };
        ($T:ty) => {
            titanium_model::json::from_borrowed_value::<$T>(data.clone())?
        };
    }

    impl_parse_event!(event_name, deser_simd)
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
        // Since we are now using Arc, manual verification or construction might differ slightly
        // but for this test, we are just mocking the struct logic
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
