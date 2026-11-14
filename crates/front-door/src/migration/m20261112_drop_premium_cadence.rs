//! Removes `front_door_premium.cadence`.
//!
//! Front-door premium status holds only the feature flag; patient review
//! cadence lives solely in the physician runtime dispatch store.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("front_door_premium"))
                    .drop_column(Alias::new("cadence"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("front_door_premium"))
                    .add_column(
                        ColumnDef::new(Alias::new("cadence"))
                            .string()
                            .not_null()
                            .default("weekly"),
                    )
                    .to_owned(),
            )
            .await
    }
}
