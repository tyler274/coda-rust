//! Physician identity, membership, sessions, and account lifecycle.
//!
//! One durable owner for who a physician is, which practice they belong to,
//! which patients they may read, and which browser sessions act for them.
//! Sign-in is an email magic token, modeled on the Supabase GoTrue flows:
//! the server issues a one-time token, its verification mints one bearer
//! session, and tokens are stored hashed so the presented token is the only
//! secret. Practice membership is the access check every physician read
//! resolves through, so deactivating a physician ends every session they hold.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

mod cache;
mod error;
mod lifetime;
mod store;

pub use lifetime::{SessionActivity, SessionLifetime, SessionRenewal};

pub use cache::{AUTHORITY_CHANNEL, CachedAuthStore, ENTRY_TTL, follow};
pub use error::{
    AccountSetupError, InvalidTimeZone, PanelError, PracticeSetupError, SessionError, SignInError,
    StoreUnavailable,
};
pub use store::PostgresAuthStore;
pub use tokio::sync::watch;

/// The SeaORM entities and the migration chain.
pub mod migration;

/// A sign-in token is verifiable for one day.
pub const SIGN_IN_TOKEN_TTL_SECONDS: u64 = 86_400;
/// An invitation is redeemable for fourteen days.
pub const INVITATION_TTL_SECONDS: u64 = 14 * 86_400;

/// The front-door link between a practice's roster patient `(practice_id, patient_workspace_id)`
/// and the patient's Duet account, including versioned sharing consent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientLink {
    /// The practice that owns the roster patient.
    pub practice_id: String,
    /// The roster patient's workspace id.
    pub patient_workspace_id: String,
    /// The linked Duet account id, once the patient accepts the practice invitation.
    pub duet_account_id: Option<String>,
    /// Whether the patient has accepted the explicit practice invitation.
    pub linked: bool,
    /// Whether "share with my physician" consent is currently active.
    pub sharing_consented: bool,
    /// The version of the consent agreement accepted by the patient (e.g. `"v1"`).
    pub consent_version: Option<String>,
    /// The Unix second consent was granted.
    pub consented_at: Option<u64>,
    /// The Unix second consent was revoked, if revoked.
    pub revoked_at: Option<u64>,
}

/// A physician's practice-scoped identity, as the projections key it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicianIdentity {
    /// The practice this physician belongs to.
    pub practice_id: String,
    /// The physician within the practice.
    pub physician_id: String,
}

/// The clock a practice is bound to when it is created. It sets the practice's
/// clinical "now"; sign-in, sessions, and other operational time stay on real
/// time in every practice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeClock {
    /// A real practice: clinical "now" is the real time.
    RealTime,
    /// A synthetic practice: clinical "now" stays at this Unix second.
    Fixed(u64),
}

impl PracticeClock {
    /// The practice's clinical "now", given the real time.
    pub fn now(self, real_time: u64) -> u64 {
        match self {
            Self::RealTime => real_time,
            Self::Fixed(instant) => instant,
        }
    }
}

/// The validated IANA time zone a practice operates in (for example,
/// `"America/New_York"` or `"America/Los_Angeles"`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PracticeTimeZone(String);

impl PracticeTimeZone {
    /// The default practice time zone (`"America/New_York"`).
    pub const DEFAULT_IANA: &'static str = "America/New_York";

    /// Validates a candidate IANA time zone identifier against the bundled tzdb.
    pub fn parse(raw: &str) -> Result<Self, InvalidTimeZone> {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('+') || trimmed.starts_with('-') {
            let source = jiff::tz::TimeZone::get("").expect_err("empty time zone must be rejected");
            return Err(InvalidTimeZone {
                zone: raw.to_owned(),
                source,
            });
        }
        let tz = jiff::tz::TimeZone::get(trimmed).map_err(|source| InvalidTimeZone {
            zone: raw.to_owned(),
            source,
        })?;
        let canonical = tz
            .iana_name()
            .filter(|name| !name.is_empty())
            .unwrap_or(trimmed)
            .to_owned();
        Ok(Self(canonical))
    }

    /// Returns the validated IANA time zone name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Formats the local calendar date (`YYYY-MM-DD`) at `unix_seconds` in this practice's IANA time zone.
    pub fn calendar_date_iso(&self, unix_seconds: u64) -> String {
        let tz = jiff::tz::TimeZone::get(&self.0)
            .unwrap_or_else(|_| jiff::tz::TimeZone::get(Self::DEFAULT_IANA).expect("default tz"));
        let ts = jiff::Timestamp::from_second(unix_seconds as i64)
            .unwrap_or(jiff::Timestamp::UNIX_EPOCH);
        let zoned = ts.to_zoned(tz);
        format!(
            "{:04}-{:02}-{:02}",
            zoned.year(),
            zoned.month() as u8,
            zoned.day()
        )
    }
}

impl Default for PracticeTimeZone {
    fn default() -> Self {
        Self(Self::DEFAULT_IANA.to_owned())
    }
}

impl std::fmt::Display for PracticeTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<PracticeTimeZone> for String {
    fn from(zone: PracticeTimeZone) -> Self {
        zone.0
    }
}

impl TryFrom<String> for PracticeTimeZone {
    type Error = InvalidTimeZone;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

/// One practice record: its identifier, clinical clock, and validated IANA time zone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeRecord {
    /// The practice identifier.
    pub practice_id: String,
    /// The clinical clock the practice operates on.
    pub clock: PracticeClock,
    /// The validated IANA time zone the practice operates in.
    pub time_zone: PracticeTimeZone,
}

/// Whether [`AuthStore::upsert_practice`] created a new practice or verified an unchanged one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeSetupOutcome {
    /// A new practice row was inserted.
    Created,
    /// The practice already existed with identical `(clock, time_zone)`.
    Unchanged,
}

/// Whether [`AuthStore::create_account`] inserted a new physician account or verified an existing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCreateOutcome {
    /// A new active physician account was inserted.
    Created,
    /// An identical account in the same practice already existed.
    Existing,
}

/// Advances a newly minted session's `created_at` past the account's revocation fence
/// (`sessions_ended_before`) so a post-reactivation sign-in within the same Unix second
/// resolves while pre-deactivation sessions stay permanently ended.
pub(crate) fn post_fence_mint_second(now: u64, sessions_ended_before: u64) -> u64 {
    now.max(sessions_ended_before)
}

pub(crate) fn validate_practice_id(raw: &str) -> Result<String, PracticeSetupError> {
    let trimmed = raw.trim();
    let valid = !trimmed.is_empty()
        && trimmed.len() <= 96
        && trimmed
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
        && !trimmed.contains("..");
    if valid {
        Ok(trimmed.to_owned())
    } else {
        Err(PracticeSetupError::InvalidPracticeId(raw.to_owned()))
    }
}

pub(crate) fn validate_account_fields(
    practice_id: &str,
    physician_id: Option<&str>,
    email: &str,
    display_name: &str,
) -> Result<(String, String, String, String), AccountSetupError> {
    let practice = validate_practice_id(practice_id).map_err(|_| {
        AccountSetupError::InvalidInput(format!("invalid practice_id `{practice_id}`"))
    })?;
    let resolved_physician_id = match physician_id.map(str::trim).filter(|id| !id.is_empty()) {
        Some(id) => {
            let valid = id.len() <= 96
                && id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
                && !id.contains("..");
            if !valid {
                return Err(AccountSetupError::InvalidInput(format!(
                    "invalid physician_id `{id}`"
                )));
            }
            id.to_owned()
        }
        None => Uuid::new_v4().simple().to_string(),
    };
    let clean_email = email.trim().to_lowercase();
    let email_ok = clean_email
        .split_once('@')
        .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'))
        && !clean_email.chars().any(char::is_whitespace);
    if !email_ok {
        return Err(AccountSetupError::InvalidInput(format!(
            "invalid email `{email}`"
        )));
    }
    let clean_name = display_name.trim().to_owned();
    if clean_name.is_empty() || clean_name.len() > 160 {
        return Err(AccountSetupError::InvalidInput(
            "display_name must be 1..=160 characters".to_owned(),
        ));
    }
    Ok((practice, resolved_physician_id, clean_email, clean_name))
}

pub(crate) fn check_existing_account_match(
    existing: &Account,
    practice_id: &str,
    caller_named_physician: bool,
    resolved_physician_id: &str,
    clean_name: &str,
    admin: bool,
) -> Result<(), AccountSetupError> {
    if existing.identity.practice_id != practice_id {
        return Err(AccountSetupError::EmailConflict {
            email: existing.email.clone(),
            existing_practice_id: existing.identity.practice_id.clone(),
        });
    }
    if caller_named_physician && existing.identity.physician_id != resolved_physician_id {
        return Err(AccountSetupError::PhysicianIdConflict {
            practice_id: practice_id.to_owned(),
            physician_id: resolved_physician_id.to_owned(),
        });
    }
    if !existing.active || existing.admin != admin || existing.display_name != clean_name {
        return Err(AccountSetupError::AccountConflict {
            practice_id: practice_id.to_owned(),
            email: existing.email.clone(),
        });
    }
    Ok(())
}

/// A physician's access to one patient workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Membership {
    /// The physician may read this patient and act for them.
    Allowed,
    /// The patient belongs to the physician's clinic, but is not on their
    /// panel: only basic demographic information may be read.
    ClinicDemographic,
    /// The workspace exists, but this physician may not read it.
    Denied,
}

/// One physician account in one practice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    /// The account's identity.
    pub identity: PhysicianIdentity,
    /// The email sign-in address.
    pub email: String,
    /// The account's display name.
    pub display_name: String,
    /// Whether the account may sign in and hold sessions.
    pub active: bool,
    /// Whether the account may administer the practice's accounts and assign
    /// patients to a physician's panel (held by clinic admins, secretaries,
    /// or physicians granted panel-assignment permission).
    pub admin: bool,
    /// Sessions minted before this Unix second never resolve; deactivation
    /// moves it forward and reactivation leaves it there.
    pub sessions_ended_before: u64,
}

/// The authority change feed: sessions, accounts, panels, or patient
/// ownership changed somewhere.
///
/// `generation` moves on every change. `live` says whether changes are being
/// observed at all; while it is false a reader must not trust anything it
/// learned earlier and should re-read the store.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AuthorityFeed {
    /// Moves forward on every observed change and on every reconnect.
    pub generation: u64,
    /// Whether the feed is currently following the store's changes.
    pub live: bool,
}

/// One browser session a sign-in minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    /// The session's physician identity.
    pub identity: PhysicianIdentity,
    /// The presented bearer token, `sha256`-hashed at rest.
    pub token_hash: [u8; 32],
    /// The idle deadline, clamped to seven days after [`Self::created_at`].
    pub lifetime: SessionLifetime,
    /// The Unix second the session was minted, against the account's fence.
    pub created_at: u64,
}

/// The durable physician-auth owner.
///
/// `&self` everywhere so the server can share one store across handlers.
/// `async_trait` keeps the trait object-safe: the server holds
/// `Arc<dyn AuthStore>` and picks the SQL or memory store at startup.
#[async_trait]
pub trait AuthStore: Send + Sync {
    /// Follows authority changes, when this store can observe them. Holders of
    /// long-lived grants (event streams, running work) re-check on each change
    /// instead of polling. `None` means the caller must poll.
    fn authority_changes(&self) -> Option<watch::Receiver<AuthorityFeed>> {
        None
    }
    /// Issues a sign-in token for an existing active account's email.
    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError>;
    /// Verifies a sign-in token and mints one bearer session from it.
    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError>;
    /// Mints a session that ends at `ends_at` however much it is used, for
    /// the active account with exactly this identity. No email or token is
    /// involved: the caller has already proved who may act as `identity`.
    async fn mint_fixed_session(
        &self,
        identity: &PhysicianIdentity,
        now: u64,
        ends_at: u64,
    ) -> Result<String, SignInError>;
    /// Resolves a bearer token to its physician's identity.
    async fn identity(&self, token: &str, now: u64) -> Result<PhysicianIdentity, SessionError> {
        Ok(self.verified_session(token, now).await?.identity)
    }
    /// Resolves an active session with its persisted expiry.
    async fn verified_session(&self, token: &str, now: u64) -> Result<Session, SessionError>;
    /// Slides the idle deadline for one physician request.
    ///
    /// This is the only write of that deadline. A session that no longer
    /// allows `now` is left unchanged.
    async fn note_physician_activity(
        &self,
        token: &str,
        now: u64,
    ) -> Result<SessionActivity, StoreUnavailable>;
    /// Executes synchronous publication while holding the actual session,
    /// account, panel and patient authority fence. Implementations must exclude
    /// direct mutations too, not only HTTP handlers. The callback must not call
    /// this store recursively. Default denies unsupported store adapters.
    async fn publish_authorized(
        &self,
        _token: &str,
        _identity: &PhysicianIdentity,
        _workspace: &str,
        _now: u64,
        _publish: &mut (dyn FnMut() -> Result<(), String> + Send),
    ) -> Result<(), String> {
        Err("authority store does not support fenced publication".into())
    }
    /// Ends one session so its token stops resolving.
    async fn end_session(&self, token: &str) -> Result<(), StoreUnavailable>;
    /// Reads one account by email.
    async fn account_by_email(&self, email: &str) -> Option<Account>;
    /// Reads one account by its identity.
    async fn account_by_identity(&self, identity: &PhysicianIdentity) -> Option<Account>;
    /// Reads whether the physician may read one patient workspace.
    async fn membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Membership;
    /// Sets one patient workspace's membership for one physician.
    async fn set_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), StoreUnavailable>;
    /// Puts one patient workspace on one physician's panel unless a panel
    /// choice already exists for the pair, so a removal stays removed.
    async fn place_on_panel(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable>;
    /// Lists the patient workspaces on one physician's panel, sorted.
    async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String>;
    /// Assigns or removes a patient workspace on `physician_id`'s panel in
    /// `actor`'s practice. Requires `actor` to be an active `admin` account
    /// (clinic admin, secretary, or physician holding panel-assignment
    /// permission) in the same practice as `physician_id`.
    async fn assign_panel(
        &self,
        actor: &PhysicianIdentity,
        physician_id: &str,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), PanelError>;
    /// Creates `practice_id` on `clock`. A practice that already exists keeps
    /// the clock it was created with.
    async fn ensure_practice(
        &self,
        practice_id: &str,
        clock: PracticeClock,
    ) -> Result<(), StoreUnavailable>;
    /// Creates `record` with its validated clock and IANA time zone, or verifies
    /// that an existing practice matches `(clock, time_zone)`.
    async fn upsert_practice(
        &self,
        record: &PracticeRecord,
    ) -> Result<PracticeSetupOutcome, PracticeSetupError>;
    /// Reads one practice record if `practice_id` has been created.
    async fn practice(&self, practice_id: &str)
    -> Result<Option<PracticeRecord>, StoreUnavailable>;
    /// Reads the clock `practice_id` was created with. A practice with no
    /// record was never made synthetic, so it runs on real time.
    async fn practice_clock(&self, practice_id: &str) -> Result<PracticeClock, StoreUnavailable>;
    /// Registers a patient workspace as belonging to `practice_id`. A workspace
    /// that is already registered keeps its practice.
    async fn register_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable>;
    /// Returns the practice that owns `patient_workspace_id`, or `None` when it
    /// is not registered.
    async fn patient_practice(
        &self,
        patient_workspace_id: &str,
    ) -> Result<Option<String>, StoreUnavailable>;
    /// Lists the patient workspaces registered in `practice_id`, sorted.
    async fn practice_patients(&self, practice_id: &str) -> Result<Vec<String>, StoreUnavailable>;
    /// Lists the physician accounts in `practice_id`, sorted.
    async fn practice_accounts(&self, practice_id: &str) -> Result<Vec<Account>, StoreUnavailable>;
    /// Creates an active physician account in `practice_id`, or returns an
    /// identical existing account in `practice_id`.
    async fn create_account(
        &self,
        practice_id: &str,
        physician_id: Option<&str>,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<(Account, AccountCreateOutcome), AccountSetupError>;
    /// Records one pending invitation and returns its redeem token.
    async fn invite(
        &self,
        practice_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<String, SignInError>;
    /// Redeems an invitation, creating the account and minting one session.
    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError>;
    /// Deactivates one account: sign-in and every held session stop.
    async fn deactivate(&self, email: &str) -> bool;
    /// Reactivates one deactivated account.
    async fn reactivate(&self, email: &str) -> bool;
    /// Removes one account and ends its sessions.
    async fn remove_account(&self, email: &str) -> bool;
    /// Explicitly invites a roster patient `(practice_id, patient_workspace_id)` to link Duet.
    ///
    /// Never guesses identity; stores only the SHA-256 hash of the single-use invitation token.
    async fn invite_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<String, SignInError>;
    /// Redeems a single-use patient invitation token and links `(practice_id, patient_workspace_id)`
    /// to `duet_account_id`.
    async fn accept_patient_invitation(
        &self,
        token: &str,
        duet_account_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError>;
    /// Records versioned `"share with my physician"` consent for a linked roster patient.
    async fn grant_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        duet_account_id: &str,
        consent_version: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError>;
    /// Revokes `"share with my physician"` consent in one step (`stop sharing`).
    async fn revoke_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError>;
    /// Reads the `PatientLink` for `(practice_id, patient_workspace_id)`, if invited or configured.
    async fn patient_link(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Option<PatientLink>;
}

/// The SHA-256 of a presented token: the only form the store keeps.
pub(crate) fn token_hash(token: &str) -> [u8; 32] {
    Sha256::digest(token.as_bytes()).into()
}

/// Mints one presented token. The store keeps only its hash.
pub(crate) fn mint_token() -> String {
    format!("codas-{}", Uuid::new_v4().simple())
}

/// Mints one presented patient invitation token. The store keeps only its hash.
pub(crate) fn mint_patient_token() -> String {
    format!("codap-{}", Uuid::new_v4().simple())
}

#[derive(Debug, Default)]
struct MemoryState {
    accounts: HashMap<String, Account>,
    memberships: HashMap<(String, String, String), Membership>,
    practices: HashMap<String, (PracticeClock, PracticeTimeZone)>,
    patients: HashMap<String, String>,
    sessions: HashMap<[u8; 32], Session>,
    sign_ins: HashMap<String, (SignInPending, ())>,
    invitations: HashMap<String, InvitationPending>,
    patient_links: HashMap<(String, String), PatientLinkRecord>,
}

#[derive(Debug, Clone)]
struct PatientLinkRecord {
    link: PatientLink,
    pending_token_hash: Option<[u8; 32]>,
    pending_expires_at: Option<u64>,
}

#[derive(Debug, Clone)]
struct SignInPending {
    token_hash: [u8; 32],
    expires_at: u64,
}

#[derive(Debug, Clone)]
struct InvitationPending {
    practice_id: String,
    display_name: String,
    admin: bool,
    token_hash: [u8; 32],
    expires_at: u64,
}

/// Announces one memory-store mutation on drop.
struct Announce<'a>(&'a MemoryAuthStore);

impl Drop for Announce<'_> {
    fn drop(&mut self) {
        self.0.changed();
    }
}

/// The process-memory store. A deployment's SQL store replaces it.
#[derive(Debug)]
pub struct MemoryAuthStore {
    state: Mutex<MemoryState>,
    /// Every mutation is observed in-process, so the feed is always live.
    changes: watch::Sender<AuthorityFeed>,
}

impl Default for MemoryAuthStore {
    fn default() -> Self {
        Self {
            state: Mutex::default(),
            changes: watch::Sender::new(AuthorityFeed {
                generation: 0,
                live: true,
            }),
        }
    }
}

impl MemoryAuthStore {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Announces one authority mutation to followers.
    fn changed(&self) {
        self.changes.send_modify(|feed| feed.generation += 1);
    }

    /// Announces a mutation when the returned guard drops. Declared before
    /// the state lock, it drops after it, so followers never re-read while
    /// the mutation still holds the lock, and every early return announces.
    fn announce(&self) -> Announce<'_> {
        Announce(self)
    }

    /// Seeds one account directly, for fixtures and the staging practice.
    pub fn seed_account(
        &self,
        practice_id: &str,
        physician_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
    ) {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        state.accounts.insert(
            email.to_lowercase(),
            Account {
                identity: PhysicianIdentity {
                    practice_id: practice_id.to_owned(),
                    physician_id: physician_id.to_owned(),
                },
                email: email.to_owned(),
                display_name: display_name.to_owned(),
                active: true,
                admin,
                sessions_ended_before: 0,
            },
        );
    }

    /// Registers one patient directly, for fixtures and the staging practice.
    pub fn seed_patient(&self, practice_id: &str, patient_workspace_id: &str) {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        state
            .patients
            .insert(patient_workspace_id.to_owned(), practice_id.to_owned());
    }

    /// Places one patient on a physician's panel directly, for fixtures.
    pub fn seed_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        state.memberships.insert(
            (
                identity.practice_id.clone(),
                identity.physician_id.clone(),
                patient_workspace_id.to_owned(),
            ),
            membership,
        );
    }

    /// Creates one practice directly, for fixtures and the staging practice.
    /// A practice that already exists keeps its clock and time zone.
    pub fn seed_practice(&self, practice_id: &str, clock: PracticeClock) {
        self.state
            .lock()
            .expect("auth state")
            .practices
            .entry(practice_id.to_owned())
            .or_insert_with(|| (clock, PracticeTimeZone::default()));
    }

    /// Lists one practice's accounts, for the admin surface.
    pub fn practice_accounts(&self, practice_id: &str) -> Vec<Account> {
        let state = self.state.lock().expect("auth state");
        let mut accounts: Vec<Account> = state
            .accounts
            .values()
            .filter(|account| account.identity.practice_id == practice_id)
            .cloned()
            .collect();
        accounts.sort();
        accounts
    }

    /// Mints a session for one account directly, for proof setups.
    pub fn seed_session(&self, email: &str, now: u64) -> Option<String> {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        let (identity, fence) = {
            let account = state
                .accounts
                .get(&email.to_lowercase())
                .filter(|account| account.active)?;
            (account.identity.clone(), account.sessions_ended_before)
        };
        let minted_at = post_fence_mint_second(now, fence);
        self.mint_session(&mut state, identity, now, minted_at)
    }

    fn mint_session(
        &self,
        state: &mut MemoryState,
        identity: PhysicianIdentity,
        now: u64,
        created_at: u64,
    ) -> Option<String> {
        let expires_at = SessionLifetime::mint(now).expires_at();
        Some(Self::insert_session(
            state,
            identity,
            created_at,
            SessionLifetime::from_persisted(created_at, expires_at, None),
        ))
    }

    fn insert_session(
        state: &mut MemoryState,
        identity: PhysicianIdentity,
        now: u64,
        lifetime: SessionLifetime,
    ) -> String {
        let token = mint_token();
        let hash = token_hash(&token);
        state.sessions.insert(
            hash,
            Session {
                identity,
                token_hash: hash,
                lifetime,
                created_at: now,
            },
        );
        token
    }
}

#[async_trait]
impl AuthStore for MemoryAuthStore {
    fn authority_changes(&self) -> Option<watch::Receiver<AuthorityFeed>> {
        Some(self.changes.subscribe())
    }

    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError> {
        let key = email.to_lowercase();
        let mut state = self.state.lock().expect("auth state");
        let account = state.accounts.get(&key).ok_or(SignInError::Invalid)?;
        if !account.active {
            return Err(SignInError::Inactive);
        }
        // One pending token per email: re-requesting replaces the last one.
        let token = mint_token();
        state.sign_ins.insert(
            key,
            (
                SignInPending {
                    token_hash: token_hash(&token),
                    expires_at: now + SIGN_IN_TOKEN_TTL_SECONDS,
                },
                (),
            ),
        );
        Ok(token)
    }

    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError> {
        let _announce = self.announce();
        let key = email.to_lowercase();
        let mut state = self.state.lock().expect("auth state");
        // Remove-then-commit: the pending token is consumed once, even if the
        // account read below fails, so a raced second verify cannot mint two.
        let Some((pending, _)) = state.sign_ins.remove(&key) else {
            return Err(SignInError::Invalid);
        };
        if pending.expires_at <= now || pending.token_hash != token_hash(token) {
            return Err(SignInError::Invalid);
        }
        let account = state.accounts.get(&key).ok_or(SignInError::Invalid)?;
        if !account.active {
            return Err(SignInError::Inactive);
        }
        let identity = account.identity.clone();
        let minted_at = post_fence_mint_second(now, account.sessions_ended_before);
        let session_token = self.mint_session(&mut state, identity.clone(), now, minted_at);
        Ok((identity, session_token.expect("token minted")))
    }

    async fn mint_fixed_session(
        &self,
        identity: &PhysicianIdentity,
        now: u64,
        ends_at: u64,
    ) -> Result<String, SignInError> {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        let account = state
            .accounts
            .values()
            .find(|account| &account.identity == identity)
            .ok_or(SignInError::Invalid)?;
        if !account.active {
            return Err(SignInError::Inactive);
        }
        let minted_at = post_fence_mint_second(now, account.sessions_ended_before);
        let expires_at = SessionLifetime::fixed(now, ends_at).expires_at();
        Ok(Self::insert_session(
            &mut state,
            identity.clone(),
            minted_at,
            SessionLifetime::from_persisted(minted_at, expires_at, Some(expires_at)),
        ))
    }

    async fn verified_session(&self, token: &str, now: u64) -> Result<Session, SessionError> {
        let hash = token_hash(token);
        let state = self.state.lock().expect("auth state");
        let session = state.sessions.get(&hash).ok_or(SessionError::Invalid)?;
        if !session.lifetime.allows(now) {
            return Err(SessionError::Invalid);
        }
        let account = state
            .accounts
            .values()
            .find(|account| account.identity == session.identity)
            .ok_or(SessionError::Inactive)?;
        if !account.active {
            return Err(SessionError::Inactive);
        }
        if session.created_at < account.sessions_ended_before {
            // Ended by deactivation: the account is active again, this
            // session still is not.
            return Err(SessionError::Inactive);
        }
        Ok(session.clone())
    }

    async fn note_physician_activity(
        &self,
        token: &str,
        now: u64,
    ) -> Result<SessionActivity, StoreUnavailable> {
        let hash = token_hash(token);
        let mut state = self.state.lock().expect("auth state");
        let Some(session) = state.sessions.get_mut(&hash) else {
            return Ok(SessionActivity::Ended);
        };
        match session.lifetime.renew(now) {
            SessionRenewal::Covered => Ok(SessionActivity::Covered),
            SessionRenewal::Ended => Ok(SessionActivity::Ended),
            SessionRenewal::Extend { idle_deadline } => {
                session.lifetime = session.lifetime.with_idle_deadline(idle_deadline);
                Ok(SessionActivity::Extended)
            }
        }
    }

    async fn publish_authorized(
        &self,
        token: &str,
        identity: &PhysicianIdentity,
        workspace: &str,
        now: u64,
        publish: &mut (dyn FnMut() -> Result<(), String> + Send),
    ) -> Result<(), String> {
        let state = self.state.lock().expect("auth state");
        let session = state
            .sessions
            .get(&token_hash(token))
            .ok_or("session ended")?;
        let account = state
            .accounts
            .values()
            .find(|a| a.identity == *identity)
            .ok_or("account unavailable")?;
        if session.identity != *identity
            || !session.lifetime.allows(now)
            || !account.active
            || session.created_at < account.sessions_ended_before
            || state
                .patients
                .get(workspace)
                .is_some_and(|p| p != &identity.practice_id)
            || state.memberships.get(&(
                identity.practice_id.clone(),
                identity.physician_id.clone(),
                workspace.into(),
            )) != Some(&Membership::Allowed)
        {
            return Err("physician authority changed".into());
        }
        // Every direct memory mutation takes this same mutex.
        publish()
    }
    async fn end_session(&self, token: &str) -> Result<(), StoreUnavailable> {
        let _announce = self.announce();
        self.state
            .lock()
            .expect("auth state")
            .sessions
            .remove(&token_hash(token));
        Ok(())
    }

    async fn account_by_email(&self, email: &str) -> Option<Account> {
        self.state
            .lock()
            .expect("auth state")
            .accounts
            .get(&email.to_lowercase())
            .cloned()
    }

    async fn account_by_identity(&self, identity: &PhysicianIdentity) -> Option<Account> {
        let state = self.state.lock().expect("auth state");
        let mut matched = state
            .accounts
            .values()
            .filter(|account| account.identity == *identity);
        let account = matched.next()?.clone();
        if matched.next().is_some() {
            return None;
        }
        Some(account)
    }

    async fn membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Membership {
        let state = self.state.lock().expect("auth state");
        let owner_practice = state.patients.get(patient_workspace_id);
        if let Some(owner) = owner_practice
            && owner != &identity.practice_id
        {
            return Membership::Denied;
        }
        if let Some(explicit) = state.memberships.get(&(
            identity.practice_id.clone(),
            identity.physician_id.clone(),
            patient_workspace_id.to_owned(),
        )) {
            match explicit {
                Membership::Allowed => return Membership::Allowed,
                Membership::ClinicDemographic => return Membership::ClinicDemographic,
                Membership::Denied => {}
            }
        }
        if owner_practice.is_some_and(|owner| owner == &identity.practice_id) {
            return Membership::ClinicDemographic;
        }
        Membership::Denied
    }

    async fn set_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), StoreUnavailable> {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        if membership == Membership::ClinicDemographic {
            state
                .patients
                .entry(patient_workspace_id.to_owned())
                .or_insert_with(|| identity.practice_id.clone());
        }
        state.memberships.insert(
            (
                identity.practice_id.clone(),
                identity.physician_id.clone(),
                patient_workspace_id.to_owned(),
            ),
            membership,
        );
        Ok(())
    }

    async fn place_on_panel(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable> {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        state
            .memberships
            .entry((
                identity.practice_id.clone(),
                identity.physician_id.clone(),
                patient_workspace_id.to_owned(),
            ))
            .or_insert(Membership::Allowed);
        Ok(())
    }

    async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String> {
        let state = self.state.lock().expect("auth state");
        let mut workspaces: Vec<String> = state
            .memberships
            .iter()
            .filter(|((practice_id, physician_id, workspace_id), membership)| {
                *practice_id == identity.practice_id
                    && *physician_id == identity.physician_id
                    && **membership == Membership::Allowed
                    && state
                        .patients
                        .get(workspace_id)
                        .is_none_or(|owner| owner == &identity.practice_id)
            })
            .map(|((_, _, workspace_id), _)| workspace_id.clone())
            .collect();
        workspaces.sort();
        workspaces
    }

    async fn assign_panel(
        &self,
        actor: &PhysicianIdentity,
        physician_id: &str,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), PanelError> {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        let caller = state
            .accounts
            .values()
            .find(|account| account.identity == *actor)
            .ok_or(PanelError::Forbidden)?;
        if !caller.active || !caller.admin {
            return Err(PanelError::Forbidden);
        }
        let target = PhysicianIdentity {
            practice_id: actor.practice_id.clone(),
            physician_id: physician_id.to_owned(),
        };
        if !state
            .accounts
            .values()
            .any(|account| account.identity == target)
        {
            return Err(PanelError::UnknownTarget);
        }
        if state
            .patients
            .get(patient_workspace_id)
            .is_some_and(|owner| owner != &actor.practice_id)
        {
            return Err(PanelError::UnknownTarget);
        }
        state.memberships.insert(
            (
                target.practice_id,
                target.physician_id,
                patient_workspace_id.to_owned(),
            ),
            membership,
        );
        Ok(())
    }

    async fn ensure_practice(
        &self,
        practice_id: &str,
        clock: PracticeClock,
    ) -> Result<(), StoreUnavailable> {
        self.seed_practice(practice_id, clock);
        Ok(())
    }

    async fn upsert_practice(
        &self,
        record: &PracticeRecord,
    ) -> Result<PracticeSetupOutcome, PracticeSetupError> {
        let practice_id = validate_practice_id(&record.practice_id)?;
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        if let Some((existing_clock, existing_time_zone)) = state.practices.get(&practice_id) {
            if *existing_clock == record.clock && *existing_time_zone == record.time_zone {
                return Ok(PracticeSetupOutcome::Unchanged);
            }
            return Err(PracticeSetupError::Conflict {
                practice_id,
                existing_clock: *existing_clock,
                existing_time_zone: existing_time_zone.clone(),
            });
        }
        state
            .practices
            .insert(practice_id, (record.clock, record.time_zone.clone()));
        Ok(PracticeSetupOutcome::Created)
    }

    async fn practice(
        &self,
        practice_id: &str,
    ) -> Result<Option<PracticeRecord>, StoreUnavailable> {
        let trimmed = practice_id.trim();
        let state = self.state.lock().expect("auth state");
        Ok(state
            .practices
            .get(trimmed)
            .map(|(clock, time_zone)| PracticeRecord {
                practice_id: trimmed.to_owned(),
                clock: *clock,
                time_zone: time_zone.clone(),
            }))
    }

    async fn practice_clock(&self, practice_id: &str) -> Result<PracticeClock, StoreUnavailable> {
        Ok(self
            .state
            .lock()
            .expect("auth state")
            .practices
            .get(practice_id)
            .map(|(clock, _)| *clock)
            .unwrap_or(PracticeClock::RealTime))
    }

    async fn register_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable> {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        state
            .patients
            .entry(patient_workspace_id.to_owned())
            .or_insert_with(|| practice_id.to_owned());
        Ok(())
    }

    async fn patient_practice(
        &self,
        patient_workspace_id: &str,
    ) -> Result<Option<String>, StoreUnavailable> {
        Ok(self
            .state
            .lock()
            .expect("auth state")
            .patients
            .get(patient_workspace_id)
            .cloned())
    }

    async fn practice_patients(&self, practice_id: &str) -> Result<Vec<String>, StoreUnavailable> {
        let state = self.state.lock().expect("auth state");
        let mut patients: Vec<String> = state
            .patients
            .iter()
            .filter(|(_, owner)| *owner == practice_id)
            .map(|(workspace_id, _)| workspace_id.clone())
            .collect();
        patients.sort();
        Ok(patients)
    }

    async fn practice_accounts(&self, practice_id: &str) -> Result<Vec<Account>, StoreUnavailable> {
        Ok(MemoryAuthStore::practice_accounts(self, practice_id))
    }

    async fn create_account(
        &self,
        practice_id: &str,
        physician_id: Option<&str>,
        email: &str,
        display_name: &str,
        admin: bool,
        _now: u64,
    ) -> Result<(Account, AccountCreateOutcome), AccountSetupError> {
        let caller_named_physician = physician_id
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .is_some();
        let (practice_id, resolved_physician_id, key, clean_name) =
            validate_account_fields(practice_id, physician_id, email, display_name)?;
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        if let Some(existing) = state.accounts.get(&key) {
            check_existing_account_match(
                existing,
                &practice_id,
                caller_named_physician,
                &resolved_physician_id,
                &clean_name,
                admin,
            )?;
            return Ok((existing.clone(), AccountCreateOutcome::Existing));
        }
        if state.accounts.values().any(|account| {
            account.identity.practice_id == practice_id
                && account.identity.physician_id == resolved_physician_id
        }) {
            return Err(AccountSetupError::PhysicianIdConflict {
                practice_id,
                physician_id: resolved_physician_id,
            });
        }
        if let Some(invitation) = state.invitations.get(&key) {
            if invitation.practice_id != practice_id {
                return Err(AccountSetupError::EmailConflict {
                    email: key,
                    existing_practice_id: invitation.practice_id.clone(),
                });
            }
            state.invitations.remove(&key);
        }
        let account = Account {
            identity: PhysicianIdentity {
                practice_id,
                physician_id: resolved_physician_id,
            },
            email: key.clone(),
            display_name: clean_name,
            active: true,
            admin,
            sessions_ended_before: 0,
        };
        state.accounts.insert(key, account.clone());
        Ok((account, AccountCreateOutcome::Created))
    }

    async fn invite(
        &self,
        practice_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<String, SignInError> {
        let key = email.trim().to_lowercase();
        if key.is_empty() || !key.contains('@') {
            return Err(SignInError::Invalid);
        }
        let mut state = self.state.lock().expect("auth state");
        if let Some(existing) = state.invitations.get(&key)
            && existing.practice_id != practice_id
        {
            return Err(SignInError::Invalid);
        }
        let token = mint_token();
        state.invitations.insert(
            key,
            InvitationPending {
                practice_id: practice_id.to_owned(),
                display_name: display_name.to_owned(),
                admin,
                token_hash: token_hash(&token),
                expires_at: now + INVITATION_TTL_SECONDS,
            },
        );
        Ok(token)
    }

    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError> {
        let _announce = self.announce();
        let key = email.to_lowercase();
        let mut state = self.state.lock().expect("auth state");
        if state.accounts.contains_key(&key) {
            // The account exists; it signs in rather than redeeming again.
            return Err(SignInError::Invalid);
        }
        let Some(invitation) = state.invitations.get(&key).cloned() else {
            return Err(SignInError::Invalid);
        };
        if invitation.expires_at <= now || invitation.token_hash != token_hash(token) {
            return Err(SignInError::Invalid);
        }
        state.invitations.remove(&key);
        let identity = PhysicianIdentity {
            practice_id: invitation.practice_id.clone(),
            physician_id: Uuid::new_v4().simple().to_string(),
        };
        let display_name = invitation.display_name.clone();
        let admin = invitation.admin;
        state.accounts.insert(
            key.clone(),
            Account {
                identity: identity.clone(),
                email: key,
                display_name,
                active: true,
                admin,
                sessions_ended_before: 0,
            },
        );
        let session_token = self
            .mint_session(&mut state, identity.clone(), now, now)
            .expect("token minted");
        Ok((identity, session_token))
    }

    async fn deactivate(&self, email: &str) -> bool {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        let Some(identity) = state
            .accounts
            .get(&email.to_lowercase())
            .map(|account| account.identity.clone())
        else {
            return false;
        };
        // The fence, not row deletion: reactivation never resurrects the
        // sessions deactivation ended, and resolution keeps naming why.
        let fence = state
            .sessions
            .values()
            .filter(|session| session.identity == identity)
            .map(|session| session.created_at)
            .max()
            .unwrap_or(0);
        // One past the newest held session's mint second, so that session
        // and every older one stop resolving.
        let account = state
            .accounts
            .get_mut(&email.to_lowercase())
            .expect("account read above");
        account.active = false;
        account.sessions_ended_before = fence.saturating_add(1);
        true
    }

    async fn reactivate(&self, email: &str) -> bool {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        let Some(account) = state.accounts.get_mut(&email.to_lowercase()) else {
            return false;
        };
        account.active = true;
        true
    }

    async fn remove_account(&self, email: &str) -> bool {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        let Some(account) = state.accounts.remove(&email.to_lowercase()) else {
            return false;
        };
        state
            .sessions
            .retain(|_, session| session.identity != account.identity);
        state
            .memberships
            .retain(|(practice_id, physician_id, _), _| {
                !(practice_id == &account.identity.practice_id
                    && physician_id == &account.identity.physician_id)
            });
        true
    }

    async fn invite_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<String, SignInError> {
        let practice_id = practice_id.trim();
        let patient_workspace_id = patient_workspace_id.trim();
        if practice_id.is_empty() || patient_workspace_id.is_empty() {
            return Err(SignInError::Invalid);
        }
        let token = mint_patient_token();
        let hash = token_hash(&token);
        let mut state = self.state.lock().expect("auth state");
        let entry = state
            .patient_links
            .entry((practice_id.to_owned(), patient_workspace_id.to_owned()))
            .or_insert_with(|| PatientLinkRecord {
                link: PatientLink {
                    practice_id: practice_id.to_owned(),
                    patient_workspace_id: patient_workspace_id.to_owned(),
                    duet_account_id: None,
                    linked: false,
                    sharing_consented: false,
                    consent_version: None,
                    consented_at: None,
                    revoked_at: None,
                },
                pending_token_hash: None,
                pending_expires_at: None,
            });
        entry.pending_token_hash = Some(hash);
        entry.pending_expires_at = Some(now + INVITATION_TTL_SECONDS);
        Ok(token)
    }

    async fn accept_patient_invitation(
        &self,
        token: &str,
        duet_account_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        let duet_account_id = duet_account_id.trim();
        if duet_account_id.is_empty() {
            return Err(SignInError::Invalid);
        }
        let hash = token_hash(token);
        let mut state = self.state.lock().expect("auth state");
        let Some(record) = state
            .patient_links
            .values_mut()
            .find(|record| record.pending_token_hash == Some(hash))
        else {
            return Err(SignInError::Invalid);
        };
        let expires_at = record.pending_expires_at.unwrap_or(0);
        // Consume the single-use pending invitation token on verification attempt.
        record.pending_token_hash = None;
        record.pending_expires_at = None;
        if expires_at <= now {
            return Err(SignInError::Invalid);
        }
        if record.link.duet_account_id.as_deref() != Some(duet_account_id) {
            record.link.sharing_consented = false;
            record.link.consent_version = None;
            record.link.consented_at = None;
            record.link.revoked_at = None;
        }
        record.link.duet_account_id = Some(duet_account_id.to_owned());
        record.link.linked = true;
        Ok(record.link.clone())
    }

    async fn grant_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        duet_account_id: &str,
        consent_version: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        let consent_version = consent_version.trim();
        let duet_account_id = duet_account_id.trim();
        if consent_version.is_empty() || duet_account_id.is_empty() {
            return Err(SignInError::Invalid);
        }
        let mut state = self.state.lock().expect("auth state");
        let record = state
            .patient_links
            .get_mut(&(practice_id.to_owned(), patient_workspace_id.to_owned()))
            .ok_or(SignInError::Invalid)?;
        if !record.link.linked || record.link.duet_account_id.as_deref() != Some(duet_account_id) {
            return Err(SignInError::Invalid);
        }
        if record.link.revoked_at.is_some() {
            return Err(SignInError::Invalid);
        }
        record.link.sharing_consented = true;
        record.link.consent_version = Some(consent_version.to_owned());
        record.link.consented_at = Some(now);
        Ok(record.link.clone())
    }

    async fn revoke_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        let mut state = self.state.lock().expect("auth state");
        let record = state
            .patient_links
            .get_mut(&(practice_id.to_owned(), patient_workspace_id.to_owned()))
            .ok_or(SignInError::Invalid)?;
        if !record.link.linked {
            return Err(SignInError::Invalid);
        }
        record.link.sharing_consented = false;
        record.link.revoked_at = Some(now);
        Ok(record.link.clone())
    }

    async fn patient_link(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Option<PatientLink> {
        self.state
            .lock()
            .expect("auth state")
            .patient_links
            .get(&(practice_id.to_owned(), patient_workspace_id.to_owned()))
            .map(|record| record.link.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_760_000_000;

    async fn signed_in(store: &MemoryAuthStore, email: &str) -> (PhysicianIdentity, String) {
        let token = store.begin_sign_in(email, NOW).await.unwrap();
        store.complete_sign_in(email, &token, NOW).await.unwrap()
    }

    #[tokio::test]
    async fn a_practice_keeps_the_clock_it_was_created_with() {
        let store = MemoryAuthStore::new();
        let fixed = PracticeClock::Fixed(1_787_846_400);
        store.ensure_practice("synthetic", fixed).await.unwrap();
        store
            .ensure_practice("synthetic", PracticeClock::RealTime)
            .await
            .unwrap();
        assert_eq!(store.practice_clock("synthetic").await.unwrap(), fixed);
        assert_eq!(
            store.practice_clock("real").await.unwrap(),
            PracticeClock::RealTime
        );
    }

    #[tokio::test]
    async fn an_issued_token_signs_in_once_and_mints_a_session() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let token = store.begin_sign_in("doc@example.test", NOW).await.unwrap();
        let (identity, session) = store
            .complete_sign_in("doc@example.test", &token, NOW)
            .await
            .unwrap();
        assert_eq!(identity.physician_id, "physician");
        assert_eq!(store.identity(&session, NOW).await.unwrap(), identity);
        assert_eq!(
            store
                .complete_sign_in("doc@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid,
            "a verified token never verifies again"
        );
        // A re-request replaces the pending token; the replaced one is dead.
        let _ = store.begin_sign_in("doc@example.test", NOW).await.unwrap();
        assert_eq!(
            store
                .complete_sign_in("doc@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
    }

    #[tokio::test]
    async fn an_unknown_or_inactive_email_never_receives_a_token() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        assert_eq!(
            store
                .begin_sign_in("nobody@example.test", NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid,
            "an unknown email is indistinguishable from an invalid token"
        );
        store.deactivate("doc@example.test").await;
        assert_eq!(
            store
                .begin_sign_in("doc@example.test", NOW)
                .await
                .unwrap_err(),
            SignInError::Inactive
        );
    }

    #[tokio::test]
    async fn an_expired_sign_in_token_is_invalid() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let token = store.begin_sign_in("doc@example.test", NOW).await.unwrap();
        assert_eq!(
            store
                .complete_sign_in("doc@example.test", &token, NOW + SIGN_IN_TOKEN_TTL_SECONDS)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
    }

    #[tokio::test]
    async fn a_wrong_sign_in_token_is_invalid() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        store.begin_sign_in("doc@example.test", NOW).await.unwrap();
        assert_eq!(
            store
                .complete_sign_in("doc@example.test", "codas-wrong", NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
    }

    #[tokio::test]
    async fn a_session_expires_and_ends_on_sign_out() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        assert!(
            store
                .identity(&session, NOW + 12 * 60 * 60 - 1)
                .await
                .is_ok(),
            "an unused session still resolves inside 12 hours"
        );
        assert!(
            store.identity(&session, NOW + 12 * 60 * 60).await.is_err(),
            "an unused session stops resolving after 12 hours"
        );
        let (_, session) = signed_in(&store, "doc@example.test").await;
        store.end_session(&session).await.unwrap();
        assert!(
            store.identity(&session, NOW).await.is_err(),
            "sign-out ends the session"
        );
    }

    #[tokio::test]
    async fn repeated_identity_calls_do_not_move_expires_at() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        let before = store
            .verified_session(&session, NOW)
            .await
            .unwrap()
            .lifetime
            .expires_at();
        assert_eq!(before, NOW + 12 * 60 * 60);
        store.identity(&session, NOW).await.unwrap();
        store.identity(&session, NOW + 60).await.unwrap();
        store.identity(&session, NOW + 11 * 60 * 60).await.unwrap();
        let after = store
            .verified_session(&session, NOW)
            .await
            .unwrap()
            .lifetime
            .expires_at();
        assert_eq!(after, before);
    }

    #[tokio::test]
    async fn activity_inside_twelve_hours_keeps_the_session() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        let used_at = NOW + 11 * 60 * 60;
        assert_eq!(
            store
                .note_physician_activity(&session, used_at)
                .await
                .unwrap(),
            SessionActivity::Extended
        );
        assert!(store.identity(&session, NOW + 12 * 60 * 60).await.is_ok());
        assert!(
            store
                .identity(&session, used_at + 12 * 60 * 60)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn continuous_activity_is_refused_seven_days_after_sign_in() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        let mut at = NOW;
        while at < NOW + 7 * 24 * 60 * 60 {
            assert!(store.identity(&session, at).await.is_ok());
            store.note_physician_activity(&session, at).await.unwrap();
            at += 11 * 60 * 60;
        }
        assert!(
            store
                .identity(&session, NOW + 7 * 24 * 60 * 60 - 1)
                .await
                .is_ok()
        );
        assert!(
            store
                .identity(&session, NOW + 7 * 24 * 60 * 60)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn a_burst_of_notes_inside_one_quantum_persists_one_extension() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        let at = NOW + 15 * 60;
        let mut extensions = 0;
        for _ in 0..5 {
            if store.note_physician_activity(&session, at).await.unwrap()
                == SessionActivity::Extended
            {
                extensions += 1;
            }
        }
        assert_eq!(extensions, 1);
        let once = stored_deadline(&store, &session);
        assert_eq!(once, at + 12 * 60 * 60);
        assert_eq!(
            store
                .note_physician_activity(&session, at + 15 * 60 - 1)
                .await
                .unwrap(),
            SessionActivity::Covered
        );
        assert_eq!(stored_deadline(&store, &session), once);
        assert_eq!(
            store
                .note_physician_activity(&session, at + 15 * 60)
                .await
                .unwrap(),
            SessionActivity::Extended
        );
        assert_eq!(
            stored_deadline(&store, &session),
            at + 15 * 60 + 12 * 60 * 60
        );
    }

    #[tokio::test]
    async fn a_dead_sessions_note_does_not_move_expires_at() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        let deadline = stored_deadline(&store, &session);
        assert_eq!(
            store
                .note_physician_activity(&session, deadline)
                .await
                .unwrap(),
            SessionActivity::Ended
        );
        assert!(store.identity(&session, deadline).await.is_err());
        assert_eq!(stored_deadline(&store, &session), deadline);
    }

    fn stored_deadline(store: &MemoryAuthStore, token: &str) -> u64 {
        store
            .state
            .lock()
            .expect("auth state")
            .sessions
            .get(&token_hash(token))
            .expect("stored session")
            .lifetime
            .expires_at()
    }

    #[tokio::test]
    async fn deactivation_and_removal_end_every_held_session() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        assert!(store.deactivate("doc@example.test").await);
        assert_eq!(
            store.identity(&session, NOW).await.unwrap_err(),
            SessionError::Inactive,
            "a deactivated account's session cannot act"
        );
        assert!(store.reactivate("doc@example.test").await);
        assert!(
            store.identity(&session, NOW).await.is_err(),
            "sessions ended by deactivation stay ended"
        );
        let (_, session) = signed_in(&store, "doc@example.test").await;
        assert!(store.remove_account("doc@example.test").await);
        assert_eq!(
            store.identity(&session, NOW).await.unwrap_err(),
            SessionError::Invalid,
            "removal deletes the sessions, so the token is simply unknown"
        );
        assert!(!store.remove_account("doc@example.test").await);
    }

    #[tokio::test]
    async fn membership_denies_by_default_and_resolves_per_workspace() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let identity = PhysicianIdentity {
            practice_id: "practice".to_owned(),
            physician_id: "physician".to_owned(),
        };
        assert_eq!(
            store.membership(&identity, "pw-1").await,
            Membership::Denied,
            "access is granted, never assumed"
        );
        store
            .set_membership(&identity, "pw-1", Membership::Allowed)
            .await
            .unwrap();
        assert_eq!(
            store.membership(&identity, "pw-1").await,
            Membership::Allowed
        );
        assert_eq!(
            store.membership(&identity, "pw-2").await,
            Membership::Denied
        );
    }

    #[tokio::test]
    async fn an_invitation_creates_the_account_and_one_session() {
        let store = MemoryAuthStore::new();
        let token = store
            .invite("practice", "new@example.test", "New Doc", false, NOW)
            .await
            .unwrap();
        let (identity, session) = store
            .redeem_invitation("new@example.test", &token, NOW)
            .await
            .unwrap();
        assert_eq!(identity.practice_id, "practice");
        assert!(store.identity(&session, NOW).await.is_ok());
        let account = store.account_by_email("new@example.test").await.unwrap();
        assert_eq!(account.display_name, "New Doc");
        assert!(account.active);
        assert_eq!(
            store
                .redeem_invitation("new@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid,
            "an invitation redeems once"
        );
    }

    #[tokio::test]
    async fn a_deactivated_account_cannot_redeem_or_sign_in_but_reactivation_restores_it() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        store.deactivate("doc@example.test").await;
        let token = store
            .invite("practice", "doc@example.test", "Doc", false, NOW)
            .await
            .unwrap();
        assert_eq!(
            store
                .redeem_invitation("doc@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
        store.reactivate("doc@example.test").await;
        let token = store.begin_sign_in("doc@example.test", NOW).await.unwrap();
        assert!(
            store
                .complete_sign_in("doc@example.test", &token, NOW)
                .await
                .is_ok()
        );
    }

    #[test]
    fn tokens_are_stored_hashed() {
        let hash = token_hash("codas-abc");
        assert_ne!(hash.as_slice(), b"codas-abc");
        assert_eq!(
            hash,
            token_hash("codas-abc"),
            "verification is deterministic"
        );
    }

    #[tokio::test]
    async fn account_by_identity_is_absent_when_two_accounts_share_it() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "shared", "a@example.test", "A", true);
        store.seed_account("practice", "shared", "b@example.test", "B", false);
        assert!(
            store
                .account_by_identity(&PhysicianIdentity {
                    practice_id: "practice".to_owned(),
                    physician_id: "shared".to_owned(),
                })
                .await
                .is_none()
        );
        store.seed_account("practice", "only", "c@example.test", "C", false);
        let found = store
            .account_by_identity(&PhysicianIdentity {
                practice_id: "practice".to_owned(),
                physician_id: "only".to_owned(),
            })
            .await
            .expect("one match");
        assert_eq!(found.email, "c@example.test");
    }

    #[test]
    fn seeded_sessions_match_the_sign_in_session_shape() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let token = store.seed_session("doc@example.test", NOW).unwrap();
        assert!(token.starts_with("codas-"));
    }

    #[tokio::test]
    async fn clinic_admin_or_authorized_physician_assigns_panels_within_the_practice() {
        let store = MemoryAuthStore::new();
        store.seed_account(
            "practice",
            "secretary",
            "secretary@example.test",
            "Clinic Secretary",
            true,
        );
        store.seed_account(
            "practice",
            "lead-doc",
            "lead@example.test",
            "Lead Physician",
            true,
        );
        store.seed_account("practice", "doc", "doc@example.test", "Physician", false);
        store.seed_account(
            "other-practice",
            "outside-doc",
            "outside@example.test",
            "Outside Physician",
            false,
        );

        let secretary = PhysicianIdentity {
            practice_id: "practice".to_owned(),
            physician_id: "secretary".to_owned(),
        };
        let lead_doc = PhysicianIdentity {
            practice_id: "practice".to_owned(),
            physician_id: "lead-doc".to_owned(),
        };
        let doc = PhysicianIdentity {
            practice_id: "practice".to_owned(),
            physician_id: "doc".to_owned(),
        };

        // A secretary or clinic admin assigns a patient to a physician's panel.
        store
            .assign_panel(&secretary, "doc", "pw-1", Membership::Allowed)
            .await
            .unwrap();
        assert_eq!(store.membership(&doc, "pw-1").await, Membership::Allowed);
        assert_eq!(store.panel(&doc).await, vec!["pw-1".to_owned()]);

        // A physician who also holds the permission can assign panels too.
        store
            .assign_panel(&lead_doc, "doc", "pw-2", Membership::Allowed)
            .await
            .unwrap();
        assert_eq!(
            store.panel(&doc).await,
            vec!["pw-1".to_owned(), "pw-2".to_owned()]
        );

        // A physician without the permission cannot assign panels (even their own).
        assert_eq!(
            store
                .assign_panel(&doc, "doc", "pw-3", Membership::Allowed)
                .await
                .unwrap_err(),
            PanelError::Forbidden
        );

        // Even an admin cannot assign panels for a physician in another practice.
        assert_eq!(
            store
                .assign_panel(&secretary, "outside-doc", "pw-1", Membership::Allowed)
                .await
                .unwrap_err(),
            PanelError::UnknownTarget
        );

        // Removing a patient from the panel revokes membership.
        store
            .assign_panel(&secretary, "doc", "pw-1", Membership::Denied)
            .await
            .unwrap();
        assert_eq!(store.membership(&doc, "pw-1").await, Membership::Denied);
        assert_eq!(store.panel(&doc).await, vec!["pw-2".to_owned()]);
    }

    #[tokio::test]
    async fn registered_practice_patients_resolve_clinic_demographic() {
        let store = MemoryAuthStore::new();
        store.seed_account("clinic-a", "admin", "admin@a.test", "Admin A", true);
        store.seed_account("clinic-a", "doc", "doc@a.test", "Doc A", false);
        store.seed_account("clinic-b", "outside", "doc@b.test", "Doc B", false);

        let admin = PhysicianIdentity {
            practice_id: "clinic-a".to_owned(),
            physician_id: "admin".to_owned(),
        };
        let doc = PhysicianIdentity {
            practice_id: "clinic-a".to_owned(),
            physician_id: "doc".to_owned(),
        };
        let outside = PhysicianIdentity {
            practice_id: "clinic-b".to_owned(),
            physician_id: "outside".to_owned(),
        };

        store.register_patient("clinic-a", "pw-mary").await.unwrap();
        assert_eq!(
            store.patient_practice("pw-mary").await.unwrap().as_deref(),
            Some("clinic-a")
        );
        assert_eq!(
            store.practice_patients("clinic-a").await.unwrap(),
            vec!["pw-mary".to_owned()]
        );

        // Same-clinic non-panel physician receives ClinicDemographic; outside practice is Denied.
        assert_eq!(
            store.membership(&doc, "pw-mary").await,
            Membership::ClinicDemographic
        );
        assert_eq!(
            store.membership(&outside, "pw-mary").await,
            Membership::Denied
        );

        // Admin assigns pw-mary to doc's panel -> Allowed; removing returns to ClinicDemographic.
        store
            .assign_panel(&admin, "doc", "pw-mary", Membership::Allowed)
            .await
            .unwrap();
        assert_eq!(store.membership(&doc, "pw-mary").await, Membership::Allowed);
        store
            .assign_panel(&admin, "doc", "pw-mary", Membership::Denied)
            .await
            .unwrap();
        assert_eq!(
            store.membership(&doc, "pw-mary").await,
            Membership::ClinicDemographic
        );
    }

    #[tokio::test]
    async fn patient_invitation_links_explicit_roster_patient_and_tracks_versioned_consent() {
        let store = MemoryAuthStore::new();
        let token = store
            .invite_patient("practice-1", "pw-elena", NOW)
            .await
            .unwrap();
        assert!(token.starts_with("codap-"));

        // Before acceptance, the patient is not linked and has no active consent.
        let initial = store
            .patient_link("practice-1", "pw-elena")
            .await
            .expect("invited patient record");
        assert!(!initial.linked);
        assert!(!initial.sharing_consented);

        // Consent cannot be granted before accepting the explicit practice invitation.
        assert_eq!(
            store
                .grant_patient_consent("practice-1", "pw-elena", "duet-elena", "v1", NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );

        // Accepting the invitation links `(practice_id, patient_workspace_id)` to `duet_account_id`.
        let linked = store
            .accept_patient_invitation(&token, "duet-elena", NOW + 10)
            .await
            .unwrap();
        assert!(linked.linked);
        assert_eq!(linked.duet_account_id.as_deref(), Some("duet-elena"));
        assert!(!linked.sharing_consented);

        // The single-use invitation token cannot be redeemed a second time.
        assert_eq!(
            store
                .accept_patient_invitation(&token, "duet-other", NOW + 20)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );

        // Granting versioned consent activates sharing for that linked Duet account.
        let consented = store
            .grant_patient_consent("practice-1", "pw-elena", "duet-elena", "v1", NOW + 30)
            .await
            .unwrap();
        assert!(consented.sharing_consented);
        assert_eq!(consented.consent_version.as_deref(), Some("v1"));
        assert_eq!(consented.consented_at, Some(NOW + 30));
        assert_eq!(consented.revoked_at, None);

        let serialized = serde_json::to_string(&consented).unwrap();
        assert!(
            !serialized.contains("premium")
                && !serialized.contains("billing")
                && !serialized.contains("cadence"),
            "serialized link carries connection and consent only"
        );

        // One-step stop-sharing immediately revokes active consent.
        let revoked = store
            .revoke_patient_consent("practice-1", "pw-elena", NOW + 60)
            .await
            .unwrap();
        assert!(revoked.linked);
        assert!(!revoked.sharing_consented);
        assert_eq!(revoked.consent_version.as_deref(), Some("v1"));
        assert_eq!(revoked.revoked_at, Some(NOW + 60));

        assert_eq!(
            store
                .grant_patient_consent("practice-1", "pw-elena", "duet-elena", "v1", NOW + 70)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
        let still = store
            .patient_link("practice-1", "pw-elena")
            .await
            .expect("link remains");
        assert!(!still.sharing_consented);
        assert_eq!(still.revoked_at, Some(NOW + 60));
    }

    #[tokio::test]
    async fn same_second_reactivation_mints_live_session_without_reviving_prior_session() {
        let store = MemoryAuthStore::new();
        let (acct, _) = store
            .create_account(
                "practice-1",
                Some("dr-chen"),
                "dr.chen@practice.org",
                "Dr. Chen",
                false,
                NOW,
            )
            .await
            .expect("create account");
        let code_1 = store
            .begin_sign_in("dr.chen@practice.org", NOW)
            .await
            .expect("sign-in 1");
        let (_, session_1) = store
            .complete_sign_in("dr.chen@practice.org", &code_1, NOW)
            .await
            .expect("verify 1");
        assert_eq!(
            store.identity(&session_1, NOW).await,
            Ok(acct.identity.clone())
        );

        assert!(store.deactivate("dr.chen@practice.org").await);
        assert_eq!(
            store.identity(&session_1, NOW).await,
            Err(SessionError::Inactive)
        );

        assert!(store.reactivate("dr.chen@practice.org").await);
        assert_eq!(
            store.identity(&session_1, NOW).await,
            Err(SessionError::Inactive),
            "prior session must stay revoked after reactivation"
        );

        let code_2 = store
            .begin_sign_in("dr.chen@practice.org", NOW)
            .await
            .expect("sign-in 2 in same second");
        let (_, session_2) = store
            .complete_sign_in("dr.chen@practice.org", &code_2, NOW)
            .await
            .expect("verify 2 in same second");
        assert_eq!(
            store.identity(&session_2, NOW).await,
            Ok(acct.identity),
            "new session minted in same second after reactivation must resolve"
        );
    }

    #[test]
    fn practice_time_zone_computes_local_calendar_date_across_midnight() {
        // 2026-08-28T05:30:00Z = 01:30 EDT (Aug 28) in New York, 22:30 PDT (Aug 27) in Los Angeles.
        let instant_unix = 1_787_895_000;
        let ny = PracticeTimeZone::parse("America/New_York").unwrap();
        let la = PracticeTimeZone::parse("America/Los_Angeles").unwrap();
        let london = PracticeTimeZone::parse("Europe/London").unwrap();
        assert_eq!(ny.calendar_date_iso(instant_unix), "2026-08-28");
        assert_eq!(la.calendar_date_iso(instant_unix), "2026-08-27");
        assert_eq!(london.calendar_date_iso(instant_unix), "2026-08-28");
    }

    #[tokio::test]
    async fn create_account_requires_matching_role_name_and_active_standing_for_existing_email() {
        let store = MemoryAuthStore::new();
        let (created, outcome) = store
            .create_account(
                "practice-1",
                Some("dr-lee"),
                "dr.lee@practice.org",
                "Dr. Lee",
                false,
                NOW,
            )
            .await
            .expect("create physician");
        assert_eq!(outcome, AccountCreateOutcome::Created);

        let (replayed, outcome_2) = store
            .create_account(
                "practice-1",
                None,
                "DR.LEE@practice.org",
                "Dr. Lee",
                false,
                NOW,
            )
            .await
            .expect("idempotent replay");
        assert_eq!(outcome_2, AccountCreateOutcome::Existing);
        assert_eq!(replayed, created);

        // Role mismatch (admin=true vs existing admin=false) conflicts instead of pretending Existing.
        assert_eq!(
            store
                .create_account(
                    "practice-1",
                    Some("dr-lee"),
                    "dr.lee@practice.org",
                    "Dr. Lee",
                    true,
                    NOW,
                )
                .await,
            Err(AccountSetupError::AccountConflict {
                practice_id: "practice-1".to_owned(),
                email: "dr.lee@practice.org".to_owned(),
            })
        );

        // Display-name mismatch conflicts.
        assert_eq!(
            store
                .create_account(
                    "practice-1",
                    Some("dr-lee"),
                    "dr.lee@practice.org",
                    "Dr. Someone Else",
                    false,
                    NOW,
                )
                .await,
            Err(AccountSetupError::AccountConflict {
                practice_id: "practice-1".to_owned(),
                email: "dr.lee@practice.org".to_owned(),
            })
        );

        // Deactivated account conflicts rather than claiming an active account is ready.
        assert!(store.deactivate("dr.lee@practice.org").await);
        assert_eq!(
            store
                .create_account(
                    "practice-1",
                    Some("dr-lee"),
                    "dr.lee@practice.org",
                    "Dr. Lee",
                    false,
                    NOW,
                )
                .await,
            Err(AccountSetupError::AccountConflict {
                practice_id: "practice-1".to_owned(),
                email: "dr.lee@practice.org".to_owned(),
            })
        );
    }
}
