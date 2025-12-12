/// Payload for creating an emoji.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateEmoji {
    pub name: String,
    pub image: String, // Data URI
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<crate::Snowflake>,
}

/// Builder for creating an Emoji.
#[derive(Debug, Clone)]
pub struct CreateEmojiBuilder {
    params: CreateEmoji,
}

impl CreateEmojiBuilder {
    /// Create a new `CreateEmojiBuilder`.
    /// `image_data` should be a Data URI Scheme string (e.g. "data:image/jpeg;base64,...").
    pub fn new(name: impl Into<String>, image_data: impl Into<String>) -> Self {
        Self {
            params: CreateEmoji {
                name: name.into(),
                image: image_data.into(),
                roles: Vec::new(),
            },
        }
    }

    /// Add a role that can use this emoji.
    pub fn role(mut self, role_id: impl Into<crate::Snowflake>) -> Self {
        self.params.roles.push(role_id.into());
        self
    }

    /// Build the `CreateEmoji` payload.
    #[must_use]
    pub fn build(self) -> CreateEmoji {
        self.params
    }
}

/// Payload for modifying an emoji.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ModifyEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::Snowflake>>,
}

/// Builder for modifying an Emoji.
#[derive(Debug, Clone, Default)]
pub struct ModifyEmojiBuilder {
    params: ModifyEmoji,
}

impl ModifyEmojiBuilder {
    /// Create a new `ModifyEmojiBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    /// Set roles.
    #[must_use]
    pub fn roles(mut self, roles: Vec<crate::Snowflake>) -> Self {
        self.params.roles = Some(roles);
        self
    }

    /// Build the `ModifyEmoji` payload.
    #[must_use]
    pub fn build(self) -> ModifyEmoji {
        self.params
    }
}
