//! Physician feedback on Coda's responses: one row per physician and rated
//! response, holding the latest rating, any details, and a revision every save
//! increments.
use super::super::entities::response_feedback::{Column, Entity};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_response_feedback`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let key = |column: Column| ColumnDef::new(column).text().not_null().to_owned();
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .col(key(Column::Practice))
                    .col(key(Column::Physician))
                    .col(key(Column::Workspace))
                    .col(key(Column::Chat))
                    .col(key(Column::Response))
                    .col(
                        ColumnDef::new(Column::Rating)
                            .text()
                            .not_null()
                            .check(Expr::col(Column::Rating).is_in(["up", "down"])),
                    )
                    .col(ColumnDef::new(Column::Details).json_binary())
                    .col(ColumnDef::new(Column::SavedAt).big_integer().not_null())
                    .col(
                        ColumnDef::new(Column::Revision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(Column::Revision).gt(0)),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_response_feedback")
                            .col(Column::Practice)
                            .col(Column::Physician)
                            .col(Column::Workspace)
                            .col(Column::Chat)
                            .col(Column::Response),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "physician feedback is clinical working text and can't be rolled back".into(),
        ))
    }
}
