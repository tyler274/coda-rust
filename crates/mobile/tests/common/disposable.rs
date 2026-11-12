//! Throwaway databases and login roles for mobile's durable-service tests.
//!
//! Postgres binds values, not names, so each statement here takes its name as
//! an identifier the query builder quoted. Each test chooses its own names.
#![expect(
    clippy::disallowed_methods,
    reason = "creates and drops a disposable role and database"
)]

use sea_orm::sea_query::{PostgresQueryBuilder, QuotedBuilder};
use sqlx::PgConnection;

/// `name` as a quoted Postgres identifier.
fn identifier(name: &str) -> String {
    let mut quoted = String::new();
    PostgresQueryBuilder.prepare_iden(&name.to_owned().into(), &mut quoted);
    quoted
}

/// Creates the login role `role`, then the database `name`.
pub async fn create_database_and_role(admin: &mut PgConnection, name: &str, role: &str) {
    let (name, role) = (identifier(name), identifier(role));
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut *admin)
        .await
        .unwrap();
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!("create database {name}")))
        .execute(&mut *admin)
        .await
        .unwrap();
}

/// Drops the database `name`, closing any connection still open to it, then
/// the role `role`.
pub async fn drop_database_and_role(admin: &mut PgConnection, name: &str, role: &str) {
    let (name, role) = (identifier(name), identifier(role));
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!(
        "drop database {name} with (force)"
    )))
    .execute(&mut *admin)
    .await
    .unwrap();
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut *admin)
        .await
        .unwrap();
}
