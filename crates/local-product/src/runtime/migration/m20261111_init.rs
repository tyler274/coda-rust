//! The runtime's first schema. The SQL lives in `runtime.sql`; it takes no
//! values, so nothing is spliced in.
use sea_orm_migration::prelude::*;

/// Creates every runtime table.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("../../runtime.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "the physician runtime's records can't be rolled back".into(),
        ))
    }
}
