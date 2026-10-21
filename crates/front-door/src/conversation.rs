//! Duet's plain-text accepted-work owner, sharing the canonical account transaction.
//! No physician types, provider execution, claim/dispatch, effects or HTTP routes.
//! New acceptance requires an admission capability that cannot yet be constructed
//! in production: operator-switch, usage and runtime qualification are not ported.
use crate::{
    PostgresFrontDoorStore,
    identity::{self, Access, AuthorityError, Principal},
};
use sea_orm::{ConnectionTrait, DatabaseTransaction, DbBackend, Statement, TransactionTrait};
use uuid::Uuid;

/// Future admission owner must establish operator switch, allowance and runtime
/// eligibility transactionally. No public constructor, deserialization or boolean
/// bypass exists. This is NOT a reusable model/effect permission grant.
pub struct Admission {
    _private: (),
}
/// A plain text submission only. Selected context and reply-to are deliberately
/// not supported yet; no owner/account selector can be supplied.
pub struct Input {
    /// Client's stable input UUID. Retrying must retain it.
    pub id: Uuid,
    /// Exact user-authored text; never normalized or truncated.
    pub text: String,
    /// IANA timezone, validated by PostgreSQL's installed zone catalog.
    pub timezone: String,
}
/// Stored message projection, ordered by account-local sequence.
#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    /// Immutable message ID.
    pub id: Uuid,
    /// Monotonic account-local order.
    pub sequence: i64,
    /// Revision for current-state readback.
    pub revision: i64,
    /// User or assistant.
    pub role: String,
    /// Exact text; pending assistant text is empty, not a fabricated reply.
    pub text: String,
    /// Complete user message, pending or interrupted assistant.
    pub state: String,
}
/// One durable accepted input and its queued/interrupted response reservation.
#[derive(Debug, PartialEq, Eq)]
pub struct Accepted {
    /// Stable client message ID.
    pub message_id: Uuid,
    /// Stable work identity, not evidence of execution.
    pub run_id: Uuid,
    /// Stable pending response ID.
    pub reply_id: Uuid,
    /// Exact original text.
    pub text: String,
    /// Original timezone; changing timezone on retry never changes acceptance.
    pub timezone: String,
    /// Queued or interrupted, never falsely completed.
    pub state: String,
    /// Account conversation revision at this read.
    pub revision: i64,
    /// Permission revision captured at admission; execution MUST revalidate.
    pub permission_revision: i64,
}
fn sql(query: &str, values: Vec<sea_orm::Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, query, values)
}
async fn accepted(
    tx: &DatabaseTransaction,
    account: Uuid,
    id: Uuid,
) -> Result<Option<Accepted>, AuthorityError> {
    tx.query_one_raw(sql("SELECT i.*,m.text,s.revision AS current_revision FROM duet_conversation_inputs i JOIN duet_conversation_messages m ON m.account_id=i.account_id AND m.id=i.message_id JOIN duet_conversation_state s ON s.account_id=i.account_id WHERE i.account_id=$1 AND i.message_id=$2", vec![account.into(),id.into()])).await?
        .map(|r| Ok(Accepted { message_id:r.try_get("","message_id")?, run_id:r.try_get("","run_id")?,reply_id:r.try_get("","reply_id")?,text:r.try_get("","text")?,timezone:r.try_get("","timezone")?,state:r.try_get("","state")?,revision:r.try_get("","current_revision")?,permission_revision:r.try_get("","permission_revision")? })).transpose()
}
impl PostgresFrontDoorStore {
    /// Accept plain text once under the canonical account/session lock. Existing
    /// IDs are reconciled before permission/availability checks, preserving access
    /// to the person's retained history after withdrawal. Different text conflicts.
    /// None refuses NEW work without storing the draft. No model is dispatched.
    pub async fn accept_conversation(
        &self,
        principal: &Principal,
        input: &Input,
        admission: Option<&Admission>,
    ) -> Result<Accepted, AuthorityError> {
        if input.id.is_nil()
            || input.text.trim().is_empty()
            || input.text.chars().count() > 32768
            || input.timezone.is_empty()
            || input.timezone.len() > 100
        {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        let (account, permissions) = identity::locked(&tx, principal).await?;
        if let Some(existing) = accepted(&tx, account.id, input.id).await? {
            if existing.text != input.text {
                return Err(AuthorityError::AgreementChanged);
            }
            tx.commit().await?;
            return Ok(existing);
        }
        let access = Access {
            account_id: account.id,
            permissions,
        };
        if !access.allows_luna() || admission.is_none() {
            return Err(AuthorityError::Refused);
        }
        if tx
            .query_one_raw(sql(
                "SELECT name FROM pg_timezone_names WHERE name=$1",
                vec![input.timezone.clone().into()],
            ))
            .await?
            .is_none()
        {
            return Err(AuthorityError::Refused);
        }
        tx.execute_raw(sql(
            "INSERT INTO duet_conversation_state(account_id) VALUES($1) ON CONFLICT DO NOTHING",
            vec![account.id.into()],
        ))
        .await?;
        tx.execute_raw(sql(
            "SELECT duet_interrupt_accepted($1)",
            vec![account.id.into()],
        ))
        .await?;
        let row = tx.query_one_raw(sql("UPDATE duet_conversation_state SET revision=revision+1,next_sequence=next_sequence+2 WHERE account_id=$1 RETURNING revision,next_sequence-2 AS sequence",vec![account.id.into()])).await?.ok_or(AuthorityError::Refused)?;
        let revision: i64 = row.try_get("", "revision")?;
        let sequence: i64 = row.try_get("", "sequence")?;
        let run = Uuid::new_v4();
        let reply = Uuid::new_v4();
        tx.execute_raw(sql("INSERT INTO duet_conversation_inputs(account_id,message_id,run_id,reply_id,timezone,state,permission_revision,revision) VALUES($1,$2,$3,$4,$5,'queued',$6,$7)", vec![account.id.into(),input.id.into(),run.into(),reply.into(),input.timezone.clone().into(),access.permissions.revision.into(),revision.into()])).await?;
        tx.execute_raw(sql("INSERT INTO duet_conversation_messages(account_id,id,input_id,sequence,revision,role,text,state) VALUES($1,$2,$2,$4,$5,'user',$6,'complete'),($1,$3,$2,$4+1,$5,'assistant','','pending')",vec![account.id.into(),input.id.into(),reply.into(),sequence.into(),revision.into(),input.text.clone().into()])).await?;
        let result = accepted(&tx, account.id, input.id)
            .await?
            .ok_or(AuthorityError::Refused)?;
        tx.commit().await?;
        Ok(result)
    }
    /// Resolve accepted input through current canonical session authority. This is
    /// history readback only, never a grant to dispatch or publish effects.
    pub async fn conversation_acceptance(
        &self,
        principal: &Principal,
        id: Uuid,
    ) -> Result<Option<Accepted>, AuthorityError> {
        let tx = self.db.begin().await?;
        let (account, _) = identity::locked(&tx, principal).await?;
        let result = accepted(&tx, account.id, id).await?;
        tx.commit().await?;
        Ok(result)
    }
    /// Bounded chronological page before an exclusive sequence cursor. History
    /// remains readable after permission withdrawal, but not session revocation.
    pub async fn conversation_messages(
        &self,
        principal: &Principal,
        before: Option<i64>,
        limit: u64,
    ) -> Result<Vec<Message>, AuthorityError> {
        if !(1..=100).contains(&limit) || before.is_some_and(|n| n <= 0) {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        let (account, _) = identity::locked(&tx, principal).await?;
        let rows = tx.query_all_raw(sql("SELECT * FROM (SELECT * FROM duet_conversation_messages WHERE account_id=$1 AND ($2::bigint IS NULL OR sequence<$2) ORDER BY sequence DESC LIMIT $3) page ORDER BY sequence",vec![account.id.into(),before.into(),(limit as i64).into()])).await?;
        let messages = rows
            .into_iter()
            .map(|r| {
                Ok(Message {
                    id: r.try_get("", "id")?,
                    sequence: r.try_get("", "sequence")?,
                    revision: r.try_get("", "revision")?,
                    role: r.try_get("", "role")?,
                    text: r.try_get("", "text")?,
                    state: r.try_get("", "state")?,
                })
            })
            .collect::<Result<Vec<_>, sea_orm::DbErr>>()?;
        tx.commit().await?;
        Ok(messages)
    }
}
#[cfg(test)]
mod tests;
