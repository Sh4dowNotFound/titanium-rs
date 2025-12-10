//! Audit log types for Discord moderation tracking.
//!
//! Audit logs record administrative actions in a guild.

use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// An audit log entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogEntry {
    /// ID of the entry.
    pub id: Snowflake,

    /// ID of the affected entity (user, channel, role, etc.).
    #[serde(default)]
    pub target_id: Option<String>,

    /// Changes made to the target.
    #[serde(default)]
    pub changes: Vec<AuditLogChange>,

    /// User or app that made the changes.
    #[serde(default)]
    pub user_id: Option<Snowflake>,

    /// Type of action that occurred.
    pub action_type: AuditLogEvent,

    /// Additional info for certain event types.
    #[serde(default)]
    pub options: Option<AuditLogOptions>,

    /// Reason for the change (1-512 characters).
    #[serde(default)]
    pub reason: Option<String>,
}

/// A change in an audit log entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogChange {
    /// Name of the changed entity.
    pub key: String,

    /// New value of the key.
    #[serde(default)]
    pub new_value: Option<crate::json::Value>,

    /// Old value of the key.
    #[serde(default)]
    pub old_value: Option<crate::json::Value>,
}

/// Additional audit log entry options.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditLogOptions {
    /// ID of the app whose permissions were targeted.
    #[serde(default)]
    pub application_id: Option<Snowflake>,

    /// Name of the Auto Moderation rule that was triggered.
    #[serde(default)]
    pub auto_moderation_rule_name: Option<String>,

    /// Trigger type of the Auto Moderation rule.
    #[serde(default)]
    pub auto_moderation_rule_trigger_type: Option<String>,

    /// Channel in which the entities were targeted.
    #[serde(default)]
    pub channel_id: Option<Snowflake>,

    /// Number of entities that were targeted.
    #[serde(default)]
    pub count: Option<String>,

    /// Number of days for prune.
    #[serde(default)]
    pub delete_member_days: Option<String>,

    /// ID of the overwritten entity.
    #[serde(default)]
    pub id: Option<Snowflake>,

    /// Number of members removed by the prune.
    #[serde(default)]
    pub members_removed: Option<String>,

    /// ID of the message that was targeted.
    #[serde(default)]
    pub message_id: Option<Snowflake>,

    /// Name of the role.
    #[serde(default)]
    pub role_name: Option<String>,

    /// Type of overwritten entity.
    #[serde(default, rename = "type")]
    pub overwrite_type: Option<String>,

    /// Type of integration.
    #[serde(default)]
    pub integration_type: Option<String>,
}

/// Audit log event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum AuditLogEvent {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,
    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,
    GuildScheduledEventCreate = 100,
    GuildScheduledEventUpdate = 101,
    GuildScheduledEventDelete = 102,
    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,
    ApplicationCommandPermissionUpdate = 121,
    SoundboardSoundCreate = 130,
    SoundboardSoundUpdate = 131,
    SoundboardSoundDelete = 132,
    AutoModerationRuleCreate = 140,
    AutoModerationRuleUpdate = 141,
    AutoModerationRuleDelete = 142,
    AutoModerationBlockMessage = 143,
    AutoModerationFlagToChannel = 144,
    AutoModerationUserCommunicationDisabled = 145,
    CreatorMonetizationRequestCreated = 150,
    CreatorMonetizationTermsAccepted = 151,
    OnboardingPromptCreate = 163,
    OnboardingPromptUpdate = 164,
    OnboardingPromptDelete = 165,
    OnboardingCreate = 166,
    OnboardingUpdate = 167,
    HomeSettingsCreate = 190,
    HomeSettingsUpdate = 191,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_entry() {
        let json = r#"{
            "id": "123",
            "target_id": "456",
            "changes": [],
            "action_type": 1
        }"#;

        let entry: AuditLogEntry = crate::json::from_str(json).unwrap();
        assert_eq!(entry.action_type, AuditLogEvent::GuildUpdate);
    }
}
