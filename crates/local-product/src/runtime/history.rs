//! Read-only history queries over saved conversations, reply drafts, and run
//! receipts.
//!
//! Recent conversations, workspace conversation lists, workspace past-work
//! lists, and reopened conversation readback read directly from the durable
//! runtime tables without mutating state or introducing parallel tables.

use std::collections::{BTreeMap, HashMap};

use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::entities::{
    conversation, reply_draft, reply_revision, source, source_pull, source_version,
};
use super::reply::DraftStatus;
use super::{
    Attempt, Conversation, MAX_LISTED_CHATS, Message, MessageRole, Scope, Store, WorkStatus,
    control, conversion_error, engine, keys, order_by_latest_activity,
};
use crate::physician_sources::{READ_TOOL, ReadablePart, SEARCH_TOOL, SourceRevision};

/// A failure while reading saved conversations or work history.
#[derive(Debug, thiserror::Error)]
pub enum HistoryError {
    /// One of the required scope identifiers was blank.
    #[error("empty physician scope")]
    EmptyScope,
    /// Postgres refused or failed the read.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// How one saved history entry ended or currently stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryOutcome {
    /// A finished response was published.
    Answered,
    /// Work is accepted or actively running.
    Running,
    /// Work is held until the physician resumes it.
    Held,
    /// Recovery gave up; Try again can continue it.
    Failed,
    /// A saved patient-reply draft is ready for review.
    DraftReady,
    /// A patient-reply revision failed; Try again can continue it.
    RevisionFailed,
}

impl HistoryOutcome {
    /// Short physician-facing outcome label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Answered => "Answered",
            Self::Running => "In progress",
            Self::Held => "Paused",
            Self::Failed => "Couldn't finish",
            Self::DraftReady => "Reply ready",
            Self::RevisionFailed => "Revision couldn't finish",
        }
    }
}

/// Which kind of saved workspace work an entry reopens.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HistoryWorkTarget {
    /// A physician conversation with Coda.
    Conversation {
        /// Durable chat identifier.
        chat_id: String,
    },
    /// A patient-message reply draft and its revision history.
    ReplyDraft {
        /// Durable reply work identifier.
        reply_id: String,
        /// Number of archived earlier revisions retained for this draft.
        archived_revisions: usize,
    },
}

/// One authorized conversation in a cross-patient or workspace list,
/// distinguished by its question, answer summary, and outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizedConversationSummary {
    /// Patient workspace identifier.
    pub workspace: String,
    /// Conversation identifier.
    pub chat: String,
    /// One-line title derived from the opening physician question.
    pub title: String,
    /// Full opening physician question.
    pub question: String,
    /// Content-distinguishing preview of the latest answer or follow-up.
    pub summary: String,
    /// Where the conversation stands.
    pub outcome: HistoryOutcome,
    /// Database-clock seconds when the first physician message was accepted.
    pub opened_at: i64,
    /// Database-clock seconds of the latest accepted input or published reply.
    pub last_activity_at: i64,
}

/// One saved item in a patient workspace's Past work list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePastWorkItem {
    /// Stable identifier within the Past work list.
    pub id: String,
    /// Reopen target.
    pub target: HistoryWorkTarget,
    /// Content-distinguishing title (question or revision instruction).
    pub title: String,
    /// Content-distinguishing summary of the answer or draft text.
    pub summary: String,
    /// Current outcome of the saved work.
    pub outcome: HistoryOutcome,
    /// Database-clock seconds when the item last changed.
    pub updated_at: i64,
}

/// Who authored a saved conversation turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnAuthor {
    /// The signed-in physician.
    Physician,
    /// Coda's assistant output (complete or retained partial).
    Assistant {
        /// Whether this turn is a retained incomplete answer rather than a
        /// finished publication.
        partial: bool,
    },
}

/// A span of prose or a verified inline citation inside a saved turn.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HistorySegment {
    /// Verbatim prose text.
    Text {
        /// Segment text.
        text: String,
    },
    /// Verified inline citation pointing to a source Coda read during that turn.
    Citation {
        /// Key into [`SavedConversationReadback::sources`].
        source_key: String,
        /// 1-based citation number within the response.
        number: usize,
        /// Short source label for preview tooltips.
        label: String,
        /// Exact excerpt read from the source.
        quote: String,
    },
}

/// One cited record Coda actually read during a conversation, reconstructed from
/// retained tool receipts and append-only source observations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitedSourceReadback {
    /// Lookup key referenced by [`HistorySegment::Citation`].
    pub source_key: String,
    /// 1-based citation number within its response.
    pub number: usize,
    /// Source adapter identifier.
    pub source_id: String,
    /// Physician-facing source name.
    pub source_label: String,
    /// Opaque candidate record identifier.
    pub candidate_id: String,
    /// Immutable source revision read by Coda.
    pub revision: String,
    /// Record title or type label.
    pub title: String,
    /// Source's own record type.
    pub record_type: String,
    /// Primary date recorded by the source, if supplied.
    pub recorded_at: Option<String>,
    /// Quoted passage read by the answering run.
    pub quote: String,
    /// Readable record text for the side panel.
    pub body: String,
    /// Exact source-native representation for Technical details.
    pub raw_content: String,
    /// Direct link into the originating EHR or source when available.
    pub external_url: Option<String>,
}

/// One turn in a reopened conversation, with structured segments, Basis, and
/// newer-record status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedTurnReadback {
    /// Durable message identifier (`ResponseId` for assistant replies).
    pub id: String,
    /// Turn author and completion state.
    pub author: TurnAuthor,
    /// Exact saved message text before Basis splitting.
    pub raw_text: String,
    /// Main body segments with verified citations resolved.
    pub segments: Vec<HistorySegment>,
    /// Collapsed Basis and limitations segments, present on finished answers
    /// that recorded a Basis section.
    pub basis: Option<Vec<HistorySegment>>,
    /// Whether newer source observations exist in the workspace after this
    /// answer's observed workspace version.
    pub newer_records: bool,
}

/// Complete durable readback of a reopened conversation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedConversationReadback {
    /// Current outcome of the saved conversation.
    pub outcome: HistoryOutcome,
    /// Current work identifier.
    pub work_id: String,
    /// Reconstructed turns in transcript order.
    pub turns: Vec<SavedTurnReadback>,
    /// Verified cited sources keyed by `source_key`.
    pub sources: BTreeMap<String, CitedSourceReadback>,
    /// Current workspace source version at readback time.
    pub workspace_source_version: u64,
}

/// Whether newer source observations exist in the workspace after a finished
/// answer's observed source version.
///
/// Kept as one isolated pure predicate so update-marker policy has a single
/// owner across live snapshots and post-restart readback.
pub const fn answer_has_newer_records(
    answer_sources_version: Option<u64>,
    current_workspace_version: u64,
) -> bool {
    match answer_sources_version {
        Some(observed) => current_workspace_version > observed,
        None => false,
    }
}

const MAX_TITLE_CHARS: usize = 96;
const MAX_SUMMARY_CHARS: usize = 160;

fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate_clean(input: &str, max_chars: usize) -> String {
    let collapsed = collapse_whitespace(input);
    if collapsed.chars().count() <= max_chars {
        return collapsed;
    }
    let mut clipped: String = collapsed
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect();
    while clipped.ends_with(char::is_whitespace) {
        clipped.pop();
    }
    clipped.push('…');
    clipped
}

/// Splits a finished assistant message into `(main_body, optional_basis)`.
///
/// Stopped or partial answers never expose a Basis section.
pub fn split_answer_and_basis(text: &str, partial: bool) -> (&str, Option<&str>) {
    if partial {
        return (text.trim(), None);
    }
    const MARKERS: [&str; 6] = [
        "\n\nBasis and limitations:",
        "\n\n**Basis and limitations:**",
        "\n\n**Basis and limitations**:",
        "\n\nBasis:",
        "\n\n**Basis:**",
        "\n\n**Basis**:",
    ];
    for marker in MARKERS {
        if let Some((body, rest)) = text.rsplit_once(marker) {
            let trimmed_body = body.trim();
            let trimmed_basis = rest.trim();
            if !trimmed_body.is_empty() && !trimmed_basis.is_empty() {
                return (trimmed_body, Some(trimmed_basis));
            }
        }
    }
    // Also handle single-newline trailing Basis lines.
    for prefix in [
        "\nBasis and limitations:",
        "\n**Basis and limitations:**",
        "\nBasis:",
    ] {
        if let Some((body, rest)) = text.rsplit_once(prefix) {
            let trimmed_body = body.trim();
            let trimmed_basis = rest.trim();
            if !trimmed_body.is_empty() && !trimmed_basis.is_empty() {
                return (trimmed_body, Some(trimmed_basis));
            }
        }
    }
    (text.trim(), None)
}

fn strip_citation_brackets(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'['
            && let Some(end_offset) = input[index + 1..].find(']')
        {
            let inner = &input[index + 1..index + 1 + end_offset];
            if !inner.is_empty() && inner.chars().all(|ch| ch.is_ascii_digit()) {
                index += end_offset + 2;
                continue;
            }
        }
        let ch = input[index..].chars().next().expect("valid utf-8");
        if ch != '*' && ch != '`' {
            out.push(ch);
        }
        index += ch.len_utf8();
    }
    let collapsed = collapse_whitespace(&out);
    collapsed
        .replace(" .", ".")
        .replace(" ,", ",")
        .replace(" ;", ";")
}

fn conversation_outcome(state: &Conversation) -> HistoryOutcome {
    match state.status {
        WorkStatus::Pending | WorkStatus::Active => HistoryOutcome::Running,
        WorkStatus::Held | WorkStatus::Cancelled => HistoryOutcome::Held,
        WorkStatus::Failed => HistoryOutcome::Failed,
        WorkStatus::Completed => HistoryOutcome::Answered,
    }
}

fn summarize_conversation(
    chat_id: &str,
    workspace: &str,
    state: &Conversation,
) -> Option<AuthorizedConversationSummary> {
    let first_user = state
        .messages
        .iter()
        .find(|message| matches!(message.role, MessageRole::User { .. }))?;
    let question = collapse_whitespace(&first_user.text);
    if question.is_empty() {
        return None;
    }
    let title = truncate_clean(&question, MAX_TITLE_CHARS);
    let outcome = conversation_outcome(state);
    let latest_assistant = state
        .messages
        .iter()
        .rev()
        .find_map(|message| match &message.role {
            MessageRole::Assistant { partial, .. } => {
                let (body, _) = split_answer_and_basis(&message.text, *partial);
                let cleaned = strip_citation_brackets(body);
                (!cleaned.is_empty()).then(|| truncate_clean(&cleaned, MAX_SUMMARY_CHARS))
            }
            MessageRole::User { .. } => None,
        });
    let summary = latest_assistant.unwrap_or_else(|| outcome.label().to_owned());
    let last_activity_at = state.latest_activity_at();
    Some(AuthorizedConversationSummary {
        workspace: workspace.to_owned(),
        chat: chat_id.to_owned(),
        title,
        question,
        summary,
        outcome,
        opened_at: state.opened_at,
        last_activity_at,
    })
}

fn draft_outcome(status: DraftStatus) -> HistoryOutcome {
    match status {
        DraftStatus::Idle | DraftStatus::Completed | DraftStatus::Cancelled => {
            HistoryOutcome::DraftReady
        }
        DraftStatus::Pending | DraftStatus::Active => HistoryOutcome::Running,
        DraftStatus::Held => HistoryOutcome::Held,
        DraftStatus::Failed => HistoryOutcome::RevisionFailed,
    }
}

#[derive(Debug, Deserialize)]
struct SearchOutput {
    #[serde(default)]
    workspace_version: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct ReadReceiptEnvelope {
    #[serde(default)]
    results: Vec<ReadReceiptEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
enum ReadReceiptEntry {
    Available {
        reference: String,
        revision: String,
        #[serde(default)]
        record_type: String,
        #[serde(default)]
        content: String,
        #[serde(default)]
        readable_parts: Vec<crate::physician_sources::ReadablePart>,
        #[serde(default)]
        dates: Vec<crate::physician_sources::RecordDate>,
        #[serde(default)]
        provenance: Value,
    },
    Passage {
        reference: String,
        revision: String,
        #[serde(default)]
        label: String,
        #[serde(default)]
        content: String,
    },
    Unavailable {
        #[serde(default)]
        #[expect(dead_code, reason = "deserialized to match tool receipt variants")]
        reason: String,
    },
    OverBudget {
        #[serde(default)]
        #[expect(dead_code, reason = "deserialized to match tool receipt variants")]
        reason: String,
    },
}

#[derive(Clone, Debug)]
struct ObservedRead {
    candidate_id: String,
    revision: String,
    quote: String,
    fallback_record_type: String,
    fallback_body: String,
    fallback_raw: String,
    fallback_date: Option<String>,
    fallback_provenance: Value,
}

#[derive(Default)]
struct WorkObservation {
    search_version: Option<u64>,
    reads: Vec<ObservedRead>,
}

fn collect_work_observations(attempts: &[Attempt]) -> HashMap<String, WorkObservation> {
    let mut by_work: HashMap<String, WorkObservation> = HashMap::new();
    for attempt in attempts {
        let mut tool_names: HashMap<String, String> = HashMap::new();
        let entry = by_work.entry(attempt.work.clone()).or_default();
        for event in &attempt.events {
            match &event.kind {
                engine::EventKind::ToolStarted(call) => {
                    tool_names.insert(call.id.clone(), call.name.clone());
                }
                engine::EventKind::ToolReturned {
                    call_id,
                    result: engine::results::ToolResult::Original(output),
                } => {
                    let Some(tool_name) = tool_names.get(call_id).map(String::as_str) else {
                        continue;
                    };
                    if tool_name == SEARCH_TOOL
                        && let Ok(parsed) = serde_json::from_str::<SearchOutput>(output)
                        && let Some(version) = parsed.workspace_version
                    {
                        entry.search_version = Some(
                            entry
                                .search_version
                                .map_or(version, |prev| prev.max(version)),
                        );
                    } else if tool_name == READ_TOOL
                        && let Ok(envelope) = serde_json::from_str::<ReadReceiptEnvelope>(output)
                    {
                        for receipt in envelope.results {
                            match receipt {
                                ReadReceiptEntry::Available {
                                    reference,
                                    revision,
                                    record_type,
                                    content,
                                    readable_parts,
                                    dates,
                                    provenance,
                                } => {
                                    let title =
                                        cited_record_title(&provenance, &record_type, &reference);
                                    let body =
                                        format_cited_source_body(&title, &readable_parts, &content);
                                    let quote = truncate_clean(&body, 220);
                                    if !entry.reads.iter().any(|existing| {
                                        existing.candidate_id == reference
                                            && existing.revision == revision
                                    }) {
                                        entry.reads.push(ObservedRead {
                                            candidate_id: reference,
                                            revision,
                                            quote,
                                            fallback_record_type: record_type,
                                            fallback_body: body,
                                            fallback_raw: content,
                                            fallback_date: dates.first().map(|d| d.value.clone()),
                                            fallback_provenance: provenance,
                                        });
                                    }
                                }
                                ReadReceiptEntry::Passage {
                                    reference,
                                    revision,
                                    label,
                                    content,
                                } => {
                                    let title = if label.trim().is_empty() {
                                        reference.as_str()
                                    } else {
                                        label.as_str()
                                    };
                                    let readable_body =
                                        format_cited_source_body(title, &[], &content);
                                    let quote = truncate_clean(&readable_body, 220);
                                    if let Some(existing) = entry.reads.iter_mut().find(|item| {
                                        item.candidate_id == reference && item.revision == revision
                                    }) {
                                        if existing.quote.is_empty() && !quote.is_empty() {
                                            existing.quote = quote;
                                        }
                                    } else {
                                        entry.reads.push(ObservedRead {
                                            candidate_id: reference,
                                            revision,
                                            quote,
                                            fallback_record_type: label,
                                            fallback_body: readable_body,
                                            fallback_raw: content,
                                            fallback_date: None,
                                            fallback_provenance: Value::Null,
                                        });
                                    }
                                }
                                ReadReceiptEntry::Unavailable { .. }
                                | ReadReceiptEntry::OverBudget { .. } => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    by_work
}

fn provenance_string(provenance: &Value, keys: &[&str]) -> Option<String> {
    let object = provenance.as_object()?;
    for key in keys {
        if let Some(value) = object.get(*key).and_then(Value::as_str) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_owned());
            }
        }
    }
    None
}

/// Selects the physician-facing display title for a cited chart record from
/// its provenance (`title`, `display`, `label`), falling back to `record_type`
/// and then `candidate_id`.
pub fn cited_record_title(provenance: &Value, record_type: &str, candidate_id: &str) -> String {
    provenance_string(provenance, &["title", "display", "label"]).unwrap_or_else(|| {
        let trimmed = record_type.trim();
        if trimmed.is_empty() {
            candidate_id.to_owned()
        } else {
            trimmed.to_owned()
        }
    })
}

/// Formats a cited source as human-readable clinical prose for physician
/// inspection, omitting a redundant `part.label` when it matches `title`
/// case-insensitively and rendering JSON `raw_content` as `"Key: value"` lines
/// when `readable_parts` is empty (`raw_content` stays only in `raw_content`).
pub fn format_cited_source_body(
    title: &str,
    readable_parts: &[ReadablePart],
    raw_content: &str,
) -> String {
    let normalized_title = collapse_whitespace(title).to_lowercase();
    let sections: Vec<String> = readable_parts
        .iter()
        .filter_map(|part| {
            let label = part.label.trim();
            let text = part.text.trim();
            let same_as_title =
                !label.is_empty() && collapse_whitespace(label).to_lowercase() == normalized_title;
            if label.is_empty() || same_as_title {
                (!text.is_empty()).then(|| text.to_owned())
            } else if text.is_empty() {
                Some(label.to_owned())
            } else {
                Some(format!("{label}\n{text}"))
            }
        })
        .collect();
    if !sections.is_empty() {
        return sections.join("\n\n");
    }
    format_readable_fallback(raw_content)
}

fn humanize_json_key(key: &str) -> String {
    let replaced = key.replace(['_', '-'], " ");
    let collapsed = collapse_whitespace(&replaced);
    let mut chars = collapsed.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn render_json_lines(prefix: Option<&str>, value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Null => {}
        Value::Bool(flag) => {
            let word = if *flag { "Yes" } else { "No" };
            match prefix {
                Some(key) if !key.is_empty() => out.push(format!("{key}: {word}")),
                _ => out.push(word.to_owned()),
            }
        }
        Value::Number(num) => match prefix {
            Some(key) if !key.is_empty() => out.push(format!("{key}: {num}")),
            _ => out.push(num.to_string()),
        },
        Value::String(text) => {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                match prefix {
                    Some(key) if !key.is_empty() => out.push(format!("{key}: {trimmed}")),
                    _ => out.push(trimmed.to_owned()),
                }
            }
        }
        Value::Array(items) => {
            let all_scalar = items
                .iter()
                .all(|item| matches!(item, Value::String(_) | Value::Number(_) | Value::Bool(_)));
            if all_scalar {
                let joined: Vec<String> = items
                    .iter()
                    .filter_map(|item| match item {
                        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_owned()),
                        Value::Number(n) => Some(n.to_string()),
                        Value::Bool(b) => Some(if *b { "Yes" } else { "No" }.to_owned()),
                        _ => None,
                    })
                    .collect();
                if !joined.is_empty() {
                    let list = joined.join(", ");
                    match prefix {
                        Some(key) if !key.is_empty() => out.push(format!("{key}: {list}")),
                        _ => out.push(list),
                    }
                }
            } else {
                for item in items {
                    render_json_lines(prefix, item, out);
                }
            }
        }
        Value::Object(map) => {
            for (key, nested) in map {
                let label = humanize_json_key(key);
                let full_key = match prefix {
                    Some(parent) if !parent.is_empty() && !label.is_empty() => {
                        format!("{parent} · {label}")
                    }
                    Some(parent) if !parent.is_empty() => parent.to_owned(),
                    _ => label,
                };
                render_json_lines(Some(&full_key), nested, out);
            }
        }
    }
}

fn format_readable_fallback(raw_content: &str) -> String {
    let trimmed = raw_content.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
        let mut lines = Vec::new();
        render_json_lines(None, &parsed, &mut lines);
        if !lines.is_empty() {
            return lines.join("\n");
        }
    }
    trimmed.to_owned()
}

/// Parses inline `[1]` or `[candidate_id]` markers against the verified sources
/// read in that turn's work. Unverified brackets stay plain text.
pub fn parse_segments_with_citations(
    text: &str,
    work_sources: &[CitedSourceReadback],
) -> Vec<HistorySegment> {
    if text.is_empty() {
        return Vec::new();
    }
    if work_sources.is_empty() {
        return vec![HistorySegment::Text {
            text: text.to_owned(),
        }];
    }
    let mut segments: Vec<HistorySegment> = Vec::new();
    let mut plain = String::new();
    let mut index = 0;
    let bytes = text.as_bytes();
    while index < bytes.len() {
        if bytes[index] == b'['
            && let Some(end_offset) = text[index + 1..].find(']')
        {
            let token = text[index + 1..index + 1 + end_offset].trim();
            let matched = token
                .parse::<usize>()
                .ok()
                .and_then(|number| number.checked_sub(1).and_then(|pos| work_sources.get(pos)))
                .or_else(|| {
                    work_sources
                        .iter()
                        .find(|source| source.candidate_id == token)
                });
            if let Some(cited) = matched {
                if !plain.is_empty() {
                    segments.push(HistorySegment::Text {
                        text: std::mem::take(&mut plain),
                    });
                }
                segments.push(HistorySegment::Citation {
                    source_key: cited.source_key.clone(),
                    number: cited.number,
                    label: format!("{} · {}", cited.source_label, cited.title),
                    quote: cited.quote.clone(),
                });
                index += end_offset + 2;
                continue;
            }
        }
        let ch = text[index..].chars().next().expect("valid utf-8");
        plain.push(ch);
        index += ch.len_utf8();
    }
    if !plain.is_empty() {
        segments.push(HistorySegment::Text { text: plain });
    }
    segments
}

impl Store {
    /// A physician's newest conversations across all patients in `practice`,
    /// ordered from the indexed `last_activity_at` column descending, then chat ID.
    pub async fn physician_conversations(
        &self,
        practice: &str,
        physician: &str,
        limit: usize,
    ) -> Result<Vec<AuthorizedConversationSummary>, HistoryError> {
        use conversation::Column as C;
        if practice.trim().is_empty() || physician.trim().is_empty() {
            return Err(HistoryError::EmptyScope);
        }
        if limit == 0 {
            return Ok(Vec::new());
        }
        let bounded = Ord::min(limit as u64, MAX_LISTED_CHATS);
        let rows = order_by_latest_activity(
            conversation::Entity::find()
                .filter(C::Practice.eq(practice))
                .filter(C::Physician.eq(physician)),
        )
        .limit(bounded)
        .all(&self.db)
        .await?;
        self.summarize_conversation_rows_with_withholding(rows)
            .await
    }

    /// Recent conversations across the physician's currently authorized patient
    /// workspaces, ordered by latest activity descending, then chat ID.
    pub async fn recent_conversations(
        &self,
        practice: &str,
        physician: &str,
        authorized_workspaces: &[String],
        limit: usize,
    ) -> Result<Vec<AuthorizedConversationSummary>, HistoryError> {
        use conversation::Column as C;
        if practice.trim().is_empty() || physician.trim().is_empty() {
            return Err(HistoryError::EmptyScope);
        }
        let workspaces: Vec<&str> = authorized_workspaces
            .iter()
            .map(String::as_str)
            .filter(|ws| !ws.trim().is_empty())
            .collect();
        if workspaces.is_empty() || limit == 0 {
            return Ok(Vec::new());
        }
        let bounded = Ord::min(limit as u64, MAX_LISTED_CHATS);
        let scoped = conversation::Entity::find()
            .filter(C::Practice.eq(practice))
            .filter(C::Physician.eq(physician));
        let filtered = if let [single] = workspaces.as_slice() {
            scoped.filter(C::Workspace.eq(*single))
        } else {
            scoped.filter(C::Workspace.is_in(workspaces))
        };
        let rows = order_by_latest_activity(filtered)
            .limit(bounded)
            .all(&self.db)
            .await?;
        self.summarize_conversation_rows_with_withholding(rows)
            .await
    }

    async fn summarize_conversation_rows_with_withholding(
        &self,
        rows: Vec<conversation::Model>,
    ) -> Result<Vec<AuthorizedConversationSummary>, HistoryError> {
        let mut withheld_by_ws: HashMap<String, super::link_correction::WithheldWorkspaceState> =
            HashMap::new();
        for row in &rows {
            if !withheld_by_ws.contains_key(&row.workspace) {
                let state = super::link_correction::load_workspace_withheld_state_in_tx(
                    &self.db,
                    &row.workspace,
                )
                .await?;
                withheld_by_ws.insert(row.workspace.clone(), state);
            }
        }
        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let chat = row.chat.clone();
            let workspace = row.workspace.clone();
            let state = control::conversation(row, Vec::new())?;
            let withheld_reason = withheld_by_ws.get(&workspace).and_then(|w| {
                w.reason_for_work(&chat)
                    .or_else(|| w.reason_for_work(&state.work))
            });
            if let Some(mut summary) = summarize_conversation(&chat, &workspace, &state) {
                if let Some(reason) = withheld_reason {
                    summary.summary = reason;
                }
                items.push(summary);
            }
        }
        items.sort_by(|a, b| {
            b.last_activity_at
                .cmp(&a.last_activity_at)
                .then_with(|| b.chat.cmp(&a.chat))
        });
        Ok(items)
    }

    /// Saved conversations and reply work in one authorized patient workspace,
    /// distinguished by content and outcome.
    pub async fn workspace_past_work(
        &self,
        practice: &str,
        physician: &str,
        workspace: &str,
    ) -> Result<Vec<WorkspacePastWorkItem>, HistoryError> {
        if [practice, physician, workspace]
            .iter()
            .any(|part| part.trim().is_empty())
        {
            return Err(HistoryError::EmptyScope);
        }
        let tx = self.snapshot().await?;

        let conv_rows = order_by_latest_activity(
            conversation::Entity::find()
                .filter(conversation::Column::Practice.eq(practice))
                .filter(conversation::Column::Physician.eq(physician))
                .filter(conversation::Column::Workspace.eq(workspace)),
        )
        .limit(MAX_LISTED_CHATS)
        .all(&tx)
        .await?;

        let draft_rows = reply_draft::Entity::find()
            .filter(reply_draft::Column::Practice.eq(practice))
            .filter(reply_draft::Column::Physician.eq(physician))
            .filter(reply_draft::Column::Workspace.eq(workspace))
            .order_by_desc(reply_draft::Column::Reply)
            .all(&tx)
            .await?;

        let archived_rows = reply_revision::Entity::find()
            .filter(reply_revision::Column::Practice.eq(practice))
            .filter(reply_revision::Column::Physician.eq(physician))
            .filter(reply_revision::Column::Workspace.eq(workspace))
            .order_by_desc(reply_revision::Column::Ordinal)
            .all(&tx)
            .await?;

        let withheld =
            super::link_correction::load_workspace_withheld_state_in_tx(&tx, workspace).await?;
        let boundary_fenced_works =
            super::follow_ups::load_workspace_fenced_work_ids(&tx, practice, workspace).await?;

        tx.commit().await?;

        let mut archived_by_reply: HashMap<String, (usize, i64)> = HashMap::new();
        for archived in archived_rows {
            let entry = archived_by_reply.entry(archived.reply).or_insert((0, 0));
            entry.0 += 1;
            entry.1 = entry.1.max(archived.archived_at);
        }

        let mut items = Vec::new();
        for row in conv_rows {
            let chat = row.chat.clone();
            let ws = row.workspace.clone();
            let state = control::conversation(row, Vec::new())?;
            let withheld_reason = withheld
                .reason_for_work(&chat)
                .or_else(|| withheld.reason_for_work(&state.work))
                .or_else(|| {
                    if boundary_fenced_works.contains(&chat)
                        || boundary_fenced_works.contains(&state.work)
                    {
                        Some(
                            "Unavailable: patient message content was withdrawn after a wrong-patient link correction."
                                .to_owned(),
                        )
                    } else {
                        None
                    }
                });
            if let Some(conv) = summarize_conversation(&chat, &ws, &state) {
                items.push(WorkspacePastWorkItem {
                    id: format!("conversation:{chat}"),
                    target: HistoryWorkTarget::Conversation { chat_id: chat },
                    title: conv.title,
                    summary: withheld_reason.unwrap_or(conv.summary),
                    outcome: conv.outcome,
                    updated_at: conv.last_activity_at,
                });
            }
        }

        for row in draft_rows {
            let reply_id = row.reply.clone();
            let draft = control::draft(row, Vec::new())?;
            let (archived_revisions, latest_archived_at) =
                archived_by_reply.remove(&reply_id).unwrap_or((0, 0));
            let (title, summary) = if let Some(reason) = withheld.reason_for_draft_key(&reply_id) {
                ("Unavailable patient reply draft".to_owned(), reason)
            } else {
                let title = draft
                    .accepted
                    .as_ref()
                    .map(|accepted| truncate_clean(&accepted.input.instruction, MAX_TITLE_CHARS))
                    .filter(|instruction| !instruction.is_empty())
                    .unwrap_or_else(|| "Patient reply draft".to_owned());
                let summary = if draft.text.trim().is_empty() {
                    draft_outcome(draft.status).label().to_owned()
                } else {
                    truncate_clean(&draft.text, MAX_SUMMARY_CHARS)
                };
                (title, summary)
            };
            let updated_at = latest_archived_at.max(draft.dispatch_after);
            items.push(WorkspacePastWorkItem {
                id: format!("reply:{reply_id}"),
                target: HistoryWorkTarget::ReplyDraft {
                    reply_id,
                    archived_revisions,
                },
                title,
                summary,
                outcome: draft_outcome(draft.status),
                updated_at,
            });
        }

        items.sort_by(|a, b| {
            b.updated_at
                .cmp(&a.updated_at)
                .then_with(|| b.id.cmp(&a.id))
        });
        Ok(items)
    }

    /// Reconstructs a saved conversation, its Basis sections, its verified
    /// source citations, and each finished answer's newer-records marker from
    /// durable Postgres rows.
    pub async fn reopen_conversation(
        &self,
        scope: &Scope,
    ) -> Result<Option<SavedConversationReadback>, HistoryError> {
        if [
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|part| part.trim().is_empty())
        {
            return Err(HistoryError::EmptyScope);
        }
        let tx = self.snapshot().await?;
        let Some(row) = conversation::Entity::find_by_id(keys(scope))
            .one(&tx)
            .await?
        else {
            tx.commit().await?;
            return Ok(None);
        };
        let mut state = Self::conversation_control(&tx, scope, row).await?;
        Self::conversation_events(&tx, scope, &mut state).await?;

        let workspace_source_version = source_version::Entity::find_by_id(scope.workspace.clone())
            .one(&tx)
            .await?
            .map(|model| u64::try_from(model.version).unwrap_or(0))
            .unwrap_or(0);

        let pull_rows = source_pull::Entity::find()
            .filter(source_pull::Column::Workspace.eq(scope.workspace.as_str()))
            .all(&tx)
            .await?;

        let work_observations = collect_work_observations(&state.attempts);
        let candidate_ids: Vec<&str> = work_observations
            .values()
            .flat_map(|obs| obs.reads.iter().map(|read| read.candidate_id.as_str()))
            .collect();

        let source_rows = if candidate_ids.is_empty() {
            Vec::new()
        } else {
            source::Entity::find()
                .filter(source::Column::Workspace.eq(scope.workspace.as_str()))
                .filter(source::Column::Candidate.is_in(candidate_ids))
                .order_by_asc(source::Column::ObservedAtVersion)
                .all(&tx)
                .await?
        };
        let withheld =
            super::link_correction::load_workspace_withheld_state_in_tx(&tx, &scope.workspace)
                .await?;
        let boundary = super::follow_ups::load_workspace_withdrawal_boundary(
            &tx,
            &scope.practice,
            &scope.workspace,
        )
        .await?;
        let boundary_works = super::follow_ups::load_workspace_fenced_work_ids(
            &tx,
            &scope.practice,
            &scope.workspace,
        )
        .await?;
        tx.commit().await?;

        let boundary_candidates: std::collections::BTreeSet<String> = boundary
            .as_ref()
            .map(|b| b.candidate_ids.iter().cloned().collect())
            .unwrap_or_default();

        let mut source_labels: HashMap<String, String> = HashMap::new();
        let mut latest_pull_at: Option<i64> = None;
        for pull in pull_rows {
            source_labels.insert(pull.source_id, pull.source_label);
            latest_pull_at =
                Some(latest_pull_at.map_or(pull.pulled_at, |prev| prev.max(pull.pulled_at)));
        }

        let mut saved_revisions: HashMap<(String, String), (u64, SourceRevision)> = HashMap::new();
        for row in source_rows {
            let observed_version = u64::try_from(row.observed_at_version).unwrap_or(0);
            if let Some(json) = row.revision {
                let decoded: SourceRevision = serde_json::from_value(json)
                    .map_err(conversion_error("saved source revision", "SourceRevision"))?;
                saved_revisions.insert(
                    (decoded.candidate_id.clone(), decoded.revision.clone()),
                    (observed_version, decoded),
                );
            }
        }

        let mut all_sources: BTreeMap<String, CitedSourceReadback> = BTreeMap::new();
        let mut sources_by_work: HashMap<String, Vec<CitedSourceReadback>> = HashMap::new();
        let mut version_by_work: HashMap<String, u64> = HashMap::new();
        let mut tainted_work_reasons: HashMap<String, String> = HashMap::new();

        for (work_id, observation) in &work_observations {
            let mut max_version = observation.search_version;
            let mut list = Vec::with_capacity(observation.reads.len());
            for (idx, read) in observation.reads.iter().enumerate() {
                let number = idx + 1;
                let source_key = format!("{work_id}:{}:{}", read.candidate_id, read.revision);
                let withheld_reason = withheld
                    .reason_for_candidate_or_work(Some(&read.candidate_id), Some(work_id))
                    .or_else(|| {
                        if boundary_candidates.contains(&read.candidate_id)
                            || boundary_works.contains(work_id)
                        {
                            Some(
                                "Unavailable: patient message content was withdrawn after a wrong-patient link correction."
                                    .to_owned(),
                            )
                        } else {
                            None
                        }
                    });
                let cited = if let Some(reason) = withheld_reason {
                    tainted_work_reasons
                        .entry(work_id.clone())
                        .or_insert_with(|| reason.clone());
                    CitedSourceReadback {
                        source_key: source_key.clone(),
                        number,
                        source_id: "unavailable".to_owned(),
                        source_label: "Unavailable".to_owned(),
                        candidate_id: read.candidate_id.clone(),
                        revision: read.revision.clone(),
                        title: "Unavailable · Withheld patient message".to_owned(),
                        record_type: String::new(),
                        recorded_at: None,
                        quote: reason.clone(),
                        body: reason,
                        raw_content: String::new(),
                        external_url: None,
                    }
                } else if let Some((observed_at, rev)) =
                    saved_revisions.get(&(read.candidate_id.clone(), read.revision.clone()))
                {
                    max_version =
                        Some(max_version.map_or(*observed_at, |prev| prev.max(*observed_at)));
                    let source_label = source_labels
                        .get(&rev.source_id)
                        .cloned()
                        .or_else(|| provenance_string(&rev.provenance, &["source_label", "system"]))
                        .unwrap_or_else(|| rev.source_id.clone());
                    let title =
                        cited_record_title(&rev.provenance, &rev.record_type, &rev.candidate_id);
                    let external_url =
                        provenance_string(&rev.provenance, &["external_url", "url", "href"]);
                    let body = format_cited_source_body(&title, &rev.readable_parts, &rev.content);
                    let quote = if !body.is_empty() {
                        truncate_clean(&body, 220)
                    } else {
                        read.quote.clone()
                    };
                    CitedSourceReadback {
                        source_key: source_key.clone(),
                        number,
                        source_id: rev.source_id.clone(),
                        source_label,
                        candidate_id: rev.candidate_id.clone(),
                        revision: rev.revision.clone(),
                        title,
                        record_type: rev.record_type.clone(),
                        recorded_at: rev.dates.first().map(|d| d.value.clone()),
                        quote,
                        body,
                        raw_content: rev.content.clone(),
                        external_url,
                    }
                } else {
                    let source_label =
                        provenance_string(&read.fallback_provenance, &["source_label", "system"])
                            .unwrap_or_else(|| "Patient chart".to_owned());
                    let title = cited_record_title(
                        &read.fallback_provenance,
                        &read.fallback_record_type,
                        &read.candidate_id,
                    );
                    let external_url = provenance_string(
                        &read.fallback_provenance,
                        &["external_url", "url", "href"],
                    );
                    CitedSourceReadback {
                        source_key: source_key.clone(),
                        number,
                        source_id: "chart".to_owned(),
                        source_label,
                        candidate_id: read.candidate_id.clone(),
                        revision: read.revision.clone(),
                        title,
                        record_type: read.fallback_record_type.clone(),
                        recorded_at: read.fallback_date.clone(),
                        quote: read.quote.clone(),
                        body: read.fallback_body.clone(),
                        raw_content: read.fallback_raw.clone(),
                        external_url,
                    }
                };
                all_sources.insert(source_key, cited.clone());
                list.push(cited);
            }
            if let Some(v) = max_version {
                version_by_work.insert(work_id.clone(), v);
            }
            sources_by_work.insert(work_id.clone(), list);
        }

        let mut turns = Vec::with_capacity(state.messages.len());
        let mut active_work: Option<String> = None;
        let mut cumulative_observed_version: Option<u64> = None;
        let activity_clock = state.latest_activity_at();
        let chat_withheld_reason = withheld.reason_for_work(&scope.chat);

        for Message { role, text } in &state.messages {
            match role {
                MessageRole::User { id, work, .. } => {
                    active_work = Some(work.clone());
                    if let Some(&work_version) = version_by_work.get(work) {
                        cumulative_observed_version = Some(
                            cumulative_observed_version
                                .map_or(work_version, |prev| prev.max(work_version)),
                        );
                    }
                    turns.push(SavedTurnReadback {
                        id: id.clone(),
                        author: TurnAuthor::Physician,
                        raw_text: text.clone(),
                        segments: vec![HistorySegment::Text { text: text.clone() }],
                        basis: None,
                        newer_records: false,
                    });
                }
                MessageRole::Assistant { id, partial } => {
                    let work_sources = active_work
                        .as_deref()
                        .and_then(|work| sources_by_work.get(work))
                        .map(Vec::as_slice)
                        .unwrap_or(&[]);
                    let turn_withheld_reason = active_work
                        .as_deref()
                        .and_then(|work| {
                            tainted_work_reasons
                                .get(work)
                                .cloned()
                                .or_else(|| withheld.reason_for_work(work))
                        })
                        .or_else(|| chat_withheld_reason.clone());
                    if let Some(reason) = turn_withheld_reason {
                        let unavailable_source = work_sources
                            .iter()
                            .find(|src| src.source_id == "unavailable");
                        let segments = if let Some(c0) = unavailable_source {
                            vec![
                                HistorySegment::Text {
                                    text: format!("{reason} "),
                                },
                                HistorySegment::Citation {
                                    source_key: c0.source_key.clone(),
                                    number: c0.number,
                                    label: c0.title.clone(),
                                    quote: reason.clone(),
                                },
                            ]
                        } else {
                            vec![HistorySegment::Text {
                                text: reason.clone(),
                            }]
                        };
                        turns.push(SavedTurnReadback {
                            id: id.clone(),
                            author: TurnAuthor::Assistant { partial: *partial },
                            raw_text: reason,
                            segments,
                            basis: None,
                            newer_records: false,
                        });
                        continue;
                    }
                    let (body_text, basis_text) = split_answer_and_basis(text, *partial);
                    let segments = parse_segments_with_citations(body_text, work_sources);
                    let basis = basis_text
                        .map(|section| parse_segments_with_citations(section, work_sources));
                    let inferred_version = cumulative_observed_version.or_else(|| {
                        if workspace_source_version > 0
                            && activity_clock > 0
                            && latest_pull_at.is_some_and(|pulled| pulled > activity_clock)
                        {
                            Some(0)
                        } else {
                            None
                        }
                    });
                    let newer_records = !partial
                        && answer_has_newer_records(inferred_version, workspace_source_version);
                    turns.push(SavedTurnReadback {
                        id: id.clone(),
                        author: TurnAuthor::Assistant { partial: *partial },
                        raw_text: text.clone(),
                        segments,
                        basis,
                        newer_records,
                    });
                }
            }
        }

        let outcome = conversation_outcome(&state);

        Ok(Some(SavedConversationReadback {
            outcome,
            work_id: state.work,
            turns,
            sources: all_sources,
            workspace_source_version,
        }))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn isolated_newer_records_helper_marks_only_newer_workspace_versions() {
        assert!(!answer_has_newer_records(None, 0));
        assert!(!answer_has_newer_records(None, 3));
        assert!(!answer_has_newer_records(Some(2), 2));
        assert!(!answer_has_newer_records(Some(3), 2));
        assert!(answer_has_newer_records(Some(2), 3));
    }

    #[test]
    fn basis_splits_cleanly_for_finished_answers_and_skips_partials() {
        let raw = "Potassium is 5.4 mmol/L [1].\n\nBasis and limitations: Drawn 2026-11-10 [1]; no hemolysis comment supplied.";
        let (body, basis) = split_answer_and_basis(raw, false);
        assert_eq!(body, "Potassium is 5.4 mmol/L [1].");
        assert_eq!(
            basis,
            Some("Drawn 2026-11-10 [1]; no hemolysis comment supplied.")
        );

        let (partial_body, partial_basis) = split_answer_and_basis(raw, true);
        assert_eq!(partial_body, raw);
        assert_eq!(partial_basis, None);
    }

    #[test]
    fn citation_parser_resolves_verified_receipts_and_leaves_unverified_brackets_plain() {
        let sources = vec![CitedSourceReadback {
            source_key: "work-1:lab-k:rev-1".to_owned(),
            number: 1,
            source_id: "hint".to_owned(),
            source_label: "Hint Health".to_owned(),
            candidate_id: "lab-k".to_owned(),
            revision: "rev-1".to_owned(),
            title: "Metabolic panel".to_owned(),
            record_type: "Results".to_owned(),
            recorded_at: Some("2026-11-10".to_owned()),
            quote: "Potassium 5.4 mmol/L".to_owned(),
            body: "Potassium 5.4 mmol/L (ref 3.5-5.1)".to_owned(),
            raw_content: "{\"k\":5.4}".to_owned(),
            external_url: None,
        }];
        let segments =
            parse_segments_with_citations("Potassium is 5.4 [1], prior range [9] noted.", &sources);
        assert_eq!(
            segments,
            vec![
                HistorySegment::Text {
                    text: "Potassium is 5.4 ".to_owned()
                },
                HistorySegment::Citation {
                    source_key: "work-1:lab-k:rev-1".to_owned(),
                    number: 1,
                    label: "Hint Health · Metabolic panel".to_owned(),
                    quote: "Potassium 5.4 mmol/L".to_owned(),
                },
                HistorySegment::Text {
                    text: ", prior range [9] noted.".to_owned()
                },
            ]
        );
    }

    #[test]
    fn cited_source_body_omits_duplicate_title_label_and_renders_json_fallback_as_readable_lines() {
        let duplicate_label = vec![ReadablePart {
            label: "basic metabolic panel".to_owned(),
            text: "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L).".to_owned(),
        }];
        assert_eq!(
            format_cited_source_body(
                "Basic metabolic panel",
                &duplicate_label,
                r#"{"potassium":5.4}"#
            ),
            "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L)."
        );

        let distinct_sections = vec![
            ReadablePart {
                label: "Basic metabolic panel".to_owned(),
                text: "Potassium 5.4 mmol/L.".to_owned(),
            },
            ReadablePart {
                label: "Specimen comment".to_owned(),
                text: "No hemolysis detected.".to_owned(),
            },
        ];
        assert_eq!(
            format_cited_source_body(
                "Basic metabolic panel",
                &distinct_sections,
                r#"{"potassium":5.4}"#
            ),
            "Potassium 5.4 mmol/L.\n\nSpecimen comment\nNo hemolysis detected."
        );

        let raw_json = r#"{"test":"Basic metabolic panel","potassium_mmol_l":5.4,"urgent":true,"notes":null,"flags":["High","Repeat"]}"#;
        let rendered = format_cited_source_body("Basic metabolic panel", &[], raw_json);
        assert_eq!(
            rendered,
            "Flags: High, Repeat\nPotassium mmol l: 5.4\nTest: Basic metabolic panel\nUrgent: Yes"
        );
        assert!(!rendered.as_str().contains('{') && !rendered.as_str().contains('}'));
    }
}
