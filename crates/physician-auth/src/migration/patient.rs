//! One patient workspace's owning practice, at rest.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_patients")]
pub struct Model {
    /// The patient workspace identifier.
    #[sea_orm(primary_key, auto_increment = false)]
    pub patient_workspace_id: String,
    /// The practice that owns this patient workspace.
    pub practice_id: String,
    /// The Unix second the patient registration was written.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
