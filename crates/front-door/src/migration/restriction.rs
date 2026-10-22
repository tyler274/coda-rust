//! Retained Luna withdrawal events; deliberately no cascading account foreign key.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_luna_restrictions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub account_id: Uuid,
    pub revision: i64,
    pub occurred_at: i64,
    pub confirmed: bool,
    pub lease: Option<Uuid>,
    pub due_at: i64,
    pub attempts: i32,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
