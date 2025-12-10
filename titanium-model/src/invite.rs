//! Invite types for Discord guild invites.
//!
//! Invites allow users to join guilds or group DMs.

use crate::Snowflake;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// Event data for INVITE_CREATE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteCreateEvent<'a> {
    /// The channel the invite is for.
    pub channel_id: Snowflake,

    /// The unique invite code.
    #[serde(default)]
    pub code: TitanString<'a>,

    /// The time at which the invite was created (ISO8601 timestamp).
    #[serde(default)]
    pub created_at: TitanString<'a>,

    /// The guild of the invite.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// The user that created the invite.
    #[serde(default)]
    pub inviter: Option<super::User<'a>>,

    /// How long the invite is valid for (in seconds).
    pub max_age: u32,

    /// The maximum number of times the invite can be used.
    pub max_uses: u32,

    /// The target type for this voice channel invite.
    #[serde(default)]
    pub target_type: Option<u8>,

    /// The user whose stream to display for voice channel stream invites.
    #[serde(default)]
    pub target_user: Option<super::User<'a>>,

    /// The embedded application for voice channel invites.
    #[serde(default)]
    pub target_application: Option<super::Application>,

    /// Whether or not the invite is temporary.
    pub temporary: bool,

    /// How many times the invite has been used.
    pub uses: u32,
}

/// Event data for INVITE_DELETE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteDeleteEvent<'a> {
    /// The channel of the invite.
    pub channel_id: Snowflake,

    /// The guild of the invite.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// The unique invite code.
    #[serde(default)]
    pub code: TitanString<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invite_create_event() {
        let json = r#"{
            "channel_id": "123",
            "code": "abcdef",
            "created_at": "2021-01-01T00:00:00.000Z",
            "max_age": 86400,
            "max_uses": 0,
            "temporary": false,
            "uses": 0
        }"#;

        let event: InviteCreateEvent = crate::json::from_str(json).unwrap();
        assert_eq!(event.code, "abcdef");
        assert_eq!(event.max_age, 86400);
    }
}
