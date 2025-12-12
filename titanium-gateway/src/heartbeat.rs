use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Handles heartbeating.
#[derive(Debug)]
pub struct HeartbeatHandler {
    interval_ms: AtomicU64,
    last_heartbeat: RwLock<Instant>,
    last_rtt_ms: AtomicU64, // u64::MAX = None
    ack_received: AtomicBool,
}

impl HeartbeatHandler {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval_ms: AtomicU64::new(interval.as_millis() as u64),
            last_heartbeat: RwLock::new(Instant::now()),
            last_rtt_ms: AtomicU64::new(u64::MAX),
            ack_received: AtomicBool::new(true),
        }
    }

    pub fn interval(&self) -> Duration {
        Duration::from_millis(self.interval_ms.load(Ordering::Acquire))
    }

    pub fn set_interval(&self, interval: Duration) {
        self.interval_ms
            .store(interval.as_millis() as u64, Ordering::Release);
    }

    pub fn reset(&self) {
        self.ack_received.store(true, Ordering::SeqCst);
        *self.last_heartbeat.write() = Instant::now();
    }

    pub fn mark_sent(&self) {
        self.ack_received.store(false, Ordering::SeqCst);
        *self.last_heartbeat.write() = Instant::now();
    }

    pub fn mark_acked(&self) {
        let now = Instant::now();
        let last = *self.last_heartbeat.read();
        let rtt = now.duration_since(last);

        self.ack_received.store(true, Ordering::SeqCst);
        self.last_rtt_ms
            .store(rtt.as_millis() as u64, Ordering::Release);
    }

    pub fn is_acked(&self) -> bool {
        self.ack_received.load(Ordering::SeqCst)
    }

    pub fn latency(&self) -> Option<Duration> {
        let ms = self.last_rtt_ms.load(Ordering::Acquire);
        if ms == u64::MAX {
            None
        } else {
            Some(Duration::from_millis(ms))
        }
    }
}

impl Default for HeartbeatHandler {
    fn default() -> Self {
        Self::new(Duration::from_millis(45000))
    }
}
