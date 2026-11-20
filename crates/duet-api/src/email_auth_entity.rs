//! SeaORM entity mappings for Duet email challenges, non-expiring sessions, and preferences.

use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

/// Challenge intent: standalone sign-in/sign-up or verified change-email confirmation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum ChallengePurpose {
    /// Sign in or create a new Duet account from an email address.
    #[sea_orm(string_value = "sign_in")]
    SignIn,
    /// Confirm a new verified email address for the signed-in account.
    #[sea_orm(string_value = "email_change")]
    EmailChange,
}

impl ChallengePurpose {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SignIn => "sign_in",
            Self::EmailChange => "email_change",
        }
    }
}

/// User-selected Duet appearance preference; `System` follows the phone's light/dark mode.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Appearance {
    /// Follow device appearance.
    #[default]
    #[sea_orm(string_value = "system")]
    System,
    /// Force light surfaces.
    #[sea_orm(string_value = "light")]
    Light,
    /// Force dark surfaces.
    #[sea_orm(string_value = "dark")]
    Dark,
}

/// Read-only projection of `public.duet_email_directory`.
pub mod email_directory {
    use super::Appearance;
    use sea_orm::entity::prelude::*;

    /// Minimal directory row visible before entering RLS account scope.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_email_directory")]
    pub struct Model {
        /// Internal Duet account UUID (`public.users.id`).
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Provider subject (`email:<uuid>` for email-first accounts).
        pub auth_user_id: String,
        /// Verified account email when present.
        pub email: Option<String>,
        /// Stored appearance preference.
        pub appearance: Appearance,
    }

    /// Entity relations (none).
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Account-scoped profile fields on `public.users`.
pub mod user_profile {
    use super::Appearance;
    use sea_orm::entity::prelude::*;

    /// Profile and session-fence columns on `public.users`.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "users")]
    pub struct Model {
        /// Internal Duet account UUID.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Provider subject string.
        pub auth_user_id: String,
        /// Verified account email.
        pub email: Option<String>,
        /// Optional display name.
        pub display_name: Option<String>,
        /// Chosen UI appearance.
        pub appearance: Appearance,
        /// Sharing label required on `users`.
        pub sharing_label: String,
        /// Active account incarnation.
        pub duet_incarnation: Uuid,
        /// Current permission revision.
        pub intelligence_control_revision: i64,
        /// Revocation cutoff for global sign-out everywhere.
        pub sessions_revoked_before: Option<DateTimeUtc>,
        /// Latest accepted terms version.
        pub terms_version: Option<String>,
    }

    /// Entity relations (none).
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Single-use email verification challenges (`public.duet_email_challenges`).
pub mod email_challenge {
    use super::ChallengePurpose;
    use sea_orm::entity::prelude::*;

    /// One issued single-use magic link + 6-digit fallback code.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_email_challenges")]
    pub struct Model {
        /// Challenge primary key.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Sign-in or verified email change.
        pub purpose: ChallengePurpose,
        /// Normalized lower-case email recipient.
        pub email: String,
        /// Owning user for email-change challenges; null for sign-in challenges.
        pub user_id: Option<Uuid>,
        /// SHA-256 digest of the single-use URL token.
        pub link_token_hash: Vec<u8>,
        /// SHA-256 digest of `(purpose, email, short_code)`.
        pub short_code_hash: Vec<u8>,
        /// Expiry timestamp of the verification link/code only.
        pub expires_at: DateTimeUtc,
        /// Non-null once consumed.
        pub consumed_at: Option<DateTimeUtc>,
        /// Number of failed code guesses against this challenge.
        pub attempts: i32,
        /// Creation timestamp.
        pub created_at: DateTimeUtc,
    }

    /// Entity relations (none).
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Non-expiring device sessions (`public.duet_sessions`).
pub mod duet_session {
    use sea_orm::entity::prelude::*;

    /// Opaque session token record with explicit device sign-out or global revocation only.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_sessions")]
    pub struct Model {
        /// Session primary key.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account ID (`public.users.id`; retained as tombstone across account deletion for receipt retry).
        pub user_id: Uuid,
        /// Owning account incarnation UUID at session creation.
        pub account_incarnation: Uuid,
        /// SHA-256 digest of the `ds1.` opaque session token.
        pub token_hash: Vec<u8>,
        /// Session creation timestamp checked against `sessions_revoked_before`.
        pub created_at: DateTimeUtc,
        /// Non-null after explicit sign-out on this device.
        pub revoked_at: Option<DateTimeUtc>,
    }

    /// Entity relations (none).
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
