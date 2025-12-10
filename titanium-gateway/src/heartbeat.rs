use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Handles heartbeat logic for a Shard.
#[derive(Debug)]
pub struct HeartbeatHandler {
    /// Heartbeat interval.
    interval: RwLock<Duration>,
    /// Last time a heartbeat was sent.
    last_heartbeat: RwLock<Instant>,
    /// Last measured round-trip time.
    last_rtt: RwLock<Option<Duration>>,
    /// Whether the last heartbeat was acknowledged.
    ack_received: AtomicBool,
}

impl Default for HeartbeatHandler {
    fn default() -> Self {
        Self {
            interval: RwLock::new(Duration::from_millis(45000)),
            last_heartbeat: RwLock::new(Instant::now()),
            last_rtt: RwLock::new(None),
            ack_received: AtomicBool::new(true),
        }
    }
}

impl HeartbeatHandler {
    /// Create a new heartbeat handler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the heartbeat interval.
    pub fn set_interval(&self, interval: Duration) {
        *self.interval.write() = interval;
    }

    /// Get the current heartbeat interval.
    pub fn interval(&self) -> Duration {
        *self.interval.read()
    }

    /// Reset for a new connection.
    pub fn reset(&self) {
        self.ack_received.store(true, Ordering::SeqCst);
        *self.last_heartbeat.write() = Instant::now();
    }

    /// Mark a heartbeat as sent.
    pub fn mark_sent(&self) {
        self.ack_received.store(false, Ordering::SeqCst);
        *self.last_heartbeat.write() = Instant::now();
    }

    /// Mark a heartbeat as acknowledged.
    pub fn mark_acked(&self) {
        let now = Instant::now();
        let last = *self.last_heartbeat.read();
        let rtt = now.duration_since(last);

        self.ack_received.store(true, Ordering::SeqCst);
        *self.last_rtt.write() = Some(rtt);
    }

    /// Check if the last heartbeat was acknowledged.
    pub fn is_acked(&self) -> bool {
        self.ack_received.load(Ordering::SeqCst)
    }

    /// Get the last measured RTT.
    pub fn latency(&self) -> Option<Duration> {
        *self.last_rtt.read()
    }

    /// Manually set the stored RTT (e.g. for testing).
    #[allow(dead_code)]
    pub fn set_latency(&self, rtt: Option<Duration>) {
        *self.last_rtt.write() = rtt;
    }
}
