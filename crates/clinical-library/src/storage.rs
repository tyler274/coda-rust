//! Where a candidate's files are written.
//!
//! Each run builds into a folder only it uses, `candidates/<run>/`, made from
//! its [`Lease`]. A path inside the folder can't climb out of it, and every
//! file is written once: [`ObjectStore::create`] refuses a key that already
//! holds a file. So a run that lost its slot and wakes up can't touch another
//! run's folder, and can't replace a file in its own, even one search serves
//! because the run activated it before losing the slot. The one step that
//! changes what search serves is [`Store::activate`](crate::Store::activate),
//! which checks the lease. So a run needs to hold the slot only to activate,
//! not to build.

use std::fmt;

use async_trait::async_trait;

use crate::slot::{Lease, RunId};

/// Object storage: a local directory, or a bucket.
#[async_trait]
pub trait ObjectStore: Send + Sync {
    /// Writes `bytes` at `key`, which must hold no file yet; a key that does
    /// is refused with [`StorageError::Exists`], never replaced.
    async fn create(&self, key: &ObjectKey, bytes: Vec<u8>) -> Result<(), StorageError>;
}

/// A full key in object storage. Only a [`CandidateFolder`] makes one.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectKey(String);

impl ObjectKey {
    /// The key as written.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObjectKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A file's path inside a candidate's folder: `/`-separated segments of
/// letters, digits, `.`, `_` and `-`, none of them `.` or `..`, at most 512
/// characters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectPath(String);

impl ObjectPath {
    /// Checks and wraps a path.
    pub fn new(path: impl Into<String>) -> Result<Self, InvalidObjectPath> {
        let path = path.into();
        let segment_ok = |segment: &str| {
            !segment.is_empty()
                && segment != "."
                && segment != ".."
                && segment
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
        };
        if path.len() <= 512 && path.split('/').all(segment_ok) {
            Ok(Self(path))
        } else {
            Err(InvalidObjectPath(path))
        }
    }
}

/// A path that could leave its folder, or isn't one this store writes.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a path inside a candidate's folder")]
pub struct InvalidObjectPath(pub String);

/// A write that object storage refused or failed.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// The key already holds a file, which is never replaced.
    #[error("{0} already holds a file")]
    Exists(ObjectKey),
    /// The store failed the write.
    #[error("couldn't write {key}")]
    Failed {
        /// Where the write was going.
        key: ObjectKey,
        /// Why it failed.
        #[source]
        cause: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// The folder one run builds its candidate into.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateFolder {
    run: RunId,
}

impl CandidateFolder {
    /// The folder of the run holding `lease`.
    pub fn of(lease: &Lease) -> Self {
        Self { run: lease.run() }
    }

    /// The run whose folder this is.
    pub const fn run(&self) -> RunId {
        self.run
    }

    /// The full key of `path` inside this folder.
    pub fn key(&self, path: &ObjectPath) -> ObjectKey {
        ObjectKey(format!("candidates/{}/{}", self.run, path.0))
    }

    /// Writes one new file of this run's candidate.
    pub async fn create(
        &self,
        store: &dyn ObjectStore,
        path: &ObjectPath,
        bytes: Vec<u8>,
    ) -> Result<(), StorageError> {
        store.create(&self.key(path), bytes).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_stay_inside_their_folder() {
        for good in ["index/part-0.lance", "manifest.json", "a/b/c"] {
            assert!(ObjectPath::new(good).is_ok(), "{good:?}");
        }
        for bad in [
            "",
            "/abs",
            "a//b",
            "../x",
            "a/../../b",
            "./x",
            "a b",
            "a\\b",
        ] {
            assert!(ObjectPath::new(bad).is_err(), "{bad:?}");
        }
    }
}
