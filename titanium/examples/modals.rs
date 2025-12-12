use async_trait::async_trait;
use std::env;
use titanium_model::builder::{
    ActionRowBuilder, InteractionResponseBuilder, ModalBuilder, TextInputBuilder,
};
use titanium_model::component::TextInputStyle;
use titanium_model::{InteractionCallbackType, InteractionType};
use titanium_rs::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    Client::builder(env::var("DISCORD_TOKEN").expect("No Token"))
        .intents(Intents::GUILDS)
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

    async fn interaction_create(&self, ctx: Context, i: Interaction<'_>) {
        let http = &ctx.http;
        match i.interaction_type {
            InteractionType::ApplicationCommand => {
                if i.data
                    .as_ref()
                    .is_some_and(|d| d.name.as_deref() == Some("modal"))
                {
                    let modal = ModalBuilder::new("my_modal", "Tell me about yourself")
                        .row(ActionRowBuilder::new().input(TextInputBuilder::new(
                            "name",
                            TextInputStyle::Short,
                            "What is your name?",
                        )))
                        .row(ActionRowBuilder::new().input(TextInputBuilder::new(
                            "bio",
                            TextInputStyle::Paragraph,
                            "Tell us your story",
                        )));

                    let resp = InteractionResponseBuilder::new()
                        .kind(InteractionCallbackType::Modal)
                        .modal(modal.build())
                        .build();

                    let _ = http
                        .create_interaction_response(i.id, &i.token, &resp)
                        .await;
                }
            }
            InteractionType::ModalSubmit => {
                if let Some(data) = i.data {
                    if data.custom_id.as_deref() == Some("my_modal") {
                        // Extract values efficiently
                        // Deserialize Modal structures
                        #[derive(serde::Deserialize)]
                        struct ModalComponent {
                            custom_id: String,
                            value: String,
                        }
                        #[derive(serde::Deserialize)]
                        struct ActionRow {
                            components: Vec<ModalComponent>,
                        }

                        // Convert Value to strongly typed struct
                        let rows: Vec<ActionRow> = data
                            .components
                            .iter()
                            .filter_map(|v| titanium_model::json::from_value(v.clone()).ok())
                            .collect();

                        let content = rows
                            .iter()
                            .flat_map(|r| &r.components)
                            .map(|c| format!("{}: {}", c.custom_id, c.value))
                            .collect::<Vec<_>>()
                            .join(", ");

                        let resp = InteractionResponseBuilder::new()
                            .kind(InteractionCallbackType::ChannelMessageWithSource)
                            .content(format!("Received: {content}"))
                            .build();

                        let _ = http
                            .create_interaction_response(i.id, &i.token, &resp)
                            .await;
                    }
                }
            }
            _ => {}
        }
    }
}
