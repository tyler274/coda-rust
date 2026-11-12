//! Practice-owned background jobs and per-patient check/review dispatch state.
use super::super::entities::{
    patient_dispatch::{Column as DispatchColumn, Entity as DispatchEntity},
    practice_job::{Column as JobColumn, Entity as JobEntity},
    practice_job_attempt::{Column as AttemptColumn, Entity as AttemptEntity},
};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_practice_jobs`,
/// `physician_runtime_practice_job_attempts`, and
/// `physician_runtime_patient_dispatch`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let default_cadence = serde_json::json!({
            "check_every_seconds": patient_work::DEFAULT_CHECK_EVERY_SECONDS,
            "review_every_seconds": patient_work::DEFAULT_REVIEW_EVERY_SECONDS,
        });
        let empty_records = serde_json::json!({});

        manager
            .create_table(
                Table::create()
                    .table(JobEntity)
                    .col(ColumnDef::new(JobColumn::Practice).text().not_null())
                    .col(ColumnDef::new(JobColumn::Workspace).text().not_null())
                    .col(ColumnDef::new(JobColumn::JobId).text().not_null())
                    .col(ColumnDef::new(JobColumn::Kind).text().not_null().check(
                        Expr::col(JobColumn::Kind).is_in([
                            "patient_review",
                            "visit_preparation",
                            "initial_understanding",
                        ]),
                    ))
                    .col(ColumnDef::new(JobColumn::Status).text().not_null().check(
                        Expr::col(JobColumn::Status).is_in([
                            "pending",
                            "active",
                            "completed",
                            "held",
                            "failed",
                            "cancelled",
                        ]),
                    ))
                    .col(ColumnDef::new(JobColumn::Work).text().not_null())
                    .col(
                        ColumnDef::new(JobColumn::Revision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(JobColumn::Revision).gt(0)),
                    )
                    .col(
                        ColumnDef::new(JobColumn::Assignment)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobColumn::PreloadedSkills)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(JobColumn::Skills).text().not_null())
                    .col(ColumnDef::new(JobColumn::Binding).json_binary().not_null())
                    .col(ColumnDef::new(JobColumn::Runs).json_binary().not_null())
                    .col(
                        ColumnDef::new(JobColumn::Failures)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(JobColumn::Failures).gte(0)),
                    )
                    .col(
                        ColumnDef::new(JobColumn::AllowanceAfter)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(JobColumn::AllowanceAfter).gte(0)),
                    )
                    .col(
                        ColumnDef::new(JobColumn::SourcesVersion)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(JobColumn::SourcesVersion).gte(0)),
                    )
                    .col(ColumnDef::new(JobColumn::Sources).json_binary().not_null())
                    .col(ColumnDef::new(JobColumn::Result).json_binary())
                    .col(
                        ColumnDef::new(JobColumn::DispatchAfter)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(JobColumn::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobColumn::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_practice_jobs")
                            .col(JobColumn::Practice)
                            .col(JobColumn::Workspace)
                            .col(JobColumn::JobId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_practice_jobs-status")
                    .table(JobEntity)
                    .col(JobColumn::Practice)
                    .col(JobColumn::Status)
                    .col(JobColumn::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AttemptEntity)
                    .col(ColumnDef::new(AttemptColumn::Practice).text().not_null())
                    .col(ColumnDef::new(AttemptColumn::Workspace).text().not_null())
                    .col(ColumnDef::new(AttemptColumn::JobId).text().not_null())
                    .col(ColumnDef::new(AttemptColumn::Id).text().not_null())
                    .col(
                        ColumnDef::new(AttemptColumn::Position)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(AttemptColumn::Position).gte(0)),
                    )
                    .col(ColumnDef::new(AttemptColumn::Predecessor).text())
                    .col(ColumnDef::new(AttemptColumn::Work).text().not_null())
                    .col(
                        ColumnDef::new(AttemptColumn::Revision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(AttemptColumn::Revision).gt(0)),
                    )
                    .col(ColumnDef::new(AttemptColumn::Status).text().not_null())
                    .col(
                        ColumnDef::new(AttemptColumn::Expires)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_practice_job_attempts")
                            .col(AttemptColumn::Practice)
                            .col(AttemptColumn::Workspace)
                            .col(AttemptColumn::JobId)
                            .col(AttemptColumn::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DispatchEntity)
                    .col(ColumnDef::new(DispatchColumn::Practice).text().not_null())
                    .col(ColumnDef::new(DispatchColumn::Workspace).text().not_null())
                    .col(
                        ColumnDef::new(DispatchColumn::Cadence)
                            .json_binary()
                            .not_null()
                            .default(default_cadence),
                    )
                    .col(
                        ColumnDef::new(DispatchColumn::RegisteredAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DispatchColumn::LastAttemptAt).big_integer())
                    .col(ColumnDef::new(DispatchColumn::LastSuccessAt).big_integer())
                    .col(ColumnDef::new(DispatchColumn::LastCheckedVersion).big_integer())
                    .col(
                        ColumnDef::new(DispatchColumn::LastCheckedRecords)
                            .json_binary()
                            .not_null()
                            .default(empty_records),
                    )
                    .col(ColumnDef::new(DispatchColumn::LastReviewedAt).big_integer())
                    .col(ColumnDef::new(DispatchColumn::LastReviewedVersion).big_integer())
                    .col(
                        ColumnDef::new(DispatchColumn::ReviewSequence)
                            .big_integer()
                            .not_null()
                            .default(0_i64),
                    )
                    .col(ColumnDef::new(DispatchColumn::PendingReview).json_binary())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_patient_dispatch")
                            .col(DispatchColumn::Practice)
                            .col(DispatchColumn::Workspace),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "practice background jobs and patient dispatch records cannot be rolled back".into(),
        ))
    }
}
