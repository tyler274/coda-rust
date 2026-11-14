//! Typed access to physician runtime tables. The fixed starting schema lives in
//! runtime.sql; later schema changes are migrations in the runtime chain.

/// One physician's unsent composer text for one chat. The table is created
/// by the fixed runtime baseline.
pub(super) mod chat_draft {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_chat_drafts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        /// The chat this text is composed in.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub chat: String,
        #[sea_orm(column_type = "Text")]
        pub text: String,
        /// Every save increments it; a save must name the current one.
        pub revision: i64,
        /// Database-clock seconds of the first save.
        pub created_at: i64,
        /// Database-clock seconds of the latest save.
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One physician's feedback on one of Coda's responses in a chat. The table
/// is created by the runtime chain's `m20261111_response_feedback` migration.
pub(super) mod response_feedback {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_response_feedback")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub chat: String,
        /// The rated response: its `ResponseId`.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub response: String,
        /// `up` or `down`.
        #[sea_orm(column_type = "Text")]
        pub rating: String,
        /// The serialized `Details`; null when the latest save had none.
        #[sea_orm(column_type = "JsonBinary")]
        pub details: Option<Json>,
        /// Database-clock seconds of the latest save.
        pub saved_at: i64,
        /// 1 on the first save; every later save increments it, so the
        /// highest revision a reader has seen is the newest feedback.
        pub revision: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(super) mod conversation {
    use super::super::WorkStatus;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_conversations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub chat: String,
        pub status: WorkStatus,
        pub revision: i64,
        pub session_hash: Option<Vec<u8>>,
        pub state: Json,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(super) mod command {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_commands")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub key: String,
        /// The full command for 90 days, then only `{"salt", "sha256"}`.
        pub command: Json,
        /// Database-clock seconds the command was accepted.
        pub accepted_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One physician's saved reply draft and its latest revision work. The table
/// is created by the fixed runtime baseline.
pub(super) mod reply_draft {
    use super::super::reply::DraftStatus;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_reply_drafts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        /// The reply work this draft answers.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub reply: String,
        pub status: DraftStatus,
        pub revision: i64,
        pub session_hash: Option<Vec<u8>>,
        #[sea_orm(column_type = "JsonBinary")]
        pub state: Json,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One patient workspace's source version. Recording a change bumps it under
/// this row's exclusive lock; a revision commit holds it shared, so a source
/// arriving during a commit either is seen by it or waits for it.
pub(super) mod source_version {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_source_versions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        /// Monotonic; moves only when a recorded observation changes.
        pub version: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Append-only source observations. A correction or withdrawal is a new row
/// at a later version; no row is ever rewritten.
pub(super) mod source {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_sources")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        /// The source owner's opaque candidate ID.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub candidate: String,
        /// The workspace version this observation took effect at.
        #[sea_orm(primary_key, auto_increment = false)]
        pub observed_at_version: i64,
        /// The validated `SourceRevision`; null records a withdrawal.
        #[sea_orm(column_type = "JsonBinary")]
        pub revision: Option<Json>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One source's last delivery for one patient. A later pull replaces the row.
/// The table is created by the fixed runtime baseline.
pub(super) mod source_pull {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_source_pulls")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        /// The source's stable identifier.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        /// The source's name as a physician sees it.
        #[sea_orm(column_type = "Text")]
        pub source_label: String,
        /// The patient as this source identifies them.
        #[sea_orm(column_type = "JsonBinary")]
        pub patient: Json,
        /// The practice clock's Unix second when a complete pull last finished,
        /// or `None` when only partial records have landed.
        pub pulled_at: Option<i64>,
        /// Previously saved candidate IDs from this source absent on this pull.
        #[sea_orm(column_type = "JsonBinary")]
        pub missing_candidates: Json,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Binds each patient workspace to its single owning practice.
pub(super) mod workspace_practice {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_workspace_practices")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(column_type = "Text")]
        pub practice: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Binds one source patient within a practice to one Chords workspace across pulls.
pub(super) mod source_patient_link {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_source_patient_links")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_patient_id: String,
        #[sea_orm(column_type = "Text")]
        pub workspace: String,
        pub linked_at: i64,
        pub issued_pull_seq: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Practice-level connection access standing for one source adapter.
pub(super) mod practice_source_access {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_practice_source_access")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub access: Json,
        pub updated_at: i64,
        pub access_seq: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Per-patient source pull status, retaining `last_good_read_at` across failures.
pub(super) mod patient_source_status {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_patient_source_status")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        pub last_good_read_at: Option<i64>,
        #[sea_orm(column_type = "JsonBinary")]
        pub attempt: Option<Json>,
        pub applied_pull_seq: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable execution state and persisted receipt for an idempotent `reconnect_source` command.
pub(super) mod reconnect_command {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_reconnect_commands")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub key: String,
        #[sea_orm(column_type = "Text")]
        pub source_id: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub state: Json,
        #[sea_orm(column_type = "Text")]
        pub lease_owner: Option<String>,
        pub lease_until: Option<i64>,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable adapter-owned connector state and pre-checkpoint reconnect receipts
/// shared across instances per `(practice, source_id)`.
pub(super) mod source_adapter_state {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_source_adapter_state")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub state: Json,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Immutable revision metadata, written when the next revision is accepted.
/// Attempts and execution evidence retain their own rows across archival.
pub(super) mod reply_revision {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_reply_revisions")]
    pub struct Model {
        #[sea_orm(
            primary_key,
            auto_increment = false,
            column_type = "Text",
            unique_key = "draft_ordinal"
        )]
        pub practice: String,
        #[sea_orm(
            primary_key,
            auto_increment = false,
            column_type = "Text",
            unique_key = "draft_ordinal"
        )]
        pub physician: String,
        #[sea_orm(
            primary_key,
            auto_increment = false,
            column_type = "Text",
            unique_key = "draft_ordinal"
        )]
        pub workspace: String,
        #[sea_orm(
            primary_key,
            auto_increment = false,
            column_type = "Text",
            unique_key = "draft_ordinal"
        )]
        pub reply: String,
        /// The revision command's idempotency key, unique per physician.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub key: String,
        /// Database-clock seconds when the next revision replaced this one.
        pub archived_at: i64,
        /// Archive order within this draft, from 1, assigned under the draft
        /// row's lock. Reads order by it: `archived_at` has one-second
        /// resolution and `key` is an arbitrary client string.
        #[sea_orm(unique_key = "draft_ordinal")]
        pub ordinal: i64,
        /// The serialized `EarlierRevision`.
        #[sea_orm(column_type = "JsonBinary")]
        pub state: Json,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Mutable chat attempt control, without any execution history.
pub(super) mod chat_attempt {
    use super::super::AttemptStatus;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_chat_attempts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub chat: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub position: i64,
        pub predecessor: Option<String>,
        pub work: String,
        pub revision: i64,
        pub status: AttemptStatus,
        pub expires: i64,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

/// Revision attempt control and its source pins. Events are separate rows.
pub(super) mod revision_attempt {
    use super::super::AttemptStatus;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_revision_attempts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub reply: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub input_id: String,
        pub position: i64,
        pub status: AttemptStatus,
        pub expires: i64,
        pub sources_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub sources: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub refreshes: Json,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

/// One acknowledged fact. No update path exists for these rows.
pub(super) mod execution_event {
    use super::super::records::Owner;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_events")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub target: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub owner: Owner,
        #[sea_orm(primary_key, auto_increment = false)]
        pub attempt: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub sequence: i64,
        pub ordinal: i64,
        pub admissible: bool,
        #[sea_orm(column_type = "JsonBinary")]
        pub record: Json,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

/// One product-wide provider cooldown, shared by hosted worker replicas.
pub(super) mod provider_quota {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_provider_quota")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub singleton: bool,
        /// Database-clock Unix milliseconds before the provider may be called.
        pub not_before_ms: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Immutable content retained once within the physician's authorization scope.
pub(super) mod frozen_recipe {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_recipes")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub body: Json,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

/// Immutable content retained once within the physician's authorization scope.
pub(super) mod skill_package {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_skill_packages")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub physician: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub body: Json,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

/// One clinical concern Coda raised for a patient workspace in a practice.
/// Created by the runtime chain's `m20261112_concerns` migration.
pub(super) mod concern {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_concerns")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub id: String,
        /// `now` or `today`.
        #[sea_orm(column_type = "Text")]
        pub urgency: String,
        /// `open`, `done`, or `retired`.
        #[sea_orm(column_type = "Text")]
        pub status: String,
        /// Why Coda thinks the physician should see this concern.
        #[sea_orm(column_type = "Text")]
        pub reason: String,
        /// Serialized `Vec<ExactRecordReference>`.
        #[sea_orm(column_type = "JsonBinary")]
        pub records: Json,
        /// Serialized `SourceWork` that raised or most recently revised the concern.
        #[sea_orm(column_type = "JsonBinary")]
        pub source_work: Json,
        /// Physician who cleared the concern as `Done`, present iff `status = 'done'`.
        #[sea_orm(column_type = "Text")]
        pub cleared_by_physician: Option<String>,
        /// Database-clock seconds when cleared `Done`, present iff `status = 'done'`.
        pub cleared_at: Option<i64>,
        /// Feedback revision recorded with `Done`, present iff `status = 'done'`.
        pub feedback_revision: Option<i64>,
        /// One-line retirement reason for Updates, present iff `status = 'retired'`.
        #[sea_orm(column_type = "Text")]
        pub retirement_reason: Option<String>,
        /// Serialized `SourceWork` that retired the concern, present iff `status = 'retired'`.
        #[sea_orm(column_type = "JsonBinary")]
        pub retired_by_work: Option<Json>,
        /// Database-clock seconds when retired, present iff `status = 'retired'`.
        pub retired_at: Option<i64>,
        /// Database-clock seconds when first raised.
        pub created_at: i64,
        /// Database-clock seconds of the latest mutation.
        pub updated_at: i64,
        /// 1 when raised; incremented on every revise, retire, or `Done`.
        pub revision: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable physician feedback recorded when a concern is cleared as `Done`,
/// kept with the concern snapshot for evaluation. Created by `m20261112_concerns`.
pub(super) mod concern_feedback {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_concern_feedback")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub concern: String,
        /// Physician who cleared the concern `Done`.
        #[sea_orm(column_type = "Text")]
        pub physician: String,
        /// `done`.
        #[sea_orm(column_type = "Text")]
        pub action: String,
        /// Revision of the concern after clearing.
        pub concern_revision: i64,
        /// `now` or `today` snapshot when cleared.
        #[sea_orm(column_type = "Text")]
        pub urgency: String,
        /// Concern statement snapshot when cleared.
        #[sea_orm(column_type = "Text")]
        pub concern_reason: String,
        /// Cited `ExactRecordReference` list when cleared.
        #[sea_orm(column_type = "JsonBinary")]
        pub records: Json,
        /// Originating `SourceWork` snapshot when cleared.
        #[sea_orm(column_type = "JsonBinary")]
        pub source_work: Json,
        /// Database-clock seconds when the feedback was recorded.
        pub saved_at: i64,
        /// 1 on the first save; incremented if ever updated.
        pub revision: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One practice-owned background job for one patient workspace, with no
/// physician session. Created by `m20261112_practice_jobs_dispatch`.
pub(super) mod practice_job {
    use super::super::WorkStatus;
    use super::super::jobs::PracticeJobKind;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_practice_jobs")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub job_id: String,
        pub kind: PracticeJobKind,
        pub status: WorkStatus,
        #[sea_orm(column_type = "Text")]
        pub work: String,
        pub revision: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub assignment: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub preloaded_skills: Json,
        #[sea_orm(column_type = "Text")]
        pub skills: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub binding: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub runs: Json,
        pub failures: i64,
        pub allowance_after: i64,
        pub sources_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub sources: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub result: Option<Json>,
        pub dispatch_after: i64,
        pub created_at: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Mutable attempt control for a practice-owned background job.
pub(super) mod practice_job_attempt {
    use super::super::AttemptStatus;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_practice_job_attempts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub job_id: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub id: String,
        pub position: i64,
        #[sea_orm(column_type = "Text")]
        pub predecessor: Option<String>,
        #[sea_orm(column_type = "Text")]
        pub work: String,
        pub revision: i64,
        pub status: AttemptStatus,
        pub expires: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Per-patient source check and review cadence state for a practice.
/// `cadence` here is the sole stored cadence field across the product.
pub(super) mod patient_dispatch {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_patient_dispatch")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub cadence: Json,
        pub registered_at: i64,
        pub last_attempt_at: Option<i64>,
        pub last_success_at: Option<i64>,
        pub last_checked_version: Option<i64>,
        #[sea_orm(column_type = "JsonBinary")]
        pub last_checked_records: Json,
        pub last_reviewed_at: Option<i64>,
        pub last_reviewed_version: Option<i64>,
        pub review_sequence: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub pending_review: Option<Json>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
