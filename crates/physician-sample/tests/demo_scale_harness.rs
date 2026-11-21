//! Public demo end-to-end scale proof on Postgres + Valkey + ResponsesStub.
//!
//! Spawns two `physician-sample` instances and one `physician-worker` with
//! `PHYSICIAN_PUBLIC_DEMO=1`, `PHYSICIAN_AUTH=required`,
//! `PHYSICIAN_DURABLE_CHAT=1`, and `CODA_DEMO_SESSION_MAX_TURNS=1`.
//! Proves:
//! - Isolated zero-sign-in visitor sessions via `POST /auth/demo-session` (`404`
//!   when `public-demo` is disabled)
//! - Multi-instance Valkey SSE fan-out (`valkey.subscribers() >= 3`,
//!   `event: workspace_changed`, and `event: resync` on Valkey restart)
//! - Multi-round `crates/harness` tool steps (`SearchedChart`, `ReadRecords`,
//!   `SavedMemoryNote`), `CitationTargetKind::Passage` with verbatim quote +
//!   `basis`, and `GET .../sources/{source_key}` readback
//! - Visitor isolation (Visitor 2 gets a separate `demo-*` identity and cannot
//!   see Visitor 1's chat)
//! - `DemoTurnBudget` `429` (`demo_turn_budget_exhausted` + `Retry-After`)
//!   enforcement before Postgres work acceptance or another `ResponsesStub` call

use std::path::PathBuf;
use std::time::Duration;

use physician_wire::{
    AnswerState, ChatSnapshotResponse, CitationTargetKind, CitedSourceResponse, ExchangeSteps,
    MessageSegment, Step,
};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn public_demo_scale_valkey_harness_citations_and_budget() {
    let Some(database) = support::Database::create("demo_scale_harness").await else {
        return;
    };

    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .expect("auth store");
    let runtime = local_product::runtime::Store::connect(&database.url)
        .await
        .expect("runtime store");
    physician_sample::demo_session::seed_demo_practice(&auth, &runtime, unix_now())
        .await
        .expect("seed public demo practice");
    drop(auth);

    let workspace = physician_sample::demo_session::demo_workspaces()
        .into_iter()
        .next()
        .expect("demo workspace");
    let quote = "Potassium 5.2 mmol/L";
    let content = "Potassium 5.2 mmol/L on synthetic BMP (reference 3.5-5.1 mmol/L).".to_owned();
    let revision = local_product::sha256_hex(content.as_bytes());
    runtime
        .record_source(&local_product::physician_sources::SourceRevision {
            workspace: workspace.clone(),
            source_id: physician_sample::synthetic_chart::SOURCE_ID.to_owned(),
            candidate_id: "Observation/demo-bmp-1".to_owned(),
            revision: revision.clone(),
            record_type: "Observation".to_owned(),
            dates: vec![],
            sha256: revision.clone(),
            content: content.clone(),
            readable_parts: vec![],
            provenance: json!({
                "native_record": "Observation/demo-bmp-1",
                "title": "Basic Metabolic Panel",
                "recorded_at": "2026-09-18T10:00:00Z",
                "record_status": "final"
            }),
            envelope: None,
        })
        .await
        .expect("record synthetic demo observation");

    let home = Home(std::env::temp_dir().join(format!(
        "physician-demo-scale-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    )));
    std::fs::create_dir_all(&home.0).expect("home");
    let mut valkey = support::Valkey::start().await;
    let redis = valkey.url();

    let rounds = json!([
        [
            {
                "name": "search_patient_information",
                "arguments": {
                    "query": "Potassium",
                    "record_type": "Observation",
                    "after": null
                }
            }
        ],
        [
            {
                "name": "read_patient_sources",
                "arguments": {
                    "references": [
                        {
                            "candidate_id": "Observation/demo-bmp-1",
                            "revision": revision,
                            "cursor": null
                        }
                    ],
                    "max_result_bytes": 4096
                }
            }
        ],
        [
            {
                "name": "add_patient_memory_note",
                "arguments": {
                    "text": "Latest synthetic BMP shows potassium 5.2 mmol/L.",
                    "quote": quote
                }
            }
        ]
    ])
    .to_string();

    let finish_reply = json!({
        "answer": "Recent synthetic metabolic panel demonstrates serum potassium 5.2 mmol/L [S1].",
        "quotes": [{ "label": "S1", "quote": quote }],
        "descriptions": [],
        "basis": "Verified synthetic BMP lab observation [S1].",
        "cites_none": false
    })
    .to_string();

    let stub =
        support::ResponsesStub::start_with_reply(&home.0, 0, 0, &rounds, &finish_reply).await;
    let worker_port = support::free_port();
    let site_a_port = support::free_port();
    let site_b_port = support::free_port();
    let control_port = support::free_port();

    let worker_listen = format!("127.0.0.1:{worker_port}");
    let site_a_listen = format!("127.0.0.1:{site_a_port}");
    let site_b_listen = format!("127.0.0.1:{site_b_port}");
    let control_listen = format!("127.0.0.1:{control_port}");

    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
            ("PHYSICIAN_PUBLIC_DEMO", "1"),
        ],
        &home.0,
    );
    support::wait_ready(&mut worker, worker_port).await;

    let site_common = [
        ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
        ("PHYSICIAN_REDIS_URL", redis.as_str()),
        support::DURABLE,
        ("PHYSICIAN_AUTH", "required"),
        ("PHYSICIAN_PUBLIC_DEMO", "1"),
        ("CODA_DEMO_SESSION_MAX_TURNS", "1"),
    ];
    let mut site_a = support::Process::spawn(
        "site-a",
        support::site_bin(),
        &[],
        &[
            site_common[0],
            site_common[1],
            site_common[2],
            site_common[3],
            site_common[4],
            site_common[5],
            ("PHYSICIAN_LISTEN_ADDR", site_a_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site_a, site_a_port).await;

    let mut site_b = support::Process::spawn(
        "site-b",
        support::site_bin(),
        &[],
        &[
            site_common[0],
            site_common[1],
            site_common[2],
            site_common[3],
            site_common[4],
            site_common[5],
            ("PHYSICIAN_LISTEN_ADDR", site_b_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site_b, site_b_port).await;

    // A default PHYSICIAN_AUTH=required instance without public-demo must return 404.
    let mut non_demo_site = support::Process::spawn(
        "site-non-demo",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            support::DURABLE,
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", control_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut non_demo_site, control_port).await;

    let client = reqwest::Client::new();
    let control_status = client
        .post(format!("http://{control_listen}/auth/demo-session"))
        .send()
        .await
        .expect("non-demo /auth/demo-session request")
        .status();
    assert_eq!(
        control_status,
        reqwest::StatusCode::NOT_FOUND,
        "POST /auth/demo-session must return 404 when public-demo is disabled"
    );
    drop(non_demo_site);

    // Wait until both sites and the worker subscribe to Valkey.
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    while valkey.subscribers() < 3 {
        assert!(
            std::time::Instant::now() < deadline,
            "expected >= 3 Valkey subscribers, saw {}\n{}\n{}\n{}",
            valkey.subscribers(),
            site_a.tail(),
            site_b.tail(),
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Visitor 1 obtains an ephemeral isolated demo session on site-a.
    let visitor1_res = client
        .post(format!("http://{site_a_listen}/auth/demo-session"))
        .send()
        .await
        .expect("visitor 1 demo-session");
    assert_eq!(visitor1_res.status(), reqwest::StatusCode::OK);
    let visitor1: Value = visitor1_res.json().await.expect("visitor 1 json");
    let session1 = visitor1["session"]
        .as_str()
        .expect("visitor 1 session token")
        .to_owned();
    let physician1 = visitor1["physician_id"]
        .as_str()
        .expect("visitor 1 physician_id")
        .to_owned();
    assert!(physician1.starts_with("demo-"), "{visitor1}");
    assert_eq!(
        visitor1["practice_id"],
        physician_sample::demo_session::DEMO_PRACTICE_ID
    );
    assert!(visitor1["ends_at"].as_u64().is_some());

    // Visitor 1 opens SSE stream on site-b and submits Turn 1 on site-a.
    let chat_id = format!("demo-chat-{}", uuid::Uuid::new_v4().simple());
    let mut sse = support::Sse::open(
        &site_b_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session1,
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", site_b.tail()));

    let submit_res = client
        .post(format!("http://{site_a_listen}/commands"))
        .bearer_auth(&session1)
        .json(&json!({
            "type": "message.submit",
            "idempotency_key": format!("demo-turn-1-{}", uuid::Uuid::new_v4().simple()),
            "payload": {
                "patient_workspace_id": workspace,
                "chat_id": chat_id,
                "text": "What is the latest potassium and please note it?",
                "physician_time_zone": "America/New_York"
            }
        }))
        .send()
        .await
        .expect("submit turn 1");
    assert_eq!(submit_res.status(), reqwest::StatusCode::OK);
    let submit_body: Value = submit_res.json().await.expect("submit json");
    assert_eq!(submit_body["outcome"], "accepted", "{submit_body}");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(25);
    let mut heard = 0usize;
    let snapshot_wire: ChatSnapshotResponse = loop {
        heard += 1;
        sse.wait_for_nth(
            "event: workspace_changed",
            heard,
            deadline.saturating_duration_since(tokio::time::Instant::now()),
        )
        .await
        .unwrap_or_else(|error| {
            panic!(
                "{error}\n{}\n{}\n{}",
                site_a.tail(),
                site_b.tail(),
                worker.tail()
            )
        });

        let snap_res = client
            .get(format!(
                "http://{site_b_listen}/patient-workspaces/{workspace}/chats/{chat_id}/snapshot"
            ))
            .bearer_auth(&session1)
            .send()
            .await
            .expect("site-b snapshot");
        assert_eq!(snap_res.status(), reqwest::StatusCode::OK);
        let parsed: ChatSnapshotResponse = snap_res.json().await.expect("snapshot wire");
        if parsed
            .snapshot
            .transcript
            .iter()
            .any(|ex| matches!(ex.answer, AnswerState::Finished { .. }))
        {
            break parsed;
        }
    };

    assert_eq!(snapshot_wire.snapshot.transcript.len(), 1);
    let exchange = &snapshot_wire.snapshot.transcript[0];

    let ExchangeSteps::Read(steps) = &exchange.steps else {
        panic!("expected ExchangeSteps::Read, got {:?}", exchange.steps);
    };
    assert!(
        steps
            .iter()
            .any(|step| matches!(step, Step::SearchedChart { .. })),
        "missing SearchedChart step in {steps:?}"
    );
    assert!(
        steps
            .iter()
            .any(|step| matches!(step, Step::ReadRecords { .. })),
        "missing ReadRecords step in {steps:?}"
    );
    assert!(
        steps
            .iter()
            .any(|step| matches!(step, Step::SavedMemoryNote { .. })),
        "missing SavedMemoryNote step in {steps:?}"
    );

    let AnswerState::Finished {
        segments, basis, ..
    } = &exchange.answer
    else {
        panic!("expected AnswerState::Finished, got {:?}", exchange.answer);
    };
    let basis_segments = basis.as_ref().expect("finished answer must include basis");
    assert!(
        basis_segments.iter().any(|seg| matches!(
            seg,
            MessageSegment::Citation { quote: q, .. } if !q.is_empty()
        )),
        "basis missing cited source: {basis_segments:?}"
    );

    let cited = segments
        .iter()
        .find_map(|seg| match seg {
            MessageSegment::Citation {
                source_key,
                quote: seg_quote,
                target,
                ..
            } => Some((source_key.clone(), seg_quote.clone(), *target)),
            MessageSegment::Text { .. } => None,
        })
        .expect("finished answer must contain citation segment");
    assert_eq!(cited.2, CitationTargetKind::Passage);
    assert_eq!(cited.1, quote);

    let encoded_source_key = percent_encode_segment(&cited.0);
    let source_res = client
        .get(format!(
            "http://{site_b_listen}/patient-workspaces/{workspace}/chats/{chat_id}/sources/{encoded_source_key}"
        ))
        .bearer_auth(&session1)
        .send()
        .await
        .expect("get cited source");
    assert_eq!(source_res.status(), reqwest::StatusCode::OK);
    let cited_source: CitedSourceResponse = source_res.json().await.expect("cited source wire");
    assert!(
        cited_source.source.body.contains(quote),
        "cited source body `{}` does not contain `{quote}`",
        cited_source.source.body
    );

    // Visitor 2 gets a distinct isolated demo physician and cannot see Visitor 1's chat.
    let visitor2_res = client
        .post(format!("http://{site_b_listen}/auth/demo-session"))
        .send()
        .await
        .expect("visitor 2 demo-session");
    assert_eq!(visitor2_res.status(), reqwest::StatusCode::OK);
    let visitor2: Value = visitor2_res.json().await.expect("visitor 2 json");
    let session2 = visitor2["session"]
        .as_str()
        .expect("visitor 2 session")
        .to_owned();
    let physician2 = visitor2["physician_id"]
        .as_str()
        .expect("visitor 2 physician_id")
        .to_owned();
    assert!(physician2.starts_with("demo-"));
    assert_ne!(physician1, physician2);

    let visitor2_ws_res = client
        .get(format!(
            "http://{site_b_listen}/patient-workspaces/{workspace}"
        ))
        .bearer_auth(&session2)
        .send()
        .await
        .expect("visitor 2 workspace");
    assert_eq!(visitor2_ws_res.status(), reqwest::StatusCode::OK);
    let visitor2_ws: Value = visitor2_ws_res
        .json()
        .await
        .expect("visitor 2 workspace json");
    assert_eq!(
        visitor2_ws["chats"].as_array().map(Vec::len),
        Some(0),
        "visitor 2 workspace must not list visitor 1 chats: {visitor2_ws}"
    );

    let visitor2_snap: ChatSnapshotResponse = client
        .get(format!(
            "http://{site_b_listen}/patient-workspaces/{workspace}/chats/{chat_id}/snapshot"
        ))
        .bearer_auth(&session2)
        .send()
        .await
        .expect("visitor 2 snapshot")
        .json()
        .await
        .expect("visitor 2 snapshot json");
    assert!(
        visitor2_snap.snapshot.transcript.is_empty(),
        "visitor 2 must not see visitor 1's transcript"
    );

    // Restarting Valkey forces `event: resync` on site-b's SSE stream.
    valkey.restart().await;
    sse.wait_for("event: resync", Duration::from_secs(20))
        .await
        .unwrap_or_else(|error| panic!("{error}\n{}", valkey.tail()));
    assert!(
        sse.buffer.contains("event: resync\ndata: {}"),
        "resync frame missing: {}",
        sse.buffer
    );

    // CODA_DEMO_SESSION_MAX_TURNS=1 rejects Visitor 1's second turn with 429
    // before writing Postgres work or invoking ResponsesStub again.
    let stub_requests_before = stub.requests().len();
    let over_budget_res = client
        .post(format!("http://{site_a_listen}/commands"))
        .bearer_auth(&session1)
        .json(&json!({
            "type": "message.submit",
            "idempotency_key": format!("demo-turn-2-{}", uuid::Uuid::new_v4().simple()),
            "payload": {
                "patient_workspace_id": workspace,
                "chat_id": chat_id,
                "text": "Follow-up turn that exceeds the session cap.",
                "physician_time_zone": "America/New_York"
            }
        }))
        .send()
        .await
        .expect("submit over-budget turn 2");
    assert_eq!(
        over_budget_res.status(),
        reqwest::StatusCode::TOO_MANY_REQUESTS
    );
    let retry_after = over_budget_res
        .headers()
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok());
    assert!(
        retry_after.is_some_and(|secs| secs >= 1),
        "expected positive Retry-After header"
    );
    let err_body: Value = over_budget_res.json().await.expect("429 json");
    assert_eq!(err_body["code"], "demo_turn_budget_exhausted", "{err_body}");
    assert!(
        err_body["retry_after_seconds"]
            .as_u64()
            .is_some_and(|s| s >= 1)
    );

    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(
        stub.requests().len(),
        stub_requests_before,
        "over-budget turn must not invoke ResponsesStub again"
    );

    // Exhausted demo session is also blocked from follow-up review/resume bypasses (429).
    for path in [
        format!("/patient-workspaces/{workspace}/follow-ups/fu-demo-1/review-now"),
        format!("/patient-workspaces/{workspace}/follow-ups/fu-demo-1/resume"),
    ] {
        let res = client
            .post(format!("http://{site_a_listen}{path}"))
            .bearer_auth(&session1)
            .json(&json!({
                "idempotency_key": format!("demo-fu-{}", uuid::Uuid::new_v4().simple()),
            }))
            .send()
            .await
            .expect("follow-up budget request");
        assert_eq!(
            res.status(),
            reqwest::StatusCode::TOO_MANY_REQUESTS,
            "{path}"
        );
        let body: Value = res.json().await.expect("follow-up 429 json");
        assert_eq!(body["code"], "demo_turn_budget_exhausted", "{path}: {body}");
    }

    // Demo session is forbidden (403) from inviting patients/colleagues or mutating
    // practice-wide source connections, while read-only GET /connections succeeds.
    for (path, payload) in [
        (
            "/auth/invitations",
            json!({
                "patient_workspace_id": workspace,
                "email": "abuse@example.test",
                "display_name": "Abuse Target"
            }),
        ),
        (
            "/auth/invitations/resend",
            json!({ "patient_workspace_id": workspace }),
        ),
        (
            "/auth/invitations/cancel",
            json!({ "patient_workspace_id": workspace }),
        ),
        (
            "/auth/colleagues/invitations",
            json!({
                "email": "colleague-abuse@example.test",
                "display_name": "Abuse Colleague"
            }),
        ),
    ] {
        let res = client
            .post(format!("http://{site_a_listen}{path}"))
            .bearer_auth(&session1)
            .json(&payload)
            .send()
            .await
            .expect("invite mutation request");
        assert_eq!(res.status(), reqwest::StatusCode::FORBIDDEN, "{path}");
    }

    let get_connections = client
        .get(format!("http://{site_a_listen}/connections"))
        .bearer_auth(&session1)
        .send()
        .await
        .expect("get connections");
    assert_eq!(get_connections.status(), reqwest::StatusCode::OK);

    for (path, payload) in [
        (
            "/connections/spruce/connect",
            json!({ "method": "simulated_practice" }),
        ),
        ("/connections/spruce/disconnect", json!({})),
        ("/connections/spruce/simulate", json!({ "event": "outage" })),
    ] {
        let res = client
            .post(format!("http://{site_a_listen}{path}"))
            .bearer_auth(&session1)
            .json(&payload)
            .send()
            .await
            .expect("source setup mutation request");
        assert_eq!(res.status(), reqwest::StatusCode::FORBIDDEN, "{path}");
    }
}

fn percent_encode_segment(raw: &str) -> String {
    let mut encoded = String::with_capacity(raw.len());
    for byte in raw.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            use std::fmt::Write as _;
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}
