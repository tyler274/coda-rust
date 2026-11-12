//! Clinical library search and exact passage read tools for physician chat.
//!
//! Queries leaving patient chat carry clinical concepts only — never patient
//! names, birth dates, medical record numbers, workspace identifiers, or
//! contact details. `ClinicalLibraryClient` speaks the deployed clinical-evidence
//! HTTP service contract (`POST /v1/search` and `POST /v1/read`) and converts
//! both search hits and exact reads through one shared [`LibraryPassage`]
//! translator carrying source metadata, publication date, [`PublicationStatus`]
//! (including retraction and correction notices), attribution, and
//! integrity-checked text. When the clinical library service is unconfigured,
//! unreachable, or refuses a call, tools return a structured `unavailable`
//! outcome distinct from empty search results so chat can still answer from the
//! patient chart and state the literature limitation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::Duration;

use harness::engine::{Tool, ToolOutcome};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Tool name offered in physician chat to search the clinical library.
pub const SEARCH_LIBRARY_TOOL: &str = "search_clinical_library";

/// Tool name offered in physician chat to read an exact passage by identifier.
pub const READ_LIBRARY_TOOL: &str = "read_clinical_library";

/// Maximum UTF-8 byte length of a concept search query.
pub const MAX_LIBRARY_QUERY_BYTES: usize = 512;

/// Default maximum passages returned per search call.
pub const DEFAULT_SEARCH_LIMIT: usize = 8;

/// Upper bound on passages requested in one search call.
pub const MAX_SEARCH_LIMIT: usize = 20;

/// Upper bound on candidate identifiers accepted by one `POST /v1/read` call.
pub const MAX_WIRE_READ_IDS: usize = 32;

const MAX_SEARCH_PAGES: usize = 4;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

/// Whether `name` is one of the clinical library chat tools.
pub fn is_library_tool(name: &str) -> bool {
    matches!(name, SEARCH_LIBRARY_TOOL | READ_LIBRARY_TOOL)
}

/// Both read-only clinical library tools offered in ordinary physician chat.
pub fn tools() -> [Tool; 2] {
    [search_tool(), read_tool()]
}

/// Configuration failure when constructing an explicit [`ClinicalLibraryClient`].
#[derive(Debug, thiserror::Error)]
pub enum LibraryConfigError {
    /// Base URL was empty or not an `http://` or `https://` URL.
    #[error("clinical library base URL must start with http:// or https://")]
    InvalidBaseUrl,
    /// Underlying HTTP client failed to initialize.
    #[error("failed to build clinical library HTTP client: {0}")]
    HttpClient(#[source] reqwest::Error),
}

/// Correctable validation failure on clinical library tool arguments.
#[derive(Debug, thiserror::Error)]
pub enum LibraryInputError {
    /// Search arguments did not match the offered schema.
    #[error("Supply query, optional collection, and optional limit ({0}).")]
    MalformedSearch(#[source] serde_json::Error),
    /// Exact read arguments did not match the offered schema.
    #[error("Supply passage_id from a clinical library search result ({0}).")]
    MalformedRead(#[source] serde_json::Error),
    /// Search query was empty, contained control characters, or exceeded [`MAX_LIBRARY_QUERY_BYTES`].
    #[error(
        "Supply a non-empty single-line clinical concept query of at most {MAX_LIBRARY_QUERY_BYTES} bytes."
    )]
    InvalidQueryShape,
    /// Search query contained a patient or workspace identifier pattern.
    #[error(
        "Clinical library queries must contain clinical concepts only; remove patient names, birth dates, medical record numbers, workspace identifiers, and contact details."
    )]
    PatientIdentifierInQuery,
    /// Requested `limit` was outside `1..=20`.
    #[error("Supply limit between 1 and {MAX_SEARCH_LIMIT}.")]
    InvalidLimit,
    /// Passage identifier was empty, oversized, or contained control characters.
    #[error("Supply a valid non-empty passage_id returned by search_clinical_library.")]
    InvalidPassageId,
    /// Tool name was not a clinical library tool.
    #[error("Unknown clinical library tool `{0}`.")]
    UnknownTool(String),
}

/// Validated opaque identifier of one clinical library passage (`candidate_id` / `unit_id`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PassageId(String);

impl PassageId {
    /// Parses a trimmed, non-empty passage identifier of 1 to 256 characters.
    pub fn new(value: impl Into<String>) -> Result<Self, LibraryInputError> {
        let value = value.into();
        let count = value.chars().count();
        if !(1..=256).contains(&count)
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(LibraryInputError::InvalidPassageId);
        }
        Ok(Self(value))
    }

    /// Identifier string as transmitted on the wire.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PassageId {
    type Error = LibraryInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PassageId> for String {
    fn from(id: PassageId) -> Self {
        id.0
    }
}

impl fmt::Display for PassageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validated clinical-concept-only search query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConceptQuery(String);

impl ConceptQuery {
    /// Validates query length, single-line format, and absence of patient
    /// identifier shapes before any request leaves the chat host.
    pub fn parse(raw: &str) -> Result<Self, LibraryInputError> {
        if raw.chars().any(char::is_control) {
            return Err(LibraryInputError::InvalidQueryShape);
        }
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.len() > MAX_LIBRARY_QUERY_BYTES {
            return Err(LibraryInputError::InvalidQueryShape);
        }
        if contains_patient_identifier_pattern(trimmed) {
            return Err(LibraryInputError::PatientIdentifierInQuery);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Validated concept query string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn contains_patient_identifier_pattern(query: &str) -> bool {
    let lower = query.to_ascii_lowercase();

    // Workspace identifiers (`pw-<hex>`).
    for segment in lower.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-') {
        if let Some(hex) = segment.strip_prefix("pw-")
            && hex.len() >= 6
            && hex.chars().all(|ch| ch.is_ascii_hexdigit())
        {
            return true;
        }
    }

    // Email addresses (`local@domain.tld`).
    for token in lower.split_whitespace() {
        let cleaned =
            token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '@' && ch != '.');
        if let Some((local, domain)) = cleaned.split_once('@')
            && !local.is_empty()
            && domain.contains('.')
            && !domain.starts_with('.')
            && !domain.ends_with('.')
        {
            return true;
        }
    }

    // Whole-word identifier labels (`mrn`, `ssn`, `dob`) so clinical words
    // such as `mrna` and `dobutamine` are never rejected.
    for word in lower.split(|ch: char| !ch.is_ascii_alphanumeric()) {
        if matches!(word, "mrn" | "ssn" | "dob") {
            return true;
        }
    }
    for phrase in [
        "medical record number",
        "date of birth",
        "patient name",
        "patient:",
    ] {
        if lower.contains(phrase) {
            return true;
        }
    }

    // SSN shape (`DDD-DD-DDDD`) and slash calendar birth-date shape (`MM/DD/YYYY`).
    for token in lower.split_whitespace() {
        let bare =
            token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-' && ch != '/');
        if is_ssn_shape(bare) || is_slash_date_shape(bare) {
            return true;
        }
    }

    false
}

fn is_ssn_shape(token: &str) -> bool {
    let mut parts = token.split('-');
    let (Some(a), Some(b), Some(c), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    a.len() == 3
        && b.len() == 2
        && c.len() == 4
        && [a, b, c]
            .into_iter()
            .all(|part| part.chars().all(|ch| ch.is_ascii_digit()))
}

fn is_slash_date_shape(token: &str) -> bool {
    let mut parts = token.split('/');
    let (Some(m), Some(d), Some(y), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    (1..=2).contains(&m.len())
        && (1..=2).contains(&d.len())
        && y.len() == 4
        && [m, d, y]
            .into_iter()
            .all(|part| part.chars().all(|ch| ch.is_ascii_digit()))
}

/// Source collection in the clinical library.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibraryCollection {
    /// PubMed citation and status records.
    Pubmed,
    /// PubMed Central full-text articles.
    Pmc,
    /// DailyMed structured product drug labels.
    Dailymed,
    /// CDC Morbidity and Mortality Weekly Report.
    Mmwr,
    /// US Preventive Services Task Force recommendations.
    Uspstf,
}

impl LibraryCollection {
    /// Maps a clinical-evidence service `source_code` into its typed collection.
    pub fn from_source_code(source_code: &str) -> Result<Self, &'static str> {
        match source_code.trim() {
            "pubmed" | "nlm-pubmed" => Ok(Self::Pubmed),
            "pmc" => Ok(Self::Pmc),
            "dailymed" => Ok(Self::Dailymed),
            "mmwr" | "cdc-mmwr" => Ok(Self::Mmwr),
            "uspstf" => Ok(Self::Uspstf),
            _ => Err("unknown clinical library source_code"),
        }
    }
}

/// Standing of a clinical library item, carrying explicit notice text whenever
/// non-current.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum PublicationStatus {
    /// Active item with no retraction, correction, or superseding notice.
    Current,
    /// Published erratum or correction applies to this item.
    Corrected {
        /// Erratum or correction notice from the source.
        notice: String,
    },
    /// Published expression of concern applies to this item.
    ExpressionOfConcern {
        /// Expression-of-concern notice from the source.
        notice: String,
    },
    /// Retracted publication; surfaced explicitly so it is never cited as active support.
    Retracted {
        /// Retraction notice from the source.
        notice: String,
    },
    /// Superseded version; replaced by a newer document version.
    Superseded {
        /// Superseding notice from the source.
        notice: String,
    },
    /// Withdrawn item; surfaced explicitly with its withdrawal notice.
    Withdrawn {
        /// Withdrawal notice from the source.
        notice: String,
    },
    /// Source lifecycle standing has not been confirmed.
    Unknown {
        /// Explanation of unconfirmed lifecycle standing.
        notice: String,
    },
}

impl PublicationStatus {
    fn is_valid(&self) -> bool {
        match self {
            Self::Current => true,
            Self::Corrected { notice }
            | Self::ExpressionOfConcern { notice }
            | Self::Retracted { notice }
            | Self::Superseded { notice }
            | Self::Withdrawn { notice }
            | Self::Unknown { notice } => !notice.trim().is_empty(),
        }
    }
}

/// Source document metadata attached to every [`LibraryPassage`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibrarySource {
    /// Collection holding the document.
    pub collection: LibraryCollection,
    /// Source-owned document identifier (such as PMCID, PMID, or DailyMed Set ID).
    pub document_id: String,
    /// Document title.
    pub title: String,
    /// Section heading or unit locator within the document, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}

/// One passage returned by clinical library search or exact passage read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryPassage {
    /// Stable passage identifier accepted by [`READ_LIBRARY_TOOL`].
    pub passage_id: PassageId,
    /// Originating source document metadata.
    pub source: LibrarySource,
    /// Unambiguous publication, version, or acquisition date.
    pub date: String,
    /// Publication standing, including retraction or correction notices.
    pub status: PublicationStatus,
    /// Author, journal or issuing organization, and license attribution.
    pub attribution: String,
    /// Exact UTF-8 passage text.
    pub text: String,
    /// Lowercase hexadecimal SHA-256 of `text`, verified on receipt.
    pub content_sha256: String,
}

impl LibraryPassage {
    /// Checks required provenance fields and verifies `content_sha256` against `text`.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.source.document_id.trim().is_empty()
            || self.source.title.trim().is_empty()
            || self.date.trim().is_empty()
            || !self.status.is_valid()
            || self.attribution.trim().is_empty()
            || self.text.trim().is_empty()
            || self
                .source
                .section
                .as_deref()
                .is_some_and(|section| section.trim().is_empty())
        {
            return Err("clinical library passage is missing required metadata");
        }
        if crate::sha256_hex(self.text.as_bytes()) != self.content_sha256 {
            return Err("clinical library passage digest mismatch");
        }
        Ok(())
    }
}

/// Model-visible result of [`SEARCH_LIBRARY_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum LibrarySearchOutcome {
    /// Search completed; an empty `passages` list means nothing matched the query.
    Results {
        /// Matching passages with source, date, status, and attribution.
        passages: Vec<LibraryPassage>,
    },
    /// Clinical library service was unconfigured, unreachable, or refused the call.
    Unavailable {
        /// Explanation distinct from empty search results.
        reason: String,
    },
}

/// Model-visible result of [`READ_LIBRARY_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum LibraryReadOutcome {
    /// Exact passage retrieved and verified.
    Passage {
        /// Exact passage matching `passage_id`.
        passage: Box<LibraryPassage>,
    },
    /// Library responded and holds no active passage with `passage_id`.
    NotFound {
        /// Requested passage identifier.
        passage_id: PassageId,
    },
    /// Clinical library service was unconfigured, unreachable, or refused the call.
    Unavailable {
        /// Explanation of why the library could not be read.
        reason: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSearchArgs {
    query: String,
    #[serde(default)]
    collection: Option<LibraryCollection>,
    #[serde(default)]
    limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReadArgs {
    passage_id: String,
}

/// Validated clinical library tool request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClinicalLibraryRequest {
    /// Validated concept search request.
    Search {
        /// Concept-only query.
        query: ConceptQuery,
        /// Optional source collection filter.
        collection: Option<LibraryCollection>,
        /// Maximum passages to return (`1..=20`).
        limit: usize,
    },
    /// Validated exact passage read request.
    Read {
        /// Target passage identifier.
        passage_id: PassageId,
    },
}

impl ClinicalLibraryRequest {
    /// Validates tool name and arguments before touching the network.
    pub fn parse(name: &str, arguments: &Value) -> Result<Self, LibraryInputError> {
        match name {
            SEARCH_LIBRARY_TOOL => {
                let raw: RawSearchArgs = serde_json::from_value(arguments.clone())
                    .map_err(LibraryInputError::MalformedSearch)?;
                let query = ConceptQuery::parse(&raw.query)?;
                let limit = raw.limit.unwrap_or(DEFAULT_SEARCH_LIMIT);
                if !(1..=MAX_SEARCH_LIMIT).contains(&limit) {
                    return Err(LibraryInputError::InvalidLimit);
                }
                Ok(Self::Search {
                    query,
                    collection: raw.collection,
                    limit,
                })
            }
            READ_LIBRARY_TOOL => {
                let raw: RawReadArgs = serde_json::from_value(arguments.clone())
                    .map_err(LibraryInputError::MalformedRead)?;
                let passage_id = PassageId::new(raw.passage_id)?;
                Ok(Self::Read { passage_id })
            }
            other => Err(LibraryInputError::UnknownTool(other.to_owned())),
        }
    }
}

/// Discovery mode accepted by `POST /v1/search` on the clinical-evidence service.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchWireMode {
    /// Free-text clinical concept query discovery.
    Query,
    /// Identifier discovery.
    Identifier,
    /// Sibling unit discovery.
    Siblings,
    /// Relationship traversal discovery.
    Relationship,
}

/// Strict 7-key request body sent to `POST /v1/search` on the clinical-evidence service.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchWireRequest {
    /// Discovery mode (`"query"` for clinical concept search).
    pub mode: SearchWireMode,
    /// Validated clinical concept query when `mode` is [`SearchWireMode::Query`].
    pub query: Option<String>,
    /// Identifier scheme for identifier mode, else `None` (`null`).
    pub scheme: Option<String>,
    /// Identifier value for identifier mode, else `None` (`null`).
    pub value: Option<String>,
    /// Anchor candidate UUID for sibling/relationship modes, else `None` (`null`).
    pub candidate_id: Option<String>,
    /// Relationship selector for relationship mode, else `None` (`null`).
    pub relationship: Option<Value>,
    /// Continuation cursor from a prior search response, or `None` (`null`).
    pub cursor: Option<String>,
}

impl SearchWireRequest {
    /// Constructs the exact 7-key `"query"` request body required by `POST /v1/search`.
    pub fn query_page(query: &ConceptQuery, cursor: Option<String>) -> Self {
        Self {
            mode: SearchWireMode::Query,
            query: Some(query.as_str().to_owned()),
            scheme: None,
            value: None,
            candidate_id: None,
            relationship: None,
            cursor,
        }
    }
}

/// Page summary returned from `POST /v1/search`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireSearchPage {
    /// Number of index candidates examined.
    pub examined_candidate_count: usize,
    /// Number of candidates returned in `candidates`.
    pub returned_candidate_count: usize,
    /// Number of examined candidates omitted as unavailable.
    pub unavailable_candidate_count: usize,
    /// Whether additional candidates exist beyond this page.
    pub truncated: bool,
}

/// Candidate metadata item returned from `POST /v1/search`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireCandidate {
    /// Candidate unit identifier (`unit_id` UUID) accepted by `POST /v1/read`.
    pub candidate_id: String,
    /// Source collection code (`"dailymed"`, `"pmc"`, `"cdc-mmwr"`, `"uspstf"`, `"pubmed"`).
    pub source_code: String,
    /// Human-readable source name.
    pub source_display_name: String,
    /// Source-owned document identifier.
    pub native_document_id: String,
    /// Document title.
    pub title: String,
}

/// Response body returned from `POST /v1/search`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchWireResponse {
    /// Ranked candidate units matching the search request.
    pub candidates: Vec<WireCandidate>,
    /// Page accounting metadata.
    pub page: WireSearchPage,
    /// Continuation cursor when `page.truncated` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Strict request body sent to `POST /v1/read` on the clinical-evidence service.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadWireRequest {
    /// Candidate unit identifiers (`1..=32`) to read.
    pub candidate_ids: Vec<String>,
}

/// Lifecycle standing reported on a clinical-evidence unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WireLifecycleState {
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

/// Typed relationship kind reported on a clinical-evidence unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WireRelationshipType {
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

/// Document identifier attached to a clinical-evidence unit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireIdentifier {
    /// Identifier scheme (such as `"pmid"`, `"pmcid"`, `"doi"`, `"spl-set-id"`).
    pub scheme: String,
    /// Raw identifier value as published.
    pub raw_value: String,
    /// Normalized identifier value when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_value: Option<String>,
}

/// Typed relationship attached to a clinical-evidence unit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireRelationship {
    /// Semantic relationship type.
    pub relation_type: WireRelationshipType,
    /// Internal target version UUID when resolved inside the corpus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_version_id: Option<String>,
    /// External target scheme (such as `"pmid"` or `"doi"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_scheme: Option<String>,
    /// External target identifier value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<String>,
    /// Source-specific relation code (such as `"RetractedIn"` or `"ErratumIn"`).
    pub source_relation_code: String,
    /// Source URI for the related item when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_destination_uri: Option<String>,
}

/// NLM PubMed relationship inside [`WireBibliographicObservation`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireBibliographicRelationship {
    /// NLM PubMed relation code (such as `"RetractedIn"`, `"ErratumIn"`, `"ExpressionOfConcernIn"`).
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

/// Independent NLM PubMed bibliographic status assertion attached to a unit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireBibliographicObservation {
    /// Subject PubMed identifier.
    pub subject_pmid: String,
    /// Source record revision date (`YYYY-MM-DD`) when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_record_date: Option<String>,
    /// NLM publication types (such as `"Retracted Publication"`).
    #[serde(default)]
    pub publication_types: Vec<String>,
    /// NLM comments/corrections relationships.
    #[serde(default)]
    pub relationships: Vec<WireBibliographicRelationship>,
}

/// Exact unit payload returned inside an `available` `POST /v1/read` item.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireEvidenceUnit {
    /// Independent bibliographic status observations when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bibliographic_observations: Option<Vec<WireBibliographicObservation>>,
    /// Unit UUID matching `candidate_id`.
    pub unit_id: String,
    /// Admitted document version UUID.
    pub version_id: String,
    /// Source collection code.
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
    /// Publication calendar date (`YYYY-MM-DD`) when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    /// Version calendar date (`YYYY-MM-DD`) when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_date: Option<String>,
    /// ISO-8601 acquisition timestamp.
    pub acquired_at: String,
    /// Lifecycle standing of the document version.
    pub lifecycle_state: WireLifecycleState,
    /// Source status code or notice label when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_status_code: Option<String>,
    /// Source-owned unit identifier within the document.
    pub native_unit_id: String,
    /// Source locator within the document.
    pub native_locator: String,
    /// Structural kind of the unit.
    pub unit_kind: String,
    /// Zero-based position of the unit in the document.
    pub ordinal: usize,
    /// Section heading or unit label when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Exact UTF-8 passage content.
    pub content: String,
    /// Lowercase hexadecimal SHA-256 of `content`.
    pub content_sha256: String,
    /// Canonical source URI for this unit.
    pub native_destination_uri: String,
    /// Canonical source URI for the document version.
    pub version_destination_uri: String,
    /// Rights decision UUID governing display.
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
    pub identifiers: Vec<WireIdentifier>,
    /// Lifecycle and version relationships attached to the document.
    #[serde(default)]
    pub relationships: Vec<WireRelationship>,
}

/// Per-candidate unavailability reason inside a `POST /v1/read` response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WireReadUnavailableCause {
    /// Candidate identifier was not a valid UUID.
    InvalidCandidateId,
    /// Candidate unit does not exist or is not authorized for display.
    NotFoundOrNotAuthorized,
    /// Visual asset payload could not be fetched.
    VisualUnavailable,
    /// Visual read budget was exceeded.
    VisualReadLimit,
}

/// Discriminated per-candidate item in `POST /v1/read`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum WireReadItem {
    /// Candidate unit was found and returned.
    Available {
        /// Requested candidate identifier.
        candidate_id: String,
        /// Hydrated clinical-evidence unit.
        unit: Box<WireEvidenceUnit>,
    },
    /// Candidate unit could not be returned.
    Unavailable {
        /// Requested candidate identifier.
        candidate_id: String,
        /// Specific unavailability cause.
        cause: WireReadUnavailableCause,
    },
}

/// Response body returned from `POST /v1/read`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadWireResponse {
    /// Per-candidate read outcomes.
    pub results: Vec<WireReadItem>,
}

fn format_wire_relationship(rel: &WireRelationship) -> String {
    match (
        rel.target_scheme.as_deref().map(str::trim),
        rel.target_value.as_deref().map(str::trim),
    ) {
        (Some(scheme), Some(value)) if !scheme.is_empty() && !value.is_empty() => {
            format!("{} ({scheme}:{value})", rel.source_relation_code.trim())
        }
        _ => match rel.target_version_id.as_deref().map(str::trim) {
            Some(version_id) if !version_id.is_empty() => {
                format!("{} (version {version_id})", rel.source_relation_code.trim())
            }
            _ => rel.source_relation_code.trim().to_owned(),
        },
    }
}

fn format_bibliographic_relationship(rel: &WireBibliographicRelationship) -> String {
    let mut parts = vec![rel.source_relation_code.trim().to_owned()];
    if let Some(pmid) = rel
        .target_pmid
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        parts.push(format!("PMID {pmid}"));
    }
    if let Some(reference) = rel
        .source_reference
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        parts.push(reference.to_owned());
    }
    if let Some(note) = rel.note.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        parts.push(note.to_owned());
    }
    parts.join(": ")
}

fn collect_status_details(unit: &WireEvidenceUnit) -> Vec<String> {
    let mut details = Vec::new();
    if let Some(code) = unit
        .source_status_code
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        details.push(code.to_owned());
    }
    for rel in &unit.relationships {
        if matches!(
            rel.relation_type,
            WireRelationshipType::IsRetractedBy
                | WireRelationshipType::Retracts
                | WireRelationshipType::HasExpressionOfConcern
                | WireRelationshipType::ExpressesConcernAbout
                | WireRelationshipType::IsCorrectedBy
                | WireRelationshipType::Corrects
                | WireRelationshipType::IsSupersededBy
                | WireRelationshipType::Supersedes
                | WireRelationshipType::IsUpdatedBy
        ) {
            details.push(format_wire_relationship(rel));
        }
    }
    if let Some(observations) = &unit.bibliographic_observations {
        for obs in observations {
            for pub_type in &obs.publication_types {
                let lower = pub_type.to_ascii_lowercase();
                if lower.contains("retract")
                    || lower.contains("erratum")
                    || lower.contains("concern")
                    || lower.contains("corrected")
                    || lower.contains("withdrawn")
                {
                    details.push(format!(
                        "PubMed PMID {}: {}",
                        obs.subject_pmid,
                        pub_type.trim()
                    ));
                }
            }
            for rel in &obs.relationships {
                details.push(format_bibliographic_relationship(rel));
            }
        }
    }
    details
}

fn has_retraction_signal(unit: &WireEvidenceUnit) -> bool {
    if unit
        .relationships
        .iter()
        .any(|rel| rel.relation_type == WireRelationshipType::IsRetractedBy)
    {
        return true;
    }
    unit.bibliographic_observations
        .as_deref()
        .unwrap_or_default()
        .iter()
        .any(|obs| {
            obs.publication_types
                .iter()
                .any(|pt| pt.to_ascii_lowercase().contains("retracted"))
                || obs.relationships.iter().any(|rel| {
                    rel.source_relation_code
                        .to_ascii_lowercase()
                        .contains("retract")
                })
        })
}

fn has_expression_of_concern_signal(unit: &WireEvidenceUnit) -> bool {
    if unit
        .relationships
        .iter()
        .any(|rel| rel.relation_type == WireRelationshipType::HasExpressionOfConcern)
    {
        return true;
    }
    unit.bibliographic_observations
        .as_deref()
        .unwrap_or_default()
        .iter()
        .any(|obs| {
            obs.publication_types
                .iter()
                .any(|pt| pt.to_ascii_lowercase().contains("expression of concern"))
                || obs.relationships.iter().any(|rel| {
                    rel.source_relation_code
                        .to_ascii_lowercase()
                        .contains("concern")
                })
        })
}

fn has_correction_signal(unit: &WireEvidenceUnit) -> bool {
    if unit
        .relationships
        .iter()
        .any(|rel| rel.relation_type == WireRelationshipType::IsCorrectedBy)
    {
        return true;
    }
    unit.bibliographic_observations
        .as_deref()
        .unwrap_or_default()
        .iter()
        .any(|obs| {
            obs.publication_types.iter().any(|pt| {
                let lower = pt.to_ascii_lowercase();
                lower.contains("erratum") || lower.contains("corrected")
            }) || obs.relationships.iter().any(|rel| {
                let lower = rel.source_relation_code.to_ascii_lowercase();
                lower.contains("erratum") || lower.contains("corrected")
            })
        })
}

fn derive_publication_status(unit: &WireEvidenceUnit) -> PublicationStatus {
    let details = collect_status_details(unit);
    let notice_or = |fallback: &str| {
        if details.is_empty() {
            fallback.to_owned()
        } else {
            details.join("; ")
        }
    };

    if unit.lifecycle_state == WireLifecycleState::Retracted || has_retraction_signal(unit) {
        return PublicationStatus::Retracted {
            notice: notice_or("Retracted in source clinical library."),
        };
    }
    if has_expression_of_concern_signal(unit) {
        return PublicationStatus::ExpressionOfConcern {
            notice: notice_or("Expression of concern recorded in source clinical library."),
        };
    }
    if unit.lifecycle_state == WireLifecycleState::Corrected || has_correction_signal(unit) {
        return PublicationStatus::Corrected {
            notice: notice_or("Correction or erratum recorded in source clinical library."),
        };
    }

    match unit.lifecycle_state {
        WireLifecycleState::Current => PublicationStatus::Current,
        WireLifecycleState::Superseded => PublicationStatus::Superseded {
            notice: notice_or("Superseded by a newer version in the clinical library."),
        },
        WireLifecycleState::Withdrawn => PublicationStatus::Withdrawn {
            notice: notice_or("Withdrawn in source clinical library."),
        },
        WireLifecycleState::Unknown => PublicationStatus::Unknown {
            notice: notice_or("Publication lifecycle standing has not been confirmed."),
        },
        WireLifecycleState::Corrected | WireLifecycleState::Retracted => {
            unreachable!("corrected and retracted states are handled above")
        }
    }
}

fn acquired_date_prefix(acquired_at: &str) -> Option<String> {
    let trimmed = acquired_at.trim();
    if trimmed.is_empty() {
        return None;
    }
    let date_part = trimmed.split('T').next().unwrap_or(trimmed).trim();
    (!date_part.is_empty()).then(|| date_part.to_owned())
}

/// Single shared translator converting one clinical-evidence [`WireEvidenceUnit`]
/// into a validated [`LibraryPassage`] for both `search_clinical_library` and
/// `read_clinical_library`.
pub fn passage_from_wire_unit(
    candidate_id: &str,
    unit: WireEvidenceUnit,
) -> Result<LibraryPassage, &'static str> {
    if unit.unit_id != candidate_id {
        return Err("clinical library unit_id does not match requested candidate_id");
    }
    if !unit.may_display_excerpt {
        return Err("clinical library unit rights do not permit excerpt display");
    }
    let passage_id =
        PassageId::new(candidate_id).map_err(|_| "clinical library invalid candidate_id")?;
    let collection = LibraryCollection::from_source_code(&unit.source_code)?;
    let section = unit
        .label
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .or_else(|| {
            let locator = unit.native_locator.trim();
            (!locator.is_empty()).then(|| locator.to_owned())
        });
    let date = unit
        .publication_date
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .or_else(|| {
            unit.version_date
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
        })
        .or_else(|| acquired_date_prefix(&unit.acquired_at))
        .ok_or("clinical library unit is missing publication and acquisition dates")?;
    let attribution = unit
        .attribution_text
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .or_else(|| {
            let display = unit.source_display_name.trim();
            if display.is_empty() {
                return None;
            }
            match unit
                .license_identifier
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                Some(license) => Some(format!("{display} ({license})")),
                None => Some(display.to_owned()),
            }
        })
        .ok_or("clinical library unit is missing attribution")?;
    let status = derive_publication_status(&unit);

    let passage = LibraryPassage {
        passage_id,
        source: LibrarySource {
            collection,
            document_id: unit.native_document_id,
            title: unit.title,
            section,
        },
        date,
        status,
        attribution,
        text: unit.content,
        content_sha256: unit.content_sha256,
    };
    passage.validate()?;
    Ok(passage)
}

#[derive(Clone)]
enum Endpoint {
    Unconfigured {
        reason: String,
    },
    Http {
        base_url: String,
        bearer_token: Option<String>,
        client: reqwest::Client,
    },
}

/// Client for searching and reading the clinical library (`clinical-evidence`) service.
#[derive(Clone)]
pub struct ClinicalLibraryClient {
    endpoint: Endpoint,
}

impl ClinicalLibraryClient {
    /// Client that reports the clinical library as unconfigured without making
    /// network requests.
    pub fn unconfigured() -> Self {
        Self {
            endpoint: Endpoint::Unconfigured {
                reason: "clinical_library_unconfigured".into(),
            },
        }
    }

    /// Client targeting an HTTP or HTTPS clinical library service base URL.
    pub fn http(base_url: impl Into<String>) -> Result<Self, LibraryConfigError> {
        Self::http_with_timeout(base_url, DEFAULT_TIMEOUT)
    }

    /// Client targeting `base_url` with an explicit per-request `timeout`.
    pub fn http_with_timeout(
        base_url: impl Into<String>,
        timeout: Duration,
    ) -> Result<Self, LibraryConfigError> {
        let trimmed = base_url.into().trim().trim_end_matches('/').to_owned();
        if !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
            return Err(LibraryConfigError::InvalidBaseUrl);
        }
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(LibraryConfigError::HttpClient)?;
        Ok(Self {
            endpoint: Endpoint::Http {
                base_url: trimmed,
                bearer_token: None,
                client,
            },
        })
    }

    /// Attaches an `Authorization: Bearer` token for authenticated service endpoints.
    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        if let Endpoint::Http { bearer_token, .. } = &mut self.endpoint {
            let trimmed = token.into().trim().to_owned();
            *bearer_token = (!trimmed.is_empty()).then_some(trimmed);
        }
        self
    }

    /// Reads `CLINICAL_LIBRARY_URL` or `CLINICAL_EVIDENCE_URL` (and optional bearer
    /// token) from the process environment, defaulting cleanly to unconfigured when unset.
    pub fn from_env() -> Self {
        Self::from_reader(|key| std::env::var(key).ok())
    }

    /// Reads clinical library settings from an explicit environment reader.
    pub fn from_reader(read: impl Fn(&str) -> Option<String>) -> Self {
        let raw_url = read("CLINICAL_LIBRARY_URL")
            .or_else(|| read("CLINICAL_EVIDENCE_URL"))
            .or_else(|| read("CODA_CLINICAL_LIBRARY_URL"))
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let Some(url) = raw_url else {
            return Self::unconfigured();
        };
        match Self::http(url) {
            Ok(client) => match read("CLINICAL_LIBRARY_BEARER_TOKEN")
                .or_else(|| read("CLINICAL_EVIDENCE_BEARER_TOKEN"))
            {
                Some(token) => client.with_bearer_token(token),
                None => client,
            },
            Err(reason) => Self {
                endpoint: Endpoint::Unconfigured {
                    reason: format!("clinical_library_invalid_configuration: {reason}"),
                },
            },
        }
    }

    async fn post_search_page(
        &self,
        query: &ConceptQuery,
        cursor: Option<String>,
    ) -> Result<SearchWireResponse, String> {
        let Endpoint::Http {
            base_url,
            bearer_token,
            client,
        } = &self.endpoint
        else {
            let Endpoint::Unconfigured { reason } = &self.endpoint else {
                unreachable!()
            };
            return Err(reason.clone());
        };

        let url = format!("{base_url}/v1/search");
        let mut request = client
            .post(url)
            .json(&SearchWireRequest::query_page(query, cursor));
        if let Some(token) = bearer_token {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .await
            .map_err(|cause| classify_transport_error(&cause))?;
        if response.status() != StatusCode::OK {
            return Err(classify_status(response.status()));
        }
        response
            .json()
            .await
            .map_err(|_| "clinical_library_malformed_response".to_owned())
    }

    async fn post_read_units(&self, candidate_ids: &[String]) -> Result<ReadWireResponse, String> {
        let Endpoint::Http {
            base_url,
            bearer_token,
            client,
        } = &self.endpoint
        else {
            let Endpoint::Unconfigured { reason } = &self.endpoint else {
                unreachable!()
            };
            return Err(reason.clone());
        };

        let url = format!("{base_url}/v1/read");
        let mut request = client.post(url).json(&ReadWireRequest {
            candidate_ids: candidate_ids.to_vec(),
        });
        if let Some(token) = bearer_token {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .await
            .map_err(|cause| classify_transport_error(&cause))?;
        if response.status() != StatusCode::OK {
            return Err(classify_status(response.status()));
        }
        response
            .json()
            .await
            .map_err(|_| "clinical_library_malformed_response".to_owned())
    }

    /// Runs a validated concept search against `POST /v1/search`, filters matching
    /// candidates by optional `collection` and `limit`, and hydrates passages via
    /// `POST /v1/read` through [`passage_from_wire_unit`].
    pub async fn search(
        &self,
        query: &ConceptQuery,
        collection: Option<LibraryCollection>,
        limit: usize,
    ) -> LibrarySearchOutcome {
        if let Endpoint::Unconfigured { reason } = &self.endpoint {
            return LibrarySearchOutcome::Unavailable {
                reason: reason.clone(),
            };
        }

        let target_limit = limit.clamp(1, MAX_SEARCH_LIMIT);
        let mut seen_ids = BTreeSet::new();
        let mut selected_ids = Vec::with_capacity(target_limit);
        let mut cursor = None;

        for _ in 0..MAX_SEARCH_PAGES {
            let page = match self.post_search_page(query, cursor.take()).await {
                Ok(page) => page,
                Err(reason) => return LibrarySearchOutcome::Unavailable { reason },
            };
            for candidate in page.candidates {
                let Ok(candidate_collection) =
                    LibraryCollection::from_source_code(&candidate.source_code)
                else {
                    continue;
                };
                if collection.is_some_and(|expected| expected != candidate_collection) {
                    continue;
                }
                if seen_ids.insert(candidate.candidate_id.clone()) {
                    selected_ids.push(candidate.candidate_id);
                    if selected_ids.len() >= target_limit {
                        break;
                    }
                }
            }
            if selected_ids.len() >= target_limit {
                break;
            }
            match page.next_cursor.filter(|next| !next.trim().is_empty()) {
                Some(next) if page.page.truncated => cursor = Some(next),
                _ => break,
            }
        }

        if selected_ids.is_empty() {
            return LibrarySearchOutcome::Results {
                passages: Vec::new(),
            };
        }

        let mut passages = Vec::with_capacity(selected_ids.len());
        for chunk in selected_ids.chunks(MAX_WIRE_READ_IDS) {
            let read_body = match self.post_read_units(chunk).await {
                Ok(body) => body,
                Err(reason) => return LibrarySearchOutcome::Unavailable { reason },
            };
            let mut by_id: BTreeMap<String, WireReadItem> = read_body
                .results
                .into_iter()
                .map(|item| {
                    let id = match &item {
                        WireReadItem::Available { candidate_id, .. }
                        | WireReadItem::Unavailable { candidate_id, .. } => candidate_id.clone(),
                    };
                    (id, item)
                })
                .collect();

            for id in chunk {
                match by_id.remove(id) {
                    Some(WireReadItem::Available { candidate_id, unit }) => {
                        match passage_from_wire_unit(&candidate_id, *unit) {
                            Ok(passage) => passages.push(passage),
                            Err(reason) => {
                                return LibrarySearchOutcome::Unavailable {
                                    reason: format!("clinical_library_invalid_passage: {reason}"),
                                };
                            }
                        }
                    }
                    Some(WireReadItem::Unavailable {
                        cause:
                            WireReadUnavailableCause::NotFoundOrNotAuthorized
                            | WireReadUnavailableCause::InvalidCandidateId,
                        ..
                    }) => {}
                    Some(WireReadItem::Unavailable {
                        cause:
                            WireReadUnavailableCause::VisualUnavailable
                            | WireReadUnavailableCause::VisualReadLimit,
                        ..
                    }) => {
                        return LibrarySearchOutcome::Unavailable {
                            reason: "clinical_library_visual_unavailable".into(),
                        };
                    }
                    None => {
                        return LibrarySearchOutcome::Unavailable {
                            reason: "clinical_library_incomplete_read_response".into(),
                        };
                    }
                }
            }
        }

        LibrarySearchOutcome::Results { passages }
    }

    /// Reads one exact passage by [`PassageId`] from `POST /v1/read` using the
    /// same [`passage_from_wire_unit`] translator as [`Self::search`].
    pub async fn read_passage(&self, passage_id: &PassageId) -> LibraryReadOutcome {
        if let Endpoint::Unconfigured { reason } = &self.endpoint {
            return LibraryReadOutcome::Unavailable {
                reason: reason.clone(),
            };
        }

        let requested = [passage_id.as_str().to_owned()];
        let read_body = match self.post_read_units(&requested).await {
            Ok(body) => body,
            Err(reason) => return LibraryReadOutcome::Unavailable { reason },
        };
        let Some(item) = read_body.results.into_iter().find(|entry| match entry {
            WireReadItem::Available { candidate_id, .. }
            | WireReadItem::Unavailable { candidate_id, .. } => candidate_id == passage_id.as_str(),
        }) else {
            return LibraryReadOutcome::Unavailable {
                reason: "clinical_library_incomplete_read_response".into(),
            };
        };

        match item {
            WireReadItem::Available { candidate_id, unit } => {
                match passage_from_wire_unit(&candidate_id, *unit) {
                    Ok(passage) => LibraryReadOutcome::Passage {
                        passage: Box::new(passage),
                    },
                    Err(reason) => LibraryReadOutcome::Unavailable {
                        reason: format!("clinical_library_invalid_passage: {reason}"),
                    },
                }
            }
            WireReadItem::Unavailable {
                cause:
                    WireReadUnavailableCause::NotFoundOrNotAuthorized
                    | WireReadUnavailableCause::InvalidCandidateId,
                ..
            } => LibraryReadOutcome::NotFound {
                passage_id: passage_id.clone(),
            },
            WireReadItem::Unavailable {
                cause:
                    WireReadUnavailableCause::VisualUnavailable
                    | WireReadUnavailableCause::VisualReadLimit,
                ..
            } => LibraryReadOutcome::Unavailable {
                reason: "clinical_library_visual_unavailable".into(),
            },
        }
    }

    /// Validates and executes one chat tool invocation (`search_clinical_library`
    /// or `read_clinical_library`), returning the exact JSON result delivered to
    /// the model.
    pub async fn execute(&self, name: &str, arguments: &Value) -> ToolOutcome {
        let request = match ClinicalLibraryRequest::parse(name, arguments) {
            Ok(request) => request,
            Err(reason) => return ToolOutcome::refused(reason.to_string()),
        };
        match request {
            ClinicalLibraryRequest::Search {
                query,
                collection,
                limit,
            } => {
                let outcome = self.search(&query, collection, limit).await;
                ToolOutcome::Returned(
                    serde_json::to_string(&outcome).expect("LibrarySearchOutcome serializes"),
                )
            }
            ClinicalLibraryRequest::Read { passage_id } => {
                let outcome = self.read_passage(&passage_id).await;
                ToolOutcome::Returned(
                    serde_json::to_string(&outcome).expect("LibraryReadOutcome serializes"),
                )
            }
        }
    }
}

fn classify_transport_error(error: &reqwest::Error) -> String {
    if error.is_timeout() {
        "clinical_library_timeout".into()
    } else if error.is_connect() {
        "clinical_library_unreachable".into()
    } else {
        "clinical_library_transport_error".into()
    }
}

fn classify_status(status: StatusCode) -> String {
    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            format!("clinical_library_access_refused_http_{}", status.as_u16())
        }
        _ => format!("clinical_library_http_{}", status.as_u16()),
    }
}

/// Tool definition for searching clinical literature by clinical concept.
pub fn search_tool() -> Tool {
    Tool {
        name: SEARCH_LIBRARY_TOOL.into(),
        description: "Search the clinical library for published articles, drug labels, MMWR reports, and USPSTF recommendations. Supply clinical concepts only — never include patient names, birth dates, medical record numbers, workspace identifiers, or contact details in `query`. Each returned passage includes its `passage_id`, `source`, publication `date`, `status` (including retraction or correction notices), `attribution`, and `text`. An `outcome` of `unavailable` means the clinical library could not be reached and is distinct from an empty `passages` list.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Clinical concept query only (disease, drug, finding, biomarker, or guideline topic). Never include patient-identifying details."
                },
                "collection": {
                    "anyOf": [
                        { "type": "null" },
                        {
                            "type": "string",
                            "enum": ["pubmed", "pmc", "dailymed", "mmwr", "uspstf"]
                        }
                    ]
                },
                "limit": {
                    "anyOf": [
                        { "type": "null" },
                        { "type": "integer", "minimum": 1, "maximum": MAX_SEARCH_LIMIT }
                    ]
                }
            },
            "required": ["query", "collection", "limit"],
            "additionalProperties": false
        }),
    }
}

/// Tool definition for reading an exact clinical library passage by `passage_id`.
pub fn read_tool() -> Tool {
    Tool {
        name: READ_LIBRARY_TOOL.into(),
        description: "Open an exact passage from the clinical library by its `passage_id` to inspect or verify its full passage text, source metadata, publication date, publication status (including any retraction or correction notice), and attribution before citing it.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "passage_id": {
                    "type": "string",
                    "description": "Exact passage_id returned by search_clinical_library."
                }
            },
            "required": ["passage_id"],
            "additionalProperties": false
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concept_query_allows_clinical_terms_and_rejects_patient_identifiers() {
        for allowed in [
            "dapagliflozin chronic kidney disease eGFR threshold",
            "mRNA vaccine myocarditis incidence",
            "dobutamine stress echocardiography contraindications",
            "USPSTF colorectal cancer screening age 45",
        ] {
            assert!(
                ConceptQuery::parse(allowed).is_ok(),
                "expected clinical concept `{allowed}` to pass"
            );
        }

        for rejected in [
            "pw-0123456789abcdef1234567890abcdef potassium",
            "hyperkalemia MRN 448192",
            "warfarin dosing DOB 04/12/1979",
            "SSN 123-45-6789 hypertension",
            "jane.doe@example.com lab interpretation",
            "patient name Maria Gomez a1c target",
        ] {
            let err = ConceptQuery::parse(rejected).unwrap_err();
            assert!(
                matches!(err, LibraryInputError::PatientIdentifierInQuery),
                "expected `{rejected}` to fail patient identifier check, got {err:?}"
            );
        }
    }

    #[test]
    fn search_wire_request_serializes_exact_seven_strict_keys() {
        let query = ConceptQuery::parse("dapagliflozin eGFR threshold").unwrap();
        let encoded = serde_json::to_value(SearchWireRequest::query_page(&query, None)).unwrap();
        assert_eq!(
            encoded,
            json!({
                "mode": "query",
                "query": "dapagliflozin eGFR threshold",
                "scheme": null,
                "value": null,
                "candidate_id": null,
                "relationship": null,
                "cursor": null
            })
        );
    }

    #[test]
    fn source_code_mapping_handles_cdc_mmwr_and_nlm_pubmed() {
        assert_eq!(
            LibraryCollection::from_source_code("cdc-mmwr"),
            Ok(LibraryCollection::Mmwr)
        );
        assert_eq!(
            LibraryCollection::from_source_code("nlm-pubmed"),
            Ok(LibraryCollection::Pubmed)
        );
        assert_eq!(
            LibraryCollection::from_source_code("dailymed"),
            Ok(LibraryCollection::Dailymed)
        );
    }
}
