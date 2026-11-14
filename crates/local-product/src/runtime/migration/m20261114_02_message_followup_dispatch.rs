//! Durable per-patient dispatch trigger ledger for message wake events and
//! due follow-up version watermarks.

use super::super::dispatch::tables::dispatch_trigger;
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_patient_dispatch_triggers`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use dispatch_trigger::Column as C;

        manager
            .create_table(
                Table::create()
                    .table(dispatch_trigger::Entity)
                    .col(
                        ColumnDef::new(C::Practice)
                            .text()
                            .not_null()
                            .check(Expr::col(C::Practice).ne("")),
                    )
                    .col(
                        ColumnDef::new(C::Workspace)
                            .text()
                            .not_null()
                            .check(Expr::col(C::Workspace).ne("")),
                    )
                    .col(
                        ColumnDef::new(C::TriggerKey)
                            .text()
                            .not_null()
                            .check(Expr::col(C::TriggerKey).ne("")),
                    )
                    .col(
                        ColumnDef::new(C::TriggerKind)
                            .text()
                            .not_null()
                            .check(Expr::col(C::TriggerKind).is_in(["message", "follow_up"])),
                    )
                    .col(
                        ColumnDef::new(C::TargetId)
                            .text()
                            .not_null()
                            .check(Expr::col(C::TargetId).ne("")),
                    )
                    .col(ColumnDef::new(C::TargetVersion).big_integer())
                    .col(ColumnDef::new(C::DispatchedAt).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_patient_dispatch_triggers")
                            .col(C::Practice)
                            .col(C::Workspace)
                            .col(C::TriggerKey),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("ix-physician_runtime_patient_dispatch_triggers-workspace")
                    .table(dispatch_trigger::Entity)
                    .col(C::Practice)
                    .col(C::Workspace)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "patient dispatch trigger ledger cannot be rolled back".into(),
        ))
    }
}
