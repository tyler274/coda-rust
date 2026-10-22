//! Duet terms and Luna permission, not physician-sharing consent.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: Uuid,
    pub terms_version: Option<String>,
    pub terms_at: Option<i64>,
    pub luna_version: Option<String>,
    pub luna_allowed: bool,
    pub luna_at: Option<i64>,
    pub revision: i64,
    pub restriction_pending: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
