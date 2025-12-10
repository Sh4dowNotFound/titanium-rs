use crate::member::GuildMember;
use crate::reaction::ReactionEmoji;
use crate::Snowflake;

/// Voice state update event.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VoiceStateUpdateEvent<'a> {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    #[serde(default)]
    pub member: Option<GuildMember<'a>>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    #[serde(default)]
    pub self_stream: bool,
    pub self_video: bool,
    pub suppress: bool,
    #[serde(default)]
    pub request_to_speak_timestamp: Option<String>,
}

/// Voice server update event.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VoiceServerUpdateEvent {
    pub token: String,
    pub guild_id: Snowflake,
    #[serde(default)]
    pub endpoint: Option<String>,
}

/// Voice channel effect send event.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VoiceChannelEffectSendEvent<'a> {
    pub channel_id: Snowflake,
    pub guild_id: Snowflake,
    pub user_id: Snowflake,
    #[serde(default)]
    pub emoji: Option<ReactionEmoji<'a>>,
    #[serde(default)]
    pub animation_type: Option<u8>,
    #[serde(default)]
    pub animation_id: Option<u64>,
    #[serde(default)]
    pub sound_id: Option<Snowflake>,
    #[serde(default)]
    pub sound_volume: Option<f64>,
}
