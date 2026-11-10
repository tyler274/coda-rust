//! Who may run tonight, and what happens to a run that stopped answering.
//!
//! One run at a time holds the nightly's slot. Holding it gives the run a
//! [`Lease`] whose token goes up with each new holder, and every write a run
//! makes checks that its token is still the slot's current one. A run that
//! lost its slot, and later wakes up, can't overwrite the run that took it.
//!
//! The next night takes the slot over only when the holder has ended. A
//! holder whose state can't be read is waited on, never presumed ended. The
//! store asks the [`Platform`] that runs the nightly for the holder's state;
//! a caller can't supply one.

use std::collections::BTreeSet;
use std::fmt;

use async_trait::async_trait;
use uuid::Uuid;

use crate::night::{DeferralReason, NightOutcome};
use crate::source::Source;
use crate::stop_rule::HistoryEntry;

/// How many nights in a row the slot's holder can be unreadable before a
/// person is told. The first night is waited out; the second alerts.
pub const OWNER_UNREADABLE_ALERT_AFTER: usize = 2;

/// The name of one slot, such as the whole nightly or one source's runs:
/// lowercase letters, digits, underscores and hyphens, 1 to 64 characters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlotName(String);

impl SlotName {
    /// Checks and wraps a slot name.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidSlotName> {
        let name = name.into();
        let ok = (1..=64).contains(&name.len())
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
        if ok {
            Ok(Self(name))
        } else {
            Err(InvalidSlotName(name))
        }
    }

    /// The name as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SlotName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A slot name with characters other than lowercase letters, digits,
/// underscores and hyphens, or of the wrong length.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a slot name")]
pub struct InvalidSlotName(pub String);

/// One run of the nightly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RunId(pub Uuid);

impl RunId {
    /// A new run.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// What the platform that runs the nightly says about the slot's holder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HolderState {
    /// Still running.
    Alive,
    /// Finished, crashed or was cancelled; it will make no more writes.
    Ended,
    /// Its state couldn't be read.
    Unknown,
}

/// The platform that runs the nightly, such as Cloud Run jobs or a laptop.
/// The store asks it about the run holding a slot before taking the slot.
#[async_trait]
pub trait Platform: Send + Sync {
    /// What the platform says about `run`. A platform that can't tell
    /// answers [`HolderState::Unknown`]: never `Ended` on a guess.
    async fn holder_state(&self, run: RunId) -> HolderState;
}

/// What a night does about a slot someone else holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Takeover {
    /// The holder ended, so tonight takes the slot.
    TakeOver,
    /// Tonight waits, and records why.
    Wait(SlotWait),
}

/// Why a night waited for a slot another run holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotWait {
    /// The holder is alive.
    HolderAlive,
    /// The holder couldn't be confirmed alive or ended.
    HolderUnreadable,
}

impl From<SlotWait> for DeferralReason {
    fn from(wait: SlotWait) -> Self {
        match wait {
            SlotWait::HolderAlive => Self::SlotHeldByLiveRun,
            SlotWait::HolderUnreadable => Self::OwnerUnreadable,
        }
    }
}

/// Decides whether tonight may take a held slot. `Unknown` waits: an owner
/// that can't be read is never presumed ended.
pub const fn takeover(holder: HolderState) -> Takeover {
    match holder {
        HolderState::Ended => Takeover::TakeOver,
        HolderState::Alive => Takeover::Wait(SlotWait::HolderAlive),
        HolderState::Unknown => Takeover::Wait(SlotWait::HolderUnreadable),
    }
}

/// Whether the slot's holder has been unreadable for long enough that a
/// person should be told: [`OWNER_UNREADABLE_ALERT_AFTER`] nights in a row.
pub fn owner_unreadable_needs_alert(history: &[HistoryEntry]) -> bool {
    let unreadable_in_a_row = history
        .iter()
        .rev()
        .filter_map(|entry| match entry {
            HistoryEntry::Night(night) => Some(night),
            HistoryEntry::Resolved => None,
        })
        .take_while(|night| {
            matches!(night, NightOutcome::Deferred(deferred)
                if *deferred.reason() == DeferralReason::OwnerUnreadable)
        })
        .count();
    unreadable_in_a_row >= OWNER_UNREADABLE_ALERT_AFTER
}

/// A run's hold on a slot. Every write the run makes passes its lease, and
/// the store refuses the write once another run has taken the slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lease {
    pub(crate) slot: SlotName,
    pub(crate) run: RunId,
    pub(crate) token: i64,
    pub(crate) sources: BTreeSet<Source>,
}

impl Lease {
    /// The slot held.
    pub fn slot(&self) -> &SlotName {
        &self.slot
    }

    /// The run holding it.
    pub const fn run(&self) -> RunId {
        self.run
    }

    /// The slot's token when this run took it.
    pub const fn token(&self) -> i64 {
        self.token
    }

    /// The sources whose due items this night answers for.
    pub fn sources(&self) -> &BTreeSet<Source> {
        &self.sources
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AC4: only an ended holder is taken over; an unreadable one is waited
    /// on.
    #[test]
    fn unknown_is_never_treated_as_ended() {
        assert_eq!(takeover(HolderState::Ended), Takeover::TakeOver);
        assert_eq!(
            takeover(HolderState::Alive),
            Takeover::Wait(SlotWait::HolderAlive)
        );
        assert_eq!(
            takeover(HolderState::Unknown),
            Takeover::Wait(SlotWait::HolderUnreadable)
        );
    }

    /// Decision table: an unreadable owner is waited out the first night and
    /// alerts on the second night in a row.
    #[test]
    fn an_unreadable_owner_alerts_on_the_second_night() {
        let unreadable =
            || HistoryEntry::Night(NightOutcome::deferred(DeferralReason::OwnerUnreadable));
        assert!(!owner_unreadable_needs_alert(&[unreadable()]));
        assert!(owner_unreadable_needs_alert(&[unreadable(), unreadable()]));
        assert!(!owner_unreadable_needs_alert(&[
            unreadable(),
            HistoryEntry::Night(crate::night::test_success()),
            unreadable(),
        ]));
    }

    #[test]
    fn slot_names_are_bounded() {
        assert!(SlotName::new("nightly").is_ok());
        assert!(SlotName::new("pubmed-status").is_ok());
        assert!(SlotName::new("").is_err());
        assert!(SlotName::new("Nightly").is_err());
    }
}
