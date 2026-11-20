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

pub use error::{ForMyDoctorError, InviteError, LinkError, SessionError, SignInError};
pub use store::PostgresFrontDoorStore;

/// Durable Identity Platform mapping and Duet permission authority.
pub mod identity;
/// Transactional Luna privacy restrictions and independent receipt reconciliation.
pub mod journal;
/// The SeaORM entities and the migration chain.
pub mod migration;

/// A sign-in verification token is verifiable for one day.
pub const SIGN_IN_TOKEN_TTL_SECONDS: u64 = 86_400;

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
    /// The Unix second the invitation was issued.
    pub invited_at: u64,
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
    /// Reads the canonical connection status (`NotInvited` / `Invited` / `Connected` / `Ended`)
    /// for one roster patient.
    async fn roster_connection(&self, patient: &RosterPatient) -> RosterConnectionStatus;
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

/// Mints one presented token. The store keeps only its hash.
pub(crate) fn mint_token() -> String {
    format!("codaf-{}", Uuid::new_v4().simple())
}

#[derive(Debug, Default)]
struct MemoryState {
    accounts: HashMap<Uuid, StoredAccount>,
    emails: HashMap<String, Uuid>,
    invitations: HashMap<(String, String), InvitationPending>,
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
    patient: RosterPatient,
    email: String,
    display_name: String,
    token_hash: [u8; 32],
    invited_at: u64,
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
            .map(|p| Invitation {
                patient: p.patient.clone(),
                email: p.email.clone(),
                display_name: p.display_name.clone(),
                invited_at: p.invited_at,
            });
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
        let key = email.to_lowercase();
        let mut state = self.state.lock().expect("front-door state");
        let already_connected = state.connections.iter().any(|((_, prac, pat), standing)| {
            prac == &patient.practice_id
                && pat == &patient.patient_id
                && matches!(standing, ConnectionStanding::Connected { .. })
        });
        if already_connected {
            return Err(InviteError::Conflict);
        }
        let token = mint_token();
        state.invitations.insert(
            (patient.practice_id.clone(), patient.patient_id.clone()),
            InvitationPending {
                patient: patient.clone(),
                email: key,
                display_name: display_name.to_owned(),
                token_hash: token_hash(&token),
                invited_at: now,
            },
        );
        Ok(token)
    }

    async fn invitation(&self, email: &str) -> Option<Invitation> {
        let key = email.to_lowercase();
        let state = self.state.lock().expect("front-door state");
        state
            .invitations
            .values()
            .filter(|pending| pending.email == key)
            .max_by_key(|pending| pending.invited_at)
            .map(|pending| Invitation {
                patient: pending.patient.clone(),
                email: pending.email.clone(),
                display_name: pending.display_name.clone(),
                invited_at: pending.invited_at,
            })
    }

    async fn invitation_for(&self, patient: &RosterPatient) -> Option<Invitation> {
        let state = self.state.lock().expect("front-door state");
        state
            .invitations
            .get(&(patient.practice_id.clone(), patient.patient_id.clone()))
            .map(|pending| Invitation {
                patient: pending.patient.clone(),
                email: pending.email.clone(),
                display_name: pending.display_name.clone(),
                invited_at: pending.invited_at,
            })
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
        let Some(roster_key) = state
            .invitations
            .iter()
            .find(|(_, pending)| pending.email == key && pending.token_hash == presented_hash)
            .map(|(k, _)| k.clone())
        else {
            return Err(SignInError::Invalid);
        };

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

    async fn roster_connection(&self, patient: &RosterPatient) -> RosterConnectionStatus {
        let state = self.state.lock().expect("front-door state");
        Self::roster_status_in(&state, patient)
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
    async fn reinviting_same_roster_patient_replaces_the_open_invitation() {
        let store = MemoryFrontDoorStore::new();
        let patient = patient("practice", "patient-9");
        store
            .invite(&patient, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        let token = store
            .invite(&patient, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        assert!(
            store
                .redeem_invitation("luna@example.test", &token, NOW)
                .await
                .is_ok(),
            "the newest invitation redeems"
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
            store.roster_connection(&practice_a).await,
            RosterConnectionStatus::Ended {
                account_id,
                ended_at,
                reason: ConnectionEndReason::PatientLeft,
                ..
            } if account_id == id && ended_at == NOW + 10
        ));
        assert!(store.roster_connection(&practice_b).await.is_connected());

        // Ended allows re-inviting and re-connecting with advancing connected_at.
        let reinvite = store
            .invite(&practice_a, "luna@example.test", "Luna Park", NOW + 20)
            .await
            .unwrap();
        assert!(matches!(
            store.roster_connection(&practice_a).await,
            RosterConnectionStatus::Invited { .. }
        ));
        let (rejoined_id, _) = store
            .redeem_invitation("luna@example.test", &reinvite, NOW + 30)
            .await
            .unwrap();
        assert_eq!(rejoined_id, id);
        assert!(matches!(
            store.roster_connection(&practice_a).await,
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
        assert_eq!(
            store.connections(Uuid::nil()).await.unwrap_err(),
            LinkError::Unlinked,
            "an unknown account carries no connections"
        );
        assert!(store.roster_connection(&luna).await.is_connected());
    }
}
