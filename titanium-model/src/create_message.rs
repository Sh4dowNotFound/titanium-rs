use crate::{Component, Embed};

use crate::TitanString;
use serde::{Deserialize, Serialize};

// ============================================================================
// File Upload (merged from file.rs)
// ============================================================================

/// A file to be uploaded to Discord.
#[derive(Debug, Clone)]
pub struct FileUpload {
    /// The name of the file.
    pub filename: String,
    /// The binary data of the file.
    pub data: Vec<u8>,
}

impl FileUpload {
    /// Create a new `FileUpload`.
    pub fn new(filename: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self {
            filename: filename.into(),
            data: data.into(),
        }
    }
}

// ============================================================================
// Create Message
// ============================================================================

/// Payload to create a message.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMessage<'a> {
    /// Message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TitanString<'a>>,

    /// Pass true if sending a TTS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,

    /// Embeds to include.
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub embeds: Option<Vec<Embed<'a>>>,

    /// Components to include.
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub components: Option<Vec<Component<'a>>>,

    /// Message flags (`SUPPRESS_EMBEDS`, `SUPPRESS_NOTIFICATIONS`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,

    /// Message reference (reply).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<crate::MessageReference>,

    /// Files to upload (not serialized to JSON, used by HTTP client).
    #[serde(skip)]
    pub files: Vec<FileUpload>,
}

impl<'a> CreateMessage<'a> {
    /// Create a simple text message.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into().into()),
            ..Default::default()
        }
    }

    /// Add an embed.
    #[must_use]
    pub fn embed(mut self, embed: Embed<'a>) -> Self {
        if let Some(embeds) = &mut self.embeds {
            embeds.push(embed);
        } else {
            self.embeds = Some(vec![embed]);
        }
        self
    }

    /// Add a component.
    #[must_use]
    pub fn component(mut self, component: Component<'a>) -> Self {
        if let Some(components) = &mut self.components {
            components.push(component);
        } else {
            self.components = Some(vec![component]);
        }
        self
    }
}
