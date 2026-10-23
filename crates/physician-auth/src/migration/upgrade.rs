//! Non-destructive adoption of the old file-derived `mod` migration receipt.
//! The shared legacy ledger is never renamed/deleted: front-door may own it too.
use super::Migrator;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, TransactionTrait,
    sea_query::{Expr, Func, Query},
};
use sea_orm_migration::{MigratorTrait, SchemaManager, seaql_migrations};

/// Serializes startup, verifies old schema identity, adopts it into this owner's
/// ledger, and runs missing migrations in the same transaction.
pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    // PostgreSQL transaction-scoped advisory lock serializes adoption/install;
    // row locks cannot protect a migration ledger that does not yet exist.
    txn.query_one(
        &Query::select()
            .expr(Func::cust("pg_advisory_xact_lock").arg(740146001_i64))
            .to_owned(),
    )
    .await?;
    Migrator::install(&txn).await?;
    let manager = SchemaManager::new(&txn);
    let ledger_is_empty = txn
        .query_one(
            &Query::select()
                .column(seaql_migrations::Column::Version)
                .from(Migrator::migration_table_name())
                .limit(1)
                .to_owned(),
        )
        .await?
        .is_none();
    if ledger_is_empty {
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
            let legacy = seaql_migrations::Entity::find_by_id("mod")
                .one(&txn)
                .await?
                .ok_or_else(|| {
                    DbErr::Custom("unidentified physician schema: no legacy mod receipt".into())
                })?;
            let applied = legacy.applied_at;
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
            for (table, index) in [
                ("physician_accounts", "physician_accounts_practice_idx"),
                ("physician_sessions", "physician_sessions_account_idx"),
                (
                    "physician_sign_in_tokens",
                    "physician_sign_in_tokens_account_idx",
                ),
                (
                    "physician_memberships",
                    "physician_memberships_workspace_idx",
                ),
                (
                    "physician_patient_links",
                    "physician_patient_links_workspace_idx",
                ),
            ] {
                if !manager.has_index(table, index).await? {
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
                if !manager
                    .has_index("physician_patients", "physician_patients_practice_idx")
                    .await?
                {
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
    db.execute(
        &Query::insert()
            .into_table(Migrator::migration_table_name())
            .columns([
                seaql_migrations::Column::Version,
                seaql_migrations::Column::AppliedAt,
            ])
            .values_panic([Expr::val(version), Expr::val(applied)])
            .to_owned(),
    )
    .await?;
    Ok(())
}
