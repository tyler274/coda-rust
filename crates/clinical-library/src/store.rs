//! The library's records in Postgres.
//!
//! Every write a run makes takes its [`Lease`], and the store refuses it, in
//! the same transaction, once another run holds the slot. A person's
//! resolutions are the only writes made without a lease.

use std::time::Duration;

use sea_orm::sea_query::{Expr, Func, OnConflict, SimpleExpr};
use sea_orm::{
    AccessMode, ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection,
    DatabaseTransaction, DbErr, EntityTrait, IsolationLevel, QueryFilter, QueryOrder, QuerySelect,
    Set, TransactionTrait,
};

use crate::article::{ArticleEvent, ArticleState, InvalidMove, Retry, StateKind, WaitReason};
use crate::candidate::{Activation, Candidate, Checked, InvalidVersionId, VersionId};
use crate::failure::{FailureCode, InvalidFailureCode};
use crate::migration::{
    self, item, night, pubmed_relation, pubmed_status, resolution, slot, version,
};
use crate::night::{DeferralReason, NightOutcome};
use crate::pubmed::refresh::{self, CheckResult, StatusCheck, StatusWrite};
use crate::pubmed::{Pmid, Relation, StatusRecord, StoredStatus};
use crate::slot::{HolderState, Lease, RunId, SlotName, Takeover, takeover};
use crate::source::ItemKey;
use crate::stop_rule::{self, HistoryEntry, Next};

/// The library's Postgres records.
#[derive(Debug, Clone)]
pub struct Store {
    db: DatabaseConnection,
}

/// What came of asking for a slot.
#[derive(Debug)]
pub enum Acquired {
    /// The run holds the slot.
    Holds(Lease),
    /// The run must wait. Record it with [`Store::record_wait`].
    Waits(Waiting),
}

/// A run that found the slot held and must wait. It can only record that it
/// waited, and why.
#[derive(Debug, PartialEq, Eq)]
pub struct Waiting {
    slot: SlotName,
    run: RunId,
    reason: DeferralReason,
}

impl Waiting {
    /// Why the run waits.
    pub const fn reason(&self) -> DeferralReason {
        self.reason
    }
}

/// Something the store couldn't do.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// The database refused or failed the operation.
    #[error("the clinical library database failed")]
    Database(#[from] DbErr),
    /// Another run took the slot, so this run's write was refused.
    #[error("run {run} no longer holds slot {slot}")]
    LeaseLost {
        /// The slot.
        slot: SlotName,
        /// The run that lost it.
        run: RunId,
    },
    /// NLM's record has more links than the store keeps.
    #[error("PMID {pmid} has more than {} links", i16::MAX)]
    TooManyLinks {
        /// The article.
        pmid: Pmid,
        /// The position that didn't fit.
        #[source]
        cause: std::num::TryFromIntError,
    },
    /// No source has qualified the item, so it takes no moves.
    #[error("{0} isn't qualified for the library")]
    Unqualified(ItemKey),
    /// The article's rules refused the move.
    #[error("the article's state refused the move")]
    InvalidMove(#[from] InvalidMove),
    /// A stored row doesn't match any state the types allow.
    #[error("a stored {table} row is invalid")]
    Corrupt {
        /// The table.
        table: &'static str,
        /// What was wrong with it.
        #[source]
        cause: Corruption,
    },
}

/// What was wrong with a stored row.
#[derive(Debug, thiserror::Error)]
pub enum Corruption {
    /// A stored version name isn't one.
    #[error("stored version name")]
    Version(#[from] InvalidVersionId),
    /// A stored failure code isn't one.
    #[error("stored failure code")]
    FailureCode(#[from] InvalidFailureCode),
    /// A stored failure count is out of range.
    #[error("stored failure count")]
    FailureCount(#[from] std::num::TryFromIntError),
    /// A stored name matches nothing the types define.
    #[error("unknown stored {field} {value:?}")]
    UnknownName {
        /// The column.
        field: &'static str,
        /// The stored value.
        value: String,
    },
    /// The row's columns describe no state a move could produce.
    #[error("the row's columns disagree: {0}")]
    Inconsistent(&'static str),
    /// A stored PMID isn't one.
    #[error("stored PMID")]
    Pmid(#[from] crate::pubmed::InvalidPmid),
    /// A stored hash isn't 32 bytes.
    #[error("stored record hash of {0} bytes")]
    Hash(usize),
    /// A row vanished while the store held its lock.
    #[error("the locked row vanished")]
    Vanished,
}

impl Store {
    /// Opens a small pool without changing the schema. Run
    /// `clinical-library-migrate` first.
    pub async fn connect(database_url: &str) -> Result<Self, DbErr> {
        let mut options = ConnectOptions::new(database_url.to_owned());
        options
            .max_connections(4)
            .connect_timeout(Duration::from_secs(10));
        Ok(Self::from_connection(Database::connect(options).await?))
    }

    /// Uses an existing pool.
    pub fn from_connection(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Applies every pending migration.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        migration::migrate(&self.db).await
    }

    /// The run holding `slot`, if any. Ask the platform for its state before
    /// calling [`Store::acquire`].
    pub async fn holder(&self, slot: &SlotName) -> Result<Option<RunId>, StoreError> {
        let row = slot::Entity::find_by_id(slot.as_str().to_owned())
            .one(&self.db)
            .await?;
        Ok(row.and_then(|row| row.holder).map(RunId))
    }

    /// Takes `slot` for `run`, or says why the run must wait.
    ///
    /// `holder_state` is what the platform said about the run that
    /// [`Store::holder`] returned. If the holder changed since, the run waits
    /// as though the new holder were alive. An ended holder that never
    /// recorded an outcome is recorded as failed with
    /// [`FailureCode::ENDED_WITHOUT_OUTCOME`] in the same transaction.
    pub async fn acquire(
        &self,
        slot: &SlotName,
        run: RunId,
        holder_state: Option<(RunId, HolderState)>,
    ) -> Result<Acquired, StoreError> {
        let tx = self.db.begin().await?;
        slot::Entity::insert(slot::ActiveModel {
            slot: Set(slot.as_str().to_owned()),
            holder: Set(None),
            token: Set(0),
        })
        .on_conflict_do_nothing_on([slot::Column::Slot])
        .exec(&tx)
        .await?;
        let row = slot::Entity::find_by_id(slot.as_str().to_owned())
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or(StoreError::Corrupt {
                table: "clinical_library_slots",
                cause: Corruption::Vanished,
            })?;

        if let Some(holder) = row.holder.map(RunId) {
            let decision = match holder_state {
                Some((checked, state)) if checked == holder => takeover(state),
                // The holder changed since the caller asked about it.
                _ => Takeover::Wait(DeferralReason::SlotHeldByLiveRun),
            };
            if let Takeover::Wait(reason) = decision {
                tx.commit().await?;
                return Ok(Acquired::Waits(Waiting {
                    slot: slot.clone(),
                    run,
                    reason,
                }));
            }
            record_takeover(&tx, slot, holder).await?;
        }

        let token = row.token + 1;
        let mut active: slot::ActiveModel = row.into();
        active.holder = Set(Some(run.0));
        active.token = Set(token);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(Acquired::Holds(Lease {
            slot: slot.clone(),
            run,
            token,
        }))
    }

    /// Records that a run waited, and why.
    pub async fn record_wait(&self, waiting: Waiting) -> Result<(), StoreError> {
        insert_night(
            &self.db,
            &waiting.slot,
            waiting.run,
            &NightOutcome::deferred(waiting.reason),
        )
        .await
    }

    /// Records the night's one outcome and gives up the slot.
    pub async fn finish(&self, lease: Lease, outcome: &NightOutcome) -> Result<(), StoreError> {
        let tx = self.db.begin().await?;
        let row = locked_slot(&tx, &lease).await?;
        insert_night(&tx, &lease.slot, lease.run, outcome).await?;
        let mut active: slot::ActiveModel = row.into();
        active.holder = Set(None);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }

    /// One item's state, or `None` if no source has qualified it.
    pub async fn item(&self, key: &ItemKey) -> Result<Option<ArticleState>, StoreError> {
        let row = item::Entity::find_by_id(item_id(key)).one(&self.db).await?;
        row.map(state_from_row).transpose()
    }

    /// Qualifies an item for the library under `lease`: a new item starts
    /// waiting to retry, so the next night tries it. Returns the item's
    /// state, which for one already qualified is unchanged.
    ///
    /// Only a qualified item takes moves; [`Store::apply`] refuses any
    /// other, so which items are in the library is always a source's
    /// deliberate choice.
    pub async fn qualify(&self, lease: &Lease, key: &ItemKey) -> Result<ArticleState, StoreError> {
        let tx = self.db.begin().await?;
        locked_slot(&tx, lease).await?;
        item::Entity::insert(row_from_state(key, &ArticleState::new_article()))
            .on_conflict_do_nothing_on([item::Column::Source, item::Column::ItemKey])
            .exec(&tx)
            .await?;
        let row = item::Entity::find_by_id(item_id(key))
            .one(&tx)
            .await?
            .ok_or(StoreError::Corrupt {
                table: "clinical_library_items",
                cause: Corruption::Vanished,
            })?;
        let state = state_from_row(row)?;
        tx.commit().await?;
        Ok(state)
    }

    /// Applies `event` to one qualified item.
    pub async fn apply(
        &self,
        lease: &Lease,
        key: &ItemKey,
        event: &ArticleEvent,
    ) -> Result<ArticleState, StoreError> {
        let tx = self.db.begin().await?;
        locked_slot(&tx, lease).await?;
        let from = qualified(&tx, key).await?;
        let next = from.apply(event)?;
        row_from_state(key, &next).update(&tx).await?;
        tx.commit().await?;
        Ok(next)
    }

    /// Records one night's status check for one qualified PubMed article,
    /// under `lease`, and says how it ended. `None` when the article isn't
    /// one tonight checks, such as one a person is handling.
    ///
    /// The article's moves, its stored status and the slot check happen in
    /// one transaction, so a run that lost its slot changes nothing.
    pub async fn record_status_check(
        &self,
        lease: &Lease,
        check: &StatusCheck,
    ) -> Result<Option<CheckResult>, StoreError> {
        let pmid = check.pmid();
        let key = ItemKey::pubmed(pmid);
        let tx = self.db.begin().await?;
        locked_slot(&tx, lease).await?;
        let state = qualified(&tx, &key).await?;
        let stored = pubmed_status::Entity::find_by_id(pmid.as_stored())
            .lock_exclusive()
            .one(&tx)
            .await?;
        let stored_hash = stored
            .as_ref()
            .map(|row| stored_hash(&row.record_sha256))
            .transpose()?;
        let Some(plan) = refresh::plan(&state, stored_hash, check) else {
            tx.commit().await?;
            return Ok(None);
        };

        let next = plan
            .events
            .iter()
            .try_fold(state.clone(), |state, event| state.apply(event))?;
        if next != state {
            row_from_state(&key, &next).update(&tx).await?;
        }
        match (plan.status, check) {
            (StatusWrite::Replace, StatusCheck::Found(record)) => {
                replace_status(&tx, record, stored.is_some()).await?;
            }
            (StatusWrite::Confirm, _) => {
                pubmed_status::Entity::update_many()
                    .col_expr(pubmed_status::Column::ConfirmedAt, now())
                    .filter(pubmed_status::Column::Pmid.eq(pmid.as_stored()))
                    .exec(&tx)
                    .await?;
            }
            (StatusWrite::Replace | StatusWrite::Keep, _) => {}
        }
        tx.commit().await?;
        Ok(Some(plan.checked))
    }

    /// The status NLM last sent for `pmid`, or `None` if the library holds
    /// none. No status is not a statement that the article wasn't retracted.
    ///
    /// The status and its links are read in one repeatable-read
    /// transaction, so a replacement committing meanwhile can't mix two
    /// records.
    pub async fn pubmed_status(&self, pmid: Pmid) -> Result<Option<StoredStatus>, StoreError> {
        let tx = self
            .db
            .begin_with_config(
                Some(IsolationLevel::RepeatableRead),
                Some(AccessMode::ReadOnly),
            )
            .await?;
        let Some(row) = pubmed_status::Entity::find_by_id(pmid.as_stored())
            .one(&tx)
            .await?
        else {
            tx.commit().await?;
            return Ok(None);
        };
        let links = pubmed_relation::Entity::find()
            .filter(pubmed_relation::Column::Pmid.eq(pmid.as_stored()))
            .order_by_asc(pubmed_relation::Column::Position)
            .all(&tx)
            .await?;
        tx.commit().await?;
        let relations = links
            .into_iter()
            .map(|link| {
                let target = link
                    .target_pmid
                    .map(|value| {
                        u64::try_from(value)
                            .map_err(Corruption::from)
                            .and_then(|value| Pmid::new(value).map_err(Corruption::from))
                    })
                    .transpose()
                    .map_err(|cause| fault("clinical_library_pubmed_relations", cause))?;
                Ok(Relation {
                    ref_type: link.ref_type,
                    target,
                    source_reference: link.source_reference,
                    note: link.note,
                })
            })
            .collect::<Result<_, StoreError>>()?;
        let record_sha256 = stored_hash(&row.record_sha256)?;
        Ok(Some(StoredStatus {
            record: StatusRecord {
                pmid,
                publication_types: row.publication_types,
                relations,
                revised: row.revised,
                record_sha256,
                record_xml: row.record_xml,
            },
            observed_at: row.observed_at.to_utc(),
            confirmed_at: row.confirmed_at.to_utc(),
        }))
    }

    /// Makes `candidate` the one active version, in one switch.
    ///
    /// ```no_run
    /// # async fn example(store: &clinical_library::Store, lease: &clinical_library::Lease)
    /// # -> Result<(), Box<dyn std::error::Error>> {
    /// use clinical_library::{Candidate, VersionId};
    /// let built = Candidate::built(VersionId::new("v1")?);
    /// let checked = built.check(|_| Ok::<(), std::io::Error>(()))?;
    /// store.activate(lease, checked).await?;
    /// # Ok(()) }
    /// ```
    pub async fn activate(
        &self,
        lease: &Lease,
        candidate: Candidate<Checked>,
    ) -> Result<Activation, StoreError> {
        let tx = self.db.begin().await?;
        locked_slot(&tx, lease).await?;
        version::Entity::update_many()
            .col_expr(version::Column::Active, false.into())
            .filter(version::Column::Active.eq(true))
            .exec(&tx)
            .await?;
        let row = version::ActiveModel {
            version: Set(candidate.version().as_str().to_owned()),
            checked: Set(true),
            active: Set(true),
            activated_by_run: Set(Some(lease.run.0)),
        };
        version::Entity::insert(row)
            .on_conflict(
                OnConflict::column(version::Column::Version)
                    .update_columns([
                        version::Column::Checked,
                        version::Column::Active,
                        version::Column::ActivatedByRun,
                    ])
                    .to_owned(),
            )
            .exec(&tx)
            .await?;
        tx.commit().await?;
        Ok(Activation {
            version: candidate.version().clone(),
        })
    }

    /// The version search serves, if any.
    pub async fn active_version(&self) -> Result<Option<VersionId>, StoreError> {
        let row = version::Entity::find()
            .filter(version::Column::Active.eq(true))
            .one(&self.db)
            .await?;
        row.map(|row| {
            VersionId::new(row.version).map_err(|cause| fault("clinical_library_versions", cause))
        })
        .transpose()
    }

    /// Records that a person resolved what stopped the nightly. The stop
    /// rule's count starts over from here.
    pub async fn record_resolution(
        &self,
        slot: &SlotName,
        recorded_by: &str,
        reason: &str,
    ) -> Result<(), StoreError> {
        resolution::ActiveModel {
            slot: Set(slot.as_str().to_owned()),
            recorded_by: Set(recorded_by.to_owned()),
            reason: Set(reason.to_owned()),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;
        Ok(())
    }

    /// The slot's nights and resolutions, oldest first.
    pub async fn history(&self, slot: &SlotName) -> Result<Vec<HistoryEntry>, StoreError> {
        let nights = night::Entity::find()
            .filter(night::Column::Slot.eq(slot.as_str()))
            .order_by_asc(night::Column::Id)
            .all(&self.db)
            .await?;
        let resolutions = resolution::Entity::find()
            .filter(resolution::Column::Slot.eq(slot.as_str()))
            .order_by_asc(resolution::Column::Id)
            .all(&self.db)
            .await?;
        let mut entries = Vec::with_capacity(nights.len() + resolutions.len());
        for row in nights {
            entries.push((row.id, HistoryEntry::Night(outcome_from_row(row)?)));
        }
        for row in resolutions {
            entries.push((row.id, HistoryEntry::Resolved));
        }
        entries.sort_by_key(|(id, _)| *id);
        Ok(entries.into_iter().map(|(_, entry)| entry).collect())
    }

    /// Whether the next night may run, by the stop rule.
    pub async fn next(&self, slot: &SlotName) -> Result<Next, StoreError> {
        Ok(stop_rule::decide(&self.history(slot).await?))
    }
}

/// Locks the slot's row and checks the lease still holds it.
async fn locked_slot(tx: &DatabaseTransaction, lease: &Lease) -> Result<slot::Model, StoreError> {
    let row = slot::Entity::find_by_id(lease.slot.as_str().to_owned())
        .lock_exclusive()
        .one(tx)
        .await?;
    match row {
        Some(row) if row.token == lease.token && row.holder == Some(lease.run.0) => Ok(row),
        _ => Err(StoreError::LeaseLost {
            slot: lease.slot.clone(),
            run: lease.run,
        }),
    }
}

/// The store's key for an item's row.
fn item_id(key: &ItemKey) -> (String, String) {
    (key.source().as_str().to_owned(), key.as_str().to_owned())
}

/// Locks a qualified item's row and reads its state, or refuses an item no
/// source qualified.
async fn qualified(tx: &DatabaseTransaction, key: &ItemKey) -> Result<ArticleState, StoreError> {
    let row = item::Entity::find_by_id(item_id(key))
        .lock_exclusive()
        .one(tx)
        .await?
        .ok_or_else(|| StoreError::Unqualified(key.clone()))?;
    state_from_row(row)
}

/// The database's clock now, as Postgres's `clock_timestamp()`.
fn now() -> SimpleExpr {
    Func::cust("clock_timestamp").into()
}

/// Replaces an article's stored status with `record`, observed and
/// confirmed now.
async fn replace_status(
    tx: &DatabaseTransaction,
    record: &StatusRecord,
    exists: bool,
) -> Result<(), StoreError> {
    let pmid = record.pmid.as_stored();
    if exists {
        pubmed_relation::Entity::delete_many()
            .filter(pubmed_relation::Column::Pmid.eq(pmid))
            .exec(tx)
            .await?;
        pubmed_status::Entity::update_many()
            .col_expr(
                pubmed_status::Column::PublicationTypes,
                Expr::value(record.publication_types.clone()),
            )
            .col_expr(pubmed_status::Column::Revised, Expr::value(record.revised))
            .col_expr(
                pubmed_status::Column::RecordSha256,
                Expr::value(record.record_sha256.to_vec()),
            )
            .col_expr(
                pubmed_status::Column::RecordXml,
                Expr::value(record.record_xml.clone()),
            )
            .col_expr(pubmed_status::Column::ObservedAt, now())
            .col_expr(pubmed_status::Column::ConfirmedAt, now())
            .filter(pubmed_status::Column::Pmid.eq(pmid))
            .exec(tx)
            .await?;
    } else {
        pubmed_status::ActiveModel {
            pmid: Set(pmid),
            publication_types: Set(record.publication_types.clone()),
            revised: Set(record.revised),
            record_sha256: Set(record.record_sha256.to_vec()),
            record_xml: Set(record.record_xml.clone()),
            ..Default::default()
        }
        .insert(tx)
        .await?;
    }
    for (position, link) in record.relations.iter().enumerate() {
        let position = i16::try_from(position).map_err(|cause| StoreError::TooManyLinks {
            pmid: record.pmid,
            cause,
        })?;
        pubmed_relation::ActiveModel {
            pmid: Set(pmid),
            position: Set(position),
            ref_type: Set(link.ref_type.clone()),
            target_pmid: Set(link.target.map(Pmid::as_stored)),
            source_reference: Set(link.source_reference.clone()),
            note: Set(link.note.clone()),
        }
        .insert(tx)
        .await?;
    }
    Ok(())
}

fn stored_hash(bytes: &[u8]) -> Result<[u8; 32], StoreError> {
    <[u8; 32]>::try_from(bytes).map_err(|_wrong_length| {
        fault(
            "clinical_library_pubmed_status",
            Corruption::Hash(bytes.len()),
        )
    })
}

/// Records an ended holder's night as failed if it never recorded one.
async fn record_takeover(
    tx: &DatabaseTransaction,
    slot: &SlotName,
    holder: RunId,
) -> Result<(), StoreError> {
    let recorded = night::Entity::find()
        .filter(night::Column::Slot.eq(slot.as_str()))
        .filter(night::Column::RunId.eq(holder.0))
        .one(tx)
        .await?;
    if recorded.is_none() {
        let ended = NightOutcome::failed(FailureCode::ended_without_outcome());
        insert_night(tx, slot, holder, &ended).await?;
    }
    Ok(())
}

async fn insert_night(
    db: &impl sea_orm::ConnectionTrait,
    slot: &SlotName,
    run: RunId,
    outcome: &NightOutcome,
) -> Result<(), StoreError> {
    let (version, reason, code) = match outcome {
        NightOutcome::Succeeded(succeeded) => (
            succeeded.activated().map(|v| v.as_str().to_owned()),
            None,
            None,
        ),
        NightOutcome::Deferred(deferred) => (None, Some(deferred.reason().as_str()), None),
        NightOutcome::Failed(failed) => (None, None, Some(failed.code().as_str().to_owned())),
    };
    night::ActiveModel {
        slot: Set(slot.as_str().to_owned()),
        run_id: Set(run.0),
        outcome: Set(outcome.kind().as_str().to_owned()),
        activated_version: Set(version),
        deferral_reason: Set(reason.map(str::to_owned)),
        failure_code: Set(code),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(())
}

/// Wraps a stored row's fault with its table, keeping the fault as the
/// cause.
fn fault(table: &'static str, cause: impl Into<Corruption>) -> StoreError {
    StoreError::Corrupt {
        table,
        cause: cause.into(),
    }
}

fn outcome_from_row(row: night::Model) -> Result<NightOutcome, StoreError> {
    const TABLE: &str = "clinical_library_nights";
    let version = row
        .activated_version
        .map(VersionId::new)
        .transpose()
        .map_err(|cause| fault(TABLE, cause))?;
    let reason = row
        .deferral_reason
        .map(|value| {
            DeferralReason::parse(&value).ok_or(Corruption::UnknownName {
                field: "deferral_reason",
                value,
            })
        })
        .transpose()
        .map_err(|cause| fault(TABLE, cause))?;
    let code = row
        .failure_code
        .map(FailureCode::new)
        .transpose()
        .map_err(|cause| fault(TABLE, cause))?;
    crate::night::from_stored(&row.outcome, version, reason, code)
        .ok_or(Corruption::Inconsistent("the outcome and its fields"))
        .map_err(|cause| fault(TABLE, cause))
}

fn state_from_row(row: item::Model) -> Result<ArticleState, StoreError> {
    const TABLE: &str = "clinical_library_items";
    let last_failure = row
        .last_failure
        .map(FailureCode::new)
        .transpose()
        .map_err(|cause| fault(TABLE, cause))?;
    let state = match row.state.as_str() {
        "admitted" => ArticleState::Admitted,
        "waiting_to_retry" => {
            let failures = u8::try_from(row.failures).map_err(|cause| fault(TABLE, cause))?;
            let retry = Retry::from_stored(failures, last_failure)
                .ok_or(Corruption::Inconsistent(
                    "the failure count and latest failure",
                ))
                .map_err(|cause| fault(TABLE, cause))?;
            ArticleState::WaitingToRetry(retry)
        }
        "waiting_for_change" => {
            let value = row.wait_reason.unwrap_or_default();
            let reason = WaitReason::parse(&value)
                .ok_or(Corruption::UnknownName {
                    field: "wait_reason",
                    value,
                })
                .map_err(|cause| fault(TABLE, cause))?;
            ArticleState::WaitingForChange { reason }
        }
        "needs_person" => ArticleState::NeedsPerson {
            last_failure: last_failure
                .ok_or(Corruption::Inconsistent("needs a person without a failure"))
                .map_err(|cause| fault(TABLE, cause))?,
        },
        _ => {
            return Err(fault(
                TABLE,
                Corruption::UnknownName {
                    field: "state",
                    value: row.state,
                },
            ));
        }
    };
    Ok(state)
}

fn row_from_state(key: &ItemKey, state: &ArticleState) -> item::ActiveModel {
    let (failures, wait_reason, last_failure) = match state {
        ArticleState::Admitted => (0, None, None),
        ArticleState::WaitingToRetry(retry) => (
            i16::from(retry.failures()),
            None,
            retry.last_failure().cloned(),
        ),
        ArticleState::WaitingForChange { reason } => (0, Some(reason.as_str()), None),
        ArticleState::NeedsPerson { last_failure } => (0, None, Some(last_failure.clone())),
    };
    let kind: StateKind = state.kind();
    item::ActiveModel {
        source: Set(key.source().as_str().to_owned()),
        item_key: Set(key.as_str().to_owned()),
        state: Set(kind.as_str().to_owned()),
        failures: Set(failures),
        wait_reason: Set(wait_reason.map(str::to_owned)),
        last_failure: Set(last_failure.map(|code| code.as_str().to_owned())),
    }
}
