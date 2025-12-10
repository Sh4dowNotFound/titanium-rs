use crate::framework::Framework;
use std::sync::Arc;
use titanium_cache::{Cache, InMemoryCache};
use titanium_gateway::{Event, Shard, ShardConfig};
use titanium_http::HttpClient;
use titanium_model::Intents;

/// The main Titan Client.
///
/// This struct holds the connection to Discord, including the Gateway Shard(s)
/// and the HTTP client. It is the main entry point for your bot.
#[derive(Clone)]
pub struct Client {
    pub shard: Arc<Shard>, // Wrapped in Arc for concurrent access/spawning
    pub http: Arc<HttpClient>,
    pub cache: Arc<InMemoryCache>,
    pub framework: Option<Arc<Framework>>,
    pub event_handler: Option<Arc<dyn EventHandler>>,
    #[allow(dead_code)]
    token: String,
}

impl Client {
    /// Create a new Client Builder.
    #[inline]
    pub fn builder(token: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(token)
    }

    /// Start the client and return the event stream.
    ///
    /// This spawns the shard connection in the background.
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = flume::unbounded();

        let shard = self.shard.clone();
        let cache = self.cache.clone();
        let http = self.http.clone();
        let event_handler = self.event_handler.clone();

        // Spawn shard loop
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = shard.run(tx_clone).await {
                eprintln!("Shard error: {:?}", e);
            }
        });

        // Event Processing Loop
        while let Ok(event) = rx.recv_async().await {
            // Update Cache
            match &event {
                Event::Ready(ready) => {
                    cache.insert_user(ready.user.clone());
                    // ... other cache updates
                }
                Event::GuildCreate(guild) => {
                    cache.insert_guild(*guild.clone());
                }
                // ...
                _ => {}
            }

            // Dispatch to Event Handler
            if let Some(handler) = &event_handler {
                match event {
                    Event::Ready(ready) => {
                        let ctx = Context::new(http.clone(), cache.clone(), self.shard.clone(), None);
                        handler.ready(ctx, *ready).await;
                    }
                    Event::MessageCreate(msg) => {
                        let ctx = Context::new(http.clone(), cache.clone(), self.shard.clone(), None);
                        handler.message_create(ctx, *msg).await;
                    }
                    Event::InteractionCreate(interaction) => {
                        let interaction_val = *interaction;
                        let ctx = Context::new(
                            http.clone(),
                            cache.clone(),
                            self.shard.clone(),
                            Some(interaction_val.clone()),
                        );
                        handler.interaction_create(ctx, interaction_val).await;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}


use async_trait::async_trait;
use crate::prelude::*;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn ready(&self, _ctx: Context, _ready: ReadyEventData<'_>) {}
    async fn message_create(&self, _ctx: Context, _msg: Message<'_>) {}
    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction<'_>) {}
    // Add other events as needed
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

    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents;
        self
    }

    pub fn framework(mut self, framework: Framework) -> Self {
        self.framework = Some(framework);
        self
    }

    pub fn event_handler<H: EventHandler + 'static>(mut self, handler: H) -> Self {
        self.event_handler = Some(Arc::new(handler));
        self
    }

    pub async fn build(self) -> Result<Client, Box<dyn std::error::Error>> {
        let http = Arc::new(HttpClient::new(self.token.clone())?);
        let cache = Arc::new(titanium_cache::InMemoryCache::new());

        let config = ShardConfig::new(self.token.clone(), self.intents);
        let shard = Arc::new(Shard::new(0, 1, config));

        Ok(Client {
            shard,
            http,
            cache,
            framework: self.framework.map(Arc::new),
            event_handler: self.event_handler,
            token: self.token,
        })
    }
}

