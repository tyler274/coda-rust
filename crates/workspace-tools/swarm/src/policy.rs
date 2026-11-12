//! Every tunable swarm limit and the wake rule live here.
//!
//! Values marked *placeholder* are not yet confirmed against the reference
//! implementation and will be replaced once its parameters are known. Values
//! marked *observed* match documented reference behavior.

use std::num::NonZeroU32;
use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::message::{Message, MessageKind};
use crate::names::MemberName;
use crate::roster::{Member, MemberState, Roster, WakeMode};

/// Observed: channel names have at most 24 characters.
pub const MAX_CHANNEL_NAME_LEN: usize = 24;
/// Placeholder: longest member name, group suffix included.
pub const MAX_MEMBER_NAME_LEN: usize = 32;
/// Placeholder: longest channel topic, in characters.
pub const MAX_TOPIC_LEN: usize = 120;
/// Observed: names that can never be created as channels.
pub const RESERVED_CHANNEL_NAMES: [&str; 3] = ["main", "all", "here"];
/// Names that would collide with broadcast mentions.
pub const RESERVED_MEMBER_NAMES: [&str; 2] = ["all", "here"];

/// At most `max_posts` ordinary posts per author within any `window`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    pub max_posts: NonZeroU32,
    pub window: Duration,
}

impl RateLimit {
    /// Checks a new post at `now` against the author's earlier post times.
    /// On refusal, returns how long until the oldest post leaves the window.
    pub fn check(
        &self,
        earlier: impl IntoIterator<Item = DateTime<Utc>>,
        now: DateTime<Utc>,
    ) -> Result<(), Duration> {
        let window = chrono::Duration::from_std(self.window).unwrap_or(chrono::Duration::MAX);
        let start = now
            .checked_sub_signed(window)
            .unwrap_or(DateTime::<Utc>::MIN_UTC);
        let in_window: Vec<DateTime<Utc>> =
            earlier.into_iter().filter(|time| *time > start).collect();
        if in_window.len() < self.max_posts.get() as usize {
            return Ok(());
        }
        let oldest = in_window.iter().min().copied().unwrap_or(now);
        Err((oldest - start).to_std().unwrap_or(Duration::ZERO))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub rate: RateLimit,
    /// Placeholder: largest post text in UTF-8 bytes.
    pub max_post_bytes: usize,
    /// Observed: `swarm_wait` parks on mentions unless asked otherwise.
    pub default_wake: WakeMode,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            // Placeholder: 6 posts per 30 seconds.
            rate: RateLimit {
                max_posts: NonZeroU32::new(6).expect("nonzero literal"),
                window: Duration::from_secs(30),
            },
            max_post_bytes: 16 * 1024,
            default_wake: WakeMode::Mentions,
        }
    }
}

/// Whether `message` wakes the parked member `name`.
///
/// Only a waiting member can be rung, never by its own post or a brief, and
/// only by a post after the seq it parked on. A mentions-only member needs to be named;
/// any other waiting member wakes for any post it can see.
pub fn rings(roster: &Roster, name: &MemberName, member: &Member, message: &Message) -> bool {
    let MemberState::Waiting {
        wake_mode,
        wait_since,
    } = member.state
    else {
        return false;
    };
    if message.kind != MessageKind::Post || message.agent == *name || message.seq <= wait_since {
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

    fn at(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_800_000_000 + secs, 0).expect("valid timestamp")
    }

    fn limit(max: u32, secs: u64) -> RateLimit {
        RateLimit {
            max_posts: NonZeroU32::new(max).expect("nonzero"),
            window: Duration::from_secs(secs),
        }
    }

    #[test]
    fn rate_limit_allows_up_to_max_within_window() {
        let rate = limit(2, 10);
        assert_eq!(rate.check([at(0)], at(1)), Ok(()));
        assert_eq!(
            rate.check([at(0), at(1)], at(2)),
            Err(Duration::from_secs(8))
        );
    }

    #[test]
    fn rate_limit_forgets_posts_outside_window() {
        let rate = limit(2, 10);
        assert_eq!(rate.check([at(0), at(1)], at(10)), Ok(()));
    }
}
