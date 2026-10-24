//! Physician chat on the Rust local product.
//!
//! `message.submit` records the physician turn, calls the chat role, and
//! publishes `workspace_changed` when that call finishes. Codex `live`,
//! `record`, and `replay` do not use an API key. A scripted model stands in
//! for the CLI in tests.

use std::convert::Infallible;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::body::Body;
use axum::extract::{Extension, Path, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use futures_util::stream;
use harness::execution::{
    self, Call, Cancellation, Item, Owner, Provider, Record, Request, StreamEvent, Usage,
};
use local_product::ModelBinding;
use local_product::codex::CodexTransport;
use local_product::execution::{PhysicianSession, Status, Work};
use physician_bus::{Bus, Event, LocalBus};
use serde::Deserialize;
use serde_json::{Value, json};
use telemetry::genai;
use tracing::Instrument;

use crate::auth::{AuthContext, allows_workspace, now};
use crate::rejection_message;

/// The chat role's caller. `Fixed` never leaves the process.
pub(crate) enum ChatModel {
    /// Returns this text and does not call OpenAI.
    #[cfg(test)]
    Fixed(String),
    /// Structured offline provider with explicit synthetic provenance.
    StructuredReplay {
        binding: ModelBinding,
        replay: tokio::sync::Mutex<harness::transport::Replay>,
    },
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
    pub(crate) binding: ModelBinding,
    pub(crate) session: Mutex<PhysicianSession>,
    pub(crate) bus: Arc<dyn Bus>,
    pub(crate) sources: Vec<local_product::physician_sources::SourceRevision>,
    turns: AtomicU64,
    /// Set only when both the physician database and chat topic are configured.
    /// The site accepts and snapshots; the worker runs the model.
    durable: Option<Arc<crate::durable::DurableChat>>,
}

impl Live {
    #[cfg(test)]
    pub(crate) fn on(
        model: ChatModel,
        session: PhysicianSession,
        sources: Vec<local_product::physician_sources::SourceRevision>,
    ) -> Result<Arc<Self>, String> {
        for source in &sources {
            source.validate()?;
        }
        let binding = match &model {
            #[cfg(test)]
            ChatModel::Fixed(_) => ModelBinding {
                model: "controlled".into(),
                effort: "high".into(),
                adapter: "controlled".into(),
            },
            ChatModel::OpenAi { binding, .. }
            | ChatModel::Codex { binding, .. }
            | ChatModel::StructuredReplay { binding, .. } => binding.clone(),
        };
        Ok(Arc::new(Self {
            model,
            binding,
            session: Mutex::new(session),
            bus: Arc::new(LocalBus::new(64)),
            sources,
            turns: AtomicU64::new(0),
            durable: None,
        }))
    }
    /// Starts an empty chat session for the model's role.
    pub(crate) fn new(model: ChatModel) -> Arc<Self> {
        Self::with_bus(model, Arc::new(LocalBus::new(64)))
    }

    /// Same session as [`Self::new`], on a caller-owned bus.
    pub(crate) fn with_bus(model: ChatModel, bus: Arc<dyn Bus>) -> Arc<Self> {
        let binding = match &model {
            #[cfg(test)]
            ChatModel::Fixed(_) => ModelBinding {
                model: "gpt-6-sol".to_owned(),
                effort: "high".to_owned(),
                adapter: "openai-responses".to_owned(),
            },
            ChatModel::OpenAi { binding, .. }
            | ChatModel::Codex { binding, .. }
            | ChatModel::StructuredReplay { binding, .. } => binding.clone(),
        };
        Arc::new(Self {
            model,
            binding,
            session: Mutex::new(match std::env::var_os("CODA_PHYSICIAN_SESSION_PATH") {
                Some(path) => PhysicianSession::open(std::path::Path::new(&path))
                    .expect("open durable physician session (never reset corrupt evidence)"),
                #[cfg(test)]
                None => PhysicianSession::memory(),
                #[cfg(not(test))]
                None => PhysicianSession::open(&default_session_path())
                    .expect("open default persistent physician session"),
            }),
            bus,
            turns: AtomicU64::new(0),
            durable: None,
            sources: match std::env::var_os("CODA_PHYSICIAN_SOURCE_SNAPSHOT") {
                Some(path) => {
                    let sources: Vec<local_product::physician_sources::SourceRevision> =
                        serde_json::from_slice(
                            &std::fs::read(path).expect("read physician source-owner snapshot"),
                        )
                        .expect("decode physician source-owner snapshot");
                    for source in &sources {
                        source.validate().expect("verify physician source revision");
                    }
                    sources
                }
                None => Vec::new(),
            },
        })
    }

    /// Accepts and reads Postgres chat without opening a local session or calling a model.
    pub(crate) fn durable(
        bus: Arc<dyn Bus>,
        chat: Arc<crate::durable::DurableChat>,
    ) -> Result<Arc<Self>, String> {
        // The transport is never invoked. Hosted turns run in the worker.
        let binding = ModelBinding {
            model: "durable".into(),
            effort: "none".into(),
            adapter: "durable".into(),
        };
        let transport = local_product::codex::open_codex(
            local_product::ModelExecution::Live,
            local_product::codex::CodexSettings {
                cassette_authorization: None,
                cassette_output: None,
                cassette_path: None,
            },
        )?;
        Ok(Arc::new(Self {
            model: ChatModel::Codex {
                binding: binding.clone(),
                transport: Arc::new(Mutex::new(transport)),
            },
            binding,
            session: Mutex::new(PhysicianSession::memory()),
            bus,
            sources: Vec::new(),
            turns: AtomicU64::new(0),
            durable: Some(chat),
        }))
    }

    /// The `gen_ai.generation` span for one chat turn. Its parent is the
    /// current (request) span.
    fn generation_span(&self, chat_id: &str) -> tracing::Span {
        let turn = self.turns.fetch_add(1, Ordering::Relaxed) + 1;
        let scenario_id = crate::simulation_scenario_from_env();
        let ctx = genai::RunContext {
            // The Codex transport numbers its provider calls the same way.
            run_id: format!("physician-chat-{turn}"),
            job: "physician_chat",
            recipe_version: genai::recipe_version(),
            origin: if scenario_id.is_some() {
                genai::Origin::Simulation
            } else {
                genai::Origin::Real
            },
            scenario_id,
            session_id: Some(chat_id.to_owned()),
        };
        let (system, model, adapter) = match &self.model {
            #[cfg(test)]
            ChatModel::Fixed(_) => ("fixed", "gpt-6-sol", "fixed"),
            ChatModel::OpenAi { binding, .. } => {
                ("openai", binding.model.as_str(), "openai-responses")
            }
            ChatModel::Codex { binding, .. } => ("codex", binding.model.as_str(), "codex"),
            ChatModel::StructuredReplay { binding, .. } => {
                ("replay", binding.model.as_str(), "structured-replay")
            }
        };
        genai::generation_span(&ctx, system, model, adapter)
    }
}

#[cfg(not(test))]
fn default_session_path() -> std::path::PathBuf {
    let state = std::env::var_os("XDG_STATE_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(".local/state"))
        })
        .expect("set CODA_PHYSICIAN_SESSION_PATH or HOME/XDG_STATE_HOME");
    let path = state.join("coda-rust/physician/session.json");
    eprintln!(
        "Physician execution uses persistent local state; CODA_PHYSICIAN_SESSION_PATH overrides its XDG location."
    );
    path
}

#[derive(Deserialize)]
struct CommandBody {
    #[serde(rename = "type")]
    kind: String,
    idempotency_key: Option<String>,
    payload: Option<Value>,
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
    if body.kind == "reply.revise" {
        return revise(live, auth_ctx.map(|Extension(ctx)| ctx), body).await;
    }
    if body.kind != "message.submit" {
        return Json(json!({
            "outcome": "rejected",
            "message": rejection_message(&body.kind),
        }))
        .into_response();
    }
    let Some(payload) = body
        .payload
        .and_then(|p| serde_json::from_value::<SubmitPayload>(p).ok())
    else {
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
        return accept_durable(live, auth_ctx, payload, idempotency_key).await;
    }
    let request = {
        let mut session = live.session.lock().expect("chat session");
        match session.submit(
            &live.binding,
            &idempotency_key,
            &payload.patient_workspace_id,
            &payload.chat_id,
            &payload.text,
        ) {
            Ok(work) => work,
            Err(_) => return rejected_submit(),
        }
    };
    if let Some(request) = request {
        let live = Arc::clone(&live);
        let workspace = payload.patient_workspace_id.clone();
        let auth_ctx = auth_ctx.map(|Extension(ctx)| ctx);
        let span = live.generation_span(&payload.chat_id);
        genai::record_input(&span, &payload.text);
        let turn = span.clone();
        tokio::spawn(
            async move {
                let mut owner = LiveOwner {
                    live: Arc::clone(&live),
                    work: request.clone(),
                    auth_ctx,
                };
                let mut provider = LiveProvider(&live.model);
                let result = execution::run(
                    &mut provider,
                    &mut owner,
                    request.request.clone(),
                    &mut execution::Progress::default(),
                    execution::Limits::default(),
                    &Cancellation::default(),
                    &mut |_| Ok(()),
                )
                .await;
                match result {
                    Ok(response) => {
                        genai::record_output(&turn, &response.text);
                        // Lossy invalidation; subscribers refetch on resync.
                        if live
                            .bus
                            .publish(Event::WorkspaceChanged {
                                workspace_id: workspace,
                            })
                            .await
                            .is_err()
                        {
                            tracing::warn!("physician workspace invalidation not published");
                        }
                    }
                    Err(_) => {
                        // Detailed provider output remains in the protected owner,
                        // not in logs or exported spans.
                        genai::record_error(&turn, "physician chat failed");
                        tracing::error!("physician chat failed");
                        if live
                            .session
                            .lock()
                            .expect("chat session")
                            .settle(&request, Status::Failed)
                            .is_err()
                        {
                            tracing::error!("physician chat settlement unavailable");
                        }
                    }
                }
            }
            .instrument(span),
        );
    }
    Json(json!({
        "outcome": "accepted",
        "patient_workspace_id": payload.patient_workspace_id,
        "chat_id": payload.chat_id,
    }))
    .into_response()
}

async fn revise(live: Arc<Live>, ctx: Option<AuthContext>, body: CommandBody) -> Response {
    if live.durable.is_some() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "outcome": "rejected",
                "message": "Hosted reply revision is unavailable.",
            })),
        )
            .into_response();
    }
    let Some(ctx) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(id) = body.idempotency_key else {
        return rejected_submit();
    };
    let Some(payload) = body.payload else {
        return rejected_submit();
    };
    // Text-only legacy cassettes cannot pretend to execute a structured reply
    // revision. API/structured replay are the supported revision transports.
    if matches!(live.model, ChatModel::Codex { .. }) {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"outcome":"rejected","message":"This transport does not support reply revisions."}))).into_response();
    }
    let work = match crate::reply::admit(&live, &ctx, &id, payload).await {
        Ok(work) => work,
        Err(_) => return StatusCode::CONFLICT.into_response(),
    };
    let workspace = work.workspace.clone();
    tokio::spawn(async move {
        let mut owner = LiveOwner {
            live: live.clone(),
            work: work.clone(),
            auth_ctx: Some(ctx),
        };
        let result = execution::run(
            &mut LiveProvider(&live.model),
            &mut owner,
            work.request.clone(),
            &mut execution::Progress::default(),
            execution::Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(()),
        )
        .await;
        if result.is_err() {
            let _ = live
                .session
                .lock()
                .expect("physician session")
                .settle(&work, Status::Failed);
        }
        if live
            .bus
            .publish(Event::WorkspaceChanged {
                workspace_id: work.workspace,
            })
            .await
            .is_err()
        {
            tracing::warn!("physician workspace invalidation not published");
        }
    });
    Json(json!({"outcome":"accepted", "patient_workspace_id":workspace,"run_id":id}))
        .into_response()
}

/// The turns recorded for this chat.
pub(crate) async fn snapshot(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    Path((workspace, chat)): Path<(String, String)>,
) -> Response {
    if let Some(durable) = live.durable.clone() {
        return durable_snapshot(
            &durable,
            auth_ctx.map(|Extension(ctx)| ctx),
            workspace,
            chat,
        )
        .await;
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

async fn accept_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    payload: SubmitPayload,
    idempotency_key: String,
) -> Response {
    let Some(chat) = live.durable.clone() else {
        return rejected_submit();
    };
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let input = local_product::runtime::Input {
        key: idempotency_key,
        text: payload.text,
        timezone: payload.physician_time_zone,
    };
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match chat.accept(&scope, &input, session_hash).await {
        Ok(_) => Json(json!({
            "outcome": "accepted",
            "patient_workspace_id": payload.patient_workspace_id,
            "chat_id": payload.chat_id,
        }))
        .into_response(),
        Err(error) => {
            let collision = error.contains("idempotency collision");
            tracing::warn!(collision, "physician chat accept failed");
            let status = if collision {
                StatusCode::OK
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            };
            (
                status,
                Json(json!({
                    "outcome": "rejected",
                    "message": rejection_message("message.submit"),
                })),
            )
                .into_response()
        }
    }
}

async fn durable_snapshot(
    chat: &crate::durable::DurableChat,
    ctx: Option<AuthContext>,
    workspace: String,
    chat_id: String,
) -> Response {
    let Some(ctx) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id,
        physician: ctx.identity.physician_id,
        workspace,
        chat: chat_id,
    };
    // Unlocked display read. None, before the first accept, is an empty conversation.
    let conversation = match chat.store.read(&scope).await {
        Ok(conversation) => conversation,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let messages = conversation
        .map(|conversation| conversation.messages)
        .unwrap_or_default();
    Json(json!({
        "snapshot": {
            "conversation": messages
                .into_iter()
                .map(|message| json!({ "text": message.text }))
                .collect::<Vec<_>>()
        }
    }))
    .into_response()
}

/// `workspace_changed` and `resync` frames. The body stays open. A client
/// refetches what it shows on `resync` (bus lag or a Redis reconnect).
pub(crate) async fn events(State(live): State<Arc<Live>>) -> Response {
    let rx = live.bus.subscribe();
    let stream = stream::unfold(rx, |mut rx| async move {
        let frame = match rx.recv().await.ok()? {
            Event::WorkspaceChanged { workspace_id } | Event::WorkspaceSettled { workspace_id } => {
                let data = json!({ "patient_workspace_id": workspace_id }).to_string();
                format!("event: workspace_changed\ndata: {data}\n\n")
            }
            Event::Resync => "event: resync\ndata: {}\n\n".to_owned(),
        };
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

struct LiveOwner {
    live: Arc<Live>,
    work: Work,
    auth_ctx: Option<AuthContext>,
}
impl Owner for LiveOwner {
    fn terminal(&self, response: &execution::Response) -> bool {
        self.work.surface == local_product::execution::Surface::ReplyRevision
            && response
                .calls
                .iter()
                .any(|call| call.name == "finish_reply_revision")
    }
    async fn check(&mut self) -> Result<(), String> {
        if let Some(ctx) = &self.auth_ctx
            && (ctx.auth.store.session(&ctx.token, now()).await.is_err()
                || !allows_workspace(&ctx.auth, &ctx.identity, &self.work.workspace).await)
        {
            return Err("physician authorization withdrawn".into());
        }
        self.live
            .session
            .lock()
            .expect("chat session")
            .check(&self.work)
    }
    async fn record(&mut self, record: Record) -> Result<(), String> {
        self.live
            .session
            .lock()
            .expect("chat session")
            .record(&self.work, record)
    }
    async fn dispatch(&mut self, call: &Call, cancel: &Cancellation) -> Result<Value, String> {
        cancel.check()?;
        self.check().await?;
        if call.name == "read_patient_sources" {
            let references: Vec<String> = serde_json::from_value(
                call.arguments
                    .get("references")
                    .cloned()
                    .unwrap_or(Value::Null),
            )
            .map_err(|_| "invalid exact read references")?;
            let budget = call
                .arguments
                .get("max_result_bytes")
                .and_then(Value::as_u64)
                .ok_or("missing exact read budget")?;
            return local_product::physician_sources::exact_read(
                &self.work.sources,
                &self.work.workspace,
                &references,
                budget as usize,
            );
        }
        Ok(json!({"error": "tool_not_authorized", "name": call.name}))
    }
    async fn complete(&mut self, response: &execution::Response) -> Result<(), String> {
        let ctx = self.auth_ctx.clone();
        let _guard = match &ctx {
            Some(ctx) => Some(ctx.auth.publication.lock().await),
            None => None,
        };
        let text = if self.work.surface == local_product::execution::Surface::ReplyRevision {
            let [call] = response.calls.as_slice() else {
                return Err("revision requires one terminal call".into());
            };
            if call.name != "finish_reply_revision"
                || !call
                    .arguments
                    .get("review_basis")
                    .is_some_and(Value::is_string)
            {
                return Err("invalid reply revision terminal".into());
            }
            call.arguments
                .get("text")
                .and_then(Value::as_str)
                .ok_or("missing revised text")?
        } else {
            &response.text
        };
        let mut publish = || {
            self.live
                .session
                .lock()
                .expect("chat session")
                .complete(&self.work, text)
        };
        match &ctx {
            Some(ctx) => crate::publication::publish(ctx, &self.work.workspace, &mut publish).await,
            None => publish(),
        }
    }
}
struct LiveProvider<'a>(&'a ChatModel);
impl Provider for LiveProvider<'_> {
    async fn exchange(
        &mut self,
        request: &Request,
        cancel: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<execution::Response, String> {
        cancel.check()?;
        if let ChatModel::StructuredReplay { replay, .. } = self.0 {
            return replay.lock().await.exchange(request, cancel, emit).await;
        }
        if let ChatModel::OpenAi { key, .. } = self.0 {
            let mut provider = harness::provider::ResponsesProvider::new(
                "https://api.openai.com/v1/responses".into(),
                key.clone(),
            )?;
            return provider.exchange(request, cancel, emit).await;
        }
        let input = request
            .items
            .iter()
            .map(|item| match item {
                Item::Message { role, text } => Ok(json!({"role": role, "content": text})),
                _ => Err("legacy physician transport does not accept tool context".to_owned()),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let body = json!({"model": request.model, "reasoning": request.options,
            "instructions": request.instructions, "input": input, "store": false});
        let text = complete(self.0, &body, cancel).await?;
        // Do not drop late usage/evidence: the engine records this response before
        // checking cancellation/currentness. Streaming is provisional only.
        if !cancel.is_cancelled() {
            emit(StreamEvent::Text(text.clone()))?;
        }
        Ok(execution::Response {
            text,
            calls: vec![],
            usage: Usage::default(),
            provenance: json!({"transport": "physician-chat"}),
        })
    }
}

async fn complete(
    model: &ChatModel,
    request: &Value,
    cancel: &Cancellation,
) -> Result<String, String> {
    match model {
        #[cfg(test)]
        ChatModel::Fixed(text) => Ok(text.clone()),
        ChatModel::OpenAi { .. } | ChatModel::StructuredReplay { .. } => {
            Err("structured transport uses the harness provider".into())
        }
        ChatModel::Codex { transport, .. } => {
            let transport = Arc::clone(transport);
            let request = request.clone();
            let cancel = cancel.clone();
            match tokio::task::spawn_blocking(move || {
                transport
                    .lock()
                    .expect("codex")
                    .complete_cancellable(&request, &cancel)
            })
            .await
            {
                Ok(result) => result,
                Err(_) => Err("Codex exec task stopped".to_owned()),
            }
        }
    }
}
