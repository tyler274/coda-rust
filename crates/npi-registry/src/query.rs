//! NPPES search criteria and the validation NPPES applies to them.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{ClinicianKind, RegistryError};

/// The largest page NPPES is asked for. NPPES itself allows 200.
pub const MAX_LIMIT: u32 = 50;
/// The page size used when a query sets no limit (the NPPES default).
pub const DEFAULT_LIMIT: u32 = 10;
/// The largest `skip` NPPES accepts.
pub const MAX_SKIP: u32 = 1000;

/// Search criteria for the NPI Registry.
///
/// Values are matched by NPPES case-insensitively. A value may end in one `*`
/// wildcard after at least two characters (`smi*`). `Debug` prints only which
/// fields are set, never the values: which clinicians a person looks for is
/// sensitive, so search terms must stay out of logs.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Individual given name.
    #[serde(default)]
    pub first_name: Option<String>,
    /// Individual family name.
    #[serde(default)]
    pub last_name: Option<String>,
    /// Organization name.
    #[serde(default)]
    pub organization_name: Option<String>,
    /// City of any listed address.
    #[serde(default)]
    pub city: Option<String>,
    /// Two-letter state code. Not sufficient on its own.
    #[serde(default)]
    pub state: Option<String>,
    /// ZIP code; five digits match as a prefix of ZIP+4.
    #[serde(default)]
    pub postal_code: Option<String>,
    /// Taxonomy (specialty) description, for example `Cardiology`.
    #[serde(default)]
    pub taxonomy_description: Option<String>,
    /// Restrict to individuals or organizations. Not sufficient on its own.
    #[serde(default)]
    pub kind: Option<ClinicianKind>,
    /// Page size; defaults to [`DEFAULT_LIMIT`] and is capped at [`MAX_LIMIT`].
    #[serde(default)]
    pub limit: Option<u32>,
    /// Results to skip, for paging; capped at [`MAX_SKIP`].
    #[serde(default)]
    pub skip: Option<u32>,
}

impl fmt::Debug for SearchQuery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut set: Vec<&str> = Vec::new();
        for (name, value) in self.text_fields() {
            if value.is_some_and(|value| !value.trim().is_empty()) {
                set.push(name);
            }
        }
        formatter
            .debug_struct("SearchQuery")
            .field("fields_set", &set)
            .field("kind", &self.kind)
            .field("limit", &self.limit)
            .field("skip", &self.skip)
            .finish()
    }
}

impl SearchQuery {
    fn text_fields(&self) -> [(&'static str, Option<&str>); 7] {
        [
            ("first_name", self.first_name.as_deref()),
            ("last_name", self.last_name.as_deref()),
            ("organization_name", self.organization_name.as_deref()),
            ("city", self.city.as_deref()),
            ("state", self.state.as_deref()),
            ("postal_code", self.postal_code.as_deref()),
            ("taxonomy_description", self.taxonomy_description.as_deref()),
        ]
    }

    /// The page size that will be requested: the limit, defaulted and capped.
    pub fn effective_limit(&self) -> u32 {
        self.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT)
    }

    /// The skip that will be requested, capped at [`MAX_SKIP`].
    pub fn effective_skip(&self) -> u32 {
        self.skip.unwrap_or(0).min(MAX_SKIP)
    }

    /// Checks the query against the NPPES rules without calling NPPES.
    ///
    /// Error messages name the offending field, never its value.
    pub fn validate(&self) -> Result<(), RegistryError> {
        self.normalized().map(|_| ())
    }

    /// Validated, trimmed criteria as `(NPPES parameter, value)` pairs,
    /// excluding `version`, `limit`, and `skip`.
    pub fn criteria(&self) -> Result<Vec<(&'static str, String)>, RegistryError> {
        self.normalized()
    }

    fn normalized(&self) -> Result<Vec<(&'static str, String)>, RegistryError> {
        let mut pairs = Vec::new();
        let mut sufficient = false;
        for (name, value) in self.text_fields() {
            let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
                continue;
            };
            check_wildcard(name, value)?;
            let value = match name {
                "state" => {
                    if value.len() != 2 || !value.bytes().all(|byte| byte.is_ascii_alphabetic()) {
                        return Err(invalid("state must be a two-letter code"));
                    }
                    value.to_ascii_uppercase()
                }
                "postal_code" => {
                    let digits = value.trim_end_matches('*');
                    if digits.is_empty()
                        || digits.len() > 9
                        || !digits.bytes().all(|byte| byte.is_ascii_digit())
                    {
                        return Err(invalid("postal_code must be up to 9 digits"));
                    }
                    value.to_owned()
                }
                _ => {
                    if value.chars().count() > 100 {
                        return Err(RegistryError::InvalidQuery(format!("{name} is too long")));
                    }
                    value.to_owned()
                }
            };
            if name != "state" {
                sufficient = true;
            }
            pairs.push((name, value));
        }
        if !sufficient {
            return Err(invalid(
                "a search needs at least one criterion besides state and kind",
            ));
        }
        if let Some(kind) = self.kind {
            pairs.push(("enumeration_type", kind.enumeration_type().to_owned()));
        }
        Ok(pairs)
    }
}

fn check_wildcard(name: &str, value: &str) -> Result<(), RegistryError> {
    let Some(star) = value.find('*') else {
        return Ok(());
    };
    if star != value.len() - 1 {
        return Err(RegistryError::InvalidQuery(format!(
            "{name} may only use `*` as a trailing wildcard"
        )));
    }
    if value[..star].chars().count() < 2 {
        return Err(RegistryError::InvalidQuery(format!(
            "{name} needs at least two characters before `*`"
        )));
    }
    Ok(())
}

fn invalid(message: &str) -> RegistryError {
    RegistryError::InvalidQuery(message.to_owned())
}
