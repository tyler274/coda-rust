//! Premise gate: real Langfuse 3.225.11, configured as staging is with the
//! issuer on, signs an engineer in through this issuer with no role, links the
//! admin, and refuses every password sign-in and sign-up. Needs Docker with
//! the images in `dev/observability/test/langfuse.yaml` already pulled.
use langfuse_sso::{Config, router_with_verifier};
use reqwest::{StatusCode, header};
use serde_json::Value;
use sqlx::PgPool;
use std::time::Duration;
use telemetry::testing::{Langfuse, Shape};
use url::Url;

const AUDIENCE: &str = "/projects/123456789/locations/us-east1/services/public-site-staging";
const CLIENT_ID: &str = "langfuse";
const CLIENT_SECRET: &str = "langfuse-sso-test-client-secret-0123456789";
const ADMIN: &str = "admin@codahealth.ai";
const ADMIN_PASSWORD: &str = "correct-horse-battery-staple-1";

fn browser() -> reqwest::Client {
    reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

fn location(response: &reqwest::Response) -> String {
    response
        .headers()
        .get(header::LOCATION)
        .unwrap_or_else(|| panic!("{} without a Location", response.status()))
        .to_str()
        .unwrap()
        .to_owned()
}

async fn csrf(browser: &reqwest::Client, langfuse: &str) -> String {
    let body: Value = browser
        .get(format!("{langfuse}/api/auth/csrf"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    body["csrfToken"].as_str().unwrap().to_owned()
}

/// `(org_id, role)` for each of the user's organization memberships.
async fn memberships(database: &PgPool, email: &str) -> Vec<(String, String)> {
    sqlx::query_as(
        "select m.org_id, m.role::text from organization_memberships m \
         join users u on u.id = m.user_id where u.email = $1 order by m.org_id",
    )
    .bind(email)
    .fetch_all(database)
    .await
    .unwrap()
}

/// `(provider, providerAccountId)` for each of the user's linked accounts.
async fn accounts(database: &PgPool, email: &str) -> Vec<(String, String)> {
    sqlx::query_as(
        r#"select a.provider, a."providerAccountId" from "Account" a
           join users u on u.id = a.user_id where u.email = $1 order by a.provider"#,
    )
    .bind(email)
    .fetch_all(database)
    .await
    .unwrap()
}

async fn current_session(browser: &reqwest::Client, langfuse: &str) -> Value {
    browser
        .get(format!("{langfuse}/api/auth/session"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

/// Runs the custom provider's sign-in as `email`, acting as the browser behind
/// public-site, with `authorize` able to alter the authorize URL Langfuse
/// built. Returns the browser and where NextAuth's callback sent it.
async fn run_sso(
    langfuse: &str,
    sub: &str,
    email: &str,
    authorize: impl Fn(&str) -> String,
) -> (reqwest::Client, String) {
    let browser = browser();
    let csrf = csrf(&browser, langfuse).await;
    let started: Value = browser
        .post(format!("{langfuse}/api/auth/signin/custom"))
        .form(&[
            ("csrfToken", csrf.as_str()),
            ("callbackUrl", &format!("{langfuse}/")),
            ("json", "true"),
        ])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let url = started["url"].as_str().unwrap();
    assert!(
        url.starts_with("http://localhost:") && url.contains("/langfuse-sso/authorize?"),
        "sign-in did not reach the issuer: {started}"
    );
    let parameters: Vec<String> = Url::parse(url)
        .unwrap()
        .query_pairs()
        .map(|(name, _)| name.into_owned())
        .collect();
    for required in ["state", "code_challenge", "nonce"] {
        assert!(
            parameters.iter().any(|name| name == required),
            "Langfuse sent no {required}: {url}"
        );
    }
    let response = browser
        .get(authorize(url))
        .header(
            "x-coda-iap-assertion",
            http::iap::fixture::identity_assertion(AUDIENCE, sub, email),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FOUND, "authorize refused");
    let callback = location(&response);
    assert!(
        callback.starts_with(&format!("{langfuse}/api/auth/callback/custom?")),
        "{callback}"
    );
    let response = browser.get(&callback).send().await.unwrap();
    let landed = location(&response);
    (browser, landed)
}

/// Signs in through the custom provider as `email` and returns the session.
async fn sign_in_with_sso(langfuse: &str, sub: &str, email: &str) -> Value {
    let (browser, landed) = run_sso(langfuse, sub, email, str::to_owned).await;
    assert!(
        !landed.contains("error"),
        "NextAuth refused the callback: {landed}"
    );
    current_session(&browser, langfuse).await
}

/// The authorize URL with its `nonce` replaced, as if the code had been
/// issued for another sign-in.
fn other_nonce(url: &str) -> String {
    let mut url = Url::parse(url).unwrap();
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(name, value)| match name.as_ref() {
            "nonce" => (name.into_owned(), "another-sign-in".to_owned()),
            _ => (name.into_owned(), value.into_owned()),
        })
        .collect();
    url.query_pairs_mut().clear().extend_pairs(pairs);
    url.into()
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "starts Langfuse in Docker"]
async fn langfuse_signs_engineers_in_through_the_issuer() {
    let issuer_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let issuer_port = issuer_listener.local_addr().unwrap().port();
    let issuer = format!("http://127.0.0.1:{issuer_port}/langfuse-sso");
    let compose = Langfuse::start(
        "langfuse-sso-test",
        Shape::Overlay {
            file: "langfuse-sso.yaml",
            env: vec![
                ("ISSUER_PORT", issuer_port.to_string()),
                ("CLIENT_ID", CLIENT_ID.into()),
                ("CLIENT_SECRET", CLIENT_SECRET.into()),
                ("ADMIN_PASSWORD", ADMIN_PASSWORD.into()),
            ],
        },
    );
    let langfuse = compose.url.clone();
    let key = http::iap::fixture::key_file()["private_pkcs8_base64"]
        .as_str()
        .unwrap()
        .to_owned();
    let config = Config::from_lookup(|name| {
        Some(match name {
            "LANGFUSE_SSO_ISSUER" => issuer.clone(),
            // A different origin from the issuer, as public-site is in staging.
            "LANGFUSE_SSO_AUTHORIZE_URL" => {
                format!("http://localhost:{issuer_port}/langfuse-sso/authorize")
            }
            "LANGFUSE_SSO_CLIENT_ID" => CLIENT_ID.into(),
            "LANGFUSE_SSO_CLIENT_SECRET" => CLIENT_SECRET.into(),
            "LANGFUSE_SSO_REDIRECT_URI" => format!("{langfuse}/api/auth/callback/custom"),
            "LANGFUSE_SSO_SIGNING_KEY" => key.clone(),
            "LANGFUSE_SSO_IAP_AUDIENCE" => AUDIENCE.into(),
            _ => return None,
        })
    })
    .unwrap();
    let app = router_with_verifier(config, http::iap::fixture::verifier(AUDIENCE));
    let server = tokio::spawn(async move { axum::serve(issuer_listener, app).await });

    let database = PgPool::connect(&format!(
        "postgres://langfuse:langfuse@127.0.0.1:{}/langfuse",
        compose.postgres_port
    ))
    .await
    .unwrap();

    // NextAuth binds the ID token to the nonce it sent: a token carrying
    // another nonce does not sign anyone in.
    let replayed = "replayed@codahealth.ai";
    let (replay_browser, landed) =
        run_sso(&langfuse, "accounts.google.com:2003", replayed, other_nonce).await;
    println!("mismatched nonce: {landed}");
    assert!(landed.contains("error"), "{landed}");
    let session = current_session(&replay_browser, &langfuse).await;
    assert!(session.get("user").is_none(), "{session}");
    assert!(memberships(&database, replayed).await.is_empty());

    let engineer = "new-engineer@codahealth.ai";
    let session = sign_in_with_sso(&langfuse, "accounts.google.com:2001", engineer).await;
    println!("engineer session: {session}");
    assert_eq!(session["user"]["email"], engineer, "{session}");
    let engineer_memberships = memberships(&database, engineer).await;
    println!("engineer organization_memberships: {engineer_memberships:?}");
    assert_eq!(
        engineer_memberships,
        [("coda".to_owned(), "NONE".to_owned())]
    );
    let projects: i64 = sqlx::query_scalar(
        "select count(*) from project_memberships pm \
         join users u on u.id = pm.user_id where u.email = $1",
    )
    .bind(engineer)
    .fetch_one(&database)
    .await
    .unwrap();
    println!("engineer project_memberships: {projects}");
    assert_eq!(projects, 0);
    let engineer_accounts = accounts(&database, engineer).await;
    println!("engineer accounts: {engineer_accounts:?}");
    assert_eq!(
        engineer_accounts,
        [("custom".to_owned(), "accounts.google.com:2001".to_owned())]
    );

    let session = sign_in_with_sso(&langfuse, "accounts.google.com:2002", ADMIN).await;
    println!("admin SSO session: {session}");
    assert_eq!(session["user"]["email"], ADMIN, "{session}");
    let admin_accounts = accounts(&database, ADMIN).await;
    println!("admin accounts: {admin_accounts:?}");
    assert_eq!(
        admin_accounts,
        [("custom".to_owned(), "accounts.google.com:2002".to_owned())]
    );
    let admin_memberships = memberships(&database, ADMIN).await;
    println!("admin organization_memberships: {admin_memberships:?}");
    assert_eq!(admin_memberships, [("coda".to_owned(), "OWNER".to_owned())]);

    // With the issuer on, no password signs anyone in or creates anyone,
    // whichever path a caller behind IAP reaches.
    let browser = browser();
    let csrf = csrf(&browser, &langfuse).await;
    let response = browser
        .post(format!("{langfuse}/api/auth/callback/credentials"))
        .form(&[
            ("csrfToken", csrf.as_str()),
            ("email", ADMIN),
            ("password", ADMIN_PASSWORD),
            ("callbackUrl", &format!("{langfuse}/")),
            ("json", "true"),
        ])
        .send()
        .await
        .unwrap();
    let status = response.status();
    let body = response.text().await.unwrap();
    println!("admin credentials sign-in: {status} {body}");
    assert!(
        body.contains("error"),
        "credentials sign-in: {status} {body}"
    );
    let session = current_session(&browser, &langfuse).await;
    assert!(session.get("user").is_none(), "{session}");
    let squatter = "squatter@codahealth.ai";
    let response = browser
        .post(format!("{langfuse}/api/auth/signup"))
        .json(&serde_json::json!({
            "name": "Squatter",
            "email": squatter,
            "password": ADMIN_PASSWORD,
        }))
        .send()
        .await
        .unwrap();
    let status = response.status();
    let body = response.text().await.unwrap();
    println!("password sign-up: {status} {body}");
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let squatters: i64 = sqlx::query_scalar("select count(*) from users where email = $1")
        .bind(squatter)
        .fetch_one(&database)
        .await
        .unwrap();
    assert_eq!(squatters, 0);

    drop(compose);
    server.abort();
}
