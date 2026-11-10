//! What failed, as a short stable code.
//!
//! The stop rule compares codes to tell "the same failure twice" apart from
//! two different failures, so a code names the cause, never the moment: two
//! timeouts from NCBI share a code even though their messages differ. The
//! failure's own words stay with its record, not in the code.

use std::fmt;

/// A stable name for a failure's cause: lowercase letters, digits and
/// underscores, starting with a letter, at most 64 characters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FailureCode(String);

impl FailureCode {
    /// A night ended without recording its outcome, and the next night took
    /// over its slot.
    pub const ENDED_WITHOUT_OUTCOME: &str = "ended_without_outcome";

    /// A night said it finished, but the store's tally shows work due
    /// tonight that it neither admitted nor set aside, or admitted content
    /// that no new version publishes.
    pub const DUE_WORK_LEFT_UNDONE: &str = "due_work_left_undone";

    /// Checks and wraps a code.
    pub fn new(code: impl Into<String>) -> Result<Self, InvalidFailureCode> {
        let code = code.into();
        let mut chars = code.chars();
        let starts_with_letter = chars.next().is_some_and(|c| c.is_ascii_lowercase());
        let rest_ok = chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if starts_with_letter && rest_ok && code.len() <= 64 {
            Ok(Self(code))
        } else {
            Err(InvalidFailureCode(code))
        }
    }

    /// One of this crate's own codes, written as a literal. A test checks
    /// every one is a valid code.
    pub(crate) fn known(code: &'static str) -> Self {
        debug_assert!(Self::new(code).is_ok(), "{code:?} is not a failure code");
        Self(code.to_owned())
    }

    /// The code a taken-over night is recorded with.
    pub fn ended_without_outcome() -> Self {
        Self(Self::ENDED_WITHOUT_OUTCOME.to_owned())
    }

    /// The code as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FailureCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A failure code that isn't lowercase letters, digits and underscores
/// starting with a letter, or is longer than 64 characters.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a failure code")]
pub struct InvalidFailureCode(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_short_stable_names() {
        assert!(FailureCode::new("ncbi_timeout").is_ok());
        assert!(FailureCode::new("http_503").is_ok());
        for bad in [
            "",
            "Timeout",
            "9lives",
            "two words",
            "x".repeat(65).as_str(),
        ] {
            assert!(FailureCode::new(bad).is_err(), "{bad:?}");
        }
        assert!(FailureCode::new(FailureCode::ENDED_WITHOUT_OUTCOME).is_ok());
        assert!(FailureCode::new(FailureCode::DUE_WORK_LEFT_UNDONE).is_ok());
    }
}
