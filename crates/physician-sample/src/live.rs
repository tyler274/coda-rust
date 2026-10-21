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
use harness::execution::{
    self, Call, Cancellation, Item, Owner, Provider, Record, Request, StreamEvent, Usage,
};
use local_product::ModelBinding;
use local_product::codex::CodexTransport;
use local_product::execution::{PhysicianSession, Status, Work};
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
    events: broadcast::Sender<String>,
    pub(crate) sources: Vec<local_product::physician_sources::SourceRevision>,
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
        let (events, _) = broadcast::channel(64);
        Ok(Arc::new(Self {
            model,
            binding,
            session: Mutex::new(session),
            events,
            sources,
        }))
    }
    /// Starts an empty chat session for the model's role.
    pub(crate) fn new(model: ChatModel) -> Arc<Self> {
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
        let (events, _) = broadcast::channel(64);
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
            events,
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
        tokio::spawn(async move {
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
                Ok(_) => {
                    let _ = live.events.send(workspace);
                }
                Err(_) => {
                    // Detailed provider output remains in the protected owner, not logs.
                    if live
                        .session
                        .lock()
                        .expect("chat session")
                        .settle(&request, Status::Failed)
                        .is_err()
                    {
                        eprintln!("physician chat settlement unavailable");
                    }
                }
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

async fn revise(live: Arc<Live>, ctx: Option<AuthContext>, body: CommandBody) -> Response {
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
        let _ = live.events.send(work.workspace);
    });
    Json(json!({"outcome":"accepted", "patient_workspace_id":workspace,"run_id":id}))
        .into_response()
}

/// The turns recorded for this chat.
pub(crate) async fn snapshot(
    State(live): State<Arc<Live>>,
    Path((workspace, chat)): Path<(String, String)>,
) -> Json<Value> {
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
}

/// `workspace_changed` frames. The body stays open.
pub(crate) async fn events(State(live): State<Arc<Live>>) -> Response {
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
            let mut provider = harness::responses::ResponsesProvider::new(
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
