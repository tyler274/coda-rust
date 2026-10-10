//! Physician reads the Dioxus site uses.
//!
//! The physician server publishes TypeScript types, not OpenAPI, so these structs
//! follow `src/server/apiTypes.ts` and `TodayListProjection` for the fields the
//! screens show. Extra JSON fields are ignored, which keeps a newer server readable.

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    #[serde(default)]
    pub retrying: Vec<Value>,
    #[serde(default)]
    pub held_by_newer_release: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TodayListResponse {
    pub date: String,
    pub rows: Vec<TodayRow>,
    pub as_of: i64,
    #[serde(default)]
    pub connections: Vec<TodayConnection>,
    #[serde(default)]
    pub incomplete: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TodayConnection {
    pub condition: String,
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TodayRow {
    pub patient_workspace_id: String,
    pub patient_display_name: String,
    pub item_id: String,
    pub version_id: String,
    pub waiting_since: String,
    pub urgency: String,
    pub kind: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub work_kind: Option<String>,
    #[serde(default)]
    pub resume_target: Option<WorkResumeTarget>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct WorkResumeTarget {
    pub work_item_id: String,
    pub expected_run_id: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PatientWorkspacesResponse {
    pub patients: Vec<PatientWorkspaceSummary>,
    #[serde(default)]
    pub incomplete: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PatientWorkspaceSummary {
    pub patient_workspace_id: String,
    pub display_name: String,
    pub cursor: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct WorkspaceChat {
    pub chat_id: String,
    #[serde(default)]
    pub title: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PracticeDayResponse {
    pub current_time: String,
    pub time_zone: String,
    pub source_state: PracticeSourceState,
    #[serde(default)]
    pub visits: Vec<PracticeVisit>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PracticeSourceState {
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PracticeVisit {
    #[serde(default)]
    pub patient_display_name: Option<String>,
    #[serde(default)]
    pub patient_workspace_id: Option<String>,
    pub label: String,
    pub starts_at: String,
    #[serde(default)]
    pub ends_at: Option<String>,
    pub status: String,
    #[serde(default)]
    pub patient_time_zone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RecentConversationsResponse {
    #[serde(default)]
    pub conversations: Vec<RecentConversation>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RecentConversation {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub patient_display_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct UpdatesResponse {
    #[serde(default)]
    pub rows: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PatientSearchResponse {
    #[serde(default)]
    pub patients: Vec<PatientSearchRow>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PatientSearchRow {
    pub patient_workspace_id: String,
    pub patient_display_name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PatientWorkspaceResponse {
    pub patient_workspace_id: String,
    pub display_name: String,
    #[serde(default)]
    pub chats: Vec<WorkspaceChat>,
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
