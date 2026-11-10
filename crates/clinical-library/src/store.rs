//! The library's records in Postgres.
//!
//! Every write a run makes takes its [`Lease`], and the store refuses it, in
//! the same transaction, once another run holds the slot. A person's
//! resolutions are the only writes made without a lease.
//!
//! The store also keeps each night's ledger: the items due when the night
//! took the slot or that it qualified, and what it did with each. A night's
//! outcome is counted from that ledger in [`Store::finish`], never taken
//! from the runner.

use std::collections::BTreeSet;
use std::time::Duration;

use sea_orm::sea_query::{Expr, Func, OnConflict, Query, SimpleExpr};
use sea_orm::{
    AccessMode, ActiveModelTrait, ColumnTrait, ConnectOptions, ConnectionTrait, Database,
    DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, IsolationLevel, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait,
};

use crate::alert::{self, Alert};
use crate::article::{ArticleEvent, ArticleState, InvalidMove, Retry, StateKind, WaitReason};
use crate::candidate::{Activation, Candidate, Checked, InvalidVersionId, VersionId};
use crate::failure::{FailureCode, InvalidFailureCode};
use crate::migration::{
    self, item, night, night_item, pubmed_relation, pubmed_status, resolution, slot, unpublished,
    version,
};
use crate::night::{DeferralReason, NightOutcome, Tally};
use crate::pubmed::refresh::{self, CheckResult, StatusCheck, StatusWrite};
use crate::pubmed::{Pmid, Relation, StatusRecord, StoredStatus};
use crate::runner::{Ending, FailureDetail};
use crate::slot::{Lease, Platform, RunId, SlotName, SlotWait, Takeover, takeover};
use crate::source::{ItemKey, Source};
use crate::stop_rule::{self, HistoryEntry, Next, StopReason};

/// The library's Postgres records.
#[derive(Debug, Clone)]
pub struct Store {
    db: DatabaseConnection,
}

/// What came of asking for a slot. A night that doesn't get the slot has
/// already been recorded as deferred, with why.
#[derive(Debug)]
pub enum Acquired {
    /// The run holds the slot.
    Holds(Lease),
    /// Another run holds the slot, so this night waited.
    Waited(SlotWait),
    /// The stop rule says automatic attempts have stopped, so this night
    /// didn't run. A person must record a resolution first.
    Stopped(StopReason),
}

/// What a run did with one item it was due to do tonight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemResult {
    /// Due, and not yet admitted or set aside.
    Due,
    /// Admitted, or its status re-confirmed.
    Done,
    /// Set aside, with its reason in the item's state.
    SetAside,
}

impl ItemResult {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Due => "due",
            Self::Done => "done",
            Self::SetAside => "set_aside",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        [Self::Due, Self::Done, Self::SetAside]
            .into_iter()
            .find(|result| result.as_str() == value)
    }

    /// What tonight's `event` makes of an item, for the ledger.
    const fn of(event: &ArticleEvent) -> Self {
        match event {
            ArticleEvent::Admitted => Self::Done,
            ArticleEvent::OutsideFailure(_) | ArticleEvent::Refused(_) => Self::SetAside,
            ArticleEvent::SourceChanged
            | ArticleEvent::ReaderChanged
            | ArticleEvent::PersonResolved => Self::Due,
        }
    }
}

/// Something the store couldn't do.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// The database refused or failed the operation.
    #[error("the clinical library database failed")]
    Database(#[from] DbErr),
    /// A query couldn't be built.
    #[error("a clinical library query couldn't be built")]
    Query(#[from] sea_orm::sea_query::error::Error),
    /// Another run took the slot, so this run's write was refused.
    #[error("run {run} no longer holds slot {slot}")]
    LeaseLost {
        /// The slot.
        slot: SlotName,
        /// The run that lost it.
        run: RunId,
    },
    /// A night tried to finish with an activation another run made, or one
    /// whose version is no longer the active one it made.
    #[error("run {run} can't finish with version {version}'s activation, which isn't its own")]
    ForeignActivation {
        /// The run finishing.
        run: RunId,
        /// The version the activation names.
        version: VersionId,
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
    /// The item's source isn't one tonight's night answers for, so its
    /// result wouldn't count toward the night.
    #[error("{0} isn't from a source tonight's night answers for")]
    NotTonightsSource(ItemKey),
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
    /// A stored item key isn't one of its source's.
    #[error("stored item key")]
    ItemKey(#[from] crate::source::InvalidItemKey),
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

    /// The run holding `slot`, if any.
    pub async fn holder(&self, slot: &SlotName) -> Result<Option<RunId>, StoreError> {
        let row = slot::Entity::find_by_id(slot.as_str().to_owned())
            .one(&self.db)
            .await?;
        Ok(row.and_then(|row| row.holder).map(RunId))
    }

    /// Takes `slot` for `run`, whose night answers for the due items of
    /// `sources`, or records why tonight doesn't run.
    ///
    /// - **Held by another run:** the store asks `platform` about the holder.
    ///   Only an ended holder is taken over; one that never recorded an
    ///   outcome is recorded as failed with
    ///   [`FailureCode::ENDED_WITHOUT_OUTCOME`] in the same transaction. A
    ///   live or unreadable holder, or one that changed since the platform
    ///   was asked, means tonight waits.
    /// - **Stopped:** if the stop rule says automatic attempts have stopped,
    ///   tonight doesn't run, whether or not the runner asked
    ///   [`Store::next`].
    /// - **Taken:** every item of `sources` waiting to retry goes into
    ///   tonight's ledger as due, and the night must admit or set aside each
    ///   one to succeed.
    pub async fn acquire(
        &self,
        slot: &SlotName,
        sources: &[Source],
        run: RunId,
        platform: &dyn Platform,
    ) -> Result<Acquired, StoreError> {
        // Asked before the transaction, so the slot isn't locked while the
        // platform answers. The holder is compared again under the lock.
        let asked = match self.holder(slot).await? {
            Some(holder) => Some((holder, platform.holder_state(holder).await)),
            None => None,
        };
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

        let held = row.holder.map(RunId);
        if let Some(holder) = held {
            let decision = match asked {
                Some((checked, state)) if checked == holder => takeover(state),
                // The holder changed since the platform was asked about it.
                _ => Takeover::Wait(SlotWait::HolderAlive),
            };
            if let Takeover::Wait(wait) = decision {
                insert_night(&tx, slot, run, &NightOutcome::deferred(wait.into()), None).await?;
                tx.commit().await?;
                return Ok(Acquired::Waited(wait));
            }
            record_takeover(&tx, slot, holder).await?;
        }

        if let Next::Stop(why) = stop_rule::decide(&history(&tx, slot).await?) {
            let stopped = NightOutcome::deferred(DeferralReason::Stopped(why.clone()));
            insert_night(&tx, slot, run, &stopped, None).await?;
            if held.is_some() {
                // The ended holder's lease can no longer write.
                let mut active: slot::ActiveModel = row.into();
                active.holder = Set(None);
                active.update(&tx).await?;
            }
            tx.commit().await?;
            return Ok(Acquired::Stopped(why));
        }

        let token = row.token + 1;
        let mut active: slot::ActiveModel = row.into();
        active.holder = Set(Some(run.0));
        active.token = Set(token);
        active.update(&tx).await?;
        let sources: BTreeSet<Source> = sources.iter().copied().collect();
        snapshot_due(&tx, run, &sources).await?;
        tx.commit().await?;
        Ok(Acquired::Holds(Lease {
            slot: slot.clone(),
            run,
            token,
            sources,
        }))
    }

    /// Records the night's one outcome, counted from its ledger, and gives
    /// up the slot. Returns the outcome recorded.
    ///
    /// - [`Ending::Completed`]: the outcome is what the ledger earns (see
    ///   [`NightOutcome`]), with the night's own activation, if it made one.
    /// - [`Ending::NoCapacity`]: deferred, waiting for capacity.
    /// - [`Ending::Failed`]: failed, with the failure's code and words.
    pub async fn finish(&self, lease: Lease, ending: Ending) -> Result<NightOutcome, StoreError> {
        let tx = self.db.begin().await?;
        let row = locked_slot(&tx, &lease).await?;
        let (outcome, detail) = match ending {
            Ending::Completed { activation } => {
                let activated = match activation {
                    None => None,
                    Some(activation) => Some(own_activation(&tx, &lease, activation).await?),
                };
                let tally = tally(&tx, &lease).await?;
                let outcome = NightOutcome::from_tally(&tally, activated);
                // Say what the night left undone, since it said it finished.
                let detail = matches!(outcome, NightOutcome::Failed(_)).then(|| {
                    FailureDetail::new(format!(
                        "the night finished with {} due items neither admitted nor set aside{}",
                        tally.due_left(),
                        if tally.needs_version() {
                            ", or admitted content without activating a version"
                        } else {
                            ""
                        }
                    ))
                });
                (outcome, detail)
            }
            Ending::NoCapacity => (
                NightOutcome::deferred(DeferralReason::WaitingForCapacity),
                None,
            ),
            Ending::Failed { code, detail } => (NightOutcome::failed(code), Some(detail)),
        };
        insert_night(&tx, &lease.slot, lease.run, &outcome, detail.as_ref()).await?;
        let mut active: slot::ActiveModel = row.into();
        active.holder = Set(None);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// What tonight's ledger shows so far for the run holding `lease`.
    pub async fn tally(&self, lease: &Lease) -> Result<Tally, StoreError> {
        tally(&self.db, lease).await
    }

    /// The items of `source` still due tonight for the run holding `lease`:
    /// due when it took the slot, or qualified since, and not yet admitted
    /// or set aside.
    pub async fn due_tonight(
        &self,
        lease: &Lease,
        source: Source,
    ) -> Result<Vec<ItemKey>, StoreError> {
        let rows = night_item::Entity::find()
            .filter(night_item::Column::RunId.eq(lease.run.0))
            .filter(night_item::Column::Source.eq(source.as_str()))
            .filter(night_item::Column::Result.eq(ItemResult::Due.as_str()))
            .order_by_asc(night_item::Column::ItemKey)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(|row| {
                ItemKey::from_stored(source, &row.item_key)
                    .map_err(|cause| fault("clinical_library_night_items", cause))
            })
            .collect()
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
        if !lease.sources.contains(&key.source()) {
            return Err(StoreError::NotTonightsSource(key.clone()));
        }
        let inserted = item::Entity::insert(row_from_state(key, &ArticleState::new_article()))
            .on_conflict_do_nothing_on([item::Column::Source, item::Column::ItemKey])
            .exec(&tx)
            .await?;
        let new = matches!(inserted, sea_orm::TryInsertResult::Inserted(_));
        let row = item::Entity::find_by_id(item_id(key))
            .one(&tx)
            .await?
            .ok_or(StoreError::Corrupt {
                table: "clinical_library_items",
                cause: Corruption::Vanished,
            })?;
        let state = state_from_row(row)?;
        if new {
            // A newly qualified item is due tonight. One already qualified is
            // tonight's only if it was due when the night took the slot; one a
            // person made due since is the next night's.
            mark(&tx, lease, key, ItemResult::Due, Mark::IfAbsent).await?;
        }
        tx.commit().await?;
        Ok(state)
    }

    /// Applies tonight's `event` to one qualified item of tonight's sources,
    /// and records the result in tonight's ledger.
    pub async fn apply(
        &self,
        lease: &Lease,
        key: &ItemKey,
        event: &ArticleEvent,
    ) -> Result<ArticleState, StoreError> {
        let tx = self.db.begin().await?;
        locked_slot(&tx, lease).await?;
        if !lease.sources.contains(&key.source()) {
            return Err(StoreError::NotTonightsSource(key.clone()));
        }
        let from = qualified(&tx, key).await?;
        let next = from.apply(event)?;
        row_from_state(key, &next).update(&tx).await?;
        mark(&tx, lease, key, ItemResult::of(event), Mark::Replace).await?;
        await_version(&tx, key, &next).await?;
        tx.commit().await?;
        Ok(next)
    }

    /// Records that a person resolved what stopped one item, which makes it
    /// due again. Like [`Store::record_resolution`], it needs no lease. A
    /// night already running doesn't count the item as due; the next one
    /// does.
    pub async fn resolve_item(&self, key: &ItemKey) -> Result<ArticleState, StoreError> {
        let tx = self.db.begin().await?;
        let from = qualified(&tx, key).await?;
        let next = from.apply(&ArticleEvent::PersonResolved)?;
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
        if !lease.sources.contains(&Source::PubMed) {
            return Err(StoreError::NotTonightsSource(key));
        }
        let state = qualified(&tx, &key).await?;
        // Tonight's work: an article in tonight's ledger, or an admitted one
        // to re-confirm. Any other, such as one a person made due after the
        // night took the slot, is the next night's.
        let in_ledger = night_item::Entity::find_by_id((
            lease.run.0,
            Source::PubMed.as_str().to_owned(),
            key.as_str().to_owned(),
        ))
        .one(&tx)
        .await?
        .is_some();
        if !in_ledger && state != ArticleState::Admitted {
            tx.commit().await?;
            return Ok(None);
        }
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
        // An article due tonight is tonight's work, and so is an admitted
        // one whose source sent new content. Re-confirming an admitted one
        // that NLM sent unchanged refreshed its status, which moved the
        // library on; one that went unanswered stays unconfirmed, which the
        // freshness report shows, and doesn't hold the night back.
        let new_content = plan.events.first() == Some(&ArticleEvent::SourceChanged);
        let result = match (&plan.checked, in_ledger || new_content) {
            (CheckResult::Done, _) => Some(ItemResult::Done),
            (CheckResult::SetAside(_), true) => Some(ItemResult::SetAside),
            (CheckResult::SetAside(_), false) => None,
        };
        if let Some(result) = result {
            mark(&tx, lease, &key, result, Mark::Replace).await?;
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
        // The night built this version after admitting its content, so the
        // content of the night's sources awaiting a version is published.
        unpublished::Entity::delete_many()
            .filter(
                unpublished::Column::Source
                    .is_in(lease.sources.iter().map(|source| source.as_str())),
            )
            .exec(&tx)
            .await?;
        tx.commit().await?;
        Ok(Activation {
            version: candidate.version().clone(),
            run: lease.run,
            token: lease.token,
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
        history(&self.db, slot).await
    }

    /// The words a failed night recorded about its failure, newest night
    /// first, for a person reading why the nightly failed.
    pub async fn failure_details(
        &self,
        slot: &SlotName,
    ) -> Result<Vec<(FailureCode, String)>, StoreError> {
        let rows = night::Entity::find()
            .filter(night::Column::Slot.eq(slot.as_str()))
            .filter(night::Column::FailureDetail.is_not_null())
            .order_by_desc(night::Column::Id)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .filter_map(|row| Some((row.failure_code?, row.failure_detail?)))
            .map(|(code, detail)| {
                FailureCode::new(code)
                    .map(|code| (code, detail))
                    .map_err(|cause| fault("clinical_library_nights", cause))
            })
            .collect()
    }

    /// Whether the next night may run, by the stop rule.
    /// [`Store::acquire`] applies the same rule itself.
    pub async fn next(&self, slot: &SlotName) -> Result<Next, StoreError> {
        Ok(stop_rule::decide(&self.history(slot).await?))
    }

    /// What a person should be told about `slot` and the items of
    /// `sources`, read from the records.
    pub async fn alerts(
        &self,
        slot: &SlotName,
        sources: &[Source],
    ) -> Result<Vec<Alert>, StoreError> {
        let history = self.history(slot).await?;
        let rows = item::Entity::find()
            .filter(item::Column::Source.is_in(sources.iter().map(|source| source.as_str())))
            .filter(item::Column::State.is_in(["needs_person", "waiting_for_change"]))
            .all(&self.db)
            .await?;
        let mut needing_person = 0_u64;
        for row in rows {
            if state_from_row(row)?.tells_person_now() {
                needing_person += 1;
            }
        }
        Ok(alert::from_records(&history, needing_person))
    }
}

/// Whether a ledger write replaces what tonight already recorded for the
/// item, or only adds a row it lacks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mark {
    Replace,
    IfAbsent,
}

/// Records what the run holding `lease` did with one item tonight.
async fn mark(
    tx: &DatabaseTransaction,
    lease: &Lease,
    key: &ItemKey,
    result: ItemResult,
    mark: Mark,
) -> Result<(), StoreError> {
    let row = night_item::ActiveModel {
        run_id: Set(lease.run.0),
        source: Set(key.source().as_str().to_owned()),
        item_key: Set(key.as_str().to_owned()),
        result: Set(result.as_str().to_owned()),
    };
    let columns = [
        night_item::Column::RunId,
        night_item::Column::Source,
        night_item::Column::ItemKey,
    ];
    let on_conflict = match mark {
        Mark::Replace => OnConflict::columns(columns)
            .update_column(night_item::Column::Result)
            .to_owned(),
        Mark::IfAbsent => OnConflict::columns(columns).do_nothing().to_owned(),
    };
    // `try_insert`: a row kept by `Mark::IfAbsent` isn't an error.
    night_item::Entity::insert(row)
        .on_conflict(on_conflict)
        .try_insert()
        .exec(tx)
        .await?;
    Ok(())
}

/// Keeps the record of content awaiting a version in step with an item's
/// state: admitted content of a source that publishes through versions
/// awaits one, and an item no longer admitted has nothing to publish.
async fn await_version(
    tx: &DatabaseTransaction,
    key: &ItemKey,
    state: &ArticleState,
) -> Result<(), StoreError> {
    if !key.source().publishes_through_versions() {
        return Ok(());
    }
    if *state == ArticleState::Admitted {
        unpublished::Entity::insert(unpublished::ActiveModel {
            source: Set(key.source().as_str().to_owned()),
            item_key: Set(key.as_str().to_owned()),
        })
        .on_conflict_do_nothing_on([unpublished::Column::Source, unpublished::Column::ItemKey])
        .exec(tx)
        .await?;
    } else {
        unpublished::Entity::delete_many()
            .filter(unpublished::Column::Source.eq(key.source().as_str()))
            .filter(unpublished::Column::ItemKey.eq(key.as_str()))
            .exec(tx)
            .await?;
    }
    Ok(())
}

/// Puts every item of `sources` waiting to retry into `run`'s ledger as due.
async fn snapshot_due(
    tx: &DatabaseTransaction,
    run: RunId,
    sources: &BTreeSet<Source>,
) -> Result<(), StoreError> {
    if sources.is_empty() {
        return Ok(());
    }
    let due = Query::select()
        .expr(Expr::val(run.0))
        .columns([item::Column::Source, item::Column::ItemKey])
        .expr(Expr::val(ItemResult::Due.as_str()))
        .from(item::Entity)
        .and_where(item::Column::Source.is_in(sources.iter().map(|source| source.as_str())))
        .and_where(item::Column::State.eq(ArticleState::new_article().kind().as_str()))
        .to_owned();
    let insert = Query::insert()
        .into_table(night_item::Entity)
        .columns([
            night_item::Column::RunId,
            night_item::Column::Source,
            night_item::Column::ItemKey,
            night_item::Column::Result,
        ])
        .select_from(due)?
        .to_owned();
    tx.execute(&insert).await?;
    Ok(())
}

/// Counts the ledger of the run holding `lease`.
async fn tally(db: &impl ConnectionTrait, lease: &Lease) -> Result<Tally, StoreError> {
    const TABLE: &str = "clinical_library_night_items";
    let rows: Vec<(String, String, i64)> = night_item::Entity::find()
        .select_only()
        .column(night_item::Column::Source)
        .column(night_item::Column::Result)
        .column_as(night_item::Column::ItemKey.count(), "n")
        .filter(night_item::Column::RunId.eq(lease.run.0))
        .group_by(night_item::Column::Source)
        .group_by(night_item::Column::Result)
        .into_tuple()
        .all(db)
        .await?;
    let mut tally = Tally::default();
    for (source, result, n) in rows {
        let source = Source::parse(&source).ok_or_else(|| {
            fault(
                TABLE,
                Corruption::UnknownName {
                    field: "source",
                    value: source.clone(),
                },
            )
        })?;
        let result = ItemResult::parse(&result).ok_or_else(|| {
            fault(
                TABLE,
                Corruption::UnknownName {
                    field: "result",
                    value: result.clone(),
                },
            )
        })?;
        let n = u64::try_from(n).map_err(|cause| fault(TABLE, cause))?;
        // Every source in the night's ledger is one of the lease's.
        debug_assert!(lease.sources.contains(&source));
        match result {
            ItemResult::Due => tally.due_left += n,
            ItemResult::Done => tally.done += n,
            ItemResult::SetAside => tally.set_aside += n,
        }
    }
    // Content awaiting a version, whichever night admitted it, is the
    // night's to publish.
    tally.needs_version = unpublished::Entity::find()
        .filter(unpublished::Column::Source.is_in(lease.sources.iter().map(|s| s.as_str())))
        .one(db)
        .await?
        .is_some();
    Ok(tally)
}

/// Checks that `activation` is the night's own, made under this lease, and
/// that its version is still the active one this run made.
async fn own_activation(
    tx: &DatabaseTransaction,
    lease: &Lease,
    activation: Activation,
) -> Result<VersionId, StoreError> {
    let foreign = || StoreError::ForeignActivation {
        run: lease.run,
        version: activation.version.clone(),
    };
    if activation.run != lease.run || activation.token != lease.token {
        return Err(foreign());
    }
    let row = version::Entity::find_by_id(activation.version.as_str().to_owned())
        .one(tx)
        .await?;
    match row {
        Some(row) if row.active && row.activated_by_run == Some(lease.run.0) => {
            Ok(activation.version)
        }
        _ => Err(foreign()),
    }
}

/// The slot's nights and resolutions, oldest first.
async fn history(
    db: &impl ConnectionTrait,
    slot: &SlotName,
) -> Result<Vec<HistoryEntry>, StoreError> {
    let nights = night::Entity::find()
        .filter(night::Column::Slot.eq(slot.as_str()))
        .order_by_asc(night::Column::Id)
        .all(db)
        .await?;
    let resolutions = resolution::Entity::find()
        .filter(resolution::Column::Slot.eq(slot.as_str()))
        .order_by_asc(resolution::Column::Id)
        .all(db)
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
        insert_night(tx, slot, holder, &ended, None).await?;
    }
    Ok(())
}

async fn insert_night(
    db: &impl ConnectionTrait,
    slot: &SlotName,
    run: RunId,
    outcome: &NightOutcome,
    detail: Option<&FailureDetail>,
) -> Result<(), StoreError> {
    let (version, reason, code, stopped) = match outcome {
        NightOutcome::Succeeded(succeeded) => (
            succeeded.activated().map(|v| v.as_str().to_owned()),
            None,
            None,
            None,
        ),
        NightOutcome::Deferred(deferred) => {
            let stopped = match deferred.reason() {
                DeferralReason::Stopped(why) => Some(why),
                _ => None,
            };
            (
                None,
                Some(deferred.reason().as_str()),
                stopped
                    .and_then(StopReason::code)
                    .map(|code| code.as_str().to_owned()),
                stopped.map(StopReason::as_str),
            )
        }
        NightOutcome::Failed(failed) => (None, None, Some(failed.code().as_str().to_owned()), None),
    };
    night::ActiveModel {
        slot: Set(slot.as_str().to_owned()),
        run_id: Set(run.0),
        outcome: Set(outcome.kind().as_str().to_owned()),
        activated_version: Set(version),
        deferral_reason: Set(reason.map(str::to_owned)),
        failure_code: Set(code),
        stop_reason: Set(stopped.map(str::to_owned)),
        failure_detail: Set(detail.map(|detail| detail.as_str().to_owned())),
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
    let code = row
        .failure_code
        .map(FailureCode::new)
        .transpose()
        .map_err(|cause| fault(TABLE, cause))?;
    // A stopped night stores its rule, and the failure code of a stop on
    // the same failure twice; any other night stores its own code.
    let (stopped, code) = match row.stop_reason {
        Some(value) => {
            let why = StopReason::from_stored(&value, code).ok_or(Corruption::UnknownName {
                field: "stop_reason",
                value,
            });
            (Some(why.map_err(|cause| fault(TABLE, cause))?), None)
        }
        None => (None, code),
    };
    let reason = row
        .deferral_reason
        .map(|value| {
            DeferralReason::from_stored(&value, stopped.clone()).ok_or(Corruption::UnknownName {
                field: "deferral_reason",
                value,
            })
        })
        .transpose()
        .map_err(|cause| fault(TABLE, cause))?;
    if reason.is_none() && stopped.is_some() {
        return Err(fault(
            TABLE,
            Corruption::Inconsistent("a stop reason on a night that didn't defer"),
        ));
    }
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
