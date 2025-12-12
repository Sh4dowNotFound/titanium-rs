//! Titan Voice - Discord Voice Gateway client with E2EE support
#![deny(unsafe_code)]
//!
//! This crate provides a robust voice client for Discord's Voice Gateway,
//! designed for bots scaling to many simultaneous voice connections.
//!
//! # Features
//!
//! - Voice WebSocket signaling
//! - UDP voice transport
//! - XSalsa20-Poly1305 encryption (lite, suffix, and normal modes)
//! - IP discovery
//! - Speaking state management
//! - Experimental Dave E2EE protocol opcodes
//!
//! # Architecture
//!
//! The voice system consists of:
//! - `VoiceConnection` - High-level state machine for voice connections
//! - `VoiceWebSocket` - WebSocket signaling for session management
//! - `VoiceUdp` - UDP transport for audio data
//! - `VoiceCrypto` - Encryption/decryption for voice packets
//!
//! # Example
//!
//! ```ignore
//! use titanium_voice::{VoiceConnection, VoiceConfig};
//!
//! // Get voice_state_update and voice_server_update from gateway
//! let config = VoiceConfig {
//!     guild_id: guild_id,
//!     channel_id: channel_id,
//!     user_id: user_id,
//!     session_id: voice_state.session_id,
//!     endpoint: voice_server.endpoint.unwrap(),
//!     token: voice_server.token,
//! };
//!
//! let connection = VoiceConnection::new(config);
pub mod connection;
pub mod crypto;
pub mod error;
pub mod opcode;
pub mod payload;
pub mod udp;
pub mod ws;

// Re-exports
pub use connection::{VoiceConfig, VoiceConnection, VoiceState};
pub use crypto::{
    build_rtp_header, parse_rtp_header, VoiceCrypto, KEY_SIZE, NONCE_SIZE, RTP_HEADER_SIZE,
    TAG_SIZE,
};
pub use error::{VoiceCloseCode, VoiceError};
pub use opcode::VoiceOpCode;
pub use payload::{
    EncryptionMode, HelloPayload, IdentifyPayload, ReadyPayload, ResumePayload,
    SelectProtocolPayload, SessionDescriptionPayload, SpeakingFlags, SpeakingPayload,
};
pub use udp::VoiceUdp;
pub use ws::{VoiceEvent, VoiceReadyInfo, VoiceSessionDescription, VoiceWebSocket};
