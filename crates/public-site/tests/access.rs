//! Public API regressions. Successful signed requests use cfg(test)-only local
//! fixture injection in server/access unit tests, not a deployable trust override.
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use public_site::{
    Config, Route,
    access::AccessConfig,
    prerender,
    runtime::RuntimeConfig,
    server::{Readiness, router_with_options},
};
use tower::ServiceExt;

const AUDIENCE: &str = "/projects/134570912572/locations/us-east1/services/public-site-staging";

#[tokio::test]
async fn staging_denies_every_page_and_resource_without_signed_iap() {
    let app = router_with_options(
        prerender(&Config::default()),
        RuntimeConfig {
            staging: true,
            environment: "staging".into(),
            access: AccessConfig {
                audience: Some(AUDIENCE.into()),
            },
            health_token: Some("probe".into()),
            ..Default::default()
        },
        [],
        Readiness::ready(),
    );
    let mut paths: Vec<_> = Route::pages().iter().map(ToString::to_string).collect();
    paths.extend(
        [
            "/healthz",
            "/readyz",
            "/api/health",
            "/favicon.ico",
            "/robots.txt",
            "/sitemap.xml",
            "/motion/bootstrap.js",
            "/motion/public_site_motion_bg.wasm",
            "/missing",
        ]
        .map(str::to_owned),
    );
    for path in paths {
        for method in [Method::GET, Method::HEAD, Method::POST] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(method.clone())
                        .uri(&path)
                        .header("cf-access-client-id", "id")
                        .header("cf-access-client-secret", "secret")
                        .header("cf-access-jwt-assertion", "legacy")
                        .header(
                            "x-goog-authenticated-user-email",
                            "accounts.google.com:engineer@example.test",
                        )
                        .header("x-goog-authenticated-user-id", "accounts.google.com:123")
                        .header("x-coda-staging-token", "probe")
                        .header("if-none-match", "*")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN, "{method} {path}");
            assert_eq!(
                response.headers()["cache-control"],
                "private, no-store, max-age=0"
            );
            assert_eq!(response.headers()["x-robots-tag"], "noindex, nofollow");
            assert_eq!(response.headers()["vary"], "*");
            assert_eq!(response.headers()["cdn-cache-control"], "no-store");
            if method == Method::HEAD {
                assert!(
                    axum::body::to_bytes(response.into_body(), 1024)
                        .await
                        .unwrap()
                        .is_empty()
                );
            }
        }
    }
}

#[test]
fn executable_refuses_missing_or_malformed_staging_configuration() {
    for audience in [None, Some(""), Some("https://attacker.test"), Some("wrong")] {
        let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_public-site"));
        command
            .env_clear()
            .env("APP_ENV", "staging")
            .env("CF_ACCESS_CLIENT_ID", "legacy")
            .env("CF_ACCESS_CLIENT_SECRET", "legacy");
        if let Some(audience) = audience {
            command.env("PUBLIC_SITE_IAP_AUDIENCE", audience);
        }
        let output = command.output().unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("PUBLIC_SITE_IAP_AUDIENCE"));
    }
}
