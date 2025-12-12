use async_trait::async_trait;
use std::env;
use titanium_model::builder::MessageBuilder;
use titanium_model::MessageReactionAddEvent;
use titanium_rs::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    Client::builder(env::var("DISCORD_TOKEN").expect("No Token"))
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGE_REACTIONS | Intents::MESSAGE_CONTENT)
        .event_handler(Handler)
        .build()
        .await
        .unwrap()
        .start()
        .await
        .unwrap();
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, r: ReadyEventData<'_>) {
        println!("Logged in as {}", r.user.username);
    }

    async fn reaction_add(&self, ctx: Context, add: MessageReactionAddEvent<'async_trait>) {
        if add.emoji.name.as_deref() == Some("🤖") {
            let channel_id = add.channel_id;
            let msg = MessageBuilder::new().content("Beep boop!").build();
            let _ = ctx.http.create_message(channel_id, &msg).await;
        }
    }
}
