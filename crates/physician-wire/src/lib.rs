//! Physician reads the Dioxus site uses.
//!
//! The physician server publishes TypeScript types, not OpenAPI, so these structs
//! follow `src/server/apiTypes.ts` and `TodayListProjection` for the fields the
//! screens show. Extra JSON fields are ignored, which keeps a newer server readable.
//! Book: `book/src/north-star.md`.
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
    #[serde(default)]
    /// Active simulation status and investor metric when running in simulation mode.
    pub simulation: Option<SimulationStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Investor metric for a simulation run: items the physician touched per 100 patients per week.
pub struct InvestorMetricWire {
    /// Count of items directly touched by the physician during the run.
    pub physician_touched_items: u32,
    /// Count of episodes/reviews handled ambiently by Coda without physician touch.
    pub ambient_handled_items: u32,
    /// Number of patients on the simulated panel.
    pub panel_size: u32,
    /// Duration of the simulation run in weeks.
    pub simulated_weeks: f64,
    /// Normalized investor metric: `(physician_touched_items / (panel_size * simulated_weeks)) * 100.0`.
    pub items_per_100_patients_per_week: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Aggregate investor metric across all simulation runs in the session.
pub struct SimulationAggregateMetricWire {
    /// Total simulation runs executed in this session.
    pub runs_count: u32,
    /// Total physician-touched items across all runs.
    pub total_physician_touched_items: u32,
    /// Total ambiently handled items across all runs.
    pub total_ambient_handled_items: u32,
    /// Total patient-weeks simulated across all runs.
    pub total_patient_weeks: f64,
    /// Aggregate items the physician touched per 100 patients per week across runs.
    pub items_per_100_patients_per_week: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Metadata for one rerunnable simulation scenario.
pub struct SimulationScenarioWire {
    /// Scenario key (`connected-walk`, `reviewability`, etc.).
    pub key: String,
    /// Short human-readable scenario title.
    pub label: String,
    /// Detailed description of what the scenario exercises.
    pub description: String,
    /// Simulated duration in days.
    pub simulated_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Summary of one synthetic patient on Roger's panel.
pub struct SimulationPanelPatientWire {
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Patient display name.
    pub display_name: String,
    /// Date of birth (`YYYY-MM-DD`).
    pub birth_date: String,
    /// Medical record number.
    pub mrn: String,
    /// Clinical role in the simulation scenarios.
    pub role_summary: String,
    /// Whether the simulated patient takes no unprompted action.
    pub passive_patient: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Simulation status and investor metric exposed on `/today`, `/patient-workspaces/{id}`, and `/local-proof/simulations`.
pub struct SimulationStatus {
    /// Active scenario key.
    pub active_scenario_key: String,
    /// Human-readable label for the active scenario.
    pub active_scenario_label: String,
    /// Current simulated clock (`ISO-8601`).
    pub current_time: String,
    /// Investor metric for the current run.
    pub investor_metric: InvestorMetricWire,
    /// Aggregate investor metric across runs in this session.
    pub aggregate_metric: SimulationAggregateMetricWire,
    #[serde(default)]
    /// Available scenarios Roger can run or rerun.
    pub scenarios: Vec<SimulationScenarioWire>,
    #[serde(default)]
    /// Synthetic patients on Roger's panel.
    pub panel_patients: Vec<SimulationPanelPatientWire>,
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `WorkResumeTarget` value exchanged by this crate.
pub struct WorkResumeTarget {
    /// The `work_item_id` value.
    pub work_item_id: String,
    /// The `expected_run_id` value.
    pub expected_run_id: String,
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
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// The `PatientWorkspaceResponse` value exchanged by this crate.
pub struct PatientWorkspaceResponse {
    /// The `patient_workspace_id` value.
    pub patient_workspace_id: String,
    /// The `display_name` value.
    pub display_name: String,
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
    /// Active simulation status and investor metric when running in simulation mode.
    pub simulation: Option<SimulationStatus>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `PatientIdentity` value exchanged by this crate.
pub struct PatientIdentity {
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
/// The `ChatSnapshot` value exchanged by this crate.
pub struct ChatSnapshot {
    #[serde(default)]
    /// The `conversation` value.
    pub conversation: Vec<ChatTurn>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The `ChatTurn` value exchanged by this crate.
pub struct ChatTurn {
    #[serde(default)]
    /// The `text` value.
    pub text: Option<String>,
    #[serde(default)]
    /// The `authored_text` value.
    pub authored_text: Option<String>,
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
    /// The `status` value: `token_issued` or `unavailable`.
    pub status: String,
    #[serde(default)]
    /// The sign-in token when the server issued one (proof delivery).
    pub token: Option<String>,
    #[serde(default)]
    /// The human line for `unavailable`, such as the no-account deferral.
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
/// The whoami route's answer about the session's physician.
pub struct WhoamiResponse {
    /// The `physician_id` value.
    pub physician_id: String,
    /// The `practice_id` value.
    pub practice_id: String,
    /// The physician's display name.
    pub display_name: String,
    #[serde(default)]
    /// Whether this account holds clinic admin / panel-assignment permission.
    pub admin: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
/// The panel route's answer: the patient workspaces on one physician's panel.
pub struct PanelResponse {
    /// The `practice_id` value.
    pub practice_id: String,
    /// The `physician_id` value.
    pub physician_id: String,
    #[serde(default)]
    /// The patient workspace IDs assigned to this physician's panel.
    pub patient_workspace_ids: Vec<String>,
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
}
