//! Duet's retained health context, one row per account.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "health_contexts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub snapshot_id: Uuid,
    pub source_boundary_id: Uuid,
    pub schema_version: i32,
    pub revision: i64,
    pub generated_at: DateTimeUtc,
    pub timezone: String,
    pub source_generations: Json,
    pub facts: Json,
    pub projections: Json,
    pub producer_dependencies: Json,
    pub producer_limits: Json,
    pub producer_fingerprint: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
