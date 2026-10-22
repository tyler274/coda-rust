//! An in-memory [`Directory`] for tests and offline fakes.

use std::sync::Mutex;

use crate::{Clinician, Directory, Npi, RegistryError, SearchQuery, parse_response};

/// Synthetic recorded NPPES v2.1 response bodies, for downstream tests and
/// local fixture servers. Every NPI, name, and number in them is invented.
pub mod samples {
    /// A `number=` lookup of an individual (`NPI-1`) internist with a Direct address.
    pub const INDIVIDUAL: &str = include_str!("../tests/fixtures/individual.json");
    /// A `number=` lookup of an organization (`NPI-2`) clinic.
    pub const ORGANIZATION: &str = include_str!("../tests/fixtures/organization.json");
    /// A name search returning two individuals.
    pub const SEARCH: &str = include_str!("../tests/fixtures/search.json");
    /// A `number=` lookup NPPES does not list.
    pub const EMPTY: &str = include_str!("../tests/fixtures/empty.json");
    /// An NPPES `Errors` body (state given without other criteria).
    pub const ERROR: &str = include_str!("../tests/fixtures/error.json");
}

/// Answers lookups from a fixed set of [`Clinician`]s, without network.
///
/// Matching mimics NPPES loosely: case-insensitive, exact unless the value
/// ends in `*` (then prefix), `postal_code` always a prefix, and
/// `taxonomy_description` a substring of any listed taxonomy. The query is
/// validated with the same rules as [`crate::Client`].
#[derive(Debug, Default)]
pub struct FixtureDirectory {
    clinicians: Mutex<Vec<Clinician>>,
    failure: Option<RegistryError>,
}

impl FixtureDirectory {
    /// A directory listing exactly these clinicians.
    pub fn new(clinicians: impl IntoIterator<Item = Clinician>) -> Self {
        Self {
            clinicians: Mutex::new(clinicians.into_iter().collect()),
            failure: None,
        }
    }

    /// A directory built from recorded NPPES response bodies.
    pub fn from_responses<'a>(
        bodies: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, RegistryError> {
        let mut clinicians = Vec::new();
        for body in bodies {
            clinicians.extend(parse_response(body)?);
        }
        Ok(Self::new(clinicians))
    }

    /// The synthetic clinicians from [`samples`]: one individual, one
    /// organization, and the two search results.
    pub fn sample() -> Self {
        Self::from_responses([samples::INDIVIDUAL, samples::ORGANIZATION, samples::SEARCH])
            .expect("bundled NPPES samples parse")
    }

    /// A directory whose every lookup fails with `error`, for testing
    /// product behaviour when NPPES is down or rejects a query.
    pub fn failing(error: RegistryError) -> Self {
        Self {
            clinicians: Mutex::new(Vec::new()),
            failure: Some(error),
        }
    }

    /// Adds or replaces a clinician (matched by NPI).
    pub fn insert(&self, clinician: Clinician) {
        let mut clinicians = self.clinicians.lock().expect("fixture directory lock");
        clinicians.retain(|existing| existing.npi != clinician.npi);
        clinicians.push(clinician);
    }

    /// Removes a clinician, as if NPPES deactivated the NPI.
    pub fn remove(&self, npi: &Npi) {
        self.clinicians
            .lock()
            .expect("fixture directory lock")
            .retain(|existing| &existing.npi != npi);
    }

    /// Every listed clinician, in insertion order.
    pub fn clinicians(&self) -> Vec<Clinician> {
        self.clinicians
            .lock()
            .expect("fixture directory lock")
            .clone()
    }
}

#[async_trait::async_trait]
impl Directory for FixtureDirectory {
    async fn by_number(&self, npi: &Npi) -> Result<Option<Clinician>, RegistryError> {
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        Ok(self
            .clinicians
            .lock()
            .expect("fixture directory lock")
            .iter()
            .find(|clinician| &clinician.npi == npi)
            .cloned())
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<Clinician>, RegistryError> {
        let criteria = query.criteria()?;
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        let skip = query.effective_skip() as usize;
        let limit = query.effective_limit() as usize;
        Ok(self
            .clinicians
            .lock()
            .expect("fixture directory lock")
            .iter()
            .filter(|clinician| {
                criteria
                    .iter()
                    .all(|(field, value)| matches(clinician, field, value))
            })
            .skip(skip)
            .take(limit)
            .cloned()
            .collect())
    }
}

fn matches(clinician: &Clinician, field: &str, wanted: &str) -> bool {
    let addresses = || {
        clinician
            .practice_address
            .iter()
            .chain(clinician.mailing_address.iter())
    };
    let person = clinician.person.as_ref();
    match field {
        "first_name" => person.is_some_and(|person| text_matches(&person.first, wanted)),
        "last_name" => person.is_some_and(|person| text_matches(&person.last, wanted)),
        "organization_name" => clinician.person.is_none() && text_matches(&clinician.name, wanted),
        "city" => addresses().any(|address| text_matches(&address.city, wanted)),
        "state" => addresses().any(|address| address.state.eq_ignore_ascii_case(wanted)),
        "postal_code" => {
            let prefix = wanted.trim_end_matches('*');
            addresses().any(|address| address.postal_code.replace('-', "").starts_with(prefix))
        }
        "taxonomy_description" => {
            let wanted = wanted.trim_end_matches('*').to_lowercase();
            clinician
                .taxonomies
                .iter()
                .any(|taxonomy| taxonomy.description.to_lowercase().contains(&wanted))
        }
        "enumeration_type" => clinician.kind.enumeration_type() == wanted,
        _ => false,
    }
}

fn text_matches(have: &str, wanted: &str) -> bool {
    let have = have.to_lowercase();
    match wanted.strip_suffix('*') {
        Some(prefix) => have.starts_with(&prefix.to_lowercase()),
        None => have == wanted.to_lowercase(),
    }
}
