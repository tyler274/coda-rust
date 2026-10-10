//! Duet caller. Models and operations come from the OpenAPI Generator Rust client.
//! This crate adds the app-build headers the FastAPI gate reads.

use duet_wire::apis::configuration::Configuration;
pub use duet_wire::models::AccountMoment;
pub use duet_wire::models::ConversationPage;
pub use duet_wire::models::EvidenceLibraryPage;
pub use duet_wire::models::EvidenceLibraryQuery;
pub use duet_wire::models::InsightCyclePage;
pub use duet_wire::models::IntelligenceSettings;
pub use duet_wire::models::SharingOverviewResponse;
pub use duet_wire::models::SharingPeer;
pub use duet_wire::models::evidence_library_query::Scope as EvidenceLibraryScope;

pub const BUILD_HEADER: &str = "X-Duet-App-Build";
pub const VERSION_HEADER: &str = "X-Duet-App-Version";

#[derive(Debug, Clone)]
pub struct DuetClient {
    configuration: Configuration,
}

impl DuetClient {
    pub fn new(
        base_path: impl Into<String>,
        bearer: impl Into<String>,
        app_build: &str,
        app_version: &str,
    ) -> Result<Self, String> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(BUILD_HEADER, header(app_build)?);
        headers.insert(VERSION_HEADER, header(app_version)?);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|error| error.to_string())?;
        Ok(Self {
            configuration: Configuration {
                base_path: base_path.into().trim_end_matches('/').to_owned(),
                user_agent: Some("coda-duet/0.1.0".to_owned()),
                client,
                basic_auth: None,
                oauth_access_token: None,
                bearer_access_token: Some(bearer.into()),
                api_key: None,
            },
        })
    }

    pub async fn list_moments(&self) -> Result<Vec<AccountMoment>, String> {
        duet_wire::apis::moments_api::list_moments_api_moments_get(&self.configuration)
            .await
            .map_err(|error| error.to_string())
    }

    pub async fn list_messages(&self) -> Result<ConversationPage, String> {
        duet_wire::apis::conversation_api::messages_api_conversation_messages_get(
            &self.configuration,
            None,
            Some(50),
        )
        .await
        .map_err(|error| error.to_string())
    }

    pub async fn list_insight_cycles(&self) -> Result<InsightCyclePage, String> {
        duet_wire::apis::health_intelligence_api::list_insight_cycles_api_insight_cycles_get(
            &self.configuration,
            None,
            Some(20),
        )
        .await
        .map_err(|error| error.to_string())
    }

    pub async fn intelligence_settings(&self) -> Result<IntelligenceSettings, String> {
        duet_wire::apis::health_intelligence_api::get_intelligence_settings_api_intelligence_settings_get(&self.configuration)
            .await
            .map_err(|error| error.to_string())
    }

    pub async fn evidence_library(&self) -> Result<EvidenceLibraryPage, String> {
        let mut query = EvidenceLibraryQuery::new();
        query.scope = Some(EvidenceLibraryScope::Summary);
        duet_wire::apis::default_api::evidence_library_api_evidence_library_post(
            &self.configuration,
            query,
        )
        .await
        .map_err(|error| error.to_string())
    }

    pub async fn sharing_overview(&self) -> Result<SharingOverviewResponse, String> {
        duet_wire::apis::sharing_api::overview_api_sharing_overview_get(&self.configuration)
            .await
            .map_err(|error| error.to_string())
    }
}

fn header(value: &str) -> Result<reqwest::header::HeaderValue, String> {
    reqwest::header::HeaderValue::from_str(value).map_err(|error| error.to_string())
}

/// The shared HTTP crate stays on the Duet side for stream parsing even though
/// the generated client owns the JSON calls.
pub fn parse_sse(buffer: &str) -> Vec<http::SseEvent> {
    http::parse_sse(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::routing::get;
    use duet_wire::models::MomentKind;

    #[tokio::test]
    async fn lists_moments_from_a_json_server() {
        let moment = AccountMoment::new(
            uuid::Uuid::nil(),
            MomentKind::FirstSummaryPrepared,
            chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05Z").unwrap(),
        );
        let body = vec![moment];
        let app = axum::Router::new().route(
            "/api/moments",
            get(move || {
                let body = body.clone();
                async move { Json(body) }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let client =
            DuetClient::new(format!("http://{address}"), "test-token", "200", "1.0.0").unwrap();
        let moments = client.list_moments().await.unwrap();
        assert_eq!(moments.len(), 1);
        assert_eq!(moments[0].kind, MomentKind::FirstSummaryPrepared);
    }

    #[tokio::test]
    async fn reads_the_four_tab_roots() {
        let app = axum::Router::new()
            .route(
                "/api/conversation/messages",
                get(|| async {
                    Json(serde_json::json!({
                        "availability": {"can_send": true},
                        "messages": [],
                        "revision": 1
                    }))
                }),
            )
            .route(
                "/api/insight-cycles",
                get(|| async { Json(serde_json::json!({"items": []})) }),
            )
            .route(
                "/api/intelligence/settings",
                get(|| async { Json(serde_json::json!({"enabled": true})) }),
            )
            .route(
                "/api/evidence/library",
                axum::routing::post(|| async {
                    Json(serde_json::json!({
                        "catalog_revision": null,
                        "counts": {},
                        "entries": [],
                        "health_records_generation": 0,
                        "next_cursor": null,
                        "revision": 1,
                        "total_count": 0
                    }))
                }),
            )
            .route(
                "/api/sharing/overview",
                get(|| async { Json(serde_json::json!({"account_label": "Ada"})) }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client =
            DuetClient::new(format!("http://{address}"), "test-token", "200", "1.0.0").unwrap();
        assert!(client.list_messages().await.unwrap().messages.is_empty());
        assert!(client.list_insight_cycles().await.unwrap().items.is_empty());
        assert!(client.intelligence_settings().await.unwrap().enabled);
        assert_eq!(client.evidence_library().await.unwrap().total_count, 0);
        assert_eq!(
            client.sharing_overview().await.unwrap().account_label,
            "Ada"
        );
    }
}
