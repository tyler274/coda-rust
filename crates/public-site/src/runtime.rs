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
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    fn from_lookup(
        lookup: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let nonempty = |name: &str| lookup(name).filter(|value| !value.trim().is_empty());
        let environment = nonempty("APP_ENV")
            .or_else(|| nonempty("NODE_ENV"))
            .unwrap_or_else(|| "unknown".into());
        let environment = environment.trim().to_owned();
        let staging = environment.eq_ignore_ascii_case("staging");
        let access = AccessConfig {
            audience: nonempty("PUBLIC_SITE_IAP_AUDIENCE"),
        };
        if staging {
            access.validate()?;
        }
        let mut config = Self {
            environment,
            staging,
            access,
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
        // Missing or malformed staging audience aborts startup above. Local and
        // public production do not require IAP configuration.
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn load(values: &[(&str, &str)]) -> Result<RuntimeConfig, Box<dyn std::error::Error>> {
        RuntimeConfig::from_lookup(|name| {
            values
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        })
    }
    #[test]
    fn staging_requires_well_formed_direct_cloud_run_audience() {
        let good = crate::access::tests::AUDIENCE;
        assert!(
            load(&[("APP_ENV", "StAgInG"), ("PUBLIC_SITE_IAP_AUDIENCE", good)])
                .unwrap()
                .staging
        );
        assert!(
            load(&[("NODE_ENV", "staging"), ("PUBLIC_SITE_IAP_AUDIENCE", good)])
                .unwrap()
                .staging
        );
        for bad in [
            "",
            " ",
            "audience",
            "https://service.run.app",
            "123.apps.googleusercontent.com",
            "/projects/123/global/backendServices/456",
            "/projects/id/locations/us-east1/services/site",
            "/projects/123/locations//services/site",
            "/projects/123/locations/us-east1/services/site/",
            "/projects/123/locations/us-east1/services/site?x=1",
            "/projects/123/locations/us-east1/services/../site",
            &format!(" {good}"),
            &format!("{good} "),
        ] {
            assert!(
                load(&[("APP_ENV", "staging"), ("PUBLIC_SITE_IAP_AUDIENCE", bad)]).is_err(),
                "{bad}"
            );
        }
        assert!(load(&[("APP_ENV", "staging")]).is_err());
        assert!(
            load(&[
                ("APP_ENV", "staging"),
                ("CF_ACCESS_CLIENT_ID", "id"),
                ("CF_ACCESS_CLIENT_SECRET", "secret"),
                ("CLOUDFLARE_ACCESS_TEAM_DOMAIN", "team.cloudflareaccess.com"),
                ("CLOUDFLARE_ACCESS_AUD", "legacy")
            ])
            .is_err()
        );
    }
    #[test]
    fn local_production_and_probe_contract_remain_explicit() {
        for env in ["local", "production", "unknown"] {
            assert!(!load(&[("APP_ENV", env)]).unwrap().staging);
        }
        assert!(load(&[("PUBLIC_SITE_INTERNAL_PROBE_ADDR", "0.0.0.0:8081")]).is_err());
        assert!(load(&[("PUBLIC_SITE_INTERNAL_PROBE_ADDR", "127.0.0.1:8081")]).is_ok());
        assert!(load(&[("PUBLIC_SITE_MAX_CONCURRENCY", "0")]).is_err());
        assert!(load(&[("PUBLIC_SITE_REQUEST_TIMEOUT_MS", "30001")]).is_err());
    }
}
