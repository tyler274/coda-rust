//! Records sources whose content may not be shown in a patient workspace
//! because access to that source was removed or the record was moved off the
//! wrong patient.
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_source_restrictions`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[derive(DeriveIden)]
enum PhysicianRuntimeSourceRestrictions {
    Table,
    Workspace,
    Candidate,
    Reason,
    RecordedAtVersion,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhysicianRuntimeSourceRestrictions::Table)
                    .col(
                        ColumnDef::new(PhysicianRuntimeSourceRestrictions::Workspace)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PhysicianRuntimeSourceRestrictions::Candidate)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PhysicianRuntimeSourceRestrictions::Reason)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PhysicianRuntimeSourceRestrictions::RecordedAtVersion)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_source_restrictions")
                            .col(PhysicianRuntimeSourceRestrictions::Workspace)
                            .col(PhysicianRuntimeSourceRestrictions::Candidate),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "the physician runtime's records can't be rolled back".into(),
        ))
    }
}
