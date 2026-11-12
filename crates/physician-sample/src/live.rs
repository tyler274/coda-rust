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
    Json(json!({
        "snapshot": durable_snapshot_body_with_readback(
            conversation.as_ref(),
            readback.as_ref(),
            &feedback,
        )
    }))
    .into_response()
}

/// The physician's feedback on a chat's responses, as the snapshot read it.
enum SnapshotFeedback {
    /// Each rated response's saved feedback, by response.
    Loaded(BTreeMap<local_product::runtime::ResponseId, Feedback>),
    /// The read failed; no response carries feedback.
    Unavailable,
}

/// Who wrote a turn, as the snapshot names it: Coda's turns are the model's, every other the physician's.
fn turn_kind(role: &str) -> &'static str {
    match role {
        "assistant" => "model",
        "mechanical" => "mechanical",
        _ => "physician",
    }
}

/// The reference `SnapshotResponse` subset this site owns. A failed turn adds
/// its run and a mechanical "couldn't finish" line whose `resume_target` names
/// the exact work and run Try again resumes. A running turn offers Stop. A
/// held turn offers Resume. An answered turn names its run and the attempt
/// that answered it, so an operator can match the reply to its telemetry by
/// attempt id. Each of Coda's published replies carries `response_id`, its
/// saved id, which stays its name across restarts, and the physician's saved
/// `feedback` on it, if they rated it; a partial answer carries neither. The
/// physician's text stays saved. When the feedback read failed, the snapshot
/// says `"feedback": "unavailable"` and no turn carries feedback.
#[cfg(test)]
fn durable_snapshot_body(
    conversation: Option<&local_product::runtime::Conversation>,
    feedback: &SnapshotFeedback,
) -> Value {
    durable_snapshot_body_with_readback(conversation, None, feedback)
}

fn durable_snapshot_body_with_readback(
    conversation: Option<&local_product::runtime::Conversation>,
    readback: Option<&local_product::runtime::history::SavedConversationReadback>,
    feedback: &SnapshotFeedback,
) -> Value {
    let (saved, mut body) = match feedback {
        SnapshotFeedback::Loaded(saved) => (Some(saved), json!({})),
        SnapshotFeedback::Unavailable => (
            None,
            json!({ "feedback": physician_wire::SnapshotFeedback::Unavailable }),
        ),
    };
    let (conversation, runs) = match conversation {
        Some(conversation) => snapshot_turns(conversation, readback, saved),
        None => (Vec::new(), Vec::new()),
    };
    body["conversation"] = json!(conversation);
    body["runs"] = json!(runs);
    if let Some(readback) = readback
        && !readback.sources.is_empty()
    {
        let sources: BTreeMap<String, physician_wire::CitedSourceWire> = readback
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
        body["sources"] = json!(sources);
    }
    body
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

/// A saved conversation's turns and its latest run, as the snapshot names
/// them, with `feedback` on each rated response when it was read.
fn snapshot_turns(
    conversation: &local_product::runtime::Conversation,
    readback: Option<&local_product::runtime::history::SavedConversationReadback>,
    feedback: Option<&BTreeMap<local_product::runtime::ResponseId, Feedback>>,
) -> (Vec<Value>, Vec<Value>) {
    let responses: BTreeMap<_, _> = conversation
        .responses()
        .map(|response| (response.place, response.id))
        .collect();
    let mut items: Vec<Value> = conversation
        .messages
        .iter()
        .enumerate()
        .flat_map(|(place, message)| {
            let mut authored =
                json!({ "kind": turn_kind(message.role.as_str()), "text": message.text });
            if let Some(id) = responses.get(&place) {
                authored["response_id"] = json!(id);
                if let Some(saved) = feedback.and_then(|feedback| feedback.get(id)) {
                    authored["feedback"] = json!(crate::feedback::wire(saved.clone()));
                }
            }
            if let Some(turn) = readback.and_then(|saved| saved.turns.get(place)) {
                let segments: Vec<physician_wire::MessageSegment> =
                    turn.segments.iter().map(wire_segment).collect();
                if !segments.is_empty() {
                    authored["segments"] = json!(segments);
                }
                if let Some(basis) = &turn.basis {
                    let basis_segments: Vec<physician_wire::MessageSegment> =
                        basis.iter().map(wire_segment).collect();
                    if !basis_segments.is_empty() {
                        authored["basis"] = json!(basis_segments);
                    }
                }
                if turn.newer_records {
                    authored["newer_records"] = json!(true);
                }
            }
            let partial = message.partial().then(|| {
                json!({
                    "kind": "mechanical", "text": "Partial response.",
                })
            });
            std::iter::once(authored).chain(partial)
        })
        .collect();
    let mut runs = Vec::new();
    use harness::engine::runs::{HoldReason, RunOutcome};
    if let Some(run) = conversation.run() {
        match run.outcome {
            Some(outcome @ (RunOutcome::Held { .. } | RunOutcome::Failed)) => {
                let reason = match outcome {
                    RunOutcome::Held { reason } => Some(reason),
                    _ => None,
                };
                let text = match reason {
                    Some(HoldReason::Stopped) => STOPPED,
                    Some(HoldReason::Limit)
                        if conversation
                            .messages
                            .last()
                            .is_some_and(|message| message.partial()) =>
                    {
                        "Partial response. Choose Try again to continue."
                    }
                    Some(HoldReason::Limit) => {
                        "Coda reached its limit for this response. Choose Try again to continue."
                    }
                    Some(HoldReason::ProviderBlocked) => {
                        "Coda couldn’t respond right now. Your message is saved."
                    }
                    None => COULDNT_FINISH,
                };
                runs.push(
                    json!({ "run_id": run.id, "state": outcome.status(), "hold_reason": reason }),
                );
                let disposition = resume_line(text, conversation, run.id);
                if conversation
                    .messages
                    .last()
                    .is_some_and(|message| message.partial())
                {
                    *items.last_mut().unwrap() = disposition;
                } else {
                    items.push(disposition);
                }
            }
            None if conversation.status.running() => {
                items.push(json!({
                    "kind": "mechanical",
                    "text": WRITING,
                    "stop_target": { "work_item_id": conversation.work },
                }));
            }
            Some(outcome @ RunOutcome::Completed) => {
                if let Some(attempt) = conversation.attempts.last() {
                    runs.push(json!({
                        "run_id": run.id,
                        "state": outcome.status(),
                        "hold_reason": null,
                        "attempt_id": attempt.id,
                    }));
                }
            }
            Some(RunOutcome::Cancelled) | None => {}
        }
    }
    (items, runs)
}

fn resume_line(
    text: &str,
    conversation: &local_product::runtime::Conversation,
    run_id: &str,
) -> Value {
    json!({
        "kind": "mechanical",
        "text": text,
        "resume_target": {
            "work_item_id": conversation.work,
            "expected_run_id": run_id,
        },
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

#[cfg(test)]
mod durable_snapshot_tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::{COULDNT_FINISH, STOPPED, SnapshotFeedback, WRITING, durable_snapshot_body};
    use local_product::runtime::feedback::{Details, Feedback, Rating, Reason};
    use local_product::runtime::{
        Attempt, AttemptStatus, Conversation, Message, ResponseId, WorkStatus,
    };
    use serde_json::Value;

    /// The snapshot of a chat with no feedback.
    fn snapshot(conversation: Option<&Conversation>) -> Value {
        durable_snapshot_body(conversation, &SnapshotFeedback::Loaded(BTreeMap::new()))
    }
    fn conversation(status: WorkStatus) -> Conversation {
        use harness::engine::runs::{HoldReason, RunHistory, RunOutcome};
        let mut runs = RunHistory::default();
        runs.start("run-1".into()).unwrap();
        match status {
            WorkStatus::Failed => runs.end(RunOutcome::Failed).unwrap(),
            WorkStatus::Held => runs
                .end(RunOutcome::Held {
                    reason: HoldReason::Stopped,
                })
                .unwrap(),
            WorkStatus::Completed => runs.end(RunOutcome::Completed).unwrap(),
            WorkStatus::Pending | WorkStatus::Active | WorkStatus::Cancelled => {}
        }
        Conversation {
            runs: [("work-1".into(), runs)].into(),
            messages: vec![Message {
                role: local_product::runtime::MessageRole::User {
                    id: "input-1".into(),
                    work: "work-1".into(),
                    revision: 1,
                },
                text: "Hello".into(),
            }],
            status,
            work: "work-1".into(),
            attempts: vec![Attempt {
                id: "attempt-1".into(),
                predecessor: None,
                work: "work-1".into(),
                revision: 1,
                status: AttemptStatus::Interrupted,
                expires: 0,
                events: vec![],
            }],
            revision: 1,
            skills: serde_json::from_value(serde_json::json!("fixture-package")).unwrap(),
            session_hash: None,
            dispatch_after: 0,
            failures: 0,
            budget_resume_after: 0,
            bindings: Default::default(),
            opened_at: 0,
            last_activity_at: 0,
            scenario: None,
        }
    }

    #[test]
    fn held_status_uses_actual_partial_state_and_hides_provider_details() {
        use harness::engine::runs::{HoldReason, RunOutcome};
        for reason in [HoldReason::Limit, HoldReason::ProviderBlocked] {
            for partial in [false, true] {
                let mut saved = conversation(WorkStatus::Active);
                saved.status = WorkStatus::Held;
                saved
                    .runs
                    .get_mut("work-1")
                    .unwrap()
                    .end(RunOutcome::Held { reason })
                    .unwrap();
                if partial {
                    saved.messages.push(Message {
                        role: local_product::runtime::MessageRole::Assistant {
                            id: "partial-1".into(),
                            partial: true,
                        },
                        text: "The saved part of the answer.".into(),
                    });
                }
                let raw = "internal_provider_error: account billing identifier 739";
                saved.attempts[0].events.push(harness::engine::Event {
                    attempt_id: "attempt-1".into(),
                    sequence: 1,
                    ordinal: 1,
                    kind: harness::engine::EventKind::ProviderBlocked { reason: raw.into() },
                });
                let body = snapshot(Some(&saved));
                let lines = body["conversation"].as_array().unwrap();
                let status = lines.last().unwrap()["text"].as_str().unwrap();
                assert!(
                    !body.to_string().contains(raw),
                    "provider details are not product copy"
                );
                if reason == HoldReason::Limit {
                    assert_eq!(
                        status.contains("Partial response"),
                        partial,
                        "limit wording must reflect an actual partial message"
                    );
                    assert!(status.contains("Try again"));
                } else {
                    assert_eq!(
                        status,
                        "Coda couldn’t respond right now. Your message is saved."
                    );
                }
                assert_eq!(
                    lines.last().unwrap()["resume_target"]["expected_run_id"],
                    "run-1"
                );
                assert_eq!(lines.len(), if partial { 3 } else { 2 });
            }
        }
    }

    #[test]
    fn a_failed_turn_keeps_the_message_and_names_the_run_to_resume() {
        let body = snapshot(Some(&conversation(WorkStatus::Failed)));
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
        for status in [WorkStatus::Pending, WorkStatus::Active] {
            let body = snapshot(Some(&conversation(status)));
            let items = body["conversation"].as_array().unwrap();
            assert_eq!(items.len(), 2, "{status:?}");
            assert_eq!(items[1]["text"], WRITING, "{status:?}");
            assert_eq!(
                items[1]["stop_target"]["work_item_id"], "work-1",
                "{status:?}"
            );
            assert!(items[1].get("resume_target").is_none(), "{status:?}");
            assert!(body["runs"].as_array().unwrap().is_empty(), "{status:?}");
        }
        let held = snapshot(Some(&conversation(WorkStatus::Held)));
        let items = held["conversation"].as_array().unwrap();
        assert_eq!(items[1]["text"], STOPPED);
        assert_eq!(items[1]["resume_target"]["work_item_id"], "work-1");
        assert_eq!(items[1]["resume_target"]["expected_run_id"], "run-1");
        assert!(items[1].get("stop_target").is_none());
        assert_eq!(held["runs"][0]["state"], "held");
        let done = snapshot(Some(&conversation(WorkStatus::Completed)));
        assert_eq!(done["conversation"].as_array().unwrap().len(), 1);
        assert_eq!(
            done["runs"],
            serde_json::json!([{
                "run_id": "run-1",
                "state": "completed",
                "hold_reason": null,
                "attempt_id": "attempt-1",
            }])
        );
        let empty = snapshot(None);
        assert!(empty["conversation"].as_array().unwrap().is_empty());
    }

    /// A completed chat whose two assistant messages are a partial answer
    /// (`attempt-0:reply`) and then the published reply (`attempt-1:reply`).
    fn answered() -> Conversation {
        use local_product::runtime::MessageRole;
        let mut answered = conversation(WorkStatus::Completed);
        for (id, partial, text) in [
            ("attempt-0:reply", true, "The saved part."),
            ("attempt-1:reply", false, "Sodium is stable."),
        ] {
            answered.messages.push(Message {
                role: MessageRole::Assistant {
                    id: id.into(),
                    partial,
                },
                text: text.into(),
            });
        }
        answered
    }

    #[test]
    fn each_published_reply_carries_its_response_id() {
        let body = snapshot(Some(&answered()));
        let items = body["conversation"].as_array().unwrap();
        let named: Vec<_> = items
            .iter()
            .map(|item| (item["kind"].as_str(), item.get("response_id")))
            .collect();
        let published = serde_json::json!("attempt-1:reply");
        assert_eq!(
            named,
            [
                (Some("physician"), None),
                (Some("model"), None),
                (Some("mechanical"), None),
                (Some("model"), Some(&published)),
            ],
            "only the published reply is a response: {body}"
        );
        assert!(items[3].get("feedback").is_none(), "unrated: {body}");
    }

    #[test]
    fn a_rated_reply_carries_its_saved_feedback() {
        let details = Details::new(BTreeSet::from([Reason::MissingContext]), "Trend?".into());
        let unrated = Feedback {
            rating: Rating::Up,
            details: None,
            saved_at: 10,
            revision: 1,
        };
        let feedback = BTreeMap::from([
            (
                ResponseId::from("attempt-1:reply".to_owned()),
                Feedback {
                    rating: Rating::Down,
                    details,
                    saved_at: 10,
                    revision: 3,
                },
            ),
            // Feedback naming a partial answer, or a response this chat
            // doesn't hold, lands nowhere.
            (
                ResponseId::from("attempt-0:reply".to_owned()),
                unrated.clone(),
            ),
            (ResponseId::from("attempt-9:reply".to_owned()), unrated),
        ]);
        let body = durable_snapshot_body(Some(&answered()), &SnapshotFeedback::Loaded(feedback));
        let items = body["conversation"].as_array().unwrap();
        let carried: Vec<_> = items.iter().map(|item| item.get("feedback")).collect();
        let rated = serde_json::json!({
            "revision": 3,
            "rating": "down",
            "details": { "reasons": ["missing_context"], "text": "Trend?" },
        });
        assert_eq!(carried, [None, None, None, Some(&rated)], "{body}");
        assert!(
            body.get("feedback").is_none(),
            "loaded is the default: {body}"
        );
    }

    /// A failed feedback read still sends the whole conversation, marked so
    /// the page can tell its responses' ratings are missing, not unrated.
    #[test]
    fn unavailable_feedback_keeps_the_conversation_and_says_so() {
        use physician_wire::{ChatSnapshotResponse, SnapshotFeedback as Wire};
        let body = durable_snapshot_body(Some(&answered()), &SnapshotFeedback::Unavailable);
        let loaded = snapshot(Some(&answered()));
        assert_eq!(body["conversation"], loaded["conversation"], "{body}");
        assert_eq!(body["runs"], loaded["runs"], "{body}");
        assert_eq!(body["feedback"], "unavailable", "{body}");
        let read: ChatSnapshotResponse =
            serde_json::from_value(serde_json::json!({ "snapshot": body })).unwrap();
        assert_eq!(read.snapshot.feedback, Wire::Unavailable);
        assert_eq!(
            read.snapshot.conversation[3].body(),
            Some("Sodium is stable.")
        );
        assert!(
            read.snapshot
                .conversation
                .iter()
                .all(|turn| turn.feedback.is_none())
        );
        let empty = durable_snapshot_body(None, &SnapshotFeedback::Unavailable);
        assert_eq!(
            empty,
            serde_json::json!({ "feedback": "unavailable", "conversation": [], "runs": [] })
        );
    }

    /// The wire crate keeps its own outcome and hold reason types. A run
    /// outcome the app cannot read fails the whole snapshot; a hold reason it
    /// does not know reads as `Unrecognized`. Each reason the site writes
    /// must map to its own wire variant, never to `Unrecognized`.
    #[test]
    fn the_app_reads_every_run_outcome_the_site_writes() {
        use harness::engine::runs::{HoldReason, RunOutcome};
        use physician_wire::{ChatRun, ChatRunOutcome, ChatSnapshotResponse};
        let held = [
            HoldReason::Stopped,
            HoldReason::Limit,
            HoldReason::ProviderBlocked,
        ]
        .map(|reason| RunOutcome::Held { reason });
        let ended = [
            RunOutcome::Completed,
            RunOutcome::Failed,
            RunOutcome::Cancelled,
        ];
        for outcome in held.into_iter().chain(ended) {
            // A new outcome or hold reason fails to compile here until this
            // match maps it. Nothing forces it into the lists above; add it.
            let expected = match outcome {
                RunOutcome::Held { reason } => Some(ChatRunOutcome::Held {
                    hold_reason: match reason {
                        HoldReason::Stopped => physician_wire::HoldReason::Stopped,
                        HoldReason::Limit => physician_wire::HoldReason::Limit,
                        HoldReason::ProviderBlocked => physician_wire::HoldReason::ProviderBlocked,
                    },
                }),
                RunOutcome::Failed => Some(ChatRunOutcome::Failed),
                RunOutcome::Completed => Some(ChatRunOutcome::Completed {
                    attempt_id: "attempt-1".into(),
                }),
                // A cancelled run is not named in the snapshot.
                RunOutcome::Cancelled => None,
            };
            let mut saved = conversation(WorkStatus::Active);
            saved.status = outcome.into();
            saved.runs.get_mut("work-1").unwrap().end(outcome).unwrap();
            let body = serde_json::json!({ "snapshot": snapshot(Some(&saved)) });
            let read: ChatSnapshotResponse = serde_json::from_value(body)
                .unwrap_or_else(|error| panic!("{outcome:?}: the app cannot read it: {error}"));
            let expected: Vec<_> = expected
                .into_iter()
                .map(|outcome| ChatRun {
                    run_id: "run-1".into(),
                    outcome,
                })
                .collect();
            assert_eq!(read.snapshot.runs, expected, "{outcome:?}");
        }
    }
}
