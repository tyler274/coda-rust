//! Focused wire contract test: synthetic HTTP service, not durable backend/end-to-end qualification.
use axum::{
    Router,
    body::Bytes,
    extract::{OriginalUri, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
};
use duet_client::{DuetClient, doctor::*};
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct TestState {
    failures: Arc<Mutex<bool>>,
    device: Arc<Mutex<serde_json::Value>>,
    device_race: Arc<Mutex<bool>>,
    device_puts: Arc<Mutex<usize>>,
}
async fn handle(
    State(s): State<TestState>,
    OriginalUri(uri): OriginalUri,
    method: Method,
    h: HeaderMap,
    body: Bytes,
) -> Response {
    assert_eq!(h["authorization"], "Bearer synthetic-test-credential");
    assert_eq!(h["cache-control"], "no-store");
    let path = uri.path();
    if path == "/api/health-context" {
        return axum::Json(s.device.lock().unwrap().clone()).into_response();
    }
    if method == Method::PUT && path.starts_with("/api/doctor-collection/facts/") {
        assert!(body.is_empty());
        *s.device_puts.lock().unwrap() += 1;
        let mut page = serde_json::to_value(duet_wire::models::DoctorCollection::new(8)).unwrap();
        let mut item =
            serde_json::to_value(duet_wire::models::DoctorCollectionItem::default()).unwrap();
        let snapshot = s.device.lock().unwrap().clone();
        item["fact_snapshot_id"] = if *s.device_race.lock().unwrap() {
            serde_json::json!(uuid::Uuid::new_v4())
        } else {
            snapshot["snapshot_id"].clone()
        };
        item["fact_snapshot"] = snapshot["facts"][0].clone();
        page["items"] = serde_json::json!([item]);
        return axum::Json(page).into_response();
    }
    if method == Method::DELETE {
        uuid::Uuid::parse_str(h["idempotency-key"].to_str().unwrap()).unwrap();
        if path == "/api/doctor-collection" {
            assert_eq!(h["if-match"], "7");
        }
        if std::mem::take(&mut *s.failures.lock().unwrap()) {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "private server prose must not escape",
            )
                .into_response();
        }
        return StatusCode::NO_CONTENT.into_response();
    }
    if path.ends_with("/save-receipts/99") {
        return StatusCode::NOT_FOUND.into_response();
    }
    axum::Json(duet_wire::models::DoctorCollection::new(8)).into_response()
}
#[tokio::test]
async fn authenticated_contracts_exact_retry_and_redacted_errors() {
    let s = TestState {
        failures: Arc::new(Mutex::new(true)),
        device: Arc::new(Mutex::new(
            serde_json::json!({"snapshot_id":uuid::Uuid::new_v4(),"generated_at":"2026-10-20T00:00:00Z","facts":[serde_json::to_value(duet_wire::models::HealthFactOutput::default()).unwrap()]}),
        )),
        device_race: Default::default(),
        device_puts: Default::default(),
    };
    s.device.lock().unwrap()["facts"][0]["id"] = serde_json::json!("fact /?#");
    let router = Router::new().fallback(any(handle)).with_state(s.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    let client = DuetClient::new(
        format!("http://{addr}"),
        "synthetic-test-credential",
        "0",
        "test",
    )
    .unwrap();
    let clear_op = uuid::Uuid::new_v4();
    let error = client
        .clear_doctor_workspace(clear_op, 7)
        .await
        .unwrap_err();
    assert_eq!(error, DoctorError::Uncertain);
    assert!(!error.to_string().contains("private"));
    client.clear_doctor_workspace(clear_op, 7).await.unwrap();
    assert_eq!(
        client.doctor_save_receipt(7).await.unwrap().note_generation,
        8
    );
    assert_eq!(
        client.doctor_save_receipt(99).await.unwrap_err(),
        DoctorError::NotFound
    );
    let snapshot = client.doctor_device_snapshot().await.unwrap();
    client
        .select_doctor_device_fact(&snapshot, "fact /?#")
        .await
        .unwrap();
    *s.device_race.lock().unwrap() = true;
    assert_eq!(
        client
            .select_doctor_device_fact(&snapshot, "fact /?#")
            .await
            .unwrap_err(),
        DoctorError::Conflict
    );
    s.device.lock().unwrap()["snapshot_id"] = serde_json::json!(uuid::Uuid::new_v4());
    assert_eq!(
        client
            .select_doctor_device_fact(&snapshot, "fact /?#")
            .await
            .unwrap_err(),
        DoctorError::Conflict
    );
    assert_eq!(
        *s.device_puts.lock().unwrap(),
        2,
        "changed preflight snapshot blocks dispatch"
    );
    server.abort();
}
