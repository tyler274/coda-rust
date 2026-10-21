//! The patient front door: accounts, practice invitations, the
//! Duet-account-to-roster-patient link, consent, and premium status.
//!
//! One durable owner, outside Duet and Agents, for who a patient is, which
//! practice's roster they belong to, whether they consented to share with
//! their physician, and whether they pay for premium. The practice invites
//! the patient explicitly — identity is never guessed from names, phones, or
//! clinical text — and the patient's accepted invitation writes the link.
//! Agents reads only a per-patient cadence setting and never knows who pays;
//! Duet reads only which premium features are on; neither stores billing.
//! Invitations and tokens are stored `sha256`-hashed so the presented token
//! is the only secret, and an invitation redeems exactly once.

#![deny(missing_docs)]

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

mod error;
pub mod http;
mod store;

pub use error::{InviteError, LinkError, SessionError, SignInError};
pub use store::PostgresFrontDoorStore;

/// Durable Identity Platform mapping and Duet permission authority.
pub mod identity;
/// Transactional Luna privacy restrictions and independent receipt reconciliation.
pub mod journal;
/// The SeaORM entities and the migration chain.
pub mod migration;

/// A sign-in token is verifiable for one day.
pub const SIGN_IN_TOKEN_TTL_SECONDS: u64 = 86_400;
/// A sign-in session is valid for thirty days.
pub const SESSION_TTL_SECONDS: u64 = 30 * 86_400;
/// An invitation is redeemable for fourteen days.
pub const INVITATION_TTL_SECONDS: u64 = 14 * 86_400;
/// The cadence Agents runs for every patient: one weekly review.
pub const CADENCE_WEEKLY: &str = "weekly";
/// The cadence for paying patients: the weekly review plus a daily check
/// that runs a full review only when something changed.
pub const CADENCE_WEEKLY_DAILY_CHECK: &str = "weekly-daily-check";

/// Which roster patient an invitation or link names.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RosterPatient {
    /// The practice whose roster the patient belongs to.
    pub practice_id: String,
    /// The patient within the practice's roster.
    pub patient_id: String,
}

/// One patient account at the front door.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    /// The account's id, stable across a re-link.
    pub id: Uuid,
    /// The email sign-in address.
    pub email: String,
    /// The account's display name.
    pub display_name: String,
    /// Whether the account may sign in and hold sessions.
    pub active: bool,
    /// The roster patient the account is linked to, once an invitation the
    /// practice issued has been redeemed. `None` until then.
    pub link: Option<RosterPatient>,
    /// Sessions minted before this Unix second never resolve; deactivation
    /// moves it forward and reactivation leaves it there.
    pub sessions_ended_before: u64,
}

/// One pending invitation a practice issued to a roster patient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invitation {
    /// Which roster patient the practice invited.
    pub patient: RosterPatient,
    /// The lower-cased email the invitation names.
    pub email: String,
    /// The invitee's display name, as the practice wrote it.
    pub display_name: String,
    /// The Unix second the invitation stops redeeming.
    pub expires_at: u64,
}

/// One account's consent to share with their physician.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consent {
    /// The patient consented; sharing is on.
    Granted {
        /// The Unix second consent was given.
        at: u64,
    },
    /// The patient consented and later withdrew it. Stopping sharing is one
    /// step, and what was already shared stays dated evidence.
    Withdrawn {
        /// The Unix second consent was given.
        at: u64,
        /// The Unix second consent was withdrawn.
        withdrawn_at: u64,
    },
    /// The patient was never asked, or never answered.
    None,
}

/// One linked patient's premium status, as the front door owns it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Premium {
    /// Whether the patient pays for premium.
    pub premium: bool,
    /// The cadence Agents runs for the patient. Agents reads only this and
    /// never knows who pays.
    pub cadence: String,
}

/// The durable front-door owner.
///
/// `&self` everywhere so the server can share one store across handlers.
/// `async_trait` keeps the trait object-safe: a server holds
/// `Arc<dyn FrontDoorStore>` and picks the SQL or memory store at startup.
#[async_trait]
pub trait FrontDoorStore: Send + Sync {
    /// Records one invitation to a roster patient and returns its redeem
    /// token. Re-inviting the same email replaces the pending invitation.
    async fn invite(
        &self,
        patient: &RosterPatient,
        email: &str,
        display_name: &str,
        now: u64,
    ) -> Result<String, InviteError>;
    /// Reads one pending invitation by email.
    async fn invitation(&self, email: &str) -> Option<Invitation>;
    /// Redeems an invitation: creates the account, writes the link, and
    /// mints one bearer session. An invitation redeems exactly once.
    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(Uuid, String), SignInError>;
    /// Issues a sign-in token for an existing active account's email.
    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError>;
    /// Verifies a sign-in token and mints one bearer session from it.
    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(Uuid, String), SignInError>;
    /// Resolves a bearer token to its account id.
    async fn session(&self, token: &str, now: u64) -> Result<Uuid, SessionError>;
    /// Ends one session so its token stops resolving.
    async fn end_session(&self, token: &str);
    /// Reads one account by email.
    async fn account_by_email(&self, email: &str) -> Option<Account>;
    /// Reads one account by id.
    async fn account(&self, account_id: Uuid) -> Option<Account>;
    /// Resolves one Duet account's link to its roster patient.
    async fn link(&self, account_id: Uuid) -> Result<RosterPatient, LinkError>;
    /// Lists the front-door accounts linked to one practice's roster
    /// patients, sorted by account id.
    async fn linked_accounts(&self, practice_id: &str) -> Vec<(Account, RosterPatient)>;
    /// Records the patient's consent to share with their physician.
    /// Re-consenting after a withdrawal restores sharing.
    async fn grant_consent(&self, account_id: Uuid, now: u64) -> Result<(), LinkError>;
    /// Withdraws the patient's consent; one step, and what was already
    /// shared stays dated evidence. Consent that was never granted stays so.
    async fn withdraw_consent(&self, account_id: Uuid, now: u64) -> Result<(), LinkError>;
    /// Reads one account's consent state.
    async fn consent(&self, account_id: Uuid) -> Result<Consent, LinkError>;
    /// Sets one linked patient's premium status. The cadence follows the
    /// status: paying patients add the daily check.
    async fn set_premium(
        &self,
        patient: &RosterPatient,
        premium: bool,
        now: u64,
    ) -> Result<(), LinkError>;
    /// Reads one linked patient's premium status. Patients without a status
    /// row read free, with the plain weekly cadence.
    async fn premium(&self, patient: &RosterPatient) -> Premium;
}

/// The SHA-256 of a presented token: the only form the store keeps.
pub(crate) fn token_hash(token: &str) -> [u8; 32] {
    Sha256::digest(token.as_bytes()).into()
}

/// Mints one presented token. The store keeps only its hash.
pub(crate) fn mint_token() -> String {
    format!("codaf-{}", Uuid::new_v4().simple())
}

/// The cadence a premium status implies: paying patients add the daily check.
fn cadence_for(premium: bool) -> &'static str {
    if premium {
        CADENCE_WEEKLY_DAILY_CHECK
    } else {
        CADENCE_WEEKLY
    }
}

#[derive(Debug, Default)]
struct MemoryState {
    accounts: HashMap<Uuid, Account>,
    emails: HashMap<String, Uuid>,
    invitations: HashMap<String, InvitationPending>,
    pending_sign_ins: HashMap<String, PendingSignIn>,
    sessions: HashMap<[u8; 32], (Uuid, u64, u64)>,
    consents: HashMap<Uuid, Consent>,
    premium: HashMap<(String, String), Premium>,
}

#[derive(Debug, Clone)]
struct InvitationPending {
    patient: RosterPatient,
    display_name: String,
    token_hash: [u8; 32],
    expires_at: u64,
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

    /// Seeds one linked account directly, for fixtures and staging setups.
    /// The account signs in through [`FrontDoorStore::begin_sign_in`].
    pub fn seed_linked_account(
        &self,
        patient: &RosterPatient,
        email: &str,
        display_name: &str,
    ) -> Uuid {
        let mut state = self.state.lock().expect("front-door state");
        let id = Uuid::new_v4();
        state.emails.insert(email.to_lowercase(), id);
        state.accounts.insert(
            id,
            Account {
                id,
                email: email.to_owned(),
                display_name: display_name.to_owned(),
                active: true,
                link: Some(patient.clone()),
                sessions_ended_before: 0,
            },
        );
        id
    }

    fn mint_session(&self, state: &mut MemoryState, id: Uuid, now: u64) -> String {
        let token = mint_token();
        let hash = token_hash(&token);
        state
            .sessions
            .insert(hash, (id, now, now + SESSION_TTL_SECONDS));
        token
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
        if state.emails.contains_key(&key) {
            // The email already holds a front-door account.
            return Err(InviteError::Conflict);
        }
        let token = mint_token();
        state.invitations.insert(
            key,
            InvitationPending {
                patient: patient.clone(),
                display_name: display_name.to_owned(),
                token_hash: token_hash(&token),
                expires_at: now + INVITATION_TTL_SECONDS,
            },
        );
        Ok(token)
    }

    async fn invitation(&self, email: &str) -> Option<Invitation> {
        let key = email.to_lowercase();
        let state = self.state.lock().expect("front-door state");
        state.invitations.get(&key).map(|pending| Invitation {
            patient: pending.patient.clone(),
            email: key,
            display_name: pending.display_name.clone(),
            expires_at: pending.expires_at,
        })
    }

    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(Uuid, String), SignInError> {
        let key = email.to_lowercase();
        let mut state = self.state.lock().expect("front-door state");
        if state.emails.contains_key(&key) {
            // The account exists; it signs in rather than redeeming again.
            return Err(SignInError::Invalid);
        }
        // Remove-then-commit: the invitation is consumed once, even if a
        // later step fails, so a raced second redeem finds nothing.
        let Some(pending) = state.invitations.remove(&key) else {
            return Err(SignInError::Invalid);
        };
        if pending.expires_at <= now || pending.token_hash != token_hash(token) {
            return Err(SignInError::Invalid);
        }
        let id = Uuid::new_v4();
        state.emails.insert(key.clone(), id);
        state.accounts.insert(
            id,
            Account {
                id,
                email: key,
                display_name: pending.display_name.clone(),
                active: true,
                link: Some(pending.patient.clone()),
                sessions_ended_before: 0,
            },
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
        // Remove-then-commit: the pending token is consumed once, even if a
        // later step fails, so a raced second verify cannot mint two.
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

    async fn session(&self, token: &str, now: u64) -> Result<Uuid, SessionError> {
        let hash = token_hash(token);
        let state = self.state.lock().expect("front-door state");
        let (id, created_at, expires_at) = state
            .sessions
            .get(&hash)
            .copied()
            .ok_or(SessionError::Invalid)?;
        if expires_at <= now {
            return Err(SessionError::Invalid);
        }
        let account = state.accounts.get(&id).ok_or(SessionError::Inactive)?;
        if !account.active {
            return Err(SessionError::Inactive);
        }
        if created_at < account.sessions_ended_before {
            // Ended by deactivation: the account is active again, this
            // session still is not.
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
        state.accounts.get(&id).cloned()
    }

    async fn account(&self, account_id: Uuid) -> Option<Account> {
        self.state
            .lock()
            .expect("front-door state")
            .accounts
            .get(&account_id)
            .cloned()
    }

    async fn link(&self, account_id: Uuid) -> Result<RosterPatient, LinkError> {
        let state = self.state.lock().expect("front-door state");
        state
            .accounts
            .get(&account_id)
            .and_then(|account| account.link.clone())
            .ok_or(LinkError::Unlinked)
    }

    async fn linked_accounts(&self, practice_id: &str) -> Vec<(Account, RosterPatient)> {
        let state = self.state.lock().expect("front-door state");
        let mut rows: Vec<(Account, RosterPatient)> = state
            .accounts
            .values()
            .filter_map(|account| {
                let patient = account.link.clone()?;
                (patient.practice_id == practice_id).then(|| (account.clone(), patient))
            })
            .collect();
        rows.sort_by_key(|(left, _)| left.id);
        rows
    }

    async fn grant_consent(&self, account_id: Uuid, now: u64) -> Result<(), LinkError> {
        let mut state = self.state.lock().expect("front-door state");
        if state
            .accounts
            .get(&account_id)
            .ok_or(LinkError::Unlinked)?
            .link
            .is_none()
        {
            return Err(LinkError::Unlinked);
        }
        state
            .consents
            .insert(account_id, Consent::Granted { at: now });
        Ok(())
    }

    async fn withdraw_consent(&self, account_id: Uuid, now: u64) -> Result<(), LinkError> {
        let mut state = self.state.lock().expect("front-door state");
        if state
            .accounts
            .get(&account_id)
            .ok_or(LinkError::Unlinked)?
            .link
            .is_none()
        {
            return Err(LinkError::Unlinked);
        }
        if let Some(Consent::Granted { at }) = state.consents.get(&account_id).copied() {
            state.consents.insert(
                account_id,
                Consent::Withdrawn {
                    at,
                    withdrawn_at: now,
                },
            );
        }
        Ok(())
    }

    async fn consent(&self, account_id: Uuid) -> Result<Consent, LinkError> {
        let state = self.state.lock().expect("front-door state");
        if state
            .accounts
            .get(&account_id)
            .ok_or(LinkError::Unlinked)?
            .link
            .is_none()
        {
            return Err(LinkError::Unlinked);
        }
        Ok(state
            .consents
            .get(&account_id)
            .copied()
            .unwrap_or(Consent::None))
    }

    async fn set_premium(
        &self,
        patient: &RosterPatient,
        premium: bool,
        _now: u64,
    ) -> Result<(), LinkError> {
        let mut state = self.state.lock().expect("front-door state");
        if !state
            .accounts
            .values()
            .any(|account| account.link.as_ref() == Some(patient))
        {
            return Err(LinkError::Unlinked);
        }
        state.premium.insert(
            (patient.practice_id.clone(), patient.patient_id.clone()),
            Premium {
                premium,
                cadence: cadence_for(premium).to_owned(),
            },
        );
        Ok(())
    }

    async fn premium(&self, patient: &RosterPatient) -> Premium {
        let state = self.state.lock().expect("front-door state");
        state
            .premium
            .get(&(patient.practice_id.clone(), patient.patient_id.clone()))
            .cloned()
            .unwrap_or(Premium {
                premium: false,
                cadence: cadence_for(false).to_owned(),
            })
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
    async fn an_invitation_redeems_once_and_mints_one_session() {
        let store = MemoryFrontDoorStore::new();
        let patient = patient("practice", "patient-9");
        let token = store
            .invite(&patient, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        let (id, session) = store
            .redeem_invitation("luna@example.test", &token, NOW)
            .await
            .unwrap();
        assert_eq!(store.session(&session, NOW).await.unwrap(), id);
        let account = store.account_by_email("luna@example.test").await.unwrap();
        assert_eq!(account.link, Some(patient));
        assert!(account.active);
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
    async fn an_expired_or_wrong_invitation_token_is_invalid() {
        let store = MemoryFrontDoorStore::new();
        let patient = patient("practice", "patient-9");
        let token = store
            .invite(&patient, "luna@example.test", "Luna Park", NOW)
            .await
            .unwrap();
        assert_eq!(
            store
                .redeem_invitation("luna@example.test", &token, NOW + INVITATION_TTL_SECONDS)
                .await
                .unwrap_err(),
            SignInError::Invalid,
            "an invitation stops redeeming after fourteen days"
        );
        let token = store
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
        let _ = token;
    }

    #[tokio::test]
    async fn reinviting_replaces_the_pending_invitation() {
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
    async fn an_email_with_an_account_cannot_be_invited_or_redeem() {
        let store = MemoryFrontDoorStore::new();
        let (patient, id, _) = invited(&store, "luna@example.test").await;
        assert_eq!(
            store
                .invite(&patient, "luna@example.test", "Luna Park", NOW)
                .await
                .unwrap_err(),
            InviteError::Conflict,
            "the practice cannot re-invite a linked account"
        );
        // A second invitation for another email that collides on redemption
        // burns neither store state nor the first account.
        let token = store
            .invite(&patient, "other@example.test", "Other", NOW)
            .await
            .unwrap();
        store
            .redeem_invitation("luna@example.test", &token, NOW)
            .await
            .unwrap_err();
        assert_eq!(store.account(id).await.unwrap().link, Some(patient));
    }

    #[tokio::test]
    async fn a_sign_in_token_verifies_once_and_mints_one_session() {
        let store = MemoryFrontDoorStore::new();
        let (_, id, _) = invited(&store, "luna@example.test").await;
        let token = store.begin_sign_in("luna@example.test", NOW).await.unwrap();
        let (who, session) = store
            .complete_sign_in("luna@example.test", &token, NOW)
            .await
            .unwrap();
        assert_eq!(who, id);
        assert_eq!(store.session(&session, NOW).await.unwrap(), id);
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
    async fn an_expired_sign_in_token_and_session_stop_resolving() {
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
        // The consumed pending token cannot verify again, even within its TTL.
        assert_eq!(
            store
                .complete_sign_in("luna@example.test", &token, NOW)
                .await
                .unwrap_err(),
            SignInError::Invalid
        );
        let token = store.begin_sign_in("luna@example.test", NOW).await.unwrap();
        let (_, session) = store
            .complete_sign_in("luna@example.test", &token, NOW)
            .await
            .unwrap();
        assert_eq!(
            store
                .session(&session, NOW + SESSION_TTL_SECONDS)
                .await
                .unwrap_err(),
            SessionError::Invalid,
            "a session is valid for thirty days"
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
    async fn the_link_resolves_the_roster_patient_and_lists_per_practice() {
        let store = MemoryFrontDoorStore::new();
        let (luna, id, _) = invited(&store, "luna@example.test").await;
        assert_eq!(store.link(id).await.unwrap(), luna);
        let other = store.seed_linked_account(
            &patient("roger-synthetic-practice", "patient-2"),
            "sofia@example.test",
            "Sofia Reyes",
        );
        let mut linked = store
            .linked_accounts("roger-synthetic-practice")
            .await
            .into_iter()
            .map(|(account, _)| account.id)
            .collect::<Vec<_>>();
        linked.sort();
        let mut expected = vec![id, other];
        expected.sort();
        assert_eq!(linked, expected);
        assert!(store.linked_accounts("other-practice").await.is_empty());
        assert_eq!(
            store.link(Uuid::nil()).await.unwrap_err(),
            LinkError::Unlinked,
            "an unknown account carries no link"
        );
    }

    #[tokio::test]
    async fn consent_reads_none_grants_and_withdraws_in_one_step() {
        let store = MemoryFrontDoorStore::new();
        let (_, id, _) = invited(&store, "luna@example.test").await;
        assert_eq!(
            store.consent(id).await.unwrap(),
            Consent::None,
            "consent is granted, never assumed"
        );
        store.grant_consent(id, NOW + 1).await.unwrap();
        assert_eq!(
            store.consent(id).await.unwrap(),
            Consent::Granted { at: NOW + 1 }
        );
        store.withdraw_consent(id, NOW + 2).await.unwrap();
        assert_eq!(
            store.consent(id).await.unwrap(),
            Consent::Withdrawn {
                at: NOW + 1,
                withdrawn_at: NOW + 2,
            },
            "what was already shared stays dated evidence"
        );
        store.grant_consent(id, NOW + 3).await.unwrap();
        assert_eq!(
            store.consent(id).await.unwrap(),
            Consent::Granted { at: NOW + 3 },
            "re-consenting restores sharing"
        );
        assert_eq!(
            store
                .withdraw_consent(Uuid::nil(), NOW + 4)
                .await
                .unwrap_err(),
            LinkError::Unlinked,
            "an unknown account cannot hold consent"
        );
    }

    #[tokio::test]
    async fn premium_defaults_free_and_follows_the_status() {
        let store = MemoryFrontDoorStore::new();
        let (luna, _, _) = invited(&store, "luna@example.test").await;
        assert_eq!(
            store.premium(&luna).await,
            Premium {
                premium: false,
                cadence: CADENCE_WEEKLY.to_owned(),
            },
            "a patient without a status row reads free"
        );
        store.set_premium(&luna, true, NOW).await.unwrap();
        assert_eq!(
            store.premium(&luna).await,
            Premium {
                premium: true,
                cadence: CADENCE_WEEKLY_DAILY_CHECK.to_owned(),
            },
            "paying patients add the daily check"
        );
        store.set_premium(&luna, false, NOW).await.unwrap();
        assert_eq!(
            store.premium(&luna).await,
            Premium {
                premium: false,
                cadence: CADENCE_WEEKLY.to_owned(),
            }
        );
        assert_eq!(
            store
                .set_premium(&patient("practice", "nobody"), true, NOW)
                .await
                .unwrap_err(),
            LinkError::Unlinked,
            "premium status is written only for a linked patient"
        );
    }
}
