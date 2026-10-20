//! Identity Platform subjects attached to the existing account owner.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_identities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub project: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub subject: String,
    pub account_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
