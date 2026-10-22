use super::*;
use npi_registry::FixtureDirectory;
use tower::ServiceExt;

async fn setup(directory: FixtureDirectory) -> (axum::Router, String, String) {
    let store = Arc::new(MemoryAuthStore::new());
    store.seed_account("test", "one", "one@test", "Dr. Ada Example", false);
    store.seed_account("other", "two", "two@test", "Other Person", false);
    let first = store.seed_session("one@test", now()).unwrap();
    let second = store.seed_session("two@test", now()).unwrap();
    let mut auth = Auth::on(store, Arc::new(MemoryFrontDoorStore::new())).await;
    Arc::get_mut(&mut auth).unwrap().directory = Arc::new(directory);
    (crate::auth::auth_routes(auth), first, second)
}
async fn send(
    app: &axum::Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Value,
) -> (StatusCode, Value) {
    let mut request = axum::http::Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 100_000)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}
fn directory() -> (FixtureDirectory, String, String) {
    let values = FixtureDirectory::sample().clinicians();
    let mut individual = values
        .iter()
        .find(|v| v.kind == npi_registry::ClinicianKind::Individual)
        .unwrap()
        .clone();
    individual.person.as_mut().unwrap().last = "Example".into();
    let org = values
        .iter()
        .find(|v| v.kind == npi_registry::ClinicianKind::Organization)
        .unwrap()
        .clone();
    let npi = individual.npi.to_string();
    let org_npi = org.npi.to_string();
    (FixtureDirectory::new([individual, org]), npi, org_npi)
}
#[tokio::test]
async fn npi_routes_require_session_and_return_self_asserted_metadata_only() {
    let (directory, npi, _) = directory();
    let (app, token, other) = setup(directory).await;
    for (method, path, body) in [
        ("PUT", "/auth/profile/npi", json!({"npi": npi})),
        ("DELETE", "/auth/profile/npi", json!({})),
        (
            "POST",
            "/auth/clinicians/search",
            json!({"last_name": "Example"}),
        ),
    ] {
        assert_eq!(
            send(&app, method, path, None, body).await.0,
            StatusCode::UNAUTHORIZED
        );
    }
    let (status, results) = send(
        &app,
        "POST",
        "/auth/clinicians/search",
        Some(&token),
        json!({"last_name": "Example"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(results[0]["npi"], npi);
    let (status, claim) = send(
        &app,
        "PUT",
        "/auth/profile/npi",
        Some(&token),
        json!({"npi": npi}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(claim["name_matched"], true);
    assert!(claim.get("verified_at").is_none());
    assert!(claim.get("verified").is_none());
    let who = send(&app, "GET", "/auth/whoami", Some(&token), json!({}))
        .await
        .1;
    assert_eq!(who["npi"], claim);
    assert_eq!(who["admin"], false);
    assert_eq!(
        send(&app, "DELETE", "/auth/profile/npi", Some(&other), json!({}))
            .await
            .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        send(&app, "GET", "/auth/whoami", Some(&token), json!({}))
            .await
            .1["npi"],
        claim
    );
    assert_eq!(
        send(&app, "DELETE", "/auth/profile/npi", Some(&token), json!({}))
            .await
            .0,
        StatusCode::NO_CONTENT
    );
    assert!(
        send(&app, "GET", "/auth/whoami", Some(&token), json!({}))
            .await
            .1["npi"]
            .is_null()
    );
}
#[tokio::test]
async fn npi_routes_reject_invalid_org_mismatch_override_and_forged_snapshot() {
    let (directory, npi, org) = directory();
    let (app, token, other) = setup(directory).await;
    for (token, body, status) in [
        (&token, json!({"npi":"123"}), StatusCode::BAD_REQUEST),
        (&token, json!({"npi":org}), StatusCode::UNPROCESSABLE_ENTITY),
        (&other, json!({"npi":npi}), StatusCode::UNPROCESSABLE_ENTITY),
        (
            &other,
            json!({"npi":npi,"override_name":true}),
            StatusCode::FORBIDDEN,
        ),
        (
            &token,
            json!({"npi":npi,"snapshot":{"verified":true}}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
    ] {
        assert_eq!(
            send(&app, "PUT", "/auth/profile/npi", Some(token), body)
                .await
                .0,
            status
        );
    }
    assert_eq!(
        send(
            &app,
            "POST",
            "/auth/clinicians/search",
            Some(&token),
            json!({"state":"CA"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
}
#[tokio::test]
async fn npi_directory_failure_is_retryable_and_does_not_save() {
    let (app, token, _) = setup(FixtureDirectory::failing(RegistryError::Unavailable(
        "private transport detail".into(),
    )))
    .await;
    let npi = FixtureDirectory::sample().clinicians()[0].npi.to_string();
    let (status, body) = send(
        &app,
        "PUT",
        "/auth/profile/npi",
        Some(&token),
        json!({"npi":npi}),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_GATEWAY);
    assert!(!body.to_string().contains("private transport"));
    assert!(
        send(&app, "GET", "/auth/whoami", Some(&token), json!({}))
            .await
            .1["npi"]
            .is_null()
    );
}
