//! Premise gate: real Langfuse 3.225.11, configured as staging is with the
//! issuer on, signs an engineer in through this issuer with no role, links the
//! admin, and refuses every password sign-in and sign-up. Needs Docker with
//! the images in `tests/langfuse/compose.yaml` already pulled.
use langfuse_sso::{Config, router_with_verifier};
use reqwest::{StatusCode, header};
use serde_json::Value;
use sqlx::PgPool;
use std::{
    net::TcpListener,
    path::PathBuf,
    process::{Command, Output},
    time::{Duration, Instant},
};

const AUDIENCE: &str = "/projects/123456789/locations/us-east1/services/public-site-staging";
const CLIENT_ID: &str = "langfuse";
const CLIENT_SECRET: &str = "langfuse-sso-test-client-secret-0123456789";
const ADMIN: &str = "admin@codahealth.ai";
const ADMIN_PASSWORD: &str = "correct-horse-battery-staple-1";

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

/// One compose project, always torn down with its volumes. A failing test
/// prints langfuse-web's logs first.
struct Compose {
    project: String,
    env: Vec<(&'static str, String)>,
}

impl Compose {
    fn command(&self, args: &[&str]) -> Command {
        let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/langfuse/compose.yaml");
        let mut command = Command::new("docker");
        command
            .args(["compose", "-p", &self.project, "-f"])
            .arg(file)
            .args(args)
            .envs(self.env.iter().map(|(name, value)| (name, value)));
        command
    }

    fn run(&self, args: &[&str]) -> Output {
        let output = self.command(args).output().expect("docker compose runs");
        assert!(
            output.status.success(),
            "docker compose {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }
}

impl Drop for Compose {
    fn drop(&mut self) {
        if std::thread::panicking()
            && let Ok(logs) = self
                .command(&["logs", "--tail", "200", "langfuse-web"])
                .output()
        {
            eprintln!(
                "langfuse-web logs:\n{}{}",
                String::from_utf8_lossy(&logs.stdout),
                String::from_utf8_lossy(&logs.stderr)
            );
        }
        let _ = self.command(&["down", "-v", "--remove-orphans"]).output();
    }
}

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

/// Signs in through the custom provider as `email`, acting as the browser
/// behind public-site, and returns the resulting session.
async fn sign_in_with_sso(langfuse: &str, sub: &str, email: &str) -> Value {
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
    let authorize = started["url"].as_str().unwrap();
    assert!(
        authorize.starts_with("http://localhost:")
            && authorize.contains("/langfuse-sso/authorize?"),
        "sign-in did not reach the issuer: {started}"
    );
    let response = browser
        .get(authorize)
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
    assert!(
        !landed.contains("error"),
        "NextAuth refused the callback: {} -> {landed}",
        response.status()
    );
    current_session(&browser, langfuse).await
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "starts Langfuse in Docker"]
async fn langfuse_signs_engineers_in_through_the_issuer() {
    let issuer_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let issuer_port = issuer_listener.local_addr().unwrap().port();
    let langfuse_port = free_port();
    let langfuse = format!("http://127.0.0.1:{langfuse_port}");
    let issuer = format!("http://127.0.0.1:{issuer_port}/langfuse-sso");
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

    let postgres_port = free_port();
    let compose = Compose {
        project: format!("langfuse-sso-test-{}", std::process::id()),
        env: vec![
            ("POSTGRES_PORT", postgres_port.to_string()),
            ("VALKEY_PORT", free_port().to_string()),
            ("CLICKHOUSE_HTTP_PORT", free_port().to_string()),
            ("CLICKHOUSE_NATIVE_PORT", free_port().to_string()),
            ("LANGFUSE_PORT", langfuse_port.to_string()),
            ("ISSUER_PORT", issuer_port.to_string()),
            ("CLIENT_ID", CLIENT_ID.into()),
            ("CLIENT_SECRET", CLIENT_SECRET.into()),
            ("ADMIN_PASSWORD", ADMIN_PASSWORD.into()),
        ],
    };
    compose.run(&["up", "-d"]);
    let probe = browser();
    let deadline = Instant::now() + Duration::from_secs(600);
    loop {
        let ready = probe
            .get(format!("{langfuse}/api/public/ready"))
            .send()
            .await
            .is_ok_and(|response| response.status().is_success());
        if ready {
            break;
        }
        assert!(Instant::now() < deadline, "Langfuse never became ready");
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    let database = PgPool::connect(&format!(
        "postgres://langfuse:langfuse@127.0.0.1:{postgres_port}/langfuse"
    ))
    .await
    .unwrap();

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
