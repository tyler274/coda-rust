//! Per-physician seen state for Today and Updates items, plus practice-scoped
//! `Done` clearances for non-restartable unhandled work on Today.

use super::super::entities::{today_updates_seen, today_work_dismissal};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_today_updates_seen` and
/// `physician_runtime_today_work_dismissals`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let seen_key = |column: today_updates_seen::Column| {
            ColumnDef::new(column).text().not_null().to_owned()
        };
        manager
            .create_table(
                Table::create()
                    .table(today_updates_seen::Entity)
                    .col(seen_key(today_updates_seen::Column::Practice))
                    .col(seen_key(today_updates_seen::Column::Physician))
                    .col(seen_key(today_updates_seen::Column::ItemKey))
                    .col(seen_key(today_updates_seen::Column::Workspace))
                    .col(seen_key(today_updates_seen::Column::SeenVersion))
                    .col(
                        ColumnDef::new(today_updates_seen::Column::SeenSnapshot)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(today_updates_seen::Column::SeenAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(today_updates_seen::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_today_updates_seen")
                            .col(today_updates_seen::Column::Practice)
                            .col(today_updates_seen::Column::Physician)
                            .col(today_updates_seen::Column::ItemKey),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_today_updates_seen-physician_workspace")
                    .table(today_updates_seen::Entity)
                    .col(today_updates_seen::Column::Practice)
                    .col(today_updates_seen::Column::Physician)
                    .col(today_updates_seen::Column::Workspace)
                    .to_owned(),
            )
            .await?;

        let dismissal_key = |column: today_work_dismissal::Column| {
            ColumnDef::new(column).text().not_null().to_owned()
        };
        manager
            .create_table(
                Table::create()
                    .table(today_work_dismissal::Entity)
                    .col(dismissal_key(today_work_dismissal::Column::Practice))
                    .col(dismissal_key(today_work_dismissal::Column::Workspace))
                    .col(dismissal_key(today_work_dismissal::Column::ItemKey))
                    .col(dismissal_key(today_work_dismissal::Column::VersionId))
                    .col(dismissal_key(
                        today_work_dismissal::Column::ClearedByPhysician,
                    ))
                    .col(
                        ColumnDef::new(today_work_dismissal::Column::ClearedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_today_work_dismissals")
                            .col(today_work_dismissal::Column::Practice)
                            .col(today_work_dismissal::Column::Workspace)
                            .col(today_work_dismissal::Column::ItemKey)
                            .col(today_work_dismissal::Column::VersionId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_today_work_dismissals-practice_workspace")
                    .table(today_work_dismissal::Entity)
                    .col(today_work_dismissal::Column::Practice)
                    .col(today_work_dismissal::Column::Workspace)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
