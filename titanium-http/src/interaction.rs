use crate::error::HttpError;
use crate::HttpClient;
use serde::Serialize;
use titanium_model::{InteractionResponse, Message, Snowflake};

impl HttpClient {
    /// Create a response to an Interaction.
    ///
    /// This is the initial response to an interaction (Slash Command, Button, etc.).
    /// You must respond within 3 seconds, or use `InteractionCallbackType::DeferredChannelMessageWithSource`.
    pub async fn create_interaction_response(
        &self,
        interaction_id: Snowflake,
        token: &str,
        response: &InteractionResponse<'_>,
    ) -> Result<(), HttpError> {
        let route = format!("/interactions/{}/{}/callback", interaction_id, token);
        self.post(&route, response).await
    }

    /// Get the original response message.
    pub async fn get_original_interaction_response(
        &self,
        application_id: Snowflake,
        token: &str,
    ) -> Result<Message<'static>, HttpError> {
        let route = format!("/webhooks/{}/{}/messages/@original", application_id, token);
        self.get(&route).await
    }

    /// Edit the original response message.
    ///
    /// This is used to update the message after a DEFER response.
    pub async fn edit_original_interaction_response<B: Serialize>(
        &self,
        application_id: Snowflake,
        token: &str,
        body: B,
    ) -> Result<Message<'static>, HttpError> {
        let route = format!("/webhooks/{}/{}/messages/@original", application_id, token);
        self.patch(&route, body).await
    }

    /// Delete the original response message.
    pub async fn delete_original_interaction_response(
        &self,
        application_id: Snowflake,
        token: &str,
    ) -> Result<(), HttpError> {
        let route = format!("/webhooks/{}/{}/messages/@original", application_id, token);
        self.delete(&route).await
    }

    /// Create a followup message.
    ///
    /// Used to send additional messages after the initial response.
    pub async fn create_followup_message<B: Serialize>(
        &self,
        application_id: Snowflake,
        token: &str,
        body: B,
    ) -> Result<Message<'static>, HttpError> {
        let route = format!("/webhooks/{}/{}", application_id, token);
        // "wait=true" ensures we get the Message object back
        self.post_with_query(&route, body, &[("wait", "true")])
            .await
    }
}
