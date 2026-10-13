//! One physician's access to one patient workspace, at rest.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_memberships")]
pub struct Model {
    /// The membership row id.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The practice the physician belongs to.
    pub practice_id: String,
    /// The physician within the practice.
    pub physician_id: String,
    /// The patient workspace the membership names.
    pub patient_workspace_id: String,
    /// Whether the physician may read this workspace.
    pub allowed: bool,
    /// The Unix second the membership was written.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
