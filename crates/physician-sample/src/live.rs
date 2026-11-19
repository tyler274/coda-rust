//! Physician chat HTTP composition over the durable Postgres owner.
//! The site accepts work and reads its state; the worker runs the shared loop.

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::response::sse::{self, Sse};
use axum::response::{IntoResponse, Response};
use futures_util::{Stream, StreamExt, stream};
use local_product::runtime::feedback::Feedback;
use physician_bus::{Bus, Event};
use physician_wire::{ChatSnapshot, ChatSnapshotResponse};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::time::{Instant, MissedTickBehavior, interval_at};

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
    #[serde(default)]
    delivery: physician_wire::SubmitDelivery,
    /// The rehearsal scenario. Only a simulation practice's session on the
    /// hosted path may send one; see [`crate::hosting::SimulationPractices`].
    #[serde(default)]
    scenario: Option<String>,
}

#[derive(Deserialize)]
struct QueueEditPayload {
    patient_workspace_id: String,
    chat_id: String,
    message_id: String,
    text: String,
}

#[derive(Deserialize)]
struct QueueWithdrawPayload {
    patient_workspace_id: String,
    chat_id: String,
    message_id: String,
}

#[derive(Deserialize)]
struct QueueSendNowPayload {
    patient_workspace_id: String,
    chat_id: String,
    message_id: String,
    physician_time_zone: String,
}

#[derive(Deserialize)]
struct DecideRecommendationPayload {
    patient_workspace_id: String,
    recommendation_id: String,
    #[serde(default)]
    decision: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

/// Accepts chat, Stop/Resume, item.open, plan.decide_recommendation, and reply-revision commands
/// into the durable owner. Every other command is rejected.
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
    if body.kind == "plan.decide_recommendation" {
        return decide_recommendation_durable(live, auth_ctx, body).await;
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
    if body.kind == "message.queue_edit" {
        return queue_edit_durable(live, auth_ctx, body).await;
    }
    if body.kind == "message.withdraw" {
        return queue_withdraw_durable(live, auth_ctx, body).await;
    }
    if body.kind == "message.queue_send_now" {
        return queue_send_now_durable(live, auth_ctx, body).await;
    }
    if body.kind == "conversation.archive" {
        return archive_durable(live, auth_ctx, body).await;
    }
    if body.kind == "conversation.restore" {
        return restore_durable(live, auth_ctx, body).await;
    }
    if body.kind == "discussion.open" {
        return discuss_durable(live, auth_ctx, body).await;
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

/// Authoritative cited source record readback for a single source key in an open chat.
pub(crate) async fn cited_source(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    Path((workspace, chat, source_key)): Path<(String, String, String)>,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id,
        physician: ctx.identity.physician_id,
        workspace,
        chat,
    };
    let readback = match live.chat.store.reopen_conversation(&scope).await {
        Ok(Some(readback)) => readback,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!(%error, "physician cited source readback failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };
    let Some(src) = readback.sources.get(&source_key) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if src.source_id == "unavailable" {
        return (StatusCode::NOT_FOUND, Json(json!({ "detail": src.body }))).into_response();
    }
    Json(physician_wire::CitedSourceResponse {
        source: wire_cited_source(src),
    })
    .into_response()
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
    let delivery = payload.delivery;
    let input = local_product::runtime::Input {
        key: idempotency_key,
        text: payload.text,
        timezone: payload.physician_time_zone,
        scenario,
    };
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match chat
        .accept_with_delivery(&scope, &input, session_hash, delivery)
        .await
    {
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
    target: physician_wire::CitationTargetKind,
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
            target,
        },
    }
}

fn wire_segments_with_targets(
    segments: &[local_product::runtime::history::HistorySegment],
    targets: &[physician_wire::CitationTargetKind],
) -> Vec<physician_wire::MessageSegment> {
    let mut target_idx = 0usize;
    segments
        .iter()
        .map(|segment| match segment {
            local_product::runtime::history::HistorySegment::Text { .. } => {
                wire_segment(segment, Default::default())
            }
            local_product::runtime::history::HistorySegment::Citation { .. } => {
                let target = targets.get(target_idx).copied().unwrap_or_default();
                target_idx += 1;
                wire_segment(segment, target)
            }
        })
        .collect()
}

fn wire_cited_source(
    src: &local_product::runtime::history::CitedSourceReadback,
) -> physician_wire::CitedSourceWire {
    physician_wire::CitedSourceWire {
        source_key: src.source_key.clone(),
        number: src.number,
        source_label: src.source_label.clone(),
        title: src.title.clone(),
        record_type: (!src.record_type.is_empty()).then(|| src.record_type.clone()),
        recorded_at: src.recorded_at.clone(),
        record_status: src.record_status.clone(),
        quote: src.quote.clone(),
        body: src.body.clone(),
        readable_parts: src.readable_parts.clone(),
        fields: src.fields.clone(),
        raw_content: (!src.raw_content.is_empty()).then(|| src.raw_content.clone()),
        external_url: src.external_url.clone(),
        origin: src.origin,
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
            .map(|(key, src)| (key.clone(), wire_cited_source(src)))
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
            if readback.chat_withheld
                || readback
                    .withheld_work_ids
                    .contains(exchange.work_item_id.as_str())
            {
                exchange.steps = physician_wire::ExchangeSteps::Read(Vec::new());
            }
            if let physician_wire::AnswerState::Finished {
                text,
                response_id,
                segments,
                basis,
                newer_records,
                ..
            } = &mut exchange.answer
                && let Some(turn) = assistant_turns.get(response_id.as_str())
            {
                *text = turn.raw_text.clone();
                *segments = wire_segments_with_targets(&turn.segments, &turn.segment_targets);
                *basis = turn
                    .basis
                    .as_ref()
                    .map(|items| wire_segments_with_targets(items, &turn.basis_targets))
                    .filter(|items: &Vec<_>| !items.is_empty());
                *newer_records = turn.newer_records;
            }
        }
    }
    let feedback = match feedback {
        SnapshotFeedback::Loaded(_) => physician_wire::SnapshotFeedback::Loaded,
        SnapshotFeedback::Unavailable => physician_wire::SnapshotFeedback::Unavailable,
    };
    let queued = conversation
        .map(local_product::runtime::Conversation::queued_messages)
        .unwrap_or_default();
    let archived = conversation.is_some_and(|conv| conv.archived);
    Ok(ChatSnapshot {
        transcript,
        feedback,
        sources,
        queued,
        archived,
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
        .stop_chat(&scope, &key, session_hash, &payload.work_item_id)
        .await
    {
        Ok(stopped) => match stopped.outcome {
            local_product::runtime::Stopped::Refused => rejected("work.stop"),
            outcome => {
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
                    "returned_messages": stopped.returned_messages,
                }))
                .into_response()
            }
        },
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

async fn decide_recommendation_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    use local_product::runtime::care_plan::{CarePlanError, RecommendationDecision};
    use local_product::runtime::concerns::WorkspaceAccess;

    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|p| serde_json::from_value::<DecideRecommendationPayload>(p).ok())
    else {
        return rejected("plan.decide_recommendation");
    };
    if payload.patient_workspace_id.trim().is_empty() || payload.recommendation_id.trim().is_empty()
    {
        return rejected("plan.decide_recommendation");
    }
    let decision = match payload.decision.as_deref().map(str::trim) {
        None | Some("" | "approve" | "approved_into_plan" | "decided") => {
            RecommendationDecision::ApproveIntoPlan { note: payload.note }
        }
        Some("decline" | "declined") => RecommendationDecision::Decline { note: payload.note },
        Some(_) => return rejected("plan.decide_recommendation"),
    };
    let Ok(clock) = ctx
        .auth
        .store
        .practice_clock(&ctx.identity.practice_id)
        .await
    else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "outcome": "rejected",
                "message": rejection_message("plan.decide_recommendation"),
            })),
        )
            .into_response();
    };
    let decided_at = clock.now(crate::auth::now()) as i64;
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    let access = WorkspaceAccess {
        practice: &ctx.identity.practice_id,
        physician: &ctx.identity.physician_id,
        workspace: &payload.patient_workspace_id,
        session_hash: &session_hash,
    };
    let mut authority = crate::durable::RuntimeAuthority;
    match live
        .chat
        .store
        .decide_medication_recommendation_for_physician(
            &access,
            &payload.recommendation_id,
            decision,
            decided_at,
            &mut authority,
        )
        .await
    {
        Ok(updated) => {
            if live
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
                "patient_workspace_id": updated.workspace,
                "recommendation_id": updated.recommendation_id,
                "standing": updated.standing,
            }))
            .into_response()
        }
        Err(CarePlanError::NotFound { .. } | CarePlanError::AccessDenied { .. }) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "outcome": "rejected",
                "message": "Recommendation not found.",
            })),
        )
            .into_response(),
        Err(CarePlanError::AlreadyDecided {
            current_standing, ..
        }) => (
            StatusCode::CONFLICT,
            Json(json!({
                "outcome": "rejected",
                "message": format!("Recommendation is already {current_standing}."),
            })),
        )
            .into_response(),
        Err(CarePlanError::InvalidInput(_)) => rejected("plan.decide_recommendation"),
        Err(CarePlanError::Database(_) | CarePlanError::CorruptRow { .. }) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "outcome": "rejected",
                "message": rejection_message("plan.decide_recommendation"),
            })),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct ArchivePayload {
    patient_workspace_id: String,
    chat_id: String,
}

#[derive(Deserialize)]
struct DiscussPayload {
    patient_workspace_id: String,
    chat_id: String,
    #[serde(default)]
    work_item_id: Option<String>,
}

async fn archive_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|value| serde_json::from_value::<ArchivePayload>(value).ok())
    else {
        return rejected("conversation.archive");
    };
    let Some(key) = body.idempotency_key.filter(|k| !k.trim().is_empty()) else {
        return rejected("conversation.archive");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match live
        .chat
        .archive_conversation(&scope, &key, session_hash)
        .await
    {
        Ok(local_product::runtime::ArchiveOutcome::RefusedBusyOrInvalid) => {
            rejected("conversation.archive")
        }
        Ok(outcome) => {
            if outcome == local_product::runtime::ArchiveOutcome::Archived
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
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "outcome": "rejected",
                "message": rejection_message("conversation.archive"),
            })),
        )
            .into_response(),
    }
}

async fn restore_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|value| serde_json::from_value::<ArchivePayload>(value).ok())
    else {
        return rejected("conversation.restore");
    };
    let Some(key) = body.idempotency_key.filter(|k| !k.trim().is_empty()) else {
        return rejected("conversation.restore");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match live
        .chat
        .restore_conversation(&scope, &key, session_hash)
        .await
    {
        Ok(local_product::runtime::RestoreOutcome::Refused) => rejected("conversation.restore"),
        Ok(outcome) => {
            if outcome == local_product::runtime::RestoreOutcome::Restored
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
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "outcome": "rejected",
                "message": rejection_message("conversation.restore"),
            })),
        )
            .into_response(),
    }
}

async fn discuss_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|value| serde_json::from_value::<DiscussPayload>(value).ok())
    else {
        return rejected("discussion.open");
    };
    if payload.patient_workspace_id.trim().is_empty() || payload.chat_id.trim().is_empty() {
        return rejected("discussion.open");
    }
    let Some(key) = body.idempotency_key.filter(|k| !k.trim().is_empty()) else {
        return rejected("discussion.open");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let mut returned_messages = Vec::new();
    let work_to_stop = match payload
        .work_item_id
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(work) => Some(work.to_owned()),
        None => match live.chat.store.read(&scope).await {
            Ok(Some(conv)) if conv.status.running() && !conv.work.trim().is_empty() => {
                Some(conv.work)
            }
            Ok(Some(_)) => None,
            Ok(None) => return rejected("discussion.open"),
            Err(_) => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({
                        "outcome": "rejected",
                        "message": rejection_message("discussion.open"),
                    })),
                )
                    .into_response();
            }
        },
    };
    if let Some(work_item_id) = work_to_stop {
        let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
        match live
            .chat
            .stop_chat(&scope, &key, session_hash, &work_item_id)
            .await
        {
            Ok(stopped) => {
                returned_messages = stopped.returned_messages;
                if stopped.outcome == local_product::runtime::Stopped::Held
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
            }
            Err(_) => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({
                        "outcome": "rejected",
                        "message": rejection_message("discussion.open"),
                    })),
                )
                    .into_response();
            }
        }
    }
    Json(json!({
        "outcome": "accepted",
        "patient_workspace_id": payload.patient_workspace_id,
        "chat_id": payload.chat_id,
        "returned_messages": returned_messages,
    }))
    .into_response()
}

async fn queue_mutation_response(
    live: &Live,
    workspace_id: &str,
    chat_id: &str,
    message_id: &str,
    command_type: &str,
    result: Result<local_product::runtime::QueueMutation, String>,
) -> Response {
    match result {
        Ok(local_product::runtime::QueueMutation::Refused) => rejected(command_type),
        Ok(outcome) => {
            if outcome == local_product::runtime::QueueMutation::Applied
                && live
                    .bus
                    .publish(Event::WorkspaceChanged {
                        workspace_id: workspace_id.to_owned(),
                    })
                    .await
                    .is_err()
            {
                tracing::warn!("physician workspace invalidation not published");
            }
            Json(json!({
                "outcome": "accepted",
                "patient_workspace_id": workspace_id,
                "chat_id": chat_id,
                "message_id": message_id,
            }))
            .into_response()
        }
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "outcome": "rejected",
                "message": rejection_message(command_type),
            })),
        )
            .into_response(),
    }
}

async fn queue_edit_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|value| serde_json::from_value::<QueueEditPayload>(value).ok())
    else {
        return rejected("message.queue_edit");
    };
    let Some(key) = body.idempotency_key.filter(|key| !key.trim().is_empty()) else {
        return rejected("message.queue_edit");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let result = live
        .chat
        .edit_queued_message(&scope, &key, &payload.message_id, &payload.text)
        .await;
    queue_mutation_response(
        &live,
        &payload.patient_workspace_id,
        &payload.chat_id,
        &payload.message_id,
        "message.queue_edit",
        result,
    )
    .await
}

async fn queue_withdraw_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|value| serde_json::from_value::<QueueWithdrawPayload>(value).ok())
    else {
        return rejected("message.withdraw");
    };
    let Some(key) = body.idempotency_key.filter(|key| !key.trim().is_empty()) else {
        return rejected("message.withdraw");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let result = live
        .chat
        .withdraw_queued_message(&scope, &key, &payload.message_id)
        .await;
    queue_mutation_response(
        &live,
        &payload.patient_workspace_id,
        &payload.chat_id,
        &payload.message_id,
        "message.withdraw",
        result,
    )
    .await
}

async fn queue_send_now_durable(
    live: Arc<Live>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: CommandBody,
) -> Response {
    let Some(Extension(ctx)) = auth_ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(payload) = body
        .payload
        .and_then(|value| serde_json::from_value::<QueueSendNowPayload>(value).ok())
    else {
        return rejected("message.queue_send_now");
    };
    let Some(key) = body.idempotency_key.filter(|key| !key.trim().is_empty()) else {
        return rejected("message.queue_send_now");
    };
    let scope = local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: payload.patient_workspace_id.clone(),
        chat: payload.chat_id.clone(),
    };
    let session_hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    let result = live
        .chat
        .send_queued_now(
            &scope,
            &key,
            session_hash,
            &payload.message_id,
            &payload.physician_time_zone,
        )
        .await;
    queue_mutation_response(
        &live,
        &payload.patient_workspace_id,
        &payload.chat_id,
        &payload.message_id,
        "message.queue_send_now",
        result,
    )
    .await
}

fn rejected(kind: &str) -> Response {
    Json(json!({
        "outcome": "rejected",
        "message": rejection_message(kind),
    }))
    .into_response()
}

/// The practice's change stream over the bus: a `workspace_changed` or
/// `resync` frame for each change. The body stays open. A client refetches
/// what it shows on `resync` (bus lag or a Redis reconnect).
pub(crate) async fn events(State(bus): State<Arc<dyn Bus>>) -> Response {
    let changes = stream::unfold(bus.subscribe(), |mut rx| async move {
        let change = rx.recv().await.ok()?;
        Some((change, rx))
    });
    change_stream(changes)
}

/// How often the change stream sends its keep-alive comment: 15 s, axum's `KeepAlive` default
/// (docs.rs, `KeepAlive::new`: "Default is 15 seconds"), ASP.NET SignalR's `KeepAliveInterval`
/// default, and sse-starlette's `DEFAULT_PING_INTERVAL` (3.7.0), whose `_ping` loop sleeps that
/// long and sends a comment, whatever else the stream sends.
pub(crate) const KEEP_ALIVE_EVERY: Duration = Duration::from_secs(15);

/// The change stream's response for `changes`: one frame for each change, and an empty comment
/// every [`KEEP_ALIVE_EVERY`] from the moment it opens, so a client can tell a quiet stream from
/// a connection that died without closing. The beat is fixed, not reset by a change: the sign-in
/// layer drops, after they were framed, the changes a reader may not see, so a change sent here
/// may never reach the reader. The stream ends when `changes` ends.
pub(crate) fn change_stream(changes: impl Stream<Item = Event> + Send + 'static) -> Response {
    // A reader slow to take bytes misses beats; `Delay` sends one comment when it takes bytes
    // again and the next a full interval later, where tokio's default `Burst` would send every
    // missed one at once. sse-starlette's `_ping` loop does the same: it sleeps after each send.
    let mut beat = interval_at(Instant::now() + KEEP_ALIVE_EVERY, KEEP_ALIVE_EVERY);
    beat.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let frames = stream::unfold(
        (Box::pin(changes), beat),
        |(mut changes, mut beat)| async move {
            let frame = tokio::select! {
                // The beat first. For a reader that takes frames faster than one an interval,
                // the beat cannot hold changes back, because it is due at most once an interval;
                // changes that stay ready would hold a due beat back without end. The stream
                // still ends at the first poll of `changes` after they end.
                biased;
                // The empty comment axum's own `KeepAlive` sends.
                _ = beat.tick() => sse::Event::DEFAULT_KEEP_ALIVE,
                change = changes.next() => change_frame(change?),
            };
            Some((Ok::<_, Infallible>(frame), (changes, beat)))
        },
    );
    Sse::new(frames).into_response()
}

/// The frame for one change.
fn change_frame(change: Event) -> sse::Event {
    match change {
        Event::WorkspaceChanged { workspace_id } | Event::WorkspaceSettled { workspace_id } => {
            sse::Event::default()
                .event("workspace_changed")
                .data(json!({ "patient_workspace_id": workspace_id }).to_string())
        }
        Event::Resync => sse::Event::default().event("resync").data("{}"),
    }
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

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use harness::engine::results::ToolResult;
    use harness::engine::runs::{RunHistory, RunOutcome};
    use harness::engine::{Event, EventKind, Invocation};
    use local_product::physician_sources::SEARCH_TOOL;
    use local_product::runtime::history::{
        HistoryOutcome, HistorySegment, SavedConversationReadback, SavedTurnReadback, TurnAuthor,
    };
    use local_product::runtime::{
        AssistantKind, Attempt, AttemptStatus, Conversation, Message, MessageRole, WorkStatus,
    };
    use physician_wire::{AnswerState, ExchangeSteps};

    use super::{SnapshotFeedback, snapshot_body};

    #[test]
    fn snapshot_body_clears_steps_and_syncs_redacted_text_for_withheld_work() {
        let mut runs_withheld = RunHistory::default();
        runs_withheld.start("run-withheld".into()).unwrap();
        runs_withheld.end(RunOutcome::Completed).unwrap();

        let mut runs_clean = RunHistory::default();
        runs_clean.start("run-clean".into()).unwrap();
        runs_clean.end(RunOutcome::Completed).unwrap();

        let search_call = Invocation {
            id: "call-1".into(),
            name: SEARCH_TOOL.into(),
            arguments: r#"{"query":"secret query"}"#.into(),
        };
        let search_result = ToolResult::Original(r#"{"results":[{}],"next":null}"#.into());

        let saved_at: local_product::runtime::SavedAt =
            serde_json::from_value(serde_json::json!(1_794_400_000)).unwrap();
        let conversation = Conversation {
            messages: vec![
                Message {
                    role: MessageRole::User {
                        id: "msg-u1".into(),
                        work: "work-withheld".into(),
                        revision: 1,
                    },
                    text: "Check message".into(),
                    saved_at,
                },
                Message {
                    role: MessageRole::Assistant {
                        id: "att-1:reply".into(),
                        kind: AssistantKind::Finished(
                            local_product::physician_sources::citations::Finished::redacted(),
                        ),
                    },
                    text: "stale unredacted text".into(),
                    saved_at,
                },
                Message {
                    role: MessageRole::User {
                        id: "msg-u2".into(),
                        work: "work-clean".into(),
                        revision: 2,
                    },
                    text: "Check clean".into(),
                    saved_at,
                },
                Message {
                    role: MessageRole::Assistant {
                        id: "att-2:reply".into(),
                        kind: AssistantKind::Finished(
                            local_product::physician_sources::citations::Finished::redacted(),
                        ),
                    },
                    text: "Clean answer".into(),
                    saved_at,
                },
            ],
            revision: 2,
            status: WorkStatus::Completed,
            work: "work-clean".into(),
            runs: BTreeMap::from([
                ("work-withheld".into(), runs_withheld),
                ("work-clean".into(), runs_clean),
            ]),
            skills: serde_json::from_value(serde_json::json!("pkg-test")).unwrap(),
            attempts: vec![
                Attempt {
                    id: "att-1".into(),
                    predecessor: None,
                    work: "work-withheld".into(),
                    revision: 1,
                    status: AttemptStatus::Completed,
                    expires: 0,
                    events: vec![
                        Event {
                            attempt_id: "att-1".into(),
                            sequence: 1,
                            ordinal: 0,
                            kind: EventKind::ToolStarted(search_call.clone()),
                        },
                        Event {
                            attempt_id: "att-1".into(),
                            sequence: 2,
                            ordinal: 0,
                            kind: EventKind::ToolReturned {
                                call_id: "call-1".into(),
                                result: search_result.clone(),
                            },
                        },
                    ],
                },
                Attempt {
                    id: "att-2".into(),
                    predecessor: None,
                    work: "work-clean".into(),
                    revision: 2,
                    status: AttemptStatus::Completed,
                    expires: 0,
                    events: vec![
                        Event {
                            attempt_id: "att-2".into(),
                            sequence: 1,
                            ordinal: 0,
                            kind: EventKind::ToolStarted(Invocation {
                                id: "call-2".into(),
                                name: SEARCH_TOOL.into(),
                                arguments: r#"{"query":"bp"}"#.into(),
                            }),
                        },
                        Event {
                            attempt_id: "att-2".into(),
                            sequence: 2,
                            ordinal: 0,
                            kind: EventKind::ToolReturned {
                                call_id: "call-2".into(),
                                result: search_result,
                            },
                        },
                    ],
                },
            ],
            session_hash: None,
            dispatch_after: 0,
            origin: None,
            failures: 0,
            budget_resume_after: 0,
            bindings: BTreeMap::new(),
            opened_at: 1,
            last_activity_at: 2,
            scenario: None,
            queued: Vec::new(),
            stopped_queue: None,
            archived: false,
        };

        let redacted =
            "Unavailable: patient message content was withdrawn after a wrong-patient link correction."
                .to_owned();
        let readback = SavedConversationReadback {
            outcome: HistoryOutcome::Answered,
            work_id: "work-clean".into(),
            turns: vec![
                SavedTurnReadback {
                    id: "att-1:reply".into(),
                    author: TurnAuthor::Assistant { partial: false },
                    raw_text: redacted.clone(),
                    segments: vec![HistorySegment::Text {
                        text: redacted.clone(),
                    }],
                    segment_targets: Vec::new(),
                    basis: None,
                    basis_targets: Vec::new(),
                    newer_records: false,
                },
                SavedTurnReadback {
                    id: "att-2:reply".into(),
                    author: TurnAuthor::Assistant { partial: false },
                    raw_text: "Clean answer".into(),
                    segments: vec![HistorySegment::Text {
                        text: "Clean answer".into(),
                    }],
                    segment_targets: Vec::new(),
                    basis: None,
                    basis_targets: Vec::new(),
                    newer_records: false,
                },
            ],
            sources: BTreeMap::new(),
            workspace_source_version: 1,
            chat_withheld: false,
            withheld_work_ids: BTreeSet::from(["work-withheld".to_owned()]),
        };

        let snap = snapshot_body(
            Some(&conversation),
            Some(&readback),
            &SnapshotFeedback::Loaded(BTreeMap::new()),
        )
        .expect("snapshot_body");
        assert_eq!(snap.transcript.len(), 2);
        assert_eq!(snap.transcript[0].steps, ExchangeSteps::Read(Vec::new()));
        let AnswerState::Finished { text, .. } = &snap.transcript[0].answer else {
            panic!("expected Finished on withheld exchange");
        };
        assert_eq!(text, &redacted);

        let ExchangeSteps::Read(clean_steps) = &snap.transcript[1].steps else {
            panic!("expected Read steps on clean exchange");
        };
        assert_eq!(clean_steps.len(), 1);
    }
}
