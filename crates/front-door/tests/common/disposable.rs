//! Throwaway databases and schemas for front-door's Postgres tests.
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

/// Creates the database `name`.
pub async fn create_database(admin: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await
        .expect("create test database");
}

/// Drops the database `name`, closing any connection still open to it.
pub async fn drop_database(admin: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await
        .expect("drop test database");
}

/// Creates the schema `name`.
pub async fn create_schema(db: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    db.execute_unprepared(&format!("create schema {name}"))
        .await
        .expect("create test schema");
}

/// Points this connection's search path at the schema `name`.
pub async fn use_schema(db: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    db.execute_unprepared(&format!("set search_path to {name}"))
        .await
        .expect("set search path");
}

/// Drops the schema `name` and everything in it.
pub async fn drop_schema(db: &impl ConnectionTrait, name: &str) {
    let name = identifier(name);
    db.execute_unprepared(&format!("drop schema {name} cascade"))
        .await
        .expect("drop test schema");
}
