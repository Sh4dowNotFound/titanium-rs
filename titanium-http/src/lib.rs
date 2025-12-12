//! Titan HTTP - Discord REST API client
//!
//! High-performance HTTP client for Discord's REST API with:
//! - Automatic rate limit handling
//! - Request queuing per-route
//! - Retry logic with exponential backoff

#![allow(dead_code)]
pub mod automod;
pub mod channel;
pub mod client;
pub mod emoji;
pub mod error;
pub mod guild;
pub mod interaction;
pub mod monetization;
pub mod ratelimit;
pub mod routes;

pub use client::HttpClient;
pub use error::HttpError;
pub use ratelimit::RateLimiter;
pub use routes::*;
