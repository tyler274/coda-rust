//! One invitation into a practice, at rest.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_invitations")]
pub struct Model {
    /// The invitation row id.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The practice the invitee would join.
    pub practice_id: String,
    /// The lower-cased email the invitation names; unique.
    pub email: String,
    /// The invitee's display name.
    pub display_name: String,
    /// Whether the invited account may administer the practice.
    pub admin: bool,
    /// The SHA-256 of the presented redeem token.
    pub token_hash: Vec<u8>,
    /// The Unix second the invitation was issued.
    pub created_at: i64,
    /// The Unix second the invitation stops redeeming.
    pub expires_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
