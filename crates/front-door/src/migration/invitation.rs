//! One open practice invitation to a roster patient, at rest.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "front_door_invitations")]
pub struct Model {
    /// The invitation row id.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The practice the invited patient belongs to.
    pub practice_id: String,
    /// The roster patient the practice invited.
    pub patient_id: String,
    /// The lower-cased email the invitation names.
    pub email: String,
    /// The invitee's display name, as the practice wrote it.
    pub display_name: String,
    /// The SHA-256 of the latest link redeem token.
    pub token_hash: Vec<u8>,
    /// The Unix second the invitation was first issued.
    pub created_at: i64,
    /// Practice display name shown to the patient after DOB verification.
    pub practice_name: String,
    /// Physician display name shown to the patient after DOB verification.
    pub physician_name: String,
    /// Practice chart date of birth (`YYYY-MM-DD`) required before preview/Share.
    pub expected_birth_date: Option<String>,
    /// Count of consecutive wrong DOB answers against this open invitation.
    pub wrong_dob_attempts: i32,
    /// Unix second this invitation locked after 5 wrong DOB answers, if locked.
    pub locked_at: Option<i64>,
    /// Unix second the most recent outbound email attempt failed, if any.
    pub last_delivery_failed_at: Option<i64>,
    /// Plain summary when outbound email delivery failed while keeping the invitation open.
    pub last_delivery_error: Option<String>,
    /// Unix second this open invitation was last issued or resent.
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
