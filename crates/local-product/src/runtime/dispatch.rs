//! Practice patient source checks, review dispatch, and operator cadence controls.
//!
//! On each wake-up, due registered patients in the practice have their sources
//! checked and at most one review request started when records changed
//! (`"new records"`) or the periodic review window elapsed (`"periodic"`).
//! Default cadence is a daily check and weekly review. Operators can read and
//! update cadence; physicians cannot, and billing or Connected Care state never
//! alters cadence.

use std::collections::BTreeMap;
use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use patient_work::{
    Cadence, CadenceError, CheckedSources, DispatchState, OperatorCadenceView, PendingReview,
    ReviewRequest,
};
use physician_auth::AuthStore;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::entities::patient_dispatch;
use super::jobs::{
    Assignment, PRACTICE_OWNER, PracticeAccessError, PracticeAuthority, PracticeJobError,
    PracticeJobKind, PracticeScope,
};
use super::receipt::{self, Receipt};
use super::{Accepted, Store, conversion_error, error, now};
use crate::physician_chat::PhysicianSkills;

/// Failure while checking one patient's sources on wake-up.
#[derive(Debug, thiserror::Error)]
pub enum SourceCheckError {
    /// Stored source observation lookup failed in the runtime database.
    #[error("stored source observation lookup failed: {0}")]
    Database(#[from] DbErr),
    /// External source pull failed for this patient.
    #[error("patient source pull failed: {0}")]
    Upstream(#[from] std::io::Error),
}

/// Errors returned by patient dispatch and operator cadence operations.
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    /// Practice patient membership check refused the workspace.
    #[error("{0}")]
    Access(#[from] PracticeAccessError),
    /// Proposed cadence failed domain validation.
    #[error("{0}")]
    Cadence(#[from] CadenceError),
    /// Runtime database operation failed.
    #[error("{0}")]
    Database(#[from] DbErr),
    /// Review handoff failed before completion.
    #[error("review handoff for workspace {workspace} failed: {source}")]
    ReviewHandoff {
        /// Patient workspace whose review handoff failed.
        workspace: String,
        /// Underlying practice job handoff failure.
        #[source]
        source: PracticeJobError,
    },
}

/// Checks one patient workspace's sources on wake-up and reports its current
/// source version and candidate-to-revision map.
#[async_trait]
pub trait PatientSourceChecker: Send + Sync {
    /// Pulls or inspects sources for `(practice_id, workspace_id)` at `now`.
    async fn check_patient_sources(
        &self,
        practice_id: &str,
        workspace_id: &str,
        now: i64,
    ) -> Result<CheckedSources, SourceCheckError>;
}

/// Receives a [`ReviewRequest`] decided by the dispatcher. Implementations must
/// treat `request.command_key` as idempotent.
#[async_trait]
pub trait ReviewReceiver: Send + Sync {
    /// Starts or records the requested patient review once.
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError>;
}

/// Source checker backed by the runtime store's versioned source observations
/// for a patient workspace.
pub struct StoredObservationChecker {
    store: Store,
}

impl StoredObservationChecker {
    /// Wraps `store` to inspect retained patient source observations.
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

#[async_trait]
impl PatientSourceChecker for StoredObservationChecker {
    async fn check_patient_sources(
        &self,
        practice_id: &str,
        workspace_id: &str,
        _now: i64,
    ) -> Result<CheckedSources, SourceCheckError> {
        let tx = self.store.db.begin().await?;
        let observation = super::sources::observed(&tx, workspace_id).await?;
        let previous =
            patient_dispatch::Entity::find_by_id((practice_id.to_owned(), workspace_id.to_owned()))
                .one(&tx)
                .await?
                .map(decode_dispatch_state)
                .transpose()?;
        tx.commit().await?;
        let record_versions: BTreeMap<String, String> = observation
            .sources
            .into_iter()
            .map(|source| (source.candidate_id, source.revision))
            .collect();
        let changed = match previous.and_then(|state| {
            state
                .last_checked_version
                .map(|version| (version, state.last_checked_records))
        }) {
            Some((prev_version, prev_records)) => {
                prev_version != observation.version || prev_records != record_versions
            }
            None => !record_versions.is_empty(),
        };
        Ok(if changed {
            CheckedSources::changed(observation.version, record_versions)
        } else {
            CheckedSources::unchanged(observation.version, record_versions)
        })
    }
}

/// Review receiver that hands each [`ReviewRequest`] to the Chords run store as
/// a [`PracticeJobKind::PatientReview`] practice background job.
pub struct PracticeJobReviewReceiver<'a> {
    store: Store,
    roles: &'a crate::ModelRoles,
    skills: &'a PhysicianSkills,
    authority: &'a dyn PracticeAuthority,
}

impl<'a> PracticeJobReviewReceiver<'a> {
    /// Creates a receiver feeding `store` on `roles.background` with `skills`.
    pub fn new(
        store: Store,
        roles: &'a crate::ModelRoles,
        skills: &'a PhysicianSkills,
        authority: &'a dyn PracticeAuthority,
    ) -> Self {
        Self {
            store,
            roles,
            skills,
            authority,
        }
    }
}

#[async_trait]
impl ReviewReceiver for PracticeJobReviewReceiver<'_> {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        let scope = PracticeScope {
            practice: request.practice_id.clone(),
            workspace: request.patient_workspace_id.clone(),
            job_id: request.command_key.clone(),
        };
        let reason_label = request.reason.as_str();
        let prompt = format!(
            "Conduct a {reason_label} chart review for patient workspace {} at source version {}.",
            request.patient_workspace_id, request.workspace_version
        );
        let assignment = Assignment::new(
            PracticeJobKind::PatientReview,
            request.command_key.clone(),
            prompt,
            &["patient-record-retrieval"],
            self.skills,
        )?;
        match self
            .store
            .accept_practice_job(&scope, &assignment, self.roles, self.skills, self.authority)
            .await?
        {
            Accepted::New | Accepted::Duplicate => Ok(()),
        }
    }
}

/// Failed source check reported for one patient without blocking other patients.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientCheckFailure {
    /// Patient workspace whose source check failed.
    pub patient_workspace_id: String,
    /// Failure message reported by the source checker.
    pub reason: String,
}

/// Summary of one practice dispatch wake-up pass.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeSummary {
    /// Registered patients whose check was not yet due at `now`.
    pub skipped_not_due: Vec<String>,
    /// Patients whose source check succeeded during this wake-up.
    pub checked_patients: Vec<String>,
    /// Patients whose source check failed (recorded as attempt, never success).
    pub failed_checks: Vec<PatientCheckFailure>,
    /// Review requests started (or completed from a replayed pending check).
    pub started_reviews: Vec<ReviewRequest>,
}

fn decode_dispatch_state(row: patient_dispatch::Model) -> Result<DispatchState, DbErr> {
    let cadence: Cadence = serde_json::from_value(row.cadence)
        .map_err(conversion_error("patient_dispatch.cadence", "Cadence"))?;
    let last_checked_records: BTreeMap<String, String> =
        serde_json::from_value(row.last_checked_records).map_err(conversion_error(
            "patient_dispatch.last_checked_records",
            "BTreeMap<String, String>",
        ))?;
    let pending_review: Option<PendingReview> = row
        .pending_review
        .map(|value| {
            serde_json::from_value(value).map_err(conversion_error(
                "patient_dispatch.pending_review",
                "PendingReview",
            ))
        })
        .transpose()?;
    let last_checked_version = row
        .last_checked_version
        .map(|v| {
            u64::try_from(v).map_err(conversion_error(
                "patient_dispatch.last_checked_version",
                "u64",
            ))
        })
        .transpose()?;
    let last_reviewed_version = row
        .last_reviewed_version
        .map(|v| {
            u64::try_from(v).map_err(conversion_error(
                "patient_dispatch.last_reviewed_version",
                "u64",
            ))
        })
        .transpose()?;
    let review_sequence = u64::try_from(row.review_sequence)
        .map_err(conversion_error("patient_dispatch.review_sequence", "u64"))?;
    Ok(DispatchState {
        cadence,
        registered_at: row.registered_at,
        last_attempt_at: row.last_attempt_at,
        last_success_at: row.last_success_at,
        last_checked_version,
        last_checked_records,
        last_reviewed_at: row.last_reviewed_at,
        last_reviewed_version,
        review_sequence,
        pending_review,
    })
}

async fn ensure_dispatch_row(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
    registered_at: i64,
) -> Result<(), DbErr> {
    // Leaves `cadence` unset so Postgres populates its default column value;
    // `Store::set_patient_cadence` remains the sole writer of `cadence`.
    patient_dispatch::Entity::insert(patient_dispatch::ActiveModel {
        practice: Set(practice_id.to_owned()),
        workspace: Set(workspace_id.to_owned()),
        cadence: ActiveValue::NotSet,
        registered_at: Set(registered_at),
        last_attempt_at: Set(None),
        last_success_at: Set(None),
        last_checked_version: Set(None),
        last_checked_records: ActiveValue::NotSet,
        last_reviewed_at: Set(None),
        last_reviewed_version: Set(None),
        review_sequence: ActiveValue::NotSet,
        pending_review: Set(None),
    })
    .on_conflict(
        OnConflict::columns([
            patient_dispatch::Column::Practice,
            patient_dispatch::Column::Workspace,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;
    Ok(())
}

async fn save_dispatch_progress(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
    state: &DispatchState,
) -> Result<(), DbErr> {
    let checked_version = state
        .last_checked_version
        .map(|v| i64::try_from(v).map_err(conversion_error("last_checked_version", "i64")))
        .transpose()?;
    let reviewed_version = state
        .last_reviewed_version
        .map(|v| i64::try_from(v).map_err(conversion_error("last_reviewed_version", "i64")))
        .transpose()?;
    let review_sequence =
        i64::try_from(state.review_sequence).map_err(conversion_error("review_sequence", "i64"))?;
    let records = serde_json::to_value(&state.last_checked_records).map_err(conversion_error(
        "BTreeMap<String, String>",
        "last_checked_records",
    ))?;
    let pending = state
        .pending_review
        .as_ref()
        .map(|item| {
            serde_json::to_value(item).map_err(conversion_error("PendingReview", "pending_review"))
        })
        .transpose()?;
    let active = patient_dispatch::ActiveModel {
        practice: Set(practice_id.to_owned()),
        workspace: Set(workspace_id.to_owned()),
        // Never touch `cadence` on check/review progress writes.
        cadence: ActiveValue::NotSet,
        registered_at: Set(state.registered_at),
        last_attempt_at: Set(state.last_attempt_at),
        last_success_at: Set(state.last_success_at),
        last_checked_version: Set(checked_version),
        last_checked_records: Set(records),
        last_reviewed_at: Set(state.last_reviewed_at),
        last_reviewed_version: Set(reviewed_version),
        review_sequence: Set(review_sequence),
        pending_review: Set(pending),
    };
    active.update(tx).await?;
    Ok(())
}

fn scope_for_membership(practice_id: &str, workspace_id: &str) -> PracticeScope {
    PracticeScope {
        practice: practice_id.to_owned(),
        workspace: workspace_id.to_owned(),
        job_id: "dispatch".to_owned(),
    }
}

impl Store {
    /// Sole writer of stored patient cadence across the entire product.
    /// Validates `(practice_id, patient_workspace_id)` membership, updates the
    /// stored cadence, and returns the resulting operator view.
    pub async fn set_patient_cadence<A: PracticeAuthority + ?Sized>(
        &self,
        authority: &A,
        practice_id: &str,
        patient_workspace_id: &str,
        cadence: Cadence,
        now_sec: i64,
    ) -> Result<OperatorCadenceView, DispatchError> {
        authority
            .check_practice_patient(&scope_for_membership(practice_id, patient_workspace_id))
            .await?;
        let cadence_json = serde_json::to_value(cadence)
            .map_err(conversion_error("Cadence", "patient_dispatch.cadence"))?;
        let tx = self.db.begin().await?;
        patient_dispatch::Entity::insert(patient_dispatch::ActiveModel {
            practice: Set(practice_id.to_owned()),
            workspace: Set(patient_workspace_id.to_owned()),
            cadence: Set(cadence_json),
            registered_at: Set(now_sec),
            last_attempt_at: Set(None),
            last_success_at: Set(None),
            last_checked_version: Set(None),
            last_checked_records: ActiveValue::NotSet,
            last_reviewed_at: Set(None),
            last_reviewed_version: Set(None),
            review_sequence: ActiveValue::NotSet,
            pending_review: Set(None),
        })
        .on_conflict(
            OnConflict::columns([
                patient_dispatch::Column::Practice,
                patient_dispatch::Column::Workspace,
            ])
            .update_columns([patient_dispatch::Column::Cadence])
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        let row = patient_dispatch::Entity::find_by_id((
            practice_id.to_owned(),
            patient_workspace_id.to_owned(),
        ))
        .one(&tx)
        .await?
        .ok_or_else(|| error("patient dispatch row missing after cadence write"))?;
        let state = decode_dispatch_state(row)?;
        tx.commit().await?;
        Ok(state.operator_view(practice_id, patient_workspace_id))
    }

    /// Reads the operator cadence view (`cadence`, `next_due_at`,
    /// `last_attempt_at`, `last_success_at`) for one registered patient without
    /// mutating stored state.
    pub async fn patient_dispatch_status<A: PracticeAuthority + ?Sized>(
        &self,
        authority: &A,
        practice_id: &str,
        patient_workspace_id: &str,
        now_sec: i64,
    ) -> Result<OperatorCadenceView, DispatchError> {
        authority
            .check_practice_patient(&scope_for_membership(practice_id, patient_workspace_id))
            .await?;
        let row = patient_dispatch::Entity::find_by_id((
            practice_id.to_owned(),
            patient_workspace_id.to_owned(),
        ))
        .one(&self.db)
        .await?;
        let state = match row {
            Some(existing) => decode_dispatch_state(existing)?,
            None => DispatchState::new(Cadence::default(), now_sec),
        };
        Ok(state.operator_view(practice_id, patient_workspace_id))
    }

    /// Reads the operator cadence view for every registered patient in `practice_id`.
    pub async fn practice_dispatch_statuses(
        &self,
        auth: &dyn AuthStore,
        practice_id: &str,
        now_sec: i64,
    ) -> Result<BTreeMap<String, OperatorCadenceView>, DispatchError> {
        if practice_id.trim().is_empty() {
            return Err(PracticeAccessError::InvalidScope.into());
        }
        let patients = auth.practice_patients(practice_id).await;
        let saved_rows = patient_dispatch::Entity::find()
            .filter(patient_dispatch::Column::Practice.eq(practice_id))
            .order_by_asc(patient_dispatch::Column::Workspace)
            .all(&self.db)
            .await?;
        let mut saved_by_workspace = BTreeMap::new();
        for row in saved_rows {
            let workspace = row.workspace.clone();
            saved_by_workspace.insert(workspace, decode_dispatch_state(row)?);
        }
        let mut views = BTreeMap::new();
        for workspace in patients {
            let state = saved_by_workspace
                .remove(&workspace)
                .unwrap_or_else(|| DispatchState::new(Cadence::default(), now_sec));
            let view = state.operator_view(practice_id, &workspace);
            views.insert(workspace, view);
        }
        Ok(views)
    }

    /// Phase 1 of wake-up: checks due registered patients in `practice_id` that
    /// do not already hold an undelivered `pending_review`, committing each
    /// patient's check outcome (`last_attempt_at`, `last_success_at`, and any
    /// newly decided `pending_review`) independently so a failed check on one
    /// patient never holds up other patients.
    pub async fn check_due_patients(
        &self,
        auth: &dyn AuthStore,
        practice_id: &str,
        now_sec: i64,
        checker: &dyn PatientSourceChecker,
    ) -> Result<WakeSummary, DispatchError> {
        if practice_id.trim().is_empty() {
            return Err(PracticeAccessError::InvalidScope.into());
        }
        let mut summary = WakeSummary::default();
        for workspace in auth.practice_patients(practice_id).await {
            let tx = self.db.begin().await?;
            ensure_dispatch_row(&tx, practice_id, &workspace, now_sec).await?;
            let row =
                patient_dispatch::Entity::find_by_id((practice_id.to_owned(), workspace.clone()))
                    .lock_exclusive()
                    .one(&tx)
                    .await?
                    .ok_or_else(|| error("missing patient dispatch row"))?;
            let state = decode_dispatch_state(row)?;
            tx.commit().await?;

            // A patient with an undelivered pending review awaits delivery first.
            if state.pending_review.is_some() {
                continue;
            }
            if !state.is_due_at(now_sec) {
                summary.skipped_not_due.push(workspace);
                continue;
            }

            let check_result = checker
                .check_patient_sources(practice_id, &workspace, now_sec)
                .await;

            let tx = self.db.begin().await?;
            let row =
                patient_dispatch::Entity::find_by_id((practice_id.to_owned(), workspace.clone()))
                    .lock_exclusive()
                    .one(&tx)
                    .await?
                    .ok_or_else(|| error("missing patient dispatch row"))?;
            let mut current = decode_dispatch_state(row)?;
            if current.pending_review.is_none() && current.is_due_at(now_sec) {
                match &check_result {
                    Ok(checked) => {
                        current.record_successful_check(
                            practice_id,
                            &workspace,
                            checked.clone(),
                            now_sec,
                        );
                    }
                    Err(_) => {
                        current.record_failed_check(now_sec);
                    }
                }
                save_dispatch_progress(&tx, practice_id, &workspace, &current).await?;
            }
            tx.commit().await?;

            match check_result {
                Ok(_) => summary.checked_patients.push(workspace),
                Err(err) => summary.failed_checks.push(PatientCheckFailure {
                    patient_workspace_id: workspace,
                    reason: err.to_string(),
                }),
            }
        }
        Ok(summary)
    }

    /// Phase 2 of wake-up (shared by live execution and crash replay): delivers
    /// every durable `pending_review` in `practice_id` once under a command
    /// receipt and settles `last_reviewed_at` / `last_reviewed_version`.
    pub async fn deliver_pending_reviews(
        &self,
        auth: &dyn AuthStore,
        practice_id: &str,
        receiver: &dyn ReviewReceiver,
    ) -> Result<Vec<ReviewRequest>, DispatchError> {
        if practice_id.trim().is_empty() {
            return Err(PracticeAccessError::InvalidScope.into());
        }
        let mut delivered = Vec::new();
        for workspace in auth.practice_patients(practice_id).await {
            let row =
                patient_dispatch::Entity::find_by_id((practice_id.to_owned(), workspace.clone()))
                    .one(&self.db)
                    .await?;
            let Some(row) = row else {
                continue;
            };
            let state = decode_dispatch_state(row)?;
            let Some(pending) = state.pending_review.clone() else {
                continue;
            };
            let request = pending.to_request(practice_id, &workspace);

            let receipt_key = format!("dispatch-handoff:{}", request.command_key);
            let receipt_payload = json!({
                "dispatch_review_handoff": &request,
            });

            // Check whether an earlier interrupted attempt already delivered to
            // the receiver and recorded the handoff receipt.
            let already_handed_off = {
                let tx = self.db.begin().await?;
                let existing = super::entities::command::Entity::find_by_id((
                    practice_id.to_owned(),
                    PRACTICE_OWNER.to_owned(),
                    receipt_key.clone(),
                ))
                .one(&tx)
                .await?;
                tx.commit().await?;
                existing.is_some()
            };

            if !already_handed_off {
                receiver.submit_review(&request).await.map_err(|source| {
                    DispatchError::ReviewHandoff {
                        workspace: workspace.clone(),
                        source,
                    }
                })?;
            }

            let tx = self.db.begin().await?;
            match receipt::record(
                &tx,
                practice_id,
                PRACTICE_OWNER,
                &receipt_key,
                &receipt_payload,
            )
            .await?
            {
                Receipt::New | Receipt::Same => {}
                Receipt::Different => {
                    return Err(DispatchError::Database(error(
                        "dispatch review handoff receipt collision",
                    )));
                }
            }
            let locked =
                patient_dispatch::Entity::find_by_id((practice_id.to_owned(), workspace.clone()))
                    .lock_exclusive()
                    .one(&tx)
                    .await?
                    .ok_or_else(|| error("missing patient dispatch row"))?;
            let mut current = decode_dispatch_state(locked)?;
            if current
                .pending_review
                .as_ref()
                .is_some_and(|saved| saved.command_key == request.command_key)
            {
                current.complete_pending_review(pending.queued_at);
                save_dispatch_progress(&tx, practice_id, &workspace, &current).await?;
                tx.commit().await?;
                if !already_handed_off {
                    delivered.push(request);
                }
            } else {
                tx.commit().await?;
            }
        }
        Ok(delivered)
    }

    /// Runs a complete practice dispatch wake-up: first replays any durable
    /// `pending_review` left by an interrupted wake-up, then checks all due
    /// patients, and finally delivers any newly decided reviews.
    pub async fn wake_practice_dispatch(
        &self,
        auth: &dyn AuthStore,
        practice_id: &str,
        now_sec: i64,
        checker: &dyn PatientSourceChecker,
        receiver: &dyn ReviewReceiver,
    ) -> Result<WakeSummary, DispatchError> {
        let mut started_reviews = self
            .deliver_pending_reviews(auth, practice_id, receiver)
            .await?;
        let mut summary = self
            .check_due_patients(auth, practice_id, now_sec, checker)
            .await?;
        let fresh_reviews = self
            .deliver_pending_reviews(auth, practice_id, receiver)
            .await?;
        started_reviews.extend(fresh_reviews);
        summary.started_reviews = started_reviews;
        Ok(summary)
    }

    /// Convenience helper returning the database clock for callers without an
    /// injected clock.
    pub async fn database_clock_seconds(&self) -> Result<i64, DbErr> {
        now(&self.db).await
    }
}

/// Shared state for the direct operator cadence and wake-up HTTP endpoints.
#[derive(Clone)]
pub struct DispatchEndpointState {
    /// Runtime store owning patient dispatch state.
    pub store: Store,
    /// Practice patient registry.
    pub auth: Arc<dyn AuthStore>,
    /// Patient source checker invoked on wake-up.
    pub checker: Arc<dyn PatientSourceChecker>,
    /// Review request receiver invoked on wake-up.
    pub receiver: Arc<dyn ReviewReceiver>,
}

/// Optional query clock override (`?now=...`) for deterministic operator/wake calls.
#[derive(Debug, Default, Deserialize)]
pub struct ClockQuery {
    /// Unix timestamp in seconds; defaults to the database clock when omitted.
    pub now: Option<i64>,
}

/// Request body for `PUT /operator/practices/{practice_id}/patients/{workspace_id}/cadence`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SetCadenceBody {
    /// Seconds between source checks.
    pub check_every_seconds: u64,
    /// Seconds between periodic reviews when records have not changed.
    pub review_every_seconds: u64,
}

/// Builds the direct operator cadence and scheduled wake-up router.
pub fn operator_router(state: DispatchEndpointState) -> Router {
    Router::new()
        .route(
            "/operator/practices/{practice_id}/patients/{workspace_id}/cadence",
            get(get_patient_cadence).put(put_patient_cadence),
        )
        .route(
            "/internal/practices/{practice_id}/dispatch/wake",
            post(post_wake_dispatch),
        )
        .with_state(state)
}

async fn resolve_clock(store: &Store, query: ClockQuery) -> Result<i64, (StatusCode, String)> {
    match query.now {
        Some(ts) => Ok(ts),
        None => store
            .database_clock_seconds()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

fn map_dispatch_error(err: DispatchError) -> (StatusCode, String) {
    match err {
        DispatchError::Cadence(_) => (StatusCode::BAD_REQUEST, err.to_string()),
        DispatchError::Access(PracticeAccessError::InvalidScope) => {
            (StatusCode::BAD_REQUEST, err.to_string())
        }
        DispatchError::Access(
            PracticeAccessError::UnregisteredPatient { .. }
            | PracticeAccessError::ForeignPractice { .. },
        ) => (StatusCode::NOT_FOUND, err.to_string()),
        DispatchError::Access(PracticeAccessError::StoreUnavailable(_))
        | DispatchError::Database(_)
        | DispatchError::ReviewHandoff { .. } => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    }
}

async fn get_patient_cadence(
    State(state): State<DispatchEndpointState>,
    Path((practice_id, workspace_id)): Path<(String, String)>,
    Query(clock): Query<ClockQuery>,
) -> impl IntoResponse {
    let now_sec = match resolve_clock(&state.store, clock).await {
        Ok(ts) => ts,
        Err(resp) => return resp.into_response(),
    };
    match state
        .store
        .patient_dispatch_status(state.auth.as_ref(), &practice_id, &workspace_id, now_sec)
        .await
    {
        Ok(view) => (StatusCode::OK, Json(view)).into_response(),
        Err(err) => map_dispatch_error(err).into_response(),
    }
}

async fn put_patient_cadence(
    State(state): State<DispatchEndpointState>,
    Path((practice_id, workspace_id)): Path<(String, String)>,
    Query(clock): Query<ClockQuery>,
    Json(body): Json<SetCadenceBody>,
) -> impl IntoResponse {
    let now_sec = match resolve_clock(&state.store, clock).await {
        Ok(ts) => ts,
        Err(resp) => return resp.into_response(),
    };
    let cadence = match Cadence::new(body.check_every_seconds, body.review_every_seconds) {
        Ok(valid) => valid,
        Err(err) => return map_dispatch_error(DispatchError::Cadence(err)).into_response(),
    };
    match state
        .store
        .set_patient_cadence(
            state.auth.as_ref(),
            &practice_id,
            &workspace_id,
            cadence,
            now_sec,
        )
        .await
    {
        Ok(view) => (StatusCode::OK, Json(view)).into_response(),
        Err(err) => map_dispatch_error(err).into_response(),
    }
}

async fn post_wake_dispatch(
    State(state): State<DispatchEndpointState>,
    Path(practice_id): Path<String>,
    Query(clock): Query<ClockQuery>,
) -> impl IntoResponse {
    let now_sec = match resolve_clock(&state.store, clock).await {
        Ok(ts) => ts,
        Err(resp) => return resp.into_response(),
    };
    match state
        .store
        .wake_practice_dispatch(
            state.auth.as_ref(),
            &practice_id,
            now_sec,
            state.checker.as_ref(),
            state.receiver.as_ref(),
        )
        .await
    {
        Ok(summary) => (StatusCode::OK, Json(summary)).into_response(),
        Err(err) => map_dispatch_error(err).into_response(),
    }
}
