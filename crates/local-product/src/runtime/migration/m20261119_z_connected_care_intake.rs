//! Adds `priority` on `physician_runtime_practice_jobs` and the durable connected-care
//! `"For my doctor"` share intake and consent-sync ledger tables.

use sea_orm_migration::prelude::*;

use super::super::entities::{connected_care_consent, connected_care_intake, practice_job};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(practice_job::Entity)
                    .add_column(
                        ColumnDef::new(practice_job::Column::Priority)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_practice_jobs-priority")
                    .table(practice_job::Entity)
                    .col(practice_job::Column::Practice)
                    .col(practice_job::Column::Status)
                    .col(practice_job::Column::Priority)
                    .col(practice_job::Column::UpdatedAt)
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(connected_care_intake::Entity)
                    .col(
                        ColumnDef::new(connected_care_intake::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_intake::Column::ShareId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_intake::Column::Workspace)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_intake::Column::CandidateId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_intake::Column::SentAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_intake::Column::CommandKey)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_intake::Column::IngestedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_intake::Column::ReceivedWrittenAt)
                            .big_integer(),
                    )
                    .primary_key(
                        Index::create()
                            .col(connected_care_intake::Column::Practice)
                            .col(connected_care_intake::Column::ShareId),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(connected_care_consent::Entity)
                    .col(
                        ColumnDef::new(connected_care_consent::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_consent::Column::Workspace)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_consent::Column::ConsentedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_consent::Column::CommandKey)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connected_care_consent::Column::SyncedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(connected_care_consent::Column::Practice)
                            .col(connected_care_consent::Column::Workspace),
                    )
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(connected_care_consent::Entity).take())
            .await?;
        manager
            .drop_table(Table::drop().table(connected_care_intake::Entity).take())
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-physician_runtime_practice_jobs-priority")
                    .table(practice_job::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(practice_job::Entity)
                    .drop_column(practice_job::Column::Priority)
                    .to_owned(),
            )
            .await
    }
}
