use crate::channel::{Channel, ChannelMention};
use crate::component::Component;
use crate::member::{Emoji, GuildMember};
use crate::snowflake::Snowflake;
use crate::user::User;
use crate::TitanString;
use serde::{Deserialize, Serialize};

// ============================================================================
// Poll Types (merged from poll.rs)
// ============================================================================

/// A poll object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Poll<'a> {
    /// The question of the poll.
    pub question: PollMedia<'a>,

    /// Each of the answers available in the poll.
    pub answers: Vec<PollAnswer<'a>>,

    /// The expiration date of the poll.
    #[serde(default)]
    pub expiry: Option<TitanString<'a>>,

    /// Whether the poll allows multiple selections.
    #[serde(default)]
    pub allow_multiselect: bool,

    /// The layout type of the poll.
    #[serde(default)]
    pub layout_type: Option<u8>,

    /// The results of the poll.
    #[serde(default)]
    pub results: Option<PollResults>,
}

/// A poll media object (question text/emoji).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollMedia<'a> {
    /// The text of the field.
    #[serde(default)]
    pub text: Option<TitanString<'a>>,

    /// The emoji of the field.
    #[serde(default)]
    pub emoji: Option<Emoji<'a>>,
}

/// A poll answer object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollAnswer<'a> {
    /// The ID of the answer.
    #[serde(default)]
    pub answer_id: Option<u32>,

    /// The data of the answer.
    pub poll_media: PollMedia<'a>,
}

/// The results of a poll.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollResults {
    /// Whether the poll is finalized.
    #[serde(default)]
    pub is_finalized: bool,

    /// The counts for each answer.
    #[serde(default)]
    pub answer_counts: Vec<PollAnswerCount>,
}

/// Count for a specific answer.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollAnswerCount {
    /// The ID of the answer.
    pub id: u32,

    /// The number of votes.
    pub count: u32,

    /// Whether the current user voted for this answer.
    #[serde(default)]
    pub me_voted: bool,
}

// ============================================================================
// Message
// ============================================================================

/// Discord Message representation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message<'a> {
    /// Message ID.
    pub id: Snowflake,
    /// Channel ID.
    pub channel_id: Snowflake,
    /// Author of the message.
    pub author: User<'a>,
    /// Message content.
    pub content: TitanString<'a>,
    /// When the message was sent (ISO8601 timestamp).
    pub timestamp: TitanString<'a>,
    /// When the message was edited (ISO8601 timestamp).
    #[serde(default)]
    pub edited_timestamp: Option<String>,
    /// Whether this was a TTS message.
    #[serde(default)]
    pub tts: bool,
    /// Whether this message mentions everyone.
    #[serde(default)]
    pub mention_everyone: bool,
    /// Users mentioned in this message.
    #[serde(default)]
    pub mentions: smallvec::SmallVec<[User<'a>; 4]>,
    /// Roles mentioned in this message.
    #[serde(default)]
    pub mention_roles: smallvec::SmallVec<[Snowflake; 4]>,
    /// Channels mentioned in this message.
    #[serde(default)]
    pub mention_channels: smallvec::SmallVec<[ChannelMention; 2]>,
    /// Attachments.
    #[serde(default)]
    pub attachments: smallvec::SmallVec<[Attachment<'a>; 4]>,
    /// Embeds.
    #[serde(default)]
    pub embeds: smallvec::SmallVec<[Embed<'a>; 10]>,
    /// Reactions.
    #[serde(default)]
    pub reactions: smallvec::SmallVec<[Reaction<'a>; 5]>,
    /// Used for validating a message was sent.
    #[serde(default)]
    pub nonce: Option<crate::json::Value>,
    /// Whether message is pinned.
    #[serde(default)]
    pub pinned: bool,
    /// Webhook ID if sent by webhook.
    #[serde(default)]
    pub webhook_id: Option<Snowflake>,
    /// Message type.
    #[serde(rename = "type")]
    pub message_type: u8,
    /// Guild ID.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// Member properties for this message's author.
    #[serde(default)]
    pub member: Option<GuildMember<'a>>,
    /// Message flags.
    #[serde(default)]
    pub flags: Option<u64>,
    /// Reference data for replies/forwards.
    #[serde(default)]
    pub message_reference: Option<MessageReference>,
    /// The message associated with the reference.
    #[serde(default)]
    pub referenced_message: Option<Box<Message<'a>>>,
    /// Thread started from this message.
    #[serde(default)]
    pub thread: Option<Channel<'a>>,
    /// Components (buttons, selects, etc.).
    #[serde(default)]
    pub components: smallvec::SmallVec<[Component<'a>; 5]>,
    /// Sticker items.
    #[serde(default)]
    pub sticker_items: smallvec::SmallVec<[StickerItem; 3]>,
    /// Poll object.
    #[serde(default)]
    pub poll: Option<Poll<'a>>,
}

/// An attachment on a message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment<'a> {
    /// Attachment ID.
    pub id: Snowflake,
    /// Filename.
    pub filename: TitanString<'a>,
    /// Description.
    #[serde(default)]
    pub description: Option<TitanString<'a>>,
    /// Media type.
    #[serde(default)]
    pub content_type: Option<String>,
    /// Size in bytes.
    pub size: u64,
    /// Source URL.
    pub url: TitanString<'a>,
    /// Proxy URL.
    pub proxy_url: TitanString<'a>,
    /// Height (if image).
    #[serde(default)]
    pub height: Option<u32>,
    /// Width (if image).
    #[serde(default)]
    pub width: Option<u32>,
    /// Whether ephemeral.
    #[serde(default)]
    pub ephemeral: bool,
    /// Duration in seconds (for voice messages).
    #[serde(default)]
    pub duration_secs: Option<f64>,
    /// Waveform base64 (for voice messages).
    #[serde(default)]
    pub waveform: Option<String>,
    /// Attachment flags.
    #[serde(default)]
    pub flags: Option<u64>,
}

/// An embed in a message.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Embed<'a> {
    /// Title.
    #[serde(default)]
    pub title: Option<TitanString<'a>>,
    /// Type.
    #[serde(default, rename = "type")]
    pub embed_type: Option<TitanString<'a>>,
    /// Description.
    #[serde(default)]
    pub description: Option<TitanString<'a>>,
    /// URL.
    #[serde(default)]
    pub url: Option<TitanString<'a>>,
    /// Timestamp.
    #[serde(default)]
    pub timestamp: Option<TitanString<'a>>,
    /// Color.
    #[serde(default)]
    pub color: Option<u32>,
    /// Footer.
    #[serde(default)]
    pub footer: Option<EmbedFooter<'a>>,
    /// Image.
    #[serde(default)]
    pub image: Option<EmbedMedia<'a>>,
    /// Thumbnail.
    #[serde(default)]
    pub thumbnail: Option<EmbedMedia<'a>>,
    /// Video.
    #[serde(default)]
    pub video: Option<EmbedMedia<'a>>,
    /// Provider.
    #[serde(default)]
    pub provider: Option<EmbedProvider<'a>>,
    /// Author.
    #[serde(default)]
    pub author: Option<EmbedAuthor<'a>>,
    /// Fields.
    #[serde(default)]
    pub fields: Vec<EmbedField<'a>>,
}

/// Embed footer.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedFooter<'a> {
    /// Footer text.
    pub text: TitanString<'a>,
    /// Icon URL.
    #[serde(default)]
    pub icon_url: Option<TitanString<'a>>,
    /// Proxy icon URL.
    #[serde(default)]
    pub proxy_icon_url: Option<TitanString<'a>>,
}

/// Embed media (image, thumbnail, video).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedMedia<'a> {
    /// URL.
    #[serde(default)]
    pub url: Option<TitanString<'a>>,
    /// Proxy URL.
    #[serde(default)]
    pub proxy_url: Option<TitanString<'a>>,
    /// Height.
    #[serde(default)]
    pub height: Option<u32>,
    /// Width.
    #[serde(default)]
    pub width: Option<u32>,
}

/// Embed provider.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedProvider<'a> {
    /// Provider name.
    #[serde(default)]
    pub name: Option<TitanString<'a>>,
    /// Provider URL.
    #[serde(default)]
    pub url: Option<TitanString<'a>>,
}

/// Embed author.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedAuthor<'a> {
    /// Author name.
    pub name: TitanString<'a>,
    /// Author URL.
    #[serde(default)]
    pub url: Option<TitanString<'a>>,
    /// Icon URL.
    #[serde(default)]
    pub icon_url: Option<TitanString<'a>>,
    /// Proxy icon URL.
    #[serde(default)]
    pub proxy_icon_url: Option<TitanString<'a>>,
}

/// Embed field.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedField<'a> {
    /// Field name.
    pub name: TitanString<'a>,
    /// Field value.
    pub value: TitanString<'a>,
    /// Whether inline.
    #[serde(default)]
    pub inline: bool,
}

/// A reaction on a message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reaction<'a> {
    /// Times this emoji has been used.
    pub count: u32,
    /// Reaction count details.
    #[serde(default)]
    pub count_details: Option<ReactionCountDetails>,
    /// Whether current user reacted.
    pub me: bool,
    /// Whether current user super-reacted.
    #[serde(default)]
    pub me_burst: bool,
    /// Emoji information.
    pub emoji: crate::reaction::ReactionEmoji<'a>,
    /// Colors for super-reaction.
    #[serde(default)]
    pub burst_colors: Vec<String>,
}

/// Reaction count details.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReactionCountDetails {
    /// Count of normal reactions.
    pub burst: u32,
    /// Count of super reactions.
    pub normal: u32,
}

/// Message reference for replies/forwards.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageReference {
    /// ID of the originating message.
    #[serde(default)]
    pub message_id: Option<Snowflake>,
    /// ID of the originating channel.
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    /// ID of the originating guild.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// Whether to fail if referenced message doesn't exist.
    #[serde(default)]
    pub fail_if_not_exists: Option<bool>,
}

/// A sticker item (partial sticker).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StickerItem {
    /// Sticker ID.
    pub id: Snowflake,
    /// Sticker name.
    pub name: String,
    /// Sticker format type.
    pub format_type: u8,
}

/// Message update event (partial message).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageUpdateEvent<'a> {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub author: Option<User<'a>>,
    #[serde(default)]
    pub edited_timestamp: Option<String>,
}

/// Message delete event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageDeleteEvent {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// Bulk message delete event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageDeleteBulkEvent {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// Typing start event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypingStartEvent<'a> {
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub timestamp: u64,
    #[serde(default)]
    pub member: Option<GuildMember<'a>>,
}
