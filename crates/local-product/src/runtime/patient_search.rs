//! Unified patient-scoped search across saved chart records, earlier conversations
//! and reply-revision work, and durable patient memory.
//!
//! Search returns typed references, dates, standing, clinical terminology codes,
//! and authorship metadata only. Finding an item never grants permission to read
//! its body: opening a chart record still requires `read_patient_sources`, reading
//! a conversation or reply draft requires transcript readback, and reading memory
//! requires `read_patient_memory` or patient-page summary readback.

use std::collections::{BTreeMap, BTreeSet};

use harness::engine::{self, Tool, ToolOutcome};
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::entities::{
    command, conversation, patient_memory, patient_memory_version, reply_draft, reply_revision,
    source, source_version,
};
use super::memory::{
    self, CorrectionId, CorrectionStanding, MemoryAudience, MemoryAuthor, MemoryError, MemoryId,
    MemorySearchDate, MemorySearchQuery, MemorySearchRecord, MemoryValidationError,
    MemoryVersionNumber, PatientAccess, PatientKey, SearchAudienceScope, SearchVersionScope,
    SearchableMemoryContent, SourceMessageRef, TopicKey, VersionStanding,
};
use super::reply::DraftStatus;
use super::{Authority, Claim, MessageRole, Scope, Store, control, records, reply};
use crate::physician_sources::SourceRevision;
use crate::physician_sources::read::{
    InvalidSourceRevisionError, PartIdentity, PartState, ProvenancePartReader, RecordPartReader,
    RecordReadError, ResolvedPart,
};

/// Tool name offered to Coda jobs for unified patient-scoped discovery.
pub const SEARCH_TOOL: &str = "search_patient_information";

/// Maximum hits returned in one search page before continuing with [`SearchCursor`].
pub const SEARCH_PAGE_SIZE: usize = 20;

/// Maximum UTF-8 byte length accepted for a search query string.
pub const MAX_SEARCH_QUERY_BYTES: usize = 512;

/// Whether `name` names the unified patient search tool.
pub fn is_tool(name: &str) -> bool {
    name == SEARCH_TOOL
}

/// Product record buckets defined by the physician workspace contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordBucket {
    /// Demographics, identifiers, contact details, care team, and coverage.
    Patient,
    /// Active, chronic, resolved, and encounter diagnoses.
    Problems,
    /// Prescriptions, home medications, administrations, and dispenses.
    Medications,
    /// Drug, food, and environmental allergies and intolerances.
    Allergies,
    /// Lab panels, microbiology, pathology, radiology, cardiology, and diagnostic reports.
    Results,
    /// Blood pressure, heart rate, weight, BMI, SpO2, temperature, and wearable streams.
    Vitals,
    /// Progress notes, H&Ps, consults, discharge summaries, operative notes, and letters.
    Notes,
    /// Scanned PDFs, faxes, outside records, consent forms, and uploaded files.
    Documents,
    /// Past and upcoming appointments, encounters, admissions, and ED visits.
    Visits,
    /// Surgeries, endoscopies, biopsies, and in-office procedures.
    Procedures,
    /// Administered and historical vaccines.
    Immunizations,
    /// Pending and completed lab, imaging, referral, and DME orders.
    Orders,
    /// Family, social, surgical, and obstetric history.
    History,
    /// Intake packets, PHQ-9, GAD-7, PROMs, and plan check-in answers.
    Forms,
    /// Patient portal threads, Spruce threads, and patient-submitted items.
    Messages,
    /// Connected source records that do not fit an established clinical bucket.
    Other,
}

impl RecordBucket {
    /// Every product record bucket in canonical order.
    pub const ALL: [Self; 16] = [
        Self::Patient,
        Self::Problems,
        Self::Medications,
        Self::Allergies,
        Self::Results,
        Self::Vitals,
        Self::Notes,
        Self::Documents,
        Self::Visits,
        Self::Procedures,
        Self::Immunizations,
        Self::Orders,
        Self::History,
        Self::Forms,
        Self::Messages,
        Self::Other,
    ];

    /// Canonical snake_case wire label for this bucket.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Patient => "patient",
            Self::Problems => "problems",
            Self::Medications => "medications",
            Self::Allergies => "allergies",
            Self::Results => "results",
            Self::Vitals => "vitals",
            Self::Notes => "notes",
            Self::Documents => "documents",
            Self::Visits => "visits",
            Self::Procedures => "procedures",
            Self::Immunizations => "immunizations",
            Self::Orders => "orders",
            Self::History => "history",
            Self::Forms => "forms",
            Self::Messages => "messages",
            Self::Other => "other",
        }
    }
}

/// Domain slice to search within an authorized patient workspace.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchDomainFilter {
    /// Search saved chart records, earlier conversations/work, and durable patient memory.
    #[default]
    All,
    /// Search saved chart records only.
    ChartRecords,
    /// Search earlier conversations and reply-revision work only.
    EarlierWork,
    /// Search durable patient memory versions only.
    PatientMemory,
}

impl SearchDomainFilter {
    fn includes_chart(self) -> bool {
        match self {
            Self::All | Self::ChartRecords => true,
            Self::EarlierWork | Self::PatientMemory => false,
        }
    }

    fn includes_work(self) -> bool {
        match self {
            Self::All | Self::EarlierWork => true,
            Self::ChartRecords | Self::PatientMemory => false,
        }
    }

    fn includes_memory(self) -> bool {
        match self {
            Self::All | Self::PatientMemory => true,
            Self::ChartRecords | Self::EarlierWork => false,
        }
    }
}

/// Whether search inspects only current items or includes superseded and withdrawn history.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchStandingScope {
    /// Return only items whose standing is [`HitStanding::Current`].
    #[default]
    CurrentOnly,
    /// Return current items alongside superseded and withdrawn items, each labeled with [`HitStanding`].
    CurrentAndHistorical,
}

impl SearchStandingScope {
    fn admits(self, standing: &HitStanding) -> bool {
        match self {
            Self::CurrentOnly => matches!(standing, HitStanding::Current),
            Self::CurrentAndHistorical => true,
        }
    }

    fn memory_version_scope(self) -> SearchVersionScope {
        match self {
            Self::CurrentOnly => SearchVersionScope::CurrentOnly,
            Self::CurrentAndHistorical => SearchVersionScope::CurrentAndPermittedPrior,
        }
    }
}

/// Lifecycle standing of a search hit at the time of the snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum HitStanding {
    /// Active current record, completed turn, saved draft, or active head memory entry.
    Current,
    /// Earlier revision, incomplete partial output, prior memory version, or superseded correction.
    Superseded {
        /// Identifier of the newer revision or version that replaced this item, when known.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        replaced_by: Option<String>,
    },
    /// Deleted/withdrawn source record, entered-in-error chart entry, cancelled revision, or withdrawn correction.
    Withdrawn,
}

impl HitStanding {
    fn sort_rank(&self) -> u8 {
        match self {
            Self::Current => 0,
            Self::Superseded { .. } => 1,
            Self::Withdrawn => 2,
        }
    }
}

/// Identifying clinical terminology code carried on a chart record envelope.
///
/// Never carries quantitative values, units, or display prose.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClinicalCode {
    /// Terminology system URI or namespace, when supplied by the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Exact terminology code token.
    pub code: String,
}

/// Dated metadata carried on a search hit.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchDate {
    /// Role or field name of the date (such as `effectiveDateTime`, `accepted_at`, or `recorded_on`).
    pub name: String,
    /// Date or UTC timestamp value in the owner's precision.
    pub value: String,
}

/// Authorship metadata carried on a search hit.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SearchAuthor {
    /// Clinician, organization, or participant recorded on a chart source.
    ChartParticipant {
        /// Participant role on the record (`author`, `requester`, `performer`, `recorder`, etc.).
        role: String,
        /// Display name or reference recorded by the source.
        actor: String,
    },
    /// Source adapter that delivered the record when no individual participant is named.
    SourceAdapter {
        /// Stable source adapter identifier.
        source_id: String,
    },
    /// Physician in the practice.
    Physician {
        /// Physician identifier.
        physician_id: String,
    },
    /// Coda resident agent in a physician conversation.
    CodaChat {
        /// Conversation identifier.
        chat_id: String,
        /// Accepted work identifier when bound to a specific run.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        work_id: Option<String>,
    },
    /// Coda reply-revision agent revising a patient-message draft.
    CodaReplyRevision {
        /// Reply draft identifier.
        reply_id: String,
        /// Accepted revision command key.
        command_key: String,
    },
    /// Background practice job (`patient-review`, `visit-briefing`, etc.).
    PracticeJob {
        /// Job kind.
        job_kind: String,
        /// Durable job run identifier.
        job_id: String,
    },
}

/// Revision-qualified chart reference compatible with `read_patient_sources`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChartRecordReference {
    /// Source adapter identifier holding this record.
    pub source_id: String,
    /// Stable candidate identity within the patient workspace.
    pub candidate_id: String,
    /// Immutable revision identity.
    pub revision: String,
    /// Continuation cursor (`None` on fresh discovery hits, matching `read_patient_sources`).
    pub cursor: Option<()>,
}

/// Kind of earlier conversation or reply-revision item matched by search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkHitKind {
    /// Physician message accepted in a patient conversation.
    ConversationUserMessage,
    /// Coda assistant response (completed or retained partial) in a patient conversation.
    ConversationAssistantReply,
    /// Physician's saved patient-message reply draft.
    SavedReplyDraft,
    /// Cancelled reply-revision instruction on a saved draft.
    CancelledReplyRevision,
    /// Archived earlier reply revision replaced by a subsequent revision.
    ArchivedReplyRevision,
}

/// Reference to an item in earlier conversations or reply-revision work.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "owner", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkReference {
    /// Message inside `physician_runtime_conversations`.
    ConversationMessage {
        /// Conversation identifier.
        chat_id: String,
        /// Durable message identifier (`Input::key` or [`super::ResponseId`]).
        message_id: String,
        /// Zero-based position in the conversation transcript.
        message_index: usize,
    },
    /// Current saved draft or cancelled revision on `physician_runtime_reply_drafts`.
    ReplyDraft {
        /// Reply work identifier.
        reply_id: String,
        /// Current saved draft revision number.
        draft_revision: u64,
        /// Latest accepted revision command key, when present.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        command_key: Option<String>,
    },
    /// Archived earlier revision inside `physician_runtime_reply_revisions`.
    ReplyRevisionHistory {
        /// Reply work identifier.
        reply_id: String,
        /// 1-based archive ordinal within this draft.
        ordinal: u64,
        /// Accepted command key of the archived revision.
        command_key: String,
    },
}

/// Pointer to a specific item inside a permitted patient-memory version,
/// carrying exact identities without body prose.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "entry_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MemoryEntryPointer {
    /// Private operating-index topic (`AgentJob` callers only; never emitted to `PhysicianFacing`).
    OperatingTopic {
        /// Topic key (`TopicKey`). Never carries the private topic note.
        topic: TopicKey,
    },
    /// Dated physician correction recorded in patient memory (never carries statement prose).
    Correction {
        /// Correction identifier (`CorrectionId`).
        correction_id: CorrectionId,
        /// Provenance pointer to the source conversation message.
        source_message: SourceMessageRef,
    },
    /// Saved physician-facing summary (never carries summary prose).
    PatientSummary {
        /// Memory version that authored the summary.
        summary_version: MemoryVersionNumber,
    },
}

/// Version-qualified reference to a durable patient-memory entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryVersionReference {
    /// Stable patient-memory identifier.
    pub memory_id: MemoryId,
    /// Snapshot version number (`1..=N`).
    pub version: MemoryVersionNumber,
    /// Specific entry matched inside that version.
    pub entry: MemoryEntryPointer,
}

/// Content-free search hit returned by [`SEARCH_TOOL`] and [`Store`] search methods.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "domain", rename_all = "snake_case", deny_unknown_fields)]
pub enum PatientSearchHit {
    /// Saved clinical record from a connected patient source.
    ChartRecord {
        /// Revision-qualified reference accepted directly by `read_patient_sources`.
        reference: ChartRecordReference,
        /// Source-declared record type (such as `Observation`, `Condition`, or `DocumentReference`).
        record_type: String,
        /// Product bucket classification.
        bucket: RecordBucket,
        /// Identifying clinical terminology codes on the record envelope.
        codes: Vec<ClinicalCode>,
        /// Current, superseded, or withdrawn standing.
        standing: HitStanding,
        /// Record dates in source precision.
        dates: Vec<SearchDate>,
        /// Clinicians, participants, or source adapter recorded on the item.
        authors: Vec<SearchAuthor>,
    },
    /// Earlier conversation turn or reply-revision work for this patient.
    EarlierWork {
        /// Exact conversation message or reply-draft reference.
        reference: WorkReference,
        /// Kind of earlier work item.
        work_kind: WorkHitKind,
        /// Current, superseded, or withdrawn standing.
        standing: HitStanding,
        /// Acceptance, publication, or archival timestamps in UTC ISO-8601.
        dates: Vec<SearchDate>,
        /// Physician or Coda job authors of this work item.
        authors: Vec<SearchAuthor>,
    },
    /// Versioned entry in durable patient memory.
    PatientMemory {
        /// Version-qualified memory reference.
        reference: MemoryVersionReference,
        /// Current, superseded, or withdrawn standing.
        standing: HitStanding,
        /// Clinical dates and snapshot save timestamp.
        dates: Vec<SearchDate>,
        /// Author(s) of the memory version and correction provenance.
        authors: Vec<SearchAuthor>,
    },
}

/// Keyset continuation cursor tied to a specific workspace snapshot digest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchCursor {
    /// Deterministic digest of the workspace's chart, work, and memory versions.
    pub snapshot_token: String,
    /// Exclusive sort key of the last hit delivered on the previous page.
    pub sort_key: String,
}

/// Validated patient-scoped search query.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatientSearchRequest {
    /// Whitespace-separated words, clinical codes, or dates to match (empty browses all items in scope).
    pub query: String,
    /// Optional case-insensitive chart `record_type` filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    /// Optional product [`RecordBucket`] filter for chart records.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<RecordBucket>,
    /// Domain slice filter (`All` by default).
    #[serde(default)]
    pub domain: SearchDomainFilter,
    /// Standing scope (`CurrentOnly` by default).
    #[serde(default)]
    pub standing: SearchStandingScope,
    /// Keyset continuation cursor returned by the preceding page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<SearchCursor>,
}

/// One bounded page of content-free patient search hits.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatientSearchPage {
    /// Ordered hits matching the request in this workspace snapshot.
    pub results: Vec<PatientSearchHit>,
    /// Continuation cursor when additional hits follow this page; `None` at the end.
    pub next: Option<SearchCursor>,
}

/// Authorized outcome of a patient search, distinct from operational failure ([`PatientSearchError`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatientSearchOutcome {
    /// Caller is authorized for this practice and patient; `results` may legitimately be empty.
    Permitted(PatientSearchPage),
    /// Caller lacks practice, session, or clinical-panel authorization for this patient.
    Unauthorized {
        /// Refusal reason from the authority or access check.
        reason: String,
    },
}

/// Audience and personal-work ownership context of a direct search caller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchCaller {
    /// Physician-facing UI or route call: scopes personal drafts/chats to `physician_id`
    /// and structurally excludes `AgentOnly` operating-index memory entries.
    PhysicianFacing {
        /// Signed-in physician identifier.
        physician_id: String,
    },
    /// Internal Coda resident chat job acting for `physician_id`.
    ChatJob {
        /// Signed-in physician identifier owning the conversation.
        physician_id: String,
    },
    /// Practice-owned background job (`patient-review`, `visit-briefing`, etc.).
    PracticeJob {
        /// Job kind (`patient-review`, `visit-briefing`, etc.).
        job_kind: String,
        /// Durable job identifier.
        job_id: String,
    },
}

impl SearchCaller {
    fn validate(&self) -> Result<(), PatientSearchValidationError> {
        match self {
            Self::PhysicianFacing { physician_id } | Self::ChatJob { physician_id } => {
                if physician_id.trim().is_empty() {
                    return Err(PatientSearchValidationError::EmptyField("physician_id"));
                }
                Ok(())
            }
            Self::PracticeJob { job_kind, job_id } => {
                if job_kind.trim().is_empty() {
                    return Err(PatientSearchValidationError::EmptyField("job_kind"));
                }
                if job_id.trim().is_empty() {
                    return Err(PatientSearchValidationError::EmptyField("job_id"));
                }
                Ok(())
            }
        }
    }

    fn audience(&self) -> SearchAudienceScope {
        match self {
            Self::PhysicianFacing { .. } => SearchAudienceScope::PhysicianFacing,
            Self::ChatJob { .. } | Self::PracticeJob { .. } => SearchAudienceScope::AgentJob,
        }
    }

    fn personal_physician(&self) -> Option<&str> {
        match self {
            Self::PhysicianFacing { physician_id } | Self::ChatJob { physician_id } => {
                Some(physician_id.as_str())
            }
            Self::PracticeJob { .. } => None,
        }
    }
}

/// Validation failure on a [`PatientSearchRequest`] or caller scope.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PatientSearchValidationError {
    /// Required scope or caller identifier was blank.
    #[error("{0} must not be empty")]
    EmptyField(&'static str),
    /// Session hash was not a 32-byte SHA-256 digest.
    #[error("invalid session reference")]
    InvalidSessionReference,
    /// Invalid practice or workspace key.
    #[error(transparent)]
    PatientKey(#[from] MemoryValidationError),
    /// Query exceeded [`MAX_SEARCH_QUERY_BYTES`].
    #[error("use at most {MAX_SEARCH_QUERY_BYTES} bytes of search text")]
    QueryTooLong,
    /// `record_type` filter was supplied as an empty or whitespace-only string.
    #[error("record_type filter must not be empty when supplied")]
    EmptyRecordType,
    /// Continuation cursor had an empty token or sort key.
    #[error("continuation cursor fields must not be empty")]
    EmptyCursor,
}

/// Operational or data-integrity failure during patient search.
///
/// Kept strictly distinct from an empty [`PatientSearchPage`] and from
/// [`PatientSearchOutcome::Unauthorized`].
#[derive(Debug, thiserror::Error)]
pub enum PatientSearchError {
    /// Caller scope or search request failed validation.
    #[error(transparent)]
    Validation(#[from] PatientSearchValidationError),
    /// Continuation cursor was issued against an earlier workspace snapshot.
    #[error(
        "patient information changed since the previous page; repeat the search from the beginning"
    )]
    StaleCursor,
    /// Database read or transaction failed.
    #[error("patient search database failure")]
    Database(#[from] DbErr),
    /// Durable patient-memory read or integrity check failed.
    #[error("patient memory search failure")]
    Memory(#[from] MemoryError),
    /// Stored chart source observation failed JSON decoding.
    #[error("corrupt chart source observation for candidate {candidate_id}")]
    CorruptSourceJson {
        /// Candidate identifier of the unreadable row.
        candidate_id: String,
        /// Underlying JSON error.
        #[source]
        source: serde_json::Error,
    },
    /// Stored chart source observation failed SHA-256 or identity validation.
    #[error("corrupt chart source revision for candidate {candidate_id}")]
    CorruptSourceIntegrity {
        /// Candidate identifier of the invalid row.
        candidate_id: String,
        /// Underlying integrity failure.
        #[source]
        source: InvalidSourceRevisionError,
    },
    /// Stored chart record attachment or scan provenance failed resolution.
    #[error("chart record part resolution failed")]
    PartResolution(#[from] RecordReadError),
    /// Stored source version counter was negative.
    #[error("corrupt negative source version {0} in database")]
    CorruptSourceVersion(i64),
    /// Stored reply-revision archive ordinal was non-positive.
    #[error("corrupt reply revision ordinal {0} in database")]
    CorruptArchiveOrdinal(i64),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SearchToolArgs {
    query: String,
    #[serde(default)]
    record_type: Option<String>,
    #[serde(default)]
    bucket: Option<RecordBucket>,
    #[serde(default)]
    domain: Option<SearchDomainFilter>,
    #[serde(default)]
    standing: Option<SearchStandingScope>,
    #[serde(default)]
    after: Option<SearchCursor>,
}

impl SearchToolArgs {
    fn into_request(self) -> Result<PatientSearchRequest, PatientSearchValidationError> {
        let req = PatientSearchRequest {
            query: self.query,
            record_type: self.record_type,
            bucket: self.bucket,
            domain: self.domain.unwrap_or_default(),
            standing: self.standing.unwrap_or_default(),
            after: self.after,
        };
        validate_request(&req)?;
        Ok(req)
    }
}

fn validate_request(req: &PatientSearchRequest) -> Result<(), PatientSearchValidationError> {
    if req.query.len() > MAX_SEARCH_QUERY_BYTES {
        return Err(PatientSearchValidationError::QueryTooLong);
    }
    if req
        .record_type
        .as_ref()
        .is_some_and(|rt| rt.trim().is_empty())
    {
        return Err(PatientSearchValidationError::EmptyRecordType);
    }
    if req.after.as_ref().is_some_and(|cursor| {
        cursor.snapshot_token.trim().is_empty() || cursor.sort_key.trim().is_empty()
    }) {
        return Err(PatientSearchValidationError::EmptyCursor);
    }
    Ok(())
}

/// Builds the unified [`SEARCH_TOOL`] definition offered to Coda jobs.
pub fn search_tool() -> Tool {
    let buckets: Vec<&str> = RecordBucket::ALL.iter().map(|b| b.as_str()).collect();
    Tool {
        name: SEARCH_TOOL.into(),
        description: "Search everything Coda holds for this patient across saved chart records, earlier conversations and reply drafts, and durable patient memory. Matches words, clinical codes, decoded attachments, dates, and authors; use an empty query to browse. Results return references, dates, standing, codes, and authors only — never record bodies or message text. Pass a chart reference to read_patient_sources, a conversation reference to history tools, or a memory version to read_patient_memory to read the underlying content. Continue pagination by passing next as after.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "maxLength": MAX_SEARCH_QUERY_BYTES
                },
                "record_type": {
                    "type": ["string", "null"],
                    "minLength": 1
                },
                "bucket": {
                    "anyOf": [
                        { "type": "null" },
                        { "type": "string", "enum": buckets }
                    ]
                },
                "domain": {
                    "anyOf": [
                        { "type": "null" },
                        {
                            "type": "string",
                            "enum": ["all", "chart_records", "earlier_work", "patient_memory"]
                        }
                    ]
                },
                "standing": {
                    "anyOf": [
                        { "type": "null" },
                        {
                            "type": "string",
                            "enum": ["current_only", "current_and_historical"]
                        }
                    ]
                },
                "after": {
                    "anyOf": [
                        { "type": "null" },
                        {
                            "type": "object",
                            "properties": {
                                "snapshot_token": { "type": "string", "minLength": 1 },
                                "sort_key": { "type": "string", "minLength": 1 }
                            },
                            "required": ["snapshot_token", "sort_key"],
                            "additionalProperties": false
                        }
                    ]
                }
            },
            "required": ["query"],
            "additionalProperties": false
        }),
    }
}

struct RankedHit {
    sort_key: String,
    hit: PatientSearchHit,
}

struct QueryMatcher {
    terms: Vec<String>,
}

impl QueryMatcher {
    fn new(raw_query: &str) -> Self {
        let terms = raw_query
            .split_whitespace()
            .map(str::to_lowercase)
            .collect();
        Self { terms }
    }

    fn matches(&self, haystack_chunks: &[String]) -> bool {
        if self.terms.is_empty() {
            return true;
        }
        let lowered: Vec<String> = haystack_chunks.iter().map(|s| s.to_lowercase()).collect();
        self.terms
            .iter()
            .all(|term| lowered.iter().any(|chunk| chunk.contains(term.as_str())))
    }
}

/// Formats Unix epoch seconds into unambiguous UTC ISO-8601 (`YYYY-MM-DDTHH:MM:SSZ`).
fn format_unix_utc(epoch_seconds: i64) -> String {
    let days = epoch_seconds.div_euclid(86_400);
    let secs_of_day = epoch_seconds.rem_euclid(86_400);
    let hour = secs_of_day / 3_600;
    let minute = (secs_of_day % 3_600) / 60;
    let second = secs_of_day % 60;

    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{year:04}-{m:02}-{d:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn system_aliases(system: &str) -> &'static [&'static str] {
    let lower = system.to_ascii_lowercase();
    if lower.contains("loinc") {
        &["loinc"]
    } else if lower.contains("snomed") || lower.contains("sct") {
        &["snomed", "sct"]
    } else if lower.contains("rxnorm") {
        &["rxnorm"]
    } else if lower.contains("icd-10-cm") || lower.contains("icd10") || lower.contains("icd-10") {
        &["icd-10-cm", "icd-10", "icd10"]
    } else if lower.contains("cvx") {
        &["cvx"]
    } else if lower.contains("cpt") {
        &["cpt"]
    } else if lower.contains("ndc") {
        &["ndc"]
    } else {
        &[]
    }
}

fn coding_codes_in_value(value: &Value, out: &mut Vec<ClinicalCode>) {
    match value {
        Value::String(raw) => {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                out.push(ClinicalCode {
                    system: None,
                    code: trimmed.to_owned(),
                });
            }
        }
        Value::Array(items) => {
            for item in items {
                coding_codes_in_value(item, out);
            }
        }
        Value::Object(map) => {
            if let Some(codings) = map.get("coding").and_then(Value::as_array) {
                for coding in codings {
                    if let Some(coding_obj) = coding.as_object()
                        && let Some(code) = coding_obj
                            .get("code")
                            .and_then(Value::as_str)
                            .map(str::trim)
                            .filter(|c| !c.is_empty())
                    {
                        let system = coding_obj
                            .get("system")
                            .and_then(Value::as_str)
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                            .map(str::to_owned);
                        out.push(ClinicalCode {
                            system,
                            code: code.to_owned(),
                        });
                    }
                }
            } else if let Some(code) = map
                .get("code")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|c| !c.is_empty())
                && !map.contains_key("value")
                && !map.contains_key("unit")
            {
                let system = map
                    .get("system")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned);
                out.push(ClinicalCode {
                    system,
                    code: code.to_owned(),
                });
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn extract_envelope_codes(parsed_json: Option<&Value>) -> Vec<ClinicalCode> {
    let Some(Value::Object(obj)) = parsed_json else {
        return Vec::new();
    };
    let mut raw = Vec::new();
    for key in [
        "code",
        "medicationCodeableConcept",
        "vaccineCode",
        "reasonCode",
        "type",
        "serviceType",
        "class",
    ] {
        if let Some(field) = obj.get(key) {
            coding_codes_in_value(field, &mut raw);
        }
    }
    if let Some(components) = obj.get("component").and_then(Value::as_array) {
        for component in components {
            if let Some(comp_code) = component.get("code") {
                coding_codes_in_value(comp_code, &mut raw);
            }
        }
    }
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for item in raw {
        if seen.insert(item.clone()) {
            deduped.push(item);
        }
    }
    deduped
}

fn has_coding_code(value: &Value, expected_lower: &str) -> bool {
    let mut codes = Vec::new();
    coding_codes_in_value(value, &mut codes);
    codes
        .iter()
        .any(|c| c.code.eq_ignore_ascii_case(expected_lower))
}

/// Classifies a chart record into one of the 16 product [`RecordBucket`]s.
pub fn classify_record_bucket(record_type: &str, parsed_json: Option<&Value>) -> RecordBucket {
    let lower = record_type.trim().to_ascii_lowercase();
    let category = parsed_json
        .and_then(Value::as_object)
        .and_then(|o| o.get("category"));

    match lower.as_str() {
        "patient" | "relatedperson" | "coverage" => RecordBucket::Patient,
        "condition" => {
            if category.is_some_and(|cat| {
                has_coding_code(cat, "past-medical-history")
                    || has_coding_code(cat, "social-history")
            }) {
                RecordBucket::History
            } else {
                RecordBucket::Problems
            }
        }
        "medicationrequest"
        | "medicationstatement"
        | "medicationadministration"
        | "medicationdispense"
        | "medication" => RecordBucket::Medications,
        "allergyintolerance" => RecordBucket::Allergies,
        "observation" => {
            if category.is_some_and(|cat| has_coding_code(cat, "vital-signs")) {
                RecordBucket::Vitals
            } else if category.is_some_and(|cat| has_coding_code(cat, "social-history")) {
                RecordBucket::History
            } else if category.is_some_and(|cat| has_coding_code(cat, "survey")) {
                RecordBucket::Forms
            } else {
                RecordBucket::Results
            }
        }
        "diagnosticreport" | "imagingstudy" | "specimen" | "molecularsequence" => {
            RecordBucket::Results
        }
        "composition" => RecordBucket::Notes,
        "documentreference" => {
            if category.is_some_and(|cat| has_coding_code(cat, "clinical-note")) {
                RecordBucket::Notes
            } else {
                RecordBucket::Documents
            }
        }
        "media" | "binary" => RecordBucket::Documents,
        "encounter" | "appointment" | "episodeofcare" => RecordBucket::Visits,
        "procedure" => RecordBucket::Procedures,
        "immunization" => RecordBucket::Immunizations,
        "servicerequest" | "nutritionorder" | "devicerequest" | "visionprescription" => {
            RecordBucket::Orders
        }
        "familymemberhistory" => RecordBucket::History,
        "questionnaireresponse" | "questionnaire" => RecordBucket::Forms,
        "communication" | "communicationrequest" => RecordBucket::Messages,
        _ => RecordBucket::Other,
    }
}

fn fhir_intrinsic_standing(parsed_json: Option<&Value>) -> HitStanding {
    let Some(Value::Object(obj)) = parsed_json else {
        return HitStanding::Current;
    };
    let status = obj
        .get("status")
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    let doc_status = obj
        .get("docStatus")
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    let verification_error = obj
        .get("verificationStatus")
        .is_some_and(|v| has_coding_code(v, "entered-in-error"));

    if status.eq_ignore_ascii_case("entered-in-error")
        || doc_status.eq_ignore_ascii_case("entered-in-error")
        || verification_error
    {
        HitStanding::Withdrawn
    } else if status.eq_ignore_ascii_case("superseded")
        || doc_status.eq_ignore_ascii_case("superseded")
    {
        HitStanding::Superseded { replaced_by: None }
    } else {
        HitStanding::Current
    }
}

fn extract_chart_dates(source: &SourceRevision, parsed_json: Option<&Value>) -> Vec<SearchDate> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    let mut push_date = |name: &str, val: &str| {
        let name = name.trim();
        let val = val.trim();
        if !name.is_empty() && !val.is_empty() {
            let item = SearchDate {
                name: name.to_owned(),
                value: val.to_owned(),
            };
            if seen.insert((item.name.clone(), item.value.clone())) {
                out.push(item);
            }
        }
    };

    for d in &source.dates {
        push_date(&d.name, &d.value);
    }

    if let Some(Value::Object(obj)) = parsed_json {
        for key in [
            "effectiveDateTime",
            "issued",
            "authoredOn",
            "recordedDate",
            "onsetDateTime",
            "abatementDateTime",
            "performedDateTime",
            "occurrenceDateTime",
            "date",
            "sent",
            "received",
            "birthDate",
            "authored",
            "created",
        ] {
            if let Some(val) = obj.get(key).and_then(Value::as_str) {
                push_date(key, val);
            }
        }
        for (period_key, label) in [
            ("effectivePeriod", "effectivePeriod.start"),
            ("period", "period.start"),
            ("performedPeriod", "performedPeriod.start"),
        ] {
            if let Some(start) = obj
                .get(period_key)
                .and_then(Value::as_object)
                .and_then(|p| p.get("start"))
                .and_then(Value::as_str)
            {
                push_date(label, start);
            }
        }
        if let Some(start) = obj.get("start").and_then(Value::as_str) {
            push_date("start", start);
        }
    }
    out
}

fn collect_actor_strings(value: &Value, role: &str, out: &mut Vec<(String, String)>) {
    match value {
        Value::String(s) => {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                out.push((role.to_owned(), trimmed.to_owned()));
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_actor_strings(item, role, out);
            }
        }
        Value::Object(map) => {
            if let Some(display) = map
                .get("display")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                out.push((role.to_owned(), display.to_owned()));
            } else if let Some(reference) = map
                .get("reference")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                out.push((role.to_owned(), reference.to_owned()));
            }
            for nested in ["actor", "individual", "agent"] {
                if let Some(inner) = map.get(nested) {
                    collect_actor_strings(inner, role, out);
                }
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn extract_chart_authors(
    source: &SourceRevision,
    parsed_json: Option<&Value>,
) -> Vec<SearchAuthor> {
    let mut pairs = Vec::new();
    if let Some(Value::Object(obj)) = parsed_json {
        for role in [
            "author",
            "requester",
            "recorder",
            "asserter",
            "performer",
            "participant",
            "sender",
            "enterer",
            "operator",
        ] {
            if let Some(field) = obj.get(role) {
                collect_actor_strings(field, role, &mut pairs);
            }
        }
    }
    if let Some(prov_author) = source.provenance.as_object().and_then(|p| p.get("author")) {
        collect_actor_strings(prov_author, "provenance_author", &mut pairs);
    }

    let mut seen = BTreeSet::new();
    let mut authors = Vec::new();
    for (role, actor) in pairs {
        let candidate = SearchAuthor::ChartParticipant { role, actor };
        if seen.insert(candidate.clone()) {
            authors.push(candidate);
        }
    }
    if authors.is_empty() {
        authors.push(SearchAuthor::SourceAdapter {
            source_id: source.source_id.clone(),
        });
    }
    authors
}

fn collect_searchable_json_tokens(
    value: &Value,
    in_encoded_attachment: bool,
    chunks: &mut Vec<String>,
) {
    match value {
        Value::Null => {}
        Value::Bool(b) => chunks.push(b.to_string()),
        Value::Number(n) => chunks.push(n.to_string()),
        Value::String(s) => {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                chunks.push(trimmed.to_owned());
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_searchable_json_tokens(item, in_encoded_attachment, chunks);
            }
        }
        Value::Object(map) => {
            let looks_like_attachment = in_encoded_attachment
                || (map.contains_key("data") && map.contains_key("contentType"));
            if let Some(system) = map.get("system").and_then(Value::as_str).map(str::trim)
                && !system.is_empty()
            {
                for code_key in ["code", "value"] {
                    if let Some(code) = map.get(code_key).and_then(Value::as_str).map(str::trim)
                        && !code.is_empty()
                    {
                        chunks.push(format!("{system}:{code}"));
                        chunks.push(format!("{system}|{code}"));
                        for alias in system_aliases(system) {
                            chunks.push((*alias).to_owned());
                            chunks.push(format!("{alias}:{code}"));
                        }
                    }
                }
            }
            for (key, child) in map {
                if looks_like_attachment && key == "data" {
                    // Base64 attachment payloads are represented cleanly in `readable_parts`;
                    // skipping raw base64 avoids spurious substring matches on encoded bytes.
                    continue;
                }
                let child_attachment = matches!(key.as_str(), "attachment" | "presentedForm");
                collect_searchable_json_tokens(child, child_attachment, chunks);
            }
        }
    }
}

struct ChartHaystackInput<'a> {
    source: &'a SourceRevision,
    parsed_json: Option<&'a Value>,
    resolved_parts: &'a [ResolvedPart],
    bucket: RecordBucket,
    codes: &'a [ClinicalCode],
    dates: &'a [SearchDate],
    authors: &'a [SearchAuthor],
}

fn build_chart_haystack(input: ChartHaystackInput<'_>) -> Vec<String> {
    let ChartHaystackInput {
        source,
        parsed_json,
        resolved_parts,
        bucket,
        codes,
        dates,
        authors,
    } = input;
    let mut chunks = vec![
        source.source_id.clone(),
        source.candidate_id.clone(),
        source.revision.clone(),
        source.record_type.clone(),
        bucket.as_str().to_owned(),
    ];
    for code in codes {
        chunks.push(code.code.clone());
        if let Some(sys) = &code.system {
            chunks.push(sys.clone());
            chunks.push(format!("{sys}:{}", code.code));
            for alias in system_aliases(sys) {
                chunks.push((*alias).to_owned());
                chunks.push(format!("{alias}:{}", code.code));
            }
        }
    }
    for d in dates {
        chunks.push(d.name.clone());
        chunks.push(d.value.clone());
    }
    for author in authors {
        match author {
            SearchAuthor::ChartParticipant { role, actor } => {
                chunks.push(role.clone());
                chunks.push(actor.clone());
            }
            SearchAuthor::SourceAdapter { source_id } => chunks.push(source_id.clone()),
            SearchAuthor::Physician { physician_id } => chunks.push(physician_id.clone()),
            SearchAuthor::CodaChat { chat_id, work_id } => {
                chunks.push(chat_id.clone());
                if let Some(w) = work_id {
                    chunks.push(w.clone());
                }
            }
            SearchAuthor::CodaReplyRevision {
                reply_id,
                command_key,
            } => {
                chunks.push(reply_id.clone());
                chunks.push(command_key.clone());
            }
            SearchAuthor::PracticeJob { job_kind, job_id } => {
                chunks.push(job_kind.clone());
                chunks.push(job_id.clone());
            }
        }
    }
    for resolved in resolved_parts {
        match &resolved.part {
            PartIdentity::NativeRecord => {}
            PartIdentity::Retained { label, .. } => {
                chunks.push(label.clone());
            }
            PartIdentity::Attachment {
                attachment_id,
                label,
                media_type,
            } => {
                chunks.push(attachment_id.clone());
                chunks.push(label.clone());
                if let Some(mt) = media_type {
                    chunks.push(mt.clone());
                }
            }
            PartIdentity::ScanDocument { scan_id, label }
            | PartIdentity::ScanPage { scan_id, label, .. } => {
                chunks.push(scan_id.clone());
                chunks.push(label.clone());
            }
        }
        match &resolved.state {
            PartState::Readable { text } => {
                if !matches!(resolved.part, PartIdentity::NativeRecord) {
                    chunks.push(text.clone());
                }
            }
            PartState::Missing { .. }
            | PartState::Unreadable { .. }
            | PartState::Unavailable { .. } => {}
        }
    }
    match parsed_json {
        Some(json_val) => collect_searchable_json_tokens(json_val, false, &mut chunks),
        None => chunks.push(source.content.clone()),
    }
    chunks
}

fn decode_source_row(row: &source::Model) -> Result<Option<SourceRevision>, PatientSearchError> {
    row.revision
        .clone()
        .map(|raw| {
            let decoded: SourceRevision = serde_json::from_value(raw).map_err(|err| {
                PatientSearchError::CorruptSourceJson {
                    candidate_id: row.candidate.clone(),
                    source: err,
                }
            })?;
            decoded.validate_integrity().map_err(|err| {
                PatientSearchError::CorruptSourceIntegrity {
                    candidate_id: row.candidate.clone(),
                    source: err,
                }
            })?;
            Ok(decoded)
        })
        .transpose()
}

fn project_chart_candidate(
    rows_desc: &[source::Model],
    request: &PatientSearchRequest,
    matcher: &QueryMatcher,
    out: &mut Vec<RankedHit>,
) -> Result<(), PatientSearchError> {
    let Some(newest_row) = rows_desc.first() else {
        return Ok(());
    };
    let newest_decoded = decode_source_row(newest_row)?;

    let mut entries: Vec<(SourceRevision, HitStanding)> = Vec::new();
    match newest_decoded {
        Some(head) => {
            let head_parsed: Option<Value> = serde_json::from_str(&head.content).ok();
            let head_standing = fhir_intrinsic_standing(head_parsed.as_ref());
            let head_rev = head.revision.clone();
            entries.push((head, head_standing));

            if matches!(request.standing, SearchStandingScope::CurrentAndHistorical) {
                let mut seen_revisions = BTreeSet::new();
                seen_revisions.insert(head_rev.clone());
                for older_row in &rows_desc[1..] {
                    if let Some(older) = decode_source_row(older_row)?
                        && seen_revisions.insert(older.revision.clone())
                    {
                        entries.push((
                            older,
                            HitStanding::Superseded {
                                replaced_by: Some(head_rev.clone()),
                            },
                        ));
                    }
                }
            }
        }
        None => {
            if matches!(request.standing, SearchStandingScope::CurrentOnly) {
                return Ok(());
            }
            let mut seen_revisions = BTreeSet::new();
            let mut latest_withdrawn_rev: Option<String> = None;
            for older_row in &rows_desc[1..] {
                if let Some(older) = decode_source_row(older_row)?
                    && seen_revisions.insert(older.revision.clone())
                {
                    let standing = match &latest_withdrawn_rev {
                        None => {
                            latest_withdrawn_rev = Some(older.revision.clone());
                            HitStanding::Withdrawn
                        }
                        Some(newer_rev) => HitStanding::Superseded {
                            replaced_by: Some(newer_rev.clone()),
                        },
                    };
                    entries.push((older, standing));
                }
            }
        }
    }

    for (source, standing) in entries {
        let resolved_parts = ProvenancePartReader.resolve_parts(&source)?;
        if !request.standing.admits(&standing) {
            continue;
        }
        if let Some(expected_type) = &request.record_type
            && !source
                .record_type
                .eq_ignore_ascii_case(expected_type.trim())
        {
            continue;
        }
        let parsed_json: Option<Value> = serde_json::from_str(&source.content).ok();
        let bucket = classify_record_bucket(&source.record_type, parsed_json.as_ref());
        if let Some(expected_bucket) = request.bucket
            && bucket != expected_bucket
        {
            continue;
        }
        let codes = extract_envelope_codes(parsed_json.as_ref());
        let dates = extract_chart_dates(&source, parsed_json.as_ref());
        let authors = extract_chart_authors(&source, parsed_json.as_ref());
        let haystack = build_chart_haystack(ChartHaystackInput {
            source: &source,
            parsed_json: parsed_json.as_ref(),
            resolved_parts: &resolved_parts,
            bucket,
            codes: &codes,
            dates: &dates,
            authors: &authors,
        });
        if !matcher.matches(&haystack) {
            continue;
        }
        let sort_key = format!(
            "0\x1f{}\x1f{}\x1f{}\x1f{}",
            source.source_id,
            source.candidate_id,
            standing.sort_rank(),
            source.revision
        );
        out.push(RankedHit {
            sort_key,
            hit: PatientSearchHit::ChartRecord {
                reference: ChartRecordReference {
                    source_id: source.source_id,
                    candidate_id: source.candidate_id,
                    revision: source.revision,
                    cursor: None,
                },
                record_type: source.record_type,
                bucket,
                codes,
                standing,
                dates,
                authors,
            },
        });
    }
    Ok(())
}

async fn collect_chart_hits_in_tx(
    tx: &impl ConnectionTrait,
    workspace: &str,
    request: &PatientSearchRequest,
    matcher: &QueryMatcher,
    hasher: &mut Sha256,
    out: &mut Vec<RankedHit>,
) -> Result<(), PatientSearchError> {
    let version_row = source_version::Entity::find_by_id(workspace.to_owned())
        .one(tx)
        .await?;
    let pinned_version = match version_row {
        Some(row) => u64::try_from(row.version)
            .map_err(|_| PatientSearchError::CorruptSourceVersion(row.version))?,
        None => 0,
    };
    hasher.update(b"chart_version:");
    hasher.update(pinned_version.to_be_bytes());
    hasher.update(b"\n");

    if !request.domain.includes_chart() || pinned_version == 0 {
        return Ok(());
    }

    let rows = source::Entity::find()
        .filter(source::Column::Workspace.eq(workspace))
        .filter(source::Column::ObservedAtVersion.lte(pinned_version as i64))
        .order_by_asc(source::Column::Candidate)
        .order_by_desc(source::Column::ObservedAtVersion)
        .all(tx)
        .await?;

    let mut start = 0;
    while start < rows.len() {
        let candidate = &rows[start].candidate;
        let mut end = start + 1;
        while end < rows.len() && &rows[end].candidate == candidate {
            end += 1;
        }
        project_chart_candidate(&rows[start..end], request, matcher, out)?;
        start = end;
    }
    Ok(())
}

async fn load_command_timestamps(
    tx: &impl ConnectionTrait,
    practice: &str,
    physician_filter: Option<&str>,
) -> Result<BTreeMap<(String, String), i64>, DbErr> {
    use command::Column as C;
    let mut query = command::Entity::find().filter(C::Practice.eq(practice));
    if let Some(physician) = physician_filter {
        query = query.filter(C::Physician.eq(physician));
    }
    let rows = query.all(tx).await?;
    let mut out = BTreeMap::new();
    for row in rows {
        out.insert((row.physician, row.key), row.accepted_at);
    }
    Ok(out)
}

async fn collect_work_hits_in_tx(
    tx: &impl ConnectionTrait,
    patient: &PatientKey,
    caller: &SearchCaller,
    request: &PatientSearchRequest,
    matcher: &QueryMatcher,
    hasher: &mut Sha256,
    out: &mut Vec<RankedHit>,
) -> Result<(), PatientSearchError> {
    let physician_filter = caller.personal_physician();

    let mut conv_query = conversation::Entity::find()
        .filter(conversation::Column::Practice.eq(patient.practice()))
        .filter(conversation::Column::Workspace.eq(patient.workspace()));
    if let Some(physician) = physician_filter {
        conv_query = conv_query.filter(conversation::Column::Physician.eq(physician));
    }
    let conv_rows = conv_query
        .order_by_asc(conversation::Column::Physician)
        .order_by_asc(conversation::Column::Chat)
        .all(tx)
        .await?;

    for row in &conv_rows {
        hasher.update(b"conv:");
        hasher.update(row.physician.as_bytes());
        hasher.update(b":");
        hasher.update(row.chat.as_bytes());
        hasher.update(b":");
        hasher.update(row.revision.to_be_bytes());
        hasher.update(row.status.as_str().as_bytes());
        hasher.update(b"\n");
    }

    let draft_rows = if let Some(physician) = physician_filter {
        reply_draft::Entity::find()
            .filter(reply_draft::Column::Practice.eq(patient.practice()))
            .filter(reply_draft::Column::Physician.eq(physician))
            .filter(reply_draft::Column::Workspace.eq(patient.workspace()))
            .order_by_asc(reply_draft::Column::Reply)
            .all(tx)
            .await?
    } else {
        Vec::new()
    };

    for row in &draft_rows {
        hasher.update(b"draft:");
        hasher.update(row.physician.as_bytes());
        hasher.update(b":");
        hasher.update(row.reply.as_bytes());
        hasher.update(b":");
        hasher.update(row.revision.to_be_bytes());
        hasher.update(b"\n");
    }

    let archived_rows = if let Some(physician) = physician_filter {
        reply_revision::Entity::find()
            .filter(reply_revision::Column::Practice.eq(patient.practice()))
            .filter(reply_revision::Column::Physician.eq(physician))
            .filter(reply_revision::Column::Workspace.eq(patient.workspace()))
            .order_by_asc(reply_revision::Column::Reply)
            .order_by_asc(reply_revision::Column::Ordinal)
            .all(tx)
            .await?
    } else {
        Vec::new()
    };

    for row in &archived_rows {
        hasher.update(b"archived_rev:");
        hasher.update(row.reply.as_bytes());
        hasher.update(b":");
        hasher.update(row.ordinal.to_be_bytes());
        hasher.update(b"\n");
    }

    // `record_type` and `bucket` are chart-record filters; earlier work has neither.
    if !request.domain.includes_work() || request.record_type.is_some() || request.bucket.is_some()
    {
        return Ok(());
    }

    let command_times = load_command_timestamps(tx, patient.practice(), physician_filter).await?;

    for row in conv_rows {
        let physician_id = row.physician.clone();
        let chat_id = row.chat.clone();
        let conv = control::conversation(row, Vec::new())?;
        let mut active_work: Option<String> = None;

        for (idx, msg) in conv.messages.iter().enumerate() {
            let (message_id, work_kind, standing, dates, authors) = match &msg.role {
                MessageRole::User { id, work, .. } => {
                    active_work = Some(work.clone());
                    let mut dates = Vec::new();
                    if let Some(&accepted_at) =
                        command_times.get(&(physician_id.clone(), id.clone()))
                    {
                        dates.push(SearchDate {
                            name: "accepted_at".into(),
                            value: format_unix_utc(accepted_at),
                        });
                    } else if conv.opened_at > 0 {
                        dates.push(SearchDate {
                            name: "opened_at".into(),
                            value: format_unix_utc(conv.opened_at),
                        });
                    }
                    (
                        id.clone(),
                        WorkHitKind::ConversationUserMessage,
                        HitStanding::Current,
                        dates,
                        vec![SearchAuthor::Physician {
                            physician_id: physician_id.clone(),
                        }],
                    )
                }
                MessageRole::Assistant { id, partial } => {
                    let standing = if *partial {
                        HitStanding::Superseded { replaced_by: None }
                    } else {
                        HitStanding::Current
                    };
                    let mut dates = Vec::new();
                    let ts = if conv.last_activity_at > 0 {
                        Some(conv.last_activity_at)
                    } else if conv.opened_at > 0 {
                        Some(conv.opened_at)
                    } else {
                        None
                    };
                    if let Some(ts) = ts {
                        dates.push(SearchDate {
                            name: "published_at".into(),
                            value: format_unix_utc(ts),
                        });
                    }
                    (
                        id.clone(),
                        WorkHitKind::ConversationAssistantReply,
                        standing,
                        dates,
                        vec![SearchAuthor::CodaChat {
                            chat_id: chat_id.clone(),
                            work_id: active_work.clone(),
                        }],
                    )
                }
            };

            if !request.standing.admits(&standing) {
                continue;
            }
            let mut haystack = vec![
                msg.text.clone(),
                chat_id.clone(),
                message_id.clone(),
                physician_id.clone(),
            ];
            for d in &dates {
                haystack.push(d.value.clone());
            }
            if !matcher.matches(&haystack) {
                continue;
            }
            let sort_key =
                format!("1\x1f0\x1f{physician_id}\x1f{chat_id}\x1f{idx:016}\x1f{message_id}");
            out.push(RankedHit {
                sort_key,
                hit: PatientSearchHit::EarlierWork {
                    reference: WorkReference::ConversationMessage {
                        chat_id: chat_id.clone(),
                        message_id,
                        message_index: idx,
                    },
                    work_kind,
                    standing,
                    dates,
                    authors,
                },
            });
        }
    }

    let mut current_draft_revision_by_reply: BTreeMap<String, u64> = BTreeMap::new();
    for row in draft_rows {
        let physician_id = row.physician.clone();
        let reply_id = row.reply.clone();
        let draft = control::draft(row, Vec::new())?;
        current_draft_revision_by_reply.insert(reply_id.clone(), draft.revision);

        if request.standing.admits(&HitStanding::Current) {
            let mut saved_dates = Vec::new();
            let mut saved_authors = vec![SearchAuthor::Physician {
                physician_id: physician_id.clone(),
            }];
            let mut saved_command_key = None;
            let mut haystack = vec![draft.text.clone(), reply_id.clone(), physician_id.clone()];
            if matches!(draft.status, DraftStatus::Completed)
                && let Some(acc) = &draft.accepted
            {
                let key = acc.input.key.clone();
                if let Some(&accepted_at) = command_times.get(&(physician_id.clone(), key.clone()))
                {
                    let formatted = format_unix_utc(accepted_at);
                    haystack.push(formatted.clone());
                    saved_dates.push(SearchDate {
                        name: "accepted_at".into(),
                        value: formatted,
                    });
                }
                saved_authors.push(SearchAuthor::CodaReplyRevision {
                    reply_id: reply_id.clone(),
                    command_key: key.clone(),
                });
                haystack.push(acc.input.instruction.clone());
                haystack.push(key.clone());
                saved_command_key = Some(key);
            }
            if matcher.matches(&haystack) {
                let sort_key = format!(
                    "1\x1f1\x1f{physician_id}\x1f{reply_id}\x1f{:016}\x1fdraft",
                    draft.revision
                );
                out.push(RankedHit {
                    sort_key,
                    hit: PatientSearchHit::EarlierWork {
                        reference: WorkReference::ReplyDraft {
                            reply_id: reply_id.clone(),
                            draft_revision: draft.revision,
                            command_key: saved_command_key,
                        },
                        work_kind: WorkHitKind::SavedReplyDraft,
                        standing: HitStanding::Current,
                        dates: saved_dates,
                        authors: saved_authors,
                    },
                });
            }
        }

        if matches!(draft.status, DraftStatus::Cancelled)
            && request.standing.admits(&HitStanding::Withdrawn)
            && let Some(acc) = &draft.accepted
        {
            let mut cancelled_dates = Vec::new();
            if let Some(&accepted_at) =
                command_times.get(&(physician_id.clone(), acc.input.key.clone()))
            {
                cancelled_dates.push(SearchDate {
                    name: "accepted_at".into(),
                    value: format_unix_utc(accepted_at),
                });
            }
            let mut haystack = vec![
                acc.input.instruction.clone(),
                acc.input.key.clone(),
                draft.base_text.clone(),
                reply_id.clone(),
                physician_id.clone(),
            ];
            for d in &cancelled_dates {
                haystack.push(d.value.clone());
            }
            if matcher.matches(&haystack) {
                let sort_key = format!(
                    "1\x1f2\x1f{physician_id}\x1f{reply_id}\x1f{:016}\x1fcancelled:{}",
                    draft.revision, acc.input.key
                );
                out.push(RankedHit {
                    sort_key,
                    hit: PatientSearchHit::EarlierWork {
                        reference: WorkReference::ReplyDraft {
                            reply_id: reply_id.clone(),
                            draft_revision: draft.revision,
                            command_key: Some(acc.input.key.clone()),
                        },
                        work_kind: WorkHitKind::CancelledReplyRevision,
                        standing: HitStanding::Withdrawn,
                        dates: cancelled_dates,
                        authors: vec![SearchAuthor::Physician {
                            physician_id: physician_id.clone(),
                        }],
                    },
                });
            }
        }
    }

    for row in archived_rows {
        let ordinal = u64::try_from(row.ordinal)
            .map_err(|_| PatientSearchError::CorruptArchiveOrdinal(row.ordinal))?;
        if ordinal == 0 {
            return Err(PatientSearchError::CorruptArchiveOrdinal(row.ordinal));
        }
        let physician_id = row.physician.clone();
        let reply_id = row.reply.clone();
        let command_key = row.key.clone();
        let archived_at = row.archived_at;
        let scope = Scope {
            practice: patient.practice().to_owned(),
            physician: physician_id.clone(),
            workspace: patient.workspace().to_owned(),
            chat: reply_id.clone(),
        };
        let attempts = control::revision_attempts(tx, &scope, Some(&command_key)).await?;
        let mut earlier = control::earlier(row, attempts)?;

        let standing = HitStanding::Superseded {
            replaced_by: current_draft_revision_by_reply
                .get(&reply_id)
                .map(|rev| format!("draft_revision:{rev}")),
        };
        if !request.standing.admits(&standing) {
            continue;
        }

        let attempt_ids: Vec<String> = earlier.attempts.iter().map(|a| a.id.clone()).collect();
        let events_with_flags =
            records::ordered(tx, &scope, records::Owner::Revision, &attempt_ids).await?;
        let completed = earlier
            .runs
            .current()
            .is_some_and(|r| r.outcome == Some(harness::engine::runs::RunOutcome::Completed));
        let rev_messages = reply::messages(
            &earlier.accepted.input,
            completed,
            events_with_flags.iter().map(|(ev, _)| ev),
        );

        let mut dates = Vec::new();
        if let Some(&accepted_at) = command_times.get(&(physician_id.clone(), command_key.clone()))
        {
            dates.push(SearchDate {
                name: "accepted_at".into(),
                value: format_unix_utc(accepted_at),
            });
        }
        if archived_at > 0 {
            dates.push(SearchDate {
                name: "archived_at".into(),
                value: format_unix_utc(archived_at),
            });
        }

        let mut authors = vec![SearchAuthor::Physician {
            physician_id: physician_id.clone(),
        }];
        if completed {
            authors.push(SearchAuthor::CodaReplyRevision {
                reply_id: reply_id.clone(),
                command_key: command_key.clone(),
            });
        }

        let mut haystack = vec![
            std::mem::take(&mut earlier.base_text),
            earlier.accepted.input.instruction.clone(),
            command_key.clone(),
            reply_id.clone(),
            physician_id.clone(),
        ];
        for m in rev_messages {
            haystack.push(m.text);
        }
        for d in &dates {
            haystack.push(d.value.clone());
        }

        if !matcher.matches(&haystack) {
            continue;
        }
        let sort_key =
            format!("1\x1f3\x1f{physician_id}\x1f{reply_id}\x1f{ordinal:016}\x1f{command_key}");
        out.push(RankedHit {
            sort_key,
            hit: PatientSearchHit::EarlierWork {
                reference: WorkReference::ReplyRevisionHistory {
                    reply_id,
                    ordinal,
                    command_key,
                },
                work_kind: WorkHitKind::ArchivedReplyRevision,
                standing,
                dates,
                authors,
            },
        });
    }

    Ok(())
}

fn memory_record_standing(record: &MemorySearchRecord) -> HitStanding {
    match record.standing {
        VersionStanding::Prior { current_version } => match &record.content {
            SearchableMemoryContent::Correction {
                standing: CorrectionStanding::Withdrawn { .. },
                ..
            } => HitStanding::Withdrawn,
            SearchableMemoryContent::OperatingTopic { .. }
            | SearchableMemoryContent::Correction { .. }
            | SearchableMemoryContent::Summary { .. } => HitStanding::Superseded {
                replaced_by: current_version.map(|v| format!("v{}", v.get())),
            },
        },
        VersionStanding::Current => match &record.content {
            SearchableMemoryContent::OperatingTopic { .. }
            | SearchableMemoryContent::Summary { .. } => HitStanding::Current,
            SearchableMemoryContent::Correction { standing, .. } => match standing {
                CorrectionStanding::Active => HitStanding::Current,
                CorrectionStanding::Superseded { superseded_by } => HitStanding::Superseded {
                    replaced_by: Some(superseded_by.as_str().to_owned()),
                },
                CorrectionStanding::Withdrawn { .. } => HitStanding::Withdrawn,
            },
        },
    }
}

fn memory_record_dates(record: &MemorySearchRecord) -> Vec<SearchDate> {
    let mut dates = Vec::new();
    for d in &record.clinical_dates {
        let (name, value) = match d {
            MemorySearchDate::CorrectionRecordedOn(date) => {
                ("recorded_on", date.as_str().to_owned())
            }
            MemorySearchDate::CorrectionWithdrawnOn(date) => {
                ("withdrawn_on", date.as_str().to_owned())
            }
            MemorySearchDate::SummaryWrittenAt(date) => ("written_at", date.as_str().to_owned()),
            MemorySearchDate::SummaryReviewedAt(date) => ("reviewed_at", date.as_str().to_owned()),
            MemorySearchDate::SummaryPulledAt(date) => ("pulled_at", date.as_str().to_owned()),
        };
        dates.push(SearchDate {
            name: name.to_owned(),
            value,
        });
    }
    if record.saved_at > 0 {
        dates.push(SearchDate {
            name: "saved_at".into(),
            value: format_unix_utc(record.saved_at),
        });
    }
    dates
}

fn memory_record_authors(record: &MemorySearchRecord) -> Vec<SearchAuthor> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    let mut push = |author: SearchAuthor| {
        if seen.insert(author.clone()) {
            out.push(author);
        }
    };

    match &record.author {
        MemoryAuthor::PhysicianDirect { physician_id } => push(SearchAuthor::Physician {
            physician_id: physician_id.clone(),
        }),
        MemoryAuthor::ChatJob {
            physician_id,
            chat_id,
            work_id,
            call_id: _,
        } => {
            push(SearchAuthor::CodaChat {
                chat_id: chat_id.clone(),
                work_id: Some(work_id.clone()),
            });
            push(SearchAuthor::Physician {
                physician_id: physician_id.clone(),
            });
        }
        MemoryAuthor::PracticeJob { job_kind, job_id } => push(SearchAuthor::PracticeJob {
            job_kind: job_kind.clone(),
            job_id: job_id.clone(),
        }),
    }

    if let SearchableMemoryContent::Correction { source_message, .. } = &record.content {
        push(SearchAuthor::Physician {
            physician_id: source_message.physician_id.clone(),
        });
    }

    out
}

async fn collect_memory_hits_in_tx(
    tx: &impl ConnectionTrait,
    patient: &PatientKey,
    caller: &SearchCaller,
    request: &PatientSearchRequest,
    matcher: &QueryMatcher,
    hasher: &mut Sha256,
    out: &mut Vec<RankedHit>,
) -> Result<(), PatientSearchError> {
    let head = patient_memory::Entity::find_by_id((
        patient.practice().to_owned(),
        patient.workspace().to_owned(),
    ))
    .one(tx)
    .await?;

    let latest_version = patient_memory_version::Entity::find()
        .filter(patient_memory_version::Column::Practice.eq(patient.practice()))
        .filter(patient_memory_version::Column::Workspace.eq(patient.workspace()))
        .order_by_desc(patient_memory_version::Column::Version)
        .one(tx)
        .await?
        .map(|row| row.version)
        .unwrap_or(0);

    hasher.update(b"memory:");
    hasher.update(
        head.as_ref()
            .and_then(|h| h.current_version)
            .unwrap_or(0)
            .to_be_bytes(),
    );
    hasher.update(b":");
    hasher.update(latest_version.to_be_bytes());
    hasher.update(b"\n");

    if !request.domain.includes_memory()
        || request.record_type.is_some()
        || request.bucket.is_some()
        || head.is_none()
    {
        return Ok(());
    }

    let audience = caller.audience();
    let mut allowed_workspaces = BTreeSet::new();
    allowed_workspaces.insert(patient.workspace().to_owned());
    let mem_query = MemorySearchQuery {
        practice: patient.practice().to_owned(),
        allowed_workspaces,
        versions: request.standing.memory_version_scope(),
        audience,
    };

    let records = memory::read_search_records_in_tx(tx, &mem_query).await?;
    for record in records {
        if matches!(audience, SearchAudienceScope::PhysicianFacing)
            && matches!(record.access.audience, MemoryAudience::AgentOnly)
        {
            continue;
        }
        let standing = memory_record_standing(&record);
        if !request.standing.admits(&standing) {
            continue;
        }
        let dates = memory_record_dates(&record);
        let authors = memory_record_authors(&record);

        let mut haystack = vec![
            record.memory_id.as_str().to_owned(),
            format!("v{}", record.version.get()),
        ];
        for d in &dates {
            haystack.push(d.value.clone());
        }
        for author in &authors {
            match author {
                SearchAuthor::Physician { physician_id } => haystack.push(physician_id.clone()),
                SearchAuthor::CodaChat { chat_id, work_id } => {
                    haystack.push(chat_id.clone());
                    if let Some(w) = work_id {
                        haystack.push(w.clone());
                    }
                }
                SearchAuthor::PracticeJob { job_kind, job_id } => {
                    haystack.push(job_kind.clone());
                    haystack.push(job_id.clone());
                }
                SearchAuthor::ChartParticipant { actor, .. } => haystack.push(actor.clone()),
                SearchAuthor::SourceAdapter { source_id } => haystack.push(source_id.clone()),
                SearchAuthor::CodaReplyRevision {
                    reply_id,
                    command_key,
                } => {
                    haystack.push(reply_id.clone());
                    haystack.push(command_key.clone());
                }
            }
        }

        let (entry_sort_tag, entry_pointer) = match record.content {
            SearchableMemoryContent::OperatingTopic { topic, note } => {
                if matches!(audience, SearchAudienceScope::PhysicianFacing) {
                    continue;
                }
                haystack.push(topic.as_str().to_owned());
                haystack.push(note);
                let sort_tag = format!("2\x1f{}", topic.as_str());
                (sort_tag, MemoryEntryPointer::OperatingTopic { topic })
            }
            SearchableMemoryContent::Correction {
                correction_id,
                statement,
                recorded_on,
                source_message,
                standing: _,
            } => {
                haystack.push(correction_id.as_str().to_owned());
                haystack.push(statement);
                haystack.push(recorded_on.as_str().to_owned());
                haystack.push(source_message.chat_id.clone());
                haystack.push(source_message.message_id.clone());
                haystack.push(source_message.physician_id.clone());
                let sort_tag = format!("0\x1f{}", correction_id.as_str());
                (
                    sort_tag,
                    MemoryEntryPointer::Correction {
                        correction_id,
                        source_message,
                    },
                )
            }
            SearchableMemoryContent::Summary {
                summary_version,
                summary,
            } => {
                haystack.push(summary.content);
                for r in summary.references {
                    match r {
                        super::memory::SummaryReference::ChartRecord {
                            source_id,
                            candidate_id,
                            revision,
                            label,
                            record_date,
                        } => {
                            haystack.push(source_id);
                            haystack.push(candidate_id);
                            haystack.push(revision);
                            haystack.push(label);
                            if let Some(d) = record_date {
                                haystack.push(d.as_str().to_owned());
                            }
                        }
                        super::memory::SummaryReference::ConversationMessage {
                            chat_id,
                            message_id,
                            label,
                            stated_on,
                        } => {
                            haystack.push(chat_id);
                            haystack.push(message_id);
                            haystack.push(label);
                            if let Some(d) = stated_on {
                                haystack.push(d.as_str().to_owned());
                            }
                        }
                        super::memory::SummaryReference::Literature {
                            citation,
                            identifier,
                        } => {
                            haystack.push(citation);
                            if let Some(id) = identifier {
                                haystack.push(id);
                            }
                        }
                    }
                }
                (
                    "1\x1fsummary".to_owned(),
                    MemoryEntryPointer::PatientSummary { summary_version },
                )
            }
        };

        if !matcher.matches(&haystack) {
            continue;
        }
        let inv_version = u64::MAX - record.version.get();
        let sort_key = format!(
            "2\x1f{}\x1f{inv_version:020}\x1f{entry_sort_tag}",
            record.memory_id.as_str()
        );
        out.push(RankedHit {
            sort_key,
            hit: PatientSearchHit::PatientMemory {
                reference: MemoryVersionReference {
                    memory_id: record.memory_id,
                    version: record.version,
                    entry: entry_pointer,
                },
                standing,
                dates,
                authors,
            },
        });
    }

    Ok(())
}

async fn search_in_snapshot(
    tx: &impl ConnectionTrait,
    patient: &PatientKey,
    caller: &SearchCaller,
    request: &PatientSearchRequest,
) -> Result<PatientSearchPage, PatientSearchError> {
    let matcher = QueryMatcher::new(&request.query);
    let mut hasher = Sha256::new();
    hasher.update(b"patient_search_v1:");
    hasher.update(patient.practice().as_bytes());
    hasher.update(b"/");
    hasher.update(patient.workspace().as_bytes());
    hasher.update(b"\n");

    let mut ranked = Vec::new();
    collect_chart_hits_in_tx(
        tx,
        patient.workspace(),
        request,
        &matcher,
        &mut hasher,
        &mut ranked,
    )
    .await?;
    collect_work_hits_in_tx(
        tx,
        patient,
        caller,
        request,
        &matcher,
        &mut hasher,
        &mut ranked,
    )
    .await?;
    collect_memory_hits_in_tx(
        tx,
        patient,
        caller,
        request,
        &matcher,
        &mut hasher,
        &mut ranked,
    )
    .await?;

    let snapshot_token: String = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    if let Some(cursor) = &request.after
        && cursor.snapshot_token != snapshot_token
    {
        return Err(PatientSearchError::StaleCursor);
    }

    ranked.sort_by(|a, b| a.sort_key.cmp(&b.sort_key));

    let filtered: Vec<RankedHit> = match &request.after {
        Some(cursor) => ranked
            .into_iter()
            .filter(|item| item.sort_key > cursor.sort_key)
            .collect(),
        None => ranked,
    };

    let has_more = filtered.len() > SEARCH_PAGE_SIZE;
    let page_items: Vec<RankedHit> = filtered.into_iter().take(SEARCH_PAGE_SIZE).collect();
    let next = if has_more {
        page_items.last().map(|last| SearchCursor {
            snapshot_token,
            sort_key: last.sort_key.clone(),
        })
    } else {
        None
    };

    Ok(PatientSearchPage {
        results: page_items.into_iter().map(|r| r.hit).collect(),
        next,
    })
}

fn search_error_to_cause(err: PatientSearchError) -> engine::Cause {
    match err {
        PatientSearchError::Database(db) => super::persistence_failure(db),
        PatientSearchError::Validation(val) => Box::new(val),
        PatientSearchError::StaleCursor => Box::new(PatientSearchError::StaleCursor),
        PatientSearchError::Memory(mem) => Box::new(mem),
        PatientSearchError::CorruptSourceJson {
            candidate_id,
            source,
        } => Box::new(PatientSearchError::CorruptSourceJson {
            candidate_id,
            source,
        }),
        PatientSearchError::CorruptSourceIntegrity {
            candidate_id,
            source,
        } => Box::new(PatientSearchError::CorruptSourceIntegrity {
            candidate_id,
            source,
        }),
        PatientSearchError::PartResolution(part_err) => Box::new(part_err),
        PatientSearchError::CorruptSourceVersion(v) => {
            Box::new(PatientSearchError::CorruptSourceVersion(v))
        }
        PatientSearchError::CorruptArchiveOrdinal(o) => {
            Box::new(PatientSearchError::CorruptArchiveOrdinal(o))
        }
    }
}

impl WorkStatusExt for super::WorkStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Held => "held",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

trait WorkStatusExt {
    fn as_str(&self) -> &'static str;
}

impl Store {
    /// Searches everything Coda holds for one patient under a revalidated physician session.
    ///
    /// Revalidates `(practice, physician, workspace)` and `session_hash` through `authority`
    /// inside the same `RepeatableRead` snapshot transaction before reading any chart,
    /// conversation, draft, or memory rows.
    pub async fn search_patient_authorized(
        &self,
        scope: &Scope,
        session_hash: &[u8],
        audience: SearchAudienceScope,
        request: &PatientSearchRequest,
        authority: &mut dyn Authority,
    ) -> Result<PatientSearchOutcome, PatientSearchError> {
        if session_hash.len() != 32 {
            return Err(PatientSearchValidationError::InvalidSessionReference.into());
        }
        if scope.physician.trim().is_empty() {
            return Err(PatientSearchValidationError::EmptyField("physician").into());
        }
        if scope.chat.trim().is_empty() {
            return Err(PatientSearchValidationError::EmptyField("chat").into());
        }
        let patient = PatientKey::new(&scope.practice, &scope.workspace)
            .map_err(PatientSearchValidationError::from)?;
        validate_request(request)?;

        let caller = match audience {
            SearchAudienceScope::PhysicianFacing => SearchCaller::PhysicianFacing {
                physician_id: scope.physician.clone(),
            },
            SearchAudienceScope::AgentJob => SearchCaller::ChatJob {
                physician_id: scope.physician.clone(),
            },
        };

        let tx = self.snapshot().await?;
        if let Err(reason) = authority.check(&tx, scope, Some(session_hash)).await {
            tx.rollback().await?;
            return Ok(PatientSearchOutcome::Unauthorized { reason });
        }
        let page = search_in_snapshot(&tx, &patient, &caller, request).await?;
        tx.commit().await?;
        Ok(PatientSearchOutcome::Permitted(page))
    }

    /// Searches everything Coda holds for one patient gated by explicit [`PatientAccess`].
    ///
    /// Only [`PatientAccess::ClinicalPanel`] permits patient search; `ClinicDemographic`
    /// and `Denied` return [`PatientSearchOutcome::Unauthorized`] without reading rows.
    pub async fn search_patient_for_access(
        &self,
        patient: &PatientKey,
        caller: &SearchCaller,
        access: PatientAccess,
        request: &PatientSearchRequest,
    ) -> Result<PatientSearchOutcome, PatientSearchError> {
        caller.validate()?;
        validate_request(request)?;

        match access {
            PatientAccess::ClinicalPanel => {}
            PatientAccess::ClinicDemographic => {
                return Ok(PatientSearchOutcome::Unauthorized {
                    reason: "clinic demographic access does not permit patient search".into(),
                });
            }
            PatientAccess::Denied => {
                return Ok(PatientSearchOutcome::Unauthorized {
                    reason: "patient workspace access denied".into(),
                });
            }
        }

        let tx = self.snapshot().await?;
        let page = search_in_snapshot(&tx, patient, caller, request).await?;
        tx.commit().await?;
        Ok(PatientSearchOutcome::Permitted(page))
    }

    /// Executes [`SEARCH_TOOL`] inside a fenced chat attempt.
    ///
    /// `PhysicianHost` revalidates workspace and session authority before invoking
    /// tool execution; this method re-asserts claim currency and runs the search
    /// inside a single read-only snapshot transaction.
    pub async fn execute_chat_patient_search(
        &self,
        claim: &Claim,
        arguments: Value,
    ) -> Result<ToolOutcome, engine::Cause> {
        self.assert_current(claim).await?;
        let patient = PatientKey::new(&claim.scope.practice, &claim.scope.workspace)
            .map_err(PatientSearchValidationError::from)
            .map_err(PatientSearchError::from)
            .map_err(search_error_to_cause)?;
        let args: SearchToolArgs = match serde_json::from_value(arguments) {
            Ok(args) => args,
            Err(err) => {
                return Ok(ToolOutcome::refused(format!(
                    "Supply query and optional record_type, bucket, domain, standing, or after cursor ({err})."
                )));
            }
        };
        let request = match args.into_request() {
            Ok(req) => req,
            Err(err) => return Ok(ToolOutcome::refused(err.to_string())),
        };
        let caller = SearchCaller::ChatJob {
            physician_id: claim.scope.physician.clone(),
        };

        let tx = self.snapshot().await.map_err(super::persistence_failure)?;
        match search_in_snapshot(&tx, &patient, &caller, &request).await {
            Ok(page) => {
                tx.commit().await.map_err(super::persistence_failure)?;
                let encoded = serde_json::to_string(&page)
                    .map_err(|e| format!("failed to serialize search page: {e}"))?;
                Ok(ToolOutcome::Returned(encoded))
            }
            Err(PatientSearchError::StaleCursor) => {
                tx.rollback().await.map_err(super::persistence_failure)?;
                Ok(ToolOutcome::refused(
                    "Patient information changed since the previous page. Repeat the search from the beginning.",
                ))
            }
            Err(err) => {
                let _ = tx.rollback().await;
                Err(search_error_to_cause(err))
            }
        }
    }

    /// Executes [`SEARCH_TOOL`] for a background practice job gated by [`PatientAccess`].
    pub async fn execute_job_patient_search(
        &self,
        patient: &PatientKey,
        job_kind: &str,
        job_id: &str,
        access: PatientAccess,
        arguments: Value,
    ) -> Result<ToolOutcome, engine::Cause> {
        let caller = SearchCaller::PracticeJob {
            job_kind: job_kind.to_owned(),
            job_id: job_id.to_owned(),
        };
        caller
            .validate()
            .map_err(PatientSearchError::from)
            .map_err(search_error_to_cause)?;

        let args: SearchToolArgs = match serde_json::from_value(arguments) {
            Ok(args) => args,
            Err(err) => {
                return Ok(ToolOutcome::refused(format!(
                    "Supply query and optional record_type, bucket, domain, standing, or after cursor ({err})."
                )));
            }
        };
        let request = match args.into_request() {
            Ok(req) => req,
            Err(err) => return Ok(ToolOutcome::refused(err.to_string())),
        };

        match self
            .search_patient_for_access(patient, &caller, access, &request)
            .await
        {
            Ok(PatientSearchOutcome::Permitted(page)) => {
                let encoded = serde_json::to_string(&page)
                    .map_err(|e| format!("failed to serialize search page: {e}"))?;
                Ok(ToolOutcome::Returned(encoded))
            }
            Ok(PatientSearchOutcome::Unauthorized { reason }) => Ok(ToolOutcome::refused(reason)),
            Err(PatientSearchError::StaleCursor) => Ok(ToolOutcome::refused(
                "Patient information changed since the previous page. Repeat the search from the beginning.",
            )),
            Err(err) => Err(search_error_to_cause(err)),
        }
    }
}
