//! Discord Gateway opcodes.
//!
//! Opcodes define the type of payload being sent or received over the Gateway WebSocket.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Discord Gateway operation codes.
///
/// See: <https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OpCode {
    /// Dispatch - An event was dispatched.
    /// Direction: Receive
    Dispatch = 0,

    /// Heartbeat - Keep the connection alive.
    /// Direction: Send/Receive
    Heartbeat = 1,

    /// Identify - Start a new session.
    /// Direction: Send
    Identify = 2,

    /// Presence Update - Update the client's presence.
    /// Direction: Send
    PresenceUpdate = 3,

    /// Voice State Update - Join/leave/move between voice channels.
    /// Direction: Send
    VoiceStateUpdate = 4,

    /// Resume - Resume a previous session.
    /// Direction: Send
    Resume = 6,

    /// Reconnect - Server requested a reconnect.
    /// Direction: Receive
    Reconnect = 7,

    /// Request Guild Members - Request guild member chunks.
    /// Direction: Send
    RequestGuildMembers = 8,

    /// Invalid Session - Session has been invalidated.
    /// Direction: Receive
    InvalidSession = 9,

    /// Hello - Sent after connecting, contains heartbeat interval.
    /// Direction: Receive
    Hello = 10,

    /// Heartbeat ACK - Acknowledgment of heartbeat received.
    /// Direction: Receive
    HeartbeatAck = 11,

    /// Request Soundboard Sounds - Request soundboard sounds (API v10+).
    /// Direction: Send
    RequestSoundboardSounds = 31,
}

impl OpCode {
    /// Returns whether this opcode is only received (not sent).
    pub const fn is_receive_only(self) -> bool {
        matches!(
            self,
            OpCode::Dispatch
                | OpCode::Reconnect
                | OpCode::InvalidSession
                | OpCode::Hello
                | OpCode::HeartbeatAck
        )
    }

    /// Returns whether this opcode is only sent (not received).
    pub const fn is_send_only(self) -> bool {
        matches!(
            self,
            OpCode::Identify
                | OpCode::PresenceUpdate
                | OpCode::VoiceStateUpdate
                | OpCode::Resume
                | OpCode::RequestGuildMembers
                | OpCode::RequestSoundboardSounds
        )
    }

    /// Returns whether this opcode can be both sent and received.
    pub const fn is_bidirectional(self) -> bool {
        matches!(self, OpCode::Heartbeat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_serialization() {
        let json = serde_json::to_string(&OpCode::Hello).unwrap();
        assert_eq!(json, "10");

        let opcode: OpCode = serde_json::from_str("10").unwrap();
        assert_eq!(opcode, OpCode::Hello);
    }

    #[test]
    fn test_opcode_direction() {
        assert!(OpCode::Dispatch.is_receive_only());
        assert!(OpCode::Identify.is_send_only());
        assert!(OpCode::Heartbeat.is_bidirectional());
    }
}
