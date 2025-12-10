use crate::error::HttpError;
use crate::HttpClient;
use serde::Serialize;
use titanium_model::{Channel, CreateMessage, Message, Snowflake};

impl HttpClient {
    // =========================================================================
    // Channel Operations
    // =========================================================================

    /// Get a channel.
    pub async fn get_channel(&self, channel_id: Snowflake) -> Result<Channel<'static>, HttpError> {
        let route = format!("/channels/{}", channel_id);
        self.get(&route).await
    }

    /// Delete/Close a channel.
    pub async fn delete_channel(
        &self,
        channel_id: Snowflake,
        reason: Option<&str>,
    ) -> Result<Channel<'static>, HttpError> {
        let route = format!("/channels/{}", channel_id);

        let headers = reason.map(|r| {
            let mut h = reqwest::header::HeaderMap::new();
            h.insert(
                "X-Audit-Log-Reason",
                reqwest::header::HeaderValue::from_str(r).unwrap(),
            );
            h
        });

        self.delete_with_headers(&route, headers).await
    }

    /// Create a new channel in a guild.
    pub async fn create_channel(
        &self,
        guild_id: Snowflake,
        params: &titanium_model::builder::CreateChannel<'_>,
    ) -> Result<Channel<'static>, HttpError> {
        let route = format!("/guilds/{}/channels", guild_id);
        self.post(&route, params).await
    }

    // =========================================================================
    // Message Operations
    // =========================================================================

    /// Get a single message.
    pub async fn get_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
    ) -> Result<Message<'static>, HttpError> {
        let route = format!("/channels/{}/messages/{}", channel_id, message_id);
        self.get(&route).await
    }

    /// Send a message to a channel.
    ///
    /// Uses `titanium_model::CreateMessage` for type-safe building.
    pub async fn create_message_struct(
        &self,
        channel_id: Snowflake,
        message: &CreateMessage<'_>,
    ) -> Result<Message<'static>, HttpError> {
        let route = format!("/channels/{}/messages", channel_id);
        self.post(&route, message).await
    }

    /// Send a simple text message (helper).
    pub async fn send_message(
        &self,
        channel_id: Snowflake,
        content: impl Into<String>,
    ) -> Result<Message<'static>, HttpError> {
        #[derive(Serialize)]
        struct SimpleMessage {
            content: String,
        }

        let route = format!("/channels/{}/messages", channel_id);
        self.post(
            &route,
            SimpleMessage {
                content: content.into(),
            },
        )
        .await
    }

    /// Edit a message.
    pub async fn edit_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        content: impl Into<String>,
    ) -> Result<Message<'static>, HttpError> {
        #[derive(Serialize)]
        struct EditMessage {
            content: String,
        }

        let route = format!("/channels/{}/messages/{}", channel_id, message_id);
        self.patch(
            &route,
            EditMessage {
                content: content.into(),
            },
        )
        .await
    }

    /// Delete a message.
    pub async fn delete_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        reason: Option<&str>,
    ) -> Result<(), HttpError> {
        let route = format!("/channels/{}/messages/{}", channel_id, message_id);

        let headers = reason.map(|r| {
            let mut h = reqwest::header::HeaderMap::new();
            h.insert(
                "X-Audit-Log-Reason",
                reqwest::header::HeaderValue::from_str(r).unwrap(),
            );
            h
        });

        self.delete_with_headers(&route, headers).await
    }

    /// Bulk delete messages.
    ///
    /// `messages`: List of message IDs to delete (2-100).
    pub async fn delete_messages_bulk(
        &self,
        channel_id: Snowflake,
        messages: Vec<Snowflake>,
    ) -> Result<(), HttpError> {
        #[derive(Serialize)]
        struct BulkDelete {
            messages: Vec<Snowflake>,
        }

        let route = format!("/channels/{}/messages/bulk-delete", channel_id);
        self.post(&route, BulkDelete { messages }).await
    }
}
