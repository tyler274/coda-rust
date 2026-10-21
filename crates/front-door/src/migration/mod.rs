//! The SeaORM entities and the front-door migration chain.
//!
//! Modeled on the `physician-auth` table families: accounts, one session per
//! sign-in, one-time sign-in tokens, practice invitations to roster patients,
//! the durable Duet-account-to-roster-patient link, per-patient consent, and
//! per-patient premium status. Deliberate divergence from `physician-auth`:
//! the link and premium rows outlive a link's unbinding (`consents` keeps its
//! history through `withdrawn_at`), because what was already shared stays
//! dated evidence and re-linking must not resurrect withdrawn consent.

use sea_orm_migration::prelude::*;

// The entity models below carry field docs; the generated struct and relation
// types are mechanical SeaORM shapes, so the crate-wide doc lint stays off
// for this module's generated names.
#[allow(missing_docs)]
pub mod account;

#[allow(missing_docs)]
pub mod consent;

#[allow(missing_docs)]
pub mod invitation;

#[allow(missing_docs)]
pub mod link;

#[allow(missing_docs)]
pub mod premium;

#[allow(missing_docs)]
pub mod session;

#[allow(missing_docs)]
pub mod sign_in_token;

pub use account::Model as AccountRow;
pub use account::{ActiveModel as AccountActiveModel, Entity as Accounts};
pub use consent::Model as ConsentRow;
pub use consent::{ActiveModel as ConsentActiveModel, Entity as Consents};
pub use invitation::Model as InvitationRow;
pub use invitation::{ActiveModel as InvitationActiveModel, Entity as Invitations};
pub use link::Model as LinkRow;
pub use link::{ActiveModel as LinkActiveModel, Entity as Links};
pub use premium::Model as PremiumRow;
pub use premium::{ActiveModel as PremiumActiveModel, Entity as Premiums};
pub use session::Model as SessionRow;
pub use session::{ActiveModel as SessionActiveModel, Entity as Sessions};
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
        vec![Box::new(m20260101_000001_init)]
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
                    .col(ColumnDef::new(account::Column::LinkedPracticeId).string())
                    .col(ColumnDef::new(account::Column::LinkedPatientId).string())
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
                    .col(account::Column::LinkedPracticeId)
                    .col(account::Column::LinkedPatientId)
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
                        ColumnDef::new(invitation::Column::ExpiresAt)
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
                    .table(Links)
                    .col(
                        ColumnDef::new(link::Column::AccountId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(link::Column::PracticeId).string().not_null())
                    .col(ColumnDef::new(link::Column::PatientId).string().not_null())
                    .col(
                        ColumnDef::new(link::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_links_account_fk")
                            .from(Links, link::Column::AccountId)
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
                    .table(Links)
                    .col(link::Column::PracticeId)
                    .col(link::Column::PatientId)
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Consents)
                    .col(
                        ColumnDef::new(consent::Column::AccountId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(consent::Column::ConsentedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(consent::Column::WithdrawnAt).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("front_door_consents_account_fk")
                            .from(Consents, consent::Column::AccountId)
                            .to(Accounts, account::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Premiums)
                    .col(
                        ColumnDef::new(premium::Column::PatientKey)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(premium::Column::Premium)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(premium::Column::Cadence).string().not_null())
                    .col(
                        ColumnDef::new(premium::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Premiums).take())
            .await?;
        manager
            .drop_table(Table::drop().table(Consents).take())
            .await?;
        manager
            .drop_table(Table::drop().table(Links).take())
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
        assert_eq!(names, [INIT_MIGRATION_NAME]);
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
