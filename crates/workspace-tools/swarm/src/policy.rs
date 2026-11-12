//! Every tunable swarm limit and the wake rule live here.
//!
//! Values marked *placeholder* are not yet confirmed against the reference
//! implementation and will be replaced once its parameters are known. Values
//! marked *observed* match documented reference behavior.

use std::num::NonZeroU32;
use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::message::{Body, Message, MessageKind};
use crate::names::MemberName;
use crate::roster::{Member, MemberState, Roster, WakeMode};

/// Observed: channel names have at most 24 bytes.
pub const MAX_CHANNEL_NAME_LEN: usize = 24;
/// Observed: member names have at most 32 bytes, group suffix included.
pub const MAX_MEMBER_NAME_LEN: usize = 32;
/// Placeholder: longest channel topic, in characters.
pub const MAX_TOPIC_LEN: usize = 120;
/// Observed: names that can never be created as channels.
pub const RESERVED_CHANNEL_NAMES: [&str; 3] = ["main", "all", "here"];
/// Observed: the member name that would collide with `@all`.
pub const RESERVED_MEMBER_NAME: &str = "all";

/// Observed: one token bucket over every plain post in the swarm, whoever
/// posts it. It starts full at `burst`, refills at `per_second`, and each
/// admitted post spends one token. Briefs and control rows are not posts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    pub per_second: NonZeroU32,
    pub burst: NonZeroU32,
}

impl RateLimit {
    /// Checks a post at `now` against the times of the posts admitted
    /// before it, in order. Replaying them gives the same bucket an
    /// in-memory limiter would hold, so every process sharing the board
    /// agrees. On refusal, returns how long until a token is available.
    pub fn check(
        &self,
        earlier: impl IntoIterator<Item = DateTime<Utc>>,
        now: DateTime<Utc>,
    ) -> Result<(), Duration> {
        let burst = f64::from(self.burst.get());
        let rate = f64::from(self.per_second.get());
        let refill = |tokens: f64, from: DateTime<Utc>, to: DateTime<Utc>| {
            let secs = (to - from).to_std().map_or(0.0, |d| d.as_secs_f64());
            (tokens + secs * rate).min(burst)
        };
        let mut tokens = burst;
        let mut last: Option<DateTime<Utc>> = None;
        for time in earlier {
            if let Some(last) = last {
                tokens = refill(tokens, last, time);
            }
            tokens -= 1.0;
            last = Some(time);
        }
        if let Some(last) = last {
            tokens = refill(tokens, last, now);
        }
        if tokens >= 1.0 {
            return Ok(());
        }
        Err(Duration::from_secs_f64((1.0 - tokens) / rate))
    }

    /// The sustained limit in posts a minute, as the rejection reports it.
    pub fn per_minute(&self) -> u32 {
        self.per_second.get().saturating_mul(60)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub rate: RateLimit,
    /// Observed: largest post text in UTF-8 bytes.
    pub max_post_bytes: usize,
    /// Observed: `swarm_wait` parks on mentions unless asked otherwise.
    pub default_wake: WakeMode,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            // Observed: 10 tokens a second with a burst of 600.
            rate: RateLimit {
                per_second: NonZeroU32::new(10).expect("nonzero literal"),
                burst: NonZeroU32::new(600).expect("nonzero literal"),
            },
            max_post_bytes: 64 * 1024,
            default_wake: WakeMode::Mentions,
        }
    }
}

/// Whether `message` wakes the parked member `name`.
///
/// Only a waiting member can be rung, only by a plain post (never a brief
/// or a control row), never by its own post, and only by a post after the
/// seq it parked on. A mentions-only member needs to be named; any other
/// waiting member wakes for any post it can see.
pub fn rings(roster: &Roster, name: &MemberName, member: &Member, message: &Message) -> bool {
    let MemberState::Waiting {
        wake_mode,
        wait_since,
    } = member.state
    else {
        return false;
    };
    if message.body != Body::Post(MessageKind::Post)
        || message.agent == *name
        || message.seq <= wait_since
    {
        return false;
    }
    let mentions = message.mentions();
    match wake_mode {
        WakeMode::Mentions => roster.is_mentioned(name, member, &mentions),
        WakeMode::Anything => roster.sees(name, member, message, &mentions),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at_ms(ms: i64) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(1_800_000_000_000 + ms).expect("valid timestamp")
    }

    #[test]
    fn bucket_starts_full_and_allows_a_burst_of_600() {
        let rate = Policy::default().rate;
        let burst: Vec<_> = (0..599).map(|_| at_ms(0)).collect();
        assert_eq!(rate.check(burst.clone(), at_ms(0)), Ok(()));
        let full: Vec<_> = (0..600).map(|_| at_ms(0)).collect();
        assert_eq!(
            rate.check(full.clone(), at_ms(0)),
            Err(Duration::from_millis(100))
        );
        assert_eq!(rate.per_minute(), 600);
    }

    #[test]
    fn bucket_refills_ten_tokens_a_second() {
        let rate = Policy::default().rate;
        let full: Vec<_> = (0..600).map(|_| at_ms(0)).collect();
        assert!(rate.check(full.clone(), at_ms(99)).is_err());
        assert_eq!(rate.check(full.clone(), at_ms(100)), Ok(()));
        let mut drained = full;
        drained.extend((0..10).map(|_| at_ms(1_000)));
        assert!(rate.check(drained.clone(), at_ms(1_000)).is_err());
        assert_eq!(rate.check(drained, at_ms(1_100)), Ok(()));
    }

    #[test]
    fn bucket_never_holds_more_than_the_burst() {
        let rate = Policy::default().rate;
        let mut posts = vec![at_ms(0)];
        posts.extend((0..600).map(|_| at_ms(3_600_000)));
        assert!(rate.check(posts, at_ms(3_600_000)).is_err());
    }
}
