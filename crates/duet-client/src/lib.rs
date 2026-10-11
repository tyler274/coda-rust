//! Duet caller. Models and operations come from the OpenAPI Generator Rust client.
//! This crate adds the app-build headers the FastAPI gate reads.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]

use duet_wire::apis::configuration::Configuration;
pub use duet_wire::models::AccountMoment;
pub use duet_wire::models::ConversationAcceptance;
pub use duet_wire::models::ConversationPage;
pub use duet_wire::models::EvidenceLibraryPage;
pub use duet_wire::models::EvidenceLibraryQuery;
pub use duet_wire::models::InsightCyclePage;
pub use duet_wire::models::IntelligenceSettings;
pub use duet_wire::models::SharingOverviewResponse;
pub use duet_wire::models::SharingPeer;
pub use duet_wire::models::evidence_library_query::Scope as EvidenceLibraryScope;

/// The `BUILD_HEADER` constant.
pub const BUILD_HEADER: &str = "X-Duet-App-Build";
/// The `VERSION_HEADER` constant.
pub const VERSION_HEADER: &str = "X-Duet-App-Version";

#[derive(Debug, Clone)]
/// The `DuetClient` value exchanged by this crate.
pub struct DuetClient {
    configuration: Configuration,
}

impl DuetClient {
    /// Creates the client.
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

    /// Lists account moments.
    pub async fn list_moments(&self) -> Result<Vec<AccountMoment>, String> {
        duet_wire::apis::moments_api::list_moments_api_moments_get(&self.configuration)
            .await
            .map_err(|error| error.to_string())
    }

    /// Lists the conversation page.
    pub async fn list_messages(&self) -> Result<ConversationPage, String> {
        duet_wire::apis::conversation_api::messages_api_conversation_messages_get(
            &self.configuration,
            None,
            Some(50),
        )
        .await
        .map_err(|error| error.to_string())
    }

    /// Sends one message. `id` is the message identity: a retry of an uncertain
    /// send passes the same id so the server can reconcile it.
    pub async fn send_message(
        &self,
        id: uuid::Uuid,
        text: &str,
        timezone: &str,
    ) -> Result<ConversationAcceptance, String> {
        let request = duet_wire::models::ConversationSendRequest::new(
            id,
            text.to_owned(),
            timezone.to_owned(),
        );
        duet_wire::apis::conversation_api::send_message_api_conversation_messages_post(
            &self.configuration,
            request,
        )
        .await
        .map_err(|error| error.to_string())
    }

    /// Lists insight cycles for Today.
    pub async fn list_insight_cycles(&self) -> Result<InsightCyclePage, String> {
        duet_wire::apis::health_intelligence_api::list_insight_cycles_api_insight_cycles_get(
            &self.configuration,
            None,
            Some(20),
        )
        .await
        .map_err(|error| error.to_string())
    }

    /// Reads intelligence settings.
    pub async fn intelligence_settings(&self) -> Result<IntelligenceSettings, String> {
        duet_wire::apis::health_intelligence_api::get_intelligence_settings_api_intelligence_settings_get(&self.configuration)
            .await
            .map_err(|error| error.to_string())
    }

    /// Reads the evidence library.
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

    /// Lists saved Plans.
    pub async fn list_saved_routines(&self) -> Result<duet_wire::models::SavedRoutineList, String> {
        duet_wire::apis::saved_routines_api::list_saved_routines_api_saved_routines_get(
            &self.configuration,
        )
        .await
        .map_err(|error| error.to_string())
    }

    /// Lists reminders and check-ins.
    pub async fn list_follow_ups(&self) -> Result<duet_wire::models::FollowUpPage, String> {
        duet_wire::apis::follow_ups_api::list_follow_ups_api_follow_ups_get(
            &self.configuration,
            None,
            Some(50),
        )
        .await
        .map_err(|error| error.to_string())
    }

    /// Reads the For my doctor collection.
    pub async fn doctor_collection(&self) -> Result<duet_wire::models::DoctorCollection, String> {
        duet_wire::apis::doctor_workspace_api::get_doctor_collection_api_doctor_collection_get(
            &self.configuration,
        )
        .await
        .map_err(|error| error.to_string())
    }

    /// Saves one medication the person typed. `id` is reused when the save fails.
    pub async fn add_medication(&self, id: uuid::Uuid, name: &str) -> Result<(), String> {
        let mut payload = std::collections::HashMap::new();
        payload.insert(
            "concept_id".to_owned(),
            serde_json::Value::String("medication:manual".to_owned()),
        );
        payload.insert(
            "display_name".to_owned(),
            serde_json::Value::String(name.to_owned()),
        );
        payload.insert(
            "source_label".to_owned(),
            serde_json::Value::String("Added in Duet".to_owned()),
        );
        self.store_evidence(
            id,
            duet_wire::models::evidence_item::Entity::HealthEvidence,
            payload,
        )
        .await
    }

    /// Saves one note the person typed. `id` is reused when the save fails.
    pub async fn add_note(&self, id: uuid::Uuid, title: &str, body: &str) -> Result<(), String> {
        let mut payload = std::collections::HashMap::new();
        payload.insert(
            "title".to_owned(),
            serde_json::Value::String(title.to_owned()),
        );
        payload.insert(
            "body".to_owned(),
            serde_json::Value::String(body.to_owned()),
        );
        payload.insert(
            "source_label".to_owned(),
            serde_json::Value::String("Added in Duet".to_owned()),
        );
        self.store_evidence(
            id,
            duet_wire::models::evidence_item::Entity::ClinicalNote,
            payload,
        )
        .await
    }

    /// Saves the For my doctor page the person is reviewing.
    pub async fn save_doctor_page(
        &self,
        save: duet_wire::models::DoctorPageSave,
    ) -> Result<duet_wire::models::DoctorCollection, String> {
        duet_wire::apis::doctor_workspace_api::save_doctor_page_api_doctor_collection_page_put(
            &self.configuration,
            save,
        )
        .await
        .map_err(|error| error.to_string())
    }

    async fn store_evidence(
        &self,
        id: uuid::Uuid,
        entity: duet_wire::models::evidence_item::Entity,
        payload: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<(), String> {
        let mut item = duet_wire::models::EvidenceItem::new(entity, id.to_string());
        item.operation = Some(duet_wire::models::evidence_item::Operation::Upsert);
        item.payload = Some(payload);
        let mut request = duet_wire::models::EvidenceBatchRequest::new();
        request.items = Some(vec![item]);
        duet_wire::apis::default_api::ingest_evidence_api_evidence_post(
            &self.configuration,
            request,
        )
        .await
        .map(|_| ())
        .map_err(|error| error.to_string())
    }

    /// Reads who sharing includes.
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

    #[tokio::test]
    async fn send_message_posts_the_callers_id() {
        let posted = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let recorded = posted.clone();
        let app = axum::Router::new().route(
            "/api/conversation/messages",
            axum::routing::post(move |Json(body): Json<serde_json::Value>| {
                let recorded = recorded.clone();
                async move {
                    let id = body["id"].as_str().unwrap_or_default().to_owned();
                    let text = body["text"].as_str().unwrap_or_default().to_owned();
                    recorded.lock().unwrap().push(id.clone());
                    if text == "fail" {
                        return (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({"detail": "unavailable"})),
                        );
                    }
                    (
                        axum::http::StatusCode::ACCEPTED,
                        Json(serde_json::json!({
                            "message": {
                                "created_at": "2026-10-09T15:00:00Z",
                                "id": id,
                                "revision": 1,
                                "role": "user",
                                "sequence": 1,
                                "state": "complete",
                                "text": text,
                                "updated_at": "2026-10-09T15:00:00Z"
                            },
                            "revision": 2,
                            "run": {
                                "assistant_message_id": null,
                                "created_at": "2026-10-09T15:00:00Z",
                                "id": "00000000-0000-0000-0000-000000000099",
                                "message_id": null,
                                "revision": 1,
                                "state": "queued",
                                "updated_at": "2026-10-09T15:00:00Z"
                            }
                        })),
                    )
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client =
            DuetClient::new(format!("http://{address}"), "test-token", "200", "1.0.0").unwrap();
        let id = uuid::Uuid::nil();
        let accepted = client
            .send_message(id, "How did I sleep?", "America/Los_Angeles")
            .await
            .unwrap();
        assert_eq!(accepted.message.id, id);
        assert_eq!(accepted.message.text, "How did I sleep?");
        let retry = client.send_message(id, "fail", "America/Los_Angeles").await;
        assert!(retry.is_err());
        assert_eq!(
            posted.lock().unwrap().as_slice(),
            [
                "00000000-0000-0000-0000-000000000000",
                "00000000-0000-0000-0000-000000000000"
            ]
        );
    }
}
