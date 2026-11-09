//! How one night ended.
//!
//! Every night records exactly one outcome: it succeeded, it was deferred, or
//! it failed. Only [`Succeeded`] is a success, and nothing outside this crate
//! can make one: a night succeeds only when there was nothing due, or when it
//! published a version with nothing set aside. A deferred night therefore has
//! no way to report success.
//!
//! ```compile_fail
//! use clinical_library::Succeeded;
//! // Succeeded's fields are private, so code that merely deferred can't
//! // build one to log a success.
//! let _ = Succeeded { activated: None };
//! ```
//!
//! ```
//! use clinical_library::{DeferralReason, NightOutcome};
//! // A deferral is never a success.
//! let night = NightOutcome::deferred(DeferralReason::WaitingForCapacity);
//! assert!(night.success().is_none());
//! ```

use crate::candidate::{Activation, VersionId};
use crate::failure::FailureCode;

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
    /// A night that found nothing due. This is a success.
    pub fn nothing_due() -> Self {
        Self::Succeeded(Succeeded { activated: None })
    }

    /// A night that published `activation`. It succeeds only if it set
    /// nothing aside; otherwise it published part of what was due, which is
    /// progress but not success.
    pub fn published(activation: Activation, set_aside: usize) -> Self {
        if set_aside == 0 {
            Self::Succeeded(Succeeded {
                activated: Some(activation.version),
            })
        } else {
            Self::Deferred(Deferred {
                reason: DeferralReason::PublishedWithSetAside,
            })
        }
    }

    /// A night whose due work needs no new version, such as status that
    /// search reads as it is recorded: `done` items finished and `set_aside`
    /// didn't. It succeeds only if it set nothing aside; one that set
    /// everything aside moved nothing.
    pub fn recorded(done: usize, set_aside: usize) -> Self {
        match (done, set_aside) {
            (_, 0) => Self::Succeeded(Succeeded { activated: None }),
            (0, _) => Self::Deferred(Deferred {
                reason: DeferralReason::MovedNothing,
            }),
            _ => Self::Deferred(Deferred {
                reason: DeferralReason::PublishedWithSetAside,
            }),
        }
    }

    /// A night that didn't finish what was due, and why.
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

    /// Whether this night counts toward "three failed nights in a row".
    ///
    /// A failed night counts. A deferred night counts when it moved nothing,
    /// unless it moved nothing only because a live run held the slot: that
    /// night belongs to the run that held it. A night that published part of
    /// what was due moved something, so it doesn't count.
    pub fn counts_as_failed_night(&self) -> bool {
        match self {
            Self::Succeeded(_) => false,
            Self::Failed(_) => true,
            Self::Deferred(deferred) => match deferred.reason {
                DeferralReason::SlotHeldByLiveRun | DeferralReason::PublishedWithSetAside => false,
                DeferralReason::OwnerUnreadable
                | DeferralReason::WaitingForCapacity
                | DeferralReason::MovedNothing => true,
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

/// A night that succeeded. Only [`NightOutcome::nothing_due`],
/// [`NightOutcome::published`] and [`NightOutcome::recorded`] make one.
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
    pub const fn reason(&self) -> DeferralReason {
        self.reason
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

impl DeferralReason {
    /// The stored name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SlotHeldByLiveRun => "slot_held_by_live_run",
            Self::OwnerUnreadable => "owner_unreadable",
            Self::WaitingForCapacity => "waiting_for_capacity",
            Self::PublishedWithSetAside => "published_with_set_aside",
            Self::MovedNothing => "moved_nothing",
        }
    }

    /// Reads a stored name.
    pub fn parse(value: &str) -> Option<Self> {
        [
            Self::SlotHeldByLiveRun,
            Self::OwnerUnreadable,
            Self::WaitingForCapacity,
            Self::PublishedWithSetAside,
            Self::MovedNothing,
        ]
        .into_iter()
        .find(|reason| reason.as_str() == value)
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
        ("deferred", Some(reason), None) => Some(NightOutcome::Deferred(Deferred { reason })),
        ("failed", None, Some(code)) => Some(NightOutcome::Failed(Failed { code })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn activation() -> Activation {
        Activation {
            version: VersionId::new("v2").unwrap(),
        }
    }

    /// AC2: only a night with nothing due, or one that published with
    /// nothing set aside, is a success.
    #[test]
    fn only_complete_nights_succeed() {
        assert!(NightOutcome::nothing_due().success().is_some());
        let published = NightOutcome::published(activation(), 0);
        assert_eq!(
            published.success().and_then(Succeeded::activated),
            Some(&VersionId::new("v2").unwrap())
        );

        let partial = NightOutcome::published(activation(), 3);
        assert!(partial.success().is_none());
        assert_eq!(partial.kind(), OutcomeKind::Deferred);

        for reason in [
            DeferralReason::SlotHeldByLiveRun,
            DeferralReason::OwnerUnreadable,
            DeferralReason::WaitingForCapacity,
            DeferralReason::PublishedWithSetAside,
            DeferralReason::MovedNothing,
        ] {
            assert!(NightOutcome::deferred(reason).success().is_none());
            assert_eq!(DeferralReason::parse(reason.as_str()), Some(reason));
        }
        assert!(NightOutcome::recorded(4, 0).success().is_some());
        assert!(NightOutcome::recorded(0, 0).success().is_some());
        let partial = NightOutcome::recorded(3, 1);
        assert!(partial.success().is_none());
        assert!(!partial.counts_as_failed_night());
        let nothing = NightOutcome::recorded(0, 2);
        assert!(nothing.success().is_none());
        assert!(nothing.counts_as_failed_night());

        let failed = NightOutcome::failed(FailureCode::new("build_failed").unwrap());
        assert!(failed.success().is_none());
    }
}
