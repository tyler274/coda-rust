//! A built version of the search index on its way to being active.
//!
//! A candidate is built, then checked, then activated, and the types enforce
//! that order: only a `Candidate<Checked>` can be passed to
//! [`Store::activate`](crate::Store::activate), and only
//! [`Candidate::check`] makes one.
//!
//! ```compile_fail
//! use clinical_library::{Candidate, VersionId};
//! // A version that was built but never checked can't be activated.
//! fn activate_unchecked(store: &clinical_library::Store, lease: &clinical_library::Lease) {
//!     let built = Candidate::built(VersionId::new("v1").unwrap());
//!     let _ = store.activate(lease, built);
//! }
//! ```

use std::fmt;
use std::marker::PhantomData;

/// The name of one built version of the index, such as its content digest.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VersionId(String);

impl VersionId {
    /// Checks and wraps a version name: 1 to 128 visible ASCII characters.
    pub fn new(id: impl Into<String>) -> Result<Self, InvalidVersionId> {
        let id = id.into();
        if (1..=128).contains(&id.len()) && id.chars().all(|c| c.is_ascii_graphic()) {
            Ok(Self(id))
        } else {
            Err(InvalidVersionId(id))
        }
    }

    /// The name as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VersionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A version name that is empty, too long, or not visible ASCII.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a version name")]
pub struct InvalidVersionId(pub String);

/// A candidate whose build finished.
#[derive(Debug)]
pub enum Built {}

/// A candidate that passed its checks.
#[derive(Debug)]
pub enum Checked {}

/// One built version of the index, in state `S`.
#[derive(Debug)]
pub struct Candidate<S> {
    version: VersionId,
    state: PhantomData<S>,
}

impl<S> Candidate<S> {
    /// The version this candidate is.
    pub fn version(&self) -> &VersionId {
        &self.version
    }
}

impl Candidate<Built> {
    /// A version whose build finished.
    pub fn built(version: VersionId) -> Self {
        Self {
            version,
            state: PhantomData,
        }
    }

    /// Runs the candidate's checks. Only a candidate that passes can be
    /// activated; one that fails is handed back with the check's error.
    pub fn check<E>(
        self,
        check: impl FnOnce(&VersionId) -> Result<(), E>,
    ) -> Result<Candidate<Checked>, CheckFailed<E>> {
        match check(&self.version) {
            Ok(()) => Ok(Candidate {
                version: self.version,
                state: PhantomData,
            }),
            Err(source) => Err(CheckFailed {
                version: self.version,
                source,
            }),
        }
    }
}

/// A candidate's checks failed, so it stays inactive.
#[derive(Debug, thiserror::Error)]
#[error("version {version} failed its checks")]
pub struct CheckFailed<E> {
    /// The version that failed.
    pub version: VersionId,
    /// What the check found.
    #[source]
    pub source: E,
}

/// Proof that a version became the active one. Only
/// [`Store::activate`](crate::Store::activate) returns it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Activation {
    pub(crate) version: VersionId,
}

impl Activation {
    /// The version that is now active.
    pub fn version(&self) -> &VersionId {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failed_check_hands_the_version_back() {
        let built = Candidate::built(VersionId::new("sha256:abc").unwrap());
        let failed = built.check(|_| Err("passage count mismatch")).unwrap_err();
        assert_eq!(failed.version.as_str(), "sha256:abc");
        assert_eq!(failed.source, "passage count mismatch");

        let built = Candidate::built(VersionId::new("sha256:def").unwrap());
        let checked = built.check(|_| Ok::<(), &str>(())).unwrap();
        assert_eq!(checked.version().as_str(), "sha256:def");
    }

    #[test]
    fn version_names_are_bounded() {
        assert!(VersionId::new("").is_err());
        assert!(VersionId::new("two words").is_err());
        assert!(VersionId::new("x".repeat(129)).is_err());
    }
}
