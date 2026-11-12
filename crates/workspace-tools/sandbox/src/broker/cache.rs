//! Tokens the broker has read, kept in memory only.
//!
//! An entry is [`Held::Fresh`] for [`FRESH_FOR`] after the keyring last
//! gave it; until then requests skip the keyring. After that it is
//! [`Held::Stale`] and the next request reads the keyring again, so a
//! rotation or deletion lands without a reload. A stale entry is served
//! only while the keyring is locked or unreachable, as it is once the user
//! logs out and the lingering broker keeps working, and only until
//! [`STALE_FOR`] after the last good read. A reload drops everything.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{Item, Secret, Source};

pub const FRESH_FOR: Duration = Duration::from_secs(60 * 60);

/// How long after its last good read a token outlives a locked keyring.
/// Eight hours covers an agent working through an evening or a night after
/// the user logs out, and ends by the next working day, so a token the user
/// can no longer see in an unlocked wallet does not stay in use for days.
pub const STALE_FOR: Duration = Duration::from_secs(8 * 60 * 60);

#[derive(Debug, Clone)]
pub struct Entry {
    pub secret: Secret,
    pub source: Source,
    pub fetched_at: Instant,
}

/// What the cache holds for an item at a given moment.
#[derive(Debug, Clone, Copy)]
pub enum Held<'a> {
    /// Read within [`FRESH_FOR`]: serve it without asking the keyring.
    Fresh(&'a Entry),
    /// Older, but within [`STALE_FOR`]: ask the keyring, and serve this
    /// only if the keyring is locked.
    Stale(&'a Entry),
    /// Never read, dropped, or past [`STALE_FOR`].
    Nothing,
}

#[derive(Debug, Default)]
pub struct Cache {
    entries: HashMap<Item, Entry>,
}

impl Cache {
    pub fn held(&self, item: Item, now: Instant) -> Held<'_> {
        let Some(entry) = self.entries.get(&item) else {
            return Held::Nothing;
        };
        let age = now.saturating_duration_since(entry.fetched_at);
        if age < FRESH_FOR {
            Held::Fresh(entry)
        } else if age < STALE_FOR {
            Held::Stale(entry)
        } else {
            Held::Nothing
        }
    }

    pub fn insert(&mut self, item: Item, entry: Entry) {
        self.entries.insert(item, entry);
    }

    pub fn remove(&mut self, item: Item) {
        self.entries.remove(&item);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_then_stale_then_nothing_and_cleared() {
        let start = Instant::now();
        let mut cache = Cache::default();
        assert!(matches!(
            cache.held(Item::GithubAgent, start),
            Held::Nothing
        ));
        cache.insert(
            Item::GithubAgent,
            Entry {
                secret: Secret::new("s").expect("a token"),
                source: Source::Keyring,
                fetched_at: start,
            },
        );
        assert!(matches!(
            cache.held(Item::GithubAgent, start),
            Held::Fresh(_)
        ));
        assert!(matches!(cache.held(Item::Doppler, start), Held::Nothing));
        let stale = start + FRESH_FOR;
        assert!(matches!(
            cache.held(Item::GithubAgent, stale),
            Held::Stale(_)
        ));
        let expired = start + STALE_FOR;
        assert!(matches!(
            cache.held(Item::GithubAgent, expired),
            Held::Nothing
        ));
        cache.clear();
        assert!(matches!(
            cache.held(Item::GithubAgent, start),
            Held::Nothing
        ));
    }
}
