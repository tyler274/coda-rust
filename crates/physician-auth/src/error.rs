//! Why a sign-in or session step failed.

/// Why a session resolution failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    /// No account holds this token, or it expired.
    Invalid,
    /// The account is removed or deactivated.
    Inactive,
}

/// Why a sign-in step failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignInError {
    /// The email has no account, or the token is unknown, used, or expired.
    Invalid,
    /// The account exists but may not sign in.
    Inactive,
}

/// The store could not answer, so the caller must not assume an answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreUnavailable;

impl std::fmt::Display for StoreUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("auth store unavailable")
    }
}

impl std::error::Error for StoreUnavailable {}

/// Why a panel assignment failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelError {
    /// The caller is inactive or does not hold panel-assignment (`admin`) permission.
    Forbidden,
    /// The target physician does not belong to the caller's practice.
    UnknownTarget,
    /// The store could not record the assignment.
    Unavailable,
}

/// Why an IANA practice time zone string was rejected.
#[derive(Debug, Clone, thiserror::Error)]
#[error("`{zone}` is not a valid IANA time zone: {source}")]
pub struct InvalidTimeZone {
    /// The rejected time zone string.
    pub zone: String,
    /// The underlying tzdb lookup error.
    #[source]
    pub source: jiff::Error,
}

/// Why creating or verifying a practice record failed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PracticeSetupError {
    /// The practice identifier is empty or malformed.
    #[error("practice identifier `{0}` is invalid")]
    InvalidPracticeId(String),
    /// The practice already exists with a different clock or time zone.
    #[error(
        "practice `{practice_id}` already exists with clock {existing_clock:?} and time zone `{existing_time_zone}`"
    )]
    Conflict {
        /// The conflicting practice identifier.
        practice_id: String,
        /// The clock stored on the existing practice.
        existing_clock: crate::PracticeClock,
        /// The IANA time zone stored on the existing practice.
        existing_time_zone: crate::PracticeTimeZone,
    },
    /// The store could not read or write the practice record.
    #[error("practice setup store unavailable")]
    Unavailable(#[from] StoreUnavailable),
}

/// Why creating a physician account failed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AccountSetupError {
    /// One of the account fields (`practice_id`, `physician_id`, `email`, or `display_name`) is invalid.
    #[error("account setup input invalid: {0}")]
    InvalidInput(String),
    /// The email is already registered or invited in another practice (or under a different account).
    #[error("email `{email}` already belongs to practice `{existing_practice_id}`")]
    EmailConflict {
        /// The conflicting email address.
        email: String,
        /// The practice that already holds the email.
        existing_practice_id: String,
    },
    /// The `physician_id` is already taken by another account in the same practice.
    #[error("physician `{physician_id}` already exists in practice `{practice_id}`")]
    PhysicianIdConflict {
        /// The practice identifier.
        practice_id: String,
        /// The conflicting physician identifier.
        physician_id: String,
    },
    /// An account with this email already exists in the practice with different role, display name, or active standing.
    #[error(
        "account `{email}` already exists in practice `{practice_id}` with different role, name, or active standing"
    )]
    AccountConflict {
        /// The practice identifier.
        practice_id: String,
        /// The conflicting email address.
        email: String,
    },
    /// The store could not read or write the account.
    #[error("account setup store unavailable")]
    Unavailable(#[from] StoreUnavailable),
}
