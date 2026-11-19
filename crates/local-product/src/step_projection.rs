//! Shared run-step projection and staff-only reasoning-summary inspection.
//!
//! All live and replayed Coda workflows classify tool calls through one rule
//! ([`classify_tool`]) and project physician-facing run steps through
//! [`project_steps_from_events`] without exposing internal tool vocabulary.
//! Staff-only reasoning summaries ([`extract_reasoning_summaries`]) are kept
//! strictly separate for team inspection tools and never enter physician UI
//! projections.

use std::collections::{BTreeSet, HashMap};

use harness::engine::results::ToolResult;
use harness::engine::{Event, EventKind, Item};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::attachments::READ_DOCUMENT_PAGE_TOOL;
use crate::clinical_library::{READ_LIBRARY_TOOL, SEARCH_LIBRARY_TOOL};
use crate::initial_understanding::{READ_UNDERSTANDING_TOOL, SAVE_UNDERSTANDING_TOOL};
use crate::message_review_recipe::COMPLETE_MESSAGE_REVIEW_TOOL;
use crate::patient_reviews::{
    COMPLETE_PATIENT_REVIEW_TOOL, PATIENT_MESSAGE_REPLY_TOOL, READ_PATIENT_CONVERSATION_TOOL,
    READ_PATIENT_REVIEWS_TOOL,
};
use crate::physician_chat::{
    LIST_FILTERED_MESSAGES_TOOL, RESTORE_FILTERED_MESSAGE_TOOL, SKILL_VIEW,
};
use crate::physician_sources::{READ_TOOL, SEARCH_TOOL, citations};
use crate::runtime::chat_operations::{REFRESH_SOURCES_TOOL, RETRIEVE_PATIENT_WORK_TOOL};
use crate::runtime::concerns::CONCERN_TOOL;
use crate::runtime::follow_ups::FOLLOW_UP_TOOL;
use crate::runtime::memory::tools::{MemoryStepKind, MemoryTool};
use crate::runtime::reply::FINISH_TOOL as REPLY_FINISH_TOOL;
use crate::runtime::working_notes;
use crate::spruce_matching_recipe::{PATIENT_LOOKUP_TOOL, SUBMIT_CLASSIFICATION_TOOL};
use crate::visit_preparation::{SAVE_BRIEFING_TOOL, SELECT_VISITS_TOOL};

/// High-level workflow role of any Coda tool invocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolClass {
    /// Patient chart keyword search (`search_patient_information`).
    ChartSearch,
    /// Retained chart source read (`read_patient_sources`).
    ChartRead,
    /// Paged document OCR/page read (`read_document_page`).
    DocumentPageRead,
    /// Live connected source refresh (`refresh_patient_sources`).
    RefreshSources,
    /// Prior conversation/work retrieval (`retrieve_patient_work`).
    PriorPatientWork,
    /// Patient Today concern create/revise/retire (`patient_concern`).
    PatientConcern,
    /// Patient follow-up management (`patient_follow_up`).
    FollowUp,
    /// Patient understanding read (`read_patient_understanding`).
    PatientUnderstandingRead,
    /// Patient understanding write (`save_patient_understanding`).
    PatientUnderstandingSave,
    /// Prior patient reviews read (`read_patient_reviews`).
    PatientReviewsRead,
    /// Patient review disposition write (`complete_patient_review`).
    PatientReviewComplete,
    /// Patient messaging conversation read (`read_patient_conversation`).
    PatientConversationRead,
    /// Filtered patient messages inspection (`list_filtered_messages`).
    FilteredMessagesRead,
    /// Filtered patient message restoration (`restore_filtered_message`).
    FilteredMessagesRestore,
    /// Message review reply proposal or disposition (`patient_message_reply`, `complete_message_review`).
    MessageReply,
    /// Patient reply revision completion (`finish_reply_revision`).
    ReplyRevision,
    /// Pre-visit preparation or briefing (`select_visit_preparations`, `save_visit_briefing`).
    VisitPreparation,
    /// Unmatched Spruce message lookup or classification (`patient_lookup`, `submit_message_classification`).
    SpruceMatching,
    /// Clinical library search or passage read (`search_clinical_library`, `read_clinical_library`).
    ClinicalLibrary,
    /// Read-only patient memory inspection (`MemoryTool::reads`).
    MemoryRead,
    /// Patient memory mutation (`MemoryTool` writes).
    MemoryWrite,
    /// Earlier chat history inspection (`list/read/search_conversation_history`).
    ConversationHistory,
    /// Internal skill view (`skill_view`).
    Skill,
    /// Paged tool-result expansion (`read_tool_result`).
    ResultPage,
    /// Internal Coda working scratch notes (`save_working_note`, `working_notes`).
    WorkingNote,
    /// Chat final answer handoff (`finish_answer`).
    Finish,
}

/// Classifies any known Coda or harness tool name from a single owner table.
pub fn classify_tool(name: &str) -> Option<ToolClass> {
    use harness::engine::{history, results};

    match name {
        SEARCH_TOOL => Some(ToolClass::ChartSearch),
        READ_TOOL => Some(ToolClass::ChartRead),
        READ_DOCUMENT_PAGE_TOOL => Some(ToolClass::DocumentPageRead),
        REFRESH_SOURCES_TOOL => Some(ToolClass::RefreshSources),
        RETRIEVE_PATIENT_WORK_TOOL => Some(ToolClass::PriorPatientWork),
        CONCERN_TOOL => Some(ToolClass::PatientConcern),
        FOLLOW_UP_TOOL => Some(ToolClass::FollowUp),
        READ_UNDERSTANDING_TOOL => Some(ToolClass::PatientUnderstandingRead),
        SAVE_UNDERSTANDING_TOOL => Some(ToolClass::PatientUnderstandingSave),
        READ_PATIENT_REVIEWS_TOOL => Some(ToolClass::PatientReviewsRead),
        COMPLETE_PATIENT_REVIEW_TOOL => Some(ToolClass::PatientReviewComplete),
        READ_PATIENT_CONVERSATION_TOOL => Some(ToolClass::PatientConversationRead),
        LIST_FILTERED_MESSAGES_TOOL => Some(ToolClass::FilteredMessagesRead),
        RESTORE_FILTERED_MESSAGE_TOOL => Some(ToolClass::FilteredMessagesRestore),
        PATIENT_MESSAGE_REPLY_TOOL | COMPLETE_MESSAGE_REVIEW_TOOL => Some(ToolClass::MessageReply),
        REPLY_FINISH_TOOL => Some(ToolClass::ReplyRevision),
        SELECT_VISITS_TOOL | SAVE_BRIEFING_TOOL => Some(ToolClass::VisitPreparation),
        PATIENT_LOOKUP_TOOL | SUBMIT_CLASSIFICATION_TOOL => Some(ToolClass::SpruceMatching),
        SEARCH_LIBRARY_TOOL | READ_LIBRARY_TOOL => Some(ToolClass::ClinicalLibrary),
        history::LIST
        | history::READ
        | history::SEARCH
        | "history_list"
        | "history_read"
        | "history_search" => Some(ToolClass::ConversationHistory),
        SKILL_VIEW => Some(ToolClass::Skill),
        results::READ => Some(ToolClass::ResultPage),
        citations::FINISH_TOOL => Some(ToolClass::Finish),
        "save_working_note" | "working_notes" => Some(ToolClass::WorkingNote),
        n if working_notes::is_tool(n) => Some(ToolClass::WorkingNote),
        n if MemoryTool::named(n).is_some_and(MemoryTool::reads) => Some(ToolClass::MemoryRead),
        n if MemoryTool::named(n).is_some() => Some(ToolClass::MemoryWrite),
        _ => None,
    }
}

/// Execution state of a projected run step.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectedStepState {
    /// Started and currently in progress.
    Running,
    /// Completed normally.
    Done,
    /// Stopped before completing.
    Stopped,
    /// Refused, failed, or ended at a limit.
    Failed,
    /// Interrupted without a saved tool result (for example across a worker restart).
    Interrupted,
}

impl ProjectedStepState {
    /// Short ASCII status marker used by CLI output.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Done => "done",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
            Self::Interrupted => "interrupted",
        }
    }
}

/// Physician-readable run step projected from harness events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedStep {
    /// Plain clinical label describing the step.
    pub label: String,
    /// Optional human-readable target detail (search query, record titles, or page).
    pub detail: Option<String>,
    /// Current or terminal state of the step.
    pub state: ProjectedStepState,
}

impl ProjectedStep {
    /// Formats this step as a concise single line for CLI status output.
    pub fn display_line(&self) -> String {
        match self
            .detail
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty())
        {
            Some(detail) => format!("[{}] {} — {detail}", self.state.tag(), self.label),
            None => format!("[{}] {}", self.state.tag(), self.label),
        }
    }
}

/// One model reasoning summary extracted from a saved run for Coda staff inspection.
///
/// Reasoning summaries never belong on physician-facing API responses or UI surfaces.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningSummary {
    /// Attempt that recorded the model turn containing this reasoning item.
    pub attempt_id: String,
    /// Event ordinal within the attempt.
    pub ordinal: u64,
    /// Provider reasoning item identifier when supplied.
    pub item_id: Option<String>,
    /// Summary paragraphs supplied by the provider for this reasoning item.
    pub summary: Vec<String>,
    /// Whether encrypted reasoning continuation bytes accompanied the item.
    pub has_encrypted_content: bool,
}

/// Projects one tool call into a physician-facing [`ProjectedStep`], or `None`
/// for internal bookkeeping calls that should stay hidden.
pub fn project_tool_step(
    name: &str,
    arguments: &str,
    state: ProjectedStepState,
    resolve_record_title: &dyn Fn(&str) -> Option<String>,
) -> Option<ProjectedStep> {
    let parsed: Option<Value> = serde_json::from_str(arguments).ok();
    let class = classify_tool(name);
    let (label, detail) = match class {
        Some(ToolClass::ChartSearch) => {
            let query = parsed
                .as_ref()
                .and_then(|v| v.get("query"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned);
            ("Searched patient chart".to_owned(), query)
        }
        Some(ToolClass::ChartRead) => {
            let titles: Vec<String> = parsed
                .as_ref()
                .and_then(|v| v.get("references"))
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|entry| {
                            let id = entry
                                .get("candidate_id")
                                .and_then(Value::as_str)
                                .or_else(|| entry.get("reference").and_then(Value::as_str))
                                .or_else(|| entry.as_str())
                                .map(str::trim)
                                .filter(|s| !s.is_empty())?;
                            Some(resolve_record_title(id).unwrap_or_else(|| id.to_owned()))
                        })
                        .collect()
                })
                .unwrap_or_default();
            let detail = (!titles.is_empty()).then(|| titles.join(", "));
            ("Read chart records".to_owned(), detail)
        }
        Some(ToolClass::DocumentPageRead) => {
            let detail = parsed.as_ref().and_then(|v| {
                let doc = v
                    .get("document_id")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty())?;
                let label = resolve_record_title(doc).unwrap_or_else(|| doc.to_owned());
                Some(match v.get("page_number").and_then(Value::as_u64) {
                    Some(page) => format!("{label} (page {page})"),
                    None => label,
                })
            });
            ("Read document page".to_owned(), detail)
        }
        Some(ToolClass::RefreshSources) => ("Checked connected sources".to_owned(), None),
        Some(ToolClass::PatientReviewsRead) => ("Checked prior patient reviews".to_owned(), None),
        Some(ToolClass::PatientReviewComplete) => ("Recorded review outcome".to_owned(), None),
        Some(ToolClass::PatientUnderstandingRead) => ("Read patient summary".to_owned(), None),
        Some(ToolClass::PatientUnderstandingSave) => ("Updated patient summary".to_owned(), None),
        Some(ToolClass::PatientConcern) => ("Updated Today concern".to_owned(), None),
        Some(ToolClass::FollowUp) => ("Updated patient follow-up".to_owned(), None),
        Some(ToolClass::MemoryWrite) => {
            let label = match MemoryTool::named(name).and_then(MemoryTool::physician_step)? {
                MemoryStepKind::SavedNote => "Saved note to patient memory",
                MemoryStepKind::EditedMemory => "Updated patient memory",
            };
            (label.to_owned(), None)
        }
        Some(ToolClass::PriorPatientWork) => {
            ("Checked prior patient conversations".to_owned(), None)
        }
        Some(ToolClass::ClinicalLibrary) => ("Checked clinical guidance".to_owned(), None),
        Some(ToolClass::VisitPreparation) => ("Prepared visit briefing".to_owned(), None),
        Some(ToolClass::MessageReply | ToolClass::ReplyRevision) => {
            ("Updated patient reply draft".to_owned(), None)
        }
        Some(ToolClass::PatientConversationRead) => {
            ("Checked patient conversation".to_owned(), None)
        }
        Some(ToolClass::FilteredMessagesRead) => {
            ("Checked filtered patient messages".to_owned(), None)
        }
        Some(ToolClass::FilteredMessagesRestore) => {
            ("Restored filtered patient message".to_owned(), None)
        }
        Some(ToolClass::SpruceMatching) => ("Reviewed unmatched patient message".to_owned(), None),
        // Memory reads are Coda's own bookkeeping and never show a step (`Memory.Code.21`).
        Some(
            ToolClass::MemoryRead
            | ToolClass::Skill
            | ToolClass::WorkingNote
            | ToolClass::ResultPage
            | ToolClass::ConversationHistory
            | ToolClass::Finish,
        ) => return None,
        None => ("Checked clinical context".to_owned(), None),
    };

    Some(ProjectedStep {
        label,
        detail,
        state,
    })
}

/// Projects ordered physician-readable [`ProjectedStep`]s from one or more attempts'
/// harness events.
///
/// Deduplicates retried calls by `call.id` and applies any later `ToolReturned`
/// result across attempts, while using each call's own unfinished state when
/// no `ToolReturned` was recorded.
pub fn project_steps_from_events<'a, I>(
    attempts: I,
    resolve_record_title: &dyn Fn(&str) -> Option<String>,
) -> Vec<ProjectedStep>
where
    I: IntoIterator<Item = (ProjectedStepState, &'a [Event])>,
{
    let mut started: Vec<(String, ProjectedStepState, ProjectedStep)> = Vec::new();
    let mut seen_ids: BTreeSet<String> = BTreeSet::new();
    let mut returned: HashMap<&str, ProjectedStepState> = HashMap::new();

    for (unfinished_state, events) in attempts {
        for event in events {
            match &event.kind {
                EventKind::ToolStarted(call) => {
                    if seen_ids.insert(call.id.clone())
                        && let Some(step) = project_tool_step(
                            &call.name,
                            &call.arguments,
                            unfinished_state,
                            resolve_record_title,
                        )
                    {
                        started.push((call.id.clone(), unfinished_state, step));
                    }
                }
                EventKind::ToolReturned { call_id, result } => {
                    let settled = match result {
                        ToolResult::Original(_) | ToolResult::History(_) | ToolResult::View(_) => {
                            ProjectedStepState::Done
                        }
                        ToolResult::Refused(_) => ProjectedStepState::Failed,
                    };
                    returned.insert(call_id.as_str(), settled);
                }
                EventKind::ProviderStarted { .. }
                | EventKind::ProviderObservation { .. }
                | EventKind::ProviderReturned(_)
                | EventKind::ProviderFailed { .. }
                | EventKind::TransportRetry { .. }
                | EventKind::ProviderBlocked { .. }
                | EventKind::ContextRejected
                | EventKind::Candidate { .. }
                | EventKind::WindowOpened { .. }
                | EventKind::CallBegan { .. } => {}
            }
        }
    }

    started
        .into_iter()
        .map(|(call_id, default_state, mut step)| {
            step.state = returned
                .get(call_id.as_str())
                .copied()
                .unwrap_or(default_state);
            step
        })
        .collect()
}

/// Extracts staff-only [`ReasoningSummary`] entries from saved harness events.
pub fn extract_reasoning_summaries(events: &[Event]) -> Vec<ReasoningSummary> {
    let mut summaries = Vec::new();
    for event in events {
        let EventKind::ProviderReturned(turn) = &event.kind else {
            continue;
        };
        for item in &turn.items {
            let Item::Opaque(value) = item else {
                continue;
            };
            if value.get("type").and_then(Value::as_str) != Some("reasoning") {
                continue;
            }
            let item_id = value
                .get("id")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned);
            let mut summary = Vec::new();
            if let Some(entries) = value.get("summary").and_then(Value::as_array) {
                for entry in entries {
                    let text = entry
                        .as_str()
                        .or_else(|| entry.get("text").and_then(Value::as_str))
                        .map(str::trim)
                        .filter(|s| !s.is_empty());
                    if let Some(text) = text {
                        summary.push(text.to_owned());
                    }
                }
            }
            let has_encrypted_content = value
                .get("encrypted_content")
                .and_then(Value::as_str)
                .is_some_and(|s| !s.is_empty());
            if !summary.is_empty() || has_encrypted_content || item_id.is_some() {
                summaries.push(ReasoningSummary {
                    attempt_id: event.attempt_id.clone(),
                    ordinal: event.ordinal,
                    item_id,
                    summary,
                    has_encrypted_content,
                });
            }
        }
    }
    summaries
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness::engine::{Invocation, Turn};

    #[test]
    fn projects_steps_and_omits_internal_tools() {
        let events = vec![
            Event {
                attempt_id: "att-1".into(),
                sequence: 1,
                ordinal: 0,
                kind: EventKind::ToolStarted(Invocation {
                    id: "c1".into(),
                    name: SKILL_VIEW.into(),
                    arguments: r#"{"name":"patient-memory-upkeep"}"#.into(),
                }),
            },
            Event {
                attempt_id: "att-1".into(),
                sequence: 2,
                ordinal: 1,
                kind: EventKind::ToolStarted(Invocation {
                    id: "c2".into(),
                    name: READ_TOOL.into(),
                    arguments: r#"{"references":[{"candidate_id":"ehr/Observation/potassium"}]}"#
                        .into(),
                }),
            },
            Event {
                attempt_id: "att-1".into(),
                sequence: 3,
                ordinal: 2,
                kind: EventKind::ToolReturned {
                    call_id: "c2".into(),
                    result: ToolResult::Original("{}".into()),
                },
            },
        ];

        let steps =
            project_steps_from_events([(ProjectedStepState::Running, events.as_slice())], &|id| {
                (id == "ehr/Observation/potassium").then(|| "Potassium panel".to_owned())
            });
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].label, "Read chart records");
        assert_eq!(steps[0].detail.as_deref(), Some("Potassium panel"));
        assert_eq!(steps[0].state, ProjectedStepState::Done);
    }

    #[test]
    fn extracts_staff_reasoning_summaries_without_affecting_steps() {
        let events = vec![Event {
            attempt_id: "att-1".into(),
            sequence: 1,
            ordinal: 3,
            kind: EventKind::ProviderReturned(Turn {
                items: vec![Item::Opaque(serde_json::json!({
                    "type": "reasoning",
                    "id": "rs_123",
                    "summary": [{"type": "summary_text", "text": "Checking potassium trend before revising."}],
                    "encrypted_content": "opaque-blob"
                }))],
                receipt: Value::Null,
                usage: Default::default(),
            }),
        }];

        let steps =
            project_steps_from_events([(ProjectedStepState::Done, events.as_slice())], &|_| None);
        assert!(steps.is_empty());

        let reasoning = extract_reasoning_summaries(&events);
        assert_eq!(reasoning.len(), 1);
        assert_eq!(reasoning[0].item_id.as_deref(), Some("rs_123"));
        assert_eq!(
            reasoning[0].summary,
            vec!["Checking potassium trend before revising."]
        );
        assert!(reasoning[0].has_encrypted_content);
    }

    #[test]
    fn memory_reads_stay_hidden_and_memory_writes_project_distinct_steps() {
        for read_tool in [
            "list_patient_memory",
            "read_patient_memory",
            "search_patient_memory",
        ] {
            assert_eq!(
                project_tool_step(read_tool, "{}", ProjectedStepState::Done, &|_| None),
                None,
                "{read_tool} must not project a physician-facing step (Memory.Code.21)"
            );
        }

        let note_step = project_tool_step(
            "add_patient_memory_note",
            "{}",
            ProjectedStepState::Done,
            &|_| None,
        )
        .expect("add_patient_memory_note projects a step");
        assert_eq!(note_step.label, "Saved note to patient memory");
        assert_eq!(note_step.detail, None);

        let edit_step = project_tool_step(
            "edit_patient_memory",
            "{}",
            ProjectedStepState::Done,
            &|_| None,
        )
        .expect("edit_patient_memory projects a step");
        assert_eq!(edit_step.label, "Updated patient memory");
        assert_eq!(edit_step.detail, None);
    }
}
