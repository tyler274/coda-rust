//! Physician chat on the Rust local product.
//!
//! `message.submit` records the physician turn, calls the chat role, and
//! publishes `workspace_changed` when that call finishes. Codex `live`,
//! `record`, and `replay` do not use an API key. A scripted model stands in
//! for the CLI in tests.

use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::body::Body;
use axum::extract::{Extension, Path, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use futures_util::stream;
use local_product::codex::CodexTransport;
use local_product::{ChatSession, ModelBinding, Submit};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::broadcast;

use crate::auth::{AuthContext, allows_workspace, now};
use crate::rejection_message;

/// The chat role's caller. `Fixed` never leaves the process.
pub(crate) enum ChatModel {
    /// Returns this text and does not call OpenAI.
    #[cfg(test)]
    Fixed(String),
    /// Calls OpenAI Responses with this key and the chat binding.
    OpenAi {
        /// The API key. It is not written to logs.
        key: String,
        /// The chat role.
        binding: ModelBinding,
    },
    /// Codex CLI live, record, or replay. No API key.
    Codex {
        /// The chat role.
        binding: ModelBinding,
        /// The Codex transport. Record and replay mutate the cassette.
        transport: Arc<Mutex<CodexTransport>>,
    },
}

/// In-memory chats and the event other subscribers are waiting on.
pub(crate) struct Live {
    model: ChatModel,
    session: Mutex<ChatSession>,
    events: broadcast::Sender<String>,
    durable: Option<local_product::runtime::Store>,
}

impl Live {
    /// Starts an empty chat session for the model's role.
    pub(crate) fn new(model: ChatModel) -> Arc<Self> {
        let binding = match &model {
            #[cfg(test)]
            ChatModel::Fixed(_) => ModelBinding {
                model: "gpt-6-sol".to_owned(),
                effort: "high".to_owned(),
                adapter: "openai-responses".to_owned(),
            },
            ChatModel::OpenAi { binding, .. } | ChatModel::Codex { binding, .. } => binding.clone(),
        };
        let (events, _) = broadcast::channel(64);
        Arc::new(Self {
            model,
            session: Mutex::new(ChatSession::new(binding)),
            events,
            durable: None,
        })
    }

    /// Hosted composition must choose this constructor explicitly, after migrations.
    /// API-backed execution only; never falls back to an operator subscription.
    pub(crate) async fn hosted(model: ChatModel, database_url: &str) -> Result<Arc<Self>, String> {
        if !matches!(model, ChatModel::OpenAi { .. }) {
            return Err("hosted physician execution requires API mode".into());
        }
        let store = local_product::runtime::Store::connect(database_url)
            .await
            .map_err(|e| e.to_string())?;
        store.pending().await.map_err(|e| e.to_string())?;
        let mut live = Self::new(model);
        Arc::get_mut(&mut live).expect("new live owner").durable = Some(store);
        Ok(live)
    }
}

#[derive(Deserialize)]
struct CommandBody {
    #[serde(rename = "type")]
    kind: String,
    idempotency_key: Option<String>,
    payload: Option<SubmitPayload>,
}

#[derive(Deserialize)]
struct SubmitPayload {
    patient_workspace_id: String,
    chat_id: String,
    text: String,
    physician_time_zone: String,
}

/// Accepts `message.submit` and rejects every other command.
pub(crate) async fn submit(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: axum::body::Bytes,
) -> Response {
    let Ok(body) = serde_json::from_slice::<CommandBody>(&body) else {
        return rejected_submit();
    };
    if body.kind != "message.submit" {
        return Json(json!({
            "outcome": "rejected",
            "message": rejection_message(&body.kind),
        }))
        .into_response();
    }
    let Some(payload) = body.payload else {
        return rejected_submit();
    };
    if payload.patient_workspace_id.trim().is_empty()
        || payload.chat_id.trim().is_empty()
        || payload.physician_time_zone.trim().is_empty()
    {
        return rejected_submit();
    }
    let Some(idempotency_key) = body.idempotency_key.filter(|key| !key.trim().is_empty()) else {
        return rejected_submit();
    };
    if live.durable.is_some() {
        return durable_submit(
            live,
            auth_ctx.map(|Extension(ctx)| ctx),
            payload,
            idempotency_key,
        )
        .await;
    }
    let request = {
        let mut session = live.session.lock().expect("chat session");
        match session.submit(
            &idempotency_key,
            &payload.patient_workspace_id,
            &payload.chat_id,
            &payload.text,
        ) {
            Submit::Accepted { request } => Some(request),
            Submit::Duplicate => None,
            Submit::Rejected => return rejected_submit(),
        }
    };
    if let Some(request) = request {
        let live = Arc::clone(&live);
        let workspace = payload.patient_workspace_id.clone();
        let chat = payload.chat_id.clone();
        let auth_ctx = auth_ctx.map(|Extension(ctx)| ctx);
        tokio::spawn(async move {
            match complete(&live.model, &request).await {
                Ok(text) => {
                    if let Some(ref ctx) = auth_ctx {
                        let session_ok = ctx.auth.store.session(&ctx.token, now()).await.is_ok();
                        let allowed = allows_workspace(&ctx.auth, &ctx.identity, &workspace).await;
                        if !session_ok || !allowed {
                            return;
                        }
                    }
                    live.session
                        .lock()
                        .expect("chat session")
                        .record_reply(&workspace, &chat, &text);
                    let _ = live.events.send(workspace);
                }
                Err(error) => eprintln!("physician chat: {error}"),
            }
        });
    }
    Json(json!({
        "outcome": "accepted",
        "patient_workspace_id": payload.patient_workspace_id,
        "chat_id": payload.chat_id,
    }))
    .into_response()
}

/// The turns recorded for this chat.
pub(crate) async fn snapshot(
    State(live): State<Arc<Live>>,
    Path((workspace, chat)): Path<(String, String)>,
    auth_ctx: Option<Extension<AuthContext>>,
) -> Response {
    if let Some(store) = &live.durable {
        let Some(Extension(ctx)) = auth_ctx else {
            return StatusCode::UNAUTHORIZED.into_response();
        };
        let scope = durable_scope(&ctx, &workspace, &chat);
        if RuntimeAuthority(ctx).check_scope(&scope).await.is_err() {
            return StatusCode::NOT_FOUND.into_response();
        }
        return match store.conversation(&scope).await {
            Ok(state) => Json(
                json!({"snapshot": {"conversation": state.messages, "work_status": state.status}}),
            )
            .into_response(),
            Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
    }
    let conversation = live
        .session
        .lock()
        .expect("chat session")
        .conversation(&workspace, &chat);
    Json(json!({
        "snapshot": {
            "conversation": conversation
                .into_iter()
                .map(|text| json!({ "text": text }))
                .collect::<Vec<_>>()
        }
    }))
    .into_response()
}

/// `workspace_changed` frames. The body stays open.
pub(crate) async fn events(State(live): State<Arc<Live>>) -> Response {
    // The legacy global broadcast has no tenant scope or reconnect cursor.
    // Do not expose it in hosted mode; clients can read durable snapshots.
    if live.durable.is_some() {
        return StatusCode::NOT_IMPLEMENTED.into_response();
    }
    let rx = live.events.subscribe();
    let stream = stream::unfold(rx, |mut rx| async move {
        let workspace = rx.recv().await.ok()?;
        let data = json!({ "patient_workspace_id": workspace }).to_string();
        let frame = format!("event: workspace_changed\ndata: {data}\n\n");
        Some((Ok::<String, Infallible>(frame), rx))
    });
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/event-stream"),
        )],
        Body::from_stream(stream),
    )
        .into_response()
}

fn rejected_submit() -> Response {
    Json(json!({
        "outcome": "rejected",
        "message": rejection_message("message.submit"),
    }))
    .into_response()
}

fn durable_scope(ctx: &AuthContext, workspace: &str, chat: &str) -> local_product::runtime::Scope {
    local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: workspace.into(),
        chat: chat.into(),
    }
}

struct RuntimeAuthority(AuthContext);
impl RuntimeAuthority {
    async fn check_scope(&self, scope: &local_product::runtime::Scope) -> Result<(), String> {
        let ctx = &self.0;
        let identity = ctx
            .auth
            .store
            .session(&ctx.token, now())
            .await
            .map_err(|_| "session revoked")?;
        if identity.practice_id != scope.practice
            || identity.physician_id != scope.physician
            || !allows_workspace(&ctx.auth, &identity, &scope.workspace).await
        {
            return Err("workspace authority revoked".into());
        }
        Ok(())
    }
}

#[local_product::runtime::async_trait]
impl local_product::runtime::Authority for RuntimeAuthority {
    async fn check(&mut self, scope: &local_product::runtime::Scope) -> Result<(), String> {
        self.check_scope(scope).await
    }
    async fn lock_publication(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &local_product::runtime::Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        physician_auth::PostgresAuthStore::lock_runtime_authority(
            tx,
            session_hash.ok_or("accepted session is missing")?,
            &physician_auth::PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|e| e.to_string())
    }
}

async fn durable_submit(
    live: Arc<Live>,
    ctx: Option<AuthContext>,
    payload: SubmitPayload,
    key: String,
) -> Response {
    use local_product::runtime::{Accepted, Input, PhysicianHost, Responses, engine};
    let Some(ctx) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let scope = durable_scope(&ctx, &payload.patient_workspace_id, &payload.chat_id);
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    let authority = RuntimeAuthority(ctx);
    if authority.check_scope(&scope).await.is_err() {
        return StatusCode::NOT_FOUND.into_response();
    }
    let store = live.durable.as_ref().expect("durable dispatch").clone();
    let input = Input {
        key,
        text: payload.text,
        timezone: payload.physician_time_zone,
    };
    let accepted = match store.accept_authorized(&scope, &input, session_hash).await {
        Ok(accepted) => accepted,
        Err(error) if error.to_string().contains("idempotency collision") => {
            return StatusCode::CONFLICT.into_response();
        }
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    if matches!(accepted, Accepted::New | Accepted::Duplicate) {
        let ChatModel::OpenAi { key, binding } = &live.model else {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        };
        let mut provider = match Responses::new(key.clone(), binding.model.clone()) {
            Ok(provider) => provider,
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
        let claim = match store.claim(&scope, 180).await {
            Ok(claim) => claim,
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
        if let Some(claim) = claim {
            tokio::spawn(async move {
                let mut host = PhysicianHost { store, claim, authority,
                    instructions: "Physician chat runtime v1. No chart evidence or clinical action tools are available in this hosted slice. Do not claim to have read a chart or performed an action.".into() };
                let limits = engine::Limits {
                    model_calls: 8,
                    tool_calls: 0,
                    calls_per_turn: 8,
                    request_bytes: 256_000,
                    response_bytes: 128_000,
                    tool_result_bytes: 32_000,
                    total_tool_result_bytes: 128_000,
                    output_tokens: 4096,
                    elapsed: std::time::Duration::from_secs(120),
                };
                if host
                    .run(&mut provider, &limits, &engine::Cancellation::default())
                    .await
                    .is_err()
                {
                    eprintln!("physician durable execution ended without publication");
                }
            });
        }
    }
    Json(json!({"outcome": "accepted", "patient_workspace_id": scope.workspace, "chat_id": scope.chat})).into_response()
}

/// Explicit recovery seam for coordinated composition. Intentionally not called
/// during construction: hosted auth and clinical-tool qualification still gate start.
impl Live {
    pub(crate) async fn recover_pending(
        self: &Arc<Self>,
        after: Option<&local_product::runtime::Scope>,
    ) -> Result<local_product::runtime::recovery::Report, String> {
        let store = self.durable.clone().ok_or("durable runtime required")?;
        store
            .dispatch_page(&mut RecoveryExecutor(Arc::clone(self)), 32, 180, after)
            .await
            .map_err(|e| e.to_string())
    }
}

struct RecoveryAuthority(local_product::runtime::Store);
#[local_product::runtime::async_trait]
impl local_product::runtime::Authority for RecoveryAuthority {
    async fn check(&mut self, scope: &local_product::runtime::Scope) -> Result<(), String> {
        let store = self.0.clone();
        store
            .authorize(scope, self)
            .await
            .map_err(|e| e.to_string())
    }
    async fn lock_publication(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &local_product::runtime::Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        physician_auth::PostgresAuthStore::lock_runtime_authority(
            tx,
            hash.ok_or("accepted session is missing")?,
            &physician_auth::PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|e| e.to_string())
    }
}
struct RecoveryExecutor(Arc<Live>);
#[local_product::runtime::async_trait]
impl local_product::runtime::recovery::Executor for RecoveryExecutor {
    async fn authorize(
        &mut self,
        scope: &local_product::runtime::Scope,
        _: &[u8],
    ) -> Result<(), String> {
        use local_product::runtime::Authority;
        RecoveryAuthority(self.0.durable.clone().ok_or("durable runtime required")?)
            .check(scope)
            .await
    }
    async fn execute(
        &mut self,
        store: local_product::runtime::Store,
        claim: local_product::runtime::Claim,
        _: Vec<u8>,
    ) -> Result<(), String> {
        use local_product::runtime::{PhysicianHost, Responses, engine};
        let ChatModel::OpenAi { key, binding } = &self.0.model else {
            return Err("API mode required".into());
        };
        let mut provider = Responses::new(key.clone(), binding.model.clone())?;
        let mut host = PhysicianHost { authority: RecoveryAuthority(store.clone()), store, claim,
            instructions: "Physician chat runtime v1. No chart evidence or clinical action tools are available in this hosted slice. Do not claim to have read a chart or performed an action.".into() };
        host.run(
            &mut provider,
            &engine::Limits {
                model_calls: 8,
                tool_calls: 0,
                calls_per_turn: 8,
                request_bytes: 256_000,
                response_bytes: 128_000,
                tool_result_bytes: 32_000,
                total_tool_result_bytes: 128_000,
                output_tokens: 4096,
                elapsed: std::time::Duration::from_secs(120),
            },
            &engine::Cancellation::default(),
        )
        .await
    }
}

async fn complete(model: &ChatModel, request: &Value) -> Result<String, String> {
    match model {
        #[cfg(test)]
        ChatModel::Fixed(text) => Ok(text.clone()),
        ChatModel::OpenAi { key, .. } => {
            let response = reqwest::Client::new()
                .post("https://api.openai.com/v1/responses")
                .bearer_auth(key)
                .json(request)
                .send()
                .await
                .map_err(|_| "OpenAI responses request failed".to_owned())?;
            if !response.status().is_success() {
                return Err(format!("OpenAI responses returned {}", response.status()));
            }
            let body: Value = response
                .json()
                .await
                .map_err(|_| "OpenAI responses returned unreadable JSON".to_owned())?;
            local_product::authored_text(&body)
                .ok_or_else(|| "OpenAI responses returned no authored text".to_owned())
        }
        ChatModel::Codex { transport, .. } => {
            let transport = Arc::clone(transport);
            let request = request.clone();
            match tokio::task::spawn_blocking(move || {
                transport.lock().expect("codex").complete(&request)
            })
            .await
            {
                Ok(result) => result,
                Err(_) => Err("Codex exec task stopped".to_owned()),
            }
        }
    }
}
