//! Adds release-generation fencing, rollback halt state, and Spruce channel
//! sync configuration columns to `physician_runtime_spruce_channels`.

use super::super::spruce_matching::entities::spruce_channel::{
    Column as ChannelColumn, Entity as ChannelEntity,
};
use sea_orm_migration::prelude::*;

/// Extends `physician_runtime_spruce_channels` with durable release-generation
/// fencing (`highest_release_generation`, `highest_release_label`), persistent
/// `rollback_halt`, and optional persisted Spruce channel config.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ChannelEntity)
                    .add_column(
                        ColumnDef::new(ChannelColumn::HighestReleaseGeneration)
                            .big_integer()
                            .not_null()
                            .default(1)
                            .check(Expr::col(ChannelColumn::HighestReleaseGeneration).gte(1)),
                    )
                    .add_column(
                        ColumnDef::new(ChannelColumn::HighestReleaseLabel)
                            .text()
                            .not_null()
                            .default("spruce-sync/1"),
                    )
                    .add_column(ColumnDef::new(ChannelColumn::RollbackHalt).json_binary())
                    .add_column(ColumnDef::new(ChannelColumn::ChannelConfig).json_binary())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ChannelEntity)
                    .drop_column(ChannelColumn::ChannelConfig)
                    .drop_column(ChannelColumn::RollbackHalt)
                    .drop_column(ChannelColumn::HighestReleaseLabel)
                    .drop_column(ChannelColumn::HighestReleaseGeneration)
                    .to_owned(),
            )
            .await
    }
}
