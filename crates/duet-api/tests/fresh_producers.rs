//! Fresh-install HTTP producer -> deterministic executor -> source reopening.
//! All product rows originate through authenticated product APIs; SQL only
//! provisions isolated roles/schema and inspects durability/authority proofs.
use async_trait::async_trait;
use axum::{Router, body::Body, http::Request};
use chrono::{Duration, Utc};
use duet_api::{
    ApiError,
    conversation_routes::ConversationState,
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore},
    producer_database::{ProducerKind, grant},
    producer_routes::ProducerState,
};
use harness::execution::{
    Call, Cancellation, Limits, Provider, Request as ModelRequest, Response, StreamEvent,
};
use serde_json::{Value, json};
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
            subject: format!("fresh-{bearer}"),
            authenticated_at: Utc::now(),
        })
    }
}
async fn http(app: &Router, method: &str, path: &str, who: &str, body: Value) -> (u16, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("authorization", format!("Bearer {who}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status().as_u16();
    let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({"text":String::from_utf8_lossy(&bytes)})),
    )
}
async fn ok(app: &Router, method: &str, path: &str, who: &str, body: Value) -> Value {
    let (status, value) = http(app, method, path, who, body).await;
    assert!(
        (200..300).contains(&status),
        "{method} {path}: {status} {value}"
    );
    value
}
struct Script {
    batches: std::collections::VecDeque<Vec<Call>>,
    text: String,
}
impl Provider for Script {
    async fn exchange(
        &mut self,
        _: &ModelRequest,
        _: &Cancellation,
        _: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        Ok(Response {
            text: if self.batches.is_empty() {
                self.text.clone()
            } else {
                String::new()
            },
            calls: self.batches.pop_front().unwrap_or_default(),
            usage: Default::default(),
            provenance: json!({"mode":"deterministic-local"}),
        })
    }
}
fn call(id: &str, name: &str, args: Value) -> Call {
    Call {
        id: id.into(),
        name: name.into(),
        arguments: args,
    }
}
async fn execute(
    store: &duet_api::conversation::ConversationStore,
    c: &duet_api::identity::AccountContext,
    id: Uuid,
    batches: Vec<Vec<Call>>,
) {
    let mut owner = store.claim(c, id).await.unwrap();
    owner
        .execute(
            &mut Script {
                batches: batches.into(),
                text: "Completed controlled accepted work".into(),
            },
            Limits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
}
#[tokio::test]
async fn fresh_http_producers_execute_admit_reopen_and_restrict() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("real disposable loopback PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("fresh_{suffix}");
    let reader = format!("reader_{suffix}");
    let context_role = format!("context_{suffix}");
    let executor = format!("executor_{suffix}");
    for role in [&reader, &context_role, &executor] {
        sqlx::query(&format!("create role {role} login"))
            .execute(&mut admin)
            .await
            .unwrap();
    }
    sqlx::query(&format!("create database {name}"))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    let owner_url = options.to_url_lossy().to_string();
    for role in [&reader, &context_role, &executor] {
        migrate(&owner_url, role).await.unwrap();
    }
    grant(&owner_url, &context_role, ProducerKind::HealthContext)
        .await
        .unwrap();
    grant(&owner_url, &executor, ProducerKind::AcceptedWork)
        .await
        .unwrap();
    let reader_url = options.clone().username(&reader).to_url_lossy().to_string();
    let context_url = options
        .clone()
        .username(&context_role)
        .to_url_lossy()
        .to_string();
    let executor_url = options.username(&executor).to_url_lossy().to_string();
    let db = AccountDatabase::connect(&reader_url).await.unwrap();
    let write_db = AccountDatabase::connect(&executor_url).await.unwrap();
    let journal_path = std::env::temp_dir().join(format!("fresh-journal-{suffix}"));
    let durable = DurableState {
        identity: IdentityStore::new(db.clone(), Arc::new(Identity)),
        evidence: EvidenceStore::new(db.clone()),
        privacy: PrivacyStore::new(
            write_db.clone(),
            Arc::new(FileJournal::open(journal_path.clone()).await.unwrap()),
        ),
    };
    let store = duet_api::conversation::ConversationStore::new(write_db.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/responses", listener.local_addr().unwrap());
    let model=Router::new().route("/responses",axum::routing::post(|axum::Json(body):axum::Json<Value>|async move {
        let input=body["input"].as_array().unwrap();
        let inventory=input.iter().filter_map(|v|v["content"].as_str()).find(|s|s.starts_with("Assigned Today slots")).unwrap();
        let inventory:Value=serde_json::from_str(&inventory[inventory.find('{').unwrap()..]).unwrap();
        let slot=&inventory["slots"][0];
        let output=if !input.iter().any(|v|v["call_id"]=="api-read") {
            json!([{"type":"function_call","call_id":"api-read","name":"read_assessment_fact","arguments":json!({"cycle_id":slot["cycle_id"],"fact_id":slot["facts"][0]["id"]}).to_string()}])
        } else if !input.iter().any(|v|v["call_id"]=="api-publish") {
            json!([{"type":"function_call","call_id":"api-publish","name":"stage_today","arguments":json!({"cycle_id":slot["cycle_id"],"outcome":"observations","insights":[{"title":"Recorded steps","detail":"Your source recorded 7654 steps yesterday.","read_ids":["api-read"]}]}).to_string()}])
        } else {json!([{"type":"message","content":[{"type":"output_text","text":"Assessment complete"}]}])};
        let event=json!({"type":"response.completed","response":{"status":"completed","output":output,"usage":{"input_tokens":3,"output_tokens":2}}});
        ([("content-type","text/event-stream")],format!("data: {event}\n\n"))
    }));
    let model_task = tokio::spawn(async move { axum::serve(listener, model).await.unwrap() });
    let conversation = ConversationState {
        durable: durable.clone(),
        store: store.clone(),
        model: "local".into(),
        provider: Some((endpoint, "synthetic-local-only".into())),
    };
    let producer = duet_api::health_context::HealthContextProducer::connect(&context_url)
        .await
        .unwrap();
    let today = duet_api::today::TodayStore::connect_producer(&executor_url)
        .await
        .unwrap();
    let app = duet_api::durable::router(durable.clone())
        .merge(duet_api::producer_routes::router(ProducerState {
            durable: durable.clone(),
            context: producer.clone(),
            conversation: conversation.clone(),
            today: today.clone(),
        }))
        .merge(duet_api::conversation_routes::router(conversation));
    for who in ["a", "b"] {
        ok(&app, "PUT", "/api/auth/account", who, json!({})).await;
        ok(
            &app,
            "PUT",
            "/api/auth/account/terms",
            who,
            json!({"version":"2026-09-23"}),
        )
        .await;
        ok(
            &app,
            "PUT",
            "/api/auth/account/luna-consent",
            who,
            json!({"decision":"accepted","version":"2026-09-24","expected_revision":0}),
        )
        .await;
    }
    let a = durable
        .authenticate(
            &[(
                axum::http::header::AUTHORIZATION,
                "Bearer a".parse().unwrap(),
            )]
            .into_iter()
            .collect(),
        )
        .await
        .unwrap();
    let b = durable.identity.authenticate("b", false).await.unwrap();
    assert!(
        ok(&app, "GET", "/api/health-context", "a", json!(null))
            .await
            .is_null()
    );
    let start = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    let record = Uuid::new_v4();
    ok(&app,"POST","/api/canonical-evidence","a",json!({"operation_id":Uuid::new_v4(),"canonical_record_id":record,"record_id":"phone-steps-yesterday","entity":"health_evidence","source_generation":0,"payload":{"id":record,"concept_id":"steps","display_name":"Steps","verification":"verified","method":"daily_aggregate","measured_interval":{"start":start-Duration::hours(12)},"source":{"kind":"appleHealth","connection_id":"phone","issuer":"watch","provenance":"native"},"value":{"type":"quantity","quantity":{"canonical_value":7654.0,"canonical_unit":"count"}},"context":{"aggregation":"daily_sum","day_timezone":"UTC","day_partial":false}}})).await;
    let capture = json!({"expected_revision":null,"snapshot":{"id":Uuid::new_v4(),"source_boundary_id":Uuid::new_v4(),"schema_version":1,"generated_at":Utc::now(),"timezone":"UTC","source_generations":{},"projections":[{"kind":"daily","window_start":start,"window_end":start+Duration::days(1),"timezone":"UTC","first_weekday":7,"facts":[]}]}});
    let context = ok(&app, "POST", "/api/health-context", "a", capture.clone()).await;
    assert_eq!(
        ok(&app, "POST", "/api/health-context", "a", capture).await,
        context
    );
    assert!(
        ok(&app, "GET", "/api/health-context", "b", json!(null))
            .await
            .is_null()
    );
    let facts = context["snapshot"]["projections"][0]["facts"]
        .as_array()
        .unwrap();
    let fact = facts
        .iter()
        .find(|f| f["kind"] == "current_value" && f["window"] == "day")
        .unwrap()
        .clone();
    assert_eq!(fact["value"], 7654.0);
    let slot:duet_api::today::SlotRequest=serde_json::from_value(json!({"kind":"daily","window_start":start,"window_end":start+Duration::days(1),"timezone":"UTC","first_weekday":7,"snapshot_id":context["snapshot"]["id"]})).unwrap();
    let accepted = ok(&app, "POST", "/api/insight-cycles", "a", json!(slot)).await;
    let run: Uuid = serde_json::from_value(accepted["run_id"].clone()).unwrap();
    let mut tx = write_db.begin(a.account_id).await.unwrap();
    let assignment: Value =
        sqlx::query_scalar("select today_assignment from duet_commands where user_id=$1 and id=$2")
            .bind(a.account_id)
            .bind(run)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    tx.rollback().await.unwrap();
    let cycle = assignment["slots"][0]["cycle_id"].clone();
    let ready = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let status = ok(&app, "POST", "/api/insight-cycles/status", "a", json!(slot)).await;
            if status["state"] == "ready" {
                break status;
            }
            assert_ne!(status["state"], "failed", "assessment failed: {status}");
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("local HTTP assessment completed");
    assert_eq!(ready["state"], "ready");
    assert!(
        store.phone_page(&a, true).await.unwrap()["messages"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let insight = ready["cycle"]["insights"][0]["id"].clone();
    let selection = json!({"kind":"published_insight","cycle_id":cycle,"insight_id":insight});
    let selected = duet_api::conversation::Submission {
        timezone: "UTC".into(),
        selected_context: Some(serde_json::from_value(selection.clone()).unwrap()),
        reply_to_message_id: None,
    };
    assert!(
        store
            .phone_accept_submission(&b, Uuid::new_v4(), "Foreign", "local", &selected)
            .await
            .is_err()
    );
    let command = Uuid::new_v4();
    store
        .phone_accept_submission(&a, command, "Explain this observation", "local", &selected)
        .await
        .unwrap();
    execute(
        &store,
        &a,
        command,
        vec![
            vec![call("selected", "read_selected_source", json!({}))],
            vec![call(
                "cite",
                "cite_health_sources",
                json!({"read_ids":["selected"]}),
            )],
        ],
    )
    .await;
    let assistant = Uuid::from_u128(command.as_u128() ^ (1u128 << 127));
    let reopened = store
        .message_source(&a, assistant, "selected")
        .await
        .unwrap();
    assert!(reopened.to_string().contains("7654"));
    assert!(
        store
            .message_source(&b, assistant, "selected")
            .await
            .is_err()
    );
    ok(
        &app,
        "POST",
        &format!(
            "/api/insight-cycles/{}/insights/{}/correction",
            cycle.as_str().unwrap(),
            insight.as_str().unwrap()
        ),
        "a",
        json!({}),
    )
    .await;
    assert!(
        store
            .phone_accept_submission(&a, Uuid::new_v4(), "Corrected", "local", &selected)
            .await
            .is_err()
    );
    let measurement: duet_api::conversation::Submission = serde_json::from_value(json!({"timezone":"UTC","selected_context":{"kind":"measurement","source_boundary_id":context["snapshot"]["source_boundary_id"],"health_context_revision":context["revision"],"source":{"kind":"appleHealth","connection_id":"phone"},"measurement_id":"steps","period_start":fact["range_start"],"period_end":fact["range_end"],"fact":fact}})).unwrap();
    assert!(
        store
            .phone_accept_submission(
                &b,
                Uuid::new_v4(),
                "Foreign measurement",
                "local",
                &measurement
            )
            .await
            .is_err()
    );
    let measurement_command = Uuid::new_v4();
    store
        .phone_accept_submission(
            &a,
            measurement_command,
            "Explain my measurement",
            "local",
            &measurement,
        )
        .await
        .unwrap();
    execute(
        &store,
        &a,
        measurement_command,
        vec![
            vec![call("measurement-read", "read_selected_source", json!({}))],
            vec![call(
                "measurement-cite",
                "cite_health_sources",
                json!({"read_ids":["measurement-read"]}),
            )],
        ],
    )
    .await;
    let measurement_assistant = Uuid::from_u128(measurement_command.as_u128() ^ (1u128 << 127));
    assert!(
        store
            .message_source(&a, measurement_assistant, "measurement-read")
            .await
            .unwrap()
            .to_string()
            .contains("7654")
    );
    // Actual accepted conversation tool creates follow-through; API cannot import it.
    let command = Uuid::new_v4();
    let follow = Uuid::new_v4();
    store
        .phone_accept(
            &a,
            command,
            "Please remind me tomorrow to bring my questions.",
            "local",
        )
        .await
        .unwrap();
    execute(&store,&a,command,vec![vec![call("create","create_follow_up",json!({"id":follow,"operation":"reminder","accepted_instruction":"Bring my questions","schedule":{"kind":"once","timezone":"UTC","local_date":(start+Duration::days(1)).date_naive(),"local_time":"12:00:00"},"source_message_ids":[command],"display_title":"Bring questions"}))]]).await;
    let followup = ok(
        &app,
        "GET",
        &format!("/api/follow-ups/{follow}"),
        "a",
        json!(null),
    )
    .await;
    assert_eq!(followup["state"], "active");
    assert_eq!(
        http(
            &app,
            "GET",
            &format!("/api/follow-ups/{follow}"),
            "b",
            json!(null)
        )
        .await
        .0,
        404
    );
    let selected_follow = duet_api::conversation::Submission {
        timezone: "UTC".into(),
        selected_context: Some(
            serde_json::from_value(
                json!({"kind":"follow_up","id":follow,"revision":followup["revision"]}),
            )
            .unwrap(),
        ),
        reply_to_message_id: None,
    };
    let follow_command = Uuid::new_v4();
    let accepted_follow = store
        .phone_accept_submission(
            &a,
            follow_command,
            "What did I ask for?",
            "local",
            &selected_follow,
        )
        .await
        .unwrap();
    assert!(
        accepted_follow["message"]["sources"]
            .as_array()
            .unwrap()
            .is_empty(),
        "accepted work is not evidence"
    );
    execute(&store, &a, follow_command, vec![]).await;
    let event = Uuid::new_v4();
    let path = format!("/api/follow-ups/{follow}/cancel");
    let cancel = json!({"event_id":event,"expected_revision":followup["revision"]});
    let receipt = ok(&app, "POST", &path, "a", cancel.clone()).await;
    assert_eq!(ok(&app, "POST", &path, "a", cancel).await, receipt);
    assert_eq!(
        ok(
            &app,
            "GET",
            &format!("/api/follow-ups/{follow}"),
            "a",
            json!(null)
        )
        .await["state"],
        "cancelled"
    );
    assert!(
        store
            .phone_accept_submission(
                &a,
                Uuid::new_v4(),
                "Cancelled",
                "local",
                &duet_api::conversation::Submission {
                    timezone: "UTC".into(),
                    selected_context: Some(
                        serde_json::from_value(
                            json!({"kind":"follow_up","id":follow,"revision":followup["revision"]})
                        )
                        .unwrap()
                    ),
                    reply_to_message_id: None
                }
            )
            .await
            .is_err()
    );
    // A process-independent owner reads the same published source and receipt.
    let restored = duet_api::conversation::ConversationStore::new(
        AccountDatabase::connect(&executor_url).await.unwrap(),
    );
    assert!(
        restored
            .message_source(&a, assistant, "selected")
            .await
            .is_ok()
    );
    ok(&app,"POST","/api/auth/privacy-restrictions","a",json!({"event_id":Uuid::new_v4(),"operation":"device_withdraw","scope_id":a.account_id,"scope_version":0})).await;
    assert!(
        restored
            .message_source(&a, assistant, "selected")
            .await
            .is_err()
    );
    assert!(
        restored
            .message_source(&a, measurement_assistant, "measurement-read")
            .await
            .is_err()
    );
    assert!(producer.read(&a).await.is_err());
    let refreshed = ok(&app, "POST", "/api/health-context/refresh", "a", json!({})).await;
    assert!(
        refreshed["snapshot"]["projections"][0]["facts"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mut tx = db.begin(a.account_id).await.unwrap();
    assert!(
        sqlx::query("update follow_ups set state='active' where user_id=$1")
            .bind(a.account_id)
            .execute(&mut *tx)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    model_task.abort();
    drop(app);
    drop(restored);
    drop(store);
    drop(today);
    drop(producer);
    drop(durable);
    drop(db);
    drop(write_db);
    sqlx::query(&format!("drop database {name} with (force)"))
        .execute(&mut admin)
        .await
        .unwrap();
    for role in [reader, context_role, executor] {
        sqlx::query(&format!("drop role {role}"))
            .execute(&mut admin)
            .await
            .unwrap();
    }
    std::fs::remove_dir_all(journal_path).unwrap();
}
