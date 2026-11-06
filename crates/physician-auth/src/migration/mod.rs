//! The SeaORM entities and the physician-auth migration chain.
//!
//! Modeled on the Supabase GoTrue table families (`users`, `sessions`,
//! `one_time_tokens`), simplified to what the physician journeys use:
//! accounts, one session per sign-in, one-time sign-in tokens, invitations,
//! and per-workspace memberships. Deliberate divergence: one long-lived
//! bearer per sign-in, no refresh-token family or rotation.

use sea_orm_migration::prelude::*;
mod upgrade;
pub use upgrade::migrate;

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
pub mod practice;

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
pub use practice::{ActiveModel as PracticeActiveModel, Entity as Practices};
pub use session::Model as SessionRow;
pub use session::{ActiveModel as SessionActiveModel, Entity as Sessions};
pub use sign_in_token::Model as SignInTokenRow;
pub use sign_in_token::{ActiveModel as SignInTokenActiveModel, Entity as SignInTokens};

/// Every migration, oldest first.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> DynIden {
        Alias::new("physician_auth_migrations").into_iden()
    }
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_init),
            Box::new(m20260101_000002_practice_patients),
            Box::new(m20261023_000003_unique_physician_identity),
            Box::new(m20261024_000004_authority_notify),
            Box::new(m20261030_000005_practices),
            Box::new(m20261106_000006_session_lifetime),
        ]
    }
}

#[allow(non_camel_case_types, missing_docs)]
pub struct m20260101_000001_init;
impl MigrationName for m20260101_000001_init {
    fn name(&self) -> &str {
        "physician_auth_20260101_000001_init"
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

#[allow(non_camel_case_types, missing_docs)]
pub struct m20260101_000002_practice_patients;
impl MigrationName for m20260101_000002_practice_patients {
    fn name(&self) -> &str {
        "physician_auth_20260101_000002_practice_patients"
    }
}

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

/// Reassigns every duplicate `(practice_id, physician_id)` except the earliest
/// row (`created_at`, then `id`). Membership rows stay on the surviving id, so
/// a reassigned account starts with no panel.
pub const REASSIGN_DUPLICATE_PHYSICIAN_IDS: &str = "\
update physician_accounts as account
set physician_id = 'migrated-' || replace(account.id::text, '-', '')
where account.id in (
    select id from (
        select id,
               row_number() over (
                   partition by practice_id, physician_id
                   order by created_at, id
               ) as rank
        from physician_accounts
    ) ranked
    where rank > 1
)";

#[allow(non_camel_case_types, missing_docs)]
pub struct m20261023_000003_unique_physician_identity;
impl MigrationName for m20261023_000003_unique_physician_identity {
    fn name(&self) -> &str {
        "physician_auth_20261023_000003_unique_physician_identity"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for m20261023_000003_unique_physician_identity {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Set-based repair of rows that already share an identity. Ordinary
        // account writes stay on the entity API; this is not a new CRUD path.
        manager
            .get_connection()
            .execute_unprepared(REASSIGN_DUPLICATE_PHYSICIAN_IDS)
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("physician_accounts_practice_physician_uidx")
                    .table(Accounts)
                    .col(account::Column::PracticeId)
                    .col(account::Column::PhysicianId)
                    .unique()
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("physician_accounts_practice_physician_uidx")
                    .table(Accounts)
                    .to_owned(),
            )
            .await
    }
}

/// The authority tables announce every write on the `physician_authority`
/// channel, so each replica's cached reads drop what a write made stale.
///
/// Raw SQL by exception: SeaQuery has no trigger or PL/pgSQL builder. The
/// statements carry no runtime values.
#[allow(non_camel_case_types, missing_docs)]
pub struct m20261030_000005_practices;
impl MigrationName for m20261030_000005_practices {
    fn name(&self) -> &str {
        "physician_auth_20261030_000005_practices"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for m20261030_000005_practices {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Practices)
                    .col(
                        ColumnDef::new(practice::Column::PracticeId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(practice::Column::FixedTime).big_integer())
                    .col(
                        ColumnDef::new(practice::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Practices).to_owned())
            .await
    }
}

/// Rows still on the old 30-day gap (`expires_at = created_at + 2592000`)
/// move to a 12-hour idle deadline (`created_at + 43200`).
///
/// Raw SQL by exception: the repair assigns one column from another. The
/// statement carries no runtime values. SeaQuery has no trigger builder.
pub const CLAMP_SESSION_LIFETIME: &str = "\
UPDATE physician_sessions
   SET expires_at = created_at + 43200
 WHERE expires_at = created_at + 2592000";

/// A session update is an idle slide, not a revocation. Sign-out stays a delete.
const REPLACE_SESSION_NOTIFY: &str = "\
CREATE TRIGGER physician_sessions_authority_notify
    AFTER DELETE OR TRUNCATE ON physician_sessions
    FOR EACH STATEMENT
    EXECUTE FUNCTION physician_authority_notify()";

#[allow(non_camel_case_types, missing_docs)]
pub struct m20261106_000006_session_lifetime;
impl MigrationName for m20261106_000006_session_lifetime {
    fn name(&self) -> &str {
        "physician_auth_20261106_000006_session_lifetime"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for m20261106_000006_session_lifetime {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(CLAMP_SESSION_LIFETIME).await?;
        db.execute_unprepared(
            "DROP TRIGGER IF EXISTS physician_sessions_authority_notify ON physician_sessions",
        )
        .await?;
        db.execute_unprepared(REPLACE_SESSION_NOTIFY).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // New sign-ins store the same 12-hour gap, so the clamp cannot be
        // undone without lengthening those sessions.
        let db = manager.get_connection();
        db.execute_unprepared(
            "DROP TRIGGER IF EXISTS physician_sessions_authority_notify ON physician_sessions",
        )
        .await?;
        db.execute_unprepared(
            "CREATE TRIGGER physician_sessions_authority_notify \
             AFTER UPDATE OR DELETE OR TRUNCATE ON physician_sessions \
             FOR EACH STATEMENT \
             EXECUTE FUNCTION physician_authority_notify()",
        )
        .await?;
        Ok(())
    }
}

#[allow(non_camel_case_types, missing_docs)]
pub struct m20261024_000004_authority_notify;
impl MigrationName for m20261024_000004_authority_notify {
    fn name(&self) -> &str {
        "physician_auth_20261024_000004_authority_notify"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for m20261024_000004_authority_notify {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("../authority_notify.sql"))
            .await
            .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TRIGGER IF EXISTS physician_accounts_authority_notify ON physician_accounts; \
                 DROP TRIGGER IF EXISTS physician_sessions_authority_notify ON physician_sessions; \
                 DROP TRIGGER IF EXISTS physician_memberships_authority_notify ON physician_memberships; \
                 DROP TRIGGER IF EXISTS physician_patients_authority_notify ON physician_patients; \
                 DROP FUNCTION IF EXISTS physician_authority_notify();",
            )
            .await
            .map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm_migration::seaql_migrations;

    /// SeaORM keys its ledger by name, so two migrations sharing one fail
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

    /// The chain no longer records into SeaORM's default table, which the
    /// front door's legacy init also wrote `mod` into.
    #[test]
    fn the_chain_records_apart_from_the_shared_default_table() {
        assert_ne!(
            Migrator::migration_table_name(),
            seaql_migrations::Entity.into_iden()
        );
    }
}
