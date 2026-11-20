//! One patient account at the front door, at rest.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_accounts")]
pub struct Model {
    /// The account id.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// The lower-cased email the account signs in with; unique.
    pub email: String,
    /// The account's display name.
    pub display_name: String,
    /// Whether the account may sign in and hold sessions.
    pub active: bool,
    /// Sessions minted before this Unix second never resolve.
    pub sessions_ended_before: i64,
    /// The Unix second the account was created.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::connection::Entity")]
    Connections,
    #[sea_orm(has_many = "super::session::Entity")]
    Sessions,
    #[sea_orm(has_many = "super::sign_in_token::Entity")]
    SignInTokens,
}

impl ActiveModelBehavior for ActiveModel {}
