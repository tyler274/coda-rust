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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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
    /// The patient front-door link and versioned consent state.
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

/// The chat snapshot the React client reads. Extra server fields are ignored.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ChatSnapshotResponse {
    /// The `snapshot` value.
    pub snapshot: ChatSnapshot,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The run named by a chat snapshot once the turn is held, failed or answered.
pub struct ChatRun {
    /// Stable execution identity, independent of attempts.
    pub run_id: String,
    /// How the run ended, read from its `state` and that ending's fields.
    #[serde(flatten)]
    pub outcome: ChatRunOutcome,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
/// The shared terminal run outcome a chat snapshot names, with each ending's data.
pub enum ChatRunOutcome {
    /// The run waits for the physician to continue it.
    Held {
        /// Why the run needs explicit continuation.
        hold_reason: HoldReason,
    },
    /// Recovery gave up; Try again resumes the run.
    Failed,
    /// The run answered.
    Completed {
        /// The attempt that answered, so a reply can be matched to its
        /// telemetry by attempt id.
        attempt_id: String,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// Why a held chat run needs explicit continuation.
pub enum HoldReason {
    /// The physician stopped it; Resume continues it.
    Stopped,
    /// It reached its limit; Try again continues it.
    Limit,
    /// The provider refused to serve it; Try again continues it.
    ProviderBlocked,
    /// A reason this app version does not know. `#[serde(other)]` is here on
    /// purpose: the site can be newer than an open app, the reason's own text
    /// comes from the site in the conversation line, and the app treats it as
    /// any hold that is not a stop. The site never writes this.
    #[serde(other)]
    Unrecognized,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `ChatSnapshot` value exchanged by this crate.
pub struct ChatSnapshot {
    #[serde(default)]
    /// The `conversation` value.
    pub conversation: Vec<ChatTurn>,
    /// The held, failed or answered run, or an empty list when there is none.
    pub runs: Vec<ChatRun>,
    #[serde(default)]
    /// Whether the signed-in physician's feedback on the responses could be
    /// read. Sites without feedback omit it.
    pub feedback: SnapshotFeedback,
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `ChatTurn` value exchanged by this crate.
pub struct ChatTurn {
    #[serde(default)]
    /// On one of Coda's published replies: its response id. Physician and
    /// mechanical lines, and partial answers, have none.
    pub response_id: Option<ResponseId>,
    #[serde(default)]
    /// The `text` value.
    pub text: Option<String>,
    #[serde(default)]
    /// The `authored_text` value.
    pub authored_text: Option<String>,
    #[serde(default)]
    /// `physician`, `model`, or `mechanical`. Older servers omit it.
    pub kind: Option<String>,
    #[serde(default)]
    /// On a mechanical "couldn't finish" or stopped line: the work Resume names.
    pub resume_target: Option<WorkResumeTarget>,
    /// On a mechanical line while Coda is still writing: the work Stop names.
    pub stop_target: Option<WorkStopTarget>,
    #[serde(default)]
    /// On one of Coda's replies the signed-in physician rated: their saved
    /// feedback on it.
    pub feedback: Option<ResponseFeedback>,
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

impl ChatTurn {
    /// Returns the response body when this result has one.
    pub fn body(&self) -> Option<&str> {
        self.text.as_deref().filter(|text| !text.is_empty()).or(self
            .authored_text
            .as_deref()
            .filter(|text| !text.is_empty()))
    }
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
/// The wire representation of a roster patient's front-door link and consent state.
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

    #[test]
    fn reads_a_couldnt_finish_line_and_an_older_snapshot() {
        let page: ChatSnapshotResponse = serde_json::from_str(
            r#"{"snapshot": {
                "conversation": [
                    {"text": "Hello", "kind": "physician"},
                    {"text": "Coda couldn’t finish this response. Your message is saved.",
                     "kind": "mechanical",
                     "resume_target": {"work_item_id": "w1", "expected_run_id": "r1"}}
                ],
                "runs": [{"run_id": "r1", "state": "failed"}]
            }}"#,
        )
        .unwrap();
        let failed = &page.snapshot.conversation[1];
        assert_eq!(failed.kind.as_deref(), Some("mechanical"));
        assert_eq!(
            failed.resume_target,
            Some(WorkResumeTarget {
                work_item_id: "w1".into(),
                expected_run_id: "r1".into(),
            })
        );
        assert_eq!(page.snapshot.runs[0].run_id, "r1");
        assert_eq!(page.snapshot.runs[0].outcome, ChatRunOutcome::Failed);
        assert!(
            serde_json::from_str::<ChatSnapshotResponse>(r#"{"snapshot":{"conversation":[]}}"#)
                .is_err()
        );
    }

    #[test]
    fn reads_a_stop_target_on_a_running_turn() {
        let page: ChatSnapshotResponse = serde_json::from_str(
            r#"{"snapshot": {"runs": [], "conversation": [{
                "text": "Coda is writing this response.",
                "kind": "mechanical",
                "stop_target": {"work_item_id": "w1"}
            }]}}"#,
        )
        .unwrap();
        assert_eq!(
            page.snapshot.conversation[0].stop_target,
            Some(WorkStopTarget {
                work_item_id: "w1".into(),
            })
        );
        assert_eq!(page.snapshot.conversation[0].resume_target, None);
    }

    #[test]
    fn reads_the_response_id_of_a_published_reply() {
        let page: ChatSnapshotResponse = serde_json::from_str(
            r#"{"snapshot": {"runs": [], "conversation": [
                {"text": "Hello", "kind": "physician"},
                {"text": "Sodium is stable.", "kind": "model", "response_id": "a1:reply"}
            ]}}"#,
        )
        .unwrap();
        let turns = &page.snapshot.conversation;
        assert_eq!(turns[0].response_id, None);
        assert_eq!(turns[1].response_id, Some(ResponseId("a1:reply".into())));
        assert_eq!(page.snapshot.feedback, SnapshotFeedback::Loaded);
    }

    #[test]
    fn reads_a_snapshot_whose_feedback_was_unavailable() {
        let page: ChatSnapshotResponse = serde_json::from_str(
            r#"{"snapshot": {"runs": [], "feedback": "unavailable", "conversation": [
                {"text": "Sodium is stable.", "kind": "model", "response_id": "a1:reply"}
            ]}}"#,
        )
        .unwrap();
        assert_eq!(page.snapshot.feedback, SnapshotFeedback::Unavailable);
        assert_eq!(
            page.snapshot.conversation[0].body(),
            Some("Sodium is stable.")
        );
        assert!(
            serde_json::from_str::<SnapshotFeedback>(r#""partial""#).is_err(),
            "an unknown feedback state is refused, not read as loaded"
        );
    }
}
