//! How one night ended.
//!
//! Every night records exactly one outcome: it succeeded, it was deferred, or
//! it failed. Only [`Succeeded`] is a success, and nothing outside this crate
//! can make one. [`Store::finish`](crate::Store::finish) derives a finished
//! night's outcome from its own [`Tally`] of what was due and done under the
//! night's lease, so a runner can't report a success on its own word.
//!
//! ```compile_fail
//! use clinical_library::Succeeded;
//! // Succeeded's fields are private, so code that merely deferred can't
//! // build one to log a success.
//! let _ = Succeeded { activated: None };
//! ```
//!
//! ```compile_fail
//! use clinical_library::NightOutcome;
//! // No public constructor makes a success: only the store's tally does.
//! let _ = NightOutcome::nothing_due();
//! ```
//!
//! ```compile_fail
//! use clinical_library::Tally;
//! // Nor can a runner write its own tally.
//! let _ = Tally { due_left: 0, done: 4, set_aside: 0, needs_version: false };
//! ```
//!
//! ```
//! use clinical_library::{DeferralReason, NightOutcome};
//! // A deferral is never a success.
//! let night = NightOutcome::deferred(DeferralReason::WaitingForCapacity);
//! assert!(night.success().is_none());
//! ```

use crate::candidate::VersionId;
use crate::failure::FailureCode;
use crate::stop_rule::StopReason;

/// How one night ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NightOutcome {
    /// Everything due was done and published, or nothing was due.
    Succeeded(Succeeded),
    /// The night didn't finish what was due, for a reason that isn't a fault
    /// in its own work.
    Deferred(Deferred),
    /// The night's work failed.
    Failed(Failed),
}

impl NightOutcome {
    /// The outcome a finished night's tally earns. `activated` is the
    /// version the night activated, already checked to be its own.
    ///
    /// - Due work left undone, or content still awaiting a version when the
    ///   night finishes, is a fault in the night's own work: it fails. An
    ///   activation earlier in the night doesn't publish content admitted
    ///   after it.
    /// - Nothing set aside is a success.
    /// - Everything due set aside moved nothing.
    /// - Otherwise it published part of what was due.
    pub(crate) fn from_tally(tally: &Tally, activated: Option<VersionId>) -> Self {
        if tally.due_left > 0 || tally.needs_version {
            return Self::failed(FailureCode::known(FailureCode::DUE_WORK_LEFT_UNDONE));
        }
        match (tally.done, tally.set_aside) {
            (_, 0) => Self::Succeeded(Succeeded { activated }),
            (0, _) => Self::deferred(DeferralReason::MovedNothing),
            _ => Self::deferred(DeferralReason::PublishedWithSetAside),
        }
    }

    /// A night that didn't finish what was due, and why. It is never a
    /// success.
    pub fn deferred(reason: DeferralReason) -> Self {
        Self::Deferred(Deferred { reason })
    }

    /// A night whose work failed, and the cause.
    pub fn failed(code: FailureCode) -> Self {
        Self::Failed(Failed { code })
    }

    /// The success, if the night succeeded. Only this can record a success.
    pub fn success(&self) -> Option<&Succeeded> {
        match self {
            Self::Succeeded(succeeded) => Some(succeeded),
            Self::Deferred(_) | Self::Failed(_) => None,
        }
    }

    /// How this night bears on "three failed nights in a row".
    pub fn streak(&self) -> Streak {
        match self {
            Self::Succeeded(_) => Streak::Breaks,
            Self::Failed(_) => Streak::Counts,
            Self::Deferred(deferred) => match deferred.reason {
                // That night belonged to the run that held the slot, or the
                // nightly had already stopped.
                DeferralReason::SlotHeldByLiveRun | DeferralReason::Stopped(_) => Streak::Leaves,
                // It moved something.
                DeferralReason::PublishedWithSetAside => Streak::Breaks,
                DeferralReason::OwnerUnreadable
                | DeferralReason::WaitingForCapacity
                | DeferralReason::MovedNothing => Streak::Counts,
            },
        }
    }

    /// The outcome's name, as stored.
    pub const fn kind(&self) -> OutcomeKind {
        match self {
            Self::Succeeded(_) => OutcomeKind::Succeeded,
            Self::Deferred(_) => OutcomeKind::Deferred,
            Self::Failed(_) => OutcomeKind::Failed,
        }
    }
}

/// How one night bears on "three failed nights in a row".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Streak {
    /// It failed or moved nothing, so it counts.
    Counts,
    /// It succeeded or moved something, so the streak starts over.
    Breaks,
    /// It neither counts nor breaks the streak: a live run held the slot, or
    /// the nightly had already stopped.
    Leaves,
}

/// What a night was due to do and what it did, as the store counted it from
/// the moves made under the night's lease. Only the store makes one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub(crate) due_left: u64,
    pub(crate) done: u64,
    pub(crate) set_aside: u64,
    pub(crate) needs_version: bool,
}

impl Tally {
    /// Items due tonight that the night neither admitted nor set aside.
    pub const fn due_left(&self) -> u64 {
        self.due_left
    }

    /// Items the night admitted.
    pub const fn done(&self) -> u64 {
        self.done
    }

    /// Items the night set aside.
    pub const fn set_aside(&self) -> u64 {
        self.set_aside
    }

    /// Whether content of the night's sources still awaits a version: one
    /// the night must activate after admitting it.
    pub const fn needs_version(&self) -> bool {
        self.needs_version
    }
}

/// A night that succeeded. Only the store's tally makes one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Succeeded {
    activated: Option<VersionId>,
}

impl Succeeded {
    /// The version the night activated, or `None` if it needed none.
    pub fn activated(&self) -> Option<&VersionId> {
        self.activated.as_ref()
    }
}

/// A night that didn't finish what was due.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deferred {
    reason: DeferralReason,
}

impl Deferred {
    /// Why it deferred.
    pub const fn reason(&self) -> &DeferralReason {
        &self.reason
    }
}

/// A night whose work failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failed {
    code: FailureCode,
}

impl Failed {
    /// What failed.
    pub fn code(&self) -> &FailureCode {
        &self.code
    }
}

/// Why a night deferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeferralReason {
    /// Another run that is still alive held the slot.
    SlotHeldByLiveRun,
    /// The run holding the slot couldn't be confirmed alive or ended, so
    /// this night waited rather than presume it ended.
    OwnerUnreadable,
    /// No build capacity was available for the whole night.
    WaitingForCapacity,
    /// It published, but set some items aside.
    PublishedWithSetAside,
    /// It had work due and set all of it aside, so it moved nothing.
    MovedNothing,
    /// Automatic attempts had stopped, so it didn't run. A person must
    /// record a resolution first.
    Stopped(StopReason),
}

impl DeferralReason {
    /// The stored name.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SlotHeldByLiveRun => "slot_held_by_live_run",
            Self::OwnerUnreadable => "owner_unreadable",
            Self::WaitingForCapacity => "waiting_for_capacity",
            Self::PublishedWithSetAside => "published_with_set_aside",
            Self::MovedNothing => "moved_nothing",
            Self::Stopped(_) => "stopped",
        }
    }

    /// Reads a stored name, with the stop reason a stopped night stored.
    pub(crate) fn from_stored(value: &str, stopped: Option<StopReason>) -> Option<Self> {
        let reason = match (value, stopped) {
            ("stopped", Some(why)) => Self::Stopped(why),
            (_, Some(_)) => return None,
            ("slot_held_by_live_run", None) => Self::SlotHeldByLiveRun,
            ("owner_unreadable", None) => Self::OwnerUnreadable,
            ("waiting_for_capacity", None) => Self::WaitingForCapacity,
            ("published_with_set_aside", None) => Self::PublishedWithSetAside,
            ("moved_nothing", None) => Self::MovedNothing,
            _ => return None,
        };
        Some(reason)
    }
}

/// An outcome's name, as stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutcomeKind {
    /// See [`NightOutcome::Succeeded`].
    Succeeded,
    /// See [`NightOutcome::Deferred`].
    Deferred,
    /// See [`NightOutcome::Failed`].
    Failed,
}

impl OutcomeKind {
    /// The stored name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Deferred => "deferred",
            Self::Failed => "failed",
        }
    }
}

/// Rebuilds an outcome from its stored parts. The store is the only caller,
/// and only for rows it wrote from a real outcome.
pub(crate) fn from_stored(
    kind: &str,
    version: Option<VersionId>,
    reason: Option<DeferralReason>,
    code: Option<FailureCode>,
) -> Option<NightOutcome> {
    match (kind, reason, code) {
        ("succeeded", None, None) => {
            Some(NightOutcome::Succeeded(Succeeded { activated: version }))
        }
        ("deferred", Some(reason), None) if version.is_none() => {
            Some(NightOutcome::deferred(reason))
        }
        ("failed", None, Some(code)) if version.is_none() => Some(NightOutcome::failed(code)),
        _ => None,
    }
}

/// A success with nothing activated, for this crate's own tests of the rules
/// that read the history.
#[cfg(test)]
pub(crate) fn test_success() -> NightOutcome {
    NightOutcome::Succeeded(Succeeded { activated: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tally(due_left: u64, done: u64, set_aside: u64, needs_version: bool) -> Tally {
        Tally {
            due_left,
            done,
            set_aside,
            needs_version,
        }
    }

    fn v2() -> VersionId {
        VersionId::new("v2").unwrap()
    }

    /// AC8: only a tally with nothing left due and nothing set aside is a
    /// success, and due work left undone fails the night.
    #[test]
    fn only_complete_tallies_succeed() {
        assert!(
            NightOutcome::from_tally(&Tally::default(), None)
                .success()
                .is_some()
        );
        assert!(
            NightOutcome::from_tally(&tally(0, 4, 0, false), None)
                .success()
                .is_some()
        );
        let published = NightOutcome::from_tally(&tally(0, 4, 0, false), Some(v2()));
        assert_eq!(
            published.success().and_then(Succeeded::activated),
            Some(&v2())
        );

        let undone = NightOutcome::from_tally(&tally(1, 4, 0, false), None);
        assert_eq!(
            undone,
            NightOutcome::failed(FailureCode::known(FailureCode::DUE_WORK_LEFT_UNDONE))
        );
        let unpublished = NightOutcome::from_tally(&tally(0, 4, 0, true), None);
        assert_eq!(unpublished, undone);
        // Content admitted after the night's activation still awaits one.
        let admitted_after = NightOutcome::from_tally(&tally(0, 4, 0, true), Some(v2()));
        assert_eq!(admitted_after, undone);

        let partial = NightOutcome::from_tally(&tally(0, 3, 1, false), None);
        assert_eq!(
            partial,
            NightOutcome::deferred(DeferralReason::PublishedWithSetAside)
        );
        assert_eq!(partial.streak(), Streak::Breaks);
        let nothing = NightOutcome::from_tally(&tally(0, 0, 2, false), None);
        assert_eq!(
            nothing,
            NightOutcome::deferred(DeferralReason::MovedNothing)
        );
        assert_eq!(nothing.streak(), Streak::Counts);
    }

    /// AC3: a deferral or a failure is never a success, and every stored
    /// name reads back.
    #[test]
    fn deferrals_and_failures_are_never_successes() {
        let stop = StopReason::SameFailureTwice(FailureCode::new("build_failed").unwrap());
        for reason in [
            DeferralReason::SlotHeldByLiveRun,
            DeferralReason::OwnerUnreadable,
            DeferralReason::WaitingForCapacity,
            DeferralReason::PublishedWithSetAside,
            DeferralReason::MovedNothing,
            DeferralReason::Stopped(stop.clone()),
        ] {
            let stopped = match &reason {
                DeferralReason::Stopped(why) => Some(why.clone()),
                _ => None,
            };
            assert_eq!(
                DeferralReason::from_stored(reason.as_str(), stopped).as_ref(),
                Some(&reason)
            );
            assert!(NightOutcome::deferred(reason).success().is_none());
        }
        assert_eq!(DeferralReason::from_stored("stopped", None), None);
        assert_eq!(
            DeferralReason::from_stored("moved_nothing", Some(stop.clone())),
            None
        );
        assert_eq!(
            NightOutcome::deferred(DeferralReason::Stopped(stop)).streak(),
            Streak::Leaves
        );
        let failed = NightOutcome::failed(FailureCode::new("build_failed").unwrap());
        assert!(failed.success().is_none());
    }
}
