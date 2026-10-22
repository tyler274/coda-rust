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

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

mod cache;
mod error;
mod store;

pub use cache::{AUTHORITY_CHANNEL, CachedAuthStore, ENTRY_TTL, follow};
pub use error::{PanelError, SessionError, SignInError};
pub use patient_work::ReviewCadence;
pub use store::PostgresAuthStore;
pub use tokio::sync::watch;

/// The SeaORM entities and the migration chain.
pub mod migration;

/// A sign-in token is verifiable for one day.
pub const SIGN_IN_TOKEN_TTL_SECONDS: u64 = 86_400;
/// A sign-in session is valid for thirty days.
pub const SESSION_TTL_SECONDS: u64 = 30 * 86_400;
/// An invitation is redeemable for fourteen days.
pub const INVITATION_TTL_SECONDS: u64 = 14 * 86_400;

/// The per-patient review cadence exposed to Agents.
///
/// Agents reads only `Cadence` (`Weekly` or `DailyWhenChanged`) and never
/// reads payment, subscription, or billing state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cadence {
    /// Weekly review cadence (the default for all roster patients).
    #[default]
    Weekly,
    /// Daily review cadence when patient state has changed.
    DailyWhenChanged,
}

impl Cadence {
    /// Returns the wire/database string for this cadence (`"weekly"` or `"daily_when_changed"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Weekly => "weekly",
            Self::DailyWhenChanged => "daily_when_changed",
        }
    }

    /// Parses a cadence string, defaulting unknown values to `None`.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "weekly" => Some(Self::Weekly),
            "daily_when_changed" => Some(Self::DailyWhenChanged),
            _ => None,
        }
    }
}

/// The front-door link between a practice's roster patient `(practice_id, patient_workspace_id)`
/// and the patient's Duet account, including versioned sharing consent and review cadence.
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
    /// The Unix second consent was revoked via one-step stop-sharing, if revoked.
    pub revoked_at: Option<u64>,
    /// The per-patient review cadence exposed to Agents (`Weekly` | `DailyWhenChanged`).
    pub cadence: Cadence,
}

/// A physician's practice-scoped identity, as the projections key it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicianIdentity {
    /// The practice this physician belongs to.
    pub practice_id: String,
    /// The physician within the practice.
    pub physician_id: String,
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
    /// When the session stops being valid, in Unix seconds.
    pub expires_at: u64,
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
    /// Resolves a bearer token to its physician's identity.
    async fn session(&self, token: &str, now: u64) -> Result<PhysicianIdentity, SessionError>;
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
    async fn end_session(&self, token: &str);
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
    );
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
    /// Registers a patient workspace as belonging to `practice_id`.
    async fn register_patient(&self, practice_id: &str, patient_workspace_id: &str);
    /// Returns the practice that owns `patient_workspace_id`, if registered.
    async fn patient_practice(&self, patient_workspace_id: &str) -> Option<String>;
    /// Lists the patient workspaces registered in `practice_id`, sorted.
    async fn practice_patients(&self, practice_id: &str) -> Vec<String>;
    /// Lists the physician accounts in `practice_id`, sorted.
    async fn practice_accounts(&self, practice_id: &str) -> Vec<Account>;
    /// Reads the per-patient [`ReviewCadence`] without exposing payment status.
    async fn patient_cadence(&self, patient_workspace_id: &str) -> ReviewCadence;
    /// Sets the per-patient [`ReviewCadence`] (written by the front door /
    /// practice bridge when patient health-link tier changes, never by
    /// physician override).
    async fn set_patient_cadence(&self, patient_workspace_id: &str, cadence: ReviewCadence);
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
    /// Sets whether a roster patient has premium front-door features, which maps internally
    /// to `Cadence::DailyWhenChanged` (or `Cadence::Weekly` when `false`) without exposing
    /// billing or payment state to Agents.
    async fn set_patient_premium(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        premium: bool,
    );
    /// Reads the `PatientLink` for `(practice_id, patient_workspace_id)`, if invited or configured.
    async fn patient_link(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Option<PatientLink>;
    /// Reads the per-patient review `Cadence` (`Weekly` | `DailyWhenChanged`) exposed to Agents.
    ///
    /// This is the same per-patient cadence as [`AuthStore::patient_cadence`], keyed by
    /// `(practice_id, patient_workspace_id)` instead of workspace id alone.
    async fn patient_link_cadence(&self, practice_id: &str, patient_workspace_id: &str) -> Cadence;
}

/// Maps the Agents-facing `Cadence` onto the internal `ReviewCadence`.
fn cadence_to_review_cadence(cadence: Cadence) -> ReviewCadence {
    match cadence {
        Cadence::Weekly => ReviewCadence::Weekly,
        Cadence::DailyWhenChanged => ReviewCadence::WeeklyAndDailyWhenChanged,
    }
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
    patients: HashMap<String, String>,
    cadences: HashMap<String, ReviewCadence>,
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
        let identity = state
            .accounts
            .get(&email.to_lowercase())
            .filter(|account| account.active)?
            .identity
            .clone();
        self.mint_session(&mut state, identity, now)
    }

    fn mint_session(
        &self,
        state: &mut MemoryState,
        identity: PhysicianIdentity,
        now: u64,
    ) -> Option<String> {
        let token = mint_token();
        let hash = token_hash(&token);
        state.sessions.insert(
            hash,
            Session {
                identity,
                token_hash: hash,
                expires_at: now + SESSION_TTL_SECONDS,
                created_at: now,
            },
        );
        Some(token)
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
        let session_token = self.mint_session(&mut state, identity.clone(), now);
        Ok((identity, session_token.expect("token minted")))
    }

    async fn session(&self, token: &str, now: u64) -> Result<PhysicianIdentity, SessionError> {
        let hash = token_hash(token);
        let state = self.state.lock().expect("auth state");
        let session = state.sessions.get(&hash).ok_or(SessionError::Invalid)?;
        if session.expires_at <= now {
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
        Ok(account.identity.clone())
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
            || session.expires_at <= now
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
    async fn end_session(&self, token: &str) {
        let _announce = self.announce();
        self.state
            .lock()
            .expect("auth state")
            .sessions
            .remove(&token_hash(token));
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
    ) {
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

    async fn register_patient(&self, practice_id: &str, patient_workspace_id: &str) {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        state
            .patients
            .insert(patient_workspace_id.to_owned(), practice_id.to_owned());
        state
            .cadences
            .entry(patient_workspace_id.to_owned())
            .or_insert(ReviewCadence::Weekly);
    }

    async fn patient_practice(&self, patient_workspace_id: &str) -> Option<String> {
        self.state
            .lock()
            .expect("auth state")
            .patients
            .get(patient_workspace_id)
            .cloned()
    }

    async fn practice_patients(&self, practice_id: &str) -> Vec<String> {
        let state = self.state.lock().expect("auth state");
        let mut patients: Vec<String> = state
            .patients
            .iter()
            .filter(|(_, owner)| *owner == practice_id)
            .map(|(workspace_id, _)| workspace_id.clone())
            .collect();
        patients.sort();
        patients
    }

    async fn practice_accounts(&self, practice_id: &str) -> Vec<Account> {
        MemoryAuthStore::practice_accounts(self, practice_id)
    }

    async fn patient_cadence(&self, patient_workspace_id: &str) -> ReviewCadence {
        self.state
            .lock()
            .expect("auth state")
            .cadences
            .get(patient_workspace_id)
            .copied()
            .unwrap_or(ReviewCadence::Weekly)
    }

    async fn set_patient_cadence(&self, patient_workspace_id: &str, cadence: ReviewCadence) {
        let _announce = self.announce();
        let mut state = self.state.lock().expect("auth state");
        state
            .cadences
            .insert(patient_workspace_id.to_owned(), cadence);
        if let Some(record) = state
            .patient_links
            .values_mut()
            .find(|record| record.link.patient_workspace_id == patient_workspace_id)
        {
            record.link.cadence = match cadence {
                ReviewCadence::Weekly => Cadence::Weekly,
                ReviewCadence::WeeklyAndDailyWhenChanged => Cadence::DailyWhenChanged,
            };
        }
    }

    async fn invite(
        &self,
        practice_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<String, SignInError> {
        let token = mint_token();
        self.state.lock().expect("auth state").invitations.insert(
            email.to_lowercase(),
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
            .mint_session(&mut state, identity.clone(), now)
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
                    cadence: Cadence::Weekly,
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

    async fn set_patient_premium(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        premium: bool,
    ) {
        let _announce = self.announce();
        let cadence = if premium {
            Cadence::DailyWhenChanged
        } else {
            Cadence::Weekly
        };
        let review_cadence = cadence_to_review_cadence(cadence);
        let mut state = self.state.lock().expect("auth state");
        state
            .cadences
            .insert(patient_workspace_id.to_owned(), review_cadence);
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
                    cadence,
                },
                pending_token_hash: None,
                pending_expires_at: None,
            });
        entry.link.cadence = cadence;
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

    async fn patient_link_cadence(&self, practice_id: &str, patient_workspace_id: &str) -> Cadence {
        self.patient_link(practice_id, patient_workspace_id)
            .await
            .map(|link| link.cadence)
            .unwrap_or(Cadence::Weekly)
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
    async fn an_issued_token_signs_in_once_and_mints_a_session() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let token = store.begin_sign_in("doc@example.test", NOW).await.unwrap();
        let (identity, session) = store
            .complete_sign_in("doc@example.test", &token, NOW)
            .await
            .unwrap();
        assert_eq!(identity.physician_id, "physician");
        assert_eq!(store.session(&session, NOW).await.unwrap(), identity);
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
                .session(&session, NOW + SESSION_TTL_SECONDS)
                .await
                .is_err(),
            "an expired session stops resolving"
        );
        let (_, session) = signed_in(&store, "doc@example.test").await;
        store.end_session(&session).await;
        assert!(
            store.session(&session, NOW).await.is_err(),
            "sign-out ends the session"
        );
    }

    #[tokio::test]
    async fn deactivation_and_removal_end_every_held_session() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let (_, session) = signed_in(&store, "doc@example.test").await;
        assert!(store.deactivate("doc@example.test").await);
        assert_eq!(
            store.session(&session, NOW).await.unwrap_err(),
            SessionError::Inactive,
            "a deactivated account's session cannot act"
        );
        assert!(store.reactivate("doc@example.test").await);
        assert!(
            store.session(&session, NOW).await.is_err(),
            "sessions ended by deactivation stay ended"
        );
        let (_, session) = signed_in(&store, "doc@example.test").await;
        assert!(store.remove_account("doc@example.test").await);
        assert_eq!(
            store.session(&session, NOW).await.unwrap_err(),
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
            .await;
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
        assert!(store.session(&session, NOW).await.is_ok());
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
    async fn registered_practice_patients_resolve_clinic_demographic_and_store_cadence() {
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

        store.register_patient("clinic-a", "pw-mary").await;
        assert_eq!(
            store.patient_practice("pw-mary").await.as_deref(),
            Some("clinic-a")
        );
        assert_eq!(
            store.practice_patients("clinic-a").await,
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

        // Per-patient cadence defaults to Weekly and updates to WeeklyAndDailyWhenChanged.
        assert_eq!(
            store.patient_cadence("pw-mary").await,
            ReviewCadence::Weekly
        );
        store
            .set_patient_cadence("pw-mary", ReviewCadence::WeeklyAndDailyWhenChanged)
            .await;
        assert_eq!(
            store.patient_cadence("pw-mary").await,
            ReviewCadence::WeeklyAndDailyWhenChanged
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
        assert_eq!(initial.cadence, Cadence::Weekly);

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
    async fn patient_cadence_exposes_only_weekly_or_daily_when_changed_to_agents() {
        let store = MemoryAuthStore::new();
        assert_eq!(
            store.patient_link_cadence("practice-1", "pw-elena").await,
            Cadence::Weekly,
            "default roster cadence is Weekly"
        );

        store
            .set_patient_premium("practice-1", "pw-elena", true)
            .await;
        assert_eq!(
            store.patient_link_cadence("practice-1", "pw-elena").await,
            Cadence::DailyWhenChanged
        );
        let serialized = serde_json::to_string(
            &store
                .patient_link("practice-1", "pw-elena")
                .await
                .expect("link"),
        )
        .unwrap();
        assert!(serialized.contains("\"daily_when_changed\""));
        assert!(
            !serialized.contains("premium") && !serialized.contains("billing"),
            "serialized link exposes cadence only, never billing or payment state"
        );

        store
            .set_patient_premium("practice-1", "pw-elena", false)
            .await;
        assert_eq!(
            store.patient_link_cadence("practice-1", "pw-elena").await,
            Cadence::Weekly
        );
    }
}
