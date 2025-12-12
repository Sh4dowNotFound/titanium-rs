use crate::TitanString;

/// Payload for starting a thread.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartThread<'a> {
    pub name: TitanString<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<u8>, // For Start Thread without Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
}

/// Builder for starting a Thread.
#[derive(Debug, Clone)]
#[must_use]
pub struct StartThreadBuilder<'a> {
    params: StartThread<'a>,
}

impl<'a> StartThreadBuilder<'a> {
    /// Create a new `StartThreadBuilder`.
    pub fn new(name: impl Into<TitanString<'a>>) -> Self {
        Self {
            params: StartThread {
                name: name.into(),
                ..Default::default()
            },
        }
    }

    /// Set auto archive duration (60, 1440, 4320, 10080).
    pub fn auto_archive_duration(mut self, duration: u32) -> Self {
        self.params.auto_archive_duration = Some(duration);
        self
    }

    /// Set thread type (for standalone threads).
    pub fn kind(mut self, kind: u8) -> Self {
        self.params.type_ = Some(kind);
        self
    }

    /// Set invitable (private threads).
    pub fn invitable(mut self, invitable: bool) -> Self {
        self.params.invitable = Some(invitable);
        self
    }

    /// Set rate limit per user.
    pub fn rate_limit_per_user(mut self, limit: u32) -> Self {
        self.params.rate_limit_per_user = Some(limit);
        self
    }

    /// Build the `StartThread` payload.
    #[must_use]
    pub fn build(self) -> StartThread<'a> {
        self.params
    }
}
