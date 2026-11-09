use axum::{
    Router,
    body::{Body, to_bytes},
    extract::Request,
    http::{StatusCode, header},
    response::IntoResponse,
};
use observability_relay::{Config, ConfigError, router_with_verifier};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tower::ServiceExt;

const AUDIENCE: &str = "/projects/123456789/locations/us-east1/services/observability-relay";

async fn upstream(
    name: &'static str,
    calls: Arc<AtomicUsize>,
) -> (String, tokio::task::JoinHandle<()>) {
    let app = Router::new().fallback(move |request: Request| {
        let calls = calls.clone();
        async move {
            calls.fetch_add(1, Ordering::SeqCst);
            let headers = request.headers();
            for removed in [
                "x-goog-iap-jwt-assertion",
                "proxy-authorization",
                "x-forwarded-for",
                "x-private-hop",
            ] {
                assert!(!headers.contains_key(removed), "{removed}");
            }
            let echo = serde_json::json!({
                "upstream": name,
                "path": request.uri().to_string(),
                "assertion": headers.get("x-coda-iap-assertion").map(|v| v.to_str().unwrap()),
                "cookie": headers.get(header::COOKIE).map(|v| v.to_str().unwrap()),
            });
            (
                [
                    (header::SET_COOKIE, "__Secure-next-auth.session-token=s"),
                    (header::CONNECTION, "x-upstream-hop"),
                ],
                [("x-upstream-hop", "private")],
                echo.to_string(),
            )
                .into_response()
        }
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    (origin, handle)
}

fn config(grafana: &str, langfuse: &str, langfuse_sso: &str) -> Config {
    Config::from_lookup(|name| match name {
        "RELAY_IAP_AUDIENCE" => Some(AUDIENCE.into()),
        "GRAFANA_UPSTREAM" => Some(grafana.into()),
        "LANGFUSE_UPSTREAM" => Some(langfuse.into()),
        "LANGFUSE_SSO_UPSTREAM" => Some(langfuse_sso.into()),
        _ => None,
    })
    .unwrap()
}

#[tokio::test]
async fn routes_three_prefixes_unchanged_and_only_with_the_relays_own_assertion() {
    let grafana_calls = Arc::new(AtomicUsize::new(0));
    let langfuse_calls = Arc::new(AtomicUsize::new(0));
    let sso_calls = Arc::new(AtomicUsize::new(0));
    let (grafana, g) = upstream("grafana", grafana_calls.clone()).await;
    let (langfuse, l) = upstream("langfuse", langfuse_calls.clone()).await;
    let (sso, s) = upstream("langfuse-sso", sso_calls.clone()).await;
    let app = router_with_verifier(
        config(&grafana, &langfuse, &sso),
        http::iap::fixture::verifier(AUDIENCE),
    );
    let send = |path: &str, assertion: Option<String>| {
        let mut request = Request::builder()
            .uri(path)
            .header("x-coda-iap-assertion", "user-assertion")
            .header(header::COOKIE, "__Secure-next-auth.session-token=s")
            .header(header::PROXY_AUTHORIZATION, "Bearer public-site")
            .header("x-forwarded-for", "203.0.113.1")
            .header(header::CONNECTION, "x-private-hop")
            .header("x-private-hop", "private");
        if let Some(assertion) = assertion {
            request = request.header("x-goog-iap-jwt-assertion", assertion);
        }
        app.clone().oneshot(request.body(Body::empty()).unwrap())
    };
    let other = "/projects/123456789/locations/us-east1/services/public-site-staging";
    for assertion in [
        None,
        Some("garbage".into()),
        Some(http::iap::fixture::assertion(other)),
    ] {
        let response = send("/grafana/", assertion).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    for (path, name) in [
        ("/grafana/api/health?x=%2F", "grafana"),
        ("/grafana", "grafana"),
        ("/langfuse/api/public/health", "langfuse"),
        ("/langfuse-sso/authorize?state=s", "langfuse-sso"),
    ] {
        let response = send(path, Some(http::iap::fixture::assertion(AUDIENCE)))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert_eq!(
            response.headers()[header::SET_COOKIE],
            "__Secure-next-auth.session-token=s"
        );
        assert!(!response.headers().contains_key("x-upstream-hop"));
        let body: serde_json::Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
        assert_eq!(body["upstream"], name);
        assert_eq!(body["path"], path, "the path passes through unchanged");
        assert_eq!(body["assertion"], "user-assertion");
        assert_eq!(body["cookie"], "__Secure-next-auth.session-token=s");
    }
    for path in [
        "/",
        "/physician/",
        "/grafanax",
        "/langfuse-old/",
        "//evil.test/grafana/",
    ] {
        let response = send(path, Some(http::iap::fixture::assertion(AUDIENCE)))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
    }
    assert_eq!(grafana_calls.load(Ordering::SeqCst), 2);
    assert_eq!(langfuse_calls.load(Ordering::SeqCst), 1);
    assert_eq!(sso_calls.load(Ordering::SeqCst), 1);
    g.abort();
    l.abort();
    s.abort();
}

#[test]
fn configuration_is_required_and_origin_only() {
    let load = |audience: &str, grafana: &str| {
        Config::from_lookup(|name| match name {
            "RELAY_IAP_AUDIENCE" => Some(audience.into()),
            "GRAFANA_UPSTREAM" => Some(grafana.into()),
            "LANGFUSE_UPSTREAM" => Some("http://10.80.0.11:3000".into()),
            "LANGFUSE_SSO_UPSTREAM" => Some("http://10.80.0.12:3000".into()),
            _ => None,
        })
    };
    let without_sso = Config::from_lookup(|name| match name {
        "RELAY_IAP_AUDIENCE" => Some(AUDIENCE.into()),
        "GRAFANA_UPSTREAM" => Some("http://10.80.0.10:3000".into()),
        "LANGFUSE_UPSTREAM" => Some("http://10.80.0.11:3000".into()),
        _ => None,
    });
    assert!(matches!(
        without_sso,
        Err(ConfigError::Missing {
            setting: "LANGFUSE_SSO_UPSTREAM"
        })
    ));
    assert!(load(AUDIENCE, "http://10.80.0.10:3000").is_ok());
    let audience = |result| {
        matches!(
            result,
            Err(ConfigError::Audience {
                setting: "RELAY_IAP_AUDIENCE",
                ..
            })
        )
    };
    let missing =
        |result, want| matches!(result, Err(ConfigError::Missing { setting }) if setting == want);
    let not_origin = |result| {
        matches!(
            result,
            Err(ConfigError::NotOrigin {
                setting: "GRAFANA_UPSTREAM"
            })
        )
    };
    assert!(missing(
        load("", "http://10.80.0.10:3000"),
        "RELAY_IAP_AUDIENCE"
    ));
    assert!(audience(load(
        "observability-relay",
        "http://10.80.0.10:3000"
    )));
    assert!(missing(load(AUDIENCE, ""), "GRAFANA_UPSTREAM"));
    for grafana in [
        "http://10.80.0.10:3000/grafana",
        "http://user:pass@10.80.0.10:3000",
        "file:///etc/passwd",
    ] {
        assert!(not_origin(load(AUDIENCE, grafana)), "{grafana}");
    }
    let error = load(AUDIENCE, "not a url").unwrap_err();
    assert!(matches!(
        error,
        ConfigError::Unparsable {
            setting: "GRAFANA_UPSTREAM",
            ..
        }
    ));
    assert!(std::error::Error::source(&error).is_some());
}
