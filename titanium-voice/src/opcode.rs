//! Voice Gateway opcodes.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Voice Gateway operation codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VoiceOpCode {
    /// Begin speaking.
    Identify = 0,
    /// Select voice protocol.
    SelectProtocol = 1,
    /// Response to Identify, contains SSRC and other info.
    Ready = 2,
    /// Keep connection alive.
    Heartbeat = 3,
    /// Response to SelectProtocol, contains encryption key.
    SessionDescription = 4,
    /// Indicate speaking state to server.
    Speaking = 5,
    /// Acknowledgment of heartbeat.
    HeartbeatAck = 6,
    /// Resume voice connection.
    Resume = 7,
    /// Initial connection info.
    Hello = 8,
    /// Resume acknowledged.
    Resumed = 9,
    /// Someone joined/left the voice channel.
    ClientConnect = 12,
    /// A client disconnected.
    ClientDisconnect = 13,
    /// Dave protocol request (E2EE).
    DaveProtocolPrepareTransition = 21,
    /// Dave protocol execute transition.
    DaveProtocolExecuteTransition = 22,
    /// Dave protocol ready.
    DaveProtocolTransitionReady = 23,
    /// Dave protocol prepare epoch.
    DaveProtocolPrepareEpoch = 24,
    /// Dave MLS external sender.
    DaveMlsExternalSender = 25,
    /// Dave MLS key package.
    DaveMlsKeyPackage = 26,
    /// Dave MLS proposals.
    DaveMlsProposals = 27,
    /// Dave MLS commit welcome.
    DaveMlsCommitWelcome = 28,
    /// Dave MLS announce commit transition.
    DaveMlsAnnounceCommitTransition = 29,
    /// Dave MLS welcome.
    DaveMlsWelcome = 30,
    /// Dave MLS invalid commit welcome.
    DaveMlsInvalidCommitWelcome = 31,
}

impl VoiceOpCode {
    /// Whether this opcode is for receiving.
    pub fn is_receive(self) -> bool {
        matches!(
            self,
            VoiceOpCode::Ready
                | VoiceOpCode::SessionDescription
                | VoiceOpCode::HeartbeatAck
                | VoiceOpCode::Hello
                | VoiceOpCode::Resumed
                | VoiceOpCode::ClientConnect
                | VoiceOpCode::ClientDisconnect
        )
    }

    /// Whether this opcode is for sending.
    pub fn is_send(self) -> bool {
        matches!(
            self,
            VoiceOpCode::Identify
                | VoiceOpCode::SelectProtocol
                | VoiceOpCode::Heartbeat
                | VoiceOpCode::Speaking
                | VoiceOpCode::Resume
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_serialization() {
        let json = serde_json::to_string(&VoiceOpCode::Hello).unwrap();
        assert_eq!(json, "8");

        let op: VoiceOpCode = serde_json::from_str("8").unwrap();
        assert_eq!(op, VoiceOpCode::Hello);
    }
}
