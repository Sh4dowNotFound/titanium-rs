use crate::error::HttpError;
use crate::HttpClient;
use serde::Serialize;
use titanium_model::{AutoModRule, Snowflake};

impl HttpClient {
    /// List Auto Moderation rules.
    pub async fn list_auto_moderation_rules(
        &self,
        guild_id: Snowflake,
    ) -> Result<Vec<AutoModRule>, HttpError> {
        let route = format!("/guilds/{}/auto-moderation/rules", guild_id);
        self.get(&route).await
    }

    /// Get an Auto Moderation rule.
    pub async fn get_auto_moderation_rule(
        &self,
        guild_id: Snowflake,
        rule_id: Snowflake,
    ) -> Result<AutoModRule, HttpError> {
        let route = format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id);
        self.get(&route).await
    }

    /// Create an Auto Moderation rule.
    pub async fn create_auto_moderation_rule(
        &self,
        guild_id: Snowflake,
        params: &CreateAutoModRuleParams,
    ) -> Result<AutoModRule, HttpError> {
        let route = format!("/guilds/{}/auto-moderation/rules", guild_id);
        self.post(&route, params).await
    }

    /// Modify an Auto Moderation rule.
    pub async fn modify_auto_moderation_rule(
        &self,
        guild_id: Snowflake,
        rule_id: Snowflake,
        params: &ModifyAutoModRuleParams,
    ) -> Result<AutoModRule, HttpError> {
        let route = format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id);
        self.patch(&route, params).await
    }

    /// Delete an Auto Moderation rule.
    pub async fn delete_auto_moderation_rule(
        &self,
        guild_id: Snowflake,
        rule_id: Snowflake,
    ) -> Result<(), HttpError> {
        let route = format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id);
        self.delete(&route).await
    }
}

#[derive(Debug, Serialize)]
pub struct CreateAutoModRuleParams {
    pub name: String,
    pub event_type: u8,
    pub trigger_type: u8,
    pub trigger_metadata: Option<titanium_model::AutoModTriggerMetadata>,
    pub actions: Vec<titanium_model::AutoModAction>,
    pub enabled: Option<bool>,
    pub exempt_roles: Option<Vec<Snowflake>>,
    pub exempt_channels: Option<Vec<Snowflake>>,
}

#[derive(Debug, Serialize)]
pub struct ModifyAutoModRuleParams {
    pub name: Option<String>,
    pub event_type: Option<u8>,
    pub trigger_metadata: Option<titanium_model::AutoModTriggerMetadata>,
    pub actions: Option<Vec<titanium_model::AutoModAction>>,
    pub enabled: Option<bool>,
    pub exempt_roles: Option<Vec<Snowflake>>,
    pub exempt_channels: Option<Vec<Snowflake>>,
}
