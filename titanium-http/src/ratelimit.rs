//! HTTP rate limiting.
//!
//! Implements Discord's bucket-based rate limiting system.

use dashmap::DashMap;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// Rate limiter for Discord API requests.
pub struct RateLimiter {
    /// Per-route buckets.
    buckets: DashMap<String, Arc<Bucket>>,
    /// Global rate limit semaphore.
    #[allow(dead_code)]
    global: Arc<Semaphore>,
    /// Global rate limit until timestamp.
    global_until: Mutex<Option<Instant>>,
}

/// A rate limit bucket for a specific route.
struct Bucket {
    /// Remaining requests in this bucket.
    remaining: Mutex<u32>,
    /// When the bucket resets.
    reset_at: Mutex<Instant>,
    /// Semaphore to queue requests.
    semaphore: Semaphore,
}

impl RateLimiter {
    /// Create a new rate limiter.
    pub fn new() -> Self {
        Self {
            buckets: DashMap::new(),
            global: Arc::new(Semaphore::new(50)), // Discord allows 50 requests/second globally
            global_until: Mutex::new(None),
        }
    }

    /// Acquire permission to make a request to the given route.
    pub async fn acquire(&self, route: &str) -> Result<(), crate::HttpError> {
        // Check global rate limit
        let until = { *self.global_until.lock() };
        if let Some(until) = until {
            if Instant::now() < until {
                sleep(until - Instant::now()).await;
            }
        }

        // Get or create bucket for route
        let bucket = self
            .buckets
            .entry(route.to_string())
            .or_insert_with(|| {
                Arc::new(Bucket {
                    remaining: Mutex::new(1),
                    reset_at: Mutex::new(Instant::now()),
                    semaphore: Semaphore::new(1),
                })
            })
            .clone();

        // Acquire semaphore permit
        let _permit = bucket.semaphore.acquire().await.map_err(|_| {
            crate::HttpError::ClientError("Rate limit semaphore closed".to_string())
        })?;

        // Check if we need to wait for reset
        let wait = {
            let remaining = *bucket.remaining.lock();
            if remaining == 0 {
                let reset_at = *bucket.reset_at.lock();
                if Instant::now() < reset_at {
                    Some(reset_at - Instant::now())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(duration) = wait {
            sleep(duration).await;
        }

        Ok(())
    }

    /// Update rate limit info from response headers.
    pub fn update(&self, route: &str, remaining: u32, reset_after_ms: u64) {
        if let Some(bucket) = self.buckets.get(route) {
            *bucket.remaining.lock() = remaining;
            *bucket.reset_at.lock() = Instant::now() + Duration::from_millis(reset_after_ms);
        }
    }

    /// Set global rate limit.
    pub fn set_global(&self, retry_after_ms: u64) {
        *self.global_until.lock() = Some(Instant::now() + Duration::from_millis(retry_after_ms));
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
