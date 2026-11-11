//! Duet-owned SeaORM mappings of the canonical evidence relations.
//!
//! The schema is owned by `crates/duet-api/migrations` through `duet-migrate`;
//! these entities never create or alter it. Only the columns the canonical
//! mutation owner reads or writes are mapped. Statements built from them run on
//! the caller's account transaction (`AccountDatabase::begin`, which sets the
//! RLS account), never on a separate SeaORM connection, so the account lock,
//! row-level security and atomic cursor advance stay in one transaction.

/// Stable operation identity and its first acknowledged address.
pub mod ingestion {
    use sea_orm::entity::prelude::*;

    /// One `public.duet_evidence_ingestions` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_evidence_ingestions")]
    pub struct Model {
        /// Owning account.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Client-chosen, retry-stable operation identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub operation_id: Uuid,
        /// SHA-256 of the complete mutation and its authority.
        pub request_sha256: String,
        /// Acknowledged exact address.
        pub address: Json,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Current membership and change cursor of one canonical record.
pub mod membership {
    use sea_orm::entity::prelude::*;

    /// One `public.evidence_sync_changes` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "evidence_sync_changes")]
    pub struct Model {
        /// Owning account.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// `clinical_note` or `health_evidence`.
        #[sea_orm(primary_key, auto_increment = false, column_name = "entity")]
        pub entity_kind: String,
        /// Canonical identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub record_id: Uuid,
        /// Literal original source identity.
        pub client_record_id: String,
        /// Account mutation cursor of the latest membership change.
        pub revision: i64,
        /// No longer current evidence (source removal or privacy erasure).
        pub deleted: bool,
        /// Removed by its source; retained revisions stay readable.
        pub source_removed: bool,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Immutable retained source revisions.
pub mod revision {
    use sea_orm::entity::prelude::*;

    /// One `public.canonical_evidence_revisions` row (mapped columns only).
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "canonical_evidence_revisions")]
    pub struct Model {
        /// Identity-generated immutable revision.
        #[sea_orm(primary_key)]
        pub revision: i64,
        /// Owning account.
        pub user_id: Uuid,
        /// Health evidence base row, when this is health evidence.
        pub evidence_id: Option<Uuid>,
        /// Clinical note base row, when this is a note.
        pub note_id: Option<Uuid>,
        /// Literal source object.
        pub payload: Json,
        /// SHA-256 of the payload bytes.
        pub content_sha256: String,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Current health evidence base rows.
pub mod health_evidence {
    use sea_orm::entity::prelude::*;

    /// One `public.health_evidence` row (mapped columns only).
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "health_evidence")]
    pub struct Model {
        /// Canonical identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account.
        pub user_id: Uuid,
        /// Concept identity.
        pub concept_id: String,
        /// Source family that owns this record, fixed at creation.
        pub source_kind: String,
        /// Source connection, fixed at creation.
        pub source_connection_id: Option<String>,
        /// Source-native record identity.
        pub source_record_id: Option<String>,
        /// Declared source instant; `None` when the source did not date it.
        pub measured_at: Option<DateTimeUtc>,
        /// Server receipt time of the latest revision.
        pub received_at: DateTimeUtc,
        /// Latest literal source object.
        pub payload: Json,
        /// Last mutation time.
        pub updated_at: DateTimeUtc,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Current clinical note base rows.
pub mod clinical_note {
    use sea_orm::entity::prelude::*;

    /// One `public.clinical_notes` row (mapped columns only).
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "clinical_notes")]
    pub struct Model {
        /// Canonical identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account.
        pub user_id: Uuid,
        /// Source-native identity.
        pub source_id: Option<String>,
        /// Literal title.
        pub title: String,
        /// Literal preview.
        pub preview: Option<String>,
        /// Literal body.
        pub body: Option<String>,
        /// Declared source instant; `None` when undated.
        pub recorded_at: Option<DateTimeUtc>,
        /// Provenance label, fixed at creation.
        pub source_label: String,
        /// Latest literal source object.
        pub duet_source_payload: Option<Json>,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// The account cursor, source-generation fences and incarnation on `public.users`.
pub mod account {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.users` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "users")]
    pub struct Model {
        /// Account identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Account-serialized evidence cursor.
        pub evidence_revision: i64,
        /// Apple Health source generation.
        pub apple_health_generation: i32,
        /// Health Records source generation.
        pub health_records_generation: i32,
        /// Account incarnation that privacy receipts must name.
        pub duet_incarnation: Uuid,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
