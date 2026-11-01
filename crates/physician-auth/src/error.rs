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
