use crate::{Component, CreateMessage, Embed, TitanString};

/// Builder for creating a Message.
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct MessageBuilder<'a> {
    message: CreateMessage<'a>,
}

impl<'a> MessageBuilder<'a> {
    /// Create a new `MessageBuilder`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a message with just text content.
    #[inline]
    pub fn text(content: impl Into<TitanString<'a>>) -> Self {
        Self::new().content(content)
    }

    /// Set the content of the message.
    #[inline]
    pub fn content(mut self, content: impl Into<TitanString<'a>>) -> Self {
        self.message.content = Some(content.into());
        self
    }

    /// Enable/disable TTS.
    pub fn tts(mut self, tts: bool) -> Self {
        self.message.tts = Some(tts);
        self
    }

    /// Reply to a message (sets `message_reference`).
    pub fn reply(mut self, message_id: impl Into<crate::Snowflake>) -> Self {
        self.message.message_reference = Some(crate::MessageReference {
            message_id: Some(message_id.into()),
            channel_id: None,
            guild_id: None,
            fail_if_not_exists: Some(true),
        });
        self
    }

    /// Add an embed to the message.
    pub fn embed(mut self, embed: impl Into<Embed<'a>>) -> Self {
        if let Some(embeds) = &mut self.message.embeds {
            embeds.push(embed.into());
        } else {
            self.message.embeds = Some(vec![embed.into()]);
        }
        self
    }

    /// Add multiple embeds.
    pub fn embeds(mut self, embeds: Vec<Embed<'a>>) -> Self {
        if let Some(existing) = &mut self.message.embeds {
            existing.extend(embeds);
        } else {
            self.message.embeds = Some(embeds);
        }
        self
    }

    /// Add a component (`ActionRow`, etc.) to the message.
    pub fn component(mut self, component: impl Into<Component<'a>>) -> Self {
        if let Some(components) = &mut self.message.components {
            components.push(component.into());
        } else {
            self.message.components = Some(vec![component.into()]);
        }
        self
    }

    /// Add a file to upload.
    pub fn add_file(
        mut self,
        filename: impl Into<TitanString<'a>>,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        self.message
            .files
            .push(crate::create_message::FileUpload::new(
                filename.into().into_owned(),
                data,
            ));
        self
    }

    /// Build the `CreateMessage` payload.
    #[must_use]
    pub fn build(self) -> CreateMessage<'a> {
        self.message
    }
}

/// `MessageBuilder` automatically converts to `CreateMessage`
impl<'a> From<MessageBuilder<'a>> for CreateMessage<'a> {
    #[inline]
    fn from(builder: MessageBuilder<'a>) -> Self {
        builder.build()
    }
}

/// String automatically converts to `CreateMessage`
impl From<String> for CreateMessage<'static> {
    fn from(content: String) -> Self {
        MessageBuilder::text(content).build()
    }
}

/// &str automatically converts to `CreateMessage`
impl<'a> From<&'a str> for CreateMessage<'a> {
    fn from(content: &'a str) -> Self {
        MessageBuilder::text(content).build()
    }
}
