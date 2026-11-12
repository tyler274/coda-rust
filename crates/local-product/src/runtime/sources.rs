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
use crate::physician_sources::{ChartPullReceipt, SourcePatient, SourceRecordKind, SourceRevision};

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
    /// The practice clock's Unix second when a complete pull last finished, or
    /// `None` when only partial records have landed.
    pub pulled_at: Option<i64>,
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

/// Which held candidates can be evaluated for absence during a source delivery.
pub(super) enum MissingScope<'a> {
    /// Complete chart pull: every active candidate for `source_id` is expected.
    FullChart,
    /// Partial pull: only active candidates whose resource kind or category succeeded in this pull are checked.
    SucceededKinds(&'a [SourceRecordKind]),
}

fn kind_matches_existing(kind_spec: &SourceRecordKind, existing: &SourceRevision) -> bool {
    if existing.record_type != kind_spec.kind {
        return false;
    }
    match &kind_spec.category {
        None => true,
        Some(expected_cat) => existing
            .envelope
            .as_ref()
            .and_then(|env| env.source_kind.category.as_deref())
            .is_some_and(|actual| actual == expected_cat),
    }
}

/// Single authoritative source-delivery writer shared by complete and partial pulls.
///
/// Under the workspace's exclusive `source_version` lock, validates identity consistency
/// across history, appends changed candidate observations in batches, bumps `source_version`
/// when observations changed, and retains previously active candidates absent from `records`
/// while reporting them in [`ChartPullReceipt::missing_candidates`].
pub(super) async fn apply_source_delivery_tx(
    tx: &impl ConnectionTrait,
    workspace: &str,
    source_id: &str,
    records: &[SourceRevision],
    missing_scope: MissingScope<'_>,
) -> Result<ChartPullReceipt, DbErr> {
    for record in records {
        record.validate().map_err(|e| error(&e))?;
        if record.workspace != workspace || record.source_id != source_id {
            return Err(error("record outside its delivery"));
        }
    }
    ensure_version(tx, workspace).await?;
    let row = source_version::Entity::find_by_id(workspace.to_owned())
        .lock_exclusive()
        .one(tx)
        .await?
        .ok_or_else(|| error("missing source version"))?;
    let version = version_of(row.version)?;
    let next = version as i64 + 1;
    let history = source::Entity::find()
        .filter(source::Column::Workspace.eq(workspace))
        .order_by_desc(source::Column::ObservedAtVersion)
        .all(tx)
        .await?;
    let mut seen: HashMap<(String, String), SourceRevision> = HashMap::new();
    let mut current: HashMap<String, Option<SourceRevision>> = HashMap::new();
    for entry in &history {
        let decoded = decode(entry)?;
        current
            .entry(entry.candidate.clone())
            .or_insert_with(|| decoded.clone());
        if let Some(decoded) = decoded {
            seen.insert((entry.candidate.clone(), decoded.revision.clone()), decoded);
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
        if holding.is_some_and(|existing| existing.revision == record.revision) {
            continue;
        }
        let value = serde_json::to_value(record)
            .map_err(conversion_error("SourceRevision", "source revision"))?;
        rows.push(source::ActiveModel {
            workspace: Set(workspace.into()),
            candidate: Set(record.candidate_id.clone()),
            observed_at_version: Set(next),
            revision: Set(Some(value)),
        });
    }
    let delivered: HashSet<&str> = records.iter().map(|r| r.candidate_id.as_str()).collect();
    let mut missing_candidates = Vec::new();
    for (candidate, newest) in &current {
        let Some(existing) = newest.as_ref() else {
            continue;
        };
        if existing.source_id != source_id || delivered.contains(candidate.as_str()) {
            continue;
        }
        let in_scope = match missing_scope {
            MissingScope::FullChart => true,
            MissingScope::SucceededKinds(kinds) => kinds
                .iter()
                .any(|kind| kind_matches_existing(kind, existing)),
        };
        if in_scope {
            missing_candidates.push(candidate.clone());
        }
    }
    missing_candidates.sort();
    let changed = rows.len();
    for batch in rows.chunks(CHART_BATCH) {
        source::Entity::insert_many(batch.to_vec())
            .exec_without_returning(tx)
            .await?;
    }
    if changed > 0 {
        let mut bumped: source_version::ActiveModel = row.into();
        bumped.version = Set(next);
        bumped.update(tx).await?;
    }
    Ok(ChartPullReceipt {
        changed,
        missing_candidates,
    })
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
    /// each record that differs from the current one and the pull metadata, so
    /// "records as of" never outruns the records. Records from `pull.source_id`
    /// that were previously active and absent from `records` stay saved and are
    /// recorded in [`ChartPullReceipt::missing_candidates`]. A delivery says
    /// what the source sent now, so an earlier revision delivered again becomes
    /// current, unlike a replay through [`Self::record_source`]. A revision
    /// identity still holds one set of bytes. One delivery is one version.
    /// Returns how many records changed.
    pub async fn record_chart(
        &self,
        workspace: &str,
        records: &[SourceRevision],
        pull: &SourcePull,
    ) -> Result<usize, DbErr> {
        self.record_chart_with_receipt(workspace, records, pull)
            .await
            .map(|receipt| receipt.changed)
    }

    /// Records one source's delivery for `workspace` in one transaction,
    /// returning both how many observations moved and which previously active
    /// candidate IDs from `pull.source_id` were absent on this pull.
    pub async fn record_chart_with_receipt(
        &self,
        workspace: &str,
        records: &[SourceRevision],
        pull: &SourcePull,
    ) -> Result<ChartPullReceipt, DbErr> {
        let Some(pulled_at) = pull.pulled_at else {
            return Err(error("complete chart delivery requires pulled_at"));
        };
        let tx = self.db.begin().await?;
        let receipt = apply_source_delivery_tx(
            &tx,
            workspace,
            &pull.source_id,
            records,
            MissingScope::FullChart,
        )
        .await?;
        let patient = serde_json::to_value(&pull.patient)
            .map_err(conversion_error("SourcePatient", "source pull patient"))?;
        let missing_json = serde_json::to_value(&receipt.missing_candidates).map_err(
            conversion_error("Vec<String>", "source pull missing candidates"),
        )?;
        source_pull::Entity::insert(source_pull::ActiveModel {
            workspace: Set(workspace.into()),
            source_id: Set(pull.source_id.clone()),
            source_label: Set(pull.source_label.clone()),
            patient: Set(patient),
            pulled_at: Set(Some(pulled_at)),
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
        tx.commit().await?;
        Ok(receipt)
    }

    /// Previously saved candidate IDs from `source_id` that were absent on its
    /// latest recorded pull for `workspace`.
    pub async fn pull_missing_candidates(
        &self,
        workspace: &str,
        source_id: &str,
    ) -> Result<Vec<String>, DbErr> {
        let Some(row) =
            source_pull::Entity::find_by_id((workspace.to_owned(), source_id.to_owned()))
                .one(&self.db)
                .await?
        else {
            return Ok(Vec::new());
        };
        serde_json::from_value(row.missing_candidates)
            .map_err(conversion_error("Json", "source pull missing candidates"))
    }

    /// Explicitly withdraws one candidate observation from `workspace` (such as
    /// after wrong-patient link correction or an explicit source withdrawal),
    /// appending a null-revision observation and advancing the workspace version.
    /// Withdrawing an absent or already-withdrawn candidate changes nothing.
    pub async fn withdraw_source(&self, workspace: &str, candidate: &str) -> Result<u64, DbErr> {
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
        let active = latest(&tx, workspace, version, Some(&[candidate]))
            .await?
            .into_iter()
            .next()
            .and_then(|entry| entry.revision)
            .is_some();
        if !active {
            tx.commit().await?;
            return Ok(version);
        }
        let next = version + 1;
        source::Entity::insert(source::ActiveModel {
            workspace: Set(workspace.into()),
            candidate: Set(candidate.into()),
            observed_at_version: Set(next as i64),
            revision: Set(None),
        })
        .exec_without_returning(&tx)
        .await?;
        let mut bumped: source_version::ActiveModel = row.into();
        bumped.version = Set(next as i64);
        bumped.update(&tx).await?;
        tx.commit().await?;
        Ok(next)
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
