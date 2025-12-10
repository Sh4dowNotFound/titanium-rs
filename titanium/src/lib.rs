//! Titanium - The ultimate high-performance Discord API framework.
//!
//! This crate provides a high-level wrapper around the Titanium ecosystem:
//! - `titanium-gateway`: WebSocket connection management
//! - `titanium-http`: REST API client
//! - `titanium-model`: Discord data models
//! - `titanium-cache`: In-memory caching
//! - `titanium-voice`: Voice support
//!
//! # Example
//!
//! ```no_run
//! use titanium::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::builder("TOKEN")
//!         .intents(Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
//!         .build()
//!         .await
//!         .unwrap();
//!
//!     client.start().await.unwrap();
//! }
//! ```

pub mod client;
pub mod collector;
pub mod context;
pub mod framework;
pub mod prelude;

// Re-exports
pub use titanium_cache as cache;
pub use titanium_gateway as gateway;
pub use titanium_http as http;
pub use titanium_model as model;
pub use titanium_voice as voice;

pub mod error;
pub use error::TitaniumError;

pub use client::Client;
pub use framework::Framework;

#[cfg(feature = "performance")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Incremental build test modification
