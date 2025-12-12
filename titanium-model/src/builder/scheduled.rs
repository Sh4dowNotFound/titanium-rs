use crate::TitanString;

/// Payload for creating a scheduled event.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateScheduledEvent<'a> {
    pub name: TitanString<'a>,
    pub privacy_level: crate::scheduled::ScheduledEventPrivacyLevel,
    pub scheduled_start_time: TitanString<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TitanString<'a>>,
    pub entity_type: crate::scheduled::ScheduledEventEntityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<crate::scheduled::ScheduledEventEntityMetadata<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<TitanString<'a>>, // Base64
}

/// Builder for creating a Scheduled Event.
#[derive(Debug, Clone)]
pub struct ScheduledEventBuilder<'a> {
    params: CreateScheduledEvent<'a>,
}

impl<'a> ScheduledEventBuilder<'a> {
    /// Create a new `ScheduledEventBuilder`.
    pub fn new(
        name: impl Into<TitanString<'a>>,
        start_time: impl Into<TitanString<'a>>,
        entity_type: crate::scheduled::ScheduledEventEntityType,
    ) -> Self {
        Self {
            params: CreateScheduledEvent {
                name: name.into(),
                scheduled_start_time: start_time.into(),
                entity_type,
                privacy_level: crate::scheduled::ScheduledEventPrivacyLevel::GuildOnly,
                ..Default::default()
            },
        }
    }

    /// Set description.
    #[inline]
    pub fn description(mut self, description: impl Into<TitanString<'a>>) -> Self {
        self.params.description = Some(description.into());
        self
    }

    /// Set end time.
    #[inline]
    pub fn end_time(mut self, time: impl Into<TitanString<'a>>) -> Self {
        self.params.scheduled_end_time = Some(time.into());
        self
    }

    /// Set channel ID (required for Stage/Voice events).
    #[inline]
    pub fn channel_id(mut self, id: impl Into<crate::Snowflake>) -> Self {
        self.params.channel_id = Some(id.into());
        self
    }

    /// Set location (required for External events).
    #[inline]
    pub fn location(mut self, location: impl Into<TitanString<'a>>) -> Self {
        self.params.entity_metadata = Some(crate::scheduled::ScheduledEventEntityMetadata {
            location: Some(location.into()),
        });
        self
    }

    /// Set cover image (base64).
    #[inline]
    pub fn image(mut self, image: impl Into<TitanString<'a>>) -> Self {
        self.params.image = Some(image.into());
        self
    }

    /// Build the payload.
    #[inline]
    #[must_use]
    pub fn build(self) -> CreateScheduledEvent<'a> {
        self.params
    }
}
