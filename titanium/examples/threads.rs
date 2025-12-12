use async_trait::async_trait;
use std::env;
use titanium_model::Channel;
use titanium_rs::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    Client::builder(env::var("DISCORD_TOKEN").expect("No Token"))
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES)
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

    async fn thread_create(&self, _ctx: Context, thread: Channel<'async_trait>) {
        if let Some(name) = &thread.name {
            println!("Thread created: {} (ID: {})", name, thread.id);
        } else {
            println!("Thread created: (ID: {})", thread.id);
        }
    }
}
