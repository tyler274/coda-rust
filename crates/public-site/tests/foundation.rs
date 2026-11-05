//! Every route renders through the real router with the shared chrome, Sign in appears
//! only when offered, and the server serves only the bytes built at startup.
use axum::{
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
};
use public_site::{
    Config, Route,
    legal::Legal,
    prerender,
    runtime::RuntimeConfig,
    server::{Readiness, router_with_options},
};
use tower::ServiceExt;

#[test]
fn every_enumerated_route_uses_real_router_ssr_and_shared_chrome() {
    let config = Config::default();
    let table = prerender(&config).unwrap();
    let routes = Route::pages(&Legal::load().unwrap());
    assert_eq!(table.pages.len(), routes.len());
    for route in routes {
        let path = route.to_string();
        assert_eq!(path.parse::<Route>().unwrap(), route);
        let page = &table.pages[&path];
        let html = std::str::from_utf8(&page.html).unwrap();
        assert!(html.starts_with("<!DOCTYPE html><html"), "{path}: {html}");
        assert!(
            html.contains(&format!("<title>{}</title>", page.meta.title)),
            "{path}: {html}"
        );
        assert!(html.contains(&page.meta.canonical_url));
        assert!(html.contains("Main navigation"));
        assert!(html.contains("Consumer Health Data Privacy Policy"));
        assert!(
            !html.contains("href=\"/physician/\""),
            "{path}: sign-in without upstream"
        );
        assert!(html.contains("<main"));
        if path != "/" {
            assert!(
                !html.contains("id=\"heroStage\""),
                "router fell back to home for {path}"
            );
        }
        if let Route::DuetPrivacyArchive { version } | Route::DuetTermsArchive { version } = route {
            assert!(html.contains(&version));
        }
    }
    let scheduling = std::str::from_utf8(&table.pages["/scheduling/dev"].html).unwrap();
    assert!(scheduling.contains("noindex, nofollow"));
    assert!(
        std::str::from_utf8(&table.not_found.html)
            .unwrap()
            .contains("Page not found")
    );
}

#[test]
fn sign_in_appears_on_every_page_only_when_offered() {
    let table = prerender(&Config {
        physician_sign_in: true,
        ..Config::default()
    })
    .unwrap();
    for (path, page) in &table.pages {
        let html = std::str::from_utf8(&page.html).unwrap();
        assert!(html.contains("href=\"/physician/\">Sign in</a>"), "{path}");
    }
}

#[tokio::test]
async fn axum_serves_only_startup_bytes_and_preserves_404s() {
    let table = prerender(&Config::default()).unwrap();
    let asset_table = public_site::assets::AssetTable::embedded_with([]);
    let expected = asset_table.rewrite(std::str::from_utf8(&table.pages["/duet"].html).unwrap());
    let app = router_with_options(table, RuntimeConfig::default(), [], Readiness::ready());
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/duet?ref=test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["content-type"],
        "text/html; charset=utf-8"
    );
    assert_eq!(
        to_bytes(response.into_body(), usize::MAX).await.unwrap(),
        expected.as_bytes()
    );
    for path in [
        "/physicians",
        "/api/physician-waitlist",
        // No physician upstream configured, so there is no physician mount.
        "/physician",
        "/physician/today",
        "/missing",
        "/duet/privacy/unknown",
    ] {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(
            std::str::from_utf8(&body)
                .unwrap()
                .contains("Page not found")
        );
    }
    let head = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::HEAD)
                .uri("/duet")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(head.status(), StatusCode::OK);
    assert_eq!(head.headers()["content-length"], expected.len().to_string());
    assert!(
        to_bytes(head.into_body(), usize::MAX)
            .await
            .unwrap()
            .is_empty()
    );
    let post = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/duet")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(post.status(), StatusCode::METHOD_NOT_ALLOWED);
    let css = app
        .oneshot(
            Request::builder()
                .uri("/css/chrome.css")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(css.status(), StatusCode::OK);
    assert_eq!(
        to_bytes(css.into_body(), usize::MAX)
            .await
            .unwrap()
            .as_ref(),
        include_bytes!("../assets/css/chrome.css")
    );
}
