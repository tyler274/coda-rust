//! A keyring in memory, for tests: it can be locked, and it counts reads.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, PoisonError};

use super::{Fallback, Keyring, KeyringError};
use crate::broker::{Item, Secret, SecretError};

#[derive(Default)]
pub struct MemoryState {
    pub items: HashMap<Item, Secret>,
    /// Items whose stored value is not a token.
    pub not_tokens: HashSet<Item>,
    pub locked: Locked,
    pub lookups: usize,
    /// What `gh auth token` answers; `None` makes it fail.
    pub gh_token: Option<Secret>,
    /// Runs once, after the next lookup has read its answer and before it
    /// returns, with no lock of the fake held.
    pub during_lookup: Option<Box<dyn FnOnce() + Send>>,
}

/// Whether the fake keyring answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locked {
    #[default]
    Open,
    Locked,
}

#[derive(Default)]
pub struct Memory {
    pub state: Mutex<MemoryState>,
}

impl Memory {
    pub fn with(items: impl IntoIterator<Item = (Item, &'static str)>) -> Self {
        let memory = Self::default();
        memory.edit(|state| {
            state.items = items
                .into_iter()
                .map(|(item, text)| (item, Secret::new(text).expect("a token")))
                .collect();
        });
        memory
    }

    pub fn edit<T>(&self, change: impl FnOnce(&mut MemoryState) -> T) -> T {
        change(&mut self.state.lock().unwrap_or_else(PoisonError::into_inner))
    }

    pub fn set_locked(&self, locked: Locked) {
        self.edit(|state| state.locked = locked);
    }

    pub fn set_not_a_token(&self, item: Item) {
        self.edit(|state| state.not_tokens.insert(item));
    }

    fn open<T>(&self, use_items: impl FnOnce(&mut MemoryState) -> T) -> Result<T, KeyringError> {
        self.edit(|state| match state.locked {
            Locked::Locked => Err(KeyringError::Locked),
            Locked::Open => Ok(use_items(state)),
        })
    }
}

impl Keyring for Memory {
    fn lookup(&self, item: Item) -> Result<Option<Secret>, KeyringError> {
        let (found, hook) = self.edit(|state| {
            state.lookups += 1;
            (
                (state.locked, state.not_tokens.contains(&item)),
                state.during_lookup.take(),
            )
        });
        let answer = match found {
            (Locked::Locked, _) => Err(KeyringError::Locked),
            (Locked::Open, true) => Err(KeyringError::NotAToken {
                program: "memory",
                source: SecretError::NotPrintable { at: 0 },
            }),
            (Locked::Open, false) => Ok(self.edit(|state| state.items.get(&item).cloned())),
        };
        if let Some(hook) = hook {
            hook();
        }
        answer
    }

    fn store(&self, item: Item, secret: &Secret) -> Result<(), KeyringError> {
        self.open(|state| {
            state.items.insert(item, secret.clone());
        })
    }

    fn delete(&self, item: Item) -> Result<(), KeyringError> {
        self.open(|state| {
            state.items.remove(&item);
        })
    }
}

impl Fallback for Memory {
    fn gh_token(&self) -> Result<Secret, KeyringError> {
        self.edit(|state| state.gh_token.clone())
            .ok_or(KeyringError::NotAToken {
                program: "gh",
                source: SecretError::Empty,
            })
    }
}
