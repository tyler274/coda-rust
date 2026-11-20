//! SeaORM entity mappings for `"For my doctor"` (`duet_for_my_doctor_*`).

use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

/// What a patient marked in `You` for `To send`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum MarkTargetKind {
    /// A full health record or medication.
    #[sea_orm(string_value = "record")]
    Record,
    /// One specific value inside a multi-value record (e.g. systolic BP).
    #[sea_orm(string_value = "value")]
    Value,
    /// One dated measurement or lab reading.
    #[sea_orm(string_value = "reading")]
    Reading,
    /// A trend series in its currently viewed date range.
    #[sea_orm(string_value = "trend")]
    Trend,
}

/// Canonical evidence table family referenced by a mark or linked note.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum RecordFamily {
    /// Row in `public.health_evidence`.
    #[sea_orm(string_value = "health_evidence")]
    HealthEvidence,
    /// Row in `public.clinical_notes`.
    #[sea_orm(string_value = "clinical_note")]
    ClinicalNote,
}

impl RecordFamily {
    /// Stable database identifier matching `evidence_sync_changes.entity`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HealthEvidence => "health_evidence",
            Self::ClinicalNote => "clinical_note",
        }
    }
}

/// Delivery state of one immutable `"For my doctor"` send.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum StoredSendStanding {
    /// Saved by Duet and sent to the practice; awaiting agent pickup.
    #[sea_orm(string_value = "sent")]
    Sent,
    /// Picked up by the practice's agents (`"Received by the care team"`).
    #[sea_orm(string_value = "received")]
    Received,
}

/// Account-scoped marked items waiting in `To send`.
pub mod mark {
    use super::{MarkTargetKind, RecordFamily};
    use sea_orm::entity::prelude::*;

    /// One `public.duet_for_my_doctor_marks` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_for_my_doctor_marks")]
    pub struct Model {
        /// Mark identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning Duet account (`users.id`).
        pub user_id: Uuid,
        /// Kind of marked item (`record`, `value`, `reading`, `trend`).
        pub target_kind: MarkTargetKind,
        /// Deduplicating key scoped to the account.
        pub target_key: String,
        /// Referenced canonical record ID when linked to canonical evidence.
        pub record_id: Option<Uuid>,
        /// Canonical evidence family (`health_evidence` or `clinical_note`).
        pub entity: Option<RecordFamily>,
        /// Current canonical revision observed for `record_id`.
        pub record_revision: Option<i32>,
        /// Canonical revision when the patient first marked this item.
        pub marked_revision: Option<i32>,
        /// Human-readable title shown in `To send` and sends.
        pub title: String,
        /// Plain-language value/reading summary (without lab reference ranges).
        pub summary_text: String,
        /// Viewed date range label for marked trends.
        pub range_label: Option<String>,
        /// Plain source name (`"Epic"`, `"Apple Health"`, etc.).
        pub source_label: Option<String>,
        /// Whether the source updated the record after it was marked.
        pub source_corrected: bool,
        /// Timestamp when a post-mark source correction was recorded.
        pub correction_date: Option<DateTimeUtc>,
        /// Reference-range-free snapshot payload for readings or trends.
        pub payload: Json,
        /// Timestamp when marked.
        pub marked_at: DateTimeUtc,
        /// Timestamp of the last mark/source refresh.
        pub updated_at: DateTimeUtc,
    }

    /// No ORM navigation.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Account-scoped free-text patient notes in `To send`.
pub mod note {
    use super::RecordFamily;
    use sea_orm::entity::prelude::*;

    /// One `public.duet_for_my_doctor_notes` row.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_for_my_doctor_notes")]
    pub struct Model {
        /// Note identity (client-generated for idempotent creation).
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning Duet account (`users.id`).
        pub user_id: Uuid,
        /// Free-text note body.
        pub body: String,
        /// Optional canonical record linked when created from an open record in `You`.
        pub linked_record_id: Option<Uuid>,
        /// Plain label of the linked record.
        pub linked_record_title: Option<String>,
        /// Canonical family of the linked record.
        pub linked_entity: Option<RecordFamily>,
        /// Optimistic concurrency version bumped on each edit.
        pub version: i64,
        /// Creation timestamp.
        pub created_at: DateTimeUtc,
        /// Last edit timestamp.
        pub updated_at: DateTimeUtc,
    }

    /// No ORM navigation.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Account-scoped immutable sends to connected practices.
pub mod send {
    use super::StoredSendStanding;
    use sea_orm::entity::prelude::*;

    /// One `public.duet_for_my_doctor_sends` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_for_my_doctor_sends")]
    pub struct Model {
        /// Local send row identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning Duet account (`users.id`).
        pub user_id: Uuid,
        /// Client-generated idempotency key (`share_key`).
        pub share_key: String,
        /// Target connected practice identifier.
        pub practice_id: String,
        /// Target practice display name.
        pub practice_name: String,
        /// Roster patient identifier within the target practice.
        pub patient_id: String,
        /// Headline shown in the flat `Sent` list.
        pub title: String,
        /// Plain-language frozen send body sent to `front-door`.
        pub summary_text: String,
        /// Complete frozen snapshot (records, versions, correction dates, notes).
        pub snapshot: Json,
        /// Identifiers of marks cleared by this send.
        pub mark_ids: Vec<Uuid>,
        /// Identifiers of notes cleared by this send.
        pub note_ids: Vec<Uuid>,
        /// Front-door share identifier returned on intake.
        pub front_door_share_id: Option<String>,
        /// Delivery standing (`sent` or `received`).
        pub standing: StoredSendStanding,
        /// Timestamp sent.
        pub sent_at: DateTimeUtc,
        /// Timestamp picked up by practice agents, if received while connected.
        pub received_at: Option<DateTimeUtc>,
        /// Timestamp the practice connection ended before pickup, freezing `standing` at `sent`.
        pub connection_ended_at: Option<DateTimeUtc>,
    }

    /// No ORM navigation.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Account-scoped notices shown in `You` when a source removes a marked record.
pub mod notice {
    use sea_orm::entity::prelude::*;

    /// One `public.duet_for_my_doctor_notices` row.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_for_my_doctor_notices")]
    pub struct Model {
        /// Notice identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning Duet account (`users.id`).
        pub user_id: Uuid,
        /// Record title that was removed by its source.
        pub title: String,
        /// Plain-language message shown in `You`.
        pub message: String,
        /// Timestamp dismissed by the patient, if dismissed.
        pub dismissed_at: Option<DateTimeUtc>,
        /// Creation timestamp.
        pub created_at: DateTimeUtc,
    }

    /// No ORM navigation.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
