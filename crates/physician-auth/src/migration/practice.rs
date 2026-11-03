//! One practice and the clock it was bound to when it was created, at rest.
//!
//! An empty `fixed_time` is a practice on real time. A synthetic practice
//! holds the instant its clinical "now" stays at.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_practices")]
pub struct Model {
    /// The practice identifier.
    #[sea_orm(primary_key, auto_increment = false)]
    pub practice_id: String,
    /// The Unix second a synthetic practice's clinical "now" is fixed at.
    pub fixed_time: Option<i64>,
    /// The Unix second the practice was created.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
