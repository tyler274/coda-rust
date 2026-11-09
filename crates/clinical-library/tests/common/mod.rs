//! The scratch Postgres schema the database-backed tests share.
//!
//! Each test runs in its own schema of the database at
//! `CLINICAL_LIBRARY_TEST_DATABASE_URL`, which must be local. A test never
//! skips silently: without the variable it fails.

// Each test file uses its own part of this module.
#![allow(dead_code)]

use clinical_library::Store;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement, Value,
};

pub struct Scratch {
    pub db: DatabaseConnection,
    schema: String,
}

impl Scratch {
    pub async fn new() -> Self {
        let url = std::env::var("CLINICAL_LIBRARY_TEST_DATABASE_URL")
            .expect("CLINICAL_LIBRARY_TEST_DATABASE_URL must name a local Postgres");
        assert!(url.contains("localhost") || url.contains("127.0.0.1"));
        // Postgres can't bind an identifier as a parameter, so the schema's
        // name is spliced in; it is generated here from hex digits only.
        let schema = format!("clinical_proof_{}", uuid::Uuid::new_v4().simple());
        let admin = Database::connect(&url).await.unwrap();
        admin
            .execute_unprepared(&format!("create schema {schema}"))
            .await
            .unwrap();
        // One connection, so the schema's search_path holds for every query.
        let mut options = ConnectOptions::new(url);
        options.max_connections(1).min_connections(1);
        let db = Database::connect(options).await.unwrap();
        db.execute_unprepared(&format!("set search_path to {schema}"))
            .await
            .unwrap();
        Self { db, schema }
    }

    pub async fn store(&self) -> Store {
        let store = Store::from_connection(self.db.clone());
        store.migrate().await.unwrap();
        store
    }

    pub async fn refused(&self, sql: &str) -> String {
        self.refused_with(sql, []).await
    }

    /// Runs `sql` with its values bound, expecting the database to refuse it.
    pub async fn refused_with(&self, sql: &str, values: impl IntoIterator<Item = Value>) -> String {
        let statement = Statement::from_sql_and_values(DbBackend::Postgres, sql, values);
        match self.db.execute_raw(statement).await {
            Ok(_) => panic!("the database accepted: {sql}"),
            Err(error) => error.to_string(),
        }
    }

    pub async fn drop(self) {
        self.db
            .execute_unprepared(&format!("drop schema {} cascade", self.schema))
            .await
            .unwrap();
    }
}
