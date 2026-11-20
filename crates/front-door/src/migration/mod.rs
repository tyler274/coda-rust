//! The SeaORM entities and the front-door migration chain.
//!
//! Modeled on the `physician-auth` table families: accounts, non-expiring
//! sessions per sign-in, one-time sign-in tokens, practice invitations to
//! roster patients, and canonical multi-practice connections keyed by
//! `(account_id, practice_id, patient_id)`.

use sea_orm_migration::prelude::*;

// The entity models below carry field docs; the generated struct and relation
// types are mechanical SeaORM shapes, so the crate-wide doc lint stays off
// for this module's generated names.
#[allow(missing_docs)]
pub mod account;

#[allow(missing_docs)]
pub mod connection;
#[allow(missing_docs)]
pub mod for_my_doctor_share;
#[allow(missing_docs)]
pub mod identity;
mod m20261020_identity;
mod m20261021_duet_conversation;
mod m20261021_luna_journal;
mod m20261022_duet_execution;
mod m20261023_duet_admission;
mod m20261112_drop_premium_cadence;
mod m20261119_for_my_doctor_shares;
mod m20261120_multi_practice_connections;
#[allow(missing_docs)]
pub mod permission;
#[allow(missing_docs)]
pub mod restriction;
#[allow(missing_docs)]
pub mod shared_record;

#[allow(missing_docs)]
pub mod invitation;

#[allow(missing_docs)]
pub mod session;

#[allow(missing_docs)]
pub mod sign_in_token;

pub use account::Model as AccountRow;
pub use account::{ActiveModel as AccountActiveModel, Entity as Accounts};
pub use connection::Model as ConnectionRow;
pub use connection::{ActiveModel as ConnectionActiveModel, Entity as Connections};
pub use for_my_doctor_share::Model as ForMyDoctorShareRow;
pub use for_my_doctor_share::{
    ActiveModel as ForMyDoctorShareActiveModel, Entity as ForMyDoctorShares,
};
pub use invitation::Model as InvitationRow;
pub use invitation::{ActiveModel as InvitationActiveModel, Entity as Invitations};
pub use session::Model as SessionRow;
pub use session::{ActiveModel as SessionActiveModel, Entity as Sessions};
pub use shared_record::Model as SharedRecordRow;
pub use shared_record::{ActiveModel as SharedRecordActiveModel, Entity as SharedRecords};
pub use sign_in_token::Model as SignInTokenRow;
pub use sign_in_token::{ActiveModel as SignInTokenActiveModel, Entity as SignInTokens};

/// The table this chain records applied migrations in.
///
/// The practice server runs this chain and `physician-auth`'s on one
/// database, and each chain owns its ledger, so neither reads the other's
/// rows as unknown versions.
pub const MIGRATION_TABLE: &str = "front_door_migrations";

/// The stable name the init records in [`MIGRATION_TABLE`].
pub const INIT_MIGRATION_NAME: &str = "front_door_20260101_000001_init";

/// Every migration, oldest first.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> DynIden {
        Alias::new(MIGRATION_TABLE).into_iden()
    }
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_init),
            Box::new(m20261020_identity::Migration),
            Box::new(m20261021_luna_journal::Migration),
            Box::new(m20261021_duet_conversation::Migration),
            Box::new(m20261022_duet_execution::Migration),
            Box::new(m20261023_duet_admission::Migration),
            Box::new(m20261112_drop_premium_cadence::Migration),
            Box::new(m20261119_for_my_doctor_shares::Migration),
            Box::new(m20261120_multi_practice_connections::Migration),
        ]
    }
}

/// Verified adoption and migration entrypoint.
pub mod upgrade;

#[allow(non_camel_case_types, missing_docs)]
pub struct m20260101_000001_init;
impl MigrationName for m20260101_000001_init {
    fn name(&self) -> &str {
        INIT_MIGRATION_NAME
    }
}

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
                    .col(ColumnDef::new(Alias::new("linked_practice_id")).string())
                    .col(ColumnDef::new(Alias::new("linked_patient_id")).string())
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
                    .name("front_door_accounts_link_idx")
                    .table(Accounts)
                    .col(Alias::new("linked_practice_id"))
                    .col(Alias::new("linked_patient_id"))
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
                        ColumnDef::new(Alias::new("expires_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_sessions_account_fk")
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
                    .name("front_door_sessions_account_idx")
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
                            .name("front_door_sign_in_tokens_account_fk")
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
                    .name("front_door_sign_in_tokens_account_idx")
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
                        ColumnDef::new(invitation::Column::PatientId)
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
                        ColumnDef::new(Alias::new("expires_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("front_door_invitations_roster_idx")
                    .table(Invitations)
                    .col(invitation::Column::PracticeId)
                    .col(invitation::Column::PatientId)
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
                            .to(Accounts, account::Column::Id)
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
                            .to(Accounts, account::Column::Id)
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
                    .col(ColumnDef::new(Alias::new("cadence")).string().not_null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm_migration::seaql_migrations;

    /// SeaORM keys its table by name, so two migrations sharing one fail
    /// every fresh migrate on a duplicate key. No database needed.
    #[test]
    fn every_migration_records_a_distinct_name() {
        let names: Vec<String> = Migrator::migrations()
            .iter()
            .map(|migration| migration.name().to_owned())
            .collect();
        let mut unique = names.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), names.len(), "duplicate names in {names:?}");
    }

    /// Ledger rows are keyed by these names. Renaming one re-runs its
    /// migration, or fails the migrate as an unknown applied version.
    #[test]
    fn recorded_names_stay_what_databases_hold() {
        let names: Vec<String> = Migrator::migrations()
            .iter()
            .map(|migration| migration.name().to_owned())
            .collect();
        assert_eq!(
            names,
            [
                INIT_MIGRATION_NAME,
                "m20261020_identity",
                "m20261021_luna_journal",
                "m20261021_duet_conversation",
                "m20261022_duet_execution",
                "m20261023_duet_admission",
                "m20261112_drop_premium_cadence",
                "m20261119_for_my_doctor_shares",
                "m20261120_multi_practice_connections",
            ]
        );
    }

    /// `physician-auth` shares the database; sharing a ledger makes each
    /// chain see the other's rows.
    #[test]
    fn the_chain_records_apart_from_physician_auth() {
        assert_ne!(
            Migrator::migration_table_name(),
            seaql_migrations::Entity.into_iden()
        );
        assert_ne!(MIGRATION_TABLE, "physician_auth_migrations");
    }
}
