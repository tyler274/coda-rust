//! Searchable clinical evidence documents, passage units, active version
//! snapshots, active search/read queries, and endpoint latency tracking.

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use chrono::{DateTime, Utc};
use sea_orm::sea_query::{Expr, OnConflict, Order};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbBackend, EntityTrait,
    QueryFilter, QueryOrder, Set, Statement, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::semantic::{BGE_EMBEDDING_DIM, BgeEmbedding, fuse_ranked_unit_ids};

use crate::article::ArticleState;
use crate::candidate::{Activation, Candidate, Checked};
use crate::failure::FailureCode;
use crate::migration::{
    document, item, pubmed_relation, pubmed_status, search_latency, source_status, unit,
    unpublished, version, version_unit,
};
use crate::pubmed::Pmid;
use crate::slot::Lease;
use crate::source::{ItemKey, Source};
use crate::store::{Corruption, Store, StoreError, fault, locked_slot};

/// Default search page size when no limit is configured.
pub const DEFAULT_SEARCH_LIMIT: usize = 20;

/// Target upper bound for p95 search latency (milliseconds).
pub const TARGET_SEARCH_P95_MS: f64 = 3_000.0;

/// Target upper bound for p99 search latency (milliseconds).
pub const TARGET_SEARCH_P99_MS: f64 = 5_000.0;

/// Lifecycle standing of an admitted evidence document version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    /// Current active item.
    Current,
    /// Corrected item.
    Corrected,
    /// Retracted item.
    Retracted,
    /// Superseded version.
    Superseded,
    /// Withdrawn item.
    Withdrawn,
    /// Unconfirmed lifecycle state.
    Unknown,
}

impl LifecycleState {
    /// Lowercase database and wire string representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Corrected => "corrected",
            Self::Retracted => "retracted",
            Self::Superseded => "superseded",
            Self::Withdrawn => "withdrawn",
            Self::Unknown => "unknown",
        }
    }

    /// Parses a stored or wire lifecycle string.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "current" => Some(Self::Current),
            "corrected" => Some(Self::Corrected),
            "retracted" => Some(Self::Retracted),
            "superseded" => Some(Self::Superseded),
            "withdrawn" => Some(Self::Withdrawn),
            "unknown" => Some(Self::Unknown),
            _ => None,
        }
    }
}

/// Typed relationship kind between admitted or referenced documents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    /// This item corrects another item.
    Corrects,
    /// Another item corrects this item.
    IsCorrectedBy,
    /// This item retracts another item.
    Retracts,
    /// Another item retracts this item.
    IsRetractedBy,
    /// This item supersedes another item.
    Supersedes,
    /// Another item supersedes this item.
    IsSupersededBy,
    /// This item updates another item.
    Updates,
    /// Another item updates this item.
    IsUpdatedBy,
    /// This item expresses concern about another item.
    ExpressesConcernAbout,
    /// Another item expresses concern about this item.
    HasExpressionOfConcern,
    /// Version-of link.
    IsVersionOf,
    /// Has-version link.
    HasVersion,
}

/// External identifier attached to an admitted document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentIdentifier {
    /// Identifier scheme (e.g., `"pmid"`, `"pmcid"`, `"doi"`, `"spl-set-id"`).
    pub scheme: String,
    /// Raw identifier value as published.
    pub raw_value: String,
    /// Normalized identifier value when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_value: Option<String>,
}

/// Typed lifecycle or version relationship attached to a document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentRelationship {
    /// Semantic relationship type.
    pub relation_type: RelationshipType,
    /// Internal target version UUID when resolved inside the corpus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_version_id: Option<String>,
    /// External target identifier scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_scheme: Option<String>,
    /// External target identifier value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<String>,
    /// Source-specific relation code.
    pub source_relation_code: String,
    /// Canonical URI of the target item when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_destination_uri: Option<String>,
}

/// One passage unit prepared for corpus admission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IngestedUnit {
    /// Source-owned unit identifier within the document.
    pub native_unit_id: String,
    /// Structural locator within the source document.
    pub native_locator: String,
    /// Kind of unit (`"section"`, `"abstract"`, `"recommendation"`, `"dosage"`).
    pub unit_kind: String,
    /// Zero-based position inside the document.
    pub ordinal: usize,
    /// Section heading or label when present.
    pub label: Option<String>,
    /// Exact UTF-8 passage content.
    pub content: String,
    /// Lowercase hexadecimal SHA-256 digest of `content`.
    pub content_sha256: String,
    /// Canonical destination URI for this unit.
    pub native_destination_uri: String,
}

impl IngestedUnit {
    /// Constructs a validated passage unit, computing its SHA-256 digest.
    pub fn new(
        native_unit_id: impl Into<String>,
        native_locator: impl Into<String>,
        unit_kind: impl Into<String>,
        ordinal: usize,
        label: Option<String>,
        content: impl Into<String>,
        native_destination_uri: impl Into<String>,
    ) -> Result<Self, CorpusError> {
        let native_unit_id = native_unit_id.into();
        let native_locator = native_locator.into();
        let unit_kind = unit_kind.into();
        let content = content.into();
        let native_destination_uri = native_destination_uri.into();
        if native_unit_id.trim().is_empty()
            || native_locator.trim().is_empty()
            || unit_kind.trim().is_empty()
            || content.trim().is_empty()
            || native_destination_uri.trim().is_empty()
        {
            return Err(CorpusError::EmptyUnitField);
        }
        let content_sha256 = sha256_hex(content.as_bytes());
        Ok(Self {
            native_unit_id,
            native_locator,
            unit_kind,
            ordinal,
            label,
            content,
            content_sha256,
            native_destination_uri,
        })
    }
}

/// Complete normalized document ready for admission into the clinical library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IngestedDocument {
    /// Source item key (`source` + `item_key`).
    pub key: ItemKey,
    /// Source-owned document identifier.
    pub native_document_id: String,
    /// Source-owned version identifier when present.
    pub native_version_id: Option<String>,
    /// Human-readable version label when present.
    pub source_version_label: Option<String>,
    /// Document title.
    pub title: String,
    /// Document classification.
    pub document_type: String,
    /// Calendar publication date (`YYYY-MM-DD`) when present.
    pub publication_date: Option<String>,
    /// Calendar version date (`YYYY-MM-DD`) when present.
    pub version_date: Option<String>,
    /// Lifecycle standing of the document.
    pub lifecycle_state: LifecycleState,
    /// Source status code or notice label when present.
    pub source_status_code: Option<String>,
    /// Canonical destination URI for the document.
    pub native_destination_uri: String,
    /// Canonical destination URI for this version.
    pub version_destination_uri: String,
    /// Rights decision identifier governing excerpt display.
    pub rights_decision_id: String,
    /// License identifier when known.
    pub license_identifier: Option<String>,
    /// Attribution statement required by the license or source.
    pub attribution_text: Option<String>,
    /// Whether rights permit displaying passage excerpts.
    pub may_display_excerpt: bool,
    /// External identifiers attached to the document.
    pub identifiers: Vec<DocumentIdentifier>,
    /// Lifecycle and version relationships attached to the document.
    pub relationships: Vec<DocumentRelationship>,
    /// Ordered non-empty passage units belonging to the document.
    pub units: Vec<IngestedUnit>,
}

/// Wire source code and display label for a [`Source`].
pub const fn source_wire_identity(source: Source) -> (&'static str, &'static str) {
    match source {
        Source::PubMed => ("pubmed", "PubMed"),
        Source::Pmc => ("pmc", "PubMed Central"),
        Source::DailyMed => ("dailymed", "DailyMed"),
        Source::Mmwr => ("cdc-mmwr", "CDC MMWR"),
        Source::Uspstf => ("uspstf", "USPSTF"),
    }
}

/// Failure constructing or admitting corpus documents/units.
#[derive(Debug, thiserror::Error)]
pub enum CorpusError {
    /// A unit field or content body was empty or whitespace-only.
    #[error("evidence unit requires non-empty metadata and content")]
    EmptyUnitField,
    /// A document had empty required metadata or zero passage units.
    #[error("evidence document `{0}` requires non-empty metadata and at least one unit")]
    InvalidDocument(String),
}

impl IngestedDocument {
    /// Verifies document invariants before storage.
    pub fn validate(&self) -> Result<(), CorpusError> {
        if self.native_document_id.trim().is_empty()
            || self.title.trim().is_empty()
            || self.document_type.trim().is_empty()
            || self.native_destination_uri.trim().is_empty()
            || self.version_destination_uri.trim().is_empty()
            || self.rights_decision_id.trim().is_empty()
            || self.units.is_empty()
        {
            return Err(CorpusError::InvalidDocument(self.key.to_string()));
        }
        Ok(())
    }
}

/// Candidate item returned from active corpus search.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchCandidate {
    /// Candidate unit identifier (`unit_id` UUID).
    pub candidate_id: String,
    /// Source wire collection code (`"dailymed"`, `"pmc"`, `"cdc-mmwr"`, `"uspstf"`, `"pubmed"`).
    pub source_code: String,
    /// Human-readable source display name.
    pub source_display_name: String,
    /// Source-owned document identifier.
    pub native_document_id: String,
    /// Document title.
    pub title: String,
}

/// Page accounting metadata for a search response.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPage {
    /// Total active matching candidates examined.
    pub examined_candidate_count: usize,
    /// Candidates included in `candidates`.
    pub returned_candidate_count: usize,
    /// Matching candidates omitted as unavailable under rights/standing.
    pub unavailable_candidate_count: usize,
    /// Whether additional candidates follow `next_cursor`.
    pub truncated: bool,
}

/// Active corpus search response matching `/v1/search`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    /// Ranked matching candidate units.
    pub candidates: Vec<SearchCandidate>,
    /// Page accounting metadata.
    pub page: SearchPage,
    /// Continuation cursor when `page.truncated` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// PubMed comments/corrections link attached to a unit's bibliographic observation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BibliographicRelationship {
    /// NLM PubMed relation code (e.g., `"RetractedIn"`, `"ErratumIn"`).
    pub source_relation_code: String,
    /// Related PubMed identifier when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_pmid: Option<String>,
    /// Citation reference string when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<String>,
    /// Human-readable notice note when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// NLM PubMed bibliographic status assertion attached during `/v1/read`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BibliographicObservation {
    /// Subject PubMed identifier.
    pub subject_pmid: String,
    /// Source record revision date (`YYYY-MM-DD`) when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_record_date: Option<String>,
    /// NLM publication types.
    #[serde(default)]
    pub publication_types: Vec<String>,
    /// NLM comments/corrections links.
    #[serde(default)]
    pub relationships: Vec<BibliographicRelationship>,
}

/// Hydrated clinical evidence unit returned by `/v1/read`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadEvidenceUnit {
    /// Independent PubMed bibliographic status observations when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bibliographic_observations: Option<Vec<BibliographicObservation>>,
    /// Unit UUID matching `candidate_id`.
    pub unit_id: String,
    /// Active corpus version identifier serving this unit.
    pub version_id: String,
    /// Source wire collection code.
    pub source_code: String,
    /// Human-readable source display name.
    pub source_display_name: String,
    /// Source-owned document identifier.
    pub native_document_id: String,
    /// Source-owned version identifier when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_version_id: Option<String>,
    /// Human-readable version label when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_version_label: Option<String>,
    /// Document title.
    pub title: String,
    /// Document classification.
    pub document_type: String,
    /// Publication date (`YYYY-MM-DD`) when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    /// Version date (`YYYY-MM-DD`) when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_date: Option<String>,
    /// ISO-8601 acquisition timestamp.
    pub acquired_at: String,
    /// Lifecycle standing of the document version.
    pub lifecycle_state: LifecycleState,
    /// Source status code when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_status_code: Option<String>,
    /// Source-owned unit identifier within the document.
    pub native_unit_id: String,
    /// Source locator within the document.
    pub native_locator: String,
    /// Structural unit kind.
    pub unit_kind: String,
    /// Zero-based position in the document.
    pub ordinal: usize,
    /// Section heading or label when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Exact UTF-8 passage content.
    pub content: String,
    /// Lowercase hex SHA-256 of `content`.
    pub content_sha256: String,
    /// Canonical destination URI for this unit.
    pub native_destination_uri: String,
    /// Canonical destination URI for the document version.
    pub version_destination_uri: String,
    /// Rights decision identifier governing display.
    pub rights_decision_id: String,
    /// License identifier when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_identifier: Option<String>,
    /// Source attribution statement when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution_text: Option<String>,
    /// Whether rights permit displaying passage text.
    pub may_display_excerpt: bool,
    /// External identifiers attached to the document.
    #[serde(default)]
    pub identifiers: Vec<DocumentIdentifier>,
    /// Lifecycle and version relationships attached to the document.
    #[serde(default)]
    pub relationships: Vec<DocumentRelationship>,
}

/// Reason a candidate identifier could not be read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadUnavailableCause {
    /// Candidate identifier was not a valid UUID.
    InvalidCandidateId,
    /// Candidate unit does not exist in the active version or is not authorized.
    NotFoundOrNotAuthorized,
}

/// Discriminated per-candidate outcome returned by `/v1/read`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ReadItemOutcome {
    /// Candidate unit was found in the active version and returned.
    Available {
        /// Requested candidate identifier.
        candidate_id: String,
        /// Hydrated evidence unit.
        unit: Box<ReadEvidenceUnit>,
    },
    /// Candidate unit could not be returned.
    Unavailable {
        /// Requested candidate identifier.
        candidate_id: String,
        /// Specific unavailability cause.
        cause: ReadUnavailableCause,
    },
}

/// Percentile latency summary for recorded endpoint observations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LatencySummary {
    /// Endpoint name (`"search"` or `"read"`).
    pub endpoint: String,
    /// Number of recorded observations.
    pub sample_count: u64,
    /// 50th percentile duration in milliseconds.
    pub p50_ms: f64,
    /// 95th percentile duration in milliseconds.
    pub p95_ms: f64,
    /// 99th percentile duration in milliseconds.
    pub p99_ms: f64,
    /// Whether observed `p95_ms <= 3000.0` and `p99_ms <= 5000.0` (`None` when zero samples exist).
    pub within_targets: Option<bool>,
}

/// Outcome recorded against `clinical_library_source_status` when a source check finishes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceCheckStanding {
    /// Source refresh checked and succeeded without errors.
    Succeeded,
    /// Source refresh recorded a failure code.
    Failed(FailureCode),
}

impl Store {
    /// Records or refreshes a source's check status timestamp inside a leased run.
    pub async fn record_source_check_status(
        &self,
        lease: &Lease,
        source: Source,
        standing: SourceCheckStanding,
    ) -> Result<(), StoreError> {
        let tx = self.db().begin().await?;
        locked_slot(&tx, lease).await?;
        let now: chrono::DateTime<chrono::FixedOffset> =
            DateTime::<Utc>::from(std::time::SystemTime::now()).into();
        let (last_succeeded_at, last_error_code) = match standing {
            SourceCheckStanding::Succeeded => (Some(now), None),
            SourceCheckStanding::Failed(ref code) => (None, Some(code.as_str().to_owned())),
        };
        let row = source_status::ActiveModel {
            source: Set(source.as_str().to_owned()),
            last_checked_at: Set(Some(now)),
            last_succeeded_at: Set(last_succeeded_at),
            last_run_id: Set(Some(lease.run.0)),
            last_error_code: Set(last_error_code),
        };
        let mut update_cols = vec![
            source_status::Column::LastCheckedAt,
            source_status::Column::LastRunId,
            source_status::Column::LastErrorCode,
        ];
        if matches!(standing, SourceCheckStanding::Succeeded) {
            update_cols.push(source_status::Column::LastSucceededAt);
        }
        source_status::Entity::insert(row)
            .on_conflict(
                OnConflict::column(source_status::Column::Source)
                    .update_columns(update_cols)
                    .to_owned(),
            )
            .exec(&tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Upserts an admitted document and its verified passage units under `lease`.
    ///
    /// Preserves existing `unit_id` UUIDs when `(ordinal, content_sha256)` is unchanged so
    /// cached citation identifiers remain stable across benign re-ingests. When a
    /// [`crate::semantic::TextEmbedder`] is configured on this [`Store`], computes and
    /// stores 384-dimensional BGE embeddings for each passage unit inside the commit.
    pub async fn store_admitted_document(
        &self,
        lease: &Lease,
        doc: &IngestedDocument,
    ) -> Result<Vec<Uuid>, StoreError> {
        if !lease.sources.contains(&doc.key.source()) {
            return Err(StoreError::NotTonightsSource(doc.key.clone()));
        }
        doc.validate()?;

        let embeddings = if let Some(embedder) = self.embedder() {
            let texts: Vec<String> = doc
                .units
                .iter()
                .map(|unit| {
                    match unit
                        .label
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                    {
                        Some(label) => format!("{label}\n{}", unit.content),
                        None => unit.content.clone(),
                    }
                })
                .collect();
            Some(embedder.embed_passages(&texts).await?)
        } else {
            None
        };

        let tx = self.db().begin().await?;
        locked_slot(&tx, lease).await?;
        let unit_ids = upsert_document_tx(&tx, doc).await?;
        if let Some(embeddings) = &embeddings {
            let vector_schema = resolve_vector_schema(&tx).await?;
            for (unit_id, embedding) in unit_ids.iter().zip(embeddings.iter()) {
                write_unit_embedding_in_tx(&tx, &vector_schema, *unit_id, embedding).await?;
            }
        }
        tx.commit().await?;
        Ok(unit_ids)
    }

    /// Writes a precomputed [`BgeEmbedding`] for `unit_id` under `lease`.
    pub async fn store_unit_embedding(
        &self,
        lease: &Lease,
        unit_id: Uuid,
        embedding: &BgeEmbedding,
    ) -> Result<(), StoreError> {
        let tx = self.db().begin().await?;
        locked_slot(&tx, lease).await?;
        let vector_schema = resolve_vector_schema(&tx).await?;
        write_unit_embedding_in_tx(&tx, &vector_schema, unit_id, embedding).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Atomically populates `clinical_library_version_units` from every currently
    /// `Admitted` document and activates `candidate` under `lease`.
    pub async fn activate_snapshot(
        &self,
        lease: &Lease,
        candidate: Candidate<Checked>,
    ) -> Result<Activation, StoreError> {
        let tx = self.db().begin().await?;
        locked_slot(&tx, lease).await?;

        let version_str = candidate.version().as_str().to_owned();
        version::Entity::update_many()
            .col_expr(version::Column::Active, false.into())
            .filter(version::Column::Active.eq(true))
            .exec(&tx)
            .await?;

        let vrow = version::ActiveModel {
            version: Set(version_str.clone()),
            checked: Set(true),
            active: Set(true),
            activated_by_run: Set(Some(lease.run.0)),
        };
        version::Entity::insert(vrow)
            .on_conflict(
                OnConflict::column(version::Column::Version)
                    .update_columns([
                        version::Column::Checked,
                        version::Column::Active,
                        version::Column::ActivatedByRun,
                    ])
                    .to_owned(),
            )
            .exec(&tx)
            .await?;

        version_unit::Entity::delete_many()
            .filter(version_unit::Column::VersionId.eq(&version_str))
            .exec(&tx)
            .await?;

        let admitted_items = item::Entity::find()
            .filter(item::Column::State.eq("admitted"))
            .all(&tx)
            .await?;
        let mut admitted_keys = std::collections::BTreeSet::new();
        for item_row in admitted_items {
            admitted_keys.insert((item_row.source, item_row.item_key));
        }

        let all_units = unit::Entity::find()
            .order_by_asc(unit::Column::UnitId)
            .all(&tx)
            .await?;
        let snapshot_rows: Vec<version_unit::ActiveModel> = all_units
            .into_iter()
            .filter(|u| admitted_keys.contains(&(u.source.clone(), u.item_key.clone())))
            .map(|u| version_unit::ActiveModel {
                version_id: Set(version_str.clone()),
                unit_id: Set(u.unit_id),
            })
            .collect();
        if !snapshot_rows.is_empty() {
            version_unit::Entity::insert_many(snapshot_rows)
                .exec(&tx)
                .await?;
        }

        unpublished::Entity::delete_many()
            .filter(unpublished::Column::Source.is_in(lease.sources.iter().map(|s| s.as_str())))
            .exec(&tx)
            .await?;

        tx.commit().await?;
        Ok(Activation {
            version: candidate.version().clone(),
            run: lease.run,
            token: lease.token,
        })
    }

    /// Searches only the currently active version snapshot using hybrid BGE semantic
    /// vector similarity (when a [`crate::semantic::TextEmbedder`] is configured) fused via
    /// Reciprocal Rank Fusion with Postgres full-text search, recording latency without
    /// storing query text.
    pub async fn search_active(
        &self,
        query: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> Result<SearchResult, StoreError> {
        self.search_active_impl(query, None, cursor, limit).await
    }

    /// Searches only the currently active version snapshot using an explicit caller-provided
    /// [`BgeEmbedding`] fused with Postgres full-text search.
    pub async fn search_active_with_embedding(
        &self,
        query: &str,
        query_embedding: Option<&BgeEmbedding>,
        cursor: Option<&str>,
        limit: usize,
    ) -> Result<SearchResult, StoreError> {
        self.search_active_impl(query, query_embedding, cursor, limit)
            .await
    }

    async fn search_active_impl(
        &self,
        query: &str,
        explicit_embedding: Option<&BgeEmbedding>,
        cursor: Option<&str>,
        limit: usize,
    ) -> Result<SearchResult, StoreError> {
        let started = Instant::now();
        let trimmed = query.trim();
        let page_limit = limit.clamp(1, DEFAULT_SEARCH_LIMIT);
        let offset = cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);

        let Some(active_version) = self.active_version().await? else {
            self.record_latency("search", started.elapsed().as_secs_f64() * 1_000.0, 0)
                .await?;
            return Ok(SearchResult {
                candidates: Vec::new(),
                page: SearchPage {
                    examined_candidate_count: 0,
                    returned_candidate_count: 0,
                    unavailable_candidate_count: 0,
                    truncated: false,
                },
                next_cursor: None,
            });
        };

        if trimmed.is_empty() {
            self.record_latency("search", started.elapsed().as_secs_f64() * 1_000.0, 0)
                .await?;
            return Ok(SearchResult {
                candidates: Vec::new(),
                page: SearchPage {
                    examined_candidate_count: 0,
                    returned_candidate_count: 0,
                    unavailable_candidate_count: 0,
                    truncated: false,
                },
                next_cursor: None,
            });
        }

        let snapshot_unit_ids: Vec<Uuid> = version_unit::Entity::find()
            .filter(version_unit::Column::VersionId.eq(active_version.as_str()))
            .all(self.db())
            .await?
            .into_iter()
            .map(|row| row.unit_id)
            .collect();

        if snapshot_unit_ids.is_empty() {
            self.record_latency("search", started.elapsed().as_secs_f64() * 1_000.0, 0)
                .await?;
            return Ok(SearchResult {
                candidates: Vec::new(),
                page: SearchPage {
                    examined_candidate_count: 0,
                    returned_candidate_count: 0,
                    unavailable_candidate_count: 0,
                    truncated: false,
                },
                next_cursor: None,
            });
        }

        let embedded_query = match (explicit_embedding, self.embedder()) {
            (Some(_), _) | (None, None) => None,
            (None, Some(embedder)) => Some(embedder.embed_query(trimmed).await?),
        };
        let query_vec = explicit_embedding.or(embedded_query.as_ref());

        // sea-query has no typed PostgreSQL `@@` tsvector operator; `trimmed` is bound as `$1`
        // (never interpolated as SQL).
        let fts_filter = Expr::cust_with_values(
            "search_tsv @@ websearch_to_tsquery('english', $1)",
            [sea_orm::Value::from(trimmed)],
        );
        let fts_rank = Expr::cust_with_values(
            "ts_rank_cd(search_tsv, websearch_to_tsquery('english', $1))",
            [sea_orm::Value::from(trimmed)],
        );
        let fts_units = unit::Entity::find()
            .filter(unit::Column::UnitId.is_in(snapshot_unit_ids.clone()))
            .filter(fts_filter)
            .order_by(fts_rank, Order::Desc)
            .order_by_asc(unit::Column::Ordinal)
            .order_by_asc(unit::Column::UnitId)
            .all(self.db())
            .await?;

        let fts_ids: Vec<Uuid> = fts_units.iter().map(|u| u.unit_id).collect();
        let vector_ids = if let Some(emb) = query_vec {
            let vector_schema = resolve_vector_schema(self.db()).await?;
            let fetch_limit = offset
                .saturating_add(page_limit)
                .max(DEFAULT_SEARCH_LIMIT)
                .saturating_mul(4)
                .clamp(40, 200);
            query_vector_unit_ids(
                self.db(),
                &vector_schema,
                active_version.as_str(),
                emb,
                fetch_limit,
            )
            .await?
        } else {
            Vec::new()
        };

        let fused_ids = fuse_ranked_unit_ids(&vector_ids, &fts_ids);
        let mut units_by_id: HashMap<Uuid, unit::Model> =
            fts_units.into_iter().map(|u| (u.unit_id, u)).collect();
        let missing_ids: Vec<Uuid> = fused_ids
            .iter()
            .copied()
            .filter(|id| !units_by_id.contains_key(id))
            .collect();
        if !missing_ids.is_empty() {
            for u in unit::Entity::find()
                .filter(unit::Column::UnitId.is_in(missing_ids))
                .all(self.db())
                .await?
            {
                units_by_id.insert(u.unit_id, u);
            }
        }

        let mut docs_by_key: BTreeMap<(String, String), document::Model> = BTreeMap::new();
        for doc_row in document::Entity::find().all(self.db()).await? {
            docs_by_key.insert((doc_row.source.clone(), doc_row.item_key.clone()), doc_row);
        }

        let mut available_candidates = Vec::new();
        let mut unavailable_count = 0_usize;
        let examined_count = fused_ids.len();
        for unit_id in fused_ids {
            let Some(u) = units_by_id.get(&unit_id) else {
                continue;
            };
            let Some(doc) = docs_by_key.get(&(u.source.clone(), u.item_key.clone())) else {
                unavailable_count += 1;
                continue;
            };
            if !doc.may_display_excerpt {
                unavailable_count += 1;
                continue;
            }
            available_candidates.push(SearchCandidate {
                candidate_id: u.unit_id.to_string(),
                source_code: doc.source_code.clone(),
                source_display_name: doc.source_display_name.clone(),
                native_document_id: doc.native_document_id.clone(),
                title: doc.title.clone(),
            });
        }

        let slice_start = offset.min(available_candidates.len());
        let slice_end = slice_start
            .saturating_add(page_limit)
            .min(available_candidates.len());
        let truncated = slice_end < available_candidates.len();
        let next_cursor = truncated.then(|| slice_end.to_string());
        let page_candidates = available_candidates[slice_start..slice_end].to_vec();
        let returned_count = page_candidates.len();

        self.record_latency(
            "search",
            started.elapsed().as_secs_f64() * 1_000.0,
            returned_count,
        )
        .await?;

        Ok(SearchResult {
            candidates: page_candidates,
            page: SearchPage {
                examined_candidate_count: examined_count,
                returned_candidate_count: returned_count,
                unavailable_candidate_count: unavailable_count,
                truncated,
            },
            next_cursor,
        })
    }

    /// Reads exact units by `candidate_id` from the currently active version snapshot,
    /// attaching live PubMed bibliographic status observations for linked PMIDs.
    pub async fn read_active(
        &self,
        candidate_ids: &[String],
    ) -> Result<Vec<ReadItemOutcome>, StoreError> {
        let started = Instant::now();
        let active_version = self.active_version().await?;

        let mut results = Vec::with_capacity(candidate_ids.len());
        let mut available_count = 0_usize;

        for raw_id in candidate_ids {
            let Ok(uuid) = Uuid::parse_str(raw_id.trim()) else {
                results.push(ReadItemOutcome::Unavailable {
                    candidate_id: raw_id.clone(),
                    cause: ReadUnavailableCause::InvalidCandidateId,
                });
                continue;
            };

            let Some(ref version_id) = active_version else {
                results.push(ReadItemOutcome::Unavailable {
                    candidate_id: raw_id.clone(),
                    cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                });
                continue;
            };

            let membership =
                version_unit::Entity::find_by_id((version_id.as_str().to_owned(), uuid))
                    .one(self.db())
                    .await?;
            if membership.is_none() {
                results.push(ReadItemOutcome::Unavailable {
                    candidate_id: raw_id.clone(),
                    cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                });
                continue;
            }

            let Some(u) = unit::Entity::find_by_id(uuid).one(self.db()).await? else {
                results.push(ReadItemOutcome::Unavailable {
                    candidate_id: raw_id.clone(),
                    cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                });
                continue;
            };

            let Some(doc) = document::Entity::find_by_id((u.source.clone(), u.item_key.clone()))
                .one(self.db())
                .await?
            else {
                results.push(ReadItemOutcome::Unavailable {
                    candidate_id: raw_id.clone(),
                    cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                });
                continue;
            };

            let identifiers: Vec<DocumentIdentifier> = serde_json::from_str(&doc.identifiers_json)
                .map_err(|err| {
                    fault(
                        "clinical_library_documents",
                        Corruption::UnknownName {
                            field: "identifiers_json",
                            value: err.to_string(),
                        },
                    )
                })?;
            let relationships: Vec<DocumentRelationship> =
                serde_json::from_str(&doc.relationships_json).map_err(|err| {
                    fault(
                        "clinical_library_documents",
                        Corruption::UnknownName {
                            field: "relationships_json",
                            value: err.to_string(),
                        },
                    )
                })?;

            let mut lifecycle_state =
                LifecycleState::parse(&doc.lifecycle_state).ok_or_else(|| {
                    fault(
                        "clinical_library_documents",
                        Corruption::UnknownName {
                            field: "lifecycle_state",
                            value: doc.lifecycle_state.clone(),
                        },
                    )
                })?;

            let bib_observations = self.load_bibliographic_observations(&identifiers).await?;
            if let Some(obs) = &bib_observations {
                for entry in obs {
                    if entry
                        .publication_types
                        .iter()
                        .any(|pt| pt.eq_ignore_ascii_case("Retracted Publication"))
                        || entry.relationships.iter().any(|r| {
                            r.source_relation_code.eq_ignore_ascii_case("RetractionIn")
                                || r.source_relation_code.eq_ignore_ascii_case("RetractedIn")
                                || r.source_relation_code
                                    .eq_ignore_ascii_case("RetractedandRepublishedIn")
                        })
                    {
                        lifecycle_state = LifecycleState::Retracted;
                    } else if lifecycle_state == LifecycleState::Current
                        && entry.relationships.iter().any(|r| {
                            r.source_relation_code.eq_ignore_ascii_case("ErratumIn")
                                || r.source_relation_code
                                    .eq_ignore_ascii_case("CorrectedandRepublishedIn")
                        })
                    {
                        lifecycle_state = LifecycleState::Corrected;
                    }
                }
            }

            let ordinal = usize::try_from(u.ordinal)
                .map_err(|cause| fault("clinical_library_units", cause))?;
            available_count += 1;
            results.push(ReadItemOutcome::Available {
                candidate_id: raw_id.clone(),
                unit: Box::new(ReadEvidenceUnit {
                    bibliographic_observations: bib_observations,
                    unit_id: u.unit_id.to_string(),
                    version_id: version_id.as_str().to_owned(),
                    source_code: doc.source_code,
                    source_display_name: doc.source_display_name,
                    native_document_id: doc.native_document_id,
                    native_version_id: doc.native_version_id,
                    source_version_label: doc.source_version_label,
                    title: doc.title,
                    document_type: doc.document_type,
                    publication_date: doc.publication_date,
                    version_date: doc.version_date,
                    acquired_at: doc.acquired_at.with_timezone(&Utc).to_rfc3339(),
                    lifecycle_state,
                    source_status_code: doc.source_status_code,
                    native_unit_id: u.native_unit_id,
                    native_locator: u.native_locator,
                    unit_kind: u.unit_kind,
                    ordinal,
                    label: u.label,
                    content: u.content,
                    content_sha256: u.content_sha256,
                    native_destination_uri: u.native_destination_uri,
                    version_destination_uri: doc.version_destination_uri,
                    rights_decision_id: doc.rights_decision_id,
                    license_identifier: doc.license_identifier,
                    attribution_text: doc.attribution_text,
                    may_display_excerpt: doc.may_display_excerpt,
                    identifiers,
                    relationships,
                }),
            });
        }

        self.record_latency(
            "read",
            started.elapsed().as_secs_f64() * 1_000.0,
            available_count,
        )
        .await?;

        Ok(results)
    }

    async fn load_bibliographic_observations(
        &self,
        identifiers: &[DocumentIdentifier],
    ) -> Result<Option<Vec<BibliographicObservation>>, StoreError> {
        let mut observations = Vec::new();
        for ident in identifiers {
            if !ident.scheme.eq_ignore_ascii_case("pmid") {
                continue;
            }
            let raw_pmid = ident
                .normalized_value
                .as_deref()
                .unwrap_or(&ident.raw_value);
            let Some(pmid) = raw_pmid
                .trim()
                .parse::<u64>()
                .ok()
                .and_then(|n| Pmid::new(n).ok())
            else {
                continue;
            };
            let Ok(pmid_i64) = i64::try_from(pmid.get()) else {
                continue;
            };
            let Some(status) = pubmed_status::Entity::find_by_id(pmid_i64)
                .one(self.db())
                .await?
            else {
                continue;
            };
            let rels = pubmed_relation::Entity::find()
                .filter(pubmed_relation::Column::Pmid.eq(pmid_i64))
                .order_by_asc(pubmed_relation::Column::Position)
                .all(self.db())
                .await?;
            observations.push(BibliographicObservation {
                subject_pmid: pmid.to_string(),
                source_record_date: status.revised.map(|d| d.format("%Y-%m-%d").to_string()),
                publication_types: status.publication_types,
                relationships: rels
                    .into_iter()
                    .map(|rel| BibliographicRelationship {
                        source_relation_code: rel.ref_type,
                        target_pmid: rel.target_pmid.map(|id| id.to_string()),
                        source_reference: rel.source_reference,
                        note: rel.note,
                    })
                    .collect(),
            });
        }
        if observations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(observations))
        }
    }

    async fn record_latency(
        &self,
        endpoint: &str,
        duration_ms: f64,
        candidate_count: usize,
    ) -> Result<(), StoreError> {
        let count_i32 = i32::try_from(candidate_count).unwrap_or(i32::MAX);
        search_latency::ActiveModel {
            endpoint: Set(endpoint.to_owned()),
            duration_ms: Set(duration_ms.max(0.0)),
            candidate_count: Set(count_i32),
            ..Default::default()
        }
        .insert(self.db())
        .await?;
        Ok(())
    }

    /// Computes p50/p95/p99 latency percentiles from recorded endpoint observations.
    pub async fn latency_summary(&self, endpoint: &str) -> Result<LatencySummary, StoreError> {
        let rows = search_latency::Entity::find()
            .filter(search_latency::Column::Endpoint.eq(endpoint))
            .order_by_asc(search_latency::Column::DurationMs)
            .all(self.db())
            .await?;
        let samples: Vec<f64> = rows.into_iter().map(|r| r.duration_ms).collect();
        if samples.is_empty() {
            return Ok(LatencySummary {
                endpoint: endpoint.to_owned(),
                sample_count: 0,
                p50_ms: 0.0,
                p95_ms: 0.0,
                p99_ms: 0.0,
                within_targets: None,
            });
        }
        let p50_ms = percentile_sorted(&samples, 0.50);
        let p95_ms = percentile_sorted(&samples, 0.95);
        let p99_ms = percentile_sorted(&samples, 0.99);
        Ok(LatencySummary {
            endpoint: endpoint.to_owned(),
            sample_count: u64::try_from(samples.len()).unwrap_or(u64::MAX),
            p50_ms,
            p95_ms,
            p99_ms,
            within_targets: Some(p95_ms <= TARGET_SEARCH_P95_MS && p99_ms <= TARGET_SEARCH_P99_MS),
        })
    }

    /// Returns whether `key` is currently admitted in `clinical_library_items`.
    pub async fn is_admitted(&self, key: &ItemKey) -> Result<bool, StoreError> {
        Ok(matches!(
            self.item(key).await?,
            Some(ArticleState::Admitted)
        ))
    }
}

fn percentile_sorted(sorted: &[f64], quantile: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() as f64 - 1.0) * quantile).ceil() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

async fn resolve_vector_schema(db: &impl ConnectionTrait) -> Result<String, StoreError> {
    let row = db
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT n.nspname AS schema_name \
             FROM pg_extension e \
             JOIN pg_namespace n ON n.oid = e.extnamespace \
             WHERE e.extname = 'vector'"
                .to_owned(),
        ))
        .await?
        .ok_or_else(|| {
            StoreError::Database(sea_orm::DbErr::Custom(
                "pgvector extension is not installed".into(),
            ))
        })?;
    let schema_name: String = row.try_get("", "schema_name")?;
    if !schema_name
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return Err(StoreError::Database(sea_orm::DbErr::Custom(
            "unexpected pgvector schema identifier".into(),
        )));
    }
    Ok(schema_name)
}

async fn write_unit_embedding_in_tx(
    tx: &impl ConnectionTrait,
    vector_schema: &str,
    unit_id: Uuid,
    embedding: &BgeEmbedding,
) -> Result<(), StoreError> {
    let vec_val = sea_orm::Value::from(embedding.as_slice().to_vec());
    let id_val = sea_orm::Value::from(unit_id);
    let sql = format!(
        "UPDATE clinical_library_units \
         SET embedding = ($1::real[])::{vector_schema}.vector({BGE_EMBEDDING_DIM}) \
         WHERE unit_id = $2"
    );
    tx.execute_raw(Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        [vec_val, id_val],
    ))
    .await?;
    Ok(())
}

async fn query_vector_unit_ids(
    db: &impl ConnectionTrait,
    vector_schema: &str,
    active_version: &str,
    query_embedding: &BgeEmbedding,
    limit: usize,
) -> Result<Vec<Uuid>, StoreError> {
    let ver_val = sea_orm::Value::from(active_version.to_owned());
    let vec_val = sea_orm::Value::from(query_embedding.as_slice().to_vec());
    let limit_val = sea_orm::Value::from(i64::try_from(limit).unwrap_or(100));
    let sql = format!(
        "SELECT u.unit_id \
         FROM clinical_library_units u \
         JOIN clinical_library_version_units vu ON vu.unit_id = u.unit_id \
         WHERE vu.version_id = $1 AND u.embedding IS NOT NULL \
         ORDER BY (u.embedding OPERATOR({vector_schema}.<=>) ($2::real[])::{vector_schema}.vector({BGE_EMBEDDING_DIM})) ASC, \
                  u.ordinal ASC, u.unit_id ASC \
         LIMIT $3"
    );
    let rows = db
        .query_all_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            [ver_val, vec_val, limit_val],
        ))
        .await?;
    let mut ids = Vec::with_capacity(rows.len());
    for row in rows {
        ids.push(row.try_get("", "unit_id")?);
    }
    Ok(ids)
}

async fn upsert_document_tx(
    tx: &DatabaseTransaction,
    doc: &IngestedDocument,
) -> Result<Vec<Uuid>, StoreError> {
    let source_str = doc.key.source().as_str().to_owned();
    let item_key_str = doc.key.as_str().to_owned();
    let (source_code, source_display_name) = source_wire_identity(doc.key.source());
    let now: chrono::DateTime<chrono::FixedOffset> =
        DateTime::<Utc>::from(std::time::SystemTime::now()).into();
    let identifiers_json = serde_json::to_string(&doc.identifiers).unwrap_or_else(|_| "[]".into());
    let relationships_json =
        serde_json::to_string(&doc.relationships).unwrap_or_else(|_| "[]".into());

    let doc_row = document::ActiveModel {
        source: Set(source_str.clone()),
        item_key: Set(item_key_str.clone()),
        source_code: Set(source_code.to_owned()),
        source_display_name: Set(source_display_name.to_owned()),
        native_document_id: Set(doc.native_document_id.clone()),
        native_version_id: Set(doc.native_version_id.clone()),
        source_version_label: Set(doc.source_version_label.clone()),
        title: Set(doc.title.clone()),
        document_type: Set(doc.document_type.clone()),
        publication_date: Set(doc.publication_date.clone()),
        version_date: Set(doc.version_date.clone()),
        acquired_at: Set(now),
        lifecycle_state: Set(doc.lifecycle_state.as_str().to_owned()),
        source_status_code: Set(doc.source_status_code.clone()),
        native_destination_uri: Set(doc.native_destination_uri.clone()),
        version_destination_uri: Set(doc.version_destination_uri.clone()),
        rights_decision_id: Set(doc.rights_decision_id.clone()),
        license_identifier: Set(doc.license_identifier.clone()),
        attribution_text: Set(doc.attribution_text.clone()),
        may_display_excerpt: Set(doc.may_display_excerpt),
        identifiers_json: Set(identifiers_json),
        relationships_json: Set(relationships_json),
    };
    document::Entity::insert(doc_row)
        .on_conflict(
            OnConflict::columns([document::Column::Source, document::Column::ItemKey])
                .update_columns([
                    document::Column::NativeVersionId,
                    document::Column::SourceVersionLabel,
                    document::Column::Title,
                    document::Column::DocumentType,
                    document::Column::PublicationDate,
                    document::Column::VersionDate,
                    document::Column::AcquiredAt,
                    document::Column::LifecycleState,
                    document::Column::SourceStatusCode,
                    document::Column::NativeDestinationUri,
                    document::Column::VersionDestinationUri,
                    document::Column::RightsDecisionId,
                    document::Column::LicenseIdentifier,
                    document::Column::AttributionText,
                    document::Column::MayDisplayExcerpt,
                    document::Column::IdentifiersJson,
                    document::Column::RelationshipsJson,
                ])
                .to_owned(),
        )
        .exec(tx)
        .await?;

    let existing_units = unit::Entity::find()
        .filter(unit::Column::Source.eq(&source_str))
        .filter(unit::Column::ItemKey.eq(&item_key_str))
        .all(tx)
        .await?;
    let mut reusable_ids: BTreeMap<(i32, String), Uuid> = BTreeMap::new();
    for existing in existing_units {
        reusable_ids.insert(
            (existing.ordinal, existing.content_sha256),
            existing.unit_id,
        );
    }

    unit::Entity::delete_many()
        .filter(unit::Column::Source.eq(&source_str))
        .filter(unit::Column::ItemKey.eq(&item_key_str))
        .exec(tx)
        .await?;

    let mut assigned_ids = Vec::with_capacity(doc.units.len());
    let mut models = Vec::with_capacity(doc.units.len());
    for u in &doc.units {
        let ordinal_i32 =
            i32::try_from(u.ordinal).map_err(|cause| fault("clinical_library_units", cause))?;
        let unit_id = reusable_ids
            .get(&(ordinal_i32, u.content_sha256.clone()))
            .copied()
            .unwrap_or_else(Uuid::new_v4);
        assigned_ids.push(unit_id);
        models.push(unit::ActiveModel {
            unit_id: Set(unit_id),
            source: Set(source_str.clone()),
            item_key: Set(item_key_str.clone()),
            native_unit_id: Set(u.native_unit_id.clone()),
            native_locator: Set(u.native_locator.clone()),
            unit_kind: Set(u.unit_kind.clone()),
            ordinal: Set(ordinal_i32),
            label: Set(u.label.clone()),
            content: Set(u.content.clone()),
            content_sha256: Set(u.content_sha256.clone()),
            native_destination_uri: Set(u.native_destination_uri.clone()),
        });
    }
    if !models.is_empty() {
        unit::Entity::insert_many(models).exec(tx).await?;
    }
    Ok(assigned_ids)
}

/// Computes lowercase hexadecimal SHA-256 of UTF-8 bytes.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(out, "{byte:02x}");
    }
    out
}
