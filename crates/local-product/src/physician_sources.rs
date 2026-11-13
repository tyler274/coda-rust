//! The record every patient source hands Coda, whatever its format. Each source
//! has its own adapter; past the adapter a record is one of these. Imports must
//! come from a source adapter, never projection prose or a provider. This
//! reader neither discovers new records nor claims that a snapshot is a
//! complete chart.
/// Declared adapter mapping tables and shared record filing.
pub mod adapter;
mod chart;
/// Shared domain types for chart buckets, filed envelopes, and pull outcomes.
pub mod types;
pub use chart::{ChartInputError, ChartRequest, ChartScope, SEARCH_TOOL, read_tool, search_tool};
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
        let results: Vec<_> = delivered
            .iter()
            .filter(|receipt| receipt.names(source))
            .collect();
        let read = results.iter().any(|receipt| matches!(receipt,
            chart::RecordReceipt::Available { content, sha256, provenance, .. }
                if content == &source.content && sha256 == &source.sha256 && provenance == &source.provenance
        )) || (0..=source.readable_parts.len())
            .all(|part| delivered_part(source, part, &results));
        if !read && !unread.contains(&reference) {
            unread.push(reference);
        }
    }
    Ok(unread)
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
}
