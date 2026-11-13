//! Saved per-source refresh status and atomic per-source once-only refresh receipts.
use super::super::entities::{
    source_refresh_receipt::{Column as ReceiptColumn, Entity as ReceiptEntity},
    source_status::{Column as StatusColumn, Entity as StatusEntity},
};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_source_statuses` and
/// `physician_runtime_source_refresh_receipts`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StatusEntity)
                    .col(ColumnDef::new(StatusColumn::Practice).text().not_null())
                    .col(ColumnDef::new(StatusColumn::Workspace).text().not_null())
                    .col(ColumnDef::new(StatusColumn::SourceId).text().not_null())
                    .col(ColumnDef::new(StatusColumn::SourceLabel).text().not_null())
                    .col(
                        ColumnDef::new(StatusColumn::Outcome)
                            .text()
                            .not_null()
                            .check(Expr::col(StatusColumn::Outcome).is_in([
                                "updated",
                                "partly_updated",
                                "failed",
                            ])),
                    )
                    .col(
                        ColumnDef::new(StatusColumn::CheckedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StatusColumn::LastGoodPulledAt).big_integer())
                    .col(
                        ColumnDef::new(StatusColumn::WorkspaceVersion)
                            .big_integer()
                            .check(
                                Expr::col(StatusColumn::WorkspaceVersion)
                                    .is_null()
                                    .or(Expr::col(StatusColumn::WorkspaceVersion).gte(0)),
                            ),
                    )
                    .col(
                        ColumnDef::new(StatusColumn::ChangedRecords)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .check(Expr::col(StatusColumn::ChangedRecords).gte(0)),
                    )
                    .col(ColumnDef::new(StatusColumn::Reason).text())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_source_statuses")
                            .col(StatusColumn::Practice)
                            .col(StatusColumn::Workspace)
                            .col(StatusColumn::SourceId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_source_statuses-workspace")
                    .table(StatusEntity)
                    .col(StatusColumn::Practice)
                    .col(StatusColumn::Workspace)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ReceiptEntity)
                    .col(ColumnDef::new(ReceiptColumn::Practice).text().not_null())
                    .col(ColumnDef::new(ReceiptColumn::Actor).text().not_null())
                    .col(ColumnDef::new(ReceiptColumn::Workspace).text().not_null())
                    .col(ColumnDef::new(ReceiptColumn::CommandKey).text().not_null())
                    .col(ColumnDef::new(ReceiptColumn::SourceId).text().not_null())
                    .col(
                        ColumnDef::new(ReceiptColumn::Ordinal)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(ReceiptColumn::Ordinal).gte(0)),
                    )
                    .col(
                        ColumnDef::new(ReceiptColumn::RequestSha256)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ReceiptColumn::WorkspaceVersionAfter)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(ReceiptColumn::WorkspaceVersionAfter).gte(0)),
                    )
                    .col(
                        ColumnDef::new(ReceiptColumn::Status)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ReceiptColumn::RecordedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_source_refresh_receipts")
                            .col(ReceiptColumn::Practice)
                            .col(ReceiptColumn::Actor)
                            .col(ReceiptColumn::Workspace)
                            .col(ReceiptColumn::CommandKey)
                            .col(ReceiptColumn::SourceId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_source_refresh_receipts-command")
                    .table(ReceiptEntity)
                    .col(ReceiptColumn::Practice)
                    .col(ReceiptColumn::Actor)
                    .col(ReceiptColumn::Workspace)
                    .col(ReceiptColumn::CommandKey)
                    .col(ReceiptColumn::Ordinal)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReceiptEntity).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(StatusEntity).if_exists().to_owned())
            .await
    }
}
