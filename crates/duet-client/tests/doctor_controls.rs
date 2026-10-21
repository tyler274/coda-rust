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
    commands: Arc<Mutex<Vec<(String, String, String)>>>,
    failures: Arc<Mutex<bool>>,
    support: Arc<Mutex<bool>>,
    delete_status: Arc<Mutex<StatusCode>>,
    revoke_headers: Arc<Mutex<Vec<(String, String)>>>,
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
        if path.ends_with("/share-link") {
            uuid::Uuid::parse_str(h["if-match"].to_str().unwrap()).unwrap();
            s.revoke_headers.lock().unwrap().push((
                h["idempotency-key"].to_str().unwrap().into(),
                h["if-match"].to_str().unwrap().into(),
            ));
            return (
                *s.delete_status.lock().unwrap(),
                [("Duet-Publication-Precondition", "uuid-v1")],
            )
                .into_response();
        }
        return StatusCode::NO_CONTENT.into_response();
    }
    if method == Method::POST && path.ends_with("share-link") {
        let request: PublishRequest = serde_json::from_slice(&body).unwrap();
        assert_eq!(request.expected_generation, 7);
        let text = String::from_utf8(body.to_vec()).unwrap();
        s.commands
            .lock()
            .unwrap()
            .push((method.to_string(), path.into(), text));
        if std::mem::take(&mut *s.failures.lock().unwrap()) {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "private server prose must not escape",
            )
                .into_response();
        }
        return axum::Json(serde_json::json!({"id":request.operation_id,"generation":7,"expires_at":"2026-11-01T00:00:00Z","revoked":false,"web_url":"https://example.test/shared/secret","shl_url":"https://viewer.test/#shlink:/secret-key","content_sha256":"digest"})).into_response();
    }
    if path.ends_with("/share-link") {
        let mut response = axum::Json(serde_json::Value::Null).into_response();
        if *s.support.lock().unwrap() {
            response
                .headers_mut()
                .insert("Duet-Publication-Precondition", "uuid-v1".parse().unwrap());
        }
        return response;
    }
    if path.ends_with("/save-receipts/99") {
        return StatusCode::NOT_FOUND.into_response();
    }
    if path.contains("/reject") {
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap()["expected_generation"],
            7
        );
        return (
            StatusCode::CONFLICT,
            axum::Json(serde_json::json!({"code":"stale_state","message":"secret source detail"})),
        )
            .into_response();
    }
    if path.ends_with("/pdf") {
        return (
            [("content-type", "application/pdf")],
            b"%PDF-synthetic".to_vec(),
        )
            .into_response();
    }
    if path.ends_with("/preview") {
        return ([("content-type", "image/svg+xml")], "<svg/>").into_response();
    }
    axum::Json(duet_wire::models::DoctorCollection::new(8)).into_response()
}
#[tokio::test]
async fn authenticated_contracts_exact_retry_and_redacted_errors() {
    let s = TestState {
        commands: Default::default(),
        failures: Arc::new(Mutex::new(true)),
        support: Arc::new(Mutex::new(false)),
        delete_status: Arc::new(Mutex::new(StatusCode::SERVICE_UNAVAILABLE)),
        revoke_headers: Default::default(),
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
    let request = PublishRequest {
        operation_id: uuid::Uuid::new_v4(),
        expected_generation: 7,
        expires_in_seconds: 3600,
    };
    let error = client
        .create_doctor_share_link(&request)
        .await
        .err()
        .unwrap();
    assert_eq!(error, DoctorError::Uncertain);
    assert!(!error.to_string().contains("private"));
    let publication = client.create_doctor_share_link(&request).await.unwrap();
    assert_eq!(publication.generation, 7);
    assert!(publication.shl_url.contains("#shlink:/"));
    let calls = s.commands.lock().unwrap().clone();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0], calls[1]);
    assert!(client.doctor_share_status().await.unwrap().is_none());
    let operation = uuid::Uuid::new_v4();
    let target = uuid::Uuid::new_v4();
    assert_eq!(
        client
            .revoke_doctor_share_link(operation, target)
            .await
            .unwrap_err(),
        DoctorError::UnsupportedPrecondition
    );
    assert!(
        s.revoke_headers.lock().unwrap().is_empty(),
        "legacy server must never receive destructive DELETE"
    );
    *s.support.lock().unwrap() = true;
    assert_eq!(
        client
            .revoke_doctor_share_link(operation, target)
            .await
            .unwrap_err(),
        DoctorError::Uncertain
    );
    for status in [StatusCode::CONFLICT, StatusCode::PRECONDITION_FAILED] {
        *s.delete_status.lock().unwrap() = status;
        assert_eq!(
            client
                .revoke_doctor_share_link(operation, target)
                .await
                .unwrap_err(),
            DoctorError::Conflict
        );
    }
    *s.delete_status.lock().unwrap() = StatusCode::PRECONDITION_REQUIRED;
    assert_eq!(
        client
            .revoke_doctor_share_link(operation, target)
            .await
            .unwrap_err(),
        DoctorError::UnsupportedPrecondition
    );
    *s.delete_status.lock().unwrap() = StatusCode::NO_CONTENT;
    client
        .revoke_doctor_share_link(operation, target)
        .await
        .unwrap();
    {
        let attempts = s.revoke_headers.lock().unwrap();
        assert_eq!(attempts.len(), 5);
        assert!(
            attempts
                .iter()
                .all(|h| h == &(operation.to_string(), target.to_string()))
        );
    }
    client
        .clear_doctor_workspace(uuid::Uuid::new_v4(), 7)
        .await
        .unwrap();
    assert_eq!(
        client.doctor_save_receipt(7).await.unwrap().note_generation,
        8
    );
    assert_eq!(
        client.doctor_save_receipt(99).await.unwrap_err(),
        DoctorError::NotFound
    );
    assert_eq!(
        client
            .reject_doctor_proposal(uuid::Uuid::new_v4(), 7)
            .await
            .unwrap_err(),
        DoctorError::Conflict
    );
    assert_eq!(client.doctor_pdf().await.unwrap(), b"%PDF-synthetic");
    assert_eq!(client.doctor_preview_svg().await.unwrap(), "<svg/>");
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
