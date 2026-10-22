//! Public directory records, shaped for products to show and store.
//!
//! Every field comes from the public NPPES record. Text is kept as NPPES
//! publishes it (usually upper case); products choose their own presentation.

use serde::{Deserialize, Serialize};

use crate::Npi;

/// Whether an NPI belongs to a person or an organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClinicianKind {
    /// NPPES enumeration type `NPI-1`: an individual provider.
    Individual,
    /// NPPES enumeration type `NPI-2`: an organization.
    Organization,
}

impl ClinicianKind {
    /// The NPPES `enumeration_type` code.
    pub fn enumeration_type(self) -> &'static str {
        match self {
            Self::Individual => "NPI-1",
            Self::Organization => "NPI-2",
        }
    }

    /// Reads an NPPES `enumeration_type` code.
    pub fn from_enumeration_type(code: &str) -> Option<Self> {
        match code {
            "NPI-1" => Some(Self::Individual),
            "NPI-2" => Some(Self::Organization),
            _ => None,
        }
    }
}

/// The structured name of an individual (`NPI-1`) provider.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PersonName {
    /// Name prefix such as `DR.`, when listed.
    #[serde(default)]
    pub prefix: Option<String>,
    /// Given name.
    pub first: String,
    /// Middle name, when listed.
    #[serde(default)]
    pub middle: Option<String>,
    /// Family name.
    pub last: String,
    /// Name suffix such as `JR.`, when listed.
    #[serde(default)]
    pub suffix: Option<String>,
}

/// One provider taxonomy (specialty) entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Taxonomy {
    /// The NUCC taxonomy code, for example `207R00000X`.
    pub code: String,
    /// The NUCC description, for example `Internal Medicine`.
    pub description: String,
    /// Whether NPPES marks this taxonomy as primary.
    #[serde(default)]
    pub primary: bool,
    /// The state of the associated license, when listed.
    #[serde(default)]
    pub state: Option<String>,
    /// The associated license number, when listed.
    #[serde(default)]
    pub license: Option<String>,
}

/// A postal address with its contact numbers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    /// First street line.
    pub line1: String,
    /// Second street line, when listed.
    #[serde(default)]
    pub line2: Option<String>,
    /// City.
    pub city: String,
    /// State or territory code.
    pub state: String,
    /// ZIP code, formatted `12345` or `12345-6789`.
    pub postal_code: String,
    /// ISO country code, when listed.
    #[serde(default)]
    pub country_code: Option<String>,
    /// Telephone number as published.
    #[serde(default)]
    pub phone: Option<String>,
    /// Fax number as published.
    #[serde(default)]
    pub fax: Option<String>,
}

/// An electronic endpoint published in NPPES.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    /// NPPES `endpointType`, for example `DIRECT`, `FHIR`, `SOAP`, `CONNECT`.
    pub kind: String,
    /// The endpoint address: a Direct mailbox or a URL.
    pub address: String,
    /// Human description of the endpoint type, when listed.
    #[serde(default)]
    pub kind_description: Option<String>,
    /// Human description of the endpoint's use, when listed.
    #[serde(default)]
    pub use_description: Option<String>,
}

impl Endpoint {
    /// Whether this is a Direct secure-messaging address.
    pub fn is_direct(&self) -> bool {
        self.kind.eq_ignore_ascii_case("DIRECT")
    }
}

/// A clinician or organization as listed in the NPI Registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clinician {
    /// The validated NPI.
    pub npi: Npi,
    /// Individual or organization.
    pub kind: ClinicianKind,
    /// Display name: the person's name parts joined, or the organization name.
    pub name: String,
    /// Structured name; present only for individuals.
    #[serde(default)]
    pub person: Option<PersonName>,
    /// Credential text such as `MD` or `LICSW`; individuals only.
    #[serde(default)]
    pub credential: Option<String>,
    /// The taxonomy marked primary, or the first one listed.
    #[serde(default)]
    pub primary_taxonomy: Option<Taxonomy>,
    /// All listed taxonomies, in NPPES order.
    #[serde(default)]
    pub taxonomies: Vec<Taxonomy>,
    /// The primary practice location (`LOCATION` address).
    #[serde(default)]
    pub practice_address: Option<Address>,
    /// The mailing address (`MAILING` address).
    #[serde(default)]
    pub mailing_address: Option<Address>,
    /// Practice telephone number.
    #[serde(default)]
    pub phone: Option<String>,
    /// Practice fax number.
    #[serde(default)]
    pub fax: Option<String>,
    /// Published electronic endpoints, including Direct addresses.
    #[serde(default)]
    pub endpoints: Vec<Endpoint>,
    /// NPPES `last_updated` date (`YYYY-MM-DD`), when listed.
    #[serde(default)]
    pub last_updated: Option<String>,
}

impl Clinician {
    /// The Direct secure-messaging addresses, in NPPES order.
    pub fn direct_addresses(&self) -> impl Iterator<Item = &str> {
        self.endpoints
            .iter()
            .filter(|endpoint| endpoint.is_direct())
            .map(|endpoint| endpoint.address.as_str())
    }
}
