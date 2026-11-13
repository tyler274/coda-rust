//! Durable per-practice clinical source setup, backlog gate, recovery standing,
//! and disconnect retention state (`physician_practice_source_setups`).

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_practice_source_setups")]
pub struct Model {
    /// Practice identifier owning this clinical source setup row.
    #[sea_orm(primary_key, auto_increment = false)]
    pub practice_id: String,
    /// Supported clinical source slug (`hint`, `elation_fhir`, `spruce`).
    #[sea_orm(primary_key, auto_increment = false)]
    pub source_id: String,
    /// Serialized `SourceStandingWire` JSON payload.
    #[sea_orm(column_type = "Text")]
    pub standing_json: String,
    /// Serialized `SpruceBacklogWire` JSON payload.
    #[sea_orm(column_type = "Text")]
    pub backlog_json: String,
    /// Serialized background catch-up/retry activity JSON payload.
    #[sea_orm(column_type = "Text")]
    pub activity_json: String,
    /// Serialized never-linked inbound Spruce channel messages JSON payload.
    #[sea_orm(column_type = "Text")]
    pub unlinked_spruce_json: String,
    /// Serialized linked patient chart record metadata JSON payload.
    #[sea_orm(column_type = "Text")]
    pub linked_records_json: String,
    /// Serialized retained physician reply draft metadata JSON payload.
    #[sea_orm(column_type = "Text")]
    pub retained_drafts_json: String,
    /// Monotonic import generation counter for this practice source.
    pub import_generation: i64,
    /// Unix second of the most recent update.
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
