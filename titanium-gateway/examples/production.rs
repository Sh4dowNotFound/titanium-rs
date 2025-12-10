//! Production-ready example using HTTP client for shard configuration.
//!
//! This example demonstrates the recommended production setup:
//! 1. Fetch gateway info from /gateway/bot
//! 2. Use recommended shard count and max_concurrency
//! 3. Start cluster with proper configuration
//!
//! # Usage
//!
//! ```bash
//! DISCORD_TOKEN=your_bot_token cargo run --example production
//! ```

use titanium_gateway::{Cluster, ClusterConfig, Event, ShardRange};
use titanium_http::HttpClient;
use titanium_model::Intents;
use tracing::{info, warn, Level};
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

    // Create HTTP client
    let http = HttpClient::new(&token)?;

    // Fetch gateway bot info
    info!("Fetching gateway configuration...");
    let gateway_info = http.get_gateway_bot().await?;

    info!(
        "Gateway info: url={}, shards={}, max_concurrency={}, remaining_sessions={}",
        gateway_info.url,
        gateway_info.shards,
        gateway_info.session_start_limit.max_concurrency,
        gateway_info.session_start_limit.remaining
    );

    // Check if we have enough sessions remaining
    if gateway_info.session_start_limit.remaining < gateway_info.shards as u32 {
        warn!(
            "Low session starts remaining: {} (need {})",
            gateway_info.session_start_limit.remaining, gateway_info.shards
        );
    }

    // Configure cluster with recommended settings
    let config = ClusterConfig::new(
        token,
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::GUILD_MEMBERS,
        ShardRange::All {
            total: gateway_info.shards,
        },
    )
    .with_gateway_url(&gateway_info.url)
    .with_max_concurrency(gateway_info.session_start_limit.max_concurrency as usize);

    // Create cluster
    let (cluster, event_rx) = Cluster::new(config);

    // Spawn event handler
    let event_handle = tokio::spawn(async move {
        let mut guild_count = 0u64;
        let mut ready_shards = 0u16;

        while let Ok((shard_id, event)) = event_rx.recv_async().await {
            match event {
                Event::Ready(ready) => {
                    ready_shards += 1;
                    guild_count += ready.guilds.len() as u64;
                    info!(
                        "[Shard {}/{}] Ready as {}#{} with {} guilds (total: {})",
                        shard_id + 1,
                        gateway_info.shards,
                        ready.user.username,
                        ready.user.discriminator,
                        ready.guilds.len(),
                        guild_count
                    );
                }
                Event::GuildCreate(guild) => {
                    // Only log after all shards are ready (lazy-load complete)
                    if ready_shards >= gateway_info.shards {
                        info!(
                            "[Shard {}] New guild: {} ({})",
                            shard_id, guild.name, guild.id
                        );
                    }
                }
                Event::MessageCreate(message) => {
                    // Example: respond to ping
                    if message.content == "!ping" {
                        info!(
                            "[Shard {}] Ping from {} in {}",
                            shard_id, message.author.username, message.channel_id
                        );
                        // In production, use HTTP client to send response
                    }
                }
                _ => {}
            }
        }
    });

    // Start cluster
    info!("Starting cluster with {} shards...", gateway_info.shards);
    cluster.start().await?;

    // Wait for Ctrl+C
    info!("Bot running. Press Ctrl+C to shutdown.");
    tokio::signal::ctrl_c().await?;

    // Graceful shutdown
    info!("Shutting down...");
    cluster.shutdown().await;
    event_handle.abort();

    info!("Shutdown complete.");
    Ok(())
}
