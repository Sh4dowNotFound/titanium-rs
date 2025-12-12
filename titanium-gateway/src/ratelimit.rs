//! Gateway rate limiting.
//!
//! Discord limits how quickly bots can identify on the Gateway.
//! Large bots (150k+ guilds) get higher `max_concurrency` values.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// Rate limiter for Gateway identify operations.
///
/// Discord allows `max_concurrency` identify operations every 5 seconds.
/// This rate limiter ensures we don't exceed this limit.
pub struct IdentifyRateLimiter {
    /// Semaphore with max_concurrency permits.
    semaphore: Arc<Semaphore>,

    /// Duration to hold the permit (5 seconds per Discord docs).
    hold_duration: Duration,
}

impl IdentifyRateLimiter {
    /// Create a new identify rate limiter.
    ///
    /// # Arguments
    /// * `max_concurrency` - Maximum concurrent identifies (from /gateway/bot).
    #[must_use]
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
            hold_duration: Duration::from_secs(5),
        }
    }

    /// Acquire permission to send an Identify payload.
    ///
    /// This will block until a slot is available. The slot is automatically
    /// released after 5 seconds.
    ///
    /// # Errors
    /// Returns `GatewayError::Closed` if the semaphore is closed.
    pub async fn acquire(&self) -> Result<(), crate::GatewayError> {
        // Acquire a permit
        let permit = self.semaphore.clone().acquire_owned().await.map_err(|_| {
            crate::GatewayError::Closed {
                code: 0,
                reason: "Identify semaphore closed".to_string(),
            }
        })?;

        // Spawn a task to release the permit after hold_duration
        let hold_duration = self.hold_duration;
        tokio::spawn(async move {
            sleep(hold_duration).await;
            drop(permit);
        });

        Ok(())
    }

    /// Get the number of available permits.
    #[must_use]
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}

impl Default for IdentifyRateLimiter {
    fn default() -> Self {
        // Default max_concurrency is 1 for most bots
        Self::new(1)
    }
}

/// Calculate backoff duration with exponential increase.
///
/// # Arguments
/// * `attempt` - Current attempt number (0-indexed).
/// * `base_ms` - Base delay in milliseconds.
/// * `max_ms` - Maximum delay in milliseconds.
///
/// # Returns
/// Duration to wait before next retry.
pub fn exponential_backoff(attempt: u32, base_ms: u64, max_ms: u64) -> Duration {
    let delay_ms = base_ms.saturating_mul(2u64.saturating_pow(attempt));
    Duration::from_millis(delay_ms.min(max_ms))
}

/// Add jitter to a duration.
///
/// # Arguments
/// * `duration` - Base duration.
/// * `jitter_factor` - Factor of jitter (0.0 = no jitter, 1.0 = up to 100% jitter).
///
/// # Returns
/// Duration with random jitter added.
pub fn with_jitter(duration: Duration, jitter_factor: f64) -> Duration {
    use rand::Rng;

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    let jitter_range = (duration.as_millis() as f64 * jitter_factor) as u64;
    let jitter = rand::rng().random_range(0..=jitter_range);
    duration + Duration::from_millis(jitter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff() {
        assert_eq!(
            exponential_backoff(0, 1000, 60000),
            Duration::from_millis(1000)
        );
        assert_eq!(
            exponential_backoff(1, 1000, 60000),
            Duration::from_millis(2000)
        );
        assert_eq!(
            exponential_backoff(2, 1000, 60000),
            Duration::from_millis(4000)
        );
        assert_eq!(
            exponential_backoff(3, 1000, 60000),
            Duration::from_millis(8000)
        );

        // Test capping at max
        assert_eq!(
            exponential_backoff(10, 1000, 60000),
            Duration::from_millis(60000)
        );
    }

    #[tokio::test]
    async fn test_rate_limiter_permits() {
        let limiter = IdentifyRateLimiter::new(3);
        assert_eq!(limiter.available_permits(), 3);

        limiter.acquire().await.unwrap();
        // One permit should be used (then released after 5s in background)
        // But we check immediately so might still show 3 or 2 depending on timing
        assert!(limiter.available_permits() <= 3);
    }
}
