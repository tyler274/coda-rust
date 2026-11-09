//! Staging access: the shared fail-closed IAP verifier, configured for this
//! service's own Cloud Run audience. Assertions are never logged.
use axum::http::HeaderMap;
use http::iap::{IapVerifier, single_header, validate_cloud_run_audience};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

#[derive(Clone, Default)]
pub struct AccessConfig {
    pub audience: Option<String>,
}
impl AccessConfig {
    /// Direct Cloud Run signed-header audience, NOT an OAuth client ID or URL.
    pub fn validate(&self) -> Result<(), &'static str> {
        let audience = self
            .audience
            .as_deref()
            .ok_or("PUBLIC_SITE_IAP_AUDIENCE is required for staging")?;
        validate_cloud_run_audience(audience).map_err(
            |_| "PUBLIC_SITE_IAP_AUDIENCE must be /projects/NUMBER/locations/REGION/services/NAME",
        )
    }
}

#[derive(Clone)]
pub struct AccessGate {
    verifier: IapVerifier,
}
impl AccessGate {
    pub fn new(config: AccessConfig) -> Self {
        Self {
            verifier: IapVerifier::new(config.audience),
        }
    }

    pub async fn allows(&self, headers: &HeaderMap) -> bool {
        self.verifier.allows(headers).await
    }
}

pub fn secret_matches(headers: &HeaderMap, name: &str, expected: &str) -> bool {
    if expected.is_empty() {
        return false;
    }
    let Some(actual) = single_header(headers, name) else {
        return false;
    };
    // Only the optional additional health token uses this helper, never IAP.
    bool::from(Sha256::digest(actual.as_bytes()).ct_eq(&Sha256::digest(expected.as_bytes())))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) const AUDIENCE: &str =
        "/projects/134570912572/locations/us-east1/services/public-site-staging";

    pub(crate) fn assertion() -> String {
        http::iap::fixture::assertion(AUDIENCE)
    }

    pub(crate) fn cached_gate() -> AccessGate {
        AccessGate {
            verifier: http::iap::fixture::verifier(AUDIENCE),
        }
    }

    #[tokio::test]
    async fn gate_uses_this_services_audience() {
        let mut headers = HeaderMap::new();
        headers.insert("x-goog-iap-jwt-assertion", assertion().parse().unwrap());
        assert!(cached_gate().allows(&headers).await);
        let other = "/projects/134570912572/locations/us-east1/services/other";
        headers.insert(
            "x-goog-iap-jwt-assertion",
            http::iap::fixture::assertion(other).parse().unwrap(),
        );
        assert!(!cached_gate().allows(&headers).await);
        assert!(
            !AccessGate::new(AccessConfig::default())
                .allows(&headers)
                .await
        );
    }

    #[test]
    fn secret_needs_one_exact_value() {
        let mut headers = HeaderMap::new();
        assert!(!secret_matches(&headers, "x-coda-staging-token", "probe"));
        headers.insert("x-coda-staging-token", "probe".parse().unwrap());
        assert!(secret_matches(&headers, "x-coda-staging-token", "probe"));
        assert!(!secret_matches(&headers, "x-coda-staging-token", ""));
        headers.append("x-coda-staging-token", "probe".parse().unwrap());
        assert!(!secret_matches(&headers, "x-coda-staging-token", "probe"));
    }
}
