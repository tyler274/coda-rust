//! Why a front-door step failed.

/// Why a session resolution failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    /// No account holds this token, or it expired.
    Invalid,
    /// The account is removed or deactivated.
    Inactive,
}

/// Why a sign-in or redemption step failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignInError {
    /// The email has no account, or the token is unknown, used, or expired.
    Invalid,
    /// The account exists but may not sign in.
    Inactive,
}

/// Why an invitation step failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InviteError {
    /// The practice already invited this patient, or the roster patient is
    /// unknown to the caller, or the email already holds a front-door account.
    Conflict,
}

/// Why a link or consent step failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkError {
    /// No Duet account holds this id, or the account carries no link.
    Unlinked,
}
