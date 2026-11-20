//! Hosting boundary for the Duet runtime.
//!
//! Local runs are unrestricted. Hosted runs (Cloud Run sets `K_SERVICE`, or
//! `CODA_HOSTED` is set) start only under the explicit staging-preview profile:
//! `CODA_ENVIRONMENT=stg`, `CODA_HOSTED_PROFILE=staging-preview`, `DATABASE_URL`
//! and `IDENTITY_PLATFORM_PROJECT_ID` (the Identity Platform project whose tokens
//! are verified). Production and every other combination refuse to start, and so
//! does any hosted run carrying a [`LOCAL_ONLY`] stand-in. The model execution
//! switch stays off and no health-data attestation is implied.

/// The only hosted profile this binary will start under.
pub const STAGING_PROFILE: &str = "staging-preview";

/// Local-stack stand-ins for Identity Platform and the model provider. Hosted
/// startup refuses every one of them, whatever its value.
pub const LOCAL_ONLY: [&str; 3] = [
    "DUET_LOCAL_IDENTITY_SECRET",
    "DUET_LOCAL_MODEL_REPLY",
    "DUET_LOCAL_MODEL_DELAY_MS",
];

/// Refuses hosted execution unless the staging-preview profile is complete.
pub fn validate_startup(get: impl Fn(&str) -> Option<String>) -> Result<(), String> {
    if get("K_SERVICE").is_none() && get("CODA_HOSTED").is_none() {
        return Ok(());
    }
    if let Some(name) = LOCAL_ONLY.iter().find(|name| get(name).is_some()) {
        return Err(format!(
            "Duet hosted activation refuses the local-only {name}; hosted runs verify Identity Platform tokens and never serve fixed replies"
        ));
    }
    let environment = get("CODA_ENVIRONMENT").unwrap_or_default();
    let profile = get("CODA_HOSTED_PROFILE").unwrap_or_default();
    let database = get("DATABASE_URL").unwrap_or_default();
    let project = get("IDENTITY_PLATFORM_PROJECT_ID").unwrap_or_default();
    if environment == "stg"
        && profile == STAGING_PROFILE
        && !database.trim().is_empty()
        && !project.trim().is_empty()
    {
        return Ok(());
    }
    Err(format!(
        "Duet hosted activation is not qualified: hosted runs require CODA_ENVIRONMENT=stg, CODA_HOSTED_PROFILE={STAGING_PROFILE}, DATABASE_URL and IDENTITY_PLATFORM_PROJECT_ID; production and every other profile refuse to start"
    ))
}

/// Readiness for a hosted revision. The account schema must already have passed
/// `AccountDatabase::connect`. When a front door is configured, its schema must
/// answer too. A failed check is 503, never a crash.
pub async fn ready(
    accounts_ready: bool,
    front_door: Option<&front_door::PostgresFrontDoorStore>,
) -> axum::http::StatusCode {
    if !accounts_ready {
        return axum::http::StatusCode::SERVICE_UNAVAILABLE;
    }
    if let Some(front_door) = front_door
        && front_door.check_schema().await.is_err()
    {
        return axum::http::StatusCode::SERVICE_UNAVAILABLE;
    }
    axum::http::StatusCode::OK
}

/// Process liveness only (the `GET /health` of the Duet wire contract). It says
/// nothing about qualification; `/readyz` stays the only readiness answer.
pub async fn live() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok" }))
}

/// Wraps the Duet HTTP router with shared per-IP rate limiting, concurrency bounds,
/// and request body-size enforcement (`coda_http::ingress`).
pub fn with_ingress_guard(app: axum::Router, guard: http::ingress::IngressGuard) -> axum::Router {
    guard.protect(app)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hosted(extra: &[(&str, &str)]) -> Result<(), String> {
        validate_startup(|name| {
            if name == "K_SERVICE" {
                return Some("duet".to_owned());
            }
            extra
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        })
    }

    #[test]
    fn local_runs_start_without_any_profile() {
        assert!(validate_startup(|_| None).is_ok());
    }

    #[test]
    fn staging_preview_is_the_only_hosted_profile() {
        let complete = [
            ("CODA_ENVIRONMENT", "stg"),
            ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
            ("DATABASE_URL", "postgres://db/duet"),
            ("IDENTITY_PLATFORM_PROJECT_ID", "coda-rust-stg"),
        ];
        assert!(hosted(&complete).is_ok());
        for index in 0..complete.len() {
            let dropped: Vec<_> = complete
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, pair)| *pair)
                .collect();
            assert!(hosted(&dropped).is_err(), "missing {}", complete[index].0);
        }
    }

    #[test]
    fn production_and_other_profiles_refuse_even_when_complete() {
        for (environment, profile) in [
            ("prd", STAGING_PROFILE),
            ("stg", "production"),
            ("dev", STAGING_PROFILE),
            ("", STAGING_PROFILE),
        ] {
            assert!(
                hosted(&[
                    ("CODA_ENVIRONMENT", environment),
                    ("CODA_HOSTED_PROFILE", profile),
                    ("DATABASE_URL", "postgres://db/duet"),
                    ("IDENTITY_PLATFORM_PROJECT_ID", "coda-rust-stg"),
                ])
                .is_err(),
                "{environment}/{profile} must refuse"
            );
        }
    }

    #[test]
    fn local_stand_ins_refuse_hosted_even_with_a_complete_profile() {
        let complete = [
            ("CODA_ENVIRONMENT", "stg"),
            ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
            ("DATABASE_URL", "postgres://db/duet"),
            ("IDENTITY_PLATFORM_PROJECT_ID", "coda-rust-stg"),
        ];
        for name in LOCAL_ONLY {
            for value in ["", "x"] {
                let mut env = complete.to_vec();
                env.push((name, value));
                assert!(hosted(&env).is_err(), "{name}={value:?} must refuse");
                let marked = validate_startup(|key| {
                    if key == "CODA_HOSTED" {
                        return Some("1".into());
                    }
                    env.iter()
                        .find(|(k, _)| *k == key)
                        .map(|(_, v)| (*v).to_owned())
                });
                assert!(marked.is_err(), "CODA_HOSTED with {name} must refuse");
            }
        }
        let local = validate_startup(|key| LOCAL_ONLY.contains(&key).then(|| "x".to_owned()));
        assert!(local.is_ok(), "local stacks may use the stand-ins");
    }

    #[test]
    fn a_false_cloud_marker_is_still_hosted() {
        assert!(
            validate_startup(|name| (name == "K_SERVICE").then(|| "false".to_owned())).is_err()
        );
        assert!(validate_startup(|name| (name == "CODA_HOSTED").then(|| "0".to_owned())).is_err());
    }

    #[tokio::test]
    async fn liveness_says_nothing_about_readiness() {
        assert_eq!(live().await.0["status"], "ok");
    }

    #[tokio::test]
    async fn ingress_guard_rate_limits_email_auth_and_rejects_oversized_bodies() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode, header};
        use axum::routing::{get, post};
        use http::ingress::{IngressGuard, IngressGuardConfig, RateLimitQuota};
        use tower::ServiceExt;

        let guard = IngressGuard::new(IngressGuardConfig::strict(
            RateLimitQuota::per_minute(10),
            RateLimitQuota::per_minute(1),
            128,
            4,
        ));
        let app = with_ingress_guard(
            axum::Router::new()
                .route("/health", get(live))
                .route("/api/auth/email/start", post(|| async { "accepted" }))
                .route("/api/evidence", post(|body: String| async move { body })),
            guard,
        );

        let first = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/email/start")
                    .header("x-forwarded-for", "198.51.100.77")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(first.status(), StatusCode::OK);

        let second = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/email/start")
                    .header("x-forwarded-for", "198.51.100.77")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(second.status(), StatusCode::TOO_MANY_REQUESTS);
        assert!(second.headers().contains_key(header::RETRY_AFTER));
        assert_eq!(second.headers()[header::CACHE_CONTROL], "no-store");

        // Health probe remains available for the same IP while rate-limited.
        let health = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .header("x-forwarded-for", "198.51.100.77")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(health.status(), StatusCode::OK);

        // Oversized payload rejected before hitting the handler.
        let big = "a".repeat(256);
        let oversized = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/evidence")
                    .header("x-forwarded-for", "198.51.100.78")
                    .header(header::CONTENT_LENGTH, big.len().to_string())
                    .body(Body::from(big))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(oversized.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }
}
