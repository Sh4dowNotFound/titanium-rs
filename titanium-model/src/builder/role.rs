use crate::TitanString;

/// Payload for creating a role.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateRole<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}

/// Builder for creating a Role.
#[derive(Debug, Clone, Default)]
pub struct CreateRoleBuilder<'a> {
    params: CreateRole<'a>,
}

impl<'a> CreateRoleBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<TitanString<'a>>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn color(mut self, color: u32) -> Self {
        self.params.color = Some(color);
        self
    }

    #[must_use]
    pub fn hoist(mut self, hoist: bool) -> Self {
        self.params.hoist = Some(hoist);
        self
    }

    pub fn icon(mut self, icon: impl Into<TitanString<'a>>) -> Self {
        self.params.icon = Some(icon.into());
        self
    }

    pub fn unicode_emoji(mut self, emoji: impl Into<TitanString<'a>>) -> Self {
        self.params.unicode_emoji = Some(emoji.into());
        self
    }

    #[must_use]
    pub fn mentionable(mut self, mentionable: bool) -> Self {
        self.params.mentionable = Some(mentionable);
        self
    }

    #[must_use]
    pub fn build(self) -> CreateRole<'a> {
        self.params
    }
}
