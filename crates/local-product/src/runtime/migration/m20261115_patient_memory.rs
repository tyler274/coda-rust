//! Patient memory: one row for each version of each memory file, one row for
//! each note, and one row for each patient rounds run that finished. The
//! three tables are insert and select only.
use super::super::entities::{memory_note, memory_rounds, memory_version};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_memory_versions`,
/// `physician_runtime_memory_notes` and `physician_runtime_memory_rounds`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use memory_note::Column as N;
        use memory_rounds::Column as R;
        use memory_version::Column as V;
        manager
            .create_table(
                Table::create()
                    .table(memory_version::Entity)
                    .col(ColumnDef::new(V::Practice).text().not_null())
                    .col(ColumnDef::new(V::Workspace).text().not_null())
                    .col(ColumnDef::new(V::Path).text().not_null())
                    .col(
                        ColumnDef::new(V::Version)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(V::Version).gt(0)),
                    )
                    .col(ColumnDef::new(V::Content).text().not_null())
                    .col(ColumnDef::new(V::SavedAt).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_memory_versions")
                            .col(V::Practice)
                            .col(V::Workspace)
                            .col(V::Path)
                            .col(V::Version),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(memory_note::Entity)
                    .col(ColumnDef::new(N::Practice).text().not_null())
                    .col(ColumnDef::new(N::Workspace).text().not_null())
                    .col(ColumnDef::new(N::Note).text().not_null())
                    .col(
                        ColumnDef::new(N::Position)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(N::Position).gt(0)),
                    )
                    .col(ColumnDef::new(N::SavedAt).big_integer().not_null())
                    .col(ColumnDef::new(N::Physician).text().not_null())
                    .col(ColumnDef::new(N::Chat).text().not_null())
                    .col(ColumnDef::new(N::Message).text().not_null())
                    .col(ColumnDef::new(N::Text).text().not_null())
                    .col(ColumnDef::new(N::PhysicianWords).text())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_memory_notes")
                            .col(N::Practice)
                            .col(N::Workspace)
                            .col(N::Note),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_memory_notes-patient_position")
                    .table(memory_note::Entity)
                    .col(N::Practice)
                    .col(N::Workspace)
                    .col(N::Position)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(memory_rounds::Entity)
                    .col(ColumnDef::new(R::Practice).text().not_null())
                    .col(ColumnDef::new(R::Workspace).text().not_null())
                    .col(ColumnDef::new(R::Run).text().not_null())
                    .col(ColumnDef::new(R::StartedAt).big_integer().not_null())
                    .col(ColumnDef::new(R::SavedAt).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_memory_rounds")
                            .col(R::Practice)
                            .col(R::Workspace)
                            .col(R::Run),
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
