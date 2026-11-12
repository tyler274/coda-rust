//! Reading a patient record by source, version, and location in bounded, usable parts.
//!
//! Every read first checks [`PatientAccess`]: an unauthorized physician (including
//! clinic-demographic-only standing or another practice) receives [`PatientRecordRead::Denied`]
//! with no record existence, version, or content exposed.
//!
//! Within an authorized workspace, [`VersionedRecordCatalog`] resolves current or
//! retained historical revisions of a candidate, [`RecordPartReader`] resolves inline,
//! attachment, and 1-indexed scan/fax page parts with distinct [`DeliveredPart`] outcomes
//! for readable, missing, unreadable, and unavailable parts, and location resolution
//! returns the same verified source from its start with a stated [`LocationLimit`]
//! whenever a requested location cannot be recovered. Corrupt source revisions and
//! underlying reader or database failures return [`RecordReadError`].

use std::collections::BTreeSet;
use std::fmt::{self, Display};
use std::num::{NonZeroU32, NonZeroUsize};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{RecordDate, SourceRevision};

/// Standing of a physician with respect to a patient workspace on their practice panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelStanding {
    /// Active panel access to clinical records in the workspace.
    Allowed,
    /// Practice-level demographic visibility only; clinical record reads are forbidden.
    ClinicDemographic,
    /// No access to the patient workspace.
    Denied,
}

/// Evaluated clinical record read access for a patient workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatientAccess {
    /// Clinical record reads are permitted in `workspace`.
    Allowed {
        /// Authorized patient workspace identifier.
        workspace: String,
    },
    /// Only basic clinic demographics are permitted; clinical records return nothing.
    ClinicDemographicOnly,
    /// No access to this patient; clinical records return nothing.
    Denied,
}

impl PatientAccess {
    /// Evaluates clinical record access from practice ownership and panel standing.
    pub fn for_physician(
        physician_practice: &str,
        patient_practice: Option<&str>,
        workspace: &str,
        standing: PanelStanding,
    ) -> Self {
        if physician_practice.trim().is_empty() || workspace.trim().is_empty() {
            return Self::Denied;
        }
        if patient_practice != Some(physician_practice) {
            return Self::Denied;
        }
        match standing {
            PanelStanding::Allowed => Self::Allowed {
                workspace: workspace.to_owned(),
            },
            PanelStanding::ClinicDemographic => Self::ClinicDemographicOnly,
            PanelStanding::Denied => Self::Denied,
        }
    }
}

/// Top-level result of reading a patient record under [`PatientAccess`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "access", rename_all = "snake_case")]
pub enum PatientRecordRead {
    /// Access was authorized; holds the record read result for the workspace.
    Authorized(RecordReadResult),
    /// Access was refused: holds nothing regardless of whether any record exists.
    Denied,
}

/// 1-indexed page number within a multi-page scan or fax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PageNumber(NonZeroU32);

impl PageNumber {
    /// Page 1 of a scan or fax.
    pub const FIRST: Self = Self(NonZeroU32::MIN);

    /// Constructs a 1-indexed page number, returning `None` for zero.
    pub const fn new(page: u32) -> Option<Self> {
        match NonZeroU32::new(page) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the 1-indexed integer value.
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl fmt::Display for PageNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

/// Validated UTF-8 byte budget per record-read call.
///
/// The minimum of 4 bytes (`char::MAX_LEN_UTF8`) guarantees every continuation slice
/// advances by at least one complete UTF-8 scalar value without splitting codepoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PageBudget(NonZeroUsize);

impl PageBudget {
    /// Smallest permitted UTF-8 content budget (`char::MAX_LEN_UTF8`).
    pub const MIN_BYTES: usize = 4;
    /// Default UTF-8 content budget when a caller does not choose a smaller window.
    pub const DEFAULT_BYTES: usize = 16_384;
    /// Largest permitted UTF-8 content budget per read call.
    pub const MAX_BYTES: usize = 65_536;

    /// Validates `bytes` within [`Self::MIN_BYTES`]..=[`Self::MAX_BYTES`].
    pub fn new(bytes: usize) -> Result<Self, InvalidReadRequest> {
        if !(Self::MIN_BYTES..=Self::MAX_BYTES).contains(&bytes) {
            return Err(InvalidReadRequest::BudgetOutOfRange {
                requested: bytes,
                min: Self::MIN_BYTES,
                max: Self::MAX_BYTES,
            });
        }
        let non_zero = NonZeroUsize::new(bytes).expect("MIN_BYTES is non-zero");
        Ok(Self(non_zero))
    }

    /// Returns the validated byte budget.
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

impl Default for PageBudget {
    fn default() -> Self {
        Self::new(Self::DEFAULT_BYTES).expect("DEFAULT_BYTES is within bounds")
    }
}

/// Optional source qualification when addressing a candidate record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "scope",
    content = "source_id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum SourceSelector {
    /// Match the candidate across any source in the workspace.
    #[default]
    AnySource,
    /// Match only records owned by this `source_id`.
    Source(String),
}

/// Which version of a candidate record to read.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum VersionSelector {
    /// Read the newest active revision in the workspace.
    #[default]
    Latest,
    /// Read a specific immutable source revision (current or retained older version).
    Revision(String),
    /// Read the revision that was active at or before workspace observation `version`.
    ObservedAt(u64),
}

/// Half-open UTF-8 byte interval `[start_byte..end_byte)` inside a part or scan page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ByteSpan {
    /// Inclusive UTF-8 start byte offset.
    pub start_byte: usize,
    /// Exclusive UTF-8 end byte offset.
    pub end_byte: usize,
}

/// Selector for a non-scan part within a [`SourceRevision`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum PartSelector {
    /// The source-native content (`SourceRevision::content`).
    NativeRecord,
    /// Zero-based index into `SourceRevision::readable_parts`.
    RetainedIndex(usize),
    /// Source-assigned label matching a retained part or attachment.
    Label(String),
    /// Source-assigned attachment identifier from an [`AttachmentReader`].
    AttachmentId(String),
}

/// Selector for a scan or fax document attached to a [`SourceRevision`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "scope",
    content = "scan_id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ScanSelector {
    /// Select the single scan on the record (unrecoverable if zero or multiple scans exist).
    #[default]
    Primary,
    /// Select the scan with this `scan_id`.
    Named(String),
}

/// Portion of a selected part or scan page to deliver.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "slice",
    content = "span",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ContentSlice {
    /// Deliver the entire part or page (paginated via continuation if larger than budget).
    #[default]
    Full,
    /// Deliver an exact UTF-8 byte span `[start_byte..end_byte)`.
    ByteSpan(ByteSpan),
}

/// Bounds what remains to be read by a [`ContinuationCursor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "scope", rename_all = "snake_case")]
pub enum ContinuationScope {
    /// Continue across all remaining parts of the document in order.
    Document,
    /// Continue only inside `part_ordinal` up to `end_byte`.
    WithinPart {
        /// Exclusive UTF-8 end byte offset inside `part_ordinal`.
        end_byte: usize,
    },
}

impl<'de> Deserialize<'de> for ContinuationScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "scope", rename_all = "snake_case", deny_unknown_fields)]
        enum Wire {
            Document {},
            WithinPart { end_byte: usize },
        }

        Ok(match Wire::deserialize(deserializer)? {
            Wire::Document {} => Self::Document,
            Wire::WithinPart { end_byte } => Self::WithinPart { end_byte },
        })
    }
}

/// Revision-pinned cursor for continuing a multi-part or large-part read without silent cuts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationCursor {
    /// Owning source identifier.
    pub source_id: String,
    /// Opaque candidate identifier.
    pub candidate_id: String,
    /// Pinned immutable revision identifier.
    pub revision: String,
    /// Zero-based ordinal into the revision's resolved part list.
    pub part_ordinal: usize,
    /// UTF-8 byte offset within `part_ordinal` where the next slice begins.
    pub byte_offset: usize,
    /// Whether continuation spans the rest of the document or a single part/span.
    pub scope: ContinuationScope,
}

/// Selected location inside a patient record version.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RecordLocation {
    /// Read across the whole document from its first part.
    #[default]
    Whole,
    /// Read one selected part (whole part or exact UTF-8 byte span).
    Part {
        /// Which part to read.
        part: PartSelector,
        /// Full part or exact byte span.
        #[serde(default)]
        slice: ContentSlice,
    },
    /// Read one 1-indexed page of a scan or fax (whole page or exact UTF-8 byte span).
    ScanPage {
        /// Which scan document holds the page.
        #[serde(default)]
        scan: ScanSelector,
        /// 1-indexed page number.
        page: PageNumber,
        /// Full page or exact byte span on that page.
        #[serde(default)]
        slice: ContentSlice,
    },
    /// Resume reading from a returned [`ContinuationCursor`].
    Continuation(ContinuationCursor),
}

impl<'de> Deserialize<'de> for RecordLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum Wire {
            Whole {},
            Part {
                part: PartSelector,
                #[serde(default)]
                slice: ContentSlice,
            },
            ScanPage {
                #[serde(default)]
                scan: ScanSelector,
                page: PageNumber,
                #[serde(default)]
                slice: ContentSlice,
            },
            Continuation(ContinuationCursor),
        }

        Ok(match Wire::deserialize(deserializer)? {
            Wire::Whole {} => Self::Whole,
            Wire::Part { part, slice } => Self::Part { part, slice },
            Wire::ScanPage { scan, page, slice } => Self::ScanPage { scan, page, slice },
            Wire::Continuation(cursor) => Self::Continuation(cursor),
        })
    }
}

/// Request to read a patient record by source, version, location, and byte budget.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordReadRequest {
    /// Optional source qualifier.
    pub source: SourceSelector,
    /// Opaque source candidate identifier.
    pub candidate_id: String,
    /// Requested version (latest, specific revision, or workspace version).
    pub version: VersionSelector,
    /// Requested location inside the selected version.
    pub location: RecordLocation,
    /// Maximum UTF-8 content bytes to deliver in this call.
    pub budget: PageBudget,
}

impl RecordReadRequest {
    /// Creates a request for the latest version and whole document of `candidate_id` under `budget`.
    pub fn new(candidate_id: impl Into<String>, budget: PageBudget) -> Self {
        Self {
            source: SourceSelector::AnySource,
            candidate_id: candidate_id.into(),
            version: VersionSelector::Latest,
            location: RecordLocation::Whole,
            budget,
        }
    }

    /// Restricts the read to records from `source_id`.
    pub fn with_source(mut self, source_id: impl Into<String>) -> Self {
        self.source = SourceSelector::Source(source_id.into());
        self
    }

    /// Selects a specific [`VersionSelector`].
    pub fn with_version(mut self, version: VersionSelector) -> Self {
        self.version = version;
        self
    }

    /// Selects a specific [`RecordLocation`].
    pub fn with_location(mut self, location: RecordLocation) -> Self {
        self.location = location;
        self
    }

    fn validate(&self) -> Result<(), InvalidReadRequest> {
        if self.candidate_id.trim().is_empty() {
            return Err(InvalidReadRequest::EmptyCandidateId);
        }
        match &self.source {
            SourceSelector::AnySource => {}
            SourceSelector::Source(source_id) => {
                if source_id.trim().is_empty() {
                    return Err(InvalidReadRequest::EmptySourceId);
                }
            }
        }
        match &self.version {
            VersionSelector::Latest | VersionSelector::ObservedAt(_) => {}
            VersionSelector::Revision(revision) => {
                if revision.trim().is_empty() {
                    return Err(InvalidReadRequest::EmptyRevision);
                }
            }
        }
        match &self.location {
            RecordLocation::Whole | RecordLocation::Continuation(_) => {}
            RecordLocation::Part { part, slice: _ } => match part {
                PartSelector::NativeRecord | PartSelector::RetainedIndex(_) => {}
                PartSelector::Label(value) | PartSelector::AttachmentId(value) => {
                    if value.trim().is_empty() {
                        return Err(InvalidReadRequest::EmptyPartSelector);
                    }
                }
            },
            RecordLocation::ScanPage {
                scan,
                page: _,
                slice: _,
            } => match scan {
                ScanSelector::Primary => {}
                ScanSelector::Named(scan_id) => {
                    if scan_id.trim().is_empty() {
                        return Err(InvalidReadRequest::EmptyScanId);
                    }
                }
            },
        }
        Ok(())
    }
}

/// Typed identity of one addressable part or scan page on a [`SourceRevision`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PartIdentity {
    /// The source-native UTF-8 representation (`SourceRevision::content`).
    NativeRecord,
    /// Decoded text retained inline on the revision (`SourceRevision::readable_parts`).
    Retained {
        /// Zero-based index inside `readable_parts`.
        index: usize,
        /// Source-assigned label.
        label: String,
    },
    /// Named attachment supplied by an [`AttachmentReader`].
    Attachment {
        /// Source-owned attachment identifier.
        attachment_id: String,
        /// Human-readable attachment label.
        label: String,
        /// Declared MIME media type when present.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        media_type: Option<String>,
    },
    /// Entire scan or fax document when individual pages could not be opened.
    ScanDocument {
        /// Source-owned scan identifier.
        scan_id: String,
        /// Human-readable scan or fax label.
        label: String,
    },
    /// One 1-indexed page inside a scan or fax supplied by a [`ScanReader`].
    ScanPage {
        /// Source-owned scan identifier.
        scan_id: String,
        /// Human-readable scan or fax label.
        label: String,
        /// 1-indexed page number.
        page: PageNumber,
        /// Total page count of the scan when known.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        total_pages: Option<PageNumber>,
    },
}

impl PartIdentity {
    fn display_name(&self) -> String {
        match self {
            Self::NativeRecord => "native_record".into(),
            Self::Retained { index, label } => format!("retained part {index} ('{label}')"),
            Self::Attachment {
                attachment_id,
                label,
                ..
            } => format!("attachment '{label}' ({attachment_id})"),
            Self::ScanDocument { scan_id, label } => format!("scan '{label}' ({scan_id})"),
            Self::ScanPage {
                scan_id,
                label,
                page,
                ..
            } => format!("scan page {page} of '{label}' ({scan_id})"),
        }
    }
}

/// State of one part or scan page resolved from a source or part reader.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PartState {
    /// Complete UTF-8 text of the part or page.
    Readable {
        /// Exact UTF-8 text.
        text: String,
    },
    /// The source declares this part or page absent / not supplied.
    Missing {
        /// Source-reported detail explaining what is absent.
        detail: String,
    },
    /// The part or page is present, could not be decoded or OCR-read, and must be named.
    Unreadable {
        /// Why the part or page could not be read.
        reason: String,
    },
    /// The backing attachment or scan store could not provide this part right now.
    Unavailable {
        /// Why the part is currently unavailable.
        reason: String,
    },
}

/// One resolved part on a [`SourceRevision`] in canonical document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPart {
    /// Identity of the part or scan page.
    pub part: PartIdentity,
    /// Resolved content or non-readable outcome.
    pub state: PartState,
}

/// One attachment resolved by an [`AttachmentReader`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachmentPart {
    /// Source-owned attachment identifier.
    pub attachment_id: String,
    /// Human-readable attachment label.
    pub label: String,
    /// Optional MIME media type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Outcome of resolving this attachment.
    pub state: PartState,
}

/// One 1-indexed page inside a [`ScanDocument`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanPageEntry {
    /// 1-indexed page number.
    pub page: PageNumber,
    /// Outcome of resolving this page.
    pub state: PartState,
}

/// Outcome of resolving a scan or fax document through a [`ScanReader`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ScanDocumentState {
    /// Individual 1-indexed pages resolved from the scan.
    Pages {
        /// Ordered page entries.
        pages: Vec<ScanPageEntry>,
    },
    /// The source states this scan document was not supplied.
    Missing {
        /// Source-reported detail.
        detail: String,
    },
    /// The scan container itself could not be decoded.
    Unreadable {
        /// Why the scan container could not be decoded.
        reason: String,
    },
    /// The scan store could not fetch the scan right now.
    Unavailable {
        /// Why the scan is currently unavailable.
        reason: String,
    },
}

/// One scan or fax document resolved by a [`ScanReader`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanDocument {
    /// Source-owned scan identifier.
    pub scan_id: String,
    /// Human-readable scan or fax label.
    pub label: String,
    /// Page entries or whole-document failure state.
    pub content: ScanDocumentState,
}

/// Boxed error preserved as the cause when an [`AttachmentReader`] or [`ScanReader`] fails.
#[derive(Debug)]
pub struct SourcePartError(Box<dyn std::error::Error + Send + Sync>);

impl SourcePartError {
    /// Wraps an underlying adapter or I/O error while keeping it as [`std::error::Error::source`].
    pub fn new(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Box::new(error))
    }
}

impl fmt::Display for SourcePartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&*self.0, f)
    }
}

impl std::error::Error for SourcePartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.0)
    }
}

/// Contract for attachment decoders that resolve named attachments for a [`SourceRevision`].
pub trait AttachmentReader: Send + Sync {
    /// Resolves attachments belonging to `revision`. Transport or adapter failures return `Err`.
    fn read_attachments(
        &self,
        revision: &SourceRevision,
    ) -> Result<Vec<AttachmentPart>, SourcePartError>;
}

/// Contract for scan/fax readers that resolve 1-indexed pages for a [`SourceRevision`].
pub trait ScanReader: Send + Sync {
    /// Resolves scan or fax documents belonging to `revision`. Transport or adapter failures return `Err`.
    fn read_scans(&self, revision: &SourceRevision) -> Result<Vec<ScanDocument>, SourcePartError>;
}

/// Unified part resolver combining a [`SourceRevision`]'s inline parts with attachment and scan readers.
pub trait RecordPartReader: Send + Sync {
    /// Resolves all parts and scan pages of `revision` in deterministic order.
    fn resolve_parts(
        &self,
        revision: &SourceRevision,
    ) -> Result<Vec<ResolvedPart>, RecordReadError>;
}

/// Default reader that surfaces [`SourceRevision::content`] and [`SourceRevision::readable_parts`].
#[derive(Debug, Clone, Copy, Default)]
pub struct InlinePartReader;

/// No-op attachment or scan reader used when no external adapter is attached.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoExternalReader;

impl AttachmentReader for NoExternalReader {
    fn read_attachments(
        &self,
        _revision: &SourceRevision,
    ) -> Result<Vec<AttachmentPart>, SourcePartError> {
        Ok(Vec::new())
    }
}

impl ScanReader for NoExternalReader {
    fn read_scans(&self, _revision: &SourceRevision) -> Result<Vec<ScanDocument>, SourcePartError> {
        Ok(Vec::new())
    }
}

/// Composes an [`AttachmentReader`] and a [`ScanReader`] alongside a revision's inline parts.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompositePartReader<A = NoExternalReader, S = NoExternalReader> {
    /// Attachment reader implementation.
    pub attachments: A,
    /// Scan/fax page reader implementation.
    pub scans: S,
}

impl<A, S> CompositePartReader<A, S> {
    /// Combines `attachments` and `scans` into one [`RecordPartReader`].
    pub const fn new(attachments: A, scans: S) -> Self {
        Self { attachments, scans }
    }
}

/// Reader that also materializes structured `"attachments"` and `"scans"` manifests
/// carried inside [`SourceRevision::provenance`].
#[derive(Debug, Clone, Copy, Default)]
pub struct ProvenancePartReader;

/// Validation error for a [`RecordReadRequest`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvalidReadRequest {
    /// `candidate_id` was blank.
    #[error("candidate_id must not be empty")]
    EmptyCandidateId,
    /// `SourceSelector::Source` carried a blank `source_id`.
    #[error("source_id must not be empty")]
    EmptySourceId,
    /// `VersionSelector::Revision` carried a blank revision string.
    #[error("revision selector must not be empty")]
    EmptyRevision,
    /// A label or attachment selector carried a blank identifier.
    #[error("part selector identifier must not be empty")]
    EmptyPartSelector,
    /// `ScanSelector::Named` carried a blank `scan_id`.
    #[error("scan_id must not be empty")]
    EmptyScanId,
    /// `PageBudget` was outside [`PageBudget::MIN_BYTES`]..=[`PageBudget::MAX_BYTES`].
    #[error("page budget {requested} bytes must be between {min} and {max} bytes")]
    BudgetOutOfRange {
        /// Requested byte count.
        requested: usize,
        /// Minimum permitted byte count.
        min: usize,
        /// Maximum permitted byte count.
        max: usize,
    },
}

/// Reason a retained [`SourceRevision`] failed integrity validation on read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvalidSourceRevisionError {
    /// Required identity metadata on the source revision was empty.
    #[error("source revision missing required field '{field}'")]
    EmptyField {
        /// Name of the empty field.
        field: &'static str,
    },
    /// Retained UTF-8 content bytes did not match `SourceRevision::sha256`.
    #[error("source content sha256 mismatch (expected {expected}, computed {actual})")]
    DigestMismatch {
        /// Recorded SHA-256 hex digest.
        expected: String,
        /// Computed SHA-256 hex digest of `content`.
        actual: String,
    },
    /// `SourceRevision::provenance` was JSON `null`.
    #[error("source provenance must not be null")]
    NullProvenance,
}

/// Failure when reading a patient record. Never masked as empty, missing, or unavailable.
#[derive(Debug, thiserror::Error)]
pub enum RecordReadError {
    /// Caller supplied malformed request parameters.
    #[error("invalid record read request: {0}")]
    InvalidRequest(#[from] InvalidReadRequest),
    /// Retained [`SourceRevision`] failed integrity or digest validation.
    #[error("invalid source revision '{revision}' for candidate '{candidate_id}': {source}")]
    InvalidSourceRevision {
        /// Candidate identifier.
        candidate_id: String,
        /// Revision identifier.
        revision: String,
        /// Underlying validation cause.
        #[source]
        source: InvalidSourceRevisionError,
    },
    /// Stored JSON observation in the database could not be decoded.
    #[error(
        "corrupt stored source observation for candidate '{candidate_id}' at version {observed_at_version}: {source}"
    )]
    CorruptStoredObservation {
        /// Candidate identifier.
        candidate_id: String,
        /// Workspace observation version of the corrupt row.
        observed_at_version: u64,
        /// Underlying JSON decoding error.
        #[source]
        source: serde_json::Error,
    },
    /// Underlying attachment or scan reader failed to read from its source.
    #[error("part reader failed for candidate '{candidate_id}' revision '{revision}': {source}")]
    PartReader {
        /// Candidate identifier.
        candidate_id: String,
        /// Revision identifier.
        revision: String,
        /// Underlying reader cause.
        #[source]
        source: SourcePartError,
    },
    /// Database transaction or query failed.
    #[error("source database read failed: {0}")]
    Database(#[from] sea_orm::DbErr),
}

/// How a historical revision stands relative to the workspace's latest observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum HistoricalStanding {
    /// A newer revision is now active for this candidate.
    SupersededBy {
        /// Currently active revision identifier.
        current_revision: String,
    },
    /// The candidate was withdrawn at a later workspace version.
    Withdrawn,
}

/// Whether the opened revision is the current workspace version or an older historical version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "freshness", rename_all = "snake_case")]
pub enum VersionResolution {
    /// The opened revision is currently active in the workspace.
    Current,
    /// The opened revision is a retained older version.
    Historical {
        /// Current standing of the candidate in the latest workspace observation.
        standing: HistoricalStanding,
    },
}

/// Non-readable state of a specific target part referenced by a location selector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum NonReadableOutcome {
    /// Target part was declared absent by the source.
    Missing {
        /// Source-reported detail.
        detail: String,
    },
    /// Target part could not be decoded or OCR-read.
    Unreadable {
        /// Reason decoding failed.
        reason: String,
    },
    /// Target part is currently unavailable from its store.
    Unavailable {
        /// Reason the part is unavailable.
        reason: String,
    },
}

impl NonReadableOutcome {
    fn summary(&self) -> String {
        match self {
            Self::Missing { detail } => format!("not supplied ({detail})"),
            Self::Unreadable { reason } => format!("could not be read ({reason})"),
            Self::Unavailable { reason } => format!("currently unavailable ({reason})"),
        }
    }
}

/// Specific reason a [`ContinuationCursor`] could not be resumed directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum ContinuationLimit {
    /// Cursor targeted a different `(source_id, candidate_id, revision)` than the resolved record.
    IdentityMismatch {
        /// Resolved source identifier.
        expected_source_id: String,
        /// Resolved candidate identifier.
        expected_candidate_id: String,
        /// Resolved revision identifier.
        expected_revision: String,
    },
    /// `part_ordinal` exceeded the resolved part count.
    PartOutOfRange {
        /// Requested part ordinal.
        part_ordinal: usize,
        /// Number of parts on the record.
        part_count: usize,
    },
    /// Cursor pointed into a non-readable part at a non-zero byte offset or single-part scope.
    TargetPartNotReadable {
        /// Identity of the non-readable part.
        part: PartIdentity,
        /// Non-readable state of that part.
        state: NonReadableOutcome,
    },
    /// `byte_offset` exceeded the part's byte length (or pointed to the end of a non-empty part).
    OffsetOutOfBounds {
        /// Identity of the part.
        part: PartIdentity,
        /// Requested byte offset.
        byte_offset: usize,
        /// Total UTF-8 bytes of the part.
        part_bytes: usize,
    },
    /// `byte_offset` landed inside a multi-byte UTF-8 codepoint.
    InvalidUtf8Offset {
        /// Identity of the part.
        part: PartIdentity,
        /// Requested byte offset.
        byte_offset: usize,
    },
    /// `WithinPart` end offset was invalid (`end_byte <= byte_offset`, out of bounds, or non-UTF-8 boundary).
    InvalidScopeEnd {
        /// Identity of the part.
        part: PartIdentity,
        /// Requested start offset.
        byte_offset: usize,
        /// Requested end offset.
        end_byte: usize,
        /// Total UTF-8 bytes of the part.
        part_bytes: usize,
    },
}

/// Structured limit explaining why a [`RecordLocation`] could not be recovered on a valid source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LocationLimit {
    /// No part matched the requested [`PartSelector`].
    PartNotFound {
        /// Requested part selector.
        requested: PartSelector,
    },
    /// No scan matched the requested [`ScanSelector`].
    ScanNotFound {
        /// Requested scan selector.
        requested: ScanSelector,
    },
    /// `ScanSelector::Primary` was used on a record with multiple scans.
    AmbiguousPrimaryScan {
        /// Number of scans attached to the record.
        scan_count: usize,
    },
    /// Requested scan page number was not present in the scan.
    PageOutOfRange {
        /// Identifier of the matched scan.
        scan_id: String,
        /// Requested 1-indexed page number.
        requested: PageNumber,
        /// Available 1-indexed page numbers in the scan.
        available_pages: Vec<PageNumber>,
    },
    /// Requested byte span was empty, inverted, or exceeded the part's byte length.
    SpanOutOfBounds {
        /// Identity of the matched part or page.
        part: PartIdentity,
        /// Requested byte span.
        requested: ByteSpan,
        /// Total UTF-8 byte length of the part.
        part_bytes: usize,
    },
    /// Requested byte span split a multi-byte UTF-8 character.
    InvalidUtf8Boundary {
        /// Identity of the matched part or page.
        part: PartIdentity,
        /// Requested byte span.
        requested: ByteSpan,
    },
    /// Requested part or scan page exists on the record, and is missing, unreadable, or unavailable.
    TargetPartNotReadable {
        /// Identity of the matched part or scan.
        part: PartIdentity,
        /// Non-readable state of that target.
        state: NonReadableOutcome,
    },
    /// Supplied [`ContinuationCursor`] could not be resumed directly.
    InvalidContinuation {
        /// Reason the continuation cursor was invalid.
        reason: ContinuationLimit,
    },
}

impl LocationLimit {
    /// Human-readable description of why the location could not be recovered.
    pub fn stated_limit(&self) -> String {
        match self {
            Self::PartNotFound { requested } => match requested {
                PartSelector::NativeRecord => {
                    "Requested native record part could not be found; returning the same source from the start.".into()
                }
                PartSelector::RetainedIndex(index) => format!(
                    "Requested retained part index {index} is not present on this revision; returning the same source from the start."
                ),
                PartSelector::Label(label) => format!(
                    "Requested part '{label}' is not present on this revision; returning the same source from the start."
                ),
                PartSelector::AttachmentId(attachment_id) => format!(
                    "Requested attachment '{attachment_id}' is not present on this revision; returning the same source from the start."
                ),
            },
            Self::ScanNotFound { requested } => match requested {
                ScanSelector::Primary => {
                    "This revision has no attached scan or fax; returning the same source from the start.".into()
                }
                ScanSelector::Named(scan_id) => format!(
                    "Requested scan '{scan_id}' is not present on this revision; returning the same source from the start."
                ),
            },
            Self::AmbiguousPrimaryScan { scan_count } => format!(
                "This revision carries {scan_count} scans, so a scan_id is required to select a page; returning the same source from the start."
            ),
            Self::PageOutOfRange {
                scan_id,
                requested,
                available_pages,
            } => format!(
                "Requested page {requested} is outside scan '{scan_id}' ({} page(s) available); returning the same source from the start.",
                available_pages.len()
            ),
            Self::SpanOutOfBounds {
                part,
                requested,
                part_bytes,
            } => format!(
                "Requested byte span {}..{} is outside {} ({part_bytes} bytes); returning the same source from the start.",
                requested.start_byte,
                requested.end_byte,
                part.display_name()
            ),
            Self::InvalidUtf8Boundary { part, requested } => format!(
                "Requested byte span {}..{} splits a UTF-8 character in {}; returning the same source from the start.",
                requested.start_byte,
                requested.end_byte,
                part.display_name()
            ),
            Self::TargetPartNotReadable { part, state } => format!(
                "Requested {} is {}; returning the same source from the start.",
                part.display_name(),
                state.summary()
            ),
            Self::InvalidContinuation { reason } => match reason {
                ContinuationLimit::IdentityMismatch {
                    expected_source_id,
                    expected_candidate_id,
                    expected_revision,
                } => format!(
                    "Continuation cursor did not match source '{expected_source_id}' candidate '{expected_candidate_id}' revision '{expected_revision}'; returning the same source from the start."
                ),
                ContinuationLimit::PartOutOfRange {
                    part_ordinal,
                    part_count,
                } => format!(
                    "Continuation cursor part {part_ordinal} exceeds the {part_count} part(s) of this revision; returning the same source from the start."
                ),
                ContinuationLimit::TargetPartNotReadable { part, state } => format!(
                    "Continuation cursor target {} is {}; returning the same source from the start.",
                    part.display_name(),
                    state.summary()
                ),
                ContinuationLimit::OffsetOutOfBounds {
                    part,
                    byte_offset,
                    part_bytes,
                } => format!(
                    "Continuation cursor offset {byte_offset} is outside {} ({part_bytes} bytes); returning the same source from the start.",
                    part.display_name()
                ),
                ContinuationLimit::InvalidUtf8Offset { part, byte_offset } => format!(
                    "Continuation cursor offset {byte_offset} splits a UTF-8 character in {}; returning the same source from the start.",
                    part.display_name()
                ),
                ContinuationLimit::InvalidScopeEnd {
                    part,
                    byte_offset,
                    end_byte,
                    part_bytes,
                } => format!(
                    "Continuation cursor range {byte_offset}..{end_byte} is invalid for {} ({part_bytes} bytes); returning the same source from the start.",
                    part.display_name()
                ),
            },
        }
    }
}

/// Resolution of the caller's [`RecordLocation`] on the selected source revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LocationResolution {
    /// Requested location was recovered directly.
    Recovered {
        /// Recovered location.
        location: RecordLocation,
    },
    /// Requested location could not be recovered; returning the same source from its start
    /// together with a stated limit.
    Unrecoverable {
        /// Location originally requested.
        requested: RecordLocation,
        /// Structured reason the location could not be recovered.
        limit: LocationLimit,
        /// Human-readable description of the limit.
        stated_limit: String,
    },
}

/// One exact UTF-8 slice delivered from a readable part or scan page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartSlice {
    /// Identity of the part or scan page.
    pub part: PartIdentity,
    /// Inclusive UTF-8 start byte offset inside this part.
    pub start_byte: usize,
    /// Exclusive UTF-8 end byte offset inside this part.
    pub end_byte: usize,
    /// Total UTF-8 byte length of this part.
    pub total_bytes: usize,
    /// Exact UTF-8 slice `&full_text[start_byte..end_byte]`.
    pub content: String,
}

/// Delivered outcome for one part or scan page in [`OpenedRecord::parts`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum DeliveredPart {
    /// Readable UTF-8 slice matching the saved source.
    Readable(PartSlice),
    /// Part or page declared absent by the source.
    Missing {
        /// Identity of the missing part or page.
        part: PartIdentity,
        /// Source-reported detail.
        detail: String,
    },
    /// Part or page present in the source that could not be decoded or OCR-read.
    Unreadable {
        /// Identity of the unreadable part or page.
        part: PartIdentity,
        /// Why decoding or OCR failed.
        reason: String,
    },
    /// Part or page whose backing attachment or scan store is currently unavailable.
    Unavailable {
        /// Identity of the unavailable part or page.
        part: PartIdentity,
        /// Why the store could not supply the part.
        reason: String,
    },
}

/// Whether a read finished delivering all targeted bytes/parts or has a continuation cursor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", content = "cursor", rename_all = "snake_case")]
pub enum ReadCompletion {
    /// All targeted parts and UTF-8 bytes have been delivered; nothing was cut.
    Complete,
    /// Additional parts or bytes remain; pass this cursor to continue reading.
    Partial(ContinuationCursor),
}

/// Verified metadata envelope of an opened [`SourceRevision`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordEnvelope {
    /// Source identifier owning the record.
    pub source_id: String,
    /// Opaque candidate identifier.
    pub candidate_id: String,
    /// Immutable revision identifier.
    pub revision: String,
    /// Source-assigned record type.
    pub record_type: String,
    /// Source-assigned dates at source precision.
    pub dates: Vec<RecordDate>,
    /// SHA-256 hex digest of `SourceRevision::content`.
    pub sha256: String,
    /// Source-owned provenance value.
    pub provenance: Value,
}

impl RecordEnvelope {
    fn from_revision(revision: &SourceRevision) -> Self {
        Self {
            source_id: revision.source_id.clone(),
            candidate_id: revision.candidate_id.clone(),
            revision: revision.revision.clone(),
            record_type: revision.record_type.clone(),
            dates: revision.dates.clone(),
            sha256: revision.sha256.clone(),
            provenance: revision.provenance.clone(),
        }
    }
}

/// Opened patient record version with its delivered part outcomes and continuation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenedRecord {
    /// Verified source metadata envelope.
    pub envelope: RecordEnvelope,
    /// Whether this version is current or a retained older version.
    pub version_resolution: VersionResolution,
    /// Whether the requested location was recovered or fell back to the same source with a limit.
    pub location: LocationResolution,
    /// Ordered part slices and named non-readable part outcomes delivered in this call.
    pub parts: Vec<DeliveredPart>,
    /// Completion or continuation cursor for remaining parts/bytes.
    pub completion: ReadCompletion,
}

/// Why a candidate record was classified as [`RecordReadResult::Missing`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum RecordMissingReason {
    /// No observation for this candidate (and source filter) exists in the workspace.
    UnknownRecord,
    /// Candidate was first observed only after the requested `ObservedAt` workspace version.
    NotObservedAtVersion {
        /// Requested workspace observation version.
        version: u64,
    },
}

/// Why a candidate record was classified as [`RecordReadResult::Unavailable`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum RecordUnavailableReason {
    /// Candidate was withdrawn from the chart in the relevant observation.
    Withdrawn,
    /// Candidate exists, and the requested `revision` is not among its retained revisions.
    RevisionNotRetained {
        /// Requested revision identifier.
        requested: String,
    },
}

/// Outcome of reading a patient record inside an authorized workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RecordReadResult {
    /// Selected record version was found, verified, and read.
    Opened(Box<OpenedRecord>),
    /// Record does not exist in the workspace (or at the requested observation version).
    Missing {
        /// Requested candidate identifier.
        candidate_id: String,
        /// Requested source filter.
        source: SourceSelector,
        /// Specific missing reason.
        reason: RecordMissingReason,
    },
    /// Record is known to the workspace, and the requested version is withdrawn or not retained.
    Unavailable {
        /// Requested candidate identifier.
        candidate_id: String,
        /// Requested source filter.
        source: SourceSelector,
        /// Specific unavailability reason.
        reason: RecordUnavailableReason,
    },
}

/// One append-only observation event for a candidate inside a patient workspace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ObservationEntry {
    /// A [`SourceRevision`] recorded at `observed_at_version`.
    Present {
        /// Workspace observation version when recorded.
        observed_at_version: u64,
        /// Recorded source revision.
        revision: SourceRevision,
    },
    /// A withdrawal recorded at `observed_at_version`.
    Withdrawn {
        /// Workspace observation version when withdrawn.
        observed_at_version: u64,
        /// Withdrawn candidate identifier.
        candidate_id: String,
        /// Owning workspace identifier.
        workspace: String,
        /// Source identifier that withdrew the record, when known.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_id: Option<String>,
    },
}

impl ObservationEntry {
    const fn observed_at_version(&self) -> u64 {
        match self {
            Self::Present {
                observed_at_version,
                ..
            }
            | Self::Withdrawn {
                observed_at_version,
                ..
            } => *observed_at_version,
        }
    }

    fn matches(&self, workspace: &str, candidate_id: &str, source: &SourceSelector) -> bool {
        match self {
            Self::Present { revision, .. } => {
                if revision.workspace != workspace || revision.candidate_id != candidate_id {
                    return false;
                }
                match source {
                    SourceSelector::AnySource => true,
                    SourceSelector::Source(source_id) => revision.source_id == *source_id,
                }
            }
            Self::Withdrawn {
                candidate_id: entry_candidate,
                workspace: entry_workspace,
                source_id: entry_source,
                ..
            } => {
                if entry_workspace != workspace || entry_candidate != candidate_id {
                    return false;
                }
                match source {
                    SourceSelector::AnySource => true,
                    SourceSelector::Source(source_id) => {
                        entry_source.as_deref().is_none_or(|s| s == source_id)
                    }
                }
            }
        }
    }
}

enum SelectedVersion<'a> {
    Found {
        revision: &'a SourceRevision,
        version_resolution: VersionResolution,
    },
    Missing(RecordMissingReason),
    Unavailable(RecordUnavailableReason),
}

/// Versioned catalog of retained source observations for reading current and historical versions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VersionedRecordCatalog {
    workspace_version: u64,
    entries: Vec<ObservationEntry>,
}

impl VersionedRecordCatalog {
    /// Builds a catalog from a flat slice of current active [`SourceRevision`]s at `workspace_version`.
    pub fn from_current(workspace_version: u64, sources: &[SourceRevision]) -> Self {
        let entries = sources
            .iter()
            .cloned()
            .map(|revision| ObservationEntry::Present {
                observed_at_version: workspace_version,
                revision,
            })
            .collect();
        Self {
            workspace_version,
            entries,
        }
    }

    /// Builds a catalog from append-only [`ObservationEntry`] history up to `workspace_version`.
    pub fn from_observations(workspace_version: u64, mut entries: Vec<ObservationEntry>) -> Self {
        entries.sort_by_key(ObservationEntry::observed_at_version);
        Self {
            workspace_version,
            entries,
        }
    }

    /// Workspace observation version pinned by this catalog.
    pub const fn workspace_version(&self) -> u64 {
        self.workspace_version
    }

    fn select<'a>(&'a self, workspace: &str, request: &RecordReadRequest) -> SelectedVersion<'a> {
        let matching: Vec<&ObservationEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                entry.observed_at_version() <= self.workspace_version
                    && entry.matches(workspace, &request.candidate_id, &request.source)
            })
            .collect();

        let Some(latest_entry) = matching.last().copied() else {
            return SelectedVersion::Missing(RecordMissingReason::UnknownRecord);
        };

        match &request.version {
            VersionSelector::Latest => match latest_entry {
                ObservationEntry::Present { revision, .. } => SelectedVersion::Found {
                    revision,
                    version_resolution: VersionResolution::Current,
                },
                ObservationEntry::Withdrawn { .. } => {
                    SelectedVersion::Unavailable(RecordUnavailableReason::Withdrawn)
                }
            },
            VersionSelector::Revision(requested_rev) => match latest_entry {
                ObservationEntry::Withdrawn { .. } => {
                    SelectedVersion::Unavailable(RecordUnavailableReason::Withdrawn)
                }
                ObservationEntry::Present {
                    revision: current_revision,
                    ..
                } => {
                    let found = matching.iter().rev().find_map(|entry| match entry {
                        ObservationEntry::Present { revision, .. }
                            if revision.revision == *requested_rev =>
                        {
                            Some(revision)
                        }
                        ObservationEntry::Present { .. } | ObservationEntry::Withdrawn { .. } => {
                            None
                        }
                    });
                    match found {
                        Some(revision) => {
                            let version_resolution =
                                if revision.revision == current_revision.revision {
                                    VersionResolution::Current
                                } else {
                                    VersionResolution::Historical {
                                        standing: HistoricalStanding::SupersededBy {
                                            current_revision: current_revision.revision.clone(),
                                        },
                                    }
                                };
                            SelectedVersion::Found {
                                revision,
                                version_resolution,
                            }
                        }
                        None => SelectedVersion::Unavailable(
                            RecordUnavailableReason::RevisionNotRetained {
                                requested: requested_rev.clone(),
                            },
                        ),
                    }
                }
            },
            VersionSelector::ObservedAt(target_version) => {
                let at_version = matching
                    .iter()
                    .rev()
                    .copied()
                    .find(|entry| entry.observed_at_version() <= *target_version);
                let Some(entry_at_version) = at_version else {
                    return SelectedVersion::Missing(RecordMissingReason::NotObservedAtVersion {
                        version: *target_version,
                    });
                };
                match entry_at_version {
                    ObservationEntry::Withdrawn { .. } => {
                        SelectedVersion::Unavailable(RecordUnavailableReason::Withdrawn)
                    }
                    ObservationEntry::Present { revision, .. } => {
                        let version_resolution = match latest_entry {
                            ObservationEntry::Present {
                                revision: current_revision,
                                ..
                            } => {
                                if current_revision.revision == revision.revision {
                                    VersionResolution::Current
                                } else {
                                    VersionResolution::Historical {
                                        standing: HistoricalStanding::SupersededBy {
                                            current_revision: current_revision.revision.clone(),
                                        },
                                    }
                                }
                            }
                            ObservationEntry::Withdrawn { .. } => VersionResolution::Historical {
                                standing: HistoricalStanding::Withdrawn,
                            },
                        };
                        SelectedVersion::Found {
                            revision,
                            version_resolution,
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn validate_source_revision(revision: &SourceRevision) -> Result<(), RecordReadError> {
    revision
        .validate_integrity()
        .map_err(|source| RecordReadError::InvalidSourceRevision {
            candidate_id: revision.candidate_id.clone(),
            revision: revision.revision.clone(),
            source,
        })
}

fn inline_parts(revision: &SourceRevision) -> Vec<ResolvedPart> {
    let mut parts = Vec::with_capacity(1 + revision.readable_parts.len());
    parts.push(ResolvedPart {
        part: PartIdentity::NativeRecord,
        state: PartState::Readable {
            text: revision.content.clone(),
        },
    });
    for (index, retained) in revision.readable_parts.iter().enumerate() {
        parts.push(ResolvedPart {
            part: PartIdentity::Retained {
                index,
                label: retained.label.clone(),
            },
            state: PartState::Readable {
                text: retained.text.clone(),
            },
        });
    }
    parts
}

fn append_attachments(parts: &mut Vec<ResolvedPart>, attachments: Vec<AttachmentPart>) {
    for attachment in attachments {
        parts.push(ResolvedPart {
            part: PartIdentity::Attachment {
                attachment_id: attachment.attachment_id,
                label: attachment.label,
                media_type: attachment.media_type,
            },
            state: attachment.state,
        });
    }
}

fn append_scans(parts: &mut Vec<ResolvedPart>, scans: Vec<ScanDocument>) {
    for scan in scans {
        match scan.content {
            ScanDocumentState::Pages { mut pages } => {
                pages.sort_by_key(|entry| entry.page);
                let total_pages = pages.iter().map(|entry| entry.page).max();
                for entry in pages {
                    parts.push(ResolvedPart {
                        part: PartIdentity::ScanPage {
                            scan_id: scan.scan_id.clone(),
                            label: scan.label.clone(),
                            page: entry.page,
                            total_pages,
                        },
                        state: entry.state,
                    });
                }
            }
            ScanDocumentState::Missing { detail } => {
                parts.push(ResolvedPart {
                    part: PartIdentity::ScanDocument {
                        scan_id: scan.scan_id,
                        label: scan.label,
                    },
                    state: PartState::Missing { detail },
                });
            }
            ScanDocumentState::Unreadable { reason } => {
                parts.push(ResolvedPart {
                    part: PartIdentity::ScanDocument {
                        scan_id: scan.scan_id,
                        label: scan.label,
                    },
                    state: PartState::Unreadable { reason },
                });
            }
            ScanDocumentState::Unavailable { reason } => {
                parts.push(ResolvedPart {
                    part: PartIdentity::ScanDocument {
                        scan_id: scan.scan_id,
                        label: scan.label,
                    },
                    state: PartState::Unavailable { reason },
                });
            }
        }
    }
}

impl RecordPartReader for InlinePartReader {
    fn resolve_parts(
        &self,
        revision: &SourceRevision,
    ) -> Result<Vec<ResolvedPart>, RecordReadError> {
        Ok(inline_parts(revision))
    }
}

impl<A: AttachmentReader, S: ScanReader> RecordPartReader for CompositePartReader<A, S> {
    fn resolve_parts(
        &self,
        revision: &SourceRevision,
    ) -> Result<Vec<ResolvedPart>, RecordReadError> {
        let mut parts = inline_parts(revision);
        let attachments = self
            .attachments
            .read_attachments(revision)
            .map_err(|source| RecordReadError::PartReader {
                candidate_id: revision.candidate_id.clone(),
                revision: revision.revision.clone(),
                source,
            })?;
        append_attachments(&mut parts, attachments);
        let scans =
            self.scans
                .read_scans(revision)
                .map_err(|source| RecordReadError::PartReader {
                    candidate_id: revision.candidate_id.clone(),
                    revision: revision.revision.clone(),
                    source,
                })?;
        append_scans(&mut parts, scans);
        Ok(parts)
    }
}

impl RecordPartReader for ProvenancePartReader {
    fn resolve_parts(
        &self,
        revision: &SourceRevision,
    ) -> Result<Vec<ResolvedPart>, RecordReadError> {
        let mut parts = inline_parts(revision);
        if let Some(raw_attachments) = revision
            .provenance
            .get("attachments")
            .filter(|value| !value.is_null())
        {
            let attachments: Vec<AttachmentPart> = serde_json::from_value(raw_attachments.clone())
                .map_err(|err| RecordReadError::PartReader {
                    candidate_id: revision.candidate_id.clone(),
                    revision: revision.revision.clone(),
                    source: SourcePartError::new(err),
                })?;
            append_attachments(&mut parts, attachments);
        }
        if let Some(raw_scans) = revision
            .provenance
            .get("scans")
            .filter(|value| !value.is_null())
        {
            let scans: Vec<ScanDocument> =
                serde_json::from_value(raw_scans.clone()).map_err(|err| {
                    RecordReadError::PartReader {
                        candidate_id: revision.candidate_id.clone(),
                        revision: revision.revision.clone(),
                        source: SourcePartError::new(err),
                    }
                })?;
            append_scans(&mut parts, scans);
        }
        Ok(parts)
    }
}

pub(super) struct ReadWindow {
    pub(super) start_ordinal: usize,
    pub(super) start_byte: usize,
    pub(super) scope: ContinuationScope,
    pub(super) resolution: LocationResolution,
}

fn unrecoverable_window(requested: &RecordLocation, limit: LocationLimit) -> ReadWindow {
    let stated_limit = limit.stated_limit();
    ReadWindow {
        start_ordinal: 0,
        start_byte: 0,
        scope: ContinuationScope::Document,
        resolution: LocationResolution::Unrecoverable {
            requested: requested.clone(),
            limit,
            stated_limit,
        },
    }
}

fn non_readable_outcome(state: &PartState) -> Option<NonReadableOutcome> {
    match state {
        PartState::Readable { .. } => None,
        PartState::Missing { detail } => Some(NonReadableOutcome::Missing {
            detail: detail.clone(),
        }),
        PartState::Unreadable { reason } => Some(NonReadableOutcome::Unreadable {
            reason: reason.clone(),
        }),
        PartState::Unavailable { reason } => Some(NonReadableOutcome::Unavailable {
            reason: reason.clone(),
        }),
    }
}

fn resolve_part_slice(
    requested_location: &RecordLocation,
    ordinal: usize,
    part: &PartIdentity,
    text: &str,
    slice: ContentSlice,
) -> ReadWindow {
    match slice {
        ContentSlice::Full => ReadWindow {
            start_ordinal: ordinal,
            start_byte: 0,
            scope: ContinuationScope::WithinPart {
                end_byte: text.len(),
            },
            resolution: LocationResolution::Recovered {
                location: requested_location.clone(),
            },
        },
        ContentSlice::ByteSpan(span) => {
            let valid_bounds = if text.is_empty() {
                span.start_byte == 0 && span.end_byte == 0
            } else {
                span.start_byte < span.end_byte && span.end_byte <= text.len()
            };
            if !valid_bounds {
                return unrecoverable_window(
                    requested_location,
                    LocationLimit::SpanOutOfBounds {
                        part: part.clone(),
                        requested: span,
                        part_bytes: text.len(),
                    },
                );
            }
            if !text.is_char_boundary(span.start_byte) || !text.is_char_boundary(span.end_byte) {
                return unrecoverable_window(
                    requested_location,
                    LocationLimit::InvalidUtf8Boundary {
                        part: part.clone(),
                        requested: span,
                    },
                );
            }
            ReadWindow {
                start_ordinal: ordinal,
                start_byte: span.start_byte,
                scope: ContinuationScope::WithinPart {
                    end_byte: span.end_byte,
                },
                resolution: LocationResolution::Recovered {
                    location: requested_location.clone(),
                },
            }
        }
    }
}

fn matches_part_selector(identity: &PartIdentity, selector: &PartSelector) -> bool {
    match selector {
        PartSelector::NativeRecord => matches!(identity, PartIdentity::NativeRecord),
        PartSelector::RetainedIndex(target_index) => match identity {
            PartIdentity::Retained { index, .. } => index == target_index,
            PartIdentity::NativeRecord
            | PartIdentity::Attachment { .. }
            | PartIdentity::ScanDocument { .. }
            | PartIdentity::ScanPage { .. } => false,
        },
        PartSelector::Label(target_label) => match identity {
            PartIdentity::NativeRecord => target_label == "native_record",
            PartIdentity::Retained { label, .. }
            | PartIdentity::Attachment { label, .. }
            | PartIdentity::ScanDocument { label, .. } => label == target_label,
            PartIdentity::ScanPage { .. } => false,
        },
        PartSelector::AttachmentId(target_id) => match identity {
            PartIdentity::Attachment { attachment_id, .. } => attachment_id == target_id,
            PartIdentity::NativeRecord
            | PartIdentity::Retained { .. }
            | PartIdentity::ScanDocument { .. }
            | PartIdentity::ScanPage { .. } => false,
        },
    }
}

pub(super) fn resolve_location(
    revision: &SourceRevision,
    resolved_parts: &[ResolvedPart],
    location: &RecordLocation,
) -> ReadWindow {
    match location {
        RecordLocation::Whole => ReadWindow {
            start_ordinal: 0,
            start_byte: 0,
            scope: ContinuationScope::Document,
            resolution: LocationResolution::Recovered {
                location: RecordLocation::Whole,
            },
        },
        RecordLocation::Part { part, slice } => {
            let Some((ordinal, matched)) = resolved_parts
                .iter()
                .enumerate()
                .find(|(_, item)| matches_part_selector(&item.part, part))
            else {
                return unrecoverable_window(
                    location,
                    LocationLimit::PartNotFound {
                        requested: part.clone(),
                    },
                );
            };
            match &matched.state {
                PartState::Readable { text } => {
                    resolve_part_slice(location, ordinal, &matched.part, text, *slice)
                }
                other @ (PartState::Missing { .. }
                | PartState::Unreadable { .. }
                | PartState::Unavailable { .. }) => unrecoverable_window(
                    location,
                    LocationLimit::TargetPartNotReadable {
                        part: matched.part.clone(),
                        state: non_readable_outcome(other).expect("non-readable state"),
                    },
                ),
            }
        }
        RecordLocation::ScanPage { scan, page, slice } => {
            let mut distinct_scans = BTreeSet::new();
            for item in resolved_parts {
                match &item.part {
                    PartIdentity::ScanDocument { scan_id, .. }
                    | PartIdentity::ScanPage { scan_id, .. } => {
                        distinct_scans.insert(scan_id.clone());
                    }
                    PartIdentity::NativeRecord
                    | PartIdentity::Retained { .. }
                    | PartIdentity::Attachment { .. } => {}
                }
            }
            let target_scan_id = match scan {
                ScanSelector::Primary => {
                    if distinct_scans.is_empty() {
                        return unrecoverable_window(
                            location,
                            LocationLimit::ScanNotFound {
                                requested: ScanSelector::Primary,
                            },
                        );
                    }
                    if distinct_scans.len() > 1 {
                        return unrecoverable_window(
                            location,
                            LocationLimit::AmbiguousPrimaryScan {
                                scan_count: distinct_scans.len(),
                            },
                        );
                    }
                    distinct_scans
                        .into_iter()
                        .next()
                        .expect("single scan exists")
                }
                ScanSelector::Named(scan_id) => {
                    if !distinct_scans.contains(scan_id) {
                        return unrecoverable_window(
                            location,
                            LocationLimit::ScanNotFound {
                                requested: ScanSelector::Named(scan_id.clone()),
                            },
                        );
                    }
                    scan_id.clone()
                }
            };

            if let Some(whole_doc) = resolved_parts.iter().find(|item| match &item.part {
                PartIdentity::ScanDocument { scan_id, .. } => scan_id == &target_scan_id,
                PartIdentity::NativeRecord
                | PartIdentity::Retained { .. }
                | PartIdentity::Attachment { .. }
                | PartIdentity::ScanPage { .. } => false,
            }) && let Some(state) = non_readable_outcome(&whole_doc.state)
            {
                return unrecoverable_window(
                    location,
                    LocationLimit::TargetPartNotReadable {
                        part: whole_doc.part.clone(),
                        state,
                    },
                );
            }

            let matched_page = resolved_parts.iter().enumerate().find(|(_, item)| {
                matches!(
                    &item.part,
                    PartIdentity::ScanPage {
                        scan_id,
                        page: entry_page,
                        ..
                    } if scan_id == &target_scan_id && entry_page == page
                )
            });

            let Some((ordinal, matched)) = matched_page else {
                let available_pages: Vec<PageNumber> = resolved_parts
                    .iter()
                    .filter_map(|item| match &item.part {
                        PartIdentity::ScanPage {
                            scan_id,
                            page: entry_page,
                            ..
                        } if scan_id == &target_scan_id => Some(*entry_page),
                        PartIdentity::NativeRecord
                        | PartIdentity::Retained { .. }
                        | PartIdentity::Attachment { .. }
                        | PartIdentity::ScanDocument { .. }
                        | PartIdentity::ScanPage { .. } => None,
                    })
                    .collect();
                return unrecoverable_window(
                    location,
                    LocationLimit::PageOutOfRange {
                        scan_id: target_scan_id,
                        requested: *page,
                        available_pages,
                    },
                );
            };

            match &matched.state {
                PartState::Readable { text } => {
                    resolve_part_slice(location, ordinal, &matched.part, text, *slice)
                }
                other @ (PartState::Missing { .. }
                | PartState::Unreadable { .. }
                | PartState::Unavailable { .. }) => unrecoverable_window(
                    location,
                    LocationLimit::TargetPartNotReadable {
                        part: matched.part.clone(),
                        state: non_readable_outcome(other).expect("non-readable state"),
                    },
                ),
            }
        }
        RecordLocation::Continuation(cursor) => {
            if cursor.source_id != revision.source_id
                || cursor.candidate_id != revision.candidate_id
                || cursor.revision != revision.revision
            {
                return unrecoverable_window(
                    location,
                    LocationLimit::InvalidContinuation {
                        reason: ContinuationLimit::IdentityMismatch {
                            expected_source_id: revision.source_id.clone(),
                            expected_candidate_id: revision.candidate_id.clone(),
                            expected_revision: revision.revision.clone(),
                        },
                    },
                );
            }
            let Some(target) = resolved_parts.get(cursor.part_ordinal) else {
                return unrecoverable_window(
                    location,
                    LocationLimit::InvalidContinuation {
                        reason: ContinuationLimit::PartOutOfRange {
                            part_ordinal: cursor.part_ordinal,
                            part_count: resolved_parts.len(),
                        },
                    },
                );
            };
            match &target.state {
                other @ (PartState::Missing { .. }
                | PartState::Unreadable { .. }
                | PartState::Unavailable { .. }) => {
                    if cursor.byte_offset == 0
                        && matches!(cursor.scope, ContinuationScope::Document)
                    {
                        return ReadWindow {
                            start_ordinal: cursor.part_ordinal,
                            start_byte: 0,
                            scope: ContinuationScope::Document,
                            resolution: LocationResolution::Recovered {
                                location: location.clone(),
                            },
                        };
                    }
                    unrecoverable_window(
                        location,
                        LocationLimit::InvalidContinuation {
                            reason: ContinuationLimit::TargetPartNotReadable {
                                part: target.part.clone(),
                                state: non_readable_outcome(other).expect("non-readable state"),
                            },
                        },
                    )
                }
                PartState::Readable { text } => {
                    if cursor.byte_offset > text.len()
                        || (cursor.byte_offset == text.len() && !text.is_empty())
                    {
                        return unrecoverable_window(
                            location,
                            LocationLimit::InvalidContinuation {
                                reason: ContinuationLimit::OffsetOutOfBounds {
                                    part: target.part.clone(),
                                    byte_offset: cursor.byte_offset,
                                    part_bytes: text.len(),
                                },
                            },
                        );
                    }
                    if !text.is_char_boundary(cursor.byte_offset) {
                        return unrecoverable_window(
                            location,
                            LocationLimit::InvalidContinuation {
                                reason: ContinuationLimit::InvalidUtf8Offset {
                                    part: target.part.clone(),
                                    byte_offset: cursor.byte_offset,
                                },
                            },
                        );
                    }
                    match cursor.scope {
                        ContinuationScope::Document => ReadWindow {
                            start_ordinal: cursor.part_ordinal,
                            start_byte: cursor.byte_offset,
                            scope: ContinuationScope::Document,
                            resolution: LocationResolution::Recovered {
                                location: location.clone(),
                            },
                        },
                        ContinuationScope::WithinPart { end_byte } => {
                            let valid_end = end_byte <= text.len()
                                && text.is_char_boundary(end_byte)
                                && (cursor.byte_offset < end_byte
                                    || (text.is_empty() && end_byte == 0));
                            if !valid_end {
                                return unrecoverable_window(
                                    location,
                                    LocationLimit::InvalidContinuation {
                                        reason: ContinuationLimit::InvalidScopeEnd {
                                            part: target.part.clone(),
                                            byte_offset: cursor.byte_offset,
                                            end_byte,
                                            part_bytes: text.len(),
                                        },
                                    },
                                );
                            }
                            ReadWindow {
                                start_ordinal: cursor.part_ordinal,
                                start_byte: cursor.byte_offset,
                                scope: ContinuationScope::WithinPart { end_byte },
                                resolution: LocationResolution::Recovered {
                                    location: location.clone(),
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

fn slice_parts_within_budget(
    revision: &SourceRevision,
    resolved_parts: &[ResolvedPart],
    window: &ReadWindow,
    budget: PageBudget,
) -> (Vec<DeliveredPart>, ReadCompletion) {
    let mut remaining = budget.get();
    let mut parts = Vec::new();
    let end_ordinal = match window.scope {
        ContinuationScope::Document => resolved_parts.len(),
        ContinuationScope::WithinPart { .. } => {
            (window.start_ordinal + 1).min(resolved_parts.len())
        }
    };

    for (ordinal, current) in resolved_parts
        .iter()
        .enumerate()
        .take(end_ordinal)
        .skip(window.start_ordinal)
    {
        let start_byte = if ordinal == window.start_ordinal {
            window.start_byte
        } else {
            0
        };
        match &current.state {
            PartState::Missing { detail } => {
                parts.push(DeliveredPart::Missing {
                    part: current.part.clone(),
                    detail: detail.clone(),
                });
            }
            PartState::Unreadable { reason } => {
                parts.push(DeliveredPart::Unreadable {
                    part: current.part.clone(),
                    reason: reason.clone(),
                });
            }
            PartState::Unavailable { reason } => {
                parts.push(DeliveredPart::Unavailable {
                    part: current.part.clone(),
                    reason: reason.clone(),
                });
            }
            PartState::Readable { text } => {
                let target_end = match window.scope {
                    ContinuationScope::Document => text.len(),
                    ContinuationScope::WithinPart { end_byte } => end_byte.min(text.len()),
                };
                let available = &text[start_byte..target_end];
                if available.is_empty() {
                    parts.push(DeliveredPart::Readable(PartSlice {
                        part: current.part.clone(),
                        start_byte,
                        end_byte: target_end,
                        total_bytes: text.len(),
                        content: String::new(),
                    }));
                    continue;
                }
                let take = available.floor_char_boundary(available.len().min(remaining));
                if take == 0 {
                    return (
                        parts,
                        ReadCompletion::Partial(ContinuationCursor {
                            source_id: revision.source_id.clone(),
                            candidate_id: revision.candidate_id.clone(),
                            revision: revision.revision.clone(),
                            part_ordinal: ordinal,
                            byte_offset: start_byte,
                            scope: window.scope,
                        }),
                    );
                }
                let end_byte = start_byte + take;
                parts.push(DeliveredPart::Readable(PartSlice {
                    part: current.part.clone(),
                    start_byte,
                    end_byte,
                    total_bytes: text.len(),
                    content: text[start_byte..end_byte].to_owned(),
                }));
                remaining -= take;
                if end_byte < target_end {
                    return (
                        parts,
                        ReadCompletion::Partial(ContinuationCursor {
                            source_id: revision.source_id.clone(),
                            candidate_id: revision.candidate_id.clone(),
                            revision: revision.revision.clone(),
                            part_ordinal: ordinal,
                            byte_offset: end_byte,
                            scope: window.scope,
                        }),
                    );
                }
                if remaining == 0 && ordinal + 1 < end_ordinal {
                    return (
                        parts,
                        ReadCompletion::Partial(ContinuationCursor {
                            source_id: revision.source_id.clone(),
                            candidate_id: revision.candidate_id.clone(),
                            revision: revision.revision.clone(),
                            part_ordinal: ordinal + 1,
                            byte_offset: 0,
                            scope: window.scope,
                        }),
                    );
                }
            }
        }
    }

    (parts, ReadCompletion::Complete)
}

/// Reads a patient record by source, version, and location under [`PatientAccess`].
pub fn read_patient_record(
    access: &PatientAccess,
    request: &RecordReadRequest,
    catalog: &VersionedRecordCatalog,
    part_reader: &dyn RecordPartReader,
) -> Result<PatientRecordRead, RecordReadError> {
    let workspace = match access {
        PatientAccess::Allowed { workspace } if !workspace.trim().is_empty() => workspace.as_str(),
        PatientAccess::Allowed { .. }
        | PatientAccess::ClinicDemographicOnly
        | PatientAccess::Denied => return Ok(PatientRecordRead::Denied),
    };

    request.validate()?;

    let (revision, version_resolution) = match catalog.select(workspace, request) {
        SelectedVersion::Found {
            revision,
            version_resolution,
        } => (revision, version_resolution),
        SelectedVersion::Missing(reason) => {
            return Ok(PatientRecordRead::Authorized(RecordReadResult::Missing {
                candidate_id: request.candidate_id.clone(),
                source: request.source.clone(),
                reason,
            }));
        }
        SelectedVersion::Unavailable(reason) => {
            return Ok(PatientRecordRead::Authorized(
                RecordReadResult::Unavailable {
                    candidate_id: request.candidate_id.clone(),
                    source: request.source.clone(),
                    reason,
                },
            ));
        }
    };

    validate_source_revision(revision)?;
    let resolved_parts = part_reader.resolve_parts(revision)?;
    let window = resolve_location(revision, &resolved_parts, &request.location);
    let (parts, completion) =
        slice_parts_within_budget(revision, &resolved_parts, &window, request.budget);

    Ok(PatientRecordRead::Authorized(RecordReadResult::Opened(
        Box::new(OpenedRecord {
            envelope: RecordEnvelope::from_revision(revision),
            version_resolution,
            location: window.resolution,
            parts,
            completion,
        }),
    )))
}
