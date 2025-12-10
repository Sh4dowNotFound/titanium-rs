use crate::{Component, Embed};

use crate::TitanString;
use serde::{Deserialize, Serialize};

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

    /// Message flags (SUPPRESS_EMBEDS, SUPPRESS_NOTIFICATIONS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,

    /// Message reference (reply).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<crate::MessageReference>,

    /// Files to upload (not serialized to JSON, used by HTTP client).
    #[serde(skip)]
    pub files: Vec<crate::file::FileUpload>,
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
    pub fn embed(mut self, embed: Embed<'a>) -> Self {
        if let Some(embeds) = &mut self.embeds {
            embeds.push(embed);
        } else {
            self.embeds = Some(vec![embed]);
        }
        self
    }

    /// Add a component.
    pub fn component(mut self, component: Component<'a>) -> Self {
        if let Some(components) = &mut self.components {
            components.push(component);
        } else {
            self.components = Some(vec![component]);
        }
        self
    }
}
