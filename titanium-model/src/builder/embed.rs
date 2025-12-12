use crate::{Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedMedia, TitanString};

/// Builder for creating an Embed.
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct EmbedBuilder<'a> {
    embed: Embed<'a>,
}

impl<'a> EmbedBuilder<'a> {
    /// Create a new `EmbedBuilder`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a simple embed with title and description.
    #[inline]
    pub fn simple(
        title: impl Into<TitanString<'a>>,
        description: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::new().title(title).description(description)
    }

    /// Create a success embed (green color).
    #[inline]
    pub fn success(
        title: impl Into<TitanString<'a>>,
        description: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::simple(title, description).color(0x0057_F287)
    }

    /// Create an error embed (red color).
    #[inline]
    pub fn error(
        title: impl Into<TitanString<'a>>,
        description: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::simple(title, description).color(0x00ED_4245)
    }

    /// Create an info embed (blurple color).
    #[inline]
    pub fn info(
        title: impl Into<TitanString<'a>>,
        description: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::simple(title, description).color(0x0058_65F2)
    }

    /// Create a warning embed (yellow color).
    #[inline]
    pub fn warning(
        title: impl Into<TitanString<'a>>,
        description: impl Into<TitanString<'a>>,
    ) -> Self {
        Self::simple(title, description).color(0x00FE_E75C)
    }

    /// Set the title of the embed.
    #[inline]
    pub fn title(mut self, title: impl Into<TitanString<'a>>) -> Self {
        self.embed.title = Some(title.into());
        self
    }

    /// Set the description of the embed.
    pub fn description(mut self, description: impl Into<TitanString<'a>>) -> Self {
        self.embed.description = Some(description.into());
        self
    }

    /// Set the URL of the embed.
    pub fn url(mut self, url: impl Into<TitanString<'a>>) -> Self {
        self.embed.url = Some(url.into());
        self
    }

    /// Set the timestamp of the embed.
    pub fn timestamp(mut self, timestamp: impl Into<TitanString<'a>>) -> Self {
        self.embed.timestamp = Some(timestamp.into());
        self
    }

    /// Set the color of the embed.
    pub fn color(mut self, color: u32) -> Self {
        self.embed.color = Some(color);
        self
    }

    /// Set the color of the embed from RGB values.
    pub fn color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.embed.color = Some((u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b));
        self
    }

    /// Set the footer of the embed.
    pub fn footer(
        mut self,
        text: impl Into<TitanString<'a>>,
        icon_url: Option<impl Into<TitanString<'a>>>,
    ) -> Self {
        self.embed.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: icon_url.map(Into::into),
            proxy_icon_url: None,
        });
        self
    }

    /// Set the image of the embed.
    pub fn image(mut self, url: impl Into<TitanString<'a>>) -> Self {
        self.embed.image = Some(EmbedMedia {
            url: Some(url.into()),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Set the thumbnail of the embed.
    pub fn thumbnail(mut self, url: impl Into<TitanString<'a>>) -> Self {
        self.embed.thumbnail = Some(EmbedMedia {
            url: Some(url.into()),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Set the author of the embed.
    pub fn author(
        mut self,
        name: impl Into<TitanString<'a>>,
        url: Option<impl Into<TitanString<'a>>>,
        icon_url: Option<impl Into<TitanString<'a>>>,
    ) -> Self {
        self.embed.author = Some(EmbedAuthor {
            name: name.into(),
            url: url.map(Into::into),
            icon_url: icon_url.map(Into::into),
            proxy_icon_url: None,
        });
        self
    }

    /// Add a field to the embed.
    pub fn field(
        mut self,
        name: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
        inline: bool,
    ) -> Self {
        self.embed.fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    /// Add an inline field.
    pub fn field_inline(
        self,
        name: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
    ) -> Self {
        self.field(name, value, true)
    }

    /// Add a block field (not inline).
    pub fn field_block(
        self,
        name: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
    ) -> Self {
        self.field(name, value, false)
    }

    /// Build the Embed.
    #[must_use]
    pub fn build(self) -> Embed<'a> {
        self.embed
    }
}

/// `EmbedBuilder` automatically converts to Embed
impl<'a> From<EmbedBuilder<'a>> for Embed<'a> {
    #[inline]
    fn from(builder: EmbedBuilder<'a>) -> Self {
        builder.build()
    }
}

/// &str automatically converts to Embed (simple text embed)
impl<'a> From<&'a str> for Embed<'a> {
    fn from(text: &'a str) -> Self {
        EmbedBuilder::new().description(text).build()
    }
}
