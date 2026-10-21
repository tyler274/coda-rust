//! Non-destructive adoption of the legacy shared file-derived migration receipt.
use super::Migrator;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, Statement, TransactionTrait};
use sea_orm_migration::{MigratorTrait, SchemaManager};

/// SeaORM's default ledger, shared with `physician-auth`, where the init
/// first recorded `mod`.
const LEGACY_SHARED_LEDGER: &str = "seaql_migrations";

/// The interim front-door ledger, where the init also recorded `mod`.
const LEGACY_OWN_LEDGER: &str = "front_door_seaql_migrations";

/// Verify the complete original schema before recording its stable owner receipt.
/// The shared legacy ledger and unrelated co-resident schemas remain untouched.
pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let tx = db.begin().await?;
    tx.execute_unprepared("select pg_advisory_xact_lock(740146002)")
        .await?;
    Migrator::install(&tx).await?;
    let manager = SchemaManager::new(&tx);
    let count = tx
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "select count(*) as n from front_door_migrations".to_owned(),
        ))
        .await?
        .unwrap()
        .try_get::<i64>("", "n")?;
    if count == 0 {
        let tables = [
            (
                "front_door_accounts",
                "id,email,display_name,active,linked_practice_id,linked_patient_id,sessions_ended_before,created_at",
            ),
            (
                "front_door_sessions",
                "id,account_id,token_hash,created_at,expires_at",
            ),
            (
                "front_door_sign_in_tokens",
                "id,account_id,token_hash,created_at,expires_at",
            ),
            (
                "front_door_invitations",
                "id,practice_id,patient_id,email,display_name,token_hash,created_at,expires_at",
            ),
            (
                "front_door_links",
                "account_id,practice_id,patient_id,created_at",
            ),
            (
                "front_door_consents",
                "account_id,consented_at,withdrawn_at",
            ),
            (
                "front_door_premium",
                "patient_key,premium,cadence,created_at",
            ),
        ];
        let mut present = 0;
        for (table, _) in tables {
            if manager.has_table(table).await? {
                present += 1;
            }
        }
        if present > 0 {
            // Two legacy ledgers can hold the init as `mod`: SeaORM's shared
            // default table, and the interim `front_door_seaql_migrations`
            // that briefly gave the chain its own table. The front door's own
            // ledger wins when both exist; neither is renamed or deleted.
            let mut ledgers = Vec::new();
            for table in [LEGACY_OWN_LEDGER, LEGACY_SHARED_LEDGER] {
                if manager.has_table(table).await? {
                    ledgers.push(table);
                }
            }
            if present != tables.len() || ledgers.is_empty() {
                return Err(DbErr::Custom(
                    "partial or unidentified front-door schema; non-destructive repair required"
                        .into(),
                ));
            }
            let mut legacy = None;
            for ledger in ledgers {
                legacy = tx
                    .query_one_raw(Statement::from_string(
                        DbBackend::Postgres,
                        format!("select applied_at from {ledger} where version='mod'"),
                    ))
                    .await?;
                if legacy.is_some() {
                    break;
                }
            }
            let legacy =
                legacy.ok_or_else(|| DbErr::Custom("legacy front-door receipt missing".into()))?;
            let applied: i64 = legacy.try_get("", "applied_at")?;
            for (table, columns) in tables {
                for column in columns.split(',') {
                    if !manager.has_column(table, column).await? {
                        return Err(DbErr::Custom(format!(
                            "legacy schema missing {table}.{column}"
                        )));
                    }
                }
            }
            for index in [
                "front_door_accounts_link_idx",
                "front_door_sessions_account_idx",
                "front_door_sign_in_tokens_account_idx",
                "front_door_invitations_roster_idx",
                "front_door_links_roster_idx",
            ] {
                if tx.query_one_raw(Statement::from_sql_and_values(DbBackend::Postgres,"select indexname from pg_indexes where schemaname=current_schema() and indexname=$1",vec![index.into()])).await?.is_none(){return Err(DbErr::Custom(format!("legacy schema missing index {index}")));}
            }
            tx.execute_raw(Statement::from_sql_and_values(DbBackend::Postgres,"insert into front_door_migrations(version,applied_at) values('front_door_20260101_000001_init',$1)",vec![applied.into()])).await?;
        }
    }
    Migrator::up(&tx, None).await?;
    tx.commit().await
}
