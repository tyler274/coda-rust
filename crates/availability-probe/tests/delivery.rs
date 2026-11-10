use availability_probe::{
    Config, Endpoints, HEARTBEAT_METRIC, Observation, Outcome, Target,
    delivery::{Action, LEASE_SECONDS, Ledger, run_slot},
};
use axum::{
    Router,
    extract::{Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
fn sample() -> [Observation; 1] {
    [Observation {
        target: Target::Physician,
        outcome: Outcome::Timeout,
        status: None,
        latency_ms: 30,
    }]
}
#[test]
fn delayed_duplicate_overlapping_and_out_of_order_slots() {
    let mut ledger = Ledger::default();
    assert_eq!(ledger.claim(600, 650, "one").unwrap(), Action::Observe);
    assert!(ledger.claim(660, 670, "overlap").is_err());
    ledger.observed("p", &sample(), 701).unwrap();
    assert_eq!(ledger.state.minute, Some(10));
    let pending = ledger.pending.clone().unwrap();
    assert_eq!(
        pending.points[0]["points"][0]["interval"]["endTime"],
        "1970-01-01T00:10:00Z"
    );
    ledger.claim = None;
    assert_eq!(ledger.claim(600, 710, "retry").unwrap(), Action::Publish);
    assert_eq!(ledger.pending.as_ref().unwrap().points, pending.points);
    ledger.pending = None;
    ledger.claim = None;
    assert_eq!(ledger.claim(600, 710, "duplicate").unwrap(), Action::Done);
    assert_eq!(ledger.claim(540, 710, "old").unwrap(), Action::Done);
    for slot in [660, 720] {
        assert_eq!(
            ledger.claim(slot, slot + 120, "delayed").unwrap(),
            Action::Observe
        );
        ledger.observed("p", &sample(), slot + 150).unwrap();
        ledger.pending = None;
        ledger.claim = None;
    }
    assert!(ledger.state.targets[0].firing);
}
#[test]
fn abandoned_claim_is_missing_not_reobserved_and_stale_work_rejected() {
    let mut ledger = Ledger::default();
    ledger.claim(600, 600, "crashed").unwrap();
    assert_eq!(
        ledger.claim(600, 600 + LEASE_SECONDS, "duplicate").unwrap(),
        Action::Done
    );
    assert_eq!(ledger.state.minute, Some(10));
    assert_eq!(ledger.state.targets[0].failures, 0);
    assert!(ledger.claim(660, 1000, "stale").is_err());
    assert!(ledger.claim(1200, 1000, "future").is_err());
}
#[derive(Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool injects one independent fault into the fake"
)]
struct Fake {
    ledger: Option<Ledger>,
    generation: u64,
    conflict: bool,
    fail_once: bool,
    lose_response: bool,
    accepted: Vec<Value>,
    attempts: Vec<Value>,
    probes: usize,
    last_write: Option<std::time::Instant>,
    throttle_once: bool,
    throttled: usize,
}
async fn handler(State(state): State<Arc<Mutex<Fake>>>, request: Request) -> Response {
    let path = request.uri().path().to_owned();
    let query = request.uri().query().unwrap_or_default().to_owned();
    let method = request.method().clone();
    let bytes = axum::body::to_bytes(request.into_body(), 128 * 1024)
        .await
        .unwrap();
    let mut fake = state.lock().unwrap();
    match path.as_str() {
        "/metadata/token" => axum::Json(json!({"access_token": "synthetic"})).into_response(),
        "/metadata/identity" => "synthetic.id.token".into_response(),
        "/readyz" => {
            fake.probes += 1;
            StatusCode::OK.into_response()
        }
        "/storage/v1/b/test/o/state.json" => match &fake.ledger {
            Some(ledger) => (
                [("x-goog-generation", fake.generation.to_string())],
                axum::Json(serde_json::to_value(ledger).unwrap()),
            )
                .into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        "/upload/storage/v1/b/test/o" => {
            if fake.throttle_once
                || fake
                    .last_write
                    .is_some_and(|time| time.elapsed() < Duration::from_secs(1))
            {
                fake.throttle_once = false;
                fake.throttled += 1;
                return StatusCode::TOO_MANY_REQUESTS.into_response();
            }
            fake.last_write = Some(std::time::Instant::now());
            let expected = query
                .split('&')
                .find_map(|p| p.strip_prefix("ifGenerationMatch="))
                .unwrap()
                .parse::<u64>()
                .unwrap();
            if fake.conflict || expected != fake.generation {
                return StatusCode::PRECONDITION_FAILED.into_response();
            }
            fake.ledger = Some(serde_json::from_slice(&bytes).unwrap());
            fake.generation += 1;
            axum::Json(json!({"generation": fake.generation.to_string()})).into_response()
        }
        "/monitoring/p/timeSeries" if method == "GET" => {
            axum::Json(json!({"timeSeries": fake.accepted})).into_response()
        }
        "/monitoring/p/timeSeries" => {
            let body: Value = serde_json::from_slice(&bytes).unwrap();
            let point = body["timeSeries"][0].clone();
            fake.attempts.push(point.clone());
            if fake.fail_once {
                fake.fail_once = false;
                return StatusCode::SERVICE_UNAVAILABLE.into_response();
            }
            if fake.accepted.contains(&point) {
                return StatusCode::BAD_REQUEST.into_response();
            }
            fake.accepted.push(point);
            if fake.lose_response {
                fake.lose_response = false;
                return StatusCode::SERVICE_UNAVAILABLE.into_response();
            }
            StatusCode::OK.into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
async fn fixture(fake: Fake) -> (Config, Arc<Mutex<Fake>>, u64) {
    let state = Arc::new(Mutex::new(fake));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let app = Router::new().fallback(handler).with_state(state.clone());
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    (
        Config {
            project: "p".into(),
            state_bucket: Some("test".into()),
            service_account: "synthetic".into(),
            targets: vec![(Target::PhysicianWorker, origin.clone())],
            timeout: Duration::from_secs(1),
            endpoints: Endpoints {
                metadata: format!("{origin}/metadata"),
                sign_jwt: format!("{origin}/iam"),
                monitoring: format!("{origin}/monitoring"),
                storage: origin,
            },
        },
        state,
        now / 60 * 60,
    )
}
#[tokio::test]
async fn failed_publication_retries_original_points_without_reprobe_then_duplicate_noops() {
    let (config, fake, slot) = fixture(Fake {
        fail_once: true,
        ..Fake::default()
    })
    .await;
    assert!(
        run_slot(config.clone(), slot, "first".into())
            .await
            .is_err()
    );
    let original = fake
        .lock()
        .unwrap()
        .ledger
        .as_ref()
        .unwrap()
        .pending
        .clone()
        .unwrap();
    assert!(fake.lock().unwrap().accepted.is_empty());
    run_slot(config.clone(), slot, "retry".into())
        .await
        .unwrap();
    run_slot(config, slot, "duplicate".into()).await.unwrap();
    let fake = fake.lock().unwrap();
    assert_eq!(fake.probes, 1);
    assert_eq!(fake.accepted, original.points);
    assert_eq!(
        fake.accepted.last().unwrap()["metric"]["type"],
        HEARTBEAT_METRIC
    );
    assert_eq!(fake.ledger.as_ref().unwrap().state.targets[2].successes, 1);
    assert!(fake.ledger.as_ref().unwrap().pending.is_none());
}
#[tokio::test]
async fn lost_response_is_read_back_and_cas_conflict_never_probes_or_publishes() {
    let (config, fake, slot) = fixture(Fake {
        lose_response: true,
        throttle_once: true,
        ..Fake::default()
    })
    .await;
    run_slot(config, slot, "lost-response".into())
        .await
        .unwrap();
    assert_eq!(fake.lock().unwrap().accepted.len(), 5);
    assert_eq!(
        fake.lock().unwrap().throttled,
        1,
        "only injected throttling; all writes paced"
    );
    let (config, fake, slot) = fixture(Fake {
        conflict: true,
        ..Fake::default()
    })
    .await;
    assert!(
        run_slot(config, slot, "conflict".into())
            .await
            .unwrap_err()
            .to_string()
            .contains("CAS conflict")
    );
    let fake = fake.lock().unwrap();
    assert_eq!(fake.probes, 0);
    assert!(fake.accepted.is_empty());
}
