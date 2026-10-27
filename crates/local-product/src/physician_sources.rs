//! Retained physician source snapshots. Imports must come from the physician
//! source owner, never projection prose or a provider. This reader neither
//! discovers new records nor claims that a snapshot is a complete chart.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// One immutable accepted source revision from the physician source owner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRevision {
    /// Patient workspace to which the source owner admitted these bytes.
    pub workspace: String,
    /// Original opaque candidate ID, never derived from prose.
    pub candidate_id: String,
    /// Source owner's immutable revision identity.
    pub revision: String,
    /// Exact source-native retained UTF-8 representation.
    pub content: String,
    /// SHA-256 of retained content bytes, verified on import and read.
    pub sha256: String,
    /// Source-owned provenance, including native record identity.
    pub provenance: Value,
}
impl SourceRevision {
    /// Refuses corrupt or unidentified source evidence.
    pub fn validate(&self) -> Result<(), String> {
        if self.workspace.is_empty()
            || self.candidate_id.is_empty()
            || self.revision.is_empty()
            || crate::sha256_hex(self.content.as_bytes()) != self.sha256
            || self.provenance.is_null()
        {
            return Err("invalid retained physician source revision".into());
        }
        Ok(())
    }
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
                    "read_representation": "native_record", "content": source.content,
                    "sha256": source.sha256, "provenance": source.provenance})
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

    /// Records a new or corrected revision. Returns whether it changed the
    /// observation.
    pub fn record(&mut self, source: SourceRevision) -> Result<bool, String> {
        source.validate()?;
        match self
            .sources
            .iter_mut()
            .find(|s| s.workspace == source.workspace && s.candidate_id == source.candidate_id)
        {
            Some(existing) if *existing == source => return Ok(false),
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
