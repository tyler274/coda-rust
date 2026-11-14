//! Practice patient source checks, message and follow-up triggers, review dispatch,
//! and operator cadence controls.
//!
//! On each wake-up, due registered patients in the practice have their sources
//! checked, linked message arrivals and due active follow-ups evaluated, and at
//! most one coalesced review request started per patient when records changed
//! (`"new records"`), a linked message arrived (`"message"`), an active
//! follow-up came due at its current version (`"follow-up"`), or the periodic
//! review window elapsed (`"periodic"`). Default cadence is a daily check and
//! weekly review. Operators can read and update cadence; physicians cannot, and
//! billing or Connected Care state never alters cadence.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use patient_work::{
    Cadence, CadenceError, CheckedSources, DispatchState, FollowUpCandidate,
    FollowUpDispatchStanding, FollowUpScheduleState, LinkedMessageTrigger, OperatorCadenceView,
    PatientWakeInputs, PendingReview, ReviewDispatchInput, ReviewReason, ReviewRequest,
    WakeSourceObservation,
};
use physician_auth::AuthStore;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::entities::{patient_dispatch, source_pull};
use super::follow_ups::{FollowUpError, FollowUpReviewSchedule, FollowUpStanding};
use super::jobs::{
    Assignment, PRACTICE_OWNER, PracticeAccessError, PracticeAuthority, PracticeJobError,
    PracticeJobKind, PracticeScope,
};
use super::receipt::{self, Receipt};
use super::reply::proposals::{
    ChannelConnection, ConversationLink, MessageSender, RecordPatientMessage, ReplyProposalError,
    ReviewWakeEvent, tables::reply_conversation,
};
use super::spruce_matching::{SpruceMatchingError, entities::spruce_review_trigger};
use super::{Accepted, Store, conversion_error, error, now};
use crate::patient_reviews::ReviewTrigger;
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::SourcePatient;

pub(in crate::runtime) mod tables {
    pub(in crate::runtime) mod dispatch_trigger {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_patient_dispatch_triggers")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub workspace: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub trigger_key: String,
            #[sea_orm(column_type = "Text")]
            pub trigger_kind: String,
            #[sea_orm(column_type = "Text")]
            pub target_id: String,
            pub target_version: Option<i64>,
            pub dispatched_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }
}

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

    /// Starts or records a coalesced patient review carrying all motivating
    /// [`ReviewDispatchInput`]s (record changes, linked messages, due follow-ups).
    async fn submit_coalesced_review(
        &self,
        request: &ReviewRequest,
        inputs: &[ReviewDispatchInput],
    ) -> Result<(), PracticeJobError> {
        let _ = inputs;
        self.submit_review(request).await
    }
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

async fn observed_record_versions(
    tx: &impl ConnectionTrait,
    workspace_id: &str,
) -> Result<(u64, BTreeMap<String, String>), DbErr> {
    let observation = super::sources::observed(tx, workspace_id).await?;
    let record_versions: BTreeMap<String, String> = observation
        .sources
        .into_iter()
        .map(|source| (source.candidate_id, source.revision))
        .collect();
    Ok((observation.version, record_versions))
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
        let (version, record_versions) = observed_record_versions(&tx, workspace_id).await?;
        let previous =
            patient_dispatch::Entity::find_by_id((practice_id.to_owned(), workspace_id.to_owned()))
                .one(&tx)
                .await?
                .map(decode_dispatch_state)
                .transpose()?;
        tx.commit().await?;
        let changed = match previous.and_then(|state| {
            state
                .last_checked_version
                .map(|prev_version| (prev_version, state.last_checked_records))
        }) {
            Some((prev_version, prev_records)) => {
                prev_version != version || prev_records != record_versions
            }
            None => !record_versions.is_empty(),
        };
        Ok(if changed {
            CheckedSources::changed(version, record_versions)
        } else {
            CheckedSources::unchanged(version, record_versions)
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
        self.submit_coalesced_review(request, &[]).await
    }

    async fn submit_coalesced_review(
        &self,
        request: &ReviewRequest,
        inputs: &[ReviewDispatchInput],
    ) -> Result<(), PracticeJobError> {
        let scope = PracticeScope {
            practice: request.practice_id.clone(),
            workspace: request.patient_workspace_id.clone(),
            job_id: request.command_key.clone(),
        };
        let is_direct_spruce_trigger = inputs.is_empty()
            && spruce_review_trigger::Entity::find_by_id((
                request.practice_id.clone(),
                request.command_key.clone(),
            ))
            .one(&self.store.db)
            .await?
            .is_some();
        let effective_reason = if is_direct_spruce_trigger {
            ReviewReason::Message
        } else {
            request.reason
        };
        let reason_label = effective_reason.as_str();
        let prompt = format!(
            "Conduct a {reason_label} chart review for patient workspace {} at source version {}.",
            request.patient_workspace_id, request.workspace_version
        );
        let includes_message = effective_reason == ReviewReason::Message
            || inputs
                .iter()
                .any(|item| matches!(item, ReviewDispatchInput::LinkedMessage(_)));
        let has_comm_skill = matches!(
            self.skills.execute(
                crate::physician_chat::SKILL_VIEW,
                &serde_json::json!({ "name": "patient-communication" }),
            ),
            harness::engine::ToolOutcome::Returned(_)
        );
        let preloaded: &[&str] = if includes_message && has_comm_skill {
            &["patient-record-retrieval", "patient-communication"]
        } else {
            &["patient-record-retrieval"]
        };
        let assignment = Assignment::new(
            PracticeJobKind::PatientReview,
            request.command_key.clone(),
            prompt,
            preloaded,
            self.skills,
        )?;
        match self
            .store
            .accept_practice_job(&scope, &assignment, self.roles, self.skills, self.authority)
            .await?
        {
            Accepted::New | Accepted::Duplicate => {
                if is_direct_spruce_trigger {
                    self.store
                        .sync_workspace_spruce_conversations_to_reply(
                            &request.practice_id,
                            &request.patient_workspace_id,
                        )
                        .await?;
                    let tx = self.store.db.begin().await?;
                    let dispatched_at = now(&tx).await?;
                    record_admitted_message_triggers_in_tx(
                        &tx,
                        &request.practice_id,
                        &request.patient_workspace_id,
                        &[],
                        &request.command_key,
                        dispatched_at,
                    )
                    .await?;
                    tx.commit().await?;
                }
                Ok(())
            }
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
    /// Registered patients whose check was not yet due at `now` and had no due event triggers.
    pub skipped_not_due: Vec<String>,
    /// Patients whose source check succeeded during this wake-up.
    pub checked_patients: Vec<String>,
    /// Patients whose source check failed (recorded as attempt, never success).
    pub failed_checks: Vec<PatientCheckFailure>,
    /// Review requests started (or completed from a replayed pending check).
    pub started_reviews: Vec<ReviewRequest>,
    /// Coalesced dispatch inputs carried by each started review, keyed by `command_key`.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub coalesced_inputs: BTreeMap<String, Vec<ReviewDispatchInput>>,
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
        dispatched_message_keys: BTreeSet::new(),
        dispatched_follow_up_versions: BTreeMap::new(),
    })
}

async fn load_dispatch_state_with_ledger(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
    row: patient_dispatch::Model,
) -> Result<DispatchState, DbErr> {
    let mut state = decode_dispatch_state(row)?;
    let ledger_rows = tables::dispatch_trigger::Entity::find()
        .filter(tables::dispatch_trigger::Column::Practice.eq(practice_id))
        .filter(tables::dispatch_trigger::Column::Workspace.eq(workspace_id))
        .all(tx)
        .await?;
    for entry in ledger_rows {
        match entry.trigger_kind.as_str() {
            "message" => {
                state.dispatched_message_keys.insert(entry.trigger_key);
            }
            "follow_up" => {
                if let Some(raw_ver) = entry.target_version {
                    let version = u64::try_from(raw_ver)
                        .map_err(conversion_error("dispatch_trigger.target_version", "u64"))?;
                    let slot = state
                        .dispatched_follow_up_versions
                        .entry(entry.target_id)
                        .or_insert(0);
                    *slot = (*slot).max(version);
                }
            }
            other => {
                return Err(error(&format!(
                    "corrupt patient dispatch trigger kind `{other}`"
                )));
            }
        }
    }
    Ok(state)
}

async fn insert_message_dispatch_trigger_in_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
    trigger_key: String,
    conversation_id: &str,
    dispatched_at: i64,
) -> Result<(), DbErr> {
    tables::dispatch_trigger::Entity::insert(tables::dispatch_trigger::ActiveModel {
        practice: Set(practice_id.to_owned()),
        workspace: Set(workspace_id.to_owned()),
        trigger_key: Set(trigger_key),
        trigger_kind: Set("message".to_owned()),
        target_id: Set(conversation_id.to_owned()),
        target_version: Set(None),
        dispatched_at: Set(dispatched_at),
    })
    .on_conflict(
        OnConflict::columns([
            tables::dispatch_trigger::Column::Practice,
            tables::dispatch_trigger::Column::Workspace,
            tables::dispatch_trigger::Column::TriggerKey,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;
    Ok(())
}

async fn settle_conversations_and_spruce_triggers_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    practice_id: &str,
    workspace_id: &str,
    settled_conversations: &BTreeSet<String>,
    dispatched_at: i64,
) -> Result<(), DbErr> {
    if settled_conversations.is_empty() {
        return Ok(());
    }
    let spruce_rows = spruce_review_trigger::Entity::find()
        .filter(spruce_review_trigger::Column::Practice.eq(practice_id))
        .filter(spruce_review_trigger::Column::Workspace.eq(workspace_id))
        .lock_exclusive()
        .all(tx)
        .await?;
    for row in spruce_rows {
        if settled_conversations.contains(&row.conversation_id) {
            let cmd_key = row.command_key.clone();
            let conv_id = row.conversation_id.clone();
            if !row.dispatched {
                let mut active: spruce_review_trigger::ActiveModel = row.into();
                active.dispatched = Set(true);
                active.dispatched_at = Set(Some(dispatched_at));
                active.update(tx).await?;
            }
            insert_message_dispatch_trigger_in_tx(
                tx,
                practice_id,
                workspace_id,
                format!("spruce:{cmd_key}"),
                &conv_id,
                dispatched_at,
            )
            .await?;
        }
    }
    for conv_id in settled_conversations {
        let wakes = Store::conversation_woke_events_in_tx(tx, practice_id, conv_id).await?;
        for event in &wakes {
            let (wake_key, _) = reply_wake_event_key(conv_id, event);
            insert_message_dispatch_trigger_in_tx(
                tx,
                practice_id,
                workspace_id,
                wake_key,
                conv_id,
                dispatched_at,
            )
            .await?;
        }
    }
    Ok(())
}

/// Records settled message trigger keys (`spruce:{command_key}` and synced
/// `reply:{conversation_id}:...` wake keys) inside `admit_patient_review_with_triggers`'s
/// transaction so a direct `spruce_matching` `ReviewReceiver` handoff or direct
/// message review admission never causes `wake_practice_dispatch` to start a
/// duplicate review for the same message.
pub(super) async fn record_admitted_message_triggers_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    practice_id: &str,
    workspace_id: &str,
    triggers: &[ReviewTrigger],
    command_key: &str,
    dispatched_at: i64,
) -> Result<(), DbErr> {
    let mut settled_conversations = BTreeSet::new();
    let _ = triggers;
    if let Some(spruce_row) =
        spruce_review_trigger::Entity::find_by_id((practice_id.to_owned(), command_key.to_owned()))
            .lock_exclusive()
            .one(tx)
            .await?
    {
        let conv_id = spruce_row.conversation_id.clone();
        settled_conversations.insert(conv_id.clone());
        if !spruce_row.dispatched {
            let mut active: spruce_review_trigger::ActiveModel = spruce_row.into();
            active.dispatched = Set(true);
            active.dispatched_at = Set(Some(dispatched_at));
            active.update(tx).await?;
        }
        insert_message_dispatch_trigger_in_tx(
            tx,
            practice_id,
            workspace_id,
            format!("spruce:{command_key}"),
            &conv_id,
            dispatched_at,
        )
        .await?;
    }
    settle_conversations_and_spruce_triggers_in_tx(
        tx,
        practice_id,
        workspace_id,
        &settled_conversations,
        dispatched_at,
    )
    .await
}

async fn record_settled_dispatch_triggers(
    tx: &sea_orm::DatabaseTransaction,
    practice_id: &str,
    workspace_id: &str,
    pending: &PendingReview,
    dispatched_at: i64,
) -> Result<(), DbErr> {
    let mut settled_conversations = BTreeSet::new();
    for input in &pending.inputs {
        match input {
            ReviewDispatchInput::LinkedMessage(msg) => {
                settled_conversations.insert(msg.conversation_id.clone());
                insert_message_dispatch_trigger_in_tx(
                    tx,
                    practice_id,
                    workspace_id,
                    msg.trigger_key.clone(),
                    &msg.conversation_id,
                    dispatched_at,
                )
                .await?;
            }
            ReviewDispatchInput::DueFollowUp(follow_up) => {
                let target_version = i64::try_from(follow_up.version)
                    .map_err(conversion_error("follow_up.version", "i64"))?;
                let trigger_key =
                    format!("followup:{}:v{}", follow_up.follow_up_id, follow_up.version);
                tables::dispatch_trigger::Entity::insert(tables::dispatch_trigger::ActiveModel {
                    practice: Set(practice_id.to_owned()),
                    workspace: Set(workspace_id.to_owned()),
                    trigger_key: Set(trigger_key),
                    trigger_kind: Set("follow_up".to_owned()),
                    target_id: Set(follow_up.follow_up_id.clone()),
                    target_version: Set(Some(target_version)),
                    dispatched_at: Set(dispatched_at),
                })
                .on_conflict(
                    OnConflict::columns([
                        tables::dispatch_trigger::Column::Practice,
                        tables::dispatch_trigger::Column::Workspace,
                        tables::dispatch_trigger::Column::TriggerKey,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec_without_returning(tx)
                .await?;
            }
            ReviewDispatchInput::ChangedRecords { .. } | ReviewDispatchInput::Periodic { .. } => {}
        }
    }

    settle_conversations_and_spruce_triggers_in_tx(
        tx,
        practice_id,
        workspace_id,
        &settled_conversations,
        dispatched_at,
    )
    .await
}

fn reply_wake_event_key(
    conversation_id: &str,
    event: &ReviewWakeEvent,
) -> (String, Option<String>) {
    match event {
        ReviewWakeEvent::PatientMessage { message_id } => (
            format!("reply:{conversation_id}:patient:{message_id}"),
            Some(message_id.clone()),
        ),
        ReviewWakeEvent::PracticeMessage {
            practice_message_id,
        } => (
            format!("reply:{conversation_id}:practice:{practice_message_id}"),
            Some(practice_message_id.clone()),
        ),
        ReviewWakeEvent::MovedToPatient { move_epoch } => {
            (format!("reply:{conversation_id}:moved:{move_epoch}"), None)
        }
        ReviewWakeEvent::PhysicianAskedForReply { request_id } => {
            (format!("reply:{conversation_id}:ask:{request_id}"), None)
        }
    }
}

fn map_spruce_error_to_db(err: SpruceMatchingError) -> DbErr {
    match err {
        SpruceMatchingError::Database(db) => db,
        other => error(&other.to_string()),
    }
}

fn map_reply_error_to_db(err: ReplyProposalError) -> DbErr {
    match err {
        ReplyProposalError::Database(db) => db,
        other => error(&other.to_string()),
    }
}

fn map_follow_up_error_to_db(err: FollowUpError) -> DbErr {
    match err {
        FollowUpError::Database(db) => db,
        other => error(&other.to_string()),
    }
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
    /// Seeds the registration-time source observation baseline for a patient
    /// joining `practice_id` so an unchanged pre-loaded chart does not trigger a
    /// `"new records"` review on the first wake-up.
    ///
    /// Idempotent (`ON CONFLICT DO NOTHING`): re-running after a later chart load
    /// preserves both the saved cadence (`Store::set_patient_cadence` remains the
    /// sole cadence writer) and the saved check baseline so subsequent record
    /// changes still register against the prior baseline.
    pub async fn register_patient_dispatch<A: PracticeAuthority + ?Sized>(
        &self,
        authority: &A,
        practice_id: &str,
        patient_workspace_id: &str,
        registered_at: i64,
    ) -> Result<(), DispatchError> {
        authority
            .check_practice_patient(&scope_for_membership(practice_id, patient_workspace_id))
            .await?;
        let tx = self.db.begin().await?;
        let (version, record_versions) =
            observed_record_versions(&tx, patient_workspace_id).await?;
        let checked_version =
            i64::try_from(version).map_err(conversion_error("last_checked_version", "i64"))?;
        let records = serde_json::to_value(&record_versions).map_err(conversion_error(
            "BTreeMap<String, String>",
            "last_checked_records",
        ))?;
        patient_dispatch::Entity::insert(patient_dispatch::ActiveModel {
            practice: Set(practice_id.to_owned()),
            workspace: Set(patient_workspace_id.to_owned()),
            cadence: ActiveValue::NotSet,
            registered_at: Set(registered_at),
            last_attempt_at: Set(None),
            last_success_at: Set(None),
            last_checked_version: Set(Some(checked_version)),
            last_checked_records: Set(records),
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
        .exec_without_returning(&tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

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
        let patients = auth
            .practice_patients(practice_id)
            .await
            .map_err(PracticeAccessError::from)?;
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

    /// Idempotently mirrors linked Spruce conversations and messages for
    /// `(practice_id, workspace_id)` into the reply conversation store so review
    /// execution and reply drafting operate on one coherent conversation record.
    pub(crate) async fn sync_workspace_spruce_conversations_to_reply(
        &self,
        practice_id: &str,
        workspace_id: &str,
    ) -> Result<(), DbErr> {
        if practice_id.trim().is_empty() || workspace_id.trim().is_empty() {
            return Ok(());
        }
        let linked_messages = self
            .patient_linked_spruce_messages(practice_id, workspace_id)
            .await
            .map_err(map_spruce_error_to_db)?;
        if linked_messages.is_empty() {
            return Ok(());
        }
        let mut by_conversation: BTreeMap<String, Vec<super::spruce_matching::SavedSpruceMessage>> =
            BTreeMap::new();
        for msg in linked_messages {
            by_conversation
                .entry(msg.conversation_id.clone())
                .or_default()
                .push(msg);
        }

        let pull_rows = source_pull::Entity::find()
            .filter(source_pull::Column::Workspace.eq(workspace_id))
            .all(&self.db)
            .await?;
        let mut patient_chart_names: Vec<String> = Vec::new();
        for pull_row in pull_rows {
            if let Ok(patient) = serde_json::from_value::<SourcePatient>(pull_row.patient) {
                let trimmed = patient.display_name.trim();
                if !trimmed.is_empty()
                    && !patient_chart_names
                        .iter()
                        .any(|existing| existing.eq_ignore_ascii_case(trimmed))
                {
                    patient_chart_names.push(trimmed.to_owned());
                }
            }
        }

        let expected_link = ConversationLink::LinkedPatient {
            workspace: workspace_id.to_owned(),
        };
        for (conversation_id, messages) in by_conversation {
            let spruce_conv = self
                .spruce_conversation(practice_id, &conversation_id)
                .await
                .map_err(map_spruce_error_to_db)?;
            let sole_outside_participant = spruce_conv
                .as_ref()
                .is_none_or(|c| c.outside_participants.len() <= 1);
            let existing = self
                .reply_conversation(practice_id, &conversation_id)
                .await
                .map_err(map_reply_error_to_db)?;
            let conv = match existing {
                Some(conv) if conv.link != expected_link => continue,
                Some(conv) => conv,
                None => self
                    .link_reply_conversation(practice_id, &conversation_id, expected_link.clone())
                    .await
                    .map_err(map_reply_error_to_db)?,
            };
            if matches!(conv.channel, ChannelConnection::Disconnected { .. }) {
                continue;
            }
            for msg in messages {
                if conv
                    .patient_messages
                    .iter()
                    .any(|existing_msg| existing_msg.message_id == msg.message_id)
                {
                    continue;
                }
                let sender = match msg
                    .author_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|name| !name.is_empty())
                {
                    None => MessageSender::Patient,
                    Some(name) => {
                        let matches_patient = if patient_chart_names.is_empty() {
                            sole_outside_participant
                        } else {
                            patient_chart_names
                                .iter()
                                .any(|pname| pname.eq_ignore_ascii_case(name))
                        };
                        if matches_patient {
                            MessageSender::Patient
                        } else {
                            MessageSender::NamedSender {
                                name: name.to_owned(),
                            }
                        }
                    }
                };
                let text = if msg.body_text.trim().is_empty() {
                    "[Attachment]".to_owned()
                } else {
                    msg.body_text
                };
                let _ = self
                    .record_linked_patient_message(&RecordPatientMessage {
                        practice: practice_id.to_owned(),
                        conversation_id: conversation_id.clone(),
                        message_id: msg.message_id,
                        sender,
                        text,
                        sent_at: msg.kept_at,
                    })
                    .await
                    .map_err(map_reply_error_to_db)?;
            }
        }
        Ok(())
    }

    async fn gather_patient_event_triggers(
        &self,
        practice_id: &str,
        workspace_id: &str,
        state: &DispatchState,
    ) -> Result<(Vec<LinkedMessageTrigger>, Vec<FollowUpCandidate>), DbErr> {
        self.sync_workspace_spruce_conversations_to_reply(practice_id, workspace_id)
            .await?;

        let mut linked_messages = Vec::new();
        let mut covered_conversations = BTreeSet::new();
        let conv_rows = reply_conversation::Entity::find()
            .filter(reply_conversation::Column::Practice.eq(practice_id))
            .filter(reply_conversation::Column::LinkKind.eq("linked_patient"))
            .filter(reply_conversation::Column::Workspace.eq(workspace_id))
            .order_by_asc(reply_conversation::Column::ConversationId)
            .all(&self.db)
            .await?;
        for row in conv_rows {
            let Some(conv) = self
                .reply_conversation(practice_id, &row.conversation_id)
                .await
                .map_err(map_reply_error_to_db)?
            else {
                continue;
            };
            if conv.active_review.is_some() || conv.woke_events.is_empty() {
                continue;
            }
            for event in &conv.woke_events {
                let (trigger_key, maybe_msg_id) =
                    reply_wake_event_key(&conv.conversation_id, event);
                if state.dispatched_message_keys.contains(&trigger_key) {
                    continue;
                }
                covered_conversations.insert(conv.conversation_id.clone());
                let message_ids = maybe_msg_id.into_iter().collect();
                linked_messages.push(LinkedMessageTrigger {
                    trigger_key,
                    conversation_id: conv.conversation_id.clone(),
                    message_ids,
                    arrived_at: conv.updated_at,
                });
            }
        }

        let spruce_triggers = self
            .spruce_review_triggers(practice_id)
            .await
            .map_err(map_spruce_error_to_db)?;
        for trigger in spruce_triggers {
            if trigger.dispatched || trigger.workspace != workspace_id {
                continue;
            }
            if covered_conversations.contains(&trigger.conversation_id) {
                continue;
            }
            let trigger_key = format!("spruce:{}", trigger.command_key);
            if state.dispatched_message_keys.contains(&trigger_key) {
                continue;
            }
            let message_ids: Vec<String> = trigger
                .record_versions
                .keys()
                .filter_map(|key| key.strip_prefix("spruce:").map(ToOwned::to_owned))
                .collect();
            linked_messages.push(LinkedMessageTrigger {
                trigger_key,
                conversation_id: trigger.conversation_id,
                message_ids,
                arrived_at: trigger.created_at,
            });
        }

        let stored_follow_ups = self
            .workspace_follow_ups(practice_id, workspace_id)
            .await
            .map_err(map_follow_up_error_to_db)?;
        let follow_ups = stored_follow_ups
            .into_iter()
            .map(|item| {
                let standing = match item.standing {
                    FollowUpStanding::Active {
                        schedule: FollowUpReviewSchedule::Scheduled { due_at },
                    } => FollowUpDispatchStanding::Active {
                        schedule: FollowUpScheduleState::Scheduled { due_at },
                    },
                    FollowUpStanding::Active {
                        schedule: FollowUpReviewSchedule::DueNow { due_at, .. },
                    } => FollowUpDispatchStanding::Active {
                        schedule: FollowUpScheduleState::DueNow { due_at },
                    },
                    FollowUpStanding::Held { .. } => FollowUpDispatchStanding::Held,
                    FollowUpStanding::CannotRestart { .. } => {
                        FollowUpDispatchStanding::CannotRestart
                    }
                    FollowUpStanding::Completed { .. } => FollowUpDispatchStanding::Completed,
                    FollowUpStanding::Cancelled { .. } => FollowUpDispatchStanding::Cancelled,
                    FollowUpStanding::WithdrawnWrongPatient { .. } => {
                        FollowUpDispatchStanding::WithdrawnWrongPatient
                    }
                };
                FollowUpCandidate {
                    follow_up_id: item.id.as_str().to_owned(),
                    version: item.version,
                    summary: item.summary.as_str().to_owned(),
                    standing,
                }
            })
            .collect();

        Ok((linked_messages, follow_ups))
    }

    /// Phase 1 of wake-up: checks due registered patients and evaluates linked
    /// message and due follow-up triggers in `practice_id` for patients that do
    /// not already hold an undelivered `pending_review`, committing each
    /// patient's outcome independently so a failed source check on one patient
    /// never blocks other patients or waiting messages/follow-ups.
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
        let patients = auth
            .practice_patients(practice_id)
            .await
            .map_err(PracticeAccessError::from)?;
        let mut summary = WakeSummary::default();
        for workspace in patients {
            let tx = self.db.begin().await?;
            ensure_dispatch_row(&tx, practice_id, &workspace, now_sec).await?;
            let row =
                patient_dispatch::Entity::find_by_id((practice_id.to_owned(), workspace.clone()))
                    .lock_exclusive()
                    .one(&tx)
                    .await?
                    .ok_or_else(|| error("missing patient dispatch row"))?;
            let state = load_dispatch_state_with_ledger(&tx, practice_id, &workspace, row).await?;
            tx.commit().await?;

            // A patient with an undelivered pending review awaits delivery first.
            if state.pending_review.is_some() {
                continue;
            }

            let (linked_messages, follow_ups) = self
                .gather_patient_event_triggers(practice_id, &workspace, &state)
                .await?;
            let has_unsettled_message = linked_messages
                .iter()
                .any(|msg| !state.dispatched_message_keys.contains(&msg.trigger_key));
            let has_due_follow_up = follow_ups.iter().any(|candidate| {
                candidate.due_trigger_at(now_sec).is_some_and(|due| {
                    due.version
                        > state
                            .dispatched_follow_up_versions
                            .get(&due.follow_up_id)
                            .copied()
                            .unwrap_or(0)
                })
            });
            let has_event_trigger = has_unsettled_message || has_due_follow_up;
            let scheduled_check_due = state.is_due_at(now_sec);

            if !scheduled_check_due && !has_event_trigger {
                summary.skipped_not_due.push(workspace);
                continue;
            }

            if scheduled_check_due {
                let check_result = checker
                    .check_patient_sources(practice_id, &workspace, now_sec)
                    .await;

                let tx = self.db.begin().await?;
                let (obs_version, obs_records) = observed_record_versions(&tx, &workspace).await?;
                let row = patient_dispatch::Entity::find_by_id((
                    practice_id.to_owned(),
                    workspace.clone(),
                ))
                .lock_exclusive()
                .one(&tx)
                .await?
                .ok_or_else(|| error("missing patient dispatch row"))?;
                let mut current =
                    load_dispatch_state_with_ledger(&tx, practice_id, &workspace, row).await?;
                if current.pending_review.is_none()
                    && (current.is_due_at(now_sec) || has_event_trigger)
                {
                    match &check_result {
                        Ok(checked) => {
                            current.evaluate_patient_wakeup(
                                practice_id,
                                &workspace,
                                PatientWakeInputs {
                                    sources: WakeSourceObservation::ScheduledCheck(checked.clone()),
                                    linked_messages,
                                    follow_ups,
                                    workspace_version: checked.workspace_version,
                                    record_versions: checked.record_versions.clone(),
                                },
                                now_sec,
                            );
                        }
                        Err(_) => {
                            current.record_failed_check(now_sec);
                            if has_event_trigger {
                                current.evaluate_patient_wakeup(
                                    practice_id,
                                    &workspace,
                                    PatientWakeInputs {
                                        sources: WakeSourceObservation::NotChecked,
                                        linked_messages,
                                        follow_ups,
                                        workspace_version: obs_version,
                                        record_versions: obs_records,
                                    },
                                    now_sec,
                                );
                            }
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
            } else {
                let tx = self.db.begin().await?;
                let (obs_version, obs_records) = observed_record_versions(&tx, &workspace).await?;
                let row = patient_dispatch::Entity::find_by_id((
                    practice_id.to_owned(),
                    workspace.clone(),
                ))
                .lock_exclusive()
                .one(&tx)
                .await?
                .ok_or_else(|| error("missing patient dispatch row"))?;
                let mut current =
                    load_dispatch_state_with_ledger(&tx, practice_id, &workspace, row).await?;
                if current.pending_review.is_none() {
                    let changed_vs_held = match current.last_checked_version {
                        Some(prev_ver) => {
                            prev_ver != obs_version || current.last_checked_records != obs_records
                        }
                        None => !obs_records.is_empty(),
                    };
                    let snapshot = if changed_vs_held {
                        CheckedSources::changed(obs_version, obs_records.clone())
                    } else {
                        CheckedSources::unchanged(obs_version, obs_records.clone())
                    };
                    current.evaluate_patient_wakeup(
                        practice_id,
                        &workspace,
                        PatientWakeInputs {
                            sources: WakeSourceObservation::EventDrivenSnapshot(snapshot),
                            linked_messages,
                            follow_ups,
                            workspace_version: obs_version,
                            record_versions: obs_records,
                        },
                        now_sec,
                    );
                    save_dispatch_progress(&tx, practice_id, &workspace, &current).await?;
                }
                tx.commit().await?;
            }
        }
        Ok(summary)
    }

    /// Phase 2 of wake-up (shared by live execution and crash replay): delivers
    /// every durable `pending_review` in `practice_id` once under a command
    /// receipt and settles `last_reviewed_at`, `last_reviewed_version`, and
    /// dispatched message / follow-up trigger watermarks.
    pub async fn deliver_pending_reviews(
        &self,
        auth: &dyn AuthStore,
        practice_id: &str,
        receiver: &dyn ReviewReceiver,
    ) -> Result<Vec<ReviewRequest>, DispatchError> {
        let pairs = self
            .deliver_pending_reviews_with_inputs(auth, practice_id, receiver)
            .await?;
        Ok(pairs.into_iter().map(|(req, _)| req).collect())
    }

    async fn deliver_pending_reviews_with_inputs(
        &self,
        auth: &dyn AuthStore,
        practice_id: &str,
        receiver: &dyn ReviewReceiver,
    ) -> Result<Vec<(ReviewRequest, Vec<ReviewDispatchInput>)>, DispatchError> {
        if practice_id.trim().is_empty() {
            return Err(PracticeAccessError::InvalidScope.into());
        }
        let patients = auth
            .practice_patients(practice_id)
            .await
            .map_err(PracticeAccessError::from)?;
        let mut delivered = Vec::new();
        for workspace in patients {
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
                receiver
                    .submit_coalesced_review(&request, &pending.inputs)
                    .await
                    .map_err(|source| DispatchError::ReviewHandoff {
                        workspace: workspace.clone(),
                        source,
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
            let mut current =
                load_dispatch_state_with_ledger(&tx, practice_id, &workspace, locked).await?;
            if current
                .pending_review
                .as_ref()
                .is_some_and(|saved| saved.command_key == request.command_key)
            {
                record_settled_dispatch_triggers(
                    &tx,
                    practice_id,
                    &workspace,
                    &pending,
                    pending.queued_at,
                )
                .await?;
                current.complete_pending_review(pending.queued_at);
                save_dispatch_progress(&tx, practice_id, &workspace, &current).await?;
                tx.commit().await?;
                if !already_handed_off {
                    delivered.push((request, pending.inputs));
                }
            } else {
                tx.commit().await?;
            }
        }
        Ok(delivered)
    }

    /// Runs a complete practice dispatch wake-up: first replays any durable
    /// `pending_review` left by an interrupted wake-up, then checks all due
    /// patients and message/follow-up triggers, and finally delivers any newly
    /// decided reviews.
    pub async fn wake_practice_dispatch(
        &self,
        auth: &dyn AuthStore,
        practice_id: &str,
        now_sec: i64,
        checker: &dyn PatientSourceChecker,
        receiver: &dyn ReviewReceiver,
    ) -> Result<WakeSummary, DispatchError> {
        let mut delivered = self
            .deliver_pending_reviews_with_inputs(auth, practice_id, receiver)
            .await?;
        let mut summary = self
            .check_due_patients(auth, practice_id, now_sec, checker)
            .await?;
        let fresh_reviews = self
            .deliver_pending_reviews_with_inputs(auth, practice_id, receiver)
            .await?;
        delivered.extend(fresh_reviews);
        for (request, inputs) in delivered {
            if !inputs.is_empty() {
                summary
                    .coalesced_inputs
                    .insert(request.command_key.clone(), inputs);
            }
            summary.started_reviews.push(request);
        }
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
