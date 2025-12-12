/// Payload for creating an invite.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateInvite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<crate::Snowflake>,
}

/// Builder for creating an Invite.
#[derive(Debug, Clone, Default)]
pub struct CreateInviteBuilder {
    params: CreateInvite,
}

impl CreateInviteBuilder {
    /// Create a new `CreateInviteBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set max age in seconds (0 = never expire).
    #[must_use]
    pub fn max_age(mut self, seconds: u32) -> Self {
        self.params.max_age = Some(seconds);
        self
    }

    /// Set max uses (0 = unlimited).
    #[must_use]
    pub fn max_uses(mut self, uses: u32) -> Self {
        self.params.max_uses = Some(uses);
        self
    }

    /// Set temporary (kick after disconnect).
    #[must_use]
    pub fn temporary(mut self, temp: bool) -> Self {
        self.params.temporary = Some(temp);
        self
    }

    /// Set unique (don't reuse similar invite).
    #[must_use]
    pub fn unique(mut self, unique: bool) -> Self {
        self.params.unique = Some(unique);
        self
    }

    /// Build the `CreateInvite` payload.
    #[must_use]
    pub fn build(self) -> CreateInvite {
        self.params
    }
}
