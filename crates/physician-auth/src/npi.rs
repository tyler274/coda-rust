//! Self-asserted public-directory association, never identity or credential proof.
//!
//! NPPES publishes directory records, not evidence that the signed-in account
//! controls that identity. Name matching (including an administrator's bypass)
//! grants no session, practice, panel, prescribing, or other account authority.
use npi_registry::{Clinician, ClinicianKind};
use serde::{Deserialize, Serialize};
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// A directory snapshot associated by the account holder, not independently verified.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpiAssociation {
    /// Public NPPES snapshot fetched by the server.
    pub clinician: Clinician,
    /// Unix second the directory lookup was checked, not an identity verification time.
    pub lookup_checked_at: u64,
    /// Whether the normalized directory family name appears in the account display name.
    pub name_matched: bool,
}

/// Failure to associate or clear a public-directory NPI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpiAssociationError {
    /// No active account exists for this identity.
    Inactive,
    /// Only an individual directory record can be associated.
    NotIndividual,
    /// The account display name does not contain the directory family name.
    NameMismatch,
    /// Only an administrator may bypass the name consistency check.
    Forbidden,
    /// Another account already associated this NPI (not proof of its ownership).
    Conflict,
    /// Persistence failed; no success may be reported.
    Unavailable,
}

pub(crate) fn association(
    account: &crate::Account,
    clinician: Clinician,
    now: u64,
    override_name: bool,
) -> Result<NpiAssociation, NpiAssociationError> {
    if !account.active {
        return Err(NpiAssociationError::Inactive);
    }
    if clinician.kind != ClinicianKind::Individual {
        return Err(NpiAssociationError::NotIndividual);
    }
    if override_name && !account.admin {
        return Err(NpiAssociationError::Forbidden);
    }
    let normalized = |value: &str| -> Vec<String> {
        value
            .nfd()
            .filter(|c| !is_combining_mark(*c))
            .collect::<String>()
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect()
    };
    let display = normalized(&account.display_name);
    let last = clinician
        .person
        .as_ref()
        .map(|person| normalized(&person.last))
        .unwrap_or_default();
    let name_matched = !last.is_empty() && display.windows(last.len()).any(|part| part == last);
    if !name_matched && !override_name {
        return Err(NpiAssociationError::NameMismatch);
    }
    Ok(NpiAssociation {
        clinician,
        lookup_checked_at: now,
        name_matched,
    })
}
