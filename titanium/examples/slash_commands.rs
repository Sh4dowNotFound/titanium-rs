use async_trait::async_trait;
use std::env;

use titanium_rs::prelude::*;

// Example: Slash Command Bot
// Demonstrates how to handle slash commands /hello and /echo

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN");

    // We need Guilds intent for interactions mostly, although interactions are sent over gateway even without intents if you use webhooks,
    // but standard gateway based bots usually need GUILDS or empty intents if just interactions.
    let client = Client::builder(token)
        .intents(Intents::GUILDS)
        .event_handler(SlashHandler)
        .build()
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

struct SlashHandler;

#[async_trait]
impl EventHandler for SlashHandler {
    async fn ready(&self, _ctx: Context, ready: ReadyEventData<'_>) {
        println!("Logged in as {}", ready.user.username);
        println!("Note: You should register commands via HTTP here or separately.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction<'_>) {
        if let InteractionType::ApplicationCommand = interaction.interaction_type {
            if let Some(data) = interaction.data {
                match data.name.as_deref() {
                    Some("hello") => {
                        let response = titanium_model::builder::InteractionResponseBuilder::new()
                            .content("Hello there! 👋")
                            .build();

                        let _ = ctx
                            .http
                            .create_interaction_response(
                                interaction.id,
                                &interaction.token,
                                &response,
                            )
                            .await;
                    }
                    Some("echo") => {
                        let content = "Echo!";
                        let response = titanium_model::builder::InteractionResponseBuilder::new()
                            .content(content)
                            .build();

                        let _ = ctx
                            .http
                            .create_interaction_response(
                                interaction.id,
                                &interaction.token,
                                &response,
                            )
                            .await;
                    }
                    _ => {}
                }
            }
        }
    }
}
