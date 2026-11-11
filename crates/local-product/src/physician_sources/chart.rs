//! On-demand discovery and bounded reads of the physician's workspace.
use super::{READ_TOOL, SourceRevision};
use harness::engine::{Tool, ToolOutcome};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Search {
    query: String,
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
    candidate_id: String,
    revision: String,
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
        dates: Vec<super::RecordDate>,
        read_representation: ReadRepresentation,
        content: String,
        readable_parts: Vec<super::ReadablePart>,
        sha256: String,
        provenance: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native: Option<super::NativeLink>,
        /// The identity an answer cites. Only a whole record carries one; a
        /// passage is not citable. Receipts recorded before citations existed
        /// have none.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        citation_id: Option<String>,
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
    #[error("Supply query, optional record_type and the returned after cursor ({0}).")]
    Search(#[source] serde_json::Error),
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
                let search: Search =
                    serde_json::from_value(arguments.clone()).map_err(ChartInputError::Search)?;
                if search.query.len() > 512 {
                    return Err(ChartInputError::Refused(
                        "Use at most 512 bytes of search text.",
                    ));
                }
                Operation::Search(search)
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

    /// Resolve only this workspace's accepted source revisions.
    pub fn run(
        &self,
        workspace: &str,
        version: u64,
        sources: &[SourceRevision],
    ) -> Result<ToolOutcome, String> {
        match &self.0 {
            Operation::Search(search) => search.page(workspace, version, sources),
            Operation::Read(read) => read
                .page(workspace, sources)
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
            .filter(|s| {
                if s.workspace != workspace
                    || self
                        .record_type
                        .as_ref()
                        .is_some_and(|kind| !kind.eq_ignore_ascii_case(&s.record_type))
                    || self
                        .after
                        .as_ref()
                        .is_some_and(|c| s.candidate_id <= c.candidate_id)
                {
                    return false;
                }
                if query.trim().is_empty() {
                    return true;
                }
                let fields: Vec<_> = [&s.record_type, &s.candidate_id, &s.content]
                    .into_iter()
                    .chain(s.readable_parts.iter().map(|p| &p.text))
                    .chain(s.dates.iter().map(|d| &d.value))
                    .map(|s| s.to_lowercase())
                    .collect();
                query
                    .split_whitespace()
                    .all(|term| fields.iter().any(|f| f.contains(term)))
            })
            .collect();
        matches.sort_unstable_by_key(|s| &s.candidate_id);
        let mut results = Vec::new();
        let mut next = None;
        for source in matches.iter().take(SEARCH_PAGE) {
            let hit = json!({"reference":Exact {candidate_id:source.candidate_id.clone(),revision:source.revision.clone(),cursor:None}, "record_type":source.record_type,"source_id":source.source_id,"dates":source.dates});
            results.push(hit);
            let continuation = After {
                version,
                candidate_id: source.candidate_id.clone(),
            };
            if json!({"results":results,"next":continuation})
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
        Ok(ToolOutcome::Returned(
            json!({"results":results,"next":next}).to_string(),
        ))
    }
}

impl Read {
    fn page(&self, workspace: &str, sources: &[SourceRevision]) -> Result<Value, String> {
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
                        Reference::Exact(r) => read_record(source, r.cursor, budget),
                        Reference::Pinned(_) => read_record(source, None, budget),
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

fn read_record(source: &SourceRevision, cursor: Option<Position>, budget: usize) -> RecordReceipt {
    if cursor.is_none() && source.content.len() <= budget {
        let full = RecordReceipt::Available {
            reference: source.candidate_id.clone(),
            revision: source.revision.clone(),
            record_type: source.record_type.clone(),
            dates: source.dates.clone(),
            read_representation: ReadRepresentation::NativeRecord,
            content: source.content.clone(),
            readable_parts: source.readable_parts.clone(),
            sha256: source.sha256.clone(),
            provenance: source.provenance.clone(),
            native: source.native.clone(),
            citation_id: Some(super::citation_id(source)),
        };
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
    Tool {
        name: SEARCH_TOOL.into(),
        description: "Browse or search the currently retained chart for this patient. All query words must match the native record, decoded text, record type, ID or dates. Use an empty query to browse. Results are metadata in candidate order, not evidence of clinical absence. Continue the same search with next as after. Open a returned reference to read evidence.".into(),
        parameters: json!({
            "type":"object",
            "properties":{
                "query":{"type":"string"},
                "record_type":{"type":["string","null"]},
                "after":{"anyOf":[{"type":"null"}, {
                    "type":"object",
                    "properties":{"version":{"type":"integer"},"candidate_id":{"type":"string"}},
                    "required":["version","candidate_id"],"additionalProperties":false
                }]}
            },
            "required":["query","record_type","after"],"additionalProperties":false
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
