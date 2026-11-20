//! Why a front-door step failed.

/// Why a session resolution failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    /// No account holds this token.
    Invalid,
    /// The account is removed or deactivated, or its sessions were ended.
    Inactive,
}

/// Why a sign-in or redemption step failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignInError {
    /// The email has no account, the sign-in token is unknown/used/expired,
    /// or the invitation is invalid or already redeemed.
    Invalid,
    /// The account exists but may not sign in or accept new connections.
    Inactive,
}

/// Why an invitation step failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InviteError {
    /// This roster patient already holds an active Connected connection.
    Conflict,
}

/// Why a connection read or ending step failed.
#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    /// No Duet account holds this id, or no active connection matches.
    #[error("no active connection matches this account or roster patient")]
    Unlinked,
    /// Storage failed while persisting or reading connection state.
    #[error("front-door connection storage failed")]
    Storage(#[from] sea_orm::DbErr),
}

/// Why a `"For my doctor"` send, pickup, or connected-record step failed.
#[derive(Debug, thiserror::Error)]
pub enum ForMyDoctorError {
    /// No Duet account holds this id, or the target practice connection is not active.
    #[error("patient account has no active connection to that practice")]
    Unlinked,
    /// Required share or record fields are blank, or target practice is ambiguous.
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
