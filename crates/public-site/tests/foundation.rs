//! Focused scaffold proof; full route/parity/browser testing follows in its own phase.
use axum::{
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
};
use public_site::{Config, Route, prerender, server::router_with_pages};
use tower::ServiceExt;

#[test]
fn every_enumerated_route_uses_real_router_ssr_and_shared_chrome() {
    let config = Config {
        physician_app_url: "https://physicians.example.test/sign-in?from=public&source=nav".into(),
        ..Config::default()
    };
    let table = prerender(&config);
    assert_eq!(table.pages.len(), Route::pages().len());
    for route in Route::pages() {
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
            html.contains("https://physicians.example.test/sign-in?from=public&#38;source=nav"),
            "{html}"
        );
        assert!(html.contains("<main id=\"main\""));
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

#[tokio::test]
async fn axum_serves_only_startup_bytes_and_preserves_404s() {
    let table = prerender(&Config::default());
    let asset_table = public_site::assets::AssetTable::embedded_with(
        public_site::paint::assets()
            .into_iter()
            .map(|asset| (asset.path, axum::body::Bytes::from(asset.bytes)))
            .chain([(
                "/css/paint.css".into(),
                axum::body::Bytes::from(public_site::paint::stylesheet(str::to_owned)),
            )]),
    );
    let expected = asset_table.rewrite(std::str::from_utf8(&table.pages["/duet"].html).unwrap());
    let app = router_with_pages(table);
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
