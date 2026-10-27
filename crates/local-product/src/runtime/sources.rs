//! Versioned patient source observations: a stand-in source owner.
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
//! The retained snapshot file is still the only import path here; projection
//! prose and model output never become sources. This is not production
//! ingestion.
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};

use super::entities::{source, source_version};
use super::{Store, error};
use crate::physician_sources::SourceRevision;

/// A workspace's source observation at one version.
#[derive(Clone, Debug, PartialEq)]
pub struct Observation {
    /// The workspace's source version; 0 before anything was recorded.
    pub version: u64,
    /// The newest revision of each source that is not withdrawn, ordered by
    /// candidate ID.
    pub sources: Vec<SourceRevision>,
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
