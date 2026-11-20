//! Durable multi-practice patient-to-practice connection state, at rest.
//!
//! Keyed by `(account_id, practice_id, patient_id)`: one Duet account may connect
//! to multiple practices independently, and connected means sharing is live.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_connections")]
pub struct Model {
    /// Owning Duet patient account.
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: Uuid,
    /// Connected practice identifier.
    #[sea_orm(primary_key, auto_increment = false)]
    pub practice_id: String,
    /// Roster patient workspace identifier within `practice_id`.
    #[sea_orm(primary_key, auto_increment = false)]
    pub patient_id: String,
    /// Connection standing: `"connected"` or `"ended"`.
    pub status: String,
    /// Unix second when the patient connected (and granted fixed consent).
    pub connected_at: i64,
    /// Unix second when the connection ended, if `"ended"`.
    pub ended_at: Option<i64>,
    /// Why the connection ended (`"patient_left"`, `"practice_discharged"`, or `"account_deleted"`).
    pub end_reason: Option<String>,
    /// Unix second this connection row was first created.
    pub created_at: i64,
    /// Unix second this connection row was last updated.
    pub updated_at: i64,
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
