use crate::error::HttpError;
use crate::HttpClient;
use serde::{Deserialize, Serialize};
use titanium_model::{
    AuditLogEntry, AutoModRule, Channel, Integration, ScheduledEvent, Snowflake, User, Webhook,
};

/// Response structure for Get Guild Audit Log.
#[derive(Debug, Deserialize, Serialize)]
pub struct AuditLog<'a> {
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub auto_moderation_rules: Vec<AutoModRule>,
    pub guild_scheduled_events: Vec<ScheduledEvent<'a>>,
    pub integrations: Vec<Integration<'a>>,
    pub threads: Vec<Channel<'a>>, // Threads are Channels in model usually
    pub users: Vec<User<'a>>,
    pub webhooks: Vec<Webhook<'a>>,
}

impl HttpClient {
    /// Get guild audit log.
    pub async fn get_guild_audit_log(
        &self,
        guild_id: Snowflake,
        params: &GetAuditLogParams,
    ) -> Result<AuditLog<'static>, HttpError> {
        let route = format!("/guilds/{}/audit-logs", guild_id);
        self.get_with_query(&route, params).await
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GetAuditLogParams {
    pub user_id: Option<Snowflake>,
    pub action_type: Option<u8>,
    pub before: Option<Snowflake>,
    pub after: Option<Snowflake>,
    pub limit: Option<u32>,
}
