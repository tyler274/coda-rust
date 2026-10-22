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
    Ok(tx.query_one_raw(sql("SELECT *,lease_until>clock_timestamp() AS live,retry_deadline>clock_timestamp() AS retryable FROM duet_conversation_inputs WHERE account_id=$1 AND run_id=$2",vec![account.into(),run.into()])).await?)
}
async fn revision(tx: &DatabaseTransaction, account: Uuid) -> Result<i64, AuthorityError> {
    Ok(tx.query_one_raw(sql("UPDATE duet_conversation_state SET revision=revision+1 WHERE account_id=$1 RETURNING revision",vec![account.into()])).await?.ok_or(AuthorityError::Refused)?.try_get("","revision")?)
}
async fn end(
    tx: &DatabaseTransaction,
    account: Uuid,
    run: Uuid,
    state: &str,
) -> Result<(), AuthorityError> {
    let rev = revision(tx, account).await?;
    tx.execute_raw(sql(
        "UPDATE duet_conversation_attempts SET state=$2 WHERE run_id=$1 AND state='running'",
        vec![run.into(), state.into()],
    ))
    .await?;
    tx.execute_raw(sql("UPDATE duet_conversation_messages SET state=$3,revision=$4 WHERE account_id=$1 AND id=(SELECT reply_id FROM duet_conversation_inputs WHERE run_id=$2)",vec![account.into(),run.into(),state.into(),rev.into()])).await?;
    tx.execute_raw(sql("UPDATE duet_conversation_inputs SET state=$2,failure=$2,lease_until=NULL,revision=$3 WHERE run_id=$1",vec![run.into(),state.into(),rev.into()])).await?;
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
            tx.execute_raw(sql("UPDATE duet_conversation_attempts SET state='interrupted' WHERE run_id=$1 AND state='running'",vec![run.into()])).await?;
            tx.execute_raw(sql("UPDATE duet_conversation_messages SET state='interrupted',revision=$3 WHERE account_id=$1 AND id=(SELECT reply_id FROM duet_conversation_inputs WHERE run_id=$2)",vec![account.id.into(),run.into(),rev.into()])).await?;
            tx.execute_raw(sql("UPDATE duet_conversation_inputs SET state='paused',lease_until=NULL,revision=$2 WHERE run_id=$1",vec![run.into(),rev.into()])).await?;
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
        tx.execute_raw(sql("UPDATE duet_conversation_attempts SET state='expired' WHERE run_id=$1 AND state='running'",vec![run.into()])).await?;
        let token = Uuid::new_v4();
        let rev = revision(&tx, account.id).await?;
        tx.execute_raw(sql("INSERT INTO duet_conversation_attempts(id,run_id,ordinal,state) VALUES($1,$2,$3,'running')",vec![token.into(),run.into(),(count+1).into()])).await?;
        tx.execute_raw(sql("UPDATE duet_conversation_inputs SET state='running',claim_token=$2,attempt_count=attempt_count+1,lease_until=clock_timestamp()+interval '720 seconds',retry_deadline=coalesce(retry_deadline,clock_timestamp()+interval '30 minutes'),revision=$3 WHERE run_id=$1",vec![run.into(),token.into(),rev.into()])).await?;
        tx.execute_raw(sql("UPDATE duet_conversation_messages SET state='streaming',text='',revision=$3 WHERE account_id=$1 AND id=(SELECT reply_id FROM duet_conversation_inputs WHERE run_id=$2)",vec![account.id.into(),run.into(),rev.into()])).await?;
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
        let rows=tx.query_all_raw(sql("SELECT run_id FROM duet_conversation_inputs WHERE account_id=$1 AND (state='queued' OR (state='running' AND lease_until<=clock_timestamp())) ORDER BY created_at,run_id LIMIT 64",vec![a.id.into()])).await?;
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
        let result=tx.execute_raw(sql("UPDATE duet_conversation_attempts SET execution_started=true WHERE id=$1 AND NOT execution_started",vec![claim.token.into()])).await?;
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
            .query_one_raw(sql(
                "SELECT * FROM duet_attempt_events WHERE attempt_id=$1 AND sequence=$2",
                vec![claim.token.into(), event.sequence.into()],
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
        let n:i64=tx.query_one_raw(sql("SELECT coalesce(max(sequence),0) AS n FROM duet_attempt_events WHERE attempt_id=$1",vec![claim.token.into()])).await?.ok_or(AuthorityError::Refused)?.try_get("","n")?;
        if event.sequence != n + 1 {
            return Err(AuthorityError::Refused);
        }
        tx.execute_raw(sql("INSERT INTO duet_attempt_events(attempt_id,sequence,body,delta,candidate) VALUES($1,$2,$3,$4,$5)",vec![claim.token.into(),event.sequence.into(),event.body.clone().into(),event.delta.clone().into(),event.candidate.clone().into()])).await?;
        if let Some(delta) = &event.delta {
            let rev = revision(&tx, claim.account).await?;
            tx.execute_raw(sql("UPDATE duet_conversation_messages SET text=text||$3,revision=$4 WHERE account_id=$1 AND id=(SELECT reply_id FROM duet_conversation_inputs WHERE run_id=$2)",vec![claim.account.into(),claim.run.into(),delta.clone().into(),rev.into()])).await?;
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
                .query_one_raw(sql(
                    "SELECT text FROM duet_conversation_messages WHERE account_id=$1 AND id=$2",
                    vec![a.id.into(), r.try_get::<Uuid>("", "reply_id")?.into()],
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
        let last=tx.query_one_raw(sql("SELECT candidate FROM duet_attempt_events WHERE attempt_id=$1 ORDER BY sequence DESC LIMIT 1",vec![claim.token.into()])).await?.ok_or(AuthorityError::Refused)?;
        if last.try_get::<Option<String>>("", "candidate")?.as_deref() != Some(text) {
            return Err(AuthorityError::Refused);
        }
        let rev = revision(&tx, a.id).await?;
        tx.execute_raw(sql("UPDATE duet_conversation_messages SET text=$3,state='complete',revision=$4 WHERE account_id=$1 AND id=$2",vec![a.id.into(),r.try_get::<Uuid>("","reply_id")?.into(),text.into(),rev.into()])).await?;
        tx.execute_raw(sql("UPDATE duet_conversation_inputs SET state='completed',lease_until=NULL,revision=$2 WHERE run_id=$1",vec![claim.run.into(),rev.into()])).await?;
        tx.execute_raw(sql(
            "UPDATE duet_conversation_attempts SET state='completed' WHERE id=$1",
            vec![claim.token.into()],
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
        let rows=tx.query_all_raw(sql("SELECT e.* FROM duet_attempt_events e JOIN duet_conversation_attempts t ON t.id=e.attempt_id JOIN duet_conversation_inputs i ON i.run_id=t.run_id WHERE i.account_id=$1 AND e.attempt_id=$2 AND e.sequence>$3 ORDER BY e.sequence LIMIT 100",vec![a.id.into(),attempt.into(),after.into()])).await?;
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
