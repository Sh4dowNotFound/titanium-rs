use crate::error::HttpError;
use crate::HttpClient;
use titanium_model::{Snowflake, SoundboardSound};

impl HttpClient {
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
    ///
    /// Payload should contain `name`, `sound` (base64), and optional `volume`, `emoji_id`, `emoji_name`.
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
