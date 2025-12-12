//! Cluster example with multiple shards.
//!
//! This example demonstrates running a cluster with multiple shards.
//!
//! # Usage
//!
//! ```bash
//! DISCORD_TOKEN=your_bot_token cargo run --example cluster
//! ```

use titanium_gateway::{Cluster, ClusterConfig, Event, ShardRange};
use titanium_model::Intents;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Get token from environment
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    // Get shard count from environment or default to 1
    let shard_count: u16 = std::env::var("SHARD_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    // Configure cluster
    let config = ClusterConfig::new(
        token,
        Intents::GUILDS | Intents::GUILD_MESSAGES,
        ShardRange::All { total: shard_count },
    )
    .with_max_concurrency(1); // Adjust based on /gateway/bot response

    // Create cluster
    let (cluster, event_rx) = Cluster::new(config);

    // Spawn event handler
    tokio::spawn(async move {
        while let Ok((shard_id, event)) = event_rx.recv_async().await {
            match event {
                Event::Ready(ready) => {
                    info!(
                        "[Shard {}] Connected as {}#{} with {} guilds",
                        shard_id,
                        ready.user.username,
                        ready.user.discriminator,
                        ready.guilds.len()
                    );
                }
                Event::GuildCreate(guild) => {
                    info!(
                        "[Shard {}] Guild available: {} ({})",
                        shard_id, guild.name, guild.id
                    );
                }
                Event::MessageCreate(message) => {
                    info!(
                        "[Shard {}] Message from {}: {}",
                        shard_id, message.author.username, message.content
                    );
                }
                _ => {}
            }
        }
    });

    // Start cluster
    info!("Starting cluster with {} shards...", shard_count);
    cluster.start()?;

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;

    // Graceful shutdown
    info!("Shutting down...");
    cluster.shutdown().await;

    Ok(())
}
