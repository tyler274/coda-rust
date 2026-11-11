//! Mounted product refusal proof; synthetic identity is not hosted/native qualification.
use super::*;
use axum::{http::StatusCode, response::IntoResponse};
use std::sync::atomic::{AtomicU8, Ordering};
use tower::ServiceExt;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn definitive_refusal_unmounts_private_content() {
    let refuse = Arc::new(AtomicU8::new(0));
    let route_refuse = refuse.clone();
    let stub = duet_stub::router(Arc::new(Mutex::new(
        duet_stub::Store::open(
            std::env::temp_dir().join(format!("refusal-{}.json", uuid::Uuid::new_v4())),
        )
        .unwrap(),
    )));
    let context = serde_json::json!({"account_id":uuid::Uuid::new_v4(),"incarnation":uuid::Uuid::new_v4(),"permission_revision":0,"authenticated_at":"2026-10-20T00:00:00Z"});
    let api = axum::Router::new()
        .route(
            "/api/auth/account",
            axum::routing::put(|| async { axum::Json(serde_json::json!({})) }),
        )
        .route(
            "/api/auth/account/context",
            axum::routing::get(move || {
                let context = context.clone();
                async move { axum::Json(context) }
            }),
        )
        .fallback(move |request: axum::http::Request<axum::body::Body>| {
            let stub = stub.clone();
            let refuse = route_refuse.load(Ordering::SeqCst);
            async move {
                if refuse != 0
                    && request.uri().path() == "/api/evidence"
                    && request.method() == "POST"
                {
                    match refuse {
                        1 => (
                            StatusCode::UNAUTHORIZED,
                            "<html>IAP authentication required</html>",
                        )
                            .into_response(),
                        2 => duet_api::ApiError::Unavailable("Identity provider unavailable")
                            .into_response(),
                        3 => duet_api::ApiError::SessionRevoked.into_response(),
                        _ => duet_api::ApiError::AbsentAccount.into_response(),
                    }
                } else {
                    stub.oneshot(request).await.unwrap()
                }
            }
        });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let site = site_router(shared_site(), api);
    let server = tokio::spawn(async move { axum::serve(listener, site).await.unwrap() });
    let base = format!("http://{address}");
    seed_record(&base, "Private account record").await;
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    browser.text_until(&format!("{base}/?mode=client&api={base}"), "Shared");
    browser.wait_for("Private account record");
    for failure in [1, 2] {
        refuse.store(failure, Ordering::SeqCst);
        start_correction(
            &mut browser,
            "Private account record",
            "Unconfirmed private correction",
        );
        browser.wait_for("This correction couldn’t be saved.");
        assert!(
            browser
                .body_text()
                .unwrap()
                .contains("Private account record")
        );
        assert_eq!(
            browser
                .page()
                .run("inputValue", serde_json::json!([CORRECTED])),
            "Unconfirmed private correction"
        );
    }
    refuse.store(3, Ordering::SeqCst);
    start_correction(
        &mut browser,
        "Private account record",
        "Unconfirmed private correction",
    );
    browser.wait_for("Sign in to your Duet account.");
    let text = browser.body_text().unwrap();
    assert!(!text.contains("Private account record"));
    assert!(!text.contains("Unconfirmed private correction"));
    screenshot(&mut browser, "after-refusal");
    eprintln!("Product session_revoked removed mounted private records and the open correction.");
    refuse.store(0, Ordering::SeqCst);
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Retry saved sign-in"])),
        true
    );
    browser.wait_for("Private account record");
    refuse.store(4, Ordering::SeqCst);
    start_correction(
        &mut browser,
        "Private account record",
        "Deleted account correction",
    );
    browser.wait_for("Sign in to your Duet account.");
    assert!(
        !browser
            .body_text()
            .unwrap()
            .contains("Private account record")
    );
    eprintln!("Product absent-account refusal removed mounted private records.");
    server.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn late_a_refusal_cannot_end_mounted_b_session() {
    let second = Arc::new(AtomicU8::new(0));
    let entered = Arc::new(tokio::sync::Notify::new());
    let release = Arc::new(tokio::sync::Notify::new());
    let returned = Arc::new(tokio::sync::Notify::new());
    let contexts = [uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
    let identity_second = second.clone();
    let route_entered = entered.clone();
    let route_release = release.clone();
    let route_returned = returned.clone();
    let route_second = second.clone();
    let stores: Vec<_> = (0..2)
        .map(|_| {
            duet_stub::router(Arc::new(Mutex::new(
                duet_stub::Store::open(
                    std::env::temp_dir()
                        .join(format!("late-refusal-{}.json", uuid::Uuid::new_v4())),
                )
                .unwrap(),
            )))
        })
        .collect();
    let api = axum::Router::new()
        .route("/api/auth/account", axum::routing::put(|| async { axum::Json(serde_json::json!({})) }))
        .route("/api/auth/account/context", axum::routing::get(move || {
            let id = contexts[usize::from(identity_second.load(Ordering::SeqCst))];
            async move { axum::Json(serde_json::json!({"account_id":id,"incarnation":id,"permission_revision":0,"authenticated_at":"2026-10-20T00:00:00Z"})) }
        }))
        .fallback(move |request: axum::http::Request<axum::body::Body>| {
            let index = usize::from(route_second.load(Ordering::SeqCst));
            let store = stores[index].clone();
            let entered = route_entered.clone();
            let release = route_release.clone();
            let returned = route_returned.clone();
            async move {
                if index == 0 && request.method() == "POST" && request.uri().path() == "/api/evidence" {
                    let (send, receive) = tokio::sync::oneshot::channel();
                    tokio::spawn(async move {
                        entered.notify_one();
                        release.notified().await;
                        let delivered = send.send(duet_api::ApiError::SessionRevoked.into_response()).is_ok();
                        eprintln!("Released stale A refusal after mounted B activation; transport waiting: {delivered}");
                        returned.notify_one();
                    });
                    receive.await.unwrap()
                } else { store.oneshot(request).await.unwrap() }
            }
        });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let site = site_router(shared_site(), api);
    let server = tokio::spawn(async move { axum::serve(listener, site).await.unwrap() });
    let base = format!("http://{address}");
    second.store(1, Ordering::SeqCst);
    seed_record(&base, "B private record").await;
    second.store(0, Ordering::SeqCst);
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    browser.text_until(&format!("{base}/?mode=client&api={base}"), "Shared");
    browser.wait_for("Lisinopril");
    start_correction(&mut browser, "Lisinopril", "A pending correction");
    tokio::time::timeout(Duration::from_secs(10), entered.notified())
        .await
        .unwrap();
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Sign out"])),
        true
    );
    browser.wait_for("Sign in to your Duet account.");
    second.store(1, Ordering::SeqCst);
    assert_eq!(
        browser
            .page()
            .run("clickButton", serde_json::json!(["Retry saved sign-in"])),
        true
    );
    browser.wait_for("B private record");
    browser.click(r#"button[aria-label="Correct B private record"]"#);
    browser.type_into(CORRECTED, "B private draft");
    release.notify_one();
    tokio::time::timeout(Duration::from_secs(10), returned.notified())
        .await
        .unwrap();
    for _ in 0..10 {
        let text = browser.body_text().unwrap();
        assert!(text.contains("B private record"));
        assert!(!text.contains("A pending correction"));
        assert!(!text.contains("Sign in to your Duet account."));
        assert_eq!(
            browser
                .page()
                .run("inputValue", serde_json::json!([CORRECTED])),
            "B private draft"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    screenshot(&mut browser, "after-stale-a-refusal");
    eprintln!("Mounted B records and open correction survived stale A refusal.");
    server.abort();
}

const CORRECTED: &str = r#"input[aria-label="Corrected value"]"#;

/// Stores one manual medication through the mounted stub, as the account in front.
async fn seed_record(base: &str, title: &str) {
    let response = reqwest::Client::new()
        .post(format!("{base}/api/evidence"))
        .json(&serde_json::json!({"items":[{
            "id": uuid::Uuid::new_v4(),
            "entity": "health_evidence",
            "operation": "upsert",
            "payload": {
                "display_name": title,
                "concept_id": "medication:manual",
                "source_label": "Added in Duet"
            }
        }]}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

/// Opens the correction for `title`, types `value` and saves it.
fn start_correction(browser: &mut Browser, title: &str, value: &str) {
    browser.click(&format!(r#"button[aria-label="Correct {title}"]"#));
    browser.type_into(CORRECTED, value);
    browser.click(r#"button[aria-label="Save correction"]"#);
}

fn screenshot(browser: &mut Browser, name: &str) {
    use base64::Engine;
    let image = browser.page().call_session(
        "Page.captureScreenshot",
        serde_json::json!({"format":"png"}),
    );
    let folder =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/verification/session-refusal");
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(
        folder.join(format!("{name}.png")),
        base64::engine::general_purpose::STANDARD
            .decode(image["result"]["data"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
}
