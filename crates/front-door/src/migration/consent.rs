//! One patient's consent to share with their physician, at rest.
//!
//! The row exists only once consent was given; withdrawing it deletes the
//! row. Stopping sharing is one step, and what was already shared stays
//! dated evidence on the physician's side.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_consents")]
pub struct Model {
    /// The Duet account that consented.
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: Uuid,
    /// The Unix second the consent was given.
    pub consented_at: i64,
    /// The Unix second the consent was withdrawn, when it was.
    pub withdrawn_at: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_delete = "Cascade"
    )]
    Account,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
