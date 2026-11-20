//! Typed SeaORM entities for Duet AI preferences, 30-day memory retention, and harness runs.

use sea_orm::entity::prelude::*;

pub(crate) mod private_memory {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "duet_private_memory")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        pub revision: i64,
        pub content: Json,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod memory_version {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "duet_private_memory_versions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        #[sea_orm(primary_key, auto_increment = false)]
        pub revision: i64,
        pub content: Json,
        pub recorded_at: DateTimeUtc,
        pub expires_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod ai_preference {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "duet_ai_preferences")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        pub research_consent_state: String,
        pub research_consent_at: Option<DateTimeUtc>,
        pub research_consent_revision: i64,
        pub standing_disclosure_acknowledged_at: Option<DateTimeUtc>,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod harness_run {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "duet_harness_runs")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        #[sea_orm(primary_key, auto_increment = false)]
        pub run_id: String,
        pub continues_run_id: Option<String>,
        pub recipe_name: String,
        pub model: String,
        pub prompt_cache_key: String,
        pub outcome: String,
        pub hold_reason: Option<String>,
        pub events: Json,
        pub input_tokens: Option<i64>,
        pub output_tokens: Option<i64>,
        pub started_at: DateTimeUtc,
        pub ended_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod user_gate {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub terms_version: Option<String>,
        pub intelligence_enabled: bool,
        pub luna_consent_decision: Option<String>,
        pub intelligence_control_revision: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
