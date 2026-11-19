//! Existing Duet health records staged by a patient account and readable by the linked practice
//! whenever sharing consent is granted.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_shared_records")]
pub struct Model {
    /// Owning Duet patient account.
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: Uuid,
    /// Stable record identifier within the patient's Duet store.
    #[sea_orm(primary_key, auto_increment = false)]
    pub record_id: String,
    /// Monotonic revision token for this shared record.
    pub revision: String,
    /// Record kind (e.g. `"Observation"`, `"DocumentReference"`, `"Condition"`).
    pub record_type: String,
    /// Human-readable title for the physician's chart.
    pub title: String,
    /// Clinical or patient-logged content body.
    #[sea_orm(column_type = "Text")]
    pub content_text: String,
    /// Unix second when the observation or note was recorded in Duet.
    pub recorded_at: i64,
    /// Unix second when this row was last updated.
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
