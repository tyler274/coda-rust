//! Authenticated accepted commands and reconnectable durable server-sent events.
use crate::{
    ApiError,
    background::{self, api_error_class, failed},
    conversation::ConversationStore,
    durable::DurableState,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::{Instrument, instrument::WithSubscriber};
use uuid::Uuid;

/// Provider configuration is explicit; no paid calls occur unless enabled at startup.
#[derive(Clone)]
pub struct ConversationState {
    /// Identity/permission authority.
    pub durable: DurableState,
    /// Accepted-work owner.
    pub store: ConversationStore,
    /// Server-controlled model, never supplied by a patient command.
    pub model: String,
    /// Authorized transport URL and credential. None leaves commands unavailable rather than faking replies.
    pub provider: Option<(String, String)>,
}
/// Conversation routes use the same owner for admission, execution and event readback.
pub fn router(state: ConversationState) -> Router {
    Router::new()
        .route(
            "/api/conversation/messages",
            get(phone_read).post(phone_send),
        )
        .route(
            "/api/conversation/messages/{message_id}",
            get(phone_message),
        )
        .route(
            "/api/conversation/messages/{message_id}/sources/{source_id}",
            get(message_source),
        )
        .route(
            "/api/conversation/messages/{message_id}/sources/{source_id}/revision",
            get(message_source_revision),
        )
        .route("/api/conversation/{id}", get(read))
        .route("/api/conversation/{id}/commands", post(accept))
        .route("/api/conversation/commands/{id}/resume", post(resume))
        .route("/api/conversation/commands/{id}/events", get(events))
        .route("/api/conversation/commands/{id}/usage", get(usage))
        .with_state(state)
}
async fn phone_read(
    State(s): State<ConversationState>,
    h: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.store.phone_page(&c, s.provider.is_some()).await?))
}
async fn phone_message(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path(message_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.store.phone_message(&c, message_id).await?))
}
async fn phone_send(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Json(body): Json<duet_wire::models::ConversationSendRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let submission = crate::conversation::Submission {
        timezone: body.timezone,
        selected_context: body.selected_context.flatten().map(|s| *s),
        reply_to_message_id: body.reply_to_message_id.flatten(),
    };
    if s.provider.is_none() {
        return Err(ApiError::Unavailable("conversation provider not enabled"));
    }
    let accepted = s
        .store
        .phone_accept_submission(&c, body.id, &body.text, &s.model, &submission)
        .await?;
    // A steer joins the reply in progress; launch that command, not the steer.
    let run = serde_json::from_value(accepted["run"]["id"].clone()).unwrap_or(body.id);
    launch(&s, &c, run).await?;
    Ok((axum::http::StatusCode::ACCEPTED, Json(accepted)))
}
async fn message_source_revision(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path((message, source)): Path<(Uuid, String)>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(
        s.store
            .message_source_revision(&c, message, &source)
            .await?,
    ))
}
async fn message_source(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path((message, source)): Path<(Uuid, String)>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.store.message_source(&c, message, &source).await?))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Command {
    id: Uuid,
    expected_revision: i64,
    text: String,
    #[serde(default)]
    submission: crate::conversation::Submission,
}
async fn accept(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path(conversation): Path<Uuid>,
    Json(body): Json<Command>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    if s.provider.is_none() {
        return Err(ApiError::Unavailable("conversation provider not enabled"));
    }
    let result = s
        .store
        .accept_submission(
            &c,
            body.id,
            conversation,
            body.expected_revision,
            &body.text,
            &s.model,
            &body.submission,
        )
        .await?;
    // Retry of an accepted command may launch recovery; an active claim rejects duplicate dispatch.
    // A steer launches the command it joined.
    let run = serde_json::from_value(result["run_id"].clone()).unwrap_or(body.id);
    launch(&s, &c, run).await?;
    Ok(Json(result))
}
/// Poll durable pending work after process restart; claims prevent multi-instance duplication.
pub async fn recover_pending(s: ConversationState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
        interval.tick().await;
        if s.provider.is_none() {
            continue;
        }
        sweep(&s)
            .instrument(tracing::info_span!(
                "duet.recovery.sweep",
                otel.status_code = tracing::field::Empty
            ))
            .await;
    }
}
const FOLLOW_UP_FAILED: &str = "duet.follow_up.failed";
const RECOVERY_FAILED: &str = "duet.recovery.failed";
/// One recovery tick: due follow-ups first, then pending commands. A failed
/// item is reported and skipped; the next tick retries it.
async fn sweep(s: &ConversationState) {
    match s.store.due_follow_up_contexts().await {
        Ok(contexts) => {
            for c in contexts {
                match s
                    .store
                    .accept_due_follow_up(&c, Uuid::new_v4(), &s.model)
                    .await
                {
                    Ok(Some(id)) => {
                        if let Err(e) = launch(s, &c, id).await {
                            failed(FOLLOW_UP_FAILED, "launch", api_error_class(&e));
                        }
                    }
                    Ok(None) => {}
                    Err(e) => failed(FOLLOW_UP_FAILED, "accept", api_error_class(&e)),
                }
            }
        }
        Err(e) => failed(FOLLOW_UP_FAILED, "contexts", api_error_class(&e)),
    }
    match s.durable.identity.database.pending_commands().await {
        Ok(pending) => {
            for (account, id) in pending {
                match s.store.recovery_context(account, id).await {
                    Ok(c) => {
                        if let Err(e) = launch(s, &c, id).await {
                            failed(RECOVERY_FAILED, "launch", api_error_class(&e));
                        }
                    }
                    Err(e) => failed(RECOVERY_FAILED, "context", api_error_class(&e)),
                }
            }
        }
        Err(_) => failed(RECOVERY_FAILED, "pending", background::DATABASE),
    }
}
/// Immediate same-command reclaims after steers in one launch. Later steers
/// still reach the command through the 30s recovery poll.
const MAX_STEER_RECLAIMS: usize = 8;
const CONVERSATION_FAILED: &str = "duet.conversation.failed";
pub(crate) async fn launch(
    s: &ConversationState,
    c: &crate::identity::AccountContext,
    id: Uuid,
) -> Result<(), ApiError> {
    let (endpoint, key) = s
        .provider
        .clone()
        .ok_or(ApiError::Unavailable("conversation provider not enabled"))?;
    let mut provider = harness::provider::ResponsesProvider::new(endpoint, key)
        .map_err(|_| ApiError::Unavailable("provider configuration invalid"))?;
    let mut owner = match s.store.claim(c, id).await {
        Ok(o) => o,
        Err(ApiError::Conflict(_)) => return Ok(()),
        Err(e) => return Err(e),
    };
    let privacy = s.durable.privacy.clone();
    let account = c.account_id;
    let store = s.store.clone();
    let context = c.clone();
    tokio::spawn(
        async move {
            // A steer fences the attempt. Reclaim immediately, but leave later
            // steers to the recovery poll after a bounded number of restarts.
            let mut reclaims = 0;
            loop {
                let result = owner
                    .execute(
                        &mut provider,
                        harness::execution::Limits::default(),
                        &harness::execution::Cancellation::default(),
                    )
                    .await;
                if result.is_ok() {
                    break;
                }
                match owner.steered().await {
                    Ok(true) => {}
                    Ok(false) => {
                        // Execute's string error may contain provider or patient
                        // content. Do not inspect it or export it as a field.
                        failed(CONVERSATION_FAILED, "execute", "execution");
                        break;
                    }
                    Err(e) => {
                        failed(CONVERSATION_FAILED, "steered", api_error_class(&e));
                        break;
                    }
                }
                if reclaims >= MAX_STEER_RECLAIMS {
                    break;
                }
                reclaims += 1;
                match store.claim(&context, id).await {
                    Ok(next) => owner = next,
                    // Another launch already claimed or completed this command.
                    Err(ApiError::Conflict(_)) => break,
                    Err(e) => {
                        failed(CONVERSATION_FAILED, "claim", api_error_class(&e));
                        break;
                    }
                }
            }
            // Restrictions stay effective even when execution or confirmation
            // fails; privacy recovery retries the durable journal receipt.
            if let Err(e) = privacy.confirm_account(account).await {
                failed(CONVERSATION_FAILED, "confirm", api_error_class(&e));
            }
        }
        .instrument(tracing::info_span!(
            "duet.conversation.execute",
            otel.status_code = tracing::field::Empty
        ))
        .with_current_subscriber(),
    );
    Ok(())
}
async fn resume(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    launch(&s, &c, id).await?;
    Ok(Json(json!({"id":id,"accepted":true})))
}
async fn read(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.store.read(&c, id).await?))
}
#[derive(Deserialize, Default)]
struct Cursor {
    after: Option<i64>,
}
async fn usage(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let context = s.durable.authenticate(&h).await?;
    Ok(Json(s.store.usage(&context, id).await?))
}
async fn events(
    State(s): State<ConversationState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
    Query(q): Query<Cursor>,
) -> Result<impl IntoResponse, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let after = q
        .after
        .or_else(|| {
            h.get("last-event-id")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(0);
    let rows = s.store.events(&c, id, after).await?;
    // Finite SSE batches reconnect through Last-Event-ID; no process-local channel is truth.
    let mut body = String::from("retry: 1000\n\n");
    for row in rows {
        body.push_str(&format!(
            "id: {}\nevent: conversation\ndata: {}\n\n",
            row["id"], row["event"]
        ));
    }
    Ok((
        [
            ("content-type", "text/event-stream"),
            ("cache-control", "no-store"),
        ],
        body,
    ))
}
