use async_trait::async_trait;
use std::env;
use titanium_model::{
    ActionRowBuilder, ButtonBuilder, ComponentType, InteractionType, SelectMenuBuilder,
};
use titanium_rs::prelude::*;

// Example: Components Bot (Buttons & Select Menus)
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN");

    let client = Client::builder(token)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES)
        .event_handler(ComponentHandler)
        .build()
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

struct ComponentHandler;

#[async_trait]
impl EventHandler for ComponentHandler {
    async fn ready(&self, _ctx: Context, ready: ReadyEventData<'_>) {
        println!("Logged in as {}", ready.user.username);
        println!("Send !menu to test components.");
    }

    async fn message_create(&self, ctx: Context, msg: Message<'_>) {
        if msg.content == "!menu" {
            // Create a button
            let button = ButtonBuilder::new()
                .custom_id("click_me")
                .label("Click Me!")
                .style(titanium_model::component::ButtonStyle::Primary);

            // Create a select menu
            let select = SelectMenuBuilder::new("my_select")
                .placeholder("Choose an option...")
                .option("Option A", "opt_a")
                .option("Option B", "opt_b");

            // Create Action Rows
            let row1 = ActionRowBuilder::new().add_button(button);
            let row2 = ActionRowBuilder::new().add_select_menu(select);

            let response = titanium_model::builder::MessageBuilder::new()
                .content("Here are your components:")
                .component(row1)
                .component(row2)
                .build();

            let _ = ctx.http.create_message(msg.channel_id, &response).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction<'_>) {
        if let InteractionType::MessageComponent = interaction.interaction_type {
            if let Some(data) = &interaction.data {
                let custom_id = data.custom_id.as_deref().unwrap_or_default();

                let content = match (data.component_type, custom_id) {
                    (Some(ComponentType::Button), "click_me") => {
                        "You clicked the button!".to_string()
                    }
                    (Some(ComponentType::StringSelect), "my_select") => {
                        let values = &data.values;
                        format!("You selected: {:?}", values)
                    }
                    _ => "Unknown component interaction".to_string(),
                };

                let response = titanium_model::builder::InteractionResponseBuilder::new()
                    .content(content)
                    .ephemeral(true)
                    .build();
                let _ = ctx
                    .http
                    .create_interaction_response(interaction.id, &interaction.token, &response)
                    .await;
            }
        }
    }
}
