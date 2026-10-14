//! The SeaORM entities and the physician-auth migration chain.
//!
//! Modeled on the Supabase GoTrue table families (`users`, `sessions`,
//! `one_time_tokens`), simplified to what the physician journeys use:
//! accounts, one session per sign-in, one-time sign-in tokens, invitations,
//! and per-workspace memberships. Deliberate divergence: one long-lived
//! bearer per sign-in, no refresh-token family or rotation.

use sea_orm_migration::prelude::*;

// The entity models below carry field docs; the generated struct and relation
// types are mechanical SeaORM shapes, so the crate-wide doc lint stays off
// for this module's generated names.
#[allow(missing_docs)]
pub mod account;

#[allow(missing_docs)]
pub mod invitation;

#[allow(missing_docs)]
pub mod membership;

#[allow(missing_docs)]
pub mod patient;
pub mod patient_link;

#[allow(missing_docs)]
pub mod session;

#[allow(missing_docs)]
pub mod sign_in_token;

pub use account::Model as AccountRow;
pub use account::{ActiveModel as AccountActiveModel, Entity as Accounts};
pub use invitation::Model as InvitationRow;
pub use invitation::{ActiveModel as InvitationActiveModel, Entity as Invitations};
pub use membership::{ActiveModel as MembershipActiveModel, Entity as Memberships};
pub use patient::Model as PatientRow;
pub use patient::{ActiveModel as PatientActiveModel, Entity as Patients};
pub use patient_link::Model as PatientLinkRow;
pub use patient_link::{ActiveModel as PatientLinkActiveModel, Entity as PatientLinks};
pub use session::Model as SessionRow;
pub use session::{ActiveModel as SessionActiveModel, Entity as Sessions};
pub use sign_in_token::Model as SignInTokenRow;
pub use sign_in_token::{ActiveModel as SignInTokenActiveModel, Entity as SignInTokens};

/// Every migration, oldest first.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_init),
            Box::new(m20260101_000002_practice_patients),
        ]
    }
}

#[derive(DeriveMigrationName)]
#[allow(non_camel_case_types, missing_docs)]
pub struct m20260101_000001_init;

#[async_trait::async_trait]
impl MigrationTrait for m20260101_000001_init {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts)
                    .col(
                        ColumnDef::new(account::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(account::Column::PracticeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(account::Column::PhysicianId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(account::Column::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(account::Column::DisplayName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(account::Column::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(account::Column::Admin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(account::Column::SessionsEndedBefore)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(account::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_accounts_practice_idx")
                    .table(Accounts)
                    .col(account::Column::PracticeId)
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Sessions)
                    .col(
                        ColumnDef::new(session::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(session::Column::AccountId).uuid().not_null())
                    .col(
                        ColumnDef::new(session::Column::TokenHash)
                            .binary_len(32)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(session::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(session::Column::ExpiresAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("physician_sessions_account_fk")
                            .from(Sessions, session::Column::AccountId)
                            .to(Accounts, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_sessions_account_idx")
                    .table(Sessions)
                    .col(session::Column::AccountId)
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(SignInTokens)
                    .col(
                        ColumnDef::new(sign_in_token::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(sign_in_token::Column::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(sign_in_token::Column::TokenHash)
                            .binary_len(32)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(sign_in_token::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(sign_in_token::Column::ExpiresAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("physician_sign_in_tokens_account_fk")
                            .from(SignInTokens, sign_in_token::Column::AccountId)
                            .to(Accounts, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_sign_in_tokens_account_idx")
                    .table(SignInTokens)
                    .col(sign_in_token::Column::AccountId)
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Invitations)
                    .col(
                        ColumnDef::new(invitation::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(invitation::Column::PracticeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(invitation::Column::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(invitation::Column::DisplayName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(invitation::Column::Admin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(invitation::Column::TokenHash)
                            .binary_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(invitation::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(invitation::Column::ExpiresAt)
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Memberships)
                    .col(
                        ColumnDef::new(membership::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(membership::Column::PracticeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(membership::Column::PhysicianId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(membership::Column::PatientWorkspaceId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(membership::Column::Allowed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(membership::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_memberships_workspace_idx")
                    .table(Memberships)
                    .col(membership::Column::PracticeId)
                    .col(membership::Column::PhysicianId)
                    .col(membership::Column::PatientWorkspaceId)
                    .unique()
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PatientLinks)
                    .col(
                        ColumnDef::new(patient_link::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::PracticeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::PatientWorkspaceId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::PendingTokenHash)
                            .binary_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::PendingExpiresAt)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::DuetAccountId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::Linked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::SharingConsented)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::ConsentVersion)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::ConsentedAt)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::RevokedAt)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::Cadence)
                            .string()
                            .not_null()
                            .default("weekly"),
                    )
                    .col(
                        ColumnDef::new(patient_link::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_patient_links_workspace_idx")
                    .table(PatientLinks)
                    .col(patient_link::Column::PracticeId)
                    .col(patient_link::Column::PatientWorkspaceId)
                    .unique()
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PatientLinks).take())
            .await?;
        manager
            .drop_table(Table::drop().table(Memberships).take())
            .await?;
        manager
            .drop_table(Table::drop().table(Invitations).take())
            .await?;
        manager
            .drop_table(Table::drop().table(SignInTokens).take())
            .await?;
        manager
            .drop_table(Table::drop().table(Sessions).take())
            .await?;
        manager
            .drop_table(Table::drop().table(Accounts).take())
            .await
    }
}

#[derive(DeriveMigrationName)]
#[allow(non_camel_case_types, missing_docs)]
pub struct m20260101_000002_practice_patients;

#[async_trait::async_trait]
impl MigrationTrait for m20260101_000002_practice_patients {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Patients)
                    .col(
                        ColumnDef::new(patient::Column::PatientWorkspaceId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(patient::Column::PracticeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(patient::Column::Cadence)
                            .string()
                            .not_null()
                            .default("weekly"),
                    )
                    .col(
                        ColumnDef::new(patient::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_patients_practice_idx")
                    .table(Patients)
                    .col(patient::Column::PracticeId)
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Patients).take())
            .await
    }
}
