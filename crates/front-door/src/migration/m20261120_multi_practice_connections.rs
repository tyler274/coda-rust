//! Replaces single-practice `front_door_links`, paused `front_door_consents`,
//! `front_door_premium`, and invitation/session TTL columns with canonical
//! multi-practice `front_door_connections` keyed by `(account_id, practice_id, patient_id)`.

use sea_orm_migration::prelude::*;

use super::{account, connection, for_my_doctor_share, invitation, session};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(connection::Entity)
                    .col(
                        ColumnDef::new(connection::Column::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connection::Column::PracticeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connection::Column::PatientId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connection::Column::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connection::Column::ConnectedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(connection::Column::EndedAt).big_integer())
                    .col(ColumnDef::new(connection::Column::EndReason).string())
                    .col(
                        ColumnDef::new(connection::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(connection::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(connection::Column::AccountId)
                            .col(connection::Column::PracticeId)
                            .col(connection::Column::PatientId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_connections_account_fk")
                            .from(connection::Entity, connection::Column::AccountId)
                            .to(account::Entity, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("front_door_connections_roster_idx")
                    .table(connection::Entity)
                    .col(connection::Column::PracticeId)
                    .col(connection::Column::PatientId)
                    .take(),
            )
            .await?;

        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX front_door_connections_active_roster_uq \
             ON front_door_connections (practice_id, patient_id) \
             WHERE status = 'connected'",
        )
        .await?;

        // Migrate any historical consented/withdrawn links into `front_door_connections`
        // before dropping the single-practice and paused-consent tables. Links without
        // explicit consent remain unlinked.
        db.execute_unprepared(
            "INSERT INTO front_door_connections \
             (account_id, practice_id, patient_id, status, connected_at, ended_at, end_reason, created_at, updated_at) \
             SELECT l.account_id, l.practice_id, l.patient_id, \
                    CASE WHEN c.withdrawn_at IS NULL THEN 'connected' ELSE 'ended' END, \
                    c.consented_at, \
                    c.withdrawn_at, \
                    CASE WHEN c.withdrawn_at IS NULL THEN NULL ELSE 'patient_left' END, \
                    l.created_at, \
                    COALESCE(c.withdrawn_at, c.consented_at) \
             FROM front_door_links l \
             JOIN front_door_consents c ON c.account_id = l.account_id",
        )
        .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("front_door_premium")).take())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("front_door_consents"))
                    .take(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Alias::new("front_door_links")).take())
            .await?;

        db.execute_unprepared("DROP INDEX IF EXISTS front_door_accounts_link_idx")
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(account::Entity)
                    .drop_column(Alias::new("linked_practice_id"))
                    .drop_column(Alias::new("linked_patient_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(session::Entity)
                    .drop_column(Alias::new("expires_at"))
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            "ALTER TABLE front_door_invitations DROP CONSTRAINT IF EXISTS front_door_invitations_email_key",
        )
        .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS front_door_invitations_email_key")
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(invitation::Entity)
                    .drop_column(Alias::new("expires_at"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("front_door_invitations_roster_uq")
                    .table(invitation::Entity)
                    .col(invitation::Column::PracticeId)
                    .col(invitation::Column::PatientId)
                    .unique()
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("front_door_invitations_email_idx")
                    .table(invitation::Entity)
                    .col(invitation::Column::Email)
                    .take(),
            )
            .await?;

        db.execute_unprepared("DROP INDEX IF EXISTS front_door_for_my_doctor_account_key_uq")
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("front_door_for_my_doctor_target_key_uq")
                    .table(for_my_doctor_share::Entity)
                    .col(for_my_doctor_share::Column::AccountId)
                    .col(for_my_doctor_share::Column::PracticeId)
                    .col(for_my_doctor_share::Column::PatientId)
                    .col(for_my_doctor_share::Column::ShareKey)
                    .unique()
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS front_door_for_my_doctor_target_key_uq")
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

        db.execute_unprepared("DROP INDEX IF EXISTS front_door_invitations_email_idx")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS front_door_invitations_roster_uq")
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(invitation::Entity)
                    .add_column(
                        ColumnDef::new(Alias::new("expires_at"))
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared(
            "ALTER TABLE front_door_invitations ADD CONSTRAINT front_door_invitations_email_key UNIQUE (email)",
        )
        .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(session::Entity)
                    .add_column(
                        ColumnDef::new(Alias::new("expires_at"))
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(account::Entity)
                    .add_column(ColumnDef::new(Alias::new("linked_practice_id")).string())
                    .add_column(ColumnDef::new(Alias::new("linked_patient_id")).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("front_door_accounts_link_idx")
                    .table(account::Entity)
                    .col(Alias::new("linked_practice_id"))
                    .col(Alias::new("linked_patient_id"))
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("front_door_links"))
                    .col(
                        ColumnDef::new(Alias::new("account_id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("practice_id"))
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("patient_id")).string().not_null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_links_account_fk")
                            .from(Alias::new("front_door_links"), Alias::new("account_id"))
                            .to(account::Entity, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("front_door_links_roster_idx")
                    .table(Alias::new("front_door_links"))
                    .col(Alias::new("practice_id"))
                    .col(Alias::new("patient_id"))
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("front_door_consents"))
                    .col(
                        ColumnDef::new(Alias::new("account_id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("consented_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("withdrawn_at")).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_consents_account_fk")
                            .from(Alias::new("front_door_consents"), Alias::new("account_id"))
                            .to(account::Entity, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("front_door_premium"))
                    .col(
                        ColumnDef::new(Alias::new("patient_key"))
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("premium"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(connection::Entity).take())
            .await
    }
}
