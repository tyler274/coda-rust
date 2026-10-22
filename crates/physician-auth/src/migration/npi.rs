use super::{Accounts, account};
use sea_orm_migration::prelude::*;

pub struct NpiAssociation;
impl MigrationName for NpiAssociation {
    fn name(&self) -> &str {
        "physician_auth_20261022_000003_npi_association"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for NpiAssociation {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Accounts)
                    .add_column(ColumnDef::new(account::Column::Npi).string())
                    .add_column(ColumnDef::new(account::Column::NpiSnapshot).json_binary())
                    .add_column(ColumnDef::new(account::Column::NpiLookupCheckedAt).big_integer())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_accounts_npi_unique")
                    .table(Accounts)
                    .col(account::Column::Npi)
                    .unique()
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("physician_accounts_npi_unique")
                    .table(Accounts)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Accounts)
                    .drop_column(account::Column::Npi)
                    .drop_column(account::Column::NpiSnapshot)
                    .drop_column(account::Column::NpiLookupCheckedAt)
                    .to_owned(),
            )
            .await
    }
}
