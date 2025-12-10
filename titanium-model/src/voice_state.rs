use crate::Snowflake;
use crate::TitanString;

/// Partial voice state from GUILD_CREATE.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartialVoiceState<'a> {
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,

    pub session_id: TitanString<'a>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    #[serde(default)]
    pub self_video: bool,
    pub suppress: bool,
    #[serde(default)]
    pub request_to_speak_timestamp: Option<TitanString<'a>>,
}
