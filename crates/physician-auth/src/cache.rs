//! Cached authority reads, invalidated by the database's change feed.
//!
//! Every physician request resolves its session and workspace membership,
//! and every open event stream re-checks both. Without a cache each check is
//! a Postgres round trip. [`CachedAuthStore`] answers repeated reads from
//! process memory and drops everything it holds when the authority tables
//! change:
//!
//! - a revocation through this store clears the cache before it returns, so a
//!   replica reads its own writes. An idle slide drops only that session;
//! - a revocation from any replica fires the `physician_authority` trigger,
//!   and the [`follow`] task's `LISTEN` turns it into a new feed generation.
//!   A session update is an idle slide and does not notify;
//! - while that listener is disconnected the feed is not live and every read
//!   goes to the database, because notifications sent during the gap are lost;
//! - every entry also expires after [`ENTRY_TTL`], bounding staleness should a
//!   dead connection go unnoticed.
//!
//! Publication is never answered from cache: [`AuthStore::publish_authorized`]
//! passes through to the store's row-locked fence.

use std::collections::HashMap;
use std::convert::Infallible;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    Account, AuthStore, AuthorityFeed, Cadence, Membership, PanelError, PatientLink,
    PhysicianIdentity, PracticeClock, ReviewCadence, Session, SessionActivity, SessionError,
    SessionRenewal, SignInError, StoreUnavailable, token_hash,
};

/// The Postgres channel the authority triggers notify.
pub const AUTHORITY_CHANNEL: &str = "physician_authority";
/// How long one cached answer may be served without any change notice.
pub const ENTRY_TTL: Duration = Duration::from_secs(30);
/// Entries per map before the map is cleared; bounds memory under churn.
const MAX_ENTRIES: usize = 10_000;
/// The wait before a dropped listener reconnects.
const RECONNECT_DELAY: Duration = Duration::from_secs(1);

type Entry<V> = (V, Instant);

#[derive(Default)]
struct Cache {
    /// The feed generation and local write epoch the entries were read at.
    stamp: (u64, u64),
    local_epoch: u64,
    sessions: HashMap<[u8; 32], Entry<Session>>,
    memberships: HashMap<(PhysicianIdentity, String), Entry<Membership>>,
    panels: HashMap<PhysicianIdentity, Entry<Vec<String>>>,
    accounts: HashMap<PhysicianIdentity, Entry<Option<Account>>>,
    practices: HashMap<String, Entry<Option<String>>>,
    practice_patients: HashMap<String, Entry<Vec<String>>>,
    cadences: HashMap<String, Entry<ReviewCadence>>,
}

impl Cache {
    fn clear(&mut self) {
        self.sessions.clear();
        self.memberships.clear();
        self.panels.clear();
        self.accounts.clear();
        self.practices.clear();
        self.practice_patients.clear();
        self.cadences.clear();
    }

    /// Drops every entry read under an older stamp.
    fn align(&mut self, generation: u64) {
        let stamp = (generation, self.local_epoch);
        if self.stamp != stamp {
            self.clear();
            self.stamp = stamp;
        }
    }
}

/// An [`AuthStore`] that answers authority reads from memory between changes.
pub struct CachedAuthStore {
    inner: Arc<dyn AuthStore>,
    feed: watch::Receiver<AuthorityFeed>,
    cache: Mutex<Cache>,
}

impl CachedAuthStore {
    /// Caches `inner`'s reads while `feed` is live.
    pub fn new(inner: Arc<dyn AuthStore>, feed: watch::Receiver<AuthorityFeed>) -> Self {
        Self {
            inner,
            feed,
            cache: Mutex::new(Cache::default()),
        }
    }

    /// Caches a Postgres store, following its trigger notifications on one
    /// pooled connection. The listener stops when the cache is dropped.
    pub fn postgres(store: crate::PostgresAuthStore) -> Self {
        let (tx, rx) = watch::channel(AuthorityFeed::default());
        tokio::spawn(follow(store.pool().clone(), tx));
        Self::new(Arc::new(store), rx)
    }

    /// Clears every entry after a write through this store.
    fn invalidate(&self) {
        let mut cache = self.cache.lock().expect("authority cache");
        cache.local_epoch += 1;
        cache.clear();
    }

    async fn read<K, V, F>(
        &self,
        map: fn(&mut Cache) -> &mut HashMap<K, Entry<V>>,
        key: K,
        load: F,
    ) -> V
    where
        K: Eq + Hash,
        V: Clone,
        F: std::future::Future<Output = V>,
    {
        let Ok(value) = self
            .read_fallible(map, key, async { Ok::<_, Infallible>(load.await) })
            .await;
        value
    }

    /// Reads through the cache and keeps only answers: a failed read is
    /// returned and never cached.
    async fn read_fallible<K, V, E, F>(
        &self,
        map: fn(&mut Cache) -> &mut HashMap<K, Entry<V>>,
        key: K,
        load: F,
    ) -> Result<V, E>
    where
        K: Eq + Hash,
        V: Clone,
        F: std::future::Future<Output = Result<V, E>>,
    {
        let before = *self.feed.borrow();
        let epoch = {
            let mut cache = self.cache.lock().expect("authority cache");
            if before.live {
                cache.align(before.generation);
                if let Some((value, at)) = map(&mut cache).get(&key)
                    && at.elapsed() < ENTRY_TTL
                {
                    return Ok(value.clone());
                }
            }
            cache.local_epoch
        };
        let value = load.await?;
        let after = *self.feed.borrow();
        if before.live && after == before {
            let mut cache = self.cache.lock().expect("authority cache");
            // A change that landed during the read moved the stamp; the value
            // may predate it, so it is returned but not kept.
            if cache.local_epoch == epoch {
                cache.align(after.generation);
                let entries = map(&mut cache);
                if entries.len() >= MAX_ENTRIES {
                    entries.clear();
                }
                entries.insert(key, (value.clone(), Instant::now()));
            }
        }
        Ok(value)
    }

    async fn read_session(&self, token: &str, now: u64) -> Result<Session, SessionError> {
        let before = *self.feed.borrow();
        let hash = token_hash(token);
        let epoch = {
            let mut cache = self.cache.lock().expect("authority cache");
            if before.live {
                cache.align(before.generation);
                if let Some((session, at)) = cache.sessions.get(&hash)
                    && at.elapsed() < ENTRY_TTL
                {
                    return if session.lifetime.allows(now) {
                        Ok(session.clone())
                    } else {
                        Err(SessionError::Invalid)
                    };
                }
            }
            cache.local_epoch
        };
        // Failures are not cached: an unknown token must not occupy memory.
        let session = self.inner.verified_session(token, now).await?;
        let after = *self.feed.borrow();
        if before.live && after == before {
            let mut cache = self.cache.lock().expect("authority cache");
            if cache.local_epoch == epoch {
                cache.align(after.generation);
                if cache.sessions.len() >= MAX_ENTRIES {
                    cache.sessions.clear();
                }
                cache
                    .sessions
                    .insert(hash, (session.clone(), Instant::now()));
            }
        }
        Ok(session)
    }
}

/// Follows the authority channel and publishes one generation per change.
///
/// The feed is live only while `LISTEN` holds; each (re)connect starts a new
/// generation, so nothing read before the gap survives it.
pub async fn follow(pool: sea_orm::sqlx::PgPool, tx: watch::Sender<AuthorityFeed>) {
    use sea_orm::sqlx::postgres::PgListener;
    loop {
        let listening = async {
            let mut listener = PgListener::connect_with(&pool).await?;
            listener.eager_reconnect(false);
            listener.listen(AUTHORITY_CHANNEL).await?;
            Ok::<_, sea_orm::sqlx::Error>(listener)
        };
        let mut listener = tokio::select! {
            _ = tx.closed() => return,
            listening = listening => match listening {
                Ok(listener) => listener,
                Err(error) => {
                    tracing::warn!(error = %error, "physician authority listener unavailable");
                    tokio::time::sleep(RECONNECT_DELAY).await;
                    continue;
                }
            },
        };
        tx.send_modify(|feed| {
            feed.generation += 1;
            feed.live = true;
        });
        loop {
            tokio::select! {
                _ = tx.closed() => return,
                received = listener.try_recv() => match received {
                    Ok(Some(_)) => tx.send_modify(|feed| feed.generation += 1),
                    Ok(None) => break,
                    Err(error) => {
                        tracing::warn!(error = %error, "physician authority listener lost");
                        break;
                    }
                },
            }
        }
        tx.send_modify(|feed| {
            feed.generation += 1;
            feed.live = false;
        });
        tokio::time::sleep(RECONNECT_DELAY).await;
    }
}

#[async_trait]
impl AuthStore for CachedAuthStore {
    fn authority_changes(&self) -> Option<watch::Receiver<AuthorityFeed>> {
        Some(self.feed.clone())
    }

    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError> {
        self.inner.begin_sign_in(email, now).await
    }

    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError> {
        let result = self.inner.complete_sign_in(email, token, now).await;
        self.invalidate();
        result
    }

    async fn verified_session(&self, token: &str, now: u64) -> Result<Session, SessionError> {
        self.read_session(token, now).await
    }

    async fn note_physician_activity(
        &self,
        token: &str,
        now: u64,
    ) -> Result<SessionActivity, StoreUnavailable> {
        let hash = token_hash(token);
        let before = *self.feed.borrow();
        let cached = {
            let mut cache = self.cache.lock().expect("authority cache");
            if before.live {
                cache.align(before.generation);
                cache.sessions.get(&hash).and_then(|(session, at)| {
                    (at.elapsed() < ENTRY_TTL).then_some(session.lifetime)
                })
            } else {
                None
            }
        };
        if let Some(lifetime) = cached {
            match lifetime.renew(now) {
                SessionRenewal::Covered => return Ok(SessionActivity::Covered),
                SessionRenewal::Ended => return Ok(SessionActivity::Ended),
                SessionRenewal::Extend { .. } => {}
            }
        }
        let activity = self.inner.note_physician_activity(token, now).await?;
        if activity == SessionActivity::Extended {
            // An idle slide is not a revocation, so the other authority maps stay.
            let mut cache = self.cache.lock().expect("authority cache");
            cache.sessions.remove(&hash);
        }
        Ok(activity)
    }

    async fn publish_authorized(
        &self,
        token: &str,
        identity: &PhysicianIdentity,
        workspace: &str,
        now: u64,
        publish: &mut (dyn FnMut() -> Result<(), String> + Send),
    ) -> Result<(), String> {
        self.inner
            .publish_authorized(token, identity, workspace, now, publish)
            .await
    }

    async fn end_session(&self, token: &str) -> Result<(), StoreUnavailable> {
        let result = self.inner.end_session(token).await;
        self.invalidate();
        result
    }

    async fn account_by_email(&self, email: &str) -> Option<Account> {
        self.inner.account_by_email(email).await
    }

    async fn account_by_identity(&self, identity: &PhysicianIdentity) -> Option<Account> {
        self.read(
            |cache| &mut cache.accounts,
            identity.clone(),
            self.inner.account_by_identity(identity),
        )
        .await
    }

    async fn membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Membership {
        self.read(
            |cache| &mut cache.memberships,
            (identity.clone(), patient_workspace_id.to_owned()),
            self.inner.membership(identity, patient_workspace_id),
        )
        .await
    }

    async fn set_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), StoreUnavailable> {
        let written = self
            .inner
            .set_membership(identity, patient_workspace_id, membership)
            .await;
        self.invalidate();
        written
    }

    async fn place_on_panel(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable> {
        let written = self
            .inner
            .place_on_panel(identity, patient_workspace_id)
            .await;
        self.invalidate();
        written
    }

    async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String> {
        self.read(
            |cache| &mut cache.panels,
            identity.clone(),
            self.inner.panel(identity),
        )
        .await
    }

    async fn assign_panel(
        &self,
        actor: &PhysicianIdentity,
        physician_id: &str,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), PanelError> {
        let result = self
            .inner
            .assign_panel(actor, physician_id, patient_workspace_id, membership)
            .await;
        self.invalidate();
        result
    }

    async fn ensure_practice(
        &self,
        practice_id: &str,
        clock: PracticeClock,
    ) -> Result<(), StoreUnavailable> {
        self.inner.ensure_practice(practice_id, clock).await
    }

    async fn practice_clock(&self, practice_id: &str) -> Result<PracticeClock, StoreUnavailable> {
        self.inner.practice_clock(practice_id).await
    }

    async fn register_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable> {
        let written = self
            .inner
            .register_patient(practice_id, patient_workspace_id)
            .await;
        self.invalidate();
        written
    }

    async fn patient_practice(
        &self,
        patient_workspace_id: &str,
    ) -> Result<Option<String>, StoreUnavailable> {
        self.read_fallible(
            |cache| &mut cache.practices,
            patient_workspace_id.to_owned(),
            self.inner.patient_practice(patient_workspace_id),
        )
        .await
    }

    async fn practice_patients(&self, practice_id: &str) -> Vec<String> {
        self.read(
            |cache| &mut cache.practice_patients,
            practice_id.to_owned(),
            self.inner.practice_patients(practice_id),
        )
        .await
    }

    async fn practice_accounts(&self, practice_id: &str) -> Result<Vec<Account>, StoreUnavailable> {
        self.inner.practice_accounts(practice_id).await
    }

    async fn patient_cadence(&self, patient_workspace_id: &str) -> ReviewCadence {
        self.read(
            |cache| &mut cache.cadences,
            patient_workspace_id.to_owned(),
            self.inner.patient_cadence(patient_workspace_id),
        )
        .await
    }

    async fn set_patient_cadence(&self, patient_workspace_id: &str, cadence: ReviewCadence) {
        self.inner
            .set_patient_cadence(patient_workspace_id, cadence)
            .await;
        self.invalidate();
    }

    async fn invite(
        &self,
        practice_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<String, SignInError> {
        self.inner
            .invite(practice_id, email, display_name, admin, now)
            .await
    }

    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError> {
        let result = self.inner.redeem_invitation(email, token, now).await;
        self.invalidate();
        result
    }

    async fn deactivate(&self, email: &str) -> bool {
        let done = self.inner.deactivate(email).await;
        self.invalidate();
        done
    }

    async fn reactivate(&self, email: &str) -> bool {
        let done = self.inner.reactivate(email).await;
        self.invalidate();
        done
    }

    async fn remove_account(&self, email: &str) -> bool {
        let done = self.inner.remove_account(email).await;
        self.invalidate();
        done
    }

    async fn invite_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<String, SignInError> {
        self.inner
            .invite_patient(practice_id, patient_workspace_id, now)
            .await
    }

    async fn accept_patient_invitation(
        &self,
        token: &str,
        duet_account_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        self.inner
            .accept_patient_invitation(token, duet_account_id, now)
            .await
    }

    async fn grant_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        duet_account_id: &str,
        consent_version: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        self.inner
            .grant_patient_consent(
                practice_id,
                patient_workspace_id,
                duet_account_id,
                consent_version,
                now,
            )
            .await
    }

    async fn revoke_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        self.inner
            .revoke_patient_consent(practice_id, patient_workspace_id, now)
            .await
    }

    async fn set_patient_premium(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        premium: bool,
    ) {
        self.inner
            .set_patient_premium(practice_id, patient_workspace_id, premium)
            .await;
        // The memory store also moves the review cadence this cache holds.
        self.invalidate();
    }

    async fn patient_link(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Option<PatientLink> {
        self.inner
            .patient_link(practice_id, patient_workspace_id)
            .await
    }

    async fn patient_link_cadence(&self, practice_id: &str, patient_workspace_id: &str) -> Cadence {
        self.inner
            .patient_link_cadence(practice_id, patient_workspace_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryAuthStore;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    const NOW: u64 = 1_760_000_000;

    /// The memory store, counting the authority reads that reach it. While
    /// `unavailable` is set, a patient's practice cannot be read.
    #[derive(Default)]
    struct Counting {
        inner: MemoryAuthStore,
        reads: AtomicUsize,
        notes: AtomicUsize,
        unavailable: AtomicBool,
    }

    impl std::ops::Deref for Counting {
        type Target = MemoryAuthStore;
        fn deref(&self) -> &MemoryAuthStore {
            &self.inner
        }
    }

    #[async_trait]
    impl AuthStore for Counting {
        async fn verified_session(&self, token: &str, now: u64) -> Result<Session, SessionError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            self.inner.verified_session(token, now).await
        }
        async fn note_physician_activity(
            &self,
            token: &str,
            now: u64,
        ) -> Result<crate::SessionActivity, StoreUnavailable> {
            self.notes.fetch_add(1, Ordering::SeqCst);
            self.inner.note_physician_activity(token, now).await
        }
        async fn membership(&self, identity: &PhysicianIdentity, workspace: &str) -> Membership {
            self.reads.fetch_add(1, Ordering::SeqCst);
            self.inner.membership(identity, workspace).await
        }
        async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            self.inner.panel(identity).await
        }
        async fn begin_sign_in(&self, a: &str, b: u64) -> Result<String, SignInError> {
            self.inner.begin_sign_in(a, b).await
        }
        async fn complete_sign_in(
            &self,
            a: &str,
            b: &str,
            c: u64,
        ) -> Result<(PhysicianIdentity, String), SignInError> {
            self.inner.complete_sign_in(a, b, c).await
        }
        async fn end_session(&self, a: &str) -> Result<(), StoreUnavailable> {
            self.inner.end_session(a).await
        }
        async fn account_by_email(&self, a: &str) -> Option<Account> {
            self.inner.account_by_email(a).await
        }
        async fn account_by_identity(&self, a: &PhysicianIdentity) -> Option<Account> {
            self.inner.account_by_identity(a).await
        }
        async fn set_membership(
            &self,
            a: &PhysicianIdentity,
            b: &str,
            c: Membership,
        ) -> Result<(), StoreUnavailable> {
            self.inner.set_membership(a, b, c).await
        }
        async fn place_on_panel(
            &self,
            a: &PhysicianIdentity,
            b: &str,
        ) -> Result<(), StoreUnavailable> {
            self.inner.place_on_panel(a, b).await
        }
        async fn assign_panel(
            &self,
            a: &PhysicianIdentity,
            b: &str,
            c: &str,
            d: Membership,
        ) -> Result<(), PanelError> {
            self.inner.assign_panel(a, b, c, d).await
        }
        async fn ensure_practice(&self, a: &str, b: PracticeClock) -> Result<(), StoreUnavailable> {
            self.inner.ensure_practice(a, b).await
        }
        async fn practice_clock(&self, a: &str) -> Result<PracticeClock, StoreUnavailable> {
            self.inner.practice_clock(a).await
        }
        async fn register_patient(&self, a: &str, b: &str) -> Result<(), StoreUnavailable> {
            self.inner.register_patient(a, b).await
        }
        async fn patient_practice(&self, a: &str) -> Result<Option<String>, StoreUnavailable> {
            if self.unavailable.load(Ordering::SeqCst) {
                return Err(StoreUnavailable);
            }
            self.inner.patient_practice(a).await
        }
        async fn practice_patients(&self, a: &str) -> Vec<String> {
            self.inner.practice_patients(a).await
        }
        async fn practice_accounts(&self, a: &str) -> Result<Vec<Account>, StoreUnavailable> {
            AuthStore::practice_accounts(&self.inner, a).await
        }
        async fn patient_cadence(&self, a: &str) -> ReviewCadence {
            self.inner.patient_cadence(a).await
        }
        async fn set_patient_cadence(&self, a: &str, b: ReviewCadence) {
            self.inner.set_patient_cadence(a, b).await
        }
        async fn invite(
            &self,
            a: &str,
            b: &str,
            c: &str,
            d: bool,
            e: u64,
        ) -> Result<String, SignInError> {
            self.inner.invite(a, b, c, d, e).await
        }
        async fn redeem_invitation(
            &self,
            a: &str,
            b: &str,
            c: u64,
        ) -> Result<(PhysicianIdentity, String), SignInError> {
            self.inner.redeem_invitation(a, b, c).await
        }
        async fn deactivate(&self, a: &str) -> bool {
            self.inner.deactivate(a).await
        }
        async fn reactivate(&self, a: &str) -> bool {
            self.inner.reactivate(a).await
        }
        async fn remove_account(&self, a: &str) -> bool {
            self.inner.remove_account(a).await
        }
        async fn invite_patient(&self, a: &str, b: &str, c: u64) -> Result<String, SignInError> {
            self.inner.invite_patient(a, b, c).await
        }
        async fn accept_patient_invitation(
            &self,
            a: &str,
            b: &str,
            c: u64,
        ) -> Result<PatientLink, SignInError> {
            self.inner.accept_patient_invitation(a, b, c).await
        }
        async fn grant_patient_consent(
            &self,
            a: &str,
            b: &str,
            c: &str,
            d: &str,
            e: u64,
        ) -> Result<PatientLink, SignInError> {
            self.inner.grant_patient_consent(a, b, c, d, e).await
        }
        async fn revoke_patient_consent(
            &self,
            a: &str,
            b: &str,
            c: u64,
        ) -> Result<PatientLink, SignInError> {
            self.inner.revoke_patient_consent(a, b, c).await
        }
        async fn set_patient_premium(&self, a: &str, b: &str, c: bool) {
            self.inner.set_patient_premium(a, b, c).await
        }
        async fn patient_link(&self, a: &str, b: &str) -> Option<PatientLink> {
            self.inner.patient_link(a, b).await
        }
        async fn patient_link_cadence(&self, a: &str, b: &str) -> Cadence {
            self.inner.patient_link_cadence(a, b).await
        }
    }

    fn fixture() -> (
        Arc<Counting>,
        CachedAuthStore,
        watch::Sender<AuthorityFeed>,
        PhysicianIdentity,
        String,
    ) {
        let counting = Arc::new(Counting::default());
        counting.seed_account("practice", "doc", "doc@example.test", "Doc", true);
        let token = counting
            .seed_session("doc@example.test", NOW)
            .expect("session");
        let (tx, rx) = watch::channel(AuthorityFeed {
            generation: 1,
            live: true,
        });
        let cached = CachedAuthStore::new(counting.clone(), rx);
        let identity = PhysicianIdentity {
            practice_id: "practice".into(),
            physician_id: "doc".into(),
        };
        (counting, cached, tx, identity, token)
    }

    #[tokio::test]
    async fn repeated_reads_reach_the_store_once_between_changes() {
        let (counting, cached, _tx, identity, token) = fixture();
        for _ in 0..5 {
            assert_eq!(cached.identity(&token, NOW).await.unwrap(), identity);
            assert_eq!(
                cached.membership(&identity, "ws-1").await,
                Membership::Denied
            );
            assert!(cached.panel(&identity).await.is_empty());
        }
        assert_eq!(counting.reads.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn a_local_write_is_read_back_immediately() {
        let (_counting, cached, _tx, identity, _token) = fixture();
        assert_eq!(
            cached.membership(&identity, "ws-1").await,
            Membership::Denied
        );
        cached
            .set_membership(&identity, "ws-1", Membership::Allowed)
            .await
            .unwrap();
        assert_eq!(
            cached.membership(&identity, "ws-1").await,
            Membership::Allowed
        );
        assert_eq!(cached.panel(&identity).await, vec!["ws-1".to_owned()]);
    }

    #[tokio::test]
    async fn a_feed_generation_drops_what_another_writer_made_stale() {
        let (counting, cached, tx, _identity, token) = fixture();
        assert!(cached.identity(&token, NOW).await.is_ok());
        // Another replica deactivates the account: the inner store changes
        // without this cache seeing the write, then the trigger notifies.
        counting.inner.deactivate("doc@example.test").await;
        assert!(
            cached.identity(&token, NOW).await.is_ok(),
            "served from cache"
        );
        tx.send_modify(|feed| feed.generation += 1);
        assert_eq!(
            cached.identity(&token, NOW).await,
            Err(SessionError::Inactive)
        );
    }

    #[tokio::test]
    async fn an_offline_feed_reads_through_every_time() {
        let (counting, cached, tx, identity, _token) = fixture();
        tx.send_modify(|feed| feed.live = false);
        for _ in 0..3 {
            cached.panel(&identity).await;
        }
        assert_eq!(counting.reads.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn cached_session_ends_at_its_idle_deadline() {
        let (_, cache, _tx, _, token) = fixture();
        let idle_end = NOW + 12 * 60 * 60;
        assert!(cache.identity(&token, idle_end - 1).await.is_ok());
        assert_eq!(
            cache.identity(&token, idle_end).await,
            Err(SessionError::Invalid)
        );
    }

    #[tokio::test]
    async fn a_covered_note_skips_the_store_and_an_extension_keeps_other_entries() {
        let (counting, cached, _tx, identity, token) = fixture();
        assert!(cached.identity(&token, NOW).await.is_ok());
        assert_eq!(
            cached.membership(&identity, "ws-1").await,
            Membership::Denied
        );
        let reads = counting.reads.load(Ordering::SeqCst);
        let notes = counting.notes.load(Ordering::SeqCst);
        assert_eq!(
            cached.note_physician_activity(&token, NOW).await.unwrap(),
            SessionActivity::Covered
        );
        assert_eq!(
            cached
                .note_physician_activity(&token, NOW + 12 * 60 * 60)
                .await
                .unwrap(),
            SessionActivity::Ended
        );
        assert_eq!(counting.notes.load(Ordering::SeqCst), notes);
        assert_eq!(
            cached
                .note_physician_activity(&token, NOW + 15 * 60)
                .await
                .unwrap(),
            SessionActivity::Extended
        );
        assert_eq!(counting.notes.load(Ordering::SeqCst), notes + 1);
        assert_eq!(
            cached.membership(&identity, "ws-1").await,
            Membership::Denied
        );
        assert_eq!(counting.reads.load(Ordering::SeqCst), reads);
        assert_eq!(
            cached
                .verified_session(&token, NOW)
                .await
                .unwrap()
                .lifetime
                .expires_at(),
            NOW + 15 * 60 + 12 * 60 * 60
        );
    }

    #[tokio::test]
    async fn a_patient_whose_practice_could_not_be_read_is_not_kept_as_unregistered() {
        let (counting, cached, _tx, _identity, _token) = fixture();
        counting.inner.seed_patient("clinic", "ws-1");
        counting.unavailable.store(true, Ordering::SeqCst);
        assert_eq!(cached.patient_practice("ws-1").await, Err(StoreUnavailable));
        counting.unavailable.store(false, Ordering::SeqCst);
        assert_eq!(
            cached.patient_practice("ws-1").await,
            Ok(Some("clinic".to_owned()))
        );
    }

    #[tokio::test]
    async fn failures_are_not_cached() {
        let (counting, cached, _tx, _identity, _token) = fixture();
        for _ in 0..3 {
            assert!(cached.identity("codas-unknown", NOW).await.is_err());
        }
        assert_eq!(counting.reads.load(Ordering::SeqCst), 3);
    }
}
