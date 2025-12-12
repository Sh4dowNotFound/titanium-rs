//! Application Commands (Slash Commands).

use crate::snowflake::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// Application command structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommand {
    /// Unique ID of the command.
    #[serde(default)]
    pub id: Option<Snowflake>,
    /// Type of command.
    #[serde(default, rename = "type")]
    pub command_type: Option<CommandType>,
    /// Application ID.
    #[serde(default)]
    pub application_id: Option<Snowflake>,
    /// Guild ID (if guild-specific).
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// Name of the command (1-32 chars).
    pub name: String,
    /// Description (1-100 characters).
    pub description: String,
    /// Parameters for the command.
    #[serde(default)]
    pub options: Vec<CommandOption>,
    /// Default Permissions.
    #[serde(default)]
    pub default_member_permissions: Option<String>,
    /// Whether usable in DMs.
    #[serde(default)]
    pub dm_permission: Option<bool>,
    /// Whether unsafe for age-gated users.
    #[serde(default)]
    pub nsfw: bool,
    /// Version ID.
    #[serde(default)]
    pub version: Option<Snowflake>,
}

impl ApplicationCommand {
    /// Create a builder for an `ApplicationCommand`.
    pub fn builder<'a>(
        name: impl Into<TitanString<'a>>,
        description: impl Into<TitanString<'a>>,
    ) -> crate::builder::CommandBuilder<'a> {
        crate::builder::CommandBuilder::new(name, description)
    }
}

/// Type of Application Command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum CommandType {
    /// Slash Command (Chat Input).
    ChatInput = 1,
    /// User Context Menu.
    User = 2,
    /// Message Context Menu.
    Message = 3,
}

impl From<u8> for CommandType {
    fn from(value: u8) -> Self {
        match value {
            2 => CommandType::User,
            3 => CommandType::Message,
            _ => CommandType::ChatInput,
        }
    }
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        value as u8
    }
}

/// Option for a command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOption {
    /// Type of option.
    #[serde(rename = "type")]
    pub option_type: OptionType,
    /// Name (1-32 chars).
    pub name: String,
    /// Description (1-100 chars).
    pub description: String,
    /// Whether required.
    #[serde(default)]
    pub required: bool,
    /// Choices (for string/int/number).
    #[serde(default)]
    pub choices: Vec<CommandChoice>,
    /// Sub-options (for SubCommand/SubCommandGroup).
    #[serde(default)]
    pub options: Vec<CommandOption>,
    /// Channel types specific.
    #[serde(default)]
    pub channel_types: Vec<u8>,
    /// Min value (int/number).
    #[serde(default)]
    pub min_value: Option<crate::json::Value>,
    /// Max value (int/number).
    #[serde(default)]
    pub max_value: Option<crate::json::Value>,
    /// Min length (string).
    #[serde(default)]
    pub min_length: Option<u16>,
    /// Max length (string).
    #[serde(default)]
    pub max_length: Option<u16>,
    /// Whether to autocomplete.
    #[serde(default)]
    pub autocomplete: bool,
}

/// Type of command option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum OptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

impl From<u8> for OptionType {
    fn from(value: u8) -> Self {
        match value {
            1 => OptionType::SubCommand,
            2 => OptionType::SubCommandGroup,

            4 => OptionType::Integer,
            5 => OptionType::Boolean,
            6 => OptionType::User,
            7 => OptionType::Channel,
            8 => OptionType::Role,
            9 => OptionType::Mentionable,
            10 => OptionType::Number,
            11 => OptionType::Attachment,
            _ => OptionType::String,
        }
    }
}

impl From<OptionType> for u8 {
    fn from(value: OptionType) -> Self {
        value as u8
    }
}

/// Command Choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandChoice {
    /// Choice name.
    pub name: String,
    /// Choice value (string/int/double).
    pub value: crate::json::Value,
}
