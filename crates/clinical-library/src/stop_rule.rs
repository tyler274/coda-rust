//! When automatic attempts stop and a person is told why.
//!
//! The same failure twice, or three failed nights in a row, stops automatic
//! attempts. The rule reads the nights since the last fresh start: a night
//! that succeeded, or a person recording that they resolved the cause.
//! Earlier nights stay in the history but no longer count.

use crate::failure::FailureCode;
use crate::night::{NightOutcome, Streak};

/// How many failed nights in a row stop automatic attempts.
pub const FAILED_NIGHTS_LIMIT: usize = 3;

/// One entry in the nightly history, oldest first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryEntry {
    /// A night and how it ended.
    Night(NightOutcome),
    /// A person recorded that they resolved what stopped the nightly.
    Resolved,
}

/// What the next night should do.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Next {
    /// Run as usual.
    Continue,
    /// Don't attempt automatically; a person must resolve this first.
    Stop(StopReason),
}

/// Why automatic attempts stopped.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum StopReason {
    /// The same failure happened on two nights.
    #[error("the same failure, {0}, happened on two nights since the last fresh start")]
    SameFailureTwice(FailureCode),
    /// Three nights in a row failed or moved nothing.
    #[error("three nights in a row failed or moved nothing since the last fresh start")]
    FailedNightsInARow,
}

impl StopReason {
    /// The stored name.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SameFailureTwice(_) => "same_failure_twice",
            Self::FailedNightsInARow => "failed_nights_in_a_row",
        }
    }

    /// The repeated failure's code, for a stop on the same failure twice.
    pub const fn code(&self) -> Option<&FailureCode> {
        match self {
            Self::SameFailureTwice(code) => Some(code),
            Self::FailedNightsInARow => None,
        }
    }

    /// Reads a stored name and its code.
    pub(crate) fn from_stored(value: &str, code: Option<FailureCode>) -> Option<Self> {
        match (value, code) {
            ("same_failure_twice", Some(code)) => Some(Self::SameFailureTwice(code)),
            ("failed_nights_in_a_row", None) => Some(Self::FailedNightsInARow),
            _ => None,
        }
    }
}

/// Decides from the history, oldest first, whether the next night may run.
pub fn decide(history: &[HistoryEntry]) -> Next {
    let fresh_start = history.iter().rposition(|entry| match entry {
        HistoryEntry::Resolved => true,
        HistoryEntry::Night(night) => night.success().is_some(),
    });
    let window = fresh_start.map_or(history, |start| &history[start + 1..]);

    let mut seen: Vec<&FailureCode> = Vec::new();
    let mut in_a_row = 0usize;
    for entry in window {
        let HistoryEntry::Night(night) = entry else {
            continue;
        };
        if let NightOutcome::Failed(failed) = night {
            if seen.contains(&failed.code()) {
                return Next::Stop(StopReason::SameFailureTwice(failed.code().clone()));
            }
            seen.push(failed.code());
        }
        match night.streak() {
            Streak::Counts => in_a_row += 1,
            Streak::Breaks => in_a_row = 0,
            Streak::Leaves => {}
        }
        if in_a_row >= FAILED_NIGHTS_LIMIT {
            return Next::Stop(StopReason::FailedNightsInARow);
        }
    }
    Next::Continue
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::night::DeferralReason;

    fn failed(code: &str) -> HistoryEntry {
        HistoryEntry::Night(NightOutcome::failed(FailureCode::new(code).unwrap()))
    }

    fn deferred(reason: DeferralReason) -> HistoryEntry {
        HistoryEntry::Night(NightOutcome::deferred(reason))
    }

    fn succeeded() -> HistoryEntry {
        HistoryEntry::Night(crate::night::test_success())
    }

    fn partial() -> HistoryEntry {
        deferred(DeferralReason::PublishedWithSetAside)
    }

    fn code(value: &str) -> FailureCode {
        FailureCode::new(value).unwrap()
    }

    #[test]
    fn the_same_failure_twice_stops() {
        let history = [failed("build_failed"), failed("build_failed")];
        assert_eq!(
            decide(&history),
            Next::Stop(StopReason::SameFailureTwice(code("build_failed")))
        );
        // Not necessarily on consecutive nights.
        let history = [failed("build_failed"), partial(), failed("build_failed")];
        assert_eq!(
            decide(&history),
            Next::Stop(StopReason::SameFailureTwice(code("build_failed")))
        );
    }

    #[test]
    fn three_different_failed_nights_in_a_row_stop() {
        let history = [failed("a"), failed("b")];
        assert_eq!(decide(&history), Next::Continue);
        let history = [failed("a"), failed("b"), failed("c")];
        assert_eq!(decide(&history), Next::Stop(StopReason::FailedNightsInARow));
    }

    /// Decision table: a deferred night that moved nothing counts toward
    /// three failed nights. This is the case production logged as success.
    #[test]
    fn deferred_nights_that_moved_nothing_count() {
        let history = [
            deferred(DeferralReason::WaitingForCapacity),
            deferred(DeferralReason::OwnerUnreadable),
            failed("a"),
        ];
        assert_eq!(decide(&history), Next::Stop(StopReason::FailedNightsInARow));
        let history = [
            deferred(DeferralReason::WaitingForCapacity),
            deferred(DeferralReason::WaitingForCapacity),
            deferred(DeferralReason::WaitingForCapacity),
        ];
        assert_eq!(decide(&history), Next::Stop(StopReason::FailedNightsInARow));
    }

    /// Decision table: a night deferred only because a live run held the
    /// slot neither counts nor breaks the streak; a night that published part
    /// of what was due breaks it.
    #[test]
    fn live_slot_holders_and_partial_nights() {
        let held = deferred(DeferralReason::SlotHeldByLiveRun);
        let history = [failed("a"), held.clone(), held.clone(), held, failed("b")];
        assert_eq!(decide(&history), Next::Continue);
        let history = [
            failed("a"),
            deferred(DeferralReason::SlotHeldByLiveRun),
            failed("b"),
            failed("c"),
        ];
        assert_eq!(decide(&history), Next::Stop(StopReason::FailedNightsInARow));
        let history = [failed("a"), failed("b"), partial(), failed("c")];
        assert_eq!(decide(&history), Next::Continue);
    }

    /// Decision table: a night the stop rule kept from running neither
    /// counts nor breaks the streak, so a stop holds until a fresh start.
    #[test]
    fn stopped_nights_leave_the_streak() {
        let stopped = deferred(DeferralReason::Stopped(StopReason::FailedNightsInARow));
        let history = [
            failed("a"),
            stopped.clone(),
            failed("b"),
            stopped,
            failed("c"),
        ];
        assert_eq!(decide(&history), Next::Stop(StopReason::FailedNightsInARow));
    }

    #[test]
    fn stop_reasons_read_back_and_say_why() {
        for why in [
            StopReason::SameFailureTwice(code("build_failed")),
            StopReason::FailedNightsInARow,
        ] {
            assert_eq!(
                StopReason::from_stored(why.as_str(), why.code().cloned()),
                Some(why.clone())
            );
            assert!(why.to_string().contains("since the last fresh start"));
        }
        assert_eq!(StopReason::from_stored("same_failure_twice", None), None);
    }

    /// Decision table: a taken-over night counts like any failed night, with
    /// the code it was recorded under, so two takeovers stop the nightly.
    #[test]
    fn taken_over_nights_count_as_failures() {
        let ended =
            || HistoryEntry::Night(NightOutcome::failed(FailureCode::ended_without_outcome()));
        assert_eq!(decide(&[ended()]), Next::Continue);
        assert_eq!(
            decide(&[ended(), ended()]),
            Next::Stop(StopReason::SameFailureTwice(
                FailureCode::ended_without_outcome()
            ))
        );
    }

    /// Decision table: the count starts over after a person records a
    /// resolution or a night succeeds. The earlier nights stay in the
    /// history.
    #[test]
    fn a_resolution_or_a_success_starts_the_count_over() {
        let history = [failed("a"), failed("a"), HistoryEntry::Resolved];
        assert_eq!(decide(&history), Next::Continue);
        let history = [failed("a"), HistoryEntry::Resolved, failed("a")];
        assert_eq!(decide(&history), Next::Continue);
        let history = [failed("a"), failed("b"), succeeded(), failed("c")];
        assert_eq!(decide(&history), Next::Continue);
        let history = [failed("a"), succeeded(), failed("a")];
        assert_eq!(decide(&history), Next::Continue);
        let history = [
            failed("a"),
            HistoryEntry::Resolved,
            failed("a"),
            failed("a"),
        ];
        assert_eq!(
            decide(&history),
            Next::Stop(StopReason::SameFailureTwice(code("a")))
        );
    }
}
