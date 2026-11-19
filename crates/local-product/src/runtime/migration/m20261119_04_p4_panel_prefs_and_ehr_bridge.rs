//! Per-physician panel collapse preferences and Hint Chrome connector bridge tables.
use super::super::hint_bridge::tables::{
    hint_capture::{Column as CaptureCol, Entity as CaptureEntity},
    hint_job::{Column as JobCol, Entity as JobEntity},
    hint_pairing::{Column as PairingCol, Entity as PairingEntity},
    hint_schedule::{Column as ScheduleCol, Entity as ScheduleEntity},
};
use super::super::panel_preferences::table::{Column as PanelCol, Entity as PanelEntity};
use sea_orm::EntityName;
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_panel_preferences`,
/// `physician_runtime_hint_pairings`, `physician_runtime_hint_captures`,
/// `physician_runtime_hint_schedules`, and `physician_runtime_hint_jobs`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PanelEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PanelCol::Practice)
                            .text()
                            .not_null()
                            .check(Expr::col(PanelCol::Practice).ne("")),
                    )
                    .col(
                        ColumnDef::new(PanelCol::Physician)
                            .text()
                            .not_null()
                            .check(Expr::col(PanelCol::Physician).ne("")),
                    )
                    .col(
                        ColumnDef::new(PanelCol::SidebarCollapsed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PanelCol::ScheduleCollapsed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(PanelCol::UpdatedAt).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_panel_preferences")
                            .col(PanelCol::Practice)
                            .col(PanelCol::Physician),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PairingEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PairingCol::Practice)
                            .text()
                            .not_null()
                            .check(Expr::col(PairingCol::Practice).ne("")),
                    )
                    .col(
                        ColumnDef::new(PairingCol::PairId)
                            .text()
                            .not_null()
                            .check(Expr::col(PairingCol::PairId).ne("")),
                    )
                    .col(
                        ColumnDef::new(PairingCol::PairKeyHash)
                            .text()
                            .not_null()
                            .check(Expr::col(PairingCol::PairKeyHash).ne("")),
                    )
                    .col(
                        ColumnDef::new(PairingCol::SourceLabel)
                            .text()
                            .not_null()
                            .check(Expr::col(PairingCol::SourceLabel).ne("")),
                    )
                    .col(
                        ColumnDef::new(PairingCol::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PairingCol::RevokedAt).big_integer())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_hint_pairings")
                            .col(PairingCol::Practice)
                            .col(PairingCol::PairId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CaptureEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CaptureCol::Practice)
                            .text()
                            .not_null()
                            .check(Expr::col(CaptureCol::Practice).ne("")),
                    )
                    .col(
                        ColumnDef::new(CaptureCol::SourcePatientId)
                            .text()
                            .not_null()
                            .check(Expr::col(CaptureCol::SourcePatientId).ne("")),
                    )
                    .col(
                        ColumnDef::new(CaptureCol::Workspace)
                            .text()
                            .not_null()
                            .check(Expr::col(CaptureCol::Workspace).ne("")),
                    )
                    .col(ColumnDef::new(CaptureCol::CaptureJson).text().not_null())
                    .col(
                        ColumnDef::new(CaptureCol::CapturedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_hint_captures")
                            .col(CaptureCol::Practice)
                            .col(CaptureCol::SourcePatientId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ScheduleEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleCol::Practice)
                            .text()
                            .not_null()
                            .check(Expr::col(ScheduleCol::Practice).ne("")),
                    )
                    .col(
                        ColumnDef::new(ScheduleCol::AppointmentsJson)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ScheduleCol::DeliveredAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_hint_schedules")
                            .col(ScheduleCol::Practice),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(JobEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JobCol::Practice)
                            .text()
                            .not_null()
                            .check(Expr::col(JobCol::Practice).ne("")),
                    )
                    .col(
                        ColumnDef::new(JobCol::JobId)
                            .text()
                            .not_null()
                            .check(Expr::col(JobCol::JobId).ne("")),
                    )
                    .col(ColumnDef::new(JobCol::ReadKind).text().not_null().check(
                        Expr::col(JobCol::ReadKind).is_in([
                            "roster",
                            "appointments",
                            "chart_capture",
                        ]),
                    ))
                    .col(ColumnDef::new(JobCol::PatientId).text())
                    .col(ColumnDef::new(JobCol::State).text().not_null().check(
                        Expr::col(JobCol::State).is_in([
                            "pending",
                            "leased",
                            "completed",
                            "failed",
                        ]),
                    ))
                    .col(ColumnDef::new(JobCol::LeaseId).text())
                    .col(ColumnDef::new(JobCol::LeasedUntil).big_integer())
                    .col(ColumnDef::new(JobCol::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(JobCol::UpdatedAt).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_hint_jobs")
                            .col(JobCol::Practice)
                            .col(JobCol::JobId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in [
            JobEntity.table_ref(),
            ScheduleEntity.table_ref(),
            CaptureEntity.table_ref(),
            PairingEntity.table_ref(),
            PanelEntity.table_ref(),
        ] {
            manager
                .drop_table(Table::drop().table(table).if_exists().to_owned())
                .await?;
        }
        Ok(())
    }
}
