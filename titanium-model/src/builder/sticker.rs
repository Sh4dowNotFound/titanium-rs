/// Payload for creating a sticker.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateSticker {
    pub name: String,
    pub description: String,
    pub tags: String,
}

/// Builder for creating a Sticker.
#[derive(Debug, Clone)]
pub struct CreateStickerBuilder {
    params: CreateSticker,
}

impl CreateStickerBuilder {
    /// Create a new `CreateStickerBuilder`.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        tags: impl Into<String>,
    ) -> Self {
        Self {
            params: CreateSticker {
                name: name.into(),
                description: description.into(),
                tags: tags.into(),
            },
        }
    }

    /// Build the `CreateSticker` payload.
    #[must_use]
    pub fn build(self) -> CreateSticker {
        self.params
    }
}
