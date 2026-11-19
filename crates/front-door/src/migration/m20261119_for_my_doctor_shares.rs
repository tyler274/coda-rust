//! Adds `front_door_for_my_doctor_shares` and `front_door_shared_records`.

use sea_orm_migration::prelude::*;

use super::{account, for_my_doctor_share, shared_record};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(for_my_doctor_share::Entity)
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::ShareId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::PracticeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::PatientId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::ShareKey)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::Title)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::SummaryText)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::OriginLabel)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::SentAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(for_my_doctor_share::Column::ConsentAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(for_my_doctor_share::Column::ReceivedAt).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_for_my_doctor_account_fk")
                            .from(
                                for_my_doctor_share::Entity,
                                for_my_doctor_share::Column::AccountId,
                            )
                            .to(account::Entity, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("front_door_for_my_doctor_account_key_uq")
                    .table(for_my_doctor_share::Entity)
                    .col(for_my_doctor_share::Column::AccountId)
                    .col(for_my_doctor_share::Column::ShareKey)
                    .unique()
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("front_door_for_my_doctor_practice_idx")
                    .table(for_my_doctor_share::Entity)
                    .col(for_my_doctor_share::Column::PracticeId)
                    .col(for_my_doctor_share::Column::ReceivedAt)
                    .col(for_my_doctor_share::Column::SentAt)
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(shared_record::Entity)
                    .col(
                        ColumnDef::new(shared_record::Column::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shared_record::Column::RecordId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shared_record::Column::Revision)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shared_record::Column::RecordType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shared_record::Column::Title)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shared_record::Column::ContentText)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shared_record::Column::RecordedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shared_record::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(shared_record::Column::AccountId)
                            .col(shared_record::Column::RecordId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_shared_records_account_fk")
                            .from(shared_record::Entity, shared_record::Column::AccountId)
                            .to(account::Entity, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(shared_record::Entity).take())
            .await?;
        manager
            .drop_table(Table::drop().table(for_my_doctor_share::Entity).take())
            .await
    }
}
