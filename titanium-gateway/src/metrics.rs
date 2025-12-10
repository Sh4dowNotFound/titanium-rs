//! Gateway metrics collection.
//!
//! Provides observable metrics for monitoring shard health,
//! event throughput, and connection stability.

use parking_lot::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Metrics for the entire Gateway cluster.
#[derive(Debug, Default)]
pub struct GatewayMetrics {
    /// Total events received across all shards.
    pub events_received: AtomicU64,
    /// Total events dispatched to handlers.
    pub events_dispatched: AtomicU64,
    /// Total WebSocket messages received.
    pub ws_messages_received: AtomicU64,
    /// Total bytes received.
    pub bytes_received: AtomicU64,
    /// Total heartbeats sent.
    pub heartbeats_sent: AtomicU64,
    /// Total heartbeats acknowledged.
    pub heartbeats_acked: AtomicU64,
    /// Total reconnections.
    pub reconnections: AtomicU64,
    /// Total session resumes.
    pub session_resumes: AtomicU64,
    /// Total identifies sent.
    pub identifies_sent: AtomicU64,
}

impl GatewayMetrics {
    /// Create new gateway metrics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment events received counter.
    pub fn inc_events_received(&self) {
        self.events_received.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment events dispatched counter.
    pub fn inc_events_dispatched(&self) {
        self.events_dispatched.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment WebSocket messages received counter.
    pub fn inc_ws_messages(&self) {
        self.ws_messages_received.fetch_add(1, Ordering::Relaxed);
    }

    /// Add bytes to received counter.
    pub fn add_bytes_received(&self, bytes: u64) {
        self.bytes_received.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Increment heartbeats sent counter.
    pub fn inc_heartbeats_sent(&self) {
        self.heartbeats_sent.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment heartbeats acknowledged counter.
    pub fn inc_heartbeats_acked(&self) {
        self.heartbeats_acked.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment reconnections counter.
    pub fn inc_reconnections(&self) {
        self.reconnections.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment session resumes counter.
    pub fn inc_session_resumes(&self) {
        self.session_resumes.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment identifies sent counter.
    pub fn inc_identifies(&self) {
        self.identifies_sent.fetch_add(1, Ordering::Relaxed);
    }

    /// Get a snapshot of all metrics.
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            events_received: self.events_received.load(Ordering::Relaxed),
            events_dispatched: self.events_dispatched.load(Ordering::Relaxed),
            ws_messages_received: self.ws_messages_received.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            heartbeats_sent: self.heartbeats_sent.load(Ordering::Relaxed),
            heartbeats_acked: self.heartbeats_acked.load(Ordering::Relaxed),
            reconnections: self.reconnections.load(Ordering::Relaxed),
            session_resumes: self.session_resumes.load(Ordering::Relaxed),
            identifies_sent: self.identifies_sent.load(Ordering::Relaxed),
        }
    }
}

/// A snapshot of gateway metrics.
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub events_received: u64,
    pub events_dispatched: u64,
    pub ws_messages_received: u64,
    pub bytes_received: u64,
    pub heartbeats_sent: u64,
    pub heartbeats_acked: u64,
    pub reconnections: u64,
    pub session_resumes: u64,
    pub identifies_sent: u64,
}

/// Metrics for a single shard.
#[derive(Debug)]
pub struct ShardMetrics {
    /// Shard ID.
    pub shard_id: u16,
    /// Last heartbeat latency.
    last_heartbeat_latency: RwLock<Duration>,
    /// Connection uptime start.
    connected_at: RwLock<Option<Instant>>,
    /// Events received on this shard.
    pub events_received: AtomicU64,
    /// Guilds on this shard.
    pub guild_count: AtomicU64,
}

impl ShardMetrics {
    /// Create new shard metrics.
    pub fn new(shard_id: u16) -> Self {
        Self {
            shard_id,
            last_heartbeat_latency: RwLock::new(Duration::ZERO),
            connected_at: RwLock::new(None),
            events_received: AtomicU64::new(0),
            guild_count: AtomicU64::new(0),
        }
    }

    /// Record heartbeat latency.
    pub fn record_heartbeat_latency(&self, latency: Duration) {
        *self.last_heartbeat_latency.write() = latency;
    }

    /// Get last heartbeat latency.
    pub fn heartbeat_latency(&self) -> Duration {
        *self.last_heartbeat_latency.read()
    }

    /// Mark shard as connected.
    pub fn mark_connected(&self) {
        *self.connected_at.write() = Some(Instant::now());
    }

    /// Mark shard as disconnected.
    pub fn mark_disconnected(&self) {
        *self.connected_at.write() = None;
    }

    /// Get connection uptime.
    pub fn uptime(&self) -> Option<Duration> {
        self.connected_at.read().map(|t| t.elapsed())
    }

    /// Increment events received counter.
    pub fn inc_events(&self) {
        self.events_received.fetch_add(1, Ordering::Relaxed);
    }

    /// Set guild count.
    pub fn set_guild_count(&self, count: u64) {
        self.guild_count.store(count, Ordering::Relaxed);
    }

    /// Increment guild count.
    pub fn inc_guild_count(&self) {
        self.guild_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement guild count.
    pub fn dec_guild_count(&self) {
        self.guild_count.fetch_sub(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_metrics() {
        let metrics = GatewayMetrics::new();
        metrics.inc_events_received();
        metrics.inc_events_received();
        metrics.add_bytes_received(1024);

        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.events_received, 2);
        assert_eq!(snapshot.bytes_received, 1024);
    }

    #[test]
    fn test_shard_metrics() {
        let metrics = ShardMetrics::new(0);
        metrics.mark_connected();
        metrics.record_heartbeat_latency(Duration::from_millis(50));
        metrics.inc_events();

        assert!(metrics.uptime().is_some());
        assert_eq!(metrics.heartbeat_latency(), Duration::from_millis(50));
    }
}
