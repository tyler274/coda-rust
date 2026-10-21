//! Operational settings are separate to preserve the public page Config API.
use crate::access::AccessConfig;
use std::{net::SocketAddr, time::Duration};

/// Secrets deliberately do not implement Debug.
#[derive(Clone)]
pub struct RuntimeConfig {
    pub environment: String,
    pub staging: bool,
    pub access: AccessConfig,
    pub health_token: Option<String>,
    pub release_sha: Option<String>,
    pub max_concurrency: usize,
    pub request_timeout: Duration,
    /// Optional separate loopback-only probe listener; never mounted on public HTTP.
    pub internal_probe_addr: Option<SocketAddr>,
}
impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            environment: "unknown".into(),
            staging: false,
            access: AccessConfig::default(),
            health_token: None,
            release_sha: None,
            max_concurrency: 1024,
            request_timeout: Duration::from_secs(5),
            internal_probe_addr: None,
        }
    }
}
impl RuntimeConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        fn nonempty(name: &str) -> Option<String> {
            std::env::var(name)
                .ok()
                .filter(|value| !value.trim().is_empty())
        }
        let environment = nonempty("APP_ENV")
            .or_else(|| nonempty("NODE_ENV"))
            .unwrap_or_else(|| "unknown".into());
        let environment = environment.trim().to_owned();
        let staging = environment.eq_ignore_ascii_case("staging");
        let issuer = match nonempty("CLOUDFLARE_ACCESS_TEAM_DOMAIN") {
            Some(value) => Some(AccessConfig::normalize_issuer(&value)?),
            None => None,
        };
        let mut config = Self {
            environment,
            staging,
            access: AccessConfig {
                issuer,
                audience: nonempty("CLOUDFLARE_ACCESS_AUD"),
                service_client_id: nonempty("CF_ACCESS_CLIENT_ID"),
                service_client_secret: nonempty("CF_ACCESS_CLIENT_SECRET"),
            },
            health_token: nonempty("STAGING_HEALTH_TOKEN"),
            release_sha: nonempty("RELEASE_SHA").or_else(|| nonempty("VERCEL_GIT_COMMIT_SHA")),
            ..Default::default()
        };
        if let Some(value) = nonempty("PUBLIC_SITE_MAX_CONCURRENCY") {
            config.max_concurrency = value.parse()?;
        }
        if !(1..=65536).contains(&config.max_concurrency) {
            return Err("PUBLIC_SITE_MAX_CONCURRENCY must be 1..65536".into());
        }
        if let Some(value) = nonempty("PUBLIC_SITE_REQUEST_TIMEOUT_MS") {
            let millis: u64 = value.parse()?;
            if !(100..=30000).contains(&millis) {
                return Err("PUBLIC_SITE_REQUEST_TIMEOUT_MS must be 100..30000".into());
            }
            config.request_timeout = Duration::from_millis(millis);
        }
        if let Some(value) = nonempty("PUBLIC_SITE_INTERNAL_PROBE_ADDR") {
            let addr: SocketAddr = value.parse()?;
            if !addr.ip().is_loopback() {
                return Err("PUBLIC_SITE_INTERNAL_PROBE_ADDR must bind loopback only".into());
            }
            config.internal_probe_addr = Some(addr);
        }
        // Missing credentials do NOT disable staging. It remains deny-all until
        // configured. Malformed issuer configuration aborts startup above.
        Ok(config)
    }
}
