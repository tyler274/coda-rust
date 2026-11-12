//! Integration proof for clinical library search and exact passage read tools,
//! served over a local ephemeral Axum HTTP server and exercised through both
//! `ClinicalLibraryClient` and end-to-end `PhysicianHost` chat turns.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse, Response},
    routing::post,
};
use local_product::{
    ModelBinding,
    clinical_library::{
        ClinicalLibraryClient, LibraryCollection, LibraryPassage, LibraryReadOutcome,
        LibrarySearchOutcome, LibrarySource, PassageId, PublicationStatus, READ_LIBRARY_TOOL,
        ReadWireRequest, ReadWireResponse, SEARCH_LIBRARY_TOOL, SearchWireRequest,
        SearchWireResponse,
    },
    physician_chat::{self, PhysicianSkills},
    physician_sources::{READ_TOOL, RecordDate, SEARCH_TOOL, SourceRevision},
    runtime::{
        Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store, WorkStatus,
        async_trait, engine,
    },
    sha256_hex,
};
use serde_json::{Value, json};

#[derive(Clone)]
enum ServerBehavior {
    Healthy {
        passages: Vec<LibraryPassage>,
        required_bearer: Option<String>,
    },
    ServiceUnavailable,
    TamperedDigest {
        passage: Box<LibraryPassage>,
    },
}

#[derive(Clone)]
struct RecordedLibraryState {
    behavior: ServerBehavior,
    requests: Arc<AtomicUsize>,
}

fn fixture_passage(
    id: &str,
    source: LibrarySource,
    date: &str,
    status: PublicationStatus,
    attribution: &str,
    text: &str,
) -> LibraryPassage {
    LibraryPassage {
        passage_id: PassageId::new(id).unwrap(),
        source,
        date: date.into(),
        status,
        attribution: attribution.into(),
        text: text.into(),
        content_sha256: sha256_hex(text.as_bytes()),
    }
}

fn recorded_passages() -> Vec<LibraryPassage> {
    vec![
        fixture_passage(
            "uspstf:colorectal-2025#rec-a",
            LibrarySource {
                collection: LibraryCollection::Uspstf,
                document_id: "uspstf-crc-2025".into(),
                title: "Screening for Colorectal Cancer: US Preventive Services Task Force Recommendation Statement".into(),
                section: Some("Recommendation Summary".into()),
            },
            "2025-05-20",
            PublicationStatus::Current,
            "US Preventive Services Task Force; JAMA 2025; public domain",
            "The USPSTF recommends screening for colorectal cancer in all adults aged 45 to 75 years (A recommendation).",
        ),
        fixture_passage(
            "dailymed:dapagliflozin-label#renal-dosing",
            LibrarySource {
                collection: LibraryCollection::Dailymed,
                document_id: "setid-88a1bc02-dapagliflozin".into(),
                title: "Dapagliflozin Film-Coated Tablets Prescribing Information".into(),
                section: Some("2.2 Recommended Dosage in Chronic Kidney Disease".into()),
            },
            "2025-11-03",
            PublicationStatus::Corrected {
                notice: "Label erratum 2025-11-03: clarified initiation eGFR threshold of 25 mL/min/1.73 m2 for chronic kidney disease.".into(),
            },
            "DailyMed / National Library of Medicine; public domain label text",
            "For chronic kidney disease, initiate dapagliflozin 10 mg orally once daily in adults with eGFR at least 25 mL/min/1.73 m2; if eGFR falls below 25 mL/min/1.73 m2 during treatment, continuation of 10 mg once daily is supported to reduce eGFR decline and heart failure hospitalization.",
        ),
        fixture_passage(
            "pmc:PMC9981042#abstract",
            LibrarySource {
                collection: LibraryCollection::Pmc,
                document_id: "PMC9981042".into(),
                title: "High-Dose Potassium Binder Monotherapy in Acute Hyperkalemia".into(),
                section: Some("Abstract".into()),
            },
            "2024-08-14",
            PublicationStatus::Retracted {
                notice: "Retracted (PMID 39200418): primary electrolyte endpoint tables could not be verified from trial source data.".into(),
            },
            " Patel R et al., J Acute Med 2024; CC BY 4.0",
            "Single-agent binder monotherapy normalized serum potassium within 45 minutes without insulin-dextrose.",
        ),
    ]
}

async fn search_handler(
    State(state): State<RecordedLibraryState>,
    headers: HeaderMap,
    Json(request): Json<SearchWireRequest>,
) -> Response {
    state.requests.fetch_add(1, Ordering::SeqCst);
    match &state.behavior {
        ServerBehavior::ServiceUnavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "index replica warming"})),
        )
            .into_response(),
        ServerBehavior::TamperedDigest { passage } => {
            let mut corrupted = (**passage).clone();
            corrupted.content_sha256 = "0".repeat(64);
            Json(SearchWireResponse {
                passages: vec![corrupted],
            })
            .into_response()
        }
        ServerBehavior::Healthy {
            passages,
            required_bearer,
        } => {
            if let Some(expected) = required_bearer {
                let actual = headers
                    .get(AUTHORIZATION)
                    .and_then(|value| value.to_str().ok());
                if actual != Some(&format!("Bearer {expected}")) {
                    return (
                        StatusCode::FORBIDDEN,
                        Json(json!({"error": "missing or invalid invoker token"})),
                    )
                        .into_response();
                }
            }
            let terms: Vec<String> = request
                .query
                .split_whitespace()
                .map(str::to_ascii_lowercase)
                .collect();
            let filtered: Vec<LibraryPassage> = passages
                .iter()
                .filter(|passage| {
                    if let Some(collection) = request.collection
                        && passage.source.collection != collection
                    {
                        return false;
                    }
                    let haystack = format!(
                        "{} {} {}",
                        passage.source.title.to_ascii_lowercase(),
                        passage
                            .source
                            .section
                            .as_deref()
                            .unwrap_or_default()
                            .to_ascii_lowercase(),
                        passage.text.to_ascii_lowercase()
                    );
                    terms.iter().any(|term| haystack.contains(term))
                })
                .take(request.limit)
                .cloned()
                .collect();
            Json(SearchWireResponse { passages: filtered }).into_response()
        }
    }
}

async fn read_handler(
    State(state): State<RecordedLibraryState>,
    headers: HeaderMap,
    Json(request): Json<ReadWireRequest>,
) -> Response {
    state.requests.fetch_add(1, Ordering::SeqCst);
    match &state.behavior {
        ServerBehavior::ServiceUnavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "index replica warming"})),
        )
            .into_response(),
        ServerBehavior::TamperedDigest { passage } => {
            let mut corrupted = (**passage).clone();
            corrupted.content_sha256 = "0".repeat(64);
            Json(ReadWireResponse {
                passage: Some(corrupted),
            })
            .into_response()
        }
        ServerBehavior::Healthy {
            passages,
            required_bearer,
        } => {
            if let Some(expected) = required_bearer {
                let actual = headers
                    .get(AUTHORIZATION)
                    .and_then(|value| value.to_str().ok());
                if actual != Some(&format!("Bearer {expected}")) {
                    return (
                        StatusCode::FORBIDDEN,
                        Json(json!({"error": "missing or invalid invoker token"})),
                    )
                        .into_response();
                }
            }
            let found = passages
                .iter()
                .find(|passage| passage.passage_id == request.passage_id)
                .cloned();
            match found {
                Some(passage) => Json(ReadWireResponse {
                    passage: Some(passage),
                })
                .into_response(),
                None => (
                    StatusCode::NOT_FOUND,
                    Json(ReadWireResponse { passage: None }),
                )
                    .into_response(),
            }
        }
    }
}

async fn spawn_library_server(behavior: ServerBehavior) -> (String, Arc<AtomicUsize>) {
    let requests = Arc::new(AtomicUsize::new(0));
    let state = RecordedLibraryState {
        behavior,
        requests: Arc::clone(&requests),
    };
    let app = Router::new()
        .route("/v1/passages/search", post(search_handler))
        .route("/v1/passages/read", post(read_handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    (format!("http://{address}"), requests)
}

#[tokio::test]
async fn search_returns_passages_with_source_date_status_and_attribution_and_refuses_patient_identifiers()
 {
    let (base_url, requests) = spawn_library_server(ServerBehavior::Healthy {
        passages: recorded_passages(),
        required_bearer: Some("test-invoker-token".into()),
    })
    .await;
    let client = ClinicalLibraryClient::http(&base_url)
        .unwrap()
        .with_bearer_token("test-invoker-token");

    // Patient-identifying queries are refused before touching the library server.
    for identifier_query in [
        "pw-0123456789abcdef1234567890abcdef dapagliflozin",
        "hyperkalemia MRN 994821",
        "colorectal screening DOB 05/19/1968",
        "patient.email@codahealth.test potassium binder",
    ] {
        let outcome = client
            .execute(
                SEARCH_LIBRARY_TOOL,
                &json!({
                    "query": identifier_query,
                    "collection": null,
                    "limit": null
                }),
            )
            .await;
        assert!(
            matches!(outcome, engine::ToolOutcome::Refused(_)),
            "expected patient identifier query `{identifier_query}` to be refused: {outcome:?}"
        );
    }
    assert_eq!(
        requests.load(Ordering::SeqCst),
        0,
        "refused identifier queries must never reach the clinical library network service"
    );

    // Clinical concept query returns current, corrected, and retracted passages with full metadata.
    let engine::ToolOutcome::Returned(payload) = client
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "colorectal dapagliflozin hyperkalemia",
                "collection": null,
                "limit": 10
            }),
        )
        .await
    else {
        panic!("expected Returned outcome from search_clinical_library");
    };
    let parsed: LibrarySearchOutcome = serde_json::from_str(&payload).unwrap();
    let LibrarySearchOutcome::Results { passages } = parsed else {
        panic!("expected search results, got {parsed:?}");
    };
    assert_eq!(passages.len(), 3);
    for passage in &passages {
        assert!(!passage.source.document_id.is_empty());
        assert!(!passage.source.title.is_empty());
        assert!(!passage.date.is_empty());
        assert!(!passage.attribution.is_empty());
        assert!(passage.validate().is_ok());
    }
    assert!(
        passages
            .iter()
            .any(|p| matches!(p.status, PublicationStatus::Current))
    );
    assert!(passages.iter().any(|p| matches!(
        &p.status,
        PublicationStatus::Corrected { notice } if notice.contains("eGFR threshold of 25")
    )));
    assert!(passages.iter().any(|p| matches!(
        &p.status,
        PublicationStatus::Retracted { notice } if notice.contains("Retracted")
    )));
}

#[tokio::test]
async fn exact_read_returns_identical_passage_and_reports_not_found_when_absent() {
    let (base_url, _) = spawn_library_server(ServerBehavior::Healthy {
        passages: recorded_passages(),
        required_bearer: None,
    })
    .await;
    let client = ClinicalLibraryClient::http(&base_url).unwrap();

    let engine::ToolOutcome::Returned(search_json) = client
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "dapagliflozin hyperkalemia colorectal",
                "collection": null,
                "limit": 10
            }),
        )
        .await
    else {
        panic!("expected search results");
    };
    let LibrarySearchOutcome::Results { passages } = serde_json::from_str(&search_json).unwrap()
    else {
        panic!("expected Results variant");
    };
    assert_eq!(passages.len(), 3);

    for searched in &passages {
        let engine::ToolOutcome::Returned(read_json) = client
            .execute(
                READ_LIBRARY_TOOL,
                &json!({
                    "passage_id": searched.passage_id.as_str()
                }),
            )
            .await
        else {
            panic!("expected Returned outcome from read_clinical_library");
        };
        let LibraryReadOutcome::Passage { passage } = serde_json::from_str(&read_json).unwrap()
        else {
            panic!("expected exact passage for {}", searched.passage_id);
        };
        assert_eq!(passage.as_ref(), searched);
    }

    let engine::ToolOutcome::Returned(missing_json) = client
        .execute(
            READ_LIBRARY_TOOL,
            &json!({
                "passage_id": "pmc:PMC0000000#missing"
            }),
        )
        .await
    else {
        panic!("expected Returned outcome for missing passage");
    };
    let missing: LibraryReadOutcome = serde_json::from_str(&missing_json).unwrap();
    assert_eq!(
        missing,
        LibraryReadOutcome::NotFound {
            passage_id: PassageId::new("pmc:PMC0000000#missing").unwrap(),
        }
    );
}

#[tokio::test]
async fn unavailable_library_differs_from_nothing_found_across_failure_modes() {
    let (healthy_url, _) = spawn_library_server(ServerBehavior::Healthy {
        passages: recorded_passages(),
        required_bearer: Some("expected-secret".into()),
    })
    .await;
    let healthy = ClinicalLibraryClient::http(&healthy_url)
        .unwrap()
        .with_bearer_token("expected-secret");

    // Empty search ("nothing found"): outcome == "results" with empty passages.
    let engine::ToolOutcome::Returned(empty_json) = healthy
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "neurocysticercosis albendazole",
                "collection": null,
                "limit": 5
            }),
        )
        .await
    else {
        panic!("expected Returned empty search");
    };
    let empty_value: Value = serde_json::from_str(&empty_json).unwrap();
    assert_eq!(empty_value["outcome"], "results");
    assert_eq!(empty_value["passages"], json!([]));

    // Obtain a closed ephemeral port for connection-refused testing.
    let closed_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let closed_addr = closed_listener.local_addr().unwrap();
    drop(closed_listener);

    let (unavailable_url, _) = spawn_library_server(ServerBehavior::ServiceUnavailable).await;
    let (tampered_url, _) = spawn_library_server(ServerBehavior::TamperedDigest {
        passage: Box::new(recorded_passages().remove(0)),
    })
    .await;

    let unavailable_clients = [
        ("unconfigured", ClinicalLibraryClient::unconfigured()),
        (
            "unreachable",
            ClinicalLibraryClient::http(format!("http://{closed_addr}")).unwrap(),
        ),
        (
            "http_503",
            ClinicalLibraryClient::http(&unavailable_url).unwrap(),
        ),
        (
            "http_403_wrong_token",
            ClinicalLibraryClient::http(&healthy_url)
                .unwrap()
                .with_bearer_token("wrong-secret"),
        ),
        (
            "tampered_digest",
            ClinicalLibraryClient::http(&tampered_url).unwrap(),
        ),
    ];

    for (label, client) in unavailable_clients {
        let engine::ToolOutcome::Returned(search_raw) = client
            .execute(
                SEARCH_LIBRARY_TOOL,
                &json!({
                    "query": "dapagliflozin chronic kidney disease",
                    "collection": null,
                    "limit": 5
                }),
            )
            .await
        else {
            panic!("{label}: expected Returned unavailable outcome");
        };
        let search_val: Value = serde_json::from_str(&search_raw).unwrap();
        assert_eq!(
            search_val["outcome"], "unavailable",
            "{label}: search outcome must be `unavailable`, never `results`"
        );
        assert!(search_val.get("passages").is_none());
        assert_ne!(search_val, empty_value);

        let engine::ToolOutcome::Returned(read_raw) = client
            .execute(
                READ_LIBRARY_TOOL,
                &json!({
                    "passage_id": "uspstf:colorectal-2025#rec-a"
                }),
            )
            .await
        else {
            panic!("{label}: expected Returned unavailable outcome on read");
        };
        let read_val: Value = serde_json::from_str(&read_raw).unwrap();
        assert_eq!(read_val["outcome"], "unavailable", "{label}");
    }
}

struct AllowedAuthority;

#[async_trait]
impl Authority for AllowedAuthority {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

fn chat_recipe(skills: &PhysicianSkills) -> engine::recipe::RequestRecipe {
    let binding = ModelBinding {
        model: "controlled-chat".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 250_000,
        effort: "high".into(),
        adapter: "controlled".into(),
    };
    physician_chat::recipe(&binding, physician_chat::INSTRUCTIONS, skills)
}

fn chart_renal_lab(workspace: &str) -> SourceRevision {
    let content =
        r#"{"resourceType":"Observation","code":"eGFR and Potassium","effectiveDate":"2026-11-10","eGFR_mL_min":32,"potassium_mmol_L":4.6}"#
            .to_owned();
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "clinic-ehr".into(),
        candidate_id: "obs-renal-2026-11-10".into(),
        revision: "rev-renal-1".into(),
        record_type: "Observation".into(),
        dates: vec![RecordDate {
            name: "effectiveDate".into(),
            value: "2026-11-10".into(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: vec![],
        provenance: json!({"system": "clinic-ehr", "id": "obs-renal-2026-11-10"}),
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn unavailable_library_still_answers_chart_question_and_states_literature_limitation() {
    use engine::recipe::{Observer, Provider, Request};

    struct ChartWhenLibraryDown {
        turn: usize,
    }

    impl Provider for ChartWhenLibraryDown {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    for expected_tool in [
                        SEARCH_TOOL,
                        READ_TOOL,
                        SEARCH_LIBRARY_TOOL,
                        READ_LIBRARY_TOOL,
                    ] {
                        assert!(
                            request
                                .tools
                                .iter()
                                .any(|tool| tool["name"] == expected_tool),
                            "missing expected chat tool {expected_tool}"
                        );
                    }
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "lib-search-1".into(),
                                name: SEARCH_LIBRARY_TOOL.into(),
                                arguments: json!({
                                    "query": "dapagliflozin initiation eGFR threshold chronic kidney disease",
                                    "collection": "dailymed",
                                    "limit": 5
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "chart-search-1".into(),
                                name: SEARCH_TOOL.into(),
                                arguments: json!({
                                    "query": "eGFR",
                                    "record_type": null,
                                    "after": null
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let lib_output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "lib-search-1" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("library tool result delivered");
                    let lib_json: Value = serde_json::from_str(lib_output).unwrap();
                    assert_eq!(lib_json["outcome"], "unavailable");

                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "chart-read-1".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "obs-renal-2026-11-10",
                                    "revision": "rev-renal-1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 8192
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let chart_output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "chart-read-1" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("chart read result delivered");
                    let chart_json: Value = serde_json::from_str(chart_output).unwrap();
                    let delivered_content = chart_json["results"][0]["content"]
                        .as_str()
                        .expect("available chart record content");
                    assert!(delivered_content.contains("\"eGFR_mL_min\":32"));

                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "On the 2026-11-10 metabolic observation (obs-renal-2026-11-10), eGFR is 32 mL/min/1.73 m2 and potassium is 4.6 mmol/L.\n\nBasis: Grounded in the 2026-11-10 clinic EHR Observation record (obs-renal-2026-11-10, rev-renal-1) and general medical knowledge; the clinical library was unavailable during this turn, so current DailyMed label thresholds could not be verified directly.",
                        )],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected provider turn {other}"),
            }
        }
    }

    let db_url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let (unavailable_url, _) = spawn_library_server(ServerBehavior::ServiceUnavailable).await;
    let store = Store::connect(&db_url)
        .await
        .unwrap()
        .with_clinical_library(ClinicalLibraryClient::http(&unavailable_url).unwrap());
    store.migrate().await.unwrap();

    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    store
        .record_source(&chart_renal_lab(&workspace))
        .await
        .unwrap();

    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "dr-rivera".into(),
        workspace,
        chat: "chat-lib-down".into(),
    };
    let skills = PhysicianSkills::configured(None, false).unwrap();
    let recipe = chat_recipe(&skills);
    store
        .accept_authorized(
            &scope,
            &Input {
                key: "ask-renal-1".into(),
                text: "What is this patient's latest eGFR and does dapagliflozin labeling allow initiation?".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            vec![9_u8; 32],
            &skills,
            &recipe,
        )
        .await
        .unwrap();

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowedAuthority,
    };
    let mut provider = ChartWhenLibraryDown { turn: 0 };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let saved = store.conversation(&scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    let reply = saved.responses().next().expect("published assistant reply");
    assert!(reply.text.contains("32 mL/min/1.73 m2"));
    assert!(reply.text.contains("2026-11-10"));
    assert!(reply.text.contains("clinical library was unavailable"));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn chat_question_needing_literature_searches_reads_and_cites_library_passage_with_source_and_date()
 {
    use engine::recipe::{Observer, Provider, Request};

    struct CiteLiteratureProvider {
        turn: usize,
        cited_passage_id: Option<String>,
    }

    impl Provider for CiteLiteratureProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "lib-search-ckd".into(),
                        name: SEARCH_LIBRARY_TOOL.into(),
                        arguments: json!({
                            "query": "dapagliflozin chronic kidney disease eGFR threshold",
                            "collection": "dailymed",
                            "limit": 5
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "lib-search-ckd" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("search result delivered");
                    let LibrarySearchOutcome::Results { passages } =
                        serde_json::from_str(output).unwrap()
                    else {
                        panic!("expected library search hits");
                    };
                    let hit = &passages[0];
                    self.cited_passage_id = Some(hit.passage_id.as_str().to_owned());
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "lib-read-ckd".into(),
                            name: READ_LIBRARY_TOOL.into(),
                            arguments: json!({
                                "passage_id": hit.passage_id.as_str()
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "lib-read-ckd" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("exact passage read delivered");
                    let LibraryReadOutcome::Passage { passage } =
                        serde_json::from_str(output).unwrap()
                    else {
                        panic!("expected exact passage");
                    };
                    let answer = format!(
                        "Dapagliflozin 10 mg once daily may be initiated in chronic kidney disease when eGFR is at least 25 mL/min/1.73 m2 [{} — {}, {}, {}]. Note published label correction: {}\n\nBasis: DailyMed passage {} ({}, {}).",
                        passage.passage_id,
                        passage.source.title,
                        passage.source.document_id,
                        passage.date,
                        match &passage.status {
                            PublicationStatus::Corrected { notice } => notice.as_str(),
                            _ => "",
                        },
                        passage.passage_id,
                        passage.source.document_id,
                        passage.date,
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(&answer)],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected provider turn {other}"),
            }
        }
    }

    let db_url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let (healthy_url, _) = spawn_library_server(ServerBehavior::Healthy {
        passages: recorded_passages(),
        required_bearer: None,
    })
    .await;
    let library_client = ClinicalLibraryClient::http(&healthy_url).unwrap();
    let store = Store::connect(&db_url)
        .await
        .unwrap()
        .with_clinical_library(library_client.clone());
    store.migrate().await.unwrap();

    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "dr-rivera".into(),
        workspace: format!("pw-{}", uuid::Uuid::new_v4().simple()),
        chat: "chat-lib-cite".into(),
    };
    let skills = PhysicianSkills::configured(None, false).unwrap();
    let recipe = chat_recipe(&skills);
    store
        .accept_authorized(
            &scope,
            &Input {
                key: "ask-dapa-label".into(),
                text:
                    "What eGFR threshold does the dapagliflozin label specify for CKD initiation?"
                        .into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            vec![5_u8; 32],
            &skills,
            &recipe,
        )
        .await
        .unwrap();

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowedAuthority,
    };
    let mut provider = CiteLiteratureProvider {
        turn: 0,
        cited_passage_id: None,
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let cited_id = provider.cited_passage_id.expect("cited passage id");
    let saved = store.conversation(&scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    let reply = saved.responses().next().expect("published reply");
    assert!(reply.text.contains(&cited_id));
    assert!(reply.text.contains("setid-88a1bc02-dapagliflozin"));
    assert!(reply.text.contains("2025-11-03"));

    // Opening the cited passage ID returns the exact passage with its source and date.
    let reopened = library_client
        .read_passage(&PassageId::new(cited_id).unwrap())
        .await;
    let LibraryReadOutcome::Passage { passage } = reopened else {
        panic!("expected cited passage to reopen");
    };
    assert_eq!(passage.source.document_id, "setid-88a1bc02-dapagliflozin");
    assert_eq!(passage.date, "2025-11-03");
}
