//! One night of the refresh, for any source.
//!
//! Every source runs through [`run_night`] rather than its own loop. The
//! runner takes the slot (the store asks the platform about any holder and
//! applies the stop rule), runs the source's [`Job`] under the lease, and
//! records the night's one outcome, which the store counts from the night's
//! ledger. A failed night's work resumes the next night: what it admitted
//! stays admitted, and what it left due is due again.
//!
//! A run that crashes records nothing; the next night finds it ended, takes
//! the slot over and records it as failed.

use std::error::Error;

use async_trait::async_trait;

use crate::candidate::Activation;
use crate::failure::FailureCode;
use crate::night::NightOutcome;
use crate::slot::{Lease, Platform, RunId, SlotName, SlotWait};
use crate::source::Source;
use crate::stop_rule::StopReason;
use crate::store::{Acquired, Store, StoreError};

/// How a source's job ended its night's work.
#[derive(Debug)]
pub enum Completion {
    /// It did what it could tonight, and activated `activation` if it built
    /// a version. Whether that is a success is the ledger's to say.
    Done {
        /// The version tonight activated, if any.
        activation: Option<Activation>,
    },
    /// No build capacity was free all night.
    NoCapacity,
}

/// How a night ends, as [`Store::finish`] records it.
#[derive(Debug)]
pub enum Ending {
    /// The night's job finished; the outcome is counted from its ledger.
    Completed {
        /// The version tonight activated, if any.
        activation: Option<Activation>,
    },
    /// No build capacity was free all night.
    NoCapacity,
    /// The night's own work failed.
    Failed {
        /// The failure's stable code.
        code: FailureCode,
        /// What failed, in words a person reads.
        detail: FailureDetail,
    },
}

impl From<Completion> for Ending {
    fn from(completion: Completion) -> Self {
        match completion {
            Completion::Done { activation } => Self::Completed { activation },
            Completion::NoCapacity => Self::NoCapacity,
        }
    }
}

/// What a failure means for the night.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// Another run took the slot. The night is the new holder's, so this
    /// run records nothing.
    LostSlot,
    /// The night failed, with this code.
    Code(FailureCode),
}

/// An error a source's job ends with.
pub trait JobError: Error + Send + Sync + 'static {
    /// What it means for the night.
    fn failure(&self) -> Failure;
}

impl JobError for StoreError {
    fn failure(&self) -> Failure {
        match self {
            Self::LeaseLost { .. } => Failure::LostSlot,
            Self::Database(_)
            | Self::Query(_)
            | Self::ForeignActivation { .. }
            | Self::TooManyLinks { .. }
            | Self::Unqualified(_)
            | Self::NotTonightsSource(_)
            | Self::InvalidMove(_)
            | Self::Corrupt { .. } => Failure::Code(FailureCode::known(STORE_FAILED)),
        }
    }
}

/// The code a night records when the library's records failed.
pub const STORE_FAILED: &str = "clinical_library_store_failed";

/// One source's work for a night.
#[async_trait]
pub trait Job: Send + Sync {
    /// How the job fails.
    type Error: JobError;

    /// The sources whose due items this job answers for.
    fn sources(&self) -> Vec<Source>;

    /// Does tonight's work under `lease`: qualifies, admits or sets aside
    /// each due item through `store`, and activates a version if it built
    /// one.
    async fn run(&self, store: &Store, lease: &Lease) -> Result<Completion, Self::Error>;
}

/// What came of one night.
#[derive(Debug)]
pub enum Night {
    /// The night ran and recorded this outcome.
    Finished(NightOutcome),
    /// Another run held the slot; the wait is recorded.
    Waited(SlotWait),
    /// Automatic attempts had stopped; the night that didn't run is
    /// recorded.
    Stopped(StopReason),
    /// Another run took the slot while this one ran, so this run recorded
    /// nothing.
    LostSlot,
}

/// Runs one night of `job` under `slot` as `run`.
pub async fn run_night<J: Job>(
    store: &Store,
    platform: &dyn Platform,
    slot: &SlotName,
    run: RunId,
    job: &J,
) -> Result<Night, StoreError> {
    let lease = match store.acquire(slot, &job.sources(), run, platform).await? {
        Acquired::Holds(lease) => lease,
        Acquired::Waited(wait) => return Ok(Night::Waited(wait)),
        Acquired::Stopped(why) => return Ok(Night::Stopped(why)),
    };
    let ending = match job.run(store, &lease).await {
        Ok(completion) => Ending::from(completion),
        Err(error) => match error.failure() {
            Failure::LostSlot => return Ok(Night::LostSlot),
            Failure::Code(code) => Ending::Failed {
                code,
                detail: FailureDetail::of(&error),
            },
        },
    };
    match store.finish(lease, ending).await {
        Ok(outcome) => Ok(Night::Finished(outcome)),
        Err(StoreError::LeaseLost { .. }) => Ok(Night::LostSlot),
        Err(error) => Err(error),
    }
}

/// A failure in words a person reads: the error and each of its causes,
/// joined with `: `, from the head, at most [`FailureDetail::LIMIT`]
/// characters. The head is kept because it says what failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FailureDetail(String);

impl FailureDetail {
    /// The most characters kept.
    pub const LIMIT: usize = 2000;

    /// An error and each of its causes.
    pub fn of(error: &dyn Error) -> Self {
        let mut words = error.to_string();
        let mut cause = error.source();
        while let Some(next) = cause {
            words.push_str(": ");
            words.push_str(&next.to_string());
            cause = next.source();
        }
        Self::new(words)
    }

    /// Words a person reads, cut at [`FailureDetail::LIMIT`] characters
    /// from the head. Empty words read as "no detail".
    pub fn new(words: impl Into<String>) -> Self {
        let words = words.into();
        let words = words.trim();
        let mut kept: String = words.chars().take(Self::LIMIT).collect();
        if kept.is_empty() {
            kept.push_str("no detail");
        }
        Self(kept)
    }

    /// The words as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, thiserror::Error)]
    #[error("PMC article PMC1 couldn't be admitted")]
    struct Outer(#[source] Inner);

    #[derive(Debug, thiserror::Error)]
    #[error("{0}")]
    struct Inner(String);

    /// The line that says what failed survives, however long the causes.
    #[test]
    fn failure_detail_keeps_the_head() {
        let error = Outer(Inner("x".repeat(5000)));
        let detail = FailureDetail::of(&error);
        assert!(
            detail
                .as_str()
                .starts_with("PMC article PMC1 couldn't be admitted: xxx")
        );
        assert_eq!(detail.as_str().chars().count(), FailureDetail::LIMIT);
        assert_eq!(FailureDetail::new("  ").as_str(), "no detail");
    }
}
