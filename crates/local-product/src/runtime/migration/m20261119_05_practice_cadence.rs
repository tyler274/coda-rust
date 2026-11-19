//! Practice-level default check and review cadence table.

use sea_orm_migration::prelude::*;
use serde_json::json;

use super::super::entities::practice_cadence;

/// Creates `physician_runtime_practice_cadence` holding optional per-practice
/// default `Cadence` settings inherited by newly registered patients.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let default_cadence = json!({
            "check": { "interval_seconds": 86_400 },
            "review": { "kind": "periodic", "interval_seconds": 604_800 },
        });
        manager
            .create_table(
                Table::create()
                    .table(practice_cadence::Entity)
                    .col(
                        ColumnDef::new(practice_cadence::Column::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(practice_cadence::Column::Cadence)
                            .json_binary()
                            .not_null()
                            .default(default_cadence),
                    )
                    .col(
                        ColumnDef::new(practice_cadence::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_practice_cadence")
                            .col(practice_cadence::Column::Practice),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration(
            "practice cadence configuration cannot be rolled back".into(),
        ))
    }
}
