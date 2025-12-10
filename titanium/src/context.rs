use std::sync::Arc;
use titanium_http::HttpClient;
use titanium_model::{
    Embed, Interaction, InteractionCallbackData, InteractionCallbackType, InteractionResponse,
    Message, Snowflake, User,
};
use tokio::sync::RwLock;

/// Context for a Slash Command execution.
#[derive(Clone)]
pub struct Context {
    pub http: Arc<HttpClient>,
    pub cache: Arc<titanium_cache::InMemoryCache>,
    pub interaction: Arc<Interaction<'static>>,
    /// Whether the interaction has been deferred or replied to.
    pub has_responded: Arc<RwLock<bool>>,
}

impl Context {
    pub fn new(
        http: Arc<HttpClient>,
        cache: Arc<titanium_cache::InMemoryCache>,
        interaction: Interaction<'static>,
    ) -> Self {
        Self {
            http,
            cache,
            interaction: Arc::new(interaction),
            has_responded: Arc::new(RwLock::new(false)),
        }
    }

    /// Defer the interaction.
    ///
    /// This sends a "Thinking..." state (Opcode 5) to discord.
    /// You must call this within 3 seconds if your task takes long.
    pub async fn defer(
        &self,
        ephemeral: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut responded = self.has_responded.write().await;
        if *responded {
            return Ok(()); // Already accepted
        }

        let response = InteractionResponse {
            response_type: InteractionCallbackType::DeferredChannelMessageWithSource,
            data: Some(InteractionCallbackData {
                flags: if ephemeral { Some(64) } else { None }, // 64 = Ephemeral
                content: None,
                tts: false,
                embeds: vec![],
                allowed_mentions: None,
                components: vec![],
                attachments: vec![],
                choices: None,
            }),
        };

        self.http
            .create_interaction_response(self.interaction.id, &self.interaction.token, &response)
            .await?;

        *responded = true;
        Ok(())
    }

    /// Reply to the command.
    ///
    /// This smarter method checks if we have deferred.
    /// If NOT deferred -> calls `create_interaction_response`
    /// If DEFERRED -> calls `edit_original_interaction_response`
    ///
    /// This solves the "3 second rule" complexity for the user!
    pub async fn reply(
        &self,
        content: impl Into<String>,
    ) -> Result<Message<'static>, Box<dyn std::error::Error + Send + Sync>> {
        let content = content.into();
        let mut responded = self.has_responded.write().await;

        if *responded {
            // We already deferred (or replied), so we must EDIT the original
            // Note: technically if we replied, we should create followup.
            // But for simple defer -> reply flow, this is Edit.
            let app_id = self.interaction.application_id;
            let token = &self.interaction.token;

            // Simple struct for body
            #[derive(serde::Serialize)]
            struct EditBody {
                content: String,
            }

            let message = self
                .http
                .edit_original_interaction_response(app_id, token, EditBody { content })
                .await?;

            Ok(message)
        } else {
            // Initial response
            let response = InteractionResponse {
                response_type: InteractionCallbackType::ChannelMessageWithSource,
                data: Some(InteractionCallbackData {
                    content: Some(content.clone().into()),
                    tts: false,
                    embeds: vec![],
                    allowed_mentions: None,
                    flags: None,
                    components: vec![],
                    attachments: vec![],
                    choices: None,
                }),
            };

            self.http
                .create_interaction_response(
                    self.interaction.id,
                    &self.interaction.token,
                    &response,
                )
                .await?;

            *responded = true;

            // Fetch the interaction response to return a full Message object.
            // This is required because create_interaction_response returns 204 No Content.
            let msg = self
                .http
                .get_original_interaction_response(
                    self.interaction.application_id,
                    &self.interaction.token,
                )
                .await?;
            Ok(msg)
        }
    }

    /// Reply with an embed (discord.js-style).
    pub async fn reply_embed(
        &self,
        embed: impl Into<Embed<'static>>,
    ) -> Result<Message<'static>, Box<dyn std::error::Error + Send + Sync>> {
        let embed = embed.into();
        let mut responded = self.has_responded.write().await;

        if *responded {
            // Edit original response
            #[derive(serde::Serialize)]
            struct EditBody {
                embeds: Vec<Embed<'static>>,
            }

            let message = self
                .http
                .edit_original_interaction_response(
                    self.interaction.application_id,
                    &self.interaction.token,
                    EditBody {
                        embeds: vec![embed],
                    },
                )
                .await?;
            Ok(message)
        } else {
            let response = InteractionResponse {
                response_type: InteractionCallbackType::ChannelMessageWithSource,
                data: Some(InteractionCallbackData {
                    embeds: vec![embed],
                    content: None,
                    tts: false,
                    allowed_mentions: None,
                    flags: None,
                    components: vec![],
                    attachments: vec![],
                    choices: None,
                }),
            };

            self.http
                .create_interaction_response(
                    self.interaction.id,
                    &self.interaction.token,
                    &response,
                )
                .await?;

            *responded = true;

            // Fetch the interaction response
            let msg = self
                .http
                .get_original_interaction_response(
                    self.interaction.application_id,
                    &self.interaction.token,
                )
                .await?;
            Ok(msg)
        }
    }

    /// Reply with an ephemeral message (only visible to user).
    pub async fn reply_ephemeral(
        &self,
        content: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let content = content.into();
        let mut responded = self.has_responded.write().await;

        if *responded {
            // Can't make an existing response ephemeral if it wasn't already.
            // But we can try to send a followup invisible message?
            // Actually, followup flags work.
            // For now, simpler error or fallback
            return Err("Cannot reply ephemeral after already responding".into());
        }

        let response = InteractionResponse {
            response_type: InteractionCallbackType::ChannelMessageWithSource,
            data: Some(InteractionCallbackData {
                content: Some(content.into()),
                flags: Some(64), // EPHEMERAL
                tts: false,
                embeds: vec![],
                allowed_mentions: None,
                components: vec![],
                attachments: vec![],
                choices: None,
            }),
        };

        self.http
            .create_interaction_response(self.interaction.id, &self.interaction.token, &response)
            .await?;

        *responded = true;
        Ok(())
    }

    /// Edit the original interaction response.
    pub async fn edit_reply(
        &self,
        content: impl Into<String>,
    ) -> Result<Message<'static>, Box<dyn std::error::Error + Send + Sync>> {
        #[derive(serde::Serialize)]
        struct EditBody {
            content: String,
        }

        let message = self
            .http
            .edit_original_interaction_response(
                self.interaction.application_id,
                &self.interaction.token,
                EditBody {
                    content: content.into(),
                },
            )
            .await?;

        Ok(message)
    }

    /// Send a follow-up message.
    ///
    /// This allows sending multiple messages for one interaction.
    /// Note: Followups are actually Webhook executions.
    pub async fn followup(
        &self,
        content: impl Into<String>,
    ) -> Result<Message<'static>, Box<dyn std::error::Error + Send + Sync>> {
        use titanium_model::builder::ExecuteWebhook;

        let params = ExecuteWebhook {
            content: Some(content.into()),
            ..Default::default()
        };

        // Interaction followups use the webhook endpoint with application_id as webhook_id
        let msg = self
            .http
            .execute_webhook(
                self.interaction.application_id,
                &self.interaction.token,
                &params,
            )
            .await?;

        // execute_webhook returns Option<Message>, but with wait=true it should return Some
        Ok(msg.ok_or("Failed to get followup message")?)
    }

    /// Get the user who triggered the interaction.
    #[inline]
    pub fn user(&self) -> Option<&User<'static>> {
        self.interaction
            .member
            .as_ref()
            .and_then(|m| m.user.as_ref())
            .or(self.interaction.user.as_ref())
    }

    /// Get the guild ID if in a guild.
    #[inline]
    pub fn guild_id(&self) -> Option<Snowflake> {
        self.interaction.guild_id
    }

    /// Get the channel ID.
    #[inline]
    pub fn channel_id(&self) -> Option<Snowflake> {
        self.interaction.channel_id
    }
}
