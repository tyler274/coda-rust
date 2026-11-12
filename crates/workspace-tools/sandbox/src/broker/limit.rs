//! A token bucket per principal. It surfaces a runaway loop in the
//! container; it is not a secrecy control, since a granted token is held
//! by the caller anyway.

use std::time::{Duration, Instant};

/// How many requests a bucket allows: `burst` at once, refilled at
/// `per_minute`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rate {
    pub per_minute: u32,
    pub burst: u32,
}

impl Rate {
    pub const AGENT: Self = Self {
        per_minute: 30,
        burst: 10,
    };
    pub const OWNER: Self = Self {
        per_minute: 30,
        burst: 10,
    };
}

#[derive(Debug)]
pub struct Bucket {
    rate: Rate,
    tokens: f64,
    refilled: Instant,
}

impl Bucket {
    pub fn new(rate: Rate, now: Instant) -> Self {
        Self {
            rate,
            tokens: f64::from(rate.burst),
            refilled: now,
        }
    }

    /// Takes one request's worth; `false` when the bucket is empty.
    pub fn take(&mut self, now: Instant) -> bool {
        let elapsed = now.saturating_duration_since(self.refilled);
        let refill = elapsed.as_secs_f64() * f64::from(self.rate.per_minute) / 60.0;
        self.tokens = (self.tokens + refill).min(f64::from(self.rate.burst));
        self.refilled = now;
        if self.tokens < 1.0 {
            return false;
        }
        self.tokens -= 1.0;
        true
    }
}

/// How often a limited principal gets an audit line, so a runaway loop
/// does not flood the journal.
pub const LOG_EVERY: Duration = Duration::from_secs(60);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burst_then_refill_at_the_rate() {
        let start = Instant::now();
        let mut bucket = Bucket::new(Rate::AGENT, start);
        for n in 0..10 {
            assert!(bucket.take(start), "request {n} is within the burst");
        }
        assert!(!bucket.take(start), "the 11th at once is limited");
        assert!(!bucket.take(start + Duration::from_secs(1)));
        assert!(
            bucket.take(start + Duration::from_secs(2)),
            "30/min is one per 2 s"
        );
        assert!(!bucket.take(start + Duration::from_secs(2)));
    }

    #[test]
    fn idle_refills_only_up_to_the_burst() {
        let start = Instant::now();
        let mut bucket = Bucket::new(Rate::AGENT, start);
        let later = start + Duration::from_secs(3600);
        let granted = (0..20).filter(|_| bucket.take(later)).count();
        assert_eq!(granted, 10);
    }
}
