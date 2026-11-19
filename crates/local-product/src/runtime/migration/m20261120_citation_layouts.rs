//! Creates `physician_runtime_citation_layouts` for per-`(source_id, record_type)`
//! declarative field layouts (`Citations` Layout 2–5).

use sea_orm_migration::prelude::*;

use super::super::citation_layouts::table::{Column as LayoutCol, Entity as LayoutEntity};

/// Creates `physician_runtime_citation_layouts`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LayoutEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LayoutCol::SourceId)
                            .text()
                            .not_null()
                            .check(Expr::col(LayoutCol::SourceId).ne("")),
                    )
                    .col(
                        ColumnDef::new(LayoutCol::RecordType)
                            .text()
                            .not_null()
                            .check(Expr::col(LayoutCol::RecordType).ne("")),
                    )
                    .col(ColumnDef::new(LayoutCol::Layout).json_binary().not_null())
                    .col(
                        ColumnDef::new(LayoutCol::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LayoutCol::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_citation_layouts")
                            .col(LayoutCol::SourceId)
                            .col(LayoutCol::RecordType),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LayoutEntity).to_owned())
            .await
    }
}
