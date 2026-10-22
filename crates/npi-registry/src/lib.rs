//! Product-neutral client for the public CMS NPI Registry (NPPES API v2.1).
//!
//! This crate holds only public directory data: who a clinician or
//! organization is, what they practice, and how to reach them. It has no
//! clinical types, accounts, or storage. Duet and the physician product each
//! keep their own copy of what they look up.
//!
//! - [`Npi`] validates the 10-digit Luhn check digit (`80840` prefix).
//! - [`Clinician`] is the typed record; it serializes so products can store a
//!   snapshot as JSON.
//! - [`parse_response`] and [`parse_record`] turn NPPES JSON into
//!   [`Clinician`]s without I/O.
//! - [`Directory`] is the lookup seam. [`Client`] calls NPPES (server builds
//!   only); [`FixtureDirectory`] answers from memory for tests and fakes.
//!
//! Search terms are never logged or put in error messages: which clinicians a
//! person looks for is sensitive.
#![deny(missing_docs)]

#[cfg(not(target_arch = "wasm32"))]
mod client;
mod fixture;
mod model;
mod npi;
mod parse;
mod query;

use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
pub use client::{Client, ClientConfig, DEFAULT_BASE_URL};
pub use fixture::{FixtureDirectory, samples};
pub use model::{Address, Clinician, ClinicianKind, Endpoint, PersonName, Taxonomy};
pub use npi::{Npi, NpiError};
pub use parse::{parse_record, parse_response};
pub use query::{DEFAULT_LIMIT, MAX_LIMIT, MAX_SKIP, SearchQuery};

/// One error entry from an NPPES `Errors` response.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RegistryMessage {
    /// NPPES's description, for example `Field state requires additional search criteria`.
    pub description: String,
    /// The field NPPES names, when present.
    #[serde(default)]
    pub field: Option<String>,
    /// NPPES's error number, when present.
    #[serde(default)]
    pub code: Option<String>,
}

/// Why a directory lookup failed.
///
/// Messages name fields, never the values searched for.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RegistryError {
    /// The query breaks an NPPES rule; nothing was sent.
    #[error("invalid NPI Registry query: {0}")]
    InvalidQuery(String),
    /// NPPES answered with an `Errors` body.
    #[error("NPI Registry rejected the query: {}", describe(.0))]
    Rejected(Vec<RegistryMessage>),
    /// NPPES could not be reached, timed out, or answered a non-success status.
    #[error("NPI Registry unavailable: {0}")]
    Unavailable(String),
    /// NPPES answered with a body this crate could not read.
    #[error("NPI Registry response could not be read: {0}")]
    Decode(String),
}

fn describe(messages: &[RegistryMessage]) -> String {
    messages
        .iter()
        .map(|message| message.description.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

/// A source of NPI Registry records.
///
/// Products depend on this trait so tests can inject a [`FixtureDirectory`]
/// instead of calling NPPES.
#[async_trait::async_trait]
pub trait Directory: Send + Sync {
    /// Looks up one NPI. `Ok(None)` means NPPES does not list it.
    async fn by_number(&self, npi: &Npi) -> Result<Option<Clinician>, RegistryError>;

    /// Searches by criteria. The query is validated first; at most
    /// [`SearchQuery::effective_limit`] records are returned.
    async fn search(&self, query: &SearchQuery) -> Result<Vec<Clinician>, RegistryError>;
}

#[async_trait::async_trait]
impl<T: Directory + ?Sized> Directory for Arc<T> {
    async fn by_number(&self, npi: &Npi) -> Result<Option<Clinician>, RegistryError> {
        (**self).by_number(npi).await
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<Clinician>, RegistryError> {
        (**self).search(query).await
    }
}

#[async_trait::async_trait]
impl<T: Directory + ?Sized> Directory for Box<T> {
    async fn by_number(&self, npi: &Npi) -> Result<Option<Clinician>, RegistryError> {
        (**self).by_number(npi).await
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<Clinician>, RegistryError> {
        (**self).search(query).await
    }
}
