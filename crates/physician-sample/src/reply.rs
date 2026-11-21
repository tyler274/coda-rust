//! Physician-specific reply revision HTTP boundary. A saved draft is required;
//! model output never creates reply work or sends a patient message.
use crate::{
    auth::{AuthContext, allows_workspace, now},
    live::Live,
};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::sync::Arc;

fn scope(ctx: &AuthContext, workspace: &str, reply: &str) -> local_product::runtime::Scope {
    local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: workspace.into(),
        chat: reply.into(),
    }
}

#[derive(Deserialize)]
pub(crate) struct Revision {
    pub patient_workspace_id: String,
    pub reply_work_id: String,
    pub expected_draft_revision: u64,
    pub instruction: String,
    pub physician_time_zone: String,
    /// The rehearsal scenario, admitted as on `message.submit`.
    #[serde(default)]
    pub scenario: Option<String>,
}
#[derive(Deserialize)]
pub(crate) struct Save {
    revision: u64,
    text: String,
}

pub(crate) fn revision_run(
    draft: &local_product::runtime::reply::Draft,
    reply: &str,
) -> Option<physician_wire::ReplyRevisionRun> {
    draft
        .runs
        .current()
        .map(|run| physician_wire::ReplyRevisionRun {
            run_id: run.id.into(),
            status: sea_orm::ActiveEnum::to_value(&draft.status),
            resume_target: draft.resumable_run().map(|resumable| {
                physician_wire::WorkResumeTarget {
                    work_item_id: physician_wire::WorkItemId(reply.into()),
                    expected_run_id: resumable.id.into(),
                }
            }),
        })
}

pub(crate) async fn read(
    State(live): State<Arc<Live>>,
    Path((workspace, reply)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
) -> Response {
    let Some(Extension(ctx)) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let durable = &live.chat;
    match durable.store.draft(&scope(&ctx, &workspace, &reply)).await {
        // The latest accepted revision run and its status (pending,
        // active, completed, held, failed or cancelled), so a terminal
        // failure is visible to the physician, not only in Postgres.
        Ok(Some(draft)) => Json(json!({
            "revision": draft.revision,
            "text": draft.text,
            "restore_text": draft.restore_text,
            "revision_run": revision_run(&draft, &reply),
        }))
        .into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

pub(crate) async fn save(
    State(live): State<Arc<Live>>,
    Path((workspace, reply)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
    Json(save): Json<Save>,
) -> Response {
    let Some(Extension(ctx)) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let durable = &live.chat;
    if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let mut authority = crate::durable::RuntimeAuthority;
    let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    // The runtime locks an existing draft and current authority together.
    // A PUT cannot open reply work, even with revision zero.
    match durable
        .store
        .save_draft(
            &scope(&ctx, &workspace, &reply),
            save.revision,
            &save.text,
            &hash,
            &mut authority,
        )
        .await
    {
        Ok(revision) => Json(json!({"revision": revision, "text": save.text})).into_response(),
        Err(_) => StatusCode::CONFLICT.into_response(),
    }
}

/// Hosted acceptance only records durable work; the store wakes the workers.
/// It never runs a model in the site process.
pub(crate) async fn accept_durable(
    live: &Arc<Live>,
    ctx: &AuthContext,
    id: &str,
    payload: Value,
    headers: &HeaderMap,
    peer_addr: Option<SocketAddr>,
) -> Response {
    use local_product::runtime::reply::{RevisionAccepted, RevisionInput};
    let Ok(input) = serde_json::from_value::<Revision>(payload) else {
        return StatusCode::CONFLICT.into_response();
    };
    if [
        id,
        &input.patient_workspace_id,
        &input.reply_work_id,
        &input.instruction,
        &input.physician_time_zone,
    ]
    .iter()
    .any(|field| field.trim().is_empty())
    {
        return StatusCode::CONFLICT.into_response();
    }
    if ctx.auth.store.identity(&ctx.token, now()).await.is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    if !allows_workspace(&ctx.auth, &ctx.identity, &input.patient_workspace_id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let durable = &live.chat;
    let scope = scope(ctx, &input.patient_workspace_id, &input.reply_work_id);
    let Ok(scenario) = durable.admit_scenario(&scope.practice, input.scenario) else {
        return crate::live::rejected_scenario();
    };
    if let Err(exhausted) = live
        .demo_budget
        .check_and_record(&ctx.identity, headers, peer_addr)
    {
        return crate::demo_session::demo_budget_exhausted_response(exhausted);
    }
    let revision = RevisionInput {
        key: id.into(),
        expected_revision: input.expected_draft_revision,
        instruction: input.instruction,
        timezone: input.physician_time_zone,
        scenario,
    };
    let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match durable.accept_revision(&scope, &revision, hash).await {
        Ok(RevisionAccepted::New | RevisionAccepted::Duplicate) => Json(json!({
            "outcome": "accepted", "patient_workspace_id": input.patient_workspace_id, "run_id": id,
        }))
        .into_response(),
        Ok(RevisionAccepted::Refused) => StatusCode::CONFLICT.into_response(),
        Err(error) if error.contains("idempotency collision") => {
            StatusCode::CONFLICT.into_response()
        }
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}
