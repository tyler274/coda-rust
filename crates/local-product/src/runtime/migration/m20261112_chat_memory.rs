//! Durable patient memory entries carried across physician conversations and
//! background runs for `(practice, workspace)`.

use super::super::entities::patient_memory;
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_patient_memory`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let key_col =
            |column: patient_memory::Column| ColumnDef::new(column).text().not_null().to_owned();
        let active_standing = Condition::all()
            .add(Expr::col(patient_memory::Column::Status).eq("active"))
            .add(Expr::col(patient_memory::Column::RetirementReason).is_null())
            .add(Expr::col(patient_memory::Column::RetiredBy).is_null())
            .add(Expr::col(patient_memory::Column::RetiredAt).is_null());
        let retired_standing = Condition::all()
            .add(Expr::col(patient_memory::Column::Status).eq("retired"))
            .add(Expr::col(patient_memory::Column::RetirementReason).is_not_null())
            .add(Expr::col(patient_memory::Column::RetiredBy).is_not_null())
            .add(Expr::col(patient_memory::Column::RetiredAt).is_not_null());

        manager
            .create_table(
                Table::create()
                    .table(patient_memory::Entity)
                    .col(key_col(patient_memory::Column::Practice))
                    .col(key_col(patient_memory::Column::Workspace))
                    .col(key_col(patient_memory::Column::Id))
                    .col(
                        ColumnDef::new(patient_memory::Column::Status)
                            .text()
                            .not_null()
                            .check(
                                Expr::col(patient_memory::Column::Status)
                                    .is_in(["active", "retired"]),
                            ),
                    )
                    .col(
                        ColumnDef::new(patient_memory::Column::Topic)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_memory::Column::Statement)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_memory::Column::Provenance)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(patient_memory::Column::RetirementReason).text())
                    .col(ColumnDef::new(patient_memory::Column::RetiredBy).json_binary())
                    .col(ColumnDef::new(patient_memory::Column::RetiredAt).big_integer())
                    .col(
                        ColumnDef::new(patient_memory::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_memory::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_memory::Column::Revision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(patient_memory::Column::Revision).gt(0)),
                    )
                    .check(Condition::any().add(active_standing).add(retired_standing))
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_patient_memory")
                            .col(patient_memory::Column::Practice)
                            .col(patient_memory::Column::Workspace)
                            .col(patient_memory::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_patient_memory-workspace_status")
                    .table(patient_memory::Entity)
                    .col(patient_memory::Column::Practice)
                    .col(patient_memory::Column::Workspace)
                    .col(patient_memory::Column::Status)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "patient memory records are clinical working records and can't be rolled back".into(),
        ))
    }
}
