//! One browser session a sign-in minted, at rest.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_sessions")]
pub struct Model {
    /// The session id.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The account the session acts for.
    pub account_id: Uuid,
    /// The SHA-256 of the presented bearer token; unique.
    pub token_hash: Vec<u8>,
    /// The Unix second the session was minted, against the account's fence.
    pub created_at: i64,
    /// The Unix second the session stops being valid.
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
