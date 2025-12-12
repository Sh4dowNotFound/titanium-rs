use async_trait::async_trait;
use std::env;
use titanium_rs::prelude::*;

// Example: Voice Receive Bot (Placeholder)
// Demonstrates connecting to a voice channel.
// Note: Full voice receive requires complex UDP handling which is handled by titanium-voice crate.

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN");

    // Voice requires GUILD_VOICE_STATES intent
    let intents = Intents::GUILDS | Intents::GUILD_VOICE_STATES;

    let client = Client::builder(token)
        .intents(intents)
        .event_handler(VoiceHandler)
        .build()
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

struct VoiceHandler;

#[async_trait]
impl EventHandler for VoiceHandler {
    async fn ready(&self, _ctx: Context, ready: ReadyEventData<'_>) {
        println!("Voice Bot ready: {}", ready.user.username);
        println!("Use !join to make the bot join your voice channel.");
    }

    async fn message_create(&self, ctx: Context, msg: Message<'_>) {
        if msg.content == "!join" {
            // In a real bot, we would look up the user's voice state in the cache
            // let guild = ctx.cache.guild(msg.guild_id.unwrap()).await;
            // let channel_id = guild.voice_states.get(&msg.author.id).unwrap().channel_id;

            // Then send a Gateway Update Voice State op
            // ctx.shard.voice_state_update(msg.guild_id.unwrap(), Some(channel_id), false, false).await;

            let response = titanium_model::CreateMessage {
                content: Some("Attempting to join voice channel... (Logic placeholder)".into()),
                ..Default::default()
            };
            let _ = ctx.http.create_message(msg.channel_id, &response).await;
        }
    }
}
