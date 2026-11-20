//! Hashed link tokens and short codes belonging to an open invitation.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_invitation_credentials")]
pub struct Model {
    /// Credential row id.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Owning open invitation (`ON DELETE CASCADE` clears all credentials on cancel/accept).
    pub invitation_id: i64,
    /// Credential kind (`"link"` or `"code"`).
    pub kind: String,
    /// SHA-256 digest of the link token or normalized short code.
    pub secret_hash: Vec<u8>,
    /// Unix second this credential was issued.
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
