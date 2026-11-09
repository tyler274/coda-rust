//! PubMed's retraction and correction status.
//!
//! NLM publishes, for each PubMed article, its publication types and its
//! links to retractions, errata, expressions of concern and updates. This
//! source reads them from NCBI's EFetch ([`client`]), parses them
//! ([`efetch`]) and records them for each article ([`refresh`]), under the
//! nightly's failure rules. The library keeps NLM's words; [`Notices`] reads
//! the ones that say an article was retracted, corrected, updated or given an
//! expression of concern.

pub mod client;
pub mod efetch;
mod record;
pub mod refresh;

pub use client::{Eutils, FetchError, Pacing, StatusSource};
pub use record::{InvalidPmid, Notices, Pmid, Relation, StatusRecord, StoredStatus};
pub use refresh::{CheckResult, RefreshError, Report, SetAside, StatusCheck, refresh};
