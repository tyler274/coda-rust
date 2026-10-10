//! Physician website caller for the existing Node `CodaServer`.

use physician_wire::{HealthResponse, PatientWorkspaceResponse, PatientWorkspacesResponse, TodayListResponse};

#[derive(Debug, Clone)]
pub struct PhysicianClient {
    base: String,
    http: reqwest::Client,
}

impl PhysicianClient {
    pub fn new(base: impl Into<String>) -> Self {
        Self { base: base.into().trim_end_matches('/').to_owned(), http: reqwest::Client::new() }
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

    pub async fn workspace(&self, id: &str) -> Result<PatientWorkspaceResponse, http::HttpError> {
        http::get_json(&self.http, &format!("{}/patient-workspaces/{id}", self.base)).await
    }

    pub fn events_path(&self) -> String {
        format!("{}/patient-workspaces/events", self.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::routing::get;

    #[tokio::test]
    async fn reads_today_from_the_server_shape() {
        let app = axum::Router::new().route("/today", get(|| async {
            Json(serde_json::json!({
                "date": "2026-10-09",
                "rows": [],
                "connections": [],
                "cursors": {},
                "as_of": 0,
                "seen": []
            }))
        }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = PhysicianClient::new(format!("http://{address}"));
        let today = client.today().await.unwrap();
        assert!(today.rows.is_empty());
        assert_eq!(today.date, "2026-10-09");
    }
}
