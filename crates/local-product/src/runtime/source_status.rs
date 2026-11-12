//! SeaORM persistence for practice-scoped workspace ownership, source patient links,
//! practice-level source access state, and per-patient pull status.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionTrait, sea_query::OnConflict,
};

use super::entities::{
    patient_source_status, practice_source_access, source_patient_link, source_pull,
    workspace_practice,
};
use super::sources::{MissingScope, apply_source_delivery_tx};
use super::{Store, conversion_error, error};
use crate::physician_sources::PullOutcome;
use crate::source_status::{
    LinkTarget, PartialDelivery, PatientSourceRecordStatus, PracticeSourceReport, PullAttempt,
    SourceAccessState, SourcePatientKey, SourcePatientLink, SourceStanding, SourceStatusError,
    derive_standing, summarize_practice_source, validate_non_empty,
};

async fn ensure_workspace_practice_tx(
    tx: &impl ConnectionTrait,
    practice_id: &str,
    workspace_id: &str,
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

    let row = workspace_practice::Entity::find_by_id(workspace_id.to_owned())
        .lock_shared()
        .one(tx)
        .await?
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

impl Store {
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
        key.validate()?;
        validate_non_empty("workspace_id", target.workspace())?;

        let tx = self.db.begin().await?;
        let existing = source_patient_link::Entity::find_by_id((
            key.practice_id.clone(),
            key.source_id.clone(),
            key.source_patient_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;

        if let Some(existing) = existing {
            if let LinkTarget::Exact(requested) = target
                && requested != existing.workspace
            {
                return Err(SourceStatusError::ConflictingWorkspaceLink {
                    practice_id: key.practice_id.clone(),
                    source_id: key.source_id.clone(),
                    source_patient_id: key.source_patient_id.clone(),
                    existing_workspace_id: existing.workspace,
                    requested_workspace_id: requested.to_owned(),
                });
            }
            ensure_workspace_practice_tx(&tx, &key.practice_id, &existing.workspace).await?;
            tx.commit().await?;
            return Ok(decode_link(existing));
        }

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
        })
        .exec_without_returning(&tx)
        .await?;

        tx.commit().await?;
        Ok(SourcePatientLink {
            key: key.clone(),
            workspace_id: candidate_workspace.to_owned(),
            linked_at,
        })
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

    /// Sets the practice-level access state for `(practice_id, source_id)`.
    pub async fn set_practice_source_access(
        &self,
        practice_id: &str,
        source_id: &str,
        access: &SourceAccessState,
        updated_at: i64,
    ) -> Result<(), SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("source_id", source_id)?;
        let access_json =
            serde_json::to_value(access).map_err(conversion_error("SourceAccessState", "Json"))?;

        practice_source_access::Entity::insert(practice_source_access::ActiveModel {
            practice: Set(practice_id.to_owned()),
            source_id: Set(source_id.to_owned()),
            access: Set(access_json),
            updated_at: Set(updated_at),
        })
        .on_conflict(
            OnConflict::columns([
                practice_source_access::Column::Practice,
                practice_source_access::Column::SourceId,
            ])
            .update_columns([
                practice_source_access::Column::Access,
                practice_source_access::Column::UpdatedAt,
            ])
            .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
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
        validate_non_empty("practice_id", practice_id)?;
        validate_non_empty("workspace_id", workspace_id)?;
        validate_non_empty("source_id", source_id)?;

        let attempt_json =
            serde_json::to_value(attempt).map_err(conversion_error("PullAttempt", "Json"))?;

        let tx = self.db.begin().await?;
        ensure_workspace_practice_tx(&tx, practice_id, workspace_id).await?;

        let existing = patient_source_status::Entity::find_by_id((
            practice_id.to_owned(),
            workspace_id.to_owned(),
            source_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;

        let prior_good = match existing.as_ref() {
            Some(row) => row.last_good_read_at,
            None => {
                source_pull::Entity::find_by_id((workspace_id.to_owned(), source_id.to_owned()))
                    .one(&tx)
                    .await?
                    .and_then(|pull| (pull.pulled_at > 0).then_some(pull.pulled_at))
            }
        };
        let last_good_read_at = match &attempt.outcome {
            PullOutcome::Succeeded { .. } => Some(attempt.attempted_at),
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
            ])
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;

        if matches!(
            attempt.outcome,
            PullOutcome::Succeeded { .. } | PullOutcome::Partial { .. }
        ) {
            let existing_access = practice_source_access::Entity::find_by_id((
                practice_id.to_owned(),
                source_id.to_owned(),
            ))
            .one(&tx)
            .await?
            .map(decode_access)
            .transpose()?;
            let should_set_connected = match existing_access {
                None => true,
                Some(SourceAccessState::ReconnectNeeded { since, .. }) => {
                    attempt.attempted_at > since
                }
                Some(SourceAccessState::Connected { .. }) => false,
            };
            if should_set_connected {
                let connected = SourceAccessState::Connected {
                    connected_at: attempt.attempted_at,
                };
                let access_json = serde_json::to_value(&connected)
                    .map_err(conversion_error("SourceAccessState", "Json"))?;
                practice_source_access::Entity::insert(practice_source_access::ActiveModel {
                    practice: Set(practice_id.to_owned()),
                    source_id: Set(source_id.to_owned()),
                    access: Set(access_json),
                    updated_at: Set(attempt.attempted_at),
                })
                .on_conflict(
                    OnConflict::columns([
                        practice_source_access::Column::Practice,
                        practice_source_access::Column::SourceId,
                    ])
                    .update_columns([
                        practice_source_access::Column::Access,
                        practice_source_access::Column::UpdatedAt,
                    ])
                    .to_owned(),
                )
                .exec_without_returning(&tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok(PatientSourceRecordStatus {
            practice_id: practice_id.to_owned(),
            workspace_id: workspace_id.to_owned(),
            source_id: source_id.to_owned(),
            last_good_read_at,
            latest_attempt: Some(attempt.clone()),
        })
    }

    /// Saves the records that succeeded during a partial pull without withdrawing
    /// records from failed parts or advancing `source_pull.pulled_at`. When `delivery.patient`
    /// is present, ensures `physician_runtime_source_pulls` holds the patient's directory
    /// identity without fabricating a complete-pull timestamp.
    pub async fn record_partial_records(
        &self,
        delivery: PartialDelivery<'_>,
    ) -> Result<(usize, Vec<String>), SourceStatusError> {
        validate_non_empty("practice_id", delivery.practice_id)?;
        validate_non_empty("workspace_id", delivery.workspace_id)?;
        validate_non_empty("source_id", delivery.source_id)?;

        let tx = self.db.begin().await?;
        ensure_workspace_practice_tx(&tx, delivery.practice_id, delivery.workspace_id).await?;
        let receipt = apply_source_delivery_tx(
            &tx,
            delivery.workspace_id,
            delivery.source_id,
            delivery.records,
            MissingScope::SucceededKinds(delivery.succeeded_kinds),
        )
        .await?;
        if let Some(source_patient) = delivery.patient {
            let patient_json = serde_json::to_value(source_patient)
                .map_err(conversion_error("SourcePatient", "source pull patient"))?;
            let missing_json = serde_json::to_value(&receipt.missing_candidates).map_err(
                conversion_error("Vec<String>", "source pull missing candidates"),
            )?;
            source_pull::Entity::insert(source_pull::ActiveModel {
                workspace: Set(delivery.workspace_id.to_owned()),
                source_id: Set(delivery.source_id.to_owned()),
                source_label: Set(delivery.source_label.to_owned()),
                patient: Set(patient_json),
                pulled_at: Set(0),
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
        }
        tx.commit().await?;
        Ok((receipt.changed, receipt.missing_candidates))
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
}
