//! Adds invitation short codes, DOB lockout, preview attribution, and delivery failure tracking.

use sea_orm_migration::prelude::*;

use super::{invitation, invitation_credential};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(invitation::Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::PracticeName)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::PhysicianName)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::ExpectedBirthDate)
                            .string()
                            .null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::WrongDobAttempts)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::LockedAt)
                            .big_integer()
                            .null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::LastDeliveryFailedAt)
                            .big_integer()
                            .null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::LastDeliveryError)
                            .string()
                            .null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(invitation::Column::UpdatedAt)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE front_door_invitations SET updated_at = created_at WHERE updated_at = 0",
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(invitation_credential::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(invitation_credential::Column::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(invitation_credential::Column::InvitationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(invitation_credential::Column::Kind)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(invitation_credential::Column::SecretHash)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(invitation_credential::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_invitation_credentials_inv_fk")
                            .from(
                                invitation_credential::Entity,
                                invitation_credential::Column::InvitationId,
                            )
                            .to(invitation::Entity, invitation::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("front_door_invitation_credentials_hash_uq")
                    .table(invitation_credential::Entity)
                    .if_not_exists()
                    .col(invitation_credential::Column::SecretHash)
                    .unique()
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("front_door_invitation_credentials_inv_idx")
                    .table(invitation_credential::Entity)
                    .if_not_exists()
                    .col(invitation_credential::Column::InvitationId)
                    .take(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "INSERT INTO front_door_invitation_credentials (invitation_id, kind, secret_hash, created_at) \
                 SELECT id, 'link', token_hash, created_at FROM front_door_invitations \
                 ON CONFLICT (secret_hash) DO NOTHING",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS front_door_invitation_credentials_inv_idx")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS front_door_invitation_credentials_hash_uq")
            .await?;
        manager
            .drop_table(Table::drop().table(invitation_credential::Entity).take())
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(invitation::Entity)
                    .drop_column(invitation::Column::UpdatedAt)
                    .drop_column(invitation::Column::LastDeliveryError)
                    .drop_column(invitation::Column::LastDeliveryFailedAt)
                    .drop_column(invitation::Column::LockedAt)
                    .drop_column(invitation::Column::WrongDobAttempts)
                    .drop_column(invitation::Column::ExpectedBirthDate)
                    .drop_column(invitation::Column::PhysicianName)
                    .drop_column(invitation::Column::PracticeName)
                    .to_owned(),
            )
            .await
    }
}
