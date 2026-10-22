//! The HTTP [`Client`] for NPPES. Server builds only.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::{Clinician, Directory, Npi, RegistryError, SearchQuery, parse_response};

/// The public NPPES API endpoint.
pub const DEFAULT_BASE_URL: &str = "https://npiregistry.cms.hhs.gov/api/";
const API_VERSION: &str = "2.1";

/// How a [`Client`] reaches NPPES.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    /// The API URL, without query. Tests point this at a local server.
    pub base_url: String,
    /// The whole-request timeout.
    pub timeout: Duration,
    /// How long a looked-up record is reused before NPPES is asked again.
    /// Zero disables the cache.
    pub cache_ttl: Duration,
    /// The most records the cache holds.
    pub cache_capacity: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_owned(),
            timeout: Duration::from_secs(10),
            cache_ttl: Duration::from_secs(15 * 60),
            cache_capacity: 1024,
        }
    }
}

/// Calls the NPPES API over HTTPS.
///
/// Requests time out, never follow redirects, and request at most
/// [`crate::MAX_LIMIT`] records. Records are cached by NPI for
/// [`ClientConfig::cache_ttl`]; searches are not cached, but the records they
/// return warm the NPI cache. Nothing is logged, and transport errors are
/// stripped of the request URL so search terms cannot leak through them.
#[derive(Debug)]
pub struct Client {
    http: reqwest::Client,
    config: ClientConfig,
    cache: Mutex<HashMap<Npi, (Instant, Clinician)>>,
}

impl Client {
    /// A client for the public NPPES API with default settings.
    pub fn new() -> Result<Self, RegistryError> {
        Self::with_config(ClientConfig::default())
    }

    /// A client for an NPPES-compatible API at `base_url`, for example a
    /// local fixture server in tests.
    pub fn with_base_url(base_url: impl Into<String>) -> Result<Self, RegistryError> {
        Self::with_config(ClientConfig {
            base_url: base_url.into(),
            ..ClientConfig::default()
        })
    }

    /// A client with explicit settings.
    pub fn with_config(config: ClientConfig) -> Result<Self, RegistryError> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!("coda-npi-registry/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|error| RegistryError::Unavailable(error.without_url().to_string()))?;
        Ok(Self {
            http,
            config,
            cache: Mutex::new(HashMap::new()),
        })
    }

    /// The settings this client was built with.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Drops every cached record.
    pub fn clear_cache(&self) {
        self.cache.lock().expect("npi cache lock").clear();
    }

    async fn get(&self, params: &[(&str, String)]) -> Result<Vec<Clinician>, RegistryError> {
        let response = self
            .http
            .get(&self.config.base_url)
            .query(&[("version", API_VERSION)])
            .query(params)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .map_err(|error| RegistryError::Unavailable(error.without_url().to_string()))?;
        let status = response.status();
        if !status.is_success() {
            return Err(RegistryError::Unavailable(format!(
                "HTTP {}",
                status.as_u16()
            )));
        }
        let body = response
            .text()
            .await
            .map_err(|error| RegistryError::Unavailable(error.without_url().to_string()))?;
        let clinicians = parse_response(&body)?;
        self.remember(&clinicians);
        Ok(clinicians)
    }

    fn cached(&self, npi: &Npi) -> Option<Clinician> {
        if self.config.cache_ttl.is_zero() {
            return None;
        }
        let mut cache = self.cache.lock().expect("npi cache lock");
        match cache.get(npi) {
            Some((stored, clinician)) if stored.elapsed() < self.config.cache_ttl => {
                Some(clinician.clone())
            }
            Some(_) => {
                cache.remove(npi);
                None
            }
            None => None,
        }
    }

    fn remember(&self, clinicians: &[Clinician]) {
        if self.config.cache_ttl.is_zero() || self.config.cache_capacity == 0 {
            return;
        }
        let ttl = self.config.cache_ttl;
        let mut cache = self.cache.lock().expect("npi cache lock");
        let now = Instant::now();
        for clinician in clinicians {
            if cache.len() >= self.config.cache_capacity && !cache.contains_key(&clinician.npi) {
                cache.retain(|_, (stored, _)| stored.elapsed() < ttl);
                if cache.len() >= self.config.cache_capacity
                    && let Some(oldest) = cache
                        .iter()
                        .min_by_key(|(_, (stored, _))| *stored)
                        .map(|(npi, _)| npi.clone())
                {
                    cache.remove(&oldest);
                }
            }
            cache.insert(clinician.npi.clone(), (now, clinician.clone()));
        }
    }
}

#[async_trait::async_trait]
impl Directory for Client {
    async fn by_number(&self, npi: &Npi) -> Result<Option<Clinician>, RegistryError> {
        if let Some(clinician) = self.cached(npi) {
            return Ok(Some(clinician));
        }
        let clinicians = self.get(&[("number", npi.to_string())]).await?;
        Ok(clinicians
            .into_iter()
            .find(|clinician| &clinician.npi == npi))
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<Clinician>, RegistryError> {
        let mut params = query.criteria()?;
        let limit = query.effective_limit();
        params.push(("limit", limit.to_string()));
        let skip = query.effective_skip();
        if skip > 0 {
            params.push(("skip", skip.to_string()));
        }
        let mut clinicians = self.get(&params).await?;
        clinicians.truncate(limit as usize);
        Ok(clinicians)
    }
}
