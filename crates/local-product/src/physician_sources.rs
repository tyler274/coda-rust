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

/// The ordinary exact-read definition. No memory or delegation tools are
/// advertised by a reply revision.
pub fn read_definition() -> Value {
    json!({"type":"function", "name":"read_patient_sources", "description":"Read exact retained source revisions from this work's recorded patient source observation. Unavailable is not absence. No search or projection grants citation authority.",
        "parameters":{"type":"object", "properties":{"references":{"type":"array","items":{"type":"string"},"minItems":1,"maxItems":8},"max_result_bytes":{"type":"integer","minimum":4096,"maximum":65536}},"required":["references","max_result_bytes"],"additionalProperties":false}})
}
