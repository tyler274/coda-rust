//! Throwaway databases and schemas for physician-auth's Postgres tests.
//!
//! Postgres binds values, not names, so each statement here takes its name as
//! an identifier the query builder quoted. Each test chooses its own name.
#![allow(
    dead_code,
    reason = "each test binary uses its own part of this module"
)]

use sea_orm::ConnectionTrait;
use sea_orm::sea_query::{PostgresQueryBuilder, QuotedBuilder};

/// `name` as a quoted Postgres identifier.
fn identifier(name: &str) -> String {
    let mut quoted = String::new();
    PostgresQueryBuilder.prepare_iden(&name.to_owned().into(), &mut quoted);
    quoted
}

/// Drops the database `name` if an earlier run left it, then creates it empty.
pub async fn replace_database(admin: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    admin
        .execute_unprepared(&format!("DROP DATABASE IF EXISTS {name}"))
        .await
        .expect("drop stale test database");
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await
        .expect("create test database");
}

/// Creates the schema `name`.
pub async fn create_schema(db: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    db.execute_unprepared(&format!("CREATE SCHEMA {name}"))
        .await
        .expect("create test schema");
}

/// Drops the schema `name` and everything in it.
pub async fn drop_schema(db: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    db.execute_unprepared(&format!("DROP SCHEMA {name} CASCADE"))
        .await
        .expect("drop test schema");
}
