//! Mounted-client isolation with a synthetic identity boundary and delayed HTTP response.
//! This does not qualify Identity Platform, native retention or hosted account isolation.
use std::sync::atomic::{AtomicBool, Ordering};

use super::*;
use axum::http::{Request, StatusCode};
use base64::Engine;
use tokio::sync::Notify;
use tower::ServiceExt;

const A_TRANSCRIPT: &str = "Account A private transcript";
const A_PENDING: &str = "Account A pending send";
const A_DRAFT: &str = "Account A unsent draft";
const B_TRANSCRIPT: &str = "Account B private transcript";
const B_DRAFT: &str = "Account B unsent draft";

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn pending_conversation_response_cannot_cross_account_switch() {
    let second = Arc::new(AtomicBool::new(false));
    let hold = Arc::new(AtomicBool::new(false));
    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let returned = Arc::new(AtomicBool::new(false));
    let contexts = [context(), context()];
    let apis = [stub(), stub()];
    let identity_contexts = contexts.clone();
    let identity_second = second.clone();
    let identity = axum::Router::new()
        .route(
            "/api/auth/account",
            axum::routing::put(|headers| async move {
                verify(headers)?;
                Ok::<_, StatusCode>(axum::Json(serde_json::json!({})))
            }),
        )
        .route(
            "/api/auth/account/context",
            axum::routing::get(move |headers| {
                let context =
                    identity_contexts[usize::from(identity_second.load(Ordering::SeqCst))].clone();
                async move {
                    verify(headers)?;
                    Ok::<_, StatusCode>(axum::Json(context))
                }
            }),
        );
    let routed_second = second.clone();
    let routed_hold = hold.clone();
    let routed_entered = entered.clone();
    let routed_release = release.clone();
    let routed_returned = returned.clone();
    let api = identity.fallback(move |request: Request<axum::body::Body>| {
        let index = usize::from(routed_second.load(Ordering::SeqCst));
        let api = apis[index].clone();
        let should_hold = index == 0
            && request.method() == "POST"
            && request.uri().path() == "/api/conversation/messages"
            && routed_hold.load(Ordering::SeqCst);
        let entered = routed_entered.clone();
        let release = routed_release.clone();
        let returned = routed_returned.clone();
        async move {
            let response = api.oneshot(request).await.unwrap();
            if should_hold {
                assert_eq!(response.status(), StatusCode::ACCEPTED);
                let (sender, receiver) = tokio::sync::oneshot::channel();
                tokio::spawn(async move {
                    entered.notify_one();
                    release.notified().await;
                    let delivered = sender.send(response).is_ok();
                    eprintln!("Released A response after B activation; transport still waiting: {delivered}");
                    returned.store(true, Ordering::SeqCst);
                });
                return receiver.await.unwrap();
            }
            response
        }
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = site_router(shared_site(), api);
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let base = format!("http://{address}");
    let http = reqwest::Client::new();
    for (is_b, text) in [(false, A_TRANSCRIPT), (true, B_TRANSCRIPT)] {
        second.store(is_b, Ordering::SeqCst);
        let response = http
            .post(format!("{base}/api/conversation/messages"))
            .json(&serde_json::json!({"id":uuid::Uuid::new_v4(),"text":text,"timezone":"UTC"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }
    second.store(false, Ordering::SeqCst);
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    let initial = browser.text_until(&format!("{base}/?mode=client&api={base}"), A_TRANSCRIPT);
    assert!(!initial.contains(B_TRANSCRIPT));
    let document = browser
        .page()
        .run("beginAccountSwitchProof", serde_json::json!([]));
    hold.store(true, Ordering::SeqCst);
    browser.type_into(".composer-input", A_PENDING);
    browser.click(".duet-send");
    tokio::time::timeout(Duration::from_secs(10), entered.notified())
        .await
        .unwrap();
    browser.type_into(".composer-input", A_DRAFT);
    assert_eq!(composer(&mut browser), A_DRAFT);
    click_button(&mut browser, "Sign out");
    browser.wait_for("Sign in to your Duet account.");
    assert_private_absent(&mut browser);
    // The test identity adapter supplies B at the next verification; DuetRoot itself stays mounted.
    second.store(true, Ordering::SeqCst);
    click_button(&mut browser, "Retry saved sign-in");
    browser.wait_for(B_TRANSCRIPT);
    assert_private_absent(&mut browser);
    assert_eq!(composer(&mut browser), "");
    browser.type_into(".composer-input", B_DRAFT);
    release.notify_one();
    tokio::time::timeout(Duration::from_secs(10), async {
        while !returned.load(Ordering::SeqCst) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    // Repeated browser reads give the released request and scheduled UI work time to settle.
    for _ in 0..12 {
        assert_private_absent(&mut browser);
        assert!(browser.body_text().unwrap().contains(B_TRANSCRIPT));
        assert_eq!(composer(&mut browser), B_DRAFT);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let snapshot = browser
        .page()
        .run("accountSwitchSnapshot", serde_json::json!([]));
    assert_eq!(
        snapshot["document"], document,
        "account switch reloaded the document"
    );
    assert_eq!(snapshot["storageKeys"], serde_json::json!([]));
    let evidence =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/verification/account-switch");
    std::fs::create_dir_all(&evidence).unwrap();
    std::fs::write(
        evidence.join("after-release.json"),
        serde_json::to_vec_pretty(&snapshot).unwrap(),
    )
    .unwrap();
    browser
        .page()
        .run("scrollTo", serde_json::json!([".composer-input"]));
    let screenshot = browser.page().call_session(
        "Page.captureScreenshot",
        serde_json::json!({"format":"png"}),
    );
    std::fs::write(
        evidence.join("after-release.png"),
        base64::engine::general_purpose::STANDARD
            .decode(screenshot["result"]["data"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    eprintln!(
        "Mounted isolation: A transcript visible; A send held; A draft entered; signed out; B verified without document reload; B transcript and draft retained after A response release; A data absent."
    );
    server.abort();
}

fn context() -> serde_json::Value {
    serde_json::json!({"account_id":uuid::Uuid::new_v4(),"incarnation":uuid::Uuid::new_v4(),"permission_revision":0,"authenticated_at":"2026-10-20T00:00:00Z"})
}
fn verify(headers: axum::http::HeaderMap) -> Result<(), StatusCode> {
    if headers.get("authorization").and_then(|h| h.to_str().ok())
        == Some("Bearer synthetic-stub-browser-only")
    {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
fn stub() -> axum::Router {
    duet_stub::router(Arc::new(Mutex::new(
        duet_stub::Store::open(
            std::env::temp_dir().join(format!("duet-switch-{}.json", uuid::Uuid::new_v4())),
        )
        .unwrap(),
    )))
}
fn composer(browser: &mut Browser) -> String {
    browser
        .page()
        .run("inputValue", serde_json::json!([".composer-input"]))
        .as_str()
        .unwrap()
        .to_owned()
}
fn assert_private_absent(browser: &mut Browser) {
    let text = browser.body_text().unwrap();
    for value in [A_TRANSCRIPT, A_PENDING, A_DRAFT] {
        assert!(!text.contains(value), "A data leaked after switch: {text}");
    }
}
fn click_button(browser: &mut Browser, text: &str) {
    assert_eq!(
        browser.page().run("clickButton", serde_json::json!([text])),
        serde_json::json!(true)
    );
}
