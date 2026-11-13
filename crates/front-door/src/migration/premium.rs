//! One linked patient's premium status, at rest.
//!
//! The front door owns premium feature state. Duet reads only which premium
//! features are on; payment records hold no clinical content and never set
//! care-team review cadence.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_premium")]
pub struct Model {
    /// The linked patient the status belongs to.
    #[sea_orm(primary_key, auto_increment = false)]
    pub patient_key: String,
    /// Whether the patient has premium features enabled.
    pub premium: bool,
    /// The Unix second the status was written.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
