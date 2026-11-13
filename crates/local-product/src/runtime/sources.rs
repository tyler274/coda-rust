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

/// Reconstructs the workspace's retained observation as of an earlier or current
/// `version` inside an already-fenced transaction.
pub(super) async fn observed_at(
    tx: &impl ConnectionTrait,
    workspace: &str,
    version: u64,
) -> Result<Observation, DbErr> {
    let withheld = super::link_correction::withheld_candidate_ids_in_tx(tx, workspace).await?;
    let mut sources = Vec::new();
    for row in latest(tx, workspace, version, None).await? {
        if withheld.contains(&row.candidate) {
            continue;
        }
        if let Some(source) = decode(&row)? {
            sources.push(source);
        }
    }
    Ok(Observation { version, sources })
}

/// Pins the current source version for every reader that must serialize with ingestion.
pub(super) async fn pin_version(tx: &impl ConnectionTrait, workspace: &str) -> Result<u64, DbErr> {
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
    let withheld = super::link_correction::withheld_candidate_ids_in_tx(tx, workspace).await?;
    let mut sources = Vec::new();
    for row in latest(tx, workspace, version, candidates).await? {
        if withheld.contains(&row.candidate) {
            continue;
        }
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
        validate_delivery_records(workspace, &pull.source_id, records)?;
        let tx = self.db.begin().await?;
        let (_, changed) =
            record_delivery_in_tx(&tx, workspace, DeliveryScope::WholeSource, records, pull)
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
            let workspace = row.workspace.clone();
            pulls
                .entry(workspace)
                .or_default()
                .push(decode_source_pull(row)?);
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
    /// the requested candidates, including their current withdrawal state.
    pub async fn chart_observation(
        &self,
        workspace: &str,
        request: &crate::physician_sources::ChartRequest,
    ) -> Result<Observation, DbErr> {
        let tx = self.db.begin().await?;
        let observation = selected(&tx, workspace, request.candidates().as_deref()).await?;
        tx.commit().await?;
        Ok(observation)
    }
}

/// How an incoming source delivery treats records omitted from `records`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DeliveryScope {
    /// Whole-source chart snapshot: withdraws current candidates held by
    /// `pull.source_id` that are absent from `records`.
    WholeSource,
    /// Incremental or partial refresh: appends new and changed revisions for
    /// `pull.source_id` without withdrawing unmentioned candidates.
    ChangedRecords,
}

pub(super) fn validate_delivery_records(
    workspace: &str,
    source_id: &str,
    records: &[SourceRevision],
) -> Result<(), DbErr> {
    for record in records {
        record.validate().map_err(|e| error(&e))?;
        if record.workspace != workspace || record.source_id != source_id {
            return Err(error("record outside its delivery"));
        }
    }
    Ok(())
}

fn decode_source_pull(row: source_pull::Model) -> Result<SourcePull, DbErr> {
    Ok(SourcePull {
        source_id: row.source_id,
        source_label: row.source_label,
        patient: serde_json::from_value(row.patient)
            .map_err(|_| error("corrupt source patient"))?,
        pulled_at: row.pulled_at,
    })
}

/// Reads the workspace's current pinned source version under a shared lock.
pub(super) async fn current_version_in_tx(
    tx: &impl ConnectionTrait,
    workspace: &str,
) -> Result<u64, DbErr> {
    pin_version(tx, workspace).await
}

/// Reads the workspace's source version inside a read-only snapshot without
/// inserting a missing row or acquiring row locks.
pub(super) async fn snapshot_version_in_tx(
    tx: &impl ConnectionTrait,
    workspace: &str,
) -> Result<u64, DbErr> {
    match source_version::Entity::find_by_id(workspace.to_owned())
        .one(tx)
        .await?
    {
        Some(row) => version_of(row.version),
        None => Ok(0),
    }
}

/// Reads one source's last good pull for `workspace` inside `tx`, if any.
pub(super) async fn last_source_pull_in_tx(
    tx: &impl ConnectionTrait,
    workspace: &str,
    source_id: &str,
) -> Result<Option<SourcePull>, DbErr> {
    source_pull::Entity::find_by_id((workspace.to_owned(), source_id.to_owned()))
        .one(tx)
        .await?
        .map(decode_source_pull)
        .transpose()
}

/// Records a source delivery inside `tx` under the workspace version row's
/// exclusive lock and updates `physician_runtime_source_pulls`. Returns
/// `(workspace_version_after, changed_records)`.
pub(super) async fn record_delivery_in_tx(
    tx: &impl ConnectionTrait,
    workspace: &str,
    scope: DeliveryScope,
    records: &[SourceRevision],
    pull: &SourcePull,
) -> Result<(u64, usize), DbErr> {
    validate_delivery_records(workspace, &pull.source_id, records)?;
    ensure_version(tx, workspace).await?;
    let row = source_version::Entity::find_by_id(workspace.to_owned())
        .lock_exclusive()
        .one(tx)
        .await?
        .ok_or_else(|| error("missing source version"))?;
    let version = version_of(row.version)?;
    let next_u64 = version
        .checked_add(1)
        .ok_or_else(|| error("source version overflow"))?;
    let next = i64::try_from(next_u64).map_err(conversion_error("source version", "i64"))?;
    let history = source::Entity::find()
        .filter(source::Column::Workspace.eq(workspace))
        .order_by_desc(source::Column::ObservedAtVersion)
        .all(tx)
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
    if scope == DeliveryScope::WholeSource {
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
    }
    let changed = rows.len();
    for batch in rows.chunks(CHART_BATCH) {
        source::Entity::insert_many(batch.to_vec())
            .exec_without_returning(tx)
            .await?;
    }
    let version_after = if changed > 0 {
        let mut bumped: source_version::ActiveModel = row.into();
        bumped.version = Set(next);
        bumped.update(tx).await?;
        next_u64
    } else {
        version
    };
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
    .exec_without_returning(tx)
    .await?;
    Ok((version_after, changed))
}
