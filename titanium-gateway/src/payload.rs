//! Gateway payload structures.
//!
//! These structures represent the JSON payloads sent and received over the Gateway WebSocket.
//! Where possible, zero-copy parsing is used via `serde_json::value::RawValue`.

use crate::opcode::OpCode;
use serde::{Deserialize, Serialize};
use titanium_model::{Application, Intents, UnavailableGuild, User};

/// A raw Gateway payload for initial parsing.
///
/// Uses `RawValue` (standard) or `Value` (SIMD) for the `d` field to defer parsing.
#[derive(Debug, Deserialize)]
pub struct RawGatewayPayload<'a> {
    /// Opcode for the payload.
    pub op: OpCode,

    /// Event data.
    #[cfg(not(feature = "simd"))]
    #[serde(borrow)]
    pub d: Option<&'a serde_json::value::RawValue>,

    /// Event data (fully parsed to Value when using SIMD).
    #[cfg(feature = "simd")]
    pub d: Option<titanium_model::json::Value>,

    /// Sequence number (for Dispatch events).
    #[allow(dead_code)]
    pub s: Option<u64>,

    /// Event name (for Dispatch events).
    #[allow(dead_code)]
    pub t: Option<String>,

    #[cfg(feature = "simd")]
    #[serde(skip)]
    pub _marker: std::marker::PhantomData<&'a ()>,
}

/// A fully parsed Gateway payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPayload<D> {
    /// Opcode for the payload.
    pub op: OpCode,

    /// Event data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<D>,

    /// Sequence number (for Dispatch events).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,

    /// Event name (for Dispatch events).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}

impl<D: Serialize> GatewayPayload<D> {
    /// Create a new payload with only opcode and data.
    pub fn new(op: OpCode, data: D) -> Self {
        Self {
            op,
            d: Some(data),
            s: None,
            t: None,
        }
    }

    /// Create a payload with no data (e.g., HeartbeatAck).
    pub fn opcode_only(op: OpCode) -> GatewayPayload<()> {
        GatewayPayload {
            op,
            d: None,
            s: None,
            t: None,
        }
    }
}

// ============================================================================
// Hello Payload (Received after connection)
// ============================================================================

/// Payload for the Hello opcode (op 10).
///
/// Received immediately after connecting to the Gateway.
#[derive(Debug, Clone, Deserialize)]
pub struct HelloPayload {
    /// Interval (in milliseconds) at which to send heartbeats.
    pub heartbeat_interval: u64,
}

// ============================================================================
// Identify Payload (Sent to authenticate)
// ============================================================================

/// Payload for the Identify opcode (op 2).
///
/// Sent to authenticate and start a new session.
#[derive(Debug, Clone, Serialize)]
pub struct IdentifyPayload<'a> {
    /// Authentication token.
    pub token: std::borrow::Cow<'a, str>,

    /// Gateway intents.
    pub intents: Intents,

    /// Connection properties.
    pub properties: ConnectionProperties<'a>,

    /// Whether to enable payload compression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,

    /// Threshold for large guilds (50-250, default 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<u8>,

    /// Shard information: [shard_id, total_shards].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u16; 2]>,

    /// Initial presence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceUpdate>,
}

impl<'a> IdentifyPayload<'a> {
    /// Create a new Identify payload with required fields.
    pub fn new(token: impl Into<std::borrow::Cow<'a, str>>, intents: Intents) -> Self {
        Self {
            token: token.into(),
            intents,
            properties: ConnectionProperties::default(),
            compress: None,
            large_threshold: Some(250),
            shard: None,
            presence: None,
        }
    }

    /// Set shard information.
    pub fn with_shard(mut self, shard_id: u16, total_shards: u16) -> Self {
        self.shard = Some([shard_id, total_shards]);
        self
    }
}

/// Connection properties sent with Identify.
#[derive(Debug, Clone, Serialize)]
pub struct ConnectionProperties<'a> {
    /// Operating system.
    pub os: std::borrow::Cow<'a, str>,

    /// Library name.
    pub browser: std::borrow::Cow<'a, str>,

    /// Library name (again, for device).
    pub device: std::borrow::Cow<'a, str>,
}

impl<'a> Default for ConnectionProperties<'a> {
    fn default() -> Self {
        Self {
            os: std::borrow::Cow::Owned(std::env::consts::OS.to_string()),
            browser: std::borrow::Cow::Borrowed("titanium-rs"),
            device: std::borrow::Cow::Borrowed("titanium-rs"),
        }
    }
}

/// Presence update payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceUpdate {
    /// Unix timestamp (milliseconds) of when the client went idle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,

    /// User's activities.
    pub activities: Vec<Activity>,

    /// User's status.
    pub status: Status,

    /// Whether the client is AFK.
    pub afk: bool,
}

/// Activity for presence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    /// Activity name.
    pub name: String,

    /// Activity type.
    #[serde(rename = "type")]
    pub activity_type: ActivityType,

    /// Stream URL (only for Streaming type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Activity type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum ActivityType {
    /// Playing {name}
    Playing,
    /// Streaming {name}
    Streaming,
    /// Listening to {name}
    Listening,
    /// Watching {name}
    Watching,
    /// {emoji} {name}
    Custom,
    /// Competing in {name}
    Competing,
}

impl From<u8> for ActivityType {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivityType::Playing,
            1 => ActivityType::Streaming,
            2 => ActivityType::Listening,
            3 => ActivityType::Watching,
            4 => ActivityType::Custom,
            5 => ActivityType::Competing,
            _ => ActivityType::Playing,
        }
    }
}

impl From<ActivityType> for u8 {
    fn from(value: ActivityType) -> Self {
        match value {
            ActivityType::Playing => 0,
            ActivityType::Streaming => 1,
            ActivityType::Listening => 2,
            ActivityType::Watching => 3,
            ActivityType::Custom => 4,
            ActivityType::Competing => 5,
        }
    }
}

/// User status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Online.
    #[default]
    Online,
    /// Do Not Disturb.
    Dnd,
    /// Away / Idle.
    Idle,
    /// Invisible (shown as offline).
    Invisible,
    /// Offline.
    Offline,
}

// ============================================================================
// Resume Payload (Sent to resume a session)
// ============================================================================

/// Payload for the Resume opcode (op 6).
#[derive(Debug, Clone, Serialize)]
pub struct ResumePayload<'a> {
    /// Authentication token.
    pub token: std::borrow::Cow<'a, str>,

    /// Session ID from the previous Ready event.
    pub session_id: std::borrow::Cow<'a, str>,

    /// Last sequence number received.
    pub seq: u64,
}

// ============================================================================
// Ready Event (Received after successful Identify)
// ============================================================================

/// Payload for the READY dispatch event.
#[derive(Debug, Clone, Deserialize)]
pub struct ReadyEvent<'a> {
    /// Gateway protocol version.
    pub v: u8,

    /// Current user.
    #[serde(borrow)]
    pub user: User<'a>,

    /// Guilds the user is in (unavailable during initial connection).
    pub guilds: Vec<UnavailableGuild>,

    /// Session ID for resuming.
    pub session_id: String,

    /// URL to use for resuming the session.
    pub resume_gateway_url: String,

    /// Shard information: [shard_id, total_shards].
    #[serde(default)]
    pub shard: Option<[u16; 2]>,

    /// Application information.
    pub application: Application,
}

// ============================================================================
// Heartbeat Payload
// ============================================================================

/// Create a Heartbeat payload.
///
/// The heartbeat payload is just the sequence number (or null if no events received).
pub fn create_heartbeat_payload(sequence: Option<u64>) -> String {
    match sequence {
        Some(seq) => format!(r#"{{"op":1,"d":{}}}"#, seq),
        None => r#"{"op":1,"d":null}"#.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_payload() {
        let json = r#"{"heartbeat_interval": 41250}"#;
        let payload: HelloPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.heartbeat_interval, 41250);
    }

    #[test]
    fn test_identify_serialization() {
        let identify =
            IdentifyPayload::new("test_token", Intents::GUILDS | Intents::GUILD_MESSAGES)
                .with_shard(0, 1);

        let json = serde_json::to_string(&identify).unwrap();
        assert!(json.contains("test_token"));
        assert!(json.contains("shard"));
    }

    #[test]
    fn test_heartbeat_payload() {
        let payload = create_heartbeat_payload(Some(42));
        assert_eq!(payload, r#"{"op":1,"d":42}"#);

        let payload_null = create_heartbeat_payload(None);
        assert_eq!(payload_null, r#"{"op":1,"d":null}"#);
    }
}
