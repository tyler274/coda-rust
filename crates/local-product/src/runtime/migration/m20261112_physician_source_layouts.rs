//! Creates the immutable `(source_id, record_type)` layout table from its
//! owning SeaORM entity (`crate::physician_layouts::entity`).
use sea_orm_migration::prelude::*;

/// Adds the `physician_source_layouts` table.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::physician_layouts::migrate(manager.get_connection()).await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "the physician runtime's records can't be rolled back".into(),
        ))
    }
}
