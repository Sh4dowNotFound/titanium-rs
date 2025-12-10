use titanium_rs::prelude::*;
use async_trait::async_trait;
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 1. Setup logging
    tracing_subscriber::fmt::init();

    // 2. Get token
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");

    // 3. Define intents (Messages for !ping, Guilds for /ping interactions)
    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILDS;

    // 4. Build Client
    let client = Client::builder(token)
        .intents(intents)
        .event_handler(Handler)
        .build()
        .await
        .expect("Failed to create client");

    // 5. Start
    println!("Starting Ping Bot...");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: ReadyEventData<'_>) {
        println!("Logged in as {}", ready.user.username);
        println!("Use !ping or /ping to test latency.");
        
        // Register slash command on startup (global)
        // Note: Global commands take up to an hour to propagate. Guild commands are instant.
        // For this example we just log that we would register it.
        println!("(Note: Slash command registration would go here via HTTP API)");
    }

    async fn message_create(&self, ctx: Context, msg: Message<'_>) {
        // Ignore bots
        if msg.author.bot {
            return;
        }

        // Handle !ping
        if msg.content == "!ping" {
            let latency = ctx.shard.latency().unwrap_or(Duration::from_millis(0));
            let content = format!("Pong! 🏓\nLatency: `{}ms`", latency.as_millis());

            use titanium_model::CreateMessage;
            
            let params = CreateMessage {
                content: Some(content.into()),
                ..Default::default()
            };

            if let Err(e) = ctx.http.create_message(msg.channel_id, &params).await 
            {
                eprintln!("Error sending message: {:?}", e);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction<'_>) {
        if let InteractionType::ApplicationCommand = interaction.interaction_type {
            if let Some(data) = interaction.data {
                if data.name.as_deref() == Some("ping") {
                    let latency = ctx.shard.latency().unwrap_or(Duration::from_millis(0));
                    let content = format!("Pong! 🏓 (Slash)\nLatency: `{}ms`", latency.as_millis());

                    let response = InteractionResponse {
                        response_type: InteractionCallbackType::ChannelMessageWithSource,
                        data: Some(InteractionCallbackData {
                            content: Some(content.into()),
                            ..Default::default()
                        }),
                    };

                    // Respond to interaction
                    if let Err(e) = ctx.http.create_interaction_response(interaction.id, &interaction.token, &response)
                        .await
                    {
                        eprintln!("Error responding to interaction: {:?}", e);
                    }
                }
            }
        }
    }
}
