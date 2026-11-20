use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use public_site::{
    Config, Route,
    legal::Legal,
    prerender,
    proxy::ProxyConfig,
    runtime::RuntimeConfig,
    server::{Readiness, router_with_options},
};
use tower::ServiceExt;

#[tokio::test]
async fn staging_sign_in_reaches_the_physician_service() {
    let inputs = include_str!("../../../infra/gcp/public-site/env/staging.tfvars");
    let destination = inputs
        .lines()
        .find(|line| line.starts_with("physician_app_url"))
        .and_then(|line| line.split('"').nth(1))
        .expect("staging must configure physician_app_url");
    // As the binary does: the configured upstream mounts /physician/ and offers Sign in.
    let runtime = RuntimeConfig {
        physician_proxy: Some(
            ProxyConfig::new(
                destination,
                "https://staging.coda.dabney.moe",
                None,
                true,
                false,
            )
            .unwrap(),
        ),
        ..RuntimeConfig::default()
    };
    let config = Config {
        physician_sign_in: runtime.physician_proxy.is_some(),
        ..Config::default()
    };
    let app = router_with_options(prerender(&config).unwrap(), runtime, [], Readiness::ready());
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let html = String::from_utf8(
        to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    let rendered = html.contains("href=\"/physician/\">Sign in</a>");
    assert!(
        !html.contains(destination),
        "upstream origin must not enter HTML"
    );
    let correct = destination == "https://physician-836508863523.us-east1.run.app";
    assert!(
        rendered,
        "Sign in must remain on the public origin under /physician/"
    );
    assert!(
        correct,
        "staging Sign in must reach physician in coda-rust-staging, not the marketing site"
    );
}

#[tokio::test]
async fn every_route_metadata_sitemap_and_retired_routes() {
    let app = router_with_options(
        prerender(&Config::default()).unwrap(),
        RuntimeConfig::default(),
        [],
        Readiness::ready(),
    );
    let legal = Legal::load().unwrap();
    for route in Route::pages(&legal) {
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
        let meta = route.meta(&legal);
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
    // Current documents only, in this order; archived versions are reached from their
    // policy pages.
    let locations: Vec<_> = sitemap
        .split("<loc>")
        .skip(1)
        .map(|part| part.split("</loc>").next().unwrap())
        .collect();
    assert_eq!(
        locations,
        [
            "https://coda.dabney.moe/",
            "https://coda.dabney.moe/duet",
            "https://coda.dabney.moe/clinical-transparency",
            "https://coda.dabney.moe/privacy",
            "https://coda.dabney.moe/terms",
            "https://coda.dabney.moe/duet/privacy",
            "https://coda.dabney.moe/duet/terms",
            "https://coda.dabney.moe/duet/consumer-health-data",
        ]
    );
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/robots.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        to_bytes(response.into_body(), usize::MAX).await.unwrap(),
        "User-Agent: *\nAllow: /\n\nHost: https://coda.dabney.moe\nSitemap: https://coda.dabney.moe/sitemap.xml\n"
    );
    for path in [
        "/physicians",
        "/scheduling/dev",
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

/// Every page answers 200; a trailing slash is removed with a 308 that keeps the query,
/// for pages, unknown paths and assets alike; an unknown path is 404. Every response
/// carries HSTS.
#[tokio::test]
async fn urls_resolve_redirect_and_set_hsts() {
    let app = router_with_options(
        prerender(&Config::default()).unwrap(),
        RuntimeConfig::default(),
        [],
        Readiness::ready(),
    );
    let pages = [
        "/",
        "/duet",
        "/duet/privacy",
        "/duet/privacy/2026-09-09",
        "/duet/privacy/2026-09-23",
        "/duet/privacy/2026-09-24",
        "/duet/terms",
        "/duet/terms/2026-09-08",
        "/duet/consumer-health-data",
        "/privacy",
        "/terms",
        "/clinical-transparency",
    ];
    let mut probes: Vec<(String, u16, Option<String>)> = Vec::new();
    for page in pages {
        probes.push((page.to_owned(), 200, None));
        if page != "/" {
            probes.push((format!("{page}/"), 308, Some(page.to_owned())));
        }
    }
    for (path, status, location) in [
        ("/duet//", 308, Some("/duet/")),
        ("/duet/?a=1", 308, Some("/duet?a=1")),
        ("/does-not-exist", 404, None),
        ("/does-not-exist/", 308, Some("/does-not-exist")),
        ("/brand/duet/mark.svg/", 308, Some("/brand/duet/mark.svg")),
        ("/api/health/", 308, Some("/api/health")),
    ] {
        probes.push((path.to_owned(), status, location.map(str::to_owned)));
    }
    for (path, status, location) in probes {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(&path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), status, "{path}");
        assert_eq!(
            response
                .headers()
                .get("location")
                .map(|value| value.to_str().unwrap()),
            location.as_deref(),
            "{path}"
        );
        assert_eq!(
            response.headers().get("strict-transport-security").unwrap(),
            "max-age=63072000",
            "{path}"
        );
    }
}

#[tokio::test]
async fn real_scripts_are_content_addressed_and_immutable() {
    let app = router_with_options(
        prerender(&Config::default()).unwrap(),
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
                .uri("/brand/coda-health-social-preview-v4.png")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(social.status(), StatusCode::OK);
    assert_eq!(social.headers()["content-type"], "image/png");
    let png = to_bytes(social.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
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
        prerender(&Config::default()).unwrap(),
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

#[tokio::test]
async fn invite_route_redirects_mobile_platforms_and_renders_fallback_deep_link() {
    let runtime = RuntimeConfig {
        app_store_url: "https://apps.apple.com/app/test-duet".into(),
        play_store_url: "https://play.google.com/store/apps/details?id=test.duet".into(),
        ..RuntimeConfig::default()
    };
    let app = router_with_options(
        prerender(&Config::default()).unwrap(),
        runtime,
        [],
        Readiness::ready(),
    );

    let ios = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/invite?token=inv-123")
                .header(
                    "user-agent",
                    "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X)",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ios.status(), StatusCode::FOUND);
    assert_eq!(
        ios.headers()["location"],
        "https://apps.apple.com/app/test-duet"
    );

    let android = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/invite?code=ABC123")
                .header("user-agent", "Mozilla/5.0 (Linux; Android 15; Pixel 9)")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(android.status(), StatusCode::FOUND);
    assert_eq!(
        android.headers()["location"],
        "https://play.google.com/store/apps/details?id=test.duet"
    );

    let desktop = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/invite?token=inv_99-ok")
                .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64)")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(desktop.status(), StatusCode::OK);
    assert_eq!(desktop.headers()["x-robots-tag"], "noindex, nofollow");
    let body = String::from_utf8(
        to_bytes(desktop.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(body.contains("duet://invite?token=inv_99-ok"), "{body}");
    assert!(body.contains("App Store"), "{body}");
    assert!(body.contains("Google Play"), "{body}");
}
