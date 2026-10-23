//! Duet-only durable claims. Database time is read after canonical account locks.
use super::*;
use sea_orm::QueryResult;

/// Opaque attempt identity. Only a successful claim can construct one.
#[derive(Clone, Debug)]
pub struct Claim {
    pub(super) principal: Principal,
    pub(super) account: Uuid,
    pub(super) run: Uuid,
    pub(super) token: Uuid,
}
impl Claim {
    /// Stable accepted run, retained across successor attempts.
    pub fn run_id(&self) -> Uuid {
        self.run
    }
    /// Fresh attempt identity; never reusable for a successor.
    pub fn attempt_id(&self) -> Uuid {
        self.token
    }
}
/// Bounded acknowledged event. Body is exact serialized neutral evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredEvent {
    /// One-based attempt-local sequence.
    pub sequence: i64,
    /// Exact event bytes, compared on retry.
    pub body: String,
    /// Optional visible text fragment, committed with its acknowledgment.
    pub delta: Option<String>,
    /// Optional completion candidate, not yet authoritative output.
    pub candidate: Option<String>,
}
async fn row(
    tx: &DatabaseTransaction,
    account: Uuid,
    run: Uuid,
) -> Result<Option<QueryResult>, AuthorityError> {
    Ok(tx
        .query_one_raw(statement(
            Query::select()
                .column(sea_orm::sea_query::Asterisk)
                .expr_as(
                    Expr::col(I::LeaseUntil).gt(query::clock()),
                    Alias::new("live"),
                )
                .expr_as(
                    Expr::col(I::RetryDeadline).gt(query::clock()),
                    Alias::new("retryable"),
                )
                .from(I::Table)
                .and_where(Expr::col(I::AccountId).eq(account))
                .and_where(Expr::col(I::RunId).eq(run)),
        ))
        .await?)
}
pub(super) async fn revision(
    tx: &DatabaseTransaction,
    account: Uuid,
) -> Result<i64, AuthorityError> {
    Ok(tx
        .query_one_raw(statement(&query::revision_update(account)))
        .await?
        .ok_or(AuthorityError::Refused)?
        .try_get("", "revision")?)
}
async fn end(
    tx: &DatabaseTransaction,
    account: Uuid,
    run: Uuid,
    state: &str,
) -> Result<(), AuthorityError> {
    let rev = revision(tx, account).await?;
    tx.execute_raw(statement(&query::running_attempts(run, state)))
        .await?;
    tx.execute_raw(statement(
        query::reply_update(account, run)
            .value(M::State, state)
            .value(M::Revision, rev),
    ))
    .await?;
    tx.execute_raw(statement(
        query::run_update(run)
            .value(I::State, state)
            .value(I::Failure, state)
            .value(
                I::LeaseUntil,
                Expr::Keyword(sea_orm::sea_query::Keyword::Null),
            )
            .value(I::Revision, rev),
    ))
    .await?;
    Ok(())
}
pub(super) async fn fence(
    tx: &DatabaseTransaction,
    claim: &Claim,
) -> Result<QueryResult, AuthorityError> {
    let (account, permissions) = identity::locked(tx, &claim.principal).await?;
    // Publication fence: holds the switch's shared lock until this write commits.
    if !super::admission::enabled(tx).await? {
        return Err(AuthorityError::ModelExecutionOff);
    }
    if account.id != claim.account
        || !(Access {
            account_id: account.id,
            permissions,
        })
        .allows_luna()
    {
        return Err(AuthorityError::Refused);
    }
    let r = row(tx, account.id, claim.run)
        .await?
        .ok_or(AuthorityError::Refused)?;
    if r.try_get::<String>("", "state")? != "running"
        || r.try_get::<Option<Uuid>>("", "claim_token")? != Some(claim.token)
        || !r.try_get::<Option<bool>>("", "live")?.unwrap_or(false)
    {
        return Err(AuthorityError::Refused);
    }
    Ok(r)
}
impl PostgresFrontDoorStore {
    /// Claim original accepted work under current and original session authority.
    /// Existing pre-origin migration rows fail closed, never acquire inferred
    /// authority. Fixed source bounds: 720s lease, three attempts, 30m window.
    pub async fn claim_conversation(
        &self,
        principal: &Principal,
        run: Uuid,
    ) -> Result<Option<Claim>, AuthorityError> {
        let tx = self.db.begin().await?;
        let (account, permissions) = identity::locked(&tx, principal).await?;
        let Some(r) = row(&tx, account.id, run).await? else {
            return Ok(None);
        };
        let state: String = r.try_get("", "state")?;
        if !["queued", "running", "paused"].contains(&state.as_str()) {
            return Ok(None);
        }
        let origin = r.try_get::<Option<String>>("", "origin_project")?
            == Some(principal.project.clone())
            && r.try_get::<Option<String>>("", "origin_subject")?
                == Some(principal.subject.clone())
            && r.try_get::<Option<i64>>("", "origin_authenticated_at")?
                .is_some_and(|n| n > 0 && n >= account.sessions_ended_before);
        if !origin {
            end(&tx, account.id, run, "failed").await?;
            tx.commit().await?;
            return Ok(None);
        }
        let access = Access {
            account_id: account.id,
            permissions,
        };
        if access.permissions.terms_version.as_deref() != Some(identity::TERMS_VERSION) {
            // Retain accepted work; resumption is deliberately not implemented.
            let rev = revision(&tx, account.id).await?;
            tx.execute_raw(statement(&query::running_attempts(run, "interrupted")))
                .await?;
            tx.execute_raw(statement(
                query::reply_update(account.id, run)
                    .value(M::State, "interrupted")
                    .value(M::Revision, rev),
            ))
            .await?;
            tx.execute_raw(statement(
                query::run_update(run)
                    .value(I::State, "paused")
                    .value(
                        I::LeaseUntil,
                        Expr::Keyword(sea_orm::sea_query::Keyword::Null),
                    )
                    .value(I::Revision, rev),
            ))
            .await?;
            tx.commit().await?;
            return Ok(None);
        }
        if !access.allows_luna() {
            end(&tx, account.id, run, "failed").await?;
            tx.commit().await?;
            return Ok(None);
        }
        // Switched-off work stays paused as the same accepted run; nothing is
        // claimed while the operator switch is off.
        if !super::admission::enabled(&tx).await? {
            return Ok(None);
        }
        if state == "paused"
            || (state == "running" && r.try_get::<Option<bool>>("", "live")?.unwrap_or(false))
        {
            return Ok(None);
        }
        let count: i32 = r.try_get("", "attempt_count")?;
        if count >= 3
            || (count > 0 && !r.try_get::<Option<bool>>("", "retryable")?.unwrap_or(false))
        {
            end(&tx, account.id, run, "failed").await?;
            tx.commit().await?;
            return Ok(None);
        }
        tx.execute_raw(statement(&query::running_attempts(run, "expired")))
            .await?;
        let token = Uuid::new_v4();
        let rev = revision(&tx, account.id).await?;
        tx.execute_raw(statement(
            Query::insert()
                .into_table(T::Table)
                .columns([T::Id, T::RunId, T::Ordinal, T::State])
                .values_panic([
                    token.into(),
                    run.into(),
                    (count + 1).into(),
                    "running".into(),
                ]),
        ))
        .await?;
        // PostgreSQL intervals and wall clock preserve lease/retry timing after lock acquisition.
        tx.execute_raw(statement(
            query::run_update(run)
                .value(I::State, "running")
                .value(I::ClaimToken, token)
                .value(I::AttemptCount, Expr::col(I::AttemptCount).add(1))
                .value(
                    I::LeaseUntil,
                    query::clock().add(Expr::cust("interval '720 seconds'")),
                )
                .value(
                    I::RetryDeadline,
                    Func::coalesce([
                        Expr::col(I::RetryDeadline),
                        query::clock().add(Expr::cust("interval '30 minutes'")),
                    ]),
                )
                .value(I::Revision, rev),
        ))
        .await?;
        tx.execute_raw(statement(
            query::reply_update(account.id, run)
                .value(M::State, "streaming")
                .value(M::Text, "")
                .value(M::Revision, rev),
        ))
        .await?;
        tx.commit().await?;
        let mut original = principal.clone();
        original.authenticated_at = r
            .try_get::<Option<i64>>("", "origin_authenticated_at")?
            .ok_or(AuthorityError::Refused)?;
        Ok(Some(Claim {
            principal: original,
            account: account.id,
            run,
            token,
        }))
    }
    /// Bounded restart scan for this canonical identity. Caller invokes claim;
    /// the scan alone grants no authority and duplicate delivery is harmless.
    pub async fn pending_conversations(
        &self,
        principal: &Principal,
    ) -> Result<Vec<Uuid>, AuthorityError> {
        let tx = self.db.begin().await?;
        let (a, _) = identity::locked(&tx, principal).await?;
        let rows = tx
            .query_all_raw(statement(
                Query::select()
                    .column(I::RunId)
                    .from(I::Table)
                    .and_where(Expr::col(I::AccountId).eq(a.id))
                    .cond_where(
                        Cond::any().add(Expr::col(I::State).eq("queued")).add(
                            Cond::all()
                                .add(Expr::col(I::State).eq("running"))
                                .add(Expr::col(I::LeaseUntil).lte(query::clock())),
                        ),
                    )
                    .order_by(I::CreatedAt, Order::Asc)
                    .order_by(I::RunId, Order::Asc)
                    .limit(64),
            ))
            .await?;
        let ids = rows
            .into_iter()
            .map(|r| r.try_get("", "run_id"))
            .collect::<Result<Vec<_>, _>>()?;
        tx.commit().await?;
        Ok(ids)
    }
    /// Revalidate immediately before neutral engine dispatch. Every write also
    /// repeats this fence in its own transaction; this read is not a grant.
    pub async fn assert_conversation_claim(&self, claim: &Claim) -> Result<(), AuthorityError> {
        let tx = self.db.begin().await?;
        fence(&tx, claim).await?;
        tx.commit().await?;
        Ok(())
    }
    /// Reserve one engine invocation for this attempt. A lost acknowledgment is
    /// recovered only through lease expiry and a successor, never by invoking
    /// the provider twice with the same attempt/event namespace.
    pub async fn begin_conversation_execution(&self, claim: &Claim) -> Result<(), AuthorityError> {
        let tx = self.db.begin().await?;
        fence(&tx, claim).await?;
        let result = tx
            .execute_raw(statement(
                Query::update()
                    .table(T::Table)
                    .value(T::ExecutionStarted, true)
                    .and_where(Expr::col(T::Id).eq(claim.token))
                    .and_where(Expr::col(T::ExecutionStarted).eq(false)),
            ))
            .await?;
        if result.rows_affected() != 1 {
            return Err(AuthorityError::Refused);
        }
        tx.commit().await?;
        Ok(())
    }
    /// Persist ordered evidence and visible text atomically before acknowledging.
    /// Duplicate exact events do not append text twice; gaps/conflicts refuse.
    pub async fn acknowledge_conversation_event(
        &self,
        claim: &Claim,
        event: &StoredEvent,
    ) -> Result<(), AuthorityError> {
        if !(1..=4096).contains(&event.sequence)
            || event.body.len() > 262144
            || event
                .delta
                .as_ref()
                .is_some_and(|s| s.chars().count() > 32768)
            || event
                .candidate
                .as_ref()
                .is_some_and(|s| s.chars().count() > 32768)
            || (event.delta.is_some() && event.candidate.is_some())
        {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        fence(&tx, claim).await?;
        if let Some(r) = tx
            .query_one_raw(statement(
                Query::select()
                    .column(sea_orm::sea_query::Asterisk)
                    .from(E::Table)
                    .and_where(Expr::col(E::AttemptId).eq(claim.token))
                    .and_where(Expr::col(E::Sequence).eq(event.sequence)),
            ))
            .await?
        {
            if r.try_get::<String>("", "body")? != event.body
                || r.try_get::<Option<String>>("", "delta")? != event.delta
                || r.try_get::<Option<String>>("", "candidate")? != event.candidate
            {
                return Err(AuthorityError::Refused);
            }
            tx.commit().await?;
            return Ok(());
        }
        let n: i64 = tx
            .query_one_raw(statement(
                Query::select()
                    .expr_as(
                        Func::coalesce([Expr::col(E::Sequence).max(), Expr::val(0)]),
                        Alias::new("n"),
                    )
                    .from(E::Table)
                    .and_where(Expr::col(E::AttemptId).eq(claim.token)),
            ))
            .await?
            .ok_or(AuthorityError::Refused)?
            .try_get("", "n")?;
        if event.sequence != n + 1 {
            return Err(AuthorityError::Refused);
        }
        tx.execute_raw(statement(
            Query::insert()
                .into_table(E::Table)
                .columns([E::AttemptId, E::Sequence, E::Body, E::Delta, E::Candidate])
                .values_panic([
                    claim.token.into(),
                    event.sequence.into(),
                    event.body.clone().into(),
                    event.delta.clone().into(),
                    event.candidate.clone().into(),
                ]),
        ))
        .await?;
        if let Some(delta) = &event.delta {
            let rev = revision(&tx, claim.account).await?;
            use sea_orm::sea_query::extension::postgres::PgExpr;
            tx.execute_raw(statement(
                query::reply_update(claim.account, claim.run)
                    .value(M::Text, Expr::col(M::Text).concatenate(delta.clone()))
                    .value(M::Revision, rev),
            ))
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
    /// Publish only a durably acknowledged last candidate from the current lease.
    /// Repeating an already committed identical result reconciles once; never
    /// invokes provider/tools again. No external effects or billing guarantees.
    pub async fn complete_conversation(
        &self,
        claim: &Claim,
        text: &str,
    ) -> Result<(), AuthorityError> {
        if text.trim().is_empty() || text.chars().count() > 32768 {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        let (a, _) = identity::locked(&tx, &claim.principal).await?;
        let r = row(&tx, a.id, claim.run)
            .await?
            .ok_or(AuthorityError::Refused)?;
        if r.try_get::<String>("", "state")? == "completed"
            && r.try_get::<Option<Uuid>>("", "claim_token")? == Some(claim.token)
        {
            let existing = tx
                .query_one_raw(statement(
                    Query::select()
                        .column(M::Text)
                        .from(M::Table)
                        .and_where(Expr::col(M::AccountId).eq(a.id))
                        .and_where(Expr::col(M::Id).eq(r.try_get::<Uuid>("", "reply_id")?)),
                ))
                .await?
                .ok_or(AuthorityError::Refused)?;
            if existing.try_get::<String>("", "text")? != text {
                return Err(AuthorityError::Refused);
            }
            tx.commit().await?;
            return Ok(());
        }
        fence(&tx, claim).await?;
        let last = tx
            .query_one_raw(statement(
                Query::select()
                    .column(E::Candidate)
                    .from(E::Table)
                    .and_where(Expr::col(E::AttemptId).eq(claim.token))
                    .order_by(E::Sequence, Order::Desc)
                    .limit(1),
            ))
            .await?
            .ok_or(AuthorityError::Refused)?;
        if last.try_get::<Option<String>>("", "candidate")?.as_deref() != Some(text) {
            return Err(AuthorityError::Refused);
        }
        let rev = revision(&tx, a.id).await?;
        tx.execute_raw(statement(
            Query::update()
                .table(M::Table)
                .value(M::Text, text)
                .value(M::State, "complete")
                .value(M::Revision, rev)
                .and_where(Expr::col(M::AccountId).eq(a.id))
                .and_where(Expr::col(M::Id).eq(r.try_get::<Uuid>("", "reply_id")?)),
        ))
        .await?;
        tx.execute_raw(statement(
            query::run_update(claim.run)
                .value(I::State, "completed")
                .value(
                    I::LeaseUntil,
                    Expr::Keyword(sea_orm::sea_query::Keyword::Null),
                )
                .value(I::Revision, rev),
        ))
        .await?;
        tx.execute_raw(statement(
            Query::update()
                .table(T::Table)
                .value(T::State, "completed")
                .and_where(Expr::col(T::Id).eq(claim.token)),
        ))
        .await?;
        tx.commit().await?;
        Ok(())
    }
    /// Intentional Stop is terminal, unlike lease expiry. Preserve visible prose.
    pub async fn stop_conversation(
        &self,
        principal: &Principal,
        run: Uuid,
    ) -> Result<(), AuthorityError> {
        let tx = self.db.begin().await?;
        let (a, _) = identity::locked(&tx, principal).await?;
        let r = row(&tx, a.id, run).await?.ok_or(AuthorityError::Refused)?;
        if ["queued", "running", "paused"].contains(&r.try_get::<String>("", "state")?.as_str()) {
            end(&tx, a.id, run, "interrupted").await?;
        }
        tx.commit().await?;
        Ok(())
    }
    /// Read acknowledged attempt events after reconnect, bounded to 100 rows.
    /// Does not require still-current Luna permission, only account authority.
    pub async fn conversation_events(
        &self,
        principal: &Principal,
        attempt: Uuid,
        after: i64,
    ) -> Result<Vec<StoredEvent>, AuthorityError> {
        if after < 0 {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        let (a, _) = identity::locked(&tx, principal).await?;
        let rows = tx
            .query_all_raw(statement(
                Query::select()
                    .expr(Expr::col((E::Table, sea_orm::sea_query::Asterisk)))
                    .from(E::Table)
                    .join(
                        JoinType::InnerJoin,
                        T::Table,
                        Expr::col((T::Table, T::Id)).equals((E::Table, E::AttemptId)),
                    )
                    .join(
                        JoinType::InnerJoin,
                        I::Table,
                        Expr::col((I::Table, I::RunId)).equals((T::Table, T::RunId)),
                    )
                    .and_where(Expr::col((I::Table, I::AccountId)).eq(a.id))
                    .and_where(Expr::col((E::Table, E::AttemptId)).eq(attempt))
                    .and_where(Expr::col((E::Table, E::Sequence)).gt(after))
                    .order_by((E::Table, E::Sequence), Order::Asc)
                    .limit(100),
            ))
            .await?;
        let events = rows
            .into_iter()
            .map(|r| {
                Ok(StoredEvent {
                    sequence: r.try_get("", "sequence")?,
                    body: r.try_get("", "body")?,
                    delta: r.try_get("", "delta")?,
                    candidate: r.try_get("", "candidate")?,
                })
            })
            .collect::<Result<Vec<_>, sea_orm::DbErr>>()?;
        tx.commit().await?;
        Ok(events)
    }
}
