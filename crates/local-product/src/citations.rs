//! Shared citation and source-preview publication contract for physician jobs.
//!
//! Every job offers [`PUBLISH_CITATIONS_TOOL`] so the model records a dated preview for each
//! source record it read in its current working window and declares any whole-record, passage,
//! or structured-field citations. Finish-time publication checks grounding against the exact
//! bytes delivered in the latest context window, refuses ungrounded or unpreviewed reads with a
//! specific explanation to the model, and saves validated [`Citation`] and [`ReadRecordPreview`]
//! entries alongside the job's output.

use std::collections::{BTreeMap, BTreeSet};

use harness::engine::{EventKind, ProductMessage, Tool, ToolOutcome, recipe::ReplayEvent, results};
use physician_wire::{CitationLocator, CitedPart, SourcePath};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::physician_chat::Citation;
use crate::physician_sources::{
    DeliveredCitationError, NativeLink, ReadReceipt, RecordDate, RecordReceipt, SourceRevision,
    citation_id, citation_id_for, source_part,
};

/// Tool name shared by every physician job to submit previews and citations.
pub const PUBLISH_CITATIONS_TOOL: &str = "publish_citations";

/// Maximum Unicode scalar count of a model-written source preview.
pub const PREVIEW_MAX_CHARS: usize = 400;

const RECEIPT_KIND: &str = "published_citations_receipt";
const CITATION_OPEN: &str = "[[patient_source:";
const BASIS_OPEN: &str = "[[basis]]";
const BASIS_CLOSE: &str = "[[/basis]]";

/// Validated model-authored preview prose for one source record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PreviewText(String);

/// Why a model-submitted preview string was rejected.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PreviewError {
    /// Preview text was empty or whitespace-only.
    #[error("Write a non-empty preview sentence describing what the dated record says.")]
    Empty,
    /// Preview exceeded [`PREVIEW_MAX_CHARS`].
    #[error("Keep each source preview to at most {PREVIEW_MAX_CHARS} characters.")]
    TooLong,
    /// Preview contained bracketed marker syntax.
    #[error("Source previews describe the record plainly and must not contain `[[` markers.")]
    ContainsMarker,
    /// Model attempted to write `"Unavailable"`, which belongs exclusively to failed source reads.
    #[error(
        "\"Unavailable\" comes only from a source read that failed, never from a model-written preview."
    )]
    ReservedUnavailable,
}

impl PreviewText {
    /// Validates and trims model-submitted preview prose.
    pub fn parse(raw: &str) -> Result<Self, PreviewError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(PreviewError::Empty);
        }
        if trimmed.chars().count() > PREVIEW_MAX_CHARS {
            return Err(PreviewError::TooLong);
        }
        if trimmed.contains("[[") {
            return Err(PreviewError::ContainsMarker);
        }
        let bare = trimmed.trim_matches(|ch: char| ch.is_ascii_punctuation() || ch.is_whitespace());
        if bare.eq_ignore_ascii_case("unavailable") {
            return Err(PreviewError::ReservedUnavailable);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Validated preview string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes this wrapper and returns the validated preview string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Outcome saved for each patient record touched in a job's finishing window.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ReadRecordPreview {
    /// A record whose content reached the model, paired with its validated preview and dates.
    Preview {
        /// Source-owned record handle (`candidate_id`).
        reference: String,
        /// Exact source revision read by the model.
        revision: String,
        /// Short dated preview written by the model through [`PUBLISH_CITATIONS_TOOL`].
        preview: String,
        /// Dates supplied by the source record, when present.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        dates: Vec<RecordDate>,
        /// Live link into the source system when its adapter supplied one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native: Option<NativeLink>,
    },
    /// A record read that failed (`RecordReceipt::Unavailable`), never authored by the model.
    Unavailable {
        /// Requested record handle.
        reference: String,
        /// Failure code recorded by `read_patient_sources`.
        reason: String,
    },
}

impl ReadRecordPreview {
    /// Display status label: `"Unavailable"` only for failed reads, otherwise the model's preview.
    pub fn display_text(&self) -> &str {
        match self {
            Self::Preview { preview, .. } => preview.as_str(),
            Self::Unavailable { .. } => "Unavailable",
        }
    }
}

/// Sub-record target requested by the model in [`PUBLISH_CITATIONS_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RequestedLocator {
    /// Exact verbatim quote inside part `0` (`native_record`) or `1..` (`readable_parts`).
    Passage {
        /// Part index matching `read_patient_sources` (`0` for native record, `1..` for readable parts).
        part: usize,
        /// Exact UTF-8 substring from the delivered part text.
        quote: String,
        /// Optional UTF-8 start byte within `part` to disambiguate a repeated quote.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        start_byte: Option<usize>,
    },
    /// Canonical JSON field path inside a structured record.
    Field {
        /// Parsed path from the structured JSON root.
        path: SourcePath,
    },
}

/// One source preview submitted by the model through [`PUBLISH_CITATIONS_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviewSubmission {
    /// Source-owned record handle (`candidate_id`).
    pub reference: String,
    /// Exact revision read in the current window.
    pub revision: String,
    /// One or two plain sentences with the record's date on what the record itself says.
    pub preview: String,
}

/// One citation submitted by the model through [`PUBLISH_CITATIONS_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CitationSubmission {
    /// Handle used by the model (for example in `[[patient_source:ID]]`).
    pub id: String,
    /// Source-owned record handle (`candidate_id`).
    pub reference: String,
    /// Exact revision read in the current window.
    pub revision: String,
    /// `None` for a whole-record citation, or a passage/field locator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<RequestedLocator>,
}

/// Validated arguments to [`PUBLISH_CITATIONS_TOOL`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishCitationsCall {
    /// Previews for records read in the current context window.
    #[serde(default)]
    pub previews: Vec<PreviewSubmission>,
    /// Citations backing claims in the job's output.
    #[serde(default)]
    pub citations: Vec<CitationSubmission>,
    /// When true, replaces any earlier `publish_citations` submissions in the run.
    #[serde(default)]
    pub replace: bool,
}

/// Per-citation acknowledgement returned from [`PUBLISH_CITATIONS_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationHandleReceipt {
    /// Handle supplied by the model, usable in `[[patient_source:ID]]`.
    pub id: String,
    /// Source-owned record handle.
    pub reference: String,
    /// Source revision.
    pub revision: String,
    /// Resolved canonical citation digest when the source revision is already available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_id: Option<String>,
}

/// Structured tool output retained in `EventKind::ToolReturned` for [`PUBLISH_CITATIONS_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedCitationsReceipt {
    kind: String,
    /// The parsed call recorded by this receipt.
    pub call: PublishCitationsCall,
    /// Count of previews in this call.
    pub recorded_previews: usize,
    /// Receipts for each declared citation handle.
    pub citations: Vec<CitationHandleReceipt>,
    /// Non-fatal warnings (such as staging a reference not yet read in the current window).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

/// Contiguous UTF-8 byte span delivered in the current working window for one part of a record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeliveredSpan {
    /// Inclusive UTF-8 start byte within the part.
    pub start_byte: usize,
    /// Exclusive UTF-8 end byte within the part.
    pub end_byte: usize,
    /// Exact UTF-8 slice covering `start_byte..end_byte`.
    pub content: String,
}

/// Delivered passage spans for one `(reference, revision)` read partially in the current window.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeliveredPassages {
    /// Delivered spans keyed by part index (`0` for native record, `1..` for readable parts),
    /// paired with the part label reported by `read_patient_sources`.
    pub parts: BTreeMap<usize, (String, Vec<DeliveredSpan>)>,
}

/// Whole record delivered via `RecordReceipt::Available` in the current window.
#[derive(Clone, Debug, PartialEq)]
pub struct WholeDelivery {
    /// Reconstructed source bytes and metadata from the receipt.
    pub source: SourceRevision,
    /// Exact whole-record `citation_id` emitted by `read_patient_sources` on the receipt.
    pub receipt_citation_id: Option<String>,
}

/// Evidence that actually reached the model in full in its latest working window.
#[derive(Clone, Debug, Default)]
pub struct WindowEvidence {
    /// Whole records (`RecordReceipt::Available`) delivered in the current window.
    pub whole: BTreeMap<(String, String), WholeDelivery>,
    /// Partial passage spans (`RecordReceipt::Passage`) delivered in the current window.
    pub passages: BTreeMap<(String, String), DeliveredPassages>,
    /// Failed reads (`RecordReceipt::Unavailable`) delivered in the current window, keyed by reference.
    pub failed_reads: BTreeMap<String, String>,
}

impl WindowEvidence {
    /// Reconstructs current-window chart delivery strictly from [`results::delivered_originals`].
    pub fn from_records<'a>(
        records: impl IntoIterator<Item = impl Into<ReplayEvent<'a>>>,
        messages: &[ProductMessage],
        workspace: &str,
    ) -> Result<Self, DeliveredCitationError> {
        let mut evidence = Self::default();
        for output in results::delivered_originals(records, messages)? {
            let Ok(receipt) = serde_json::from_str::<ReadReceipt>(output) else {
                continue;
            };
            for item in receipt.results {
                match item {
                    RecordReceipt::Available {
                        reference,
                        revision,
                        source_id,
                        record_type,
                        dates,
                        content,
                        readable_parts,
                        sha256,
                        provenance,
                        native,
                        citation_id: receipt_citation_id,
                        ..
                    } => {
                        if crate::sha256_hex(content.as_bytes()) != sha256 {
                            return Err(DeliveredCitationError::Corrupt { reference });
                        }
                        let source = SourceRevision {
                            workspace: workspace.to_owned(),
                            source_id: source_id
                                .or_else(|| {
                                    provenance
                                        .get("source_id")
                                        .and_then(Value::as_str)
                                        .map(str::to_owned)
                                })
                                .unwrap_or_default(),
                            candidate_id: reference.clone(),
                            revision: revision.clone(),
                            record_type,
                            dates,
                            content,
                            sha256,
                            readable_parts,
                            provenance,
                            native,
                        };
                        evidence.whole.insert(
                            (reference, revision),
                            WholeDelivery {
                                source,
                                receipt_citation_id,
                            },
                        );
                    }
                    RecordReceipt::Passage {
                        reference,
                        revision,
                        part,
                        label,
                        start_byte,
                        end_byte,
                        content,
                        ..
                    } => {
                        if end_byte.saturating_sub(start_byte) != content.len() {
                            return Err(DeliveredCitationError::Corrupt { reference });
                        }
                        let entry = evidence
                            .passages
                            .entry((reference, revision))
                            .or_default()
                            .parts
                            .entry(part)
                            .or_insert_with(|| (label, Vec::new()));
                        insert_merged_span(
                            &mut entry.1,
                            DeliveredSpan {
                                start_byte,
                                end_byte,
                                content,
                            },
                        );
                    }
                    RecordReceipt::Unavailable {
                        reference: Some(reference),
                        reason,
                    } => {
                        evidence.failed_reads.insert(reference, reason);
                    }
                    RecordReceipt::Unavailable {
                        reference: None, ..
                    }
                    | RecordReceipt::OverBudget { .. } => {}
                }
            }
        }
        Ok(evidence)
    }

    /// Whether any content of `(reference, revision)` reached the model in the current window.
    pub fn has_record(&self, reference: &str, revision: &str) -> bool {
        let key = (reference.to_owned(), revision.to_owned());
        self.whole.contains_key(&key) || self.passages.contains_key(&key)
    }

    /// Identities `(reference, revision)` of every record whose content reached the model in this window.
    pub fn read_identities(&self) -> BTreeSet<(String, String)> {
        self.whole
            .keys()
            .chain(self.passages.keys())
            .cloned()
            .collect()
    }
}

fn insert_merged_span(spans: &mut Vec<DeliveredSpan>, incoming: DeliveredSpan) {
    spans.push(incoming);
    spans.sort_unstable_by_key(|span| (span.start_byte, span.end_byte));
    let mut merged: Vec<DeliveredSpan> = Vec::with_capacity(spans.len());
    for span in spans.drain(..) {
        if let Some(prev) = merged.last_mut()
            && span.start_byte <= prev.end_byte
        {
            if span.end_byte > prev.end_byte {
                let overlap = prev.end_byte - span.start_byte;
                if let Some(tail) = span.content.get(overlap..) {
                    prev.content.push_str(tail);
                    prev.end_byte = span.end_byte;
                }
            }
            continue;
        }
        merged.push(span);
    }
    *spans = merged;
}

/// Active model submissions in the current working window, plus handles retired by an earlier window.
#[derive(Clone, Debug, Default)]
pub struct StagedPublications {
    /// Latest preview per `(reference, revision)` submitted in the current window.
    pub previews: BTreeMap<(String, String), PreviewSubmission>,
    /// Declared citations submitted in the current window, in submission order.
    pub citations: Vec<CitationSubmission>,
    /// Citations submitted only before the latest `WindowOpened`, keyed by model handle
    /// (and canonical receipt `citation_id` when known), retained solely so an inline marker
    /// in the new window that still names an earlier-window citation receives a window-specific refusal.
    pub prior_window_citations: BTreeMap<String, CitationSubmission>,
}

impl StagedPublications {
    /// Extracts [`PUBLISH_CITATIONS_TOOL`] submissions scoped to the run's latest working window.
    pub fn from_records<'a>(records: impl IntoIterator<Item = impl Into<ReplayEvent<'a>>>) -> Self {
        let mut staged = Self::default();
        let mut handle_canonicals: BTreeMap<String, String> = BTreeMap::new();
        for event in
            harness::engine::recipe::replay_events(records).filter_map(ReplayEvent::admitted)
        {
            match &event.kind {
                EventKind::WindowOpened { .. } => {
                    for citation in &staged.citations {
                        if let Some(canonical) = handle_canonicals.get(&citation.id) {
                            staged
                                .prior_window_citations
                                .insert(canonical.clone(), citation.clone());
                        }
                    }
                    handle_canonicals.clear();
                    staged.open_fresh_window();
                }
                EventKind::ToolReturned {
                    result: results::ToolResult::Original(output),
                    ..
                } => {
                    let Ok(receipt) = serde_json::from_str::<RecordedCitationsReceipt>(output)
                    else {
                        continue;
                    };
                    if receipt.kind != RECEIPT_KIND {
                        continue;
                    }
                    if receipt.call.replace {
                        handle_canonicals.clear();
                    }
                    for item in &receipt.citations {
                        if let Some(canonical) = &item.citation_id {
                            staged.prior_window_citations.remove(canonical);
                            handle_canonicals.insert(item.id.clone(), canonical.clone());
                        }
                    }
                    staged.apply(receipt.call);
                }
                _ => {}
            }
        }
        staged
    }

    /// Retires the current window's staged previews and citations when a fresh context window opens.
    pub fn open_fresh_window(&mut self) {
        self.previews.clear();
        for citation in self.citations.drain(..) {
            self.prior_window_citations
                .insert(citation.id.clone(), citation);
        }
    }

    /// Applies one [`PublishCitationsCall`] submitted in the current window.
    pub fn apply(&mut self, call: PublishCitationsCall) {
        if call.replace {
            self.previews.clear();
            self.citations.clear();
        }
        for preview in call.previews {
            self.previews.insert(
                (preview.reference.clone(), preview.revision.clone()),
                preview,
            );
        }
        for citation in call.citations {
            self.prior_window_citations.remove(&citation.id);
            if let Some(existing) = self
                .citations
                .iter_mut()
                .find(|saved| saved.id == citation.id)
            {
                *existing = citation;
            } else {
                self.citations.push(citation);
            }
        }
    }
}

/// Collects `(reference, revision)` identities named by raw [`PUBLISH_CITATIONS_TOOL`] arguments and `evidence`.
pub fn call_identities(arguments: &Value, evidence: &WindowEvidence) -> BTreeSet<(String, String)> {
    let mut identities = evidence.read_identities();
    for array_key in ["previews", "citations"] {
        if let Some(items) = arguments.get(array_key).and_then(Value::as_array) {
            for item in items {
                if let (Some(reference), Some(revision)) = (
                    item.get("reference").and_then(Value::as_str),
                    item.get("revision").and_then(Value::as_str),
                ) {
                    let reference = reference.trim();
                    let revision = revision.trim();
                    if !reference.is_empty() && !revision.is_empty() {
                        identities.insert((reference.to_owned(), revision.to_owned()));
                    }
                }
            }
        }
    }
    identities
}

/// Collects `(reference, revision)` identities named across `evidence` and `staged` submissions.
pub fn publication_identities(
    evidence: &WindowEvidence,
    staged: &StagedPublications,
) -> BTreeSet<(String, String)> {
    let mut identities = evidence.read_identities();
    identities.extend(staged.previews.keys().cloned());
    for citation in &staged.citations {
        identities.insert((citation.reference.clone(), citation.revision.clone()));
    }
    identities
}

/// How a job binds citations to its authored output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobCitationMode {
    /// Chat answers embed `[[patient_source:ID]]` inline beside each claim and an optional `[[basis]]` block.
    InlineMarkers,
    /// Tool-finished jobs (such as reply revisions) save the declared citation set alongside plain prose.
    DeclaredSet,
}

/// Validated output of [`publish_job_citations`], ready to persist with the job.
#[derive(Clone, Debug, PartialEq)]
pub struct PublishedJob {
    /// Authored text with `[[basis]]` extracted (in [`JobCitationMode::InlineMarkers`]) and citation handles canonicalized.
    pub text: String,
    /// Extracted `[[basis]]` block when present and marker-free.
    pub basis: Option<String>,
    /// Grounded citations in first-cited or declared order, each carrying its preview and locator.
    pub citations: Vec<Citation>,
    /// Complete per-record preview and failed-read ledger for the finishing window.
    pub source_previews: Vec<ReadRecordPreview>,
}

/// Why a job's citations or previews cannot finish yet.
///
/// Every variant is a correctable model mistake converted to `Finish::Refused` so the model
/// sees the exact reason and corrects it before publishing.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PublicationError {
    /// An inline `[[patient_source:...` marker lacked its closing `]]`.
    #[error(
        "A citation marker is missing its closing ]]. Close every [[patient_source:CITATION_ID]] marker."
    )]
    Unterminated,

    /// An inline marker named an identity not grounded by any whole-record read or `publish_citations` entry in this window.
    #[error(
        "The answer cites `{0}`, which was not received and declared in the current context window. Read the source in this window with read_patient_sources and declare the citation with publish_citations, or remove the citation."
    )]
    NotRead(String),

    /// A staged preview named evidence that was not delivered in the current working window.
    #[error(
        "Preview for `{reference}` (revision `{revision}`) refers to evidence you did not receive in your current context window. Read the record in this window with read_patient_sources, or call publish_citations with replace: true to publish only records read in this window."
    )]
    PreviewNotInWindow {
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
    },

    /// A staged whole-record or structured-field citation named evidence not delivered whole in the current window.
    #[error(
        "Citation `{id}` for `{reference}` (revision `{revision}`) refers to evidence you did not receive whole in your current context window. Read the record in this window with read_patient_sources before citing it, or call publish_citations with replace: true."
    )]
    CitationNotInWindow {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
    },

    /// A staged passage citation named a record or byte span not delivered in the current window.
    #[error(
        "Passage citation `{id}` for `{reference}` (revision `{revision}`, part {part}) refers to text you did not receive in your current context window. Read that passage in this window with read_patient_sources before citing it."
    )]
    PassageNotInWindow {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
        /// Requested part index.
        part: usize,
    },

    /// A preview failed validation (for example `"Unavailable"`, blank, or containing markers).
    #[error("Invalid preview for `{reference}` (revision `{revision}`): {reason}")]
    InvalidPreview {
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
        /// Validation failure reason.
        reason: PreviewError,
    },

    /// A citation lacked a model-written preview.
    #[error(
        "Citation `{id}` for `{reference}` (revision `{revision}`) has no preview. Call publish_citations to write a short dated preview for `{reference}` before finishing."
    )]
    CitationMissingPreview {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
    },

    /// A record read in the current window lacked a model-written preview.
    #[error(
        "Every record read in the current context window requires a preview, and `{reference}` (revision `{revision}`) has none. Call publish_citations to write a short dated preview for `{reference}` before finishing."
    )]
    MissingPreview {
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
    },

    /// Requested part index does not exist on the delivered record.
    #[error(
        "Passage citation `{id}` requested part {part} on `{reference}` (revision `{revision}`), which has only parts 0..={max_part}."
    )]
    PassagePartMissing {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
        /// Requested part index.
        part: usize,
        /// Highest valid part index on the record.
        max_part: usize,
    },

    /// Passage quote was not present in the delivered text (or did not match `start_byte`).
    #[error(
        "Passage citation `{id}` on `{reference}` (revision `{revision}`, part {part}) quotes text that does not appear verbatim in the delivered record text."
    )]
    PassageNotInDeliveredText {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
        /// Requested part index.
        part: usize,
    },

    /// Passage quote appeared more than once in the delivered part without a disambiguating `start_byte`.
    #[error(
        "Passage citation `{id}` on `{reference}` (revision `{revision}`, part {part}) matches multiple locations; supply start_byte or quote a longer unique span."
    )]
    AmbiguousPassageQuote {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
        /// Requested part index.
        part: usize,
    },

    /// Field citation supplied an empty root path instead of a specific field or element.
    #[error(
        "Field citation `{id}` on `{reference}` must name a specific field path rather than the document root."
    )]
    RootFieldPath {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
    },

    /// Field citation named a path absent from the delivered structured JSON record.
    #[error(
        "Field citation `{id}` on `{reference}` (revision `{revision}`) names path `{path}`, which does not exist in the delivered structured record."
    )]
    FieldNotInDeliveredRecord {
        /// Model citation handle.
        id: String,
        /// Source-owned record handle.
        reference: String,
        /// Source revision.
        revision: String,
        /// Canonical path string requested.
        path: String,
    },
}

/// Model-facing definition of [`PUBLISH_CITATIONS_TOOL`], offered to every physician job.
pub fn tool() -> Tool {
    let passage_locator = json!({
        "type": "object",
        "properties": {
            "kind": { "type": "string", "enum": ["passage"] },
            "part": { "type": "integer", "minimum": 0 },
            "quote": { "type": "string" },
            "start_byte": { "type": ["integer", "null"], "minimum": 0 }
        },
        "required": ["kind", "part", "quote"],
        "additionalProperties": false
    });
    let field_locator = json!({
        "type": "object",
        "properties": {
            "kind": { "type": "string", "enum": ["field"] },
            "path": { "type": "string" }
        },
        "required": ["kind", "path"],
        "additionalProperties": false
    });
    Tool {
        name: PUBLISH_CITATIONS_TOOL.into(),
        description: "Record dated previews for every patient record read in the current context window and declare whole-record, passage, or structured-field citations before finishing. Never write an Unavailable preview; unavailable status comes only from failed reads.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "previews": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "reference": { "type": "string" },
                            "revision": { "type": "string" },
                            "preview": { "type": "string" }
                        },
                        "required": ["reference", "revision", "preview"],
                        "additionalProperties": false
                    }
                },
                "citations": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "string" },
                            "reference": { "type": "string" },
                            "revision": { "type": "string" },
                            "locator": {
                                "anyOf": [
                                    { "type": "null" },
                                    passage_locator,
                                    field_locator
                                ]
                            }
                        },
                        "required": ["id", "reference", "revision", "locator"],
                        "additionalProperties": false
                    }
                },
                "replace": { "type": "boolean" }
            },
            "required": ["previews", "citations"],
            "additionalProperties": false
        }),
    }
}

/// Validates and executes [`PUBLISH_CITATIONS_TOOL`] against the current window's delivered reads.
///
/// Malformed arguments, `"Unavailable"` previews, quotes absent from delivered text, and field
/// paths absent from delivered structured records are refused immediately. Declarations naming a
/// record or passage span not yet received in the current window return a warning receipt and are
/// refused at finish if the run attempts to finish without reading that evidence in the current window.
pub fn execute_publish_citations(
    arguments: &Value,
    evidence: &WindowEvidence,
    retained_sources: &[SourceRevision],
) -> ToolOutcome {
    let mut call: PublishCitationsCall = match serde_json::from_value(arguments.clone()) {
        Ok(parsed) => parsed,
        Err(err) => {
            return ToolOutcome::refused(format!(
                "Invalid publish_citations arguments: {err}. Supply previews, citations, and optional replace."
            ));
        }
    };
    if !call.replace && call.previews.is_empty() && call.citations.is_empty() {
        return ToolOutcome::refused(
            "Supply at least one preview or citation, or set replace: true to clear earlier submissions.",
        );
    }

    let mut warnings = Vec::new();
    for entry in &mut call.previews {
        entry.reference = entry.reference.trim().to_owned();
        entry.revision = entry.revision.trim().to_owned();
        if entry.reference.is_empty() || entry.revision.is_empty() {
            return ToolOutcome::refused(
                "Every preview entry requires a non-empty reference and revision.",
            );
        }
        let validated = match PreviewText::parse(&entry.preview) {
            Ok(valid) => valid,
            Err(reason) => return ToolOutcome::refused(reason.to_string()),
        };
        entry.preview = validated.into_inner();

        if !evidence.has_record(&entry.reference, &entry.revision) {
            if evidence.failed_reads.contains_key(&entry.reference) {
                return ToolOutcome::refused(format!(
                    "Read of `{}` failed as unavailable; do not submit a model-written preview for a failed read.",
                    entry.reference
                ));
            }
            warnings.push(format!(
                "`{}` (revision `{}`) has not been delivered in the current context window; read it in this window before finishing.",
                entry.reference, entry.revision
            ));
        }
    }

    let mut seen_ids = BTreeSet::new();
    let mut receipts = Vec::with_capacity(call.citations.len());
    for entry in &mut call.citations {
        entry.id = entry.id.trim().to_owned();
        entry.reference = entry.reference.trim().to_owned();
        entry.revision = entry.revision.trim().to_owned();
        if entry.id.is_empty()
            || entry.id.contains("[[")
            || entry.id.contains("]]")
            || entry.id.chars().any(char::is_whitespace)
        {
            return ToolOutcome::refused(
                "Every citation requires a non-empty id without whitespace or bracket markers.",
            );
        }
        if entry.reference.is_empty() || entry.revision.is_empty() {
            return ToolOutcome::refused(
                "Every citation requires a non-empty reference and revision.",
            );
        }
        if !seen_ids.insert(entry.id.clone()) {
            return ToolOutcome::refused(format!(
                "Duplicate citation id `{}` in publish_citations; give each distinct citation its own id.",
                entry.id
            ));
        }

        let source = find_source(
            &entry.reference,
            &entry.revision,
            evidence,
            retained_sources,
        );
        let key = (entry.reference.clone(), entry.revision.clone());
        let mut resolved_locator: Option<CitationLocator> = None;

        match &entry.locator {
            None => {
                if !evidence.whole.contains_key(&key) {
                    warnings.push(format!(
                        "Citation `{}` for `{}` (revision `{}`) requires the whole record to be delivered in the current context window before finishing.",
                        entry.id, entry.reference, entry.revision
                    ));
                }
            }
            Some(RequestedLocator::Passage {
                part,
                quote,
                start_byte,
            }) => {
                if quote.is_empty() {
                    return ToolOutcome::refused(format!(
                        "Passage citation `{}` requires a non-empty verbatim quote.",
                        entry.id
                    ));
                }
                if let Some(whole) = evidence.whole.get(&key) {
                    let target = source.unwrap_or(&whole.source);
                    match resolve_whole_passage(&entry.id, target, *part, quote, *start_byte) {
                        Ok(locator) => resolved_locator = Some(locator),
                        Err(err) => return ToolOutcome::refused(err.to_string()),
                    }
                } else if let Some(passages) = evidence.passages.get(&key) {
                    match resolve_delivered_passage(
                        &entry.id,
                        &entry.reference,
                        &entry.revision,
                        passages,
                        source,
                        *part,
                        quote,
                        *start_byte,
                    ) {
                        Ok(locator) => resolved_locator = Some(locator),
                        Err(err @ PublicationError::PassageNotInWindow { .. }) => {
                            warnings.push(err.to_string());
                        }
                        Err(err) => return ToolOutcome::refused(err.to_string()),
                    }
                } else {
                    if let Some(retained) = source
                        && let Err(
                            err @ (PublicationError::PassagePartMissing { .. }
                            | PublicationError::PassageNotInDeliveredText { .. }
                            | PublicationError::AmbiguousPassageQuote { .. }),
                        ) = resolve_whole_passage(&entry.id, retained, *part, quote, *start_byte)
                    {
                        return ToolOutcome::refused(err.to_string());
                    }
                    warnings.push(format!(
                        "Passage citation `{}` for `{}` (revision `{}`, part {}) has not been delivered in the current context window.",
                        entry.id, entry.reference, entry.revision, part
                    ));
                }
            }
            Some(RequestedLocator::Field { path }) => {
                if path.is_root() {
                    return ToolOutcome::refused(format!(
                        "Field citation `{}` on `{}` must name a specific field path rather than the document root.",
                        entry.id, entry.reference
                    ));
                }
                if let Some(whole) = evidence.whole.get(&key) {
                    let target = source.unwrap_or(&whole.source);
                    match resolve_field_locator(&entry.id, target, path) {
                        Ok(locator) => resolved_locator = Some(locator),
                        Err(err) => return ToolOutcome::refused(err.to_string()),
                    }
                } else {
                    if let Some(retained) = source
                        && let Err(err) = resolve_field_locator(&entry.id, retained, path)
                    {
                        return ToolOutcome::refused(err.to_string());
                    }
                    warnings.push(format!(
                        "Field citation `{}` for `{}` (revision `{}`) requires the structured record to be delivered whole in the current context window before finishing.",
                        entry.id, entry.reference, entry.revision
                    ));
                }
            }
        }

        let canonical_id = source
            .map(|src| match (&entry.locator, resolved_locator.as_ref()) {
                (None, _) => canonical_citation_id(src, None, evidence, retained_sources),
                (Some(_), Some(resolved)) => {
                    canonical_citation_id(src, Some(resolved), evidence, retained_sources)
                }
                (Some(_), None) => String::new(),
            })
            .filter(|id| !id.is_empty());

        receipts.push(CitationHandleReceipt {
            id: entry.id.clone(),
            reference: entry.reference.clone(),
            revision: entry.revision.clone(),
            citation_id: canonical_id,
        });
    }

    let receipt = RecordedCitationsReceipt {
        kind: RECEIPT_KIND.to_owned(),
        recorded_previews: call.previews.len(),
        call,
        citations: receipts,
        warnings,
    };
    ToolOutcome::Returned(serde_json::to_string(&receipt).expect("citation receipt serializes"))
}

/// Validates all previews, citations, passages, and structured field paths at finish time against
/// `evidence` (the exact bytes delivered in the run's latest working window).
pub fn publish_job_citations(
    text: &str,
    mode: JobCitationMode,
    evidence: &WindowEvidence,
    staged: &StagedPublications,
    retained_sources: &[SourceRevision],
) -> Result<PublishedJob, PublicationError> {
    let mut parsed_previews: BTreeMap<(String, String), PreviewText> = BTreeMap::new();
    for ((reference, revision), submission) in &staged.previews {
        let parsed = PreviewText::parse(&submission.preview).map_err(|reason| {
            PublicationError::InvalidPreview {
                reference: reference.clone(),
                revision: revision.clone(),
                reason,
            }
        })?;
        parsed_previews.insert((reference.clone(), revision.clone()), parsed);
    }

    let mut resolved_declarations: Vec<(String, Citation)> =
        Vec::with_capacity(staged.citations.len());
    for submission in &staged.citations {
        let citation =
            resolve_citation_submission(submission, evidence, retained_sources, &parsed_previews)?;
        resolved_declarations.push((submission.id.clone(), citation));
    }

    let (visible, basis, citations) = match mode {
        JobCitationMode::InlineMarkers => {
            let (visible, basis) = split_basis(text);
            let (rewritten, cited) = resolve_inline_markers(
                &visible,
                evidence,
                retained_sources,
                &parsed_previews,
                &resolved_declarations,
                &staged.prior_window_citations,
            )?;
            (rewritten, basis, cited)
        }
        JobCitationMode::DeclaredSet => {
            let mut cited: Vec<Citation> = Vec::new();
            for (_, citation) in resolved_declarations {
                if !cited.iter().any(|existing| existing.id == citation.id) {
                    cited.push(citation);
                }
            }
            (text.to_owned(), None, cited)
        }
    };

    for (reference, revision) in parsed_previews.keys() {
        if !evidence.has_record(reference, revision) {
            return Err(PublicationError::PreviewNotInWindow {
                reference: reference.clone(),
                revision: revision.clone(),
            });
        }
    }

    let mut source_previews = Vec::new();
    let mut read_references = BTreeSet::new();
    for (reference, revision) in evidence.read_identities() {
        let Some(preview) = parsed_previews.get(&(reference.clone(), revision.clone())) else {
            return Err(PublicationError::MissingPreview {
                reference,
                revision,
            });
        };
        read_references.insert(reference.clone());
        let source = find_source(&reference, &revision, evidence, retained_sources);
        source_previews.push(ReadRecordPreview::Preview {
            reference,
            revision,
            preview: preview.as_str().to_owned(),
            dates: source.map(|s| s.dates.clone()).unwrap_or_default(),
            native: source.and_then(|s| s.native.clone()),
        });
    }
    for (reference, reason) in &evidence.failed_reads {
        if !read_references.contains(reference) {
            source_previews.push(ReadRecordPreview::Unavailable {
                reference: reference.clone(),
                reason: reason.clone(),
            });
        }
    }

    Ok(PublishedJob {
        text: visible,
        basis,
        citations,
        source_previews,
    })
}

/// Retains a partial chat turn on a run limit without refusing: keeps only citations that are
/// grounded in the current window and already carry a valid preview, stripping unsupported markers.
pub fn publish_partial_chat(
    text: &str,
    evidence: &WindowEvidence,
    staged: &StagedPublications,
    retained_sources: &[SourceRevision],
) -> PublishedJob {
    let mut validated_previews: BTreeMap<(String, String), PreviewText> = BTreeMap::new();
    for ((reference, revision), submission) in &staged.previews {
        if evidence.has_record(reference, revision)
            && let Ok(parsed) = PreviewText::parse(&submission.preview)
        {
            validated_previews.insert((reference.clone(), revision.clone()), parsed);
        }
    }

    let mut resolved_declarations: Vec<(String, Citation)> = Vec::new();
    for submission in &staged.citations {
        if let Ok(citation) =
            resolve_citation_submission(submission, evidence, retained_sources, &validated_previews)
        {
            resolved_declarations.push((submission.id.clone(), citation));
        }
    }

    let (visible, basis) = split_basis(text);
    let mut kept = String::with_capacity(visible.len());
    let mut cited: Vec<Citation> = Vec::new();
    let mut remaining = visible.as_str();
    while let Some((before, suffix)) = remaining.split_once(CITATION_OPEN) {
        kept.push_str(before);
        let Some((token, rest)) = suffix.split_once("]]") else {
            remaining = suffix;
            break;
        };
        if let Ok(citation) = lookup_marker_citation(
            token,
            evidence,
            retained_sources,
            &validated_previews,
            &resolved_declarations,
            &staged.prior_window_citations,
        ) {
            kept.push_str(CITATION_OPEN);
            kept.push_str(&citation.id);
            kept.push_str("]]");
            if !cited.iter().any(|existing| existing.id == citation.id) {
                cited.push(citation);
            }
        }
        remaining = rest;
    }
    kept.push_str(remaining);

    let mut source_previews = Vec::new();
    let mut read_references = BTreeSet::new();
    for (reference, revision) in evidence.read_identities() {
        read_references.insert(reference.clone());
        if let Some(preview) = validated_previews.get(&(reference.clone(), revision.clone())) {
            let source = find_source(&reference, &revision, evidence, retained_sources);
            source_previews.push(ReadRecordPreview::Preview {
                reference,
                revision,
                preview: preview.as_str().to_owned(),
                dates: source.map(|s| s.dates.clone()).unwrap_or_default(),
                native: source.and_then(|s| s.native.clone()),
            });
        }
    }
    for (reference, reason) in &evidence.failed_reads {
        if !read_references.contains(reference) {
            source_previews.push(ReadRecordPreview::Unavailable {
                reference: reference.clone(),
                reason: reason.clone(),
            });
        }
    }

    PublishedJob {
        text: kept,
        basis,
        citations: cited,
        source_previews,
    }
}

fn find_source<'a>(
    reference: &str,
    revision: &str,
    evidence: &'a WindowEvidence,
    retained_sources: &'a [SourceRevision],
) -> Option<&'a SourceRevision> {
    if let Some(retained) = retained_sources
        .iter()
        .find(|s| s.candidate_id == reference && s.revision == revision)
    {
        return Some(retained);
    }
    evidence
        .whole
        .get(&(reference.to_owned(), revision.to_owned()))
        .map(|whole| &whole.source)
}

fn canonical_citation_id(
    source: &SourceRevision,
    locator: Option<&CitationLocator>,
    evidence: &WindowEvidence,
    retained_sources: &[SourceRevision],
) -> String {
    if locator.is_none() {
        if retained_sources
            .iter()
            .any(|s| s.candidate_id == source.candidate_id && s.revision == source.revision)
        {
            return citation_id(source);
        }
        if let Some(Some(receipt_id)) = evidence
            .whole
            .get(&(source.candidate_id.clone(), source.revision.clone()))
            .map(|whole| whole.receipt_citation_id.as_deref())
        {
            return receipt_id.to_owned();
        }
        return citation_id(source);
    }
    citation_id_for(source, locator)
}

fn resolve_citation_submission(
    submission: &CitationSubmission,
    evidence: &WindowEvidence,
    retained_sources: &[SourceRevision],
    validated_previews: &BTreeMap<(String, String), PreviewText>,
) -> Result<Citation, PublicationError> {
    let key = (submission.reference.clone(), submission.revision.clone());
    let (source, locator) = match &submission.locator {
        None => {
            let Some(whole) = evidence.whole.get(&key) else {
                return Err(PublicationError::CitationNotInWindow {
                    id: submission.id.clone(),
                    reference: submission.reference.clone(),
                    revision: submission.revision.clone(),
                });
            };
            let source = find_source(
                &submission.reference,
                &submission.revision,
                evidence,
                retained_sources,
            )
            .unwrap_or(&whole.source);
            (source, None)
        }
        Some(RequestedLocator::Passage {
            part,
            quote,
            start_byte,
        }) => {
            if let Some(whole) = evidence.whole.get(&key) {
                let source = find_source(
                    &submission.reference,
                    &submission.revision,
                    evidence,
                    retained_sources,
                )
                .unwrap_or(&whole.source);
                let locator =
                    resolve_whole_passage(&submission.id, source, *part, quote, *start_byte)?;
                (source, Some(locator))
            } else if let Some(passages) = evidence.passages.get(&key) {
                let Some(retained) = retained_sources.iter().find(|s| {
                    s.candidate_id == submission.reference && s.revision == submission.revision
                }) else {
                    return Err(PublicationError::PassageNotInWindow {
                        id: submission.id.clone(),
                        reference: submission.reference.clone(),
                        revision: submission.revision.clone(),
                        part: *part,
                    });
                };
                let locator = resolve_delivered_passage(
                    &submission.id,
                    &submission.reference,
                    &submission.revision,
                    passages,
                    Some(retained),
                    *part,
                    quote,
                    *start_byte,
                )?;
                (retained, Some(locator))
            } else {
                return Err(PublicationError::PassageNotInWindow {
                    id: submission.id.clone(),
                    reference: submission.reference.clone(),
                    revision: submission.revision.clone(),
                    part: *part,
                });
            }
        }
        Some(RequestedLocator::Field { path }) => {
            if path.is_root() {
                return Err(PublicationError::RootFieldPath {
                    id: submission.id.clone(),
                    reference: submission.reference.clone(),
                });
            }
            let Some(whole) = evidence.whole.get(&key) else {
                return Err(PublicationError::CitationNotInWindow {
                    id: submission.id.clone(),
                    reference: submission.reference.clone(),
                    revision: submission.revision.clone(),
                });
            };
            let source = find_source(
                &submission.reference,
                &submission.revision,
                evidence,
                retained_sources,
            )
            .unwrap_or(&whole.source);
            let locator = resolve_field_locator(&submission.id, source, path)?;
            (source, Some(locator))
        }
    };

    let Some(preview) = validated_previews.get(&key) else {
        return Err(PublicationError::CitationMissingPreview {
            id: submission.id.clone(),
            reference: submission.reference.clone(),
            revision: submission.revision.clone(),
        });
    };

    Ok(build_citation(
        source,
        preview,
        locator,
        evidence,
        retained_sources,
    ))
}

fn build_citation(
    source: &SourceRevision,
    preview: &PreviewText,
    locator: Option<CitationLocator>,
    evidence: &WindowEvidence,
    retained_sources: &[SourceRevision],
) -> Citation {
    let id = canonical_citation_id(source, locator.as_ref(), evidence, retained_sources);
    Citation {
        id,
        reference: source.candidate_id.clone(),
        revision: source.revision.clone(),
        source_id: (!source.source_id.is_empty()).then(|| source.source_id.clone()),
        record_type: source.record_type.clone(),
        dates: source.dates.clone(),
        content: source.content.clone(),
        readable_parts: source.readable_parts.clone(),
        sha256: source.sha256.clone(),
        provenance: source.provenance.clone(),
        summary: Some(preview.as_str().to_owned()),
        native: source.native.clone(),
        locator,
    }
}

fn cited_part(part: usize, label: &str) -> CitedPart {
    if part == 0 {
        CitedPart::NativeRecord
    } else {
        CitedPart::ReadablePart {
            index: part - 1,
            label: label.to_owned(),
        }
    }
}

fn resolve_whole_passage(
    id: &str,
    source: &SourceRevision,
    part: usize,
    quote: &str,
    start_byte: Option<usize>,
) -> Result<CitationLocator, PublicationError> {
    if quote.is_empty() {
        return Err(PublicationError::PassageNotInDeliveredText {
            id: id.to_owned(),
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            part,
        });
    }
    let Some((label, full_text)) = source_part(source, part) else {
        return Err(PublicationError::PassagePartMissing {
            id: id.to_owned(),
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            part,
            max_part: source.readable_parts.len(),
        });
    };
    let (resolved_start, resolved_end) = match_quote_in_span(
        id,
        &source.candidate_id,
        &source.revision,
        part,
        0,
        full_text,
        quote,
        start_byte,
    )?;
    Ok(CitationLocator::Passage {
        part: cited_part(part, label),
        start_byte: resolved_start,
        end_byte: resolved_end,
        quote: quote.to_owned(),
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "passage verification binds the citation identity, record identity, delivered spans, and quote together"
)]
fn resolve_delivered_passage(
    id: &str,
    reference: &str,
    revision: &str,
    passages: &DeliveredPassages,
    retained: Option<&SourceRevision>,
    part: usize,
    quote: &str,
    start_byte: Option<usize>,
) -> Result<CitationLocator, PublicationError> {
    if quote.is_empty() {
        return Err(PublicationError::PassageNotInDeliveredText {
            id: id.to_owned(),
            reference: reference.to_owned(),
            revision: revision.to_owned(),
            part,
        });
    }
    if let Some(source) = retained
        && source_part(source, part).is_none()
    {
        return Err(PublicationError::PassagePartMissing {
            id: id.to_owned(),
            reference: reference.to_owned(),
            revision: revision.to_owned(),
            part,
            max_part: source.readable_parts.len(),
        });
    }
    let Some((label, spans)) = passages.parts.get(&part) else {
        return Err(PublicationError::PassageNotInWindow {
            id: id.to_owned(),
            reference: reference.to_owned(),
            revision: revision.to_owned(),
            part,
        });
    };

    let mut matches = Vec::new();
    for span in spans {
        if let Some(requested_start) = start_byte {
            if requested_start >= span.start_byte {
                let local_start = requested_start - span.start_byte;
                let local_end = local_start.saturating_add(quote.len());
                if span.content.get(local_start..local_end) == Some(quote) {
                    matches.push((requested_start, requested_start + quote.len()));
                }
            }
        } else {
            for (local_offset, _) in span.content.match_indices(quote) {
                let abs_start = span.start_byte + local_offset;
                matches.push((abs_start, abs_start + quote.len()));
            }
        }
    }

    match matches.len() {
        1 => {
            let (resolved_start, resolved_end) = matches[0];
            if let Some(source) = retained {
                let Some((_, full_text)) = source_part(source, part) else {
                    return Err(PublicationError::PassagePartMissing {
                        id: id.to_owned(),
                        reference: reference.to_owned(),
                        revision: revision.to_owned(),
                        part,
                        max_part: source.readable_parts.len(),
                    });
                };
                if full_text.get(resolved_start..resolved_end) != Some(quote) {
                    return Err(PublicationError::PassageNotInDeliveredText {
                        id: id.to_owned(),
                        reference: reference.to_owned(),
                        revision: revision.to_owned(),
                        part,
                    });
                }
            }
            Ok(CitationLocator::Passage {
                part: cited_part(part, label),
                start_byte: resolved_start,
                end_byte: resolved_end,
                quote: quote.to_owned(),
            })
        }
        0 => {
            if let Some(source) = retained
                && let Some((_, full_text)) = source_part(source, part)
                && match start_byte {
                    Some(req) => full_text.get(req..req.saturating_add(quote.len())) == Some(quote),
                    None => full_text.contains(quote),
                }
            {
                return Err(PublicationError::PassageNotInWindow {
                    id: id.to_owned(),
                    reference: reference.to_owned(),
                    revision: revision.to_owned(),
                    part,
                });
            }
            Err(PublicationError::PassageNotInDeliveredText {
                id: id.to_owned(),
                reference: reference.to_owned(),
                revision: revision.to_owned(),
                part,
            })
        }
        _ => Err(PublicationError::AmbiguousPassageQuote {
            id: id.to_owned(),
            reference: reference.to_owned(),
            revision: revision.to_owned(),
            part,
        }),
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "span matching reports precise record/part context on refusal"
)]
fn match_quote_in_span(
    id: &str,
    reference: &str,
    revision: &str,
    part: usize,
    span_offset: usize,
    text: &str,
    quote: &str,
    start_byte: Option<usize>,
) -> Result<(usize, usize), PublicationError> {
    if let Some(requested_start) = start_byte {
        if requested_start < span_offset {
            return Err(PublicationError::PassageNotInDeliveredText {
                id: id.to_owned(),
                reference: reference.to_owned(),
                revision: revision.to_owned(),
                part,
            });
        }
        let local_start = requested_start - span_offset;
        let local_end = local_start.saturating_add(quote.len());
        if text.get(local_start..local_end) == Some(quote) {
            return Ok((requested_start, requested_start + quote.len()));
        }
        return Err(PublicationError::PassageNotInDeliveredText {
            id: id.to_owned(),
            reference: reference.to_owned(),
            revision: revision.to_owned(),
            part,
        });
    }
    let mut iter = text.match_indices(quote);
    let Some((first, _)) = iter.next() else {
        return Err(PublicationError::PassageNotInDeliveredText {
            id: id.to_owned(),
            reference: reference.to_owned(),
            revision: revision.to_owned(),
            part,
        });
    };
    if iter.next().is_some() {
        return Err(PublicationError::AmbiguousPassageQuote {
            id: id.to_owned(),
            reference: reference.to_owned(),
            revision: revision.to_owned(),
            part,
        });
    }
    let abs_start = span_offset + first;
    Ok((abs_start, abs_start + quote.len()))
}

fn resolve_field_locator(
    id: &str,
    source: &SourceRevision,
    path: &SourcePath,
) -> Result<CitationLocator, PublicationError> {
    let Ok(parsed) = serde_json::from_str::<Value>(&source.content) else {
        return Err(PublicationError::FieldNotInDeliveredRecord {
            id: id.to_owned(),
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            path: path.to_string(),
        });
    };
    if !matches!(parsed, Value::Object(_) | Value::Array(_)) || path.resolve(&parsed).is_none() {
        return Err(PublicationError::FieldNotInDeliveredRecord {
            id: id.to_owned(),
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            path: path.to_string(),
        });
    }
    Ok(CitationLocator::Field { path: path.clone() })
}

fn resolve_inline_markers(
    visible: &str,
    evidence: &WindowEvidence,
    retained_sources: &[SourceRevision],
    validated_previews: &BTreeMap<(String, String), PreviewText>,
    resolved_declarations: &[(String, Citation)],
    prior_window_citations: &BTreeMap<String, CitationSubmission>,
) -> Result<(String, Vec<Citation>), PublicationError> {
    let mut rewritten = String::with_capacity(visible.len());
    let mut cited: Vec<Citation> = Vec::new();
    let mut remaining = visible;
    while let Some((before, suffix)) = remaining.split_once(CITATION_OPEN) {
        rewritten.push_str(before);
        let (token, rest) = suffix
            .split_once("]]")
            .ok_or(PublicationError::Unterminated)?;
        let citation = lookup_marker_citation(
            token,
            evidence,
            retained_sources,
            validated_previews,
            resolved_declarations,
            prior_window_citations,
        )?;
        rewritten.push_str(CITATION_OPEN);
        rewritten.push_str(&citation.id);
        rewritten.push_str("]]");
        if !cited.iter().any(|existing| existing.id == citation.id) {
            cited.push(citation);
        }
        remaining = rest;
    }
    rewritten.push_str(remaining);
    Ok((rewritten, cited))
}

fn lookup_marker_citation(
    token: &str,
    evidence: &WindowEvidence,
    retained_sources: &[SourceRevision],
    validated_previews: &BTreeMap<(String, String), PreviewText>,
    resolved_declarations: &[(String, Citation)],
    prior_window_citations: &BTreeMap<String, CitationSubmission>,
) -> Result<Citation, PublicationError> {
    if let Some((_, declared)) = resolved_declarations
        .iter()
        .find(|(handle, citation)| handle == token || citation.id == token)
    {
        return Ok(declared.clone());
    }
    if let Some(whole) = evidence.whole.values().find(|whole| {
        whole.receipt_citation_id.as_deref() == Some(token)
            || canonical_citation_id(&whole.source, None, evidence, retained_sources) == token
    }) {
        let key = (
            whole.source.candidate_id.clone(),
            whole.source.revision.clone(),
        );
        let Some(preview) = validated_previews.get(&key) else {
            return Err(PublicationError::CitationMissingPreview {
                id: token.to_owned(),
                reference: whole.source.candidate_id.clone(),
                revision: whole.source.revision.clone(),
            });
        };
        let source = find_source(
            &whole.source.candidate_id,
            &whole.source.revision,
            evidence,
            retained_sources,
        )
        .unwrap_or(&whole.source);
        return Ok(build_citation(
            source,
            preview,
            None,
            evidence,
            retained_sources,
        ));
    }
    if let Some(prior) = prior_window_citations.get(token) {
        return Err(refuse_prior_window_marker(
            token,
            prior,
            evidence,
            retained_sources,
            validated_previews,
        ));
    }
    Err(PublicationError::NotRead(token.to_owned()))
}

fn refuse_prior_window_marker(
    token: &str,
    prior: &CitationSubmission,
    evidence: &WindowEvidence,
    retained_sources: &[SourceRevision],
    validated_previews: &BTreeMap<(String, String), PreviewText>,
) -> PublicationError {
    let key = (prior.reference.clone(), prior.revision.clone());
    match &prior.locator {
        Some(RequestedLocator::Passage {
            part,
            quote,
            start_byte,
        }) => {
            let delivered_in_window = if evidence.whole.contains_key(&key) {
                true
            } else if let Some(passages) = evidence.passages.get(&key) {
                let retained = find_source(
                    &prior.reference,
                    &prior.revision,
                    evidence,
                    retained_sources,
                );
                !matches!(
                    resolve_delivered_passage(
                        &prior.id,
                        &prior.reference,
                        &prior.revision,
                        passages,
                        retained,
                        *part,
                        quote,
                        *start_byte,
                    ),
                    Err(PublicationError::PassageNotInWindow { .. })
                )
            } else {
                false
            };
            if !delivered_in_window {
                return PublicationError::PassageNotInWindow {
                    id: prior.id.clone(),
                    reference: prior.reference.clone(),
                    revision: prior.revision.clone(),
                    part: *part,
                };
            }
        }
        None | Some(RequestedLocator::Field { .. }) => {
            if !evidence.whole.contains_key(&key) {
                return PublicationError::CitationNotInWindow {
                    id: prior.id.clone(),
                    reference: prior.reference.clone(),
                    revision: prior.revision.clone(),
                };
            }
        }
    }
    if !validated_previews.contains_key(&key) {
        return PublicationError::CitationMissingPreview {
            id: prior.id.clone(),
            reference: prior.reference.clone(),
            revision: prior.revision.clone(),
        };
    }
    PublicationError::NotRead(token.to_owned())
}

/// Removes the closed `[[basis]]` block. A basis that is blank or carries a
/// marker is dropped, so a citation never hides in the collapsed section.
pub(crate) fn split_basis(text: &str) -> (String, Option<String>) {
    let (visible, basis) = split_block(text, BASIS_OPEN, BASIS_CLOSE);
    let basis = Some(basis.trim())
        .filter(|basis| !basis.is_empty() && !basis.contains("[["))
        .map(str::to_owned);
    (visible, basis)
}

fn split_block<'a>(text: &'a str, open: &str, close: &str) -> (String, &'a str) {
    let Some((before, after)) = text.rsplit_once(open) else {
        return (text.to_owned(), "");
    };
    let Some((block, tail)) = after.split_once(close) else {
        return (text.to_owned(), "");
    };
    let head = before.trim_end();
    let tail = tail.trim();
    let visible = match (head.is_empty(), tail.is_empty()) {
        (_, true) => head.to_owned(),
        (true, false) => tail.to_owned(),
        (false, false) => format!("{head}\n\n{tail}"),
    };
    if visible.trim().is_empty() {
        return (text.to_owned(), "");
    }
    (visible, block)
}
