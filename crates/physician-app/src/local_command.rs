//! Same-tab recovery of in-flight (`Pending`) and `Unconfirmed` chat commands
//! so reloading the page restores pending/unconfirmed command state and its
//! `Check again` action with the original idempotency key (`Chat Command 8`,
//! `Chat Rule 13`).
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use serde::{Deserialize, Serialize};

use crate::local_draft::{Backend, Scope};

const PREFIX: &str = "coda.physician.command.v1|";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PersistedCommandStage {
    Pending,
    Unconfirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PersistedCommand {
    pub(crate) slot: String,
    pub(crate) workspace: String,
    pub(crate) chat: String,
    pub(crate) command: serde_json::Value,
    pub(crate) stage: PersistedCommandStage,
}

fn storage_key(scope: &Scope, slot: &str) -> String {
    let [base, practice, physician] = scope.account_parts();
    let parts = serde_json::to_string(&[base, practice, physician, slot]).unwrap_or_default();
    format!("{PREFIX}{parts}")
}

fn parse(key: &str) -> Option<[String; 4]> {
    let rest = key.strip_prefix(PREFIX)?;
    serde_json::from_str::<[String; 4]>(rest).ok()
}

pub(crate) fn write(backend: &dyn Backend, scope: &Scope, record: &PersistedCommand) {
    if let Ok(encoded) = serde_json::to_string(record) {
        let _ = backend.set(&storage_key(scope, &record.slot), &encoded);
    }
}

pub(crate) fn remove(backend: &dyn Backend, scope: &Scope, slot: &str) {
    backend.remove(&storage_key(scope, slot));
}

pub(crate) fn read(backend: &dyn Backend, scope: &Scope, slot: &str) -> Option<PersistedCommand> {
    let raw = backend.get(&storage_key(scope, slot))?;
    serde_json::from_str::<PersistedCommand>(&raw).ok()
}

pub(crate) fn forget_workspace(backend: &dyn Backend, scope: &Scope, workspace: &str) {
    let [base, practice, physician] = scope.account_parts();
    for key in backend.keys() {
        let Some(parts) = parse(&key) else { continue };
        if parts[0] == base
            && parts[1] == practice
            && parts[2] == physician
            && backend
                .get(&key)
                .and_then(|raw| serde_json::from_str::<PersistedCommand>(&raw).ok())
                .is_some_and(|cmd| cmd.workspace == workspace)
        {
            backend.remove(&key);
        }
    }
}

pub(crate) fn forget_all(backend: &dyn Backend) {
    for key in backend.keys() {
        if key.starts_with(PREFIX) {
            backend.remove(&key);
        }
    }
}

pub(crate) fn forget_everything() {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = crate::local_draft::SessionStorage::open() {
        forget_all(&storage);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    use super::*;

    #[derive(Default)]
    struct MemBackend(RefCell<BTreeMap<String, String>>);

    impl Backend for MemBackend {
        fn keys(&self) -> Vec<String> {
            self.0.borrow().keys().cloned().collect()
        }

        fn get(&self, key: &str) -> Option<String> {
            self.0.borrow().get(key).cloned()
        }

        fn set(&self, key: &str, value: &str) -> Result<(), ()> {
            self.0.borrow_mut().insert(key.to_owned(), value.to_owned());
            Ok(())
        }

        fn remove(&self, key: &str) {
            self.0.borrow_mut().remove(key);
        }
    }

    #[test]
    fn persisted_commands_survive_reload_within_scope_and_clear_on_workspace_removal() {
        let backend = MemBackend::default();
        let scope = Scope::new("https://coda.test", "prac-1", "phys-1").unwrap();
        let other = Scope::new("https://coda.test", "prac-1", "phys-2").unwrap();
        let rec = PersistedCommand {
            slot: "chat-stop".into(),
            workspace: "pw-1".into(),
            chat: "chat-1".into(),
            command: serde_json::json!({"type": "work.stop", "idempotency_key": "k-1"}),
            stage: PersistedCommandStage::Unconfirmed,
        };
        write(&backend, &scope, &rec);
        assert_eq!(read(&backend, &scope, "chat-stop"), Some(rec.clone()));
        assert_eq!(read(&backend, &other, "chat-stop"), None);

        forget_workspace(&backend, &scope, "pw-1");
        assert_eq!(read(&backend, &scope, "chat-stop"), None);
    }
}
