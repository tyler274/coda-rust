//! Clinical library search and exact passage read tools for physician chat.
//!
//! Queries leaving patient chat carry clinical concepts only — never patient
//! names, birth dates, medical record numbers, workspace identifiers, or
//! contact details. Search and exact read return [`LibraryPassage`] records
//! carrying source metadata, publication date, [`PublicationStatus`] (including
//! retraction and correction notices), attribution, and integrity-checked text.
//! When the clinical library service is unconfigured, unreachable, or refuses
//! a call, tools return a structured `unavailable` outcome distinct from empty
//! search results so chat can still answer from the patient chart and state the
//! literature limitation.

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

/// Validated opaque identifier of one clinical library passage.
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

/// Standing of a clinical library item, carrying explicit notice text whenever
/// corrected, flagged with an expression of concern, or retracted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum PublicationStatus {
    /// Active item with no retraction, correction, or expression-of-concern notice.
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
}

impl PublicationStatus {
    fn is_valid(&self) -> bool {
        match self {
            Self::Current => true,
            Self::Corrected { notice }
            | Self::ExpressionOfConcern { notice }
            | Self::Retracted { notice } => !notice.trim().is_empty(),
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
    /// Section heading within the document, when present.
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
    /// Unambiguous publication or revision date.
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

/// Wire payload sent to `POST /v1/passages/search`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchWireRequest {
    /// Validated clinical concept query.
    pub query: String,
    /// Optional collection filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<LibraryCollection>,
    /// Maximum passages requested.
    pub limit: usize,
}

/// Wire payload returned from `POST /v1/passages/search`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchWireResponse {
    /// Matching passages.
    pub passages: Vec<LibraryPassage>,
}

/// Wire payload sent to `POST /v1/passages/read`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadWireRequest {
    /// Requested passage identifier.
    pub passage_id: PassageId,
}

/// Wire payload returned from `POST /v1/passages/read`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadWireResponse {
    /// Matching passage when present in the active library version.
    pub passage: Option<LibraryPassage>,
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

/// Client for searching and reading the clinical library service.
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

    /// Reads `CLINICAL_LIBRARY_URL` (and optional `CLINICAL_LIBRARY_BEARER_TOKEN`)
    /// from the process environment, defaulting cleanly to unconfigured when unset.
    pub fn from_env() -> Self {
        Self::from_reader(|key| std::env::var(key).ok())
    }

    /// Reads clinical library settings from an explicit environment reader.
    pub fn from_reader(read: impl Fn(&str) -> Option<String>) -> Self {
        let raw_url = read("CLINICAL_LIBRARY_URL")
            .or_else(|| read("CODA_CLINICAL_LIBRARY_URL"))
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let Some(url) = raw_url else {
            return Self::unconfigured();
        };
        match Self::http(url) {
            Ok(client) => match read("CLINICAL_LIBRARY_BEARER_TOKEN") {
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

    /// Runs a validated concept search against the clinical library.
    pub async fn search(
        &self,
        query: &ConceptQuery,
        collection: Option<LibraryCollection>,
        limit: usize,
    ) -> LibrarySearchOutcome {
        let Endpoint::Http {
            base_url,
            bearer_token,
            client,
        } = &self.endpoint
        else {
            let Endpoint::Unconfigured { reason } = &self.endpoint else {
                unreachable!()
            };
            return LibrarySearchOutcome::Unavailable {
                reason: reason.clone(),
            };
        };

        let url = format!("{base_url}/v1/passages/search");
        let mut request = client.post(url).json(&SearchWireRequest {
            query: query.as_str().to_owned(),
            collection,
            limit,
        });
        if let Some(token) = bearer_token {
            request = request.bearer_auth(token);
        }

        let response = match request.send().await {
            Ok(response) => response,
            Err(cause) => {
                return LibrarySearchOutcome::Unavailable {
                    reason: classify_transport_error(&cause),
                };
            }
        };
        if response.status() != StatusCode::OK {
            return LibrarySearchOutcome::Unavailable {
                reason: classify_status(response.status()),
            };
        }
        let body: SearchWireResponse = match response.json().await {
            Ok(body) => body,
            Err(_) => {
                return LibrarySearchOutcome::Unavailable {
                    reason: "clinical_library_malformed_response".into(),
                };
            }
        };
        for passage in &body.passages {
            if let Err(reason) = passage.validate() {
                return LibrarySearchOutcome::Unavailable {
                    reason: format!("clinical_library_invalid_passage: {reason}"),
                };
            }
        }
        LibrarySearchOutcome::Results {
            passages: body.passages,
        }
    }

    /// Reads one exact passage by [`PassageId`] from the clinical library.
    pub async fn read_passage(&self, passage_id: &PassageId) -> LibraryReadOutcome {
        let Endpoint::Http {
            base_url,
            bearer_token,
            client,
        } = &self.endpoint
        else {
            let Endpoint::Unconfigured { reason } = &self.endpoint else {
                unreachable!()
            };
            return LibraryReadOutcome::Unavailable {
                reason: reason.clone(),
            };
        };

        let url = format!("{base_url}/v1/passages/read");
        let mut request = client.post(url).json(&ReadWireRequest {
            passage_id: passage_id.clone(),
        });
        if let Some(token) = bearer_token {
            request = request.bearer_auth(token);
        }

        let response = match request.send().await {
            Ok(response) => response,
            Err(cause) => {
                return LibraryReadOutcome::Unavailable {
                    reason: classify_transport_error(&cause),
                };
            }
        };
        if response.status() == StatusCode::NOT_FOUND {
            return LibraryReadOutcome::NotFound {
                passage_id: passage_id.clone(),
            };
        }
        if response.status() != StatusCode::OK {
            return LibraryReadOutcome::Unavailable {
                reason: classify_status(response.status()),
            };
        }
        let body: ReadWireResponse = match response.json().await {
            Ok(body) => body,
            Err(_) => {
                return LibraryReadOutcome::Unavailable {
                    reason: "clinical_library_malformed_response".into(),
                };
            }
        };
        let Some(passage) = body.passage else {
            return LibraryReadOutcome::NotFound {
                passage_id: passage_id.clone(),
            };
        };
        if passage.passage_id != *passage_id {
            return LibraryReadOutcome::Unavailable {
                reason: "clinical_library_passage_id_mismatch".into(),
            };
        }
        if let Err(reason) = passage.validate() {
            return LibraryReadOutcome::Unavailable {
                reason: format!("clinical_library_invalid_passage: {reason}"),
            };
        }
        LibraryReadOutcome::Passage {
            passage: Box::new(passage),
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
}
