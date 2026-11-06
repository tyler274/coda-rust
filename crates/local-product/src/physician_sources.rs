//! The record every patient source hands Coda, whatever its format. Each source
//! has its own adapter; past the adapter a record is one of these. Imports must
//! come from a source adapter, never projection prose or a provider. This
//! reader neither discovers new records nor claims that a snapshot is a
//! complete chart.
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
    async fn patient_ids(&self) -> Result<Vec<String>, String>;
    /// One patient's identity, read alone so one bad patient stays one patient.
    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, String>;
    /// One patient's records, admitted to `workspace`.
    async fn records(
        &self,
        patient: &SourcePatient,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, String>;
}

/// The stable workspace of one source patient. It carries no patient
/// identifier, because workspace ids travel in URLs.
pub fn workspace_id(source_id: &str, source_patient_id: &str) -> String {
    let digest = crate::sha256_hex(format!("{source_id}|{source_patient_id}").as_bytes());
    format!("pw-{}", &digest[..32])
}

/// Exact, per-reference outcomes with a byte bound. A missing reference never
/// means no clinical information exists. No source version is silently selected.
pub fn exact_read(
    sources: &[SourceRevision],
    workspace: &str,
    references: &[String],
    max_bytes: usize,
) -> Result<Value, String> {
    if references.is_empty() || references.len() > 8 {
        return Err("read requires one to eight exact references".into());
    }
    let budget = max_bytes.clamp(4096, 65536);
    let mut results = Vec::new();
    for reference in references {
        let result = match sources
            .iter()
            .find(|s| s.workspace == workspace && s.candidate_id == *reference)
        {
            Some(source) => {
                source.validate()?;
                json!({"reference": reference, "outcome": "available", "revision": source.revision,
                    "record_type": source.record_type, "dates": source.dates,
                    "read_representation": "native_record", "content": source.content,
                    "readable_parts": source.readable_parts,
                    "sha256": source.sha256, "provenance": source.provenance,
                    "citation_id": citation_id(source)})
            }
            None => {
                json!({"reference": reference, "outcome": "unavailable", "reason": "not_in_recorded_observation"})
            }
        };
        results.push(result);
        if serde_json::to_vec(&json!({"results": results}))
            .map_err(|e| e.to_string())?
            .len()
            > budget
        {
            *results.last_mut().unwrap() =
                json!({"reference": reference, "outcome": "over_budget"});
        }
    }
    Ok(json!({"results": results}))
}

/// Opaque citation identity for one exact record, including its provenance.
pub fn citation_id(source: &SourceRevision) -> String {
    crate::sha256_hex(&serde_json::to_vec(source).expect("source serialization"))
}

/// Bounded discovery over the recorded chart, not a claim of chart completeness.
/// Metadata identifies candidates; only an exact read grants citation authority.
pub fn search(sources: &[SourceRevision], workspace: &str, query: &str, offset: usize) -> Value {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    let matches: Vec<_> = sources
        .iter()
        .filter(|source| {
            source.workspace == workspace
                && words.iter().all(|word| {
                    source.content.to_lowercase().contains(word)
                        || source.record_type.to_lowercase().contains(word)
                        || source
                            .readable_parts
                            .iter()
                            .any(|part| part.text.to_lowercase().contains(word))
                })
        })
        .collect();
    let records: Vec<_> = matches
        .iter()
        .skip(offset)
        .take(20)
        .map(|source| {
            json!({"reference": source.candidate_id, "revision": source.revision,
            "record_type": source.record_type, "dates": source.dates})
        })
        .collect();
    let next = offset.saturating_add(records.len());
    json!({"records": records, "next_offset": (next < matches.len()).then_some(next),
        "coverage": "recorded_sources_only", "citation_authority": false})
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
    /// How many execution records the work had retained when it was
    /// refreshed. Only a model call dispatched after this point may finish.
    #[serde(default)]
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
pub fn caught_up(
    records: &[harness::execution::Record],
    refreshes: &[SourceRefresh],
    current: &[SourceRevision],
) -> Result<Vec<String>, String> {
    use harness::execution::{Item, Record};
    let Some(latest) = refreshes.last() else {
        return Ok(vec![]);
    };
    let Some((index, context)) = records
        .iter()
        .enumerate()
        .rev()
        .find_map(|(index, record)| match record {
            Record::Dispatch { request, .. } => Some((index, &request.items)),
            _ => None,
        })
    else {
        return Err("no model call followed the source refresh".into());
    };
    if index < latest.after_records {
        return Err("no model call followed the source refresh".into());
    }
    let mut unread = Vec::new();
    for reference in refreshes.iter().flat_map(SourceRefresh::to_read) {
        let Some(source) = current.iter().find(|s| s.candidate_id == reference) else {
            continue; // withdrawn again since
        };
        let read = context.iter().any(|item| match item {
            Item::Result { value, .. } => value["results"].as_array().is_some_and(|results| {
                results.iter().any(|r| {
                    r["reference"] == source.candidate_id.as_str()
                        && r["outcome"] == "available"
                        && r["revision"] == source.revision.as_str()
                        && r["read_representation"] == "native_record"
                        && r["content"] == source.content.as_str()
                        && r["sha256"] == source.sha256.as_str()
                        && r["provenance"] == source.provenance
                })
            }),
            _ => false,
        });
        if !read && !unread.contains(&reference) {
            unread.push(reference);
        }
    }
    Ok(unread)
}

/// A versioned in-memory source observation for the single-process inline
/// path. The version moves only when a recorded observation changes, the
/// same rule as the hosted runtime's source store. A stand-in for a source
/// owner, seeded from the retained snapshot; not production ingestion.
#[derive(Debug, Clone, Default)]
pub struct Observation {
    version: u64,
    sources: Vec<SourceRevision>,
    // Retained after correction and withdrawal so a replay cannot roll back
    // the observation or redefine an immutable revision identity.
    history: Vec<SourceRevision>,
}

impl Observation {
    /// Seeds version 1 with validated, uniquely identified sources (version 0
    /// when empty).
    pub fn seeded(sources: Vec<SourceRevision>) -> Result<Self, String> {
        let mut observation = Self::default();
        for source in sources {
            observation.record(source)?;
        }
        observation.version = u64::from(!observation.sources.is_empty());
        Ok(observation)
    }

    /// Records an unseen revision, including restoration after withdrawal.
    /// Identical historical revisions are no-ops; conflicting content or
    /// provenance for the same identity is refused. Returns whether changed.
    pub fn record(&mut self, source: SourceRevision) -> Result<bool, String> {
        source.validate()?;
        if let Some(recorded) = self.history.iter().find(|recorded| {
            recorded.workspace == source.workspace
                && recorded.candidate_id == source.candidate_id
                && recorded.revision == source.revision
        }) {
            return if *recorded == source {
                Ok(false)
            } else {
                Err("conflicting source revision identity".into())
            };
        }
        self.history.push(source.clone());
        match self
            .sources
            .iter_mut()
            .find(|s| s.workspace == source.workspace && s.candidate_id == source.candidate_id)
        {
            Some(existing) => *existing = source,
            None => self.sources.push(source),
        }
        self.version += 1;
        Ok(true)
    }

    /// Withdraws a source. Returns whether it was present.
    pub fn withdraw(&mut self, workspace: &str, candidate_id: &str) -> bool {
        let before = self.sources.len();
        self.sources
            .retain(|s| !(s.workspace == workspace && s.candidate_id == candidate_id));
        let changed = self.sources.len() != before;
        if changed {
            self.version += 1;
        }
        changed
    }

    /// The current version and this workspace's sources.
    pub fn workspace(&self, workspace: &str) -> (u64, Vec<SourceRevision>) {
        (
            self.version,
            self.sources
                .iter()
                .filter(|s| s.workspace == workspace)
                .cloned()
                .collect(),
        )
    }
}

/// The ordinary exact-read definition. No memory or delegation tools are
/// advertised by a reply revision.
pub fn read_definition() -> Value {
    json!({"type":"function", "name":"read_patient_sources", "description":"Read exact retained source revisions from this work's recorded patient source observation. Unavailable is not absence. No search or projection grants citation authority.",
        "parameters":{"type":"object", "properties":{"references":{"type":"array","items":{"type":"string"},"minItems":1,"maxItems":8},"max_result_bytes":{"type":"integer","minimum":4096,"maximum":65536}},"required":["references","max_result_bytes"],"additionalProperties":false}})
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness::execution::{Item, Record, Request};

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
        }
    }

    #[test]
    fn discovery_is_scoped_and_exact_reads_name_immutable_citations() {
        let original = source("r1", "Potassium 4.1");
        let mut other = source("r2", "Potassium 9.9");
        other.workspace = "another-patient".into();
        let all = vec![original.clone(), other];
        let found = search(&all, "patient", "potassium", 0);
        assert_eq!(found["records"].as_array().unwrap().len(), 1);
        assert_eq!(found["records"][0]["revision"], "r1");
        let read = exact_read(&all, "patient", &["lab".into()], 4096).unwrap();
        assert_eq!(read["results"][0]["citation_id"], citation_id(&original));
        assert_eq!(read["results"][0]["content"], original.content);
        assert_ne!(
            citation_id(&original),
            citation_id(&source("r2", "Potassium 6.8"))
        );
        assert!(
            search(&all, "unknown", "", 0)["records"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn inline_historical_reimports_do_not_rollback_or_restore() {
        let original = source("r1", "Potassium 4.1");
        let corrected = source("r2", "Potassium 6.8");
        let mut observation = Observation::seeded(vec![original.clone()]).unwrap();
        assert!(observation.record(corrected.clone()).unwrap());
        let before = observation.workspace("patient");
        assert!(!observation.record(original.clone()).unwrap());
        assert_eq!(observation.workspace("patient"), before);
        assert!(observation.withdraw("patient", "lab"));
        let withdrawn = observation.workspace("patient");
        for revision in [original, corrected] {
            assert!(!observation.record(revision).unwrap());
            assert_eq!(observation.workspace("patient"), withdrawn);
        }
        let restored = source("r3", "Potassium 4.2");
        assert!(observation.record(restored.clone()).unwrap());
        assert_eq!(observation.workspace("patient"), (4, vec![restored]));
    }

    #[test]
    fn inline_revision_identity_is_immutable_even_after_withdrawal() {
        let original = source("r1", "Potassium 4.1");
        let mut observation = Observation::seeded(vec![original.clone()]).unwrap();
        for phase in 0..3 {
            let before = observation.workspace("patient");
            let changed_bytes = source("r1", "Potassium 6.8");
            let mut changed_provenance = original.clone();
            changed_provenance.provenance = json!({"native_record": "another-lab"});
            for conflict in [changed_bytes, changed_provenance] {
                assert!(observation.record(conflict).is_err());
                assert_eq!(observation.workspace("patient"), before);
            }
            if phase == 0 {
                observation.record(source("r2", "Corrected")).unwrap();
            } else {
                observation.withdraw("patient", "lab");
            }
        }
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
        let check = |value| {
            caught_up(
                &[Record::Dispatch {
                    sequence: 2,
                    request: Request {
                        model: "fixed".into(),
                        options: json!({}),
                        instructions: String::new(),
                        tools: vec![],
                        items: vec![Item::Result {
                            call_id: "read".into(),
                            value,
                        }],
                    },
                }],
                std::slice::from_ref(&refresh),
                std::slice::from_ref(&current),
            )
            .unwrap()
        };
        assert!(check(exact.clone()).is_empty());
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
        assert_eq!(check(stale), vec!["lab".to_string()]);
    }
}
