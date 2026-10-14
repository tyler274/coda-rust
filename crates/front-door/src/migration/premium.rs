//! One linked patient's premium status and cadence, at rest.
//!
//! The front door owns premium truth. Agents reads only the cadence setting
//! and never knows who pays; Duet reads only which premium features are on;
//! neither stores billing, and payment records hold no clinical content.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_premium")]
pub struct Model {
    /// The linked patient the status belongs to.
    #[sea_orm(primary_key, auto_increment = false)]
    pub patient_key: String,
    /// Whether the patient pays for premium.
    pub premium: bool,
    /// The cadence Agents runs for the patient: `"weekly"`, or
    /// `"weekly-daily-check"` when the daily check is added.
    pub cadence: String,
    /// The Unix second the status was written.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
