//! Transactional Luna restriction outbox. Existing pending restrictions remain
//! blocked: their historical event identity/time cannot be invented by migration.
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE front_door_luna_restrictions (
                id uuid PRIMARY KEY,
                account_id uuid NOT NULL,
                revision bigint NOT NULL CHECK (revision > 0),
                occurred_at bigint NOT NULL,
                confirmed boolean NOT NULL DEFAULT false,
                lease uuid,
                due_at bigint NOT NULL DEFAULT 0,
                attempts integer NOT NULL DEFAULT 0,
                UNIQUE(account_id, revision)
            );
            CREATE INDEX front_door_luna_restrictions_due
              ON front_door_luna_restrictions(account_id, due_at) WHERE NOT confirmed;",
            )
            .await?;
        Ok(())
    }
    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "Privacy receipts cannot be rolled back".into(),
        ))
    }
}
