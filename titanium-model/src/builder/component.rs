use crate::TitanString;

/// Builder for creating a Button.
#[derive(Debug, Clone)]
#[must_use]
pub struct ButtonBuilder<'a> {
    inner: crate::component::Button<'a>,
}

impl Default for ButtonBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ButtonBuilder<'a> {
    /// Create a new `ButtonBuilder`.
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: crate::component::Button {
                style: crate::component::ButtonStyle::Primary,
                label: None,
                emoji: None,
                custom_id: None,
                url: None,
                disabled: false,
                component_type: crate::component::ComponentType::Button,
            },
        }
    }

    /// Create a primary (blurple) button.
    #[inline]
    pub fn primary(
        label: impl Into<TitanString<'a>>,
        custom_id: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::new()
            .style(crate::component::ButtonStyle::Primary)
            .label(label)
            .custom_id(custom_id)
    }

    /// Create a secondary (grey) button.
    #[inline]
    pub fn secondary(
        label: impl Into<TitanString<'a>>,
        custom_id: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::new()
            .style(crate::component::ButtonStyle::Secondary)
            .label(label)
            .custom_id(custom_id)
    }

    /// Create a success (green) button.
    #[inline]
    pub fn success(
        label: impl Into<TitanString<'a>>,
        custom_id: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::new()
            .style(crate::component::ButtonStyle::Success)
            .label(label)
            .custom_id(custom_id)
    }

    /// Create a danger (red) button.
    #[inline]
    pub fn danger(
        label: impl Into<TitanString<'a>>,
        custom_id: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::new()
            .style(crate::component::ButtonStyle::Danger)
            .label(label)
            .custom_id(custom_id)
    }

    /// Create a link button (opens URL).
    #[inline]
    pub fn link(label: impl Into<TitanString<'a>>, url: impl Into<TitanString<'a>>) -> Self {
        Self::new()
            .style(crate::component::ButtonStyle::Link)
            .label(label)
            .url(url)
    }

    /// Set style.
    pub fn style(mut self, style: crate::component::ButtonStyle) -> Self {
        self.inner.style = style;
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<TitanString<'a>>) -> Self {
        self.inner.label = Some(label.into());
        self
    }

    /// Set emoji.
    pub fn emoji(mut self, emoji: impl Into<crate::reaction::ReactionEmoji<'a>>) -> Self {
        self.inner.emoji = Some(emoji.into());
        self
    }

    /// Set custom ID.
    pub fn custom_id(mut self, id: impl Into<TitanString<'a>>) -> Self {
        self.inner.custom_id = Some(id.into());
        self
    }

    /// Set URL.
    pub fn url(mut self, url: impl Into<TitanString<'a>>) -> Self {
        self.inner.url = Some(url.into());
        self
    }

    /// Set disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner.disabled = disabled;
        self
    }

    /// Build the Component.
    #[must_use]
    pub fn build(self) -> crate::Component<'a> {
        crate::Component::Button(self.inner)
    }
}

/// Builder for creating a Select Menu.
#[derive(Debug, Clone)]
#[must_use]
pub struct SelectMenuBuilder<'a> {
    inner: crate::component::SelectMenu<'a>,
}

impl Default for SelectMenuBuilder<'_> {
    fn default() -> Self {
        Self::new("default_select")
    }
}

impl<'a> SelectMenuBuilder<'a> {
    /// Create a new `SelectMenuBuilder`.
    #[inline]
    pub fn new(custom_id: impl Into<TitanString<'a>>) -> Self {
        Self {
            inner: crate::component::SelectMenu {
                custom_id: custom_id.into(),
                options: Vec::with_capacity(25), // Discord max options
                placeholder: None,
                min_values: None,
                max_values: None,
                disabled: false,
                component_type: crate::component::ComponentType::StringSelect, // Default
            },
        }
    }

    /// Add an option.
    pub fn option(
        mut self,
        label: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
    ) -> Self {
        self.inner.options.push(crate::component::SelectOption {
            label: label.into(),
            value: value.into(),
            description: None,
            emoji: None,
            default: false,
        });
        self
    }

    /// Set placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<TitanString<'a>>) -> Self {
        self.inner.placeholder = Some(placeholder.into());
        self
    }

    /// Set min values.
    pub fn min_values(mut self, min: u8) -> Self {
        self.inner.min_values = Some(min);
        self
    }

    /// Set max values.
    pub fn max_values(mut self, max: u8) -> Self {
        self.inner.max_values = Some(max);
        self
    }

    /// Set disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner.disabled = disabled;
        self
    }

    /// Build the Component.
    #[must_use]
    pub fn build(self) -> crate::Component<'a> {
        crate::Component::SelectMenu(self.inner)
    }
}

/// Builder for creating an Action Row.
#[derive(Debug, Clone)]
#[must_use]
pub struct ActionRowBuilder<'a> {
    inner: crate::component::ActionRow<'a>,
}

impl Default for ActionRowBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ActionRowBuilder<'a> {
    pub fn new() -> Self {
        Self {
            inner: crate::component::ActionRow {
                components: Vec::with_capacity(5), // Discord max components per row
                component_type: crate::component::ComponentType::ActionRow,
            },
        }
    }

    pub fn add_button(mut self, button: ButtonBuilder<'a>) -> Self {
        self.inner.components.push(button.build());
        self
    }

    pub fn add_select_menu(mut self, menu: SelectMenuBuilder<'a>) -> Self {
        self.inner.components.push(menu.build());
        self
    }

    #[must_use]
    pub fn build(self) -> crate::Component<'a> {
        crate::Component::ActionRow(self.inner)
    }

    pub fn input(mut self, input: TextInputBuilder<'a>) -> Self {
        self.inner.components.push(input.build());
        self
    }
}

/// Builder for creating a Text Input.
#[derive(Debug, Clone)]
#[must_use]
pub struct TextInputBuilder<'a> {
    inner: crate::component::TextInput<'a>,
}

impl<'a> TextInputBuilder<'a> {
    pub fn new(
        custom_id: impl Into<TitanString<'a>>,
        style: crate::component::TextInputStyle,
        label: impl Into<TitanString<'a>>,
    ) -> Self {
        Self {
            inner: crate::component::TextInput {
                component_type: crate::component::ComponentType::TextInput,
                custom_id: custom_id.into(),
                style,
                label: label.into(),
                min_length: None,
                max_length: None,
                required: None,
                value: None,
                placeholder: None,
            },
        }
    }

    pub fn min_length(mut self, min: u16) -> Self {
        self.inner.min_length = Some(min);
        self
    }

    pub fn max_length(mut self, max: u16) -> Self {
        self.inner.max_length = Some(max);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.inner.required = Some(required);
        self
    }

    pub fn value(mut self, value: impl Into<TitanString<'a>>) -> Self {
        self.inner.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TitanString<'a>>) -> Self {
        self.inner.placeholder = Some(placeholder.into());
        self
    }

    #[must_use]
    pub fn build(self) -> crate::Component<'a> {
        crate::Component::TextInput(self.inner)
    }
}

/// `ButtonBuilder` automatically converts to Component
impl<'a> From<ButtonBuilder<'a>> for crate::Component<'a> {
    #[inline]
    fn from(builder: ButtonBuilder<'a>) -> Self {
        builder.build()
    }
}

/// `SelectMenuBuilder` automatically converts to Component
impl<'a> From<SelectMenuBuilder<'a>> for crate::Component<'a> {
    #[inline]
    fn from(builder: SelectMenuBuilder<'a>) -> Self {
        builder.build()
    }
}

/// `ActionRowBuilder` automatically converts to Component
impl<'a> From<ActionRowBuilder<'a>> for crate::Component<'a> {
    #[inline]
    fn from(builder: ActionRowBuilder<'a>) -> Self {
        builder.build()
    }
}
