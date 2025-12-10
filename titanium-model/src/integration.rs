//! Integration and webhook types.
//!
//! Integrations connect external services to Discord guilds.

use crate::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// A Discord integration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Integration<'a> {
    /// Integration ID.
    pub id: Snowflake,

    /// Integration name.
    pub name: String,

    /// Integration type (twitch, youtube, discord, guild_subscription).
    #[serde(rename = "type")]
    pub integration_type: String,

    /// Is this integration enabled.
    #[serde(default)]
    pub enabled: bool,

    /// Is this integration syncing.
    #[serde(default)]
    pub syncing: Option<bool>,

    /// ID that this integration uses for "subscribers".
    #[serde(default)]
    pub role_id: Option<Snowflake>,

    /// Whether emoticons should be synced.
    #[serde(default)]
    pub enable_emoticons: Option<bool>,

    /// The behavior of expiring subscribers.
    #[serde(default)]
    pub expire_behavior: Option<u8>,

    /// The grace period (in days) before expiring subscribers.
    #[serde(default)]
    pub expire_grace_period: Option<u32>,

    /// User for this integration.
    #[serde(default)]
    pub user: Option<super::User<'a>>,

    /// Integration account information.
    #[serde(default)]
    pub account: Option<IntegrationAccount>,

    /// When this integration was last synced (ISO8601 timestamp).
    #[serde(default)]
    pub synced_at: Option<String>,

    /// How many subscribers this integration has.
    #[serde(default)]
    pub subscriber_count: Option<u32>,

    /// Has this integration been revoked.
    #[serde(default)]
    pub revoked: Option<bool>,

    /// The bot/OAuth2 application for Discord integrations.
    #[serde(default)]
    pub application: Option<IntegrationApplication<'a>>,

    /// The scopes the application has been authorized for.
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// Integration account information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationAccount {
    /// ID of the account.
    pub id: String,

    /// Name of the account.
    pub name: String,
}

/// Application for a Discord integration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationApplication<'a> {
    /// The ID of the app.
    pub id: Snowflake,

    /// The name of the app.
    pub name: String,

    /// The icon hash of the app.
    #[serde(default)]
    pub icon: Option<String>,

    /// The description of the app.
    pub description: String,

    /// The bot associated with this application.
    #[serde(default)]
    pub bot: Option<super::User<'a>>,
}

/// Event data for INTEGRATION_DELETE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationDeleteEvent {
    /// Integration ID.
    pub id: Snowflake,

    /// ID of the guild.
    pub guild_id: Snowflake,

    /// ID of the bot/OAuth2 application for this discord integration.
    #[serde(default)]
    pub application_id: Option<Snowflake>,
}

/// Event data for GUILD_INTEGRATIONS_UPDATE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildIntegrationsUpdateEvent {
    /// ID of the guild.
    pub guild_id: Snowflake,
}

/// A Discord webhook.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook<'a> {
    /// The ID of the webhook.
    pub id: Snowflake,

    /// The type of the webhook.
    #[serde(rename = "type")]
    pub webhook_type: u8,

    /// The guild ID this webhook is for.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// The channel ID this webhook is for.
    #[serde(default)]
    pub channel_id: Option<Snowflake>,

    /// The user this webhook was created by.
    #[serde(default)]
    pub user: Option<super::User<'a>>,

    /// The default name of the webhook.
    #[serde(default)]
    pub name: Option<TitanString<'a>>,

    /// The default avatar of the webhook.
    #[serde(default)]
    pub avatar: Option<TitanString<'a>>,

    /// The secure token of the webhook.
    #[serde(default)]
    pub token: Option<TitanString<'a>>,

    /// The bot/OAuth2 application that created this webhook.
    #[serde(default)]
    pub application_id: Option<Snowflake>,

    /// The guild of the channel that this webhook is following.
    #[serde(default)]
    pub source_guild: Option<WebhookSourceGuild>,

    /// The channel that this webhook is following.
    #[serde(default)]
    pub source_channel: Option<WebhookSourceChannel>,

    /// The URL used for executing the webhook.
    #[serde(default)]
    pub url: Option<TitanString<'a>>,
}

/// Source guild for a webhook.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookSourceGuild {
    /// The ID of the guild.
    pub id: Snowflake,

    /// The name of the guild.
    pub name: String,

    /// The icon hash of the guild.
    #[serde(default)]
    pub icon: Option<String>,
}

/// Source channel for a webhook.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookSourceChannel {
    /// The ID of the channel.
    pub id: Snowflake,

    /// The name of the channel.
    pub name: String,
}

/// Event data for WEBHOOKS_UPDATE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhooksUpdateEvent {
    /// ID of the guild.
    pub guild_id: Snowflake,

    /// ID of the channel.
    pub channel_id: Snowflake,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        let json = r#"{
            "id": "123",
            "name": "Test Bot",
            "type": "discord",
            "enabled": true
        }"#;

        let integration: Integration = crate::json::from_str(json).unwrap();
        assert_eq!(integration.name, "Test Bot");
        assert_eq!(integration.integration_type, "discord");
    }

    #[test]
    fn test_webhook() {
        let json = r#"{
            "id": "123",
            "type": 1,
            "guild_id": "456",
            "channel_id": "789",
            "name": "Test Webhook"
        }"#;

        let _webhook: Webhook = crate::json::from_str(json).unwrap();
        let webhook: Webhook = crate::json::from_str(json).unwrap();
        assert_eq!(webhook.name, Some(TitanString::Borrowed("Test Webhook")));
    }
}
