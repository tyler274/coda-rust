//! Durable patient memory: one head row per practice and patient workspace and
//! one versioned snapshot row per accepted memory mutation receipt.

use super::super::entities::{
    patient_memory::{Column as HeadColumn, Entity as HeadEntity},
    patient_memory_version::{Column as VersionColumn, Entity as VersionEntity},
};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_patient_memories` and
/// `physician_runtime_patient_memory_versions`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let head_text = |column: HeadColumn| ColumnDef::new(column).text().not_null().to_owned();
        manager
            .create_table(
                Table::create()
                    .table(HeadEntity)
                    .col(head_text(HeadColumn::Practice))
                    .col(head_text(HeadColumn::Workspace))
                    .col(head_text(HeadColumn::MemoryId))
                    .col(
                        ColumnDef::new(HeadColumn::LatestVersion)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(HeadColumn::LatestVersion).gt(0)),
                    )
                    .col(ColumnDef::new(HeadColumn::CurrentVersion).big_integer())
                    .col(ColumnDef::new(HeadColumn::LatestSummaryVersion).big_integer())
                    .col(
                        ColumnDef::new(HeadColumn::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HeadColumn::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .check(
                        Expr::col(HeadColumn::CurrentVersion)
                            .is_null()
                            .or(Expr::col(HeadColumn::CurrentVersion).gt(0).and(
                                Expr::col(HeadColumn::CurrentVersion)
                                    .lte(Expr::col(HeadColumn::LatestVersion)),
                            )),
                    )
                    .check(
                        Expr::col(HeadColumn::LatestSummaryVersion)
                            .is_null()
                            .or(Expr::col(HeadColumn::LatestSummaryVersion).gt(0).and(
                                Expr::col(HeadColumn::LatestSummaryVersion)
                                    .lte(Expr::col(HeadColumn::LatestVersion)),
                            )),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_patient_memories")
                            .col(HeadColumn::Practice)
                            .col(HeadColumn::Workspace),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_patient_memories-memory_id")
                    .table(HeadEntity)
                    .col(HeadColumn::MemoryId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        let version_text =
            |column: VersionColumn| ColumnDef::new(column).text().not_null().to_owned();
        manager
            .create_table(
                Table::create()
                    .table(VersionEntity)
                    .col(version_text(VersionColumn::Practice))
                    .col(version_text(VersionColumn::Workspace))
                    .col(
                        ColumnDef::new(VersionColumn::Version)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(VersionColumn::Version).gt(0)),
                    )
                    .col(version_text(VersionColumn::MemoryId))
                    .col(version_text(VersionColumn::ReceiptPhysician))
                    .col(version_text(VersionColumn::ReceiptKey))
                    .col(
                        ColumnDef::new(VersionColumn::Author)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(VersionColumn::ChangeKind)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(VersionColumn::Visibility)
                            .text()
                            .not_null()
                            .check(
                                Expr::col(VersionColumn::Visibility)
                                    .is_in(["permitted", "retracted"]),
                            ),
                    )
                    .col(ColumnDef::new(VersionColumn::Retraction).json_binary())
                    .col(
                        ColumnDef::new(VersionColumn::OperatingIndex)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(VersionColumn::Corrections)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(VersionColumn::Summary).json_binary())
                    .col(ColumnDef::new(VersionColumn::SummaryVersion).big_integer())
                    .col(
                        ColumnDef::new(VersionColumn::SavedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .check(
                        Expr::col(VersionColumn::Visibility)
                            .eq("permitted")
                            .and(Expr::col(VersionColumn::Retraction).is_null())
                            .or(Expr::col(VersionColumn::Visibility)
                                .eq("retracted")
                                .and(Expr::col(VersionColumn::Retraction).is_not_null())),
                    )
                    .check(
                        Expr::col(VersionColumn::Summary)
                            .is_null()
                            .and(Expr::col(VersionColumn::SummaryVersion).is_null())
                            .or(Expr::col(VersionColumn::Summary)
                                .is_not_null()
                                .and(Expr::col(VersionColumn::SummaryVersion).is_not_null())
                                .and(Expr::col(VersionColumn::SummaryVersion).gt(0))
                                .and(
                                    Expr::col(VersionColumn::SummaryVersion)
                                        .lte(Expr::col(VersionColumn::Version)),
                                )),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_patient_memory_versions")
                            .col(VersionColumn::Practice)
                            .col(VersionColumn::Workspace)
                            .col(VersionColumn::Version),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physician_runtime_patient_memory_versions-head")
                            .from(
                                VersionEntity,
                                (VersionColumn::Practice, VersionColumn::Workspace),
                            )
                            .to(HeadEntity, (HeadColumn::Practice, HeadColumn::Workspace)),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_patient_memory_versions-receipt")
                    .table(VersionEntity)
                    .col(VersionColumn::Practice)
                    .col(VersionColumn::ReceiptPhysician)
                    .col(VersionColumn::ReceiptKey)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "patient memory is durable clinical working state and can't be rolled back".into(),
        ))
    }
}
