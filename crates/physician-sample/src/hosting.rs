//! Fail-closed hosting boundary for the current sample runtime.

/// Refuses hosted execution until product-owned identity and persistence qualify.
/// This is not an opt-in staging bypass: no environment value enables hosting.
pub fn validate_startup(get: impl Fn(&str) -> Option<String>) -> Result<(), &'static str> {
    if get("K_SERVICE").is_some() || get("CODA_HOSTED").is_some() {
        return Err(
            "Physician hosted activation is not qualified: verified identity delivery and durable product runtime are required; sample, replay, proxy and subscription modes cannot be hosted",
        );
    }
    Ok(())
}

/// A packaged sample must never satisfy the hosted readiness probe.
pub async fn ready() -> axum::http::StatusCode {
    axum::http::StatusCode::SERVICE_UNAVAILABLE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cloud_markers_cannot_be_disabled_with_a_false_value() {
        assert!(validate_startup(|_| None).is_ok());
        for key in ["K_SERVICE", "CODA_HOSTED"] {
            assert!(validate_startup(|name| (name == key).then(|| "false".to_owned())).is_err());
        }
    }

    #[tokio::test]
    async fn sample_is_not_a_ready_hosted_runtime() {
        assert_eq!(ready().await, axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }
}
