//! Additive account identity and permission migration; no legacy account adoption.
use super::{account, identity, permission};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(identity::Entity)
                    .col(
                        ColumnDef::new(identity::Column::Project)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(identity::Column::Subject)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(identity::Column::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(identity::Column::Project)
                            .col(identity::Column::Subject),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_identity_account_fk")
                            .from(identity::Entity, identity::Column::AccountId)
                            .to(account::Entity, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("front_door_identity_account_idx")
                    .table(identity::Entity)
                    .col(identity::Column::AccountId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(permission::Entity)
                    .col(
                        ColumnDef::new(permission::Column::AccountId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(permission::Column::TermsVersion).string())
                    .col(ColumnDef::new(permission::Column::TermsAt).big_integer())
                    .col(ColumnDef::new(permission::Column::LunaVersion).string())
                    .col(
                        ColumnDef::new(permission::Column::LunaAllowed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(permission::Column::LunaAt).big_integer())
                    .col(
                        ColumnDef::new(permission::Column::Revision)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(permission::Column::RestrictionPending)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_permission_account_fk")
                            .from(permission::Entity, permission::Column::AccountId)
                            .to(account::Entity, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(permission::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(identity::Entity).to_owned())
            .await
    }
}
