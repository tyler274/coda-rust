//! Physician website caller for the existing Node `CodaServer`.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]

pub mod briefings;

pub use http::HttpError;
use physician_wire::{
    ChatSnapshotResponse, HealthResponse, PanelAssignmentResponse, PastWorkResponse,
    PatientSearchResponse, PatientWorkspaceResponse, PatientWorkspacesResponse, PhysicianDraft,
    PracticeDayResponse, RecentConversationsResponse, ReplyDraftWire, ResponseFeedback, ResponseId,
    SaveFeedback, SignInResponse, TodayListResponse, UpdatesResponse, VerifyResponse,
    WhoamiResponse,
};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
/// The bearer session the client presents on every physician read.
pub struct Session(Arc<std::sync::RwLock<Option<String>>>);

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl Session {
    /// Creates an empty session: signed out.
    pub fn new() -> Self {
        Self(Arc::new(std::sync::RwLock::new(None)))
    }

    /// Stores the bearer session from `/auth/verify`.
    pub fn set(&self, session: String) {
        *self.0.write().expect("session lock") = Some(session);
    }

    /// The held bearer session, when signed in.
    pub fn get(&self) -> Option<String> {
        self.0.read().expect("session lock").clone()
    }

    /// Ends the held session locally; the server has already ended it.
    pub fn clear(&self) {
        *self.0.write().expect("session lock") = None;
    }

    /// Ends `session` locally if it is still the one held; a newer session
    /// set meanwhile stays.
    pub fn clear_if(&self, session: &str) {
        let mut held = self.0.write().expect("session lock");
        if held.as_deref() == Some(session) {
            *held = None;
        }
    }

    /// Whether a session is held.
    pub fn is_some(&self) -> bool {
        self.0.read().expect("session lock").is_some()
    }
}

#[derive(Debug, Clone, Default)]
/// The `PhysicianClient` value exchanged by this crate.
pub struct PhysicianClient {
    base: String,
    http: reqwest::Client,
    session: Session,
}

impl PartialEq for PhysicianClient {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.session == other.session
    }
}

impl PhysicianClient {
    /// Creates the client, signed out.
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into().trim_end_matches('/').to_owned(),
            http: reqwest::Client::new(),
            session: Session::new(),
        }
    }

    /// The client's bearer session, shared by every method.
    pub fn session(&self) -> &Session {
        &self.session
    }

    fn bearer(&self) -> Option<String> {
        self.session
            .get()
            .map(|session| format!("Bearer {session}"))
    }

    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, http::HttpError> {
        let mut request = self.http.get(format!("{}{path}", self.base));
        if let Some(bearer) = self.bearer() {
            request = request.header(reqwest::header::AUTHORIZATION, bearer);
        }
        let response = request
            .send()
            .await
            .map_err(|error| http::HttpError::Transport(error.to_string()))?;
        read_body(response).await
    }

    pub(crate) async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, http::HttpError> {
        self.post_as(self.bearer(), path, body).await
    }

    async fn post_as<T: serde::de::DeserializeOwned>(
        &self,
        bearer: Option<String>,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, http::HttpError> {
        let mut request = self.http.post(format!("{}{path}", self.base)).json(body);
        if let Some(bearer) = bearer {
            request = request.header(reqwest::header::AUTHORIZATION, bearer);
        }
        let response = request
            .send()
            .await
            .map_err(|error| http::HttpError::Transport(error.to_string()))?;
        read_body(response).await
    }

    async fn put<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, http::HttpError> {
        let mut request = self.http.put(format!("{}{path}", self.base)).json(body);
        if let Some(bearer) = self.bearer() {
            request = request.header(reqwest::header::AUTHORIZATION, bearer);
        }
        let response = request
            .send()
            .await
            .map_err(|error| http::HttpError::Transport(error.to_string()))?;
        read_body(response).await
    }

    async fn delete<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, http::HttpError> {
        let mut request = self.http.delete(format!("{}{path}", self.base));
        if let Some(bearer) = self.bearer() {
            request = request.header(reqwest::header::AUTHORIZATION, bearer);
        }
        let response = request
            .send()
            .await
            .map_err(|error| http::HttpError::Transport(error.to_string()))?;
        read_body(response).await
    }

    /// Reads the physician health check.
    pub async fn health(&self) -> Result<HealthResponse, http::HttpError> {
        self.get("/health").await
    }

    /// Requests a single-use sign-in token for the email's account.
    pub async fn begin_sign_in(&self, email: &str) -> Result<SignInResponse, http::HttpError> {
        self.post(
            "/auth/sign-in",
            &serde_json::json!({ "email": email.trim() }),
        )
        .await
    }

    /// Verifies a sign-in token and holds the returned bearer session.
    pub async fn verify(
        &self,
        email: &str,
        token: &str,
    ) -> Result<VerifyResponse, http::HttpError> {
        let verified: VerifyResponse = self
            .post(
                "/auth/verify",
                &serde_json::json!({ "email": email.trim(), "token": token.trim() }),
            )
            .await?;
        self.session.set(verified.session.clone());
        Ok(verified)
    }

    /// Ends `session` on the server, and drops it locally if it is still the
    /// one held; a newer session set meanwhile stays.
    pub async fn sign_out(&self, session: &str) -> Result<(), http::HttpError> {
        let result: Result<serde_json::Value, http::HttpError> = self
            .post_as(
                Some(format!("Bearer {session}")),
                "/auth/sign-out",
                &serde_json::json!({}),
            )
            .await;
        self.session.clear_if(session);
        result.map(|_| ())
    }

    /// Reads the session's physician when the held session still resolves.
    pub async fn whoami(&self) -> Result<WhoamiResponse, http::HttpError> {
        self.get("/auth/whoami").await
    }

    /// Assigns or removes `patient_workspace_id` on `physician_id`'s panel.
    /// Requires the signed-in account to hold clinic admin / panel-assignment
    /// permission (`admin: true`).
    pub async fn assign_panel(
        &self,
        physician_id: &str,
        patient_workspace_id: &str,
        allowed: bool,
    ) -> Result<PanelAssignmentResponse, http::HttpError> {
        if !allowed {
            return self.remove_panel(physician_id, patient_workspace_id).await;
        }
        self.put(
            &format!(
                "/auth/panels/{}/{}",
                query_escape(physician_id),
                query_escape(patient_workspace_id)
            ),
            &serde_json::json!({ "allowed": true }),
        )
        .await
    }

    /// Removes `patient_workspace_id` from `physician_id`'s panel via `DELETE`.
    pub async fn remove_panel(
        &self,
        physician_id: &str,
        patient_workspace_id: &str,
    ) -> Result<PanelAssignmentResponse, http::HttpError> {
        self.delete(&format!(
            "/auth/panels/{}/{}",
            query_escape(physician_id),
            query_escape(patient_workspace_id)
        ))
        .await
    }

    /// Reads Today.
    pub async fn today(&self) -> Result<TodayListResponse, http::HttpError> {
        self.get("/today").await
    }

    /// Reads the patient workspaces.
    pub async fn workspaces(&self) -> Result<PatientWorkspacesResponse, http::HttpError> {
        self.get("/patient-workspaces").await
    }

    /// Reads the practice day.
    pub async fn practice_day(&self) -> Result<PracticeDayResponse, http::HttpError> {
        self.get("/practice-day").await
    }

    /// Reads recent conversations.
    pub async fn recent_conversations(
        &self,
    ) -> Result<RecentConversationsResponse, http::HttpError> {
        self.get("/recent-conversations").await
    }

    /// Reads Updates.
    pub async fn updates(&self) -> Result<UpdatesResponse, http::HttpError> {
        self.get("/updates").await
    }

    /// Reads Today for one patient.
    pub async fn today_for(
        &self,
        workspace_id: &str,
    ) -> Result<TodayListResponse, http::HttpError> {
        self.get(&format!(
            "/today?patient_workspace_id={}",
            query_escape(workspace_id)
        ))
        .await
    }

    /// Sends a physician command.
    pub async fn command(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, http::HttpError> {
        self.post("/commands", body).await
    }

    /// Reads one chat snapshot.
    pub async fn chat_snapshot(
        &self,
        workspace_id: &str,
        chat_id: &str,
    ) -> Result<ChatSnapshotResponse, http::HttpError> {
        self.get(&format!(
            "/patient-workspaces/{workspace_id}/chats/{chat_id}/snapshot"
        ))
        .await
    }

    /// Reads the saved draft.
    pub async fn read_draft(
        &self,
        workspace_id: &str,
        chat_id: &str,
    ) -> Result<PhysicianDraft, http::HttpError> {
        self.get(&format!("/physician-state/drafts/{workspace_id}/{chat_id}"))
            .await
    }

    /// Saves the draft over the revision it was read or last saved at.
    /// A stale revision answers 409 and writes nothing; only a validated
    /// `{"outcome": "conflict", "current": {revision, text}}` body becomes
    /// [`DraftSave::Conflict`]. A bare or malformed 409 stays an error and
    /// never names a revision to overwrite.
    pub async fn save_draft(
        &self,
        workspace_id: &str,
        chat_id: &str,
        revision: i64,
        text: &str,
    ) -> Result<DraftSave, http::HttpError> {
        let answer = self
            .put::<PhysicianDraft>(
                &format!("/physician-state/drafts/{workspace_id}/{chat_id}"),
                &serde_json::json!({ "expected_revision": revision, "text": text }),
            )
            .await;
        match answer {
            Ok(saved) => Ok(DraftSave::Saved(saved)),
            Err(http::HttpError::Status { status: 409, body }) => match draft_conflict(&body) {
                Some(current) => Ok(DraftSave::Conflict(current)),
                None => Err(http::HttpError::Status { status: 409, body }),
            },
            Err(error) => Err(error),
        }
    }

    /// Saves feedback on the response named `response` in a chat, replacing
    /// its rating and details; answers the saved feedback with its revision.
    /// The chat snapshot carries saved feedback on each response.
    pub async fn save_feedback(
        &self,
        workspace_id: &str,
        chat_id: &str,
        response: &ResponseId,
        feedback: &SaveFeedback,
    ) -> Result<ResponseFeedback, http::HttpError> {
        self.put(
            &format!(
                "/physician-state/feedback/{workspace_id}/{chat_id}/{}",
                response.as_str()
            ),
            feedback,
        )
        .await
    }

    /// Reads one reply draft with its exact revision and latest revision run.
    pub async fn read_reply_draft(
        &self,
        workspace_id: &str,
        reply_work_id: &str,
    ) -> Result<ReplyDraftWire, http::HttpError> {
        self.get(&format!(
            "/physician-state/reply-drafts/{workspace_id}/{reply_work_id}"
        ))
        .await
    }

    /// The same client with its own copy of the current session. A request
    /// begun under one account keeps that account's bearer even if the shared
    /// session is signed out or replaced before the request is sent.
    pub fn pinned(&self) -> Self {
        let session = Session::new();
        if let Some(current) = self.session.get() {
            session.set(current);
        }
        Self {
            base: self.base.clone(),
            http: self.http.clone(),
            session,
        }
    }

    /// The server this client calls.
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Sends recorded audio for transcription.
    pub async fn dictate(&self) -> Result<(), http::HttpError> {
        let _: serde_json::Value = self.post("/dictation", &serde_json::json!({})).await?;
        Ok(())
    }

    /// Searches the patient roster.
    pub async fn search(&self, query: &str) -> Result<PatientSearchResponse, http::HttpError> {
        self.get(&format!("/patient-roster?query={}", query_escape(query)))
            .await
    }

    /// Reads one patient workspace.
    pub async fn workspace(&self, id: &str) -> Result<PatientWorkspaceResponse, http::HttpError> {
        self.get(&format!("/patient-workspaces/{id}")).await
    }

    /// Reads saved conversations and past work for one authorized patient workspace.
    pub async fn past_work(&self, id: &str) -> Result<PastWorkResponse, http::HttpError> {
        self.get(&format!("/patient-workspaces/{id}/past-work"))
            .await
    }

    /// Records that the signed-in physician opened an attention item at least once.
    pub async fn open_item(
        &self,
        workspace_id: &str,
        item_id: &str,
        idempotency_key: &str,
    ) -> Result<serde_json::Value, http::HttpError> {
        self.command(&serde_json::json!({
            "type": "item.open",
            "idempotency_key": idempotency_key,
            "payload": {
                "patient_workspace_id": workspace_id,
                "item_id": item_id,
            }
        }))
        .await
    }

    /// Returns the event stream path for this client.
    pub fn events_path(&self) -> String {
        format!("{}/patient-workspaces/events", self.base)
    }
}

pub(crate) fn query_escape(value: &str) -> String {
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

/// How the server answered a draft save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftSave {
    /// Saved at the returned revision.
    Saved(PhysicianDraft),
    /// Another save got there first; this is the server's current draft.
    /// Nothing was written.
    Conflict(PhysicianDraft),
}

/// The current draft in a validated conflict body, and nothing else.
fn draft_conflict(body: &str) -> Option<PhysicianDraft> {
    #[derive(serde::Deserialize)]
    struct Conflict {
        outcome: String,
        current: PhysicianDraft,
    }
    let conflict: Conflict = serde_json::from_str(body).ok()?;
    (conflict.outcome == "conflict" && conflict.current.revision >= 0).then_some(conflict.current)
}

/// Decodes one response, keeping the server's status and body on failure.
async fn read_body<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, http::HttpError> {
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| http::HttpError::Transport(error.to_string()))?;
    if !status.is_success() {
        return Err(http::HttpError::Status {
            status: status.as_u16(),
            body,
        });
    }
    // An empty success body, such as a 204 sign-out, reads as null.
    if body.is_empty() {
        return serde_json::from_str("null")
            .map_err(|error| http::HttpError::Transport(error.to_string()));
    }
    serde_json::from_str(&body).map_err(|error| http::HttpError::Transport(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::response::IntoResponse;
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
                            "runs": [],
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
        let saved = client
            .save_draft("ws", "chat", draft.revision, "revised")
            .await
            .unwrap();
        assert_eq!(
            saved,
            DraftSave::Saved(PhysicianDraft {
                text: "revised".into(),
                revision: 5
            })
        );
        let snapshot = client.chat_snapshot("ws", "chat").await.unwrap();
        assert_eq!(snapshot.snapshot.conversation[0].body(), Some("Hello"));
    }

    /// Only a validated conflict body names the server's current draft; a
    /// bare or malformed 409 (an authority refusal, a proxy) stays an error.
    #[tokio::test]
    async fn only_a_validated_conflict_body_is_a_draft_conflict() {
        use axum::extract::Path;
        use axum::http::StatusCode;
        let app = axum::Router::new().route(
            "/physician-state/drafts/{workspace}/{chat}",
            axum::routing::put(|Path((_, chat)): Path<(String, String)>| async move {
                let body = match chat.as_str() {
                    "valid" => r#"{"outcome":"conflict","current":{"revision":7,"text":"newer"}}"#,
                    "bare" => "",
                    "wrong-outcome" => {
                        r#"{"outcome":"rejected","current":{"revision":7,"text":"newer"}}"#
                    }
                    "no-current" => r#"{"outcome":"conflict"}"#,
                    "negative" => r#"{"outcome":"conflict","current":{"revision":-1,"text":"x"}}"#,
                    _ => "not json",
                };
                (StatusCode::CONFLICT, body.to_owned()).into_response()
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = PhysicianClient::new(format!("http://{address}"));
        assert_eq!(
            client.save_draft("ws", "valid", 1, "mine").await.unwrap(),
            DraftSave::Conflict(PhysicianDraft {
                text: "newer".into(),
                revision: 7
            })
        );
        for malformed in ["bare", "wrong-outcome", "no-current", "negative", "garbage"] {
            assert!(
                matches!(
                    client.save_draft("ws", malformed, 1, "mine").await,
                    Err(HttpError::Status { status: 409, .. })
                ),
                "{malformed}"
            );
        }
    }

    #[test]
    fn a_pinned_client_keeps_the_session_it_was_pinned_with() {
        let client = PhysicianClient::new("http://example.test");
        client.session().set("first".into());
        let pinned = client.pinned();
        client.session().clear();
        client.session().set("second".into());
        assert_eq!(pinned.session().get().as_deref(), Some("first"));
        assert_eq!(pinned.base(), "http://example.test");
    }

    #[test]
    fn ending_an_old_session_keeps_the_one_that_replaced_it() {
        let session = Session::new();
        session.set("old".into());
        session.set("new".into());
        session.clear_if("old");
        assert_eq!(session.get().as_deref(), Some("new"));
        session.clear_if("new");
        assert_eq!(session.get(), None);
    }

    #[tokio::test]
    async fn reads_a_reply_draft_with_its_revision_run() {
        let app = axum::Router::new().route(
            "/physician-state/reply-drafts/{workspace}/{reply}",
            get(|| async {
                Json(serde_json::json!({
                    "revision": 3,
                    "text": "Reply",
                    "restore_text": null,
                    "revision_run": { "run_id": "k1", "status": "pending" }
                }))
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = PhysicianClient::new(format!("http://{address}"));
        let draft = client.read_reply_draft("ws", "reply").await.unwrap();
        assert_eq!(draft.revision, 3);
        assert!(draft.revision_run.unwrap().running());
    }

    #[tokio::test]
    async fn sign_in_attaches_the_bearer_and_sign_out_ends_it() {
        // A stand-in server that only accepts the bearer session the
        // verify route returns, exactly like the guarded sample server.
        let app = axum::Router::new()
            .route(
                "/auth/sign-in",
                axum::routing::post(|Json(body): Json<serde_json::Value>| async move {
                    assert_eq!(body["email"], "physician@codahealth.test");
                    Json(serde_json::json!({
                        "status": "token_issued",
                        "delivery": "stored",
                        "token": "token-1",
                    }))
                }),
            )
            .route(
                "/auth/verify",
                axum::routing::post(|Json(body): Json<serde_json::Value>| async move {
                    assert_eq!(body["token"], "token-1");
                    Json(serde_json::json!({
                        "session": "codas-session-1",
                        "physician_id": "staging-physician",
                        "practice_id": "roger-synthetic-practice",
                    }))
                }),
            )
            .route(
                "/auth/sign-out",
                axum::routing::post(|| async { axum::http::StatusCode::NO_CONTENT }),
            )
            .route(
                "/auth/whoami",
                get(|headers: axum::http::HeaderMap| async move {
                    let bearer = headers
                        .get(axum::http::header::AUTHORIZATION)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("");
                    if bearer != "Bearer codas-session-1" {
                        return axum::http::StatusCode::UNAUTHORIZED.into_response();
                    }
                    Json(serde_json::json!({
                        "physician_id": "staging-physician",
                        "practice_id": "roger-synthetic-practice",
                        "display_name": "Staging Physician",
                    }))
                    .into_response()
                }),
            )
            .route(
                "/today",
                get(|headers: axum::http::HeaderMap| async move {
                    let bearer = headers
                        .get(axum::http::header::AUTHORIZATION)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("");
                    if bearer == "Bearer codas-session-1" {
                        Json(serde_json::json!({
                            "date": "2026-10-09",
                            "rows": [],
                            "as_of": 0,
                        }))
                        .into_response()
                    } else {
                        axum::http::StatusCode::UNAUTHORIZED.into_response()
                    }
                }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = PhysicianClient::new(format!("http://{address}"));

        // Signed out: /today is 401, and whoami cannot resolve.
        assert!(client.whoami().await.is_err());
        assert!(matches!(
            client.today().await,
            Err(http::HttpError::Status { status: 401, .. })
        ));

        // Sign in: the client holds the session, reads resolve with it.
        client
            .begin_sign_in("physician@codahealth.test")
            .await
            .unwrap();
        client
            .verify("physician@codahealth.test", "token-1")
            .await
            .unwrap();
        let who = client.whoami().await.unwrap();
        assert_eq!(who.display_name, "Staging Physician");
        let today = client.today().await.unwrap();
        assert_eq!(today.rows.len(), 0);

        // Sign out: the session ends locally and reads stop presenting it.
        let held = client.session().get().expect("signed in");
        client.sign_out(&held).await.unwrap();
        assert!(matches!(
            client.today().await,
            Err(http::HttpError::Status { status: 401, .. })
        ));
    }
}
