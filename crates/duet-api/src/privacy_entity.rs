//! Duet-owned SeaORM mappings for privacy restrictions, outbox receipts, and replay.
//!
//! The schema is owned by `crates/duet-api/migrations` through `duet-migrate`;
//! these entities never create or alter it. Only the columns read or written by
//! [`crate::privacy`] are mapped, and every statement runs on the caller's
//! account-scoped transaction (`AccountDatabase::begin`).

use sea_orm::entity::prelude::*;

/// Source family targeted by a privacy source withdrawal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum WithdrawnSourceKind {
    /// Device-uploaded Apple Health records.
    #[sea_orm(string_value = "appleHealth")]
    AppleHealth,
    /// Imported clinical Health Records.
    #[sea_orm(string_value = "healthRecords")]
    HealthRecords,
}

/// Clinical note provenance label targeted by Health Records withdrawal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum NoteSourceLabel {
    /// Imported clinical Health Records note.
    #[sea_orm(string_value = "Health Records")]
    HealthRecords,
    /// Patient-authored note added directly in Duet.
    #[sea_orm(string_value = "Added in Duet")]
    AddedInDuet,
}

/// Privacy-relevant columns on `public.users`.
pub mod user_privacy {
    use sea_orm::entity::prelude::*;

    /// Current Luna consent version recorded when withdrawing consent.
    pub const LUNA_CONSENT_VERSION: &str = "2026-09-24";

    /// `users.luna_consent_decision` values written by consent and withdrawal.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "Text")]
    pub enum LunaConsentDecision {
        /// Patient granted Luna consent.
        #[sea_orm(string_value = "accepted")]
        Accepted,
        /// Patient withdrew or declined Luna consent.
        #[sea_orm(string_value = "declined")]
        Declined,
    }

    /// Mapped privacy subset of one `public.users` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "users")]
    pub struct Model {
        /// Internal account identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Immutable authentication provider subject.
        pub auth_user_id: String,
        /// Account incarnation that privacy receipts must match.
        pub duet_incarnation: Uuid,
        /// Whether Luna intelligence features are enabled.
        pub intelligence_enabled: bool,
        /// Recorded Luna consent decision.
        pub luna_consent_decision: Option<LunaConsentDecision>,
        /// Recorded Luna consent policy version.
        pub luna_consent_version: Option<String>,
        /// Timestamp of the latest Luna consent decision.
        pub luna_consent_at: Option<DateTimeUtc>,
        /// Monotonic revision fencing Luna consent and withdrawal.
        pub intelligence_control_revision: i64,
        /// Cutoff at or before which authenticated sessions are revoked.
        pub sessions_revoked_before: Option<DateTimeUtc>,
        /// Patient's saved doctor note prose.
        pub doctor_note: Option<String>,
        /// Patient's approved doctor page JSON object.
        pub doctor_page: Json,
        /// Pending staged doctor page proposal, when present.
        pub doctor_proposal: Option<Json>,
        /// Monotonic generation fencing doctor workspace saves and deletions.
        pub doctor_note_generation: i64,
        /// Timestamp of the latest doctor page update.
        pub doctor_page_updated_at: DateTimeUtc,
        /// Monotonic account cursor for canonical evidence mutations.
        pub evidence_revision: i64,
        /// Monotonic generation fencing Apple Health uploads and withdrawals.
        pub apple_health_generation: i32,
        /// Timestamp when Apple Health was last disconnected.
        pub apple_health_disconnected_at: Option<DateTimeUtc>,
        /// Monotonic generation fencing Health Records uploads and withdrawals.
        pub health_records_generation: i32,
        /// Timestamp when Health Records was last disconnected.
        pub health_records_disconnected_at: Option<DateTimeUtc>,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable SQL outbox rows in `public.privacy_recovery_events`.
pub mod privacy_event {
    use crate::privacy::Restriction;
    use sea_orm::entity::prelude::*;

    /// One `public.privacy_recovery_events` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "privacy_recovery_events")]
    pub struct Model {
        /// Stable caller or sub-effect receipt event identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account identity.
        pub account_id: Uuid,
        /// Typed restriction kind.
        pub operation: Restriction,
        /// Restricted account or resource identity.
        pub scope_id: Uuid,
        /// Restricted revision encoded as text for schema compatibility.
        pub scope_version: String,
        /// Original server timestamp when the restriction was recorded.
        pub occurred_at: DateTimeUtc,
        /// Timestamp when independent journal readback succeeded.
        pub journaled_at: Option<DateTimeUtc>,
        /// Account incarnation bound into the v1 receipt.
        pub account_incarnation: Option<Uuid>,
        /// Canonical JSON receipt bytes verified against the independent journal.
        pub receipt_bytes: Option<Vec<u8>>,
        /// Delivery attempt count.
        pub attempts: i32,
        /// Earliest time for the next recovery delivery attempt.
        pub next_attempt_at: DateTimeUtc,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Replayed legacy v1 receipts in `public.duet_legacy_privacy_replays`.
pub mod legacy_replay {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.duet_legacy_privacy_replays` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_legacy_privacy_replays")]
    pub struct Model {
        /// Legacy receipt event identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub event_id: Uuid,
        /// Owning account identity.
        pub account_id: Uuid,
        /// Original unmodified legacy receipt bytes.
        pub receipt_bytes: Vec<u8>,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Migration-admitted legacy cohort accounts in `public.duet_legacy_privacy_accounts`.
pub mod legacy_account {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.duet_legacy_privacy_accounts` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_legacy_privacy_accounts")]
    pub struct Model {
        /// Account identity admitted at cohort migration time.
        #[sea_orm(primary_key, auto_increment = false)]
        pub account_id: Uuid,
        /// Account incarnation captured during cohort migration.
        pub incarnation: Uuid,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Tombstones blocking stale subject reprovisioning in `public.duet_deleted_identities`.
pub mod deleted_identity {
    use sea_orm::entity::prelude::*;

    /// One `public.duet_deleted_identities` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_deleted_identities")]
    pub struct Model {
        /// External authentication provider subject.
        #[sea_orm(primary_key, auto_increment = false)]
        pub auth_user_id: String,
        /// Deleted internal account identity.
        pub account_id: Uuid,
        /// Deleted account incarnation.
        pub incarnation: Uuid,
        /// Timestamp of the account deletion receipt.
        pub deleted_at: DateTimeUtc,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Selected items on the doctor workspace in `public.doctor_collection_items`.
pub mod doctor_item {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.doctor_collection_items` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "doctor_collection_items")]
    pub struct Model {
        /// Selected item identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account identity.
        pub user_id: Uuid,
        /// Canonical record selection JSON when backed by canonical evidence.
        pub record_selection: Option<Json>,
        /// Device fact snapshot identity when backed by a device snapshot.
        pub fact_snapshot_id: Option<Uuid>,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Idempotent save receipts for the doctor page in `public.duet_doctor_saves`.
pub mod doctor_save {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.duet_doctor_saves` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_doctor_saves")]
    pub struct Model {
        /// Owning account identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Workspace generation fenced by the save.
        #[sea_orm(primary_key, auto_increment = false)]
        pub expected_generation: i64,
        /// Saved collection response JSON.
        pub result: Json,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Staged doctor workspace proposals in `public.duet_doctor_proposals`.
pub mod doctor_proposal {
    use sea_orm::entity::prelude::*;

    /// `duet_doctor_proposals.state`, as its check constraint allows.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "Text")]
    pub enum State {
        /// Waiting for patient review.
        #[sea_orm(string_value = "pending")]
        Pending,
        /// Accepted into a saved doctor page.
        #[sea_orm(string_value = "approved")]
        Approved,
        /// Explicitly rejected by the patient.
        #[sea_orm(string_value = "rejected")]
        Rejected,
        /// Replaced or invalidated by a newer workspace change or item deletion.
        #[sea_orm(string_value = "superseded")]
        Superseded,
    }

    /// Mapped subset of one `public.duet_doctor_proposals` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_doctor_proposals")]
    pub struct Model {
        /// Owning account identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Proposal identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Proposed doctor collection JSON.
        pub proposal: Json,
        /// Proposal lifecycle state.
        pub state: State,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Uploaded device fact snapshots in `public.duet_device_snapshots`.
pub mod device_snapshot {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.duet_device_snapshots` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_device_snapshots")]
    pub struct Model {
        /// Owning account identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Snapshot identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Pinned external doctor page publications in `public.duet_publications`.
pub mod publication {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.duet_publications` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_publications")]
    pub struct Model {
        /// Publication identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account identity.
        pub user_id: Uuid,
        /// Frozen snapshot JSON containing published items.
        pub snapshot: Json,
        /// Revocation timestamp; `None` while active.
        pub revoked_at: Option<DateTimeUtc>,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
