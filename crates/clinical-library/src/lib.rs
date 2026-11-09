//! The clinical library's records and the nightly's failure rules.
//!
//! Coda's clinical library is refreshed every night from its sources. This
//! crate holds the records that refresh reads and writes, and the rules it
//! follows when something fails, stated in
//! `verification/clinical-library/nightly/README.md`:
//!
//! - [`article`]: the four states an article is in, and the moves the rules
//!   allow between them.
//! - [`night`]: a night's one outcome; only a complete night is a success.
//! - [`stop_rule`]: the same failure twice, or three failed nights in a row,
//!   stops automatic attempts.
//! - [`slot`]: one run at a time, takeover only of an ended run, and the
//!   lease every write checks.
//! - [`candidate`]: a version is built, then checked, then activated.
//! - [`pubmed`]: PubMed's retraction and correction status, the first source.
//! - [`Store`]: the Postgres records, whose constraints repeat these rules.
//!
//! The library serves both products, so it depends on neither.

#![deny(missing_docs)]

pub mod article;
pub mod candidate;
pub mod failure;
pub mod migration;
pub mod night;
pub mod pubmed;
pub mod slot;
pub mod source;
pub mod stop_rule;
mod store;

pub use article::{ArticleEvent, ArticleState, InvalidMove, RETRY_LIMIT, Retry, WaitReason};
pub use candidate::{Activation, Built, Candidate, CheckFailed, Checked, VersionId};
pub use failure::FailureCode;
pub use night::{DeferralReason, NightOutcome, OutcomeKind, Succeeded};
pub use slot::{HolderState, Lease, RunId, SlotName, Takeover};
pub use source::{InvalidItemKey, ItemKey, OtherSource, Source};
pub use stop_rule::{HistoryEntry, Next, StopReason};
pub use store::{Acquired, Corruption, Store, StoreError, Waiting};
