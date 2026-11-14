//! Duet caller. Models and operations come from the OpenAPI Generator Rust client.
//! This crate adds the app-build headers the FastAPI gate reads.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

#[cfg(not(target_arch = "wasm32"))]
pub mod apple_health;
#[cfg(target_os = "android")]
pub use duet_health::request_health_connect_permission;
pub mod doctor;
pub mod health_upload;
pub mod identity;
pub mod retention;
pub mod session;

pub mod refusal;

use duet_wire::apis::configuration::Configuration;
pub use duet_wire::models::AccountMoment;
pub use duet_wire::models::EvidenceLibraryPage;
pub use duet_wire::models::EvidenceLibraryQuery;
pub use duet_wire::models::evidence_library_query::Scope as EvidenceLibraryScope;

/// The `BUILD_HEADER` constant.
pub const BUILD_HEADER: &str = "X-Duet-App-Build";
/// The `VERSION_HEADER` constant.
pub const VERSION_HEADER: &str = "X-Duet-App-Version";

#[derive(Clone)]
/// The `DuetClient` value exchanged by this crate. Debug output deliberately omits credentials.
pub struct DuetClient {
    configuration: Configuration,
    refusal: Option<refusal::RefusalBinding>,
}

impl std::fmt::Debug for DuetClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DuetClient { credentials: [REDACTED] }")
    }
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
            refusal: None,
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
            .map_err(|error| self.wire_error(error))
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
        .map_err(|error| self.wire_error(error))
    }

    /// Reads one bounded page of the complete exact record inventory. Pass the previous
    /// page's `next_cursor` until it is `None`; a conflict means evidence changed and
    /// browsing restarts from the first page.
    pub async fn browse_records(
        &self,
        cursor: Option<String>,
    ) -> Result<duet_wire::models::EvidenceBrowsePage, String> {
        let mut query = duet_wire::models::EvidenceBrowseQuery::new();
        query.cursor = cursor;
        query.include_removed = Some(true);
        duet_wire::apis::default_api::browse_evidence_records(&self.configuration, query)
            .await
            .map_err(|error| self.wire_error(error))
    }

    /// Reads every page of the record inventory, restarting once if evidence changed
    /// mid-walk. Never returns a truncated list.
    pub async fn all_records(
        &self,
    ) -> Result<Vec<duet_wire::models::EvidenceBrowseRecord>, String> {
        let mut last = String::new();
        for _ in 0..2 {
            let mut records = Vec::new();
            let mut cursor = None;
            loop {
                match self.browse_records(cursor.take()).await {
                    Ok(page) => {
                        records.extend(page.records);
                        match page.next_cursor {
                            Some(next) => cursor = Some(next),
                            None => return Ok(records),
                        }
                    }
                    Err(error) => {
                        last = error;
                        break;
                    }
                }
            }
        }
        Err(last)
    }

    /// Reads one exact record revision with its truthful status.
    pub async fn record_revision(
        &self,
        address: duet_wire::models::CanonicalSourceAddress,
    ) -> Result<duet_wire::models::EvidenceRevisionView, String> {
        duet_wire::apis::default_api::read_evidence_revision(
            &self.configuration,
            duet_wire::models::EvidenceRevisionRequest::new(address),
        )
        .await
        .map_err(|error| self.wire_error(error))
    }

    /// Replaces the visible title of an existing library row.
    ///
    /// The correction is a new evidence upsert whose payload names the row it
    /// corrects. A second plain upsert of the same id stays the first write.
    pub async fn correct_evidence(
        &self,
        target_id: &str,
        entity: duet_wire::models::evidence_item::Entity,
        title: &str,
    ) -> Result<(), String> {
        let mut payload = std::collections::HashMap::new();
        payload.insert(
            "corrects".to_owned(),
            serde_json::Value::String(target_id.to_owned()),
        );
        payload.insert(
            "source_label".to_owned(),
            serde_json::Value::String("Added in Duet".to_owned()),
        );
        match entity {
            duet_wire::models::evidence_item::Entity::HealthEvidence => {
                payload.insert(
                    "display_name".to_owned(),
                    serde_json::Value::String(title.to_owned()),
                );
                payload.insert(
                    "concept_id".to_owned(),
                    serde_json::Value::String("medication:manual".to_owned()),
                );
            }
            duet_wire::models::evidence_item::Entity::ClinicalNote => {
                payload.insert(
                    "title".to_owned(),
                    serde_json::Value::String(title.to_owned()),
                );
            }
        }
        self.store_evidence(uuid::Uuid::new_v4(), entity, payload)
            .await
    }

    /// Removes one library row. The id is the library entry id.
    pub async fn delete_evidence(
        &self,
        id: &str,
        entity: duet_wire::models::evidence_item::Entity,
    ) -> Result<(), String> {
        let mut item = duet_wire::models::EvidenceItem::new(entity, id.to_owned());
        item.operation = Some(duet_wire::models::evidence_item::Operation::Delete);
        let mut request = duet_wire::models::EvidenceBatchRequest::new();
        request.items = Some(vec![item]);
        duet_wire::apis::default_api::ingest_evidence_api_evidence_post(
            &self.configuration,
            request,
        )
        .await
        .map(|_| ())
        .map_err(|error| self.wire_error(error))
    }

    /// Reads the For my doctor collection.
    pub async fn doctor_collection(&self) -> Result<duet_wire::models::DoctorCollection, String> {
        self.doctor_workspace().await.map_err(|e| e.to_string())
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
    pub async fn checked_doctor_page(
        &self,
        expected_generation: i32,
    ) -> Result<duet_wire::models::DoctorCollection, String> {
        self.check_doctor_workspace(expected_generation)
            .await
            .map_err(|e| e.to_string())
    }

    /// Save only the explicit patient-approved content with its original generation fence.
    pub async fn save_doctor_page(
        &self,
        save: duet_wire::models::DoctorPageSave,
    ) -> Result<duet_wire::models::DoctorCollection, String> {
        self.save_doctor_workspace(&save)
            .await
            .map_err(|e| e.to_string())
    }

    /// Clears the approved page using the same privacy-event identity and generation on retry.
    pub async fn clear_doctor_page(
        &self,
        operation_id: uuid::Uuid,
        generation: i64,
    ) -> Result<(), String> {
        self.clear_doctor_workspace(operation_id, generation)
            .await
            .map_err(|e| e.to_string())
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
        .map_err(|error| self.wire_error(error))
    }

    /// Redeems the practice's invitation through the front door. The minted
    /// session becomes this client's bearer.
    pub async fn join(&self, email: &str, token: &str) -> Result<FrontDoorSession, FrontDoorError> {
        // Join stays open: the invitation token is the proof, so no bearer is
        // attached.
        let request = self
            .configuration
            .client
            .post(format!("{}/api/join", self.configuration.base_path))
            .json(&serde_json::json!({ "email": email, "token": token }))
            .build()
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        let response = self
            .configuration
            .client
            .execute(request)
            .await
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        if !status.is_success() {
            return Err(front_door_error(&body, status));
        }
        serde_json::from_str::<FrontDoorSession>(&body)
            .map_err(|error| FrontDoorError::Other(error.to_string()))
    }

    /// The same client presenting one session token as its bearer: the app
    /// rebuilds its client after a join with the minted session.
    pub fn with_bearer(&self, session: impl Into<String>) -> Self {
        let mut configuration = self.configuration.clone();
        configuration.bearer_access_token = Some(session.into());
        Self {
            configuration,
            refusal: None,
        }
    }

    /// Reads the account's consent state: sharing with the physician.
    pub async fn consent(&self) -> Result<ConsentState, FrontDoorError> {
        self.front_door_get("/api/consent").await
    }

    /// Grants consent: sharing with the physician is on.
    pub async fn grant_consent(&self) -> Result<(), FrontDoorError> {
        self.front_door_send(reqwest::Method::POST, "/api/consent")
            .await
    }

    /// Withdraws consent: one step, and what was already shared stays dated
    /// evidence.
    pub async fn withdraw_consent(&self) -> Result<(), FrontDoorError> {
        self.front_door_send(reqwest::Method::DELETE, "/api/consent")
            .await
    }

    /// Reads which premium features are on. Duet reads nothing about billing.
    pub async fn premium(&self) -> Result<PremiumState, FrontDoorError> {
        self.front_door_get("/api/premium").await
    }

    async fn front_door_get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, FrontDoorError> {
        let mut request = self
            .configuration
            .client
            .get(format!("{}{path}", self.configuration.base_path))
            .build()
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        if let Some(token) = self.configuration.bearer_access_token.clone() {
            request.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
                    .map_err(|error| FrontDoorError::Other(error.to_string()))?,
            );
        }
        self.front_door_body(request).await
    }

    async fn front_door_send(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> Result<(), FrontDoorError> {
        let mut request = self
            .configuration
            .client
            .request(method, format!("{}{path}", self.configuration.base_path))
            .build()
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        if let Some(token) = self.configuration.bearer_access_token.clone() {
            request.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
                    .map_err(|error| FrontDoorError::Other(error.to_string()))?,
            );
        }
        let response = self
            .configuration
            .client
            .execute(request)
            .await
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(front_door_error(&body, status));
        }
        Ok(())
    }

    async fn front_door_body<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::Request,
    ) -> Result<T, FrontDoorError> {
        let response = self
            .configuration
            .client
            .execute(request)
            .await
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|error| FrontDoorError::Other(error.to_string()))?;
        if !status.is_success() {
            return Err(front_door_error(&body, status));
        }
        serde_json::from_str::<T>(&body).map_err(|error| FrontDoorError::Other(error.to_string()))
    }
}

/// The session the front door mints on join: the account and the bearer token
/// Duet then presents.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct FrontDoorSession {
    /// The front-door account id the session acts for.
    pub account_id: uuid::Uuid,
    /// The bearer token.
    pub session: String,
}

/// One account's consent state, as the front door reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ConsentState {
    /// Sharing is on.
    Granted {
        /// The Unix second consent was given.
        at: u64,
    },
    /// Consent was withdrawn; what was already shared stays dated evidence.
    Withdrawn {
        /// The Unix second consent was given.
        at: u64,
        /// The Unix second consent was withdrawn.
        withdrawn_at: u64,
    },
    /// The patient was never asked, or never answered.
    None,
}

impl ConsentState {
    /// Whether sharing is on right now.
    pub fn sharing(&self) -> bool {
        matches!(self, Self::Granted { .. })
    }
}

/// Which premium features are on, as the front door reports it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct PremiumState {
    /// Whether premium features are on.
    pub premium: bool,
}

/// Why a front-door call failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontDoorError {
    /// The presented session no longer resolves: the person joins again with
    /// a fresh invitation.
    Unauthorized,
    /// The invitation or token is unknown, used, or expired.
    Invalid,
    /// The front door could not answer; try again.
    Unavailable,
    /// Anything else; the detail sentence says what went wrong.
    Other(String),
}

impl FrontDoorError {
    /// The sentence the app shows.
    pub fn message(&self) -> String {
        match self {
            Self::Unauthorized => "Sign in again with your practice invitation.".to_owned(),
            Self::Invalid => "That invitation isn't valid, or it has already been used.".to_owned(),
            Self::Unavailable => {
                "Duet couldn't reach your practice just now. Try again.".to_owned()
            }
            Self::Other(detail) => detail.clone(),
        }
    }
}

fn front_door_error(body: &str, status: reqwest::StatusCode) -> FrontDoorError {
    let detail = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|value| value["detail"].as_str().map(str::to_owned));
    match status {
        reqwest::StatusCode::UNAUTHORIZED
            if detail.as_deref().is_some_and(|d| d.contains("invitation")) =>
        {
            FrontDoorError::Invalid
        }
        reqwest::StatusCode::UNAUTHORIZED => FrontDoorError::Unauthorized,
        reqwest::StatusCode::SERVICE_UNAVAILABLE => FrontDoorError::Unavailable,
        _ => FrontDoorError::Other(
            detail.unwrap_or_else(|| "Something went wrong. Try again.".to_owned()),
        ),
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
    async fn reads_the_library() {
        let app = axum::Router::new().route(
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
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client =
            DuetClient::new(format!("http://{address}"), "test-token", "200", "1.0.0").unwrap();
        assert_eq!(client.evidence_library().await.unwrap().total_count, 0);
    }

    #[tokio::test]
    async fn join_consent_and_premium_follow_the_front_door() {
        let state = std::sync::Arc::new(std::sync::Mutex::new(FrontDoorFixture::default()));
        let join_state = state.clone();
        let read_state = state.clone();
        let grant_state = state.clone();
        let withdraw_state = state.clone();
        let premium_state = state.clone();
        let app = axum::Router::new()
            .route(
                "/api/join",
                axum::routing::post(move |Json(body): Json<serde_json::Value>| {
                    let state = join_state.clone();
                    async move {
                        if body["token"] != "codaf-invite" || state.lock().unwrap().redeemed {
                            return (
                                axum::http::StatusCode::UNAUTHORIZED,
                                Json(serde_json::json!({"detail": "used or expired"})),
                            );
                        }
                        state.lock().unwrap().redeemed = true;
                        state
                            .lock()
                            .unwrap()
                            .sessions
                            .insert("codaf-session".to_owned());
                        (
                            axum::http::StatusCode::OK,
                            Json(serde_json::json!({
                                "account_id": "00000000-0000-0000-0000-000000000007",
                                "session": "codaf-session",
                            })),
                        )
                    }
                }),
            )
            .route(
                "/api/consent",
                get(move |headers: axum::http::HeaderMap| {
                    let state = read_state.clone();
                    async move { consent_reply(&state, &headers) }
                })
                .post(move |headers: axum::http::HeaderMap| {
                    let state = grant_state.clone();
                    async move { consent_set(&state, &headers, true) }
                })
                .delete(move |headers: axum::http::HeaderMap| {
                    let state = withdraw_state.clone();
                    async move { consent_set(&state, &headers, false) }
                }),
            )
            .route(
                "/api/premium",
                get(move |headers: axum::http::HeaderMap| {
                    let state = premium_state.clone();
                    async move {
                        match session_of(&state, &headers) {
                            None => (
                                axum::http::StatusCode::UNAUTHORIZED,
                                Json(serde_json::json!({"detail": "Sign in."})),
                            ),
                            Some(()) => (
                                axum::http::StatusCode::OK,
                                Json(serde_json::json!({
                                    "premium": false,
                                })),
                            ),
                        }
                    }
                }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        // Before joining, the client's bearer is unknown to the front door.
        let stranger =
            DuetClient::new(format!("http://{address}"), "codaf-other", "200", "1.0.0").unwrap();
        assert!(stranger.consent().await.is_err());
        let client = DuetClient::new(format!("http://{address}"), "stale", "200", "1.0.0").unwrap();
        let joined = client
            .join("luna@example.test", "codaf-invite")
            .await
            .unwrap();
        assert_eq!(
            joined.account_id,
            uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000007").unwrap()
        );
        assert_eq!(joined.session, "codaf-session");
        // The app rebuilds its client around the minted session.
        let client = client.with_bearer(joined.session.clone());
        assert_eq!(
            client.consent().await.unwrap(),
            ConsentState::None,
            "consent is granted, never assumed"
        );
        client.grant_consent().await.unwrap();
        assert!(client.consent().await.unwrap().sharing());
        client.withdraw_consent().await.unwrap();
        let after = client.consent().await.unwrap();
        assert!(!after.sharing(), "stopping sharing is one step");
        assert!(
            matches!(
                after,
                ConsentState::Withdrawn {
                    at: _,
                    withdrawn_at: _
                }
            ),
            "what was already shared stays dated evidence"
        );
        let premium = client.premium().await.unwrap();
        assert!(!premium.premium);
        // A used invitation never joins again.
        assert!(
            client
                .join("luna@example.test", "codaf-invite")
                .await
                .is_err()
        );
        let stranger =
            DuetClient::new(format!("http://{address}"), "codaf-other", "200", "1.0.0").unwrap();
        assert!(
            stranger
                .join("luna@example.test", "codaf-invite")
                .await
                .is_err()
        );
    }

    #[derive(Default)]
    struct FrontDoorFixture {
        redeemed: bool,
        sessions: std::collections::HashSet<String>,
        granted: Option<u64>,
        withdrawn: Option<u64>,
    }

    fn bearer_of(headers: &axum::http::HeaderMap) -> Option<String> {
        headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(str::to_owned)
    }

    fn session_of(
        state: &std::sync::Mutex<FrontDoorFixture>,
        headers: &axum::http::HeaderMap,
    ) -> Option<()> {
        let token = bearer_of(headers)?;
        state
            .lock()
            .unwrap()
            .sessions
            .contains(&token)
            .then_some(())
    }

    fn consent_reply(
        state: &std::sync::Mutex<FrontDoorFixture>,
        headers: &axum::http::HeaderMap,
    ) -> (axum::http::StatusCode, Json<serde_json::Value>) {
        if session_of(state, headers).is_none() {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"detail": "Sign in."})),
            );
        }
        let state = state.lock().unwrap();
        let body = match (state.granted, state.withdrawn) {
            (Some(at), Some(withdrawn_at)) => serde_json::json!({
                "state": "withdrawn", "at": at, "withdrawn_at": withdrawn_at,
            }),
            (Some(at), None) => serde_json::json!({ "state": "granted", "at": at }),
            _ => serde_json::json!({ "state": "none" }),
        };
        (axum::http::StatusCode::OK, Json(body))
    }

    fn consent_set(
        state: &std::sync::Mutex<FrontDoorFixture>,
        headers: &axum::http::HeaderMap,
        grant: bool,
    ) -> (axum::http::StatusCode, Json<serde_json::Value>) {
        if session_of(state, headers).is_none() {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"detail": "Sign in."})),
            );
        }
        let mut state = state.lock().unwrap();
        if grant {
            state.granted = Some(1_760_000_001);
            state.withdrawn = None;
        } else {
            state.withdrawn = Some(1_760_000_002);
        }
        (
            axum::http::StatusCode::NO_CONTENT,
            Json(serde_json::json!({})),
        )
    }
}
