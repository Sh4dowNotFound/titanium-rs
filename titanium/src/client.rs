//! Discord bot client implementation.
//!
//! This module provides the main [`Client`] struct and [`ClientBuilder`] for
//! creating and running a Discord bot.
//!
//! # Architecture
//!
//! The client manages:
//! - A **Gateway Cluster** that handles WebSocket connections to Discord
//! - An **HTTP Client** for REST API requests
//! - An **In-Memory Cache** for frequently accessed entities
//! - **Event Handlers** for responding to Discord events
//!
//! # Example
//!
//! ```no_run
//! use titanium_rs::prelude::*;
//! use async_trait::async_trait;
//!
//! struct MyHandler;
//!
//! #[async_trait]
//! impl EventHandler for MyHandler {
//!     async fn message_create(&self, ctx: Context, msg: Message<'_>) {
//!         if &*msg.content == "!ping" {
//!             let _ = ctx.send(msg.channel_id, "Pong!").await;
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let token = std::env::var("DISCORD_TOKEN")?;
//!     
//!     Client::builder(token)
//!         .intents(Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
//!         .event_handler(MyHandler)
//!         .build()
//!         .await?
//!         .start()
//!         .await?;
//!     
//!     Ok(())
//! }
//! ```

use crate::error::TitaniumError;
use crate::framework::Framework;
use std::sync::Arc;
use titanium_cache::{Cache, InMemoryCache};
use titanium_gateway::{Cluster, ClusterConfig, Event};
use titanium_http::HttpClient;
use titanium_model::Intents;

/// The main Titanium Discord Client.
///
/// This struct is the central hub of your bot. It manages the connection to Discord,
/// caches entities, and dispatches events to your handlers.
///
/// # Creating a Client
///
/// Use [`Client::builder`] to create a new client:
///
/// ```no_run
/// # use titanium_rs::Client;
/// # use titanium_model::Intents;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::builder("your-token")
///     .intents(Intents::GUILD_MESSAGES)
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// # Thread Safety
///
/// `Client` is `Clone` and all internal state is wrapped in `Arc`, making it
/// safe to share across tasks.
#[derive(Clone)]
pub struct Client {
    /// The gateway cluster managing WebSocket connections.
    pub cluster: Arc<Cluster>,
    /// HTTP client for REST API requests.
    pub http: Arc<HttpClient>,
    /// In-memory cache for guilds, channels, users, etc.
    pub cache: Arc<InMemoryCache>,
    /// Optional command framework.
    pub framework: Option<Arc<Framework>>,
    /// Event handler for processing Discord events.
    pub event_handler: Option<Arc<dyn EventHandler>>,
    /// Channel receiving events from all shards.
    pub event_rx: flume::Receiver<(u16, Event<'static>)>,
    /// Bot token (stored for potential reconnection).
    #[allow(dead_code)]
    token: String,
}

impl Client {
    /// Create a new Client Builder.
    #[inline]
    pub fn builder(token: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(token)
    }

    /// Start the client and run the event loop.
    pub async fn start(&self) -> Result<(), TitaniumError> {
        // Start the cluster (spawns shards)
        self.cluster.start()?;

        // Event Processing Loop
        while let Ok((shard_id, event)) = self.event_rx.recv_async().await {
            // Automatic Cache Updates
            match &event {
                Event::Ready(ready) => {
                    self.cache.insert_user(Arc::new(ready.user.clone()));
                    // We don't cache guilds here as they come in GuildCreate
                }
                Event::GuildCreate(guild) => {
                    self.cache.insert_guild(Arc::clone(guild));
                }
                Event::GuildUpdate(guild) => {
                    self.cache.insert_guild(Arc::clone(guild));
                }
                Event::GuildDelete(unavailable) => {
                    if !unavailable.unavailable {
                        self.cache.remove_guild(unavailable.id);
                    }
                }
                Event::ChannelCreate(channel) => {
                    self.cache.insert_channel(Arc::clone(channel));
                }
                Event::ChannelUpdate(channel) => {
                    self.cache.insert_channel(Arc::clone(channel));
                }
                Event::ChannelDelete(channel) => {
                    self.cache.remove_channel(channel.id);
                }
                Event::GuildMemberAdd(member) => {
                    // Convert GuildMemberAddEvent to GuildMember for cache
                    let guild_member = titanium_model::GuildMember {
                        user: member.user.clone(),
                        nick: member.nick.clone().map(Into::into),
                        avatar: member.avatar.clone().map(Into::into),
                        roles: member.roles.clone().into(),
                        joined_at: member.joined_at.clone().into(),
                        premium_since: None, // Not present in Add event in current model
                        deaf: member.deaf,
                        mute: member.mute,
                        flags: member.flags,
                        pending: member.pending,
                        permissions: None,
                        communication_disabled_until: None,
                    };
                    self.cache
                        .insert_member(member.guild_id, Arc::new(guild_member));
                }
                Event::GuildMemberUpdate(member) => {
                    // Update member in cache if exists
                    if let Some(cached_member) = self.cache.member(member.guild_id, member.user.id)
                    {
                        // cached_member is Arc<GuildMember>.
                        let mut new_member = (*cached_member).clone();

                        new_member.roles = member.roles.clone().into();
                        new_member.nick = member.nick.clone().map(Into::into);
                        new_member.avatar = member.avatar.clone().map(Into::into);
                        if let Some(joined) = &member.joined_at {
                            new_member.joined_at = joined.clone().into();
                        }
                        new_member.deaf = member.deaf.unwrap_or(new_member.deaf);
                        new_member.mute = member.mute.unwrap_or(new_member.mute);
                        new_member.pending = member.pending;
                        new_member.communication_disabled_until =
                            member.communication_disabled_until.clone().map(Into::into);

                        self.cache
                            .insert_member(member.guild_id, Arc::new(new_member));
                    }
                }
                Event::GuildMemberRemove(event) => {
                    self.cache.remove_member(event.guild_id, event.user.id);
                }
                Event::GuildRoleCreate(event) => {
                    self.cache
                        .insert_role(event.role.id, Arc::new(event.role.clone()));
                }
                Event::GuildRoleUpdate(event) => {
                    self.cache
                        .insert_role(event.role.id, Arc::new(event.role.clone()));
                }
                Event::GuildRoleDelete(event) => {
                    self.cache.remove_role(event.role_id);
                }
                Event::UserUpdate(user) => {
                    self.cache.insert_user(user.clone());
                }
                _ => {}
            }

            // Dispatch to Event Handler
            if let Some(handler) = &self.event_handler {
                let http = self.http.clone();
                let cache = self.cache.clone();
                // Retrieve specific shard for context - skip event if shard not found
                let Some(shard) = self.cluster.shard(shard_id) else {
                    tracing::warn!(shard_id, "Shard not found for event, skipping dispatch");
                    continue;
                };

                // Prepare variables for the spawned task
                let handler = handler.clone();
                let event = event.clone(); // Cheap clone (enum of Arcs)

                // Spawn a new task for the handler to ensure the event loop remains unblocked
                tokio::spawn(async move {
                    // Helper to create Context
                    let make_ctx = || {
                        super::context::Context::new(
                            http.clone(),
                            cache.clone(),
                            shard.clone(),
                            None,
                        )
                    };

                    match event {
                        Event::Ready(ready) => {
                            handler.ready(make_ctx(), (*ready).clone()).await;
                        }
                        Event::MessageCreate(msg) => {
                            handler.message_create(make_ctx(), (*msg).clone()).await;
                        }
                        Event::InteractionCreate(interaction) => {
                            // Interaction is Arc<Interaction>
                            let interaction_val = (*interaction).clone();
                            let ctx = super::context::Context::new(
                                http.clone(),
                                cache.clone(),
                                shard.clone(),
                                Some(interaction_val.clone()),
                            );
                            handler.interaction_create(ctx, interaction_val).await;
                        }
                        Event::MessageReactionAdd(ev) => {
                            handler.reaction_add(make_ctx(), (*ev).clone()).await;
                        }
                        Event::ThreadCreate(thread) => {
                            handler.thread_create(make_ctx(), (*thread).clone()).await;
                        }
                        Event::GuildRoleCreate(ev) => {
                            handler.role_create(make_ctx(), (*ev).clone()).await;
                        }
                        _ => {}
                    }
                });
            }
        }

        Ok(())
    }
}

use crate::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn ready(&self, _ctx: Context, _ready: ReadyEventData<'_>) {}
    async fn message_create(&self, _ctx: Context, _msg: Message<'_>) {}
    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction<'_>) {}
    async fn reaction_add(
        &self,
        _ctx: Context,
        _add: titanium_model::MessageReactionAddEvent<'async_trait>,
    ) {
    }
    async fn thread_create(&self, _ctx: Context, _thread: Channel<'async_trait>) {}
    async fn role_create(&self, _ctx: Context, _role: GuildRoleEvent<'async_trait>) {}
}

pub struct ClientBuilder {
    token: String,
    intents: Intents,
    framework: Option<Framework>,
    event_handler: Option<Arc<dyn EventHandler>>,
}

impl ClientBuilder {
    #[inline]
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            intents: Intents::default(),
            framework: None,
            event_handler: None,
        }
    }

    /// Set the initial intents.
    #[must_use]
    pub const fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents;
        self
    }

    /// Set the command framework.
    #[must_use]
    pub fn framework(mut self, framework: Framework) -> Self {
        self.framework = Some(framework);
        self
    }

    pub fn event_handler<H: EventHandler + 'static>(mut self, handler: H) -> Self {
        self.event_handler = Some(Arc::new(handler));
        self
    }

    pub async fn build(self) -> Result<Client, TitaniumError> {
        let http = Arc::new(HttpClient::new(self.token.clone())?);
        let cache = Arc::new(titanium_cache::InMemoryCache::new());

        // Use auto-scaling cluster configuration
        // This fetches the recommended shard count from Discord
        let config = ClusterConfig::autoscaled(self.token.clone(), self.intents).await?;

        // Initialize Cluster
        let (cluster, rx) = Cluster::new(config);
        let cluster = Arc::new(cluster);

        Ok(Client {
            cluster,
            http,
            cache,
            framework: self.framework.map(Arc::new),
            event_handler: self.event_handler,
            event_rx: rx,
            token: self.token,
        })
    }
}
