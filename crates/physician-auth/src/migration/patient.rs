//! One patient workspace's owning practice and review cadence, at rest.
//!
//! Stores only `practice_id` and `cadence` (`weekly` or
//! `weekly_and_daily_when_changed`) so Agents and the physician projection
//! read per-patient cadence without knowing payment status.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_patients")]
pub struct Model {
    /// The patient workspace identifier.
    #[sea_orm(primary_key, auto_increment = false)]
    pub patient_workspace_id: String,
    /// The practice that owns this patient workspace.
    pub practice_id: String,
    /// The patient's review cadence (`weekly` or `weekly_and_daily_when_changed`).
    pub cadence: String,
    /// The Unix second the patient registration was written.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
