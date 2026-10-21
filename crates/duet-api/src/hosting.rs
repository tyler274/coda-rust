//! Hosting qualification is distinct from building a runnable local application.

/// Refuses hosted execution until the product-owned durable/auth gates land.
/// Presence of Cloud Run's marker cannot be overridden by a local flag.
pub fn validate_startup(get: impl Fn(&str) -> Option<String>) -> Result<(), &'static str> {
    if get("K_SERVICE").is_some() || get("CODA_HOSTED").is_some() {
        return Err(
            "Duet hosted activation is not qualified: durable account-scoped routes, conversation execution, and complete workspace-owned migrations are required",
        );
    }
    Ok(())
}

/// Readiness is deliberately unavailable until the actual product runtime qualifies.
pub async fn ready() -> axum::http::StatusCode {
    axum::http::StatusCode::SERVICE_UNAVAILABLE
}

/// Process liveness only (the `GET /health` of the Duet wire contract). It says
/// nothing about qualification; `/readyz` stays the only readiness answer.
pub async fn live() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok" }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cloud_marker_and_even_false_hosted_flags_cannot_bypass_qualification() {
        assert!(validate_startup(|_| None).is_ok());
        for key in ["K_SERVICE", "CODA_HOSTED"] {
            assert!(validate_startup(|name| (name == key).then(|| "0".to_owned())).is_err());
        }
    }

    #[tokio::test]
    async fn unqualified_runtime_never_reports_ready() {
        assert_eq!(ready().await, axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn liveness_is_not_readiness() {
        assert_eq!(live().await.0["status"], "ok");
        assert_eq!(ready().await, axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }
}
