//! Operational settings are separate to preserve the public page Config API.
use crate::access::AccessConfig;
use std::{net::SocketAddr, time::Duration};

/// Secrets deliberately do not implement Debug.
#[derive(Clone)]
pub struct RuntimeConfig {
    pub environment: String,
    pub staging: bool,
    pub access: AccessConfig,
    /// Present only when `PHYSICIAN_APP_URL` names a physician upstream. It alone
    /// decides whether this deployment offers sign-in: the nav link and the
    /// `/physician/` mount both follow it.
    pub physician_proxy: Option<crate::proxy::ProxyConfig>,
    /// Present only when `OBSERVABILITY_RELAY_URL` names the staging relay. It
    /// alone mounts `/grafana/` and `/langfuse/`, signed with the same
    /// `PHYSICIAN_PROXY_SERVICE_ACCOUNT` identity as the physician mount.
    pub observability_proxy: Option<crate::proxy::ProxyConfig>,
    /// `LANGFUSE_SIGN_IN`: `password` (the default) or `sso`, matching
    /// whether the `observability` root deploys `langfuse-sso`. Only with the
    /// relay.
    pub langfuse_sign_in: crate::proxy::LangfuseSignIn,
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
            physician_proxy: None,
            observability_proxy: None,
            langfuse_sign_in: crate::proxy::LangfuseSignIn::default(),
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
        let deployed = staging || environment.eq_ignore_ascii_case("production");
        let public_origin = || -> Result<String, Box<dyn std::error::Error>> {
            if let Some(origin) = nonempty("PUBLIC_SITE_ORIGIN") {
                return Ok(origin);
            }
            Ok(if staging {
                "https://staging.codahealth.ai".to_owned()
            } else if deployed {
                crate::site::SITE_ORIGIN.to_owned()
            } else {
                // Wildcard bind addresses are not browser destinations. Keep the
                // image's bare local startup usable without relaxing URL validation.
                let mut address: SocketAddr = nonempty("PUBLIC_SITE_LISTEN_ADDR")
                    .unwrap_or_else(|| crate::site::DEFAULT_LISTEN_ADDR.into())
                    .parse()?;
                if address.ip().is_unspecified() {
                    address.set_ip(if address.is_ipv4() {
                        std::net::Ipv4Addr::LOCALHOST.into()
                    } else {
                        std::net::Ipv6Addr::LOCALHOST.into()
                    });
                }
                format!("http://{address}")
            })
        };
        let proxy = |name: &str| {
            nonempty(name)
                .map(|upstream| -> Result<_, Box<dyn std::error::Error>> {
                    Ok(crate::proxy::ProxyConfig::new(
                        &upstream,
                        &public_origin()?,
                        nonempty("PHYSICIAN_PROXY_SERVICE_ACCOUNT"),
                        deployed,
                        staging,
                    )?)
                })
                .transpose()
        };
        let physician_proxy = proxy("PHYSICIAN_APP_URL")?;
        let observability_proxy = proxy("OBSERVABILITY_RELAY_URL")?;
        if observability_proxy.is_some() && !staging {
            return Err("OBSERVABILITY_RELAY_URL is staging-only".into());
        }
        let langfuse_sign_in = match nonempty("LANGFUSE_SIGN_IN").as_deref() {
            None | Some("password") => crate::proxy::LangfuseSignIn::Password,
            Some("sso") => crate::proxy::LangfuseSignIn::Sso,
            Some(_) => return Err("LANGFUSE_SIGN_IN must be password or sso".into()),
        };
        if langfuse_sign_in == crate::proxy::LangfuseSignIn::Sso && observability_proxy.is_none() {
            return Err("LANGFUSE_SIGN_IN=sso needs OBSERVABILITY_RELAY_URL".into());
        }
        let mut config = Self {
            physician_proxy,
            observability_proxy,
            langfuse_sign_in,
            environment,
            staging,
            access,
            health_token: nonempty("STAGING_HEALTH_TOKEN"),
            release_sha: nonempty("RELEASE_SHA"),
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
                .or_else(|| match name {
                    "PHYSICIAN_APP_URL" => Some("https://physician.example.test".into()),
                    "PHYSICIAN_PROXY_SERVICE_ACCOUNT" => {
                        Some("public-site@project.iam.gserviceaccount.com".into())
                    }
                    _ => None,
                })
        })
    }
    #[test]
    fn bare_container_bind_address_uses_a_loopback_browser_origin() {
        for (bind, origin) in [
            ("0.0.0.0:8080", "http://127.0.0.1:8080"),
            ("[::]:8080", "http://[::1]:8080"),
        ] {
            // Match the image defaults: no APP_ENV or credentials.
            let config = RuntimeConfig::from_lookup(|name| match name {
                "PUBLIC_SITE_LISTEN_ADDR" => Some(bind.to_owned()),
                "PHYSICIAN_APP_URL" => Some("http://127.0.0.1:8790".into()),
                _ => None,
            })
            .unwrap();
            let proxy = config.physician_proxy.unwrap();
            assert_eq!(proxy.public_origin, origin);
            assert!(proxy.service_account.is_none());
        }
        let config = RuntimeConfig::from_lookup(|name| match name {
            "PUBLIC_SITE_LISTEN_ADDR" => Some("0.0.0.0:8080".into()),
            "PUBLIC_SITE_ORIGIN" => Some("http://localhost:9090".into()),
            "PHYSICIAN_APP_URL" => Some("http://127.0.0.1:8790".into()),
            _ => None,
        })
        .unwrap();
        assert_eq!(
            config.physician_proxy.unwrap().public_origin,
            "http://localhost:9090"
        );
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
    fn staging_requires_proxy_signer_and_deployed_upstream_https() {
        let load_proxy = |upstream: &str, signer: Option<&str>| {
            RuntimeConfig::from_lookup(|name| match name {
                "APP_ENV" => Some("staging".into()),
                "PUBLIC_SITE_IAP_AUDIENCE" => Some(crate::access::tests::AUDIENCE.into()),
                "PHYSICIAN_APP_URL" => Some(upstream.into()),
                "PHYSICIAN_PROXY_SERVICE_ACCOUNT" => signer.map(str::to_owned),
                _ => None,
            })
        };
        assert!(load_proxy("https://physician.example.test", None).is_err());
        assert!(
            load_proxy(
                "http://127.0.0.1:8790",
                Some("site@project.iam.gserviceaccount.com")
            )
            .is_err()
        );
        let config = load_proxy(
            "https://physician.example.test",
            Some("site@project.iam.gserviceaccount.com"),
        )
        .unwrap();
        assert_eq!(
            config.physician_proxy.unwrap().public_origin,
            "https://staging.codahealth.ai"
        );
    }

    #[test]
    fn sign_in_exists_only_with_a_physician_upstream() {
        for env in ["local", "production"] {
            let config =
                RuntimeConfig::from_lookup(|name| (name == "APP_ENV").then(|| env.to_owned()))
                    .unwrap();
            assert!(config.physician_proxy.is_none(), "{env}");
        }
        let staging = RuntimeConfig::from_lookup(|name| match name {
            "APP_ENV" => Some("staging".into()),
            "PUBLIC_SITE_IAP_AUDIENCE" => Some(crate::access::tests::AUDIENCE.into()),
            _ => None,
        })
        .unwrap();
        assert!(staging.physician_proxy.is_none());
        assert!(
            load(&[("APP_ENV", "production")])
                .unwrap()
                .physician_proxy
                .is_some()
        );
    }

    #[test]
    fn observability_relay_is_staging_only_https_and_signed() {
        let load_relay = |env: &str, relay: &str, signer: Option<&str>| {
            RuntimeConfig::from_lookup(|name| match name {
                "APP_ENV" => Some(env.into()),
                "PUBLIC_SITE_IAP_AUDIENCE" => Some(crate::access::tests::AUDIENCE.into()),
                "OBSERVABILITY_RELAY_URL" => Some(relay.into()),
                "PHYSICIAN_PROXY_SERVICE_ACCOUNT" => signer.map(str::to_owned),
                _ => None,
            })
        };
        let signer = Some("site@project.iam.gserviceaccount.com");
        let config = load_relay("staging", "https://relay.example.test", signer).unwrap();
        let relay = config.observability_proxy.unwrap();
        assert_eq!(relay.upstream.as_str(), "https://relay.example.test/");
        assert_eq!(relay.public_origin, "https://staging.codahealth.ai");
        assert!(config.physician_proxy.is_none());
        assert!(load_relay("staging", "https://relay.example.test", None).is_err());
        assert!(load_relay("staging", "http://127.0.0.1:8790", signer).is_err());
        assert!(load_relay("staging", "https://relay.example.test/grafana", signer).is_err());
        for env in ["local", "production"] {
            assert!(
                load_relay(env, "https://relay.example.test", signer).is_err(),
                "{env}"
            );
        }
        assert!(
            load_relay("staging", "", signer)
                .unwrap()
                .observability_proxy
                .is_none()
        );
    }

    #[test]
    fn langfuse_sign_in_is_password_unless_sso_is_named_with_the_relay() {
        use crate::proxy::LangfuseSignIn;
        let load_sign_in = |relay: Option<&str>, sign_in: Option<&str>| {
            RuntimeConfig::from_lookup(|name| match name {
                "APP_ENV" => Some("staging".into()),
                "PUBLIC_SITE_IAP_AUDIENCE" => Some(crate::access::tests::AUDIENCE.into()),
                "OBSERVABILITY_RELAY_URL" => relay.map(str::to_owned),
                "PHYSICIAN_PROXY_SERVICE_ACCOUNT" => {
                    Some("site@project.iam.gserviceaccount.com".into())
                }
                "LANGFUSE_SIGN_IN" => sign_in.map(str::to_owned),
                _ => None,
            })
        };
        let relay = Some("https://relay.example.test");
        for (sign_in, expected) in [
            (None, LangfuseSignIn::Password),
            (Some("password"), LangfuseSignIn::Password),
            (Some("sso"), LangfuseSignIn::Sso),
        ] {
            assert_eq!(
                load_sign_in(relay, sign_in).unwrap().langfuse_sign_in,
                expected,
                "{sign_in:?}"
            );
        }
        assert!(load_sign_in(relay, Some("SSO")).is_err());
        assert!(load_sign_in(None, Some("sso")).is_err());
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
