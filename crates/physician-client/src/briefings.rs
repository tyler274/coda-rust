//! Client calls for reading visit briefings and refreshing the practice schedule.

use physician_wire::PracticeDayResponse;
use physician_wire::briefings::{
    PanelPreferencesWire, VisitBriefingResponse, WorkspaceBriefingsResponse,
};

use crate::{HttpError, PhysicianClient, query_escape};

impl PhysicianClient {
    /// Reads all saved visit briefings (current, moved, and cancelled) for an authorized patient workspace.
    pub async fn workspace_briefings(
        &self,
        workspace_id: &str,
    ) -> Result<WorkspaceBriefingsResponse, HttpError> {
        self.get(&format!(
            "/patient-workspaces/{}/briefings",
            query_escape(workspace_id)
        ))
        .await
    }

    /// Reads one saved visit briefing and its cited source passages.
    pub async fn visit_briefing(
        &self,
        workspace_id: &str,
        briefing_id: &str,
    ) -> Result<VisitBriefingResponse, HttpError> {
        self.get(&format!(
            "/patient-workspaces/{}/briefings/{}",
            query_escape(workspace_id),
            query_escape(briefing_id)
        ))
        .await
    }

    /// Requests a live refresh of the practice's calendar schedule and returns the resulting practice day.
    pub async fn refresh_practice_day(&self) -> Result<PracticeDayResponse, HttpError> {
        self.post("/practice-day/refresh", &serde_json::json!({}))
            .await
    }

    /// Reads the signed-in physician's server-saved sidebar and schedule collapse choices.
    pub async fn panel_preferences(&self) -> Result<PanelPreferencesWire, HttpError> {
        if self.is_demo() {
            return Ok(PanelPreferencesWire::default());
        }
        self.get("/physician-state/panels").await
    }

    /// Persists the signed-in physician's sidebar and schedule collapse choices on the server.
    pub async fn save_panel_preferences(
        &self,
        preferences: &PanelPreferencesWire,
    ) -> Result<PanelPreferencesWire, HttpError> {
        if self.is_demo() {
            return Ok(*preferences);
        }
        self.put("/physician-state/panels", preferences).await
    }
}
