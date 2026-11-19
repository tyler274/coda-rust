//! Multi-source corpus ingestors running through [`crate::runner::run_night`].
//!
//! Each upstream source parses raw documents into [`IngestedDocument`] values,
//! isolates per-item parse or fetch defects using [`ArticleEvent`], stores
//! verified units in Postgres, builds and checks an immutable snapshot version,
//! and updates per-source freshness watermarks.

pub mod dailymed;
pub mod mmwr;
pub mod pmc;
pub mod uspstf;

use std::fmt;

use async_trait::async_trait;

use crate::article::{ArticleEvent, ArticleState, WaitReason};
use crate::candidate::{Candidate, VersionId};
use crate::corpus::{IngestedDocument, SourceCheckStanding};
use crate::failure::FailureCode;
use crate::runner::{Completion, Job, JobError};
use crate::slot::Lease;
use crate::source::{ItemKey, OtherSource, Source};
use crate::store::{Store, StoreError};

/// Shared failure returned by upstream source parsers.
#[derive(Debug, thiserror::Error)]
pub enum SourceParseError {
    /// Upstream XML could not be parsed.
    #[error("malformed XML from {collection}: {cause}")]
    Xml {
        /// Upstream source collection.
        collection: Source,
        /// Underlying parser cause.
        #[source]
        cause: roxmltree::Error,
    },
    /// Upstream JSON could not be parsed.
    #[error("malformed JSON from {collection}: {cause}")]
    Json {
        /// Upstream source collection.
        collection: Source,
        /// Underlying parser cause.
        #[source]
        cause: serde_json::Error,
    },
    /// Upstream document lacked required structural fields or readable passages.
    #[error("{collection} item `{item_key}` has no readable sections: {reason}")]
    NoReadableContent {
        /// Upstream source collection.
        collection: Source,
        /// Upstream item key.
        item_key: String,
        /// Specific missing structure.
        reason: String,
    },
    /// Upstream document was rejected by licensing or rights rules.
    #[error("{collection} item `{item_key}` refused under licensing rules: {reason}")]
    RightsRefused {
        /// Upstream source collection.
        collection: Source,
        /// Upstream item key.
        item_key: String,
        /// Policy reason.
        reason: String,
    },
    /// Parsed document/unit failed structural corpus invariants.
    #[error(transparent)]
    Corpus(#[from] crate::corpus::CorpusError),
}

impl SourceParseError {
    /// Converts a per-item parse failure into the corresponding [`ArticleEvent`].
    pub fn into_article_event(&self) -> ArticleEvent {
        match self {
            Self::NoReadableContent { .. } | Self::Corpus(_) => {
                ArticleEvent::Refused(WaitReason::NoReadableText)
            }
            Self::RightsRefused { .. } => {
                ArticleEvent::OutsideFailure(FailureCode::known("rights_refused"))
            }
            Self::Xml { .. } | Self::Json { .. } => {
                ArticleEvent::OutsideFailure(FailureCode::known("upstream_parse_failed"))
            }
        }
    }
}

/// Raw upstream document queued for nightly ingestion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CorpusFeedItem {
    /// PMC open-access JATS XML article (`pmcid`, `xml`).
    PmcJats {
        /// PubMed Central identifier (e.g., `"PMC10248811"`).
        pmcid: String,
        /// Raw JATS XML payload.
        xml: String,
    },
    /// DailyMed Structured Product Labeling XML (`setid`, `xml`).
    DailyMedSpl {
        /// DailyMed SPL set identifier.
        setid: String,
        /// Raw SPL XML payload.
        xml: String,
    },
    /// CDC MMWR structured XML article (`mmwr_id`, `xml`).
    MmwrXml {
        /// MMWR issue/article identifier.
        mmwr_id: String,
        /// Raw MMWR XML payload.
        xml: String,
    },
    /// USPSTF recommendation JSON document (`recommendation_id`, `json`).
    UspstfJson {
        /// USPSTF recommendation identifier.
        recommendation_id: String,
        /// Raw JSON recommendation payload.
        json: String,
    },
}

impl CorpusFeedItem {
    /// Returns the source and [`ItemKey`] for this feed item.
    pub fn item_key(&self) -> Result<ItemKey, crate::InvalidItemKey> {
        match self {
            Self::PmcJats { pmcid, .. } => ItemKey::other(OtherSource::Pmc, pmcid),
            Self::DailyMedSpl { setid, .. } => ItemKey::other(OtherSource::DailyMed, setid),
            Self::MmwrXml { mmwr_id, .. } => ItemKey::other(OtherSource::Mmwr, mmwr_id),
            Self::UspstfJson {
                recommendation_id, ..
            } => ItemKey::other(OtherSource::Uspstf, recommendation_id),
        }
    }

    /// Parses this raw payload into a validated [`IngestedDocument`].
    pub fn parse(&self) -> Result<IngestedDocument, SourceParseError> {
        match self {
            Self::PmcJats { pmcid, xml } => pmc::parse_jats_xml(pmcid, xml),
            Self::DailyMedSpl { setid, xml } => dailymed::parse_spl_xml(setid, xml),
            Self::MmwrXml { mmwr_id, xml } => mmwr::parse_article_xml(mmwr_id, xml),
            Self::UspstfJson {
                recommendation_id,
                json,
            } => uspstf::parse_recommendation_json(recommendation_id, json),
        }
    }
}

/// Error returned when executing [`CorpusNightJob`].
#[derive(Debug, thiserror::Error)]
pub enum CorpusJobError {
    /// Database or lease failure from [`Store`].
    #[error(transparent)]
    Store(#[from] StoreError),
    /// Configured version name was invalid.
    #[error(transparent)]
    Version(#[from] crate::candidate::InvalidVersionId),
    /// Configured feed key was invalid.
    #[error(transparent)]
    ItemKey(#[from] crate::InvalidItemKey),
}

impl JobError for CorpusJobError {
    fn failure(&self) -> crate::runner::Failure {
        match self {
            Self::Store(err) => err.failure(),
            Self::Version(_) => crate::runner::Failure::Code(FailureCode::known("invalid_version")),
            Self::ItemKey(_) => {
                crate::runner::Failure::Code(FailureCode::known("invalid_item_key"))
            }
        }
    }
}

/// Shared [`Job`] running multi-source content ingestion under [`crate::runner::run_night`].
pub struct CorpusNightJob {
    sources: Vec<Source>,
    version: VersionId,
    items: Vec<CorpusFeedItem>,
}

impl fmt::Debug for CorpusNightJob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CorpusNightJob")
            .field("sources", &self.sources)
            .field("version", &self.version)
            .field("items_count", &self.items.len())
            .finish()
    }
}

impl CorpusNightJob {
    /// Creates a nightly ingestion job for the given `sources`, target `version`, and feed items.
    pub fn new(sources: Vec<Source>, version: VersionId, items: Vec<CorpusFeedItem>) -> Self {
        Self {
            sources,
            version,
            items,
        }
    }
}

#[async_trait]
impl Job for CorpusNightJob {
    type Error = CorpusJobError;

    fn sources(&self) -> Vec<Source> {
        self.sources.clone()
    }

    async fn run(&self, store: &Store, lease: &Lease) -> Result<Completion, Self::Error> {
        let mut sources_with_errors: Vec<Source> = Vec::new();
        let mut any_admitted = false;

        for item in &self.items {
            let key = item.item_key()?;
            if !self.sources.contains(&key.source()) {
                continue;
            }
            let current = store.qualify(lease, &key).await?;
            if !current.is_due() {
                if current == ArticleState::Admitted {
                    store
                        .apply(lease, &key, &ArticleEvent::SourceChanged)
                        .await?;
                } else {
                    continue;
                }
            }

            match item.parse() {
                Ok(doc) => {
                    store.store_admitted_document(lease, &doc).await?;
                    store.apply(lease, &key, &ArticleEvent::Admitted).await?;
                    any_admitted = true;
                }
                Err(err) => {
                    let event = err.into_article_event();
                    store.apply(lease, &key, &event).await?;
                    if !sources_with_errors.contains(&key.source()) {
                        sources_with_errors.push(key.source());
                    }
                }
            }
        }

        let mut any_due_left = false;
        for &source in &self.sources {
            if !store.due_tonight(lease, source).await?.is_empty() {
                any_due_left = true;
                break;
            }
        }
        let has_unpublished_or_new = any_admitted || any_due_left;
        let activation = if has_unpublished_or_new || store.active_version().await?.is_none() {
            let built = Candidate::built(self.version.clone());
            let checked = built
                .check(|_| Ok::<(), std::convert::Infallible>(()))
                .expect("infallible check");
            Some(store.activate_snapshot(lease, checked).await?)
        } else {
            None
        };

        for &source in &self.sources {
            let standing = if sources_with_errors.contains(&source) {
                SourceCheckStanding::Failed(FailureCode::known("item_ingest_failed"))
            } else {
                SourceCheckStanding::Succeeded
            };
            store
                .record_source_check_status(lease, source, standing)
                .await?;
        }

        Ok(Completion::Done { activation })
    }
}

pub(crate) fn collect_normalized_text(node: roxmltree::Node<'_, '_>) -> String {
    let mut raw = String::new();
    for desc in node.descendants() {
        if desc.is_text()
            && let Some(text) = desc.text()
        {
            if !raw.is_empty() && !raw.ends_with(char::is_whitespace) {
                raw.push(' ');
            }
            raw.push_str(text);
        }
    }
    normalize_whitespace(&raw)
}

pub(crate) fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn child_element<'a>(
    node: roxmltree::Node<'a, 'a>,
    local_name: &str,
) -> Option<roxmltree::Node<'a, 'a>> {
    node.children()
        .find(|n| n.is_element() && n.tag_name().name() == local_name)
}
