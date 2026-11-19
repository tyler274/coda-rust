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
//! - [`candidate`]: a version is built, then checked, then activated, and
//!   [`storage`]: each candidate builds into a folder only it uses.
//! - [`runner`]: one night for any source: take the slot, run the source's
//!   job, and record the outcome its ledger earns.
//! - [`alert`]: what a person should be told, read from the records.
//! - [`pubmed`]: PubMed's retraction and correction status, the first source.
//! - [`Store`]: the Postgres records, whose constraints repeat these rules.
//!
//! The library serves both products, so it depends on neither.

#![deny(missing_docs)]

pub mod alert;
pub mod article;
pub mod candidate;
pub mod corpus;
pub mod failure;
pub mod freshness;
pub mod migration;
pub mod night;
pub mod pubmed;
pub mod runner;
pub mod semantic;
pub mod server;
pub mod slot;
pub mod source;
pub mod sources;
pub mod stop_rule;
pub mod storage;
mod store;

pub use alert::Alert;
pub use article::{ArticleEvent, ArticleState, InvalidMove, RETRY_LIMIT, Retry, WaitReason};
pub use candidate::{Activation, Built, Candidate, CheckFailed, Checked, VersionId};
pub use corpus::{
    BibliographicObservation, BibliographicRelationship, CorpusError, DEFAULT_SEARCH_LIMIT,
    DocumentIdentifier, DocumentRelationship, IngestedDocument, IngestedUnit, LatencySummary,
    LifecycleState, ReadEvidenceUnit, ReadItemOutcome, ReadUnavailableCause, RelationshipType,
    SearchCandidate, SearchPage, SearchResult, SourceCheckStanding, TARGET_SEARCH_P95_MS,
    TARGET_SEARCH_P99_MS, sha256_hex, source_wire_identity,
};
pub use failure::FailureCode;
pub use freshness::{
    LibraryFreshnessReport, LibraryStanding, SourceFreshness, SourceFreshnessState,
};
pub use night::{DeferralReason, NightOutcome, OutcomeKind, Streak, Succeeded, Tally};
pub use runner::{Completion, Ending, Failure, FailureDetail, Job, JobError, Night, run_night};
pub use semantic::{
    BGE_EMBEDDING_DIM, BgeEmbedding, EmbedError, SubprocessBgeEncoder, TextEmbedder,
};
pub use slot::{HolderState, Lease, Platform, RunId, SlotName, SlotWait, Takeover};
pub use source::{InvalidItemKey, ItemKey, OtherSource, Source};
pub use sources::{CorpusFeedItem, CorpusJobError, CorpusNightJob, SourceParseError};
pub use stop_rule::{HistoryEntry, Next, StopReason};
pub use store::{Acquired, Corruption, Store, StoreError};
