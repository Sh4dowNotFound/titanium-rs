/// Payload for creating a stage instance.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateStageInstance {
    pub channel_id: crate::Snowflake,
    pub topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<crate::voice::StagePrivacyLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_start_notification: Option<bool>,
}

/// Builder for creating a Stage Instance.
#[derive(Debug, Clone)]
pub struct StageInstanceBuilder {
    params: CreateStageInstance,
}

impl StageInstanceBuilder {
    /// Create a new `StageInstanceBuilder`.
    pub fn new(channel_id: impl Into<crate::Snowflake>, topic: impl Into<String>) -> Self {
        Self {
            params: CreateStageInstance {
                channel_id: channel_id.into(),
                topic: topic.into(),
                privacy_level: None,
                send_start_notification: None,
            },
        }
    }

    /// Set privacy level.
    #[inline]
    #[must_use]
    pub fn privacy_level(mut self, level: crate::voice::StagePrivacyLevel) -> Self {
        self.params.privacy_level = Some(level);
        self
    }

    /// Set send start notification.
    #[inline]
    #[must_use]
    pub fn send_start_notification(mut self, send: bool) -> Self {
        self.params.send_start_notification = Some(send);
        self
    }

    /// Build the `CreateStageInstance` payload.
    #[inline]
    #[must_use]
    pub fn build(self) -> CreateStageInstance {
        self.params
    }
}
