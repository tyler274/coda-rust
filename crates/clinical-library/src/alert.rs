//! What a person should be told, read from the records.
//!
//! One alert reads the records: something needs a person, or the library
//! should have updated and didn't. These are the conditions; where they are
//! sent, and how often reminders repeat, belong to the alerts that read
//! them.

use crate::night::Streak;
use crate::slot::owner_unreadable_needs_alert;
use crate::stop_rule::{self, HistoryEntry, Next, StopReason};

/// How many nights in a row that didn't update the library mean it should
/// have and didn't. A night updates the library when it succeeds or
/// publishes part of what was due, the same progress that breaks the stop
/// rule's streak. It matches the stop rule's three failed nights, and also
/// catches nights the stop rule doesn't count, such as a live run that holds
/// the slot for days.
pub const NOT_UPDATING_AFTER: usize = 3;

/// One condition a person should be told about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Alert {
    /// Automatic attempts stopped, and why.
    Stopped(StopReason),
    /// The slot's holder has been unreadable on consecutive nights.
    OwnerUnreadable,
    /// This many nights in a row, at least [`NOT_UPDATING_AFTER`], neither
    /// succeeded nor published part of what was due.
    NotUpdating {
        /// Nights in a row that didn't update the library.
        nights: usize,
    },
    /// This many items need a person, or wait on our own defect.
    ItemsNeedPerson {
        /// How many.
        count: u64,
    },
}

/// The alerts that hold for a slot's history, oldest first, and the number
/// of its sources' items that need a person.
pub fn from_records(history: &[HistoryEntry], items_needing_person: u64) -> Vec<Alert> {
    let mut alerts = Vec::new();
    if let Next::Stop(why) = stop_rule::decide(history) {
        alerts.push(Alert::Stopped(why));
    }
    if owner_unreadable_needs_alert(history) {
        alerts.push(Alert::OwnerUnreadable);
    }
    let not_updating = history
        .iter()
        .rev()
        .filter_map(|entry| match entry {
            HistoryEntry::Night(night) => Some(night),
            // A resolution doesn't update the library.
            HistoryEntry::Resolved => None,
        })
        .take_while(|night| night.streak() != Streak::Breaks)
        .count();
    if not_updating >= NOT_UPDATING_AFTER {
        alerts.push(Alert::NotUpdating {
            nights: not_updating,
        });
    }
    if items_needing_person > 0 {
        alerts.push(Alert::ItemsNeedPerson {
            count: items_needing_person,
        });
    }
    alerts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::night::{DeferralReason, NightOutcome, test_success};

    fn night(outcome: NightOutcome) -> HistoryEntry {
        HistoryEntry::Night(outcome)
    }

    fn held() -> HistoryEntry {
        night(NightOutcome::deferred(DeferralReason::SlotHeldByLiveRun))
    }

    /// AC7: nights that didn't update the library alert once there are
    /// enough of them in a row, even when the stop rule doesn't count them;
    /// a success, or a night that published part of what was due, clears it.
    #[test]
    fn a_library_that_stops_updating_alerts() {
        assert_eq!(from_records(&[held(), held()], 0), []);
        assert_eq!(
            from_records(&[held(), held(), held()], 0),
            [Alert::NotUpdating { nights: 3 }]
        );
        let partial = || {
            night(NightOutcome::deferred(
                DeferralReason::PublishedWithSetAside,
            ))
        };
        assert_eq!(from_records(&[partial(), partial(), partial()], 0), []);
        assert_eq!(
            from_records(
                &[partial(), held(), HistoryEntry::Resolved, held(), held()],
                0
            ),
            [Alert::NotUpdating { nights: 3 }]
        );
        assert_eq!(
            from_records(&[held(), held(), held(), night(test_success())], 0),
            []
        );
    }

    #[test]
    fn stops_unreadable_owners_and_items_alert() {
        let failed = || {
            night(NightOutcome::failed(
                crate::FailureCode::new("build_failed").unwrap(),
            ))
        };
        let alerts = from_records(&[failed(), failed()], 2);
        assert_eq!(
            alerts,
            [
                Alert::Stopped(StopReason::SameFailureTwice(
                    crate::FailureCode::new("build_failed").unwrap()
                )),
                Alert::ItemsNeedPerson { count: 2 },
            ]
        );
        let unreadable = || night(NightOutcome::deferred(DeferralReason::OwnerUnreadable));
        assert!(from_records(&[unreadable(), unreadable()], 0).contains(&Alert::OwnerUnreadable));
    }
}
