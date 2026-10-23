//! Typed identifiers for the existing Duet-owned conversation schema.
//! These are query mappings, not a new schema or migration history. SeaQuery
//! keeps values bound, including PostgreSQL-only clock/interval expressions.
use sea_orm::sea_query::*;
use sea_orm::{DbBackend, Statement, sea_query};

#[derive(Iden)]
pub(super) enum Input {
    #[iden = "duet_conversation_inputs"]
    Table,
    AccountId,
    MessageId,
    RunId,
    ReplyId,
    Timezone,
    State,
    PermissionRevision,
    Revision,
    CreatedAt,
    OriginProject,
    OriginSubject,
    OriginAuthenticatedAt,
    AttemptCount,
    ClaimToken,
    LeaseUntil,
    RetryDeadline,
    Failure,
    AdmittedAt,
}
#[derive(Iden)]
pub(super) enum Message {
    #[iden = "duet_conversation_messages"]
    Table,
    AccountId,
    Id,
    InputId,
    Sequence,
    Revision,
    Role,
    Text,
    State,
}
#[derive(Iden)]
pub(super) enum State {
    #[iden = "duet_conversation_state"]
    Table,
    AccountId,
    Revision,
    NextSequence,
}
#[derive(Iden)]
pub(super) enum Attempt {
    #[iden = "duet_conversation_attempts"]
    Table,
    Id,
    RunId,
    Ordinal,
    ExecutionStarted,
    State,
}
#[derive(Iden)]
pub(super) enum Event {
    #[iden = "duet_attempt_events"]
    Table,
    AttemptId,
    Sequence,
    Body,
    Delta,
    Candidate,
}
#[derive(Iden)]
pub(super) enum Switch {
    #[iden = "duet_model_execution_switch"]
    Table,
    Id,
    Enabled,
    ChangedBy,
    Reason,
}

pub(super) fn statement(query: &impl sea_orm::StatementBuilder) -> Statement {
    DbBackend::Postgres.build(query)
}

/// PostgreSQL wall clock is deliberate: transaction-start time is stale after
/// waiting for the account lock and must never extend an expired lease.
pub(super) fn clock() -> Expr {
    Func::cust(Alias::new("clock_timestamp")).into()
}

pub(super) fn run_update(run: uuid::Uuid) -> UpdateStatement {
    Query::update()
        .table(Input::Table)
        .and_where(Expr::col(Input::RunId).eq(run))
        .to_owned()
}

pub(super) fn running_attempts(run: uuid::Uuid, state: &str) -> UpdateStatement {
    Query::update()
        .table(Attempt::Table)
        .value(Attempt::State, state)
        .and_where(Expr::col(Attempt::RunId).eq(run))
        .and_where(Expr::col(Attempt::State).eq("running"))
        .to_owned()
}

pub(super) fn reply_update(account: uuid::Uuid, run: uuid::Uuid) -> UpdateStatement {
    Query::update()
        .table(Message::Table)
        .and_where(Expr::col(Message::AccountId).eq(account))
        .and_where(
            Expr::col(Message::Id).eq(Query::select()
                .column(Input::ReplyId)
                .from(Input::Table)
                .and_where(Expr::col(Input::RunId).eq(run))
                .to_owned()),
        )
        .to_owned()
}

pub(super) fn revision_update(account: uuid::Uuid) -> UpdateStatement {
    Query::update()
        .table(State::Table)
        .value(State::Revision, Expr::col(State::Revision).add(1))
        .and_where(Expr::col(State::AccountId).eq(account))
        .returning(Query::returning().column(State::Revision))
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reply_update_keeps_account_scope_and_scalar_run_lookup() {
        let account = uuid::Uuid::new_v4();
        let run = uuid::Uuid::new_v4();
        let sql = statement(
            reply_update(account, run).value(Message::Text, "' ; DROP TABLE accounts --"),
        );
        assert_eq!(
            sql.sql,
            "UPDATE \"duet_conversation_messages\" SET \"text\" = $1 WHERE \"account_id\" = $2 AND \"id\" = (SELECT \"reply_id\" FROM \"duet_conversation_inputs\" WHERE \"run_id\" = $3)"
        );
        assert_eq!(
            sql.values.unwrap().0,
            vec![
                "' ; DROP TABLE accounts --".into(),
                account.into(),
                run.into()
            ]
        );
    }

    #[test]
    fn revision_is_atomic_and_returns_the_written_value() {
        let account = uuid::Uuid::new_v4();
        let sql = statement(&revision_update(account));
        assert_eq!(
            sql.sql,
            "UPDATE \"duet_conversation_state\" SET \"revision\" = \"revision\" + $1 WHERE \"account_id\" = $2 RETURNING \"revision\""
        );
        assert_eq!(sql.values.unwrap().0, vec![1i32.into(), account.into()]);
    }

    #[test]
    fn clearing_a_lease_uses_sql_null_not_a_text_parameter() {
        let run = uuid::Uuid::new_v4();
        let sql = statement(run_update(run).value(Input::LeaseUntil, Expr::Keyword(Keyword::Null)));
        assert_eq!(
            sql.sql,
            "UPDATE \"duet_conversation_inputs\" SET \"lease_until\" = NULL WHERE \"run_id\" = $1"
        );
        assert_eq!(sql.values.unwrap().0, vec![run.into()]);
    }

    #[test]
    fn attempt_transition_only_changes_running_attempts() {
        let run = uuid::Uuid::new_v4();
        let sql = statement(&running_attempts(run, "expired"));
        assert_eq!(
            sql.sql,
            "UPDATE \"duet_conversation_attempts\" SET \"state\" = $1 WHERE \"run_id\" = $2 AND \"state\" = $3"
        );
        assert_eq!(
            sql.values.unwrap().0,
            vec!["expired".into(), run.into(), "running".into()]
        );
    }
}
