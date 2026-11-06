//! Synthetic targets on loopback: every outcome class, the no-redirect rule,
//! the bearer each target receives, and the published Monitoring body.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use availability_probe::{
    Config, Endpoints, HEARTBEAT_METRIC, Outcome, READINESS_METRIC, Target, check, probe_client,
};
use axum::Router;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde_json::{Value, json};

async fn serve(router: Router) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    format!("http://{address}")
}

async fn answer(status: StatusCode) -> String {
    serve(Router::new().route("/readyz", get(move || async move { status }))).await
}

async fn probe(origin: &str, timeout: Duration) -> Outcome {
    let client = probe_client(timeout).unwrap();
    check(&client, Target::Duet, origin, None, timeout)
        .await
        .outcome
}

#[tokio::test]
async fn ready_is_success() {
    assert_eq!(
        probe(&answer(StatusCode::OK).await, Duration::from_secs(5)).await,
        Outcome::Success
    );
    assert_eq!(
        probe(
            &answer(StatusCode::NO_CONTENT).await,
            Duration::from_secs(5)
        )
        .await,
        Outcome::Success
    );
}

#[tokio::test]
async fn sign_in_redirect_is_auth_failure_and_never_followed() {
    // IAP's unauthenticated answer: a 302 to Google sign-in. The redirect
    // target here answers 200, so following it would wrongly read as ready.
    let followed = Arc::new(Mutex::new(false));
    let seen = followed.clone();
    let origin = serve(
        Router::new()
            .route(
                "/readyz",
                get(|| async { (StatusCode::FOUND, [(header::LOCATION, "/signin")]) }),
            )
            .route(
                "/signin",
                get(move || {
                    *seen.lock().unwrap() = true;
                    async { StatusCode::OK }
                }),
            ),
    )
    .await;
    assert_eq!(
        probe(&origin, Duration::from_secs(5)).await,
        Outcome::AuthFailure
    );
    assert!(!*followed.lock().unwrap(), "the probe followed a redirect");
}

#[tokio::test]
async fn unauthorized_and_forbidden_are_auth_failures() {
    assert_eq!(
        probe(
            &answer(StatusCode::UNAUTHORIZED).await,
            Duration::from_secs(5)
        )
        .await,
        Outcome::AuthFailure
    );
    assert_eq!(
        probe(&answer(StatusCode::FORBIDDEN).await, Duration::from_secs(5)).await,
        Outcome::AuthFailure
    );
}

#[tokio::test]
async fn not_ready_is_target_failure() {
    assert_eq!(
        probe(
            &answer(StatusCode::SERVICE_UNAVAILABLE).await,
            Duration::from_secs(5)
        )
        .await,
        Outcome::TargetFailure
    );
    assert_eq!(
        probe(
            &answer(StatusCode::INTERNAL_SERVER_ERROR).await,
            Duration::from_secs(5)
        )
        .await,
        Outcome::TargetFailure
    );
    assert_eq!(
        probe(&answer(StatusCode::NOT_FOUND).await, Duration::from_secs(5)).await,
        Outcome::TargetFailure
    );
}

#[tokio::test]
async fn slow_answer_is_timeout() {
    let origin = serve(Router::new().route(
        "/readyz",
        get(|| async {
            tokio::time::sleep(Duration::from_secs(5)).await;
            StatusCode::OK
        }),
    ))
    .await;
    assert_eq!(
        probe(&origin, Duration::from_millis(300)).await,
        Outcome::Timeout
    );
}

#[tokio::test]
async fn closed_port_is_refused() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    assert_eq!(
        probe(&format!("http://{address}"), Duration::from_secs(5)).await,
        Outcome::Refused
    );
}

#[derive(Default)]
struct Google {
    targets: Mutex<Vec<(String, Option<String>)>>,
    signed: Mutex<Vec<Value>>,
    identity_audiences: Mutex<Vec<String>>,
    published: Mutex<Option<Value>>,
    reject_service_metrics: bool,
    publication_attempts: Mutex<Vec<Value>>,
}

async fn google(State(state): State<Arc<Google>>, request: Request) -> Response {
    let path = request.uri().path().to_owned();
    let query = request.uri().query().unwrap_or_default().to_owned();
    let headers: HeaderMap = request.headers().clone();
    let body = axum::body::to_bytes(request.into_body(), 1 << 20)
        .await
        .unwrap();
    let metadata = headers
        .get("metadata-flavor")
        .is_some_and(|value| value == "Google");
    match path.as_str() {
        "/metadata/token" if metadata => {
            axum::Json(json!({ "access_token": "synthetic-access", "expires_in": 3600 }))
                .into_response()
        }
        "/metadata/identity" if metadata => {
            let audience = query
                .split('&')
                .find_map(|pair| pair.strip_prefix("audience="))
                .unwrap_or_default();
            state
                .identity_audiences
                .lock()
                .unwrap()
                .push(audience.replace("%3A", ":").replace("%2F", "/"));
            "id.synthetic.token".into_response()
        }
        "/iam/probe@test.iam.gserviceaccount.com:signJwt" => {
            assert_eq!(
                headers.get(header::AUTHORIZATION).unwrap(),
                "Bearer synthetic-access"
            );
            let request: Value = serde_json::from_slice(&body).unwrap();
            let payload: Value =
                serde_json::from_str(request["payload"].as_str().unwrap()).unwrap();
            state.signed.lock().unwrap().push(payload);
            axum::Json(json!({ "keyId": "k", "signedJwt": "iap.synthetic.jwt" })).into_response()
        }
        "/monitoring/test-project/timeSeries" => {
            assert_eq!(
                headers.get(header::AUTHORIZATION).unwrap(),
                "Bearer synthetic-access"
            );
            let batch: Value = serde_json::from_slice(&body).unwrap();
            state
                .publication_attempts
                .lock()
                .unwrap()
                .push(batch.clone());
            if state.reject_service_metrics
                && batch["timeSeries"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|s| s["metric"]["type"] != HEARTBEAT_METRIC)
            {
                return StatusCode::BAD_REQUEST.into_response();
            }
            let mut published = state.published.lock().unwrap();
            if let Some(existing) = published.as_mut() {
                existing["timeSeries"]
                    .as_array_mut()
                    .unwrap()
                    .extend(batch["timeSeries"].as_array().unwrap().iter().cloned());
            } else {
                *published = Some(batch);
            }
            axum::Json(json!({})).into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn target(
    State((state, name, status)): State<(Arc<Google>, &'static str, StatusCode)>,
    headers: HeaderMap,
) -> StatusCode {
    let bearer = headers
        .get(header::AUTHORIZATION)
        .map(|value| value.to_str().unwrap().to_owned());
    state
        .targets
        .lock()
        .unwrap()
        .push((name.to_owned(), bearer));
    status
}

#[tokio::test]
async fn run_authenticates_each_target_and_publishes_points_and_heartbeat() {
    let state = Arc::new(Google::default());
    let api = serve(Router::new().fallback(google).with_state(state.clone())).await;
    let physician = serve(Router::new().route("/readyz", get(target)).with_state((
        state.clone(),
        "physician",
        StatusCode::OK,
    )))
    .await;
    let duet = serve(Router::new().route("/readyz", get(target)).with_state((
        state.clone(),
        "duet",
        StatusCode::FOUND,
    )))
    .await;
    let worker = serve(Router::new().route("/readyz", get(target)).with_state((
        state.clone(),
        "physician-worker",
        StatusCode::SERVICE_UNAVAILABLE,
    )))
    .await;
    let config = Config {
        project: "test-project".into(),
        state_bucket: None,
        service_account: "probe@test.iam.gserviceaccount.com".into(),
        targets: vec![
            (Target::Physician, physician.clone()),
            (Target::Duet, duet.clone()),
            (Target::PhysicianWorker, worker.clone()),
        ],
        timeout: Duration::from_secs(5),
        endpoints: Endpoints {
            metadata: format!("{api}/metadata"),
            sign_jwt: format!("{api}/iam"),
            monitoring: format!("{api}/monitoring"),
            storage: api.clone(),
        },
    };

    let observations = availability_probe::run(config).await.unwrap();
    let outcomes: Vec<_> = observations.iter().map(|o| (o.target, o.outcome)).collect();
    assert_eq!(
        outcomes,
        [
            (Target::Physician, Outcome::Success),
            (Target::Duet, Outcome::AuthFailure),
            (Target::PhysicianWorker, Outcome::TargetFailure)
        ]
    );

    // IAP targets get a self-signed JWT for `<origin>/*`; the worker an ID token for its URL.
    let mut audiences: Vec<String> = state
        .signed
        .lock()
        .unwrap()
        .iter()
        .map(|p| p["aud"].as_str().unwrap().to_owned())
        .collect();
    audiences.sort();
    let mut expected = vec![format!("{physician}/*"), format!("{duet}/*")];
    expected.sort();
    assert_eq!(audiences, expected);
    for payload in state.signed.lock().unwrap().iter() {
        assert_eq!(payload["iss"], "probe@test.iam.gserviceaccount.com");
        assert_eq!(payload["sub"], "probe@test.iam.gserviceaccount.com");
        assert!(payload["exp"].as_u64().unwrap() - payload["iat"].as_u64().unwrap() <= 3600);
    }
    assert_eq!(
        *state.identity_audiences.lock().unwrap(),
        std::slice::from_ref(&worker)
    );
    let mut seen = state.targets.lock().unwrap().clone();
    seen.sort();
    assert_eq!(
        seen,
        [
            (
                "duet".to_owned(),
                Some("Bearer iap.synthetic.jwt".to_owned())
            ),
            (
                "physician".to_owned(),
                Some("Bearer iap.synthetic.jwt".to_owned())
            ),
            (
                "physician-worker".to_owned(),
                Some("Bearer id.synthetic.token".to_owned())
            ),
        ]
    );

    let published = state.published.lock().unwrap().clone().unwrap();
    let series = published["timeSeries"].as_array().unwrap();
    assert_eq!(series.len(), 4);
    let readiness: Vec<_> = series
        .iter()
        .filter(|s| s["metric"]["type"] == READINESS_METRIC)
        .map(|s| {
            (
                s["metric"]["labels"]["target"].as_str().unwrap(),
                s["metric"]["labels"]["outcome"].as_str().unwrap(),
                s["points"][0]["value"]["int64Value"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        readiness,
        [
            ("physician", "success", "0"),
            ("duet", "auth_failure", "1"),
            ("physician-worker", "target_failure", "1")
        ]
    );
    let heartbeat: Vec<_> = series
        .iter()
        .filter(|s| s["metric"]["type"] == HEARTBEAT_METRIC)
        .collect();
    assert_eq!(heartbeat.len(), 1);
    assert_eq!(heartbeat[0]["points"][0]["value"]["int64Value"], "1");
    for s in series {
        assert_eq!(
            s["resource"],
            json!({ "type": "global", "labels": { "project_id": "test-project" } })
        );
        assert_eq!(s["metricKind"], "GAUGE");
        // Only the fixed labels: no URL, status code or latency becomes a dimension.
        let labels = s["metric"]["labels"].as_object().unwrap();
        assert!(labels.keys().all(|key| key == "target" || key == "outcome"));
    }
}

#[tokio::test]
async fn credential_failure_is_auth_failure_without_a_request() {
    let state = Arc::new(Google::default());
    let api = serve(Router::new().fallback(google).with_state(state.clone())).await;
    let physician = serve(Router::new().route("/readyz", get(target)).with_state((
        state.clone(),
        "physician",
        StatusCode::OK,
    )))
    .await;
    let config = Config {
        project: "test-project".into(),
        state_bucket: None,
        // signJwt for another account answers 404 here.
        service_account: "other@test.iam.gserviceaccount.com".into(),
        targets: vec![(Target::Physician, physician)],
        timeout: Duration::from_secs(5),
        endpoints: Endpoints {
            metadata: format!("{api}/metadata"),
            sign_jwt: format!("{api}/iam"),
            monitoring: format!("{api}/monitoring"),
            storage: api.clone(),
        },
    };
    let observations = availability_probe::run(config).await.unwrap();
    assert_eq!(observations[0].outcome, Outcome::AuthFailure);
    assert!(state.targets.lock().unwrap().is_empty());
    assert!(
        state.published.lock().unwrap().is_some(),
        "the heartbeat is still published"
    );
}

#[tokio::test]
async fn publish_failure_fails_the_run() {
    let state = Arc::new(Google::default());
    let api = serve(Router::new().fallback(google).with_state(state.clone())).await;
    let physician = serve(Router::new().route("/readyz", get(target)).with_state((
        state.clone(),
        "physician",
        StatusCode::OK,
    )))
    .await;
    let config = Config {
        project: "wrong-project".into(),
        state_bucket: None,
        service_account: "probe@test.iam.gserviceaccount.com".into(),
        targets: vec![(Target::Physician, physician)],
        timeout: Duration::from_secs(5),
        endpoints: Endpoints {
            metadata: format!("{api}/metadata"),
            sign_jwt: format!("{api}/iam"),
            monitoring: format!("{api}/monitoring"),
            storage: api.clone(),
        },
    };
    assert!(
        availability_probe::run(config)
            .await
            .unwrap_err()
            .to_string()
            .contains("404")
    );
}

#[tokio::test]
async fn rejected_service_metrics_never_publish_a_healthy_heartbeat() {
    let state = Arc::new(Google {
        reject_service_metrics: true,
        ..Google::default()
    });
    let api = serve(Router::new().fallback(google).with_state(state.clone())).await;
    let config = Config {
        project: "test-project".into(),
        state_bucket: None,
        service_account: "probe@test.iam.gserviceaccount.com".into(),
        targets: vec![(Target::Physician, answer(StatusCode::OK).await)],
        timeout: Duration::from_secs(5),
        endpoints: Endpoints {
            metadata: format!("{api}/metadata"),
            sign_jwt: format!("{api}/iam"),
            monitoring: format!("{api}/monitoring"),
            storage: api,
        },
    };
    assert!(availability_probe::run(config).await.is_err());
    let attempts = state.publication_attempts.lock().unwrap();
    assert_eq!(attempts.len(), 1);
    assert!(
        attempts[0]["timeSeries"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["metric"]["type"] != HEARTBEAT_METRIC)
    );
}

#[test]
fn config_requires_identity_and_https_targets() {
    fn env(pairs: &[(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        let pairs = pairs.to_vec();
        move |key| {
            if key == "AVAILABILITY_PROBE_STATE_BUCKET" {
                return Some("synthetic-bucket".into());
            }
            pairs
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.to_string())
        }
    }
    let base = [
        ("AVAILABILITY_PROBE_PROJECT", "p"),
        (
            "AVAILABILITY_PROBE_SERVICE_ACCOUNT",
            "probe@p.iam.gserviceaccount.com",
        ),
    ];
    assert!(Config::from_env(env(&[("AVAILABILITY_PROBE_PROJECT", "p")])).is_err());
    assert!(
        Config::from_env(env(&[base[0], base[1]]))
            .unwrap_err()
            .to_string()
            .contains("no AVAILABILITY_PROBE")
    );
    assert!(
        Config::from_env(env(&[
            base[0],
            base[1],
            ("AVAILABILITY_PROBE_DUET_URL", "http://duet")
        ]))
        .is_err()
    );
    assert!(
        Config::from_env(env(&[
            base[0],
            base[1],
            ("AVAILABILITY_PROBE_DUET_URL", "https://d"),
            ("AVAILABILITY_PROBE_TIMEOUT_SECONDS", "90")
        ]))
        .is_err()
    );
    let config = Config::from_env(env(&[
        base[0],
        base[1],
        (
            "AVAILABILITY_PROBE_PHYSICIAN_URL",
            "https://physician-1.us-east1.run.app/",
        ),
        (
            "AVAILABILITY_PROBE_WORKER_URL",
            "https://physician-worker-1.us-east1.run.app",
        ),
    ]))
    .unwrap();
    assert_eq!(
        config.timeout,
        Duration::from_secs(30),
        "cold starts get a generous default"
    );
    assert_eq!(
        config.targets,
        [
            (
                Target::Physician,
                "https://physician-1.us-east1.run.app".to_owned()
            ),
            (
                Target::PhysicianWorker,
                "https://physician-worker-1.us-east1.run.app".to_owned()
            )
        ]
    );
}
