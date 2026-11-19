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

/// Why a `"For my doctor"` send, pickup, or consented record step failed.
#[derive(Debug, thiserror::Error)]
pub enum ForMyDoctorError {
    /// No Duet account holds this id, or the account carries no practice link.
    #[error("patient account has no linked practice")]
    Unlinked,
    /// Sharing consent is absent or withdrawn.
    #[error("patient sharing consent is required")]
    ConsentRequired,
    /// Required share or record fields are blank or invalid.
    #[error("invalid connected-care payload: {0}")]
    InvalidInput(&'static str),
    /// A share with the same key already exists with different content.
    #[error("a For my doctor share with this key already exists with different content")]
    Conflict,
    /// No matching share was found for this practice.
    #[error("For my doctor share not found")]
    NotFound,
    /// Storage failed while persisting or reading connected-care state.
    #[error("front-door connected-care storage failed")]
    Storage(#[from] sea_orm::DbErr),
}
