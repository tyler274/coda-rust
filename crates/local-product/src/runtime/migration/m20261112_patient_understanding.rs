//! Durable patient understanding per `(practice, workspace)` and its append-only
//! revision and idempotency ledger.

use super::super::entities::{patient_understanding, patient_understanding_revision};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_patient_understanding` and
/// `physician_runtime_patient_understanding_revisions`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let understanding_key = |column: patient_understanding::Column| {
            ColumnDef::new(column).text().not_null().to_owned()
        };

        manager
            .create_table(
                Table::create()
                    .table(patient_understanding::Entity)
                    .col(understanding_key(patient_understanding::Column::Practice))
                    .col(understanding_key(patient_understanding::Column::Workspace))
                    .col(
                        ColumnDef::new(patient_understanding::Column::Revision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(patient_understanding::Column::Revision).gt(0)),
                    )
                    .col(
                        ColumnDef::new(patient_understanding::Column::SourceVersion)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(patient_understanding::Column::SourceVersion).gte(0)),
                    )
                    .col(
                        ColumnDef::new(patient_understanding::Column::EvidenceScope)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding::Column::OperatingIndex)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding::Column::Summary)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding::Column::Origin)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_patient_understanding")
                            .col(patient_understanding::Column::Practice)
                            .col(patient_understanding::Column::Workspace),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_patient_understanding-workspace")
                    .table(patient_understanding::Entity)
                    .col(patient_understanding::Column::Workspace)
                    .to_owned(),
            )
            .await?;

        let revision_key = |column: patient_understanding_revision::Column| {
            ColumnDef::new(column).text().not_null().to_owned()
        };

        manager
            .create_table(
                Table::create()
                    .table(patient_understanding_revision::Entity)
                    .col(revision_key(
                        patient_understanding_revision::Column::Practice,
                    ))
                    .col(revision_key(
                        patient_understanding_revision::Column::Workspace,
                    ))
                    .col(
                        ColumnDef::new(patient_understanding_revision::Column::Revision)
                            .big_integer()
                            .not_null()
                            .check(
                                Expr::col(patient_understanding_revision::Column::Revision).gt(0),
                            ),
                    )
                    .col(revision_key(
                        patient_understanding_revision::Column::IdempotencyKey,
                    ))
                    .col(
                        ColumnDef::new(patient_understanding_revision::Column::SourceVersion)
                            .big_integer()
                            .not_null()
                            .check(
                                Expr::col(patient_understanding_revision::Column::SourceVersion)
                                    .gte(0),
                            ),
                    )
                    .col(
                        ColumnDef::new(patient_understanding_revision::Column::EvidenceScope)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding_revision::Column::OperatingIndex)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding_revision::Column::Summary)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding_revision::Column::Origin)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_understanding_revision::Column::SavedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_patient_understanding_revisions")
                            .col(patient_understanding_revision::Column::Practice)
                            .col(patient_understanding_revision::Column::Workspace)
                            .col(patient_understanding_revision::Column::Revision),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq-physician_runtime_patient_understanding_revisions-key")
                    .table(patient_understanding_revision::Entity)
                    .col(patient_understanding_revision::Column::Practice)
                    .col(patient_understanding_revision::Column::Workspace)
                    .col(patient_understanding_revision::Column::IdempotencyKey)
                    .unique()
                    .to_owned(),
            )
            .await
    }
}
