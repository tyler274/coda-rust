//! Additive schema for pull missing-record receipts and nullable partial-pull
//! timestamps on `physician_runtime_source_pulls`.

use sea_orm_migration::prelude::*;

use super::super::entities::source_pull;

#[derive(DeriveMigrationName)]
pub(in crate::runtime) struct Migration;

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
                    .modify_column(
                        ColumnDef::new(source_pull::Column::PulledAt)
                            .big_integer()
                            .null(),
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
