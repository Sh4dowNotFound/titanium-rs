//! Cluster management for multi-shard deployments.
//!
//! A Cluster manages multiple Shards, distributing work across them.
//! For very large bots (1M+ guilds), multiple Clusters can run on
//! different machines with coordinated shard ranges.

use crate::error::GatewayError;
use crate::event::Event;
use crate::ratelimit::IdentifyRateLimiter;
use crate::shard::{Shard, ShardConfig, ShardState};

use dashmap::DashMap;
use flume::{Receiver, Sender};
use std::sync::Arc;
use titanium_model::Intents;
use tokio::task::JoinHandle;
use tracing::{error, info};

/// Specifies which shards a Cluster should manage.
#[derive(Debug, Clone)]
pub enum ShardRange {
    /// Manage all shards (total count from API or config).
    All {
        /// Total number of shards.
        total: u16,
    },

    /// Manage a specific range of shards.
    Range {
        /// Starting shard ID (inclusive).
        start: u16,
        /// Ending shard ID (exclusive).
        end: u16,
        /// Total number of shards across all clusters.
        total: u16,
    },

    /// Manage specific shard IDs.
    Specific {
        /// Shard IDs to manage.
        ids: Vec<u16>,
        /// Total number of shards across all clusters.
        total: u16,
    },
}

impl ShardRange {
    /// Get the shard IDs this range covers.
    pub fn shard_ids(&self) -> Vec<u16> {
        match self {
            ShardRange::All { total } => (0..*total).collect(),
            ShardRange::Range { start, end, .. } => (*start..*end).collect(),
            ShardRange::Specific { ids, .. } => ids.clone(),
        }
    }

    /// Get the total number of shards.
    pub fn total_shards(&self) -> u16 {
        match self {
            ShardRange::All { total } => *total,
            ShardRange::Range { total, .. } => *total,
            ShardRange::Specific { total, .. } => *total,
        }
    }
}

/// Configuration for a Cluster.
#[derive(Debug, Clone)]
pub struct ClusterConfig {
    /// Bot token.
    pub token: String,

    /// Gateway intents.
    pub intents: Intents,

    /// Which shards to manage.
    pub shard_range: ShardRange,

    /// Gateway URL (usually from /gateway/bot).
    pub gateway_url: String,

    /// Maximum concurrent identify operations (from /gateway/bot).
    pub max_concurrency: usize,

    /// Large guild threshold.
    pub large_threshold: u8,
}

impl ClusterConfig {
    /// Create a new cluster configuration.
    pub fn new(token: impl Into<String>, intents: Intents, shard_range: ShardRange) -> Self {
        Self {
            token: token.into(),
            intents,
            shard_range,
            gateway_url: crate::DEFAULT_GATEWAY_URL.to_string(),
            max_concurrency: 1,
            large_threshold: 250,
        }
    }

    /// Set the maximum concurrency (from /gateway/bot response).
    pub fn with_max_concurrency(mut self, max_concurrency: usize) -> Self {
        self.max_concurrency = max_concurrency;
        self
    }

    /// Set the gateway URL.
    pub fn with_gateway_url(mut self, url: impl Into<String>) -> Self {
        self.gateway_url = url.into();
        self
    }

    /// Create a new cluster configuration with auto-detected shard count.
    ///
    /// This requires the `auto-sharding` feature.
    #[cfg(feature = "auto-sharding")]
    pub async fn autoscaled(
        token: impl Into<String>,
        intents: titanium_model::Intents,
    ) -> Result<Self, crate::error::GatewayError> {
        use titanium_http::HttpClient;

        let token = token.into();
        let client = HttpClient::new(&token).map_err(|_| crate::error::GatewayError::Closed {
            code: 0,
            reason: "Failed to create HTTP client for auto-sharding".into(),
        })?;

        let info =
            client
                .get_gateway_bot()
                .await
                .map_err(|e| crate::error::GatewayError::Closed {
                    code: 0,
                    reason: format!("Failed to fetch gateway info: {}", e),
                })?;

        Ok(Self {
            token,
            intents,
            shard_range: ShardRange::All { total: info.shards },
            gateway_url: info.url,
            max_concurrency: info.session_start_limit.max_concurrency as usize,
            large_threshold: 250,
        })
    }
}

/// A running shard with its task handle.
struct ShardRunner {
    /// The shard instance.
    shard: Arc<Shard>,
    /// The task handle for the shard's event loop.
    handle: JoinHandle<Result<(), GatewayError>>,
}

/// A Cluster manages multiple Gateway Shards.
///
/// The Cluster handles:
/// - Spawning and managing shard tasks
/// - Coordinating identify rate limiting across shards
/// - Aggregating events from all shards
///
/// # Example
///
/// ```ignore
/// use titanium_gateway::{Cluster, ClusterConfig, ShardRange};
/// use titanium_model::Intents;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = ClusterConfig::new(
///         "your-token",
///         Intents::GUILDS | Intents::GUILD_MESSAGES,
///         ShardRange::All { total: 1 },
///     );
///
///     let (cluster, mut events) = Cluster::new(config);
///     cluster.start().await?;
///
///     while let Ok((shard_id, event)) = events.recv_async().await {
///         println!("Shard {}: {:?}", shard_id, event);
///     }
///
///     Ok(())
/// }
/// ```
pub struct Cluster {
    /// Cluster configuration.
    config: ClusterConfig,

    /// Running shards.
    shards: DashMap<u16, ShardRunner>,

    /// Shared rate limiter for identify.
    rate_limiter: Arc<IdentifyRateLimiter>,

    /// Channel to send shard events.
    event_tx: Sender<(u16, Event<'static>)>,
}

impl Cluster {
    /// Create a new Cluster.
    ///
    /// Returns the Cluster and a receiver for events from all shards.
    /// Events are tagged with the shard ID they came from.
    pub fn new(config: ClusterConfig) -> (Self, Receiver<(u16, Event<'static>)>) {
        let (event_tx, event_rx) = flume::unbounded();
        let rate_limiter = Arc::new(IdentifyRateLimiter::new(config.max_concurrency));

        let cluster = Self {
            config,
            shards: DashMap::new(),
            rate_limiter,
            event_tx,
        };

        (cluster, event_rx)
    }

    /// Start all shards.
    ///
    /// This spawns a task for each shard and begins connecting to Discord.
    /// Shards will connect with proper rate limiting based on `max_concurrency`.
    pub async fn start(&self) -> Result<(), GatewayError> {
        let shard_ids = self.config.shard_range.shard_ids();
        let total_shards = self.config.shard_range.total_shards();

        info!(
            shards = ?shard_ids,
            total = total_shards,
            max_concurrency = self.config.max_concurrency,
            "Starting cluster"
        );

        for shard_id in shard_ids {
            self.spawn_shard(shard_id, total_shards)?;
        }

        Ok(())
    }

    /// Spawn a single shard.
    fn spawn_shard(&self, shard_id: u16, total_shards: u16) -> Result<(), GatewayError> {
        let shard_config = ShardConfig {
            token: self.config.token.clone(),
            intents: self.config.intents,
            gateway_url: self.config.gateway_url.clone(),
            large_threshold: self.config.large_threshold,
            compress: false,
            max_reconnect_attempts: 10,
            reconnect_base_delay_ms: 1000,
            reconnect_max_delay_ms: 60000,
        };

        let shard = Arc::new(Shard::with_rate_limiter(
            shard_id,
            total_shards,
            shard_config,
            self.rate_limiter.clone(),
        ));

        // Create per-shard event channel that forwards to cluster channel
        let (shard_tx, shard_rx) = flume::unbounded::<Event>();
        let cluster_tx = self.event_tx.clone();
        let shard_id_for_forward = shard_id;

        // Spawn forwarding task
        tokio::spawn(async move {
            while let Ok(event) = shard_rx.recv_async().await {
                if cluster_tx
                    .send_async((shard_id_for_forward, event))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        // Spawn shard task
        let shard_clone = shard.clone();
        let handle = tokio::spawn(async move { shard_clone.run(shard_tx).await });

        self.shards.insert(shard_id, ShardRunner { shard, handle });

        info!(shard_id = shard_id, "Shard spawned");
        Ok(())
    }

    /// Get the state of a specific shard.
    pub fn shard_state(&self, shard_id: u16) -> Option<ShardState> {
        self.shards.get(&shard_id).map(|r| r.shard.state())
    }

    /// Get the last measured latency for a specific shard.
    pub fn shard_latency(&self, shard_id: u16) -> Option<std::time::Duration> {
        self.shards.get(&shard_id).and_then(|r| r.shard.latency())
    }

    /// Get all shard IDs managed by this cluster.
    pub fn shard_ids(&self) -> Vec<u16> {
        self.shards.iter().map(|r| *r.key()).collect()
    }

    /// Send a raw payload to a specific shard.
    pub fn send(&self, shard_id: u16, payload: serde_json::Value) -> Result<(), GatewayError> {
        if let Some(runner) = self.shards.get(&shard_id) {
            runner.shard.send_payload(&payload)
        } else {
            Err(GatewayError::Closed {
                code: 0,
                reason: format!("Shard {} not found", shard_id),
            })
        }
    }

    /// Shutdown all shards gracefully.
    pub async fn shutdown(&self) {
        info!("Shutting down cluster");

        // Request shutdown for all shards
        for shard in self.shards.iter() {
            shard.shard.shutdown();
        }

        // Wait for all shard tasks to complete
        for mut entry in self.shards.iter_mut() {
            let runner = entry.value_mut();
            if let Err(e) = (&mut runner.handle).await {
                error!(shard_id = *entry.key(), error = %e, "Shard task panicked");
            }
        }

        info!("Cluster shutdown complete");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_range_all() {
        let range = ShardRange::All { total: 10 };
        let ids = range.shard_ids();
        assert_eq!(ids.len(), 10);
        assert_eq!(ids[0], 0);
        assert_eq!(ids[9], 9);
    }

    #[test]
    fn test_shard_range_specific() {
        let range = ShardRange::Specific {
            ids: vec![0, 5, 10],
            total: 20,
        };
        let ids = range.shard_ids();
        assert_eq!(ids, vec![0, 5, 10]);
        assert_eq!(range.total_shards(), 20);
    }

    #[test]
    fn test_cluster_config() {
        let config = ClusterConfig::new(
            "test_token",
            Intents::GUILDS,
            ShardRange::Range {
                start: 0,
                end: 5,
                total: 10,
            },
        )
        .with_max_concurrency(16);

        assert_eq!(config.max_concurrency, 16);
        assert_eq!(config.shard_range.shard_ids().len(), 5);
    }
}
