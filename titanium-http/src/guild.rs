use crate::error::HttpError;
use crate::HttpClient;
use serde::{Deserialize, Serialize};
use titanium_model::{
    AuditLogEntry, AutoModRule, Channel, GuildMember, Integration, Role, ScheduledEvent, Snowflake,
    SoundboardSound, User, Webhook,
};

// ============================================================================
// Audit Log Response (merged from audit_log.rs)
// ============================================================================

/// Response structure for Get Guild Audit Log.
#[derive(Debug, Deserialize, Serialize)]
pub struct AuditLog<'a> {
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub auto_moderation_rules: Vec<AutoModRule>,
    pub guild_scheduled_events: Vec<ScheduledEvent<'a>>,
    pub integrations: Vec<Integration<'a>>,
    pub threads: Vec<Channel<'a>>,
    pub users: Vec<User<'a>>,
    pub webhooks: Vec<Webhook<'a>>,
}

#[derive(Debug, Default, Serialize)]
pub struct GetAuditLogParams {
    pub user_id: Option<Snowflake>,
    pub action_type: Option<u8>,
    pub before: Option<Snowflake>,
    pub after: Option<Snowflake>,
    pub limit: Option<u32>,
}

impl HttpClient {
    // =========================================================================
    // Guild Member Operations
    // =========================================================================

    /// Get a guild member.
    pub async fn get_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<GuildMember<'static>, HttpError> {
        let route = format!("/guilds/{}/members/{}", guild_id, user_id);
        self.get(&route).await
    }

    /// List guild members.
    pub async fn list_members(
        &self,
        guild_id: Snowflake,
        limit: Option<u32>,
        after: Option<Snowflake>,
    ) -> Result<Vec<GuildMember<'static>>, HttpError> {
        #[derive(Serialize)]
        struct Query {
            limit: u32,
            after: Option<Snowflake>,
        }

        let query = Query {
            limit: limit.unwrap_or(1),
            after,
        };

        let route = format!("/guilds/{}/members", guild_id);
        self.get_with_query(&route, &query).await
    }

    /// Kick a member from the guild.
    pub async fn kick_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
        reason: Option<&str>,
    ) -> Result<(), HttpError> {
        let route = format!("/guilds/{}/members/{}", guild_id, user_id);

        let headers = reason
            .map(|r| -> Result<_, HttpError> {
                let mut h = reqwest::header::HeaderMap::new();
                h.insert(
                    "X-Audit-Log-Reason",
                    reqwest::header::HeaderValue::from_str(r)?,
                );
                Ok(h)
            })
            .transpose()?;

        self.delete_with_headers(&route, headers).await
    }

    /// Ban a member from the guild.
    pub async fn ban_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
        delete_message_seconds: Option<u32>,
        reason: Option<&str>,
    ) -> Result<(), HttpError> {
        #[derive(Serialize)]
        struct BanBody {
            delete_message_seconds: Option<u32>,
        }

        let body = BanBody {
            delete_message_seconds,
        };
        let route = format!("/guilds/{}/bans/{}", guild_id, user_id);

        let headers = reason.and_then(|r| {
            reqwest::header::HeaderValue::from_str(r).ok().map(|v| {
                let mut h = reqwest::header::HeaderMap::new();
                h.insert("X-Audit-Log-Reason", v);
                h
            })
        });

        self.put_with_headers(&route, Some(body), headers).await
    }

    /// Unban a member.
    pub async fn unban_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
        reason: Option<&str>,
    ) -> Result<(), HttpError> {
        let route = format!("/guilds/{}/bans/{}", guild_id, user_id);

        let headers = reason.and_then(|r| {
            reqwest::header::HeaderValue::from_str(r).ok().map(|v| {
                let mut h = reqwest::header::HeaderMap::new();
                h.insert("X-Audit-Log-Reason", v);
                h
            })
        });

        self.delete_with_headers(&route, headers).await
    }

    /// Modify a guild member.
    pub async fn modify_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
        params: &titanium_model::builder::ModifyMember<'_>,
    ) -> Result<GuildMember<'static>, HttpError> {
        let route = format!("/guilds/{}/members/{}", guild_id, user_id);
        self.patch(&route, params).await
    }

    // =========================================================================
    // Role Operations
    // =========================================================================

    /// Get all roles.
    pub async fn get_roles(&self, guild_id: Snowflake) -> Result<Vec<Role<'static>>, HttpError> {
        let route = format!("/guilds/{}/roles", guild_id);
        self.get(&route).await
    }

    /// Create a new role.
    pub async fn create_role(
        &self,
        guild_id: Snowflake,
        params: &titanium_model::builder::CreateRole<'_>,
    ) -> Result<Role<'static>, HttpError> {
        let route = format!("/guilds/{}/roles", guild_id);
        self.post(&route, params).await
    }

    /// Delete a role.
    pub async fn delete_role(
        &self,
        guild_id: Snowflake,
        role_id: Snowflake,
        reason: Option<&str>,
    ) -> Result<(), HttpError> {
        let route = format!("/guilds/{}/roles/{}", guild_id, role_id);

        let headers = reason
            .map(|r| -> Result<_, HttpError> {
                let mut h = reqwest::header::HeaderMap::new();
                h.insert(
                    "X-Audit-Log-Reason",
                    reqwest::header::HeaderValue::from_str(r)?,
                );
                Ok(h)
            })
            .transpose()?;

        self.delete_with_headers(&route, headers).await
    }

    // =========================================================================
    // Audit Log (merged from audit_log.rs)
    // =========================================================================

    /// Get guild audit log.
    pub async fn get_guild_audit_log(
        &self,
        guild_id: Snowflake,
        params: &GetAuditLogParams,
    ) -> Result<AuditLog<'static>, HttpError> {
        let route = format!("/guilds/{}/audit-logs", guild_id);
        self.get_with_query(&route, params).await
    }

    // =========================================================================
    // Soundboard (merged from soundboard.rs)
    // =========================================================================

    /// List soundboard sounds for a guild.
    pub async fn list_guild_soundboard_sounds(
        &self,
        guild_id: Snowflake,
    ) -> Result<Vec<SoundboardSound<'static>>, HttpError> {
        self.get(&format!("/guilds/{}/soundboard-sounds", guild_id))
            .await
    }

    /// Get a specific soundboard sound.
    pub async fn get_guild_soundboard_sound(
        &self,
        guild_id: Snowflake,
        sound_id: Snowflake,
    ) -> Result<SoundboardSound<'static>, HttpError> {
        self.get(&format!(
            "/guilds/{}/soundboard-sounds/{}",
            guild_id, sound_id
        ))
        .await
    }

    /// Create a soundboard sound.
    pub async fn create_guild_soundboard_sound(
        &self,
        guild_id: Snowflake,
        payload: &serde_json::Value,
    ) -> Result<SoundboardSound<'static>, HttpError> {
        self.post(&format!("/guilds/{}/soundboard-sounds", guild_id), payload)
            .await
    }

    /// Modify a soundboard sound.
    pub async fn modify_guild_soundboard_sound(
        &self,
        guild_id: Snowflake,
        sound_id: Snowflake,
        payload: &serde_json::Value,
    ) -> Result<SoundboardSound<'static>, HttpError> {
        self.patch(
            &format!("/guilds/{}/soundboard-sounds/{}", guild_id, sound_id),
            payload,
        )
        .await
    }

    /// Delete a soundboard sound.
    pub async fn delete_guild_soundboard_sound(
        &self,
        guild_id: Snowflake,
        sound_id: Snowflake,
    ) -> Result<(), HttpError> {
        self.delete(&format!(
            "/guilds/{}/soundboard-sounds/{}",
            guild_id, sound_id
        ))
        .await
    }
}
