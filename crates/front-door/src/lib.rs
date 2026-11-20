//! Canonical Connected Care invitation, multi-practice connection, and
//! patient-to-practice sharing owner.
//!
//! One owner holds invitations (`(practice_id, patient_id)`), multi-practice
//! connections (`(account_id, practice_id, patient_id)` with `Connected` or
//! `Ended` standing), non-expiring patient sessions, Duet shared health records,
//! and `"For my doctor"` sends. Connected means sharing is live; there is no
//! paused consent state and no separate `physician-auth` patient-link store.
//!
//! This crate also contains Duet identity and privacy-journal modules.

#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

mod error;
pub mod http;
mod store;

pub use error::{
    ForMyDoctorError, InvitationAcceptError, InvitationCheckError, InviteError, LinkError,
    SessionError, SignInError,
};
pub use store::PostgresFrontDoorStore;

/// Durable Identity Platform mapping and Duet permission authority.
pub mod identity;
/// Transactional Luna privacy restrictions and independent receipt reconciliation.
pub mod journal;
/// The SeaORM entities and the migration chain.
pub mod migration;

/// A sign-in verification token is verifiable for one day.
pub const SIGN_IN_TOKEN_TTL_SECONDS: u64 = 86_400;
/// Number of wrong date-of-birth answers that lock an open invitation (`COD-48`).
pub const MAX_DOB_ATTEMPTS: u8 = 5;
/// Plain preview copy describing what Connected Care shares after `Share` consent (`Rule 6`).
pub const SHARES_SUMMARY: &str = "Connected health records, phone health readings, medications you entered, and anything you choose to send from For my doctor. Private AI notes never share.";

/// Which roster patient an invitation or connection names.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RosterPatient {
    /// The practice whose roster the patient belongs to.
    pub practice_id: String,
    /// The patient within the practice's roster.
    pub patient_id: String,
}

/// Why an active Connected Care connection ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionEndReason {
    /// The patient left the practice in Duet.
    PatientLeft,
    /// The practice asked Coda to discharge the patient.
    PracticeDischarged,
    /// The patient deleted their Duet account.
    AccountDeleted,
}

impl ConnectionEndReason {
    /// Stable database representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PatientLeft => "patient_left",
            Self::PracticeDischarged => "practice_discharged",
            Self::AccountDeleted => "account_deleted",
        }
    }

    /// Parses the database representation.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "patient_left" => Some(Self::PatientLeft),
            "practice_discharged" => Some(Self::PracticeDischarged),
            "account_deleted" => Some(Self::AccountDeleted),
            _ => None,
        }
    }
}

/// Standing of one `(account_id, practice_id, patient_id)` connection.
///
/// Connected means sharing is live; there is no paused state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ConnectionStanding {
    /// Patient accepted an invitation and granted fixed consent on `connected_at`.
    Connected {
        /// Unix second the connection (and sharing consent) began.
        connected_at: u64,
    },
    /// Connection ended; sharing stopped at `ended_at` while historical copies stay dated.
    Ended {
        /// Unix second the most recent connection began.
        connected_at: u64,
        /// Unix second the connection ended.
        ended_at: u64,
        /// Why the connection ended.
        reason: ConnectionEndReason,
    },
}

/// One practice connection attached to a patient account.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AccountConnection {
    /// The connected practice and roster patient.
    pub patient: RosterPatient,
    /// Whether this connection is currently active (`Connected`) or `Ended`.
    #[serde(flatten)]
    pub standing: ConnectionStanding,
}

/// Canonical Connected Care status for one `(practice_id, patient_id)` on a roster.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RosterConnectionStatus {
    /// Never invited, or open invitation cancelled with no prior connection.
    NotInvited,
    /// Practice sent an open invitation; waiting for the patient to join and share.
    Invited {
        /// Email address the invitation was sent to.
        email: String,
        /// Invitee display name.
        display_name: String,
        /// Unix second the invitation was sent.
        invited_at: u64,
        /// Consecutive wrong DOB answers recorded against this open invitation.
        #[serde(default)]
        wrong_dob_attempts: u8,
        /// Unix second this invitation locked after 5 wrong DOB answers, if locked.
        #[serde(default)]
        locked_at: Option<u64>,
        /// Unix second the most recent email delivery failed while keeping the invite open.
        #[serde(default)]
        delivery_failed_at: Option<u64>,
        /// Plain summary when outbound email delivery failed.
        #[serde(default)]
        delivery_error: Option<String>,
    },
    /// Patient joined and is actively sharing with the practice.
    Connected {
        /// Linked Duet account id.
        account_id: Uuid,
        /// Unix second the connection began.
        connected_at: u64,
    },
    /// Previous connection ended; a practice member may invite again at any time.
    Ended {
        /// Previously linked Duet account id.
        account_id: Uuid,
        /// Unix second the ended connection began.
        connected_at: u64,
        /// Unix second the connection ended.
        ended_at: u64,
        /// Why the connection ended.
        reason: ConnectionEndReason,
    },
}

impl RosterConnectionStatus {
    /// Short snake-case status label matching the spec (`not_invited`, `invited`, `connected`, `ended`).
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotInvited => "not_invited",
            Self::Invited { .. } => "invited",
            Self::Connected { .. } => "connected",
            Self::Ended { .. } => "ended",
        }
    }

    /// Active or most recently connected Duet account id, if any.
    pub fn account_id(&self) -> Option<Uuid> {
        match self {
            Self::Connected { account_id, .. } | Self::Ended { account_id, .. } => {
                Some(*account_id)
            }
            Self::NotInvited | Self::Invited { .. } => None,
        }
    }

    /// Whether this roster patient is actively connected (which implies sharing).
    pub const fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { .. })
    }
}

/// One patient account at the front door.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    /// The account's id, stable across multiple practices and re-connections.
    pub id: Uuid,
    /// The email sign-in address.
    pub email: String,
    /// The account's display name.
    pub display_name: String,
    /// Whether the account may sign in and hold sessions.
    pub active: bool,
    /// All practice connections (`Connected` or `Ended`) owned by this account.
    pub connections: Vec<AccountConnection>,
    /// Sessions minted before this Unix second never resolve; deactivation
    /// moves it forward and reactivation leaves it there.
    pub sessions_ended_before: u64,
}

impl Account {
    /// Returns the `Connected` timestamp for `patient` if this account is actively connected to it.
    pub fn active_connection_at(&self, patient: &RosterPatient) -> Option<u64> {
        self.connections.iter().find_map(|conn| {
            if &conn.patient == patient
                && let ConnectionStanding::Connected { connected_at } = conn.standing
            {
                Some(connected_at)
            } else {
                None
            }
        })
    }

    /// Lists all active `(RosterPatient, connected_at)` connections for this account.
    pub fn active_connections(&self) -> Vec<(&RosterPatient, u64)> {
        self.connections
            .iter()
            .filter_map(|conn| match conn.standing {
                ConnectionStanding::Connected { connected_at } => {
                    Some((&conn.patient, connected_at))
                }
                ConnectionStanding::Ended { .. } => None,
            })
            .collect()
    }
}

/// One open invitation a practice issued to a roster patient.
///
/// Open invitations never expire with time; they remain open until accepted or cancelled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invitation {
    /// Which roster patient the practice invited.
    pub patient: RosterPatient,
    /// The lower-cased email the invitation names.
    pub email: String,
    /// The invitee's display name, as the practice wrote it.
    pub display_name: String,
    /// The Unix second the invitation was first issued.
    pub invited_at: u64,
    /// Practice display name shown after DOB passes.
    pub practice_name: String,
    /// Physician display name shown after DOB passes.
    pub physician_name: String,
    /// Chart date of birth (`YYYY-MM-DD`) required before preview/Share, if set.
    pub expected_birth_date: Option<String>,
    /// Wrong DOB answers recorded against this open invitation (`0..=5`).
    pub wrong_dob_attempts: u8,
    /// Unix second this invitation locked after 5 wrong DOB answers.
    pub locked_at: Option<u64>,
    /// Unix second outbound email delivery last failed while keeping the invitation open.
    pub last_delivery_failed_at: Option<u64>,
    /// Plain summary of the most recent outbound email failure, if any.
    pub last_delivery_error: Option<String>,
    /// Unix second this invitation was last issued or resent.
    pub updated_at: u64,
}

/// Parameters for creating or replacing an open roster-patient invitation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvitePatientParams {
    /// Practice and roster patient id.
    pub patient: RosterPatient,
    /// Invitee email address.
    pub email: String,
    /// Invitee display name.
    pub display_name: String,
    /// Practice display name revealed after DOB passes.
    pub practice_name: String,
    /// Physician display name revealed after DOB passes.
    pub physician_name: String,
    /// Practice chart DOB (`YYYY-MM-DD`) required for verification.
    pub expected_birth_date: Option<String>,
    /// Unix timestamp of issuance.
    pub now: u64,
}

/// Newly issued credentials for an open or resent invitation.
#[derive(Clone, PartialEq, Eq)]
pub struct IssuedInvitation {
    /// Mailed link token (`/invite?token=...`).
    pub link_token: String,
    /// Human-enterable short code (`XXXX-XXXX`).
    pub short_code: String,
    /// Current invitation record.
    pub invitation: Invitation,
}

impl std::fmt::Debug for IssuedInvitation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IssuedInvitation")
            .field("link_token", &"<redacted>")
            .field("short_code", &"<redacted>")
            .field("invitation", &self.invitation)
            .finish()
    }
}

/// Typed invitation credential supplied by a patient in Duet.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum InvitationCredential {
    /// Full invitation token carried by `/invite?token=...` or `duet://invite?token=...`.
    LinkToken(String),
    /// Human-enterable short code from the invitation email.
    ShortCode(String),
}

impl std::fmt::Debug for InvitationCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LinkToken(_) => f.write_str("InvitationCredential::LinkToken(<redacted>)"),
            Self::ShortCode(_) => f.write_str("InvitationCredential::ShortCode(<redacted>)"),
        }
    }
}

impl InvitationCredential {
    /// Parses a raw token or short code string (`codaf-...` => link token, otherwise short code).
    pub fn from_raw(raw: &str) -> Self {
        let trimmed = raw.trim();
        if trimmed.starts_with("codaf-") {
            Self::LinkToken(trimmed.to_owned())
        } else {
            Self::ShortCode(trimmed.to_owned())
        }
    }

    pub(crate) fn secret_hash(&self) -> Option<[u8; 32]> {
        match self {
            Self::LinkToken(tok) => {
                let trimmed = tok.trim();
                (!trimmed.is_empty()).then(|| token_hash(trimmed))
            }
            Self::ShortCode(code) => {
                let normalized = normalize_short_code(code);
                (!normalized.is_empty()).then(|| short_code_hash(&normalized))
            }
        }
    }
}

/// Lock standing of an open invitation before DOB is verified (`Rule 6`, `Rule 7`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum InvitationOpenStanding {
    /// Open and accepting date-of-birth checks.
    Active {
        /// Wrong DOB answers recorded so far.
        wrong_dob_attempts: u8,
        /// Wrong DOB answers remaining before lock.
        attempts_remaining: u8,
    },
    /// Locked after 5 wrong DOB answers (`COD-48`).
    Locked {
        /// Unix second the invitation locked.
        locked_at: u64,
    },
}

/// Pre-DOB gate status for an open invitation (reveals zero practice or sharing details).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvitationGateState {
    /// Whether the invitation can still accept DOB attempts or is locked.
    pub standing: InvitationOpenStanding,
}

/// Plain preview shown only after the patient's entered DOB matches (`Rule 6`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvitationSharingPreview {
    /// Practice identifier.
    pub practice_id: String,
    /// Practice display name.
    pub practice_name: String,
    /// Physician display name.
    pub physician_name: String,
    /// Patient display name on the invitation.
    pub display_name: String,
    /// Plain summary of what tapping Share connects.
    pub shares_summary: String,
}

/// Parses a strict `YYYY-MM-DD` calendar date, rejecting impossible dates.
pub fn parse_birth_date(raw: &str) -> Option<(i32, u32, u32)> {
    let trimmed = raw.trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    let year: i32 = trimmed[0..4].parse().ok()?;
    let month: u32 = trimmed[5..7].parse().ok()?;
    let day: u32 = trimmed[8..10].parse().ok()?;
    if !(1900..=2100).contains(&year) || !(1..=12).contains(&month) {
        return None;
    }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if leap { 29 } else { 28 }
        }
        _ => return None,
    };
    (1..=max_day).contains(&day).then_some((year, month, day))
}

/// Formats a validated `(year, month, day)` as canonical `YYYY-MM-DD`.
pub fn format_birth_date((year, month, day): (i32, u32, u32)) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

/// Converts Unix seconds to UTC `(year, month, day)` (Hinnant civil-from-days).
fn utc_date_from_unix(unix_seconds: u64) -> (i32, u32, u32) {
    let days = (unix_seconds / 86_400) as i64 + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = (days - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let year = (y + i64::from(m <= 2)) as i32;
    (year, m, d)
}

/// Returns whether a patient born on `birth_date` is at least 18 years old at `now_unix`.
pub fn is_adult_on(birth_date: (i32, u32, u32), now_unix: u64) -> bool {
    let (cy, cm, cd) = utc_date_from_unix(now_unix);
    let (by, bm, bd) = birth_date;
    let age = cy - by - i32::from((cm, cd) < (bm, bd));
    age >= 18
}

/// Patient-facing delivery standing of a `"For my doctor"` share.
///
/// Tracks only initial delivery vs pickup by the practice's agents (`Received`),
/// never whether a physician looked at it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ForMyDoctorShareStatus {
    /// Sent by the patient and waiting pickup by practice agents.
    Sent,
    /// Picked up by the practice's agents at `received_at`.
    Received {
        /// Unix second the practice's agents picked up the share.
        received_at: u64,
    },
}

/// Request payload to send one patient-prepared `"For my doctor"` share from Duet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorSendRequest {
    /// Caller-supplied idempotency key unique per patient connection.
    pub share_key: String,
    /// Concise share headline.
    pub title: String,
    /// Approved patient-prepared summary text.
    pub summary_text: String,
    /// Optional explicit origin description; defaults to `"Duet — For my doctor"`.
    pub origin_label: Option<String>,
    /// Target connected practice when the patient holds more than one connection.
    /// Optional when the patient has exactly one active connection.
    #[serde(default)]
    pub target: Option<RosterPatient>,
}

/// Durable `"For my doctor"` share held at the front door.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorShare {
    /// Stable front-door share identifier.
    pub share_id: String,
    /// Owning Duet patient account.
    pub account_id: Uuid,
    /// Linked practice and roster patient.
    pub patient: RosterPatient,
    /// Caller-supplied idempotency key.
    pub share_key: String,
    /// Share headline.
    pub title: String,
    /// Approved patient-prepared summary text.
    pub summary_text: String,
    /// Explicit patient-prepared origin description.
    pub origin_label: String,
    /// Unix second the patient sent the share.
    pub sent_at: u64,
    /// Connection consent timestamp verified when sent.
    pub consent_at: u64,
    /// Delivery status visible to Duet (`Sent` or `Received`, never `"physician looked"`).
    pub standing: ForMyDoctorShareStatus,
}

/// Patient-logged or Duet-held shared health record input uploaded by an active account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedPatientRecordInput {
    /// Stable record identifier within the patient's Duet store.
    pub record_id: String,
    /// Monotonic revision token for this shared record.
    pub revision: String,
    /// Clinical/patient record kind (e.g. `"Observation"`, `"DocumentReference"`).
    pub record_type: String,
    /// Human-readable title for the chart.
    pub title: String,
    /// Patient-approved clinical/log text.
    pub content_text: String,
    /// Unix second when this observation or note was recorded.
    pub recorded_at: u64,
}

/// Shared patient record visible to a connected practice while connection is active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedPatientRecord {
    /// Owning Duet account.
    pub account_id: Uuid,
    /// Connected practice and roster patient.
    pub patient: RosterPatient,
    /// Stable record identifier.
    pub record_id: String,
    /// Revision token.
    pub revision: String,
    /// Record kind.
    pub record_type: String,
    /// Title.
    pub title: String,
    /// Shared record text.
    pub content_text: String,
    /// Unix second recorded.
    pub recorded_at: u64,
    /// Unix second staged/updated in front door.
    pub updated_at: u64,
}

/// Active connected patient link for a practice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsentedPatientLink {
    /// Connected Duet account.
    pub account: Account,
    /// Connected roster patient.
    pub patient: RosterPatient,
    /// Timestamp (`connected_at`) when this connection was established/re-established.
    pub consented_at: u64,
}

/// Resolves one `(practice_id, patient_id)` roster status from its connection rows
/// and optional open invitation. Single shared rule for memory and SQL stores.
pub(crate) fn resolve_roster_status<'a>(
    connections: impl IntoIterator<Item = (Uuid, &'a ConnectionStanding)>,
    invitation: Option<&Invitation>,
) -> RosterConnectionStatus {
    let mut best_ended: Option<(Uuid, u64, u64, ConnectionEndReason)> = None;
    for (account_id, standing) in connections {
        match *standing {
            ConnectionStanding::Connected { connected_at } => {
                return RosterConnectionStatus::Connected {
                    account_id,
                    connected_at,
                };
            }
            ConnectionStanding::Ended {
                connected_at,
                ended_at,
                reason,
            } => {
                let replace = best_ended.is_none_or(|(_, _, prev_ended, _)| ended_at >= prev_ended);
                if replace {
                    best_ended = Some((account_id, connected_at, ended_at, reason));
                }
            }
        }
    }
    if let Some(inv) = invitation {
        return RosterConnectionStatus::Invited {
            email: inv.email.clone(),
            display_name: inv.display_name.clone(),
            invited_at: inv.invited_at,
            wrong_dob_attempts: inv.wrong_dob_attempts,
            locked_at: inv.locked_at,
            delivery_failed_at: inv.last_delivery_failed_at,
            delivery_error: inv.last_delivery_error.clone(),
        };
    }
    if let Some((account_id, connected_at, ended_at, reason)) = best_ended {
        return RosterConnectionStatus::Ended {
            account_id,
            connected_at,
            ended_at,
            reason,
        };
    }
    RosterConnectionStatus::NotInvited
}

/// Resolves which active `Connected` connection a `"For my doctor"` send targets.
pub(crate) fn resolve_for_my_doctor_target(
    active: &[(&RosterPatient, u64)],
    requested: Option<&RosterPatient>,
) -> Result<(RosterPatient, u64), ForMyDoctorError> {
    if let Some(target) = requested {
        return active
            .iter()
            .find(|(patient, _)| *patient == target)
            .map(|(patient, ts)| ((*patient).clone(), *ts))
            .ok_or(ForMyDoctorError::Unlinked);
    }
    match active {
        [] => Err(ForMyDoctorError::Unlinked),
        [(only_patient, ts)] => Ok(((*only_patient).clone(), *ts)),
        _ => Err(ForMyDoctorError::InvalidInput(
            "target practice is required when connected to multiple practices",
        )),
    }
}

/// The durable front-door owner.
///
/// `&self` everywhere so the server can share one store across handlers.
/// `async_trait` keeps the trait object-safe: a server holds
/// `Arc<dyn FrontDoorStore>` and picks the SQL or memory store at startup.
#[async_trait]
pub trait FrontDoorStore: Send + Sync {
    /// Records or replaces the open invitation for `(patient.practice_id, patient.patient_id)`.
    ///
    /// Existing Duet account emails may be invited (for another practice or after an
    /// `Ended` connection); only an already-`Connected` roster patient returns `Conflict`.
    async fn invite(
        &self,
        patient: &RosterPatient,
        email: &str,
        display_name: &str,
        now: u64,
    ) -> Result<String, InviteError>;
    /// Records or replaces the open invitation for a roster patient with chart DOB,
    /// practice/physician names, link token, and short code (`COD-53`).
    ///
    /// Refuses with [`InviteError::Conflict`] if actively `Connected`, or
    /// [`InviteError::Locked`] if an existing open invitation is `Locked` (`COD-49`).
    async fn invite_patient(
        &self,
        params: InvitePatientParams,
    ) -> Result<IssuedInvitation, InviteError>;
    /// Re-mails the existing unlocked open invitation for `patient`, minting a fresh
    /// link token and short code while keeping all prior credentials valid (`Rule 4`).
    async fn resend_invitation(
        &self,
        patient: &RosterPatient,
        now: u64,
    ) -> Result<IssuedInvitation, InviteError>;
    /// Cancels the open invitation for `patient`, closing every link and short code (`Rule 5`).
    async fn cancel_invitation(&self, patient: &RosterPatient) -> Result<(), InviteError>;
    /// Records an outbound mail delivery failure while keeping the invitation open (`Rule 8`).
    async fn mark_invitation_delivery_failed(
        &self,
        patient: &RosterPatient,
        detail: &str,
        now: u64,
    ) -> Result<(), InviteError>;
    /// Inspects whether a link token or short code maps to an open invitation (`Active` vs `Locked`)
    /// without leaking practice/physician preview (`Rule 6`) or granting consent (`Rule 3`).
    async fn inspect_invitation(
        &self,
        credential: &InvitationCredential,
    ) -> Result<InvitationGateState, InvitationCheckError>;
    /// Verifies the patient's entered DOB (`YYYY-MM-DD`) and `18+` eligibility on `credential`.
    /// Increments wrong-attempt count and locks at 5 (`COD-48`). Still grants no consent (`Rule 3`).
    async fn verify_invitation_dob(
        &self,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<InvitationSharingPreview, InvitationCheckError>;
    /// Connects the currently signed-in Duet account (`Rule 9`) on explicit `Share` tap (`Rule 2`).
    /// Re-verifies open standing, unlocked state, matching DOB, and `18+` atomically.
    /// When `account_id` is `None`, resolves/provisions by `account_email` (required) without
    /// duplicating an existing account.
    async fn accept_invitation_share(
        &self,
        account_id: Option<Uuid>,
        account_email: Option<&str>,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<(Uuid, AccountConnection), InvitationAcceptError>;
    /// Reads an open invitation by email.
    async fn invitation(&self, email: &str) -> Option<Invitation>;
    /// Reads the open invitation for `(patient.practice_id, patient.patient_id)`, if any.
    async fn invitation_for(&self, patient: &RosterPatient) -> Option<Invitation>;
    /// Redeems an open invitation by `email` + `token`: attaches `(account_id, practice_id, patient_id)`
    /// as `Connected` (reusing the existing account if `email` already holds one, otherwise creating
    /// a new account) and mints one non-expiring bearer session.
    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(Uuid, String), SignInError>;
    /// Issues a sign-in token for an existing active account's email.
    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError>;
    /// Verifies a sign-in token and mints one non-expiring bearer session from it.
    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(Uuid, String), SignInError>;
    /// Resolves a bearer token to its account id (sessions never expire with time).
    async fn session(&self, token: &str, now: u64) -> Result<Uuid, SessionError>;
    /// Ends one session so its token stops resolving.
    async fn end_session(&self, token: &str);
    /// Reads one account by email.
    async fn account_by_email(&self, email: &str) -> Option<Account>;
    /// Reads one account by id.
    async fn account(&self, account_id: Uuid) -> Option<Account>;
    /// Lists all practice connections (`Connected` and `Ended`) for `account_id`.
    async fn connections(&self, account_id: Uuid) -> Result<Vec<AccountConnection>, LinkError>;
    /// Lists the front-door accounts actively `Connected` to `practice_id`, sorted by account id.
    async fn connected_accounts(&self, practice_id: &str) -> Vec<(Account, RosterPatient)>;
    /// Ends one active `Connected` connection between `account_id` and `patient`, preserving
    /// other practice connections and historical records.
    async fn end_connection(
        &self,
        account_id: Uuid,
        patient: &RosterPatient,
        reason: ConnectionEndReason,
        now: u64,
    ) -> Result<(), LinkError>;
    /// Patient leaves a practice in Duet (`X1`): ends every active `Connected` connection
    /// between `account_id` and `practice_id` with [`ConnectionEndReason::PatientLeft`].
    async fn leave_connection(
        &self,
        account_id: Uuid,
        practice_id: &str,
        now: u64,
    ) -> Result<Vec<AccountConnection>, LinkError>;
    /// Practice discharges a roster patient (`X2`): ends the active `Connected` connection
    /// for `patient` with [`ConnectionEndReason::PracticeDischarged`].
    async fn discharge_connection(
        &self,
        patient: &RosterPatient,
        now: u64,
    ) -> Result<AccountConnection, LinkError>;
    /// Ends every active `Connected` connection for `account_id` with
    /// [`ConnectionEndReason::AccountDeleted`] (`X3`), deactivates the account, and revokes open
    /// sessions and sign-in tokens while preserving `Ended` roster standing and historical records.
    async fn end_all_connections_for_account(
        &self,
        account_id: Uuid,
        now: u64,
    ) -> Result<usize, ForMyDoctorError>;
    /// Reads the canonical connection status (`NotInvited` / `Invited` / `Connected` / `Ended`)
    /// for one roster patient.
    async fn roster_connection(
        &self,
        patient: &RosterPatient,
    ) -> Result<RosterConnectionStatus, ForMyDoctorError>;
    /// Reads canonical non-`NotInvited` connection statuses keyed by `patient_id` for `practice_id`.
    async fn practice_connections(
        &self,
        practice_id: &str,
    ) -> Result<BTreeMap<String, RosterConnectionStatus>, ForMyDoctorError>;
    /// Sends one patient-prepared `"For my doctor"` share to an actively `Connected` practice.
    async fn send_for_my_doctor(
        &self,
        account_id: Uuid,
        request: ForMyDoctorSendRequest,
        now: u64,
    ) -> Result<ForMyDoctorShare, ForMyDoctorError>;
    /// Lists `"For my doctor"` shares sent by `account_id`, showing only `Sent` vs `Received`.
    async fn for_my_doctor_shares_for_account(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<ForMyDoctorShare>, ForMyDoctorError>;
    /// Lists unreceived (`Sent`) `"For my doctor"` shares awaiting pickup by `practice_id`.
    async fn pending_for_my_doctor_shares_for_practice(
        &self,
        practice_id: &str,
    ) -> Result<Vec<ForMyDoctorShare>, ForMyDoctorError>;
    /// Marks one `"For my doctor"` share as `Received` once picked up by the practice's agents.
    async fn mark_for_my_doctor_received(
        &self,
        practice_id: &str,
        share_id: &str,
        received_at: u64,
    ) -> Result<ForMyDoctorShare, ForMyDoctorError>;
    /// Stages or updates existing Duet shared records on `account_id` (readable by connected practices).
    async fn upsert_shared_records(
        &self,
        account_id: Uuid,
        records: &[SharedPatientRecordInput],
        now: u64,
    ) -> Result<(), ForMyDoctorError>;
    /// Lists actively `Connected` patients in `practice_id`.
    async fn consented_links_for_practice(
        &self,
        practice_id: &str,
    ) -> Result<Vec<ConsentedPatientLink>, ForMyDoctorError>;
    /// Lists existing Duet shared records for `patient_id` in `practice_id` when actively `Connected`.
    async fn shared_records_for_practice_patient(
        &self,
        practice_id: &str,
        patient_id: &str,
    ) -> Result<Vec<SharedPatientRecord>, ForMyDoctorError>;
    /// Lists distinct `practice_id`s with active `Connected` patients or pending `"For my doctor"` shares.
    async fn connected_care_practices(&self) -> Result<Vec<String>, ForMyDoctorError>;
}

/// The SHA-256 of a presented token: the only form the store keeps.
pub(crate) fn token_hash(token: &str) -> [u8; 32] {
    Sha256::digest(token.as_bytes()).into()
}

/// Normalizes a human-entered short invitation code (`XXXX-XXXX` -> `XXXXXXXX`).
pub fn normalize_short_code(raw: &str) -> String {
    raw.trim()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Domain-separated SHA-256 hash of a normalized short code.
pub(crate) fn short_code_hash(normalized: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"code:");
    hasher.update(normalized.as_bytes());
    hasher.finalize().into()
}

/// Mints one presented token. The store keeps only its hash.
pub(crate) fn mint_token() -> String {
    format!("codaf-{}", Uuid::new_v4().simple())
}

/// Mints an 8-character human-enterable short code (`XXXX-XXXX`) from CSPRNG bytes.
pub(crate) fn mint_short_code() -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let bytes = *Uuid::new_v4().as_bytes();
    let mut out = String::with_capacity(9);
    for (idx, b) in bytes.iter().take(8).enumerate() {
        if idx == 4 {
            out.push('-');
        }
        out.push(ALPHABET[usize::from(b & 31)] as char);
    }
    out
}

#[derive(Debug, Default)]
struct MemoryState {
    accounts: HashMap<Uuid, StoredAccount>,
    emails: HashMap<String, Uuid>,
    invitations: HashMap<(String, String), InvitationPending>,
    invitation_credentials: HashMap<[u8; 32], (u64, String, String)>,
    next_invitation_id: u64,
    connections: HashMap<(Uuid, String, String), ConnectionStanding>,
    pending_sign_ins: HashMap<String, PendingSignIn>,
    sessions: HashMap<[u8; 32], (Uuid, u64)>,
    for_my_doctor_shares: HashMap<String, ForMyDoctorShare>,
    shared_records: HashMap<(Uuid, String), StoredSharedRecord>,
}

#[derive(Debug, Clone)]
struct StoredAccount {
    id: Uuid,
    email: String,
    display_name: String,
    active: bool,
    sessions_ended_before: u64,
}

#[derive(Debug, Clone)]
struct StoredSharedRecord {
    record_id: String,
    revision: String,
    record_type: String,
    title: String,
    content_text: String,
    recorded_at: u64,
    updated_at: u64,
}

#[derive(Debug, Clone)]
struct InvitationPending {
    id: u64,
    patient: RosterPatient,
    email: String,
    display_name: String,
    invited_at: u64,
    practice_name: String,
    physician_name: String,
    expected_birth_date: Option<String>,
    wrong_dob_attempts: u8,
    locked_at: Option<u64>,
    last_delivery_failed_at: Option<u64>,
    last_delivery_error: Option<String>,
    updated_at: u64,
}

impl InvitationPending {
    fn to_public(&self) -> Invitation {
        Invitation {
            patient: self.patient.clone(),
            email: self.email.clone(),
            display_name: self.display_name.clone(),
            invited_at: self.invited_at,
            practice_name: self.practice_name.clone(),
            physician_name: self.physician_name.clone(),
            expected_birth_date: self.expected_birth_date.clone(),
            wrong_dob_attempts: self.wrong_dob_attempts,
            locked_at: self.locked_at,
            last_delivery_failed_at: self.last_delivery_failed_at,
            last_delivery_error: self.last_delivery_error.clone(),
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone)]
struct PendingSignIn {
    token_hash: [u8; 32],
    expires_at: u64,
}

/// The process-memory store. A deployment's SQL store replaces it.
#[derive(Debug, Default)]
pub struct MemoryFrontDoorStore {
    state: Mutex<MemoryState>,
}

impl MemoryFrontDoorStore {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Seeds one actively `Connected` account directly, for fixtures and staging setups.
    /// The account signs in through [`FrontDoorStore::begin_sign_in`].
    pub fn seed_linked_account(
        &self,
        patient: &RosterPatient,
        email: &str,
        display_name: &str,
    ) -> Uuid {
        let mut state = self.state.lock().expect("front-door state");
        let key = email.to_lowercase();
        let id = if let Some(&existing) = state.emails.get(&key) {
            existing
        } else {
            let id = Uuid::new_v4();
            state.emails.insert(key.clone(), id);
            state.accounts.insert(
                id,
                StoredAccount {
                    id,
                    email: key,
                    display_name: display_name.to_owned(),
                    active: true,
                    sessions_ended_before: 0,
                },
            );
            id
        };
        state.connections.insert(
            (id, patient.practice_id.clone(), patient.patient_id.clone()),
            ConnectionStanding::Connected { connected_at: 1 },
        );
        id
    }

    fn hydrate_account(state: &MemoryState, acct: &StoredAccount) -> Account {
        let mut connections: Vec<AccountConnection> = state
            .connections
            .iter()
            .filter(|((aid, _, _), _)| *aid == acct.id)
            .map(
                |((_, practice_id, patient_id), standing)| AccountConnection {
                    patient: RosterPatient {
                        practice_id: practice_id.clone(),
                        patient_id: patient_id.clone(),
                    },
                    standing: *standing,
                },
            )
            .collect();
        connections.sort_by(|a, b| a.patient.cmp(&b.patient));
        Account {
            id: acct.id,
            email: acct.email.clone(),
            display_name: acct.display_name.clone(),
            active: acct.active,
            connections,
            sessions_ended_before: acct.sessions_ended_before,
        }
    }

    fn mint_session(&self, state: &mut MemoryState, id: Uuid, now: u64) -> String {
        let token = mint_token();
        let hash = token_hash(&token);
        state.sessions.insert(hash, (id, now));
        token
    }

    fn roster_status_in(state: &MemoryState, patient: &RosterPatient) -> RosterConnectionStatus {
        let conns = state
            .connections
            .iter()
            .filter(|((_, prac, pat), _)| {
                prac == &patient.practice_id && pat == &patient.patient_id
            })
            .map(|((aid, _, _), standing)| (*aid, standing));
        let inv = state
            .invitations
            .get(&(patient.practice_id.clone(), patient.patient_id.clone()))
            .map(InvitationPending::to_public);
        resolve_roster_status(conns, inv.as_ref())
    }
}

#[async_trait]
impl FrontDoorStore for MemoryFrontDoorStore {
    async fn invite(
        &self,
        patient: &RosterPatient,
        email: &str,
        display_name: &str,
        now: u64,
    ) -> Result<String, InviteError> {
        let issued = self
            .invite_patient(InvitePatientParams {
                patient: patient.clone(),
                email: email.to_owned(),
                display_name: display_name.to_owned(),
                practice_name: patient.practice_id.clone(),
                physician_name: String::new(),
                expected_birth_date: None,
                now,
            })
            .await?;
        Ok(issued.link_token)
    }

    async fn invite_patient(
        &self,
        params: InvitePatientParams,
    ) -> Result<IssuedInvitation, InviteError> {
        let key = params.email.trim().to_lowercase();
        if !key.contains('@') {
            return Err(InviteError::InvalidInput("email must be valid"));
        }
        let normalized_dob = match params.expected_birth_date.as_deref().map(str::trim) {
            Some("") | None => None,
            Some(raw) => {
                let parsed = parse_birth_date(raw).ok_or(InviteError::InvalidInput(
                    "expected_birth_date must be YYYY-MM-DD",
                ))?;
                Some(format_birth_date(parsed))
            }
        };

        let mut state = self.state.lock().expect("front-door state");
        let already_connected = state.connections.iter().any(|((_, prac, pat), standing)| {
            prac == &params.patient.practice_id
                && pat == &params.patient.patient_id
                && matches!(standing, ConnectionStanding::Connected { .. })
        });
        if already_connected {
            return Err(InviteError::Conflict);
        }
        let roster_key = (
            params.patient.practice_id.clone(),
            params.patient.patient_id.clone(),
        );
        if let Some(existing) = state.invitations.get(&roster_key) {
            if existing.locked_at.is_some() || existing.wrong_dob_attempts >= MAX_DOB_ATTEMPTS {
                return Err(InviteError::Locked);
            }
            return Err(InviteError::Conflict);
        }

        state.next_invitation_id += 1;
        let inv_id = state.next_invitation_id;
        let token = mint_token();
        let short_code = mint_short_code();
        let tok_hash = token_hash(&token);
        let code_hash = short_code_hash(&normalize_short_code(&short_code));
        state.invitation_credentials.insert(
            tok_hash,
            (inv_id, roster_key.0.clone(), roster_key.1.clone()),
        );
        state.invitation_credentials.insert(
            code_hash,
            (inv_id, roster_key.0.clone(), roster_key.1.clone()),
        );

        let pending = InvitationPending {
            id: inv_id,
            patient: params.patient.clone(),
            email: key,
            display_name: params.display_name.trim().to_owned(),
            invited_at: params.now,
            practice_name: params.practice_name.trim().to_owned(),
            physician_name: params.physician_name.trim().to_owned(),
            expected_birth_date: normalized_dob,
            wrong_dob_attempts: 0,
            locked_at: None,
            last_delivery_failed_at: None,
            last_delivery_error: None,
            updated_at: params.now,
        };
        let public = pending.to_public();
        state.invitations.insert(roster_key, pending);
        Ok(IssuedInvitation {
            link_token: token,
            short_code,
            invitation: public,
        })
    }

    async fn resend_invitation(
        &self,
        patient: &RosterPatient,
        now: u64,
    ) -> Result<IssuedInvitation, InviteError> {
        let mut state = self.state.lock().expect("front-door state");
        let already_connected = state.connections.iter().any(|((_, prac, pat), standing)| {
            prac == &patient.practice_id
                && pat == &patient.patient_id
                && matches!(standing, ConnectionStanding::Connected { .. })
        });
        if already_connected {
            return Err(InviteError::Conflict);
        }
        let roster_key = (patient.practice_id.clone(), patient.patient_id.clone());
        let (inv_id, public) = {
            let pending = state
                .invitations
                .get_mut(&roster_key)
                .ok_or(InviteError::NotFound)?;
            if pending.locked_at.is_some() || pending.wrong_dob_attempts >= MAX_DOB_ATTEMPTS {
                return Err(InviteError::Locked);
            }
            pending.last_delivery_failed_at = None;
            pending.last_delivery_error = None;
            pending.updated_at = now;
            (pending.id, pending.to_public())
        };
        let link_token = mint_token();
        let short_code = mint_short_code();
        state.invitation_credentials.insert(
            token_hash(&link_token),
            (inv_id, roster_key.0.clone(), roster_key.1.clone()),
        );
        state.invitation_credentials.insert(
            short_code_hash(&normalize_short_code(&short_code)),
            (inv_id, roster_key.0, roster_key.1),
        );
        Ok(IssuedInvitation {
            link_token,
            short_code,
            invitation: public,
        })
    }

    async fn cancel_invitation(&self, patient: &RosterPatient) -> Result<(), InviteError> {
        let mut state = self.state.lock().expect("front-door state");
        let roster_key = (patient.practice_id.clone(), patient.patient_id.clone());
        let removed = state
            .invitations
            .remove(&roster_key)
            .ok_or(InviteError::NotFound)?;
        state
            .invitation_credentials
            .retain(|_, (inv_id, _, _)| *inv_id != removed.id);
        Ok(())
    }

    async fn mark_invitation_delivery_failed(
        &self,
        patient: &RosterPatient,
        detail: &str,
        now: u64,
    ) -> Result<(), InviteError> {
        let mut state = self.state.lock().expect("front-door state");
        let roster_key = (patient.practice_id.clone(), patient.patient_id.clone());
        let pending = state
            .invitations
            .get_mut(&roster_key)
            .ok_or(InviteError::NotFound)?;
        pending.last_delivery_failed_at = Some(now);
        pending.last_delivery_error = Some(detail.trim().chars().take(200).collect());
        Ok(())
    }

    async fn inspect_invitation(
        &self,
        credential: &InvitationCredential,
    ) -> Result<InvitationGateState, InvitationCheckError> {
        let secret_hash = credential
            .secret_hash()
            .ok_or(InvitationCheckError::NotOpen)?;
        let state = self.state.lock().expect("front-door state");
        let (inv_id, prac, pat) = state
            .invitation_credentials
            .get(&secret_hash)
            .ok_or(InvitationCheckError::NotOpen)?;
        let pending = state
            .invitations
            .get(&(prac.clone(), pat.clone()))
            .filter(|p| p.id == *inv_id)
            .ok_or(InvitationCheckError::NotOpen)?;
        let standing = if let Some(locked_at) = pending.locked_at {
            InvitationOpenStanding::Locked { locked_at }
        } else if pending.wrong_dob_attempts >= MAX_DOB_ATTEMPTS {
            InvitationOpenStanding::Locked {
                locked_at: pending.updated_at,
            }
        } else {
            InvitationOpenStanding::Active {
                wrong_dob_attempts: pending.wrong_dob_attempts,
                attempts_remaining: MAX_DOB_ATTEMPTS.saturating_sub(pending.wrong_dob_attempts),
            }
        };
        Ok(InvitationGateState { standing })
    }

    async fn verify_invitation_dob(
        &self,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<InvitationSharingPreview, InvitationCheckError> {
        let parsed = parse_birth_date(birth_date).ok_or(InvitationCheckError::InvalidBirthDate)?;
        let normalized_input = format_birth_date(parsed);
        let secret_hash = credential
            .secret_hash()
            .ok_or(InvitationCheckError::NotOpen)?;
        let mut state = self.state.lock().expect("front-door state");
        let (inv_id, prac, pat) = state
            .invitation_credentials
            .get(&secret_hash)
            .cloned()
            .ok_or(InvitationCheckError::NotOpen)?;
        let pending = state
            .invitations
            .get_mut(&(prac, pat))
            .filter(|p| p.id == inv_id)
            .ok_or(InvitationCheckError::NotOpen)?;

        if pending.locked_at.is_some() || pending.wrong_dob_attempts >= MAX_DOB_ATTEMPTS {
            return Err(InvitationCheckError::Locked);
        }

        let Some(expected) = pending.expected_birth_date.as_deref() else {
            return Err(InvitationCheckError::NotOpen);
        };
        if expected != normalized_input {
            pending.wrong_dob_attempts = pending.wrong_dob_attempts.saturating_add(1);
            if pending.wrong_dob_attempts >= MAX_DOB_ATTEMPTS {
                pending.locked_at = Some(now);
                return Err(InvitationCheckError::Locked);
            }
            return Err(InvitationCheckError::WrongBirthDate {
                attempts_remaining: MAX_DOB_ATTEMPTS.saturating_sub(pending.wrong_dob_attempts),
            });
        }
        if !is_adult_on(parsed, now) {
            return Err(InvitationCheckError::Underage);
        }
        Ok(InvitationSharingPreview {
            practice_id: pending.patient.practice_id.clone(),
            practice_name: if pending.practice_name.is_empty() {
                pending.patient.practice_id.clone()
            } else {
                pending.practice_name.clone()
            },
            physician_name: pending.physician_name.clone(),
            display_name: pending.display_name.clone(),
            shares_summary: SHARES_SUMMARY.to_owned(),
        })
    }

    async fn accept_invitation_share(
        &self,
        account_id: Option<Uuid>,
        account_email: Option<&str>,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<(Uuid, AccountConnection), InvitationAcceptError> {
        let parsed = parse_birth_date(birth_date).ok_or(InvitationCheckError::InvalidBirthDate)?;
        let normalized_input = format_birth_date(parsed);
        let secret_hash = credential
            .secret_hash()
            .ok_or(InvitationCheckError::NotOpen)?;
        let mut state = self.state.lock().expect("front-door state");
        let (inv_id, prac, pat) = state
            .invitation_credentials
            .get(&secret_hash)
            .cloned()
            .ok_or(InvitationCheckError::NotOpen)?;
        let roster_key = (prac, pat);
        {
            let pending = state
                .invitations
                .get_mut(&roster_key)
                .filter(|p| p.id == inv_id)
                .ok_or(InvitationCheckError::NotOpen)?;
            if pending.locked_at.is_some() || pending.wrong_dob_attempts >= MAX_DOB_ATTEMPTS {
                return Err(InvitationCheckError::Locked.into());
            }
            let Some(expected) = pending.expected_birth_date.as_deref() else {
                return Err(InvitationCheckError::NotOpen.into());
            };
            if expected != normalized_input {
                pending.wrong_dob_attempts = pending.wrong_dob_attempts.saturating_add(1);
                if pending.wrong_dob_attempts >= MAX_DOB_ATTEMPTS {
                    pending.locked_at = Some(now);
                    return Err(InvitationCheckError::Locked.into());
                }
                return Err(InvitationCheckError::WrongBirthDate {
                    attempts_remaining: MAX_DOB_ATTEMPTS.saturating_sub(pending.wrong_dob_attempts),
                }
                .into());
            }
            if !is_adult_on(parsed, now) {
                return Err(InvitationCheckError::Underage.into());
            }
        }

        let target_account_id = match account_id {
            Some(existing_id) => {
                let acct = state.accounts.get(&existing_id);
                if let Some(acct) = acct {
                    if !acct.active {
                        return Err(InvitationAcceptError::InactiveAccount);
                    }
                    existing_id
                } else {
                    let email_key = account_email
                        .map(|e| e.trim().to_lowercase())
                        .filter(|e| !e.is_empty())
                        .unwrap_or_else(|| {
                            state
                                .invitations
                                .get(&roster_key)
                                .map(|p| p.email.clone())
                                .unwrap_or_default()
                        });
                    let name = state
                        .invitations
                        .get(&roster_key)
                        .map(|p| p.display_name.clone())
                        .unwrap_or_default();
                    state.emails.insert(email_key.clone(), existing_id);
                    state.accounts.insert(
                        existing_id,
                        StoredAccount {
                            id: existing_id,
                            email: email_key,
                            display_name: name,
                            active: true,
                            sessions_ended_before: 0,
                        },
                    );
                    existing_id
                }
            }
            None => {
                let email_key = account_email
                    .map(|e| e.trim().to_lowercase())
                    .filter(|e| !e.is_empty())
                    .ok_or(InvitationAcceptError::InactiveAccount)?;
                if let Some(&existing_id) = state.emails.get(&email_key) {
                    let acct = state
                        .accounts
                        .get(&existing_id)
                        .ok_or(InvitationAcceptError::InactiveAccount)?;
                    if !acct.active {
                        return Err(InvitationAcceptError::InactiveAccount);
                    }
                    existing_id
                } else {
                    let new_id = Uuid::new_v4();
                    let name = state
                        .invitations
                        .get(&roster_key)
                        .map(|p| p.display_name.clone())
                        .unwrap_or_default();
                    state.emails.insert(email_key.clone(), new_id);
                    state.accounts.insert(
                        new_id,
                        StoredAccount {
                            id: new_id,
                            email: email_key,
                            display_name: name,
                            active: true,
                            sessions_ended_before: 0,
                        },
                    );
                    new_id
                }
            }
        };

        let other_active = state
            .connections
            .iter()
            .any(|((aid, prac, pat), standing)| {
                *aid != target_account_id
                    && prac == &roster_key.0
                    && pat == &roster_key.1
                    && matches!(standing, ConnectionStanding::Connected { .. })
            });
        if other_active {
            return Err(InvitationAcceptError::Conflict);
        }

        let removed = state
            .invitations
            .remove(&roster_key)
            .ok_or(InvitationCheckError::NotOpen)?;
        state
            .invitation_credentials
            .retain(|_, (id, _, _)| *id != removed.id);

        let standing = ConnectionStanding::Connected { connected_at: now };
        state.connections.insert(
            (
                target_account_id,
                removed.patient.practice_id.clone(),
                removed.patient.patient_id.clone(),
            ),
            standing,
        );
        Ok((
            target_account_id,
            AccountConnection {
                patient: removed.patient,
                standing,
            },
        ))
    }

    async fn invitation(&self, email: &str) -> Option<Invitation> {
        let key = email.to_lowercase();
        let state = self.state.lock().expect("front-door state");
        state
            .invitations
            .values()
            .filter(|pending| pending.email == key)
            .max_by_key(|pending| pending.invited_at)
            .map(InvitationPending::to_public)
    }

    async fn invitation_for(&self, patient: &RosterPatient) -> Option<Invitation> {
        let state = self.state.lock().expect("front-door state");
        state
            .invitations
            .get(&(patient.practice_id.clone(), patient.patient_id.clone()))
            .map(InvitationPending::to_public)
    }

    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(Uuid, String), SignInError> {
        let key = email.to_lowercase();
        let presented_hash = token_hash(token);
        let mut state = self.state.lock().expect("front-door state");
        let roster_key =
            if let Some((_, prac, pat)) = state.invitation_credentials.get(&presented_hash) {
                (prac.clone(), pat.clone())
            } else {
                return Err(SignInError::Invalid);
            };
        let Some(existing_pending) = state.invitations.get(&roster_key) else {
            return Err(SignInError::Invalid);
        };
        if existing_pending.email != key
            || existing_pending.locked_at.is_some()
            || existing_pending.expected_birth_date.is_some()
        {
            return Err(SignInError::Invalid);
        }

        if let Some(&existing_id) = state.emails.get(&key)
            && !state
                .accounts
                .get(&existing_id)
                .ok_or(SignInError::Invalid)?
                .active
        {
            return Err(SignInError::Inactive);
        }

        let pending = state
            .invitations
            .remove(&roster_key)
            .ok_or(SignInError::Invalid)?;
        state
            .invitation_credentials
            .retain(|_, (id, _, _)| *id != pending.id);

        let id = if let Some(&existing_id) = state.emails.get(&key) {
            existing_id
        } else {
            let new_id = Uuid::new_v4();
            state.emails.insert(key.clone(), new_id);
            state.accounts.insert(
                new_id,
                StoredAccount {
                    id: new_id,
                    email: key,
                    display_name: pending.display_name.clone(),
                    active: true,
                    sessions_ended_before: 0,
                },
            );
            new_id
        };

        let other_active = state
            .connections
            .iter()
            .any(|((aid, prac, pat), standing)| {
                *aid != id
                    && prac == &pending.patient.practice_id
                    && pat == &pending.patient.patient_id
                    && matches!(standing, ConnectionStanding::Connected { .. })
            });
        if other_active {
            return Err(SignInError::Invalid);
        }

        state.connections.insert(
            (
                id,
                pending.patient.practice_id.clone(),
                pending.patient.patient_id.clone(),
            ),
            ConnectionStanding::Connected { connected_at: now },
        );
        let session = self.mint_session(&mut state, id, now);
        Ok((id, session))
    }

    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError> {
        let key = email.to_lowercase();
        let mut state = self.state.lock().expect("front-door state");
        let id = state
            .emails
            .get(&key)
            .copied()
            .ok_or(SignInError::Invalid)?;
        if !state.accounts.get(&id).ok_or(SignInError::Invalid)?.active {
            return Err(SignInError::Inactive);
        }
        // One pending token per email: re-requesting replaces the last one.
        let token = mint_token();
        state.pending_sign_ins.insert(
            key,
            PendingSignIn {
                token_hash: token_hash(&token),
                expires_at: now + SIGN_IN_TOKEN_TTL_SECONDS,
            },
        );
        Ok(token)
    }

    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(Uuid, String), SignInError> {
        let key = email.to_lowercase();
        let mut state = self.state.lock().expect("front-door state");
        let Some(pending) = state.pending_sign_ins.remove(&key) else {
            return Err(SignInError::Invalid);
        };
        if pending.expires_at <= now || pending.token_hash != token_hash(token) {
            return Err(SignInError::Invalid);
        }
        let id = state
            .emails
            .get(&key)
            .copied()
            .ok_or(SignInError::Invalid)?;
        if !state.accounts.get(&id).ok_or(SignInError::Invalid)?.active {
            return Err(SignInError::Inactive);
        }
        let session = self.mint_session(&mut state, id, now);
        Ok((id, session))
    }

    async fn session(&self, token: &str, _now: u64) -> Result<Uuid, SessionError> {
        let hash = token_hash(token);
        let state = self.state.lock().expect("front-door state");
        let (id, created_at) = state
            .sessions
            .get(&hash)
            .copied()
            .ok_or(SessionError::Invalid)?;
        let account = state.accounts.get(&id).ok_or(SessionError::Inactive)?;
        if !account.active {
            return Err(SessionError::Inactive);
        }
        if created_at < account.sessions_ended_before {
            return Err(SessionError::Inactive);
        }
        Ok(id)
    }

    async fn end_session(&self, token: &str) {
        self.state
            .lock()
            .expect("front-door state")
            .sessions
            .remove(&token_hash(token));
    }

    async fn account_by_email(&self, email: &str) -> Option<Account> {
        let state = self.state.lock().expect("front-door state");
        let id = state.emails.get(&email.to_lowercase()).copied()?;
        let acct = state.accounts.get(&id)?;
        Some(Self::hydrate_account(&state, acct))
    }

    async fn account(&self, account_id: Uuid) -> Option<Account> {
        let state = self.state.lock().expect("front-door state");
        let acct = state.accounts.get(&account_id)?;
        Some(Self::hydrate_account(&state, acct))
    }

    async fn connections(&self, account_id: Uuid) -> Result<Vec<AccountConnection>, LinkError> {
        let state = self.state.lock().expect("front-door state");
        let acct = state.accounts.get(&account_id).ok_or(LinkError::Unlinked)?;
        Ok(Self::hydrate_account(&state, acct).connections)
    }

    async fn connected_accounts(&self, practice_id: &str) -> Vec<(Account, RosterPatient)> {
        let state = self.state.lock().expect("front-door state");
        let mut rows: Vec<(Account, RosterPatient)> = state
            .connections
            .iter()
            .filter_map(|((aid, prac, pat), standing)| {
                if prac != practice_id {
                    return None;
                }
                if !matches!(standing, ConnectionStanding::Connected { .. }) {
                    return None;
                }
                let acct = state.accounts.get(aid)?;
                Some((
                    Self::hydrate_account(&state, acct),
                    RosterPatient {
                        practice_id: prac.clone(),
                        patient_id: pat.clone(),
                    },
                ))
            })
            .collect();
        rows.sort_by(|(la, lp), (ra, rp)| la.id.cmp(&ra.id).then(lp.cmp(rp)));
        rows
    }

    async fn end_connection(
        &self,
        account_id: Uuid,
        patient: &RosterPatient,
        reason: ConnectionEndReason,
        now: u64,
    ) -> Result<(), LinkError> {
        let mut state = self.state.lock().expect("front-door state");
        let key = (
            account_id,
            patient.practice_id.clone(),
            patient.patient_id.clone(),
        );
        let entry = state.connections.get_mut(&key).ok_or(LinkError::Unlinked)?;
        let ConnectionStanding::Connected { connected_at } = *entry else {
            return Err(LinkError::Unlinked);
        };
        *entry = ConnectionStanding::Ended {
            connected_at,
            ended_at: now,
            reason,
        };
        Ok(())
    }

    async fn leave_connection(
        &self,
        account_id: Uuid,
        practice_id: &str,
        now: u64,
    ) -> Result<Vec<AccountConnection>, LinkError> {
        let mut state = self.state.lock().expect("front-door state");
        let mut ended = Vec::new();
        for ((aid, prac, pat), standing) in &mut state.connections {
            if *aid == account_id
                && prac == practice_id
                && let ConnectionStanding::Connected { connected_at } = *standing
            {
                let next = ConnectionStanding::Ended {
                    connected_at,
                    ended_at: now,
                    reason: ConnectionEndReason::PatientLeft,
                };
                *standing = next;
                ended.push(AccountConnection {
                    patient: RosterPatient {
                        practice_id: prac.clone(),
                        patient_id: pat.clone(),
                    },
                    standing: next,
                });
            }
        }
        if ended.is_empty() {
            return Err(LinkError::Unlinked);
        }
        ended.sort_by(|a, b| a.patient.cmp(&b.patient));
        Ok(ended)
    }

    async fn discharge_connection(
        &self,
        patient: &RosterPatient,
        now: u64,
    ) -> Result<AccountConnection, LinkError> {
        let mut state = self.state.lock().expect("front-door state");
        for ((_, prac, pat), standing) in &mut state.connections {
            if prac == &patient.practice_id
                && pat == &patient.patient_id
                && let ConnectionStanding::Connected { connected_at } = *standing
            {
                let next = ConnectionStanding::Ended {
                    connected_at,
                    ended_at: now,
                    reason: ConnectionEndReason::PracticeDischarged,
                };
                *standing = next;
                return Ok(AccountConnection {
                    patient: patient.clone(),
                    standing: next,
                });
            }
        }
        Err(LinkError::Unlinked)
    }

    async fn end_all_connections_for_account(
        &self,
        account_id: Uuid,
        now: u64,
    ) -> Result<usize, ForMyDoctorError> {
        let mut state = self.state.lock().expect("front-door state");
        let mut count = 0;
        for ((aid, _, _), standing) in &mut state.connections {
            if *aid == account_id
                && let ConnectionStanding::Connected { connected_at } = *standing
            {
                *standing = ConnectionStanding::Ended {
                    connected_at,
                    ended_at: now,
                    reason: ConnectionEndReason::AccountDeleted,
                };
                count += 1;
            }
        }
        let acct_email = if let Some(acct) = state.accounts.get_mut(&account_id) {
            acct.active = false;
            acct.sessions_ended_before = acct.sessions_ended_before.max(now);
            Some(acct.email.clone())
        } else {
            None
        };
        state.sessions.retain(|_, (aid, _)| *aid != account_id);
        if let Some(email) = acct_email {
            state.pending_sign_ins.remove(&email);
        }
        Ok(count)
    }

    async fn roster_connection(
        &self,
        patient: &RosterPatient,
    ) -> Result<RosterConnectionStatus, ForMyDoctorError> {
        let state = self.state.lock().expect("front-door state");
        Ok(Self::roster_status_in(&state, patient))
    }

    async fn practice_connections(
        &self,
        practice_id: &str,
    ) -> Result<BTreeMap<String, RosterConnectionStatus>, ForMyDoctorError> {
        let state = self.state.lock().expect("front-door state");
        let mut patient_ids: std::collections::BTreeSet<String> = state
            .connections
            .keys()
            .filter(|(_, prac, _)| prac == practice_id)
            .map(|(_, _, pat)| pat.clone())
            .collect();
        patient_ids.extend(
            state
                .invitations
                .keys()
                .filter(|(prac, _)| prac == practice_id)
                .map(|(_, pat)| pat.clone()),
        );
        let mut out = BTreeMap::new();
        for patient_id in patient_ids {
            let status = Self::roster_status_in(
                &state,
                &RosterPatient {
                    practice_id: practice_id.to_owned(),
                    patient_id: patient_id.clone(),
                },
            );
            if !matches!(status, RosterConnectionStatus::NotInvited) {
                out.insert(patient_id, status);
            }
        }
        Ok(out)
    }

    async fn send_for_my_doctor(
        &self,
        account_id: Uuid,
        request: ForMyDoctorSendRequest,
        now: u64,
    ) -> Result<ForMyDoctorShare, ForMyDoctorError> {
        let share_key = request.share_key.trim();
        let title = request.title.trim();
        let summary_text = request.summary_text.trim();
        if share_key.is_empty() {
            return Err(ForMyDoctorError::InvalidInput(
                "share_key must be non-empty",
            ));
        }
        if title.is_empty() {
            return Err(ForMyDoctorError::InvalidInput("title must be non-empty"));
        }
        if summary_text.is_empty() {
            return Err(ForMyDoctorError::InvalidInput(
                "summary_text must be non-empty",
            ));
        }
        let origin_label = request
            .origin_label
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Duet — For my doctor")
            .to_owned();

        let mut state = self.state.lock().expect("front-door state");
        let acct = state
            .accounts
            .get(&account_id)
            .ok_or(ForMyDoctorError::Unlinked)?;
        let hydrated = Self::hydrate_account(&state, acct);
        let active = hydrated.active_connections();
        let (patient, connected_at) =
            resolve_for_my_doctor_target(&active, request.target.as_ref())?;

        if let Some(existing) = state.for_my_doctor_shares.values().find(|s| {
            s.account_id == account_id && s.patient == patient && s.share_key == share_key
        }) {
            if existing.title == title
                && existing.summary_text == summary_text
                && existing.origin_label == origin_label
            {
                return Ok(existing.clone());
            }
            return Err(ForMyDoctorError::Conflict);
        }

        let share = ForMyDoctorShare {
            share_id: format!("fmd-{}", Uuid::new_v4().simple()),
            account_id,
            patient,
            share_key: share_key.to_owned(),
            title: title.to_owned(),
            summary_text: summary_text.to_owned(),
            origin_label,
            sent_at: now,
            consent_at: connected_at,
            standing: ForMyDoctorShareStatus::Sent,
        };
        state
            .for_my_doctor_shares
            .insert(share.share_id.clone(), share.clone());
        Ok(share)
    }

    async fn for_my_doctor_shares_for_account(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<ForMyDoctorShare>, ForMyDoctorError> {
        let state = self.state.lock().expect("front-door state");
        let has_connection = state
            .connections
            .keys()
            .any(|(aid, _, _)| *aid == account_id);
        if !has_connection {
            return Err(ForMyDoctorError::Unlinked);
        }
        let mut out: Vec<_> = state
            .for_my_doctor_shares
            .values()
            .filter(|s| s.account_id == account_id)
            .cloned()
            .collect();
        out.sort_by(|a, b| a.sent_at.cmp(&b.sent_at).then(a.share_id.cmp(&b.share_id)));
        Ok(out)
    }

    async fn pending_for_my_doctor_shares_for_practice(
        &self,
        practice_id: &str,
    ) -> Result<Vec<ForMyDoctorShare>, ForMyDoctorError> {
        let state = self.state.lock().expect("front-door state");
        let mut out: Vec<_> = state
            .for_my_doctor_shares
            .values()
            .filter(|s| {
                s.patient.practice_id == practice_id
                    && matches!(s.standing, ForMyDoctorShareStatus::Sent)
                    && matches!(
                        state.connections.get(&(
                            s.account_id,
                            s.patient.practice_id.clone(),
                            s.patient.patient_id.clone()
                        )),
                        Some(ConnectionStanding::Connected { .. })
                    )
            })
            .cloned()
            .collect();
        out.sort_by(|a, b| a.sent_at.cmp(&b.sent_at).then(a.share_id.cmp(&b.share_id)));
        Ok(out)
    }

    async fn mark_for_my_doctor_received(
        &self,
        practice_id: &str,
        share_id: &str,
        received_at: u64,
    ) -> Result<ForMyDoctorShare, ForMyDoctorError> {
        let mut state = self.state.lock().expect("front-door state");
        let share = state
            .for_my_doctor_shares
            .get_mut(share_id)
            .filter(|s| s.patient.practice_id == practice_id)
            .ok_or(ForMyDoctorError::NotFound)?;
        if matches!(share.standing, ForMyDoctorShareStatus::Sent) {
            share.standing = ForMyDoctorShareStatus::Received { received_at };
        }
        Ok(share.clone())
    }

    async fn upsert_shared_records(
        &self,
        account_id: Uuid,
        records: &[SharedPatientRecordInput],
        now: u64,
    ) -> Result<(), ForMyDoctorError> {
        for rec in records {
            if rec.record_id.trim().is_empty()
                || rec.revision.trim().is_empty()
                || rec.record_type.trim().is_empty()
                || rec.title.trim().is_empty()
                || rec.content_text.trim().is_empty()
            {
                return Err(ForMyDoctorError::InvalidInput(
                    "shared record fields must be non-empty",
                ));
            }
        }
        let mut state = self.state.lock().expect("front-door state");
        if !state.accounts.contains_key(&account_id) {
            return Err(ForMyDoctorError::Unlinked);
        }
        for rec in records {
            let record_id = rec.record_id.trim().to_owned();
            state.shared_records.insert(
                (account_id, record_id.clone()),
                StoredSharedRecord {
                    record_id,
                    revision: rec.revision.trim().to_owned(),
                    record_type: rec.record_type.trim().to_owned(),
                    title: rec.title.trim().to_owned(),
                    content_text: rec.content_text.trim().to_owned(),
                    recorded_at: rec.recorded_at,
                    updated_at: now,
                },
            );
        }
        Ok(())
    }

    async fn consented_links_for_practice(
        &self,
        practice_id: &str,
    ) -> Result<Vec<ConsentedPatientLink>, ForMyDoctorError> {
        let state = self.state.lock().expect("front-door state");
        let mut out = Vec::new();
        for ((aid, prac, pat), standing) in &state.connections {
            if prac != practice_id {
                continue;
            }
            let ConnectionStanding::Connected { connected_at } = *standing else {
                continue;
            };
            let Some(acct) = state.accounts.get(aid) else {
                continue;
            };
            out.push(ConsentedPatientLink {
                account: Self::hydrate_account(&state, acct),
                patient: RosterPatient {
                    practice_id: prac.clone(),
                    patient_id: pat.clone(),
                },
                consented_at: connected_at,
            });
        }
        out.sort_by(|a, b| {
            a.account
                .id
                .cmp(&b.account.id)
                .then(a.patient.cmp(&b.patient))
        });
        Ok(out)
    }

    async fn shared_records_for_practice_patient(
        &self,
        practice_id: &str,
        patient_id: &str,
    ) -> Result<Vec<SharedPatientRecord>, ForMyDoctorError> {
        let state = self.state.lock().expect("front-door state");
        let mut out = Vec::new();
        for ((aid, prac, pat), standing) in &state.connections {
            if prac != practice_id || pat != patient_id {
                continue;
            }
            if !matches!(standing, ConnectionStanding::Connected { .. }) {
                continue;
            }
            let patient = RosterPatient {
                practice_id: prac.clone(),
                patient_id: pat.clone(),
            };
            for ((rec_aid, _), rec) in &state.shared_records {
                if rec_aid == aid {
                    out.push(SharedPatientRecord {
                        account_id: *aid,
                        patient: patient.clone(),
                        record_id: rec.record_id.clone(),
                        revision: rec.revision.clone(),
                        record_type: rec.record_type.clone(),
                        title: rec.title.clone(),
                        content_text: rec.content_text.clone(),
                        recorded_at: rec.recorded_at,
                        updated_at: rec.updated_at,
                    });
                }
            }
        }
        out.sort_by(|a, b| a.record_id.cmp(&b.record_id));
        Ok(out)
    }

    async fn connected_care_practices(&self) -> Result<Vec<String>, ForMyDoctorError> {
        let state = self.state.lock().expect("front-door state");
        let mut practices = std::collections::BTreeSet::new();
        for ((_, prac, _), standing) in &state.connections {
            if matches!(standing, ConnectionStanding::Connected { .. }) {
                practices.insert(prac.clone());
            }
        }
        for share in state.for_my_doctor_shares.values() {
            if matches!(share.standing, ForMyDoctorShareStatus::Sent)
                && matches!(
                    state.connections.get(&(
                        share.account_id,
                        share.patient.practice_id.clone(),
                        share.patient.patient_id.clone()
                    )),
                    Some(ConnectionStanding::Connected { .. })
                )
            {
                practices.insert(share.patient.practice_id.clone());
            }
        }
        Ok(practices.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_760_000_000;

    fn patient(practice: &str, id: &str) -> RosterPatient {
        RosterPatient {
            practice_id: practice.to_owned(),
            patient_id: id.to_owned(),
        }
    }

    /// One invitation and its redemption, the shape every journey starts from.
    async fn invited(store: &MemoryFrontDoorStore, email: &str) -> (RosterPatient, Uuid, String) {
        let patient = patient("roger-synthetic-practice", "patient-1");
        let token = store
            .invite(&patient, email, "Luna Park", NOW)
            .await
            .unwrap();
        let (id, session) = store.redeem_invitation(email, &token, NOW).await.unwrap();
        (patient, id, session)
    }

    #[tokio::test]
    async fn an_invitation_redeems_once_and_mints_one_non_expiring_session() {
        let store = MemoryFrontDoorStore::new();
        let patient = patient("practice", "patient-9");
        let token = store
            .invite(&patient, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        let (id, session) = store
            .redeem_invitation("luna@example.test", &token, NOW + 365 * 86_400)
            .await
            .expect("invitations never expire with time");
        assert_eq!(
            store.session(&session, NOW + 3650 * 86_400).await.unwrap(),
            id,
            "sessions never expire with time"
        );
        let account = store.account_by_email("luna@example.test").await.unwrap();
        assert!(account.active);
        assert_eq!(
            account.active_connection_at(&patient),
            Some(NOW + 365 * 86_400)
        );
        assert_eq!(
            store
                .redeem_invitation("luna@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid,
            "an invitation redeems once"
        );
    }

    #[tokio::test]
    async fn a_wrong_invitation_token_is_invalid() {
        let store = MemoryFrontDoorStore::new();
        let patient = patient("practice", "patient-9");
        let _token = store
            .invite(&patient, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        assert_eq!(
            store
                .redeem_invitation("luna@example.test", "codaf-wrong", NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
    }

    #[tokio::test]
    async fn reinviting_already_invited_roster_patient_conflicts_until_cancelled() {
        let store = MemoryFrontDoorStore::new();
        let patient = patient("practice", "patient-9");
        let first_token = store
            .invite(&patient, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        assert_eq!(
            store
                .invite(&patient, "luna@example.test", "Luna Park", NOW + 1)
                .await
                .unwrap_err(),
            InviteError::Conflict,
            "second Invite(p) while open must fail with Conflict without clobbering first token"
        );
        // After cancelling, a fresh invitation may be issued while the prior token stays invalid.
        store.cancel_invitation(&patient).await.unwrap();
        let second_token = store
            .invite(&patient, "luna@example.test", "Luna Park", NOW + 2)
            .await
            .unwrap();
        assert!(
            store
                .redeem_invitation("luna@example.test", &first_token, NOW + 3)
                .await
                .is_err(),
            "cancelled invitation token must not redeem"
        );
        assert!(
            store
                .redeem_invitation("luna@example.test", &second_token, NOW + 4)
                .await
                .is_ok(),
            "post-cancel invitation redeems"
        );
    }

    #[tokio::test]
    async fn already_connected_patient_conflicts_while_existing_account_joins_second_practice() {
        let store = MemoryFrontDoorStore::new();
        let (patient_a, id, _) = invited(&store, "luna@example.test").await;
        assert_eq!(
            store
                .invite(&patient_a, "luna@example.test", "Luna Park", NOW)
                .await
                .unwrap_err(),
            InviteError::Conflict,
            "cannot invite a roster patient that already holds an active Connected connection"
        );

        let patient_b = patient("second-practice", "patient-2");
        let token_b = store
            .invite(&patient_b, "LUNA@example.test", "Luna Park", NOW + 1)
            .await
            .expect("existing Duet account can be invited to another practice");
        let (second_id, _) = store
            .redeem_invitation("luna@example.test", &token_b, NOW + 2)
            .await
            .unwrap();
        assert_eq!(
            second_id, id,
            "joining another practice attaches to the existing Duet account"
        );
        let account = store.account(id).await.unwrap();
        assert_eq!(account.connections.len(), 2);
        assert_eq!(account.active_connection_at(&patient_a), Some(NOW));
        assert_eq!(account.active_connection_at(&patient_b), Some(NOW + 2));
    }

    #[tokio::test]
    async fn existing_account_can_be_invited_to_a_second_practice_without_expiry() {
        let store = MemoryFrontDoorStore::new();
        let practice_a = patient("practice-a", "patient-1");
        let practice_b = patient("practice-b", "patient-2");
        let token_a = store
            .invite(&practice_a, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        let (account_id, session) = store
            .redeem_invitation("luna@example.test", &token_a, NOW)
            .await
            .unwrap();
        assert_eq!(
            store
                .session(&session, NOW + 365 * 86_400)
                .await
                .expect("session does not expire after 30 days"),
            account_id
        );
        let token_b = store
            .invite(&practice_b, "luna@example.test", "Luna Park", NOW + 1)
            .await
            .expect("existing Duet account can be invited to a second practice");
        let (second_id, _) = store
            .redeem_invitation("luna@example.test", &token_b, NOW + 365 * 86_400)
            .await
            .expect("invitation does not expire after 14 days and connects existing account");
        assert_eq!(second_id, account_id);
        let account = store.account(account_id).await.unwrap();
        assert_eq!(account.active_connections().len(), 2);
    }

    #[tokio::test]
    async fn ending_one_connection_preserves_others_and_allows_reinvite() {
        let store = MemoryFrontDoorStore::new();
        let practice_a = patient("practice-a", "patient-1");
        let practice_b = patient("practice-b", "patient-2");
        let token_a = store
            .invite(&practice_a, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        let (id, _) = store
            .redeem_invitation("luna@example.test", &token_a, NOW)
            .await
            .unwrap();
        let token_b = store
            .invite(&practice_b, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        store
            .redeem_invitation("luna@example.test", &token_b, NOW + 1)
            .await
            .unwrap();

        store
            .end_connection(id, &practice_a, ConnectionEndReason::PatientLeft, NOW + 10)
            .await
            .unwrap();
        assert!(matches!(
            store.roster_connection(&practice_a).await.unwrap(),
            RosterConnectionStatus::Ended {
                account_id,
                ended_at,
                reason: ConnectionEndReason::PatientLeft,
                ..
            } if account_id == id && ended_at == NOW + 10
        ));
        assert!(
            store
                .roster_connection(&practice_b)
                .await
                .unwrap()
                .is_connected()
        );

        // Ended allows re-inviting and re-connecting with advancing connected_at.
        let reinvite = store
            .invite(&practice_a, "luna@example.test", "Luna Park", NOW + 20)
            .await
            .unwrap();
        assert!(matches!(
            store.roster_connection(&practice_a).await.unwrap(),
            RosterConnectionStatus::Invited { .. }
        ));
        let (rejoined_id, _) = store
            .redeem_invitation("luna@example.test", &reinvite, NOW + 30)
            .await
            .unwrap();
        assert_eq!(rejoined_id, id);
        assert!(matches!(
            store.roster_connection(&practice_a).await.unwrap(),
            RosterConnectionStatus::Connected {
                account_id,
                connected_at,
            } if account_id == id && connected_at == NOW + 30
        ));
    }

    #[tokio::test]
    async fn a_sign_in_token_verifies_once_and_mints_non_expiring_session() {
        let store = MemoryFrontDoorStore::new();
        let (_, id, _) = invited(&store, "luna@example.test").await;
        let token = store.begin_sign_in("luna@example.test", NOW).await.unwrap();
        let (who, session) = store
            .complete_sign_in("luna@example.test", &token, NOW)
            .await
            .unwrap();
        assert_eq!(who, id);
        assert_eq!(
            store.session(&session, NOW + 365 * 86_400).await.unwrap(),
            id
        );
        assert_eq!(
            store
                .complete_sign_in("luna@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid,
            "a verified token never verifies again"
        );
    }

    #[tokio::test]
    async fn an_expired_sign_in_token_stops_verifying() {
        let store = MemoryFrontDoorStore::new();
        let (_, _, _) = invited(&store, "luna@example.test").await;
        let token = store.begin_sign_in("luna@example.test", NOW).await.unwrap();
        assert_eq!(
            store
                .complete_sign_in("luna@example.test", &token, NOW + SIGN_IN_TOKEN_TTL_SECONDS)
                .await
                .unwrap_err(),
            SignInError::Invalid,
            "a sign-in token is verifiable for one day"
        );
        assert_eq!(
            store
                .complete_sign_in("luna@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
    }

    #[tokio::test]
    async fn tokens_are_stored_hashed() {
        let hash = token_hash("codaf-abc");
        assert_ne!(hash.as_slice(), b"codaf-abc");
        assert_eq!(
            hash,
            token_hash("codaf-abc"),
            "verification is deterministic"
        );
    }

    #[tokio::test]
    async fn connections_list_per_account_and_practice() {
        let store = MemoryFrontDoorStore::new();
        let (luna, id, _) = invited(&store, "luna@example.test").await;
        assert_eq!(store.connections(id).await.unwrap().len(), 1);
        let other = store.seed_linked_account(
            &patient("roger-synthetic-practice", "patient-2"),
            "sofia@example.test",
            "Sofia Reyes",
        );
        let mut connected = store
            .connected_accounts("roger-synthetic-practice")
            .await
            .into_iter()
            .map(|(account, _)| account.id)
            .collect::<Vec<_>>();
        connected.sort();
        let mut expected = vec![id, other];
        expected.sort();
        assert_eq!(connected, expected);
        assert!(store.connected_accounts("other-practice").await.is_empty());
        assert!(matches!(
            store.connections(Uuid::nil()).await,
            Err(LinkError::Unlinked)
        ));
        assert!(store.roster_connection(&luna).await.unwrap().is_connected());
    }

    #[tokio::test]
    async fn leave_discharge_and_account_delete_preserve_ended_status_and_allow_reinvite() {
        let store = MemoryFrontDoorStore::new();
        let p_a = patient("practice-a", "pat-1");
        let p_b = patient("practice-b", "pat-2");
        let p_c = patient("practice-c", "pat-3");

        let tok_a = store
            .invite(&p_a, "luna@example.test", "Luna", NOW)
            .await
            .unwrap();
        let (id, sess) = store
            .redeem_invitation("luna@example.test", &tok_a, NOW)
            .await
            .unwrap();
        let tok_b = store
            .invite(&p_b, "luna@example.test", "Luna", NOW)
            .await
            .unwrap();
        store
            .redeem_invitation("luna@example.test", &tok_b, NOW + 1)
            .await
            .unwrap();
        let tok_c = store
            .invite(&p_c, "luna@example.test", "Luna", NOW)
            .await
            .unwrap();
        store
            .redeem_invitation("luna@example.test", &tok_c, NOW + 2)
            .await
            .unwrap();

        // Patient leaves practice-a: practice-b and practice-c remain connected.
        let left = store
            .leave_connection(id, "practice-a", NOW + 10)
            .await
            .unwrap();
        assert_eq!(left.len(), 1);
        assert!(matches!(
            left[0].standing,
            ConnectionStanding::Ended {
                ended_at,
                reason: ConnectionEndReason::PatientLeft,
                ..
            } if ended_at == NOW + 10
        ));
        assert!(matches!(
            store.roster_connection(&p_a).await.unwrap(),
            RosterConnectionStatus::Ended {
                reason: ConnectionEndReason::PatientLeft,
                ..
            }
        ));
        assert!(store.roster_connection(&p_b).await.unwrap().is_connected());

        // Leaving an already-ended practice fails cleanly with Unlinked.
        assert!(matches!(
            store.leave_connection(id, "practice-a", NOW + 11).await,
            Err(LinkError::Unlinked)
        ));

        // Practice discharges practice-b by roster key: immediately Ended with PracticeDischarged.
        let discharged = store.discharge_connection(&p_b, NOW + 20).await.unwrap();
        assert!(matches!(
            discharged.standing,
            ConnectionStanding::Ended {
                ended_at,
                reason: ConnectionEndReason::PracticeDischarged,
                ..
            } if ended_at == NOW + 20
        ));
        assert!(matches!(
            store.roster_connection(&p_b).await.unwrap(),
            RosterConnectionStatus::Ended {
                reason: ConnectionEndReason::PracticeDischarged,
                ..
            }
        ));
        assert!(matches!(
            store.discharge_connection(&p_b, NOW + 21).await,
            Err(LinkError::Unlinked)
        ));

        // Both Left and Discharged can be re-invited right away.
        assert!(
            store
                .invite(&p_a, "luna@example.test", "Luna", NOW + 25)
                .await
                .is_ok()
        );
        assert!(
            store
                .invite(&p_b, "luna@example.test", "Luna", NOW + 26)
                .await
                .is_ok()
        );

        // Deleting account ends all remaining active connections (practice-c), revokes sessions,
        // keeps Ended standing on roster, and is idempotent on replay.
        assert_eq!(
            store
                .end_all_connections_for_account(id, NOW + 30)
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            store
                .end_all_connections_for_account(id, NOW + 31)
                .await
                .unwrap(),
            0
        );
        assert!(matches!(
            store.roster_connection(&p_c).await.unwrap(),
            RosterConnectionStatus::Ended {
                ended_at,
                reason: ConnectionEndReason::AccountDeleted,
                ..
            } if ended_at == NOW + 30
        ));
        assert_eq!(
            store.session(&sess, NOW + 32).await.unwrap_err(),
            SessionError::Invalid
        );
    }

    #[tokio::test]
    async fn invitation_short_code_resend_preserves_prior_credentials_and_cancel_invalidates_all() {
        let store = MemoryFrontDoorStore::new();
        let rp = patient("practice-one", "pat-1");
        let issued1 = store
            .invite_patient(InvitePatientParams {
                patient: rp.clone(),
                email: "pat1@example.test".to_owned(),
                display_name: "Pat One".to_owned(),
                practice_name: "River Bend Clinic".to_owned(),
                physician_name: "Dr. Roger".to_owned(),
                expected_birth_date: Some("1987-02-18".to_owned()),
                now: NOW,
            })
            .await
            .unwrap();
        assert_eq!(issued1.short_code.len(), 9);
        assert_eq!(issued1.short_code.as_bytes()[4], b'-');

        let issued2 = store.resend_invitation(&rp, NOW + 10).await.unwrap();
        assert_ne!(issued1.link_token, issued2.link_token);
        assert_ne!(issued1.short_code, issued2.short_code);

        // Both original and resent links and normalized short codes resolve Active with no preview leak.
        let lowered_no_dash = issued1.short_code.replace('-', "").to_ascii_lowercase();
        assert_eq!(
            store
                .inspect_invitation(&InvitationCredential::ShortCode(lowered_no_dash))
                .await
                .unwrap()
                .standing,
            InvitationOpenStanding::Active {
                wrong_dob_attempts: 0,
                attempts_remaining: MAX_DOB_ATTEMPTS,
            }
        );
        assert_eq!(
            store
                .inspect_invitation(&InvitationCredential::LinkToken(issued1.link_token.clone()))
                .await
                .unwrap()
                .standing,
            InvitationOpenStanding::Active {
                wrong_dob_attempts: 0,
                attempts_remaining: MAX_DOB_ATTEMPTS,
            }
        );
        assert_eq!(
            store
                .inspect_invitation(&InvitationCredential::ShortCode(issued2.short_code.clone()))
                .await
                .unwrap()
                .standing,
            InvitationOpenStanding::Active {
                wrong_dob_attempts: 0,
                attempts_remaining: MAX_DOB_ATTEMPTS,
            }
        );

        // Cancel invalidates ALL credentials (Rule 5).
        store.cancel_invitation(&rp).await.unwrap();
        assert_eq!(
            store.cancel_invitation(&rp).await.unwrap_err(),
            InviteError::NotFound
        );
        assert!(matches!(
            store.roster_connection(&rp).await.unwrap(),
            RosterConnectionStatus::NotInvited
        ));
        assert_eq!(
            store
                .inspect_invitation(&InvitationCredential::LinkToken(issued1.link_token))
                .await
                .unwrap_err(),
            InvitationCheckError::NotOpen
        );
        assert_eq!(
            store
                .inspect_invitation(&InvitationCredential::ShortCode(issued2.short_code))
                .await
                .unwrap_err(),
            InvitationCheckError::NotOpen
        );
    }

    #[tokio::test]
    async fn invitation_dob_gate_withholds_preview_locks_after_five_misses_and_blocks_underage() {
        let store = MemoryFrontDoorStore::new();
        let rp = patient("practice-dob", "pat-dob");
        let issued = store
            .invite_patient(InvitePatientParams {
                patient: rp.clone(),
                email: "dob@example.test".to_owned(),
                display_name: "DOB Patient".to_owned(),
                practice_name: "River Bend Clinic".to_owned(),
                physician_name: "Dr. Roger".to_owned(),
                expected_birth_date: Some("1987-02-18".to_owned()),
                now: NOW,
            })
            .await
            .unwrap();

        let cred = InvitationCredential::ShortCode(issued.short_code.clone());
        assert_eq!(
            store.inspect_invitation(&cred).await.unwrap().standing,
            InvitationOpenStanding::Active {
                wrong_dob_attempts: 0,
                attempts_remaining: MAX_DOB_ATTEMPTS,
            }
        );

        // Wrong birth dates 1..=4 return attempts remaining and never lock yet.
        for wrong_idx in 1..=4 {
            let err = store
                .verify_invitation_dob(&cred, "1987-02-19", NOW + wrong_idx)
                .await
                .unwrap_err();
            assert_eq!(
                err,
                InvitationCheckError::WrongBirthDate {
                    attempts_remaining: (5 - wrong_idx) as u8
                }
            );
        }

        // 5th wrong DOB locks the open invitation (Rule 7) and refuses Resend (COD-49).
        assert_eq!(
            store
                .verify_invitation_dob(&cred, "1987-02-19", NOW + 5)
                .await
                .unwrap_err(),
            InvitationCheckError::Locked
        );
        assert_eq!(
            store.inspect_invitation(&cred).await.unwrap(),
            InvitationGateState {
                standing: InvitationOpenStanding::Locked { locked_at: NOW + 5 },
            }
        );
        assert_eq!(
            store.resend_invitation(&rp, NOW + 6).await.unwrap_err(),
            InviteError::Locked
        );
        // Even the right DOB is refused once locked.
        assert_eq!(
            store
                .verify_invitation_dob(&cred, "1987-02-18", NOW + 7)
                .await
                .unwrap_err(),
            InvitationCheckError::Locked
        );
        assert!(matches!(
            store.roster_connection(&rp).await.unwrap(),
            RosterConnectionStatus::Invited {
                wrong_dob_attempts: 5,
                locked_at: Some(_),
                ..
            }
        ));

        // Cancel clears lock and allows fresh invitation, which can then be unlocked with chart DOB.
        store.cancel_invitation(&rp).await.unwrap();
        let reopened = store
            .invite_patient(InvitePatientParams {
                patient: rp.clone(),
                email: "dob@example.test".to_owned(),
                display_name: "DOB Patient".to_owned(),
                practice_name: "River Bend Clinic".to_owned(),
                physician_name: "Dr. Roger".to_owned(),
                expected_birth_date: Some("1987-02-18".to_owned()),
                now: NOW + 9,
            })
            .await
            .unwrap();
        let preview = store
            .verify_invitation_dob(
                &InvitationCredential::LinkToken(reopened.link_token.clone()),
                "1987-02-18",
                NOW + 10,
            )
            .await
            .unwrap();
        assert_eq!(preview.practice_name, "River Bend Clinic");
        assert_eq!(preview.physician_name, "Dr. Roger");
        assert!(!preview.shares_summary.is_empty());
        // Checking DOB alone never connects the patient (Rule 6).
        assert!(matches!(
            store.roster_connection(&rp).await.unwrap(),
            RosterConnectionStatus::Invited { .. }
        ));

        // 18+ rule blocks underage invitee (NOW ~ Oct 2025 -> born 2010-01-01 is 15).
        let underage_rp = patient("practice-dob", "pat-young");
        let young = store
            .invite_patient(InvitePatientParams {
                patient: underage_rp,
                email: "young@example.test".to_owned(),
                display_name: "Young Patient".to_owned(),
                practice_name: "River Bend Clinic".to_owned(),
                physician_name: "Dr. Roger".to_owned(),
                expected_birth_date: Some("2010-01-01".to_owned()),
                now: NOW,
            })
            .await
            .unwrap();
        assert_eq!(
            store
                .verify_invitation_dob(
                    &InvitationCredential::LinkToken(young.link_token),
                    "2010-01-01",
                    NOW + 1,
                )
                .await
                .unwrap_err(),
            InvitationCheckError::Underage
        );
    }

    #[tokio::test]
    async fn invitation_share_connects_signed_in_account_and_delivery_failure_clears_on_resend() {
        let store = MemoryFrontDoorStore::new();
        let other = patient("practice-a", "pat-other");
        let tok_other = store
            .invite(&other, "signedin@example.test", "Signed In", NOW)
            .await
            .unwrap();
        let (signed_in_id, _) = store
            .redeem_invitation("signedin@example.test", &tok_other, NOW + 1)
            .await
            .unwrap();

        let rp = patient("practice-b", "pat-b");
        let _issued = store
            .invite_patient(InvitePatientParams {
                patient: rp.clone(),
                email: "chart-different@example.test".to_owned(),
                display_name: "Chart Email".to_owned(),
                practice_name: "Oak Clinic".to_owned(),
                physician_name: "Dr. Chen".to_owned(),
                expected_birth_date: Some("1987-02-18".to_owned()),
                now: NOW + 2,
            })
            .await
            .unwrap();

        // Mark mail delivery failed: status stays Invited with delivery error recorded (Rule 8).
        store
            .mark_invitation_delivery_failed(&rp, "smtp connection refused", NOW + 3)
            .await
            .unwrap();
        assert!(matches!(
            store.roster_connection(&rp).await.unwrap(),
            RosterConnectionStatus::Invited {
                delivery_failed_at: Some(t),
                delivery_error: Some(ref err),
                ..
            } if t == NOW + 3 && err == "smtp connection refused"
        ));

        // Resend clears delivery failure notice.
        let resent = store.resend_invitation(&rp, NOW + 4).await.unwrap();
        assert!(resent.invitation.last_delivery_failed_at.is_none());

        // Share connects the existing signed-in Duet account regardless of differing email (Rule 9).
        let (acc_id, conn) = store
            .accept_invitation_share(
                Some(signed_in_id),
                Some("signedin@example.test"),
                &InvitationCredential::ShortCode(resent.short_code),
                "1987-02-18",
                NOW + 5,
            )
            .await
            .unwrap();
        assert_eq!(acc_id, signed_in_id);
        assert_eq!(conn.patient, rp);
        assert_eq!(store.connections(signed_in_id).await.unwrap().len(), 2);
    }
}
