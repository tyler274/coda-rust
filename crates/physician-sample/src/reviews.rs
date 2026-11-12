//! HTTP handlers and projection owner for saved patient reviews on the patient page.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use axum::Json;
use axum::body::Body;
use axum::extract::{Extension, Path, State};
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use harness::engine::EventKind;
use harness::engine::results::ToolResult;
use local_product::patient_reviews::{
    COMPLETE_PATIENT_REVIEW_TOOL, READ_PATIENT_REVIEWS_TOOL, ReviewCitation, ReviewOutcome,
    ReviewTrigger, UnfinishedReviewWork,
};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{SEARCH_TOOL, SourceRevision};
use local_product::runtime::chat_operations::{PATIENT_MEMORY_TOOL, RecordLimitKind};
use local_product::runtime::concerns::ExactRecordReference;
use local_product::runtime::history::{
    CitedSourceReadback, HistorySegment, cited_record_title, format_cited_source_body,
    parse_segments_with_citations,
};
use local_product::runtime::jobs::{PracticeJob, PracticeJobAttempt};
use local_product::runtime::reviews::{
    PatientReviewError, PatientReviewTarget, RecordedReviewAction, ReviewHoldReason,
    ReviewLifecycleState, ReviewRecipeVersion, ReviewSupersededReason, SavedPatientReview,
    VersionedReviewAction, VersionedReviewControl, patient_review_version,
};
use local_product::{ModelBinding, ModelRoles};
use physician_bus::{Bus, Event as BusEvent};
use physician_wire::{
    CitedSourceWire, MessageSegment, PatientReviewOutcomeWire, PatientReviewStateWire,
    PatientReviewWire, PatientReviewsResponse, ReviewAcceptedInputWire, ReviewActionWire,
    ReviewControlActionWire, ReviewControlRefusalReason, ReviewControlRequest,
    ReviewControlResponse, ReviewEvidenceRefWire, ReviewHoldReasonWire, ReviewLimitKindWire,
    ReviewStepStateWire, ReviewStepWire, ReviewSupersededReasonWire, ReviewTriggerWire,
    ReviewUnfinishedWorkWire, ReviewVersion,
};
use serde_json::Value;

use crate::care::ConnectedCare;
use crate::live::Live;
use crate::timestamp::format_rfc3339;

/// Shared durable review state attached to `router()` when Postgres is enabled
/// without a separate Pub/Sub chat queue.
#[derive(Clone)]
pub(crate) struct StandaloneReviews {
    store: local_product::runtime::Store,
    roles: Option<ModelRoles>,
    skills: Option<PhysicianSkills>,
    bus: Arc<dyn Bus>,
}

impl StandaloneReviews {
    pub(crate) fn from_env(
        get: &impl Fn(&str) -> Option<String>,
        store: local_product::runtime::Store,
        bus: Arc<dyn Bus>,
        hosted: bool,
    ) -> Self {
        let settings: Vec<_> = local_product::MODEL_SETTING_KEYS
            .into_iter()
            .filter_map(|key| {
                get(key)
                    .map(|value| value.trim().to_owned())
                    .filter(|value| !value.is_empty())
                    .map(|value| (key, value))
            })
            .collect();
        let env_pairs: Vec<_> = settings
            .iter()
            .map(|(key, value)| (*key, value.as_str()))
            .collect();
        let roles = local_product::model_roles("responses", &env_pairs).ok();
        let skills_dir = get("PHYSICIAN_SKILLS_DIR")
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let skills = PhysicianSkills::configured(skills_dir.as_deref(), hosted).ok();
        Self {
            store,
            roles,
            skills,
            bus,
        }
    }
}

struct ReviewCtx<'a> {
    store: &'a local_product::runtime::Store,
    chat: Option<&'a crate::durable::DurableChat>,
    roles: Option<&'a ModelRoles>,
    skills: Option<&'a PhysicianSkills>,
    bus: &'a dyn Bus,
}

impl<'a> ReviewCtx<'a> {
    fn from_live(live: &'a Live) -> Self {
        Self {
            store: &live.chat.store,
            chat: Some(&live.chat),
            roles: Some(live.chat.roles()),
            skills: Some(live.chat.skills()),
            bus: live.bus.as_ref(),
        }
    }

    fn from_standalone(standalone: &'a StandaloneReviews) -> Self {
        Self {
            store: &standalone.store,
            chat: None,
            roles: standalone.roles.as_ref(),
            skills: standalone.skills.as_ref(),
            bus: standalone.bus.as_ref(),
        }
    }
}

fn fallback_roles(recipe: &ReviewRecipeVersion) -> ModelRoles {
    let background = ModelBinding {
        model: recipe.model.clone(),
        effort: recipe.effort.clone(),
        context_window_tokens: recipe.context_window_tokens,
        max_input_tokens: recipe.max_input_tokens,
        adapter: "responses".to_owned(),
    };
    ModelRoles {
        chat: background.clone(),
        background,
    }
}

struct AuthPractice<'a>(&'a dyn physician_auth::AuthStore);

#[async_trait::async_trait]
impl local_product::runtime::jobs::PracticeAuthority for AuthPractice<'_> {
    async fn check_practice_patient(
        &self,
        scope: &local_product::runtime::jobs::PracticeScope,
    ) -> Result<(), local_product::runtime::jobs::PracticeAccessError> {
        <dyn physician_auth::AuthStore as local_product::runtime::jobs::PracticeAuthority>::check_practice_patient(
            self.0, scope,
        )
        .await
    }
}

/// Lists saved patient reviews on the non-Pub/Sub router: serves from Postgres
/// when [`StandaloneReviews`] is attached, or returns an empty list on static fixtures.
pub(crate) async fn list_fixture(
    Path(id): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    if let Some(standalone) = request.extensions().get::<StandaloneReviews>().cloned() {
        return handle_list(
            &ReviewCtx::from_standalone(&standalone),
            id,
            &connected,
            request,
        )
        .await;
    }
    if !crate::on_panel_in_practice(&connected, request.extensions(), &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    Json(PatientReviewsResponse {
        reviews: Vec::new(),
    })
    .into_response()
}

/// Reads a single saved patient review on the non-Pub/Sub router.
pub(crate) async fn get_fixture(
    Path((id, review_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    if let Some(standalone) = request.extensions().get::<StandaloneReviews>().cloned() {
        return handle_get(
            &ReviewCtx::from_standalone(&standalone),
            id,
            review_id,
            &connected,
            request,
        )
        .await;
    }
    if !crate::on_panel_in_practice(&connected, request.extensions(), &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    StatusCode::NOT_FOUND.into_response()
}

/// Executes a review control on the non-Pub/Sub router when Postgres is attached,
/// or refuses (`503`) on pure static fixtures.
pub(crate) async fn control_fixture(
    Path((id, review_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    if let Some(standalone) = request.extensions().get::<StandaloneReviews>().cloned() {
        return handle_control(
            &ReviewCtx::from_standalone(&standalone),
            id,
            review_id,
            &connected,
            request,
        )
        .await;
    }
    if !crate::on_panel_in_practice(&connected, request.extensions(), &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    StatusCode::SERVICE_UNAVAILABLE.into_response()
}

/// Lists saved patient reviews for an authorized workspace on the durable site.
pub(crate) async fn list_durable(
    State(live): State<Arc<Live>>,
    Path(id): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    handle_list(&ReviewCtx::from_live(&live), id, &connected, request).await
}

/// Reads one saved patient review for an authorized workspace on the durable site.
pub(crate) async fn get_durable(
    State(live): State<Arc<Live>>,
    Path((id, review_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    handle_get(
        &ReviewCtx::from_live(&live),
        id,
        review_id,
        &connected,
        request,
    )
    .await
}

/// Executes a versioned `Stop`, `Resume`, or `Try again` control on one patient review.
pub(crate) async fn control_durable(
    State(live): State<Arc<Live>>,
    Path((id, review_id)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    handle_control(
        &ReviewCtx::from_live(&live),
        id,
        review_id,
        &connected,
        request,
    )
    .await
}

async fn handle_list(
    ctx: &ReviewCtx<'_>,
    id: String,
    connected: &ConnectedCare,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if !crate::on_panel_in_practice(connected, extensions, &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let authority = AuthPractice(&*auth_store);
    let reviews = match ctx
        .store
        .patient_reviews(&identity.practice_id, &id, &authority)
        .await
    {
        Ok(reviews) => reviews,
        Err(
            PatientReviewError::Access(_)
            | PatientReviewError::PhysicianUnauthorized { .. }
            | PatientReviewError::NotFound { .. }
            | PatientReviewError::InvalidScope,
        ) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!(%error, "patient reviews list read failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };
    if reviews.is_empty() {
        return Json(PatientReviewsResponse {
            reviews: Vec::new(),
        })
        .into_response();
    }

    let Some(context) = load_workspace_source_context(ctx.store, &id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let mut projected = Vec::with_capacity(reviews.len());
    for review in &reviews {
        let job = match ctx.store.practice_job(&review.practice_scope()).await {
            Ok(job) => job,
            Err(error) => {
                tracing::error!(%error, review_id = %review.review_id, "practice job read failed");
                return StatusCode::SERVICE_UNAVAILABLE.into_response();
            }
        };
        projected.push(project_patient_review(review, job.as_ref(), &context));
    }
    Json(PatientReviewsResponse { reviews: projected }).into_response()
}

async fn handle_get(
    ctx: &ReviewCtx<'_>,
    id: String,
    review_id: String,
    connected: &ConnectedCare,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if !crate::on_panel_in_practice(connected, extensions, &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let authority = AuthPractice(&*auth_store);
    match load_projected_review(
        ctx.store,
        &identity.practice_id,
        &id,
        &review_id,
        &authority,
    )
    .await
    {
        Ok(Some(wire)) => Json(wire).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(LoadReviewFailure::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(LoadReviewFailure::Unavailable) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

async fn handle_control(
    ctx: &ReviewCtx<'_>,
    id: String,
    review_id: String,
    connected: &ConnectedCare,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if !crate::on_panel_in_practice(connected, extensions, &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let authority = AuthPractice(&*auth_store);

    let Ok(bytes) = axum::body::to_bytes(body, 64 * 1024).await else {
        return refused(
            StatusCode::BAD_REQUEST,
            ReviewControlRefusalReason::InvalidRequest,
            "The review control request could not be read.",
            None,
        );
    };
    let Ok(command) = serde_json::from_slice::<ReviewControlRequest>(&bytes) else {
        return refused(
            StatusCode::BAD_REQUEST,
            ReviewControlRefusalReason::InvalidRequest,
            "The review control request was malformed.",
            None,
        );
    };
    if command.review_id.trim() != review_id.trim()
        || command.version.as_str().trim().is_empty()
        || command.idempotency_key.trim().is_empty()
    {
        return refused(
            StatusCode::BAD_REQUEST,
            ReviewControlRefusalReason::InvalidRequest,
            "Review controls require the review ID, current version, and command key.",
            None,
        );
    }

    let saved = match ctx
        .store
        .patient_review(&identity.practice_id, &id, &review_id, &authority)
        .await
    {
        Ok(Some(saved)) => saved,
        Ok(None)
        | Err(
            PatientReviewError::Access(_)
            | PatientReviewError::PhysicianUnauthorized { .. }
            | PatientReviewError::NotFound { .. }
            | PatientReviewError::InvalidScope,
        ) => {
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(error) => {
            tracing::error!(%error, "patient review read failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };

    let target = PatientReviewTarget {
        practice: &identity.practice_id,
        workspace: &id,
        review_id: &review_id,
    };
    let owned_roles;
    let versioned_action = match command.action {
        ReviewControlActionWire::Stop => VersionedReviewAction::Stop,
        ReviewControlActionWire::Resume => {
            let Some(skills) = ctx.skills else {
                return StatusCode::SERVICE_UNAVAILABLE.into_response();
            };
            let roles = match ctx.roles {
                Some(roles) => roles,
                None => {
                    owned_roles = fallback_roles(&saved.recipe_version);
                    &owned_roles
                }
            };
            VersionedReviewAction::Resume { roles, skills }
        }
        ReviewControlActionWire::TryAgain => {
            let Some(skills) = ctx.skills else {
                return StatusCode::SERVICE_UNAVAILABLE.into_response();
            };
            let roles = match ctx.roles {
                Some(roles) => roles,
                None => {
                    owned_roles = fallback_roles(&saved.recipe_version);
                    &owned_roles
                }
            };
            VersionedReviewAction::TryAgain { roles, skills }
        }
    };

    match ctx
        .store
        .control_patient_review_versioned(
            target,
            command.version.as_str(),
            command.idempotency_key.trim(),
            versioned_action,
            &authority,
        )
        .await
    {
        Ok(VersionedReviewControl::Accepted { delivery }) => {
            if let (Some(chat), Some(delivery)) = (ctx.chat, delivery.as_ref()) {
                chat.publish_practice_job(delivery).await;
            }
            if let Err(error) = ctx
                .bus
                .publish(BusEvent::WorkspaceChanged {
                    workspace_id: id.clone(),
                })
                .await
            {
                tracing::warn!(%error, "patient review workspace_changed publish failed");
            }
            match load_projected_review(
                ctx.store,
                &identity.practice_id,
                &id,
                &review_id,
                &authority,
            )
            .await
            {
                Ok(Some(updated)) => Json(ReviewControlResponse::Accepted {
                    review: Box::new(updated),
                })
                .into_response(),
                Ok(None) | Err(LoadReviewFailure::NotFound) => {
                    StatusCode::NOT_FOUND.into_response()
                }
                Err(LoadReviewFailure::Unavailable) => {
                    StatusCode::SERVICE_UNAVAILABLE.into_response()
                }
            }
        }
        Ok(VersionedReviewControl::StaleVersion { .. }) => {
            let latest = load_projected_review(
                ctx.store,
                &identity.practice_id,
                &id,
                &review_id,
                &authority,
            )
            .await
            .ok()
            .flatten();
            refused(
                StatusCode::CONFLICT,
                ReviewControlRefusalReason::StaleVersion,
                "This review changed before your request arrived. Showing its current status.",
                latest,
            )
        }
        Ok(VersionedReviewControl::InvalidState) => {
            let latest = load_projected_review(
                ctx.store,
                &identity.practice_id,
                &id,
                &review_id,
                &authority,
            )
            .await
            .ok()
            .flatten();
            let message = latest
                .as_ref()
                .map(|wire| invalid_state_message(command.action, &wire.state))
                .unwrap_or_else(|| {
                    "That control does not apply to the review’s current status.".to_owned()
                });
            refused(
                StatusCode::CONFLICT,
                ReviewControlRefusalReason::InvalidState,
                &message,
                latest,
            )
        }
        Ok(VersionedReviewControl::IdempotencyConflict)
        | Err(PatientReviewError::IdempotencyCollision { .. }) => {
            let latest = load_projected_review(
                ctx.store,
                &identity.practice_id,
                &id,
                &review_id,
                &authority,
            )
            .await
            .ok()
            .flatten();
            refused(
                StatusCode::CONFLICT,
                ReviewControlRefusalReason::IdempotencyCollision,
                "That control request key was already used for a different action.",
                latest,
            )
        }
        Err(
            PatientReviewError::NotFound { .. }
            | PatientReviewError::Access(_)
            | PatientReviewError::PhysicianUnauthorized { .. },
        ) => StatusCode::NOT_FOUND.into_response(),
        Err(PatientReviewError::InvalidScope | PatientReviewError::InvalidInput { .. }) => refused(
            StatusCode::BAD_REQUEST,
            ReviewControlRefusalReason::InvalidRequest,
            "The review control request was invalid.",
            None,
        ),
        Err(PatientReviewError::ControlRefused { .. }) => {
            let latest = load_projected_review(
                ctx.store,
                &identity.practice_id,
                &id,
                &review_id,
                &authority,
            )
            .await
            .ok()
            .flatten();
            let message = latest
                .as_ref()
                .map(|wire| invalid_state_message(command.action, &wire.state))
                .unwrap_or_else(|| {
                    "That control does not apply to the review’s current status.".to_owned()
                });
            refused(
                StatusCode::CONFLICT,
                ReviewControlRefusalReason::InvalidState,
                &message,
                latest,
            )
        }
        Err(PatientReviewError::Assignment(error)) => {
            tracing::error!(%error, "patient review control assignment failed");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
        Err(PatientReviewError::Runtime(error)) => {
            tracing::error!(%error, "patient review control runtime failed");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
        Err(PatientReviewError::Database(error)) => {
            tracing::error!(%error, "patient review control database failed");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

enum LoadReviewFailure {
    NotFound,
    Unavailable,
}

struct WorkspaceSourceContext {
    by_pair: BTreeMap<(String, String), SourceRevision>,
    latest_by_candidate: BTreeMap<String, SourceRevision>,
    source_labels: BTreeMap<String, String>,
}

async fn load_workspace_source_context(
    store: &local_product::runtime::Store,
    workspace_id: &str,
) -> Option<WorkspaceSourceContext> {
    let observation = match store.sources(workspace_id).await {
        Ok(observation) => observation,
        Err(error) => {
            tracing::error!(%error, "patient review source observation read failed");
            return None;
        }
    };
    let pulls = match store.source_pulls(&[workspace_id.to_owned()]).await {
        Ok(mut map) => map.remove(workspace_id).unwrap_or_default(),
        Err(error) => {
            tracing::error!(%error, "patient review source pulls read failed");
            return None;
        }
    };
    let source_labels: BTreeMap<String, String> = pulls
        .into_iter()
        .map(|pull| (pull.source_id, pull.source_label))
        .collect();
    let mut by_pair = BTreeMap::new();
    let mut latest_by_candidate = BTreeMap::new();
    for source in observation.sources {
        latest_by_candidate.insert(source.candidate_id.clone(), source.clone());
        by_pair.insert(
            (source.candidate_id.clone(), source.revision.clone()),
            source,
        );
    }
    Some(WorkspaceSourceContext {
        by_pair,
        latest_by_candidate,
        source_labels,
    })
}

async fn load_projected_review(
    store: &local_product::runtime::Store,
    practice_id: &str,
    workspace_id: &str,
    review_id: &str,
    authority: &dyn local_product::runtime::jobs::PracticeAuthority,
) -> Result<Option<PatientReviewWire>, LoadReviewFailure> {
    let review = match store
        .patient_review(practice_id, workspace_id, review_id, authority)
        .await
    {
        Ok(Some(review)) => review,
        Ok(None) => return Ok(None),
        Err(
            PatientReviewError::Access(_)
            | PatientReviewError::PhysicianUnauthorized { .. }
            | PatientReviewError::NotFound { .. }
            | PatientReviewError::InvalidScope,
        ) => return Err(LoadReviewFailure::NotFound),
        Err(error) => {
            tracing::error!(%error, "patient review read failed");
            return Err(LoadReviewFailure::Unavailable);
        }
    };
    let context = load_workspace_source_context(store, workspace_id)
        .await
        .ok_or(LoadReviewFailure::Unavailable)?;
    let job = store
        .practice_job(&review.practice_scope())
        .await
        .map_err(|error| {
            tracing::error!(%error, "patient review practice job read failed");
            LoadReviewFailure::Unavailable
        })?;
    Ok(Some(project_patient_review(
        &review,
        job.as_ref(),
        &context,
    )))
}

fn refused(
    status: StatusCode,
    reason: ReviewControlRefusalReason,
    message: &str,
    review: Option<PatientReviewWire>,
) -> Response {
    (
        status,
        Json(ReviewControlResponse::Refused {
            reason,
            message: message.to_owned(),
            review: review.map(Box::new),
        }),
    )
        .into_response()
}

fn invalid_state_message(
    action: ReviewControlActionWire,
    state: &PatientReviewStateWire,
) -> String {
    match (action, state) {
        (ReviewControlActionWire::Stop, PatientReviewStateWire::Completed { .. }) => {
            "This review has already finished.".to_owned()
        }
        (ReviewControlActionWire::Stop, PatientReviewStateWire::Held { .. }) => {
            "This review is already paused.".to_owned()
        }
        (ReviewControlActionWire::Stop, _) => "This review is no longer running.".to_owned(),
        (
            ReviewControlActionWire::Resume,
            PatientReviewStateWire::Pending | PatientReviewStateWire::Running { .. },
        ) => "This review is already running.".to_owned(),
        (ReviewControlActionWire::Resume, PatientReviewStateWire::Completed { .. }) => {
            "This review has already finished.".to_owned()
        }
        (ReviewControlActionWire::Resume, _) => "Only a stopped review can be resumed.".to_owned(),
        (
            ReviewControlActionWire::TryAgain,
            PatientReviewStateWire::Pending | PatientReviewStateWire::Running { .. },
        ) => "This review is already running.".to_owned(),
        (ReviewControlActionWire::TryAgain, PatientReviewStateWire::Completed { .. }) => {
            "This review has already finished.".to_owned()
        }
        (ReviewControlActionWire::TryAgain, _) => {
            "Try again is available only when a review couldn’t finish.".to_owned()
        }
    }
}

/// Computes the single authoritative [`ReviewVersion`] for `(review, job)` so
/// every lifecycle, run-history, input, or action transition advances the token.
pub(crate) fn review_version(
    review: &SavedPatientReview,
    job: Option<&PracticeJob>,
) -> ReviewVersion {
    ReviewVersion(patient_review_version(review, job))
}

fn project_patient_review(
    review: &SavedPatientReview,
    job: Option<&PracticeJob>,
    context: &WorkspaceSourceContext,
) -> PatientReviewWire {
    let version = review_version(review, job);
    let triggers = review
        .triggers
        .iter()
        .map(|trigger| project_trigger(trigger, context))
        .collect();
    let accepted_inputs = review
        .accepted_inputs
        .iter()
        .map(|input| ReviewAcceptedInputWire {
            id: input.id.clone(),
            text: input.text.clone(),
            recorded_at: unix_to_rfc3339(input.recorded_at),
        })
        .collect();
    let (evidence, sources, readback_sources) = collect_review_evidence(review, context);
    let steps = job
        .map(|j| extract_steps(&j.attempts, &review.state, context))
        .unwrap_or_default();
    let state = project_state(&review.state, &readback_sources);
    let actions = review.actions.iter().map(project_action).collect();
    let unfinished_work = review
        .unfinished_work
        .iter()
        .map(project_unfinished)
        .collect();

    PatientReviewWire {
        review_id: review.review_id.clone(),
        version,
        work_id: review.work_id.clone(),
        triggers,
        accepted_inputs,
        state,
        steps,
        evidence,
        sources,
        actions,
        unfinished_work,
        queued_at: unix_to_rfc3339(review.queued_at),
        updated_at: unix_to_rfc3339(review.updated_at),
    }
}

fn project_trigger(trigger: &ReviewTrigger, context: &WorkspaceSourceContext) -> ReviewTriggerWire {
    match trigger {
        ReviewTrigger::ChangedRecords {
            workspace_version,
            changed_candidates,
            ..
        } => {
            let changed_records = changed_candidates
                .iter()
                .map(|candidate_id| {
                    context
                        .latest_by_candidate
                        .get(candidate_id)
                        .map(|src| {
                            let title = humanize_record_title(
                                &src.provenance,
                                &src.record_type,
                                candidate_id,
                            );
                            match src
                                .dates
                                .first()
                                .and_then(|d| format_trigger_date(&d.value))
                            {
                                Some(date) => format!("{title} ({date})"),
                                None => title,
                            }
                        })
                        .unwrap_or_else(|| candidate_id.clone())
                })
                .collect();
            ReviewTriggerWire::ChangedRecords {
                workspace_version: *workspace_version,
                changed_records,
            }
        }
        ReviewTrigger::Periodic {
            workspace_version,
            last_reviewed_at,
            ..
        } => ReviewTriggerWire::Periodic {
            workspace_version: *workspace_version,
            last_reviewed_at: last_reviewed_at.map(unix_to_rfc3339),
        },
        ReviewTrigger::PhysicianRequest {
            physician_id,
            request_text,
            workspace_version,
            ..
        } => ReviewTriggerWire::PhysicianRequest {
            physician_id: physician_id.clone(),
            request_text: request_text.clone(),
            workspace_version: *workspace_version,
        },
    }
}

fn project_state(
    state: &ReviewLifecycleState,
    readback_sources: &[CitedSourceReadback],
) -> PatientReviewStateWire {
    match state {
        ReviewLifecycleState::Pending => PatientReviewStateWire::Pending,
        ReviewLifecycleState::Running {
            attempt_id,
            started_at,
        } => PatientReviewStateWire::Running {
            attempt_id: attempt_id.clone(),
            started_at: unix_to_rfc3339(*started_at),
        },
        ReviewLifecycleState::Held {
            reason,
            held_at,
            partial_text,
        } => PatientReviewStateWire::Held {
            reason: match reason {
                ReviewHoldReason::Stopped => ReviewHoldReasonWire::Stopped,
                ReviewHoldReason::Limit => ReviewHoldReasonWire::Limit,
            },
            held_at: unix_to_rfc3339(*held_at),
            partial_text: partial_text.clone().filter(|t| !t.trim().is_empty()),
        },
        ReviewLifecycleState::Completed {
            outcome,
            completed_at,
            ..
        } => PatientReviewStateWire::Completed {
            outcome: project_outcome(outcome, readback_sources),
            completed_at: unix_to_rfc3339(*completed_at),
        },
        ReviewLifecycleState::Failed {
            reason, failed_at, ..
        } => PatientReviewStateWire::Failed {
            reason: reason.clone(),
            failed_at: unix_to_rfc3339(*failed_at),
        },
        ReviewLifecycleState::Superseded {
            reason,
            superseded_at,
        } => PatientReviewStateWire::Superseded {
            reason: match reason {
                ReviewSupersededReason::AccessLost { detail } => {
                    ReviewSupersededReasonWire::AccessLost {
                        detail: detail.clone(),
                    }
                }
                ReviewSupersededReason::NewInput {
                    replacement_review_id,
                } => ReviewSupersededReasonWire::NewInput {
                    replacement_review_id: replacement_review_id.clone(),
                },
            },
            superseded_at: unix_to_rfc3339(*superseded_at),
        },
    }
}

fn project_outcome(
    outcome: &ReviewOutcome,
    readback_sources: &[CitedSourceReadback],
) -> PatientReviewOutcomeWire {
    match outcome {
        ReviewOutcome::Quiet {
            explanation,
            citations,
            basis_and_limitations,
            ..
        } => PatientReviewOutcomeWire::Quiet {
            explanation: explanation.clone(),
            segments: build_outcome_segments(explanation, citations, readback_sources),
            basis: build_basis_segments(basis_and_limitations, readback_sources),
        },
        ReviewOutcome::Assessed {
            summary,
            citations,
            basis_and_limitations,
        } => PatientReviewOutcomeWire::Assessed {
            summary: summary.clone(),
            segments: build_outcome_segments(summary, citations, readback_sources),
            basis: build_basis_segments(basis_and_limitations, readback_sources),
        },
    }
}

fn build_outcome_segments(
    text: &str,
    citations: &[ReviewCitation],
    readback_sources: &[CitedSourceReadback],
) -> Vec<MessageSegment> {
    let mut segments: Vec<MessageSegment> = parse_segments_with_citations(text, readback_sources)
        .iter()
        .map(history_to_wire_segment)
        .collect();
    let already_cited: BTreeSet<String> = segments
        .iter()
        .filter_map(|seg| match seg {
            MessageSegment::Citation { source_key, .. } => Some(source_key.clone()),
            MessageSegment::Text { .. } => None,
        })
        .collect();
    for citation in citations {
        if let Some(source) = readback_sources.iter().find(|item| {
            item.candidate_id == citation.reference.candidate_id
                && item.revision == citation.reference.revision
        }) && !already_cited.contains(&source.source_key)
        {
            if !segments.is_empty() {
                segments.push(MessageSegment::Text {
                    text: " ".to_owned(),
                });
            }
            segments.push(MessageSegment::Citation {
                source_key: source.source_key.clone(),
                number: source.number,
                label: format!("{} · {}", source.source_label, source.title),
                quote: source.quote.clone(),
            });
        }
    }
    segments
}

fn build_basis_segments(
    basis_text: &str,
    readback_sources: &[CitedSourceReadback],
) -> Vec<MessageSegment> {
    if basis_text.trim().is_empty() {
        return Vec::new();
    }
    parse_segments_with_citations(basis_text, readback_sources)
        .iter()
        .map(history_to_wire_segment)
        .collect()
}

fn history_to_wire_segment(segment: &HistorySegment) -> MessageSegment {
    match segment {
        HistorySegment::Text { text } => MessageSegment::Text { text: text.clone() },
        HistorySegment::Citation {
            source_key,
            number,
            label,
            quote,
        } => MessageSegment::Citation {
            source_key: source_key.clone(),
            number: *number,
            label: label.clone(),
            quote: quote.clone(),
        },
    }
}

fn collect_review_evidence(
    review: &SavedPatientReview,
    context: &WorkspaceSourceContext,
) -> (
    Vec<ReviewEvidenceRefWire>,
    BTreeMap<String, CitedSourceWire>,
    Vec<CitedSourceReadback>,
) {
    let mut evidence = Vec::new();
    let mut sources = BTreeMap::new();
    let mut readback = Vec::new();
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();

    if let Some(outcome) = review.completed_outcome() {
        let citations = match outcome {
            ReviewOutcome::Quiet { citations, .. } | ReviewOutcome::Assessed { citations, .. } => {
                citations
            }
        };
        for citation in citations {
            let pair = (
                citation.reference.candidate_id.clone(),
                citation.reference.revision.clone(),
            );
            if !seen.insert(pair.clone()) {
                continue;
            }
            let number = evidence.len() + 1;
            let source_key = format!("review-{}-src-{number}", review.review_id);
            let observed = context.by_pair.get(&pair);
            let record_type = observed.map(|s| s.record_type.clone()).unwrap_or_default();
            let body = observed
                .map(|s| {
                    format_cited_source_body(&citation.display_label, &s.readable_parts, &s.content)
                })
                .filter(|b| !b.trim().is_empty())
                .unwrap_or_else(|| citation.excerpt.clone());
            let raw_content = observed.map(|s| s.content.clone()).unwrap_or_default();
            let external_url =
                observed.and_then(|s| provenance_str(&s.provenance, &["url", "external_url"]));
            let quote = if citation.excerpt.trim().is_empty() {
                truncate_chars(&body, 220)
            } else {
                citation.excerpt.clone()
            };

            evidence.push(ReviewEvidenceRefWire {
                source_key: source_key.clone(),
                number,
                candidate_id: citation.reference.candidate_id.clone(),
                revision: citation.reference.revision.clone(),
                source_label: citation.source_label.clone(),
                display_label: citation.display_label.clone(),
                as_of: citation.as_of.clone(),
                excerpt: quote.clone(),
            });
            sources.insert(
                source_key.clone(),
                CitedSourceWire {
                    source_key: source_key.clone(),
                    number,
                    source_label: citation.source_label.clone(),
                    title: citation.display_label.clone(),
                    record_type: (!record_type.is_empty()).then_some(record_type.clone()),
                    recorded_at: citation.as_of.clone(),
                    quote: quote.clone(),
                    body: body.clone(),
                    raw_content: (!raw_content.is_empty()).then_some(raw_content.clone()),
                    external_url: external_url.clone(),
                },
            );
            readback.push(CitedSourceReadback {
                source_key,
                number,
                source_id: citation.source_id.clone(),
                source_label: citation.source_label.clone(),
                candidate_id: citation.reference.candidate_id.clone(),
                revision: citation.reference.revision.clone(),
                title: citation.display_label.clone(),
                record_type,
                recorded_at: citation.as_of.clone(),
                quote,
                body,
                raw_content,
                external_url,
            });
        }
    }

    for ExactRecordReference {
        candidate_id,
        revision,
    } in &review.evidence_read
    {
        let pair = (candidate_id.clone(), revision.clone());
        if !seen.insert(pair.clone()) {
            continue;
        }
        let number = evidence.len() + 1;
        let source_key = format!("review-{}-src-{number}", review.review_id);
        if let Some(src) = context.by_pair.get(&pair) {
            let source_label = context
                .source_labels
                .get(&src.source_id)
                .cloned()
                .unwrap_or_else(|| src.source_id.clone());
            let title = cited_record_title(&src.provenance, &src.record_type, candidate_id);
            let body = format_cited_source_body(&title, &src.readable_parts, &src.content);
            let quote = truncate_chars(&body, 220);
            let as_of = src.dates.first().map(|d| d.value.clone());
            let external_url = provenance_str(&src.provenance, &["url", "external_url"]);

            evidence.push(ReviewEvidenceRefWire {
                source_key: source_key.clone(),
                number,
                candidate_id: candidate_id.clone(),
                revision: revision.clone(),
                source_label: source_label.clone(),
                display_label: title.clone(),
                as_of: as_of.clone(),
                excerpt: quote.clone(),
            });
            sources.insert(
                source_key.clone(),
                CitedSourceWire {
                    source_key: source_key.clone(),
                    number,
                    source_label: source_label.clone(),
                    title: title.clone(),
                    record_type: (!src.record_type.is_empty()).then(|| src.record_type.clone()),
                    recorded_at: as_of.clone(),
                    quote: quote.clone(),
                    body: body.clone(),
                    raw_content: (!src.content.is_empty()).then(|| src.content.clone()),
                    external_url: external_url.clone(),
                },
            );
            readback.push(CitedSourceReadback {
                source_key,
                number,
                source_id: src.source_id.clone(),
                source_label,
                candidate_id: candidate_id.clone(),
                revision: revision.clone(),
                title,
                record_type: src.record_type.clone(),
                recorded_at: as_of,
                quote,
                body,
                raw_content: src.content.clone(),
                external_url,
            });
        } else {
            evidence.push(ReviewEvidenceRefWire {
                source_key: source_key.clone(),
                number,
                candidate_id: candidate_id.clone(),
                revision: revision.clone(),
                source_label: "Chart record".to_owned(),
                display_label: candidate_id.clone(),
                as_of: None,
                excerpt: format!("Revision {revision}"),
            });
        }
    }

    (evidence, sources, readback)
}

fn extract_steps(
    attempts: &[PracticeJobAttempt],
    review_state: &ReviewLifecycleState,
    context: &WorkspaceSourceContext,
) -> Vec<ReviewStepWire> {
    let mut steps: Vec<(String, ReviewStepWire)> = Vec::new();
    let unfinished_step_state = match review_state {
        ReviewLifecycleState::Pending | ReviewLifecycleState::Running { .. } => {
            ReviewStepStateWire::Running
        }
        ReviewLifecycleState::Held {
            reason: ReviewHoldReason::Stopped,
            ..
        } => ReviewStepStateWire::Stopped,
        ReviewLifecycleState::Held {
            reason: ReviewHoldReason::Limit,
            ..
        }
        | ReviewLifecycleState::Failed { .. }
        | ReviewLifecycleState::Superseded { .. } => ReviewStepStateWire::Failed,
        ReviewLifecycleState::Completed { .. } => ReviewStepStateWire::Done,
    };

    for attempt in attempts {
        for event in &attempt.events {
            match &event.kind {
                EventKind::ToolStarted(call) => {
                    if let Some((label, detail)) =
                        describe_tool_step(&call.name, &call.arguments, context)
                    {
                        steps.push((
                            call.id.clone(),
                            ReviewStepWire {
                                label,
                                detail,
                                state: unfinished_step_state,
                            },
                        ));
                    }
                }
                EventKind::ToolReturned { call_id, result } => {
                    let settled = match result {
                        ToolResult::Original(_) | ToolResult::History(_) | ToolResult::View(_) => {
                            ReviewStepStateWire::Done
                        }
                        ToolResult::Refused(_) => ReviewStepStateWire::Failed,
                    };
                    if let Some((_, step)) = steps.iter_mut().rev().find(|(id, _)| id == call_id) {
                        step.state = settled;
                    }
                }
                EventKind::ProviderStarted { .. }
                | EventKind::ProviderObservation { .. }
                | EventKind::ProviderReturned(_)
                | EventKind::ProviderFailed { .. }
                | EventKind::TransportRetry { .. }
                | EventKind::ProviderBlocked { .. }
                | EventKind::ContextRejected
                | EventKind::Candidate { .. }
                | EventKind::WindowOpened { .. } => {}
            }
        }
    }
    steps.into_iter().map(|(_, step)| step).collect()
}

fn describe_tool_step(
    name: &str,
    arguments: &str,
    context: &WorkspaceSourceContext,
) -> Option<(String, Option<String>)> {
    let parsed: Option<Value> = serde_json::from_str(arguments).ok();
    match name {
        SEARCH_TOOL => {
            let query = parsed
                .as_ref()
                .and_then(|v| v.get("query"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned);
            Some(("Searched patient chart".to_owned(), query))
        }
        "read_patient_sources" => {
            let titles: Vec<String> = parsed
                .as_ref()
                .and_then(|v| v.get("references"))
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|entry| {
                            let id = entry
                                .get("candidate_id")
                                .and_then(Value::as_str)
                                .or_else(|| entry.get("reference").and_then(Value::as_str))
                                .or_else(|| entry.as_str())
                                .map(str::trim)
                                .filter(|s| !s.is_empty())?;
                            Some(
                                context
                                    .latest_by_candidate
                                    .get(id)
                                    .map(|s| {
                                        humanize_record_title(&s.provenance, &s.record_type, id)
                                    })
                                    .unwrap_or_else(|| id.to_owned()),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default();
            let detail = (!titles.is_empty()).then(|| titles.join(", "));
            Some(("Read chart records".to_owned(), detail))
        }
        "refresh_patient_sources" => Some(("Checked connected sources".to_owned(), None)),
        READ_PATIENT_REVIEWS_TOOL => Some(("Checked prior patient reviews".to_owned(), None)),
        COMPLETE_PATIENT_REVIEW_TOOL => Some(("Recorded review outcome".to_owned(), None)),
        "read_patient_understanding" => Some(("Read patient summary".to_owned(), None)),
        "save_patient_understanding" => Some(("Updated patient summary".to_owned(), None)),
        "patient_concern" => Some(("Updated Today concern".to_owned(), None)),
        PATIENT_MEMORY_TOOL => Some(("Updated patient clinical memory".to_owned(), None)),
        "retrieve_patient_work" => Some(("Checked prior patient conversations".to_owned(), None)),
        "search_clinical_library" | "read_clinical_library" => {
            Some(("Checked clinical guidance".to_owned(), None))
        }
        "select_visit_preparations" | "save_visit_briefing" => {
            Some(("Prepared visit briefing".to_owned(), None))
        }
        "finish_reply_revision" => Some(("Updated patient reply draft".to_owned(), None)),
        "skill_view"
        | "save_working_note"
        | "working_notes"
        | "read_tool_result"
        | "list_conversation_history"
        | "read_conversation_message"
        | "search_conversation_history"
        | "history_list"
        | "history_read"
        | "history_search" => None,
        _ => Some(("Checked clinical context".to_owned(), None)),
    }
}

fn project_action(action: &RecordedReviewAction) -> ReviewActionWire {
    let summary = match action {
        RecordedReviewAction::ConcernOpen {
            urgency, reason, ..
        } => format!("Raised {} concern on Today: {reason}", urgency.as_str()),
        RecordedReviewAction::ConcernRetired {
            retirement_reason, ..
        } => format!("Retired Today concern: {retirement_reason}"),
        RecordedReviewAction::MemoryRecorded { topic, .. } => {
            format!("Updated clinical memory: {topic}")
        }
        RecordedReviewAction::MemoryRetired { reason, .. } => {
            format!("Retired clinical memory: {reason}")
        }
        RecordedReviewAction::UnderstandingSaved {
            revision,
            source_version,
        } => format!("Updated patient summary (revision {revision}, chart v{source_version})"),
        RecordedReviewAction::SourceRefreshed {
            source_id,
            workspace_version_after,
        } => format!("Refreshed {source_id} (chart v{workspace_version_after})"),
    };
    ReviewActionWire { summary }
}

fn project_unfinished(item: &UnfinishedReviewWork) -> ReviewUnfinishedWorkWire {
    let (limit_kind, limit_detail) = match &item.limit {
        Some(limit) => {
            let kind = match limit.kind {
                RecordLimitKind::Missing => ReviewLimitKindWire::Missing,
                RecordLimitKind::Unreadable => ReviewLimitKindWire::Unreadable,
                RecordLimitKind::Unavailable => ReviewLimitKindWire::Unavailable,
            };
            (Some(kind), Some(limit.detail.clone()))
        }
        None => (None, None),
    };
    ReviewUnfinishedWorkWire {
        topic: item.topic.clone(),
        detail: item.detail.clone(),
        limit_kind,
        limit_detail,
    }
}

fn unix_to_rfc3339(seconds: i64) -> String {
    format_rfc3339(UNIX_EPOCH + Duration::from_secs(seconds.max(0) as u64))
}

fn provenance_str(provenance: &Value, keys: &[&str]) -> Option<String> {
    let object = provenance.as_object()?;
    for key in keys {
        if let Some(value) = object.get(*key).and_then(Value::as_str) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_owned());
            }
        }
    }
    None
}

fn format_trigger_date(raw: &str) -> Option<&str> {
    let date = raw.split('T').next().unwrap_or(raw).trim();
    (!date.is_empty()).then_some(date)
}

fn humanize_record_title(provenance: &Value, record_type: &str, candidate_id: &str) -> String {
    let base = cited_record_title(provenance, record_type, candidate_id);
    if provenance_str(provenance, &["title", "display", "label"]).is_some() || base == candidate_id
    {
        return base;
    }
    humanize_pascal_type(&base)
}

fn humanize_pascal_type(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.contains(' ') {
        return trimmed.to_owned();
    }
    let mut out = String::with_capacity(trimmed.len() + 4);
    for (idx, ch) in trimmed.chars().enumerate() {
        if idx > 0 && ch.is_ascii_uppercase() {
            out.push(' ');
            for lower in ch.to_lowercase() {
                out.push(lower);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= max_chars {
        return collapsed;
    }
    let prefix: String = collapsed.chars().take(max_chars).collect();
    format!("{prefix}…")
}

#[cfg(test)]
mod tests {
    use local_product::runtime::reviews::{ReviewRecipeVersion, ReviewUsage};

    use super::*;

    fn sample_saved_review(state: ReviewLifecycleState, updated_at: i64) -> SavedPatientReview {
        SavedPatientReview {
            practice: "practice-1".into(),
            workspace: "pw-1".into(),
            review_id: "rev-1".into(),
            command_key: "cmd-1".into(),
            job_id: "job-1".into(),
            work_id: "work-1".into(),
            patient: None,
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 3,
                record_versions: BTreeMap::new(),
                last_reviewed_at: Some(1_700_000_000),
            }],
            accepted_inputs: Vec::new(),
            recipe_version: ReviewRecipeVersion {
                model: "gpt-5.4".into(),
                effort: "medium".into(),
                context_window_tokens: 128_000,
                max_input_tokens: 200_000,
                binding_sha256: "b".into(),
                skills_package_id: "s".into(),
                preloaded_skills: Vec::new(),
            },
            initial_source_version: 3,
            completed_source_version: None,
            evidence_read: Vec::new(),
            actions: Vec::new(),
            unfinished_work: Vec::new(),
            state,
            usage: ReviewUsage::Unknown,
            duration_ms: None,
            queued_at: 1_700_000_100,
            started_at: None,
            completed_at: None,
            updated_at,
        }
    }

    #[test]
    fn review_version_advances_across_every_state_transition_even_at_same_timestamp() {
        let pending = sample_saved_review(ReviewLifecycleState::Pending, 1_700_000_100);
        let running = sample_saved_review(
            ReviewLifecycleState::Running {
                attempt_id: "att-1".into(),
                started_at: 1_700_000_100,
            },
            1_700_000_100,
        );
        let stopped = sample_saved_review(
            ReviewLifecycleState::Held {
                reason: ReviewHoldReason::Stopped,
                held_at: 1_700_000_100,
                partial_text: None,
            },
            1_700_000_100,
        );
        let failed = sample_saved_review(
            ReviewLifecycleState::Failed {
                reason: "timeout".into(),
                failed_at: 1_700_000_100,
                attempts: 2,
            },
            1_700_000_100,
        );

        let v_pending = review_version(&pending, None);
        let v_running = review_version(&running, None);
        let v_stopped = review_version(&stopped, None);
        let v_failed = review_version(&failed, None);

        let distinct: BTreeSet<_> = [&v_pending, &v_running, &v_stopped, &v_failed]
            .into_iter()
            .collect();
        assert_eq!(distinct.len(), 4);
    }

    #[test]
    fn project_trigger_and_tool_steps_use_physician_language_and_candidate_ids() {
        use local_product::physician_sources::RecordDate;
        use serde_json::json;

        let obs = SourceRevision {
            workspace: "pw-1".into(),
            source_id: "quest".into(),
            candidate_id: "Observation/creatinine-2026-08".into(),
            revision: "rev-obs-1".into(),
            record_type: "Observation".into(),
            dates: vec![RecordDate {
                name: "effectiveDateTime".into(),
                value: "2026-08-22T09:15:00Z".into(),
            }],
            content: "{}".into(),
            sha256: "00".repeat(32),
            readable_parts: Vec::new(),
            provenance: json!({"system": "https://quest.example"}),
        };
        let doc = SourceRevision {
            workspace: "pw-1".into(),
            source_id: "ucsf".into(),
            candidate_id: "DocumentReference/consult-afib-2026".into(),
            revision: "rev-doc-1".into(),
            record_type: "DocumentReference".into(),
            dates: vec![RecordDate {
                name: "date".into(),
                value: "2026-08-20T14:30:00Z".into(),
            }],
            content: "{}".into(),
            sha256: "11".repeat(32),
            readable_parts: Vec::new(),
            provenance: json!({"system": "https://ucsf.example"}),
        };
        let context = WorkspaceSourceContext {
            by_pair: BTreeMap::from([
                (
                    (obs.candidate_id.clone(), obs.revision.clone()),
                    obs.clone(),
                ),
                (
                    (doc.candidate_id.clone(), doc.revision.clone()),
                    doc.clone(),
                ),
            ]),
            latest_by_candidate: BTreeMap::from([
                (obs.candidate_id.clone(), obs),
                (doc.candidate_id.clone(), doc),
            ]),
            source_labels: BTreeMap::new(),
        };

        let trigger = ReviewTrigger::ChangedRecords {
            workspace_version: 2,
            record_versions: BTreeMap::new(),
            changed_candidates: vec![
                "Observation/creatinine-2026-08".into(),
                "DocumentReference/consult-afib-2026".into(),
            ],
        };
        assert_eq!(
            project_trigger(&trigger, &context),
            ReviewTriggerWire::ChangedRecords {
                workspace_version: 2,
                changed_records: vec![
                    "Observation (2026-08-22)".into(),
                    "Document reference (2026-08-20)".into(),
                ],
            }
        );

        let read_args = json!({
            "references": [
                { "candidate_id": "Observation/creatinine-2026-08", "revision": "rev-obs-1", "cursor": null },
                { "candidate_id": "DocumentReference/consult-afib-2026", "revision": "rev-doc-1", "cursor": null }
            ]
        })
        .to_string();
        assert_eq!(
            describe_tool_step("read_patient_sources", &read_args, &context),
            Some((
                "Read chart records".to_owned(),
                Some("Observation, Document reference".to_owned()),
            ))
        );

        for plumbing in [
            "skill_view",
            "save_working_note",
            "working_notes",
            "read_tool_result",
            "list_conversation_history",
            "read_conversation_message",
            "search_conversation_history",
        ] {
            assert_eq!(describe_tool_step(plumbing, "{}", &context), None);
        }

        let fallback = describe_tool_step("future_internal_tool_name", "{}", &context)
            .expect("fallback step label");
        assert_eq!(fallback, ("Checked clinical context".to_owned(), None));
        assert!(!fallback.0.contains("future"));
    }
}
