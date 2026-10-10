//! Physician website caller for the existing Node `CodaServer`.

use physician_wire::{
    HealthResponse, PatientSearchResponse, PatientWorkspaceResponse, PatientWorkspacesResponse,
    PracticeDayResponse, RecentConversationsResponse, TodayListResponse, UpdatesResponse,
};

#[derive(Debug, Clone)]
pub struct PhysicianClient {
    base: String,
    http: reqwest::Client,
}

impl PhysicianClient {
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into().trim_end_matches('/').to_owned(),
            http: reqwest::Client::new(),
        }
    }

    pub async fn health(&self) -> Result<HealthResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/health", self.base)).await
    }

    pub async fn today(&self) -> Result<TodayListResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/today", self.base)).await
    }

    pub async fn workspaces(&self) -> Result<PatientWorkspacesResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/patient-workspaces", self.base)).await
    }

    pub async fn practice_day(&self) -> Result<PracticeDayResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/practice-day", self.base)).await
    }

    pub async fn recent_conversations(
        &self,
    ) -> Result<RecentConversationsResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/recent-conversations", self.base)).await
    }

    pub async fn updates(&self) -> Result<UpdatesResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/updates", self.base)).await
    }

    pub async fn search(&self, query: &str) -> Result<PatientSearchResponse, http::HttpError> {
        http::get_json(
            &self.http,
            &format!("{}/patient-roster?query={}", self.base, query_escape(query)),
        )
        .await
    }

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

    pub async fn workspace(&self, id: &str) -> Result<PatientWorkspaceResponse, http::HttpError> {
        http::get_json(
            &self.http,
            &format!("{}/patient-workspaces/{id}", self.base),
        )
        .await
    }

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
}
