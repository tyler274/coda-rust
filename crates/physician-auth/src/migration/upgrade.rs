//! Non-destructive adoption of the old file-derived `mod` migration receipt.
//! The shared legacy ledger is never renamed/deleted: front-door may own it too.
use super::Migrator;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, Statement, TransactionTrait};
use sea_orm_migration::{MigratorTrait, SchemaManager};

/// Serializes startup, verifies old schema identity, adopts it into this owner's
/// ledger, and runs missing migrations in the same transaction.
pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    txn.execute_unprepared("SELECT pg_advisory_xact_lock(740146001)")
        .await?;
    Migrator::install(&txn).await?;
    let manager = SchemaManager::new(&txn);
    let count = txn
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT count(*) AS n FROM physician_auth_migrations".to_owned(),
        ))
        .await?
        .unwrap()
        .try_get::<i64>("", "n")?;
    if count == 0 {
        let tables = [
            (
                "physician_accounts",
                "id,practice_id,physician_id,email,display_name,active,admin,sessions_ended_before,created_at",
            ),
            (
                "physician_sessions",
                "id,account_id,token_hash,created_at,expires_at",
            ),
            (
                "physician_sign_in_tokens",
                "id,account_id,token_hash,created_at,expires_at",
            ),
            (
                "physician_invitations",
                "id,practice_id,email,display_name,admin,token_hash,created_at,expires_at",
            ),
            (
                "physician_memberships",
                "id,practice_id,physician_id,patient_workspace_id,allowed,created_at",
            ),
            (
                "physician_patient_links",
                "id,practice_id,patient_workspace_id,pending_token_hash,pending_expires_at,duet_account_id,linked,sharing_consented,consent_version,consented_at,revoked_at,cadence,created_at",
            ),
        ];
        let mut present = 0;
        for (table, _) in tables {
            if manager.has_table(table).await? {
                present += 1;
            }
        }
        if present != 0 {
            if present != tables.len() || !manager.has_table("seaql_migrations").await? {
                return Err(DbErr::Custom(
                    "partial physician legacy schema; repair required without resetting data"
                        .into(),
                ));
            }
            let legacy = txn
                .query_one_raw(Statement::from_string(
                    DbBackend::Postgres,
                    "SELECT applied_at FROM seaql_migrations WHERE version='mod'".to_owned(),
                ))
                .await?
                .ok_or_else(|| {
                    DbErr::Custom("unidentified physician schema: no legacy mod receipt".into())
                })?;
            let applied: i64 = legacy.try_get("", "applied_at")?;
            for (table, columns) in tables {
                for column in columns.split(',') {
                    if !manager.has_column(table, column).await? {
                        return Err(DbErr::Custom(format!(
                            "legacy physician schema missing {table}.{column}"
                        )));
                    }
                }
            }
            // Check the original named indexes as well as columns; mere table
            // existence is not enough to recognize a completed legacy migration.
            for index in [
                "physician_accounts_practice_idx",
                "physician_sessions_account_idx",
                "physician_sign_in_tokens_account_idx",
                "physician_memberships_workspace_idx",
                "physician_patient_links_workspace_idx",
            ] {
                let exists = txn.query_one_raw(Statement::from_sql_and_values(DbBackend::Postgres,
                    "SELECT indexname FROM pg_indexes WHERE schemaname=current_schema() AND indexname=$1", vec![index.into()])).await?;
                if exists.is_none() {
                    return Err(DbErr::Custom(format!(
                        "legacy physician schema missing index {index}"
                    )));
                }
            }
            adopt(&txn, "physician_auth_20260101_000001_init", applied).await?;
            if manager.has_table("physician_patients").await? {
                for column in [
                    "patient_workspace_id",
                    "practice_id",
                    "cadence",
                    "created_at",
                ] {
                    if !manager.has_column("physician_patients", column).await? {
                        return Err(DbErr::Custom("partial physician patients migration".into()));
                    }
                }
                let index = txn.query_one_raw(Statement::from_string(DbBackend::Postgres,
                    "SELECT indexname FROM pg_indexes WHERE schemaname=current_schema() AND indexname='physician_patients_practice_idx'".to_owned())).await?;
                if index.is_none() {
                    return Err(DbErr::Custom(
                        "partial physician patients migration: missing index".into(),
                    ));
                }
                adopt(
                    &txn,
                    "physician_auth_20260101_000002_practice_patients",
                    applied,
                )
                .await?;
            }
        } else if manager.has_table("physician_patients").await? {
            return Err(DbErr::Custom("orphan physician patients schema".into()));
        }
    }
    Migrator::up(&txn, None).await?;
    txn.commit().await
}
async fn adopt(db: &impl ConnectionTrait, version: &str, applied: i64) -> Result<(), DbErr> {
    db.execute_raw(Statement::from_sql_and_values(
        DbBackend::Postgres,
        "INSERT INTO physician_auth_migrations(version,applied_at) VALUES($1,$2)",
        vec![version.into(), applied.into()],
    ))
    .await?;
    Ok(())
}
