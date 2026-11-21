//! Physician website caller for the existing Node `CodaServer`.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]

#[cfg(any(target_arch = "wasm32", test))]
macro_rules! try_demo {
    ($self:ident, $store:ident => $call:expr) => {
        if let crate::ClientTransport::Demo($store) = &$self.transport {
            return $call.await.map_err(crate::demo_error);
        }
    };
}

#[cfg(not(any(target_arch = "wasm32", test)))]
macro_rules! try_demo {
    ($self:ident, $store:ident => $call:expr) => {};
}

pub(crate) use try_demo;

pub mod briefings;
pub mod follow_ups;
pub mod matching_reply;
mod reviews;
pub mod source_setup;
pub mod today_updates;

pub use http::HttpError;
use physician_wire::{
    CancelInvitationResponse, ChatSnapshotResponse, CitedSourceResponse,
    ColleagueInvitationResponse, DemoSessionResponse, DictationRequest, DictationResponse,
    HealthResponse, InviteColleagueRequest, InvitePatientRequest, PanelAssignmentResponse,
    PastWorkResponse, PatientInvitationActionRequest, PatientInvitationResponse,
    PatientMemoryResponse, PatientSearchResponse, PatientWorkspaceResponse,
    PatientWorkspacesResponse, PhysicianDraft, PracticeDayResponse, RecentConversationsResponse,
    ReplyDraftWire, ResponseFeedback, ResponseId, SaveFeedback, SavePhysicianMemoryEditRequest,
    SavePhysicianMemoryEditResponse, SignInResponse, TodayListResponse, UpdatesResponse,
    VerifyResponse, WhoamiResponse,
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

/// How long a command waits for the server's answer. A hung request must end, so the page can ask
/// again and the person isn't left with an item that stays pending. The cost: a server slower than
/// this to take a command is treated as having given no answer, and is asked again.
pub const COMMAND_TIME_LIMIT: std::time::Duration = std::time::Duration::from_secs(10);

/// Preview details for a cited chart source record in interactive demo mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoSourcePreview {
    /// Clinical record title.
    pub title: String,
    /// Recorded date string.
    pub recorded_at: String,
    /// Full clinical prose.
    pub body: String,
    /// Source system label.
    pub source_label: String,
}

#[cfg(target_arch = "wasm32")]
use physician_demo_db::SqliteDriver;
#[cfg(any(target_arch = "wasm32", test))]
use physician_demo_db::{DbError, DemoStore, DraftSaveOutcome};

#[derive(Debug, Clone)]
enum ClientTransport {
    Http(reqwest::Client),
    #[cfg(any(target_arch = "wasm32", test))]
    Demo(DemoStore),
}

impl Default for ClientTransport {
    fn default() -> Self {
        Self::Http(reqwest::Client::new())
    }
}

#[derive(Debug, Clone, Default)]
/// The `PhysicianClient` value exchanged by this crate.
pub struct PhysicianClient {
    base: String,
    transport: ClientTransport,
    session: Session,
}

impl PartialEq for PhysicianClient {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.session == other.session
            && self.is_demo() == other.is_demo()
    }
}

impl PhysicianClient {
    /// Creates the client, signed out.
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into().trim_end_matches('/').to_owned(),
            transport: ClientTransport::Http(reqwest::Client::new()),
            session: Session::new(),
        }
    }

    /// Creates a browser-local SQLite demo client backed by `db_url`.
    #[cfg(target_arch = "wasm32")]
    pub fn demo_sqlite(db_url: impl Into<String>) -> Self {
        let session = Session::new();
        session.set("demo-session".to_owned());
        Self {
            base: String::new(),
            transport: ClientTransport::Demo(DemoStore::new(SqliteDriver::from_browser_url(
                db_url,
            ))),
            session,
        }
    }

    /// Creates a demo client wrapping an in-process SQLite store (tests).
    #[cfg(test)]
    pub fn demo_store(store: DemoStore) -> Self {
        let session = Session::new();
        session.set("demo-session".to_owned());
        Self {
            base: String::new(),
            transport: ClientTransport::Demo(store),
            session,
        }
    }

    /// Whether this client serves requests from the local browser SQLite demo store.
    pub fn is_demo(&self) -> bool {
        match &self.transport {
            ClientTransport::Http(_) => false,
            #[cfg(any(target_arch = "wasm32", test))]
            ClientTransport::Demo(_) => true,
        }
    }

    /// Resets the browser SQLite demo database back to its initial snapshot.
    pub async fn demo_reset(&self) -> Result<(), http::HttpError> {
        match &self.transport {
            ClientTransport::Http(_) => Ok(()),
            #[cfg(any(target_arch = "wasm32", test))]
            ClientTransport::Demo(store) => store.reset().await.map_err(demo_error),
        }
    }

    /// Reads one full chart source record for demo citation inspection.
    pub async fn demo_source_preview(
        &self,
        workspace_id: &str,
        record_id: &str,
    ) -> Result<DemoSourcePreview, http::HttpError> {
        #[cfg(not(any(target_arch = "wasm32", test)))]
        let _ = (workspace_id, record_id);
        match &self.transport {
            ClientTransport::Http(_) => Err(http::HttpError::Transport(
                "source_record lookup is only available in demo mode".to_owned(),
            )),
            #[cfg(any(target_arch = "wasm32", test))]
            ClientTransport::Demo(store) => {
                let rec = store
                    .source_record(workspace_id, record_id)
                    .await
                    .map_err(demo_error)?;
                Ok(DemoSourcePreview {
                    title: rec.title,
                    recorded_at: rec.recorded_at,
                    body: rec.body,
                    source_label: rec.source_label,
                })
            }
        }
    }

    fn http_client(&self) -> Result<&reqwest::Client, http::HttpError> {
        match &self.transport {
            ClientTransport::Http(client) => Ok(client),
            #[cfg(any(target_arch = "wasm32", test))]
            ClientTransport::Demo(_) => Err(http::HttpError::Transport(
                "HTTP client requested on demo transport".to_owned(),
            )),
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
        let mut request = self.http_client()?.get(format!("{}{path}", self.base));
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
        self.post_as(self.bearer(), path, body, None).await
    }

    /// Posts `body` to `path` with `bearer`. With a `time_limit`, a request with no answer by then
    /// ends as a transport error.
    async fn post_as<T: serde::de::DeserializeOwned>(
        &self,
        bearer: Option<String>,
        path: &str,
        body: &impl serde::Serialize,
        time_limit: Option<std::time::Duration>,
    ) -> Result<T, http::HttpError> {
        let mut request = self
            .http_client()?
            .post(format!("{}{path}", self.base))
            .json(body);
        if let Some(limit) = time_limit {
            request = request.timeout(limit);
        }
        if let Some(bearer) = bearer {
            request = request.header(reqwest::header::AUTHORIZATION, bearer);
        }
        let response = request
            .send()
            .await
            .map_err(|error| http::HttpError::Transport(error.to_string()))?;
        read_body(response).await
    }

    pub(crate) async fn put<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, http::HttpError> {
        let mut request = self
            .http_client()?
            .put(format!("{}{path}", self.base))
            .json(body);
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
        let mut request = self.http_client()?.delete(format!("{}{path}", self.base));
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
        #[cfg(any(target_arch = "wasm32", test))]
        if let ClientTransport::Demo(store) = &self.transport {
            store.ensure_ready().await.map_err(demo_error)?;
            return Ok(store.health());
        }
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
                None,
            )
            .await;
        self.session.clear_if(session);
        result.map(|_| ())
    }

    /// Mints an isolated live demo session via `POST /auth/demo-session`
    /// with a short 3s probe timeout so static or offline hosts fall back quickly.
    pub async fn start_demo_session(&self) -> Result<DemoSessionResponse, http::HttpError> {
        let started: DemoSessionResponse = self
            .post_as(
                None,
                "/auth/demo-session",
                &serde_json::json!({}),
                Some(std::time::Duration::from_secs(3)),
            )
            .await?;
        self.session.set(started.session.clone());
        Ok(started)
    }

    /// Extracts the human-facing detail when a live demo command failed due
    /// to a `429` `demo_turn_budget_exhausted` error body.
    pub fn demo_budget_message(error: &http::HttpError) -> Option<String> {
        #[derive(serde::Deserialize)]
        struct BudgetBody {
            code: String,
            #[serde(default)]
            detail: String,
        }
        let http::HttpError::Status { status: 429, body } = error else {
            return None;
        };
        let parsed: BudgetBody = serde_json::from_str(body).ok()?;
        if parsed.code == "demo_turn_budget_exhausted" {
            let trimmed = parsed.detail.trim();
            Some(if trimmed.is_empty() {
                "The public live demo turn limit has been reached.".to_owned()
            } else {
                trimmed.to_owned()
            })
        } else {
            None
        }
    }

    /// Reads the session's physician when the held session still resolves.
    pub async fn whoami(&self) -> Result<WhoamiResponse, http::HttpError> {
        #[cfg(any(target_arch = "wasm32", test))]
        if let ClientTransport::Demo(store) = &self.transport {
            store.ensure_ready().await.map_err(demo_error)?;
            return Ok(store.whoami());
        }
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
        try_demo!(self, s => s.assign_panel(physician_id, patient_workspace_id, allowed));
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
        try_demo!(self, s => s.assign_panel(physician_id, patient_workspace_id, false));
        self.delete(&format!(
            "/auth/panels/{}/{}",
            query_escape(physician_id),
            query_escape(patient_workspace_id)
        ))
        .await
    }

    /// Invites a patient on this practice's roster to Duet from the patient's page.
    pub async fn invite_patient(
        &self,
        request: &InvitePatientRequest,
    ) -> Result<PatientInvitationResponse, http::HttpError> {
        try_demo!(self, s => s.invite_patient(request));
        self.post("/auth/invitations", request).await
    }

    /// Re-sends an open patient Duet invitation (`COD-49`).
    pub async fn resend_patient_invitation(
        &self,
        patient_workspace_id: &str,
    ) -> Result<PatientInvitationResponse, http::HttpError> {
        try_demo!(self, s => s.resend_patient_invitation(patient_workspace_id));
        self.post(
            "/auth/invitations/resend",
            &PatientInvitationActionRequest {
                patient_workspace_id: patient_workspace_id.to_owned(),
            },
        )
        .await
    }

    /// Cancels an open patient Duet invitation (`Rule 5`).
    pub async fn cancel_patient_invitation(
        &self,
        patient_workspace_id: &str,
    ) -> Result<CancelInvitationResponse, http::HttpError> {
        try_demo!(self, s => s.cancel_patient_invitation(patient_workspace_id));
        self.post(
            "/auth/invitations/cancel",
            &PatientInvitationActionRequest {
                patient_workspace_id: patient_workspace_id.to_owned(),
            },
        )
        .await
    }

    /// Invites a colleague physician to join the signed-in member's practice.
    pub async fn invite_colleague(
        &self,
        request: &InviteColleagueRequest,
    ) -> Result<ColleagueInvitationResponse, http::HttpError> {
        try_demo!(self, s => s.invite_colleague(request));
        self.post("/auth/colleagues/invitations", request).await
    }

    /// Reads Today.
    pub async fn today(&self) -> Result<TodayListResponse, http::HttpError> {
        try_demo!(self, s => s.today());
        self.get("/today").await
    }

    /// Reads the patient workspaces.
    pub async fn workspaces(&self) -> Result<PatientWorkspacesResponse, http::HttpError> {
        try_demo!(self, s => s.workspaces());
        self.get("/patient-workspaces").await
    }

    /// Reads the practice day.
    pub async fn practice_day(&self) -> Result<PracticeDayResponse, http::HttpError> {
        try_demo!(self, s => s.practice_day());
        self.get("/practice-day").await
    }

    /// Reads recent conversations.
    pub async fn recent_conversations(
        &self,
    ) -> Result<RecentConversationsResponse, http::HttpError> {
        try_demo!(self, s => s.recent_conversations());
        self.get("/recent-conversations").await
    }

    /// Reads Updates.
    pub async fn updates(&self) -> Result<UpdatesResponse, http::HttpError> {
        try_demo!(self, s => s.updates());
        self.get("/updates").await
    }

    /// Reads Today for one patient.
    pub async fn today_for(
        &self,
        workspace_id: &str,
    ) -> Result<TodayListResponse, http::HttpError> {
        try_demo!(self, s => s.today_for(workspace_id));
        self.get(&format!(
            "/today?patient_workspace_id={}",
            query_escape(workspace_id)
        ))
        .await
    }

    /// Sends a physician command. With no answer within [`COMMAND_TIME_LIMIT`] the request ends
    /// as a transport error, as any other lost answer does.
    pub async fn command(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, http::HttpError> {
        try_demo!(self, s => s.command(body));
        self.post_as(self.bearer(), "/commands", body, Some(COMMAND_TIME_LIMIT))
            .await
    }

    /// Reads the physician-facing patient memory (`summary.md` and `direction.md` only).
    pub async fn patient_memory(
        &self,
        workspace_id: &str,
    ) -> Result<PatientMemoryResponse, http::HttpError> {
        try_demo!(self, s => s.patient_memory(workspace_id));
        self.get(&format!(
            "/patient-workspaces/{}/memory",
            path_escape(workspace_id)
        ))
        .await
    }

    /// Saves a physician's edit to `summary.md` or `direction.md` as a log note
    /// in their own words so patient rounds fold it in.
    pub async fn save_patient_memory_edit(
        &self,
        workspace_id: &str,
        request: &SavePhysicianMemoryEditRequest,
    ) -> Result<SavePhysicianMemoryEditResponse, http::HttpError> {
        try_demo!(self, s => s.save_patient_memory_edit(workspace_id, request));
        self.post(
            &format!("/patient-workspaces/{}/memory", path_escape(workspace_id)),
            request,
        )
        .await
    }

    /// Reads one chat snapshot.
    pub async fn chat_snapshot(
        &self,
        workspace_id: &str,
        chat_id: &str,
    ) -> Result<ChatSnapshotResponse, http::HttpError> {
        try_demo!(self, s => s.chat_snapshot(workspace_id, chat_id));
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
        try_demo!(self, s => s.read_draft(workspace_id, chat_id));
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
        #[cfg(any(target_arch = "wasm32", test))]
        if let ClientTransport::Demo(store) = &self.transport {
            return match store
                .save_draft(workspace_id, chat_id, revision, text)
                .await
                .map_err(demo_error)?
            {
                DraftSaveOutcome::Saved(saved) => Ok(DraftSave::Saved(saved)),
                DraftSaveOutcome::Conflict(current) => Ok(DraftSave::Conflict(current)),
            };
        }
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
        try_demo!(self, s => s.save_feedback(workspace_id, chat_id, response, feedback));
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
        try_demo!(self, s => s.read_reply_draft(workspace_id, reply_work_id));
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
            transport: self.transport.clone(),
            session,
        }
    }

    /// The server this client calls.
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Sends recorded audio for transcription.
    pub async fn dictate(
        &self,
        request: &DictationRequest,
    ) -> Result<DictationResponse, http::HttpError> {
        #[cfg(any(target_arch = "wasm32", test))]
        if matches!(self.transport, ClientTransport::Demo(_)) {
            return Err(http::HttpError::Status {
                status: 503,
                body: "Audio dictation requires the live server.".to_owned(),
            });
        }
        self.post("/dictation", request).await
    }

    /// Reads one cited source record on demand for an open conversation.
    pub async fn cited_source(
        &self,
        workspace_id: &str,
        chat_id: &str,
        source_key: &str,
    ) -> Result<CitedSourceResponse, http::HttpError> {
        try_demo!(self, s => s.cited_source(workspace_id, chat_id, source_key));
        let encoded_key = query_escape(source_key);
        self.get(&format!(
            "/patient-workspaces/{workspace_id}/chats/{chat_id}/sources/{encoded_key}"
        ))
        .await
    }

    /// Searches the patient roster.
    pub async fn search(&self, query: &str) -> Result<PatientSearchResponse, http::HttpError> {
        try_demo!(self, s => s.search(query));
        self.search_page(query, None).await
    }

    /// Reads one page of the patient roster, optionally continuing after `after`.
    pub async fn search_page(
        &self,
        query: &str,
        after: Option<&str>,
    ) -> Result<PatientSearchResponse, http::HttpError> {
        if after.is_none() {
            try_demo!(self, s => s.search(query));
        } else {
            try_demo!(self, _s => async {
                Ok::<_, physician_demo_db::DbError>(PatientSearchResponse {
                    patients: Vec::new(),
                    next_after: None,
                    may_be_missing_someone: false,
                    match_mode: Default::default(),
                })
            });
        }
        let mut path = format!("/patient-roster?query={}", query_escape(query));
        if let Some(cursor) = after.filter(|cursor| !cursor.is_empty()) {
            path.push_str("&after=");
            path.push_str(&query_escape(cursor));
        }
        self.get(&path).await
    }

    /// Reads one patient workspace.
    pub async fn workspace(&self, id: &str) -> Result<PatientWorkspaceResponse, http::HttpError> {
        try_demo!(self, s => s.workspace(id));
        self.get(&format!("/patient-workspaces/{id}")).await
    }

    /// Reads saved conversations and past work for one authorized patient workspace.
    pub async fn past_work(&self, id: &str) -> Result<PastWorkResponse, http::HttpError> {
        try_demo!(self, s => s.past_work(id));
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

#[cfg(any(target_arch = "wasm32", test))]
fn demo_error(err: DbError) -> http::HttpError {
    match err {
        DbError::NotFound { .. } => http::HttpError::Status {
            status: 404,
            body: err.to_string(),
        },
        DbError::InvalidCommand(_) => http::HttpError::Status {
            status: 400,
            body: err.to_string(),
        },
        DbError::Conflict(_) => http::HttpError::Status {
            status: 409,
            body: err.to_string(),
        },
        other => http::HttpError::Transport(other.to_string()),
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

pub(crate) fn path_escape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for byte in raw.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
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
    async fn a_command_with_no_answer_ends_at_its_time_limit() {
        let app = axum::Router::new().route(
            "/commands",
            axum::routing::post(std::future::pending::<Json<serde_json::Value>>),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = PhysicianClient::new(format!("http://{address}"));
        let started = std::time::Instant::now();
        let answer = tokio::time::timeout(
            COMMAND_TIME_LIMIT + std::time::Duration::from_secs(5),
            client.command(&serde_json::json!({ "type": "message.submit" })),
        )
        .await
        .expect("a command with no answer never ended");
        assert!(
            matches!(answer, Err(http::HttpError::Transport(_))),
            "{answer:?}"
        );
        assert!(started.elapsed() >= COMMAND_TIME_LIMIT);
    }

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
                            "feedback": "loaded",
                            "transcript": [{
                                "work_item_id": "w1",
                                "messages": [{ "id": "k1", "text": "Hello", "saved_at": 1794600000 }],
                                "steps": {"state": "read", "steps": []},
                                "kept": [],
                                "answer": { "state": "working", "stop": { "work_item_id": "w1" }, "writing": false }
                            }]
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
        assert_eq!(snapshot.snapshot.transcript[0].messages[0].text, "Hello");
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

    #[tokio::test]
    async fn demo_transport_serves_sqlite_reads_search_and_chat() {
        let driver = physician_demo_db::SqliteDriver::open_in_memory().unwrap();
        let store = physician_demo_db::DemoStore::new(driver);
        let units = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../patients/units");
        physician_demo_db::seed::seed_from_units_dir(&store, &units, 4).unwrap();
        let client = PhysicianClient::demo_store(store);
        assert!(client.is_demo());
        assert_eq!(
            client.whoami().await.unwrap().display_name,
            "Dr. Hannah Alvarez"
        );
        assert_eq!(client.search("Kowalczyk").await.unwrap().patients.len(), 1);
        assert_eq!(
            client.workspace("unit-2013").await.unwrap().display_name,
            "Gary Kowalczyk"
        );
        client
            .command(&serde_json::json!({
                "type": "message.submit",
                "idempotency_key": "k-demo-1",
                "payload": {
                    "patient_workspace_id": "unit-2013",
                    "chat_id": "chat-unit-2013-1",
                    "text": "Review metformin and diabetes history"
                }
            }))
            .await
            .unwrap();
        let snap = client
            .chat_snapshot("unit-2013", "chat-unit-2013-1")
            .await
            .unwrap();
        assert_eq!(snap.snapshot.transcript.len(), 2);
    }

    #[tokio::test]
    async fn demo_transport_supports_every_physician_app_endpoint_and_reset() {
        use physician_wire::follow_ups::{CannotRestartResolutionWire, FollowUpStandingWire};
        use physician_wire::matching_reply::{
            ExpectedDraftBaseWire, MoveDestinationWire, PatientReplyConversationWire,
            RequestReplyProposalRevisionBody, SenderConfirmationPolicyWire,
        };
        use physician_wire::reviews::{
            PatientReviewStateWire, ReviewControlActionWire, ReviewControlRequest,
            ReviewControlResponse,
        };
        use physician_wire::source_setup::{
            ConnectSourceBody, RequestAssistedSetupBody, SimulateSourceEventBody,
            SourceConnectionMethodWire, SupportedSourceWire,
        };
        use physician_wire::today_updates::TodayDoneOutcomeWire;
        use physician_wire::{MemoryEditTarget, SavePhysicianMemoryEditRequest};

        let ns = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("coda-demo-client-all-{}-{ns}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (seed_path, active_path) = (dir.join("seed.sqlite"), dir.join("active.sqlite"));
        let seed_store = physician_demo_db::DemoStore::new(
            physician_demo_db::SqliteDriver::open_file(&seed_path).unwrap(),
        );
        let units = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../patients/units");
        assert_eq!(
            physician_demo_db::seed::seed_from_units_dir(&seed_store, &units, 6).unwrap(),
            6
        );
        drop(seed_store);
        std::fs::copy(&seed_path, &active_path).unwrap();
        let driver = physician_demo_db::SqliteDriver::open_file(&active_path).unwrap();
        driver.bind_reset_files(seed_path, active_path);
        let client = PhysicianClient::demo_store(physician_demo_db::DemoStore::new(driver));

        // Health & Today / Updates
        let health = client.health().await.unwrap();
        assert!(health.capabilities.unwrap().practice_sources);
        let today = client.today().await.unwrap();
        assert_eq!(today.rows.len(), 6);
        assert_eq!(today.messages_to_match_count, 1);

        let opened = client
            .open_today_update_item("unit-2013", "today-unit-2013", "open-1")
            .await
            .unwrap();
        assert_eq!(opened.linked_chat_id.as_deref(), Some("chat-unit-2013-1"));
        assert!(!opened.citations.is_empty());
        assert!(
            client
                .today()
                .await
                .unwrap()
                .seen
                .contains(&"today-unit-2013".to_owned())
        );

        let done = client
            .mark_today_item_done("unit-2013", "today-unit-2013", &opened.version_id, "done-1")
            .await
            .unwrap();
        assert_eq!(done.outcome, TodayDoneOutcomeWire::Accepted);
        assert_eq!(client.today().await.unwrap().rows.len(), 5);

        let updates = client.updates().await.unwrap();
        assert!(!updates.today_items.is_empty());
        assert!(!updates.earlier_days.is_empty());

        // Briefings & Schedule refresh
        let pday = client.refresh_practice_day().await.unwrap();
        assert_eq!(pday.visits.len(), 6);
        let briefings = client.workspace_briefings("unit-2013").await.unwrap();
        assert_eq!(briefings.briefings.len(), 1);
        let briefing = client
            .visit_briefing("unit-2013", &briefings.briefings[0].briefing_id)
            .await
            .unwrap();
        assert!(!briefing.briefing.sources.is_empty());

        // Past work, Chat citations, Archive/Restore, Discussion, Queue commands
        let pw = client.past_work("unit-2013").await.unwrap();
        assert!(!pw.items.is_empty());
        let snap = client
            .chat_snapshot("unit-2013", "chat-unit-2013-1")
            .await
            .unwrap();
        let source_key = snap.snapshot.sources.keys().next().unwrap().clone();
        let cited = client
            .cited_source("unit-2013", "chat-unit-2013-1", &source_key)
            .await
            .unwrap();
        assert!(!cited.source.body.is_empty());

        let arch = client
            .command(&serde_json::json!({
                "type": "conversation.archive",
                "idempotency_key": "arch-1",
                "payload": { "patient_workspace_id": "unit-2013", "chat_id": "chat-unit-2013-1" }
            }))
            .await
            .unwrap();
        assert_eq!(arch["outcome"], "accepted");
        assert!(client.workspace("unit-2013").await.unwrap().chats[0].archived);
        let rest = client
            .command(&serde_json::json!({
                "type": "conversation.restore",
                "idempotency_key": "rest-1",
                "payload": { "patient_workspace_id": "unit-2013", "chat_id": "chat-unit-2013-1" }
            }))
            .await
            .unwrap();
        assert_eq!(rest["outcome"], "accepted");
        assert!(!client.workspace("unit-2013").await.unwrap().chats[0].archived);

        let disc = client
            .command(&serde_json::json!({
                "type": "discussion.open",
                "idempotency_key": "disc-1",
                "payload": { "patient_workspace_id": "unit-2013", "chat_id": "chat-disc-1" }
            }))
            .await
            .unwrap();
        assert_eq!(disc["outcome"], "accepted");

        client
            .command(&serde_json::json!({
                "type": "message.submit",
                "idempotency_key": "q-msg-1",
                "payload": {
                    "patient_workspace_id": "unit-2013",
                    "chat_id": "chat-disc-1",
                    "text": "Queued test",
                    "delivery": "send_next"
                }
            }))
            .await
            .unwrap();
        let qedit = client
            .command(&serde_json::json!({
                "type": "message.queue_edit",
                "idempotency_key": "qe-1",
                "payload": { "patient_workspace_id": "unit-2013", "chat_id": "chat-disc-1", "message_id": "q-msg-1", "text": "Edited queued test" }
            }))
            .await
            .unwrap();
        assert_eq!(qedit["outcome"], "accepted");
        let qsend = client
            .command(&serde_json::json!({
                "type": "message.queue_send_now",
                "idempotency_key": "qs-1",
                "payload": { "patient_workspace_id": "unit-2013", "chat_id": "chat-disc-1", "message_id": "q-msg-1" }
            }))
            .await
            .unwrap();
        assert_eq!(qsend["outcome"], "accepted");

        // Follow-ups lifecycle
        let fus = client.patient_follow_ups("unit-2013").await.unwrap();
        assert!(fus.durable);
        assert_eq!(fus.follow_ups.len(), 1);
        let created = client
            .create_follow_up("unit-2013", "Check potassium", "2026-09-10", "fu-c-1")
            .await
            .unwrap();
        let fid = created.follow_up.follow_up_id.clone();
        let stopped = client
            .stop_and_discuss_follow_up(
                "unit-2013",
                &fid,
                Some("Discuss".into()),
                "America/New_York",
                "fu-sd-1",
            )
            .await
            .unwrap();
        assert!(stopped.discuss_chat_id.is_some());
        let resumed = client
            .resume_follow_up("unit-2013", &fid, "fu-res-1")
            .await
            .unwrap();
        assert!(matches!(
            resumed.follow_up.standing,
            FollowUpStandingWire::Scheduled { .. }
        ));
        let reviewed = client
            .review_follow_up_now("unit-2013", &fid, "fu-rn-1")
            .await
            .unwrap();
        assert!(reviewed.review_started);
        let resolved = client
            .resolve_cannot_restart_follow_up(
                "unit-2013",
                &fid,
                CannotRestartResolutionWire::Complete {
                    resolution: "Reviewed".into(),
                },
                "fu-done-1",
            )
            .await
            .unwrap();
        assert!(matches!(
            resolved.follow_up.standing,
            FollowUpStandingWire::Completed { .. }
        ));

        // Spruce Replies + Matching + Filtered
        let match_list = client.messages_to_match().await.unwrap();
        assert_eq!(match_list.messages.len(), 1);
        assert_eq!(match_list.filtered_count, 1);
        assert!(
            !client
                .filtered_spruce_messages()
                .await
                .unwrap()
                .messages
                .is_empty()
        );
        client
            .mark_spruce_message_as_patient("msg-filt-1", "sp-f-1")
            .await
            .unwrap();
        client
            .confirm_spruce_message_link(
                "msg-match-1",
                "unit-2013",
                SenderConfirmationPolicyWire::ConversationOnly,
                "sp-c-1",
            )
            .await
            .unwrap();

        let replies = client.patient_spruce_replies("unit-2188").await.unwrap();
        assert_eq!(replies.conversations.len(), 1);
        let saved_reply = client
            .save_reply_proposal_draft(
                "unit-2188",
                "conv-unit-2188-1",
                ExpectedDraftBaseWire::ProposalVersion { version: 1 },
                "Physician edited draft.",
            )
            .await
            .unwrap();
        assert!(matches!(
            saved_reply,
            PatientReplyConversationWire::OpenCard(_)
        ));
        let revised_reply = client
            .request_reply_proposal_revision(
                "unit-2188",
                "conv-unit-2188-1",
                &RequestReplyProposalRevisionBody {
                    expected_base: ExpectedDraftBaseWire::PhysicianDraftRevision { revision: 1 },
                    current_text: "Physician edited draft.".into(),
                    instruction: "Make warmer".into(),
                    physician_time_zone: "America/New_York".into(),
                    idempotency_key: "rev-1".into(),
                },
            )
            .await
            .unwrap();
        assert!(matches!(
            revised_reply,
            PatientReplyConversationWire::OpenCard(_)
        ));
        let restored_reply = client
            .restore_my_reply_version("unit-2188", "conv-unit-2188-1", 2, "rest-ver-1")
            .await
            .unwrap();
        assert!(matches!(
            restored_reply,
            PatientReplyConversationWire::OpenCard(_)
        ));
        let copied = client
            .copy_spruce_reply(
                "unit-2188",
                "conv-unit-2188-1",
                ExpectedDraftBaseWire::PhysicianDraftRevision { revision: 3 },
                "copy-1",
            )
            .await
            .unwrap();
        assert_eq!(copied.copied_text, "Physician edited draft.");
        let closed = client
            .close_reply_no_reply_needed("unit-2188", "conv-unit-2188-1", 1, "nrn-1")
            .await
            .unwrap();
        assert!(matches!(
            closed,
            PatientReplyConversationWire::ClosedUndoBanner { .. }
        ));
        let reopened = client
            .undo_close_no_reply_needed("unit-2188", "conv-unit-2188-1", "undo-nrn-1")
            .await
            .unwrap();
        assert!(matches!(
            reopened,
            PatientReplyConversationWire::OpenCard(_)
        ));
        let moved = client
            .move_spruce_conversation(
                "unit-2188",
                "conv-unit-2188-1",
                MoveDestinationWire::AnotherPatient {
                    patient_workspace_id: "unit-2013".into(),
                },
                Some(30),
                "mv-1",
            )
            .await
            .unwrap();
        assert!(matches!(moved, PatientReplyConversationWire::OpenCard(_)));
        client
            .undo_spruce_conversation_move("unit-2188", "conv-unit-2188-1", "mv-undo-1")
            .await
            .unwrap();

        // Patient Reviews + Memory + Connections + Roster Admin + Dictate 503
        let reviews = client.patient_reviews("unit-2013").await.unwrap();
        assert_eq!(reviews.reviews.len(), 2);
        let held_rev = reviews
            .reviews
            .iter()
            .find(|r| r.review_id.ends_with("-held"))
            .unwrap();
        let ctrl = client
            .control_patient_review(
                "unit-2013",
                &ReviewControlRequest {
                    review_id: held_rev.review_id.clone(),
                    version: held_rev.version.clone(),
                    action: ReviewControlActionWire::TryAgain,
                    idempotency_key: "ctrl-1".into(),
                },
            )
            .await
            .unwrap();
        match ctrl {
            ReviewControlResponse::Accepted { review } => {
                assert!(matches!(
                    review.state,
                    PatientReviewStateWire::Running { .. }
                ));
            }
            other => panic!("expected accepted review control, got {other:?}"),
        }

        let mem = client.patient_memory("unit-2013").await.unwrap();
        assert!(mem.durable);
        let mem_saved = client
            .save_patient_memory_edit(
                "unit-2013",
                &SavePhysicianMemoryEditRequest {
                    target: MemoryEditTarget::Direction,
                    text: "Recheck lipid panel in 6 weeks.".into(),
                    idempotency_key: "mem-1".into(),
                },
            )
            .await
            .unwrap();
        assert_eq!(mem_saved.note_number, "N1");

        let conns = client.connections().await.unwrap();
        assert_eq!(conns.sources.len(), 3);
        client
            .connect_source(
                SupportedSourceWire::Hint,
                &ConnectSourceBody {
                    method: SourceConnectionMethodWire::SimulatedPractice,
                    account_label: "Clinic".into(),
                    secret_or_code: None,
                    prerequisites_confirmed: true,
                },
            )
            .await
            .unwrap();
        client
            .refresh_source(SupportedSourceWire::Hint)
            .await
            .unwrap();
        client
            .request_assisted_setup(
                SupportedSourceWire::ElationFhir,
                &RequestAssistedSetupBody {
                    contact_email: "admin@clinic.example".into(),
                    notes: None,
                },
            )
            .await
            .unwrap();
        client
            .simulate_source_event(
                SupportedSourceWire::Spruce,
                &SimulateSourceEventBody::RevokedGrant {
                    reason: "Token expired".into(),
                },
            )
            .await
            .unwrap();
        client
            .reconnect_source(
                SupportedSourceWire::Spruce,
                &ConnectSourceBody {
                    method: SourceConnectionMethodWire::SimulatedPractice,
                    account_label: "Spruce Demo".into(),
                    secret_or_code: None,
                    prerequisites_confirmed: true,
                },
            )
            .await
            .unwrap();

        let panel = client
            .assign_panel("phys-demo-hannah", "unit-2013", true)
            .await
            .unwrap();
        assert!(panel.allowed);
        let inv = client
            .invite_patient(&physician_wire::InvitePatientRequest {
                patient_workspace_id: "unit-2013".into(),
                email: "gary@example.com".into(),
                display_name: "Gary Kowalczyk".into(),
            })
            .await
            .unwrap();
        assert_eq!(inv.status, "invited");
        client.cancel_patient_invitation("unit-2013").await.unwrap();
        client
            .invite_colleague(&physician_wire::InviteColleagueRequest {
                email: "colleague@example.com".into(),
                display_name: "Dr. Sam Lee".into(),
            })
            .await
            .unwrap();

        let dict_err = client
            .dictate(&physician_wire::DictationRequest::default())
            .await
            .unwrap_err();
        assert!(matches!(
            dict_err,
            http::HttpError::Status { status: 503, .. }
        ));

        // Reset restores pristine seeded state cleanly.
        client.demo_reset().await.unwrap();
        let reset_today = client.today().await.unwrap();
        assert_eq!(reset_today.rows.len(), 6);
        assert!(reset_today.seen.is_empty());
        assert_eq!(
            client
                .patient_follow_ups("unit-2013")
                .await
                .unwrap()
                .follow_ups
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn start_demo_session_sets_bearer_and_reads_live_routes() {
        use axum::response::IntoResponse as _;

        let app = axum::Router::new()
            .route(
                "/auth/demo-session",
                axum::routing::post(|| async {
                    axum::Json(serde_json::json!({
                        "session": "demo-live-session-1",
                        "physician_id": "phys-demo-hannah",
                        "practice_id": "coda-demo-practice",
                        "ends_at": 1_800_000_000u64,
                    }))
                }),
            )
            .route(
                "/auth/whoami",
                axum::routing::get(|headers: axum::http::HeaderMap| async move {
                    if headers
                        .get(axum::http::header::AUTHORIZATION)
                        .and_then(|v| v.to_str().ok())
                        == Some("Bearer demo-live-session-1")
                    {
                        axum::Json(serde_json::json!({
                            "physician_id": "phys-demo-hannah",
                            "practice_id": "coda-demo-practice",
                            "display_name": "Dr. Hannah Alvarez",
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
        let started = client.start_demo_session().await.unwrap();
        assert_eq!(started.session, "demo-live-session-1");
        assert_eq!(started.practice_id, "coda-demo-practice");
        assert_eq!(started.ends_at, 1_800_000_000);
        assert_eq!(
            client.session().get().as_deref(),
            Some("demo-live-session-1")
        );
        assert_eq!(
            client.whoami().await.unwrap().display_name,
            "Dr. Hannah Alvarez"
        );
    }

    #[test]
    fn demo_budget_message_parses_only_429_exhausted_code() {
        let exhausted = http::HttpError::Status {
            status: 429,
            body: serde_json::json!({
                "code": "demo_turn_budget_exhausted",
                "detail": "Live demo turn budget reached for this visitor."
            })
            .to_string(),
        };
        assert_eq!(
            PhysicianClient::demo_budget_message(&exhausted).as_deref(),
            Some("Live demo turn budget reached for this visitor.")
        );

        let generic_429 = http::HttpError::Status {
            status: 429,
            body: "rate limited".to_owned(),
        };
        assert_eq!(PhysicianClient::demo_budget_message(&generic_429), None);

        let wrong_code = http::HttpError::Status {
            status: 429,
            body: serde_json::json!({
                "code": "rate_limited",
                "detail": "Try again later"
            })
            .to_string(),
        };
        assert_eq!(PhysicianClient::demo_budget_message(&wrong_code), None);

        let status_503 = http::HttpError::Status {
            status: 503,
            body: serde_json::json!({
                "code": "demo_turn_budget_exhausted",
                "detail": "Wrong status"
            })
            .to_string(),
        };
        assert_eq!(PhysicianClient::demo_budget_message(&status_503), None);
    }
}
