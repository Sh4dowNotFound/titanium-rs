use crate::builder::component::ActionRowBuilder;
use crate::TitanString;

/// Builder for creating a Modal.
#[derive(Debug, Clone)]
#[must_use]
pub struct ModalBuilder<'a> {
    custom_id: TitanString<'a>,
    title: TitanString<'a>,
    components: Vec<crate::Component<'a>>,
}

impl<'a> ModalBuilder<'a> {
    pub fn new(custom_id: impl Into<TitanString<'a>>, title: impl Into<TitanString<'a>>) -> Self {
        Self {
            custom_id: custom_id.into(),
            title: title.into(),
            components: Vec::new(),
        }
    }

    pub fn row(mut self, row: ActionRowBuilder<'a>) -> Self {
        self.components.push(row.build());
        self
    }

    #[must_use]
    pub fn build(self) -> crate::interaction::Modal<'a> {
        crate::interaction::Modal {
            custom_id: self.custom_id,
            title: self.title,
            components: self.components,
        }
    }
}

/// Builder for Interaction Response.
#[derive(Debug, Clone)]
#[must_use]
pub struct InteractionResponseBuilder<'a> {
    response: crate::interaction::InteractionResponse<'a>,
}

impl Default for InteractionResponseBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> InteractionResponseBuilder<'a> {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            response: crate::interaction::InteractionResponse {
                response_type:
                    crate::interaction::InteractionCallbackType::ChannelMessageWithSource,
                data: Some(Default::default()),
            },
        }
    }

    pub fn kind(mut self, kind: crate::interaction::InteractionCallbackType) -> Self {
        self.response.response_type = kind;
        self
    }

    pub fn content(mut self, content: impl Into<TitanString<'a>>) -> Self {
        if let Some(data) = &mut self.response.data {
            data.content = Some(content.into());
        }
        self
    }

    pub fn embed(mut self, embed: impl Into<crate::Embed<'a>>) -> Self {
        if self.response.data.is_none() {
            self.response.data = Some(Default::default());
        }
        if let Some(data) = &mut self.response.data {
            data.embeds.push(embed.into());
        }
        self
    }

    pub fn component(mut self, component: impl Into<crate::Component<'a>>) -> Self {
        if self.response.data.is_none() {
            self.response.data = Some(Default::default());
        }
        if let Some(data) = &mut self.response.data {
            data.components.push(component.into());
        }
        self
    }

    pub fn components(mut self, components: Vec<crate::Component<'a>>) -> Self {
        if self.response.data.is_none() {
            self.response.data = Some(Default::default());
        }
        if let Some(data) = &mut self.response.data {
            data.components.extend(components);
        }
        self
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        if ephemeral {
            if self.response.data.is_none() {
                self.response.data = Some(Default::default());
            }
            if let Some(data) = &mut self.response.data {
                data.flags = Some(64);
            }
        }
        self
    }

    /// Sets response type to DeferredChannelMessageWithSource (5)
    pub fn deferred(mut self, ephemeral: bool) -> Self {
        self.response.response_type =
            crate::interaction::InteractionCallbackType::DeferredChannelMessageWithSource;
        if ephemeral {
            self = self.ephemeral(true);
        }
        self
    }

    /// Sets response type to UpdateMessage (7) - for component interactions
    pub fn update_message(mut self) -> Self {
        self.response.response_type = crate::interaction::InteractionCallbackType::UpdateMessage;
        self
    }

    #[must_use]
    pub fn build(self) -> crate::interaction::InteractionResponse<'a> {
        self.response
    }

    pub fn modal(mut self, modal: crate::interaction::Modal<'a>) -> Self {
        self.response.response_type = crate::interaction::InteractionCallbackType::Modal;
        self.response.data = Some(crate::interaction::InteractionCallbackData {
            custom_id: Some(modal.custom_id),
            title: Some(modal.title),
            components: modal.components,
            ..Default::default()
        });
        self
    }
}

impl<'a> From<InteractionResponseBuilder<'a>> for crate::interaction::InteractionResponse<'a> {
    fn from(builder: InteractionResponseBuilder<'a>) -> Self {
        builder.build()
    }
}
