//! Patient-prepared `"For my doctor"` share held at the front door until picked up by the practice.
//!
//! Tracks only `sent_at` and `received_at` (never whether a physician looked).

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_for_my_doctor_shares")]
pub struct Model {
    /// Stable share identifier generated once on accept.
    #[sea_orm(primary_key, auto_increment = false)]
    pub share_id: String,
    /// Owning Duet patient account.
    pub account_id: Uuid,
    /// Linked practice identifier when sent.
    pub practice_id: String,
    /// Linked roster patient identifier when sent.
    pub patient_id: String,
    /// Caller-supplied idempotency key unique per account.
    pub share_key: String,
    /// Share headline (e.g. `"For my doctor"` or visit topic).
    pub title: String,
    /// Approved patient-prepared summary text.
    #[sea_orm(column_type = "Text")]
    pub summary_text: String,
    /// Explicit origin label identifying patient preparation (e.g. `"Duet — For my doctor"`).
    pub origin_label: String,
    /// Unix second the patient sent the share.
    pub sent_at: i64,
    /// Consent grant timestamp verified when the share was sent.
    pub consent_at: i64,
    /// Unix second the practice's agents picked up the share (`None` while still `Sent`).
    pub received_at: Option<i64>,
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
