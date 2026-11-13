//! Additive schema for durable reconnect command execution and shared source
//! adapter state across instances.

use sea_orm_migration::prelude::*;

use super::super::entities::{reconnect_command, source_adapter_state};

#[derive(DeriveMigrationName)]
pub(in crate::runtime) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(reconnect_command::Entity)
                    .col(
                        ColumnDef::new(reconnect_command::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reconnect_command::Column::Physician)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reconnect_command::Column::Key)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reconnect_command::Column::SourceId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reconnect_command::Column::State)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(reconnect_command::Column::LeaseOwner).text())
                    .col(ColumnDef::new(reconnect_command::Column::LeaseUntil).big_integer())
                    .col(
                        ColumnDef::new(reconnect_command::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(reconnect_command::Column::Practice)
                            .col(reconnect_command::Column::Physician)
                            .col(reconnect_command::Column::Key),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(source_adapter_state::Entity)
                    .col(
                        ColumnDef::new(source_adapter_state::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(source_adapter_state::Column::SourceId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(source_adapter_state::Column::State)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(source_adapter_state::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(source_adapter_state::Column::Practice)
                            .col(source_adapter_state::Column::SourceId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "the physician runtime's records can't be rolled back".into(),
        ))
    }
}
