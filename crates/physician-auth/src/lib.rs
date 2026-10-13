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
use sha2::{Digest, Sha256};
use uuid::Uuid;

mod error;
mod store;

pub use error::{SessionError, SignInError};
pub use store::PostgresAuthStore;

/// The SeaORM entities and the migration chain.
pub mod migration;

/// A sign-in token is verifiable for one day.
pub const SIGN_IN_TOKEN_TTL_SECONDS: u64 = 86_400;
/// A sign-in session is valid for thirty days.
pub const SESSION_TTL_SECONDS: u64 = 30 * 86_400;
/// An invitation is redeemable for fourteen days.
pub const INVITATION_TTL_SECONDS: u64 = 14 * 86_400;

/// A physician's practice-scoped identity, as the projections key it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Whether the account may administer the practice's accounts.
    pub admin: bool,
    /// Sessions minted before this Unix second never resolve; deactivation
    /// moves it forward and reactivation leaves it there.
    pub sessions_ended_before: u64,
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
}

/// The SHA-256 of a presented token: the only form the store keeps.
pub(crate) fn token_hash(token: &str) -> [u8; 32] {
    Sha256::digest(token.as_bytes()).into()
}

/// Mints one presented token. The store keeps only its hash.
pub(crate) fn mint_token() -> String {
    format!("codas-{}", Uuid::new_v4().simple())
}

#[derive(Debug, Default)]
struct MemoryState {
    accounts: HashMap<String, Account>,
    memberships: HashMap<(String, String, String), Membership>,
    sessions: HashMap<[u8; 32], Session>,
    sign_ins: HashMap<String, (SignInPending, ())>,
    invitations: HashMap<String, InvitationPending>,
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

/// The process-memory store. A deployment's SQL store replaces it.
#[derive(Debug, Default)]
pub struct MemoryAuthStore {
    state: Mutex<MemoryState>,
}

impl MemoryAuthStore {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self::default()
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

    async fn end_session(&self, token: &str) {
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
        self.state
            .lock()
            .expect("auth state")
            .accounts
            .values()
            .find(|account| account.identity == *identity)
            .cloned()
    }

    async fn membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Membership {
        self.state
            .lock()
            .expect("auth state")
            .memberships
            .get(&(
                identity.practice_id.clone(),
                identity.physician_id.clone(),
                patient_workspace_id.to_owned(),
            ))
            .copied()
            .unwrap_or(Membership::Denied)
    }

    async fn set_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) {
        self.state.lock().expect("auth state").memberships.insert(
            (
                identity.practice_id.clone(),
                identity.physician_id.clone(),
                patient_workspace_id.to_owned(),
            ),
            membership,
        );
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
        let mut state = self.state.lock().expect("auth state");
        let Some(account) = state.accounts.get_mut(&email.to_lowercase()) else {
            return false;
        };
        account.active = true;
        true
    }

    async fn remove_account(&self, email: &str) -> bool {
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

    #[test]
    fn seeded_sessions_match_the_sign_in_session_shape() {
        let store = MemoryAuthStore::new();
        store.seed_account("practice", "physician", "doc@example.test", "Doc", false);
        let token = store.seed_session("doc@example.test", NOW).unwrap();
        assert!(token.starts_with("codas-"));
    }
}
