use crate::error::HttpError;
use crate::HttpClient;
use serde::Serialize;
use titanium_model::{Channel, CreateMessage, Message, Snowflake, Webhook};

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

        let headers = reason
            .map(|r| -> Result<_, HttpError> {
                let mut h = reqwest::header::HeaderMap::new();
                h.insert(
                    "X-Audit-Log-Reason",
                    reqwest::header::HeaderValue::from_str(r)?,
                );
                Ok(h)
            })
            .transpose()?;

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
    pub async fn create_message_struct(
        &self,
        channel_id: Snowflake,
        message: &CreateMessage<'_>,
    ) -> Result<Message<'static>, HttpError> {
        let route = format!("/channels/{}/messages", channel_id);
        self.post(&route, message).await
    }

    /// Send a simple text message.
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

        let headers = reason
            .map(|r| -> Result<_, HttpError> {
                let mut h = reqwest::header::HeaderMap::new();
                h.insert(
                    "X-Audit-Log-Reason",
                    reqwest::header::HeaderValue::from_str(r)?,
                );
                Ok(h)
            })
            .transpose()?;

        self.delete_with_headers(&route, headers).await
    }

    /// Bulk delete messages (2-100 messages, not older than 14 days).
    /// This is much faster than deleting one by one.
    pub async fn bulk_delete_messages(
        &self,
        channel_id: Snowflake,
        message_ids: &[Snowflake],
    ) -> Result<(), HttpError> {
        if message_ids.len() < 2 || message_ids.len() > 100 {
            return Err(HttpError::ClientError(
                "bulk_delete_messages requires 2-100 message IDs".into(),
            ));
        }

        #[derive(Serialize)]
        struct BulkDelete<'a> {
            messages: &'a [Snowflake],
        }

        let route = format!("/channels/{}/messages/bulk-delete", channel_id);
        self.post_no_response(
            &route,
            BulkDelete {
                messages: message_ids,
            },
        )
        .await
    }

    // =========================================================================
    // Webhook Operations (merged from webhook.rs)
    // =========================================================================

    /// Create a webhook.
    pub async fn create_webhook(
        &self,
        channel_id: Snowflake,
        name: &str,
        avatar: Option<&str>,
    ) -> Result<Webhook<'static>, HttpError> {
        #[derive(Serialize)]
        struct CreateWebhook<'a> {
            name: &'a str,
            avatar: Option<&'a str>,
        }

        let route = format!("/channels/{}/webhooks", channel_id);
        self.post(&route, CreateWebhook { name, avatar }).await
    }

    /// Get channel webhooks.
    pub async fn get_channel_webhooks(
        &self,
        channel_id: Snowflake,
    ) -> Result<Vec<Webhook<'static>>, HttpError> {
        let route = format!("/channels/{}/webhooks", channel_id);
        self.get(&route).await
    }

    /// Get guild webhooks.
    pub async fn get_guild_webhooks(
        &self,
        guild_id: Snowflake,
    ) -> Result<Vec<Webhook<'static>>, HttpError> {
        let route = format!("/guilds/{}/webhooks", guild_id);
        self.get(&route).await
    }

    /// Execute webhook (send message).
    pub async fn execute_webhook(
        &self,
        webhook_id: Snowflake,
        webhook_token: &str,
        params: &titanium_model::builder::ExecuteWebhook,
    ) -> Result<Option<Message<'static>>, HttpError> {
        let route = format!("/webhooks/{}/{}", webhook_id, webhook_token);
        self.post_with_query(&route, params, &[("wait", "true")])
            .await
    }
}
