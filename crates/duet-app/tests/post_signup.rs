//! D1.PostSignup UI mount and full email-auth -> Terms -> PrivateRoot -> Settings tests.

use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use dioxus::prelude::*;
use duet_app::{AppDeepLink, DuetRoot, InviteHandoff, parse_app_link};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Default)]
struct StubBackendState {
    terms_accepted: bool,
    display_name: Option<String>,
    appearance: String,
    email: String,
    fail_terms_with_upgrade: bool,
}

async fn start_stub_server(state: Arc<Mutex<StubBackendState>>) -> String {
    let app = Router::new()
        .route(
            "/api/auth/email/start",
            post(|| async {
                (StatusCode::ACCEPTED, Json(json!({ "message": "Check your email" })))
            }),
        )
        .route(
            "/api/auth/email/verify",
            post({
                let st = state.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        if body["credential"]["value"] == "dl1.expired" {
                            return (
                                StatusCode::UNAUTHORIZED,
                                Json(json!({ "detail": { "code": "challenge_invalid", "message": "This link no longer works" } })),
                            )
                                .into_response();
                        }
                        let s = st.lock().unwrap();
                        (
                            StatusCode::OK,
                            Json(json!({
                                "bearer": "ds1.session-token",
                                "account_id": Uuid::nil(),
                                "incarnation": Uuid::nil(),
                                "email": s.email,
                                "appearance": s.appearance,
                            })),
                        )
                            .into_response()
                    }
                }
            }),
        )
        .route(
            "/api/auth/account",
            get({
                let st = state.clone();
                move || {
                    let st = st.clone();
                    async move {
                        let s = st.lock().unwrap();
                        Json(json!({
                            "display_name": s.display_name,
                            "terms_accepted": s.terms_accepted,
                            "terms_version": "2026-09-23",
                        }))
                    }
                }
            })
            .put({
                let st = state.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        let mut s = st.lock().unwrap();
                        if let Some(name) = body.get("display_name") {
                            s.display_name = name.as_str().map(str::to_owned);
                        }
                        Json(json!({
                            "display_name": s.display_name,
                            "terms_accepted": s.terms_accepted,
                            "terms_version": "2026-09-23",
                        }))
                    }
                }
            }),
        )
        .route(
            "/api/auth/account/context",
            get(|| async {
                Json(json!({
                    "account_id": Uuid::nil(),
                    "incarnation": Uuid::nil(),
                    "permission_revision": 0,
                    "authenticated_at": "2026-11-19T00:00:00Z",
                }))
            }),
        )
        .route(
            "/api/auth/account/profile",
            get({
                let st = state.clone();
                move || {
                    let st = st.clone();
                    async move {
                        let s = st.lock().unwrap();
                        Json(json!({ "email": s.email, "appearance": s.appearance }))
                    }
                }
            }),
        )
        .route(
            "/api/auth/account/terms",
            put({
                let st = state.clone();
                move || {
                    let st = st.clone();
                    async move {
                        let mut s = st.lock().unwrap();
                        if s.fail_terms_with_upgrade {
                            return (
                                StatusCode::CONFLICT,
                                Json(json!({ "detail": { "code": "agreement_version_changed", "message": "Update Duet to review the current terms." } })),
                            )
                                .into_response();
                        }
                        s.terms_accepted = true;
                        (
                            StatusCode::OK,
                            Json(json!({
                                "display_name": s.display_name,
                                "terms_accepted": true,
                                "terms_version": "2026-09-23",
                            })),
                        )
                            .into_response()
                    }
                }
            }),
        )
        .route(
            "/api/evidence/library",
            post(|| async { Json(json!({ "entries": [], "counts": {}, "total_count": 0 })) }),
        )
        .route(
            "/api/doctor-collection",
            get(|| async { Json(json!({ "note_generation": 1, "items": [] })) }),
        )
        .route(
            "/api/for-my-doctor",
            get(|| async {
                Json(json!({
                    "marks": [],
                    "notes": [],
                    "sends": [],
                    "notices": [],
                    "practices": []
                }))
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{addr}")
}

#[test]
fn signed_out_mount_renders_email_flow_without_apple_sign_in() {
    #[component]
    fn Root() -> Element {
        rsx! { DuetRoot { base_url: "http://127.0.0.1:1".to_owned(), bearer: String::new() } }
    }
    let mut dom = VirtualDom::new(Root);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Sign in to your Duet account"), "{html}");
    assert!(html.contains("Continue"), "{html}");
    assert!(!html.contains("Sign in with Apple"), "{html}");
}

#[tokio::test(flavor = "current_thread")]
async fn d1_post_signup_mounts_terms_gate_and_then_you_and_shared_tabs() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let state = Arc::new(Mutex::new(StubBackendState {
                terms_accepted: false,
                appearance: "system".into(),
                email: "ada@example.test".into(),
                ..Default::default()
            }));
            let base = start_stub_server(state.clone()).await;

            #[derive(Props, Clone, PartialEq)]
            struct HarnessProps {
                base: String,
                bearer: String,
                link: Option<String>,
            }
            #[component]
            fn Harness(props: HarnessProps) -> Element {
                rsx! {
                    DuetRoot {
                        base_url: props.base,
                        bearer: props.bearer,
                        initial_link: props.link,
                    }
                }
            }

            async fn wait_for_render(dom: &mut VirtualDom, needle: &str) -> String {
                let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(3);
                loop {
                    dom.render_immediate(&mut dioxus::core::NoOpMutations);
                    let html = dioxus_ssr::render(dom);
                    if html.contains(needle) || tokio::time::Instant::now() >= deadline {
                        return html;
                    }
                    tokio::select! {
                        _ = dom.wait_for_work() => {}
                        _ = tokio::time::sleep(std::time::Duration::from_millis(25)) => {}
                    }
                }
            }

            // 1. Before Terms are accepted, supplied bearer lands on Terms gate (blocking You & Shared)
            //    while keeping Rights-never-wait Settings accessible.
            let mut dom = VirtualDom::new_with_props(
                Harness,
                HarnessProps {
                    base: base.clone(),
                    bearer: "ds1.session-token".into(),
                    link: Some("duet://invite?token=inv-77".into()),
                },
            );
            dom.rebuild_in_place();
            let terms_html = wait_for_render(&mut dom, "Duet Terms").await;
            assert!(terms_html.contains("Duet Terms"), "{terms_html}");
            assert!(
                terms_html.contains("I am 18 or older and live in the United States"),
                "{terms_html}"
            );
            assert!(terms_html.contains("I agree and continue"), "{terms_html}");
            assert!(terms_html.contains("Settings"), "{terms_html}");
            assert!(
                !terms_html.contains("Add health information"),
                "You tab must stay gated before Terms acceptance:\n{terms_html}"
            );

            // 2. Once Terms are accepted, mounting DuetRoot opens PrivateRoot on You with Shared beside it
            //    and opens the handed-off practice invitation inside the signed-in account.
            state.lock().unwrap().terms_accepted = true;
            let mut post_signup_dom = VirtualDom::new_with_props(
                Harness,
                HarnessProps {
                    base,
                    bearer: "ds1.session-token".into(),
                    link: Some("duet://invite?token=inv-77".into()),
                },
            );
            post_signup_dom.rebuild_in_place();
            let private_html = wait_for_render(&mut post_signup_dom, "Practice invitation").await;
            assert!(private_html.contains(">You<"), "{private_html}");
            assert!(private_html.contains(">Shared<"), "{private_html}");
            assert!(private_html.contains("Settings"), "{private_html}");
            assert!(
                private_html.contains("Practice invitation") && private_html.contains("inv-77"),
                "invite deep link must hand off inside signed-in account:\n{private_html}"
            );
        })
        .await;
}

#[test]
fn app_link_parser_accepts_invite_and_auth_links() {
    assert_eq!(
        parse_app_link("https://codahealth.ai/invite?token=join-abc"),
        Some(AppDeepLink::Invite(InviteHandoff::Token("join-abc".into())))
    );
    assert_eq!(
        parse_app_link("duet://auth/email?token=dl1.xyz"),
        Some(AppDeepLink::EmailSignIn {
            token: "dl1.xyz".into(),
            invite: None,
        })
    );
}
