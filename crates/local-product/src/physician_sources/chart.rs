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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    location: Option<super::read::RecordLocation>,
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
        superseded_by: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location_limit: Option<String>,
    },
    Passage {
        reference: String,
        revision: String,
        part: usize,
        label: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        part_identity: Option<super::read::PartIdentity>,
        start_byte: usize,
        end_byte: usize,
        total_bytes: usize,
        content: String,
        next: Option<Box<Exact>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        superseded_by: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location_limit: Option<String>,
    },
    MissingPart {
        reference: String,
        revision: String,
        part: usize,
        part_identity: super::read::PartIdentity,
        detail: String,
        next: Option<Box<Exact>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        superseded_by: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location_limit: Option<String>,
    },
    UnreadablePart {
        reference: String,
        revision: String,
        part: usize,
        part_identity: super::read::PartIdentity,
        reason: String,
        next: Option<Box<Exact>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        superseded_by: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location_limit: Option<String>,
    },
    UnavailablePart {
        reference: String,
        revision: String,
        part: usize,
        part_identity: super::read::PartIdentity,
        reason: String,
        next: Option<Box<Exact>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        superseded_by: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location_limit: Option<String>,
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
            }
            | Self::MissingPart {
                reference,
                revision,
                ..
            }
            | Self::UnreadablePart {
                reference,
                revision,
                ..
            }
            | Self::UnavailablePart {
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
                if read.references.iter().any(|r| match r {
                    Reference::Pinned(_) => false,
                    Reference::Exact(exact) => exact.cursor.is_some() && exact.location.is_some(),
                }) {
                    return Err(ChartInputError::Refused(
                        "Supply either cursor or location on a reference, not both.",
                    ));
                }
                if read.references.iter().any(|r| match r {
                    Reference::Pinned(_) => false,
                    Reference::Exact(exact) => {
                        exact
                            .source_id
                            .as_ref()
                            .is_some_and(|sid| sid.trim().is_empty())
                            || exact
                                .location
                                .as_ref()
                                .is_some_and(|loc| !valid_location_identifiers(loc))
                    }
                }) {
                    return Err(ChartInputError::Refused(
                        "Reference source_id and location identifiers must not be empty.",
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

    /// Exact `(candidate_id, revision)` pairs requested by a read call, so
    /// retained historical revisions can be loaded alongside current ones.
    pub(crate) fn exact_revisions(&self) -> Option<Vec<(&str, &str)>> {
        match &self.0 {
            Operation::Search(_) => None,
            Operation::Read(read) => Some(
                read.references
                    .iter()
                    .filter_map(|reference| match reference {
                        Reference::Pinned(_) => None,
                        Reference::Exact(exact) => {
                            Some((exact.candidate_id.as_str(), exact.revision.as_str()))
                        }
                    })
                    .collect(),
            ),
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
            let hit = json!({"reference":Exact {candidate_id:source.candidate_id.clone(),revision:source.revision.clone(),cursor:None,source_id:None,location:None}, "record_type":source.record_type,"source_id":source.source_id,"dates":source.dates});
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
            let result = match reference {
                Reference::Pinned(id) => {
                    match sources
                        .iter()
                        .find(|s| s.workspace == workspace && s.candidate_id == *id)
                    {
                        Some(source) => {
                            source.validate()?;
                            read_record(source, None, &None, None, None, budget)
                        }
                        None => RecordReceipt::Unavailable {
                            reference: Some(id.clone()),
                            reason: "not_in_recorded_observation".into(),
                        },
                    }
                }
                Reference::Exact(exact) => {
                    let matching: Vec<&SourceRevision> = sources
                        .iter()
                        .filter(|s| {
                            s.workspace == workspace
                                && s.candidate_id == exact.candidate_id
                                && exact
                                    .source_id
                                    .as_ref()
                                    .is_none_or(|sid| &s.source_id == sid)
                        })
                        .collect();
                    if let Some(source) = matching
                        .iter()
                        .copied()
                        .find(|s| s.revision == exact.revision)
                    {
                        source.validate()?;
                        let superseded_by = matching
                            .first()
                            .copied()
                            .filter(|head| head.revision != source.revision)
                            .map(|head| head.revision.clone());
                        if let Some(location) = &exact.location {
                            read_selected_location(
                                source,
                                location,
                                &exact.source_id,
                                superseded_by,
                                budget,
                            )?
                        } else {
                            read_record(
                                source,
                                exact.cursor,
                                &exact.source_id,
                                superseded_by,
                                None,
                                budget,
                            )
                        }
                    } else if sources
                        .iter()
                        .any(|s| s.workspace == workspace && s.candidate_id == exact.candidate_id)
                    {
                        RecordReceipt::Unavailable {
                            reference: Some(exact.candidate_id.clone()),
                            reason: "revision_changed_repeat_discovery".into(),
                        }
                    } else {
                        RecordReceipt::Unavailable {
                            reference: Some(exact.candidate_id.clone()),
                            reason: "not_in_recorded_observation".into(),
                        }
                    }
                }
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

fn valid_location_identifiers(location: &super::read::RecordLocation) -> bool {
    use super::read::{PartSelector, RecordLocation, ScanSelector};
    match location {
        RecordLocation::Whole => true,
        RecordLocation::Part { part, slice: _ } => match part {
            PartSelector::NativeRecord | PartSelector::RetainedIndex(_) => true,
            PartSelector::Label(value) | PartSelector::AttachmentId(value) => {
                !value.trim().is_empty()
            }
        },
        RecordLocation::ScanPage {
            scan,
            page: _,
            slice: _,
        } => match scan {
            ScanSelector::Primary => true,
            ScanSelector::Named(scan_id) => !scan_id.trim().is_empty(),
        },
        RecordLocation::Continuation(cursor) => {
            !cursor.source_id.trim().is_empty()
                && !cursor.candidate_id.trim().is_empty()
                && !cursor.revision.trim().is_empty()
        }
    }
}

fn part_label(identity: &super::read::PartIdentity) -> String {
    use super::read::PartIdentity;
    match identity {
        PartIdentity::NativeRecord => "native_record".into(),
        PartIdentity::Retained { label, .. }
        | PartIdentity::Attachment { label, .. }
        | PartIdentity::ScanDocument { label, .. } => label.clone(),
        PartIdentity::ScanPage { label, page, .. } => format!("{label} (page {page})"),
    }
}

fn read_selected_location(
    source: &SourceRevision,
    location: &super::read::RecordLocation,
    exact_source_id: &Option<String>,
    superseded_by: Option<String>,
    budget: usize,
) -> Result<RecordReceipt, String> {
    use super::read::{
        ContinuationCursor, ContinuationScope, LocationResolution, PartState, ProvenancePartReader,
        RecordLocation, RecordPartReader as _, resolve_location,
    };
    let resolved_parts = ProvenancePartReader
        .resolve_parts(source)
        .map_err(|e| e.to_string())?;
    let window = resolve_location(source, &resolved_parts, location);
    let location_limit = match window.resolution {
        LocationResolution::Recovered { .. } => None,
        LocationResolution::Unrecoverable { stated_limit, .. } => Some(stated_limit),
    };
    let inline_only = resolved_parts.len() == 1 + source.readable_parts.len();
    if inline_only
        && window.start_ordinal == 0
        && window.start_byte == 0
        && matches!(window.scope, ContinuationScope::Document)
        && source.content.len() <= budget
    {
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
            superseded_by: superseded_by.clone(),
            location_limit: location_limit.clone(),
        };
        if full.encoded_len() <= budget {
            return Ok(full);
        }
    }

    let ordinal = window.start_ordinal;
    let end_ordinal = match window.scope {
        ContinuationScope::Document => resolved_parts.len(),
        ContinuationScope::WithinPart { .. } => (ordinal + 1).min(resolved_parts.len()),
    };
    let current = resolved_parts
        .get(ordinal)
        .expect("resolve_location guarantees start_ordinal is in bounds");
    let make_continuation = |part_ordinal: usize, byte_offset: usize| {
        Box::new(Exact {
            candidate_id: source.candidate_id.clone(),
            revision: source.revision.clone(),
            cursor: None,
            source_id: exact_source_id.clone(),
            location: Some(RecordLocation::Continuation(ContinuationCursor {
                source_id: source.source_id.clone(),
                candidate_id: source.candidate_id.clone(),
                revision: source.revision.clone(),
                part_ordinal,
                byte_offset,
                scope: window.scope,
            })),
        })
    };
    let next_part = (ordinal + 1 < end_ordinal).then(|| make_continuation(ordinal + 1, 0));

    match &current.state {
        PartState::Missing { detail } => {
            let receipt = RecordReceipt::MissingPart {
                reference: source.candidate_id.clone(),
                revision: source.revision.clone(),
                part: ordinal,
                part_identity: current.part.clone(),
                detail: detail.clone(),
                next: next_part,
                superseded_by,
                location_limit,
            };
            Ok(if receipt.encoded_len() <= budget {
                receipt
            } else {
                RecordReceipt::over_budget()
            })
        }
        PartState::Unreadable { reason } => {
            let receipt = RecordReceipt::UnreadablePart {
                reference: source.candidate_id.clone(),
                revision: source.revision.clone(),
                part: ordinal,
                part_identity: current.part.clone(),
                reason: reason.clone(),
                next: next_part,
                superseded_by,
                location_limit,
            };
            Ok(if receipt.encoded_len() <= budget {
                receipt
            } else {
                RecordReceipt::over_budget()
            })
        }
        PartState::Unavailable { reason } => {
            let receipt = RecordReceipt::UnavailablePart {
                reference: source.candidate_id.clone(),
                revision: source.revision.clone(),
                part: ordinal,
                part_identity: current.part.clone(),
                reason: reason.clone(),
                next: next_part,
                superseded_by,
                location_limit,
            };
            Ok(if receipt.encoded_len() <= budget {
                receipt
            } else {
                RecordReceipt::over_budget()
            })
        }
        PartState::Readable { text } => {
            let start_byte = window.start_byte;
            let target_end = match window.scope {
                ContinuationScope::Document => text.len(),
                ContinuationScope::WithinPart { end_byte } => end_byte.min(text.len()),
            };
            let label = part_label(&current.part);
            let passage = |end: usize| {
                let next = if end < target_end {
                    Some(make_continuation(ordinal, end))
                } else {
                    next_part.clone()
                };
                RecordReceipt::Passage {
                    reference: source.candidate_id.clone(),
                    revision: source.revision.clone(),
                    part: ordinal,
                    label: label.clone(),
                    part_identity: Some(current.part.clone()),
                    start_byte,
                    end_byte: end,
                    total_bytes: text.len(),
                    content: text[start_byte..end].to_owned(),
                    next,
                    superseded_by: superseded_by.clone(),
                    location_limit: location_limit.clone(),
                }
            };
            let remaining = &text[start_byte..target_end];
            if remaining.is_empty() {
                let empty_passage = passage(start_byte);
                return Ok(if empty_passage.encoded_len() <= budget {
                    empty_passage
                } else {
                    RecordReceipt::over_budget()
                });
            }
            let limit = remaining.floor_char_boundary(remaining.len().min(budget));
            let ends: Vec<usize> = remaining[..limit]
                .char_indices()
                .map(|(i, c)| start_byte + i + c.len_utf8())
                .collect();
            let candidate = passage(start_byte + limit);
            if candidate.encoded_len() <= budget {
                return Ok(candidate);
            }
            let count = ends.partition_point(|end| passage(*end).encoded_len() <= budget);
            Ok(match count.checked_sub(1) {
                Some(idx) => passage(ends[idx]),
                None => RecordReceipt::over_budget(),
            })
        }
    }
}

fn read_record(
    source: &SourceRevision,
    cursor: Option<Position>,
    exact_source_id: &Option<String>,
    superseded_by: Option<String>,
    explicit_limit: Option<String>,
    budget: usize,
) -> RecordReceipt {
    let mut effective_cursor = cursor;
    let mut location_limit = explicit_limit;

    if let Some(requested_pos) = effective_cursor {
        let valid = match part(source, requested_pos.part) {
            Some((_, text)) => {
                text.is_char_boundary(requested_pos.offset)
                    && !(requested_pos.offset == text.len() && !text.is_empty())
            }
            None => false,
        };
        if !valid {
            effective_cursor = None;
            location_limit = Some(format!(
                "Requested cursor part {} at byte offset {} could not be recovered on revision '{}'; returning the same source from the start.",
                requested_pos.part, requested_pos.offset, source.revision
            ));
        }
    }

    if effective_cursor.is_none() && source.content.len() <= budget {
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
            superseded_by: superseded_by.clone(),
            location_limit: location_limit.clone(),
        };
        if full.encoded_len() <= budget {
            return full;
        }
    }
    let position = effective_cursor.unwrap_or_default();
    let (label, text) = part(source, position.part).expect("validated or default part 0");
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
            part_identity: None,
            start_byte: position.offset,
            end_byte: end,
            total_bytes: text.len(),
            content: text[position.offset..end].into(),
            next: next.map(|cursor| {
                Box::new(Exact {
                    candidate_id: source.candidate_id.clone(),
                    revision: source.revision.clone(),
                    cursor: Some(cursor),
                    source_id: exact_source_id.clone(),
                    location: None,
                })
            }),
            superseded_by: superseded_by.clone(),
            location_limit: location_limit.clone(),
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
    let byte_span = json!({
        "type":"object",
        "properties":{
            "start_byte":{"type":"integer","minimum":0},
            "end_byte":{"type":"integer","minimum":0}
        },
        "required":["start_byte","end_byte"],
        "additionalProperties":false
    });
    let content_slice = json!({
        "anyOf":[
            {
                "type":"object",
                "properties":{"slice":{"type":"string","enum":["full"]}},
                "required":["slice"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "slice":{"type":"string","enum":["byte_span"]},
                    "span":byte_span
                },
                "required":["slice","span"],
                "additionalProperties":false
            }
        ]
    });
    let part_selector = json!({
        "anyOf":[
            {
                "type":"object",
                "properties":{"kind":{"type":"string","enum":["native_record"]}},
                "required":["kind"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "kind":{"type":"string","enum":["retained_index"]},
                    "value":{"type":"integer","minimum":0}
                },
                "required":["kind","value"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "kind":{"type":"string","enum":["label","attachment_id"]},
                    "value":{"type":"string","minLength":1}
                },
                "required":["kind","value"],
                "additionalProperties":false
            }
        ]
    });
    let scan_selector = json!({
        "anyOf":[
            {
                "type":"object",
                "properties":{"scope":{"type":"string","enum":["primary"]}},
                "required":["scope"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "scope":{"type":"string","enum":["named"]},
                    "scan_id":{"type":"string","minLength":1}
                },
                "required":["scope","scan_id"],
                "additionalProperties":false
            }
        ]
    });
    let continuation_scope = json!({
        "anyOf":[
            {
                "type":"object",
                "properties":{"scope":{"type":"string","enum":["document"]}},
                "required":["scope"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "scope":{"type":"string","enum":["within_part"]},
                    "end_byte":{"type":"integer","minimum":0}
                },
                "required":["scope","end_byte"],
                "additionalProperties":false
            }
        ]
    });
    let location = json!({
        "anyOf":[
            {"type":"null"},
            {
                "type":"object",
                "properties":{"kind":{"type":"string","enum":["whole"]}},
                "required":["kind"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "kind":{"type":"string","enum":["part"]},
                    "part":part_selector,
                    "slice":content_slice.clone()
                },
                "required":["kind","part"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "kind":{"type":"string","enum":["scan_page"]},
                    "scan":scan_selector,
                    "page":{"type":"integer","minimum":1},
                    "slice":content_slice
                },
                "required":["kind","page"],
                "additionalProperties":false
            },
            {
                "type":"object",
                "properties":{
                    "kind":{"type":"string","enum":["continuation"]},
                    "source_id":{"type":"string","minLength":1},
                    "candidate_id":{"type":"string","minLength":1},
                    "revision":{"type":"string","minLength":1},
                    "part_ordinal":{"type":"integer","minimum":0},
                    "byte_offset":{"type":"integer","minimum":0},
                    "scope":continuation_scope
                },
                "required":["kind","source_id","candidate_id","revision","part_ordinal","byte_offset","scope"],
                "additionalProperties":false
            }
        ]
    });
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
            }]},
            "source_id":{"type":["string","null"],"minLength":1},
            "location":location
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
        description: "Open exact references for this patient, optionally scoped by source_id and location (whole record, named or indexed part, 1-indexed scan page, UTF-8 byte span, or continuation). A whole inline record includes native content and decoded parts. For a large or multi-part record, each passage names its part and exact UTF-8 byte span, and non-readable parts report missing_part, unreadable_part, or unavailable_part without stopping remaining parts. Pass next back to continue; null ends the read. Base claims only on the passages delivered.".into(),
        parameters: json!({
            "type":"object","properties":{
                "references":{"type":"array","items":reference,"minItems":1,"maxItems":8},
                "max_result_bytes":{"type":"integer","minimum":4096,"maximum":65536}
            },
            "required":["references","max_result_bytes"],"additionalProperties":false
        }),
    }
}
