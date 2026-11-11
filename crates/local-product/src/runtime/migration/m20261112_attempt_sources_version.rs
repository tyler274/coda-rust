//! Records the workspace source version each chat attempt was claimed at, so
//! an answer can say when records arrived after it. Attempts saved before this
//! migration keep no version and are never marked.
use sea_orm_migration::prelude::*;

/// Adds the nullable `sources_version` column to chat attempts.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[derive(DeriveIden)]
enum PhysicianRuntimeChatAttempts {
    Table,
    SourcesVersion,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PhysicianRuntimeChatAttempts::Table)
                    .add_column(
                        ColumnDef::new(PhysicianRuntimeChatAttempts::SourcesVersion)
                            .big_integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "the physician runtime's records can't be rolled back".into(),
        ))
    }
}
