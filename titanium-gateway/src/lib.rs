//! Titan Gateway - High-performance Discord Gateway WebSocket client
#![deny(unsafe_code)]
//!
//! This crate provides a robust WebSocket client for Discord's Gateway API,
//! designed for bots scaling to 1M+ guilds.
//!
//! # Features
//!
//! - Zero-copy JSON parsing with optional SIMD acceleration
//! - ETF (Erlang Term Format) support for smaller payloads
//! - Automatic heartbeat management
//! - Session resumption support
//! - Cluster-native shard management
//! - Strict error handling (no unwrap)
//!
//! # Cargo Features
//!
//! - `simd` - Enable SIMD-accelerated JSON parsing (~2-3x faster on supported CPUs)
//! - `etf` - Enable Erlang Term Format encoding (more compact than JSON)
//!
//! # Example
//!
//! ```ignore
//! use titanium_gateway::{Shard, ShardConfig};
//! use titanium_model::Intents;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ShardConfig::new("your-token", Intents::default());
//!     let mut shard = Shard::new(0, 1, config);
//!     
//!     let (event_tx, event_rx) = flume::unbounded();
//!     shard.run(event_tx).await?;
//!     
//!     Ok(())
//! }
//! ```
mod cluster;
mod compression;
pub mod error;
pub mod etf;
pub mod event;
pub mod heartbeat;
mod metrics;
mod opcode;
mod parsing;
mod payload;
mod ratelimit;
mod shard;

// Public re-exports
pub use cluster::{Cluster, ClusterConfig, ShardRange};
pub use compression::{ZlibDecompressor, ZlibTransport};
pub use error::GatewayError;
pub use etf::{EtfDecoder, EtfTerm, GatewayEncoding};
pub use event::Event;
pub use metrics::{GatewayMetrics, ShardMetrics};
pub use opcode::OpCode;
pub use parsing::{from_str, from_string, to_string};
pub use payload::{
    ConnectionProperties, GatewayPayload, HelloPayload, IdentifyPayload, ReadyEvent, ResumePayload,
};
pub use ratelimit::IdentifyRateLimiter;
pub use shard::{Shard, ShardConfig, ShardState};

/// Discord Gateway API version used by this library.
pub const GATEWAY_VERSION: u8 = 10;

/// Default gateway URL (will be overridden by /gateway/bot response).
pub const DEFAULT_GATEWAY_URL: &str = "wss://gateway.discord.gg";
