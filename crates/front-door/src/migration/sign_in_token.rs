//! One pending sign-in token, at rest.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_sign_in_tokens")]
pub struct Model {
    /// The token row id.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The account the token would sign in.
    pub account_id: Uuid,
    /// The SHA-256 of the presented one-time token; unique.
    pub token_hash: Vec<u8>,
    /// The Unix second the token was issued.
    pub created_at: i64,
    /// The Unix second the token stops verifying.
    pub expires_at: i64,
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
