//! Context module for Discord interaction handling.
//!
//! This module provides the [`Context`] struct which is passed to event handlers
//! and provides convenient methods for responding to Discord interactions.
//!
//! # Features
//!
//! - **Automatic defer/reply handling**: The context tracks whether you've already
//!   responded to an interaction and automatically uses the correct Discord API
//!   (initial response vs. edit).
//!
//! - **Thread-safe**: Uses `AtomicBool` for response tracking, avoiding locks during
//!   async HTTP operations.
//!
//! - **JS/discord.py-like ergonomics**: Methods like `reply()`, `reply_embed()`,
//!   `success()`, `error()` for quick responses.
//!
//! # Example
//!
//! ```no_run
//! use titanium_rs::prelude::*;
//!
//! async fn handle_command(ctx: Context) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     // For long operations, defer first
//!     ctx.defer(false).await?;
//!     
//!     // Do some work...
//!     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//!     
//!     // Then reply (automatically uses edit since we deferred)
//!     ctx.reply("Done!").await?;
//!     Ok(())
//! }
//! ```

use crate::error::{ContextError, TitaniumError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use titanium_gateway::Shard;
use titanium_http::HttpClient;
use titanium_model::{Embed, Interaction, Message, Snowflake, User};

/// Context for Discord interaction handling.
///
/// This struct is passed to event handlers and provides access to:
/// - The HTTP client for making API requests
/// - The cache for looking up cached entities
/// - The shard that received the event
/// - The interaction data (for slash commands, buttons, etc.)
///
/// # Thread Safety
///
/// `Context` is `Clone` and can be safely shared between tasks. The response
/// tracking uses `AtomicBool` to avoid holding locks during async operations.
///
/// # Response Flow
///
/// Discord interactions must receive a response within 3 seconds. If your
/// operation takes longer, use [`Context::defer`] first, then [`Context::reply`].
/// The context automatically tracks this and uses the correct API endpoint.
#[derive(Clone)]
pub struct Context {
    /// HTTP client for making Discord API requests.
    pub http: Arc<HttpClient>,
    /// In-memory cache for guilds, channels, users, etc.
    pub cache: Arc<titanium_cache::InMemoryCache>,
    /// The shard that received this event.
    pub shard: Arc<Shard>,
    /// The interaction data (if this context is for an interaction).
    pub interaction: Option<Arc<Interaction<'static>>>,
    /// Whether the interaction has been deferred or replied to.
    /// Uses AtomicBool to avoid holding locks during async HTTP calls.
    has_responded: Arc<AtomicBool>,
}

impl Context {
    pub fn new(
        http: Arc<HttpClient>,
        cache: Arc<titanium_cache::InMemoryCache>,
        shard: Arc<Shard>,
        interaction: Option<Interaction<'static>>,
    ) -> Self {
        Self {
            http,
            cache,
            shard,
            interaction: interaction.map(Arc::new),
            has_responded: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Defer the interaction response.
    ///
    /// This sends a "Thinking..." state to Discord, giving you up to 15 minutes
    /// to send the actual response. You must call this within 3 seconds if your
    /// operation takes longer.
    ///
    /// # Arguments
    ///
    /// * `ephemeral` - If true, the "Thinking..." and subsequent reply will only
    ///   be visible to the user who invoked the command.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use titanium_rs::prelude::*;
    /// # async fn example(ctx: Context) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// // Defer for a long operation
    /// ctx.defer(false).await?;
    ///
    /// // Do expensive work...
    /// tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    ///
    /// // Reply (automatically edits the deferred message)
    /// ctx.reply("Done!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn defer(&self, ephemeral: bool) -> Result<(), TitaniumError> {
        // Use compare_exchange to atomically check and set - no lock held during HTTP call
        if self
            .has_responded
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Ok(()); // Already responded
        }

        let interaction = self
            .interaction
            .as_ref()
            .ok_or(ContextError::NoInteraction)?;

        let response = titanium_model::builder::InteractionResponseBuilder::new()
            .deferred(ephemeral)
            .build();

        // If HTTP fails, we should reset the flag, but for defer this is acceptable
        // since Discord will timeout the interaction anyway
        self.http
            .create_interaction_response(interaction.id, &interaction.token, &response)
            .await?;

        Ok(())
    }

    /// Reply to the command.
    ///
    /// This smarter method checks if we have deferred.
    /// If NOT deferred -> calls `create_interaction_response`
    /// If DEFERRED -> calls `edit_original_interaction_response`
    ///
    /// This solves the "3 second rule" complexity for the user!
    /// Reply to the interaction or message.
    ///
    /// # Errors
    /// Returns `TitaniumError` if the HTTP request fails.
    pub async fn reply(
        &self,
        content: impl Into<String>,
    ) -> Result<Message<'static>, TitaniumError> {
        let content = content.into();
        let interaction = self
            .interaction
            .as_ref()
            .ok_or(ContextError::NoInteraction)?;

        // Check if already responded (atomically)
        if self.has_responded.load(Ordering::SeqCst) {
            // We already deferred (or replied), so we must EDIT the original
            #[derive(serde::Serialize)]
            struct EditBody {
                content: String,
            }

            let message = self
                .http
                .edit_original_interaction_response(
                    interaction.application_id,
                    &interaction.token,
                    EditBody { content },
                )
                .await?;

            Ok(message)
        } else {
            // Initial response - try to claim the first response
            if self
                .has_responded
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_err()
            {
                // Lost race, someone else responded first - use edit instead
                #[derive(serde::Serialize)]
                struct EditBody {
                    content: String,
                }
                let message = self
                    .http
                    .edit_original_interaction_response(
                        interaction.application_id,
                        &interaction.token,
                        EditBody { content },
                    )
                    .await?;
                return Ok(message);
            }

            let response = titanium_model::builder::InteractionResponseBuilder::new()
                .content(content.clone())
                .build();

            self.http
                .create_interaction_response(interaction.id, &interaction.token, &response)
                .await?;

            // Fetch the interaction response to return a full Message object.
            let msg = self
                .http
                .get_original_interaction_response(interaction.application_id, &interaction.token)
                .await?;
            Ok(msg)
        }
    }

    /// Reply with an embed.
    ///
    /// Works like [`Context::reply`] but sends an embed instead of text.
    /// Automatically handles deferred interactions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use titanium_rs::prelude::*;
    /// # async fn example(ctx: Context) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let embed = EmbedBuilder::new()
    ///     .title("Hello!")
    ///     .description("This is an embed")
    ///     .color(0x5865F2)
    ///     .build();
    /// ctx.reply_embed(embed).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// Reply with an embed.
    ///
    /// # Errors
    /// Returns `TitaniumError` if the HTTP request fails.
    pub async fn reply_embed(
        &self,
        embed: impl Into<Embed<'static>>,
    ) -> Result<Message<'static>, TitaniumError> {
        let embed = embed.into();
        let interaction = self
            .interaction
            .as_ref()
            .ok_or(ContextError::NoInteraction)?;

        if self.has_responded.load(Ordering::SeqCst) {
            // Edit original response
            #[derive(serde::Serialize)]
            struct EditBody {
                embeds: Vec<Embed<'static>>,
            }

            let message = self
                .http
                .edit_original_interaction_response(
                    interaction.application_id,
                    &interaction.token,
                    EditBody {
                        embeds: vec![embed],
                    },
                )
                .await?;
            Ok(message)
        } else {
            // Try to claim first response
            if self
                .has_responded
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_err()
            {
                // Lost race - use edit
                #[derive(serde::Serialize)]
                struct EditBody {
                    embeds: Vec<Embed<'static>>,
                }
                let message = self
                    .http
                    .edit_original_interaction_response(
                        interaction.application_id,
                        &interaction.token,
                        EditBody {
                            embeds: vec![embed],
                        },
                    )
                    .await?;
                return Ok(message);
            }

            let response = titanium_model::builder::InteractionResponseBuilder::new()
                .embed(embed.clone())
                .build();

            self.http
                .create_interaction_response(interaction.id, &interaction.token, &response)
                .await?;

            // Fetch the interaction response
            let msg = self
                .http
                .get_original_interaction_response(interaction.application_id, &interaction.token)
                .await?;
            Ok(msg)
        }
    }

    /// Reply with an ephemeral message (only visible to user).
    /// Reply with an ephemeral message (only visible to user).
    ///
    /// # Errors
    /// Returns `TitaniumError` if the HTTP request fails.
    pub async fn reply_ephemeral(&self, content: impl Into<String>) -> Result<(), TitaniumError> {
        let content = content.into();
        let interaction = self
            .interaction
            .as_ref()
            .ok_or(ContextError::NoInteraction)?;

        // Ephemeral can only be set on initial response
        if self
            .has_responded
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(ContextError::AlreadyResponded.into());
        }

        let response = titanium_model::builder::InteractionResponseBuilder::new()
            .content(content)
            .ephemeral(true)
            .build();

        self.http
            .create_interaction_response(interaction.id, &interaction.token, &response)
            .await?;

        Ok(())
    }

    /// Edit the original interaction response.
    pub async fn edit_reply(
        &self,
        content: impl Into<String>,
    ) -> Result<Message<'static>, TitaniumError> {
        let interaction = self
            .interaction
            .as_ref()
            .ok_or(ContextError::NoInteraction)?;
        #[derive(serde::Serialize)]
        #[allow(clippy::items_after_statements)]
        struct EditBody {
            content: String,
        }

        let message = self
            .http
            .edit_original_interaction_response(
                interaction.application_id,
                &interaction.token,
                EditBody {
                    content: content.into(),
                },
            )
            .await?;

        Ok(message)
    }

    /// Send a follow-up message.
    pub async fn followup(
        &self,
        content: impl Into<String>,
    ) -> Result<Message<'static>, TitaniumError> {
        use titanium_model::builder::ExecuteWebhook;
        let interaction = self
            .interaction
            .as_ref()
            .ok_or(ContextError::NoInteraction)?;

        let params = ExecuteWebhook {
            content: Some(content.into()),
            ..Default::default()
        };

        // Interaction followups use the webhook endpoint with application_id as webhook_id
        let msg = self
            .http
            .execute_webhook(interaction.application_id, &interaction.token, &params)
            .await?;

        // execute_webhook returns Option<Message>, but with wait=true it should return Some
        msg.ok_or(TitaniumError::Other(
            "Failed to get followup message".into(),
        ))
    }

    /// Get the user who triggered the interaction.
    #[inline]
    pub fn user(&self) -> Option<&User<'static>> {
        self.interaction.as_ref().and_then(|i| {
            i.member
                .as_ref()
                .and_then(|m| m.user.as_ref())
                .or(i.user.as_ref())
        })
    }

    /// Get the guild ID if in a guild.
    #[inline]
    #[must_use]
    pub fn guild_id(&self) -> Option<Snowflake> {
        self.interaction.as_ref().and_then(|i| i.guild_id)
    }

    /// Get the channel ID.
    #[inline]
    #[must_use]
    pub fn channel_id(&self) -> Option<Snowflake> {
        self.interaction.as_ref().and_then(|i| i.channel_id)
    }

    // =========================================================================
    // Quick Reply Helpers (JS-like ergonomics)
    // =========================================================================

    /// Reply with a success embed (green).
    #[inline]
    pub async fn success(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Message<'static>, TitaniumError> {
        let embed =
            titanium_model::builder::EmbedBuilder::success(title.into(), description.into())
                .build();
        self.reply_embed(embed).await
    }

    /// Reply with an error embed (red).
    #[inline]
    pub async fn error(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Message<'static>, TitaniumError> {
        let embed =
            titanium_model::builder::EmbedBuilder::error(title.into(), description.into()).build();
        self.reply_embed(embed).await
    }

    /// Reply with an info embed (blurple).
    #[inline]
    pub async fn info(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Message<'static>, TitaniumError> {
        let embed =
            titanium_model::builder::EmbedBuilder::info(title.into(), description.into()).build();
        self.reply_embed(embed).await
    }

    /// Reply with a warning embed (yellow).
    #[inline]
    pub async fn warning(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Message<'static>, TitaniumError> {
        let embed =
            titanium_model::builder::EmbedBuilder::warning(title.into(), description.into())
                .build();
        self.reply_embed(embed).await
    }

    /// Send a message to a specific channel (bypass interaction).
    #[inline]
    pub async fn send(
        &self,
        channel_id: impl Into<Snowflake>,
        content: impl Into<String>,
    ) -> Result<Message<'static>, titanium_http::HttpError> {
        self.http.send_message(channel_id.into(), content).await
    }

    /// Send an embed to a specific channel.
    #[inline]
    pub async fn send_embed(
        &self,
        channel_id: impl Into<Snowflake>,
        embed: impl Into<Embed<'static>>,
    ) -> Result<Message<'static>, titanium_http::HttpError> {
        let msg = titanium_model::builder::MessageBuilder::new()
            .embed(embed)
            .build();
        self.http
            .create_message_struct(channel_id.into(), &msg)
            .await
    }
}
