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

/// Why an invitation creation, resend, or cancel step failed.
#[derive(Debug, thiserror::Error)]
pub enum InviteError {
    /// This roster patient already holds an active Connected connection.
    #[error("this roster patient already has an active connection")]
    Conflict,
    /// The open invitation is locked after too many wrong date-of-birth answers.
    #[error("this invitation is locked; cancel it before inviting again")]
    Locked,
    /// No open invitation exists for this roster patient.
    #[error("no open invitation exists for this roster patient")]
    NotFound,
    /// Invitation input failed validation.
    #[error("invalid invitation input: {0}")]
    InvalidInput(&'static str),
    /// Storage failed while persisting or reading invitation state.
    #[error("front-door invitation storage failed")]
    Storage(#[from] sea_orm::DbErr),
}

/// Why inspecting an invitation or checking date of birth failed.
#[derive(Debug, thiserror::Error)]
pub enum InvitationCheckError {
    /// No open invitation matches that link token or short code.
    #[error("this invitation is no longer open")]
    NotOpen,
    /// The invitation locked after too many wrong date-of-birth answers.
    #[error("this invitation is locked after too many wrong date-of-birth answers")]
    Locked,
    /// The entered date of birth did not match the practice chart.
    #[error("date of birth does not match ({attempts_remaining} attempts remaining)")]
    WrongBirthDate {
        /// Wrong-answer budget remaining before lock.
        attempts_remaining: u8,
    },
    /// The patient does not meet the 18+ age requirement for Connected Care.
    #[error("Duet Connected Care requires being 18 or older")]
    Underage,
    /// The supplied date of birth is malformed.
    #[error("enter a valid date of birth (YYYY-MM-DD)")]
    InvalidBirthDate,
    /// Storage failed while checking invitation state.
    #[error("front-door invitation storage failed")]
    Storage(#[from] sea_orm::DbErr),
}

/// Why accepting an invitation via Share failed.
#[derive(Debug, thiserror::Error)]
pub enum InvitationAcceptError {
    /// Checking the open invitation, DOB, lock, or age failed.
    #[error(transparent)]
    Check(#[from] InvitationCheckError),
    /// The signed-in Duet account is deactivated or cannot accept connections.
    #[error("this account is inactive")]
    InactiveAccount,
    /// Another account already holds an active connection for this roster patient.
    #[error("this roster patient is already connected")]
    Conflict,
    /// Storage failed while accepting the invitation.
    #[error("front-door invitation acceptance failed")]
    Storage(#[from] sea_orm::DbErr),
}

impl PartialEq for InviteError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Conflict, Self::Conflict)
            | (Self::Locked, Self::Locked)
            | (Self::NotFound, Self::NotFound) => true,
            (Self::InvalidInput(a), Self::InvalidInput(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for InviteError {}

impl PartialEq for InvitationCheckError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NotOpen, Self::NotOpen)
            | (Self::Locked, Self::Locked)
            | (Self::Underage, Self::Underage)
            | (Self::InvalidBirthDate, Self::InvalidBirthDate) => true,
            (
                Self::WrongBirthDate {
                    attempts_remaining: a,
                },
                Self::WrongBirthDate {
                    attempts_remaining: b,
                },
            ) => a == b,
            _ => false,
        }
    }
}

impl Eq for InvitationCheckError {}

impl PartialEq for InvitationAcceptError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Check(a), Self::Check(b)) => a == b,
            (Self::InactiveAccount, Self::InactiveAccount) | (Self::Conflict, Self::Conflict) => {
                true
            }
            _ => false,
        }
    }
}

impl Eq for InvitationAcceptError {}

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
