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

/// One saved version of one patient memory file. Insert and select only: a
/// write adds the next version, and no row is ever changed or removed. The
/// table is created by the runtime chain's `m20261115_patient_memory`.
pub(super) mod memory_version {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_memory_versions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        /// The patient workspace.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        /// The file's `MemoryPath`, as text.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub path: String,
        /// 1 for the first write; each later write adds one.
        #[sea_orm(primary_key, auto_increment = false)]
        pub version: i64,
        /// The file's markdown text at this version; empty for a version
        /// that deletes the file.
        #[sea_orm(column_type = "Text")]
        pub content: String,
        /// Database-clock seconds of the write.
        pub saved_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One note in a patient's log. Insert and select only. The table is created
/// by the runtime chain's `m20261115_patient_memory`.
pub(super) mod memory_note {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_memory_notes")]
    pub struct Model {
        #[sea_orm(
            primary_key,
            auto_increment = false,
            column_type = "Text",
            unique_key = "patient_position"
        )]
        pub practice: String,
        /// The patient workspace.
        #[sea_orm(
            primary_key,
            auto_increment = false,
            column_type = "Text",
            unique_key = "patient_position"
        )]
        pub workspace: String,
        /// The `NoteId`: the run's work and the tool call that added it.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub note: String,
        /// The note's place among the patient's notes, from 1, assigned under
        /// the patient's note lock: the note's number. Reads order by it:
        /// `saved_at` has one-second resolution and the note id is not
        /// ordered.
        #[sea_orm(unique_key = "patient_position")]
        pub position: i64,
        /// Database-clock seconds when the note was saved.
        pub saved_at: i64,
        /// The physician whose run added the note.
        #[sea_orm(column_type = "Text")]
        pub physician: String,
        /// The chat or reply work the run belongs to.
        #[sea_orm(column_type = "Text")]
        pub chat: String,
        /// The id of the message the run answers.
        #[sea_orm(column_type = "Text")]
        pub message: String,
        #[sea_orm(column_type = "Text")]
        pub text: String,
        /// The physician's words, as the model quoted them.
        #[sea_orm(column_type = "Text")]
        pub physician_words: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One rounds mark: a patient rounds run that finished. Insert and select
/// only. The table is created by the runtime chain's
/// `m20261115_patient_memory`.
pub(super) mod memory_rounds {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_memory_rounds")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        /// The patient workspace.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        /// The run's identity.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub run: String,
        /// Unix seconds of the time the run was given as its start.
        pub started_at: i64,
        /// Database-clock seconds when the run finished and this row was
        /// saved.
        pub saved_at: i64,
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
/// Created by the runtime chain's `m20261118_01_concerns` migration.
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
/// kept with the concern snapshot for evaluation. Created by `m20261118_01_concerns`.
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
/// physician session. Created by `m20261118_02_practice_jobs_dispatch`.
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

/// Latest saved refresh status per `(practice, workspace, source_id)`:
/// `updated`, `partly_updated`, or `failed`. Created by `m20261118_03_source_refresh`.
pub(super) mod source_status {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_source_statuses")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(column_type = "Text")]
        pub source_label: String,
        /// `updated`, `partly_updated`, or `failed`.
        #[sea_orm(column_type = "Text")]
        pub outcome: String,
        pub checked_at: i64,
        pub last_good_pulled_at: Option<i64>,
        pub workspace_version: Option<i64>,
        pub changed_records: i64,
        #[sea_orm(column_type = "Text")]
        pub reason: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable once-only per-source refresh receipt, committed in the same
/// transaction as the source's evidence and saved status update so a retry
/// after an interrupted source check never repeats a settled source.
/// Created by `m20261118_03_source_refresh`.
pub(super) mod source_refresh_receipt {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_source_refresh_receipts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub actor: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub command_key: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        pub ordinal: i64,
        #[sea_orm(column_type = "Text")]
        pub request_sha256: String,
        pub workspace_version_after: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub status: Json,
        pub recorded_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Practice schedule source standing, `"as of"` timestamp, and partial/failed
/// refresh diagnostics. Exclusive row lock on `(practice, source_id)` serializes
/// schedule syncs for that source.
pub(super) mod schedule_source {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_schedule_sources")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(column_type = "Text")]
        pub source_label: String,
        pub schedule_version: i64,
        #[sea_orm(column_type = "Text")]
        pub standing: String,
        pub as_of: Option<i64>,
        pub last_complete_as_of: Option<i64>,
        #[sea_orm(column_type = "JsonBinary")]
        pub issues: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub last_failure: Option<Json>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Seeded schedule patient links mapping `(practice, source_id, source_patient_id)`
/// to an existing registered Coda `workspace`.
pub(super) mod schedule_patient_link {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_schedule_patient_links")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_patient_id: String,
        #[sea_orm(column_type = "Text")]
        pub workspace: String,
        pub seeded_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Saved practice schedule appointments, versioned per appointment and matched
/// (or retained unmatched) against seeded schedule patient links.
pub(super) mod schedule_appointment {
    use super::super::schedule::AppointmentStatus;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_appointments")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub appointment_id: String,
        pub saved_version: i64,
        #[sea_orm(column_type = "Text")]
        pub source_revision: String,
        #[sea_orm(column_type = "Text")]
        pub starts_at: String,
        pub starts_at_unix: i64,
        #[sea_orm(column_type = "Text")]
        pub ends_at: String,
        pub ends_at_unix: i64,
        pub status: AppointmentStatus,
        #[sea_orm(column_type = "Text")]
        pub source_status: String,
        #[sea_orm(column_type = "Text")]
        pub label: String,
        #[sea_orm(column_type = "Text")]
        pub match_state: String,
        #[sea_orm(column_type = "Text")]
        pub workspace: Option<String>,
        #[sea_orm(column_type = "Text")]
        pub source_patient_id: Option<String>,
        #[sea_orm(column_type = "JsonBinary")]
        pub dates: Json,
        #[sea_orm(column_type = "Text")]
        pub content: String,
        #[sea_orm(column_type = "Text")]
        pub sha256: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub provenance: Json,
        pub as_of: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable patient understanding per `(practice, workspace)`: private operating
/// index for Coda's runs and physician-facing summary for the patient page.
/// Created by `m20261118_05_patient_understanding`.
pub(super) mod patient_understanding {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_patient_understanding")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        pub revision: i64,
        pub source_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub evidence_scope: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub operating_index: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub summary: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub origin: Json,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Append-only revision and once-only idempotency ledger for patient understanding.
pub(super) mod patient_understanding_revision {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_patient_understanding_revisions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub revision: i64,
        #[sea_orm(column_type = "Text")]
        pub idempotency_key: String,
        pub source_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub evidence_scope: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub operating_index: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub summary: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub origin: Json,
        pub saved_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable patient review record per `(practice, workspace, review_id)` pairing
/// trigger provenance, recipe/source versions, read evidence, authoritative tool
/// actions, controlled outcome (`"nothing to do"` vs assessed), unfinished work,
/// duration, usage (`Unknown` vs `Known`), and lifecycle state with its
/// underlying `physician_runtime_practice_jobs` execution run.
pub(super) mod patient_review {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_patient_reviews")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub review_id: String,
        #[sea_orm(column_type = "Text")]
        pub command_key: String,
        #[sea_orm(column_type = "Text")]
        pub job_id: String,
        #[sea_orm(column_type = "Text")]
        pub work_id: String,
        #[sea_orm(column_type = "Text")]
        pub status: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub patient_snapshot: Option<Json>,
        #[sea_orm(column_type = "JsonBinary")]
        pub triggers: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub accepted_inputs: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub recipe_version: Json,
        pub initial_source_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub initial_sources: Json,
        pub completed_source_version: Option<i64>,
        #[sea_orm(column_type = "JsonBinary")]
        pub completed_sources: Option<Json>,
        #[sea_orm(column_type = "JsonBinary")]
        pub evidence_read: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub actions: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub unfinished_work: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub state: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub usage: Json,
        pub duration_ms: Option<i64>,
        #[sea_orm(column_type = "JsonBinary")]
        pub staged_completion: Option<Json>,
        pub queued_at: i64,
        pub started_at: Option<i64>,
        pub completed_at: Option<i64>,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Per-visit-slot preparation selection decision and execution standing.
/// Created by `m20261118_07_visit_briefings`.
pub(super) mod visit_preparation {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_visit_preparations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub appointment_id: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub starts_at_unix: i64,
        #[sea_orm(column_type = "Text")]
        pub workspace: String,
        pub ends_at_unix: i64,
        pub appointment_saved_version: i64,
        pub observed_source_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub decision: Json,
        pub decided_at: i64,
        #[sea_orm(column_type = "Text")]
        pub selection_job_id: String,
        #[sea_orm(column_type = "Text")]
        pub selection_work_id: String,
        #[sea_orm(column_type = "Text")]
        pub selection_idempotency_key: String,
        #[sea_orm(column_type = "Text")]
        pub state: String,
        #[sea_orm(column_type = "Text")]
        pub briefing_job_id: Option<String>,
        #[sea_orm(column_type = "Text")]
        pub briefing_id: Option<String>,
        #[sea_orm(column_type = "Text")]
        pub failed_reason: Option<String>,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable pre-visit clinical briefing keyed to `(practice, source_id, appointment_id, starts_at_unix)`.
/// Created by `m20261118_07_visit_briefings`.
pub(super) mod visit_briefing {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_visit_briefings")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub appointment_id: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub starts_at_unix: i64,
        #[sea_orm(column_type = "Text")]
        pub briefing_id: String,
        #[sea_orm(column_type = "Text")]
        pub workspace: String,
        #[sea_orm(column_type = "Text")]
        pub idempotency_key: String,
        #[sea_orm(column_type = "Text")]
        pub starts_at: String,
        #[sea_orm(column_type = "Text")]
        pub ends_at: String,
        pub ends_at_unix: i64,
        pub appointment_saved_version: i64,
        pub prepared_at: i64,
        pub source_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub source_versions: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub source_freshness: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub limits: Json,
        #[sea_orm(column_type = "Text")]
        pub visit_card_orientation: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub trajectory: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub meaningful_changes: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub unresolved_issues: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub visit_decisions: Json,
        #[sea_orm(column_type = "Text")]
        pub basis_and_limitations: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub basis_citations: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub citations: Json,
        #[sea_orm(column_type = "Text")]
        pub narrative: String,
        #[sea_orm(column_type = "Text")]
        pub raised_concern_id: Option<String>,
        #[sea_orm(column_type = "Text")]
        pub visit_standing: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub visit_standing_detail: Option<Json>,
        #[sea_orm(column_type = "JsonBinary")]
        pub origin: Json,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Visit-scoped lifecycle tracking for pre-visit concerns raised by visit preparation.
/// Ordinary (`today`) visit concerns retire when the visit ends; `now` concerns
/// stay open past the visit end until opened by an authorized physician.
/// Created by `m20261118_07_visit_briefings`.
pub(super) mod visit_concern {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_visit_concerns")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub concern_id: String,
        #[sea_orm(column_type = "Text")]
        pub source_id: String,
        #[sea_orm(column_type = "Text")]
        pub appointment_id: String,
        pub starts_at_unix: i64,
        pub ends_at_unix: i64,
        #[sea_orm(column_type = "Text")]
        pub urgency: String,
        pub opened_at: Option<i64>,
        #[sea_orm(column_type = "Text")]
        pub opened_by_physician: Option<String>,
        pub created_at: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
