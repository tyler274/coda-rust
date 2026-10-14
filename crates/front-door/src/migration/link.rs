//! The durable Duet-account-to-roster-patient link, at rest.
//!
//! One link per Duet account. The practice's redeemed invitation writes it;
//! identity is never guessed from names, phones, or clinical text.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_links")]
pub struct Model {
    /// The link row id.
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: Uuid,
    /// The practice whose roster the linked patient belongs to.
    pub practice_id: String,
    /// The roster patient within the practice.
    pub patient_id: String,
    /// The Unix second the link was written.
    pub created_at: i64,
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
