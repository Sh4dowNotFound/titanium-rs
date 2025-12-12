//! `AutoMod` types for Discord's automatic moderation system.
//!
//! `AutoMod` allows guilds to automatically filter and moderate content.

use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// An `AutoMod` rule.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoModRule {
    /// The ID of this rule.
    pub id: Snowflake,

    /// The ID of the guild which this rule belongs to.
    pub guild_id: Snowflake,

    /// The rule name.
    pub name: String,

    /// The user which first created this rule.
    pub creator_id: Snowflake,

    /// The rule event type.
    pub event_type: AutoModEventType,

    /// The rule trigger type.
    pub trigger_type: AutoModTriggerType,

    /// The rule trigger metadata.
    pub trigger_metadata: AutoModTriggerMetadata,

    /// The actions which will execute when the rule is triggered.
    pub actions: Vec<AutoModAction>,

    /// Whether the rule is enabled.
    pub enabled: bool,

    /// The role IDs that should not be affected by the rule.
    #[serde(default)]
    pub exempt_roles: Vec<Snowflake>,

    /// The channel IDs that should not be affected by the rule.
    #[serde(default)]
    pub exempt_channels: Vec<Snowflake>,
}

/// `AutoMod` event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModEventType {
    /// When a member sends or edits a message.
    MessageSend = 1,
    /// When a member edits their profile.
    MemberUpdate = 2,
}

/// `AutoMod` trigger types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModTriggerType {
    /// Check if content contains words from a user defined list.
    Keyword = 1,
    /// Check if content represents generic spam.
    Spam = 3,
    /// Check if content contains words from internal pre-defined wordsets.
    KeywordPreset = 4,
    /// Check if content contains more unique mentions than allowed.
    MentionSpam = 5,
    /// Check if member profile contains words from a user defined list.
    MemberProfile = 6,
}

/// Metadata for `AutoMod` triggers.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoModTriggerMetadata {
    /// Substrings which will be searched for in content.
    #[serde(default)]
    pub keyword_filter: Vec<String>,

    /// Regular expression patterns which will be matched against content.
    #[serde(default)]
    pub regex_patterns: Vec<String>,

    /// The internally pre-defined wordsets which will be searched for.
    #[serde(default)]
    pub presets: Vec<AutoModKeywordPresetType>,

    /// Substrings which should not trigger the rule.
    #[serde(default)]
    pub allow_list: Vec<String>,

    /// Total number of unique role and user mentions allowed per message.
    #[serde(default)]
    pub mention_total_limit: Option<u32>,

    /// Whether to automatically detect mention raids.
    #[serde(default)]
    pub mention_raid_protection_enabled: bool,
}

/// Pre-defined keyword preset types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModKeywordPresetType {
    /// Words that may be considered forms of swearing or cursing.
    Profanity = 1,
    /// Words that refer to sexually explicit behavior or activity.
    SexualContent = 2,
    /// Personal insults or words that may be considered hate speech.
    Slurs = 3,
}

/// An action which will execute whenever a rule is triggered.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoModAction {
    /// The type of action.
    #[serde(rename = "type")]
    pub action_type: AutoModActionType,

    /// Additional metadata needed during execution.
    #[serde(default)]
    pub metadata: Option<AutoModActionMetadata>,
}

/// `AutoMod` action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModActionType {
    /// Blocks a member's message and prevents it from being posted.
    BlockMessage = 1,
    /// Logs user content to a specified channel.
    SendAlertMessage = 2,
    /// Timeout user for a specified duration.
    Timeout = 3,
    /// Prevents a member from using text, voice, or other interactions.
    BlockMemberInteraction = 4,
}

/// Metadata for `AutoMod` actions.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoModActionMetadata {
    /// Channel to which user content should be logged.
    #[serde(default)]
    pub channel_id: Option<Snowflake>,

    /// Timeout duration in seconds.
    #[serde(default)]
    pub duration_seconds: Option<u64>,

    /// Additional explanation that will be shown to members.
    #[serde(default)]
    pub custom_message: Option<String>,
}

/// Sent when a rule is triggered and an action is executed.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoModActionExecution {
    /// ID of the guild in which action was executed.
    pub guild_id: Snowflake,

    /// Action which was executed.
    pub action: AutoModAction,

    /// ID of the rule which action belongs to.
    pub rule_id: Snowflake,

    /// Trigger type of rule which was triggered.
    pub rule_trigger_type: AutoModTriggerType,

    /// ID of the user which generated the content which triggered the rule.
    pub user_id: Snowflake,

    /// ID of the channel in which user content was posted.
    #[serde(default)]
    pub channel_id: Option<Snowflake>,

    /// ID of any user message which content belongs to.
    #[serde(default)]
    pub message_id: Option<Snowflake>,

    /// ID of any system auto moderation messages posted as a result of this action.
    #[serde(default)]
    pub alert_system_message_id: Option<Snowflake>,

    /// User-generated text content.
    #[serde(default)]
    pub content: String,

    /// Word or phrase configured in the rule that triggered the rule.
    #[serde(default)]
    pub matched_keyword: Option<String>,

    /// Substring in content that triggered the rule.
    #[serde(default)]
    pub matched_content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automod_action_type() {
        let json = "1";
        let action_type: AutoModActionType = crate::json::from_str(json).unwrap();
        assert_eq!(action_type, AutoModActionType::BlockMessage);
    }

    #[test]
    fn test_automod_trigger_metadata() {
        let json = r#"{
            "keyword_filter": ["badword"],
            "regex_patterns": ["\\d+"],
            "allow_list": ["exception"]
        }"#;

        let metadata: AutoModTriggerMetadata = crate::json::from_str(json).unwrap();
        assert_eq!(metadata.keyword_filter.len(), 1);
    }
}
