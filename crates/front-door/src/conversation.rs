//! Duet's plain-text accepted-work owner, sharing the canonical account transaction.
//! No physician types, provider execution, effects or HTTP routes. The execution
//! submodule owns claims, acknowledged events and completion, not model mechanics.
//! New acceptance requires an [`Admission`] capability, constructible only with an
//! operator health-data attestation; the operator switch, consent and allowance
//! are then rechecked under the account lock on every acceptance.
use crate::{
    PostgresFrontDoorStore,
    identity::{self, Access, AuthorityError, Principal},
};
use sea_orm::sea_query::ExprTrait as _;
use sea_orm::sea_query::{Alias, Cond, Expr, Func, JoinType, OnConflict, Order, Query};
use sea_orm::{ConnectionTrait, DatabaseTransaction, DbBackend, Statement, TransactionTrait};
use uuid::Uuid;
mod query;
use query::{
    Attempt as T, Event as E, Input as I, Message as M, State as S, Switch as W, statement,
};

pub use admission::{
    Admission, DEFAULT_DAILY_REPLY_LIMIT, DailyLimit, OperatorAttestation, PHI_READY_ENV,
    SwitchChange,
};
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
    /// Product lifecycle state; completed only by the transactional result owner.
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
    tx.query_one_raw(statement(
        Query::select()
            .expr(Expr::col((I::Table, sea_orm::sea_query::Asterisk)))
            .column((M::Table, M::Text))
            .expr_as(
                Expr::col((S::Table, S::Revision)),
                Alias::new("current_revision"),
            )
            .from(I::Table)
            .join(
                JoinType::InnerJoin,
                M::Table,
                Cond::all()
                    .add(Expr::col((M::Table, M::AccountId)).equals((I::Table, I::AccountId)))
                    .add(Expr::col((M::Table, M::Id)).equals((I::Table, I::MessageId))),
            )
            .join(
                JoinType::InnerJoin,
                S::Table,
                Expr::col((S::Table, S::AccountId)).equals((I::Table, I::AccountId)),
            )
            .and_where(Expr::col((I::Table, I::AccountId)).eq(account))
            .and_where(Expr::col((I::Table, I::MessageId)).eq(id)),
    ))
    .await?
    .map(|r| {
        Ok(Accepted {
            message_id: r.try_get("", "message_id")?,
            run_id: r.try_get("", "run_id")?,
            reply_id: r.try_get("", "reply_id")?,
            text: r.try_get("", "text")?,
            timezone: r.try_get("", "timezone")?,
            state: r.try_get("", "state")?,
            revision: r.try_get("", "current_revision")?,
            permission_revision: r.try_get("", "permission_revision")?,
        })
    })
    .transpose()
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
        let Some(admission) = admission else {
            return Err(AuthorityError::Refused);
        };
        if !access.allows_luna() {
            return Err(AuthorityError::Refused);
        }
        // PostgreSQL's installed timezone catalog, not an application list,
        // remains authoritative (including aliases and installed tzdata).
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
        // Refused while switched off, so no message waits behind the switch.
        if !admission::enabled(&tx).await? {
            return Err(AuthorityError::ModelExecutionOff);
        }
        // Counted under the account lock after the same-ID lookup: a resend
        // never counts twice and concurrent sends serialize here. The input
        // records the exact instant it was counted against.
        let admitted_at =
            admission::allowance(&tx, account.id, &input.timezone, admission.limit).await?;
        tx.execute_raw(statement(
            Query::insert()
                .into_table(S::Table)
                .columns([S::AccountId])
                .values_panic([account.id.into()])
                .on_conflict(OnConflict::new().do_nothing().to_owned()),
        ))
        .await?;
        // Invoke the existing PostgreSQL trigger-owned transition; duplicating
        // its body here would split the acceptance and consent interruption owner.
        tx.execute_raw(sql(
            "SELECT duet_interrupt_accepted($1)",
            vec![account.id.into()],
        ))
        .await?;
        let row = tx
            .query_one_raw(statement(
                Query::update()
                    .table(S::Table)
                    .value(S::Revision, Expr::col(S::Revision).add(1))
                    .value(S::NextSequence, Expr::col(S::NextSequence).add(2))
                    .and_where(Expr::col(S::AccountId).eq(account.id))
                    .returning(Query::returning().columns([S::Revision, S::NextSequence])),
            ))
            .await?
            .ok_or(AuthorityError::Refused)?;
        let revision: i64 = row.try_get("", "revision")?;
        let sequence = row.try_get::<i64>("", "next_sequence")? - 2;
        let run = Uuid::new_v4();
        let reply = Uuid::new_v4();
        tx.execute_raw(statement(
            Query::insert()
                .into_table(I::Table)
                .columns([
                    I::AccountId,
                    I::MessageId,
                    I::RunId,
                    I::ReplyId,
                    I::Timezone,
                    I::State,
                    I::PermissionRevision,
                    I::Revision,
                    I::AdmittedAt,
                ])
                .values_panic([
                    account.id.into(),
                    input.id.into(),
                    run.into(),
                    reply.into(),
                    input.timezone.clone().into(),
                    "queued".into(),
                    access.permissions.revision.into(),
                    revision.into(),
                    Expr::val(admitted_at).cast_as(Alias::new("timestamptz")),
                ]),
        ))
        .await?;
        tx.execute_raw(statement(
            Query::insert()
                .into_table(M::Table)
                .columns([
                    M::AccountId,
                    M::Id,
                    M::InputId,
                    M::Sequence,
                    M::Revision,
                    M::Role,
                    M::Text,
                    M::State,
                ])
                .values_panic([
                    account.id.into(),
                    input.id.into(),
                    input.id.into(),
                    sequence.into(),
                    revision.into(),
                    "user".into(),
                    input.text.clone().into(),
                    "complete".into(),
                ])
                .values_panic([
                    account.id.into(),
                    reply.into(),
                    input.id.into(),
                    (sequence + 1).into(),
                    revision.into(),
                    "assistant".into(),
                    "".into(),
                    "pending".into(),
                ]),
        ))
        .await?;
        tx.execute_raw(statement(
            Query::update()
                .table(I::Table)
                .values([
                    (I::OriginProject, principal.project.clone().into()),
                    (I::OriginSubject, principal.subject.clone().into()),
                    (I::OriginAuthenticatedAt, principal.authenticated_at.into()),
                ])
                .and_where(Expr::col(I::AccountId).eq(account.id))
                .and_where(Expr::col(I::MessageId).eq(input.id)),
        ))
        .await?;
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
        let mut page = Query::select();
        page.column(sea_orm::sea_query::Asterisk)
            .from(M::Table)
            .and_where(Expr::col(M::AccountId).eq(account.id))
            .order_by(M::Sequence, Order::Desc)
            .limit(limit);
        if let Some(before) = before {
            page.and_where(Expr::col(M::Sequence).lt(before));
        }
        let rows = tx
            .query_all_raw(statement(
                Query::select()
                    .column(sea_orm::sea_query::Asterisk)
                    .from_subquery(page, Alias::new("page"))
                    .order_by(M::Sequence, Order::Asc),
            ))
            .await?;
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
/// Operator switch, allowance and attested admission capability.
pub mod admission;
/// Duet-owned model context projection for one claimed attempt.
pub mod context;
/// Durable Duet claims, acknowledged events and transactional completion.
pub mod execution;
#[cfg(test)]
mod tests;
