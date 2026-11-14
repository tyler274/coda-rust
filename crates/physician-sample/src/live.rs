//! Physician chat HTTP composition over the durable Postgres owner.
//! The site accepts work and reads its state; the worker runs the shared loop.

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::body::Body;
use axum::extract::{Extension, Path, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use futures_util::stream;
use local_product::runtime::feedback::Feedback;
use physician_bus::{Bus, Event};
use physician_wire::{ChatSnapshot, ChatSnapshotResponse};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::auth::AuthContext;
use crate::rejection_message;

pub(crate) struct Live {
    pub(crate) bus: Arc<dyn Bus>,
    pub(crate) chat: Arc<crate::durable::DurableChat>,
    synthetic_reply: Option<String>,
}

impl Live {
    pub(crate) fn new(bus: Arc<dyn Bus>, chat: Arc<crate::durable::DurableChat>) -> Arc<Self> {
        Arc::new(Self {
            bus,
            chat,
            synthetic_reply: synthetic_reply_from_env(),
        })
    }

    pub(crate) fn synthetic_reply(&self) -> Option<&str> {
        self.synthetic_reply.as_deref()
    }

    /// The physician's retained chats in this patient workspace, distinguished
    /// by question title, answer summary, and outcome.
    pub(crate) async fn chats(
        &self,
        ctx: Option<&AuthContext>,
        workspace: &str,
    ) -> Result<Vec<Value>, ()> {
        let ctx = ctx.ok_or(())?;
        let chats = self
            .chat
            .store
            .recent_conversations(
                &ctx.identity.practice_id,
                &ctx.identity.physician_id,
                &[workspace.to_owned()],
                200,
            )
            .await
            .map_err(|_| ())?;
        Ok(chats
            .into_iter()
            .map(|chat| {
                let created_at = crate::timestamp::format_rfc3339(
                    std::time::UNIX_EPOCH
                        + std::time::Duration::from_secs(chat.opened_at.max(0) as u64),
                );
                let updated_at = crate::timestamp::format_rfc3339(
                    std::time::UNIX_EPOCH
                        + std::time::Duration::from_secs(chat.last_activity_at.max(0) as u64),
                );
                json!({
                    "chat_id": chat.chat,
                    "title": chat_title(&chat.question),
                    "summary": chat.summary,
                    "outcome": chat.outcome,
                    "created_at": created_at,
                    "updated_at": updated_at,
                })
            })
            .collect())
    }

    /// Recent conversations across all workspaces currently on the physician's panel.
    pub(crate) async fn recent_conversations(
        &self,
        ctx: Option<&AuthContext>,
        panel: &[String],
        directory: &dyn crate::patients::PatientDirectory,
    ) -> Result<Vec<Value>, ()> {
        let ctx = ctx.ok_or(())?;
        if panel.is_empty() {
            return Ok(Vec::new());
        }
        let found_patients = crate::patients::read_patients(directory, panel)
            .await
            .map_err(|_| ())?;
        let names: BTreeMap<String, String> = found_patients
            .into_iter()
            .map(|patient| {
                let row = patient.list_row();
                (
                    row["patient_workspace_id"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                    row["display_name"].as_str().unwrap_or_default().to_owned(),
                )
            })
            .collect();
        let conversations = self
            .chat
            .store
            .recent_conversations(
                &ctx.identity.practice_id,
                &ctx.identity.physician_id,
                panel,
                50,
            )
            .await
            .map_err(|_| ())?;
        Ok(conversations
            .into_iter()
            .filter_map(|conv| {
                let patient_name = names.get(&conv.workspace)?.clone();
                let updated_at = crate::timestamp::format_rfc3339(
                    std::time::UNIX_EPOCH
                        + std::time::Duration::from_secs(conv.last_activity_at.max(0) as u64),
                );
                Some(json!({
                    "chat_id": conv.chat,
                    "patient_workspace_id": conv.workspace,
                    "patient_display_name": patient_name,
                    "title": chat_title(&conv.question),
                    "summary": conv.summary,
                    "outcome": conv.outcome,
                    "updated_at": updated_at,
                }))
            })
            .collect())
    }
}

fn synthetic_reply_from_env() -> Option<String> {
    crate::synthetic_reply::patient(
        std::env::var(crate::synthetic_reply::SWITCH)
            .ok()
            .as_deref(),
    )
}

/// Load only source-owner observations; an absent snapshot means no sources,
/// never fabricated patient evidence. The site imports it into the runtime
/// source store at startup.
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

/// Accepts chat, Stop/Resume, item.open, and reply-revision commands into the durable owner.
/// Every other command is rejected.
pub(crate) async fn submit(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    sample_feed: Option<Extension<Arc<crate::SampleFeed>>>,
    body: axum::body::Bytes,
) -> Response {
    let Ok(body) = serde_json::from_slice::<CommandBody>(&body) else {
        return rejected_submit();
    };
    if body.kind == "item.open" {
        return crate::accept_item_open(
            Some(&live.chat.store),
            auth_ctx.map(|Extension(ctx)| ctx),
            sample_feed.map(|Extension(feed)| feed),
            Some(&live.bus),
            body.idempotency_key,
            body.payload,
        )
        .await;
    }
    if body.kind == "reply.revise" {
        return revise(live, auth_ctx.map(|Extension(ctx)| ctx), body).await;
    }
    if body.kind == "work.resume" {
        return resume_durable(live, auth_ctx, body).await;
    }
    if body.kind == "work.stop" {
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
    accept_durable(live, auth_ctx, payload, idempotency_key).await
}

/// Saved conversations and reply work in one authorized patient workspace.
pub(crate) async fn past_work(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    Path(workspace): Path<String>,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let items = match live
        .chat
        .store
        .workspace_past_work(
            &ctx.identity.practice_id,
            &ctx.identity.physician_id,
            &workspace,
        )
        .await
    {
        Ok(items) => items,
        Err(error) => {
            tracing::error!(%error, "physician workspace past_work read failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };
    let wire_items: Vec<Value> = items
        .into_iter()
        .map(|item| {
            let updated_at = crate::timestamp::format_rfc3339(
                std::time::UNIX_EPOCH
                    + std::time::Duration::from_secs(item.updated_at.max(0) as u64),
            );
            json!({
                "id": item.id,
                "target": item.target,
                "title": item.title,
                "summary": item.summary,
                "outcome": item.outcome,
                "updated_at": updated_at,
            })
        })
        .collect();
    Json(json!({ "items": wire_items })).into_response()
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
    crate::reply::accept_durable(&live, &ctx, &id, payload).await
}

/// The turns recorded for this chat.
pub(crate) async fn snapshot(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    Path((workspace, chat)): Path<(String, String)>,
) -> Response {
    durable_snapshot(
        &live.chat,
        auth_ctx.map(|Extension(ctx)| ctx),
        workspace,
        chat,
    )
    .await
}

async fn accept_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    payload: SubmitPayload,
    idempotency_key: String,
) -> Response {
    let chat = &live.chat;
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
        Ok(accepted) => {
            if accepted == local_product::runtime::Accepted::New
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
            }))
            .into_response()
        }
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
        Err(error) => {
            tracing::error!(%error, "physician chat snapshot conversation read failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };
    let readback = match chat.store.reopen_conversation(&scope).await {
        Ok(readback) => readback,
        Err(error) => {
            tracing::error!(%error, "physician chat snapshot readback failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };
    // The conversation comes even when its feedback can't be read, so one
    // unreadable feedback row never keeps the physician from Coda's replies;
    // the snapshot then says feedback is unavailable instead of showing rated
    // responses as unrated.
    let feedback = match chat.store.chat_feedback(&scope).await {
        Ok(feedback) => SnapshotFeedback::Loaded(feedback),
        Err(error) => {
            tracing::error!(%error, "physician chat snapshot feedback read failed");
            SnapshotFeedback::Unavailable
        }
    };
    match snapshot_body(conversation.as_ref(), readback.as_ref(), &feedback) {
        Ok(snapshot) => Json(ChatSnapshotResponse { snapshot }).into_response(),
        Err(error) => {
            tracing::error!(%error, "physician chat snapshot transcript unreadable");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

/// The physician's feedback on a chat's responses, as the snapshot read it.
enum SnapshotFeedback {
    /// Each rated response's saved feedback, by response.
    Loaded(BTreeMap<local_product::runtime::ResponseId, Feedback>),
    /// The read failed; no response carries feedback.
    Unavailable,
}

fn wire_segment(
    segment: &local_product::runtime::history::HistorySegment,
) -> physician_wire::MessageSegment {
    match segment {
        local_product::runtime::history::HistorySegment::Text { text } => {
            physician_wire::MessageSegment::Text { text: text.clone() }
        }
        local_product::runtime::history::HistorySegment::Citation {
            source_key,
            number,
            label,
            quote,
        } => physician_wire::MessageSegment::Citation {
            source_key: source_key.clone(),
            number: *number,
            label: label.clone(),
            quote: quote.clone(),
        },
    }
}

/// The chat's transcript, with the physician's saved feedback and verified
/// source citations on each finished answer when they were read. A chat with
/// no accepted message yet has an empty transcript.
fn snapshot_body(
    conversation: Option<&local_product::runtime::Conversation>,
    readback: Option<&local_product::runtime::history::SavedConversationReadback>,
    feedback: &SnapshotFeedback,
) -> Result<ChatSnapshot, local_product::runtime::TranscriptError> {
    let mut transcript = match conversation {
        Some(conversation) => conversation.transcript(|id| match feedback {
            SnapshotFeedback::Loaded(saved) => saved.get(id).cloned().map(crate::feedback::wire),
            SnapshotFeedback::Unavailable => None,
        })?,
        None => Vec::new(),
    };
    let mut sources = BTreeMap::new();
    if let Some(readback) = readback {
        sources = readback
            .sources
            .iter()
            .map(|(key, src)| {
                (
                    key.clone(),
                    physician_wire::CitedSourceWire {
                        source_key: src.source_key.clone(),
                        number: src.number,
                        source_label: src.source_label.clone(),
                        title: src.title.clone(),
                        record_type: (!src.record_type.is_empty()).then(|| src.record_type.clone()),
                        recorded_at: src.recorded_at.clone(),
                        quote: src.quote.clone(),
                        body: src.body.clone(),
                        raw_content: (!src.raw_content.is_empty()).then(|| src.raw_content.clone()),
                        external_url: src.external_url.clone(),
                    },
                )
            })
            .collect();
        let assistant_turns: BTreeMap<&str, &local_product::runtime::history::SavedTurnReadback> =
            readback
                .turns
                .iter()
                .filter(|turn| {
                    matches!(
                        turn.author,
                        local_product::runtime::history::TurnAuthor::Assistant { partial: false }
                    )
                })
                .map(|turn| (turn.id.as_str(), turn))
                .collect();
        for exchange in &mut transcript {
            if let physician_wire::AnswerState::Finished {
                response_id,
                segments,
                basis,
                newer_records,
                ..
            } = &mut exchange.answer
                && let Some(turn) = assistant_turns.get(response_id.as_str())
            {
                *segments = turn.segments.iter().map(wire_segment).collect();
                *basis = turn
                    .basis
                    .as_ref()
                    .map(|items| items.iter().map(wire_segment).collect())
                    .filter(|items: &Vec<_>| !items.is_empty());
                *newer_records = turn.newer_records;
            }
        }
    }
    let feedback = match feedback {
        SnapshotFeedback::Loaded(_) => physician_wire::SnapshotFeedback::Loaded,
        SnapshotFeedback::Unavailable => physician_wire::SnapshotFeedback::Unavailable,
    };
    Ok(ChatSnapshot {
        transcript,
        feedback,
        sources,
    })
}

async fn resume_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let chat = &live.chat;
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
        Ok(outcome) => {
            if matches!(outcome, local_product::runtime::Resumed::Pending { .. })
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
                "run_id": payload.expected_run_id,
            }))
            .into_response()
        }
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
    let chat = &live.chat;
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
pub(crate) async fn events(State(bus): State<Arc<dyn Bus>>) -> Response {
    let rx = bus.subscribe();
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
