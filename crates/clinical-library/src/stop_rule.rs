//! When automatic attempts stop and a person is told why.
//!
//! The same failure twice, or three failed nights in a row, stops automatic
//! attempts. The rule reads the nights since the last fresh start: a night
//! that succeeded, or a person recording that they resolved the cause.
//! Earlier nights stay in the history but no longer count.

use crate::failure::FailureCode;
use crate::night::{DeferralReason, NightOutcome};

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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StopReason {
    /// The same failure happened on two nights.
    SameFailureTwice(FailureCode),
    /// Three nights in a row failed or moved nothing.
    FailedNightsInARow,
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
        if night.counts_as_failed_night() {
            in_a_row += 1;
        } else if let NightOutcome::Deferred(deferred) = night
            && deferred.reason() == DeferralReason::SlotHeldByLiveRun
        {
            // That night belonged to the run that held the slot; it neither
            // counts nor breaks the streak.
        } else {
            in_a_row = 0;
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
    use crate::candidate::{Activation, VersionId};

    fn failed(code: &str) -> HistoryEntry {
        HistoryEntry::Night(NightOutcome::failed(FailureCode::new(code).unwrap()))
    }

    fn deferred(reason: DeferralReason) -> HistoryEntry {
        HistoryEntry::Night(NightOutcome::deferred(reason))
    }

    fn succeeded() -> HistoryEntry {
        HistoryEntry::Night(NightOutcome::nothing_due())
    }

    fn partial() -> HistoryEntry {
        let activation = Activation {
            version: VersionId::new("v1").unwrap(),
        };
        HistoryEntry::Night(NightOutcome::published(activation, 2))
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
