use crate::TitanString;

/// Payload for creating a channel.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateChannel<'a> {
    pub name: TitanString<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<crate::json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

/// Builder for creating a Channel.
#[derive(Debug, Clone, Default)]
pub struct CreateChannelBuilder<'a> {
    params: CreateChannel<'a>,
}

impl<'a> CreateChannelBuilder<'a> {
    pub fn new(name: impl Into<TitanString<'a>>) -> Self {
        let mut builder = Self::default();
        builder.params.name = name.into();
        builder
    }

    #[must_use]
    pub fn kind(mut self, kind: u8) -> Self {
        self.params.kind = Some(kind);
        self
    }

    pub fn topic(mut self, topic: impl Into<TitanString<'a>>) -> Self {
        self.params.topic = Some(topic.into());
        self
    }

    #[must_use]
    pub fn build(self) -> CreateChannel<'a> {
        self.params
    }
}
