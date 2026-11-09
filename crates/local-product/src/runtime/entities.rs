//! Physician-owned mappings of the recorded runtime schema; not migrations.

/// One physician's unsent composer text for one chat. The table is created
/// from this entity by the store's migration, not by `runtime.sql`.
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

pub(super) mod conversation {
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
        /// Database-clock seconds the command was accepted. Added by the
        /// store's migration; rows older than that column carry its run time.
        pub accepted_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One physician's saved reply draft and its latest revision work. The table
/// is created from this entity by the store's migration, not by `runtime.sql`.
pub(super) mod reply_draft {
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
/// The table is created from this entity by the store's migration.
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
        /// The practice clock's Unix second when the pull finished.
        pub pulled_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One practice's messaging channel: the connection, its sealed credential,
/// the reader's cursor and lease, and how the last read went. Created from
/// this entity by the store's migration.
pub(super) mod channel_connection {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_channel_connections")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        /// The serialized `Connection`.
        #[sea_orm(column_type = "JsonBinary")]
        pub state: Json,
        /// The bound Spruce organization while connected. Unique, so one
        /// organization feeds one practice; null once disconnected.
        #[sea_orm(column_type = "Text", nullable, unique)]
        pub organization: Option<String>,
        /// Nonce and AES-256-GCM ciphertext of the credential, bound to the practice.
        pub credential: Option<Vec<u8>>,
        /// `spruce::changes::Cursor::encode`, stored only with the page it follows.
        #[sea_orm(column_type = "Text", nullable)]
        pub cursor: Option<String>,
        /// The reader that holds the lease, for operators.
        #[sea_orm(column_type = "Text", nullable)]
        pub lease_holder: Option<String>,
        /// Fresh for every claim. A commit carrying any other token fails,
        /// including an earlier claim by the same holder.
        #[sea_orm(column_type = "Text", nullable)]
        pub lease_token: Option<String>,
        /// Database-clock seconds the lease runs out.
        pub lease_expires: i64,
        /// The channel schema generation of the release that last wrote the
        /// row. An older release refuses a row a newer one has touched.
        pub generation: i64,
        /// Database-clock seconds of the last page that committed.
        pub last_read_at: Option<i64>,
        /// The serialized `ReadFailure` while the channel can't be read.
        #[sea_orm(column_type = "JsonBinary", nullable)]
        pub failure: Option<Json>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// One channel conversation as intake knows it: who is on it, whether it is
/// owed a full read, and the link marker that message matching writes.
pub(super) mod channel_conversation {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_channel_conversations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub conversation: String,
        /// The serialized `ConversationDetails`, once read.
        #[sea_orm(column_type = "JsonBinary", nullable)]
        pub details: Option<Json>,
        /// The serialized `Owed` while the conversation needs a full read.
        #[sea_orm(column_type = "JsonBinary", nullable)]
        pub owed: Option<Json>,
        /// When the owed read was last attempted, as a per-practice sequence
        /// assigned under the connection row's lock, so each pass starts with
        /// the conversations tried longest ago.
        pub owed_attempt: Option<i64>,
        /// Database-clock seconds the conversation was first saved or last
        /// reread. A routine reread is due once it is old.
        pub checked_at: i64,
        /// Database-clock seconds a new message version was last saved in it.
        /// Only recently active conversations get routine rereads.
        pub saved_at: i64,
        /// Set by message matching when the conversation is linked to a
        /// patient. Intake only reads it: disconnect keeps linked evidence.
        #[sea_orm(column_type = "Text", nullable)]
        pub link: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Append-only message versions. An edit or a Spruce deletion is a new row;
/// a repeated delivery of the same version inserts nothing.
pub(super) mod channel_message {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_channel_messages")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        /// Spruce item id.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub item: String,
        /// `spruce::payload::Item::version`, or an unreadable item's marker.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub version: String,
        #[sea_orm(column_type = "Text")]
        pub conversation: String,
        /// Version order within the item, from 1, assigned under the
        /// connection row's lock.
        pub ordinal: i64,
        /// Database-clock seconds the version was saved.
        pub recorded_at: i64,
        /// The serialized `MessageVersion`.
        #[sea_orm(column_type = "JsonBinary")]
        pub record: Json,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// File bytes Coda saved from a conversation, addressed by their SHA-256.
pub(super) mod channel_file {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_channel_files")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub conversation: String,
        /// Lower-case hex SHA-256 of `bytes`.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub sha256: String,
        pub bytes: Vec<u8>,
        /// Database-clock seconds the file was saved.
        pub saved_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Append-only history: one row per earlier revision of a reply draft, with
/// its retained execution evidence. Written when the next revision is
/// accepted, so the draft row holds only current work. Afterwards only late
/// execution records from that revision's own attempts are appended.
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
