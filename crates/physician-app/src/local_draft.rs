//! Same-tab recovery of unsent composer text.
//!
//! The composer's unsent text, baseline revision, edit generation and any
//! Send or clear still being reconciled are kept in this tab's
//! `sessionStorage`, so a reload of the same tab can put them back. Nothing
//! is promised after the tab or browser closes, and nothing is kept in
//! `localStorage` (unlike the long-lived panel preferences in `panels.rs`).
//!
//! Every record is scoped: the key carries a version, the backend the page
//! talks to, the practice, the physician, the patient workspace, the chat and
//! the writing document. The scope parts are a JSON array, so no id can be
//! confused with another. A page restores only under a known physician
//! identity, and only after the workspace itself was read for that physician.
//! Sign-out, another physician's sign-in and removed access forget records.
//!
//! Storage can be missing, denied or full. Writes then report failure; the
//! shell keeps the text in memory and says reload recovery is unavailable.
// Storage is browser-only; native builds keep recovery in memory.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use serde::{Deserialize, Serialize};

use crate::composer::{DraftTarget, Recovery};

/// Every recovery key starts with this, version included.
const PREFIX: &str = "coda.physician.draft.v1|";

/// Who a record belongs to: the backend and the signed-in physician.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Scope {
    base: String,
    practice: String,
    physician: String,
}

impl Scope {
    /// A scope only for a known identity; an empty id never restores.
    pub(crate) fn new(base: &str, practice: &str, physician: &str) -> Option<Self> {
        let known = |value: &str| !value.trim().is_empty();
        (known(base) && known(practice) && known(physician)).then(|| Self {
            base: base.trim_end_matches('/').to_owned(),
            practice: practice.to_owned(),
            physician: physician.to_owned(),
        })
    }

    fn parts(&self, target: &DraftTarget) -> [String; 5] {
        [
            self.base.clone(),
            self.practice.clone(),
            self.physician.clone(),
            target.workspace.clone(),
            target.chat.clone(),
        ]
    }
}

/// One stored record and when it was written.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Stored {
    written_at: f64,
    record: Recovery,
}

/// The storage the records live in: the tab's `sessionStorage` in a
/// browser, a map in tests.
pub(crate) trait Backend {
    fn keys(&self) -> Vec<String>;
    fn get(&self, key: &str) -> Option<String>;
    /// Fails when storage is unavailable or full.
    fn set(&self, key: &str, value: &str) -> Result<(), ()>;
    fn remove(&self, key: &str);
}

fn target_prefix(scope: &Scope, target: &DraftTarget) -> String {
    let parts = serde_json::to_string(&scope.parts(target)).unwrap_or_default();
    format!("{PREFIX}{parts}|")
}

/// The scope parts and writer a key names, or `None` for any other key.
fn parse(key: &str) -> Option<([String; 5], &str)> {
    let rest = key.strip_prefix(PREFIX)?;
    let mut values = serde_json::Deserializer::from_str(rest).into_iter::<[String; 5]>();
    let parts = values.next()?.ok()?;
    let writer = rest[values.byte_offset()..].strip_prefix('|')?;
    Some((parts, writer))
}

/// The newest record any document of this tab kept for `target`.
pub(crate) fn read(backend: &dyn Backend, scope: &Scope, target: &DraftTarget) -> Option<Recovery> {
    let parts = scope.parts(target);
    backend
        .keys()
        .into_iter()
        .filter(|key| parse(key).is_some_and(|(found, _)| found == parts))
        .filter_map(|key| serde_json::from_str::<Stored>(&backend.get(&key)?).ok())
        .max_by(|a, b| a.written_at.total_cmp(&b.written_at))
        .map(|stored| stored.record)
}

/// Keeps `record` for `target` under this document's `writer`, or removes it
/// when `None`. Once this document's copy is written, older documents'
/// copies are dropped. A failed write keeps the older copies.
pub(crate) fn write(
    backend: &dyn Backend,
    scope: &Scope,
    target: &DraftTarget,
    writer: &str,
    record: Option<&Recovery>,
    now: f64,
) -> Result<(), ()> {
    let prefix = target_prefix(scope, target);
    let mine = format!("{prefix}{writer}");
    if let Some(record) = record {
        let stored = serde_json::to_string(&Stored {
            written_at: now,
            record: record.clone(),
        })
        .map_err(|_| ())?;
        backend.set(&mine, &stored)?;
    }
    let parts = scope.parts(target);
    for key in backend.keys() {
        if parse(&key).is_some_and(|(found, _)| found == parts) && (record.is_none() || key != mine)
        {
            backend.remove(&key);
        }
    }
    Ok(())
}

/// Forgets this physician's records for one workspace (access removed).
pub(crate) fn forget_workspace(backend: &dyn Backend, scope: &Scope, workspace: &str) {
    for key in backend.keys() {
        if parse(&key).is_some_and(|(parts, _)| {
            parts[..3]
                == [
                    scope.base.clone(),
                    scope.practice.clone(),
                    scope.physician.clone(),
                ]
                && parts[3] == workspace
        }) {
            backend.remove(&key);
        }
    }
}

/// Forgets every record (sign-out, another physician signing in).
pub(crate) fn forget_all(backend: &dyn Backend) {
    for key in backend.keys() {
        if key.starts_with(PREFIX) {
            backend.remove(&key);
        }
    }
}

/// This tab's `sessionStorage`. Denied or missing storage is `None`.
#[cfg(target_arch = "wasm32")]
pub(crate) struct SessionStorage(web_sys::Storage);

#[cfg(target_arch = "wasm32")]
impl SessionStorage {
    pub(crate) fn open() -> Option<Self> {
        web_sys::window()?
            .session_storage()
            .ok()
            .flatten()
            .map(Self)
    }
}

#[cfg(target_arch = "wasm32")]
impl Backend for SessionStorage {
    fn keys(&self) -> Vec<String> {
        let length = self.0.length().unwrap_or(0);
        (0..length)
            .filter_map(|index| self.0.key(index).ok().flatten())
            .collect()
    }

    fn get(&self, key: &str) -> Option<String> {
        self.0.get_item(key).ok().flatten()
    }

    fn set(&self, key: &str, value: &str) -> Result<(), ()> {
        self.0.set_item(key, value).map_err(|_| ())
    }

    fn remove(&self, key: &str) {
        let _ = self.0.remove_item(key);
    }
}

/// Forgets every record in this tab's storage, when there is storage.
pub(crate) fn forget_everything() {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = SessionStorage::open() {
        forget_all(&storage);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    use super::{Backend, Scope, forget_all, forget_workspace, read, write};
    use crate::composer::{DraftTarget, Recovery};

    #[derive(Default)]
    pub(crate) struct Memory {
        pub(crate) items: RefCell<BTreeMap<String, String>>,
        pub(crate) full: bool,
    }

    impl Backend for Memory {
        fn keys(&self) -> Vec<String> {
            self.items.borrow().keys().cloned().collect()
        }
        fn get(&self, key: &str) -> Option<String> {
            self.items.borrow().get(key).cloned()
        }
        fn set(&self, key: &str, value: &str) -> Result<(), ()> {
            if self.full {
                return Err(());
            }
            self.items
                .borrow_mut()
                .insert(key.to_owned(), value.to_owned());
            Ok(())
        }
        fn remove(&self, key: &str) {
            self.items.borrow_mut().remove(key);
        }
    }

    fn record(text: &str) -> Recovery {
        Recovery {
            text: text.into(),
            saved: String::new(),
            revision: 0,
            generation: 1,
            conflict: None,
            pending_send: None,
            pending_clear: false,
            pending_save: None,
        }
    }

    fn scope(physician: &str) -> Scope {
        Scope::new("http://coda.test/", "practice", physician).unwrap()
    }

    #[test]
    fn an_unknown_identity_has_no_scope() {
        assert!(Scope::new("http://coda.test", "", "doc").is_none());
        assert!(Scope::new("http://coda.test", "practice", " ").is_none());
        assert!(Scope::new("", "practice", "doc").is_none());
    }

    #[test]
    fn a_record_comes_back_only_for_its_own_backend_practice_physician_and_target() {
        let store = Memory::default();
        let target = DraftTarget::new("pw-1", Some("chat-1"));
        write(
            &store,
            &scope("doc"),
            &target,
            "w1",
            Some(&record("mine")),
            1.0,
        )
        .unwrap();
        assert_eq!(read(&store, &scope("doc"), &target).unwrap().text, "mine");
        assert!(read(&store, &scope("other"), &target).is_none());
        let elsewhere = Scope::new("http://other.test", "practice", "doc").unwrap();
        assert!(read(&store, &elsewhere, &target).is_none());
        let other_practice = Scope::new("http://coda.test", "practice-2", "doc").unwrap();
        assert!(read(&store, &other_practice, &target).is_none());
        assert!(
            read(
                &store,
                &scope("doc"),
                &DraftTarget::new("pw-1", Some("chat-2"))
            )
            .is_none()
        );
        assert!(
            read(
                &store,
                &scope("doc"),
                &DraftTarget::new("pw-2", Some("chat-1"))
            )
            .is_none()
        );
    }

    /// Ids holding the separator never alias another scope.
    #[test]
    fn ids_containing_the_separator_stay_unambiguous() {
        let store = Memory::default();
        let tricky = DraftTarget::new("pw|1", Some("chat"));
        let plain = DraftTarget::new("pw", Some("1|chat"));
        write(
            &store,
            &scope("doc"),
            &tricky,
            "w|1",
            Some(&record("tricky")),
            1.0,
        )
        .unwrap();
        assert!(read(&store, &scope("doc"), &plain).is_none());
        assert_eq!(read(&store, &scope("doc"), &tricky).unwrap().text, "tricky");
    }

    /// A reload is a new writer: it reads the newest record, and once its own
    /// copy is written the previous document's copy is gone.
    #[test]
    fn a_new_document_claims_the_previous_documents_record() {
        let store = Memory::default();
        let target = DraftTarget::new("pw-1", Some("chat-1"));
        write(
            &store,
            &scope("doc"),
            &target,
            "old",
            Some(&record("before")),
            1.0,
        )
        .unwrap();
        let found = read(&store, &scope("doc"), &target).unwrap();
        write(&store, &scope("doc"), &target, "new", Some(&found), 2.0).unwrap();
        assert_eq!(store.keys().len(), 1);
        assert!(store.keys()[0].ends_with("|new"));
        write(&store, &scope("doc"), &target, "new", None, 3.0).unwrap();
        assert!(store.keys().is_empty());
    }

    #[test]
    fn a_full_store_reports_failure_and_keeps_the_older_copy() {
        let mut store = Memory::default();
        let target = DraftTarget::new("pw-1", Some("chat-1"));
        write(
            &store,
            &scope("doc"),
            &target,
            "old",
            Some(&record("before")),
            1.0,
        )
        .unwrap();
        store.full = true;
        assert!(
            write(
                &store,
                &scope("doc"),
                &target,
                "new",
                Some(&record("after")),
                2.0
            )
            .is_err()
        );
        assert_eq!(read(&store, &scope("doc"), &target).unwrap().text, "before");
    }

    #[test]
    fn removed_access_and_sign_out_forget_records() {
        let store = Memory::default();
        let one = DraftTarget::new("pw-1", Some("chat-1"));
        let two = DraftTarget::new("pw-2", Some("chat-1"));
        write(&store, &scope("doc"), &one, "w", Some(&record("one")), 1.0).unwrap();
        write(&store, &scope("doc"), &two, "w", Some(&record("two")), 1.0).unwrap();
        write(
            &store,
            &scope("other"),
            &one,
            "w",
            Some(&record("other")),
            1.0,
        )
        .unwrap();
        store.set("coda.physician.session", "token").unwrap();
        forget_workspace(&store, &scope("doc"), "pw-1");
        assert!(read(&store, &scope("doc"), &one).is_none());
        assert!(read(&store, &scope("doc"), &two).is_some());
        assert!(read(&store, &scope("other"), &one).is_some());
        forget_all(&store);
        assert_eq!(store.keys(), vec!["coda.physician.session".to_owned()]);
    }
}
