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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use sea_orm::sea_query::StringLen;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait,
    sea_query::{Expr, OnConflict, extension::postgres::PgExpr},
};
use serde::{Deserialize, Serialize};

use super::entities::{source, source_pull, source_restriction, source_version};
use super::{Store, conversion_error, error};
use crate::physician_sources::{SourcePatient, SourceRevision};

/// Why a candidate's content is restricted in a patient workspace.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sea_orm::EnumIter,
    sea_orm::DeriveActiveEnum,
)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SourceRestriction {
    /// Access to this source was removed.
    #[sea_orm(string_value = "access_removed")]
    AccessRemoved,
    /// The record was withheld from this patient's chart after a wrong-patient correction.
    #[sea_orm(string_value = "withheld_wrong_patient")]
    WithheldWrongPatient,
}

/// Whether a cited revision is still the newest active revision of its source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CitedSourceVersion {
    /// No newer active revision has been observed for this source.
    Current,
    /// A newer active revision of this source now exists in the workspace.
    NewerAvailable,
}

/// Current access and version state when opening a retained citation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CitedSourceState {
    /// The cited revision may be shown, along with whether a newer revision exists.
    Readable {
        /// Whether a newer active revision exists for this candidate.
        version: CitedSourceVersion,
    },
    /// The cited candidate's content may not be returned.
    Unavailable {
        /// Why the content is unavailable.
        reason: SourceRestriction,
    },
}

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
    let mut restriction_query = source_restriction::Entity::find()
        .filter(source_restriction::Column::Workspace.eq(workspace))
        .filter(source_restriction::Column::RecordedAtVersion.lte(version as i64));
    if let Some(ids) = candidates {
        restriction_query = restriction_query
            .filter(source_restriction::Column::Candidate.is_in(ids.iter().copied()));
    }
    let restricted: HashSet<String> = restriction_query
        .all(tx)
        .await?
        .into_iter()
        .map(|row| row.candidate)
        .collect();
    let mut sources = Vec::new();
    for row in latest(tx, workspace, version, candidates).await? {
        if restricted.contains(&row.candidate) {
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

    /// A workspace's current source version, without its sources; 0 before
    /// anything was recorded.
    pub async fn source_version(&self, workspace: &str) -> Result<u64, DbErr> {
        source_version::Entity::find_by_id(workspace.to_owned())
            .one(&self.db)
            .await?
            .map_or(Ok(0), |row| version_of(row.version))
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

    /// Loads retained source revisions matching exact `(candidate_id, revision)` identities.
    pub async fn sources_by_identities(
        &self,
        workspace: &str,
        identities: &BTreeSet<(String, String)>,
    ) -> Result<Vec<SourceRevision>, DbErr> {
        let tx = self.db.begin().await?;
        let found = by_identities(&tx, workspace, identities).await?;
        tx.commit().await?;
        Ok(found)
    }

    /// Marks `candidate` restricted in `workspace` (access removed or withheld
    /// after a wrong-patient correction) under the workspace's version lock.
    /// Excluded from active chart observations and refused on citation reads.
    /// Re-recording the same restriction is a no-op.
    pub async fn restrict_source(
        &self,
        workspace: &str,
        candidate: &str,
        reason: SourceRestriction,
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
        let existing =
            source_restriction::Entity::find_by_id((workspace.to_owned(), candidate.to_owned()))
                .one(&tx)
                .await?;
        if existing
            .as_ref()
            .is_some_and(|saved| saved.reason == reason)
        {
            tx.commit().await?;
            return Ok(version);
        }
        let next = version + 1;
        source_restriction::Entity::insert(source_restriction::ActiveModel {
            workspace: Set(workspace.into()),
            candidate: Set(candidate.into()),
            reason: Set(reason),
            recorded_at_version: Set(next as i64),
        })
        .on_conflict(
            OnConflict::columns([
                source_restriction::Column::Workspace,
                source_restriction::Column::Candidate,
            ])
            .update_columns([
                source_restriction::Column::Reason,
                source_restriction::Column::RecordedAtVersion,
            ])
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        let mut bumped: source_version::ActiveModel = row.into();
        bumped.version = Set(next as i64);
        bumped.update(&tx).await?;
        tx.commit().await?;
        Ok(next)
    }

    /// Every restricted candidate in `workspace`, keyed by candidate ID.
    pub async fn restricted_sources(
        &self,
        workspace: &str,
    ) -> Result<HashMap<String, SourceRestriction>, DbErr> {
        let rows = source_restriction::Entity::find()
            .filter(source_restriction::Column::Workspace.eq(workspace))
            .all(&self.db)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| (row.candidate, row.reason))
            .collect())
    }

    /// Current access and version status for a citation to `(candidate, cited_revision)`
    /// in `workspace`, fenced under the workspace's shared source-version lock.
    pub async fn cited_source_state(
        &self,
        workspace: &str,
        candidate: &str,
        cited_revision: &str,
    ) -> Result<CitedSourceState, DbErr> {
        let tx = self.db.begin().await?;
        let version = pin_version(&tx, workspace).await?;
        let restriction =
            source_restriction::Entity::find_by_id((workspace.to_owned(), candidate.to_owned()))
                .filter(source_restriction::Column::RecordedAtVersion.lte(version as i64))
                .one(&tx)
                .await?;
        if let Some(restriction) = restriction {
            tx.commit().await?;
            return Ok(CitedSourceState::Unavailable {
                reason: restriction.reason,
            });
        }
        let newest = latest(&tx, workspace, version, Some(&[candidate])).await?;
        let version_state = match newest.first() {
            Some(row) => match decode(row)? {
                Some(active) if active.revision != cited_revision => {
                    CitedSourceVersion::NewerAvailable
                }
                Some(_) | None => CitedSourceVersion::Current,
            },
            None => CitedSourceVersion::Current,
        };
        tx.commit().await?;
        Ok(CitedSourceState::Readable {
            version: version_state,
        })
    }
}

/// Historical source revisions matching exact `(candidate_id, revision)` pairs in `workspace`.
pub(super) async fn by_identities(
    tx: &impl ConnectionTrait,
    workspace: &str,
    identities: &BTreeSet<(String, String)>,
) -> Result<Vec<SourceRevision>, DbErr> {
    if identities.is_empty() {
        return Ok(Vec::new());
    }
    let candidates: BTreeSet<&str> = identities.iter().map(|(c, _)| c.as_str()).collect();
    let rows = source::Entity::find()
        .filter(source::Column::Workspace.eq(workspace))
        .filter(source::Column::Candidate.is_in(candidates))
        .filter(source::Column::Revision.is_not_null())
        .order_by_asc(source::Column::Candidate)
        .order_by_asc(source::Column::ObservedAtVersion)
        .all(tx)
        .await?;
    let mut seen = BTreeSet::new();
    let mut found = Vec::new();
    for row in rows {
        if let Some(source) = decode(&row)? {
            let key = (source.candidate_id.clone(), source.revision.clone());
            if identities.contains(&key) && seen.insert(key) {
                found.push(source);
            }
        }
    }
    Ok(found)
}
