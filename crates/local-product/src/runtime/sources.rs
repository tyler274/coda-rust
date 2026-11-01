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
    QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};

use super::entities::{source, source_pull, source_version};
use super::{Store, error};
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
) -> Result<Vec<source::Model>, DbErr> {
    let rows = source::Entity::find()
        .filter(source::Column::Workspace.eq(workspace))
        .filter(source::Column::ObservedAtVersion.lte(version as i64))
        .order_by_asc(source::Column::Candidate)
        .order_by_desc(source::Column::ObservedAtVersion)
        .all(tx)
        .await?;
    let mut newest: Vec<source::Model> = Vec::new();
    for row in rows {
        if newest
            .last()
            .is_none_or(|last| last.candidate != row.candidate)
        {
            newest.push(row);
        }
    }
    Ok(newest)
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
    ensure_version(tx, workspace).await?;
    let row = source_version::Entity::find_by_id(workspace.to_owned())
        .lock_shared()
        .one(tx)
        .await?
        .ok_or_else(|| error("missing source version"))?;
    let version = version_of(row.version)?;
    let mut sources = Vec::new();
    for row in latest(tx, workspace, version).await? {
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
    pub async fn record_source(&self, source: &SourceRevision) -> Result<u64, DbErr> {
        source.validate().map_err(|e| error(&e))?;
        let value = serde_json::to_value(source).map_err(|_| error("invalid source"))?;
        self.observe(&source.workspace, &source.candidate_id, Some(value))
            .await
    }

    /// Withdraws a source: a later observation without it. Withdrawing an
    /// unknown or already withdrawn source changes nothing. Returns the
    /// workspace version after.
    pub async fn withdraw_source(&self, workspace: &str, candidate: &str) -> Result<u64, DbErr> {
        self.observe(workspace, candidate, None).await
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

    async fn observe(
        &self,
        workspace: &str,
        candidate: &str,
        revision: Option<serde_json::Value>,
    ) -> Result<u64, DbErr> {
        if workspace.trim().is_empty() || candidate.trim().is_empty() {
            return Err(error("empty source identity"));
        }
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
        let incoming = revision
            .clone()
            .map(serde_json::from_value::<SourceRevision>)
            .transpose()
            .map_err(|_| error("invalid source"))?;
        let unchanged = if let Some(incoming) = &incoming {
            let mut seen = false;
            for row in &history {
                if let Some(recorded) = decode(row)?
                    && recorded.revision == incoming.revision
                {
                    if recorded != *incoming {
                        return Err(error("conflicting source revision identity"));
                    }
                    seen = true;
                }
            }
            seen
        } else {
            // Withdrawing what was never recorded or is already withdrawn
            // changes nothing.
            history.first().is_none_or(|row| row.revision.is_none())
        };
        if unchanged {
            tx.commit().await?;
            return Ok(version);
        }
        let next = version + 1;
        source::Entity::insert(source::ActiveModel {
            workspace: Set(workspace.into()),
            candidate: Set(candidate.into()),
            observed_at_version: Set(next as i64),
            revision: Set(revision),
        })
        .exec_without_returning(&tx)
        .await?;
        let mut bumped: source_version::ActiveModel = row.into();
        bumped.version = Set(next as i64);
        bumped.update(&tx).await?;
        tx.commit().await?;
        Ok(next)
    }
}
