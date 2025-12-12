use async_trait::async_trait;
use std::env;
use std::time::Duration;
use titanium_rs::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: ReadyEventData<'_>) {
        println!("Logged in as {}", ready.user.username);

        // Register slash command
        // Optimized: Manual construction avoids builder overhead and resolves type issues
        let command = titanium_rs::model::ApplicationCommand {
            id: None,
            command_type: Some(titanium_rs::model::command::CommandType::ChatInput),
            application_id: None,
            guild_id: None,
            name: "ping".to_string(),
            description: "Check the bot's latency".to_string(),
            options: Vec::new(),
            default_member_permissions: None,
            dm_permission: None,
            nsfw: false,
            version: None,
        };

        if let Some(app) = ready.application {
            if let Err(e) = ctx
                .http
                .create_global_application_command(app.id, &command)
                .await
            {
                eprintln!("Failed to register command: {e:?}");
            } else {
                println!("Registered /ping command");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction<'_>) {
        if let Some(data) = &interaction.data {
            if data.name.as_ref().map(|s| s.as_ref()) == Some("ping") {
                let start = std::time::Instant::now();

                // 1. Defer or initial reply (fastest response)
                if ctx.reply("Pinging...").await.is_err() {
                    return;
                }

                // 2. Measure REST roundtrip
                let rest_latency = start.elapsed();
                let gateway_latency = ctx.shard.latency().unwrap_or(Duration::from_millis(0));

                // 3. Create nice Embed result
                let embed = titanium_rs::model::builder::EmbedBuilder::new()
                    .title("Pong! 🏓")
                    .color(0x0058_65F2)
                    .field(
                        "Gateway",
                        format!("`{:.2}ms`", gateway_latency.as_secs_f64() * 1000.0),
                        true,
                    )
                    .field(
                        "API",
                        format!("`{:.2}ms`", rest_latency.as_secs_f64() * 1000.0),
                        true,
                    )
                    .footer("Titanium-RS High Performance", None::<String>)
                    .build();

                // 4. Send the embed
                let response = titanium_rs::model::builder::MessageBuilder::new()
                    .embed(embed)
                    .build();

                // For interactions, we should use interaction tokens/webhooks properly, but if we lack library support,
                // we can try creating a message in the channel IF we have the ID.
                if let Some(channel_id) = interaction.channel_id {
                    if let Err(e) = ctx.http.create_message_struct(channel_id, &response).await {
                        eprintln!("Failed to send ping result: {e:?}");
                    }
                } else {
                    eprintln!("Cannot allow ping outside of channel (no channel_id available)");
                }
            }
        }
    }

    async fn message_create(&self, ctx: Context, msg: Message<'_>) {
        // Prefix command handling
        if msg.content.as_bytes() == b"!ping" {
            let start = std::time::Instant::now();
            let channel_id = msg.channel_id;

            // 1. Send initial message
            let response = titanium_rs::model::builder::MessageBuilder::new()
                .content("Pinging...")
                .reply(msg.id)
                .build();

            match ctx.http.create_message(channel_id, &response).await {
                Ok(sent_msg) => {
                    let rest_latency = start.elapsed();
                    let gateway_latency = ctx.shard.latency().unwrap_or_default();

                    // 2. Edit with Embed
                    // Hack: DELETE then SEND NEW because lib edit support is minimal
                    let _ = ctx.http.delete_message(channel_id, sent_msg.id, None).await;

                    let embed = titanium_rs::model::builder::EmbedBuilder::new()
                        .title("Pong! 🏓")
                        .color(0x5865F2)
                        .field(
                            "Gateway",
                            format!("`{:.2}ms`", gateway_latency.as_secs_f64() * 1000.0),
                            true,
                        )
                        .field(
                            "API",
                            format!("`{:.2}ms`", rest_latency.as_secs_f64() * 1000.0),
                            true,
                        )
                        .footer("Titanium-RS High Performance", None::<String>)
                        .build();

                    let final_msg = titanium_rs::model::builder::MessageBuilder::new()
                        .embed(embed)
                        .reply(msg.id)
                        .build();

                    if let Err(e) = ctx.http.create_message(channel_id, &final_msg).await {
                        eprintln!("Failed to send result: {e:?}");
                    }
                }
                Err(e) => eprintln!("Failed to send prefix ping: {e:?}"),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");

    let client = Client::builder(token)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .event_handler(Handler)
        .build()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
