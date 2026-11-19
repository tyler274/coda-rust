//! The record every patient source hands Coda, whatever its format. Each source
//! has its own adapter; past the adapter a record is one of these. Imports must
//! come from a source adapter, never projection prose or a provider. This
//! reader neither discovers new records nor claims that a snapshot is a
//! complete chart.
/// Declared adapter mapping tables and shared record filing.
pub mod adapter;
mod chart;
/// Elation / FHIR R4 roster, schedule, and patient chart adapter.
pub mod elation_fhir;
/// Hint Chrome capture extraction into shared records and delivery outcomes.
pub mod hint;
/// Shared domain types for chart buckets, filed envelopes, and pull outcomes.
pub mod types;
pub use chart::{
    ChartInputError, ChartRequest, ChartScope, Labels, SEARCH_TOOL, read_tool, search_tool,
};
pub(crate) use chart::{MemoryNoteRef, memory_note_candidate_id, parse_memory_notes_markdown};
/// The chat's typed finish and its citations.
pub mod citations;
pub use types::{
    Bucket, ChartPullReceipt, DateRole, FailedPart, FiledDate, PullOutcome, ReconnectReceipt,
    RecordCode, RecordEnvelope, RecordVersion, SourceAdapterError, SourceCommandKey,
    SourcePullOutput, SourceRecordKind, UnknownBucketError,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// One date on a record, as the source wrote it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordDate {
    /// The source's own name for this date.
    pub name: String,
    /// The value as the source gave it, at the source's precision.
    pub value: String,
}

/// Text inside a record that its exact representation doesn't show plainly,
/// such as an encoded attachment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadablePart {
    /// The source's own name for this part.
    pub label: String,
    /// The part's text.
    pub text: String,
}

/// One immutable accepted source revision from the physician source owner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRevision {
    /// Patient workspace to which the source owner admitted these bytes.
    pub workspace: String,
    /// The source that holds this record.
    pub source_id: String,
    /// Original opaque candidate ID, never derived from prose.
    pub candidate_id: String,
    /// Source owner's immutable revision identity.
    pub revision: String,
    /// The source's own name for this kind of record.
    pub record_type: String,
    /// The record's dates, in the source's words.
    pub dates: Vec<RecordDate>,
    /// Exact source-native retained UTF-8 representation.
    pub content: String,
    /// SHA-256 of retained content bytes, verified on import and read.
    pub sha256: String,
    /// The record's readable text, beside its exact representation.
    pub readable_parts: Vec<ReadablePart>,
    /// Source-owned provenance, including native record identity.
    pub provenance: Value,
    /// Shared record envelope produced by the source adapter's declared mapping table.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelope: Option<RecordEnvelope>,
}
impl SourceRevision {
    /// Refuses corrupt or unidentified source evidence.
    pub fn validate(&self) -> Result<(), String> {
        if self.workspace.is_empty()
            || self.source_id.is_empty()
            || self.candidate_id.is_empty()
            || self.revision.is_empty()
            || self.record_type.is_empty()
            || crate::sha256_hex(self.content.as_bytes()) != self.sha256
            || self.provenance.is_null()
        {
            return Err("invalid retained physician source revision".into());
        }
        Ok(())
    }
}

/// One patient a source holds, as the source identifies them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePatient {
    /// The source's own identity for this patient.
    pub source_patient_id: String,
    /// The patient's name as the source gives it.
    pub display_name: String,
    /// The birth date as the source gives it, when it gives one.
    pub birth_date: Option<String>,
    /// The sex or gender as the source gives it, when it gives one.
    pub sex_or_gender: Option<String>,
    /// The medical record numbers the source holds for this patient.
    pub medical_record_numbers: Vec<MedicalRecordNumber>,
}

/// One medical record number and the system that issued it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MedicalRecordNumber {
    /// The issuing system.
    pub system: String,
    /// The number.
    pub value: String,
}

/// A source read as whole charts: who it holds and each patient's records. Each
/// source implements this in its own terms; past it, every chart is the same.
#[async_trait]
pub trait PatientSource: Send + Sync {
    /// The source's stable identifier.
    fn source_id(&self) -> &str;
    /// The source's name as a physician sees it.
    fn source_label(&self) -> &str;
    /// The source's own id for every patient it holds.
    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError>;
    /// One patient's identity, read alone so one bad patient stays one patient.
    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, SourceAdapterError>;
    /// One patient's records, admitted to `workspace`.
    async fn records(
        &self,
        patient: &SourcePatient,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError>;

    /// Pulls one patient's identity and records in a single connector operation,
    /// distinguishing complete pulls, partial pulls, whole failures, and lapsed access.
    async fn pull_patient(
        &self,
        _practice_id: &str,
        source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        match self.patient(source_patient_id).await {
            Ok(patient) => match self.records(&patient, workspace).await {
                Ok(records) => SourcePullOutput::Complete { patient, records },
                Err(SourceAdapterError::LapsedAccess { reason }) => {
                    SourcePullOutput::LapsedAccess {
                        error: SourceAdapterError::LapsedAccess { reason },
                    }
                }
                Err(error) => SourcePullOutput::Failed { error },
            },
            Err(SourceAdapterError::LapsedAccess { reason }) => SourcePullOutput::LapsedAccess {
                error: SourceAdapterError::LapsedAccess { reason },
            },
            Err(error) => SourcePullOutput::Failed { error },
        }
    }

    /// Reconnects a practice's access to this source when credentials have lapsed.
    ///
    /// When `command_key` is `Some(...)`, the adapter deduplicates or recovers any prior
    /// successful reconnect performed for `(practice_id, command_key)` so retrying an interrupted
    /// command before its database checkpoint commits does not repeat the external side effect.
    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        _command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        Ok(ReconnectReceipt {
            source_id: self.source_id().to_owned(),
            practice_id: practice_id.to_owned(),
            reconnected_at: now,
        })
    }
}

/// The stable workspace of one source patient. It carries no patient
/// identifier, because workspace ids travel in URLs.
pub fn workspace_id(source_id: &str, source_patient_id: &str) -> String {
    let digest = crate::sha256_hex(format!("{source_id}|{source_patient_id}").as_bytes());
    format!("pw-{}", &digest[..32])
}

/// The only source-reading capability granted to a physician reply revision.
pub const READ_TOOL: &str = "read_patient_sources";

/// Dispatch a tool against this revision's pinned sources. A correctable call
/// yields model feedback. Invalid evidence still stops the reader.
pub fn read_sources(
    sources: &[SourceRevision],
    workspace: &str,
    call: &harness::engine::Invocation<Value>,
) -> Result<harness::engine::ToolOutcome, harness::engine::Cause> {
    use harness::engine::ToolOutcome;
    match ChartRequest::parse(&call.name, &call.arguments, ChartScope::PinnedReply) {
        Ok(read) => read.run(workspace, 0, sources).map_err(Into::into),
        Err(reason) => Ok(ToolOutcome::refused(reason.to_string())),
    }
}

/// Reads candidate IDs inside the supplied pinned observation.
pub fn exact_read(
    sources: &[SourceRevision],
    workspace: &str,
    references: &[String],
    max_bytes: usize,
) -> Result<harness::engine::ToolOutcome, String> {
    match ChartRequest::parse(
        READ_TOOL,
        &json!({"references":references,"max_result_bytes":max_bytes}),
        ChartScope::PinnedReply,
    ) {
        Ok(read) => read.run(workspace, 0, sources),
        Err(reason) => Ok(harness::engine::ToolOutcome::refused(reason)),
    }
}

/// A source that changed revision between two pins of the same work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceCorrection {
    /// The revision the earlier pin held.
    pub previous: SourceRevision,
    /// The revision that replaced it.
    pub current: SourceRevision,
}

/// How a piece of work's pinned sources moved while it was still running.
/// It keeps the full earlier revisions, so the record shows exactly what each
/// stretch of the work could read; nothing is rewritten.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceRefresh {
    /// Source version the work had pinned.
    pub from_version: u64,
    /// Source version it pins now.
    pub to_version: u64,
    /// Sources that were not in the earlier pin.
    pub added: Vec<SourceRevision>,
    /// Sources whose revision changed.
    pub corrected: Vec<SourceCorrection>,
    /// Sources that are no longer in the observation.
    pub withdrawn: Vec<SourceRevision>,
    /// How many execution facts the work had retained when it was
    /// refreshed. Only a model call dispatched after this point may finish.
    pub after_records: usize,
}

impl SourceRefresh {
    /// The change from `before` (pinned at `from_version`) to `after`
    /// (observed at `to_version`), compared by candidate ID.
    pub fn between(
        from_version: u64,
        before: &[SourceRevision],
        to_version: u64,
        after: &[SourceRevision],
    ) -> Self {
        let mut refresh = Self {
            from_version,
            to_version,
            added: vec![],
            corrected: vec![],
            withdrawn: vec![],
            after_records: 0,
        };
        for current in after {
            match before
                .iter()
                .find(|s| s.candidate_id == current.candidate_id)
            {
                None => refresh.added.push(current.clone()),
                Some(previous) if previous != current => refresh.corrected.push(SourceCorrection {
                    previous: previous.clone(),
                    current: current.clone(),
                }),
                Some(_) => {}
            }
        }
        for previous in before {
            if !after
                .iter()
                .any(|s| s.candidate_id == previous.candidate_id)
            {
                refresh.withdrawn.push(previous.clone());
            }
        }
        refresh
    }

    /// Whether nothing the work could read changed.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.corrected.is_empty() && self.withdrawn.is_empty()
    }

    /// References the work must read before it may finish again.
    pub fn to_read(&self) -> Vec<String> {
        self.added
            .iter()
            .map(|s| s.candidate_id.clone())
            .chain(
                self.corrected
                    .iter()
                    .map(|c| c.current.candidate_id.clone()),
            )
            .collect()
    }

    /// The structured result returned to the model in place of a refused
    /// finish. It names references only; content is read through
    /// `read_patient_sources`.
    pub fn result(&self, current: &[SourceRevision]) -> Value {
        json!({
            "outcome": SOURCES_CHANGED,
            "message": "Patient sources changed while you were revising, so nothing was saved. Read the added and corrected references with read_patient_sources, then call finish_reply_revision again. Withdrawn references are no longer available.",
            "added": self.added.iter().map(|s| &s.candidate_id).collect::<Vec<_>>(),
            "corrected": self.corrected.iter().map(|c| &c.current.candidate_id).collect::<Vec<_>>(),
            "withdrawn": self.withdrawn.iter().map(|s| &s.candidate_id).collect::<Vec<_>>(),
            "source_references": current.iter().map(|s| &s.candidate_id).collect::<Vec<_>>(),
        })
    }
}

/// Outcome of a finish refused because the work's sources moved.
pub const SOURCES_CHANGED: &str = "sources_changed";
/// Outcome of a finish refused because changed sources were not read yet.
pub const SOURCES_UNREAD: &str = "sources_unread";

/// The result returned when the model finishes again without reading the
/// sources that changed under it.
pub fn unread_result(references: &[String]) -> Value {
    json!({
        "outcome": SOURCES_UNREAD,
        "message": "Read these changed references with read_patient_sources before calling finish_reply_revision again. Nothing was saved.",
        "references": references,
    })
}

/// Whether work refreshed mid-run has caught up before finishing again. The
/// latest refresh must be followed by a model call, and that call's context
/// must hold an exact read of every added or corrected source, at the
/// revision now pinned (`current`). Withdrawn sources need no read.
///
/// `Err` when no model call followed the latest refresh, which the run loop
/// never does: a replayed or stale terminal. `Ok` lists the references still
/// unread; empty means the work may finish.
#[cfg(test)]
fn caught_up(
    events: &[harness::engine::Event],
    refreshes: &[SourceRefresh],
    current: &[SourceRevision],
) -> Result<Vec<String>, String> {
    use harness::engine::EventKind;
    let dispatched = events
        .iter()
        .rposition(|event| matches!(event.kind, EventKind::ProviderStarted { .. }));
    caught_up_context(
        dispatched,
        events
            .iter()
            .map(harness::engine::recipe::ReplayEvent::Admitted),
        refreshes,
        current,
    )
}

/// Product source rules consume the harness's delivery coverage. The caller
/// supplies the latest dispatch's position relative to its source refresh.
pub fn caught_up_context<'a>(
    dispatched: Option<usize>,
    records: impl IntoIterator<Item = impl Into<harness::engine::recipe::ReplayEvent<'a>>>,
    refreshes: &[SourceRefresh],
    current: &[SourceRevision],
) -> Result<Vec<String>, String> {
    use harness::engine::results;
    let Some(latest) = refreshes.last() else {
        return Ok(vec![]);
    };
    let Some(index) = dispatched else {
        return Err("no model call followed the source refresh".into());
    };
    if index < latest.after_records {
        return Err("no model call followed the source refresh".into());
    }
    let delivered: Vec<chart::RecordReceipt> = results::delivered_originals(records, &[])
        .map_err(|cause| cause.to_string())?
        .into_iter()
        .filter_map(|output| serde_json::from_str::<chart::ReadReceipt>(output).ok())
        .flat_map(|receipt| receipt.results)
        .collect();
    let mut unread = Vec::new();
    for reference in refreshes.iter().flat_map(SourceRefresh::to_read) {
        let Some(source) = current.iter().find(|s| s.candidate_id == reference) else {
            continue; // withdrawn again since
        };
        if !record_completely_delivered(source, &delivered) && !unread.contains(&reference) {
            unread.push(reference);
        }
    }
    Ok(unread)
}

fn record_completely_delivered(
    source: &SourceRevision,
    delivered: &[chart::RecordReceipt],
) -> bool {
    let results: Vec<_> = delivered
        .iter()
        .filter(|receipt| receipt.names(source))
        .collect();
    results.iter().any(|receipt| matches!(receipt,
        chart::RecordReceipt::Available { content, sha256, provenance, .. }
            if content == &source.content && sha256 == &source.sha256 && provenance == &source.provenance
    )) || (0..=source.readable_parts.len())
        .all(|part| delivered_part(source, part, &results))
}

/// A patient chart record added after the chat run's known observation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddedChartRecord {
    /// Candidate record identifier in the workspace.
    pub candidate_id: String,
    /// Current immutable revision identifier.
    pub revision: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Record type label.
    pub record_type: String,
    /// Clinically meaningful dates on the record.
    pub dates: Vec<RecordDate>,
}

/// A patient chart record corrected to a newer revision after the chat run's known observation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectedChartRecord {
    /// Candidate record identifier in the workspace.
    pub candidate_id: String,
    /// Revision previously known to the run.
    pub previous_revision: String,
    /// Current immutable revision identifier.
    pub current_revision: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Record type label.
    pub record_type: String,
    /// Clinically meaningful dates on the current record.
    pub dates: Vec<RecordDate>,
}

/// How the patient source store represents a withdrawn chart record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WithdrawnShape {
    /// Omitted by a later whole-source delivery, so `read_patient_sources`
    /// returns `not_in_recorded_observation`.
    RemovedFromObservation {
        /// Revision that was present before withdrawal.
        withdrawn_revision: String,
    },
    /// Replaced by a newer revision whose clinical status marks it retracted
    /// (`entered-in-error` or `withdrawn`).
    RetractedRevision {
        /// Revision that was present before retraction.
        previous_revision: String,
        /// Current retracted revision identifier.
        current_revision: String,
    },
    /// Arrived during the run already bearing a retracted clinical status.
    ArrivedWithdrawn {
        /// Current retracted revision identifier.
        revision: String,
    },
}

/// A patient chart record withdrawn after the chat run's known observation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawnChartRecord {
    /// Candidate record identifier in the workspace.
    pub candidate_id: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Record type label.
    pub record_type: String,
    /// Clinically meaningful dates on the record.
    pub dates: Vec<RecordDate>,
    /// How the withdrawal is represented in the retained chart.
    pub shape: WithdrawnShape,
}

/// Exact re-read obligation for a corrected or withdrawn record the run relied on.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ReliedRereadAction {
    /// Read the complete current revision via `read_patient_sources`.
    ReadCurrentRevision {
        /// Current revision identifier to open.
        revision: String,
    },
    /// Call `read_patient_sources` on the candidate to confirm its withdrawal
    /// (`not_in_recorded_observation`) from the retained chart.
    ConfirmWithdrawnFromChart {
        /// Last revision relied on before withdrawal.
        withdrawn_revision: String,
    },
}

/// One record the run relied on that changed and must be read again before finishing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReliedRecordRequirement {
    /// Candidate record identifier in the workspace.
    pub candidate_id: String,
    /// Required `read_patient_sources` follow-up action.
    #[serde(flatten)]
    pub action: ReliedRereadAction,
}

/// Reason why a proposed chat finish could not publish against current sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatArrivalOutcome {
    /// Patient chart records were added, corrected, or withdrawn during the run.
    SourcesChanged,
    /// Corrected or withdrawn records relied on by the run have not been read again yet.
    SourcesUnread,
}

/// Developer note attached to a refused chat finish when chart records changed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatChartArrivalNote {
    /// Whether this refusal reports newly observed chart changes or remaining unread obligations.
    pub outcome: ChatArrivalOutcome,
    /// Current workspace chart version at the time of the finish check.
    pub workspace_version: u64,
    /// Records newly added to the active chart.
    pub added: Vec<AddedChartRecord>,
    /// Records updated to a newer active revision.
    pub corrected: Vec<CorrectedChartRecord>,
    /// Records withdrawn or retracted from clinical reliance.
    pub withdrawn: Vec<WithdrawnChartRecord>,
    /// Relied-on corrected or withdrawn records that must be opened via `read_patient_sources`.
    pub must_read_again: Vec<ReliedRecordRequirement>,
    /// Plain model-facing instruction describing how to proceed in this run.
    pub message: String,
}

impl ChatChartArrivalNote {
    /// Encodes the note as JSON for `harness::engine::recipe::Refusal`.
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("ChatChartArrivalNote serializes as JSON")
    }
}

/// Result of checking chat chart currency at finish time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChatFinishCurrency {
    /// The chart matches what the run has observed and read; the finish may publish.
    Current,
    /// Chart arrivals or unread relied-on changes require continuing in the same run.
    Refused(ChatChartArrivalNote),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum KnownChartEntry {
    Present { revision: String },
    Withdrawn { retracted_revision: Option<String> },
}

enum CurrentChartEntry<'a> {
    Active(&'a SourceRevision),
    Retracted(&'a SourceRevision),
    Absent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ReliedRecordState {
    OpenedInitial { revision: String },
    OpenedAcrossChange,
    ConfirmedWithdrawnInEarlierWindow,
    ConfirmedWithdrawnInLatestWindow,
}

#[derive(Clone, Debug)]
struct SourceSummaryMeta {
    source_id: String,
    record_type: String,
    dates: Vec<RecordDate>,
}

fn is_retracted_json(content: &str) -> bool {
    let Ok(Value::Object(map)) = serde_json::from_str::<Value>(content) else {
        return false;
    };
    let Some(status) = map.get("status").and_then(Value::as_str) else {
        return false;
    };
    matches!(
        status.trim().to_ascii_lowercase().as_str(),
        "entered-in-error" | "entered_in_error" | "withdrawn"
    )
}

fn is_retracted_source(source: &SourceRevision) -> bool {
    is_retracted_json(&source.content)
}

fn known_entry_for_revision(
    candidate_id: &str,
    revision: String,
    content_hint: Option<&str>,
    baseline_map: &std::collections::BTreeMap<&str, &SourceRevision>,
    current_map: &std::collections::BTreeMap<&str, &SourceRevision>,
) -> KnownChartEntry {
    let retracted = content_hint
        .map(is_retracted_json)
        .or_else(|| {
            current_map
                .get(candidate_id)
                .filter(|src| src.revision == revision)
                .map(|src| is_retracted_source(src))
        })
        .or_else(|| {
            baseline_map
                .get(candidate_id)
                .filter(|src| src.revision == revision)
                .map(|src| is_retracted_source(src))
        })
        .unwrap_or(false);
    if retracted {
        KnownChartEntry::Withdrawn {
            retracted_revision: Some(revision),
        }
    } else {
        KnownChartEntry::Present { revision }
    }
}

/// Returns `true` when `record` belongs to a live clinical domain
/// (`Medications`, `Allergies`, `Results`, or `Vitals`) whose current orders
/// or values require re-reading after refreshing connected live sources.
pub fn record_requires_live_chart_reread(record: &SourceRevision) -> bool {
    if let Some(env) = &record.envelope {
        return matches!(
            env.bucket,
            Bucket::Medications | Bucket::Allergies | Bucket::Results | Bucket::Vitals
        );
    }
    let kind = record.record_type.to_ascii_lowercase();
    if matches!(
        kind.as_str(),
        "medicationrequest"
            | "medicationstatement"
            | "medicationadministration"
            | "medicationdispense"
            | "allergyintolerance"
            | "diagnosticreport"
    ) {
        return true;
    }
    if kind == "observation" {
        return record
            .provenance
            .get("category")
            .and_then(|v| v.as_array())
            .is_some_and(|cats| {
                cats.iter().any(|c| {
                    c.get("coding")
                        .and_then(|arr| arr.as_array())
                        .into_iter()
                        .flatten()
                        .filter_map(|coding| coding.get("code").and_then(|s| s.as_str()))
                        .any(|code| {
                            code.eq_ignore_ascii_case("laboratory")
                                || code.eq_ignore_ascii_case("vital-signs")
                        })
                })
            });
    }
    false
}

/// Enforces `P1.Rule.LiveChartReRead` on chat `finish_answer` when connected
/// patient sources are configured and the turn relies on medications, allergies,
/// lab results, or vital values.
pub fn evaluate_chat_live_chart_reread<'a>(
    current: &[SourceRevision],
    records: impl IntoIterator<Item = impl Into<harness::engine::recipe::ReplayEvent<'a>>>,
    messages: &[harness::engine::ProductMessage],
    finished: &citations::Finished,
) -> Result<Option<String>, String> {
    use harness::engine::{EventKind, results};
    use std::collections::{BTreeMap, BTreeSet};

    let records: Vec<harness::engine::recipe::ReplayEvent<'a>> =
        records.into_iter().map(Into::into).collect();
    let delivery = results::Delivery::from_events(records.iter().copied(), messages)
        .map_err(|cause| cause.to_string())?;
    let current_by_id: BTreeMap<&str, &SourceRevision> = current
        .iter()
        .map(|src| (src.candidate_id.as_str(), src))
        .collect();

    let mut cited_live_candidates: BTreeSet<String> = finished
        .marks()
        .iter()
        .chain(finished.basis().marks().iter())
        .map(|mark| mark.source.candidate_id.as_str())
        .filter(|cid| {
            current_by_id
                .get(cid)
                .is_some_and(|src| record_requires_live_chart_reread(src))
        })
        .map(ToOwned::to_owned)
        .collect();

    let mut refresh_ordinal: Option<usize> = None;
    let mut refresh_had_failure = false;
    let mut read_ordinals: BTreeMap<String, usize> = BTreeMap::new();

    for (idx, record) in records.iter().enumerate() {
        let event = match *record {
            harness::engine::recipe::ReplayEvent::Admitted(ev)
            | harness::engine::recipe::ReplayEvent::EvidenceOnly(ev) => ev,
        };
        let EventKind::ToolReturned {
            call_id,
            result: results::ToolResult::Original(output),
        } = &event.kind
        else {
            continue;
        };
        if !delivery.covers(call_id, None) {
            continue;
        }
        if let Ok(read_receipt) = serde_json::from_str::<chart::ReadReceipt>(output) {
            for item in read_receipt.results {
                match item {
                    chart::RecordReceipt::Available { reference, .. }
                    | chart::RecordReceipt::Passage { reference, .. } => {
                        if current_by_id
                            .get(reference.as_str())
                            .is_some_and(|src| record_requires_live_chart_reread(src))
                        {
                            cited_live_candidates.insert(reference.clone());
                            read_ordinals.insert(reference, idx);
                        }
                    }
                    chart::RecordReceipt::Unavailable { .. }
                    | chart::RecordReceipt::OverBudget { .. } => {}
                }
            }
            continue;
        }
        if let Ok(val) = serde_json::from_str::<Value>(output) {
            let looks_like_refresh = val.get("outcome").and_then(Value::as_str)
                == Some("refreshed")
                || (val.get("outcome").and_then(Value::as_str) == Some("failed")
                    && val.get("limits").is_some())
                || (val.get("sources").and_then(Value::as_array).is_some()
                    && val.get("workspace_version").is_some());
            if looks_like_refresh {
                refresh_ordinal = Some(idx);
                let any_failed = val
                    .get("sources")
                    .and_then(Value::as_array)
                    .is_some_and(|arr| {
                        arr.iter().any(|s| {
                            matches!(
                                s.get("status").and_then(Value::as_str),
                                Some("failed" | "partly_updated")
                            )
                        })
                    })
                    || val.get("outcome").and_then(Value::as_str) == Some("failed");
                if any_failed {
                    refresh_had_failure = true;
                }
            }
        }
    }

    if cited_live_candidates.is_empty() {
        return Ok(None);
    }

    let Some(refresh_idx) = refresh_ordinal else {
        return Ok(Some(
            json!({
                "kind": "live_chart_reread_required",
                "message": "Call `refresh_patient_sources` and re-read current medication, allergy, laboratory, or vital records with `read_patient_sources` before concluding.",
                "candidates": cited_live_candidates.into_iter().collect::<Vec<_>>(),
            })
            .to_string(),
        ));
    };

    let all_reread_after_refresh = cited_live_candidates
        .iter()
        .all(|cid| read_ordinals.get(cid).is_some_and(|&ord| ord > refresh_idx));
    if all_reread_after_refresh {
        return Ok(None);
    }
    if refresh_had_failure && !finished.basis().text().trim().is_empty() {
        return Ok(None);
    }

    Ok(Some(
        json!({
            "kind": "live_chart_reread_required",
            "message": "After calling `refresh_patient_sources`, re-read the affected medication, allergy, laboratory, or vital records with `read_patient_sources` (or explain the refresh limitation in `basis`) before finishing.",
            "candidates": cited_live_candidates.into_iter().collect::<Vec<_>>(),
        })
        .to_string(),
    ))
}

/// Single rule owner checking whether patient chart records were added,
/// corrected, or withdrawn while a chat answer ran, and whether any corrected
/// or withdrawn record the run relied on has been read again.
pub fn evaluate_chat_finish_currency<'a>(
    baseline: &[SourceRevision],
    current_version: u64,
    current: &[SourceRevision],
    records: impl IntoIterator<Item = impl Into<harness::engine::recipe::ReplayEvent<'a>>>,
    messages: &[harness::engine::ProductMessage],
) -> Result<ChatFinishCurrency, String> {
    use harness::engine::{EventKind, Item, results};
    use std::collections::{BTreeMap, BTreeSet};

    let records: Vec<harness::engine::recipe::ReplayEvent<'a>> =
        records.into_iter().map(Into::into).collect();
    let delivery = results::Delivery::from_events(records.iter().copied(), messages)
        .map_err(|cause| cause.to_string())?;
    let latest_window_receipts: Vec<chart::RecordReceipt> =
        results::delivered_originals(records.iter().copied(), messages)
            .map_err(|cause| cause.to_string())?
            .into_iter()
            .filter_map(|output| serde_json::from_str::<chart::ReadReceipt>(output).ok())
            .flat_map(|receipt| receipt.results)
            .collect();

    let mut metadata: BTreeMap<String, SourceSummaryMeta> = BTreeMap::new();
    let mut baseline_map: BTreeMap<&str, &SourceRevision> = BTreeMap::new();
    let mut known_chart: BTreeMap<String, KnownChartEntry> = BTreeMap::new();

    for src in baseline {
        baseline_map.insert(src.candidate_id.as_str(), src);
        metadata.insert(
            src.candidate_id.clone(),
            SourceSummaryMeta {
                source_id: src.source_id.clone(),
                record_type: src.record_type.clone(),
                dates: src.dates.clone(),
            },
        );
        let entry = if is_retracted_source(src) {
            KnownChartEntry::Withdrawn {
                retracted_revision: Some(src.revision.clone()),
            }
        } else {
            KnownChartEntry::Present {
                revision: src.revision.clone(),
            }
        };
        known_chart.insert(src.candidate_id.clone(), entry);
    }

    let mut current_map: BTreeMap<&str, &SourceRevision> = BTreeMap::new();
    for src in current {
        current_map.insert(src.candidate_id.as_str(), src);
        metadata.insert(
            src.candidate_id.clone(),
            SourceSummaryMeta {
                source_id: src.source_id.clone(),
                record_type: src.record_type.clone(),
                dates: src.dates.clone(),
            },
        );
    }

    let mut relied_records: BTreeMap<String, ReliedRecordState> = BTreeMap::new();
    let mut last_opened_revision: BTreeMap<String, String> = BTreeMap::new();
    let mut pending_requests: BTreeMap<(&str, u64), &harness::engine::SavedRequest> =
        BTreeMap::new();
    let latest_window = records
        .iter()
        .filter(|record| {
            matches!(
                record,
                harness::engine::recipe::ReplayEvent::Admitted(harness::engine::Event {
                    kind: EventKind::WindowOpened { .. },
                    ..
                })
            )
        })
        .count() as u64;

    for record in &records {
        let (event, is_admitted) = match *record {
            harness::engine::recipe::ReplayEvent::Admitted(event) => (event, true),
            harness::engine::recipe::ReplayEvent::EvidenceOnly(event) => (event, false),
        };
        let key = (event.attempt_id.as_str(), event.ordinal);
        match &event.kind {
            EventKind::ToolReturned {
                call_id,
                result: results::ToolResult::Original(output),
            } if delivery.covers(call_id, None) => {
                let in_latest = delivery.covers(call_id, Some(latest_window));
                if let Ok(read_receipt) = serde_json::from_str::<chart::ReadReceipt>(output) {
                    for item in read_receipt.results {
                        match item {
                            chart::RecordReceipt::Available {
                                reference,
                                revision,
                                record_type,
                                dates,
                                content,
                                ..
                            } => {
                                let dates = dates.into_record_dates();
                                metadata
                                    .entry(reference.clone())
                                    .and_modify(|m| {
                                        m.record_type = record_type.clone();
                                        m.dates = dates.clone();
                                    })
                                    .or_insert_with(|| SourceSummaryMeta {
                                        source_id: String::new(),
                                        record_type,
                                        dates,
                                    });
                                known_chart.insert(
                                    reference.clone(),
                                    known_entry_for_revision(
                                        &reference,
                                        revision.clone(),
                                        Some(&content),
                                        &baseline_map,
                                        &current_map,
                                    ),
                                );
                                last_opened_revision.insert(reference.clone(), revision.clone());
                                let next_state = match relied_records.get(&reference) {
                                    None => ReliedRecordState::OpenedInitial { revision },
                                    Some(ReliedRecordState::OpenedInitial { revision: prev })
                                        if prev == &revision =>
                                    {
                                        ReliedRecordState::OpenedInitial { revision }
                                    }
                                    Some(
                                        ReliedRecordState::OpenedInitial { .. }
                                        | ReliedRecordState::OpenedAcrossChange
                                        | ReliedRecordState::ConfirmedWithdrawnInEarlierWindow
                                        | ReliedRecordState::ConfirmedWithdrawnInLatestWindow,
                                    ) => ReliedRecordState::OpenedAcrossChange,
                                };
                                relied_records.insert(reference, next_state);
                            }
                            chart::RecordReceipt::Passage {
                                reference,
                                revision,
                                ..
                            } => {
                                known_chart.insert(
                                    reference.clone(),
                                    known_entry_for_revision(
                                        &reference,
                                        revision.clone(),
                                        None,
                                        &baseline_map,
                                        &current_map,
                                    ),
                                );
                                last_opened_revision.insert(reference.clone(), revision.clone());
                                let next_state = match relied_records.get(&reference) {
                                    None => ReliedRecordState::OpenedInitial { revision },
                                    Some(ReliedRecordState::OpenedInitial { revision: prev })
                                        if prev == &revision =>
                                    {
                                        ReliedRecordState::OpenedInitial { revision }
                                    }
                                    Some(
                                        ReliedRecordState::OpenedInitial { .. }
                                        | ReliedRecordState::OpenedAcrossChange
                                        | ReliedRecordState::ConfirmedWithdrawnInEarlierWindow
                                        | ReliedRecordState::ConfirmedWithdrawnInLatestWindow,
                                    ) => ReliedRecordState::OpenedAcrossChange,
                                };
                                relied_records.insert(reference, next_state);
                            }
                            chart::RecordReceipt::Unavailable {
                                reference: Some(reference),
                                reason,
                            } if reason == "not_in_recorded_observation" => {
                                known_chart.insert(
                                    reference.clone(),
                                    KnownChartEntry::Withdrawn {
                                        retracted_revision: None,
                                    },
                                );
                                if relied_records.contains_key(&reference) {
                                    let next_state = if in_latest {
                                        ReliedRecordState::ConfirmedWithdrawnInLatestWindow
                                    } else {
                                        ReliedRecordState::ConfirmedWithdrawnInEarlierWindow
                                    };
                                    relied_records.insert(reference, next_state);
                                }
                            }
                            chart::RecordReceipt::Unavailable { .. }
                            | chart::RecordReceipt::OverBudget { .. } => {}
                        }
                    }
                } else if let Ok(search_receipt) =
                    serde_json::from_str::<chart::SearchReceipt>(output)
                {
                    for hit in search_receipt.results {
                        let cid = hit.reference.candidate_id;
                        let rev = hit.reference.revision;
                        metadata.entry(cid.clone()).or_insert(SourceSummaryMeta {
                            source_id: hit.source_id,
                            record_type: hit.record_type,
                            dates: hit.dates.into_record_dates(),
                        });
                        if !relied_records.contains_key(&cid) {
                            let entry = known_entry_for_revision(
                                &cid,
                                rev,
                                None,
                                &baseline_map,
                                &current_map,
                            );
                            known_chart.insert(cid, entry);
                        }
                    }
                }
            }
            EventKind::ProviderStarted { request, .. } if is_admitted => {
                pending_requests.insert(key, request);
            }
            EventKind::ContextRejected
            | EventKind::ProviderFailed { .. }
            | EventKind::ProviderBlocked { .. }
            | EventKind::TransportRetry { .. }
                if is_admitted =>
            {
                pending_requests.remove(&key);
            }
            EventKind::ProviderReturned(_) if is_admitted => {
                if let Some(request) = pending_requests.remove(&key) {
                    for item in &request.added {
                        let refusal_opt = match item {
                            Item::FinishRefused(refusal) => Some(refusal),
                            Item::Result { output, .. } => match &output.source {
                                results::ResultSource::FinishRefused { refusal } => Some(refusal),
                                _ => None,
                            },
                            _ => None,
                        };
                        if let Some(refusal) = refusal_opt
                            && let Ok(note) =
                                serde_json::from_str::<ChatChartArrivalNote>(refusal.reason())
                        {
                            for added in note.added {
                                metadata.insert(
                                    added.candidate_id.clone(),
                                    SourceSummaryMeta {
                                        source_id: added.source_id,
                                        record_type: added.record_type,
                                        dates: added.dates,
                                    },
                                );
                                known_chart.insert(
                                    added.candidate_id,
                                    KnownChartEntry::Present {
                                        revision: added.revision,
                                    },
                                );
                            }
                            for corrected in note.corrected {
                                metadata.insert(
                                    corrected.candidate_id.clone(),
                                    SourceSummaryMeta {
                                        source_id: corrected.source_id,
                                        record_type: corrected.record_type,
                                        dates: corrected.dates,
                                    },
                                );
                                known_chart.insert(
                                    corrected.candidate_id,
                                    KnownChartEntry::Present {
                                        revision: corrected.current_revision,
                                    },
                                );
                            }
                            for withdrawn in note.withdrawn {
                                metadata.insert(
                                    withdrawn.candidate_id.clone(),
                                    SourceSummaryMeta {
                                        source_id: withdrawn.source_id,
                                        record_type: withdrawn.record_type,
                                        dates: withdrawn.dates,
                                    },
                                );
                                let retracted_revision = match withdrawn.shape {
                                    WithdrawnShape::RemovedFromObservation { .. } => None,
                                    WithdrawnShape::RetractedRevision {
                                        current_revision, ..
                                    } => Some(current_revision),
                                    WithdrawnShape::ArrivedWithdrawn { revision } => Some(revision),
                                };
                                known_chart.insert(
                                    withdrawn.candidate_id,
                                    KnownChartEntry::Withdrawn { retracted_revision },
                                );
                            }
                        }
                    }
                }
            }
            EventKind::ToolReturned { .. }
            | EventKind::ProviderStarted { .. }
            | EventKind::ProviderReturned(_)
            | EventKind::ContextRejected
            | EventKind::ProviderFailed { .. }
            | EventKind::ProviderBlocked { .. }
            | EventKind::TransportRetry { .. }
            | EventKind::ProviderObservation { .. }
            | EventKind::ToolStarted(_)
            | EventKind::Candidate { .. }
            | EventKind::WindowOpened { .. }
            | EventKind::CallBegan { .. } => {}
        }
    }

    let mut all_candidates: BTreeSet<&str> = BTreeSet::new();
    for cid in known_chart.keys() {
        all_candidates.insert(cid.as_str());
    }
    for src in current {
        all_candidates.insert(src.candidate_id.as_str());
    }

    let mut added = Vec::new();
    let mut corrected = Vec::new();
    let mut withdrawn = Vec::new();

    for cid in all_candidates {
        let current_entry = match current_map.get(cid).copied() {
            Some(src) if is_retracted_source(src) => CurrentChartEntry::Retracted(src),
            Some(src) => CurrentChartEntry::Active(src),
            None => CurrentChartEntry::Absent,
        };
        let meta = metadata.get(cid);
        let source_id = meta.map_or_else(String::new, |m| m.source_id.clone());
        let record_type = meta.map_or_else(String::new, |m| m.record_type.clone());
        let dates = meta.map_or_else(Vec::new, |m| m.dates.clone());

        match (known_chart.get(cid), current_entry) {
            (None, CurrentChartEntry::Active(curr))
            | (Some(KnownChartEntry::Withdrawn { .. }), CurrentChartEntry::Active(curr)) => {
                added.push(AddedChartRecord {
                    candidate_id: curr.candidate_id.clone(),
                    revision: curr.revision.clone(),
                    source_id: curr.source_id.clone(),
                    record_type: curr.record_type.clone(),
                    dates: curr.dates.clone(),
                });
            }
            (None, CurrentChartEntry::Retracted(curr)) => {
                withdrawn.push(WithdrawnChartRecord {
                    candidate_id: curr.candidate_id.clone(),
                    source_id: curr.source_id.clone(),
                    record_type: curr.record_type.clone(),
                    dates: curr.dates.clone(),
                    shape: WithdrawnShape::ArrivedWithdrawn {
                        revision: curr.revision.clone(),
                    },
                });
            }
            (None, CurrentChartEntry::Absent) => {}
            (
                Some(KnownChartEntry::Present { revision: prev }),
                CurrentChartEntry::Active(curr),
            ) => {
                if prev != &curr.revision {
                    corrected.push(CorrectedChartRecord {
                        candidate_id: curr.candidate_id.clone(),
                        previous_revision: prev.clone(),
                        current_revision: curr.revision.clone(),
                        source_id: curr.source_id.clone(),
                        record_type: curr.record_type.clone(),
                        dates: curr.dates.clone(),
                    });
                }
            }
            (
                Some(KnownChartEntry::Present { revision: prev }),
                CurrentChartEntry::Retracted(curr),
            ) => {
                withdrawn.push(WithdrawnChartRecord {
                    candidate_id: curr.candidate_id.clone(),
                    source_id: curr.source_id.clone(),
                    record_type: curr.record_type.clone(),
                    dates: curr.dates.clone(),
                    shape: WithdrawnShape::RetractedRevision {
                        previous_revision: prev.clone(),
                        current_revision: curr.revision.clone(),
                    },
                });
            }
            (Some(KnownChartEntry::Present { revision: prev }), CurrentChartEntry::Absent) => {
                withdrawn.push(WithdrawnChartRecord {
                    candidate_id: cid.to_owned(),
                    source_id,
                    record_type,
                    dates,
                    shape: WithdrawnShape::RemovedFromObservation {
                        withdrawn_revision: prev.clone(),
                    },
                });
            }
            (
                Some(KnownChartEntry::Withdrawn { retracted_revision }),
                CurrentChartEntry::Retracted(curr),
            ) => match retracted_revision {
                Some(prev) if prev == &curr.revision => {}
                Some(prev) => {
                    withdrawn.push(WithdrawnChartRecord {
                        candidate_id: curr.candidate_id.clone(),
                        source_id: curr.source_id.clone(),
                        record_type: curr.record_type.clone(),
                        dates: curr.dates.clone(),
                        shape: WithdrawnShape::RetractedRevision {
                            previous_revision: prev.clone(),
                            current_revision: curr.revision.clone(),
                        },
                    });
                }
                None => {
                    withdrawn.push(WithdrawnChartRecord {
                        candidate_id: curr.candidate_id.clone(),
                        source_id: curr.source_id.clone(),
                        record_type: curr.record_type.clone(),
                        dates: curr.dates.clone(),
                        shape: WithdrawnShape::ArrivedWithdrawn {
                            revision: curr.revision.clone(),
                        },
                    });
                }
            },
            (
                Some(KnownChartEntry::Withdrawn { retracted_revision }),
                CurrentChartEntry::Absent,
            ) => {
                if let Some(prev) = retracted_revision {
                    withdrawn.push(WithdrawnChartRecord {
                        candidate_id: cid.to_owned(),
                        source_id,
                        record_type,
                        dates,
                        shape: WithdrawnShape::RemovedFromObservation {
                            withdrawn_revision: prev.clone(),
                        },
                    });
                }
            }
        }
    }

    let mut must_read_again = Vec::new();
    for (cid, relied_state) in &relied_records {
        match current_map.get(cid.as_str()).copied() {
            None => {
                let confirmed = matches!(
                    relied_state,
                    ReliedRecordState::ConfirmedWithdrawnInLatestWindow
                );
                if !confirmed {
                    let withdrawn_revision =
                        last_opened_revision.get(cid).cloned().unwrap_or_default();
                    must_read_again.push(ReliedRecordRequirement {
                        candidate_id: cid.clone(),
                        action: ReliedRereadAction::ConfirmWithdrawnFromChart {
                            withdrawn_revision,
                        },
                    });
                }
            }
            Some(curr) => {
                let curr_retracted = is_retracted_source(curr);
                let baseline_unchanged = baseline_map.get(cid.as_str()).is_some_and(|base| {
                    base.revision == curr.revision && is_retracted_source(base) == curr_retracted
                });
                let unchanged_initial_read = matches!(
                    relied_state,
                    ReliedRecordState::OpenedInitial { revision }
                        if revision == &curr.revision && (baseline_unchanged || !curr_retracted)
                );
                if !unchanged_initial_read
                    && !record_completely_delivered(curr, &latest_window_receipts)
                {
                    must_read_again.push(ReliedRecordRequirement {
                        candidate_id: cid.clone(),
                        action: ReliedRereadAction::ReadCurrentRevision {
                            revision: curr.revision.clone(),
                        },
                    });
                }
            }
        }
    }

    if added.is_empty()
        && corrected.is_empty()
        && withdrawn.is_empty()
        && must_read_again.is_empty()
    {
        return Ok(ChatFinishCurrency::Current);
    }

    let (outcome, message) = if !added.is_empty() || !corrected.is_empty() || !withdrawn.is_empty()
    {
        let guidance = if must_read_again.is_empty() {
            "Patient chart records were added, corrected, or withdrawn while you were answering, so your reply was not published yet. Judge whether any added records affect the question, open relevant revisions with read_patient_sources if needed, update any affected patient memory or open concern, and then provide your complete updated answer."
        } else {
            "Patient chart records were added, corrected, or withdrawn while you were answering, so your reply was not published yet. Call read_patient_sources on every record in must_read_again, judge whether any added records affect the question, update or retire any affected patient memory or open concern, and then provide your complete updated answer."
        };
        (ChatArrivalOutcome::SourcesChanged, guidance.to_owned())
    } else {
        (
            ChatArrivalOutcome::SourcesUnread,
            "Records you relied on were corrected or withdrawn during this run and have not been read again yet. Call read_patient_sources on every entry in must_read_again before providing your updated answer.".to_owned(),
        )
    };

    Ok(ChatFinishCurrency::Refused(ChatChartArrivalNote {
        outcome,
        workspace_version: current_version,
        added,
        corrected,
        withdrawn,
        must_read_again,
        message,
    }))
}

/// Exact `(candidate_id, revision)` pairs delivered via `read_patient_sources`
/// (`Available` or `Passage` receipts) in `records`.
pub fn delivered_exact_revisions<'a>(
    records: impl IntoIterator<Item = impl Into<harness::engine::recipe::ReplayEvent<'a>>>,
    messages: &[harness::engine::ProductMessage],
) -> Result<std::collections::BTreeSet<(String, String)>, String> {
    use harness::engine::results;
    let delivered = results::delivered_originals(records, messages)
        .map_err(|cause| cause.to_string())?
        .into_iter()
        .filter_map(|output| serde_json::from_str::<chart::ReadReceipt>(output).ok())
        .flat_map(|receipt| receipt.results);
    let mut read = std::collections::BTreeSet::new();
    for item in delivered {
        match item {
            chart::RecordReceipt::Available {
                reference,
                revision,
                ..
            }
            | chart::RecordReceipt::Passage {
                reference,
                revision,
                ..
            } => {
                read.insert((reference, revision));
            }
            chart::RecordReceipt::Unavailable { .. } | chart::RecordReceipt::OverBudget { .. } => {}
        }
    }
    Ok(read)
}

/// A last page can be requested directly. Only the union of delivered exact
/// spans can establish that a changed record was read after a refresh.
fn delivered_part(source: &SourceRevision, part: usize, results: &[&chart::RecordReceipt]) -> bool {
    let (_, text) = chart::part(source, part).expect("existing source part");
    let mut spans: Vec<_> = results
        .iter()
        .filter_map(|receipt| match receipt {
            chart::RecordReceipt::Passage {
                part: delivered_part,
                start_byte,
                end_byte,
                content,
                ..
            } if *delivered_part == part && text.get(*start_byte..*end_byte) == Some(content) => {
                Some((*start_byte, *end_byte))
            }
            _ => None,
        })
        .collect();
    if spans.is_empty() {
        return false;
    }
    spans.sort_unstable();
    let mut through = 0;
    for (start, end) in spans {
        if start > through {
            return false;
        }
        through = through.max(end);
    }
    through == text.len()
}

/// The ordinary exact-read definition. No memory or delegation tools are
/// advertised by a reply revision.
pub fn read_definition() -> Value {
    read_tool(ChartScope::PinnedReply).function()
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness::engine::{Event, EventKind, Item};

    fn source(revision: &str, content: &str) -> SourceRevision {
        SourceRevision {
            workspace: "patient".into(),
            source_id: "ehr".into(),
            candidate_id: "lab".into(),
            revision: revision.into(),
            record_type: "Observation".into(),
            dates: vec![],
            content: content.into(),
            sha256: crate::sha256_hex(content.as_bytes()),
            readable_parts: vec![],
            provenance: json!({"native_record": "lab"}),
            envelope: None,
        }
    }

    fn accepted_result(
        events: &mut Vec<Event>,
        id: &str,
        original: &str,
        view: harness::engine::results::ResultView,
    ) {
        use harness::engine::{RequestItem, SavedRequest, Turn, results::ToolResult};
        if !events.iter().any(
            |event| matches!(&event.kind, EventKind::ToolReturned { call_id, .. } if call_id == id),
        ) {
            events.push(Event {
                attempt_id: "inline".into(),
                sequence: events.len() as u64 + 1,
                ordinal: events.len() as u64 + 1,
                kind: EventKind::ToolReturned {
                    call_id: id.into(),
                    result: ToolResult::Original(original.into()),
                },
            });
        }
        let sequence = events.len() as u64 + 1;
        events.push(Event {
            attempt_id: "inline".into(),
            sequence,
            ordinal: sequence,
            kind: EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                context: Default::default(),
                request: SavedRequest {
                    added: vec![Item::Result {
                        call_id: id.into(),
                        output: view,
                    }],
                    items: vec![RequestItem::Event {
                        attempt: "inline".into(),
                        sequence,
                        index: 0,
                    }],
                },
            },
        });
        events.push(Event {
            attempt_id: "inline".into(),
            sequence: sequence + 1,
            ordinal: sequence,
            kind: EventKind::ProviderReturned(Turn {
                items: vec![Item::assistant("Finish revision")],
                receipt: json!({}),
                usage: Default::default(),
            }),
        });
    }

    #[test]
    fn refreshed_source_requires_all_retained_result_pages() {
        use harness::engine::results::{ByteRange, Representation, ResultView};
        let current = source("r2", &"New clinical text. ".repeat(600));
        let refresh = SourceRefresh::between(
            1,
            &[source("r1", "Old text")],
            2,
            std::slice::from_ref(&current),
        );
        let original = exact_read(
            std::slice::from_ref(&current),
            "patient",
            &["lab".into()],
            65_536,
        )
        .unwrap()
        .text()
        .unwrap()
        .to_owned();
        let mut preview = ResultView::whole("source", &original);
        preview.representation = Representation::Preview {
            head: ByteRange { start: 0, end: 100 },
            tail: ByteRange {
                start: original.len() - 100,
                end: original.len(),
            },
        };
        let mut events = Vec::new();
        accepted_result(&mut events, "source", &original, preview);
        assert_eq!(
            caught_up(
                &events,
                std::slice::from_ref(&refresh),
                std::slice::from_ref(&current)
            )
            .unwrap(),
            vec!["lab"]
        );
        let middle = original.len() / 2;
        for (n, (start, end)) in [(0, middle), (middle, original.len())]
            .into_iter()
            .enumerate()
        {
            let mut page = ResultView::whole("source", &original);
            page.representation = Representation::Page {
                range: ByteRange { start, end },
            };
            accepted_result(&mut events, "source", &original, page);
            let unread = caught_up(
                &events,
                std::slice::from_ref(&refresh),
                std::slice::from_ref(&current),
            )
            .unwrap();
            assert_eq!(
                unread.is_empty(),
                n == 1,
                "full delivered pages must satisfy the refreshed source read"
            );
        }
    }

    #[test]
    fn a_refreshed_source_from_an_earlier_window_is_not_current_delivery() {
        use harness::engine::results::ResultView;
        let current = source("r2", "New clinical text");
        let refresh = SourceRefresh::between(0, &[], 1, std::slice::from_ref(&current));
        let original = exact_read(
            std::slice::from_ref(&current),
            "patient",
            &["lab".into()],
            65_536,
        )
        .unwrap()
        .text()
        .unwrap()
        .to_owned();
        let mut events = Vec::new();
        accepted_result(
            &mut events,
            "source",
            &original,
            ResultView::whole("source", &original),
        );
        assert!(
            caught_up(
                &events,
                std::slice::from_ref(&refresh),
                std::slice::from_ref(&current)
            )
            .unwrap()
            .is_empty()
        );
        events.push(Event {
            attempt_id: "inline".into(),
            sequence: events.len() as u64 + 1,
            ordinal: 2,
            kind: EventKind::WindowOpened {
                opened_at_unix_ms: 0,
                reason: harness::engine::WindowReason::Full,
                handoff: None,
            },
        });
        let sequence = events.len() as u64 + 1;
        events.extend([
            Event {
                attempt_id: "inline".into(),
                sequence,
                ordinal: 3,
                kind: EventKind::ProviderStarted {
                    dispatched_at_unix_ms: 2,
                    context: Default::default(),
                    request: harness::engine::SavedRequest {
                        added: vec![],
                        items: vec![],
                    },
                },
            },
            Event {
                attempt_id: "inline".into(),
                sequence: sequence + 1,
                ordinal: 3,
                kind: EventKind::ProviderReturned(harness::engine::Turn {
                    items: vec![Item::assistant("Finish revision.")],
                    receipt: json!({}),
                    usage: Default::default(),
                }),
            },
        ]);
        assert_eq!(
            caught_up(&events, &[refresh], &[current]).unwrap(),
            vec!["lab"],
            "a refreshed source must be delivered in the current window before finishing"
        );
    }

    #[test]
    fn catch_up_requires_exact_bytes_hash_and_provenance() {
        let current = source("r1", "Potassium 6.8");
        let refresh = SourceRefresh::between(0, &[], 1, std::slice::from_ref(&current));
        let exact = exact_read(
            std::slice::from_ref(&current),
            "patient",
            &["lab".into()],
            65536,
        )
        .unwrap();
        let exact: Value = serde_json::from_str(exact.text().unwrap()).unwrap();
        let check = |value: serde_json::Value| {
            let original = value.to_string();
            let mut events = Vec::new();
            accepted_result(
                &mut events,
                "read",
                &original,
                harness::engine::results::ResultView::whole("read", &original),
            );
            caught_up(
                &events,
                std::slice::from_ref(&refresh),
                std::slice::from_ref(&current),
            )
            .unwrap()
        };
        assert!(check(exact.clone()).is_empty());
        let split = |start: usize, end: usize| {
            json!({"reference":"lab","revision":current.revision,
            "outcome":"passage","part":0,"label":"native_record","start_byte":start,"end_byte":end,
            "total_bytes":current.content.len(),"next":null,"content":&current.content[start..end]})
        };
        let end = current.content.len();
        assert_eq!(check(json!({"results":[split(5,end)]})), vec!["lab"]);
        assert_eq!(
            check(json!({"results":[split(0,3),split(5,end)]})),
            vec!["lab"]
        );
        assert!(check(json!({"results":[split(5,end),split(0,5)]})).is_empty());
        for (field, wrong) in [
            ("content", json!("Potassium 4.1")),
            ("sha256", json!(crate::sha256_hex(b"Potassium 4.1"))),
            ("provenance", json!({"native_record": "another-lab"})),
            ("read_representation", json!("summary")),
        ] {
            let mut stale = exact.clone();
            stale["results"][0][field] = wrong;
            assert_eq!(check(stale), vec!["lab".to_string()], "{field}");
        }
        let stale = exact_read(
            &[source("r1", "Potassium 4.1")],
            "patient",
            &["lab".into()],
            65536,
        )
        .unwrap();
        let stale: Value = serde_json::from_str(stale.text().unwrap()).unwrap();
        assert_eq!(check(stale), vec!["lab".to_string()]);
    }

    #[test]
    fn catch_up_accepts_exact_read_of_source_with_filed_envelope() {
        let mut current = source("r2", r#"{"resourceType":"Observation","id":"k-1"}"#);
        current.envelope = Some(RecordEnvelope {
            bucket: Bucket::Results,
            source_kind: SourceRecordKind {
                kind: "Observation".into(),
                category: Some("laboratory".into()),
            },
            native_id: "k-1".into(),
            version: RecordVersion::Source("2".into()),
            dates: vec![FiledDate {
                role: DateRole::Effective,
                field: "effectiveDateTime".into(),
                value: "2026-08-21T06:15:00-04:00".into(),
            }],
            status: Some("final".into()),
            codes: Vec::new(),
            title: Some("Potassium".into()),
            native_link: None,
            unavailable_parts: Vec::new(),
        });
        let refresh = SourceRefresh::between(0, &[], 1, std::slice::from_ref(&current));
        let exact = exact_read(
            std::slice::from_ref(&current),
            "patient",
            &["lab".into()],
            65536,
        )
        .unwrap();
        let original = exact.text().unwrap().to_owned();
        let mut events = Vec::new();
        accepted_result(
            &mut events,
            "read",
            &original,
            harness::engine::results::ResultView::whole("read", &original),
        );
        assert!(
            caught_up(
                &events,
                std::slice::from_ref(&refresh),
                std::slice::from_ref(&current),
            )
            .unwrap()
            .is_empty()
        );
    }

    fn named_source(candidate_id: &str, revision: &str, content: &str) -> SourceRevision {
        SourceRevision {
            workspace: "patient".into(),
            source_id: "ehr".into(),
            candidate_id: candidate_id.into(),
            revision: revision.into(),
            record_type: "Observation".into(),
            dates: vec![RecordDate {
                name: "effectiveDate".into(),
                value: "2026-11-12".into(),
            }],
            content: content.into(),
            sha256: crate::sha256_hex(content.as_bytes()),
            readable_parts: vec![],
            provenance: json!({"native_record": candidate_id}),
            envelope: None,
        }
    }

    fn accepted_refusal(events: &mut Vec<Event>, note: &ChatChartArrivalNote) {
        let sequence = events.len() as u64 + 1;
        let refusal = harness::engine::recipe::Refusal::new(note.to_json_string());
        events.push(Event {
            attempt_id: "inline".into(),
            sequence,
            ordinal: sequence,
            kind: EventKind::ProviderStarted {
                dispatched_at_unix_ms: 2,
                context: Default::default(),
                request: harness::engine::SavedRequest {
                    added: vec![Item::FinishRefused(refusal)],
                    items: vec![harness::engine::RequestItem::Event {
                        attempt: "inline".into(),
                        sequence,
                        index: 0,
                    }],
                },
            },
        });
        events.push(Event {
            attempt_id: "inline".into(),
            sequence: sequence + 1,
            ordinal: sequence,
            kind: EventKind::ProviderReturned(harness::engine::Turn {
                items: vec![Item::assistant("Updated reply")],
                receipt: json!({}),
                usage: Default::default(),
            }),
        });
    }

    #[test]
    fn chat_finish_allows_unchanged_partial_passage_and_judged_added_records() {
        let long_note = named_source("note-1", "r1", &"Clinical note text. ".repeat(40));
        let passage = json!({
            "results": [{
                "outcome": "passage",
                "reference": "note-1",
                "revision": "r1",
                "part": 0,
                "label": "native_record",
                "start_byte": 0,
                "end_byte": 20,
                "total_bytes": long_note.content.len(),
                "next": {"candidate_id": "note-1", "revision": "r1", "cursor": {"part": 0, "offset": 20}},
                "content": &long_note.content[..20]
            }]
        })
        .to_string();
        let mut events = Vec::new();
        accepted_result(
            &mut events,
            "read-note-part",
            &passage,
            harness::engine::results::ResultView::whole("read-note-part", &passage),
        );

        assert_eq!(
            evaluate_chat_finish_currency(
                std::slice::from_ref(&long_note),
                1,
                std::slice::from_ref(&long_note),
                events
                    .iter()
                    .map(harness::engine::recipe::ReplayEvent::Admitted),
                &[],
            )
            .unwrap(),
            ChatFinishCurrency::Current,
            "unchanged record opened as a partial passage must commit without extra requests"
        );

        let added_derm = named_source("derm-2", "r1", "Unrelated skin biopsy benign");
        let current = vec![long_note.clone(), added_derm];
        let ChatFinishCurrency::Refused(note) = evaluate_chat_finish_currency(
            std::slice::from_ref(&long_note),
            2,
            &current,
            events
                .iter()
                .map(harness::engine::recipe::ReplayEvent::Admitted),
            &[],
        )
        .unwrap() else {
            panic!("expected arrival note when a record is added mid-answer");
        };
        assert_eq!(note.outcome, ChatArrivalOutcome::SourcesChanged);
        assert_eq!(note.added.len(), 1);
        assert_eq!(note.added[0].candidate_id, "derm-2");
        assert!(note.must_read_again.is_empty());

        accepted_refusal(&mut events, &note);
        assert_eq!(
            evaluate_chat_finish_currency(
                std::slice::from_ref(&long_note),
                2,
                &current,
                events
                    .iter()
                    .map(harness::engine::recipe::ReplayEvent::Admitted),
                &[],
            )
            .unwrap(),
            ChatFinishCurrency::Current,
            "model judges added records itself once notified"
        );
    }

    #[test]
    fn chat_finish_enforces_reread_for_relied_corrected_and_withdrawn_records() {
        let lab_r1 = named_source("lab-k", "r1", "Potassium 6.2 mmol/L (hemolyzed)");
        let lab_r2 = named_source("lab-k", "r2", "Potassium 4.1 mmol/L (repeat)");
        let read_r1 = exact_read(
            std::slice::from_ref(&lab_r1),
            "patient",
            &["lab-k".into()],
            65_536,
        )
        .unwrap()
        .text()
        .unwrap()
        .to_owned();
        let mut events = Vec::new();
        accepted_result(
            &mut events,
            "read-r1",
            &read_r1,
            harness::engine::results::ResultView::whole("read-r1", &read_r1),
        );

        let ChatFinishCurrency::Refused(first_note) = evaluate_chat_finish_currency(
            std::slice::from_ref(&lab_r1),
            2,
            std::slice::from_ref(&lab_r2),
            events
                .iter()
                .map(harness::engine::recipe::ReplayEvent::Admitted),
            &[],
        )
        .unwrap() else {
            panic!("expected refusal when relied-on lab is corrected");
        };
        assert_eq!(first_note.outcome, ChatArrivalOutcome::SourcesChanged);
        assert_eq!(first_note.corrected.len(), 1);
        assert_eq!(
            first_note.must_read_again,
            vec![ReliedRecordRequirement {
                candidate_id: "lab-k".into(),
                action: ReliedRereadAction::ReadCurrentRevision {
                    revision: "r2".into(),
                },
            }]
        );

        accepted_refusal(&mut events, &first_note);
        let ChatFinishCurrency::Refused(second_note) = evaluate_chat_finish_currency(
            std::slice::from_ref(&lab_r1),
            2,
            std::slice::from_ref(&lab_r2),
            events
                .iter()
                .map(harness::engine::recipe::ReplayEvent::Admitted),
            &[],
        )
        .unwrap() else {
            panic!("skipping read_patient_sources on relied-on correction must still refuse");
        };
        assert_eq!(second_note.outcome, ChatArrivalOutcome::SourcesUnread);
        assert_eq!(second_note.must_read_again.len(), 1);

        let read_r2 = exact_read(
            std::slice::from_ref(&lab_r2),
            "patient",
            &["lab-k".into()],
            65_536,
        )
        .unwrap()
        .text()
        .unwrap()
        .to_owned();
        accepted_result(
            &mut events,
            "read-r2",
            &read_r2,
            harness::engine::results::ResultView::whole("read-r2", &read_r2),
        );
        assert_eq!(
            evaluate_chat_finish_currency(
                std::slice::from_ref(&lab_r1),
                2,
                std::slice::from_ref(&lab_r2),
                events
                    .iter()
                    .map(harness::engine::recipe::ReplayEvent::Admitted),
                &[],
            )
            .unwrap(),
            ChatFinishCurrency::Current
        );

        // Now withdraw `lab-k` from observation (`current = []`): requires post-withdrawal read!
        let ChatFinishCurrency::Refused(withdrawn_note) = evaluate_chat_finish_currency(
            std::slice::from_ref(&lab_r1),
            3,
            &[],
            events
                .iter()
                .map(harness::engine::recipe::ReplayEvent::Admitted),
            &[],
        )
        .unwrap() else {
            panic!("expected refusal when relied-on lab is withdrawn");
        };
        assert_eq!(withdrawn_note.withdrawn.len(), 1);
        assert_eq!(
            withdrawn_note.withdrawn[0].shape,
            WithdrawnShape::RemovedFromObservation {
                withdrawn_revision: "r2".into(),
            }
        );
        assert_eq!(
            withdrawn_note.must_read_again,
            vec![ReliedRecordRequirement {
                candidate_id: "lab-k".into(),
                action: ReliedRereadAction::ConfirmWithdrawnFromChart {
                    withdrawn_revision: "r2".into(),
                },
            }]
        );
        accepted_refusal(&mut events, &withdrawn_note);
        assert!(matches!(
            evaluate_chat_finish_currency(
                std::slice::from_ref(&lab_r1),
                3,
                &[],
                events
                    .iter()
                    .map(harness::engine::recipe::ReplayEvent::Admitted),
                &[],
            )
            .unwrap(),
            ChatFinishCurrency::Refused(ChatChartArrivalNote {
                outcome: ChatArrivalOutcome::SourcesUnread,
                ..
            })
        ));

        let missing_receipt = json!({
            "results": [{
                "outcome": "unavailable",
                "reference": "lab-k",
                "reason": "not_in_recorded_observation"
            }]
        })
        .to_string();
        accepted_result(
            &mut events,
            "read-withdrawn",
            &missing_receipt,
            harness::engine::results::ResultView::whole("read-withdrawn", &missing_receipt),
        );
        assert_eq!(
            evaluate_chat_finish_currency(
                std::slice::from_ref(&lab_r1),
                3,
                &[],
                events
                    .iter()
                    .map(harness::engine::recipe::ReplayEvent::Admitted),
                &[],
            )
            .unwrap(),
            ChatFinishCurrency::Current
        );
    }
}
