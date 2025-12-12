//! Soundboard types for Discord's soundboard feature.
//!
//! Soundboard sounds can be played in voice channels.

use crate::Snowflake;
use serde::{Deserialize, Serialize};

/// A soundboard sound.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoundboardSound<'a> {
    /// The name of this sound.
    pub name: String,

    /// The ID of this sound.
    pub sound_id: Snowflake,

    /// The volume of this sound (0 to 1).
    pub volume: f64,

    /// The ID of this sound's custom emoji.
    #[serde(default)]
    pub emoji_id: Option<Snowflake>,

    /// The unicode character of this sound's standard emoji.
    #[serde(default)]
    pub emoji_name: Option<String>,

    /// The ID of the guild this sound is in.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// Whether this sound can be used.
    #[serde(default)]
    pub available: bool,

    /// The user who created this sound.
    #[serde(default)]
    pub user: Option<super::User<'a>>,
}

/// Event data for `SOUNDBOARD_SOUND_DELETE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoundboardSoundDeleteEvent {
    /// The ID of the sound.
    pub sound_id: Snowflake,

    /// The ID of the guild.
    pub guild_id: Snowflake,
}

/// Event data for `SOUNDBOARD_SOUNDS_UPDATE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoundboardSoundsUpdateEvent<'a> {
    /// The soundboard sounds.
    pub soundboard_sounds: Vec<SoundboardSound<'a>>,

    /// The ID of the guild (not in payload but added by gateway).
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// Event data for `GUILD_SOUNDBOARD_SOUNDS_UPDATE`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildSoundboardSoundsUpdateEvent<'a> {
    /// The soundboard sounds.
    pub soundboard_sounds: Vec<SoundboardSound<'a>>,

    /// The ID of the guild.
    pub guild_id: Snowflake,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soundboard_sound() {
        let json = r#"{
            "name": "airhorn",
            "sound_id": "123456789",
            "volume": 0.5,
            "available": true
        }"#;

        let sound: SoundboardSound = crate::json::from_str(json).unwrap();
        assert_eq!(sound.name, "airhorn");
        assert_eq!(sound.volume, 0.5);
    }
}
