//! Protected, account-owned retention. No browser or plaintext fallback.
use crate::session::{AccountSession, CredentialVault, SessionFence};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Device-protected storage for credentials and health drafts, in separate namespaces.
pub trait ProtectedStore: Send + Sync {
    /// Missing is distinct from locked, damaged or unavailable storage.
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String>;
    /// Atomically replace an item; never remove the old value before a successful replacement.
    fn write(&self, key: &str, bytes: &[u8]) -> Result<(), String>;
    /// Explicitly erase an item. Missing items count as erased.
    fn remove(&self, key: &str) -> Result<(), String>;
    /// Nonsecret durable deny fence, available even while device-protected secrets are locked.
    /// It contains only a boolean, never an account ID, token, draft or other patient information.
    fn cleanup_pending(&self) -> Result<bool, String> {
        Ok(self.read("cleanup-pending")?.is_some())
    }
    /// Persist the deny fence before any credential deletion or account replacement.
    fn set_cleanup_pending(&self, pending: bool) -> Result<(), String> {
        if pending {
            self.write("cleanup-pending", b"1")
        } else {
            self.remove("cleanup-pending")
        }
    }
}

/// Explicit volatile browser session store. It is NOT durable storage and never touches localStorage.
#[derive(Default)]
pub struct VolatileSessionStore(Mutex<std::collections::HashMap<String, Vec<u8>>>);
impl ProtectedStore for VolatileSessionStore {
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        Ok(self.0.lock().unwrap().get(key).cloned())
    }
    fn write(&self, key: &str, bytes: &[u8]) -> Result<(), String> {
        self.0.lock().unwrap().insert(key.into(), bytes.into());
        Ok(())
    }
    fn remove(&self, key: &str) -> Result<(), String> {
        self.0.lock().unwrap().remove(key);
        Ok(())
    }
}
/// Native Keychain/Keystore adapter. Other platforms fail closed.
#[derive(Clone, Copy, Default)]
pub struct NativeProtectedStore;
impl ProtectedStore for NativeProtectedStore {
    fn cleanup_pending(&self) -> Result<bool, String> {
        platform_call(serde_json::json!({"op":"cleanup_read"}))?
            .as_bool()
            .ok_or("Invalid cleanup fence".into())
    }
    fn set_cleanup_pending(&self, pending: bool) -> Result<(), String> {
        platform_call(serde_json::json!({"op":"cleanup_write", "pending":pending})).map(|_| ())
    }
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        let result = platform_call(serde_json::json!({"op":"read","key":key}))?;
        serde_json::from_value(result)
            .map_err(|_| "Protected storage returned invalid bytes".into())
    }
    fn write(&self, key: &str, bytes: &[u8]) -> Result<(), String> {
        platform_call(serde_json::json!({"op":"write","key":key,"bytes":bytes})).map(|_| ())
    }
    fn remove(&self, key: &str) -> Result<(), String> {
        platform_call(serde_json::json!({"op":"remove","key":key})).map(|_| ())
    }
}
impl CredentialVault for NativeProtectedStore {
    fn load(&self, subject: &str) -> Result<Option<Vec<u8>>, String> {
        self.read(&key("credential", subject))
    }
    fn retain(&self, subject: &str, credential: &[u8]) -> Result<(), String> {
        self.write(&key("credential", subject), credential)
    }
    fn erase(&self, subject: &str) -> Result<(), String> {
        self.remove(&key("credential", subject))
    }
}
fn key(kind: &str, owner: &str) -> String {
    format!("{kind}:{}:{owner}", owner.len())
}
/// Owner-scoped health outbox key. The scope is length-prefixed so no
/// (owner, scope) pair can collide with another.
fn health_key(owner: &str, scope: &str) -> String {
    format!("{}:{}:{scope}", key("health-outbox", owner), scope.len())
}
/// Scopes written for one owner, so sign-out can erase them without listing the store.
fn health_scopes_key(owner: &str) -> String {
    key("health-scopes", owner)
}

/// Storage failures shared with the health outbox.
pub use duet_health::outbox::StorageError as HealthStorageError;

/// Provider refresh credential and its verified Duet owner. Never Debug/log this type.
#[derive(Clone, Serialize, Deserialize)]
pub struct RetainedIdentity {
    /// Identity Platform subject, verified by the account endpoint.
    pub subject: String,
    /// Internal UUID plus incarnation, not email or provider subject.
    pub owner: String,
    /// Refresh token; ID tokens are held only in memory.
    pub refresh_token: String,
}

/// Exact local edit and the base it was edited against. Approval is a separate server action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoctorDraft {
    /// Exact collection base and item labels, retained for offline editing, never treated as current server evidence.
    pub base: duet_wire::models::DoctorCollection,
    /// Existing save DTO, including generation and proposal fences.
    pub save: duet_wire::models::DoctorPageSave,
    /// A save with an uncertain acknowledgement must first reconcile exact readback.
    pub uncertain_save: bool,
}
/// Exact readback outcome for an uncertain save. No inferred approval or silent rebasing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveReadback {
    /// The next generation contains precisely the submitted page and selection.
    Saved,
    /// The exact original base is still current; explicit retry can reuse the same request.
    RetryOriginal,
    /// The server advanced differently. Keep the local edit for explicit review.
    Conflict,
}
impl DoctorDraft {
    /// Compare complete saved-page content and ordered membership, never a summary/hash alone.
    pub fn reconcile(&self, current: &duet_wire::models::DoctorCollection) -> SaveReadback {
        let ids: Vec<_> = current.items.iter().flatten().map(|item| item.id).collect();
        if self.save.expected_generation.checked_add(1) == Some(current.note_generation)
            && current.page.as_deref() == Some(self.save.page.as_ref())
            && current.note == self.save.note
            && ids == self.save.item_ids
            && current.items.iter().flatten().all(|item| {
                item.medication_confirmed.unwrap_or(false)
                    == self
                        .save
                        .confirmed_medication_ids
                        .as_ref()
                        .is_some_and(|ids| ids.contains(&item.id))
            })
        {
            SaveReadback::Saved
        } else if current == &self.base {
            SaveReadback::RetryOriginal
        } else {
            SaveReadback::Conflict
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Envelope {
    version: u32,
    owner: String,
    draft: DoctorDraft,
}

/// Recovery never silently overwrites damaged bytes or a newer generation.
#[derive(Clone, Debug, PartialEq)]
pub enum DraftRecovery {
    /// No retained edit.
    Empty,
    /// Recoverable edit; it is not an approved page.
    Draft(Box<DoctorDraft>),
    /// Corrupt/unsupported/misowned draft remains retained until explicit discard.
    Damaged,
}

/// Serializes retention and account transitions with request-fence validation.
#[derive(Clone)]
pub struct RetentionOwner {
    store: Arc<dyn ProtectedStore>,
    session: AccountSession,
    gate: Arc<Mutex<()>>,
}
impl PartialEq for RetentionOwner {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.gate, &other.gate)
    }
}
impl RetentionOwner {
    /// Use a native store in the app; tests may inject a faulting protected-store double.
    pub fn new(store: Arc<dyn ProtectedStore>) -> Self {
        Self {
            store,
            session: AccountSession::default(),
            gate: Arc::new(Mutex::new(())),
        }
    }
    /// Request fences shared by UI and background work.
    pub fn session(&self) -> AccountSession {
        self.session.clone()
    }
    /// A persisted sign-out interruption must be completed before any restored account opens.
    pub fn cleanup_required(&self) -> Result<bool, String> {
        self.store.cleanup_pending()
    }
    /// Restore the protected identity marker. A locked store is not sign-out.
    pub fn retained_identity(&self) -> Result<Option<RetainedIdentity>, String> {
        self.store
            .read("active-identity")?
            .map(|b| serde_json::from_slice(&b).map_err(|_| "Saved sign-in is damaged".into()))
            .transpose()
    }
    /// Open a verified account. Old account cleanup must succeed before a different owner opens.
    pub fn activate(&self, identity: RetainedIdentity) -> Result<SessionFence, String> {
        let _guard = self.gate.lock().unwrap();
        self.session.switch(None);
        if self.store.cleanup_pending()? {
            return Err("Secure sign-out cleanup must finish before sign-in".into());
        }
        if let Some(previous) = self.retained_identity()?
            && (previous.owner != identity.owner || previous.subject != identity.subject)
        {
            self.store.set_cleanup_pending(true)?;
            self.erase_account(&previous.owner)?;
            self.store.remove("active-identity")?;
            self.store.set_cleanup_pending(false)?;
        }
        self.store.write(
            "active-identity",
            &serde_json::to_vec(&identity).map_err(|_| "Cannot retain sign-in")?,
        )?;
        Ok(self.session.switch(Some(identity.owner)).unwrap())
    }
    /// Unlock only the last verified owner's local draft while offline. This grants no network authority.
    pub fn recover_offline(&self) -> Result<DraftRecovery, String> {
        if self.cleanup_required()? {
            return Err("Secure cleanup required".into());
        }
        let identity = self.retained_identity()?.ok_or("No retained identity")?;
        let fence = {
            let _guard = self.gate.lock().unwrap();
            if self.cleanup_required()? {
                return Err("Secure cleanup required".into());
            }
            self.session.switch(Some(identity.owner)).unwrap()
        };
        self.recover(&fence)
    }
    /// Rotate a verified refresh credential without changing the UI/session generation.
    pub fn rotate_credential(
        &self,
        fence: &SessionFence,
        subject: &str,
        refresh_token: String,
    ) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        if self.cleanup_required()? {
            return Err("Secure cleanup required".into());
        }
        let mut retained = self.retained_identity()?.ok_or("No retained credential")?;
        if retained.owner != fence.owner() || retained.subject != subject {
            return Err("Credential owner changed".into());
        }
        retained.refresh_token = refresh_token;
        self.store.write(
            "active-identity",
            &serde_json::to_vec(&retained).map_err(|_| "Cannot encode credential")?,
        )
    }
    /// Ephemeral web/test identity. It grants no persistence and does not consume a native marker.
    pub fn activate_ephemeral(&self, owner: String) -> SessionFence {
        let _guard = self.gate.lock().unwrap();
        self.session.switch(Some(owner)).unwrap()
    }
    /// Invalidate visible state FIRST. Failed erase leaves the protected marker for cleanup retry.
    pub fn sign_out(&self) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.session.switch(None);
        self.store.set_cleanup_pending(true)?;
        if let Some(identity) = self.retained_identity()? {
            self.erase_account(&identity.owner)?;
        }
        self.store.remove("active-identity")?;
        self.store.set_cleanup_pending(false)
    }
    /// Recover metadata-only pending publication/privacy intent. Never stores capability URLs.
    pub fn pending_operation(
        &self,
        fence: &SessionFence,
    ) -> Result<Option<crate::doctor::PendingDoctorOperation>, String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        self.store
            .read(&key("doctor-operation", fence.owner()))?
            .map(|bytes| {
                serde_json::from_slice(&bytes)
                    .map_err(|_| "Pending operation is damaged; do not issue a replacement".into())
            })
            .transpose()
    }
    /// Persist original bytes before dispatch. An uncertain operation cannot be silently replaced.
    pub fn retain_operation(
        &self,
        fence: &SessionFence,
        operation: &crate::doctor::PendingDoctorOperation,
    ) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        let k = key("doctor-operation", fence.owner());
        let bytes = serde_json::to_vec(operation).map_err(|_| "Cannot encode operation")?;
        if self.store.read(&k)?.is_some_and(|old| old != bytes) {
            return Err("Resolve the pending operation before starting another".into());
        }
        self.store.write(&k, &bytes)
    }
    /// Record an explicit 409/412 refusal, distinct from an unknown transport outcome.
    pub fn mark_operation_conflict(
        &self,
        fence: &SessionFence,
        operation: &crate::doctor::PendingDoctorOperation,
    ) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        let bytes = serde_json::to_vec(operation).map_err(|_| "Cannot encode operation")?;
        if self.store.read(&key("doctor-operation", fence.owner()))? != Some(bytes.clone()) {
            return Err("Pending operation changed".into());
        }
        self.store
            .write(&key("doctor-conflict", fence.owner()), &bytes)
    }
    /// A conflicting command may only be released after explicit review. Preserve its metadata
    /// in an account-owned archive; the draft and server receipts are not touched.
    pub fn archive_conflicting_operation(
        &self,
        fence: &SessionFence,
        expected: &crate::doctor::PendingDoctorOperation,
    ) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        let bytes = serde_json::to_vec(expected).map_err(|_| "Cannot encode operation")?;
        if self.store.read(&key("doctor-operation", fence.owner()))? != Some(bytes.clone())
            || self.store.read(&key("doctor-conflict", fence.owner()))? != Some(bytes.clone())
        {
            return Err(
                "Only the reviewed, definitively conflicting operation may be released".into(),
            );
        }
        let archive_key = key("doctor-operation-archive", fence.owner());
        let mut archive: Vec<crate::doctor::PendingDoctorOperation> = self
            .store
            .read(&archive_key)?
            .map(|v| serde_json::from_slice(&v).map_err(|_| "Operation archive is damaged"))
            .transpose()?
            .unwrap_or_default();
        if !archive.contains(expected) {
            archive.push(expected.clone());
        }
        self.store.write(
            &archive_key,
            &serde_json::to_vec(&archive).map_err(|_| "Cannot encode archive")?,
        )?;
        self.store.remove(&key("doctor-operation", fence.owner()))?;
        self.store.remove(&key("doctor-conflict", fence.owner()))
    }
    /// Read only a conflict matching the still-pending exact intent.
    pub fn conflicting_operation(
        &self,
        fence: &SessionFence,
    ) -> Result<Option<crate::doctor::PendingDoctorOperation>, String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        let pending = self.store.read(&key("doctor-operation", fence.owner()))?;
        let conflict = self.store.read(&key("doctor-conflict", fence.owner()))?;
        if pending.is_some() && pending == conflict {
            conflict
                .map(|v| {
                    serde_json::from_slice(&v).map_err(|_| "Conflict metadata is damaged".into())
                })
                .transpose()
        } else {
            Ok(None)
        }
    }
    /// Clear only after a definitive acknowledgement/readback, not a network failure.
    pub fn finish_operation(&self, fence: &SessionFence) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        self.store.remove(&key("doctor-operation", fence.owner()))?;
        self.store.remove(&key("doctor-conflict", fence.owner()))
    }
    /// Recover without contacting the network, but only under a current owner fence.
    pub fn recover(&self, fence: &SessionFence) -> Result<DraftRecovery, String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        let Some(bytes) = self.store.read(&key("doctor-draft", fence.owner()))? else {
            return Ok(DraftRecovery::Empty);
        };
        Ok(match serde_json::from_slice::<Envelope>(&bytes) {
            Ok(envelope) if envelope.version == 1 && envelope.owner == fence.owner() => {
                DraftRecovery::Draft(Box::new(envelope.draft))
            }
            _ => DraftRecovery::Damaged,
        })
    }
    /// Retain edits; damaged drafts require explicit discard first.
    pub fn save(&self, fence: &SessionFence, draft: DoctorDraft) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        let storage_key = key("doctor-draft", fence.owner());
        if let Some(bytes) = self.store.read(&storage_key)? {
            match serde_json::from_slice::<Envelope>(&bytes) {
                Ok(e) if e.version == 1 && e.owner == fence.owner() => {
                    if e.draft.uncertain_save && e.draft.save != draft.save {
                        return Err(
                            "Reconcile the uncertain save before editing its exact request".into(),
                        );
                    }
                }
                _ => return Err("Discard the damaged draft explicitly before replacing it".into()),
            }
        }
        let bytes = serde_json::to_vec(&Envelope {
            version: 1,
            owner: fence.owner().into(),
            draft,
        })
        .map_err(|_| "Cannot encode draft")?;
        self.store.write(&storage_key, &bytes)
    }
    /// Explicit patient discard, or exact acknowledged save; never call on navigation/network failure.
    pub fn discard(&self, fence: &SessionFence) -> Result<(), String> {
        let _guard = self.gate.lock().unwrap();
        self.require(fence)?;
        self.store.remove(&key("doctor-draft", fence.owner()))
    }
    /// Stable namespace for this device's HealthKit queue. The native protected
    /// store does not migrate between devices. Never derive it from the bearer.
    pub fn health_installation(
        &self,
        fence: &SessionFence,
    ) -> Result<uuid::Uuid, HealthStorageError> {
        let _guard = self.gate.lock().unwrap();
        self.require_health_session(fence)?;
        const KEY: &str = "health-installation";
        if let Some(bytes) = self
            .store
            .read(KEY)
            .map_err(HealthStorageError::Unavailable)?
        {
            return uuid::Uuid::from_slice(&bytes)
                .map_err(|error| HealthStorageError::Unavailable(error.to_string()));
        }
        let id = uuid::Uuid::new_v4();
        self.store
            .write(KEY, id.as_bytes())
            .map_err(HealthStorageError::Unavailable)?;
        Ok(id)
    }
    /// Read this owner's health outbox document. Refused unless `fence` is the
    /// current session, so another account can never read or consume it.
    pub fn read_health_outbox(
        &self,
        fence: &SessionFence,
        scope: &str,
    ) -> Result<Option<Vec<u8>>, HealthStorageError> {
        let _guard = self.gate.lock().unwrap();
        self.require_health_session(fence)?;
        self.store
            .read(&health_key(fence.owner(), scope))
            .map_err(HealthStorageError::Unavailable)
    }
    /// Atomically replace this owner's health outbox document under the same fence.
    pub fn write_health_outbox(
        &self,
        fence: &SessionFence,
        scope: &str,
        bytes: &[u8],
    ) -> Result<(), HealthStorageError> {
        let _guard = self.gate.lock().unwrap();
        self.require_health_session(fence)?;
        self.remember_health_scope(fence.owner(), scope)
            .map_err(HealthStorageError::Unavailable)?;
        self.store
            .write(&health_key(fence.owner(), scope), bytes)
            .map_err(HealthStorageError::Unavailable)
    }
    /// Drops every account-owned record: drafts, operations, and queued health uploads.
    fn erase_account(&self, owner: &str) -> Result<(), String> {
        self.store.remove(&key("doctor-draft", owner))?;
        self.store.remove(&key("doctor-operation", owner))?;
        self.store.remove(&key("doctor-conflict", owner))?;
        self.store.remove(&key("doctor-operation-archive", owner))?;
        let scopes = self.health_scopes(owner)?;
        for scope in scopes {
            self.store.remove(&health_key(owner, &scope))?;
        }
        self.store.remove(&health_scopes_key(owner))
    }
    fn health_scopes(&self, owner: &str) -> Result<Vec<String>, String> {
        match self.store.read(&health_scopes_key(owner))? {
            None => Ok(Vec::new()),
            Some(bytes) => {
                serde_json::from_slice(&bytes).map_err(|_| "Health scope index is damaged".into())
            }
        }
    }
    fn remember_health_scope(&self, owner: &str, scope: &str) -> Result<(), String> {
        let mut scopes = self.health_scopes(owner)?;
        if scopes.iter().any(|known| known == scope) {
            return Ok(());
        }
        scopes.push(scope.to_owned());
        self.store.write(
            &health_scopes_key(owner),
            &serde_json::to_vec(&scopes).map_err(|_| "Cannot encode health scopes")?,
        )
    }
    fn require_health_session(&self, fence: &SessionFence) -> Result<(), HealthStorageError> {
        if !self.session.is_current(fence) {
            return Err(HealthStorageError::AccountChanged);
        }
        match self.store.cleanup_pending() {
            Ok(false) => Ok(()),
            Ok(true) => Err(HealthStorageError::AccountChanged),
            Err(error) => Err(HealthStorageError::Unavailable(error)),
        }
    }
    fn require(&self, fence: &SessionFence) -> Result<(), String> {
        if self.session.is_current(fence) {
            Ok(())
        } else {
            Err("Account changed; response ignored".into())
        }
    }
}

/// Copy a capability only on explicit patient action; native hosts bound clipboard retention.
pub fn copy_native_link(value: &str) -> Result<(), String> {
    platform_call(serde_json::json!({"op":"copy_link", "value":value})).map(|_| ())
}
/// Present an already source-checked PDF through the native share sheet. No publication authority is granted.
pub fn present_native_pdf(bytes: &[u8]) -> Result<(), String> {
    platform_call(serde_json::json!({"op":"share_pdf", "bytes":bytes})).map(|_| ())
}

pub(crate) fn platform_call(request: serde_json::Value) -> Result<serde_json::Value, String> {
    #[cfg(any(target_os = "ios", target_os = "android"))]
    {
        let response = native_call(&request.to_string())?;
        let reply: serde_json::Value =
            serde_json::from_str(&response).map_err(|_| "Invalid protected storage response")?;
        if reply["ok"] != true {
            return Err("Protected storage is locked, damaged or unavailable".into());
        }
        Ok(reply["value"].clone())
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = request;
        Err("Protected retention requires the native iOS or Android host".into())
    }
}

#[cfg(target_os = "ios")]
fn native_call(request: &str) -> Result<String, String> {
    unsafe extern "C" {
        fn duet_secure_call(request: *const std::ffi::c_char) -> *mut std::ffi::c_char;
        fn duet_secure_free(value: *mut std::ffi::c_char);
    }
    let request = std::ffi::CString::new(request).map_err(|_| "Invalid storage request")?;
    let ptr = unsafe { duet_secure_call(request.as_ptr()) };
    if ptr.is_null() {
        return Err("Protected storage unavailable".into());
    }
    let result = unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_str()
        .map(str::to_owned)
        .map_err(|_| "Invalid storage response".into());
    unsafe { duet_secure_free(ptr) };
    result
}
#[cfg(target_os = "android")]
fn native_call(request: &str) -> Result<String, String> {
    use jni::objects::{JClass, JObject, JString, JValue};
    let ctx = ndk_context::android_context();
    let vm =
        unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.map_err(|_| "Native VM unavailable")?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|_| "Native thread unavailable")?;
    let context = unsafe { JObject::from_raw(ctx.context().cast()) };
    let result = (|| -> jni::errors::Result<String> {
        let class = env.get_object_class(&context)?;
        let loader = env
            .call_method(class, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?
            .l()?;
        let name = env.new_string("coda.duet.SecureRetentionBridge")?;
        let bridge = env
            .call_method(
                loader,
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[JValue::Object(&name)],
            )?
            .l()?;
        let input = env.new_string(request)?;
        let value = env
            .call_static_method(
                JClass::from(bridge),
                "call",
                "(Landroid/content/Context;Ljava/lang/String;)Ljava/lang/String;",
                &[JValue::Object(&context), JValue::Object(&input)],
            )?
            .l()?;
        let string = JString::from(value);
        Ok(env.get_string(&string)?.to_string_lossy().into_owned())
    })();
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_clear();
    }
    std::mem::forget(context);
    result.map_err(|_| "Protected storage unavailable".into())
}
