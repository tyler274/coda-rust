//! One roster patient's front-door invitation, Duet link, consent, and cadence at rest.

use sea_orm::entity::prelude::*;

/// One roster patient's front-door invitation, Duet link, consent, and cadence at rest.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "physician_patient_links")]
pub struct Model {
    /// The patient-link row id.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The practice that owns the roster patient.
    pub practice_id: String,
    /// The roster patient's workspace id.
    pub patient_workspace_id: String,
    /// The SHA-256 of the pending single-use invitation token, when unredeemed.
    pub pending_token_hash: Option<Vec<u8>>,
    /// The Unix second the pending invitation expires.
    pub pending_expires_at: Option<i64>,
    /// The linked Duet account id, once the invitation is accepted.
    pub duet_account_id: Option<String>,
    /// Whether the patient accepted the practice invitation.
    pub linked: bool,
    /// Whether "share with my physician" consent is currently active.
    pub sharing_consented: bool,
    /// The accepted consent version (such as `"v1"`).
    pub consent_version: Option<String>,
    /// The Unix second consent was granted.
    pub consented_at: Option<i64>,
    /// The Unix second consent was revoked via one-step stop-sharing.
    pub revoked_at: Option<i64>,
    /// The review cadence exposed to Agents (`"weekly"` or `"daily_when_changed"`).
    pub cadence: String,
    /// The Unix second the row was created.
    pub created_at: i64,
}

/// SeaORM relations for the patient-link entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
