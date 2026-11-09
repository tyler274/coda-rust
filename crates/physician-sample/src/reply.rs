//! Physician-specific reply revision HTTP boundary. A saved draft is required;
//! model output never creates reply work or sends a patient message.
use crate::{
    auth::{AuthContext, allows_workspace, now},
    live::Live,
};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

pub(crate) fn target(ctx: &AuthContext, reply: &str) -> String {
    // JSON tuple avoids ambiguous delimiter-based physician identity joins.
    serde_json::to_string(&(&ctx.identity.practice_id, &ctx.identity.physician_id, reply)).unwrap()
}
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
    if let Some(durable) = &live.durable {
        return match durable.store.draft(&scope(&ctx, &workspace, &reply)).await {
            // The latest accepted revision run and its status (pending,
            // active, completed, failed or superseded), so a terminal
            // failure is visible to the physician, not only in Postgres.
            Ok(Some(draft)) => Json(json!({
                "revision": draft.revision,
                "text": draft.text,
                "restore_text": draft.restore_text,
                "revision_run": draft.input.as_ref().map(|input| json!({
                    "run_id": input.key,
                    "status": draft.status,
                })),
            }))
            .into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
    }
    match live
        .session
        .lock()
        .expect("physician session")
        .draft(&workspace, &target(&ctx, &reply))
    {
        Some((revision, text, restore_text)) => {
            Json(json!({"revision": revision,"text":text,"restore_text":restore_text}))
                .into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
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
    if let Some(durable) = &live.durable {
        if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
            return StatusCode::NOT_FOUND.into_response();
        }
        let mut authority = crate::durable::RuntimeAuthority {
            store: durable.store.clone(),
        };
        let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
        // The runtime locks an existing draft and current authority together.
        // A PUT cannot open reply work, even with revision zero.
        return match durable
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
        };
    }
    let _guard = ctx.auth.publication.lock().await;
    let mut revision = None;
    // This slice never invents initial reply work. The initial saved draft must
    // have been admitted by the physician reply owner during owner loading.
    let mut publish = || {
        let mut session = live.session.lock().expect("physician session");
        let target = target(&ctx, &reply);
        if session.draft(&workspace, &target).is_none() {
            return Err("reply draft missing".into());
        }
        revision = Some(session.save_draft(&workspace, &target, save.revision, &save.text)?);
        Ok(())
    };
    let result = crate::publication::publish(&ctx, &workspace, &mut publish).await;
    match result {
        Ok(()) => Json(json!({"revision":revision,"text":save.text})).into_response(),
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
    let Some(durable) = &live.durable else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let scope = scope(ctx, &input.patient_workspace_id, &input.reply_work_id);
    let Ok(scenario) = durable.admit_scenario(&scope.practice, input.scenario) else {
        return crate::live::rejected_scenario();
    };
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

pub(crate) async fn admit(
    live: &Arc<Live>,
    ctx: &AuthContext,
    id: &str,
    payload: Value,
) -> Result<local_product::execution::Work, String> {
    let input: Revision =
        serde_json::from_value(payload).map_err(|_| "invalid revision command")?;
    if input.scenario.is_some() {
        return Err("inline revisions take no scenario".into());
    }
    if id.trim().is_empty()
        || input.instruction.trim().is_empty()
        || input.physician_time_zone.trim().is_empty()
    {
        return Err("invalid revision input".into());
    }
    if ctx.auth.store.identity(&ctx.token, now()).await.is_err()
        || !allows_workspace(&ctx.auth, &ctx.identity, &input.patient_workspace_id).await
    {
        return Err("not authorized".into());
    }
    let target = target(ctx, &input.reply_work_id);
    // Observation before session, the order `LiveOwner::finish` takes them.
    // Anything recorded after this read is caught before the work finishes.
    let (version, sources) = live.observed_sources(&input.patient_workspace_id);
    let mut session = live.session.lock().expect("physician session");
    let (revision, text, _) = session
        .draft(&input.patient_workspace_id, &target)
        .ok_or("reply work has no saved draft")?;
    if revision != input.expected_draft_revision {
        return Err("stale draft".into());
    }
    let request = local_product::runtime::reply::request(
        &live.binding.model,
        &live.binding.effort,
        &text,
        &input.instruction,
        &input.physician_time_zone,
        &sources,
    );
    let work = session.revise(id, &input.patient_workspace_id, &target, revision, request)?;
    session.pin_sources(&work, version, sources)
}
