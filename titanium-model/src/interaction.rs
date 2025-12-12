//! Interactions (Slash Commands, Components, Modals).

use crate::command::CommandType;
use crate::component::ComponentType;
use crate::member::GuildMember;
use crate::snowflake::Snowflake;
use crate::Message;
use crate::TitanString;
use crate::User;
use serde::{Deserialize, Serialize};

/// Incoming Interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction<'a> {
    /// Interaction ID.
    pub id: Snowflake,
    /// Application ID.
    pub application_id: Snowflake,
    /// Type of interaction.
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    /// Data payload.
    #[serde(default)]
    pub data: Option<InteractionData<'a>>,
    /// Guild ID.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// Channel ID.
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    /// Member (if in guild).
    #[serde(default)]
    pub member: Option<GuildMember<'a>>,
    /// User (if in DM).
    #[serde(default)]
    pub user: Option<User<'a>>,
    /// Token for responding.
    pub token: String,
    /// Protocol version.
    pub version: u8,
    /// Message (primary for components).
    #[serde(default)]
    pub message: Option<Box<Message<'a>>>,
    /// Locale.
    #[serde(default)]
    pub locale: Option<TitanString<'a>>,
    /// Guild locale.
    #[serde(default)]
    pub guild_locale: Option<TitanString<'a>>,
}

/// Interaction Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum InteractionType {
    /// Ping (HTTP only).
    Ping = 1,
    /// Application Command (Slash Command).
    ApplicationCommand = 2,
    /// Component (Button/Select).
    MessageComponent = 3,
    /// Autocomplete.
    ApplicationCommandAutocomplete = 4,
    /// Modal Submit.
    ModalSubmit = 5,
}

impl From<u8> for InteractionType {
    fn from(value: u8) -> Self {
        match value {
            1 => InteractionType::Ping,

            3 => InteractionType::MessageComponent,
            4 => InteractionType::ApplicationCommandAutocomplete,
            5 => InteractionType::ModalSubmit,
            _ => InteractionType::ApplicationCommand,
        }
    }
}

impl From<InteractionType> for u8 {
    fn from(value: InteractionType) -> Self {
        value as u8
    }
}

/// Data payload for an interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionData<'a> {
    /// ID of the invoked command/component.
    #[serde(default)]
    pub id: Option<Snowflake>,
    /// Name of the invoked command.
    #[serde(default)]
    pub name: Option<TitanString<'a>>,
    /// Command type.
    #[serde(default, rename = "type")]
    pub command_type: Option<CommandType>,
    /// Resolved data (users/roles/channels).
    #[serde(default)]
    pub resolved: Option<ResolvedData<'a>>,
    /// Parameters/Options.
    #[serde(default)]
    pub options: Vec<InteractionDataOption<'a>>,
    /// Custom ID (for components/modals).
    #[serde(default)]
    pub custom_id: Option<TitanString<'a>>,
    /// Component type (for components).
    #[serde(default)]
    pub component_type: Option<ComponentType>,
    /// Selected values (for select menus).
    #[serde(default)]
    pub values: Vec<String>,
    /// Target ID (for context menus).
    #[serde(default)]
    pub target_id: Option<Snowflake>,
    /// Components (for modals).
    #[serde(default)]
    pub components: Vec<crate::json::Value>,
}

/// Resolved/Hydrated data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedData<'a> {
    /// Users map.
    #[serde(default)]
    pub users: ahash::AHashMap<Snowflake, User<'a>>,
    /// Members map.
    #[serde(default)]
    pub members: ahash::AHashMap<Snowflake, GuildMember<'a>>,
    /// Roles map.
    #[serde(default)]
    pub roles: ahash::AHashMap<Snowflake, crate::member::Role<'a>>,
    /// Channels map.
    #[serde(default)]
    pub channels: ahash::AHashMap<Snowflake, crate::Channel<'a>>,
    /// Messages map.
    #[serde(default)]
    pub messages: ahash::AHashMap<Snowflake, Message<'a>>,
    /// Attachments map.
    #[serde(default)]
    pub attachments: ahash::AHashMap<Snowflake, crate::Attachment<'a>>,
}

/// Option received in Interaction Data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionDataOption<'a> {
    /// Name of the parameter.
    pub name: TitanString<'a>,
    /// Value type.
    #[serde(rename = "type")]
    pub option_type: crate::command::OptionType,
    /// Value (can be string, int, double, bool).
    #[serde(default)]
    pub value: Option<crate::json::Value>,
    /// Sub-options.
    #[serde(default)]
    pub options: Vec<InteractionDataOption<'a>>,
    /// Whether focused (autocomplete).
    #[serde(default)]
    pub focused: bool,
}

/// Response to an interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResponse<'a> {
    /// Response type.
    #[serde(rename = "type")]
    pub response_type: InteractionCallbackType,
    /// Response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData<'a>>,
}

impl<'a> InteractionResponse<'a> {
    /// Create a builder for an `InteractionResponse`.
    pub fn builder() -> crate::builder::InteractionResponseBuilder<'a> {
        crate::builder::InteractionResponseBuilder::new()
    }

    /// Shortcut to create a simple reply.
    pub fn reply(
        content: impl Into<TitanString<'a>>,
    ) -> crate::builder::InteractionResponseBuilder<'a> {
        Self::builder().content(content)
    }

    /// Shortcut for a deferred response (think "loading...").
    pub fn deferred() -> crate::builder::InteractionResponseBuilder<'a> {
        Self::builder().kind(InteractionCallbackType::DeferredChannelMessageWithSource)
    }
}

/// Callback Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum InteractionCallbackType {
    /// Pong (HTTP).
    Pong = 1,
    /// Channel Message With Source.
    ChannelMessageWithSource = 4,
    /// Deferred Channel Message With Source.
    DeferredChannelMessageWithSource = 5,
    /// Deferred Update Message (Components).
    DeferredUpdateMessage = 6,
    /// Update Message (Components).
    UpdateMessage = 7,
    /// Autocomplete Result.
    ApplicationCommandAutocompleteResult = 8,
    /// Modal.
    Modal = 9,
}

impl From<u8> for InteractionCallbackType {
    fn from(value: u8) -> Self {
        match value {
            1 => InteractionCallbackType::Pong,

            5 => InteractionCallbackType::DeferredChannelMessageWithSource,
            6 => InteractionCallbackType::DeferredUpdateMessage,
            7 => InteractionCallbackType::UpdateMessage,
            8 => InteractionCallbackType::ApplicationCommandAutocompleteResult,
            9 => InteractionCallbackType::Modal,
            _ => InteractionCallbackType::ChannelMessageWithSource,
        }
    }
}

impl From<InteractionCallbackType> for u8 {
    fn from(value: InteractionCallbackType) -> Self {
        value as u8
    }
}

/// Callback Data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionCallbackData<'a> {
    /// TTS.
    #[serde(default)]
    pub tts: bool,
    /// Message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TitanString<'a>>,
    /// Embeds.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<crate::Embed<'a>>,
    /// Allowed mentions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<crate::json::Value>,
    /// Message flags (e.g. EPHEMERAL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// Components.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<crate::Component<'a>>,
    /// Autocomplete choices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<InteractionDataOption<'a>>>,
    /// Attachments.
    #[serde(default)]
    pub attachments: Vec<crate::Attachment<'a>>,
    /// Custom ID (for Modals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<TitanString<'a>>,
    /// Title (for Modals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TitanString<'a>>,
}

/// Modal Payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modal<'a> {
    pub custom_id: TitanString<'a>,
    pub title: TitanString<'a>,
    pub components: Vec<crate::Component<'a>>,
}
