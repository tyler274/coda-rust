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
use local_product::execution::{ChatOwner, ChatScope, PhysicianSession, Status, Work};
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
    /// Inline path's versioned source observation, seeded from the retained
    /// snapshot. Hosted sites import the snapshot into the runtime store
    /// instead and leave this empty.
    pub(crate) sources: Mutex<local_product::physician_sources::Observation>,
    turns: AtomicU64,
    /// Set only when both the physician database and chat topic are configured.
    /// The site accepts and snapshots; the worker runs the model.
    pub(crate) durable: Option<Arc<crate::durable::DurableChat>>,
    /// Whether the explicitly enabled synthetic reply source is on.
    synthetic_reply: Mutex<Option<String>>,
}

impl Live {
    #[cfg(test)]
    pub(crate) fn on(
        model: ChatModel,
        session: PhysicianSession,
        sources: Vec<local_product::physician_sources::SourceRevision>,
    ) -> Result<Arc<Self>, String> {
        let sources = local_product::physician_sources::Observation::seeded(sources)?;
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
            sources: Mutex::new(sources),
            turns: AtomicU64::new(0),
            durable: None,
            synthetic_reply: Mutex::new(None),
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
            synthetic_reply: Mutex::new(synthetic_reply_from_env()),
            sources: Mutex::new(
                source_snapshot()
                    .and_then(local_product::physician_sources::Observation::seeded)
                    .expect("load physician source-owner snapshot"),
            ),
        })
    }

    /// Records a source on the inline observation: the stand-in source
    /// owner's entry point, used by tests. Returns whether it changed.
    #[cfg(test)]
    pub(crate) fn record_source(
        &self,
        source: local_product::physician_sources::SourceRevision,
    ) -> Result<bool, String> {
        self.sources
            .lock()
            .expect("physician sources")
            .record(source)
    }

    /// The inline observation's version and one workspace's sources.
    pub(crate) fn observed_sources(
        &self,
        workspace: &str,
    ) -> (u64, Vec<local_product::physician_sources::SourceRevision>) {
        self.sources
            .lock()
            .expect("physician sources")
            .workspace(workspace)
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
            // The runtime store owns hosted sources; see `import_sources`.
            sources: Mutex::new(Default::default()),
            turns: AtomicU64::new(0),
            durable: Some(chat),
            synthetic_reply: Mutex::new(synthetic_reply_from_env()),
        }))
    }

    /// Whether the explicitly enabled synthetic reply source is on.
    pub(crate) fn synthetic_reply(&self) -> Option<String> {
        self.synthetic_reply
            .lock()
            .expect("synthetic reply")
            .clone()
    }

    /// Turns the synthetic reply source on for an in-process test server.
    #[cfg(test)]
    pub(crate) fn enable_synthetic_reply(&self, patient: &str) {
        *self.synthetic_reply.lock().expect("synthetic reply") = Some(patient.to_owned());
    }

    /// The signed-in physician's chats in this workspace as wire
    /// `WorkspaceChat`s, latest activity first. Hosted chats come from Postgres,
    /// scoped to this physician. Inline chats use the same identity in the
    /// one-process session; anonymous local chats stay in their own scope.
    pub(crate) async fn chats(
        &self,
        ctx: Option<&AuthContext>,
        workspace: &str,
    ) -> Result<Vec<Value>, ()> {
        let chats: Vec<(String, String, i64)> = match (&self.durable, ctx) {
            // Hosted routes always carry a session; without one there is no physician.
            (Some(_), None) => vec![],
            (Some(durable), Some(ctx)) => durable
                .store
                .chats(
                    &ctx.identity.practice_id,
                    &ctx.identity.physician_id,
                    workspace,
                )
                .await
                .map_err(|_| ())?
                .into_iter()
                .map(|chat| (chat.chat, chat.first_message, chat.opened_at))
                .collect(),
            (None, _) => self
                .session
                .lock()
                .expect("chat session")
                .chats(chat_owner(ctx).as_ref(), workspace)
                .into_iter()
                .map(|(chat, first, opened_at)| {
                    (chat, first, i64::try_from(opened_at).unwrap_or_default())
                })
                .collect(),
        };
        Ok(chats
            .into_iter()
            .map(|(chat_id, first, opened_at)| {
                let created_at = local_product::codex::format_rfc3339(
                    std::time::UNIX_EPOCH + std::time::Duration::from_secs(opened_at.max(0) as u64),
                );
                json!({
                    "chat_id": chat_id,
                    "title": chat_title(&first),
                    "created_at": created_at,
                })
            })
            .collect())
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

fn chat_owner(ctx: Option<&AuthContext>) -> Option<ChatOwner> {
    ctx.map(|ctx| ChatOwner {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
    })
}

fn synthetic_reply_from_env() -> Option<String> {
    crate::synthetic_reply::patient(
        std::env::var(crate::synthetic_reply::SWITCH)
            .ok()
            .as_deref(),
    )
}

/// Load only source-owner observations; an absent snapshot means no sources,
/// never fabricated patient evidence. It seeds the inline observation, and a
/// hosted site imports it into the runtime source store at startup.
pub(crate) fn source_snapshot()
-> Result<Vec<local_product::physician_sources::SourceRevision>, String> {
    let Some(path) = std::env::var_os("CODA_PHYSICIAN_SOURCE_SNAPSHOT") else {
        return Ok(Vec::new());
    };
    let bytes = std::fs::read(path).map_err(|_| "read physician source-owner snapshot")?;
    let sources: Vec<local_product::physician_sources::SourceRevision> =
        serde_json::from_slice(&bytes).map_err(|_| "decode physician source-owner snapshot")?;
    for source in &sources {
        source.validate()?;
    }
    Ok(sources)
}

/// Chat titles until the model names chats: the first physician message on
/// one line, cut at a word boundary near [`CHAT_TITLE_CHARS`].
pub(crate) const CHAT_TITLE_CHARS: usize = 60;

pub(crate) fn chat_title(first: &str) -> String {
    let line = first.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.chars().count() <= CHAT_TITLE_CHARS {
        return line;
    }
    let cut: String = line.chars().take(CHAT_TITLE_CHARS).collect();
    let cut = match cut.rfind(' ') {
        Some(space) if space > CHAT_TITLE_CHARS / 2 => &cut[..space],
        _ => cut.as_str(),
    };
    format!("{}…", cut.trim_end())
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

/// `work.resume` on a failed chat turn. The workspace is checked by the auth
/// middleware; the runtime checks that the work and run are the failed ones.
#[derive(Deserialize)]
struct ResumePayload {
    patient_workspace_id: String,
    chat_id: String,
    work_item_id: String,
    expected_run_id: String,
}

/// The mechanical line a failed turn shows, as the reference product words it.
pub(crate) const COULDNT_FINISH: &str =
    "Coda couldn’t finish this response. Your message is saved.";

/// The mechanical line while a hosted turn is still running.
pub(crate) const WRITING: &str = "Coda is writing this response.";

/// The mechanical line after an intentional Stop. Resume stays until the
/// physician resumes or sends something else.
pub(crate) const STOPPED: &str = "Coda stopped this response. Your message is saved.";

#[derive(Deserialize)]
struct SubmitPayload {
    patient_workspace_id: String,
    chat_id: String,
    text: String,
    physician_time_zone: String,
    /// The rehearsal scenario. Only a simulation practice's session on the
    /// hosted path may send one; see [`crate::hosting::SimulationPractices`].
    #[serde(default)]
    scenario: Option<String>,
}

/// Accepts `message.submit`, and, on the hosted path, `work.resume`,
/// `work.stop`, and `reply.revise`. Every other command is rejected.
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
    if body.kind == "work.resume" && live.durable.is_some() {
        return resume_durable(live, auth_ctx, body).await;
    }
    if body.kind == "work.stop" && live.durable.is_some() {
        return stop_durable(live, auth_ctx, body).await;
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
    // Inline chat has no rehearsal practices; its simulation is the local one.
    if payload.scenario.is_some() {
        return rejected_scenario();
    }
    let request = {
        let mut session = live.session.lock().expect("chat session");
        let owner = chat_owner(auth_ctx.as_ref().map(|Extension(ctx)| ctx));
        match session.submit(
            &live.binding,
            &idempotency_key,
            ChatScope {
                owner: owner.as_ref(),
                workspace: &payload.patient_workspace_id,
                chat: &payload.chat_id,
            },
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
    let Some(ctx) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(id) = body.idempotency_key else {
        return rejected_submit();
    };
    let Some(payload) = body.payload else {
        return rejected_submit();
    };
    if live.durable.is_some() {
        return crate::reply::accept_durable(&live, &ctx, &id, payload).await;
    }
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
    let owner = chat_owner(auth_ctx.as_ref().map(|Extension(ctx)| ctx));
    let conversation = live
        .session
        .lock()
        .expect("chat session")
        .conversation(ChatScope {
            owner: owner.as_ref(),
            workspace: &workspace,
            chat: &chat,
        });
    Json(json!({
        "snapshot": {
            "conversation": conversation
                .into_iter()
                .map(|(role, text)| json!({ "kind": turn_kind(&role), "text": text }))
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
    let Ok(scenario) = chat.admit_scenario(&scope.practice, payload.scenario) else {
        return rejected_scenario();
    };
    let input = local_product::runtime::Input {
        key: idempotency_key,
        text: payload.text,
        timezone: payload.physician_time_zone,
        scenario,
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
    Json(json!({ "snapshot": durable_snapshot_body(conversation.as_ref()) })).into_response()
}

/// Who wrote a turn, as the snapshot names it: Coda's turns are the model's, every other the physician's.
fn turn_kind(role: &str) -> &'static str {
    if role == "assistant" {
        "model"
    } else {
        "physician"
    }
}

/// The reference `SnapshotResponse` subset this site owns. A failed turn adds
/// its run and a mechanical "couldn't finish" line whose `resume_target` names
/// the exact work and run Try again resumes. A running turn offers Stop. A
/// held turn offers Resume. The physician's text stays saved.
fn durable_snapshot_body(conversation: Option<&local_product::runtime::Conversation>) -> Value {
    let Some(conversation) = conversation else {
        return json!({ "conversation": [], "runs": [] });
    };
    let mut items: Vec<Value> = conversation
        .messages
        .iter()
        .map(|message| json!({ "kind": turn_kind(&message.role), "text": message.text }))
        .collect();
    let mut runs = Vec::new();
    match conversation.status.as_str() {
        "failed" => {
            if let Some(attempt) = conversation.attempts.last() {
                runs.push(json!({ "run_id": attempt.id, "state": "failed" }));
                items.push(resume_line(COULDNT_FINISH, conversation, attempt));
            }
        }
        "held" => {
            if let Some(attempt) = conversation.attempts.last() {
                runs.push(json!({ "run_id": attempt.id, "state": "held" }));
                items.push(resume_line(STOPPED, conversation, attempt));
            }
        }
        "active" | "pending" => {
            items.push(json!({
                "kind": "mechanical",
                "text": WRITING,
                "stop_target": { "work_item_id": conversation.work },
            }));
        }
        _ => {}
    }
    json!({ "conversation": items, "runs": runs })
}

fn resume_line(
    text: &str,
    conversation: &local_product::runtime::Conversation,
    attempt: &local_product::runtime::Attempt,
) -> Value {
    json!({
        "kind": "mechanical",
        "text": text,
        "resume_target": {
            "work_item_id": conversation.work,
            "expected_run_id": attempt.id,
        },
    })
}

async fn resume_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(chat) = live.durable.clone() else {
        return rejected("work.resume");
    };
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|p| serde_json::from_value::<ResumePayload>(p).ok())
    else {
        return rejected("work.resume");
    };
    let Some(key) = body.idempotency_key.filter(|key| !key.trim().is_empty()) else {
        return rejected("work.resume");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match chat
        .resume(
            &scope,
            &key,
            session_hash,
            &payload.work_item_id,
            &payload.expected_run_id,
        )
        .await
    {
        Ok(local_product::runtime::Resumed::Refused) => rejected("work.resume"),
        Ok(_) => Json(json!({
            "outcome": "accepted",
            "patient_workspace_id": payload.patient_workspace_id,
            "chat_id": payload.chat_id,
            "run_id": payload.expected_run_id,
        }))
        .into_response(),
        Err(_) => {
            tracing::warn!("physician work resume failed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "outcome": "rejected",
                    "message": rejection_message("work.resume"),
                })),
            )
                .into_response()
        }
    }
}

/// `work.stop` on the current chat turn. The workspace is checked by the auth
/// middleware; the runtime holds only this physician's named work.
#[derive(Deserialize)]
struct StopPayload {
    patient_workspace_id: String,
    chat_id: String,
    work_item_id: String,
}

async fn stop_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(chat) = live.durable.clone() else {
        return rejected("work.stop");
    };
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|value| serde_json::from_value::<StopPayload>(value).ok())
    else {
        return rejected("work.stop");
    };
    let Some(key) = body.idempotency_key.filter(|key| !key.trim().is_empty()) else {
        return rejected("work.stop");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match chat
        .stop(&scope, &key, session_hash, &payload.work_item_id)
        .await
    {
        Ok(local_product::runtime::Stopped::Refused) => rejected("work.stop"),
        Ok(outcome) => {
            if outcome == local_product::runtime::Stopped::Held
                && live
                    .bus
                    .publish(Event::WorkspaceChanged {
                        workspace_id: payload.patient_workspace_id.clone(),
                    })
                    .await
                    .is_err()
            {
                tracing::warn!("physician workspace invalidation not published");
            }
            Json(json!({
                "outcome": "accepted",
                "patient_workspace_id": payload.patient_workspace_id,
                "chat_id": payload.chat_id,
                "work_item_id": payload.work_item_id,
            }))
            .into_response()
        }
        Err(_) => {
            tracing::warn!("physician work stop failed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "outcome": "rejected",
                    "message": rejection_message("work.stop"),
                })),
            )
                .into_response()
        }
    }
}

fn rejected(kind: &str) -> Response {
    Json(json!({
        "outcome": "rejected",
        "message": rejection_message(kind),
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

/// A scenario tag from a session that may not send one, or a malformed tag.
/// Nothing is saved.
pub(crate) fn rejected_scenario() -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!({
            "outcome": "rejected",
            "message": "Only a rehearsal practice can tag a message with a scenario.",
        })),
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

pub(crate) struct LiveOwner {
    pub(crate) live: Arc<Live>,
    pub(crate) work: Work,
    pub(crate) auth_ctx: Option<AuthContext>,
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
            && (ctx.auth.store.identity(&ctx.token, now()).await.is_err()
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
        match self.finish(response).await? {
            execution::Finish::Committed => Ok(()),
            execution::Finish::Continue(_) => Err("physician sources changed".into()),
        }
    }
    /// A reply revision publishes only on the current source observation.
    /// Under the publication guard and the auth owner's locks, a moved
    /// observation re-pins the work and returns what changed to the model
    /// instead of publishing; so does a finish that has not yet read every
    /// changed source. Chat turns read no patient sources and commit as
    /// before.
    async fn finish(
        &mut self,
        response: &execution::Response,
    ) -> Result<execution::Finish, String> {
        use local_product::physician_sources as sources;
        let ctx = self.auth_ctx.clone();
        let _guard = match &ctx {
            Some(ctx) => Some(ctx.auth.publication.lock().await),
            None => None,
        };
        let revision = self.work.surface == local_product::execution::Surface::ReplyRevision;
        let text = if revision {
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
        let mut outcome = execution::Finish::Committed;
        let mut publish = || {
            // Source observation, then session: the only order either is
            // taken together, and `record_source` takes only the first.
            let observation =
                revision.then(|| self.live.sources.lock().expect("physician sources"));
            let mut session = self.live.session.lock().expect("chat session");
            if let Some(observation) = &observation {
                let (version, current) = observation.workspace(&self.work.workspace);
                if let Some(refresh) = session.refresh_sources(&self.work, version, current)? {
                    let work = session.work(&self.work).ok_or("missing accepted work")?;
                    outcome = execution::Finish::Continue(refresh.result(&work.sources));
                    return Ok(());
                }
                let work = session.work(&self.work).ok_or("missing accepted work")?;
                let unread = sources::caught_up(&work.records, &work.refreshes, &work.sources)?;
                if !unread.is_empty() {
                    outcome = execution::Finish::Continue(sources::unread_result(&unread));
                    return Ok(());
                }
            }
            outcome = execution::Finish::Committed;
            session.complete(&self.work, text)
        };
        match &ctx {
            Some(ctx) => {
                crate::publication::publish(ctx, &self.work.workspace, &mut publish).await?
            }
            None => publish()?,
        }
        if let execution::Finish::Continue(_) = outcome
            && let Some(work) = self
                .live
                .session
                .lock()
                .expect("chat session")
                .work(&self.work)
        {
            // Reads answer from the refreshed pin.
            self.work.sources = work.sources.clone();
            self.work.sources_version = work.sources_version;
        }
        Ok(outcome)
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

#[cfg(test)]
mod durable_snapshot_tests {
    use super::{COULDNT_FINISH, STOPPED, WRITING, durable_snapshot_body};
    use local_product::runtime::{Attempt, Conversation, Message};

    fn conversation(status: &str) -> Conversation {
        Conversation {
            messages: vec![Message {
                role: "user".into(),
                text: "Hello".into(),
            }],
            status: status.into(),
            work: "work-1".into(),
            attempts: vec![Attempt {
                id: "run-1".into(),
                predecessor: None,
                work: "work-1".into(),
                revision: 1,
                status: "interrupted".into(),
                expires: 0,
                events: vec![],
            }],
            ..Default::default()
        }
    }

    #[test]
    fn a_failed_turn_keeps_the_message_and_names_the_run_to_resume() {
        let body = durable_snapshot_body(Some(&conversation("failed")));
        let items = body["conversation"].as_array().unwrap();
        assert_eq!(items[0]["kind"], "physician");
        assert_eq!(items[0]["text"], "Hello");
        assert_eq!(items[1]["kind"], "mechanical");
        assert_eq!(items[1]["text"], COULDNT_FINISH);
        assert_eq!(items[1]["resume_target"]["work_item_id"], "work-1");
        assert_eq!(items[1]["resume_target"]["expected_run_id"], "run-1");
        assert_eq!(body["runs"][0]["run_id"], "run-1");
        assert_eq!(body["runs"][0]["state"], "failed");
    }

    #[test]
    fn a_running_turn_offers_stop_and_a_held_turn_offers_resume() {
        for status in ["pending", "active"] {
            let body = durable_snapshot_body(Some(&conversation(status)));
            let items = body["conversation"].as_array().unwrap();
            assert_eq!(items.len(), 2, "{status}");
            assert_eq!(items[1]["text"], WRITING, "{status}");
            assert_eq!(
                items[1]["stop_target"]["work_item_id"], "work-1",
                "{status}"
            );
            assert!(items[1].get("resume_target").is_none(), "{status}");
            assert!(body["runs"].as_array().unwrap().is_empty(), "{status}");
        }
        let held = durable_snapshot_body(Some(&conversation("held")));
        let items = held["conversation"].as_array().unwrap();
        assert_eq!(items[1]["text"], STOPPED);
        assert_eq!(items[1]["resume_target"]["work_item_id"], "work-1");
        assert_eq!(items[1]["resume_target"]["expected_run_id"], "run-1");
        assert!(items[1].get("stop_target").is_none());
        assert_eq!(held["runs"][0]["state"], "held");
        let done = durable_snapshot_body(Some(&conversation("completed")));
        assert_eq!(done["conversation"].as_array().unwrap().len(), 1);
        assert!(done["runs"].as_array().unwrap().is_empty());
        let empty = durable_snapshot_body(None);
        assert!(empty["conversation"].as_array().unwrap().is_empty());
    }
}
