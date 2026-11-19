//! On-demand discovery and bounded reads of the physician's workspace.
use super::{
    Bucket, DateRole, FailedPart, FiledDate, READ_TOOL, ReadablePart, RecordCode, RecordDate,
    RecordVersion, SourceRecordKind, SourceRevision, UnknownBucketError,
};
use crate::physician_chat::{CallEnding, StepError};
use harness::engine::results::ToolResult;
use harness::engine::{Event, EventKind, Tool, ToolOutcome};
use physician_wire::{Found, RecordAbout, RecordGroup, RecordRead, SourceVersion, Step, StepState};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Discovery in the retained chart. Results are pointers, never read receipts.
pub const SEARCH_TOOL: &str = "search_patient_information";
const SEARCH_PAGE: usize = 20;
const MAX_PAGE: usize = 65536;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct After {
    version: u64,
    candidate_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct SearchWire {
    #[serde(default)]
    query: String,
    #[serde(default)]
    bucket: Option<String>,
    #[serde(default)]
    record_type: Option<String>,
    #[serde(default)]
    after: Option<After>,
}

#[derive(Debug, Clone)]
struct Search {
    query: String,
    bucket: Option<Bucket>,
    record_type: Option<String>,
    after: Option<After>,
}

/// The position of the next UTF-8 byte in one immutable representation.
/// Part zero is the native record. The decoded texts follow in source order.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Position {
    part: usize,
    offset: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Exact {
    pub(super) candidate_id: String,
    pub(super) revision: String,
    cursor: Option<Position>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Reference {
    Pinned(String),
    Exact(Exact),
}

impl Reference {
    fn id(&self) -> &str {
        match self {
            Self::Pinned(id) => id,
            Self::Exact(reference) => &reference.candidate_id,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Read {
    references: Vec<Reference>,
    max_result_bytes: usize,
}

/// One page of search results, as the search returns it and its step reads it.
#[derive(Serialize, Deserialize)]
struct SearchPage {
    results: Vec<Value>,
    next: Option<After>,
}

/// Dates carried on a discovery hit, preserving declared roles when filed through
/// an adapter table and raw date pairs for un-enveloped revisions.
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum SearchHitDates<'a> {
    Filed(&'a [FiledDate]),
    Raw(&'a [RecordDate]),
}

fn codes_is_empty(codes: &&[RecordCode]) -> bool {
    codes.is_empty()
}

#[derive(Debug, Serialize)]
struct SearchHit<'a> {
    reference: Exact,
    record_type: &'a str,
    source_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<Bucket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_kind: Option<&'a SourceRecordKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a RecordVersion>,
    dates: SearchHitDates<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<&'a str>,
    #[serde(skip_serializing_if = "codes_is_empty")]
    codes: &'a [RecordCode],
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
}

/// Dates carried on a read or discovery receipt, round-tripping both filed
/// role-bearing dates and un-enveloped date pairs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum ReceiptDates {
    Filed(Vec<FiledDate>),
    Raw(Vec<RecordDate>),
}

impl ReceiptDates {
    pub(super) fn into_record_dates(self) -> Vec<RecordDate> {
        match self {
            Self::Filed(filed) => filed
                .into_iter()
                .map(|d| RecordDate {
                    name: d.field,
                    value: d.value,
                })
                .collect(),
            Self::Raw(raw) => raw,
        }
    }
}

/// Deserialized discovery pointer returned by [`SEARCH_TOOL`].
#[derive(Debug, Clone, Deserialize)]
pub(super) struct SearchReceiptHit {
    pub(super) reference: Exact,
    pub(super) record_type: String,
    pub(super) source_id: String,
    pub(super) dates: ReceiptDates,
}

/// Chart discovery receipts have one wire owner for production and delivery checks.
#[derive(Debug, Deserialize)]
pub(super) struct SearchReceipt {
    pub(super) results: Vec<SearchReceiptHit>,
}

/// A label such as `S3`, `L1`, or `D1`: the short name of one version in this work.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(super) struct Label(pub(super) String);

/// The labels of one work. The Nth distinct version a chart read asks for is `S{N}`,
/// the Nth clinical library passage read is `L{N}`, and the Nth document page read is
/// `D{N}`. Reads ask in their saved `ToolStarted` order, which is fixed before any
/// read runs, so reads that run together agree. A label names its version for a
/// citation only once a read of this work returned that version.
pub struct Labels {
    chart: Vec<Version>,
    library: Vec<LibraryReadLabel>,
    document_pages: Vec<DocumentPageReadLabel>,
}

/// One version a chart read of this work asked for.
struct Version {
    candidate_id: String,
    revision: String,
    /// Whether a read's own result holds a record or passage that carries
    /// this version's label.
    returned: bool,
}

/// One clinical library passage a read of this work asked for.
struct LibraryReadLabel {
    passage_id: String,
    revision: String,
    returned: bool,
}

/// One document page a read of this work asked for.
struct DocumentPageReadLabel {
    document_id: String,
    requested_version: Option<u64>,
    page_number: u32,
    revision: String,
    returned: bool,
}

impl Labels {
    /// The labels of the work whose records these are, in any window.
    pub fn of_work<'a>(records: impl IntoIterator<Item = &'a (Event, bool)>) -> Self {
        let mut labels = Self {
            chart: Vec::new(),
            library: Vec::new(),
            document_pages: Vec::new(),
        };
        for (event, _) in records {
            if let EventKind::ToolStarted(call) = &event.kind {
                labels.ask(&call.name, &call.arguments);
            } else if let EventKind::ToolReturned { result, .. } = &event.kind {
                labels.return_labels(result);
            }
        }
        labels
    }

    /// Returns the assigned `L{N}` citation label for a `read_clinical_library`
    /// call whose `ToolStarted` was recorded in this work.
    pub(crate) fn library_label(&self, arguments: &Value) -> Option<String> {
        let Ok(crate::clinical_library::ClinicalLibraryRequest::Read { passage_id }) =
            crate::clinical_library::ClinicalLibraryRequest::parse(
                crate::clinical_library::READ_LIBRARY_TOOL,
                arguments,
            )
        else {
            return None;
        };
        self.library
            .iter()
            .position(|known| known.passage_id == passage_id.as_str())
            .map(|idx| format!("L{}", idx + 1))
    }

    /// Returns the assigned `D{N}` citation label for a `read_document_page`
    /// call whose `ToolStarted` was recorded in this work.
    pub(crate) fn document_page_label(&self, arguments: &Value) -> Option<String> {
        let doc_id = arguments.get("document_id").and_then(Value::as_str)?.trim();
        let requested_version = arguments.get("version").and_then(Value::as_u64);
        let page_number =
            u32::try_from(arguments.get("page_number").and_then(Value::as_u64)?).ok()?;
        if doc_id.is_empty() || page_number == 0 {
            return None;
        }
        self.document_pages
            .iter()
            .position(|known| {
                known.document_id == doc_id
                    && known.requested_version == requested_version
                    && known.page_number == page_number
            })
            .map(|idx| format!("D{}", idx + 1))
    }

    /// Adds each version a chart, clinical library, or document-page read asks for
    /// that no earlier read asked for.
    fn ask(&mut self, name: &str, arguments: &str) {
        let Ok(arguments) = serde_json::from_str::<Value>(arguments) else {
            return;
        };
        if name == crate::clinical_library::READ_LIBRARY_TOOL {
            if let Ok(crate::clinical_library::ClinicalLibraryRequest::Read { passage_id }) =
                crate::clinical_library::ClinicalLibraryRequest::parse(name, &arguments)
                && self.library_label(&arguments).is_none()
            {
                self.library.push(LibraryReadLabel {
                    passage_id: passage_id.as_str().to_owned(),
                    revision: String::new(),
                    returned: false,
                });
            }
            return;
        }
        if name == crate::attachments::READ_DOCUMENT_PAGE_TOOL {
            if let Some(doc_id) = arguments.get("document_id").and_then(Value::as_str)
                && let Some(page_num_u64) = arguments.get("page_number").and_then(Value::as_u64)
                && let Ok(page_number) = u32::try_from(page_num_u64)
            {
                let doc_id = doc_id.trim();
                let requested_version = arguments.get("version").and_then(Value::as_u64);
                if !doc_id.is_empty()
                    && page_number > 0
                    && self.document_page_label(&arguments).is_none()
                {
                    self.document_pages.push(DocumentPageReadLabel {
                        document_id: doc_id.to_owned(),
                        requested_version,
                        page_number,
                        revision: String::new(),
                        returned: false,
                    });
                }
            }
            return;
        }
        let Ok(read) = ChartRequest::parse(name, &arguments, ChartScope::Chat) else {
            return;
        };
        for (candidate_id, revision) in read.versions() {
            if self.of(candidate_id, revision).is_none() {
                self.chart.push(Version {
                    candidate_id: candidate_id.to_owned(),
                    revision: revision.to_owned(),
                    returned: false,
                });
            }
        }
    }

    /// Marks returned each version whose label a record, passage, library read, or
    /// document page read carries. Only a result's own original counts.
    fn return_labels(&mut self, result: &ToolResult) {
        let Some(original) = result.original() else {
            return;
        };
        if let Ok(receipt) = serde_json::from_str::<ReadReceipt>(original) {
            for record in &receipt.results {
                let label = match record {
                    RecordReceipt::Available {
                        cite_as: Some(label),
                        ..
                    }
                    | RecordReceipt::Passage {
                        cite_as: Some(label),
                        ..
                    } => label,
                    RecordReceipt::Available { cite_as: None, .. }
                    | RecordReceipt::Passage { cite_as: None, .. }
                    | RecordReceipt::Unavailable { .. }
                    | RecordReceipt::OverBudget { .. } => continue,
                };
                if let Some(index) = self.chart_index(label) {
                    self.chart[index].returned = true;
                }
            }
            return;
        }
        if let Ok(crate::clinical_library::LibraryReadOutcome::Passage {
            passage,
            cite_as: Some(label),
        }) = serde_json::from_str::<crate::clinical_library::LibraryReadOutcome>(original)
        {
            if let Some(idx) = prefixed_index(&label, 'L', self.library.len())
                && let Some(entry) = self.library.get_mut(idx)
                && entry.passage_id == passage.passage_id.as_str()
            {
                entry.revision = passage.content_sha256;
                entry.returned = true;
            }
            return;
        }
        if let Ok(val) = serde_json::from_str::<Value>(original)
            && val.get("outcome").and_then(Value::as_str) == Some("available")
            && let Some(label) = val.get("cite_as").and_then(Value::as_str)
            && let Some(doc_id) = val.get("document_id").and_then(Value::as_str)
            && let Some(page_number) = val
                .get("page_number")
                .and_then(Value::as_u64)
                .and_then(|n| u32::try_from(n).ok())
            && let Some(idx) = prefixed_index(label, 'D', self.document_pages.len())
            && let Some(entry) = self.document_pages.get_mut(idx)
            && entry.document_id == doc_id
            && entry.page_number == page_number
        {
            let base_rev = val
                .get("revision")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .or_else(|| {
                    val.get("version")
                        .and_then(Value::as_u64)
                        .map(|v| format!("v{v}"))
                })
                .unwrap_or_else(|| "v1".to_owned());
            entry.revision = format!("{base_rev}:p{page_number}");
            entry.returned = true;
        }
    }

    fn of(&self, candidate_id: &str, revision: &str) -> Option<Label> {
        self.chart
            .iter()
            .position(|known| known.candidate_id == candidate_id && known.revision == revision)
            .map(label_at)
    }

    /// Where the chart version `label` names sits, when it names one.
    fn chart_index(&self, label: &Label) -> Option<usize> {
        prefixed_index(&label.0, 'S', self.chart.len())
    }

    /// The version, as `(candidate_id, revision)`, that a read of this work
    /// gave this label, or `None` when no read gave it: no read asked for a
    /// version with this label, or none returned it.
    pub(super) fn version(&self, label: &Label) -> Option<(&str, &str)> {
        if let Some(index) = self.chart_index(label) {
            let version = &self.chart[index];
            return version
                .returned
                .then_some((version.candidate_id.as_str(), version.revision.as_str()));
        }
        if let Some(index) = prefixed_index(&label.0, 'L', self.library.len()) {
            let entry = &self.library[index];
            return (entry.returned && !entry.revision.is_empty())
                .then_some((entry.passage_id.as_str(), entry.revision.as_str()));
        }
        if let Some(index) = prefixed_index(&label.0, 'D', self.document_pages.len()) {
            let entry = &self.document_pages[index];
            return (entry.returned && !entry.revision.is_empty())
                .then_some((entry.document_id.as_str(), entry.revision.as_str()));
        }
        None
    }
}

/// Canonical `{prefix}{1..=len}` index parser (rejects non-canonical forms like `L01`).
fn prefixed_index(label: &str, prefix: char, len: usize) -> Option<usize> {
    let rest = label.strip_prefix(prefix)?;
    let index = rest.parse::<usize>().ok()?.checked_sub(1)?;
    (index < len && format!("{prefix}{}", index + 1) == label).then_some(index)
}

/// The label of the version at `index`.
fn label_at(index: usize) -> Label {
    Label(format!("S{}", index + 1))
}

/// Chart read receipts have one wire owner for production and delivery checks.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ReadReceipt {
    pub results: Vec<RecordReceipt>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(super) enum RecordReceipt {
    Available {
        reference: String,
        revision: String,
        record_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bucket: Option<Bucket>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_kind: Option<SourceRecordKind>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        version: Option<RecordVersion>,
        dates: ReceiptDates,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        codes: Vec<RecordCode>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native_link: Option<String>,
        read_representation: ReadRepresentation,
        content: String,
        readable_parts: Vec<ReadablePart>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unavailable_parts: Vec<FailedPart>,
        sha256: String,
        provenance: Value,
        /// The version's label, when the work's reads carry labels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cite_as: Option<Label>,
    },
    Passage {
        reference: String,
        revision: String,
        part: usize,
        label: String,
        start_byte: usize,
        end_byte: usize,
        total_bytes: usize,
        content: String,
        next: Option<Exact>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unavailable_parts: Vec<FailedPart>,
        /// The version's label, when the work's reads carry labels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cite_as: Option<Label>,
    },
    Unavailable {
        #[serde(skip_serializing_if = "Option::is_none")]
        reference: Option<String>,
        reason: String,
    },
    OverBudget {
        reason: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReadRepresentation {
    NativeRecord,
}

/// The group a step names for a record filed in `bucket`.
impl From<Bucket> for RecordGroup {
    fn from(bucket: Bucket) -> Self {
        match bucket {
            Bucket::Patient => Self::Patient,
            Bucket::Problems => Self::Problems,
            Bucket::Medications => Self::Medications,
            Bucket::Allergies => Self::Allergies,
            Bucket::Results => Self::Results,
            Bucket::Vitals => Self::Vitals,
            Bucket::Notes => Self::Notes,
            Bucket::Documents => Self::Documents,
            Bucket::Visits => Self::Visits,
            Bucket::Procedures => Self::Procedures,
            Bucket::Immunizations => Self::Immunizations,
            Bucket::Orders => Self::Orders,
            Bucket::History => Self::History,
            Bucket::Forms => Self::Forms,
            Bucket::Messages => Self::Messages,
            Bucket::Other => Self::Other,
        }
    }
}

/// The role a step or a citation names for a date filed with `role`.
impl From<DateRole> for physician_wire::DateRole {
    fn from(role: DateRole) -> Self {
        match role {
            DateRole::BirthDate => Self::BirthDate,
            DateRole::Onset => Self::Onset,
            DateRole::Abatement => Self::Abatement,
            DateRole::Recorded => Self::Recorded,
            DateRole::Authored => Self::Authored,
            DateRole::Effective => Self::Effective,
            DateRole::Issued => Self::Issued,
            DateRole::Performed => Self::Performed,
            DateRole::Occurred => Self::Occurred,
            DateRole::Start => Self::Start,
            DateRole::End => Self::End,
            DateRole::Sent => Self::Sent,
            DateRole::Received => Self::Received,
        }
    }
}

/// A date filed through an adapter table, as a step or a citation names it:
/// its declared role and the source's value. The source's own name for the
/// date field stays with the record.
impl From<&FiledDate> for physician_wire::RecordDate {
    fn from(date: &FiledDate) -> Self {
        Self {
            role: Some(date.role.into()),
            value: date.value.clone(),
        }
    }
}

impl RecordReceipt {
    pub(super) fn names(&self, source: &SourceRevision) -> bool {
        match self {
            Self::Available {
                reference,
                revision,
                ..
            }
            | Self::Passage {
                reference,
                revision,
                ..
            } => reference == &source.candidate_id && revision == &source.revision,
            Self::Unavailable { .. } | Self::OverBudget { .. } => false,
        }
    }

    /// What this record of a read gave, in the receipt's own facts.
    fn read(&self) -> RecordRead {
        match self {
            Self::Available {
                reference,
                revision,
                bucket,
                title,
                dates,
                ..
            } => RecordRead::Whole {
                version: SourceVersion {
                    candidate_id: reference.clone(),
                    revision: revision.clone(),
                },
                about: RecordAbout {
                    group: bucket.map(RecordGroup::from),
                    title: title.clone(),
                    dates: match dates {
                        ReceiptDates::Filed(dates) => {
                            dates.iter().map(physician_wire::RecordDate::from).collect()
                        }
                        // A date the record carries without an adapter
                        // table declares no role.
                        ReceiptDates::Raw(dates) => dates
                            .iter()
                            .map(|date| physician_wire::RecordDate {
                                role: None,
                                value: date.value.clone(),
                            })
                            .collect(),
                    },
                },
            },
            Self::Passage {
                reference,
                revision,
                ..
            } => RecordRead::Passage {
                version: SourceVersion {
                    candidate_id: reference.clone(),
                    revision: revision.clone(),
                },
            },
            Self::Unavailable { .. } => RecordRead::Unavailable,
            Self::OverBudget { .. } => RecordRead::TooLarge,
        }
    }

    fn encoded_len(&self) -> usize {
        serde_json::to_vec(self)
            .expect("chart receipt serializes")
            .len()
    }

    fn over_budget() -> Self {
        Self::OverBudget {
            reason: "Request this reference alone with a larger byte budget.".into(),
        }
    }
}

enum Operation {
    Search(Search),
    Read(Read),
}

/// A reply revision already owns an immutable observation. Ordinary chat
/// must name the revision it found in the current workspace.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChartScope {
    /// Search and exact reads in a signed-in chat.
    Chat,
    /// Exact reads inside the reply's pinned observation.
    PinnedReply,
}

/// A correctable chart tool request.
#[derive(Debug, thiserror::Error)]
pub enum ChartInputError {
    /// A discovery request did not match the offered search fields.
    #[error("Supply query, optional bucket or record_type, and the returned after cursor ({0}).")]
    Search(#[source] serde_json::Error),
    /// A discovery request supplied a bucket name outside the sixteen chart buckets.
    #[error("{0}")]
    InvalidBucket(#[source] UnknownBucketError),
    /// An exact read did not name its references and byte budget.
    #[error("Supply references as an array and max_result_bytes as an integer ({0}).")]
    Read(#[source] serde_json::Error),
    /// Well-formed arguments outside the tool's contract.
    #[error("{0}")]
    Refused(&'static str),
}

/// Validated arguments. Chat requires revision-qualified references; a reply
/// revision can also address its own pinned observation by candidate alone.
pub struct ChartRequest(Operation);

impl ChartRequest {
    /// Validate before consulting the chart. Errors are feedback to the caller.
    pub fn parse(
        name: &str,
        arguments: &Value,
        scope: ChartScope,
    ) -> Result<Self, ChartInputError> {
        let operation = match name {
            SEARCH_TOOL if scope == ChartScope::Chat => {
                let wire: SearchWire =
                    serde_json::from_value(arguments.clone()).map_err(ChartInputError::Search)?;
                if wire.query.len() > 512 {
                    return Err(ChartInputError::Refused(
                        "Use at most 512 bytes of search text.",
                    ));
                }
                let bucket = wire
                    .bucket
                    .as_deref()
                    .map(Bucket::parse)
                    .transpose()
                    .map_err(ChartInputError::InvalidBucket)?;
                Operation::Search(Search {
                    query: wire.query,
                    bucket,
                    record_type: wire.record_type,
                    after: wire.after,
                })
            }
            READ_TOOL => {
                let read: Read =
                    serde_json::from_value(arguments.clone()).map_err(ChartInputError::Read)?;
                if !(1..=8).contains(&read.references.len())
                    || !(4096..=MAX_PAGE).contains(&read.max_result_bytes)
                {
                    return Err(ChartInputError::Refused(
                        "Supply one to eight references and max_result_bytes between 4096 and 65536.",
                    ));
                }
                if scope == ChartScope::Chat
                    && read
                        .references
                        .iter()
                        .any(|r| matches!(r, Reference::Pinned(_)))
                {
                    return Err(ChartInputError::Refused(
                        "Use the candidate_id and revision returned by chart discovery.",
                    ));
                }
                Operation::Read(read)
            }
            _ => {
                return Err(ChartInputError::Refused(
                    "Choose a tool offered for this work.",
                ));
            }
        };
        Ok(Self(operation))
    }

    /// An exact read needs only these candidates from the workspace store.
    pub(crate) fn candidates(&self) -> Option<Vec<&str>> {
        match &self.0 {
            Operation::Search(_) => None,
            Operation::Read(read) => Some(read.references.iter().map(Reference::id).collect()),
        }
    }

    /// The versions an exact read names, in order. A search, or a reference
    /// by candidate alone, names none.
    pub(super) fn versions(&self) -> Vec<(&str, &str)> {
        match &self.0 {
            Operation::Search(_) => Vec::new(),
            Operation::Read(read) => read
                .references
                .iter()
                .filter_map(|reference| match reference {
                    Reference::Exact(exact) => Some((&*exact.candidate_id, &*exact.revision)),
                    Reference::Pinned(_) => None,
                })
                .collect(),
        }
    }

    /// The step this call shows the physician, as far as it has ended.
    pub fn step(&self, ending: CallEnding<'_>) -> Result<Step, StepError> {
        match &self.0 {
            Operation::Search(search) => search.step(ending),
            Operation::Read(read) => read.step(ending),
        }
    }

    /// Resolve only this workspace's accepted source revisions. The receipts
    /// carry no labels.
    pub fn run(
        &self,
        workspace: &str,
        version: u64,
        sources: &[SourceRevision],
    ) -> Result<ToolOutcome, String> {
        self.outcome(workspace, version, sources, None)
    }

    /// [`Self::run`] for a job whose reads carry labels: each record and
    /// passage a read returns has its version's label from the work's
    /// `labels`, counted in its size.
    pub fn run_labelled(
        &self,
        workspace: &str,
        version: u64,
        sources: &[SourceRevision],
        labels: &Labels,
    ) -> Result<ToolOutcome, String> {
        self.outcome(workspace, version, sources, Some(labels))
    }

    fn outcome(
        &self,
        workspace: &str,
        version: u64,
        sources: &[SourceRevision],
        labels: Option<&Labels>,
    ) -> Result<ToolOutcome, String> {
        match &self.0 {
            Operation::Search(search) => search.page(workspace, version, sources),
            Operation::Read(read) => read
                .page(workspace, sources, labels)
                .map(|value| ToolOutcome::Returned(value.to_string())),
        }
    }
}

impl Search {
    fn page(
        &self,
        workspace: &str,
        version: u64,
        sources: &[SourceRevision],
    ) -> Result<ToolOutcome, String> {
        if self.after.as_ref().is_some_and(|c| c.version != version) {
            return Ok(ToolOutcome::refused(
                "The retained chart changed. Repeat the search from the beginning.",
            ));
        }
        let query = self.query.to_lowercase();
        let mut matches: Vec<_> = sources
            .iter()
            .filter(|s| self.matches_source(workspace, &query, s))
            .collect();
        matches.sort_unstable_by_key(|s| &s.candidate_id);
        let mut results = Vec::new();
        let mut next = None;
        for source in matches.iter().take(SEARCH_PAGE) {
            let hit = serde_json::to_value(search_hit(source)).expect("search hit serializes");
            results.push(hit);
            let continuation = After {
                version,
                candidate_id: source.candidate_id.clone(),
            };
            if json!({"results": results, "next": continuation})
                .to_string()
                .len()
                > MAX_PAGE
            {
                results.pop();
                if results.is_empty() {
                    return Ok(ToolOutcome::refused(
                        "The matching record's metadata exceeds the discovery result limit.",
                    ));
                }
                break;
            }
            next = Some(continuation);
        }
        if matches.len() == results.len() {
            next = None;
        }
        let page = serde_json::to_value(SearchPage { results, next })
            .expect("chart search page serializes");
        Ok(ToolOutcome::Returned(page.to_string()))
    }

    /// The search with how many records its page found, and whether the call
    /// went on from an earlier page. A refused search returned nothing.
    fn step(&self, ending: CallEnding<'_>) -> Result<Step, StepError> {
        let state = ending.state(|result| match result {
            ToolResult::Original(output) => {
                let page: SearchPage = serde_json::from_str(output).map_err(StepError::Receipt)?;
                Ok(StepState::Returned {
                    result: Found {
                        records: page.results.len(),
                        more: page.next.is_some(),
                    },
                })
            }
            ToolResult::Refused(_) => Ok(StepState::Failed),
            ToolResult::History(_) | ToolResult::View(_) => Err(StepError::UnexpectedResult),
        })?;
        Ok(Step::SearchedChart {
            query: self.query.clone(),
            group: self.bucket.map(RecordGroup::from),
            page: if self.after.is_some() {
                physician_wire::SearchPage::More
            } else {
                physician_wire::SearchPage::First
            },
            state,
        })
    }

    fn matches_source(&self, workspace: &str, query: &str, source: &SourceRevision) -> bool {
        if source.workspace != workspace {
            return false;
        }
        if let Some(target_bucket) = self.bucket
            && !source
                .envelope
                .as_ref()
                .is_some_and(|env| env.bucket == target_bucket)
        {
            return false;
        }
        if self
            .record_type
            .as_ref()
            .is_some_and(|kind| !kind.eq_ignore_ascii_case(&source.record_type))
        {
            return false;
        }
        if self
            .after
            .as_ref()
            .is_some_and(|c| source.candidate_id <= c.candidate_id)
        {
            return false;
        }
        if query.trim().is_empty() {
            return true;
        }
        let fields = searchable_fields(source);
        query
            .split_whitespace()
            .all(|term| fields.iter().any(|field| field.contains(term)))
    }
}

fn searchable_fields(source: &SourceRevision) -> Vec<String> {
    let mut fields: Vec<String> = [
        source.record_type.as_str(),
        source.candidate_id.as_str(),
        source.content.as_str(),
    ]
    .into_iter()
    .chain(source.readable_parts.iter().map(|part| part.text.as_str()))
    .chain(source.dates.iter().map(|date| date.value.as_str()))
    .map(str::to_lowercase)
    .collect();
    if let Some(env) = &source.envelope {
        fields.push(env.bucket.as_str().to_owned());
        fields.push(env.source_kind.kind.to_lowercase());
        if let Some(category) = &env.source_kind.category {
            fields.push(category.to_lowercase());
        }
        if let Some(title) = &env.title {
            fields.push(title.to_lowercase());
        }
        if let Some(status) = &env.status {
            fields.push(status.to_lowercase());
        }
        for date in &env.dates {
            fields.push(date.role.as_str().to_owned());
            fields.push(date.field.to_lowercase());
            fields.push(date.value.to_lowercase());
        }
        for code in &env.codes {
            for part in [
                code.system.as_deref(),
                code.code.as_deref(),
                code.display.as_deref(),
                code.text.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                fields.push(part.to_lowercase());
            }
        }
    }
    fields
}

fn search_hit(source: &SourceRevision) -> SearchHit<'_> {
    let reference = Exact {
        candidate_id: source.candidate_id.clone(),
        revision: source.revision.clone(),
        cursor: None,
    };
    match &source.envelope {
        Some(env) => SearchHit {
            reference,
            record_type: &source.record_type,
            source_id: &source.source_id,
            bucket: Some(env.bucket),
            source_kind: Some(&env.source_kind),
            version: Some(&env.version),
            dates: SearchHitDates::Filed(&env.dates),
            status: env.status.as_deref(),
            codes: &env.codes,
            title: env.title.as_deref(),
        },
        None => SearchHit {
            reference,
            record_type: &source.record_type,
            source_id: &source.source_id,
            bucket: None,
            source_kind: None,
            version: None,
            dates: SearchHitDates::Raw(&source.dates),
            status: None,
            codes: &[],
            title: None,
        },
    }
}

impl Read {
    /// The read with what each requested record gave, from its receipt. A
    /// refused read returned nothing.
    fn step(&self, ending: CallEnding<'_>) -> Result<Step, StepError> {
        let state = ending.state(|result| match result {
            ToolResult::Original(output) => {
                let receipt: ReadReceipt =
                    serde_json::from_str(output).map_err(StepError::Receipt)?;
                Ok(StepState::Returned {
                    result: receipt.results.iter().map(RecordReceipt::read).collect(),
                })
            }
            ToolResult::Refused(_) => Ok(StepState::Failed),
            ToolResult::History(_) | ToolResult::View(_) => Err(StepError::UnexpectedResult),
        })?;
        Ok(Step::ReadRecords {
            requested: self.references.len(),
            state,
        })
    }

    fn page(
        &self,
        workspace: &str,
        sources: &[SourceRevision],
        labels: Option<&Labels>,
    ) -> Result<Value, String> {
        // Reserve the envelope and commas. Each reference has space even when
        // an earlier one is large, and the bound includes JSON escaping.
        let budget = (self.max_result_bytes - r#"{"results":[]}"#.len() - self.references.len())
            / self.references.len();
        let mut results = Vec::new();
        for reference in &self.references {
            let source = sources
                .iter()
                .find(|s| s.workspace == workspace && s.candidate_id == reference.id());
            let result = match source {
                Some(source) => {
                    source.validate()?;
                    match reference {
                        Reference::Exact(r) if r.revision != source.revision => {
                            RecordReceipt::Unavailable {
                                reference: Some(r.candidate_id.clone()),
                                reason: "revision_changed_repeat_discovery".into(),
                            }
                        }
                        Reference::Exact(r) => {
                            let label = labels
                                .map(|labels| {
                                    labels.of(&r.candidate_id, &r.revision).ok_or(
                                        "a read returned a version that no read of this work asked for",
                                    )
                                })
                                .transpose()?;
                            read_record(source, r.cursor, budget, label)
                        }
                        Reference::Pinned(_) => read_record(source, None, budget, None),
                    }
                }
                None => RecordReceipt::Unavailable {
                    reference: Some(reference.id().into()),
                    reason: "not_in_recorded_observation".into(),
                },
            };
            if result.encoded_len() > budget {
                results.push(RecordReceipt::over_budget());
            } else {
                results.push(result);
            }
        }
        Ok(serde_json::to_value(ReadReceipt { results }).expect("chart receipt serializes"))
    }
}

/// One record of a read, with its label when the read carries labels. The
/// label is part of the receipt when its size is checked.
fn read_record(
    source: &SourceRevision,
    cursor: Option<Position>,
    budget: usize,
    cite_as: Option<Label>,
) -> RecordReceipt {
    if cursor.is_none() && source.content.len() <= budget {
        let full = available_receipt(source, cite_as.clone());
        if full.encoded_len() <= budget {
            return full;
        }
    }
    let position = cursor.unwrap_or_default();
    let Some((label, text)) = part(source, position.part) else {
        return RecordReceipt::Unavailable {
            reference: None,
            reason: "invalid_part_use_returned_cursor".into(),
        };
    };
    if !text.is_char_boundary(position.offset)
        || (position.offset == text.len() && !text.is_empty())
    {
        return RecordReceipt::Unavailable {
            reference: None,
            reason: "invalid_offset_use_returned_cursor".into(),
        };
    }
    let passage = |end| {
        let next = if end < text.len() {
            Some(Position {
                offset: end,
                ..position
            })
        } else if position.part < source.readable_parts.len() {
            Some(Position {
                part: position.part + 1,
                offset: 0,
            })
        } else {
            None
        };
        RecordReceipt::Passage {
            unavailable_parts: source
                .envelope
                .as_ref()
                .map(|env| env.unavailable_parts.clone())
                .unwrap_or_default(),
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            part: position.part,
            label: label.into(),
            start_byte: position.offset,
            end_byte: end,
            total_bytes: text.len(),
            content: text[position.offset..end].into(),
            next: next.map(|cursor| Exact {
                candidate_id: source.candidate_id.clone(),
                revision: source.revision.clone(),
                cursor: Some(cursor),
            }),
            cite_as: cite_as.clone(),
        }
    };
    // Search UTF-8 boundaries of at most one page, measuring the encoded
    // envelope. The terminal page can be smaller because it drops its cursor.
    let remaining = &text[position.offset..];
    let limit = remaining.floor_char_boundary(remaining.len().min(budget));
    let ends: Vec<_> = remaining[..limit]
        .char_indices()
        .map(|(i, c)| position.offset + i + c.len_utf8())
        .collect();
    let end = position.offset + limit;
    let candidate = passage(end);
    if candidate.encoded_len() <= budget {
        return candidate;
    }
    let count = ends.partition_point(|end| passage(*end).encoded_len() <= budget);
    match count.checked_sub(1) {
        Some(i) => passage(ends[i]),
        None => RecordReceipt::over_budget(),
    }
}

fn available_receipt(source: &SourceRevision, cite_as: Option<Label>) -> RecordReceipt {
    match &source.envelope {
        Some(env) => RecordReceipt::Available {
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            record_type: source.record_type.clone(),
            bucket: Some(env.bucket),
            source_kind: Some(env.source_kind.clone()),
            version: Some(env.version.clone()),
            dates: ReceiptDates::Filed(env.dates.clone()),
            status: env.status.clone(),
            codes: env.codes.clone(),
            title: env.title.clone(),
            native_link: env.native_link.clone(),
            read_representation: ReadRepresentation::NativeRecord,
            content: source.content.clone(),
            readable_parts: source.readable_parts.clone(),
            unavailable_parts: source
                .envelope
                .as_ref()
                .map(|env| env.unavailable_parts.clone())
                .unwrap_or_default(),
            sha256: source.sha256.clone(),
            provenance: source.provenance.clone(),
            cite_as,
        },
        None => RecordReceipt::Available {
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            record_type: source.record_type.clone(),
            bucket: None,
            source_kind: None,
            version: None,
            dates: ReceiptDates::Raw(source.dates.clone()),
            status: None,
            codes: Vec::new(),
            title: None,
            native_link: None,
            read_representation: ReadRepresentation::NativeRecord,
            content: source.content.clone(),
            readable_parts: source.readable_parts.clone(),
            unavailable_parts: source
                .envelope
                .as_ref()
                .map(|env| env.unavailable_parts.clone())
                .unwrap_or_default(),
            sha256: source.sha256.clone(),
            provenance: source.provenance.clone(),
            cite_as,
        },
    }
}

pub(super) fn part(source: &SourceRevision, index: usize) -> Option<(&str, &str)> {
    if index == 0 {
        return Some(("native_record", &source.content));
    }
    source
        .readable_parts
        .get(index - 1)
        .map(|p| (p.label.as_str(), p.text.as_str()))
}

/// The search definition used by both chat hosts.
pub fn search_tool() -> Tool {
    let bucket_names: Vec<&str> = Bucket::ALL.into_iter().map(Bucket::as_str).collect();
    Tool {
        name: SEARCH_TOOL.into(),
        description: "Browse or search the currently retained chart for this patient. Filter by chart bucket or source record_type when narrowing, and use an empty query to list a bucket. All query words must match the native record, decoded text, bucket, source kind, codes, title, status, ID or dates. Results are metadata in candidate order, not evidence of clinical absence. Continue the same search with next as after. Open a returned reference to read evidence.".into(),
        parameters: json!({
            "type":"object",
            "properties":{
                "query":{"type":"string"},
                "bucket":{"anyOf":[{"type":"null"},{"type":"string","enum":bucket_names}]},
                "record_type":{"type":["string","null"]},
                "after":{"anyOf":[{"type":"null"}, {
                    "type":"object",
                    "properties":{"version":{"type":"integer"},"candidate_id":{"type":"string"}},
                    "required":["version","candidate_id"],"additionalProperties":false
                }]}
            },
            "required":["query","bucket","record_type","after"],"additionalProperties":false
        }),
    }
}

/// Reply revisions can also use bare IDs in their existing pinned observation.
pub fn read_tool(scope: ChartScope) -> Tool {
    let exact = json!({
        "type":"object","properties":{
            "candidate_id":{"type":"string"},
            "revision":{"type":"string"},
            "cursor":{"anyOf":[{"type":"null"}, {
                "type":"object","properties":{
                    "part":{"type":"integer","minimum":0},
                    "offset":{"type":"integer","minimum":0}
                },
                "required":["part","offset"],"additionalProperties":false
            }]}
        },
        "required":["candidate_id","revision","cursor"],"additionalProperties":false
    });
    let reference = if scope == ChartScope::PinnedReply {
        json!({"anyOf":[{"type":"string"},exact]})
    } else {
        exact
    };
    Tool {
        name: READ_TOOL.into(),
        description: "Open exact references for this patient. A whole record includes native content and decoded parts. For a large record, each passage names its part and exact UTF-8 byte span. Pass its next reference back to continue; null ends the record. Part 0 is native content and parts 1 onward are decoded text. Base claims only on the passages delivered.".into(),
        parameters: json!({
            "type":"object","properties":{
                "references":{"type":"array","items":reference,"minItems":1,"maxItems":8},
                "max_result_bytes":{"type":"integer","minimum":4096,"maximum":65536}
            },
            "required":["references","max_result_bytes"],"additionalProperties":false
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physician_sources::adapter::file_fhir_record;
    use harness::engine::Invocation;

    fn event(kind: EventKind) -> (Event, bool) {
        (
            Event {
                attempt_id: "attempt".into(),
                sequence: 1,
                ordinal: 1,
                kind,
            },
            true,
        )
    }

    /// A record filed through an adapter table, read with labels, carries
    /// its label beside its envelope, and the work's labels rebuilt from that
    /// read's own result give the label the version the read returned.
    #[test]
    fn a_filed_record_read_with_labels_carries_its_label_beside_its_envelope() {
        let source = file_fhir_record(
            "patient",
            "ehr",
            r#"{"resourceType":"Condition","id":"c1","meta":{"versionId":"2"},"code":{"text":"Hypertension"},"recordedDate":"2026-08-01"}"#,
            None,
        )
        .unwrap();
        let arguments = json!({
            "references": [{
                "candidate_id": source.candidate_id,
                "revision": source.revision,
                "cursor": null
            }],
            "max_result_bytes": 4096
        });
        let read = ChartRequest::parse(READ_TOOL, &arguments, ChartScope::Chat).unwrap();
        let started = event(EventKind::ToolStarted(Invocation {
            id: "read".into(),
            name: READ_TOOL.into(),
            arguments: arguments.to_string(),
        }));
        let asked = Labels::of_work([&started]);

        let outcome = read
            .run_labelled("patient", 1, std::slice::from_ref(&source), &asked)
            .unwrap();
        let original = outcome.text().unwrap().to_owned();
        let receipt: Value = serde_json::from_str(&original).unwrap();
        let record = &receipt["results"][0];
        assert_eq!(record["outcome"], "available");
        assert_eq!(record["bucket"], "problems");
        assert_eq!(record["title"], "Hypertension");
        assert_eq!(record["cite_as"], "S1");

        let returned = event(EventKind::ToolReturned {
            call_id: "read".into(),
            result: ToolResult::Original(original),
        });
        let label = Label("S1".into());
        assert_eq!(asked.version(&label), None);
        assert_eq!(
            Labels::of_work([&started, &returned]).version(&label),
            Some((source.candidate_id.as_str(), source.revision.as_str()))
        );
    }
}
