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
    pub async fn start(
        &self,
    ) -> Result<flume::Receiver<Event<'static>>, Box<dyn std::error::Error>> {
        let (tx, rx) = flume::unbounded();

        let shard = self.shard.clone();
        let cache = self.cache.clone();

        let tx_clone = tx.clone();

        // Spawn shard loop
        tokio::spawn(async move {
            if let Err(e) = shard.run(tx_clone).await {
                eprintln!("Shard error: {:?}", e);
            }
        });

        // Return a receiver that INTERCEPTS events for caching?
        // Or duplicate the channel?
        // Better: Client::start should probably consume the receiver from shard, cache it, and forward it.
        // But `shard.run` takes a sender.
        // So we can wrap the sender? No, `shard.run` sends directly.
        // We need a middleman loop if we want "Automatic Cache Updates" without the user calling a method.
        // Or we just update cache in a separate task if we had a broadcast channel.
        // Flume is MPMC.

        // Let's create a proxy channel.
        let (proxy_tx, proxy_rx) = flume::unbounded();

        tokio::spawn(async move {
            // Read from the channel that Shard writes to
            while let Ok(event) = rx.recv_async().await {
                // Update cache
                match &event {
                    Event::Ready(ready) => {
                        cache.insert_user(ready.user.clone());
                        for _guild in &ready.guilds {
                            // These are unavailable guilds initially
                        }
                    }
                    Event::GuildCreate(guild) => {
                        cache.insert_guild(*guild.clone());
                    }
                    Event::ChannelCreate(channel) | Event::ChannelUpdate(channel) => {
                        cache.insert_channel(*channel.clone());
                    }
                    // Add more cache updates here...
                    _ => {}
                }

                // Forward to user
                let _ = proxy_tx.send_async(event).await;
            }
        });

        Ok(proxy_rx)
    }
}

pub struct ClientBuilder {
    token: String,
    intents: Intents,
    framework: Option<Framework>,
    // In a real generic trait impl we might store the dashboard config here
}

impl ClientBuilder {
    #[inline]
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            intents: Intents::default(),
            framework: None,
        }
    }

    // Note: To deeply integrate Dashboard, we would typically wrap it
    // or run it indiscriminately. For v1, users start it separately
    // or we provide a helper to spawn it alongside.

    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents;
        self
    }

    pub fn framework(mut self, framework: Framework) -> Self {
        self.framework = Some(framework);
        self
    }

    pub async fn build(self) -> Result<Client, Box<dyn std::error::Error>> {
        let http = Arc::new(HttpClient::new(self.token.clone())?);
        let cache = Arc::new(titanium_cache::InMemoryCache::new());

        // Fetch gateway info for recommended shards? For now, 1 shard.
        let config = ShardConfig::new(self.token.clone(), self.intents);
        let shard = Arc::new(Shard::new(0, 1, config));

        Ok(Client {
            shard,
            http,
            cache,
            framework: self.framework.map(Arc::new),
            token: self.token,
        })
    }
}
