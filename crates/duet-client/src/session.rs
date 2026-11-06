//! Account-owned request/cache fences. Credentials never enter this cache or browser storage.
use std::sync::{Arc, Mutex};

/// Opaque owner fence captured before dispatch; delayed responses must present the same fence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionFence {
    owner: String,
    generation: u64,
}
impl SessionFence {
    /// Verified account/incarnation, never a credential.
    pub fn owner(&self) -> &str {
        &self.owner
    }
    /// Monotonic local lifecycle generation for keyed UI remounting.
    pub fn generation(&self) -> u64 {
        self.generation
    }
}
/// Process-local account-owned state. Native credential retention remains behind the platform vault.
#[derive(Clone, Default)]
pub struct AccountSession {
    inner: Arc<Mutex<SessionState>>,
}
#[derive(Default)]
struct SessionState {
    owner: Option<String>,
    generation: u64,
    drafts: std::collections::HashMap<String, Vec<u8>>,
}
impl AccountSession {
    /// Select a verified internal owner. Switching always drops old account drafts and invalidates responses.
    pub fn switch(&self, owner: Option<String>) -> Option<SessionFence> {
        let mut state = self.inner.lock().expect("account session");
        state.generation = state
            .generation
            .checked_add(1)
            .expect("session generation exhausted");
        state.drafts.clear();
        state.owner = owner;
        state.owner.clone().map(|owner| SessionFence {
            owner,
            generation: state.generation,
        })
    }
    /// Capture this before dispatching an authenticated request.
    pub fn fence(&self) -> Option<SessionFence> {
        let state = self.inner.lock().expect("account session");
        state.owner.clone().map(|owner| SessionFence {
            owner,
            generation: state.generation,
        })
    }
    /// Refuse late responses after sign-out or account switch, including switching back to the same owner.
    pub fn is_current(&self, fence: &SessionFence) -> bool {
        self.fence().as_ref() == Some(fence)
    }
    /// End only the session that dispatched the refused request. Durable storage is untouched.
    pub fn invalidate(&self, fence: &SessionFence) -> bool {
        let mut state = self.inner.lock().expect("account session");
        if state.owner.as_deref() != Some(&fence.owner) || state.generation != fence.generation {
            return false;
        }
        state.generation = state
            .generation
            .checked_add(1)
            .expect("session generation exhausted");
        state.owner = None;
        state.drafts.clear();
        true
    }
    /// Accept a response/draft atomically with checking ownership. Never stores bearer credentials.
    pub fn store_draft(&self, fence: &SessionFence, key: String, value: Vec<u8>) -> bool {
        let mut state = self.inner.lock().expect("account session");
        if state.owner.as_deref() != Some(&fence.owner) || state.generation != fence.generation {
            return false;
        }
        state.drafts.insert(key, value);
        true
    }
    /// Read only this session's draft.
    pub fn draft(&self, fence: &SessionFence, key: &str) -> Option<Vec<u8>> {
        let state = self.inner.lock().expect("account session");
        if state.owner.as_deref() != Some(&fence.owner) || state.generation != fence.generation {
            return None;
        }
        state.drafts.get(key).cloned()
    }
}
/// Native hosts implement this with iOS Keychain/Android Keystore. No plaintext fallback is allowed.
/// This interface deliberately does not claim device protection until a host adapter is installed/tested.
pub trait CredentialVault {
    /// Read provider refresh credentials for exactly the verified provider subject.
    fn load(&self, subject: &str) -> Result<Option<Vec<u8>>, String>;
    /// Persist using device-protected non-synchronizing credential storage.
    fn retain(&self, subject: &str, credential: &[u8]) -> Result<(), String>;
    /// Erase only this account's credentials on explicit sign-out/provider-confirmed deletion.
    fn erase(&self, subject: &str) -> Result<(), String>;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn switch_and_signout_fence_delayed_responses_and_clear_drafts() {
        let session = AccountSession::default();
        let a = session.switch(Some("a".into())).unwrap();
        assert!(session.store_draft(&a, "doctor-page".into(), b"private".to_vec()));
        session.switch(Some("b".into()));
        assert!(!session.store_draft(&a, "doctor-page".into(), b"late".to_vec()));
        let again = session.switch(Some("a".into())).unwrap();
        assert_ne!(a, again);
        assert!(session.draft(&again, "doctor-page").is_none());
        session.switch(None);
        assert!(!session.is_current(&again));
    }
}
