//! Live incremental callback -> durable event batches -> actual patient reducer.
//! Synthetic identity and deterministic provider only; disposable PostgreSQL.
use async_trait::async_trait;
use axum::{Router, body::Body, http::Request};
use duet_api::{
    ApiError,
    conversation::ConversationStore,
    conversation_routes::ConversationState,
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore},
};
use duet_client::streaming::{Provisional, parse_batch};
use harness::execution::{
    Cancellation, Limits, Provider, Request as ModelRequest, Response, StreamEvent,
};
use serde_json::{Value, json};
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;
struct Identity;
#[async_trait]
impl IdentityVerifier for Identity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        if !matches!(bearer, "a" | "b") {
            return Err(ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: format!("stream-{bearer}"),
            authenticated_at: chrono::Utc::now(),
        })
    }
}
struct Paused {
    sent: Arc<tokio::sync::Notify>,
    release: Arc<tokio::sync::Notify>,
    fail: bool,
    final_text: &'static str,
}
impl Provider for Paused {
    async fn exchange(
        &mut self,
        _: &ModelRequest,
        cancel: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        emit(StreamEvent::Text("Provisional only".into()))?;
        self.sent.notify_one();
        self.release.notified().await;
        // Even cancelled output can report billable usage. The owner retains it
        // without permitting publication after withdrawal.
        emit(StreamEvent::Usage(harness::execution::Usage {
            input_tokens: Some(17),
            output_tokens: Some(3),
        }))
        .ok();
        cancel.check()?;
        if self.fail {
            return Err("controlled interruption".into());
        }
        emit(StreamEvent::Text(" late".into()))?;
        Ok(Response {
            text: self.final_text.into(),
            calls: vec![],
            usage: Default::default(),
            provenance: json!({"mode":"local"}),
        })
    }
}
async fn request(
    app: &Router,
    method: &str,
    path: &str,
    bearer: &str,
    body: Value,
) -> (u16, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("authorization", format!("Bearer {bearer}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let code = response.status().as_u16();
    let bytes = axum::body::to_bytes(response.into_body(), 3_000_000)
        .await
        .unwrap();
    (code, String::from_utf8(bytes.to_vec()).unwrap())
}
#[tokio::test]
async fn provisional_replay_interruption_recovery_and_withdrawal() {
    let url = std::env::var("DUET_TEST_DATABASE_URL").expect("real local PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("stream_{suffix}");
    let role = format!("stream_role_{suffix}");
    sqlx::query(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    migrate(options.to_url_lossy().as_str(), &role)
        .await
        .unwrap();
    let runtime = options.username(&role).to_url_lossy().to_string();
    let db = AccountDatabase::connect(&runtime).await.unwrap();
    let directory = std::env::temp_dir().join(format!("stream-journal-{suffix}"));
    let durable = DurableState {
        identity: IdentityStore::new(db.clone(), Arc::new(Identity)),
        evidence: EvidenceStore::new(db.clone()),
        privacy: PrivacyStore::new(
            db.clone(),
            Arc::new(FileJournal::open(directory.clone()).await.unwrap()),
        ),
    };
    let store = ConversationStore::new(db.clone());
    let app = duet_api::durable::router(durable.clone()).merge(
        duet_api::conversation_routes::router(ConversationState {
            durable: durable.clone(),
            store: store.clone(),
            model: "local".into(),
            provider: None,
        }),
    );
    for who in ["a", "b"] {
        for (path, body) in [
            ("/api/auth/account", json!({})),
            ("/api/auth/account/terms", json!({"version":"2026-09-23"})),
            (
                "/api/auth/account/luna-consent",
                json!({"decision":"accepted","version":"2026-09-24","expected_revision":0}),
            ),
        ] {
            let result = request(&app, "PUT", path, who, body).await;
            assert!((200..300).contains(&result.0), "{result:?}");
        }
    }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let api = duet_client::DuetClient::new(
        format!("http://{}", listener.local_addr().unwrap()),
        "a",
        "1",
        "test",
    )
    .unwrap();
    let served = app.clone();
    let server = tokio::spawn(async move { axum::serve(listener, served).await.unwrap() });
    let c = durable.identity.authenticate("a", true).await.unwrap();
    for mode in ["interrupt", "complete", "retain_stream", "withdraw"] {
        let id = Uuid::new_v4();
        store
            .phone_accept(&c, id, "Please answer", "local")
            .await
            .unwrap();
        let sent = Arc::new(tokio::sync::Notify::new());
        let release = Arc::new(tokio::sync::Notify::new());
        let mut owner = store.claim(&c, id).await.unwrap();
        let final_text = if mode == "retain_stream" {
            "Provisional only late"
        } else {
            "Approved final"
        };
        let mut provider = Paused {
            sent: sent.clone(),
            release: release.clone(),
            fail: mode == "interrupt",
            final_text,
        };
        let task = tokio::spawn(async move {
            owner
                .execute(&mut provider, Limits::default(), &Cancellation::default())
                .await
        });
        sent.notified().await;
        let path = format!("/api/conversation/commands/{id}/events");
        let mut consumer = Provisional::new(id);
        tokio::time::timeout(std::time::Duration::from_secs(3), async {
            loop {
                let (status, body) = request(&app, "GET", &path, "a", Value::Null).await;
                assert_eq!(status, 200, "{body}");
                let batch = api.conversation_events(id, 0).await.unwrap();
                consumer.apply(id, &batch).unwrap();
                consumer.apply(id, &batch).unwrap();
                if consumer.text == "Provisional only" {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        assert!(
            !task.is_finished(),
            "must deliver while provider is still open"
        );
        let page = store.phone_page(&c, true).await.unwrap();
        assert!(
            !page["messages"].as_array().unwrap().iter().any(|message| {
                message["run_id"] == json!(id) && message["role"] == "assistant"
            }),
            "provisional text must not become a completed reply: {page}"
        );
        assert_eq!(request(&app, "GET", &path, "b", Value::Null).await.0, 404);
        assert_eq!(
            request(
                &app,
                "GET",
                &format!("{path}?after={}", consumer.after),
                "a",
                Value::Null
            )
            .await
            .1,
            "retry: 1000\n\n"
        );
        if mode == "withdraw" {
            let result = request(
                &app,
                "PUT",
                "/api/auth/account/luna-consent",
                "a",
                json!({"decision":"declined","version":"2026-09-24","expected_revision":1}),
            )
            .await;
            assert!((200..300).contains(&result.0), "{result:?}");
            assert_ne!(request(&app, "GET", &path, "a", Value::Null).await.0, 200);
            consumer.clear();
            assert!(consumer.text.is_empty());
        }
        let usage_path = format!("/api/conversation/commands/{id}/usage");
        let (status, body) = request(&app, "GET", &usage_path, "a", Value::Null).await;
        assert_eq!(status, 200, "{body}");
        let usage: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(usage["unknown_usage_model_calls"], 1);
        assert_eq!(
            request(&app, "GET", &usage_path, "b", Value::Null).await.0,
            404
        );
        release.notify_one();
        let result = task.await.unwrap();
        let (status, body) = request(&app, "GET", &usage_path, "a", Value::Null).await;
        assert_eq!(status, 200, "{body}");
        let usage: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(usage["model_calls"], 1);
        assert_eq!(usage["input_tokens"], 17);
        assert_eq!(usage["output_tokens"], 3);
        assert_eq!(usage["unknown_usage_model_calls"], 0);
        assert_eq!(usage["cost_status"], "unknown", "local model has no price");
        if matches!(mode, "complete" | "retain_stream") {
            assert_eq!(result.unwrap().text, final_text);
            // Read through a recreated owner: the shared loop's candidate must
            // reach durable finalization, not just the in-memory return value.
            let restored =
                ConversationStore::new(AccountDatabase::connect(&runtime).await.unwrap());
            let page = restored.phone_page(&c, true).await.unwrap();
            let replies: Vec<_> = page["messages"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|message| message["run_id"] == json!(id) && message["role"] == "assistant")
                .collect();
            assert_eq!(replies.len(), 1, "{page}");
            assert_eq!(replies[0]["text"], final_text, "{page}");
        } else {
            assert!(result.is_err());
        }
        if mode != "withdraw" {
            let (_, body) = request(&app, "GET", &path, "a", Value::Null).await;
            consumer.apply(id, &parse_batch(&body).unwrap()).unwrap();
            assert!(consumer.stopped);
            assert!(consumer.text.is_empty());
        }
        if mode == "interrupt" {
            let restored =
                ConversationStore::new(AccountDatabase::connect(&runtime).await.unwrap());
            let mut owner = restored.claim(&c, id).await.unwrap();
            let release = Arc::new(tokio::sync::Notify::new());
            release.notify_one();
            owner
                .execute(
                    &mut Paused {
                        sent: Arc::new(tokio::sync::Notify::new()),
                        release,
                        fail: false,
                        final_text: "Approved final",
                    },
                    Limits::default(),
                    &Cancellation::default(),
                )
                .await
                .unwrap();
            let (_, body) = request(&app, "GET", &path, "a", Value::Null).await;
            consumer.apply(id, &parse_batch(&body).unwrap()).unwrap();
            assert!(consumer.stopped);
            assert!(consumer.text.is_empty());
        }
    }
    server.abort();
    drop(app);
    drop(store);
    drop(durable);
    drop(db);
    sqlx::query(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
    std::fs::remove_dir_all(directory).unwrap();
}
