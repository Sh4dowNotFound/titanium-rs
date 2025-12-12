/// Payload for executing a webhook.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ExecuteWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<crate::Embed<'static>>,
    // Note: files and components omitted for brevity, but can be added
}

/// Builder for executing a Webhook.
#[derive(Debug, Clone, Default)]
pub struct WebhookExecuteBuilder<'a> {
    params: ExecuteWebhook,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl WebhookExecuteBuilder<'_> {
    /// Create a new `WebhookExecuteBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set content.
    #[inline]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.params.content = Some(content.into());
        self
    }

    /// Set username override.
    #[inline]
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.params.username = Some(username.into());
        self
    }

    /// Set avatar URL override.
    #[inline]
    pub fn avatar_url(mut self, url: impl Into<String>) -> Self {
        self.params.avatar_url = Some(url.into());
        self
    }

    /// Set TTS.
    #[inline]
    #[must_use]
    pub fn tts(mut self, tts: bool) -> Self {
        self.params.tts = Some(tts);
        self
    }

    /// Add an embed.
    #[inline]
    pub fn embed(mut self, embed: impl Into<crate::Embed<'static>>) -> Self {
        self.params.embeds.push(embed.into());
        self
    }

    /// Add multiple embeds.
    #[inline]
    #[must_use]
    pub fn embeds(mut self, embeds: Vec<crate::Embed<'static>>) -> Self {
        self.params.embeds.extend(embeds);
        self
    }

    /// Build the payload.
    #[inline]
    #[must_use]
    pub fn build(self) -> ExecuteWebhook {
        self.params
    }
}
