use crate::error::HttpError;
use crate::HttpClient;
use serde::Serialize;
use titanium_model::{Message, Snowflake, Webhook};

impl HttpClient {
    /// Create a webhook.
    pub async fn create_webhook(
        &self,
        channel_id: Snowflake,
        name: &str,
        avatar: Option<&str>, // Data URI
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
    ///
    /// Note: Webhooks can be executed without a bot token if usage via `id/token` url.
    /// This method complies with the library structure using the client's auth or just payload.
    pub async fn execute_webhook(
        &self,
        webhook_id: Snowflake,
        webhook_token: &str,
        params: &titanium_model::builder::ExecuteWebhook,
    ) -> Result<Option<Message<'static>>, HttpError> {
        let route = format!("/webhooks/{}/{}", webhook_id, webhook_token);
        // wait=true to get message back
        self.post_with_query(&route, params, &[("wait", "true")])
            .await
    }
}

// Params struct removed in favor of titanium_model::builder::ExecuteWebhook
