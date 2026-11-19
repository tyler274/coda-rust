//! Integration proof for clinical library search and exact passage read tools,
//! served over a local ephemeral Axum HTTP server implementing the real
//! `ClinicalEvidenceServer` (`POST /v1/search` and `POST /v1/read`) strict wire
//! schemas and exercised through both `ClinicalLibraryClient` and end-to-end
//! `PhysicianHost` chat turns.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::{
        HeaderMap, StatusCode,
        header::{AUTHORIZATION, LOCATION},
    },
    response::{IntoResponse, Response},
    routing::{get, post},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use local_product::{
    ModelBinding,
    clinical_library::{
        ClinicalLibraryClient, LibraryCollection, LibraryReadOutcome, LibrarySearchFreshness,
        LibrarySearchOutcome, PassageId, PublicationStatus, READ_LIBRARY_TOOL, SEARCH_LIBRARY_TOOL,
        WireBibliographicObservation, WireBibliographicRelationship, WireCandidate,
        WireEvidenceUnit, WireIdentifier, WireLifecycleState, WireReadItem,
        WireReadUnavailableCause, WireRelationship, WireRelationshipType, WireSearchPage,
    },
    physician_chat::{self, PhysicianSkills},
    physician_sources::{
        READ_TOOL, RecordDate, SEARCH_TOOL, SourceRevision, citations::FINISH_TOOL,
    },
    runtime::{
        AssistantKind, Authority, Input, MessageRole, PhysicianHost, PublicationTransaction, Scope,
        ShownSteps, Store, WorkStatus, async_trait, engine,
    },
    sha256_hex,
};
use serde_json::{Value, json};

const USPSTF_UNIT_ID: &str = "11111111-1111-4111-8111-111111111101";
const DAILYMED_UNIT_ID: &str = "22222222-2222-4222-8222-222222222202";
const PMC_RETRACTED_UNIT_ID: &str = "33333333-3333-4333-8333-333333333303";
const MMWR_UNIT_ID: &str = "44444444-4444-4444-8444-444444444404";
const MISSING_UNIT_ID: &str = "99999999-9999-4999-8999-999999999999";

#[derive(Clone)]
enum BearerPolicy {
    Open,
    Exact(String),
    Prefix(String),
}

impl BearerPolicy {
    fn allows(&self, headers: &HeaderMap) -> bool {
        match self {
            Self::Open => true,
            Self::Exact(expected) => {
                headers
                    .get(AUTHORIZATION)
                    .and_then(|value| value.to_str().ok())
                    == Some(&format!("Bearer {expected}"))
            }
            Self::Prefix(prefix) => headers
                .get(AUTHORIZATION)
                .and_then(|value| value.to_str().ok())
                .and_then(|raw| raw.strip_prefix("Bearer "))
                .is_some_and(|token| token.starts_with(prefix) && token.split('.').count() == 3),
        }
    }
}

#[derive(Clone)]
enum ServerBehavior {
    Healthy {
        units: Vec<WireEvidenceUnit>,
        auth: BearerPolicy,
    },
    ServiceUnavailable,
    RedirectTo {
        target_base: String,
    },
    TamperedDigest {
        unit: Box<WireEvidenceUnit>,
    },
    CorruptCandidateId {
        unit: Box<WireEvidenceUnit>,
    },
    TruncatedCollectionScan {
        unit: Box<WireEvidenceUnit>,
    },
}

#[derive(Clone)]
struct RecordedLibraryState {
    behavior: ServerBehavior,
    requests: Arc<AtomicUsize>,
}

struct UnitSpec<'a> {
    unit_id: &'a str,
    version_id: &'a str,
    source_code: &'a str,
    source_display_name: &'a str,
    native_document_id: &'a str,
    title: &'a str,
    label: Option<&'a str>,
    publication_date: Option<&'a str>,
    lifecycle_state: WireLifecycleState,
    source_status_code: Option<&'a str>,
    attribution_text: Option<&'a str>,
    license_identifier: Option<&'a str>,
    content: &'a str,
    relationships: Vec<WireRelationship>,
    bibliographic_observations: Option<Vec<WireBibliographicObservation>>,
}

fn build_wire_unit(spec: UnitSpec<'_>) -> WireEvidenceUnit {
    WireEvidenceUnit {
        bibliographic_observations: spec.bibliographic_observations,
        unit_id: spec.unit_id.into(),
        version_id: spec.version_id.into(),
        source_code: spec.source_code.into(),
        source_display_name: spec.source_display_name.into(),
        native_document_id: spec.native_document_id.into(),
        native_version_id: Some("v1".into()),
        source_version_label: None,
        title: spec.title.into(),
        document_type: "clinical_guidance".into(),
        publication_date: spec.publication_date.map(str::to_owned),
        version_date: spec.publication_date.map(str::to_owned),
        acquired_at: "2026-11-10T04:00:00Z".into(),
        lifecycle_state: spec.lifecycle_state,
        source_status_code: spec.source_status_code.map(str::to_owned),
        native_unit_id: format!("{}-u1", spec.native_document_id),
        native_locator: "sec-1".into(),
        unit_kind: "section".into(),
        ordinal: 0,
        label: spec.label.map(str::to_owned),
        content: spec.content.into(),
        content_sha256: sha256_hex(spec.content.as_bytes()),
        native_destination_uri: format!("https://clinical.example.org/{}", spec.native_document_id),
        version_destination_uri: format!(
            "https://clinical.example.org/{}/v1",
            spec.native_document_id
        ),
        rights_decision_id: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".into(),
        license_identifier: spec.license_identifier.map(str::to_owned),
        attribution_text: spec.attribution_text.map(str::to_owned),
        may_display_excerpt: true,
        identifiers: vec![WireIdentifier {
            scheme: "native".into(),
            raw_value: spec.native_document_id.into(),
            normalized_value: Some(spec.native_document_id.into()),
        }],
        relationships: spec.relationships,
    }
}

fn recorded_units() -> Vec<WireEvidenceUnit> {
    vec![
        build_wire_unit(UnitSpec {
            unit_id: USPSTF_UNIT_ID,
            version_id: "11111111-1111-4111-8111-000000000001",
            source_code: "uspstf",
            source_display_name: "US Preventive Services Task Force",
            native_document_id: "uspstf-crc-2025",
            title: "Screening for Colorectal Cancer: US Preventive Services Task Force Recommendation Statement",
            label: Some("Recommendation Summary"),
            publication_date: Some("2025-05-20"),
            lifecycle_state: WireLifecycleState::Current,
            source_status_code: None,
            attribution_text: Some(
                "US Preventive Services Task Force; JAMA 2025; public domain",
            ),
            license_identifier: Some("CC0-1.0"),
            content: "The USPSTF recommends screening for colorectal cancer in all adults aged 45 to 75 years (A recommendation).",
            relationships: vec![],
            bibliographic_observations: None,
        }),
        build_wire_unit(UnitSpec {
            unit_id: DAILYMED_UNIT_ID,
            version_id: "22222222-2222-4222-8222-000000000002",
            source_code: "dailymed",
            source_display_name: "DailyMed",
            native_document_id: "setid-88a1bc02-dapagliflozin",
            title: "Dapagliflozin Film-Coated Tablets Prescribing Information",
            label: Some("2.2 Recommended Dosage in Chronic Kidney Disease"),
            publication_date: Some("2025-11-03"),
            lifecycle_state: WireLifecycleState::Corrected,
            source_status_code: Some(
                "Label erratum 2025-11-03: clarified initiation eGFR threshold of 25 mL/min/1.73 m2 for chronic kidney disease.",
            ),
            attribution_text: None,
            license_identifier: Some("public-domain"),
            content: "For chronic kidney disease, initiate dapagliflozin 10 mg orally once daily in adults with eGFR at least 25 mL/min/1.73 m2; if eGFR falls below 25 mL/min/1.73 m2 during treatment, continuation of 10 mg once daily is supported to reduce eGFR decline and heart failure hospitalization.",
            relationships: vec![WireRelationship {
                relation_type: WireRelationshipType::IsCorrectedBy,
                target_version_id: None,
                target_scheme: Some("spl-set-id".into()),
                target_value: Some("setid-88a1bc02-erratum-2025".into()),
                source_relation_code: "LabelErratum".into(),
                native_destination_uri: None,
            }],
            bibliographic_observations: None,
        }),
        build_wire_unit(UnitSpec {
            unit_id: PMC_RETRACTED_UNIT_ID,
            version_id: "33333333-3333-4333-8333-000000000003",
            source_code: "pmc",
            source_display_name: "PubMed Central Open Access",
            native_document_id: "PMC9981042",
            title: "High-Dose Potassium Binder Monotherapy in Acute Hyperkalemia",
            label: Some("Abstract"),
            publication_date: Some("2024-08-14"),
            lifecycle_state: WireLifecycleState::Retracted,
            source_status_code: None,
            attribution_text: Some("Patel R et al., J Acute Med 2024; CC BY 4.0"),
            license_identifier: Some("CC-BY-4.0"),
            content: "Single-agent binder monotherapy normalized serum potassium within 45 minutes without insulin-dextrose.",
            relationships: vec![WireRelationship {
                relation_type: WireRelationshipType::IsRetractedBy,
                target_version_id: None,
                target_scheme: Some("pmid".into()),
                target_value: Some("39200418".into()),
                source_relation_code: "RetractedIn".into(),
                native_destination_uri: None,
            }],
            bibliographic_observations: Some(vec![WireBibliographicObservation {
                subject_pmid: "39110200".into(),
                source_record_date: Some("2025-01-12".into()),
                publication_types: vec!["Retracted Publication".into()],
                relationships: vec![WireBibliographicRelationship {
                    source_relation_code: "RetractedIn".into(),
                    target_pmid: Some("39200418".into()),
                    source_reference: Some("J Acute Med. 2025;14(1):88.".into()),
                    note: Some(
                        "Primary electrolyte endpoint tables could not be verified from trial source data.".into(),
                    ),
                }],
            }]),
        }),
        build_wire_unit(UnitSpec {
            unit_id: MMWR_UNIT_ID,
            version_id: "44444444-4444-4444-8444-000000000004",
            source_code: "cdc-mmwr",
            source_display_name: "CDC Morbidity and Mortality Weekly Report",
            native_document_id: "mmwr-rr7402a1",
            title: "Prevention and Control of Seasonal Influenza with Vaccines",
            label: Some("Immunocompromised Adults"),
            publication_date: Some("2025-08-29"),
            lifecycle_state: WireLifecycleState::Current,
            source_status_code: None,
            attribution_text: None,
            license_identifier: None,
            content: "Adults receiving immunosuppressive therapy should receive age-appropriate inactivated or recombinant influenza vaccine rather than live attenuated influenza vaccine.",
            relationships: vec![],
            bibliographic_observations: None,
        }),
    ]
}

fn neutral_error(code: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": {
                "code": code,
                "message": "Clinical evidence request is invalid",
                "retryable": false
            }
        })),
    )
        .into_response()
}

fn validate_strict_search_body(raw: &Value) -> Option<String> {
    let obj = raw.as_object()?;
    let expected_keys: BTreeSet<&str> = [
        "mode",
        "query",
        "scheme",
        "value",
        "candidate_id",
        "relationship",
        "cursor",
    ]
    .into_iter()
    .collect();
    let actual_keys: BTreeSet<&str> = obj.keys().map(String::as_str).collect();
    if actual_keys != expected_keys {
        return None;
    }
    if obj.get("mode").and_then(Value::as_str) != Some("query") {
        return None;
    }
    let query = obj
        .get("query")
        .and_then(Value::as_str)
        .filter(|q| !q.trim().is_empty())?;
    for null_field in ["scheme", "value", "candidate_id", "relationship"] {
        if !obj.get(null_field).is_some_and(Value::is_null) {
            return None;
        }
    }
    match obj.get("cursor") {
        Some(Value::Null) => {}
        Some(Value::String(s)) if !s.is_empty() => {}
        _ => return None,
    }
    Some(query.to_owned())
}

fn validate_strict_read_body(raw: &Value) -> Option<Vec<String>> {
    let obj = raw.as_object()?;
    if obj.len() != 1 || !obj.contains_key("candidate_ids") {
        return None;
    }
    let arr = obj.get("candidate_ids").and_then(Value::as_array)?;
    if !(1..=32).contains(&arr.len()) {
        return None;
    }
    let mut ids = Vec::with_capacity(arr.len());
    for entry in arr {
        let id = entry.as_str().filter(|s| !s.is_empty())?;
        ids.push(id.to_owned());
    }
    Some(ids)
}

fn candidate_from_unit(unit: &WireEvidenceUnit) -> WireCandidate {
    WireCandidate {
        candidate_id: unit.unit_id.clone(),
        source_code: unit.source_code.clone(),
        source_display_name: unit.source_display_name.clone(),
        native_document_id: unit.native_document_id.clone(),
        title: unit.title.clone(),
    }
}

async fn search_handler(
    State(state): State<RecordedLibraryState>,
    headers: HeaderMap,
    Json(raw): Json<Value>,
) -> Response {
    state.requests.fetch_add(1, Ordering::SeqCst);
    let Some(query) = validate_strict_search_body(&raw) else {
        return neutral_error("search_unavailable");
    };

    match &state.behavior {
        ServerBehavior::ServiceUnavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": {
                    "code": "search_unavailable",
                    "message": "Clinical evidence service is unavailable",
                    "retryable": true
                }
            })),
        )
            .into_response(),
        ServerBehavior::RedirectTo { target_base } => (
            StatusCode::TEMPORARY_REDIRECT,
            [(LOCATION, format!("{target_base}/v1/search"))],
        )
            .into_response(),
        ServerBehavior::TamperedDigest { unit } => Json(json!({
            "candidates": [candidate_from_unit(unit)],
            "page": WireSearchPage {
                examined_candidate_count: 1,
                returned_candidate_count: 1,
                unavailable_candidate_count: 0,
                truncated: false,
            }
        }))
        .into_response(),
        ServerBehavior::CorruptCandidateId { unit } => {
            let mut candidate = candidate_from_unit(unit);
            candidate.candidate_id = "corrupt-non-uuid".into();
            Json(json!({
                "candidates": [candidate],
                "page": WireSearchPage {
                    examined_candidate_count: 1,
                    returned_candidate_count: 1,
                    unavailable_candidate_count: 0,
                    truncated: false,
                }
            }))
            .into_response()
        }
        ServerBehavior::TruncatedCollectionScan { unit } => Json(json!({
            "candidates": [candidate_from_unit(unit)],
            "next_cursor": "next-page-token",
            "page": WireSearchPage {
                examined_candidate_count: 25,
                returned_candidate_count: 1,
                unavailable_candidate_count: 0,
                truncated: true,
            }
        }))
        .into_response(),
        ServerBehavior::Healthy { units, auth } => {
            if !auth.allows(&headers) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "missing or invalid invoker token"})),
                )
                    .into_response();
            }
            let terms: Vec<String> = query
                .split_whitespace()
                .map(str::to_ascii_lowercase)
                .collect();
            let candidates: Vec<WireCandidate> = units
                .iter()
                .filter(|unit| {
                    let haystack = format!(
                        "{} {} {}",
                        unit.title.to_ascii_lowercase(),
                        unit.label
                            .as_deref()
                            .unwrap_or_default()
                            .to_ascii_lowercase(),
                        unit.content.to_ascii_lowercase()
                    );
                    terms.iter().any(|term| haystack.contains(term))
                })
                .map(candidate_from_unit)
                .collect();
            let count = candidates.len();
            Json(json!({
                "candidates": candidates,
                "page": {
                    "examined_candidate_count": units.len(),
                    "returned_candidate_count": count,
                    "unavailable_candidate_count": 0,
                    "truncated": false
                }
            }))
            .into_response()
        }
    }
}

async fn read_handler(
    State(state): State<RecordedLibraryState>,
    headers: HeaderMap,
    Json(raw): Json<Value>,
) -> Response {
    state.requests.fetch_add(1, Ordering::SeqCst);
    let Some(candidate_ids) = validate_strict_read_body(&raw) else {
        return neutral_error("read_unavailable");
    };

    match &state.behavior {
        ServerBehavior::ServiceUnavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": {
                    "code": "read_unavailable",
                    "message": "Clinical evidence service is unavailable",
                    "retryable": true
                }
            })),
        )
            .into_response(),
        ServerBehavior::RedirectTo { target_base } => (
            StatusCode::TEMPORARY_REDIRECT,
            [(LOCATION, format!("{target_base}/v1/read"))],
        )
            .into_response(),
        ServerBehavior::TamperedDigest { unit } => {
            let mut corrupted = (**unit).clone();
            corrupted.content_sha256 = "0".repeat(64);
            let results: Vec<WireReadItem> = candidate_ids
                .into_iter()
                .map(|id| WireReadItem::Available {
                    candidate_id: id,
                    unit: Box::new(corrupted.clone()),
                })
                .collect();
            Json(json!({ "results": results })).into_response()
        }
        ServerBehavior::CorruptCandidateId { .. }
        | ServerBehavior::TruncatedCollectionScan { .. } => {
            let results: Vec<WireReadItem> = candidate_ids
                .into_iter()
                .map(|id| WireReadItem::Unavailable {
                    candidate_id: id,
                    cause: WireReadUnavailableCause::InvalidCandidateId,
                })
                .collect();
            Json(json!({ "results": results })).into_response()
        }
        ServerBehavior::Healthy { units, auth } => {
            if !auth.allows(&headers) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "missing or invalid invoker token"})),
                )
                    .into_response();
            }
            let mut results = Vec::with_capacity(candidate_ids.len());
            for id in candidate_ids {
                if uuid::Uuid::parse_str(&id).is_err() {
                    results.push(json!({
                        "candidate_id": id,
                        "outcome": "unavailable",
                        "cause": WireReadUnavailableCause::InvalidCandidateId
                    }));
                    continue;
                }
                match units.iter().find(|unit| unit.unit_id == id) {
                    Some(unit) => {
                        // Include an optional `visual` payload on one unit to verify
                        // forward-compatible wire deserialization without deny_unknown_fields.
                        let mut unit_json = serde_json::to_value(unit).unwrap();
                        if unit.unit_id == USPSTF_UNIT_ID {
                            unit_json["visual"] = json!({
                                "media_type": "image/png",
                                "caption": "Summary recommendation table"
                            });
                        }
                        results.push(json!({
                            "candidate_id": id,
                            "outcome": "available",
                            "unit": unit_json
                        }));
                    }
                    None => {
                        results.push(json!({
                            "candidate_id": id,
                            "outcome": "unavailable",
                            "cause": WireReadUnavailableCause::NotFoundOrNotAuthorized
                        }));
                    }
                }
            }
            Json(json!({ "results": results })).into_response()
        }
    }
}

async fn not_found_handler() -> Response {
    (StatusCode::NOT_FOUND, Json(json!({ "error": "not found" }))).into_response()
}

async fn spawn_library_server(behavior: ServerBehavior) -> (String, Arc<AtomicUsize>) {
    let requests = Arc::new(AtomicUsize::new(0));
    let state = RecordedLibraryState {
        behavior,
        requests: Arc::clone(&requests),
    };
    let app = Router::new()
        .route("/v1/search", post(search_handler))
        .route("/v1/read", post(read_handler))
        .fallback(not_found_handler)
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
        units: recorded_units(),
        auth: BearerPolicy::Exact("test-invoker-token".into()),
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

    // Clinical concept query hits `/v1/search` + `/v1/read` and returns current, corrected,
    // and retracted passages with full metadata, attribution fallback, and status notices.
    let engine::ToolOutcome::Returned(payload) = client
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "colorectal dapagliflozin hyperkalemia influenza",
                "collection": null,
                "limit": 10
            }),
        )
        .await
    else {
        panic!("expected Returned outcome from search_clinical_library");
    };
    let parsed: LibrarySearchOutcome = serde_json::from_str(&payload).unwrap();
    let LibrarySearchOutcome::Results { passages, .. } = parsed else {
        panic!("expected search results, got {parsed:?}");
    };
    assert_eq!(passages.len(), 4);
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
        PublicationStatus::Corrected { notice }
            if notice.contains("eGFR threshold of 25") && notice.contains("LabelErratum")
    )));
    assert!(passages.iter().any(|p| matches!(
        &p.status,
        PublicationStatus::Retracted { notice }
            if notice.contains("39200418") && notice.contains("endpoint tables could not be verified")
    )));

    // Client-side collection filter maps `cdc-mmwr` source_code to `LibraryCollection::Mmwr`
    // without sending non-schema keys on `POST /v1/search`.
    let engine::ToolOutcome::Returned(mmwr_payload) = client
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "influenza vaccine immunosuppressive",
                "collection": "mmwr",
                "limit": 2
            }),
        )
        .await
    else {
        panic!("expected Returned outcome for mmwr filter");
    };
    let LibrarySearchOutcome::Results {
        passages: mmwr_passages,
        ..
    } = serde_json::from_str(&mmwr_payload).unwrap()
    else {
        panic!("expected mmwr search results");
    };
    assert_eq!(mmwr_passages.len(), 1);
    assert_eq!(mmwr_passages[0].source.collection, LibraryCollection::Mmwr);
    assert_eq!(mmwr_passages[0].passage_id.as_str(), MMWR_UNIT_ID);
}

#[tokio::test]
async fn exact_read_returns_identical_passage_and_reports_not_found_when_absent() {
    let (base_url, _) = spawn_library_server(ServerBehavior::Healthy {
        units: recorded_units(),
        auth: BearerPolicy::Open,
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
    let LibrarySearchOutcome::Results { passages, .. } =
        serde_json::from_str(&search_json).unwrap()
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
        let LibraryReadOutcome::Passage { passage, .. } = serde_json::from_str(&read_json).unwrap()
        else {
            panic!("expected exact passage for {}", searched.passage_id);
        };
        assert_eq!(passage.as_ref(), searched);
    }

    // Absent UUID (`not_found_or_not_authorized`) and non-UUID (`invalid_candidate_id`) both map cleanly to `NotFound`.
    for absent_id in [MISSING_UNIT_ID, "non-uuid-handle"] {
        let engine::ToolOutcome::Returned(missing_json) = client
            .execute(
                READ_LIBRARY_TOOL,
                &json!({
                    "passage_id": absent_id
                }),
            )
            .await
        else {
            panic!("expected Returned outcome for missing passage {absent_id}");
        };
        let missing: LibraryReadOutcome = serde_json::from_str(&missing_json).unwrap();
        assert_eq!(
            missing,
            LibraryReadOutcome::NotFound {
                passage_id: PassageId::new(absent_id).unwrap(),
            }
        );
    }
}

#[tokio::test]
async fn unavailable_library_differs_from_nothing_found_across_failure_modes() {
    let (healthy_url, _) = spawn_library_server(ServerBehavior::Healthy {
        units: recorded_units(),
        auth: BearerPolicy::Exact("expected-secret".into()),
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
    let (redirect_url, _) = spawn_library_server(ServerBehavior::RedirectTo {
        target_base: healthy_url.clone(),
    })
    .await;
    let (tampered_url, _) = spawn_library_server(ServerBehavior::TamperedDigest {
        unit: Box::new(recorded_units().remove(0)),
    })
    .await;
    let (corrupt_id_url, _) = spawn_library_server(ServerBehavior::CorruptCandidateId {
        unit: Box::new(recorded_units().remove(0)),
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
            "http_307_redirect_not_followed",
            ClinicalLibraryClient::http(&redirect_url)
                .unwrap()
                .with_bearer_token("expected-secret"),
        ),
        (
            "http_404_wrong_base_path",
            ClinicalLibraryClient::http(format!("{healthy_url}/wrong-prefix"))
                .unwrap()
                .with_bearer_token("expected-secret"),
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
                    "passage_id": USPSTF_UNIT_ID
                }),
            )
            .await
        else {
            panic!("{label}: expected Returned unavailable outcome on read");
        };
        let read_val: Value = serde_json::from_str(&read_raw).unwrap();
        assert_eq!(read_val["outcome"], "unavailable", "{label}");
    }

    // Search hydration receiving `invalid_candidate_id` for a server-returned candidate
    // surfaces `Unavailable` rather than silently coercing into empty `"results"`.
    let corrupt_client = ClinicalLibraryClient::http(&corrupt_id_url).unwrap();
    let engine::ToolOutcome::Returned(corrupt_search_raw) = corrupt_client
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "colorectal screening",
                "collection": null,
                "limit": 5
            }),
        )
        .await
    else {
        panic!("expected Returned outcome on corrupt candidate_id search");
    };
    let corrupt_search_val: Value = serde_json::from_str(&corrupt_search_raw).unwrap();
    assert_eq!(corrupt_search_val["outcome"], "unavailable");
    assert_eq!(
        corrupt_search_val["reason"],
        "clinical_library_invalid_candidate_id"
    );
}

#[derive(Clone)]
enum MetadataBehavior {
    Healthy {
        expected_audience: String,
        header_b64: String,
        payload_b64: String,
    },
    Http500,
    MalformedToken,
    OversizedToken,
}

#[derive(Clone)]
struct MockMetadataState {
    behavior: MetadataBehavior,
    requests: Arc<AtomicUsize>,
}

async fn metadata_identity_handler(
    State(state): State<MockMetadataState>,
    headers: HeaderMap,
    Query(params): Query<BTreeMap<String, String>>,
) -> Response {
    let count = state.requests.fetch_add(1, Ordering::SeqCst) + 1;
    if headers
        .get("Metadata-Flavor")
        .and_then(|value| value.to_str().ok())
        != Some("Google")
    {
        return (StatusCode::FORBIDDEN, "missing Metadata-Flavor: Google").into_response();
    }
    if params.get("format").map(String::as_str) != Some("full") {
        return (StatusCode::BAD_REQUEST, "expected format=full").into_response();
    }

    match &state.behavior {
        MetadataBehavior::Healthy {
            expected_audience,
            header_b64,
            payload_b64,
        } => {
            if params.get("audience") != Some(expected_audience) {
                return (StatusCode::BAD_REQUEST, "unexpected audience").into_response();
            }
            let jwt = format!("{header_b64}.{payload_b64}.sig-{count}");
            (StatusCode::OK, jwt).into_response()
        }
        MetadataBehavior::Http500 => {
            (StatusCode::INTERNAL_SERVER_ERROR, "metadata unavailable").into_response()
        }
        MetadataBehavior::MalformedToken => {
            (StatusCode::OK, "not-a-three-part-jwt").into_response()
        }
        MetadataBehavior::OversizedToken => (StatusCode::OK, "a".repeat(20_000)).into_response(),
    }
}

async fn spawn_metadata_server(behavior: MetadataBehavior) -> (String, Arc<AtomicUsize>) {
    let requests = Arc::new(AtomicUsize::new(0));
    let state = MockMetadataState {
        behavior,
        requests: Arc::clone(&requests),
    };
    let app = Router::new()
        .route(
            "/computeMetadata/v1/instance/service-accounts/default/identity",
            get(metadata_identity_handler),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    (
        format!("http://{address}/computeMetadata/v1/instance/service-accounts/default/identity"),
        requests,
    )
}

#[tokio::test]
async fn google_metadata_oidc_mints_per_call_jwt_and_surfaces_metadata_failures_as_unavailable() {
    let audience = "https://coda-clinical-evidence-stg.a.run.app";
    let header_b64 = URL_SAFE_NO_PAD.encode(r#"{"alg":"RS256","typ":"JWT"}"#);
    let payload_b64 = URL_SAFE_NO_PAD.encode(format!(r#"{{"aud":"{audience}"}}"#));
    let expected_prefix = format!("{header_b64}.{payload_b64}.sig-");

    // Production constructor enforces `https://` when using Google OIDC.
    assert!(ClinicalLibraryClient::google_oidc("http://127.0.0.1:8080", audience).is_err());

    let (library_url, library_requests) = spawn_library_server(ServerBehavior::Healthy {
        units: recorded_units(),
        auth: BearerPolicy::Prefix(expected_prefix),
    })
    .await;
    let (metadata_url, metadata_requests) = spawn_metadata_server(MetadataBehavior::Healthy {
        expected_audience: audience.into(),
        header_b64,
        payload_b64,
    })
    .await;

    let oidc_client =
        ClinicalLibraryClient::loopback_google_oidc(&library_url, &metadata_url, audience).unwrap();

    // `search_clinical_library` mints fresh OIDC tokens for `/v1/search`, `/v1/read`, and `/v1/freshness`.
    let engine::ToolOutcome::Returned(search_raw) = oidc_client
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "colorectal dapagliflozin hyperkalemia influenza",
                "collection": null,
                "limit": 10
            }),
        )
        .await
    else {
        panic!("expected Returned outcome from OIDC-authenticated search");
    };
    let LibrarySearchOutcome::Results { passages, .. } = serde_json::from_str(&search_raw).unwrap()
    else {
        panic!("expected OIDC search passages");
    };
    assert_eq!(passages.len(), 4);
    assert_eq!(metadata_requests.load(Ordering::SeqCst), 3);
    assert_eq!(library_requests.load(Ordering::SeqCst), 2);

    // `read_clinical_library` mints a fresh OIDC token for `/v1/read`.
    let engine::ToolOutcome::Returned(read_raw) = oidc_client
        .execute(
            READ_LIBRARY_TOOL,
            &json!({
                "passage_id": USPSTF_UNIT_ID
            }),
        )
        .await
    else {
        panic!("expected Returned outcome from OIDC-authenticated read");
    };
    let LibraryReadOutcome::Passage { passage, .. } = serde_json::from_str(&read_raw).unwrap()
    else {
        panic!("expected OIDC read passage");
    };
    assert_eq!(passage.passage_id.as_str(), USPSTF_UNIT_ID);
    assert_eq!(metadata_requests.load(Ordering::SeqCst), 4);
    assert_eq!(library_requests.load(Ordering::SeqCst), 3);

    // Metadata failures (HTTP 500, malformed token, oversized token, unreachable metadata host)
    // surface cleanly as `Unavailable` without forwarding unauthenticated requests to the library.
    let closed_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let closed_meta_addr = closed_listener.local_addr().unwrap();
    drop(closed_listener);
    let unreachable_meta_url = format!(
        "http://{closed_meta_addr}/computeMetadata/v1/instance/service-accounts/default/identity"
    );

    let (meta_500_url, _) = spawn_metadata_server(MetadataBehavior::Http500).await;
    let (meta_malformed_url, _) = spawn_metadata_server(MetadataBehavior::MalformedToken).await;
    let (meta_oversized_url, _) = spawn_metadata_server(MetadataBehavior::OversizedToken).await;

    for (label, bad_meta_url) in [
        ("metadata_http_500", meta_500_url),
        ("metadata_malformed_jwt", meta_malformed_url),
        ("metadata_oversized_jwt", meta_oversized_url),
        ("metadata_unreachable", unreachable_meta_url),
    ] {
        let failing_client =
            ClinicalLibraryClient::loopback_google_oidc(&library_url, &bad_meta_url, audience)
                .unwrap();

        let engine::ToolOutcome::Returned(search_fail_raw) = failing_client
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
            panic!("{label}: expected Returned unavailable outcome on search");
        };
        let search_fail_val: Value = serde_json::from_str(&search_fail_raw).unwrap();
        assert_eq!(search_fail_val["outcome"], "unavailable", "{label}");

        let engine::ToolOutcome::Returned(read_fail_raw) = failing_client
            .execute(
                READ_LIBRARY_TOOL,
                &json!({
                    "passage_id": USPSTF_UNIT_ID
                }),
            )
            .await
        else {
            panic!("{label}: expected Returned unavailable outcome on read");
        };
        let read_fail_val: Value = serde_json::from_str(&read_fail_raw).unwrap();
        assert_eq!(read_fail_val["outcome"], "unavailable", "{label}");
    }

    assert_eq!(
        library_requests.load(Ordering::SeqCst),
        3,
        "failed metadata token minting must never send unauthenticated requests to the library"
    );
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
        envelope: None,
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
                    let cite_as = chart_json["results"][0]["cite_as"]
                        .as_str()
                        .expect("labelled chart read cite_as");
                    let answer = format!(
                        "On the 2026-11-10 metabolic observation (obs-renal-2026-11-10), eGFR is 32 mL/min/1.73 m2 and potassium is 4.6 mmol/L [{cite_as}]."
                    );
                    let basis = format!(
                        "Grounded in the 2026-11-10 clinic EHR Observation record [{cite_as}] and general medical knowledge; the clinical library was unavailable during this turn, so current DailyMed label thresholds could not be verified directly."
                    );

                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "finish-renal-1".into(),
                            name: FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": answer,
                                "quotes": [{"label": cite_as, "quote": "\"eGFR_mL_min\":32"}],
                                "descriptions": [],
                                "basis": basis,
                                "cites_none": false
                            })
                            .to_string(),
                        })],
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
            None,
        )
        .await
        .unwrap();

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowedAuthority,
        shown_steps: ShownSteps::new(()),
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
    let finished = saved
        .messages
        .iter()
        .find_map(|msg| match &msg.role {
            MessageRole::Assistant {
                kind: AssistantKind::Finished(finished),
                ..
            } => Some(finished),
            _ => None,
        })
        .expect("finished citation record");
    assert_eq!(finished.marks().len(), 1);
    assert!(
        finished
            .basis()
            .text()
            .contains("clinical library was unavailable")
    );
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
                    let LibrarySearchOutcome::Results { passages, .. } =
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
                    let LibraryReadOutcome::Passage { passage, .. } =
                        serde_json::from_str(output).unwrap()
                    else {
                        panic!("expected exact passage");
                    };
                    let answer = format!(
                        "Dapagliflozin 10 mg once daily may be initiated in chronic kidney disease when eGFR is at least 25 mL/min/1.73 m2 [{} — {}, {}, {}]. Note published label correction: {}",
                        passage.passage_id,
                        passage.source.title,
                        passage.source.document_id,
                        passage.date,
                        match &passage.status {
                            PublicationStatus::Corrected { notice } => notice.as_str(),
                            _ => "",
                        },
                    );
                    let basis = format!(
                        "DailyMed passage {} ({}, {}).",
                        passage.passage_id, passage.source.document_id, passage.date,
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "finish-dapa-1".into(),
                            name: FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": answer,
                                "quotes": [],
                                "descriptions": [],
                                "basis": basis,
                                "cites_none": true
                            })
                            .to_string(),
                        })],
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
        units: recorded_units(),
        auth: BearerPolicy::Open,
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
            None,
        )
        .await
        .unwrap();

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowedAuthority,
        shown_steps: ShownSteps::new(()),
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
    assert_eq!(cited_id, DAILYMED_UNIT_ID);
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
    let LibraryReadOutcome::Passage { passage, .. } = reopened else {
        panic!("expected cited passage to reopen");
    };
    assert_eq!(passage.source.document_id, "setid-88a1bc02-dapagliflozin");
    assert_eq!(passage.date, "2025-11-03");
}

#[tokio::test]
async fn phi_phone_iso_date_notice_relations_and_truncated_scan_behave_closed() {
    let notice_unit_id = "55555555-5555-4555-8555-555555555505";
    let notice_unit = build_wire_unit(UnitSpec {
        unit_id: notice_unit_id,
        version_id: "55555555-5555-4555-8555-000000000005",
        source_code: "nlm-pubmed",
        source_display_name: "PubMed",
        native_document_id: "PMID:39555001",
        title: "Retraction Note for Early Potassium Trial",
        label: Some("Notice"),
        publication_date: Some("2025-05-01"),
        lifecycle_state: WireLifecycleState::Current,
        source_status_code: None,
        attribution_text: None,
        license_identifier: None,
        content: "This notice retracts an earlier trial report.",
        relationships: vec![WireRelationship {
            relation_type: WireRelationshipType::Retracts,
            target_version_id: None,
            target_scheme: Some("pmid".into()),
            target_value: Some("39110200".into()),
            source_relation_code: "RetractionOf".into(),
            native_destination_uri: None,
        }],
        bibliographic_observations: Some(vec![WireBibliographicObservation {
            subject_pmid: "39555001".into(),
            source_record_date: Some("2025-05-01".into()),
            publication_types: vec![
                "Retraction of Publication".into(),
                "Published Erratum".into(),
                "Expression of Concern".into(),
            ],
            relationships: vec![
                WireBibliographicRelationship {
                    source_relation_code: "RetractionOf".into(),
                    target_pmid: Some("39110200".into()),
                    source_reference: None,
                    note: None,
                },
                WireBibliographicRelationship {
                    source_relation_code: "ErratumFor".into(),
                    target_pmid: Some("39110200".into()),
                    source_reference: None,
                    note: None,
                },
                WireBibliographicRelationship {
                    source_relation_code: "ExpressionOfConcernFor".into(),
                    target_pmid: Some("39110200".into()),
                    source_reference: None,
                    note: None,
                },
            ],
        }]),
    });

    let (healthy_url, requests) = spawn_library_server(ServerBehavior::Healthy {
        units: vec![notice_unit.clone()],
        auth: BearerPolicy::Open,
    })
    .await;
    let client = ClinicalLibraryClient::http(&healthy_url).unwrap();

    // 1. Phone numbers and ISO dates in query are refused before any outbound HTTP call.
    for phi_query in [
        "potassium callback (555) 867-5309",
        "potassium callback 555-867-5309",
        "potassium callback 555.867.5309",
        "metformin titration 1979-04-12",
    ] {
        let outcome = client
            .execute(
                SEARCH_LIBRARY_TOOL,
                &json!({
                    "query": phi_query,
                    "collection": null,
                    "limit": 3
                }),
            )
            .await;
        assert!(
            matches!(outcome, engine::ToolOutcome::Refused(_)),
            "expected `{phi_query}` to be refused at tool boundary"
        );
    }
    assert_eq!(requests.load(Ordering::SeqCst), 0);

    // 2. Notice-side `RetractionOf` / `ErratumFor` / `ExpressionOfConcernFor` does not mark the notice itself retracted.
    let read_notice = client
        .read_passage(&PassageId::new(notice_unit_id).unwrap())
        .await;
    let LibraryReadOutcome::Passage { passage, .. } = read_notice else {
        panic!("expected notice passage to read cleanly");
    };
    assert_eq!(passage.status, PublicationStatus::Current);

    // 3. Collection-filtered scan hitting MAX_SEARCH_PAGES while still truncated returns Unavailable, not silent empty Results.
    let (trunc_url, trunc_requests) =
        spawn_library_server(ServerBehavior::TruncatedCollectionScan {
            unit: Box::new(notice_unit),
        })
        .await;
    let trunc_client = ClinicalLibraryClient::http(&trunc_url).unwrap();
    let query = local_product::clinical_library::ConceptQuery::parse("potassium").unwrap();
    let scan_outcome = trunc_client
        .search(&query, Some(LibraryCollection::Uspstf), 3)
        .await;
    assert_eq!(trunc_requests.load(Ordering::SeqCst), 4);
    assert_eq!(
        scan_outcome,
        LibrarySearchOutcome::Unavailable {
            reason: "clinical_library_collection_scan_truncated".into(),
        }
    );
}

#[tokio::test]
async fn freshness_endpoint_reports_honest_standing_or_unavailable() {
    use local_product::clinical_library::{
        LibraryFreshnessOutcome, WireLibraryStanding, WireSourceFreshnessState,
    };

    // Unconfigured client never claims the library is current.
    let unconfigured = ClinicalLibraryClient::unconfigured();
    assert!(matches!(
        unconfigured.freshness().await,
        LibraryFreshnessOutcome::Unavailable { .. }
    ));

    // HTTP 404 or 500 surfaces as Unavailable.
    let err_app = axum::Router::new().route(
        "/v1/freshness",
        axum::routing::get(|| async { axum::http::StatusCode::SERVICE_UNAVAILABLE }),
    );
    let err_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let err_addr = err_listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(err_listener, err_app).await.unwrap();
    });
    let err_client = ClinicalLibraryClient::http(format!("http://{err_addr}")).unwrap();
    assert!(matches!(
        err_client.freshness().await,
        LibraryFreshnessOutcome::Unavailable { .. }
    ));

    // Stale report parses cleanly with named reason and NeverRefreshed/Lagging sources.
    let stale_app = axum::Router::new().route(
        "/v1/freshness",
        axum::routing::get(|| async {
            axum::Json(json!({
                "standing": "stale",
                "reason": "DailyMed has not completed refresh",
                "active_version": null,
                "last_night_recorded_at": null,
                "last_night_outcome": null,
                "sources": [
                    {
                        "source_code": "dailymed",
                        "source_display_name": "DailyMed",
                        "status": { "state": "never_refreshed" },
                        "admitted_count": 0
                    }
                ],
                "open_alerts": 0,
                "search_latency": {
                    "endpoint": "search",
                    "sample_count": 0,
                    "p50_ms": 0.0,
                    "p95_ms": 0.0,
                    "p99_ms": 0.0,
                    "within_targets": null
                }
            }))
        }),
    );
    let stale_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let stale_addr = stale_listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(stale_listener, stale_app).await.unwrap();
    });
    let stale_client = ClinicalLibraryClient::http(format!("http://{stale_addr}")).unwrap();
    let LibraryFreshnessOutcome::Report(report) = stale_client.freshness().await else {
        panic!("expected parsed freshness report");
    };
    assert!(matches!(
        report.standing,
        WireLibraryStanding::Stale { ref reason } if reason.contains("DailyMed")
    ));
    assert!(matches!(
        report.sources[0].status,
        WireSourceFreshnessState::NeverRefreshed
    ));
}

#[tokio::test]
async fn search_clinical_library_attaches_live_freshness_standing() {
    let state = RecordedLibraryState {
        behavior: ServerBehavior::Healthy {
            units: recorded_units(),
            auth: BearerPolicy::Open,
        },
        requests: Arc::new(AtomicUsize::new(0)),
    };
    let stale_app = Router::new()
        .route("/v1/search", post(search_handler))
        .route("/v1/read", post(read_handler))
        .route(
            "/v1/freshness",
            axum::routing::get(|| async {
                Json(json!({
                    "standing": "stale",
                    "reason": "DailyMed exceeded 3-day SLA",
                    "active_version": "v-2026-11-15",
                    "sources": [
                        {
                            "source_code": "dailymed",
                            "source_display_name": "DailyMed",
                            "status": { "state": "never_refreshed" },
                            "admitted_count": 0
                        }
                    ],
                    "open_alerts": 2,
                    "search_latency": {
                        "endpoint": "search",
                        "sample_count": 1,
                        "p50_ms": 10.0,
                        "p95_ms": 10.0,
                        "p99_ms": 10.0,
                        "within_targets": true
                    }
                }))
            }),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, stale_app).await.unwrap();
    });
    let client = ClinicalLibraryClient::http(format!("http://{addr}")).unwrap();

    let engine::ToolOutcome::Returned(raw) = client
        .execute(
            SEARCH_LIBRARY_TOOL,
            &json!({
                "query": "dapagliflozin",
                "collection": null,
                "limit": 5
            }),
        )
        .await
    else {
        panic!("expected Returned outcome from search_clinical_library");
    };
    let parsed: LibrarySearchOutcome = serde_json::from_str(&raw).unwrap();
    let LibrarySearchOutcome::Results {
        passages,
        freshness,
    } = parsed
    else {
        panic!("expected search Results, got {parsed:?}");
    };
    assert!(!passages.is_empty());
    assert_eq!(
        freshness,
        Some(LibrarySearchFreshness::Stale {
            reason: "DailyMed exceeded 3-day SLA".into(),
            open_alerts: 2,
            stale_sources: vec!["dailymed".into()],
        })
    );
}
