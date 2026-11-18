//! Indexed latest-activity column on `physician_runtime_conversations`, filled
//! from each existing row's saved `state` and indexed for cross-patient and
//! single-patient physician conversation lists.

use super::super::entities::conversation;
use sea_orm::sea_query::{ExprTrait, Func, IndexOrder, extension::postgres::PgExpr};
use sea_orm_migration::prelude::*;

/// Adds `last_activity_at` to `physician_runtime_conversations`, backfills
/// existing rows from `state`, and creates the physician and workspace activity
/// indexes.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(conversation::Entity)
                    .add_column(
                        ColumnDef::new(conversation::Column::LastActivityAt)
                            .big_integer()
                            .not_null()
                            .default(0_i64),
                    )
                    .to_owned(),
            )
            .await?;

        let seconds = |field: &str| {
            Func::coalesce([
                Expr::col(conversation::Column::State)
                    .cast_json_field(field)
                    .cast_as(Alias::new("bigint")),
                Expr::val(0_i64),
            ])
        };
        let backfill = Query::update()
            .table(conversation::Entity)
            .value(
                conversation::Column::LastActivityAt,
                Expr::from(Func::greatest([
                    Expr::from(seconds("last_activity_at")),
                    Expr::from(seconds("opened_at")),
                ])),
            )
            .to_owned();
        manager.exec_stmt(backfill).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_conversations-physician_activity")
                    .table(conversation::Entity)
                    .col(conversation::Column::Practice)
                    .col(conversation::Column::Physician)
                    .col((conversation::Column::LastActivityAt, IndexOrder::Desc))
                    .col((conversation::Column::Chat, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_conversations-workspace_activity")
                    .table(conversation::Entity)
                    .col(conversation::Column::Practice)
                    .col(conversation::Column::Physician)
                    .col(conversation::Column::Workspace)
                    .col((conversation::Column::LastActivityAt, IndexOrder::Desc))
                    .col((conversation::Column::Chat, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
