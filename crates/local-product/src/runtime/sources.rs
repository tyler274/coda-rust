//! Versioned patient source observations: what every source adapter's records
//! become inside Coda.
//!
//! Each patient workspace has one version row and an append-only list of
//! source observations. Recording a new or corrected revision, or withdrawing
//! a source, appends a row and bumps the version, both under the version
//! row's exclusive lock. Replaying any historical revision changes nothing;
//! reusing its identity with different content or provenance is refused.
//! Readers that must not miss an arrival (revision acceptance, claim and
//! commit) take that row's shared lock in their own transaction, so ingestion
//! either landed before them and is seen, or waits for them to commit.
//!
//! Beside the records, each source's last pull says how current they are.
//!
//! Projection prose and model output never become sources.
use std::collections::{BTreeMap, HashMap, HashSet};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait,
    sea_query::{Expr, OnConflict, extension::postgres::PgExpr},
};

use super::entities::{source, source_pull, source_version};
use super::{Store, conversion_error, error};
use crate::physician_sources::{SourcePatient, SourceRevision};

/// Rows written per insert, inside Postgres's bind limit.
const CHART_BATCH: usize = 200;

/// A workspace's source observation at one version.
#[derive(Clone, Debug, PartialEq)]
pub struct Observation {
    /// The workspace's source version; 0 before anything was recorded.
    pub version: u64,
    /// The newest revision of each source that is not withdrawn, ordered by
    /// candidate ID.
    pub sources: Vec<SourceRevision>,
}

/// One source's last delivery for a workspace: who the patient is by that
/// source's records, and how current they are.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePull {
    /// The source's stable identifier.
    pub source_id: String,
    /// The source's name as a physician sees it.
    pub source_label: String,
    /// The patient as this source identifies them.
    pub patient: SourcePatient,
    /// The practice clock's Unix second when the pull finished.
    pub pulled_at: i64,
}

fn version_of(value: i64) -> Result<u64, DbErr> {
    u64::try_from(value).map_err(|_| error("corrupt source version"))
}

/// Creates the workspace's version row if it is missing. Blocks behind a
/// concurrent uncommitted insert, so the following lock always sees a row.
async fn ensure_version(tx: &impl ConnectionTrait, workspace: &str) -> Result<(), DbErr> {
    source_version::Entity::insert(source_version::ActiveModel {
        workspace: Set(workspace.into()),
        version: Set(0),
    })
    .on_conflict(
        OnConflict::column(source_version::Column::Workspace)
            .do_nothing()
            .to_owned(),
    )
    .exec_without_returning(tx)
    .await?;
    Ok(())
}

/// The newest observation row of each candidate at or below `version`.
async fn latest(
    tx: &impl ConnectionTrait,
    workspace: &str,
    version: u64,
    candidates: Option<&[&str]>,
) -> Result<Vec<source::Model>, DbErr> {
    latest_query(workspace, version, candidates).all(tx).await
}

fn latest_query(
    workspace: &str,
    version: u64,
    candidates: Option<&[&str]>,
) -> sea_orm::Select<source::Entity> {
    let mut query = source::Entity::find()
        .filter(source::Column::Workspace.eq(workspace))
        .filter(source::Column::ObservedAtVersion.lte(version as i64));
    if let Some(ids) = candidates {
        query = query.filter(source::Column::Candidate.is_in(ids.iter().copied()));
    }
    query
        .distinct_on([source::Column::Candidate])
        .order_by_asc(source::Column::Candidate)
        .order_by_desc(source::Column::ObservedAtVersion)
}

fn decode(row: &source::Model) -> Result<Option<SourceRevision>, DbErr> {
    row.revision
        .clone()
        .map(|value| {
            let source: SourceRevision =
                serde_json::from_value(value).map_err(|_| error("corrupt source observation"))?;
            source.validate().map_err(|e| error(&e))?;
            Ok(source)
        })
        .transpose()
}

/// The workspace's current observation, holding its version row's shared
/// lock until `tx` ends. Use inside any transaction that pins or checks
/// sources, after its own row locks (draft, then authority): ingestion takes
/// only the version row, so the order cannot deadlock.
pub(super) async fn observed(
    tx: &impl ConnectionTrait,
    workspace: &str,
) -> Result<Observation, DbErr> {
    selected(tx, workspace, None).await
}

/// Pins the current source version for every reader that must serialize with ingestion.
async fn pin_version(tx: &impl ConnectionTrait, workspace: &str) -> Result<u64, DbErr> {
    ensure_version(tx, workspace).await?;
    let row = source_version::Entity::find_by_id(workspace.to_owned())
        .lock_shared()
        .one(tx)
        .await?
        .ok_or_else(|| error("missing source version"))?;
    version_of(row.version)
}

async fn selected(
    tx: &impl ConnectionTrait,
    workspace: &str,
    candidates: Option<&[&str]>,
) -> Result<Observation, DbErr> {
    let version = pin_version(tx, workspace).await?;
    let mut sources = Vec::new();
    for row in latest(tx, workspace, version, candidates).await? {
        if let Some(source) = decode(&row)? {
            sources.push(source);
        }
    }
    Ok(Observation { version, sources })
}

impl Store {
    /// Records one immutable source revision. An unseen revision appends a row
    /// and bumps the workspace version, including after withdrawal. Replaying
    /// any historical revision changes nothing, even after correction or
    /// withdrawal; conflicting bytes or provenance for its identity are refused.
    /// Returns the workspace version after.
    pub async fn record_source(&self, incoming: &SourceRevision) -> Result<u64, DbErr> {
        incoming.validate().map_err(|e| error(&e))?;
        let workspace = incoming.workspace.as_str();
        let candidate = incoming.candidate_id.as_str();
        if workspace.trim().is_empty() || candidate.trim().is_empty() {
            return Err(error("empty source identity"));
        }
        let revision = serde_json::to_value(incoming)
            .map_err(conversion_error("SourceRevision", "source revision"))?;
        let tx = self.db.begin().await?;
        ensure_version(&tx, workspace).await?;
        let row = source_version::Entity::find_by_id(workspace.to_owned())
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing source version"))?;
        let version = version_of(row.version)?;
        // The version lock serializes identity checks with all appends. Keep
        // the check over history, not just the current row: a startup snapshot
        // may contain a revision that has since been corrected or withdrawn.
        let history = source::Entity::find()
            .filter(source::Column::Workspace.eq(workspace))
            .filter(source::Column::Candidate.eq(candidate))
            .order_by_desc(source::Column::ObservedAtVersion)
            .all(&tx)
            .await?;
        let mut unchanged = false;
        for row in &history {
            if let Some(recorded) = decode(row)?
                && recorded.revision == incoming.revision
            {
                if recorded != *incoming {
                    return Err(error("conflicting source revision identity"));
                }
                unchanged = true;
            }
        }
        if unchanged {
            tx.commit().await?;
            return Ok(version);
        }
        let next = version + 1;
        source::Entity::insert(source::ActiveModel {
            workspace: Set(workspace.into()),
            candidate: Set(candidate.into()),
            observed_at_version: Set(next as i64),
            revision: Set(Some(revision)),
        })
        .exec_without_returning(&tx)
        .await?;
        let mut bumped: source_version::ActiveModel = row.into();
        bumped.version = Set(next as i64);
        bumped.update(&tx).await?;
        tx.commit().await?;
        Ok(next)
    }

    /// Imports a retained snapshot through [`Self::record_source`], so
    /// importing it again on every site start changes nothing.
    pub async fn import_sources(&self, sources: &[SourceRevision]) -> Result<(), DbErr> {
        for source in sources {
            self.record_source(source).await?;
        }
        Ok(())
    }

    /// Records one source's whole delivery for a workspace in one transaction:
    /// each record that differs from the current one, a withdrawal of each
    /// record the source no longer holds, and the pull, so "records as of"
    /// never outruns the records. A delivery says what the source holds now,
    /// so an earlier revision delivered again becomes current, unlike a replay
    /// through [`Self::record_source`]. A revision identity still holds one set
    /// of bytes. One delivery is one version. Returns how many records changed.
    pub async fn record_chart(
        &self,
        workspace: &str,
        records: &[SourceRevision],
        pull: &SourcePull,
    ) -> Result<usize, DbErr> {
        for record in records {
            record.validate().map_err(|e| error(&e))?;
            if record.workspace != workspace || record.source_id != pull.source_id {
                return Err(error("record outside its delivery"));
            }
        }
        let tx = self.db.begin().await?;
        ensure_version(&tx, workspace).await?;
        let row = source_version::Entity::find_by_id(workspace.to_owned())
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing source version"))?;
        let version = version_of(row.version)?;
        let next = version as i64 + 1;
        let history = source::Entity::find()
            .filter(source::Column::Workspace.eq(workspace))
            .order_by_desc(source::Column::ObservedAtVersion)
            .all(&tx)
            .await?;
        let mut seen: HashMap<(String, String), SourceRevision> = HashMap::new();
        let mut current: HashMap<String, Option<SourceRevision>> = HashMap::new();
        for row in &history {
            let decoded = decode(row)?;
            current
                .entry(row.candidate.clone())
                .or_insert_with(|| decoded.clone());
            if let Some(decoded) = decoded {
                seen.insert((row.candidate.clone(), decoded.revision.clone()), decoded);
            }
        }
        let mut rows = Vec::new();
        for record in records {
            let identity = (record.candidate_id.clone(), record.revision.clone());
            if seen
                .get(&identity)
                .is_some_and(|recorded| recorded != record)
            {
                return Err(error("conflicting source revision identity"));
            }
            let holding = current.get(&record.candidate_id).and_then(Option::as_ref);
            if holding.is_some_and(|source| source.revision == record.revision) {
                continue;
            }
            let value = serde_json::to_value(record).map_err(|_| error("invalid source"))?;
            rows.push(source::ActiveModel {
                workspace: Set(workspace.into()),
                candidate: Set(record.candidate_id.clone()),
                observed_at_version: Set(next),
                revision: Set(Some(value)),
            });
        }
        let delivered: HashSet<&str> = records.iter().map(|r| r.candidate_id.as_str()).collect();
        for (candidate, newest) in &current {
            let held = newest
                .as_ref()
                .is_some_and(|source| source.source_id == pull.source_id);
            if held && !delivered.contains(candidate.as_str()) {
                rows.push(source::ActiveModel {
                    workspace: Set(workspace.into()),
                    candidate: Set(candidate.clone()),
                    observed_at_version: Set(next),
                    revision: Set(None),
                });
            }
        }
        let changed = rows.len();
        for batch in rows.chunks(CHART_BATCH) {
            source::Entity::insert_many(batch.to_vec())
                .exec_without_returning(&tx)
                .await?;
        }
        if changed > 0 {
            let mut bumped: source_version::ActiveModel = row.into();
            bumped.version = Set(next);
            bumped.update(&tx).await?;
        }
        let patient =
            serde_json::to_value(&pull.patient).map_err(|_| error("invalid source patient"))?;
        source_pull::Entity::insert(source_pull::ActiveModel {
            workspace: Set(workspace.into()),
            source_id: Set(pull.source_id.clone()),
            source_label: Set(pull.source_label.clone()),
            patient: Set(patient),
            pulled_at: Set(pull.pulled_at),
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
            ])
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        tx.commit().await?;
        Ok(changed)
    }

    /// Each source's last pull for a workspace, ordered by source.
    pub async fn source_pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, DbErr> {
        let rows = source_pull::Entity::find()
            .filter(source_pull::Column::Workspace.is_in(workspaces))
            .order_by_asc(source_pull::Column::SourceId)
            .all(&self.db)
            .await?;
        let mut pulls: BTreeMap<String, Vec<SourcePull>> = BTreeMap::new();
        for row in rows {
            pulls.entry(row.workspace).or_default().push(SourcePull {
                source_id: row.source_id,
                source_label: row.source_label,
                patient: serde_json::from_value(row.patient)
                    .map_err(|_| error("corrupt source patient"))?,
                pulled_at: row.pulled_at,
            });
        }
        Ok(pulls)
    }

    /// Unlocked read of a workspace's current observation.
    pub async fn sources(&self, workspace: &str) -> Result<Observation, DbErr> {
        let tx = self.db.begin().await?;
        let observation = observed(&tx, workspace).await?;
        tx.commit().await?;
        Ok(observation)
    }

    /// Whether the workspace ever held a record from a source other than
    /// `allowed`, including a record since corrected or withdrawn: answers
    /// written from it stay in the workspace's history. Checks source identity
    /// without reading chart bodies, under the same version fence as chart reads.
    pub async fn has_sources_outside(&self, workspace: &str, allowed: &str) -> Result<bool, DbErr> {
        let tx = self.db.begin().await?;
        let version = pin_version(&tx, workspace).await?;
        // A withdrawal row has no revision; the revisions before it name the source.
        let sources = source::Entity::find()
            .filter(source::Column::Workspace.eq(workspace))
            .filter(source::Column::ObservedAtVersion.lte(version as i64))
            .filter(source::Column::Revision.is_not_null())
            .select_only()
            .expr(Expr::col(source::Column::Revision).cast_json_field("source_id"))
            .distinct()
            .into_tuple::<Option<String>>()
            .all(&tx)
            .await?;
        tx.commit().await?;
        let mut outside = false;
        for source in sources {
            outside |= source.ok_or_else(|| error("missing source identity"))? != allowed;
        }
        Ok(outside)
    }

    /// Retained data for an authorized chat tool. An exact read only loads
    /// the requested candidates, including their current withdrawal state and
    /// any retained historical revisions explicitly requested on active records.
    pub async fn chart_observation(
        &self,
        workspace: &str,
        request: &crate::physician_sources::ChartRequest,
    ) -> Result<Observation, DbErr> {
        let tx = self.db.begin().await?;
        let version = pin_version(&tx, workspace).await?;
        let latest_rows = latest(&tx, workspace, version, request.candidates().as_deref()).await?;
        let mut withdrawn_candidates = HashSet::new();
        let mut sources = Vec::new();
        for row in latest_rows {
            match decode(&row)? {
                Some(source) => sources.push(source),
                None => {
                    withdrawn_candidates.insert(row.candidate);
                }
            }
        }
        if let Some(exact_revisions) = request.exact_revisions() {
            for (candidate_id, requested_revision) in exact_revisions {
                if withdrawn_candidates.contains(candidate_id) {
                    continue;
                }
                let already_loaded = sources
                    .iter()
                    .any(|s| s.candidate_id == candidate_id && s.revision == requested_revision);
                if already_loaded {
                    continue;
                }
                let history_rows = source::Entity::find()
                    .filter(source::Column::Workspace.eq(workspace))
                    .filter(source::Column::Candidate.eq(candidate_id))
                    .filter(source::Column::ObservedAtVersion.lte(version as i64))
                    .filter(source::Column::Revision.is_not_null())
                    .order_by_desc(source::Column::ObservedAtVersion)
                    .all(&tx)
                    .await?;
                for row in history_rows {
                    if let Some(historical) = decode(&row)?
                        && historical.revision == requested_revision
                    {
                        sources.push(historical);
                        break;
                    }
                }
            }
        }
        tx.commit().await?;
        Ok(Observation { version, sources })
    }

    /// Reads a patient record by source, version, and location under transactional [`super::Authority`].
    ///
    /// When `authority` denies access, returns [`crate::physician_sources::read::PatientRecordRead::Denied`]
    /// without reading or creating any workspace source rows.
    pub async fn read_patient_record(
        &self,
        scope: &super::Scope,
        session_hash: Option<&[u8]>,
        authority: &mut dyn super::Authority,
        request: &crate::physician_sources::read::RecordReadRequest,
        part_reader: &dyn crate::physician_sources::read::RecordPartReader,
    ) -> Result<
        crate::physician_sources::read::PatientRecordRead,
        crate::physician_sources::read::RecordReadError,
    > {
        use crate::physician_sources::read::{
            ObservationEntry, PatientAccess, PatientRecordRead, RecordReadError,
            VersionedRecordCatalog, read_patient_record,
        };
        if [
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|field| field.trim().is_empty())
        {
            return Ok(PatientRecordRead::Denied);
        }
        let tx = self.db.begin().await?;
        if authority.check(&tx, scope, session_hash).await.is_err() {
            tx.rollback().await?;
            return Ok(PatientRecordRead::Denied);
        }
        let version = pin_version(&tx, &scope.workspace).await?;
        let rows = source::Entity::find()
            .filter(source::Column::Workspace.eq(scope.workspace.as_str()))
            .filter(source::Column::Candidate.eq(request.candidate_id.as_str()))
            .filter(source::Column::ObservedAtVersion.lte(version as i64))
            .order_by_asc(source::Column::ObservedAtVersion)
            .all(&tx)
            .await?;
        let mut entries = Vec::with_capacity(rows.len());
        let mut last_source_id: Option<String> = None;
        for row in rows {
            let observed_at = version_of(row.observed_at_version)?;
            match row.revision {
                Some(value) => {
                    let revision: SourceRevision =
                        serde_json::from_value(value).map_err(|source| {
                            RecordReadError::CorruptStoredObservation {
                                candidate_id: row.candidate.clone(),
                                observed_at_version: observed_at,
                                source,
                            }
                        })?;
                    last_source_id = Some(revision.source_id.clone());
                    entries.push(ObservationEntry::Present {
                        observed_at_version: observed_at,
                        revision,
                    });
                }
                None => {
                    entries.push(ObservationEntry::Withdrawn {
                        observed_at_version: observed_at,
                        candidate_id: row.candidate,
                        workspace: row.workspace,
                        source_id: last_source_id.clone(),
                    });
                }
            }
        }
        tx.commit().await?;
        let catalog = VersionedRecordCatalog::from_observations(version, entries);
        let access = PatientAccess::Allowed {
            workspace: scope.workspace.clone(),
        };
        read_patient_record(&access, request, &catalog, part_reader)
    }
}
