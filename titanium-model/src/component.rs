//! Message Components (Buttons, Select Menus, etc.)

use crate::reaction::ReactionEmoji;
// Imports removed

use crate::TitanString;
use serde::{Deserialize, Serialize};

/// Top-level component type.
///
/// In messages, this is usually an `ActionRow`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Component<'a> {
    /// An action row containing other components.
    ActionRow(ActionRow<'a>),
    /// A button component (only valid inside ActionRow).
    Button(Button<'a>),
    /// A select menu (only valid inside ActionRow).
    SelectMenu(SelectMenu<'a>),
    /// A text input (only valid in modals).
    TextInput(TextInput<'a>),
}

/// The type of component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum ComponentType {
    /// Container for other components.
    ActionRow = 1,
    /// Button object.
    Button = 2,
    /// Select menu for picking text.
    StringSelect = 3,
    /// Text input object.
    TextInput = 4,
    /// Select menu for users.
    UserSelect = 5,
    /// Select menu for roles.
    RoleSelect = 6,
    /// Select menu for mentionables (users + roles).
    MentionableSelect = 7,
    /// Select menu for channels.
    ChannelSelect = 8,
}

impl From<u8> for ComponentType {
    fn from(value: u8) -> Self {
        match value {
            1 => ComponentType::ActionRow,
            2 => ComponentType::Button,
            3 => ComponentType::StringSelect,
            4 => ComponentType::TextInput,
            5 => ComponentType::UserSelect,
            6 => ComponentType::RoleSelect,
            7 => ComponentType::MentionableSelect,
            8 => ComponentType::ChannelSelect,
            _ => ComponentType::ActionRow, // Fallback
        }
    }
}

impl From<ComponentType> for u8 {
    fn from(value: ComponentType) -> Self {
        value as u8
    }
}

/// A container for other components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRow<'a> {
    /// Type 1 (ActionRow).
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// List of child components.
    pub components: Vec<Component<'a>>,
}

impl<'a> Default for ActionRow<'a> {
    fn default() -> Self {
        Self {
            component_type: ComponentType::ActionRow,
            components: Vec::new(),
        }
    }
}

/// A clickable button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button<'a> {
    /// Type 2 (Button).
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Style of the button.
    pub style: ButtonStyle,
    /// Text label (max 80 characters).
    #[serde(default)]
    pub label: Option<TitanString<'a>>,
    /// Emoji to display.
    #[serde(default)]
    pub emoji: Option<ReactionEmoji<'a>>,
    /// Custom ID (max 100 chars). Required for non-link buttons.
    #[serde(default)]
    pub custom_id: Option<TitanString<'a>>,
    /// URL for link buttons.
    #[serde(default)]
    pub url: Option<TitanString<'a>>,
    /// Whether the button is disabled.
    #[serde(default)]
    pub disabled: bool,
}

impl<'a> Button<'a> {
    /// Create a builder for a Button.
    pub fn builder(
        custom_id: impl Into<TitanString<'a>>,
        style: ButtonStyle,
    ) -> crate::builder::ButtonBuilder<'a> {
        crate::builder::ButtonBuilder::new()
            .custom_id(custom_id)
            .style(style)
    }

    /// Create a builder for a Link Button.
    pub fn builder_link(url: impl Into<TitanString<'a>>) -> crate::builder::ButtonBuilder<'a> {
        crate::builder::ButtonBuilder::new()
            .url(url)
            .style(ButtonStyle::Link)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum ButtonStyle {
    /// Blurple (Primary).
    Primary = 1,
    /// Grey (Secondary).
    Secondary = 2,
    /// Green (Success).
    Success = 3,
    /// Red (Danger).
    Danger = 4,
    /// Grey Link (Link).
    Link = 5,
}

impl From<u8> for ButtonStyle {
    fn from(value: u8) -> Self {
        match value {
            1 => ButtonStyle::Primary,
            2 => ButtonStyle::Secondary,
            3 => ButtonStyle::Success,
            4 => ButtonStyle::Danger,
            5 => ButtonStyle::Link,
            _ => ButtonStyle::Primary,
        }
    }
}

impl From<ButtonStyle> for u8 {
    fn from(value: ButtonStyle) -> Self {
        value as u8
    }
}

/// A menu for selecting items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectMenu<'a> {
    /// Type (3, 5, 6, 7, 8).
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Custom ID to identify the menu.
    pub custom_id: TitanString<'a>,
    /// Options (only for StringSelect).
    #[serde(default)]
    pub options: Vec<SelectOption<'a>>,
    /// Placeholder text.
    #[serde(default)]
    pub placeholder: Option<TitanString<'a>>,
    /// Minimum values to append.
    #[serde(default)]
    pub min_values: Option<u8>,
    /// Maximum values to append.
    #[serde(default)]
    pub max_values: Option<u8>,
    /// Whether the menu is disabled.
    #[serde(default)]
    pub disabled: bool,
}

impl<'a> SelectMenu<'a> {
    /// Create a builder for a SelectMenu.
    pub fn builder(custom_id: impl Into<TitanString<'a>>) -> crate::builder::SelectMenuBuilder<'a> {
        crate::builder::SelectMenuBuilder::new(custom_id)
    }
}

/// An option in a string select menu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption<'a> {
    /// The user-facing name of the option.
    pub label: TitanString<'a>,
    /// The dev-facing value of the option.
    pub value: TitanString<'a>,
    /// Additional description.
    #[serde(default)]
    pub description: Option<TitanString<'a>>,
    /// Emoji for the option.
    #[serde(default)]
    pub emoji: Option<ReactionEmoji<'a>>,
    /// Whether this option is selected by default.
    #[serde(default)]
    pub default: bool,
}

/// A text input field (Modals only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextInput<'a> {
    /// Type 4 (TextInput).
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Custom ID.
    pub custom_id: TitanString<'a>,
    /// Input style (Short vs Paragraph).
    pub style: TextInputStyle,
    /// Label for the input.
    pub label: TitanString<'a>,
    /// Minimum length.
    #[serde(default)]
    pub min_length: Option<u16>,
    /// Maximum length.
    #[serde(default)]
    pub max_length: Option<u16>,
    /// Whether required.
    #[serde(default)]
    pub required: Option<bool>,
    /// Pre-filled value.
    #[serde(default)]
    pub value: Option<TitanString<'a>>,
    /// Placeholder text.
    #[serde(default)]
    pub placeholder: Option<TitanString<'a>>,
}

/// Text Input Style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum TextInputStyle {
    /// Single-line input.
    Short = 1,
    /// Multi-line input.
    Paragraph = 2,
}

impl From<u8> for TextInputStyle {
    fn from(value: u8) -> Self {
        match value {
            1 => TextInputStyle::Short,
            2 => TextInputStyle::Paragraph,
            _ => TextInputStyle::Short,
        }
    }
}

impl From<TextInputStyle> for u8 {
    fn from(value: TextInputStyle) -> Self {
        value as u8
    }
}
