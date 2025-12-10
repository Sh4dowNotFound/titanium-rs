//! Basic shard connection example.
//!
//! This example demonstrates connecting a single shard to Discord's Gateway.
//!
//! # Usage
//!
//! ```bash
//! DISCORD_TOKEN=your_bot_token cargo run --example basic_shard
//! ```

use titanium_gateway::{Event, Shard, ShardConfig};
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

    // Configure shard with desired intents
    let config = ShardConfig::new(
        token,
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    // Create shard (single shard, shard 0 of 1)
    let shard = Shard::new(0, 1, config);

    // Create event channel
    let (event_tx, event_rx) = flume::unbounded::<Event>();

    // Spawn event handler
    tokio::spawn(async move {
        while let Ok(event) = event_rx.recv_async().await {
            match event {
                Event::Ready(ready) => {
                    info!(
                        "Bot connected as {}#{} in {} guilds",
                        ready.user.username,
                        ready.user.discriminator,
                        ready.guilds.len()
                    );
                }
                Event::GuildCreate(guild) => {
                    info!("Guild available: {} ({})", guild.name, guild.id);
                }
                Event::MessageCreate(message) => {
                    info!(
                        "[{}] {}: {}",
                        message.channel_id, message.author.username, message.content
                    );
                }
                _ => {}
            }
        }
    });

    // Run shard (blocks until shutdown or error)
    info!("Starting shard...");
    shard.run(event_tx).await?;

    Ok(())
}
