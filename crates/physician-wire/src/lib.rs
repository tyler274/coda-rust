//! Physician reads the Dioxus site uses.
//!
//! The physician server publishes TypeScript types, not OpenAPI, so these structs
//! follow `src/server/apiTypes.ts` and `TodayListProjection` for the fields the
//! screens show. Extra JSON fields are ignored, which keeps a newer server readable.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `HealthResponse` value exchanged by this crate.
pub struct HealthResponse {
    /// The `status` value.
    pub status: String,
    #[serde(default)]
    /// The `retrying` value.
    pub retrying: Vec<Value>,
    #[serde(default)]
    /// The `held_by_newer_release` value.
    pub held_by_newer_release: Vec<Value>,
    #[serde(default)]
    /// Whether the physician can manage connection accounts.
    pub capabilities: Option<HealthCapabilities>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
/// The capability flags published with the health check.
pub struct HealthCapabilities {
    #[serde(default)]
    /// True when the server can open the Connections accounts dialog.
    pub connection_accounts: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `TodayListResponse` value exchanged by this crate.
pub struct TodayListResponse {
    /// The `date` value.
    pub date: String,
    /// The `rows` value.
    pub rows: Vec<TodayRow>,
    /// The `as_of` value.
    pub as_of: i64,
    #[serde(default)]
    /// The `connections` value.
    pub connections: Vec<TodayConnection>,
    #[serde(default)]
    /// The `incomplete` value.
    pub incomplete: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// A decided item in a patient's active care plan.
pub struct WorkspacePlanItem {
    /// Plan item identifier.
    pub item_id: String,
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Short plan item title.
    pub title: String,
    /// Clinical detail for the plan item.
    pub detail: String,
    /// Timestamp when the item was decided.
    pub decided_at: String,
    #[serde(default)]
    /// Recommendation identifier if promoted from a recommendation.
    pub source_recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// A medication recommendation awaiting or reflecting a physician decision.
pub struct WorkspaceMedicationRecommendation {
    /// Recommendation identifier.
    pub recommendation_id: String,
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Medication name and dose.
    pub medication: String,
    /// Clinical rationale or context.
    pub rationale: String,
    /// Source note or review label.
    pub source_label: String,
    /// Current decision state (`undecided`, `decided`, `declined`).
    pub decision_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// A drafted patient reply on a workspace projection.
pub struct WorkspaceReply {
    /// Reply work identifier.
    pub reply_work_id: String,
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Patient display name.
    pub patient_display_name: String,
    /// Outbound channel (`spruce` or `email`).
    pub channel: String,
    /// Summary headline.
    pub headline: String,
    /// Drafted message text.
    pub draft_text: String,
    /// Urgency tier (`urgent_ed` or `routine`).
    pub urgency: String,
    /// Timestamp when the reply was drafted.
    pub created_at: String,
    /// Lifecycle state (`ready`, `superseded_by_correction`, `closed_by_physician`).
    pub state: String,
    #[serde(default)]
    /// Input key that superseded this reply, if any.
    pub superseded_by_input: Option<String>,
    #[serde(default)]
    /// Reason this reply was superseded, if any.
    pub superseded_reason: Option<String>,
    #[serde(default)]
    /// The saved reply draft's exact revision, when a hosted owner holds it.
    /// A revision request names this revision; absent on recorded replies.
    pub draft_revision: Option<u64>,
    #[serde(default)]
    /// The latest revision run on the saved draft, when one was accepted.
    pub revision_run: Option<ReplyRevisionRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The latest accepted revision of a saved reply draft and how it stands.
pub struct ReplyRevisionRun {
    /// Stable execution identity.
    pub run_id: String,
    /// `pending`, `active`, `completed`, `held`, `failed` or `cancelled`.
    pub status: String,
    /// Present only while the saved draft permits this run to continue.
    pub resume_target: Option<WorkResumeTarget>,
}

impl ReplyRevisionRun {
    /// Whether the run is still waiting or working.
    pub fn running(&self) -> bool {
        matches!(self.status.as_str(), "pending" | "active")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The reply-draft route's answer: saved text at an exact revision.
pub struct ReplyDraftWire {
    /// Saved revision.
    pub revision: u64,
    /// Saved text.
    pub text: String,
    #[serde(default)]
    /// The text the latest save or revision replaced.
    pub restore_text: Option<String>,
    #[serde(default)]
    /// The latest accepted revision run, if any.
    pub revision_run: Option<ReplyRevisionRun>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `TodayConnection` value exchanged by this crate.
pub struct TodayConnection {
    /// The `condition` value.
    pub condition: String,
    #[serde(default)]
    /// The `channel` value.
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `TodayRow` value exchanged by this crate.
pub struct TodayRow {
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: String,
    /// The `patient_display_name` value.
    pub patient_display_name: String,
    /// The `item_id` value.
    pub item_id: String,
    /// The `version_id` value.
    pub version_id: String,
    /// The `waiting_since` value.
    pub waiting_since: String,
    /// The `urgency` value.
    pub urgency: String,
    /// The `kind` value.
    pub kind: String,
    #[serde(default)]
    /// The `reason` value.
    pub reason: Option<String>,
    #[serde(default)]
    /// The `work_kind` value.
    pub work_kind: Option<String>,
    #[serde(default)]
    /// The `resume_target` value.
    pub resume_target: Option<WorkResumeTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The `WorkResumeTarget` value exchanged by this crate.
pub struct WorkResumeTarget {
    /// The `work_item_id` value.
    pub work_item_id: String,
    /// The `expected_run_id` value.
    pub expected_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The work a running chat turn's Stop command names.
pub struct WorkStopTarget {
    /// The `work_item_id` value.
    pub work_item_id: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `PatientWorkspacesResponse` value exchanged by this crate.
pub struct PatientWorkspacesResponse {
    /// The `patients` value.
    pub patients: Vec<PatientWorkspaceSummary>,
    #[serde(default)]
    /// The `incomplete` value.
    pub incomplete: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `PatientWorkspaceSummary` value exchanged by this crate.
pub struct PatientWorkspaceSummary {
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: String,
    /// The `display_name` value.
    pub display_name: String,
    /// The `cursor` value.
    pub cursor: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `WorkspaceChat` value exchanged by this crate.
pub struct WorkspaceChat {
    /// The `chat_id` value.
    pub chat_id: String,
    #[serde(default)]
    /// The `title` value.
    pub title: Option<String>,
    /// The `created_at` value.
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `PracticeDayResponse` value exchanged by this crate.
pub struct PracticeDayResponse {
    /// The `current_time` value.
    pub current_time: String,
    /// The `time_zone` value.
    pub time_zone: String,
    /// The `source_state` value.
    pub source_state: PracticeSourceState,
    #[serde(default)]
    /// The `visits` value.
    pub visits: Vec<PracticeVisit>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PracticeSourceState` value exchanged by this crate.
pub struct PracticeSourceState {
    /// The `state` value.
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PracticeVisit` value exchanged by this crate.
pub struct PracticeVisit {
    #[serde(default)]
    /// The `patient_display_name` value.
    pub patient_display_name: Option<String>,
    #[serde(default)]
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: Option<String>,
    /// The `label` value.
    pub label: String,
    /// The `starts_at` value.
    pub starts_at: String,
    #[serde(default)]
    /// The `ends_at` value.
    pub ends_at: Option<String>,
    /// The `status` value.
    pub status: String,
    #[serde(default)]
    /// The `patient_time_zone` value.
    pub patient_time_zone: Option<String>,
    #[serde(default)]
    /// The `preparation` value.
    pub preparation: Option<VisitPreparation>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `VisitPreparation` value exchanged by this crate.
pub struct VisitPreparation {
    /// The `state` value.
    pub state: String,
    #[serde(default)]
    /// The `last_attempt` value.
    pub last_attempt: Option<PreparationAttempt>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PreparationAttempt` value exchanged by this crate.
pub struct PreparationAttempt {
    /// The `outcome` value.
    pub outcome: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `RecentConversationsResponse` value exchanged by this crate.
pub struct RecentConversationsResponse {
    #[serde(default)]
    /// The `conversations` value.
    pub conversations: Vec<RecentConversation>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `RecentConversation` value exchanged by this crate.
pub struct RecentConversation {
    #[serde(default)]
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: Option<String>,
    #[serde(default)]
    /// The `title` value.
    pub title: Option<String>,
    #[serde(default)]
    /// The `patient_display_name` value.
    pub patient_display_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `UpdatesResponse` value exchanged by this crate.
pub struct UpdatesResponse {
    #[serde(default)]
    /// The `rows` value.
    pub rows: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PatientSearchResponse` value exchanged by this crate.
pub struct PatientSearchResponse {
    #[serde(default)]
    /// The `patients` value.
    pub patients: Vec<PatientSearchRow>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PatientSearchRow` value exchanged by this crate.
pub struct PatientSearchRow {
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: String,
    /// The `patient_display_name` value.
    pub patient_display_name: String,
    #[serde(default)]
    /// The `birth_date` value.
    pub birth_date: Option<String>,
    #[serde(default)]
    /// Whether this roster patient has accepted the explicit practice invitation in Duet.
    pub duet_linked: bool,
    #[serde(default)]
    /// Whether this roster patient has active `"share with my physician"` consent.
    pub sharing_consented: bool,
    #[serde(default)]
    /// The per-patient review cadence exposed to Agents (`"weekly"` or `"daily_when_changed"`).
    pub cadence: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
/// The caller's membership level for a patient workspace.
pub enum WorkspaceMembership {
    /// The patient is on the physician's panel (`Membership::Allowed`).
    #[default]
    Allowed,
    /// The patient is in the same clinic, but not on the physician's panel.
    ClinicDemographic,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
/// Per-patient review cadence reported on a workspace without payment status.
pub enum ReviewCadenceWire {
    /// Settled weekly whole-patient review only.
    #[default]
    Weekly,
    /// Settled weekly whole-patient review plus a daily review when changed.
    WeeklyAndDailyWhenChanged,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `PatientWorkspaceResponse` value exchanged by this crate.
pub struct PatientWorkspaceResponse {
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: String,
    /// The `display_name` value.
    pub display_name: String,
    #[serde(default)]
    /// The caller's membership level (`allowed` or `clinic_demographic`).
    pub membership: WorkspaceMembership,
    #[serde(default)]
    /// Per-patient review cadence read without knowing payment status.
    pub review_cadence: Option<ReviewCadenceWire>,
    #[serde(default)]
    /// The `time_zone` value.
    pub time_zone: Option<String>,
    #[serde(default)]
    /// The `chats` value.
    pub chats: Vec<WorkspaceChat>,
    #[serde(default)]
    /// The `identity` value.
    pub identity: Option<PatientIdentity>,
    #[serde(default)]
    /// The `patient_synopsis` value.
    pub patient_synopsis: Option<PatientSynopsis>,
    #[serde(default)]
    /// The `automation_run` value.
    pub automation_run: Option<AutomationRun>,
    #[serde(default)]
    /// The `active_follow_ups` value.
    pub active_follow_ups: Vec<FollowUp>,
    #[serde(default)]
    /// Whether this workspace has past work. The recording sets the flag and does not include history items.
    pub has_past_work: bool,
    #[serde(default)]
    /// When Coda last completed a review of the patient's whole care.
    pub last_reviewed_at: Option<String>,
    #[serde(default)]
    /// Connected record sources, including each last good pull.
    pub connected_sources: Vec<ConnectedSource>,
    #[serde(default)]
    /// Active ready patient replies on the workspace.
    pub replies: Vec<WorkspaceReply>,
    #[serde(default)]
    /// Replies superseded by corrected laboratory results.
    pub superseded_replies: Vec<WorkspaceReply>,
    #[serde(default)]
    /// Decided active care-plan items for the patient.
    pub active_plan_items: Vec<WorkspacePlanItem>,
    #[serde(default)]
    /// Undecided medication recommendations held out of the active plan.
    pub undecided_recommendations: Vec<WorkspaceMedicationRecommendation>,
    #[serde(default)]
    /// The patient front-door link, versioned consent state, and review cadence (`ENG-44`).
    pub patient_link: Option<PatientLinkWire>,
}

impl PatientWorkspaceResponse {
    /// Whether this response represents a same-clinic non-panel demographic card.
    pub fn is_clinic_demographic(&self) -> bool {
        self.membership == WorkspaceMembership::ClinicDemographic
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PatientIdentity` value exchanged by this crate.
pub struct PatientIdentity {
    #[serde(default)]
    /// The `display_name` value.
    pub display_name: Option<String>,
    #[serde(default)]
    /// The `birth_date` value.
    pub birth_date: Option<String>,
    #[serde(default)]
    /// The `age_years` value.
    pub age_years: Option<i64>,
    #[serde(default)]
    /// The `sex_or_gender` value.
    pub sex_or_gender: Option<SexOrGender>,
    #[serde(default)]
    /// Medical record numbers the chart supplied.
    pub medical_record_numbers: Vec<MedicalRecordNumber>,
    #[serde(default)]
    /// The identity state (`available`, `missing`, etc.).
    pub state: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// One medical record number on the patient identity.
pub struct MedicalRecordNumber {
    #[serde(default)]
    /// The record number the chart supplied.
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `SexOrGender` value exchanged by this crate.
pub struct SexOrGender {
    #[serde(default)]
    /// The `value` value.
    pub value: Option<String>,
    #[serde(default)]
    /// The `label` value.
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PatientSynopsis` value exchanged by this crate.
pub struct PatientSynopsis {
    /// The `version_id` value.
    pub version_id: String,
    #[serde(default)]
    /// The `authored_text` value.
    pub authored_text: Option<String>,
    #[serde(default)]
    /// Typed text and citation segments. The page renders these, not marker strings.
    pub segments: Vec<ClinicalSegment>,
    #[serde(default)]
    /// When this summary version was written.
    pub authored_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// One connected record source on the workspace.
pub struct ConnectedSource {
    #[serde(default)]
    /// The source's current pull state.
    pub state: SourceConnectionState,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
/// The pull state of one connected source.
pub struct SourceConnectionState {
    #[serde(default)]
    /// `usable`, `refreshing`, `failed`, or another server state.
    pub state: String,
    #[serde(default)]
    /// When this state last pulled records.
    pub synchronized_at: Option<String>,
    #[serde(default)]
    /// The last good pull, kept when a later pull failed.
    pub usable_snapshot: Option<SourceSnapshot>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// A retained good pull.
pub struct SourceSnapshot {
    #[serde(default)]
    /// When that pull finished.
    pub synchronized_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
/// One piece of authored clinical text.
// The Citation variant carries six option fields against Text's one String,
// which trips `large_endian_variants`; boxing the wire struct would fight the
// generated serde shape, so the lint stays off for this enum.
#[allow(clippy::large_enum_variant)]
pub enum ClinicalSegment {
    /// Prose between citations.
    Text {
        /// The prose.
        text: String,
    },
    /// A source citation. Extra server fields stay unread.
    Citation {
        #[serde(default)]
        /// `resolved` or `unavailable`.
        state: String,
        #[serde(default)]
        /// The source title the chip names.
        display_label: Option<String>,
        #[serde(default)]
        /// Which catalog the citation belongs to.
        source: Option<String>,
        #[serde(default)]
        /// The source identity.
        source_id: Option<String>,
        #[serde(default)]
        /// The record handle inside that source.
        handle: Option<String>,
        #[serde(default)]
        /// The lead's short preview of this source.
        source_presentation: Option<SourcePresentation>,
        #[serde(default)]
        /// Where the original record can be opened.
        destination: Option<CitationDestination>,
    },
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
/// The preview stored on a citation.
pub struct SourcePresentation {
    #[serde(default)]
    /// The preview body and its as-of date.
    pub content: Option<SourcePreviewContent>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
/// The text a source chip opens.
pub struct SourcePreviewContent {
    #[serde(default)]
    /// The preview prose.
    pub text: String,
    #[serde(default)]
    /// The as-of date, already written for display.
    pub as_of: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
/// A link from a citation back to its record.
pub struct CitationDestination {
    #[serde(default)]
    /// `source_native` opens the connected chart.
    pub kind: String,
    #[serde(default)]
    /// The chart name, such as Demo chart.
    pub source_label: Option<String>,
    #[serde(default)]
    /// The destination URI as the server recorded it.
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `AutomationRun` value exchanged by this crate.
pub struct AutomationRun {
    /// The `kind` value.
    pub kind: String,
    /// The `state` value.
    pub state: String,
    #[serde(default)]
    /// The `ended_cause` value.
    pub ended_cause: Option<String>,
    #[serde(default)]
    /// The `purpose` value.
    pub purpose: Option<WorkPurpose>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `WorkPurpose` value exchanged by this crate.
pub struct WorkPurpose {
    #[serde(default)]
    /// The `headline` value.
    pub headline: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `FollowUp` value exchanged by this crate.
pub struct FollowUp {
    /// The `follow_up_id` value.
    pub follow_up_id: String,
    #[serde(default)]
    /// The `description` value.
    pub description: Option<String>,
}

/// A conversation as the page reads it: one exchange per piece of accepted
/// work, in the order the physician started them.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatSnapshotResponse {
    /// The `snapshot` value.
    pub snapshot: ChatSnapshot,
}

/// The conversation's transcript and whether its feedback could be read.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatSnapshot {
    /// Each piece of accepted work with its messages, steps and answer.
    pub transcript: Vec<Exchange>,
    /// Whether the signed-in physician's feedback on the answers could be read.
    pub feedback: SnapshotFeedback,
}

/// One piece of accepted work: the messages that started or steered it, the
/// steps Coda took for it and how its answer stands.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Exchange {
    /// The work these messages belong to.
    pub work_item_id: String,
    /// Every physician message that started or steered the work, in order.
    pub messages: Vec<PhysicianMessage>,
    /// What Coda did for the answer, in the order it started each step.
    pub steps: Vec<Step>,
    /// What Coda had written each time a limit cut this answer off, in the
    /// order it was saved. Try again continues after it, so it stays with
    /// the exchange whatever the answer's state.
    pub kept: Vec<String>,
    /// How the answer stands.
    pub answer: AnswerState,
}

/// One message the physician sent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhysicianMessage {
    /// The idempotency key the page sent the message with.
    pub id: String,
    /// The message as the physician wrote it.
    pub text: String,
}

/// How one exchange's answer stands. Only the conversation's current work
/// offers Resume or Try again; an answer whose work a later message replaced
/// keeps the ending it had, without either.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum AnswerState {
    /// Coda is working on the answer. No part of its text is shown yet.
    Working {
        /// The work Stop names.
        stop: WorkStopTarget,
    },
    /// Coda finished the answer.
    Finished {
        /// The answer's durable name, which feedback names.
        response_id: ResponseId,
        /// The attempt that published the answer, so an operator can match
        /// the reply to its telemetry by attempt id.
        attempt_id: String,
        /// The answer as Coda published it. Text a limit kept before it is
        /// in the exchange's `kept`.
        text: String,
        /// The physician's saved feedback on it, when they rated it and the
        /// snapshot could read it.
        feedback: Option<ResponseFeedback>,
    },
    /// The physician stopped the work.
    Stopped {
        /// The run Resume continues, while this is the current work.
        resume: Option<WorkResumeTarget>,
    },
    /// Coda reached a limit before it finished. What it kept is in the
    /// exchange's `kept`.
    CutOff {
        /// The run Try again continues, while this is the current work.
        try_again: Option<WorkResumeTarget>,
    },
    /// Coda's recovery was used up, or the model service refused the work.
    CouldntFinish {
        /// The run Try again continues, while this is the current work.
        try_again: Option<WorkResumeTarget>,
    },
}

/// One thing Coda did for an answer, as typed facts. The page words it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "did", rename_all = "snake_case")]
pub enum Step {
    /// Coda searched the patient's chart.
    SearchedChart {
        /// The search text, as Coda gave it; empty when it browsed.
        query: String,
        /// The record type the search was limited to, if any.
        record_type: Option<String>,
        /// How the search stands.
        state: StepState<Found>,
    },
    /// Coda opened records from the patient's chart.
    ReadRecords {
        /// How many records it asked for.
        requested: usize,
        /// How the read stands, with what each record gave once it returned.
        state: StepState<Vec<RecordRead>>,
    },
    /// Coda looked at earlier messages of the conversation. It may have found nothing.
    ReadConversation {
        /// How the look stands.
        state: StepState<()>,
    },
}

/// How one step stands.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum StepState<T> {
    /// It started and has not returned.
    Running,
    /// It returned this result.
    Returned {
        /// What it gave.
        result: T,
    },
    /// It returned no output: it was refused, failed or timed out.
    Failed,
    /// The physician stopped the work before it returned. How it ended is
    /// not known.
    Stopped,
    /// The work's attempt ended before it returned, for a reason other than
    /// Stop, such as a restart. How it ended is not known.
    Interrupted,
}

/// What a chart search found.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Found {
    /// Records on this page of results.
    pub records: usize,
    /// More results follow this page.
    pub more: bool,
}

/// What one record of a chart read gave.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum RecordRead {
    /// The whole record.
    Whole {
        /// The source's name for this kind of record.
        record_type: String,
        /// The record's dates, in the source's words.
        dates: Vec<RecordDate>,
    },
    /// One passage of a large record.
    Passage,
    /// The record is not in the chart, or its revision changed.
    Unavailable,
    /// The record did not fit the read's size budget.
    TooLarge,
}

/// One date on a record, as the source wrote it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordDate {
    /// The source's name for this date.
    pub name: String,
    /// The value as the source gave it.
    pub value: String,
}

/// Whether a chat snapshot could read the physician's feedback. The
/// conversation comes either way; only the ratings are missing when it
/// couldn't.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotFeedback {
    /// Each rated response carries its saved feedback; one without is unrated.
    #[default]
    Loaded,
    /// The feedback read failed, so no response carries feedback and none
    /// can be told apart from unrated.
    Unavailable,
}

/// The durable name of one of Coda's published replies, which stays the same
/// across restarts and reloads.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct ResponseId(pub String);

impl ResponseId {
    /// The response id as the server sent it.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A physician's rating of one of Coda's responses.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackRating {
    /// A good response.
    Up,
    /// A poor response.
    Down,
}

/// A reason picked in a rating's details.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackReason {
    /// "Incorrect information".
    IncorrectInformation,
    /// "Missing context".
    MissingContext,
    /// "Hard to follow".
    HardToFollow,
    /// "Not useful".
    NotUseful,
}

/// The optional details on a rating.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeedbackDetails {
    /// The reasons picked.
    pub reasons: Vec<FeedbackReason>,
    /// The physician's words.
    pub text: String,
}

/// A feedback save's body: the rating and the details together, replacing
/// both, so details never travel without a rating.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveFeedback {
    /// The rating chosen.
    pub rating: FeedbackRating,
    #[serde(default)]
    /// The details, or `None` to save the rating alone.
    pub details: Option<FeedbackDetails>,
}

/// The physician's saved feedback on one response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseFeedback {
    /// 1 on the first save, one more on each later save: the highest
    /// revision seen is the newest feedback.
    pub revision: u64,
    /// The saved rating.
    pub rating: FeedbackRating,
    #[serde(default)]
    /// The saved details, if any.
    pub details: Option<FeedbackDetails>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PhysicianDraft` value exchanged by this crate.
pub struct PhysicianDraft {
    /// The `text` value.
    pub text: String,
    /// The `revision` value.
    pub revision: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The sign-in route's answer: a token to verify, or the no-account deferral.
pub struct SignInResponse {
    /// The `status` value: `token_issued` when proof delivery returns a code,
    /// or `unavailable` for the indistinguishable "link is on its way" line.
    /// Hosted mail uses `unavailable` and omits the code.
    pub status: String,
    #[serde(default)]
    /// The sign-in token when the server issued one (proof delivery).
    pub token: Option<String>,
    #[serde(default)]
    /// The human line for `unavailable`, such as the no-account deferral
    /// and the hosted "link is on its way" response.
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The verify route's answer: the bearer session and who holds it.
pub struct VerifyResponse {
    /// The bearer session to present on every physician read.
    pub session: String,
    /// The `physician_id` value.
    pub physician_id: String,
    /// The `practice_id` value.
    pub practice_id: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// One active physician account in the caller's practice, for panel assignment.
pub struct PracticeAccountSummary {
    /// The `physician_id` value.
    pub physician_id: String,
    /// The physician's display name.
    pub display_name: String,
    #[serde(default)]
    /// The physician's sign-in email.
    pub email: String,
    #[serde(default)]
    /// Whether this account holds clinic admin / panel-assignment permission.
    pub admin: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The whoami route's answer about the session's physician.
pub struct WhoamiResponse {
    /// The `physician_id` value.
    pub physician_id: String,
    /// The `practice_id` value.
    pub practice_id: String,
    #[serde(default)]
    /// The account's email; the expired-session prompt mails its link here.
    pub email: String,
    /// The physician's display name.
    pub display_name: String,
    #[serde(default)]
    /// Whether this account holds clinic admin / panel-assignment permission.
    pub admin: bool,
    #[serde(default)]
    /// Active accounts in the caller's practice when `admin` is true.
    pub practice_accounts: Vec<PracticeAccountSummary>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The panel assignment route's answer after updating one panel membership.
pub struct PanelAssignmentResponse {
    /// The `practice_id` value.
    pub practice_id: String,
    /// The `physician_id` value.
    pub physician_id: String,
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: String,
    /// Whether the patient workspace is now on the physician's panel.
    pub allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The wire representation of a roster patient's front-door link, consent, and cadence (`ENG-44`).
pub struct PatientLinkWire {
    /// The practice that owns the roster patient.
    pub practice_id: String,
    /// The roster patient's workspace id.
    pub patient_workspace_id: String,
    #[serde(default)]
    /// The linked Duet account id, once the invitation is accepted.
    pub duet_account_id: Option<String>,
    #[serde(default)]
    /// Whether the patient accepted the practice invitation.
    pub linked: bool,
    #[serde(default)]
    /// Whether `"share with my physician"` consent is currently active.
    pub sharing_consented: bool,
    #[serde(default)]
    /// The accepted consent version (such as `"v1"`).
    pub consent_version: Option<String>,
    #[serde(default)]
    /// The Unix second consent was granted.
    pub consented_at: Option<u64>,
    #[serde(default)]
    /// The Unix second consent was revoked, if revoked.
    pub revoked_at: Option<u64>,
    #[serde(default)]
    /// The per-patient review cadence exposed to Agents (`"weekly"` or `"daily_when_changed"`).
    pub cadence: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_today_list() {
        let list: TodayListResponse = serde_json::from_str(
            r#"{
            "date": "2026-10-09",
            "rows": [{
                "patient_workspace_id": "p1",
                "patient_display_name": "Ada",
                "item_id": "i1",
                "version_id": "v1",
                "waiting_since": "2026-10-09T12:00:00Z",
                "urgency": "today",
                "kind": "concern",
                "reason": "New note"
            }],
            "connections": [],
            "cursors": {},
            "as_of": 3,
            "seen": []
        }"#,
        )
        .unwrap();
        assert_eq!(list.rows[0].patient_display_name, "Ada");
        assert_eq!(list.as_of, 3);
    }

    /// The page reads every answer state and step the site writes, and
    /// refuses the conversation shape older sites wrote.
    #[test]
    fn reads_a_transcript_and_refuses_the_older_shape() {
        let page: ChatSnapshotResponse = serde_json::from_str(
            r#"{"snapshot": {"feedback": "loaded", "transcript": [{
                "work_item_id": "w1",
                "messages": [{"id": "k1", "text": "Hello"}, {"id": "k2", "text": "And sodium?"}],
                "steps": [
                    {"did": "searched_chart", "query": "sodium", "record_type": null,
                     "state": {"state": "returned", "result": {"records": 2, "more": false}}},
                    {"did": "read_records", "requested": 2, "state": {"state": "returned", "result": [
                        {"outcome": "whole", "record_type": "Observation",
                         "dates": [{"name": "effective", "value": "2026-10-01"}]},
                        {"outcome": "unavailable"}]}},
                    {"did": "read_conversation", "state": {"state": "stopped"}}
                ],
                "kept": ["Sodium was 139"],
                "answer": {"state": "stopped",
                           "resume": {"work_item_id": "w1", "expected_run_id": "r1"}}
            }]}}"#,
        )
        .unwrap();
        let exchange = &page.snapshot.transcript[0];
        assert_eq!(exchange.messages[1].text, "And sodium?");
        assert_eq!(exchange.kept, ["Sodium was 139"]);
        assert_eq!(
            exchange.answer,
            AnswerState::Stopped {
                resume: Some(WorkResumeTarget {
                    work_item_id: "w1".into(),
                    expected_run_id: "r1".into(),
                }),
            }
        );
        assert_eq!(
            exchange.steps[2],
            Step::ReadConversation {
                state: StepState::Stopped
            }
        );
        let written = serde_json::to_value(&page).unwrap();
        assert_eq!(
            serde_json::from_value::<ChatSnapshotResponse>(written).unwrap(),
            page
        );
        assert!(
            serde_json::from_str::<ChatSnapshotResponse>(
                r#"{"snapshot": {"conversation": [], "runs": []}}"#
            )
            .is_err(),
            "the older conversation shape is not read"
        );
    }

    #[test]
    fn reads_a_snapshot_whose_feedback_was_unavailable() {
        let page: ChatSnapshotResponse = serde_json::from_str(
            r#"{"snapshot": {"feedback": "unavailable", "transcript": [{
                "work_item_id": "w1", "messages": [], "steps": [], "kept": [],
                "answer": {"state": "finished", "response_id": "a1:reply", "attempt_id": "a1",
                           "text": "Sodium is stable.", "feedback": null}
            }]}}"#,
        )
        .unwrap();
        assert_eq!(page.snapshot.feedback, SnapshotFeedback::Unavailable);
        assert!(
            serde_json::from_str::<SnapshotFeedback>(r#""partial""#).is_err(),
            "an unknown feedback state is refused, not read as loaded"
        );
    }
}
