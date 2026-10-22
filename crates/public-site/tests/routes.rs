use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use public_site::{
    Config, Route, prerender,
    runtime::RuntimeConfig,
    server::{Readiness, router_with_options},
};
use tower::ServiceExt;

#[tokio::test]
async fn every_route_metadata_sitemap_and_retired_routes() {
    let app = router_with_options(
        prerender(&Config::default()),
        RuntimeConfig::default(),
        [],
        Readiness::ready(),
    );
    for route in Route::pages() {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(route.to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{route}");
        assert!(
            response.headers()["cache-control"]
                .to_str()
                .unwrap()
                .contains("s-maxage=3600")
        );
        let html = String::from_utf8(
            to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        let meta = route.meta();
        assert!(
            html.contains(&format!("<title>{}</title>", meta.title)),
            "{route}"
        );
        assert!(
            html.contains(&format!(
                "rel=\"canonical\" href=\"{}\"",
                meta.canonical_url
            )),
            "{route}"
        );
        assert!(html.contains("og:image:alt") && html.contains("twitter:image:alt"));
        assert!(html.contains(&meta.image_alt));
        assert_eq!(html.contains("noindex, nofollow"), meta.noindex);
    }
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/sitemap.xml")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let sitemap = String::from_utf8(
        to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    // The same locations, in the same order, as production's sitemap.
    let locations = |xml: &str| -> Vec<String> {
        xml.split("<loc>")
            .skip(1)
            .map(|part| part.split("</loc>").next().unwrap().to_owned())
            .collect()
    };
    assert_eq!(
        locations(&sitemap),
        locations(include_str!("fixtures/served/sitemap.xml"))
    );
    for path in [
        "/physicians",
        "/api/physician-waitlist",
        "/duet/privacy/unknown",
        "/duet/terms/unknown",
        "/missing",
    ] {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
        let html = String::from_utf8(
            to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(
            !html.contains("rel=\"canonical\"")
                && !html.contains("og:image")
                && !html.contains("twitter:image")
        );
        assert!(html.contains("Page not found"));
    }
}

/// Status and redirect target for every probe, as production served them (including
/// Next's trailing-slash rule); see tests/fixtures/served/responses.json.
#[tokio::test]
async fn urls_resolve_and_redirect_as_production_serves_them() {
    let app = router_with_options(
        prerender(&Config::default()),
        RuntimeConfig::default(),
        [],
        Readiness::ready(),
    );
    let served: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/served/responses.json")).unwrap();
    for (path, expected) in served.as_object().unwrap() {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(
            response.status().as_u16(),
            expected["status"].as_u64().unwrap() as u16,
            "{path}"
        );
        assert_eq!(
            response
                .headers()
                .get("location")
                .map(|value| value.to_str().unwrap()),
            expected["location"].as_str(),
            "{path}"
        );
    }
}

#[tokio::test]
async fn real_scripts_are_content_addressed_and_immutable() {
    let app = router_with_options(
        prerender(&Config::default()),
        RuntimeConfig::default(),
        [],
        Readiness::ready(),
    );
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let html = String::from_utf8(
        to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    for (suffix, mime) in [
        ("/chrome.js", "javascript"),
        ("/homepage/site.js", "javascript"),
    ] {
        let path = html
            .split('"')
            .find(|value| value.starts_with("/a/") && value.ends_with(suffix))
            .unwrap_or_else(|| panic!("homepage must reference hashed {suffix}"));
        let response = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers()["content-type"]
                .to_str()
                .unwrap()
                .contains(mime)
        );
        assert!(
            response.headers()["cache-control"]
                .to_str()
                .unwrap()
                .contains("immutable")
        );
        let etag = response.headers()["etag"].clone();
        let cached = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header("if-none-match", etag)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(cached.status(), StatusCode::NOT_MODIFIED);
    }
    let social = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/scheduling/dev/opengraph-image")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(social.status(), StatusCode::OK);
    assert_eq!(social.headers()["content-type"], "image/png");
    let png = to_bytes(social.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    assert_eq!(u32::from_be_bytes(png[16..20].try_into().unwrap()), 1200);
    assert_eq!(u32::from_be_bytes(png[20..24].try_into().unwrap()), 630);
    for coding in ["br", "gzip"] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("accept-encoding", coding)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.headers()["content-encoding"], coding);
        assert!(
            response.headers()["vary"]
                .to_str()
                .unwrap()
                .contains("Accept-Encoding")
        );
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let mut decoded = String::new();
        use std::io::Read;
        if coding == "br" {
            brotli::Decompressor::new(bytes.as_ref(), 4096)
                .read_to_string(&mut decoded)
                .unwrap();
        } else {
            flate2::read::GzDecoder::new(bytes.as_ref())
                .read_to_string(&mut decoded)
                .unwrap();
        }
        assert_eq!(decoded, html);
    }
}

#[tokio::test]
async fn staging_denies_pages_and_assets_without_access() {
    let runtime = RuntimeConfig {
        staging: true,
        access: public_site::access::AccessConfig {
            audience: Some(
                "/projects/134570912572/locations/us-east1/services/public-site-staging".into(),
            ),
        },
        ..RuntimeConfig::default()
    };
    let app = router_with_options(
        prerender(&Config::default()),
        runtime,
        [],
        Readiness::ready(),
    );
    for path in ["/", "/api/health", "/homepage/site.js"] {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(response.headers()["x-robots-tag"], "noindex, nofollow");
        assert!(
            response.headers()["cache-control"]
                .to_str()
                .unwrap()
                .contains("no-store")
        );
    }
}
