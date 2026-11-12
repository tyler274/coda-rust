//! SeaORM persistence for practice-scoped workspace ownership, source patient links,
//! practice-level source access state, per-patient pull status, reconnect commands,
//! and shared source adapter state across instances.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait, Unchanged, sea_query::OnConflict,
};
use serde_json::Value;
use uuid::Uuid;

use super::entities::{
    command, patient_source_status, practice_source_access, reconnect_command,
    source_adapter_state, source_patient_link, source_pull, workspace_practice,
};
use super::receipt::Receipt;
use super::sources::{MissingScope, apply_source_delivery_tx};
use super::{Store, conversion_error, error, now};
use crate::physician_sources::{
    ChartPullReceipt, PullOutcome, ReconnectReceipt, SourceAdapterError,
};
use crate::source_status::{
    AdapterPresence, CompleteDelivery, LinkTarget, PartialDelivery, PatientSourceRecordStatus,
    PracticeSourceReport, PullAttempt, PullDisposition, PullOrderOutcome, PullTicket,
    ReconnectCommandClaim, ReconnectCommandFence, ReconnectCommandState, SourceAccessState,
    SourcePatientKey, SourcePatientLink, SourceStanding, SourceStatusError, decide_pull_order,
    derive_standing, summarize_practice_source, validate_non_empty,
};

async fn ensure_workspace_practice_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
) -> Result<(), SourceStatusError> {
    ensure_workspace_practice_locked_tx(tx, practice_id, workspace_id, false).await
}

async fn ensure_workspace_practice_locked_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
    exclusive: bool,
) -> Result<(), SourceStatusError> {
    validate_non_empty("practice_id", practice_id)?;
    validate_non_empty("workspace_id", workspace_id)?;

    workspace_practice::Entity::insert(workspace_practice::ActiveModel {
        workspace: Set(workspace_id.to_owned()),
        practice: Set(practice_id.to_owned()),
    })
    .on_conflict(
        OnConflict::column(workspace_practice::Column::Workspace)
            .do_nothing()
            .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;

    let select = workspace_practice::Entity::find_by_id(workspace_id.to_owned());
    let row = if exclusive {
        select.lock_exclusive().one(tx).await?
    } else {
        select.lock_shared().one(tx).await?
    }
    .ok_or_else(|| error("missing workspace practice binding"))?;

    if row.practice != practice_id {
        return Err(SourceStatusError::PracticeMismatch {
            workspace_id: workspace_id.to_owned(),
            owner_practice_id: row.practice,
            requested_practice_id: practice_id.to_owned(),
        });
    }
    Ok(())
}

async fn verify_workspace_practice_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
) -> Result<(), SourceStatusError> {
    validate_non_empty("practice_id", practice_id)?;
    validate_non_empty("workspace_id", workspace_id)?;

    if let Some(row) = workspace_practice::Entity::find_by_id(workspace_id.to_owned())
        .lock_shared()
        .one(tx)
        .await?
        && row.practice != practice_id
    {
        return Err(SourceStatusError::PracticeMismatch {
            workspace_id: workspace_id.to_owned(),
            owner_practice_id: row.practice,
            requested_practice_id: practice_id.to_owned(),
        });
    }
    Ok(())
}

fn decode_access(row: practice_source_access::Model) -> Result<SourceAccessState, DbErr> {
    serde_json::from_value(row.access).map_err(conversion_error("Json", "SourceAccessState"))
}

fn decode_status(row: patient_source_status::Model) -> Result<PatientSourceRecordStatus, DbErr> {
    let latest_attempt = row
        .attempt
        .map(|value| {
            serde_json::from_value::<PullAttempt>(value)
                .map_err(conversion_error("Json", "PullAttempt"))
        })
        .transpose()?;
    Ok(PatientSourceRecordStatus {
        practice_id: row.practice,
        workspace_id: row.workspace,
        source_id: row.source_id,
        last_good_read_at: row.last_good_read_at,
        latest_attempt,
    })
}

fn decode_link(row: source_patient_link::Model) -> SourcePatientLink {
    SourcePatientLink {
        key: SourcePatientKey {
            practice_id: row.practice,
            source_id: row.source_id,
            source_patient_id: row.source_patient_id,
        },
        workspace_id: row.workspace,
        linked_at: row.linked_at,
    }
}

fn decode_reconnect_state(value: Value) -> Result<ReconnectCommandState, DbErr> {
    serde_json::from_value(value).map_err(conversion_error("Json", "ReconnectCommandState"))
}

/// Locks the `reconnect_command` row in `tx`, verifies that `fence.lease_owner` still holds active
/// non-completed ownership, and refreshes `lease_until` using the database clock.
async fn verify_reconnect_fence_tx(
    tx: &sea_orm::DatabaseTransaction,
    fence: Option<ReconnectCommandFence<'_>>,
) -> Result<(), SourceStatusError> {
    let Some(fence) = fence else {
        return Ok(());
    };
    let lost = || SourceStatusError::CommandLeaseLost {
        practice_id: fence.practice_id.to_owned(),
        physician_id: fence.physician_id.to_owned(),
        idempotency_key: fence.idempotency_key.to_owned(),
    };
    let row = reconnect_command::Entity::find_by_id((
        fence.practice_id.to_owned(),
        fence.physician_id.to_owned(),
        fence.idempotency_key.to_owned(),
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    .ok_or_else(lost)?;

    let existing = decode_reconnect_state(row.state.clone())?;
    if matches!(existing, ReconnectCommandState::Completed { .. })
        || row.lease_owner.as_deref() != Some(fence.lease_owner)
    {
        return Err(lost());
    }

    let db_now = now(tx).await?;
    reconnect_command::ActiveModel {
        practice: Unchanged(row.practice),
        physician: Unchanged(row.physician),
        key: Unchanged(row.key),
        source_id: Unchanged(row.source_id),
        state: Unchanged(row.state),
        lease_owner: Unchanged(row.lease_owner),
        lease_until: Set(Some(db_now + fence.lease_seconds.max(2))),
        updated_at: Set(db_now),
    }
    .update(tx)
    .await?;
    Ok(())
}

enum PreparedPullOrder {
    Apply {
        applied_seq: i64,
        prior_good: Option<i64>,
        existing_pull: Option<source_pull::Model>,
    },
    Superseded {
        status: PatientSourceRecordStatus,
        missing_candidates: Vec<String>,
    },
}

async fn upsert_practice_source_access_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    source_id: &str,
    access: &SourceAccessState,
    updated_at: i64,
) -> Result<i64, SourceStatusError> {
    let access_json =
        serde_json::to_value(access).map_err(conversion_error("SourceAccessState", "Json"))?;

    practice_source_access::Entity::insert(practice_source_access::ActiveModel {
        practice: Set(practice_id.to_owned()),
        source_id: Set(source_id.to_owned()),
        access: Set(access_json.clone()),
        updated_at: Set(updated_at),
        access_seq: Set(0),
    })
    .on_conflict(
        OnConflict::columns([
            practice_source_access::Column::Practice,
            practice_source_access::Column::SourceId,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;

    let row =
        practice_source_access::Entity::find_by_id((practice_id.to_owned(), source_id.to_owned()))
            .lock_exclusive()
            .one(tx)
            .await?
            .ok_or_else(|| error("missing practice source access row after insert"))?;

    let next_seq = row.access_seq + 1;
    practice_source_access::ActiveModel {
        practice: Unchanged(row.practice),
        source_id: Unchanged(row.source_id),
        access: Set(access_json),
        updated_at: Set(updated_at),
        access_seq: Set(next_seq),
    }
    .update(tx)
    .await?;
    Ok(next_seq)
}

async fn prepare_pull_order_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
    source_id: &str,
    ticket: Option<PullTicket>,
    probe_attempt: &PullAttempt,
) -> Result<PreparedPullOrder, SourceStatusError> {
    ensure_workspace_practice_locked_tx(tx, practice_id, workspace_id, true).await?;

    let existing_status_row = patient_source_status::Entity::find_by_id((
        practice_id.to_owned(),
        workspace_id.to_owned(),
        source_id.to_owned(),
    ))
    .lock_exclusive()
    .one(tx)
    .await?;
    let existing_applied_seq = existing_status_row
        .as_ref()
        .map_or(0, |row| row.applied_pull_seq);
    let existing_status = existing_status_row.map(decode_status).transpose()?;

    let existing_pull =
        source_pull::Entity::find_by_id((workspace_id.to_owned(), source_id.to_owned()))
            .lock_exclusive()
            .one(tx)
            .await?;

    let existing_access_row =
        practice_source_access::Entity::find_by_id((practice_id.to_owned(), source_id.to_owned()))
            .one(tx)
            .await?;
    let current_access_seq = existing_access_row.as_ref().map_or(0, |row| row.access_seq);
    let current_access = existing_access_row.map(decode_access).transpose()?;

    let existing_latest_attempted_at = existing_status
        .as_ref()
        .and_then(|status| status.latest_attempt.as_ref().map(|a| a.attempted_at));

    match decide_pull_order(
        ticket,
        existing_applied_seq,
        existing_latest_attempted_at,
        current_access.as_ref(),
        current_access_seq,
        probe_attempt,
    ) {
        PullOrderOutcome::Superseded => {
            let missing_candidates = match existing_pull.as_ref() {
                Some(pull) => serde_json::from_value(pull.missing_candidates.clone())
                    .map_err(conversion_error("Json", "Vec<String>"))?,
                None => Vec::new(),
            };
            let status = existing_status.unwrap_or_else(|| PatientSourceRecordStatus {
                practice_id: practice_id.to_owned(),
                workspace_id: workspace_id.to_owned(),
                source_id: source_id.to_owned(),
                last_good_read_at: existing_pull.as_ref().and_then(|pull| pull.pulled_at),
                latest_attempt: None,
            });
            Ok(PreparedPullOrder::Superseded {
                status,
                missing_candidates,
            })
        }
        PullOrderOutcome::Apply { applied_seq } => {
            let prior_good = match existing_status.as_ref() {
                Some(status) => status.last_good_read_at,
                None => existing_pull.as_ref().and_then(|pull| pull.pulled_at),
            };
            Ok(PreparedPullOrder::Apply {
                applied_seq,
                prior_good,
                existing_pull,
            })
        }
    }
}

async fn write_applied_pull_status_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
    source_id: &str,
    attempt: &PullAttempt,
    prior_good: Option<i64>,
    applied_seq: i64,
) -> Result<PatientSourceRecordStatus, SourceStatusError> {
    let attempt_json =
        serde_json::to_value(attempt).map_err(conversion_error("PullAttempt", "Json"))?;

    let last_good_read_at = match &attempt.outcome {
        PullOutcome::Succeeded { .. } => {
            Some(prior_good.map_or(attempt.attempted_at, |prev| prev.max(attempt.attempted_at)))
        }
        PullOutcome::Partial { .. }
        | PullOutcome::Failed { .. }
        | PullOutcome::LapsedAccess { .. } => prior_good,
    };

    patient_source_status::Entity::insert(patient_source_status::ActiveModel {
        practice: Set(practice_id.to_owned()),
        workspace: Set(workspace_id.to_owned()),
        source_id: Set(source_id.to_owned()),
        last_good_read_at: Set(last_good_read_at),
        attempt: Set(Some(attempt_json)),
        applied_pull_seq: Set(applied_seq),
    })
    .on_conflict(
        OnConflict::columns([
            patient_source_status::Column::Practice,
            patient_source_status::Column::Workspace,
            patient_source_status::Column::SourceId,
        ])
        .update_columns([
            patient_source_status::Column::LastGoodReadAt,
            patient_source_status::Column::Attempt,
            patient_source_status::Column::AppliedPullSeq,
        ])
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;

    if matches!(
        attempt.outcome,
        PullOutcome::Succeeded { .. } | PullOutcome::Partial { .. }
    ) {
        let existing_access = practice_source_access::Entity::find_by_id((
            practice_id.to_owned(),
            source_id.to_owned(),
        ))
        .one(tx)
        .await?
        .map(decode_access)
        .transpose()?;
        let connected = SourceAccessState::Connected {
            connected_at: attempt.attempted_at,
        };
        match existing_access {
            None => {
                let access_json = serde_json::to_value(&connected)
                    .map_err(conversion_error("SourceAccessState", "Json"))?;
                practice_source_access::Entity::insert(practice_source_access::ActiveModel {
                    practice: Set(practice_id.to_owned()),
                    source_id: Set(source_id.to_owned()),
                    access: Set(access_json),
                    updated_at: Set(attempt.attempted_at),
                    access_seq: Set(0),
                })
                .on_conflict(
                    OnConflict::columns([
                        practice_source_access::Column::Practice,
                        practice_source_access::Column::SourceId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec_without_returning(tx)
                .await?;
            }
            Some(SourceAccessState::ReconnectNeeded { since, .. })
                if attempt.attempted_at > since =>
            {
                upsert_practice_source_access_tx(
                    tx,
                    practice_id,
                    source_id,
                    &connected,
                    attempt.attempted_at,
                )
                .await?;
            }
            Some(
                SourceAccessState::ReconnectNeeded { .. } | SourceAccessState::Connected { .. },
            ) => {}
        }
    }

    Ok(PatientSourceRecordStatus {
        practice_id: practice_id.to_owned(),
        workspace_id: workspace_id.to_owned(),
        source_id: source_id.to_owned(),
        last_good_read_at,
        latest_attempt: Some(attempt.clone()),
    })
}

pub(crate) async fn release_reconnect_command_lease_db(
    db: &DatabaseConnection,
    practice_id: &str,
    physician_id: &str,
    key: &str,
    lease_owner: &str,
) -> Result<(), DbErr> {
    reconnect_command::Entity::update_many()
        .col_expr(
            reconnect_command::Column::LeaseOwner,
            sea_orm::sea_query::Expr::value(Option::<String>::None),
        )
        .col_expr(
            reconnect_command::Column::LeaseUntil,
            sea_orm::sea_query::Expr::value(Option::<i64>::None),
        )
        .filter(reconnect_command::Column::Practice.eq(practice_id))
        .filter(reconnect_command::Column::Physician.eq(physician_id))
        .filter(reconnect_command::Column::Key.eq(key))
        .filter(reconnect_command::Column::LeaseOwner.eq(lease_owner))
        .exec(db)
        .await?;
    Ok(())
}

impl Store {
    pub(crate) fn connection(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Binds `workspace_id` to `practice_id`, refusing if `workspace_id` already belongs
    /// to another practice.
    pub async fn bind_workspace_practice(
        &self,
        practice_id: &str,
        workspace_id: &str,
    ) -> Result<(), SourceStatusError> {
        let tx = self.db.begin().await?;
        ensure_workspace_practice_tx(&tx, practice_id, workspace_id).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Verifies that `workspace_id` is either unbound or owned by `practice_id`,
    /// without claiming an unbound workspace.
    pub async fn verify_workspace_practice(
        &self,
        practice_id: &str,
        workspace_id: &str,
    ) -> Result<(), SourceStatusError> {
        let tx = self.db.begin().await?;
        verify_workspace_practice_tx(&tx, practice_id, workspace_id).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Reads the owning practice of `workspace_id`, if bound.
    pub async fn workspace_practice(
        &self,
        workspace_id: &str,
    ) -> Result<Option<String>, SourceStatusError> {
        validate_non_empty("workspace_id", workspace_id)?;
        let row = workspace_practice::Entity::find_by_id(workspace_id.to_owned())
            .one(&self.db)
            .await?;
        Ok(row.map(|model| model.practice))
    }

    /// Resolves an existing `(practice_id, source_id, source_patient_id)` link or creates it
    /// in a single transaction, enforcing practice isolation and workspace uniqueness.
    pub async fn resolve_or_link_patient(
        &self,
        key: &SourcePatientKey,
        target: LinkTarget<'_>,
        linked_at: i64,
    ) -> Result<SourcePatientLink, SourceStatusError> {
        self.resolve_or_link_patient_fenced(key, target, linked_at, None)
            .await
    }

    /// Resolves or creates a source-patient link, optionally fenced by an active
    /// [`ReconnectCommandFence`].
    pub async fn resolve_or_link_patient_fenced(
        &self,
        key: &SourcePatientKey,
        target: LinkTarget<'_>,
        linked_at: i64,
        fence: Option<ReconnectCommandFence<'_>>,
    ) -> Result<SourcePatientLink, SourceStatusError> {
        let (link, _) = self
            .resolve_or_link_patient_inner(key, target, linked_at, fence, false)
            .await?;
        Ok(link)
    }

    /// Resolves or creates a source-patient link and atomically issues a monotonic [`PullTicket`]
    /// before a patient pull begins so competing checks and reconnects cannot overwrite newer state.
    pub async fn begin_patient_pull_fenced(
        &self,
        key: &SourcePatientKey,
        target: LinkTarget<'_>,
        linked_at: i64,
        fence: Option<ReconnectCommandFence<'_>>,
    ) -> Result<(SourcePatientLink, PullTicket), SourceStatusError> {
        let (link, ticket) = self
            .resolve_or_link_patient_inner(key, target, linked_at, fence, true)
            .await?;
        Ok((
            link,
            ticket.expect("begin_patient_pull_fenced always issues a PullTicket"),
        ))
    }

    async fn resolve_or_link_patient_inner(
        &self,
        key: &SourcePatientKey,
        target: LinkTarget<'_>,
        linked_at: i64,
        fence: Option<ReconnectCommandFence<'_>>,
        issue_pull_ticket: bool,
    ) -> Result<(SourcePatientLink, Option<PullTicket>), SourceStatusError> {
        key.validate()?;
        validate_non_empty("workspace_id", target.workspace())?;

        let tx = self.db.begin().await?;
        verify_reconnect_fence_tx(&tx, fence).await?;
        let existing = source_patient_link::Entity::find_by_id((
            key.practice_id.clone(),
            key.source_id.clone(),
            key.source_patient_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;

        if existing.is_none() {
            let candidate_workspace = target.workspace();
            ensure_workspace_practice_tx(&tx, &key.practice_id, candidate_workspace).await?;

            let conflicting_patient = source_patient_link::Entity::find()
                .filter(source_patient_link::Column::Practice.eq(&key.practice_id))
                .filter(source_patient_link::Column::SourceId.eq(&key.source_id))
                .filter(source_patient_link::Column::Workspace.eq(candidate_workspace))
                .lock_shared()
                .one(&tx)
                .await?;

            if let Some(other) = conflicting_patient
                && other.source_patient_id != key.source_patient_id
            {
                return Err(SourceStatusError::ConflictingWorkspaceLink {
                    practice_id: key.practice_id.clone(),
                    source_id: key.source_id.clone(),
                    source_patient_id: key.source_patient_id.clone(),
                    existing_workspace_id: other.workspace,
                    requested_workspace_id: format!(
                        "{candidate_workspace} (already linked to `{}`)",
                        other.source_patient_id
                    ),
                });
            }

            source_patient_link::Entity::insert(source_patient_link::ActiveModel {
                practice: Set(key.practice_id.clone()),
                source_id: Set(key.source_id.clone()),
                source_patient_id: Set(key.source_patient_id.clone()),
                workspace: Set(candidate_workspace.to_owned()),
                linked_at: Set(linked_at),
                issued_pull_seq: Set(0),
            })
            .on_conflict(
                OnConflict::columns([
                    source_patient_link::Column::Practice,
                    source_patient_link::Column::SourceId,
                    source_patient_link::Column::SourcePatientId,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec_without_returning(&tx)
            .await?;
        }

        let row = match existing {
            Some(row) => row,
            None => source_patient_link::Entity::find_by_id((
                key.practice_id.clone(),
                key.source_id.clone(),
                key.source_patient_id.clone(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing source patient link row after insert"))?,
        };

        if let LinkTarget::Exact(requested) = target
            && requested != row.workspace
        {
            return Err(SourceStatusError::ConflictingWorkspaceLink {
                practice_id: key.practice_id.clone(),
                source_id: key.source_id.clone(),
                source_patient_id: key.source_patient_id.clone(),
                existing_workspace_id: row.workspace,
                requested_workspace_id: requested.to_owned(),
            });
        }
        ensure_workspace_practice_tx(&tx, &key.practice_id, &row.workspace).await?;

        let ticket = if issue_pull_ticket {
            let existing_applied_seq = patient_source_status::Entity::find_by_id((
                key.practice_id.clone(),
                row.workspace.clone(),
                key.source_id.clone(),
            ))
            .one(&tx)
            .await?
            .map_or(0, |status| status.applied_pull_seq);
            let access_seq = practice_source_access::Entity::find_by_id((
                key.practice_id.clone(),
                key.source_id.clone(),
            ))
            .one(&tx)
            .await?
            .map_or(0, |access| access.access_seq);

            let next_seq = row.issued_pull_seq.max(existing_applied_seq) + 1;
            source_patient_link::ActiveModel {
                practice: Unchanged(row.practice.clone()),
                source_id: Unchanged(row.source_id.clone()),
                source_patient_id: Unchanged(row.source_patient_id.clone()),
                workspace: Unchanged(row.workspace.clone()),
                linked_at: Unchanged(row.linked_at),
                issued_pull_seq: Set(next_seq),
            }
            .update(&tx)
            .await?;
            Some(PullTicket {
                seq: next_seq,
                access_seq,
            })
        } else {
            None
        };

        let link = decode_link(row);
        tx.commit().await?;
        Ok((link, ticket))
    }

    /// Reads the link for `(practice_id, source_id, source_patient_id)` if one exists.
    pub async fn source_patient_link(
        &self,
        key: &SourcePatientKey,
    ) -> Result<Option<SourcePatientLink>, SourceStatusError> {
        key.validate()?;
        let row = source_patient_link::Entity::find_by_id((
            key.practice_id.clone(),
            key.source_id.clone(),
            key.source_patient_id.clone(),
        ))
        .one(&self.db)
        .await?;
        Ok(row.map(decode_link))
    }

    /// Reads the link for `(practice_id, source_id, workspace_id)` if one exists,
    /// refusing if `workspace_id` belongs to another practice.
    pub async fn source_link_for_workspace(
        &self,
        practice_id: &str,
        source_id: &str,
        workspace_id: &str,
    ) -> Result<Option<SourcePatientLink>, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("source_id", source_id)?;
        validate_non_empty("workspace_id", workspace_id)?;

        let tx = self.db.begin().await?;
        verify_workspace_practice_tx(&tx, practice_id, workspace_id).await?;
        let row = source_patient_link::Entity::find()
            .filter(source_patient_link::Column::Practice.eq(practice_id))
            .filter(source_patient_link::Column::SourceId.eq(source_id))
            .filter(source_patient_link::Column::Workspace.eq(workspace_id))
            .one(&tx)
            .await?;
        tx.commit().await?;
        Ok(row.map(decode_link))
    }

    /// Lists all linked patients for `(practice_id, source_id)` ordered by `source_patient_id`.
    pub async fn practice_source_links(
        &self,
        practice_id: &str,
        source_id: &str,
    ) -> Result<Vec<SourcePatientLink>, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("source_id", source_id)?;

        let rows = source_patient_link::Entity::find()
            .filter(source_patient_link::Column::Practice.eq(practice_id))
            .filter(source_patient_link::Column::SourceId.eq(source_id))
            .order_by_asc(source_patient_link::Column::SourcePatientId)
            .all(&self.db)
            .await?;
        Ok(rows.into_iter().map(decode_link).collect())
    }

    /// Sets the practice-level access state for `(practice_id, source_id)` and advances
    /// `access_seq` so in-flight pre-reconnect `LapsedAccess` pulls cannot overwrite recovery.
    pub async fn set_practice_source_access(
        &self,
        practice_id: &str,
        source_id: &str,
        access: &SourceAccessState,
        updated_at: i64,
    ) -> Result<(), SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("source_id", source_id)?;
        let tx = self.db.begin().await?;
        upsert_practice_source_access_tx(&tx, practice_id, source_id, access, updated_at).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Reads the practice-level access state for `(practice_id, source_id)`, if recorded.
    pub async fn practice_source_access(
        &self,
        practice_id: &str,
        source_id: &str,
    ) -> Result<Option<SourceAccessState>, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("source_id", source_id)?;

        let row = practice_source_access::Entity::find_by_id((
            practice_id.to_owned(),
            source_id.to_owned(),
        ))
        .one(&self.db)
        .await?;
        Ok(row.map(decode_access).transpose()?)
    }

    /// Records a pull attempt for `(practice_id, workspace_id, source_id)`.
    ///
    /// Advances `last_good_read_at` only on [`PullOutcome::Succeeded`]; failed,
    /// partial, and lapsed-access attempts preserve `last_good_read_at`.
    pub async fn record_pull_status(
        &self,
        practice_id: &str,
        workspace_id: &str,
        source_id: &str,
        attempt: &PullAttempt,
    ) -> Result<PatientSourceRecordStatus, SourceStatusError> {
        let (status, _) = self
            .record_pull_status_fenced(practice_id, workspace_id, source_id, attempt, None, None)
            .await?;
        Ok(status)
    }

    /// Records a pull attempt ordered by `ticket` and optionally fenced by an active
    /// [`ReconnectCommandFence`], leaving stored state untouched if a newer pull or practice
    /// reconnect has already superseded this attempt.
    pub async fn record_pull_status_fenced(
        &self,
        practice_id: &str,
        workspace_id: &str,
        source_id: &str,
        attempt: &PullAttempt,
        ticket: Option<PullTicket>,
        fence: Option<ReconnectCommandFence<'_>>,
    ) -> Result<(PatientSourceRecordStatus, PullDisposition), SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("workspace_id", workspace_id)?;
        validate_non_empty("source_id", source_id)?;

        let tx = self.db.begin().await?;
        verify_reconnect_fence_tx(&tx, fence).await?;
        match prepare_pull_order_tx(&tx, practice_id, workspace_id, source_id, ticket, attempt)
            .await?
        {
            PreparedPullOrder::Superseded { status, .. } => {
                tx.commit().await?;
                Ok((status, PullDisposition::Superseded))
            }
            PreparedPullOrder::Apply {
                applied_seq,
                prior_good,
                ..
            } => {
                let status = write_applied_pull_status_tx(
                    &tx,
                    practice_id,
                    workspace_id,
                    source_id,
                    attempt,
                    prior_good,
                    applied_seq,
                )
                .await?;
                tx.commit().await?;
                Ok((status, PullDisposition::Applied))
            }
        }
    }

    /// Atomically commits a complete chart delivery (`physician_runtime_sources`,
    /// `physician_runtime_source_pulls` with `pulled_at = Some(delivery.pulled_at)`)
    /// and its [`PullOutcome::Succeeded`] status row in one Postgres transaction.
    pub async fn record_complete_pull(
        &self,
        delivery: CompleteDelivery<'_>,
    ) -> Result<(ChartPullReceipt, PatientSourceRecordStatus), SourceStatusError> {
        let (receipt, status, _) = self.record_complete_pull_fenced(delivery, None).await?;
        Ok((receipt, status))
    }

    /// Atomically commits a complete chart delivery and status row ordered by `delivery.ticket`
    /// and optionally fenced by an active [`ReconnectCommandFence`] so superseded executions or
    /// older checks never overwrite newer records or timestamps.
    pub async fn record_complete_pull_fenced(
        &self,
        delivery: CompleteDelivery<'_>,
        fence: Option<ReconnectCommandFence<'_>>,
    ) -> Result<(ChartPullReceipt, PatientSourceRecordStatus, PullDisposition), SourceStatusError>
    {
        validate_non_empty("practice_id", delivery.practice_id)?;
        validate_non_empty("workspace_id", delivery.workspace_id)?;
        validate_non_empty("source_id", delivery.source_id)?;

        let probe_attempt = PullAttempt {
            attempted_at: delivery.pulled_at,
            outcome: PullOutcome::Succeeded {
                missing_candidates: Vec::new(),
            },
        };
        let tx = self.db.begin().await?;
        verify_reconnect_fence_tx(&tx, fence).await?;
        let (applied_seq, prior_good, existing_pull) = match prepare_pull_order_tx(
            &tx,
            delivery.practice_id,
            delivery.workspace_id,
            delivery.source_id,
            delivery.ticket,
            &probe_attempt,
        )
        .await?
        {
            PreparedPullOrder::Superseded {
                status,
                missing_candidates,
            } => {
                tx.commit().await?;
                return Ok((
                    ChartPullReceipt {
                        changed: 0,
                        missing_candidates,
                    },
                    status,
                    PullDisposition::Superseded,
                ));
            }
            PreparedPullOrder::Apply {
                applied_seq,
                prior_good,
                existing_pull,
            } => (applied_seq, prior_good, existing_pull),
        };

        let receipt = apply_source_delivery_tx(
            &tx,
            delivery.workspace_id,
            delivery.source_id,
            delivery.records,
            MissingScope::FullChart,
        )
        .await?;
        let effective_pulled_at = existing_pull
            .and_then(|pull| pull.pulled_at)
            .map_or(delivery.pulled_at, |prev| prev.max(delivery.pulled_at));
        let patient_json = serde_json::to_value(delivery.patient)
            .map_err(conversion_error("SourcePatient", "source pull patient"))?;
        let missing_json = serde_json::to_value(&receipt.missing_candidates).map_err(
            conversion_error("Vec<String>", "source pull missing candidates"),
        )?;
        source_pull::Entity::insert(source_pull::ActiveModel {
            workspace: Set(delivery.workspace_id.to_owned()),
            source_id: Set(delivery.source_id.to_owned()),
            source_label: Set(delivery.source_label.to_owned()),
            patient: Set(patient_json),
            pulled_at: Set(Some(effective_pulled_at)),
            missing_candidates: Set(missing_json),
        })
        .on_conflict(
            OnConflict::columns([
                source_pull::Column::Workspace,
                source_pull::Column::SourceId,
            ])
            .update_columns([
                source_pull::Column::SourceLabel,
                source_pull::Column::Patient,
                source_pull::Column::PulledAt,
                source_pull::Column::MissingCandidates,
            ])
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;

        let attempt = PullAttempt {
            attempted_at: delivery.pulled_at,
            outcome: PullOutcome::Succeeded {
                missing_candidates: receipt.missing_candidates.clone(),
            },
        };
        let status = write_applied_pull_status_tx(
            &tx,
            delivery.practice_id,
            delivery.workspace_id,
            delivery.source_id,
            &attempt,
            prior_good,
            applied_seq,
        )
        .await?;
        tx.commit().await?;
        Ok((receipt, status, PullDisposition::Applied))
    }

    /// Atomically commits the succeeded portion of a partial pull (`physician_runtime_sources`,
    /// optional `physician_runtime_source_pulls` directory identity with `pulled_at = None`
    /// on fresh insert while preserving any existing complete `pulled_at`) and its
    /// [`PullOutcome::Partial`] status row in one Postgres transaction.
    pub async fn record_partial_pull(
        &self,
        delivery: PartialDelivery<'_>,
    ) -> Result<(ChartPullReceipt, PatientSourceRecordStatus), SourceStatusError> {
        let (receipt, status, _) = self.record_partial_pull_fenced(delivery, None).await?;
        Ok((receipt, status))
    }

    /// Atomically commits a partial chart delivery and status row ordered by `delivery.ticket`
    /// and optionally fenced by an active [`ReconnectCommandFence`] so superseded executions or
    /// older checks never overwrite newer records or status.
    pub async fn record_partial_pull_fenced(
        &self,
        delivery: PartialDelivery<'_>,
        fence: Option<ReconnectCommandFence<'_>>,
    ) -> Result<(ChartPullReceipt, PatientSourceRecordStatus, PullDisposition), SourceStatusError>
    {
        validate_non_empty("practice_id", delivery.practice_id)?;
        validate_non_empty("workspace_id", delivery.workspace_id)?;
        validate_non_empty("source_id", delivery.source_id)?;

        let probe_attempt = PullAttempt {
            attempted_at: delivery.attempted_at,
            outcome: PullOutcome::Partial {
                failed_parts: delivery.failed_parts.to_vec(),
                missing_candidates: Vec::new(),
            },
        };
        let tx = self.db.begin().await?;
        verify_reconnect_fence_tx(&tx, fence).await?;
        let (applied_seq, prior_good, existing_pull) = match prepare_pull_order_tx(
            &tx,
            delivery.practice_id,
            delivery.workspace_id,
            delivery.source_id,
            delivery.ticket,
            &probe_attempt,
        )
        .await?
        {
            PreparedPullOrder::Superseded {
                status,
                missing_candidates,
            } => {
                tx.commit().await?;
                return Ok((
                    ChartPullReceipt {
                        changed: 0,
                        missing_candidates,
                    },
                    status,
                    PullDisposition::Superseded,
                ));
            }
            PreparedPullOrder::Apply {
                applied_seq,
                prior_good,
                existing_pull,
            } => (applied_seq, prior_good, existing_pull),
        };

        let receipt = apply_source_delivery_tx(
            &tx,
            delivery.workspace_id,
            delivery.source_id,
            delivery.records,
            MissingScope::SucceededKinds(delivery.succeeded_kinds),
        )
        .await?;
        let attempt = PullAttempt {
            attempted_at: delivery.attempted_at,
            outcome: PullOutcome::Partial {
                failed_parts: delivery.failed_parts.to_vec(),
                missing_candidates: receipt.missing_candidates.clone(),
            },
        };
        let status = write_applied_pull_status_tx(
            &tx,
            delivery.practice_id,
            delivery.workspace_id,
            delivery.source_id,
            &attempt,
            prior_good,
            applied_seq,
        )
        .await?;

        let missing_json = serde_json::to_value(&receipt.missing_candidates).map_err(
            conversion_error("Vec<String>", "source pull missing candidates"),
        )?;
        if let Some(source_patient) = delivery.patient {
            let patient_json = serde_json::to_value(source_patient)
                .map_err(conversion_error("SourcePatient", "source pull patient"))?;
            source_pull::Entity::insert(source_pull::ActiveModel {
                workspace: Set(delivery.workspace_id.to_owned()),
                source_id: Set(delivery.source_id.to_owned()),
                source_label: Set(delivery.source_label.to_owned()),
                patient: Set(patient_json),
                pulled_at: Set(None),
                missing_candidates: Set(missing_json),
            })
            .on_conflict(
                OnConflict::columns([
                    source_pull::Column::Workspace,
                    source_pull::Column::SourceId,
                ])
                .update_columns([
                    source_pull::Column::SourceLabel,
                    source_pull::Column::Patient,
                    source_pull::Column::MissingCandidates,
                ])
                .to_owned(),
            )
            .exec_without_returning(&tx)
            .await?;
        } else if let Some(existing_pull) = existing_pull {
            source_pull::ActiveModel {
                workspace: Unchanged(existing_pull.workspace),
                source_id: Unchanged(existing_pull.source_id),
                source_label: Unchanged(existing_pull.source_label),
                patient: Unchanged(existing_pull.patient),
                pulled_at: Unchanged(existing_pull.pulled_at),
                missing_candidates: Set(missing_json),
            }
            .update(&tx)
            .await?;
        }
        tx.commit().await?;
        Ok((receipt, status, PullDisposition::Applied))
    }

    /// Verifies and renews `fence`'s lease ownership on `physician_runtime_reconnect_commands`.
    pub async fn renew_reconnect_command_lease(
        &self,
        fence: ReconnectCommandFence<'_>,
    ) -> Result<(), SourceStatusError> {
        let tx = self.db.begin().await?;
        verify_reconnect_fence_tx(&tx, Some(fence)).await?;
        tx.commit().await?;
        Ok(())
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "claiming a reconnect command requires practice, physician, key, source, adapter presence, payload, and lease window"
    )]
    pub(crate) async fn claim_reconnect_command(
        &self,
        practice_id: &str,
        physician_id: &str,
        key: &str,
        source_id: &str,
        presence: AdapterPresence,
        command_payload: &Value,
        lease_seconds: i64,
    ) -> Result<ReconnectCommandClaim, SourceStatusError> {
        let lease_seconds = lease_seconds.max(2);
        let tx = self.db.begin().await?;
        let existing_cmd = command::Entity::find_by_id((
            practice_id.to_owned(),
            physician_id.to_owned(),
            key.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;

        if let Some(existing_cmd) = existing_cmd {
            if !super::receipt::matches(&existing_cmd.command, command_payload) {
                tx.commit().await?;
                return Ok(ReconnectCommandClaim::ConflictingKey);
            }
            let existing_state = reconnect_command::Entity::find_by_id((
                practice_id.to_owned(),
                physician_id.to_owned(),
                key.to_owned(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?;

            if let Some(row) = existing_state {
                let state = decode_reconnect_state(row.state.clone())?;
                if let ReconnectCommandState::Completed {
                    receipt,
                    access,
                    summary,
                } = state
                {
                    tx.commit().await?;
                    return Ok(ReconnectCommandClaim::AlreadyCompleted {
                        receipt,
                        access,
                        summary,
                    });
                }
                let db_now = now(&tx).await?;
                if row.lease_until.is_some_and(|until| until > db_now) {
                    tx.commit().await?;
                    return Ok(ReconnectCommandClaim::WaitForActiveLease);
                }
                if matches!(presence, AdapterPresence::Unregistered) {
                    tx.commit().await?;
                    return Ok(ReconnectCommandClaim::UnregisteredSource);
                }
                let lease_owner = Uuid::new_v4().to_string();
                let lease_until = db_now + lease_seconds;
                reconnect_command::ActiveModel {
                    practice: Unchanged(row.practice),
                    physician: Unchanged(row.physician),
                    key: Unchanged(row.key),
                    source_id: Unchanged(row.source_id),
                    state: Unchanged(row.state),
                    lease_owner: Set(Some(lease_owner.clone())),
                    lease_until: Set(Some(lease_until)),
                    updated_at: Set(db_now),
                }
                .update(&tx)
                .await?;
                tx.commit().await?;
                return Ok(ReconnectCommandClaim::Execute { lease_owner, state });
            }

            if matches!(presence, AdapterPresence::Unregistered) {
                tx.commit().await?;
                return Ok(ReconnectCommandClaim::UnregisteredSource);
            }
            let db_now = now(&tx).await?;
            let lease_owner = Uuid::new_v4().to_string();
            let state = ReconnectCommandState::Claimed {
                source_id: source_id.to_owned(),
            };
            let state_json = serde_json::to_value(&state)
                .map_err(conversion_error("ReconnectCommandState", "Json"))?;
            reconnect_command::Entity::insert(reconnect_command::ActiveModel {
                practice: Set(practice_id.to_owned()),
                physician: Set(physician_id.to_owned()),
                key: Set(key.to_owned()),
                source_id: Set(source_id.to_owned()),
                state: Set(state_json),
                lease_owner: Set(Some(lease_owner.clone())),
                lease_until: Set(Some(db_now + lease_seconds)),
                updated_at: Set(db_now),
            })
            .exec_without_returning(&tx)
            .await?;
            tx.commit().await?;
            return Ok(ReconnectCommandClaim::Execute { lease_owner, state });
        }

        if matches!(presence, AdapterPresence::Unregistered) {
            return Ok(ReconnectCommandClaim::UnregisteredSource);
        }

        match super::receipt::record(&tx, practice_id, physician_id, key, command_payload).await? {
            Receipt::Different => {
                tx.commit().await?;
                Ok(ReconnectCommandClaim::ConflictingKey)
            }
            Receipt::Same => {
                tx.commit().await?;
                Ok(ReconnectCommandClaim::WaitForActiveLease)
            }
            Receipt::New => {
                let db_now = now(&tx).await?;
                let lease_owner = Uuid::new_v4().to_string();
                let state = ReconnectCommandState::Claimed {
                    source_id: source_id.to_owned(),
                };
                let state_json = serde_json::to_value(&state)
                    .map_err(conversion_error("ReconnectCommandState", "Json"))?;
                reconnect_command::Entity::insert(reconnect_command::ActiveModel {
                    practice: Set(practice_id.to_owned()),
                    physician: Set(physician_id.to_owned()),
                    key: Set(key.to_owned()),
                    source_id: Set(source_id.to_owned()),
                    state: Set(state_json),
                    lease_owner: Set(Some(lease_owner.clone())),
                    lease_until: Set(Some(db_now + lease_seconds)),
                    updated_at: Set(db_now),
                })
                .exec_without_returning(&tx)
                .await?;
                tx.commit().await?;
                Ok(ReconnectCommandClaim::Execute { lease_owner, state })
            }
        }
    }

    pub(crate) async fn record_reconnect_command_reconnected(
        &self,
        fence: ReconnectCommandFence<'_>,
        receipt: &ReconnectReceipt,
        access: &SourceAccessState,
    ) -> Result<(), SourceStatusError> {
        let lost = || SourceStatusError::CommandLeaseLost {
            practice_id: fence.practice_id.to_owned(),
            physician_id: fence.physician_id.to_owned(),
            idempotency_key: fence.idempotency_key.to_owned(),
        };
        let tx = self.db.begin().await?;
        let row = reconnect_command::Entity::find_by_id((
            fence.practice_id.to_owned(),
            fence.physician_id.to_owned(),
            fence.idempotency_key.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(lost)?;

        let existing = decode_reconnect_state(row.state.clone())?;
        if matches!(existing, ReconnectCommandState::Completed { .. })
            || row.lease_owner.as_deref() != Some(fence.lease_owner)
        {
            return Err(lost());
        }

        upsert_practice_source_access_tx(
            &tx,
            fence.practice_id,
            &receipt.source_id,
            access,
            receipt.reconnected_at,
        )
        .await?;

        let db_now = now(&tx).await?;
        let next_state = ReconnectCommandState::Reconnected {
            receipt: receipt.clone(),
            access: access.clone(),
        };
        let state_json = serde_json::to_value(&next_state)
            .map_err(conversion_error("ReconnectCommandState", "Json"))?;
        reconnect_command::ActiveModel {
            practice: Unchanged(row.practice),
            physician: Unchanged(row.physician),
            key: Unchanged(row.key),
            source_id: Unchanged(row.source_id),
            state: Set(state_json),
            lease_owner: Unchanged(row.lease_owner),
            lease_until: Set(Some(db_now + fence.lease_seconds.max(2))),
            updated_at: Set(db_now),
        }
        .update(&tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub(crate) async fn complete_reconnect_command(
        &self,
        fence: ReconnectCommandFence<'_>,
        completed: ReconnectCommandState,
    ) -> Result<(), SourceStatusError> {
        let lost = || SourceStatusError::CommandLeaseLost {
            practice_id: fence.practice_id.to_owned(),
            physician_id: fence.physician_id.to_owned(),
            idempotency_key: fence.idempotency_key.to_owned(),
        };
        let tx = self.db.begin().await?;
        let row = reconnect_command::Entity::find_by_id((
            fence.practice_id.to_owned(),
            fence.physician_id.to_owned(),
            fence.idempotency_key.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(lost)?;

        let existing = decode_reconnect_state(row.state.clone())?;
        if matches!(existing, ReconnectCommandState::Completed { .. })
            || row.lease_owner.as_deref() != Some(fence.lease_owner)
        {
            return Err(lost());
        }

        let db_now = now(&tx).await?;
        let state_json = serde_json::to_value(&completed)
            .map_err(conversion_error("ReconnectCommandState", "Json"))?;
        reconnect_command::ActiveModel {
            practice: Unchanged(row.practice),
            physician: Unchanged(row.physician),
            key: Unchanged(row.key),
            source_id: Unchanged(row.source_id),
            state: Set(state_json),
            lease_owner: Set(None),
            lease_until: Set(None),
            updated_at: Set(db_now),
        }
        .update(&tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Reads all stored source status rows for `(practice_id, workspace_id)`, refusing
    /// if `workspace_id` belongs to a different practice.
    pub async fn patient_source_statuses(
        &self,
        practice_id: &str,
        workspace_id: &str,
    ) -> Result<Vec<PatientSourceRecordStatus>, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("workspace_id", workspace_id)?;

        let tx = self.db.begin().await?;
        verify_workspace_practice_tx(&tx, practice_id, workspace_id).await?;
        let rows = patient_source_status::Entity::find()
            .filter(patient_source_status::Column::Practice.eq(practice_id))
            .filter(patient_source_status::Column::Workspace.eq(workspace_id))
            .order_by_asc(patient_source_status::Column::SourceId)
            .all(&tx)
            .await?;
        tx.commit().await?;
        rows.into_iter()
            .map(|row| decode_status(row).map_err(SourceStatusError::from))
            .collect()
    }

    /// Derives [`SourceStanding`] per `(workspace_id, source_id)` for a batch of workspaces
    /// using each workspace's bound practice access and status rows.
    pub async fn workspace_source_standings(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, BTreeMap<String, SourceStanding>>, SourceStatusError> {
        if workspaces.is_empty() {
            return Ok(BTreeMap::new());
        }
        let bindings = workspace_practice::Entity::find()
            .filter(workspace_practice::Column::Workspace.is_in(workspaces.iter().cloned()))
            .all(&self.db)
            .await?;
        if bindings.is_empty() {
            return Ok(BTreeMap::new());
        }

        let practice_by_ws: HashMap<String, String> = bindings
            .into_iter()
            .map(|row| (row.workspace, row.practice))
            .collect();
        let practices: Vec<String> = practice_by_ws
            .values()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let access_rows = practice_source_access::Entity::find()
            .filter(practice_source_access::Column::Practice.is_in(practices.clone()))
            .all(&self.db)
            .await?;
        let mut access_map: HashMap<(String, String), SourceAccessState> = HashMap::new();
        for row in access_rows {
            let key = (row.practice.clone(), row.source_id.clone());
            access_map.insert(key, decode_access(row)?);
        }

        let status_rows = patient_source_status::Entity::find()
            .filter(patient_source_status::Column::Practice.is_in(practices))
            .filter(patient_source_status::Column::Workspace.is_in(workspaces.iter().cloned()))
            .order_by_asc(patient_source_status::Column::SourceId)
            .all(&self.db)
            .await?;
        let mut status_map: HashMap<(String, String, String), PatientSourceRecordStatus> =
            HashMap::new();
        let mut sources_by_practice: HashMap<String, BTreeSet<String>> = HashMap::new();
        for (practice, source_id) in access_map.keys() {
            sources_by_practice
                .entry(practice.clone())
                .or_default()
                .insert(source_id.clone());
        }
        for row in status_rows {
            let decoded = decode_status(row)?;
            sources_by_practice
                .entry(decoded.practice_id.clone())
                .or_default()
                .insert(decoded.source_id.clone());
            status_map.insert(
                (
                    decoded.practice_id.clone(),
                    decoded.workspace_id.clone(),
                    decoded.source_id.clone(),
                ),
                decoded,
            );
        }

        let mut out: BTreeMap<String, BTreeMap<String, SourceStanding>> = BTreeMap::new();
        for (workspace_id, practice_id) in &practice_by_ws {
            let Some(source_ids) = sources_by_practice.get(practice_id) else {
                continue;
            };
            let ws_entry = out.entry(workspace_id.clone()).or_default();
            for source_id in source_ids {
                let access = access_map.get(&(practice_id.clone(), source_id.clone()));
                let status =
                    status_map.get(&(practice_id.clone(), workspace_id.clone(), source_id.clone()));
                ws_entry.insert(source_id.clone(), derive_standing(access, status));
            }
        }
        Ok(out)
    }

    /// Builds the authoritative [`PracticeSourceReport`] for `(practice_id, source_id)`.
    pub async fn practice_source_report(
        &self,
        practice_id: &str,
        source_id: &str,
    ) -> Result<PracticeSourceReport, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("source_id", source_id)?;

        let access = self.practice_source_access(practice_id, source_id).await?;
        let links = self.practice_source_links(practice_id, source_id).await?;
        let status_rows = patient_source_status::Entity::find()
            .filter(patient_source_status::Column::Practice.eq(practice_id))
            .filter(patient_source_status::Column::SourceId.eq(source_id))
            .order_by_asc(patient_source_status::Column::Workspace)
            .all(&self.db)
            .await?;

        let mut status_by_ws: BTreeMap<String, PatientSourceRecordStatus> = BTreeMap::new();
        for row in status_rows {
            let decoded = decode_status(row)?;
            status_by_ws.insert(decoded.workspace_id.clone(), decoded);
        }

        let mut workspaces: BTreeSet<String> =
            links.into_iter().map(|link| link.workspace_id).collect();
        for ws in status_by_ws.keys() {
            workspaces.insert(ws.clone());
        }

        let patients: Vec<(String, SourceStanding)> = workspaces
            .into_iter()
            .map(|workspace_id| {
                let standing =
                    derive_standing(access.as_ref(), status_by_ws.get(workspace_id.as_str()));
                (workspace_id, standing)
            })
            .collect();

        let summary = summarize_practice_source(practice_id, source_id, access.as_ref(), &patients);
        Ok(PracticeSourceReport {
            practice_id: practice_id.to_owned(),
            source_id: source_id.to_owned(),
            access,
            summary,
            patients,
        })
    }

    /// Builds [`PracticeSourceReport`] items for every source known to `practice_id`
    /// (via links, access state, status rows, or `registered_sources`), sorted by `source_id`.
    pub async fn practice_all_source_reports(
        &self,
        practice_id: &str,
        registered_sources: &[String],
    ) -> Result<Vec<PracticeSourceReport>, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;

        let mut source_ids: BTreeSet<String> = registered_sources.iter().cloned().collect();

        let link_sources: Vec<String> = source_patient_link::Entity::find()
            .filter(source_patient_link::Column::Practice.eq(practice_id))
            .select_only()
            .column(source_patient_link::Column::SourceId)
            .distinct()
            .into_tuple()
            .all(&self.db)
            .await?;
        source_ids.extend(link_sources);

        let access_sources: Vec<String> = practice_source_access::Entity::find()
            .filter(practice_source_access::Column::Practice.eq(practice_id))
            .select_only()
            .column(practice_source_access::Column::SourceId)
            .distinct()
            .into_tuple()
            .all(&self.db)
            .await?;
        source_ids.extend(access_sources);

        let status_sources: Vec<String> = patient_source_status::Entity::find()
            .filter(patient_source_status::Column::Practice.eq(practice_id))
            .select_only()
            .column(patient_source_status::Column::SourceId)
            .distinct()
            .into_tuple()
            .all(&self.db)
            .await?;
        source_ids.extend(status_sources);

        let mut reports = Vec::with_capacity(source_ids.len());
        for source_id in source_ids {
            reports.push(self.practice_source_report(practice_id, &source_id).await?);
        }
        Ok(reports)
    }

    /// Reads shared adapter-owned state for `(practice_id, source_id)` from Postgres, returning
    /// `S::default()` when no state row has been written yet.
    pub async fn read_source_adapter_state<S>(
        &self,
        practice_id: &str,
        source_id: &str,
    ) -> Result<S, SourceAdapterError>
    where
        S: Default + serde::de::DeserializeOwned,
    {
        validate_non_empty("practice_id", practice_id)
            .map_err(|err| adapter_state_error("validate source adapter state scope", err))?;
        validate_non_empty("source_id", source_id)
            .map_err(|err| adapter_state_error("validate source adapter state scope", err))?;

        let Some(row) = source_adapter_state::Entity::find_by_id((
            practice_id.to_owned(),
            source_id.to_owned(),
        ))
        .one(&self.db)
        .await
        .map_err(|err| adapter_state_error("read source adapter state", err))?
        else {
            return Ok(S::default());
        };

        serde_json::from_value(row.state)
            .map_err(|err| adapter_state_error("decode source adapter state", err))
    }

    /// Atomically locks, mutates, and persists shared adapter-owned state for
    /// `(practice_id, source_id)` in a single Postgres transaction (`SELECT ... FOR UPDATE`),
    /// rolling back if `mutate` returns an error.
    pub async fn mutate_source_adapter_state<S, T>(
        &self,
        practice_id: &str,
        source_id: &str,
        mutate: impl FnOnce(&mut S) -> Result<T, SourceAdapterError>,
    ) -> Result<T, SourceAdapterError>
    where
        S: Default + serde::Serialize + serde::de::DeserializeOwned,
    {
        validate_non_empty("practice_id", practice_id)
            .map_err(|err| adapter_state_error("validate source adapter state scope", err))?;
        validate_non_empty("source_id", source_id)
            .map_err(|err| adapter_state_error("validate source adapter state scope", err))?;

        let initial_state = serde_json::to_value(S::default())
            .map_err(|err| adapter_state_error("encode initial source adapter state", err))?;

        let tx =
            self.db.begin().await.map_err(|err| {
                adapter_state_error("begin source adapter state transaction", err)
            })?;
        let clock_now = now(&tx)
            .await
            .map_err(|err| adapter_state_error("read database clock for adapter state", err))?;

        source_adapter_state::Entity::insert(source_adapter_state::ActiveModel {
            practice: Set(practice_id.to_owned()),
            source_id: Set(source_id.to_owned()),
            state: Set(initial_state),
            updated_at: Set(clock_now),
        })
        .on_conflict(
            OnConflict::columns([
                source_adapter_state::Column::Practice,
                source_adapter_state::Column::SourceId,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await
        .map_err(|err| adapter_state_error("ensure source adapter state row", err))?;

        let row = source_adapter_state::Entity::find_by_id((
            practice_id.to_owned(),
            source_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(|err| adapter_state_error("lock source adapter state row", err))?
        .ok_or_else(|| {
            adapter_state_error(
                "lock source adapter state row",
                error("missing source adapter state row after insert"),
            )
        })?;

        let mut state: S = serde_json::from_value(row.state)
            .map_err(|err| adapter_state_error("decode source adapter state", err))?;
        let output = mutate(&mut state)?;
        let encoded = serde_json::to_value(&state)
            .map_err(|err| adapter_state_error("encode source adapter state", err))?;

        source_adapter_state::ActiveModel {
            practice: Unchanged(row.practice),
            source_id: Unchanged(row.source_id),
            state: Set(encoded),
            updated_at: Set(clock_now),
        }
        .update(&tx)
        .await
        .map_err(|err| adapter_state_error("update source adapter state row", err))?;

        tx.commit()
            .await
            .map_err(|err| adapter_state_error("commit source adapter state transaction", err))?;
        Ok(output)
    }
}

fn adapter_state_error(
    context: impl Into<String>,
    source: impl std::error::Error + Send + Sync + 'static,
) -> SourceAdapterError {
    SourceAdapterError::External {
        context: context.into(),
        source: Box::new(source),
    }
}
