//! Physician website caller for the existing Node `CodaServer`.
//! Book: `book/src/north-star.md`.
#![deny(missing_docs)]

use physician_wire::{
    ChatSnapshotResponse, HealthResponse, PatientSearchResponse, PatientWorkspaceResponse,
    PatientWorkspacesResponse, PhysicianDraft, PracticeDayResponse, RecentConversationsResponse,
    TodayListResponse, UpdatesResponse,
};

#[derive(Debug, Clone)]
/// The `PhysicianClient` value exchanged by this crate.
pub struct PhysicianClient {
    base: String,
    http: reqwest::Client,
}

impl PhysicianClient {
    /// Creates the client.
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into().trim_end_matches('/').to_owned(),
            http: reqwest::Client::new(),
        }
    }

    /// Reads the physician health check.
    pub async fn health(&self) -> Result<HealthResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/health", self.base)).await
    }

    /// Reads Today.
    pub async fn today(&self) -> Result<TodayListResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/today", self.base)).await
    }

    /// Reads the patient workspaces.
    pub async fn workspaces(&self) -> Result<PatientWorkspacesResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/patient-workspaces", self.base)).await
    }

    /// Reads the practice day.
    pub async fn practice_day(&self) -> Result<PracticeDayResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/practice-day", self.base)).await
    }

    /// Reads recent conversations.
    pub async fn recent_conversations(
        &self,
    ) -> Result<RecentConversationsResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/recent-conversations", self.base)).await
    }

    /// Reads Updates.
    pub async fn updates(&self) -> Result<UpdatesResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/updates", self.base)).await
    }

    /// Reads Today for one patient.
    pub async fn today_for(
        &self,
        workspace_id: &str,
    ) -> Result<TodayListResponse, http::HttpError> {
        http::get_json(
            &self.http,
            &format!(
                "{}/today?patient_workspace_id={}",
                self.base,
                query_escape(workspace_id)
            ),
        )
        .await
    }

    /// Sends a physician command.
    pub async fn command(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, http::HttpError> {
        http::post_json(&self.http, &format!("{}/commands", self.base), body).await
    }

    /// Reads one chat snapshot.
    pub async fn chat_snapshot(
        &self,
        workspace_id: &str,
        chat_id: &str,
    ) -> Result<ChatSnapshotResponse, http::HttpError> {
        http::get_json(
            &self.http,
            &format!(
                "{}/patient-workspaces/{workspace_id}/chats/{chat_id}/snapshot",
                self.base
            ),
        )
        .await
    }

    /// Reads the saved draft.
    pub async fn read_draft(
        &self,
        workspace_id: &str,
        chat_id: &str,
    ) -> Result<PhysicianDraft, http::HttpError> {
        http::get_json(
            &self.http,
            &format!(
                "{}/physician-state/drafts/{workspace_id}/{chat_id}",
                self.base
            ),
        )
        .await
    }

    /// Saves the draft at its revision.
    pub async fn save_draft(
        &self,
        workspace_id: &str,
        chat_id: &str,
        revision: i64,
        text: &str,
    ) -> Result<(), http::HttpError> {
        let _: serde_json::Value = http::put_json(
            &self.http,
            &format!(
                "{}/physician-state/drafts/{workspace_id}/{chat_id}",
                self.base
            ),
            &serde_json::json!({ "expected_revision": revision, "text": text }),
        )
        .await?;
        Ok(())
    }

    /// Sends recorded audio for transcription.
    pub async fn dictate(&self) -> Result<(), http::HttpError> {
        let _: serde_json::Value = http::post_json(
            &self.http,
            &format!("{}/dictation", self.base),
            &serde_json::json!({}),
        )
        .await?;
        Ok(())
    }

    /// Searches the patient roster.
    pub async fn search(&self, query: &str) -> Result<PatientSearchResponse, http::HttpError> {
        http::get_json(
            &self.http,
            &format!("{}/patient-roster?query={}", self.base, query_escape(query)),
        )
        .await
    }

    /// Asks the server to resume one piece of work.
    pub async fn resume_work(
        &self,
        workspace_id: &str,
        work_item_id: &str,
        expected_run_id: &str,
        time_zone: &str,
    ) -> Result<(), http::HttpError> {
        let body = serde_json::json!({
            "type": "work.resume",
            "idempotency_key": format!("resume-{work_item_id}-{expected_run_id}"),
            "payload": {
                "patient_workspace_id": workspace_id,
                "work_item_id": work_item_id,
                "expected_run_id": expected_run_id,
                "physician_time_zone": time_zone,
            }
        });
        let _: serde_json::Value =
            http::post_json(&self.http, &format!("{}/commands", self.base), &body).await?;
        Ok(())
    }

    /// Reads one patient workspace.
    pub async fn workspace(&self, id: &str) -> Result<PatientWorkspaceResponse, http::HttpError> {
        http::get_json(
            &self.http,
            &format!("{}/patient-workspaces/{id}", self.base),
        )
        .await
    }

    /// Returns the event stream path for this client.
    pub fn events_path(&self) -> String {
        format!("{}/patient-workspaces/events", self.base)
    }
}

fn query_escape(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => out.push(byte as char),
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::routing::get;

    #[tokio::test]
    async fn reads_today_from_the_server_shape() {
        let app = axum::Router::new().route(
            "/today",
            get(|| async {
                Json(serde_json::json!({
                    "date": "2026-10-09",
                    "rows": [],
                    "connections": [],
                    "cursors": {},
                    "as_of": 0,
                    "seen": []
                }))
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = PhysicianClient::new(format!("http://{address}"));
        let today = client.today().await.unwrap();
        assert!(today.rows.is_empty());
        assert_eq!(today.date, "2026-10-09");
    }

    #[tokio::test]
    async fn saves_a_draft_at_the_revision_node_returns() {
        let app = axum::Router::new()
            .route(
                "/physician-state/drafts/{workspace}/{chat}",
                get(|| async {
                    Json(serde_json::json!({
                        "text": "note",
                        "revision": 4,
                        "last_submission_revision": null
                    }))
                })
                .put(|Json(body): Json<serde_json::Value>| async move {
                    assert_eq!(body["expected_revision"], 4);
                    assert_eq!(body["text"], "revised");
                    Json(serde_json::json!({ "text": "revised", "revision": 5 }))
                }),
            )
            .route(
                "/patient-workspaces/{workspace}/chats/{chat}/snapshot",
                get(|| async {
                    Json(serde_json::json!({
                        "workspace": {},
                        "snapshot": {
                            "conversation": [{ "text": "Hello", "authored_text": "" }]
                        }
                    }))
                }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = PhysicianClient::new(format!("http://{address}"));
        let draft = client.read_draft("ws", "chat").await.unwrap();
        assert_eq!(draft.revision, 4);
        client
            .save_draft("ws", "chat", draft.revision, "revised")
            .await
            .unwrap();
        let snapshot = client.chat_snapshot("ws", "chat").await.unwrap();
        assert_eq!(snapshot.snapshot.conversation[0].body(), Some("Hello"));
    }
}
