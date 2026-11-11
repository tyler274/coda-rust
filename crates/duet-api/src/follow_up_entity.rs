//! Duet-owned SeaORM mappings of the retained follow-up tables.
//!
//! These tables stay only so privacy replay can still apply follow-up
//! restrictions. This file maps only what that replay reads or writes, plus the
//! primary key each entity needs. The schema is owned by
//! `crates/duet-api/migrations`; these entities never create or alter it.
//! Statements built from them run on the caller's account transaction.

/// Accepted follow-ups.
pub mod follow_up {
    use sea_orm::entity::prelude::*;

    /// `follow_ups.state`, as its check constraint allows.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "Text")]
    pub enum State {
        /// Runs on its schedule.
        #[sea_orm(string_value = "active")]
        Active,
        /// Held until the patient resumes it.
        #[sea_orm(string_value = "paused")]
        Paused,
        /// Ended for good by the patient.
        #[sea_orm(string_value = "cancelled")]
        Cancelled,
        /// Ended after its last occurrence.
        #[sea_orm(string_value = "completed")]
        Completed,
    }

    /// Mapped subset of one `public.follow_ups` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "follow_ups")]
    pub struct Model {
        /// Follow-up identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Owning account.
        pub user_id: Uuid,
        /// Accepted revision; every restriction raises it.
        pub revision: i64,
        /// Lifecycle state.
        pub state: State,
        /// Next scheduled occurrence; `None` when nothing is scheduled.
        pub next_occurrence_at: Option<DateTimeUtc>,
        /// Waits for patient confirmation before it runs.
        pub confirmation_required: bool,
        /// Last mutation time.
        pub updated_at: DateTimeUtc,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Scheduled runs of one follow-up.
pub mod occurrence {
    use sea_orm::entity::prelude::*;

    /// `duet_follow_up_occurrences.status`, as its check constraint allows.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "Text")]
    pub enum Status {
        /// Reserved and waiting for its command.
        #[sea_orm(string_value = "queued")]
        Queued,
        /// Its command is running.
        #[sea_orm(string_value = "running")]
        Running,
        /// Delivered its message.
        #[sea_orm(string_value = "completed")]
        Completed,
        /// Could not run.
        #[sea_orm(string_value = "unavailable")]
        Unavailable,
        /// Ran and failed.
        #[sea_orm(string_value = "failed")]
        Failed,
        /// Stopped by a restriction.
        #[sea_orm(string_value = "interrupted")]
        Interrupted,
    }

    /// Mapped subset of one `public.duet_follow_up_occurrences` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_follow_up_occurrences")]
    pub struct Model {
        /// Owning account.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Occurrence identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// The follow-up this occurrence runs.
        pub follow_up_id: Uuid,
        /// The accepted command that carries this occurrence.
        pub command_id: Uuid,
        /// Run status.
        pub status: Status,
        /// End time; `None` while queued or running.
        pub completed_at: Option<DateTimeUtc>,
        /// Last mutation time.
        pub updated_at: DateTimeUtc,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// The account's proactive follow-up hold.
pub mod settings {
    use sea_orm::entity::prelude::*;

    /// One `public.duet_follow_up_settings` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_follow_up_settings")]
    pub struct Model {
        /// Owning account.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Raised each time the hold changes.
        pub revision: i64,
        /// Proactive follow-ups may run.
        pub enabled: bool,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Accepted work.
pub mod command {
    use sea_orm::entity::prelude::*;

    /// `duet_commands.state`, as its check constraint allows.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "Text")]
    pub enum State {
        /// Waiting for a worker.
        #[sea_orm(string_value = "accepted")]
        Accepted,
        /// Held by a worker lease.
        #[sea_orm(string_value = "running")]
        Running,
        /// Finished.
        #[sea_orm(string_value = "complete")]
        Complete,
        /// Stopped before it finished.
        #[sea_orm(string_value = "cancelled")]
        Cancelled,
    }

    /// Mapped subset of one `public.duet_commands` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_commands")]
    pub struct Model {
        /// Owning account.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// Command identity.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// Execution state.
        pub state: State,
        /// Current worker lease; `None` when unleased.
        pub lease_id: Option<Uuid>,
        /// Lease expiry; `None` when unleased.
        pub lease_until: Option<DateTimeUtc>,
        /// Memory staged by the running command; `None` when nothing is staged.
        pub staged_memory: Option<Json>,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Events that wake a command's clients.
pub mod conversation_event {
    use sea_orm::entity::prelude::*;

    /// Mapped subset of one `public.duet_conversation_events` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_conversation_events")]
    pub struct Model {
        /// Owning account.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// The command whose clients the event wakes.
        #[sea_orm(primary_key, auto_increment = false)]
        pub command_id: Uuid,
        /// Identity-generated order; never written here.
        #[sea_orm(primary_key)]
        pub sequence: i64,
        /// Event object.
        pub event: Json,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Tool calls a command made and their results.
pub mod tool_effect {
    use sea_orm::entity::prelude::*;

    /// The tools whose effects name a follow-up.
    pub const FOLLOW_UP_TOOLS: [&str; 5] = [
        "create_follow_up",
        "edit_follow_up",
        "pause_follow_up",
        "resume_follow_up",
        "cancel_follow_up",
    ];

    /// Mapped subset of one `public.duet_tool_effects` row.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "public", table_name = "duet_tool_effects")]
    pub struct Model {
        /// Owning account.
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        /// The command that made the call.
        #[sea_orm(primary_key, auto_increment = false)]
        pub command_id: Uuid,
        /// Call identity within the command.
        #[sea_orm(primary_key, auto_increment = false)]
        pub invocation_id: String,
        /// The tool call; `name` is the tool.
        pub call: Json,
        /// The tool result; `id` is the follow-up it touched.
        pub result: Json,
    }

    /// No relations are navigated.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
