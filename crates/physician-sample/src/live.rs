//! Physician chat HTTP composition over the durable Postgres owner.
//! The site accepts work and reads its state; the worker runs the shared loop.

use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::body::Body;
use axum::extract::{Extension, Path, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use futures_util::stream;
use local_product::physician_layouts::{LayoutKey, LayoutStoreError, StoredLayout};
use physician_bus::{Bus, Event};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::auth::AuthContext;
use crate::rejection_message;
use crate::source_layout::Layout;

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

    /// The physician's retained chats in this patient workspace.
    pub(crate) async fn chats(
        &self,
        ctx: Option<&AuthContext>,
        workspace: &str,
    ) -> Result<Vec<Value>, ()> {
        let ctx = ctx.ok_or(())?;
        let chats = self
            .chat
            .store
            .chats(
                &ctx.identity.practice_id,
                &ctx.identity.physician_id,
                workspace,
            )
            .await
            .map_err(|_| ())?;
        Ok(chats.into_iter().map(|chat| {
            let created_at = crate::timestamp::format_rfc3339(
                std::time::UNIX_EPOCH + std::time::Duration::from_secs(chat.opened_at.max(0) as u64),
            );
            json!({"chat_id": chat.chat, "title": chat_title(&chat.first_message), "created_at": created_at})
        }).collect())
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

/// Accepts chat, Stop/Resume and reply-revision commands into the durable owner.
/// Every other command is rejected.
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
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let Ok(sources_version) = chat.store.source_version(&scope.workspace).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Ok(restricted) = chat.store.restricted_sources(&scope.workspace).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let layouts = match &conversation {
        Some(conversation) => cited_layouts(&chat.store, conversation).await,
        None => CitedLayouts::new(),
    };
    Json(json!({
        "snapshot": durable_snapshot_body(
            conversation.as_ref(),
            sources_version,
            &layouts,
            &restricted,
        )
    }))
    .into_response()
}

/// The stored layout of each `(source, record type)` a conversation cites, keyed the same way.
type CitedLayouts = HashMap<(String, String), Layout>;

/// Reads the layout of every source and record type the conversation cites, once each. A key
/// with no usable layout is left out, and its records read through the general view.
pub(crate) async fn cited_layouts(
    store: &local_product::runtime::Store,
    conversation: &local_product::runtime::Conversation,
) -> CitedLayouts {
    let mut layouts = CitedLayouts::new();
    let mut read = HashSet::new();
    for citation in conversation.messages.iter().flat_map(|m| &m.citations) {
        let Some(source_id) = &citation.source_id else {
            continue;
        };
        let key = (source_id.clone(), citation.record_type.clone());
        if !read.insert(key.clone()) {
            continue;
        }
        let stored = store
            .layout_for(LayoutKey {
                source_id,
                record_type: &citation.record_type,
            })
            .await;
        if let Some(layout) = usable_layout(&key, stored) {
            layouts.insert(key, layout);
        }
    }
    layouts
}

/// The layout a stored read yields, if any. A layout is presentation over a record the general
/// view already shows in full, so a failed read or an unparseable layout falls back to that view
/// rather than withholding the conversation; both are logged with their cause, so neither passes
/// for a key with no layout.
fn usable_layout(
    (source_id, record_type): &(String, String),
    stored: Result<Option<StoredLayout>, LayoutStoreError>,
) -> Option<Layout> {
    let stored = match stored {
        Ok(stored) => stored?,
        Err(error) => {
            tracing::warn!(
                source_id,
                record_type,
                error = &error as &dyn std::error::Error,
                "source layout unavailable; reading through the general view"
            );
            return None;
        }
    };
    match Layout::parse(&stored.layout) {
        Ok(layout) => Some(layout),
        Err(error) => {
            // A JSON error's text can quote the layout, so only its kind and position are logged.
            tracing::warn!(
                source_id,
                record_type,
                layout_sha256 = stored.sha256,
                error = %crate::source_layout::JsonFault(&error),
                "stored source layout does not parse; reading through the general view"
            );
            None
        }
    }
}

/// One citation's evidence: the exact revision this physician's conversation retained, presented
/// by [`crate::source_inspection`], under current workspace and source access. A citation the
/// conversation never made is 404. Restricted or wrong-patient withheld sources return only their
/// unavailable reason, never record content.
pub(crate) async fn citation(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    Path((workspace, chat, citation)): Path<(String, String, String)>,
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
    let conversation = match live.chat.store.read(&scope).await {
        Ok(Some(conversation)) => conversation,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::warn!(%error, "physician citation conversation read failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };
    let Some(cited) = cited(&conversation, &citation) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    match live
        .chat
        .store
        .cited_source_state(&scope.workspace, &cited.reference, &cited.revision)
        .await
    {
        Ok(state) => {
            let layout = match (&state, &cited.source_id) {
                (
                    local_product::runtime::sources::CitedSourceState::Readable { .. },
                    Some(source_id),
                ) => usable_layout(
                    &(source_id.clone(), cited.record_type.clone()),
                    live.chat
                        .store
                        .layout_for(LayoutKey {
                            source_id,
                            record_type: &cited.record_type,
                        })
                        .await,
                ),
                _ => None,
            };
            citation_response(cited, state, layout.as_ref())
        }
        Err(error) => {
            tracing::warn!(%error, "physician citation source-state read failed");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

/// Renders one retained citation's HTTP response from its current [`CitedSourceState`].
/// Restricted sources return only their structured unavailable reason, never record content.
fn citation_response(
    cited: &local_product::physician_chat::Citation,
    state: local_product::runtime::sources::CitedSourceState,
    layout: Option<&Layout>,
) -> Response {
    use local_product::runtime::sources::{
        CitedSourceState, CitedSourceVersion, SourceRestriction,
    };
    use physician_wire::{CitationUnavailableReason, CitedVersionStatus, SourceEvidence};
    match state {
        CitedSourceState::Unavailable { reason } => {
            let (status, reason) = match reason {
                SourceRestriction::AccessRemoved => (
                    StatusCode::FORBIDDEN,
                    CitationUnavailableReason::AccessRemoved,
                ),
                SourceRestriction::WithheldWrongPatient => (
                    StatusCode::GONE,
                    CitationUnavailableReason::WithheldWrongPatient,
                ),
            };
            (status, Json(SourceEvidence::Unavailable { reason })).into_response()
        }
        CitedSourceState::Readable { version } => {
            let mut inspection = crate::source_inspection::inspection_with_layout(cited, layout);
            // The id names the exact revision, which any answer may cite; each answer keeps its
            // own summary of it, and that comes with the answer, not with the source.
            inspection.coda_preview = None;
            let version = match version {
                CitedSourceVersion::Current => CitedVersionStatus::Current,
                CitedSourceVersion::NewerAvailable => CitedVersionStatus::NewerVersionAvailable,
            };
            Json(SourceEvidence::Available {
                inspection: Box::new(inspection),
                version,
            })
            .into_response()
        }
    }
}

/// The citation `id` one of Coda's answers in `conversation` made.
fn cited<'a>(
    conversation: &'a local_product::runtime::Conversation,
    id: &str,
) -> Option<&'a local_product::physician_chat::Citation> {
    conversation
        .messages
        .iter()
        .filter(|message| {
            matches!(
                message.role,
                local_product::runtime::MessageRole::Assistant { .. }
            )
        })
        .flat_map(|message| &message.citations)
        .find(|citation| citation.id == id)
}

/// Who wrote a turn, as the snapshot names it: Coda's turns are the model's, every other the physician's.
fn turn_kind(role: &str) -> &'static str {
    match role {
        "assistant" => "model",
        "mechanical" => "mechanical",
        _ => "physician",
    }
}

/// The answer split into text and the exact citations it published with. A
/// marker without its retained citation ends the split; the rest stays text.
/// Restricted sources keep only their citation identity so withheld content
/// never leaves in the snapshot.
fn citation_segments(
    message: &local_product::runtime::Message,
    layouts: &CitedLayouts,
    restricted: &HashMap<String, local_product::runtime::sources::SourceRestriction>,
) -> Vec<Value> {
    let mut parts = Vec::new();
    let mut text = message.text.as_str();
    while let Some((before, suffix)) = text.split_once("[[patient_source:") {
        let Some((id, rest)) = suffix.split_once("]]") else {
            break;
        };
        let Some(citation) = message.citations.iter().find(|citation| citation.id == id) else {
            break;
        };
        parts.push(json!({"kind":"text", "text":before}));
        if restricted.contains_key(&citation.reference) {
            parts.push(json!({
                "kind":"citation",
                "state":"resolved",
                "source":"patient",
                "source_id":citation.id,
                "handle":citation.reference,
                "display_label":"Unavailable source",
                "source_presentation":null,
                "destination":null
            }));
        } else {
            let layout = citation.source_id.as_ref().and_then(|source_id| {
                layouts.get(&(source_id.clone(), citation.record_type.clone()))
            });
            let inspection = crate::source_inspection::inspection_with_layout(citation, layout);
            let destination = citation.native.as_ref().map(|native| {
                json!({"kind":"source_native", "source_label":native.label, "uri":native.uri})
            });
            parts.push(
                json!({"kind":"citation", "state":"resolved", "source":"patient",
                "source_id":citation.id, "handle":citation.reference,
                "display_label":inspection.title,
                "source_presentation":{"content":inspection.coda_preview, "inspection":inspection},
                "destination":destination}),
            );
        }
        text = rest;
    }
    parts.push(json!({"kind":"text", "text":text}));
    parts
}

/// The reference `SnapshotResponse` subset this site owns. A failed turn adds
/// its run and a mechanical "couldn't finish" line whose `resume_target` names
/// the exact work and run Try again resumes. A running turn offers Stop. A
/// held turn offers Resume. An answered turn names its run and the attempt
/// that answered it, so an operator can match the reply to its telemetry by
/// attempt id. The physician's text stays saved. `sources_version` is the
/// workspace's current source version: an answer begun from an older one is
/// marked as having newer records.
pub(crate) fn durable_snapshot_body(
    conversation: Option<&local_product::runtime::Conversation>,
    sources_version: u64,
    layouts: &CitedLayouts,
    restricted: &HashMap<String, local_product::runtime::sources::SourceRestriction>,
) -> Value {
    let Some(conversation) = conversation else {
        return json!({ "conversation": [], "runs": [] });
    };
    let mut items: Vec<Value> = conversation
        .messages
        .iter()
        .flat_map(|message| {
            let newer_records = message
                .sources_version
                .is_some_and(|answered| answered < sources_version);
            let mut authored = json!({ "kind": turn_kind(message.role.as_str()),
                "text": message.text, "segments": citation_segments(message, layouts, restricted),
                "basis": message.basis, "newer_records": newer_records });
            if !message.source_previews.is_empty() {
                let previews: Vec<_> = message
                    .source_previews
                    .iter()
                    .map(|entry| match entry {
                        local_product::citations::ReadRecordPreview::Preview {
                            reference, ..
                        } => match restricted.get(reference) {
                            Some(restriction) => {
                                local_product::citations::ReadRecordPreview::Unavailable {
                                    reference: reference.clone(),
                                    reason: match restriction {
                                        local_product::runtime::sources::SourceRestriction::AccessRemoved => {
                                            "access_removed"
                                        }
                                        local_product::runtime::sources::SourceRestriction::WithheldWrongPatient => {
                                            "withheld_wrong_patient"
                                        }
                                    }
                                    .into(),
                                }
                            }
                            None => entry.clone(),
                        },
                        local_product::citations::ReadRecordPreview::Unavailable { .. } => {
                            entry.clone()
                        }
                    })
                    .collect();
                authored["source_previews"] = json!(previews);
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
    json!({ "conversation": items, "runs": runs })
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
    use super::{COULDNT_FINISH, STOPPED, WRITING};
    use local_product::runtime::{
        Attempt, AttemptStatus, Conversation, Message, MessageRole, WorkStatus,
    };

    fn durable_snapshot_body(
        conversation: Option<&Conversation>,
        sources_version: u64,
    ) -> serde_json::Value {
        super::durable_snapshot_body(
            conversation,
            sources_version,
            &Default::default(),
            &Default::default(),
        )
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
                citations: vec![],
                source_previews: vec![],
                basis: None,
                sources_version: None,
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
                sources_version: None,
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
                        citations: Vec::new(),
                        source_previews: Vec::new(),
                        basis: None,
                        sources_version: None,
                    });
                }
                let raw = "internal_provider_error: account billing identifier 739";
                saved.attempts[0].events.push(harness::engine::Event {
                    attempt_id: "attempt-1".into(),
                    sequence: 1,
                    ordinal: 1,
                    kind: harness::engine::EventKind::ProviderBlocked { reason: raw.into() },
                });
                let snapshot = durable_snapshot_body(Some(&saved), 0);
                let lines = snapshot["conversation"].as_array().unwrap();
                let status = lines.last().unwrap()["text"].as_str().unwrap();
                assert!(
                    !snapshot.to_string().contains(raw),
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
        let body = durable_snapshot_body(Some(&conversation(WorkStatus::Failed)), 0);
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
            let body = durable_snapshot_body(Some(&conversation(status)), 0);
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
        let held = durable_snapshot_body(Some(&conversation(WorkStatus::Held)), 0);
        let items = held["conversation"].as_array().unwrap();
        assert_eq!(items[1]["text"], STOPPED);
        assert_eq!(items[1]["resume_target"]["work_item_id"], "work-1");
        assert_eq!(items[1]["resume_target"]["expected_run_id"], "run-1");
        assert!(items[1].get("stop_target").is_none());
        assert_eq!(held["runs"][0]["state"], "held");
        let done = durable_snapshot_body(Some(&conversation(WorkStatus::Completed)), 0);
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
        let empty = durable_snapshot_body(None, 0);
        assert!(empty["conversation"].as_array().unwrap().is_empty());
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
            let body = serde_json::json!({
                "snapshot": durable_snapshot_body(Some(&saved), 0)
            });
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

    #[test]
    fn an_answer_keeps_its_basis_and_is_marked_when_records_arrive_after_it() {
        let mut done = conversation(WorkStatus::Completed);
        done.messages.push(Message {
            role: MessageRole::Assistant {
                id: "attempt-1:reply".into(),
                partial: false,
            },
            text: "None recorded.".into(),
            citations: vec![],
            source_previews: vec![],
            basis: Some("Rests on one allergy record.".into()),
            sources_version: Some(3),
        });
        let current = durable_snapshot_body(Some(&done), 3);
        let answer = &current["conversation"][1];
        assert_eq!(answer["text"], "None recorded.");
        assert_eq!(answer["basis"], "Rests on one allergy record.");
        assert_eq!(answer["newer_records"], false);
        assert_eq!(current["conversation"][0]["newer_records"], false);
        let newer = durable_snapshot_body(Some(&done), 4);
        assert_eq!(newer["conversation"][1]["newer_records"], true);
        assert_eq!(newer["conversation"][1]["text"], "None recorded.");
        done.messages[1].sources_version = None;
        let unrecorded = durable_snapshot_body(Some(&done), 4);
        assert_eq!(unrecorded["conversation"][1]["newer_records"], false);
    }
}

#[cfg(test)]
mod citation_segment_tests {
    use super::citation_segments;
    use local_product::physician_chat::Citation;
    use local_product::runtime::Message;
    use local_product::runtime::sources::SourceRestriction;
    use serde_json::json;

    fn cited(id: &str, native: serde_json::Value) -> Citation {
        let content = format!(r#"{{"secret":"sensitive-{id}"}}"#);
        serde_json::from_value(json!({
            "id":id, "reference":id, "revision":"r1", "record_type":"AllergyIntolerance",
            "content":content, "sha256":local_product::sha256_hex(content.as_bytes()),
            "summary":format!("Summary of {id}"),
            "provenance":{}, "native":native
        }))
        .unwrap()
    }

    #[test]
    fn a_source_native_link_becomes_the_citation_destination() {
        let message = Message {
            role: local_product::runtime::MessageRole::Assistant {
                id: "reply-1".into(),
                partial: false,
            },
            text: "A [[patient_source:linked]] B [[patient_source:plain]]".into(),
            source_previews: vec![],
            basis: None,
            sources_version: None,
            citations: vec![
                cited(
                    "linked",
                    json!({"label":"Demo chart","uri":"https://ehr.example/a/1"}),
                ),
                cited("plain", json!(null)),
            ],
        };
        let segments = citation_segments(
            &message,
            &Default::default(),
            &std::collections::HashMap::new(),
        );
        assert_eq!(
            segments[1]["destination"],
            json!({"kind":"source_native", "source_label":"Demo chart", "uri":"https://ehr.example/a/1"})
        );
        assert!(segments[3]["destination"].is_null());
    }

    #[test]
    fn restricted_citations_strip_content_summary_and_destination_from_the_snapshot() {
        for reason in [
            SourceRestriction::AccessRemoved,
            SourceRestriction::WithheldWrongPatient,
        ] {
            let message = Message {
                role: local_product::runtime::MessageRole::Assistant {
                    id: "reply-1".into(),
                    partial: false,
                },
                text: "See [[patient_source:wrong-patient-msg]].".into(),
                source_previews: vec![local_product::citations::ReadRecordPreview::Preview {
                    reference: "wrong-patient-msg".into(),
                    revision: "r1".into(),
                    preview: "Summary of wrong-patient-msg".into(),
                    dates: vec![],
                    native: serde_json::from_value(
                        json!({"label":"Spruce","uri":"https://spruce.example/m/9"}),
                    )
                    .unwrap(),
                }],
                basis: None,
                sources_version: None,
                citations: vec![cited(
                    "wrong-patient-msg",
                    json!({"label":"Spruce","uri":"https://spruce.example/m/9"}),
                )],
            };
            let restricted = [("wrong-patient-msg".to_owned(), reason)]
                .into_iter()
                .collect();
            let conversation = local_product::runtime::Conversation {
                runs: Default::default(),
                messages: vec![message],
                status: local_product::runtime::WorkStatus::Completed,
                work: String::new(),
                attempts: Vec::new(),
                revision: 1,
                skills: serde_json::from_value(json!("fixture-package")).unwrap(),
                session_hash: None,
                dispatch_after: 0,
                failures: 0,
                budget_resume_after: 0,
                bindings: Default::default(),
                opened_at: 0,
                last_activity_at: 0,
                scenario: None,
            };
            let snapshot = super::durable_snapshot_body(
                Some(&conversation),
                0,
                &Default::default(),
                &restricted,
            );
            let encoded = serde_json::to_string(&snapshot).unwrap();
            assert!(
                !encoded.contains("sensitive-wrong-patient-msg"),
                "{encoded}"
            );
            assert!(
                !encoded.contains("Summary of wrong-patient-msg"),
                "{encoded}"
            );
            assert!(!encoded.contains("spruce.example"), "{encoded}");
            let segments = &snapshot["conversation"][0]["segments"];
            assert_eq!(segments[1]["state"], "resolved");
            assert_eq!(segments[1]["source_id"], "wrong-patient-msg");
            assert!(segments[1]["source_presentation"].is_null());
            assert!(segments[1]["destination"].is_null());
            assert_eq!(
                snapshot["conversation"][0]["source_previews"][0]["state"],
                "unavailable"
            );
        }
    }

    #[tokio::test]
    async fn citation_response_reports_newer_versions_and_never_returns_restricted_content() {
        use super::citation_response;
        use axum::body::to_bytes;
        use axum::http::StatusCode;
        use local_product::runtime::sources::{CitedSourceState, CitedSourceVersion};
        use physician_wire::{CitationUnavailableReason, CitedVersionStatus, SourceEvidence};

        let retained = cited(
            "msg-42",
            json!({"label":"Spruce","uri":"https://spruce.example/m/42"}),
        );

        let current_resp = citation_response(
            &retained,
            CitedSourceState::Readable {
                version: CitedSourceVersion::Current,
            },
            None,
        );
        assert_eq!(current_resp.status(), StatusCode::OK);
        let current_body: SourceEvidence =
            serde_json::from_slice(&to_bytes(current_resp.into_body(), 65_536).await.unwrap())
                .unwrap();
        match current_body {
            SourceEvidence::Available {
                inspection,
                version,
            } => {
                assert_eq!(version, CitedVersionStatus::Current);
                assert_eq!(inspection.technical.revision, "r1");
                assert_eq!(inspection.technical.original, retained.content);
                assert!(inspection.coda_preview.is_none());
            }
            other => panic!("expected available evidence, got {other:?}"),
        }

        let newer_resp = citation_response(
            &retained,
            CitedSourceState::Readable {
                version: CitedSourceVersion::NewerAvailable,
            },
            None,
        );
        assert_eq!(newer_resp.status(), StatusCode::OK);
        let newer_body: SourceEvidence =
            serde_json::from_slice(&to_bytes(newer_resp.into_body(), 65_536).await.unwrap())
                .unwrap();
        match newer_body {
            SourceEvidence::Available {
                inspection,
                version,
            } => {
                assert_eq!(version, CitedVersionStatus::NewerVersionAvailable);
                assert_eq!(inspection.technical.revision, "r1");
                assert_eq!(inspection.technical.original, retained.content);
            }
            other => panic!("expected available evidence with newer version, got {other:?}"),
        }

        for (restriction, expected_status, expected_reason) in [
            (
                SourceRestriction::AccessRemoved,
                StatusCode::FORBIDDEN,
                CitationUnavailableReason::AccessRemoved,
            ),
            (
                SourceRestriction::WithheldWrongPatient,
                StatusCode::GONE,
                CitationUnavailableReason::WithheldWrongPatient,
            ),
        ] {
            let resp = citation_response(
                &retained,
                CitedSourceState::Unavailable {
                    reason: restriction,
                },
                None,
            );
            assert_eq!(resp.status(), expected_status);
            let raw = to_bytes(resp.into_body(), 65_536).await.unwrap();
            let text = std::str::from_utf8(&raw).unwrap();
            assert!(!text.contains("sensitive-msg-42"), "{text}");
            assert!(!text.contains("Summary of msg-42"), "{text}");
            assert!(!text.contains("spruce.example"), "{text}");
            let decoded: SourceEvidence = serde_json::from_slice(&raw).unwrap();
            assert_eq!(
                decoded,
                SourceEvidence::Unavailable {
                    reason: expected_reason
                }
            );
        }
    }
}

#[cfg(test)]
mod cited_layout_tests {
    use super::{cited_layouts, durable_snapshot_body};
    use crate::source_inspection::inspection;
    use crate::source_inspection::tests::{general_citation, observation};
    use local_product::physician_chat::Citation;
    use local_product::physician_layouts::LayoutKey;
    use local_product::runtime::{Conversation, Message, MessageRole, Store, WorkStatus};
    use serde_json::{Value, json};

    const OBSERVATION_LAYOUT: &str = r#"{"record_type":"Observation",
        "statuses":[{"role":"status","pointer":"/status"}],
        "sections":[{"heading":"result","slots":[
            {"role":"code","pointer":"/code/text"},
            {"role":"result","pointer":"/valueQuantity/value"},
            {"role":"unit","pointer":"/valueQuantity/unit"}]}]}"#;

    async fn store() -> Store {
        let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL"))
            .expect("PHYSICIAN_TEST_DATABASE_URL required for the stored layout proof");
        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        store
    }

    fn cited(id: &str, source_id: Option<&str>) -> Citation {
        let mut cited = general_citation(&observation());
        cited.id = id.into();
        cited.source_id = source_id.map(str::to_owned);
        cited
    }

    fn conversation(citations: Vec<Citation>) -> Conversation {
        let text = citations
            .iter()
            .map(|c| format!("[[patient_source:{}]]", c.id))
            .collect::<String>();
        Conversation {
            runs: Default::default(),
            messages: vec![Message {
                role: MessageRole::Assistant {
                    id: "reply-1".into(),
                    partial: false,
                },
                text,
                citations,
                source_previews: vec![],
                basis: None,
                sources_version: None,
            }],
            status: WorkStatus::Completed,
            work: String::new(),
            attempts: Vec::new(),
            revision: 1,
            skills: serde_json::from_value(json!("fixture-package")).unwrap(),
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

    /// Each citation's inspection, in the order the message cites them.
    fn inspections(body: &Value) -> Vec<Value> {
        body["conversation"][0]["segments"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|segment| segment["kind"] == "citation")
            .map(|segment| segment["source_presentation"]["inspection"].clone())
            .collect()
    }

    fn general(citation: &Citation) -> Value {
        serde_json::to_value(inspection(citation)).unwrap()
    }

    #[tokio::test]
    async fn a_cited_record_reads_through_its_sources_stored_layout_and_falls_back_without_one() {
        let store = store().await;
        let unique = uuid::Uuid::new_v4();
        let laid_out = format!("ehr-{unique}");
        let unparseable = format!("unparseable-{unique}");
        let refused = format!("refused-{unique}");
        let bare = format!("bare-{unique}");
        let save = |source: &str, layout: &str| {
            let store = store.clone();
            let (source, layout) = (source.to_owned(), layout.to_owned());
            async move {
                let key = LayoutKey {
                    source_id: &source,
                    record_type: "Observation",
                };
                store.save_layout(key, &layout).await.unwrap();
            }
        };
        save(&laid_out, OBSERVATION_LAYOUT).await;
        // JSON the store accepts but `Layout` does not: a literal value beside the pointers.
        save(
            &unparseable,
            &OBSERVATION_LAYOUT.replacen("\"sections\"", "\"value\":\"7.2\",\"sections\"", 1),
        )
        .await;
        // A layout that parses but reads its result from a pointer the record does not have.
        save(
            &refused,
            &OBSERVATION_LAYOUT.replace("/valueQuantity/value", "/valueQuantity/missing"),
        )
        .await;
        let citations = vec![
            cited("laid-out", Some(&laid_out)),
            cited("unparseable", Some(&unparseable)),
            cited("refused", Some(&refused)),
            cited("no-layout", Some(&bare)),
            cited("retained-before-source-ids", None),
        ];
        let conversation = conversation(citations.clone());
        let layouts = cited_layouts(&store, &conversation).await;
        let views = inspections(&durable_snapshot_body(
            Some(&conversation),
            0,
            &layouts,
            &std::collections::HashMap::new(),
        ));
        assert_eq!(views.len(), citations.len());

        let sections = &views[0]["layout_sections"];
        assert_eq!(sections[0]["heading"], "Result", "{sections}");
        assert_eq!(
            sections[0]["fields"]
                .as_array()
                .unwrap()
                .iter()
                .map(|field| (field["label"].clone(), field["text"].clone()))
                .collect::<Vec<_>>(),
            [
                (json!("Statement in this record"), json!("Hemoglobin A1c")),
                (json!("Result"), json!("7.2")),
                (json!("Unit"), json!("%")),
            ]
        );
        assert_ne!(views[0], general(&citations[0]));
        for (view, citation) in views.iter().zip(&citations).skip(1) {
            assert_eq!(*view, general(citation), "{}", citation.id);
        }
    }

    /// A layout that shows the status only in a section still leaves it in the statuses the
    /// citation carries, which the hover preview and the pane header read.
    #[test]
    fn a_status_placed_in_a_section_still_reaches_the_hover_preview() {
        let in_section = crate::source_layout::Layout::parse(
            r#"{"record_type":"Observation","statuses":[],
            "sections":[{"heading":"result","slots":[
                {"role":"result","pointer":"/valueQuantity/value"},
                {"role":"status","pointer":"/status"}]}]}"#,
        )
        .unwrap();
        let cited = cited("in-section", Some("ehr"));
        let layouts = [(("ehr".to_owned(), "Observation".to_owned()), in_section)]
            .into_iter()
            .collect();
        let body = durable_snapshot_body(
            Some(&conversation(vec![cited])),
            0,
            &layouts,
            &std::collections::HashMap::new(),
        );
        let segments: Vec<physician_wire::ClinicalSegment> =
            serde_json::from_value(body["conversation"][0]["segments"].clone()).unwrap();
        let inspection = segments
            .into_iter()
            .find_map(|segment| match segment {
                physician_wire::ClinicalSegment::Citation {
                    source_presentation,
                    ..
                } => source_presentation?.inspection,
                physician_wire::ClinicalSegment::Text { .. } => None,
            })
            .expect("the citation carries its inspection");
        assert_eq!(inspection.layout_sections[0].fields[1].text, "final");
        assert_eq!(
            inspection
                .statuses
                .iter()
                .map(|status| (status.label.as_str(), status.text.as_str()))
                .collect::<Vec<_>>(),
            [("Result status", "final")]
        );
    }

    #[tokio::test]
    async fn an_unreachable_layout_store_reads_through_the_general_view() {
        let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL"))
            .expect("PHYSICIAN_TEST_DATABASE_URL required for the stored layout proof");
        let db = sea_orm::Database::connect(&url).await.unwrap();
        db.clone().close().await.unwrap();
        let closed = Store::from_connection(db);
        let citation = cited("laid-out", Some("ehr"));
        let conversation = conversation(vec![citation.clone()]);
        let layouts = cited_layouts(&closed, &conversation).await;
        assert!(layouts.is_empty());
        let views = inspections(&durable_snapshot_body(
            Some(&conversation),
            0,
            &layouts,
            &std::collections::HashMap::new(),
        ));
        assert_eq!(views, [general(&citation)]);
    }
}
