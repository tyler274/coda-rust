//! Additive schema for pull missing-record receipts, practice-scoped source
//! patient links, practice source access state, and per-patient pull status.

use sea_orm_migration::prelude::*;

use super::super::entities::{
    patient_source_status, practice_source_access, source_patient_link, source_pull,
    workspace_practice,
};

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(source_pull::Entity)
                    .add_column(
                        ColumnDef::new(source_pull::Column::MissingCandidates)
                            .json_binary()
                            .not_null()
                            .default(Expr::cust("'[]'::jsonb")),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(workspace_practice::Entity)
                    .col(
                        ColumnDef::new(workspace_practice::Column::Workspace)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(workspace_practice::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(source_patient_link::Entity)
                    .col(
                        ColumnDef::new(source_patient_link::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(source_patient_link::Column::SourceId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(source_patient_link::Column::SourcePatientId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(source_patient_link::Column::Workspace)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(source_patient_link::Column::LinkedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(source_patient_link::Column::Practice)
                            .col(source_patient_link::Column::SourceId)
                            .col(source_patient_link::Column::SourcePatientId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("physician_runtime_source_patient_links_workspace_uq")
                    .table(source_patient_link::Entity)
                    .col(source_patient_link::Column::Practice)
                    .col(source_patient_link::Column::SourceId)
                    .col(source_patient_link::Column::Workspace)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(practice_source_access::Entity)
                    .col(
                        ColumnDef::new(practice_source_access::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(practice_source_access::Column::SourceId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(practice_source_access::Column::Access)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(practice_source_access::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(practice_source_access::Column::Practice)
                            .col(practice_source_access::Column::SourceId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(patient_source_status::Entity)
                    .col(
                        ColumnDef::new(patient_source_status::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_source_status::Column::Workspace)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_source_status::Column::SourceId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_source_status::Column::LastGoodReadAt).big_integer(),
                    )
                    .col(ColumnDef::new(patient_source_status::Column::Attempt).json_binary())
                    .primary_key(
                        Index::create()
                            .col(patient_source_status::Column::Practice)
                            .col(patient_source_status::Column::Workspace)
                            .col(patient_source_status::Column::SourceId),
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
