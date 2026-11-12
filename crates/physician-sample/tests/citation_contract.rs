//! End-to-end citation contract proof across chat and background reply revisions.
//!
//! Runs the real `physician-worker`, `physician-sample`, Pub/Sub emulator, and
//! disposable PostgreSQL databases against a scripted Responses API server so
//! every tool call, refusal message, saved row, and cited-source HTTP reopen is
//! exercised over the production wire path.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use base64::Engine as _;
use local_product::citations::{PUBLISH_CITATIONS_TOOL, ReadRecordPreview};
use local_product::physician_sources::{
    NativeLink, READ_TOOL, ReadablePart, RecordDate, SourceRevision, workspace_id,
};
use local_product::runtime::reply::{DraftStatus, FINISH_TOOL};
use local_product::runtime::{Conversation, Scope, Store, WorkStatus};
use physician_auth::PostgresAuthStore;
use physician_sample::chart_load::load_synthetic_practice;
use physician_sample::synthetic_chart;
use physician_wire::{CitationLocator, CitedPart, SourceEvidence, SourcePath};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

type StepFn = Box<dyn FnOnce(&Value) -> Vec<Value> + Send>;

struct ScriptedResponses {
    port: u16,
    requests: Arc<Mutex<Vec<Value>>>,
    task: tokio::task::JoinHandle<()>,
}

#[derive(Clone)]
struct ScriptState {
    steps: Arc<Mutex<VecDeque<StepFn>>>,
    requests: Arc<Mutex<Vec<Value>>>,
}

impl ScriptedResponses {
    async fn start(steps: Vec<StepFn>) -> Self {
        async fn readyz() -> StatusCode {
            StatusCode::OK
        }

        async fn responses(State(state): State<ScriptState>, body: Bytes) -> impl IntoResponse {
            let payload: Value = serde_json::from_slice(&body).expect("JSON request body");
            state
                .requests
                .lock()
                .expect("requests")
                .push(payload.clone());
            let step = state
                .steps
                .lock()
                .expect("steps")
                .pop_front()
                .unwrap_or_else(|| {
                    panic!(
                        "unexpected extra provider call with input: {}",
                        serde_json::to_string_pretty(&payload["input"]).unwrap_or_default()
                    )
                });
            let output = step(&payload);
            let response = json!({
                "id": format!("resp_{}", uuid::Uuid::new_v4().simple()),
                "status": "completed",
                "model": payload["model"],
                "output": output,
            });
            let mut headers = HeaderMap::new();
            if payload["stream"].as_bool() == Some(true) {
                headers.insert(
                    header::CONTENT_TYPE,
                    "text/event-stream".parse().expect("header"),
                );
                (StatusCode::OK, headers, encode_sse(&response))
            } else {
                headers.insert(
                    header::CONTENT_TYPE,
                    "application/json".parse().expect("header"),
                );
                (
                    StatusCode::OK,
                    headers,
                    serde_json::to_vec(&response).expect("response bytes"),
                )
            }
        }

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind scripted responses");
        let port = listener.local_addr().expect("local addr").port();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let state = ScriptState {
            steps: Arc::new(Mutex::new(steps.into_iter().collect())),
            requests: requests.clone(),
        };
        let app = Router::new()
            .route("/readyz", get(readyz))
            .route("/responses", post(responses))
            .with_state(state);
        let task = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        Self {
            port,
            requests,
            task,
        }
    }

    fn requests(&self) -> Vec<Value> {
        self.requests.lock().expect("requests").clone()
    }
}

impl Drop for ScriptedResponses {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn encode_sse(response: &Value) -> Vec<u8> {
    let mut events = vec![json!({
        "type": "response.created",
        "response": { "id": response["id"] }
    })];
    for (index, item) in response["output"]
        .as_array()
        .expect("output array")
        .iter()
        .enumerate()
    {
        events.push(json!({
            "type": "response.output_item.added",
            "output_index": index,
            "item": item,
        }));
        match item["type"].as_str() {
            Some("function_call") => {
                events.push(json!({
                    "type": "response.function_call_arguments.delta",
                    "output_index": index,
                    "item_id": item["id"],
                    "delta": item["arguments"],
                }));
                events.push(json!({
                    "type": "response.function_call_arguments.done",
                    "output_index": index,
                    "item_id": item["id"],
                    "arguments": item["arguments"],
                }));
            }
            Some("message") => {
                for (part, content) in item["content"]
                    .as_array()
                    .expect("message content")
                    .iter()
                    .enumerate()
                {
                    events.push(json!({
                        "type": "response.content_part.added",
                        "output_index": index,
                        "item_id": item["id"],
                        "content_index": part,
                        "part": { "type": "output_text", "text": "" },
                    }));
                    events.push(json!({
                        "type": "response.output_text.delta",
                        "output_index": index,
                        "item_id": item["id"],
                        "content_index": part,
                        "delta": content["text"],
                    }));
                    events.push(json!({
                        "type": "response.output_text.done",
                        "output_index": index,
                        "item_id": item["id"],
                        "content_index": part,
                        "text": content["text"],
                    }));
                    events.push(json!({
                        "type": "response.content_part.done",
                        "output_index": index,
                        "item_id": item["id"],
                        "content_index": part,
                        "part": content,
                    }));
                }
            }
            other => panic!("unexpected output item type {other:?}"),
        }
        events.push(json!({
            "type": "response.output_item.done",
            "output_index": index,
            "item": item,
        }));
    }
    events.push(json!({
        "type": "response.completed",
        "response": response,
    }));
    let mut wire = String::new();
    for (seq, mut event) in events.into_iter().enumerate() {
        event["sequence_number"] = json!(seq);
        wire.push_str(&format!(
            "data: {}\n\n",
            serde_json::to_string(&event).expect("sse")
        ));
    }
    wire.into_bytes()
}

fn fn_call(call_id: &str, name: &str, arguments: Value) -> Value {
    json!({
        "id": format!("fc_{}", uuid::Uuid::new_v4().simple()),
        "type": "function_call",
        "status": "completed",
        "call_id": call_id,
        "name": name,
        "arguments": serde_json::to_string(&arguments).expect("arguments"),
    })
}

fn assistant_msg(text: &str) -> Value {
    json!({
        "id": format!("msg_{}", uuid::Uuid::new_v4().simple()),
        "type": "message",
        "role": "assistant",
        "status": "completed",
        "content": [{ "type": "output_text", "text": text }],
    })
}

fn tool_output(request: &Value, call_id: &str) -> Value {
    let raw = request["input"]
        .as_array()
        .expect("input array")
        .iter()
        .rev()
        .find(|item| {
            item["type"] == "function_call_output" && item["call_id"].as_str() == Some(call_id)
        })
        .unwrap_or_else(|| {
            panic!(
                "missing function_call_output for `{call_id}` in {}",
                serde_json::to_string_pretty(&request["input"]).unwrap_or_default()
            )
        })["output"]
        .as_str()
        .expect("output string");
    serde_json::from_str(raw).unwrap_or_else(|_| json!({ "raw": raw }))
}

fn developer_texts(request: &Value) -> Vec<String> {
    request["input"]
        .as_array()
        .expect("input array")
        .iter()
        .filter(|item| item["role"].as_str() == Some("developer"))
        .filter_map(|item| item["content"].as_str().map(str::to_owned))
        .collect()
}

struct HarnessStack {
    provider: ScriptedResponses,
    database: support::Database,
    store: Store,
    _emulator: support::Emulator,
    worker: support::Process,
    site: support::Process,
    site_host: String,
    home: PathBuf,
}

impl Drop for HarnessStack {
    fn drop(&mut self) {
        self.site.kill_now();
        self.worker.kill_now();
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

impl HarnessStack {
    async fn start(label: &str, steps: Vec<StepFn>) -> Option<Self> {
        let database = support::Database::create(label).await?;
        let store = Store::connect(&database.url).await.expect("runtime store");
        let provider = ScriptedResponses::start(steps).await;
        let worker_port = support::free_port();
        let site_port = support::free_port();
        let emulator = support::Emulator::start().await;
        emulator.provision(worker_port).await;
        let home = std::env::temp_dir().join(format!(
            "physician-citation-contract-{label}-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&home).expect("home");
        let worker_listen = format!("127.0.0.1:{worker_port}");
        let site_listen = format!("127.0.0.1:{site_port}");
        let stub_port = provider.port.to_string();
        let topic = support::topic();
        let mut worker = support::Process::spawn(
            "worker",
            support::worker_bin(),
            &[],
            &[
                ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
                ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
                ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
                ("PHYSICIAN_RESPONSES_STUB_PORT", stub_port.as_str()),
            ],
            &home,
        );
        support::wait_ready(&mut worker, worker_port).await;
        let mut site = support::Process::spawn(
            "site",
            support::site_bin(),
            &[],
            &[
                ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
                ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
                ("GOOGLE_CLOUD_PROJECT", support::PROJECT),
                ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
                ("PHYSICIAN_AUTH", "required"),
                ("PHYSICIAN_LISTEN_ADDR", site_listen.as_str()),
            ],
            &home,
        );
        support::wait_ready(&mut site, site_port).await;
        Some(Self {
            provider,
            database,
            store,
            _emulator: emulator,
            worker,
            site,
            site_host: site_listen,
            home,
        })
    }

    async fn wait_chat_completed(&self, scope: &Scope) -> Conversation {
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Some(state) = self.store.read(scope).await.expect("read conversation") {
                if state.status == WorkStatus::Completed {
                    return state;
                }
                assert_ne!(
                    state.status,
                    WorkStatus::Failed,
                    "chat failed:\n{}\n{}",
                    self.worker.tail(),
                    self.site.tail()
                );
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for chat completion:\n{}\n{}",
                self.worker.tail(),
                self.site.tail()
            );
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }
}

async fn enroll_test_physician_and_workspace(url: &str, site_host: &str) -> (Scope, String) {
    support::staging_account(url).await;
    let session = support::sign_in(site_host).await;
    let scope = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: physician_sample::WORKSPACE_IDS[0].into(),
        chat: format!("chat-{}", uuid::Uuid::new_v4().simple()),
    };
    (scope, session)
}

#[tokio::test]
async fn synthetic_source_run_saves_two_citations_with_versions_dated_previews_and_live_link() {
    let _guard = lock().lock().await;
    let steps: Vec<StepFn> = vec![
        Box::new(|_req| {
            vec![fn_call(
                "search-all",
                "search_patient_information",
                json!({ "query": "" }),
            )]
        }),
        Box::new(|req| {
            let discovered = tool_output(req, "search-all");
            let hits = discovered["results"].as_array().expect("search hits");
            let find_ref = |candidate_id: &str| {
                hits.iter()
                    .find(|hit| hit["reference"]["candidate_id"].as_str() == Some(candidate_id))
                    .unwrap_or_else(|| {
                        panic!("missing search hit for {candidate_id}: {discovered}")
                    })["reference"]
                    .clone()
            };
            vec![fn_call(
                "read-two",
                READ_TOOL,
                json!({
                    "references": [
                        find_ref("DocumentReference/note-cardio-1"),
                        find_ref("Observation/obs-k-1")
                    ],
                    "max_result_bytes": 65536
                }),
            )]
        }),
        Box::new(|req| {
            let receipt = tool_output(req, "read-two");
            let results = receipt["results"].as_array().expect("read results");
            assert_eq!(results.len(), 2);
            let note_rev = results[0]["revision"]
                .as_str()
                .expect("note rev")
                .to_owned();
            let note_cid = results[0]["citation_id"]
                .as_str()
                .expect("note cid")
                .to_owned();
            let obs_rev = results[1]["revision"].as_str().expect("obs rev").to_owned();
            let obs_cid = results[1]["citation_id"]
                .as_str()
                .expect("obs cid")
                .to_owned();
            vec![fn_call(
                "publish-two",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [
                        {
                            "reference": "DocumentReference/note-cardio-1",
                            "revision": note_rev,
                            "preview": "2026-08-12 clinic note: stable on lisinopril 10 mg daily."
                        },
                        {
                            "reference": "Observation/obs-k-1",
                            "revision": obs_rev,
                            "preview": "2026-08-14 serum potassium 4.3 mmol/L within reference range."
                        }
                    ],
                    "citations": [
                        {
                            "id": note_cid,
                            "reference": "DocumentReference/note-cardio-1",
                            "revision": note_rev,
                            "locator": null
                        },
                        {
                            "id": obs_cid,
                            "reference": "Observation/obs-k-1",
                            "revision": obs_rev,
                            "locator": null
                        }
                    ]
                }),
            )]
        }),
        Box::new(|req| {
            let published = tool_output(req, "publish-two");
            assert_eq!(
                published["kind"], "published_citations_receipt",
                "{published}"
            );
            let citations = published["citations"]
                .as_array()
                .expect("citation receipts");
            let c0 = citations[0]["id"].as_str().expect("c0");
            let c1 = citations[1]["id"].as_str().expect("c1");
            vec![assistant_msg(&format!(
                "Blood pressure remains controlled on lisinopril [[patient_source:{c0}]], and serum potassium is 4.3 mmol/L [[patient_source:{c1}]].\n\n[[basis]]\nGrounded in the August 2026 cardiology note and metabolic panel.\n[[/basis]]"
            ))]
        }),
    ];

    let Some(stack) = HarnessStack::start("ac1_synthetic", steps).await else {
        return;
    };

    // Build a synthetic chart bank on disk and load it through the SyntheticCharts adapter.
    let bank = stack.home.join("synthetic-bank");
    let chart = bank.join("patients/MRN101");
    std::fs::create_dir_all(&chart).expect("chart dir");
    let patient_ndjson = r#"{"resourceType":"Patient","id":"MRN101","meta":{"lastUpdated":"2026-08-27T00:00:00+00:00"},"identifier":[{"system":"urn:mrn","value":"101"}],"name":[{"family":"Rivera","given":["Elena"]}],"gender":"female","birthDate":"1968-05-14"}"#;
    let note_text = base64::engine::general_purpose::STANDARD
        .encode("Cardiology follow-up: Blood pressure stable on lisinopril 10 mg daily.");
    let note_ndjson = format!(
        r#"{{"resourceType":"DocumentReference","id":"note-cardio-1","date":"2026-08-12T12:05:00+00:00","content":[{{"attachment":{{"contentType":"text/plain; charset=utf-8","data":"{note_text}"}}}}]}}"#
    );
    std::fs::write(chart.join("Patient.ndjson"), format!("{patient_ndjson}\n")).expect("patient");
    std::fs::write(
        chart.join("DocumentReference.ndjson"),
        format!("{note_ndjson}\n"),
    )
    .expect("note");

    let auth = PostgresAuthStore::connect_without_migrations(&stack.database.url)
        .await
        .expect("auth store");
    auth.ensure_account(
        physician_sample::STAGING_PRACTICE_ID,
        "staging-physician",
        "physician@codahealth.test",
        "Staging Physician",
        true,
        support::unix_now(),
    )
    .await
    .expect("staging physician");
    let summary = load_synthetic_practice(&bank, &auth, &stack.store)
        .await
        .expect("load synthetic practice");
    assert_eq!(summary.patients, 1);
    assert!(summary.failed.is_empty(), "{:?}", summary.failed);

    let ws = workspace_id(synthetic_chart::SOURCE_ID, "MRN101");
    // Add a second record on the same synthetic patient workspace with an adapter-supplied live link.
    let obs_content = r#"{"resourceType":"Observation","id":"obs-k-1","effectiveDateTime":"2026-08-14T09:30:00+00:00","code":{"text":"Potassium"},"valueQuantity":{"value":4.3,"unit":"mmol/L"}}"#;
    let obs_rev = local_product::sha256_hex(obs_content.as_bytes());
    let obs_source = SourceRevision {
        workspace: ws.clone(),
        source_id: synthetic_chart::SOURCE_ID.into(),
        candidate_id: "Observation/obs-k-1".into(),
        revision: obs_rev.clone(),
        record_type: "Observation".into(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".into(),
            value: "2026-08-14T09:30:00+00:00".into(),
        }],
        content: obs_content.into(),
        sha256: obs_rev.clone(),
        readable_parts: vec![],
        provenance: json!({"native_record": "Observation/obs-k-1"}),
        native: Some(NativeLink {
            label: "Open in Synthetic EHR".into(),
            uri: "https://ehr.synthetic.test/Observation/obs-k-1".into(),
        }),
    };
    stack
        .store
        .record_source(&obs_source)
        .await
        .expect("record linked observation");

    let session = support::sign_in(&stack.site_host).await;
    let chat_id = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let (status, accepted) = support::post(
        &stack.site_host,
        "/commands",
        Some(&session),
        &support::submit(&ws, &chat_id, "Summarize blood pressure and potassium."),
    )
    .await;
    assert_eq!(status, 200, "{accepted}");

    let scope = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: ws.clone(),
        chat: chat_id.clone(),
    };
    let conversation = stack.wait_chat_completed(&scope).await;
    let answer = conversation.messages.last().expect("assistant answer");
    assert_eq!(answer.citations.len(), 2);
    assert_eq!(answer.source_previews.len(), 2);

    let expected_note_rev = local_product::sha256_hex(note_ndjson.as_bytes());
    assert_eq!(
        answer.citations[0].reference,
        "DocumentReference/note-cardio-1"
    );
    assert_eq!(answer.citations[0].revision, expected_note_rev);
    assert_eq!(
        answer.citations[0].summary.as_deref(),
        Some("2026-08-12 clinic note: stable on lisinopril 10 mg daily.")
    );
    assert_eq!(answer.citations[0].native, None);

    assert_eq!(answer.citations[1].reference, "Observation/obs-k-1");
    assert_eq!(answer.citations[1].revision, obs_rev);
    assert_eq!(
        answer.citations[1].summary.as_deref(),
        Some("2026-08-14 serum potassium 4.3 mmol/L within reference range.")
    );
    assert_eq!(
        answer.citations[1].native,
        Some(NativeLink {
            label: "Open in Synthetic EHR".into(),
            uri: "https://ehr.synthetic.test/Observation/obs-k-1".into(),
        })
    );

    // Inspect the live snapshot projection served by physician-sample.
    let snap_path = format!("/patient-workspaces/{ws}/chats/{chat_id}/snapshot");
    let (status, snapshot) = support::get_json(&stack.site_host, &snap_path, &session).await;
    assert_eq!(status, 200, "{snapshot}");
    let authored = snapshot["snapshot"]["conversation"]
        .as_array()
        .expect("conversation items")
        .last()
        .expect("authored");
    let citation_segments: Vec<&Value> = authored["segments"]
        .as_array()
        .expect("segments")
        .iter()
        .filter(|segment| segment["kind"] == "citation")
        .collect();
    assert_eq!(citation_segments.len(), 2);
    assert!(citation_segments[0]["destination"].is_null());
    assert_eq!(
        citation_segments[0]["source_presentation"]["content"]["as_of"],
        "2026-08-12T12:05:00+00:00"
    );
    assert_eq!(
        citation_segments[1]["destination"],
        json!({
            "kind": "source_native",
            "source_label": "Open in Synthetic EHR",
            "uri": "https://ehr.synthetic.test/Observation/obs-k-1"
        })
    );
    assert_eq!(
        citation_segments[1]["source_presentation"]["content"]["as_of"],
        "2026-08-14T09:30:00+00:00"
    );

    // Reopen both citations over the cited-source HTTP route.
    for citation in &answer.citations {
        let cite_path = format!(
            "/patient-workspaces/{ws}/chats/{chat_id}/citations/{}",
            citation.id
        );
        let (status, body) = support::get_json(&stack.site_host, &cite_path, &session).await;
        assert_eq!(status, 200, "{body}");
        let evidence: SourceEvidence = serde_json::from_value(body).expect("SourceEvidence");
        assert_eq!(evidence.inspection.technical.revision, citation.revision);
        assert_eq!(evidence.inspection.technical.reference, citation.reference);
    }

    let evidence_dump = json!({
        "saved_citations": answer.citations,
        "saved_source_previews": answer.source_previews,
        "snapshot_citation_segments": citation_segments,
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&evidence_dump).expect("pretty json")
    );
}

#[tokio::test]
async fn passage_citations_save_exact_locators_reopen_distinct_passages_and_refuse_absent_quote() {
    let _guard = lock().lock().await;
    let note_body = "Cardiology assessment: Blood pressure 128/76 mmHg on lisinopril 10 mg daily. Plan: Continue lisinopril and repeat basic metabolic panel in 3 months.";
    let bp_quote = "Blood pressure 128/76 mmHg on lisinopril 10 mg daily.";
    let plan_quote = "Continue lisinopril and repeat basic metabolic panel in 3 months.";

    let steps: Vec<StepFn> = vec![
        Box::new(|_req| {
            vec![fn_call(
                "read-note",
                READ_TOOL,
                json!({
                    "references": [
                        { "candidate_id": "DocumentReference/visit-note", "revision": "r1" }
                    ],
                    "max_result_bytes": 65536
                }),
            )]
        }),
        // Attempt 1: passage quote is NOT in delivered text -> refused by publish_citations.
        Box::new(|_req| {
            vec![fn_call(
                "bad-passage",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "DocumentReference/visit-note",
                        "revision": "r1",
                        "preview": "2026-08-15 visit note: BP 128/76 mmHg; continue lisinopril."
                    }],
                    "citations": [{
                        "id": "wrong-quote",
                        "reference": "DocumentReference/visit-note",
                        "revision": "r1",
                        "locator": {
                            "kind": "passage",
                            "part": 1,
                            "quote": "Blood pressure 110/70 mmHg on amlodipine."
                        }
                    }]
                }),
            )]
        }),
        // Attempt 2: verify refusal message, then publish two distinct passages on the same record.
        Box::new(move |req| {
            let refusal = tool_output(req, "bad-passage");
            assert_eq!(refusal["executed"], false, "{refusal}");
            let reason = refusal["error"].as_str().expect("refusal error");
            assert!(
                reason.contains("does not appear verbatim"),
                "expected absent-quote refusal, got: {reason}"
            );
            vec![fn_call(
                "good-passages",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "DocumentReference/visit-note",
                        "revision": "r1",
                        "preview": "2026-08-15 visit note: BP 128/76 mmHg; continue lisinopril."
                    }],
                    "citations": [
                        {
                            "id": "bp-passage",
                            "reference": "DocumentReference/visit-note",
                            "revision": "r1",
                            "locator": {
                                "kind": "passage",
                                "part": 1,
                                "quote": bp_quote
                            }
                        },
                        {
                            "id": "plan-passage",
                            "reference": "DocumentReference/visit-note",
                            "revision": "r1",
                            "locator": {
                                "kind": "passage",
                                "part": 1,
                                "quote": plan_quote
                            }
                        }
                    ]
                }),
            )]
        }),
        Box::new(|req| {
            let receipt = tool_output(req, "good-passages");
            assert_eq!(receipt["kind"], "published_citations_receipt", "{receipt}");
            vec![assistant_msg(
                "Blood pressure is 128/76 mmHg on lisinopril [[patient_source:bp-passage]], and the plan continues lisinopril with a repeat panel in 3 months [[patient_source:plan-passage]].",
            )]
        }),
    ];

    let Some(stack) = HarnessStack::start("ac2_ac3_passages", steps).await else {
        return;
    };
    let (scope, session) =
        enroll_test_physician_and_workspace(&stack.database.url, &stack.site_host).await;

    let raw_json =
        r#"{"resourceType":"DocumentReference","id":"visit-note","date":"2026-08-15T10:00:00Z"}"#;
    let mut record = SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "ehr".into(),
        candidate_id: "DocumentReference/visit-note".into(),
        revision: "r1".into(),
        record_type: "DocumentReference".into(),
        dates: vec![RecordDate {
            name: "date".into(),
            value: "2026-08-15T10:00:00Z".into(),
        }],
        content: raw_json.into(),
        sha256: local_product::sha256_hex(raw_json.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "attachment".into(),
            text: note_body.into(),
        }],
        provenance: json!({"native_record": "DocumentReference/visit-note"}),
        native: None,
    };
    stack
        .store
        .record_source(&record)
        .await
        .expect("record visit note");

    let (status, body) = support::post(
        &stack.site_host,
        "/commands",
        Some(&session),
        &support::submit(
            &scope.workspace,
            &scope.chat,
            "Check visit note BP and plan.",
        ),
    )
    .await;
    assert_eq!(status, 200, "{body}");

    let conversation = stack.wait_chat_completed(&scope).await;
    let answer = conversation.messages.last().expect("assistant answer");
    assert_eq!(answer.citations.len(), 2);
    assert_ne!(answer.citations[0].id, answer.citations[1].id);

    let bp_start = note_body.find(bp_quote).expect("bp start");
    let plan_start = note_body.find(plan_quote).expect("plan start");
    let expected_bp_locator = CitationLocator::Passage {
        part: CitedPart::ReadablePart {
            index: 0,
            label: "attachment".into(),
        },
        start_byte: bp_start,
        end_byte: bp_start + bp_quote.len(),
        quote: bp_quote.into(),
    };
    let expected_plan_locator = CitationLocator::Passage {
        part: CitedPart::ReadablePart {
            index: 0,
            label: "attachment".into(),
        },
        start_byte: plan_start,
        end_byte: plan_start + plan_quote.len(),
        quote: plan_quote.into(),
    };
    assert_eq!(
        answer.citations[0].locator.as_ref(),
        Some(&expected_bp_locator)
    );
    assert_eq!(
        answer.citations[1].locator.as_ref(),
        Some(&expected_plan_locator)
    );

    // Mutate the workspace store to revision r2 to verify reopening stays pinned to r1's exact passages.
    record.revision = "r2".into();
    record.readable_parts[0].text = "Amended note with completely different text.".into();
    stack.store.record_source(&record).await.expect("record r2");

    for (citation, expected_quote, expected_locator) in [
        (&answer.citations[0], bp_quote, &expected_bp_locator),
        (&answer.citations[1], plan_quote, &expected_plan_locator),
    ] {
        let path = format!(
            "/patient-workspaces/{}/chats/{}/citations/{}",
            scope.workspace, scope.chat, citation.id
        );
        let (status, payload) = support::get_json(&stack.site_host, &path, &session).await;
        assert_eq!(status, 200, "{payload}");
        let reopened: SourceEvidence = serde_json::from_value(payload).expect("SourceEvidence");
        assert_eq!(reopened.inspection.technical.revision, "r1");
        assert_eq!(
            reopened.inspection.cited_passage.as_deref(),
            Some(expected_quote)
        );
        assert_eq!(reopened.inspection.locator.as_ref(), Some(expected_locator));
    }
}

#[tokio::test]
async fn evidence_outside_current_window_is_refused_at_finish_with_explanation() {
    let _guard = lock().lock().await;
    let note_text = "Discharge summary: Potassium 4.2 mmol/L prior to discharge.";
    let quote = "Potassium 4.2 mmol/L prior to discharge.";

    let steps: Vec<StepFn> = vec![
        // Window 0: read record and stage preview + passage citation.
        Box::new(|_req| {
            vec![fn_call(
                "w0-read",
                READ_TOOL,
                json!({
                    "references": [
                        { "candidate_id": "DocumentReference/dc-summary", "revision": "r1" }
                    ],
                    "max_result_bytes": 65536
                }),
            )]
        }),
        Box::new(move |_req| {
            vec![fn_call(
                "w0-publish",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "DocumentReference/dc-summary",
                        "revision": "r1",
                        "preview": "2026-08-01 discharge summary: potassium 4.2 mmol/L."
                    }],
                    "citations": [{
                        "id": "w0-passage",
                        "reference": "DocumentReference/dc-summary",
                        "revision": "r1",
                        "locator": {
                            "kind": "passage",
                            "part": 1,
                            "quote": quote
                        }
                    }]
                }),
            )]
        }),
        // Rotate into a fresh context window (Window 1), dropping Window 0's read receipts.
        Box::new(|_req| {
            vec![fn_call(
                "open-fresh",
                "save_working_note",
                json!({
                    "text": "Need to re-read DocumentReference/dc-summary in the fresh window before citing.",
                    "fresh_window": true
                }),
            )]
        }),
        // Window 1, Attempt A: try finishing with the Window 0 passage citation without reading in Window 1.
        Box::new(|_req| {
            vec![assistant_msg(
                "Discharge potassium was 4.2 mmol/L [[patient_source:w0-passage]].",
            )]
        }),
        // Window 1, Attempt B: see refusal for out-of-window citation/passage, then replace staged citations
        // with an ungrounded preview alone and try finishing -> refused for out-of-window preview!
        Box::new(|req| {
            let feedback = developer_texts(req).join("\n");
            assert!(
                feedback.contains("current context window")
                    && feedback.contains("DocumentReference/dc-summary"),
                "expected out-of-window passage/citation refusal, got: {feedback}"
            );
            vec![fn_call(
                "w1-ungrounded-preview",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "replace": true,
                    "previews": [{
                        "reference": "DocumentReference/dc-summary",
                        "revision": "r1",
                        "preview": "2026-08-01 discharge summary: potassium 4.2 mmol/L."
                    }],
                    "citations": []
                }),
            )]
        }),
        Box::new(|_req| {
            vec![assistant_msg(
                "I have not re-read the discharge summary in this window.",
            )]
        }),
        // Window 1, Attempt C: see refusal for out-of-window preview, re-read in Window 1, re-publish, and finish!
        Box::new(|req| {
            let feedback = developer_texts(req).join("\n");
            assert!(
                feedback.contains("Preview for `DocumentReference/dc-summary`")
                    && feedback.contains("current context window"),
                "expected out-of-window preview refusal, got: {feedback}"
            );
            vec![fn_call(
                "w1-read",
                READ_TOOL,
                json!({
                    "references": [
                        { "candidate_id": "DocumentReference/dc-summary", "revision": "r1" }
                    ],
                    "max_result_bytes": 65536
                }),
            )]
        }),
        Box::new(move |_req| {
            vec![fn_call(
                "w1-publish",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "replace": true,
                    "previews": [{
                        "reference": "DocumentReference/dc-summary",
                        "revision": "r1",
                        "preview": "2026-08-01 discharge summary: potassium 4.2 mmol/L."
                    }],
                    "citations": [{
                        "id": "w1-passage",
                        "reference": "DocumentReference/dc-summary",
                        "revision": "r1",
                        "locator": {
                            "kind": "passage",
                            "part": 1,
                            "quote": quote
                        }
                    }]
                }),
            )]
        }),
        Box::new(|_req| {
            vec![assistant_msg(
                "Discharge potassium was 4.2 mmol/L [[patient_source:w1-passage]].",
            )]
        }),
    ];

    let Some(stack) = HarnessStack::start("ac4_window", steps).await else {
        return;
    };
    let (scope, session) =
        enroll_test_physician_and_workspace(&stack.database.url, &stack.site_host).await;

    let raw_json =
        r#"{"resourceType":"DocumentReference","id":"dc-summary","date":"2026-08-01T16:00:00Z"}"#;
    stack
        .store
        .record_source(&SourceRevision {
            workspace: scope.workspace.clone(),
            source_id: "ehr".into(),
            candidate_id: "DocumentReference/dc-summary".into(),
            revision: "r1".into(),
            record_type: "DocumentReference".into(),
            dates: vec![RecordDate {
                name: "date".into(),
                value: "2026-08-01T16:00:00Z".into(),
            }],
            content: raw_json.into(),
            sha256: local_product::sha256_hex(raw_json.as_bytes()),
            readable_parts: vec![ReadablePart {
                label: "attachment".into(),
                text: note_text.into(),
            }],
            provenance: json!({"native_record": "DocumentReference/dc-summary"}),
            native: None,
        })
        .await
        .expect("record dc summary");

    let (status, body) = support::post(
        &stack.site_host,
        "/commands",
        Some(&session),
        &support::submit(
            &scope.workspace,
            &scope.chat,
            "What was discharge potassium?",
        ),
    )
    .await;
    assert_eq!(status, 200, "{body}");

    let conversation = stack.wait_chat_completed(&scope).await;
    let answer = conversation.messages.last().expect("assistant answer");
    assert_eq!(answer.citations.len(), 1);
    assert_eq!(answer.citations[0].cited_passage(), Some(quote));
    assert_eq!(stack.provider.requests().len(), 9);
}

#[tokio::test]
async fn missing_preview_and_model_unavailable_refuse_while_failed_read_records_unavailable() {
    let _guard = lock().lock().await;
    let steps: Vec<StepFn> = vec![
        // Read two real records plus one missing record that fails as unavailable.
        Box::new(|_req| {
            vec![fn_call(
                "read-three",
                READ_TOOL,
                json!({
                    "references": [
                        { "candidate_id": "Observation/lab-k", "revision": "r1" },
                        { "candidate_id": "Observation/lab-na", "revision": "r1" },
                        { "candidate_id": "Observation/missing-mg", "revision": "r1" }
                    ],
                    "max_result_bytes": 65536
                }),
            )]
        }),
        // Attempt 1: model tries to write "Unavailable" -> refused immediately.
        Box::new(|_req| {
            vec![fn_call(
                "forbid-unavailable",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "Observation/lab-k",
                        "revision": "r1",
                        "preview": "Unavailable"
                    }],
                    "citations": []
                }),
            )]
        }),
        // Attempt 2: verify "Unavailable" refusal, then preview ONLY lab-k (omitting lab-na) and cite lab-k.
        Box::new(|req| {
            let refusal = tool_output(req, "forbid-unavailable");
            assert_eq!(refusal["executed"], false, "{refusal}");
            let reason = refusal["error"].as_str().expect("error");
            assert!(
                reason.contains("comes only from a source read that failed"),
                "expected Unavailable refusal, got: {reason}"
            );
            vec![fn_call(
                "partial-preview",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "Observation/lab-k",
                        "revision": "r1",
                        "preview": "2026-08-14 potassium 4.1 mmol/L."
                    }],
                    "citations": [{
                        "id": "k-cite",
                        "reference": "Observation/lab-k",
                        "revision": "r1",
                        "locator": null
                    }]
                }),
            )]
        }),
        // Attempt 3: try finishing while Observation/lab-na lacks a preview -> refused at finish!
        Box::new(|_req| {
            vec![assistant_msg(
                "Potassium is 4.1 mmol/L [[patient_source:k-cite]]; magnesium was not on file.",
            )]
        }),
        // Attempt 4: verify finish refusal tells model to write preview for Observation/lab-na, then supply it!
        Box::new(|req| {
            let feedback = developer_texts(req).join("\n");
            assert!(
                feedback.contains("Observation/lab-na") && feedback.contains("publish_citations"),
                "expected missing-preview refusal naming Observation/lab-na, got: {feedback}"
            );
            vec![fn_call(
                "complete-preview",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "Observation/lab-na",
                        "revision": "r1",
                        "preview": "2026-08-14 sodium 139 mmol/L."
                    }],
                    "citations": []
                }),
            )]
        }),
        Box::new(|_req| {
            vec![assistant_msg(
                "Potassium is 4.1 mmol/L [[patient_source:k-cite]]; sodium is 139 mmol/L and magnesium was not on file.",
            )]
        }),
    ];

    let Some(stack) = HarnessStack::start("ac5_previews", steps).await else {
        return;
    };
    let (scope, session) =
        enroll_test_physician_and_workspace(&stack.database.url, &stack.site_host).await;

    for (id, body) in [
        ("Observation/lab-k", r#"{"value":4.1,"unit":"mmol/L"}"#),
        ("Observation/lab-na", r#"{"value":139,"unit":"mmol/L"}"#),
    ] {
        stack
            .store
            .record_source(&SourceRevision {
                workspace: scope.workspace.clone(),
                source_id: "ehr".into(),
                candidate_id: id.into(),
                revision: "r1".into(),
                record_type: "Observation".into(),
                dates: vec![RecordDate {
                    name: "effectiveDateTime".into(),
                    value: "2026-08-14T09:00:00Z".into(),
                }],
                content: body.into(),
                sha256: local_product::sha256_hex(body.as_bytes()),
                readable_parts: vec![],
                provenance: json!({"native_record": id}),
                native: None,
            })
            .await
            .expect("record lab");
    }

    let (status, body) = support::post(
        &stack.site_host,
        "/commands",
        Some(&session),
        &support::submit(&scope.workspace, &scope.chat, "Check electrolytes."),
    )
    .await;
    assert_eq!(status, 200, "{body}");

    let conversation = stack.wait_chat_completed(&scope).await;
    let answer = conversation.messages.last().expect("assistant answer");
    assert_eq!(answer.citations.len(), 1);
    assert_eq!(answer.source_previews.len(), 3);
    assert_eq!(
        answer.source_previews,
        vec![
            ReadRecordPreview::Preview {
                reference: "Observation/lab-k".into(),
                revision: "r1".into(),
                dates: vec![RecordDate {
                    name: "effectiveDateTime".into(),
                    value: "2026-08-14T09:00:00Z".into(),
                }],
                preview: "2026-08-14 potassium 4.1 mmol/L.".into(),
                native: None,
            },
            ReadRecordPreview::Preview {
                reference: "Observation/lab-na".into(),
                revision: "r1".into(),
                dates: vec![RecordDate {
                    name: "effectiveDateTime".into(),
                    value: "2026-08-14T09:00:00Z".into(),
                }],
                preview: "2026-08-14 sodium 139 mmol/L.".into(),
                native: None,
            },
            ReadRecordPreview::Unavailable {
                reference: "Observation/missing-mg".into(),
                reason: "not_in_recorded_observation".into(),
            },
        ]
    );
}

#[tokio::test]
async fn structured_field_citation_reopens_same_version_and_every_job_shares_contract() {
    let _guard = lock().lock().await;
    let steps: Vec<StepFn> = vec![
        // --- Part 1: physician_chat structured-field citation + absent-path refusal + no answer-text parsing ---
        Box::new(|_req| {
            vec![fn_call(
                "chat-read-hb",
                READ_TOOL,
                json!({
                    "references": [
                        { "candidate_id": "Observation/lab-hb", "revision": "r1" }
                    ],
                    "max_result_bytes": 65536
                }),
            )]
        }),
        Box::new(|_req| {
            vec![fn_call(
                "chat-bad-field",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "Observation/lab-hb",
                        "revision": "r1",
                        "preview": "2026-08-10 hemoglobin 13.4 g/dL."
                    }],
                    "citations": [{
                        "id": "hb-missing",
                        "reference": "Observation/lab-hb",
                        "revision": "r1",
                        "locator": {
                            "kind": "field",
                            "path": "valueQuantity.nonexistent"
                        }
                    }]
                }),
            )]
        }),
        Box::new(|req| {
            let refusal = tool_output(req, "chat-bad-field");
            assert_eq!(refusal["executed"], false, "{refusal}");
            let reason = refusal["error"].as_str().expect("error");
            assert!(
                reason.contains("valueQuantity.nonexistent"),
                "expected missing-field refusal, got: {reason}"
            );
            vec![fn_call(
                "chat-good-field",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "Observation/lab-hb",
                        "revision": "r1",
                        "preview": "2026-08-10 hemoglobin 13.4 g/dL."
                    }],
                    "citations": [{
                        "id": "hb-value",
                        "reference": "Observation/lab-hb",
                        "revision": "r1",
                        "locator": {
                            "kind": "field",
                            "path": "valueQuantity.value"
                        }
                    }]
                }),
            )]
        }),
        Box::new(|_req| {
            vec![assistant_msg(
                "Hemoglobin is 13.4 g/dL [[patient_source:hb-value]].\n\n[[source_summaries]]\nhb-value: Untrusted prose block\n[[/source_summaries]]",
            )]
        }),
        // --- Part 2: physician_reply_revision sharing the exact same contract ---
        Box::new(|_req| {
            vec![fn_call(
                "rev-read-hb",
                READ_TOOL,
                json!({
                    "references": ["Observation/lab-hb"],
                    "max_result_bytes": 65536
                }),
            )]
        }),
        // Try finishing reply revision without calling publish_citations -> refused!
        Box::new(|_req| {
            vec![fn_call(
                "rev-premature-finish",
                FINISH_TOOL,
                json!({
                    "text": "Your hemoglobin result of 13.4 g/dL is normal.",
                    "review_basis": "Checked hemoglobin lab."
                }),
            )]
        }),
        Box::new(|req| {
            let refusal = tool_output(req, "rev-premature-finish");
            assert_eq!(refusal["executed"], false, "{refusal}");
            let feedback = refusal["error"].as_str().expect("refusal error");
            assert!(
                feedback.contains("Observation/lab-hb") && feedback.contains("publish_citations"),
                "reply_revision must enforce the same preview fence as chat, got: {feedback}"
            );
            vec![fn_call(
                "rev-publish",
                PUBLISH_CITATIONS_TOOL,
                json!({
                    "previews": [{
                        "reference": "Observation/lab-hb",
                        "revision": "r1",
                        "preview": "2026-08-10 hemoglobin 13.4 g/dL."
                    }],
                    "citations": [{
                        "id": "rev-hb-val",
                        "reference": "Observation/lab-hb",
                        "revision": "r1",
                        "locator": {
                            "kind": "field",
                            "path": "valueQuantity.value"
                        }
                    }]
                }),
            )]
        }),
        Box::new(|_req| {
            vec![fn_call(
                "rev-finish",
                FINISH_TOOL,
                json!({
                    "text": "Your hemoglobin result of 13.4 g/dL is normal.",
                    "review_basis": "Checked hemoglobin lab."
                }),
            )]
        }),
    ];

    let Some(stack) = HarnessStack::start("ac6_ac7_every_job", steps).await else {
        return;
    };
    let (scope, session) =
        enroll_test_physician_and_workspace(&stack.database.url, &stack.site_host).await;

    let hb_r1 = r#"{"resourceType":"Observation","id":"lab-hb","effectiveDateTime":"2026-08-10T08:00:00Z","valueQuantity":{"value":13.4,"unit":"g/dL"}}"#;
    let mut hb_source = SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "ehr".into(),
        candidate_id: "Observation/lab-hb".into(),
        revision: "r1".into(),
        record_type: "Observation".into(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".into(),
            value: "2026-08-10T08:00:00Z".into(),
        }],
        content: hb_r1.into(),
        sha256: local_product::sha256_hex(hb_r1.as_bytes()),
        readable_parts: vec![],
        provenance: json!({"native_record": "Observation/lab-hb"}),
        native: None,
    };
    stack
        .store
        .record_source(&hb_source)
        .await
        .expect("record hb r1");

    // 1. Run chat job.
    let (status, body) = support::post(
        &stack.site_host,
        "/commands",
        Some(&session),
        &support::submit(
            &scope.workspace,
            &scope.chat,
            "What is the hemoglobin value?",
        ),
    )
    .await;
    assert_eq!(status, 200, "{body}");

    let conversation = stack.wait_chat_completed(&scope).await;
    let answer = conversation.messages.last().expect("assistant answer");
    assert_eq!(answer.citations.len(), 1);
    assert_eq!(
        answer.citations[0].summary.as_deref(),
        Some("2026-08-10 hemoglobin 13.4 g/dL.")
    );
    assert!(
        answer.text.contains("[[source_summaries]]"),
        "answer text is never parsed or stripped for summaries: {}",
        answer.text
    );
    let expected_path: SourcePath = "valueQuantity.value".parse().expect("valid SourcePath");
    assert_eq!(
        answer.citations[0].locator,
        Some(CitationLocator::Field {
            path: expected_path.clone(),
        })
    );

    // 2. Run background reply_revision job while r1 is still current.
    let reply_scope = Scope {
        chat: format!("reply-{}", uuid::Uuid::new_v4().simple()),
        ..scope.clone()
    };
    let hash = PostgresAuthStore::session_reference(&session);
    let opened = stack
        .store
        .open_draft_authorized(
            &reply_scope,
            "Following up on your labs.",
            &hash,
            &mut physician_sample::durable::RuntimeAuthority,
        )
        .await
        .expect("open draft")
        .expect("authorized draft");
    assert_eq!(opened.revision, 1);

    let (status, rev_accepted) = support::post(
        &stack.site_host,
        "/commands",
        Some(&session),
        &json!({
            "type": "reply.revise",
            "idempotency_key": "rev-1",
            "payload": {
                "patient_workspace_id": reply_scope.workspace,
                "reply_work_id": reply_scope.chat,
                "expected_draft_revision": 1,
                "instruction": "Include the hemoglobin result.",
                "physician_time_zone": "America/New_York"
            }
        }),
    )
    .await;
    assert_eq!(status, 200, "{rev_accepted}");
    assert_eq!(rev_accepted["outcome"], "accepted", "{rev_accepted}");

    let deadline = Instant::now() + Duration::from_secs(20);
    let draft = loop {
        let current = stack
            .store
            .draft(&reply_scope)
            .await
            .expect("draft read")
            .expect("draft exists");
        if current.status == DraftStatus::Completed {
            break current;
        }
        assert_ne!(
            current.status,
            DraftStatus::Failed,
            "reply revision failed:\n{}",
            stack.worker.tail()
        );
        assert!(
            Instant::now() < deadline,
            "timed out waiting for reply revision:\n{}",
            stack.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    };
    assert_eq!(draft.revision, 2);
    assert_eq!(draft.text, "Your hemoglobin result of 13.4 g/dL is normal.");
    assert_eq!(draft.citations.len(), 1);
    assert_eq!(
        draft.citations[0].locator,
        Some(CitationLocator::Field {
            path: expected_path.clone(),
        })
    );
    assert_eq!(
        draft.citations[0].summary.as_deref(),
        Some("2026-08-10 hemoglobin 13.4 g/dL.")
    );
    assert_eq!(draft.source_previews.len(), 1);

    // 3. Advance Observation/lab-hb to r2 in the workspace store, then reopen the chat citation
    // and verify it reopens valueQuantity.value at version r1 (13.4, not 9.1).
    let hb_r2 = r#"{"resourceType":"Observation","id":"lab-hb","effectiveDateTime":"2026-08-11T08:00:00Z","valueQuantity":{"value":9.1,"unit":"g/dL"}}"#;
    hb_source.revision = "r2".into();
    hb_source.content = hb_r2.into();
    hb_source.sha256 = local_product::sha256_hex(hb_r2.as_bytes());
    stack
        .store
        .record_source(&hb_source)
        .await
        .expect("record hb r2");

    let cite_path = format!(
        "/patient-workspaces/{}/chats/{}/citations/{}",
        scope.workspace, scope.chat, answer.citations[0].id
    );
    let (status, payload) = support::get_json(&stack.site_host, &cite_path, &session).await;
    assert_eq!(status, 200, "{payload}");
    let reopened: SourceEvidence = serde_json::from_value(payload).expect("SourceEvidence");
    assert_eq!(reopened.inspection.technical.revision, "r1");
    assert_eq!(
        reopened.inspection.locator,
        Some(CitationLocator::Field {
            path: expected_path.clone(),
        })
    );
    let reopened_json: Value =
        serde_json::from_str(&reopened.inspection.technical.original).expect("technical json");
    assert_eq!(expected_path.resolve(&reopened_json), Some(&json!(13.4)));
}
