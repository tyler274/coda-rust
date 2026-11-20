//! Real PostgreSQL + durable routes + compiled patient UI; no stub or live model.

use super::*;
use duet_api::{
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore},
    workspace::WorkspaceStore,
};
use sqlx::{ConnectOptions, Connection};
use uuid::Uuid;

const TOKEN: &str = "synthetic-durable-browser-only";
struct SyntheticIdentity;
#[async_trait::async_trait]
impl IdentityVerifier for SyntheticIdentity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, duet_api::ApiError> {
        if ![TOKEN, "synthetic-foreign-browser-only"].contains(&bearer) {
            return Err(duet_api::ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: if bearer == TOKEN {
                "durable-browser-patient"
            } else {
                "durable-foreign-patient"
            }
            .into(),
            authenticated_at: chrono::Utc::now(),
        })
    }
}

fn click_text(browser: &mut Browser, text: &str) {
    let result = browser.page().evaluate(&format!("(() => {{ const b = [...document.querySelectorAll('button')].find(b => b.textContent.trim() === {text:?}); if (!b || b.disabled) return false; b.click(); return true; }})()"));
    assert_eq!(
        result.as_bool(),
        Some(true),
        "button {text} unavailable: {:?}",
        browser.body_text()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn saved_page_publishes_and_revokes_through_real_durable_routes() {
    with_durable_browser(run_browser_checks).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn switching_accounts_shows_only_the_current_patients_imported_measurements() {
    with_durable_browser(async |base, _, _, _, _| {
        let http = reqwest::Client::new();
        const OTHER: &str = "synthetic-foreign-browser-only";
        for (bearer, title, value) in [(TOKEN, "First patient steps", 7654), (OTHER, "Second patient steps", 2301)] {
            http.put(format!("{base}/api/auth/account")).bearer_auth(bearer)
                .json(&serde_json::json!({})).send().await.unwrap().error_for_status().unwrap();
            http.put(format!("{base}/api/auth/account/terms")).bearer_auth(bearer)
                .json(&serde_json::json!({"accepted": true, "version": "2026-09-23"}))
                .send().await.unwrap().error_for_status().unwrap();
            let id = Uuid::new_v4();
            http.post(format!("{base}/api/canonical-evidence")).bearer_auth(bearer)
                .json(&serde_json::json!({"operation_id":Uuid::new_v4(), "canonical_record_id":id,
                    "record_id":id.to_string(),"entity":"health_evidence","source_generation":0,
                    "payload":{"concept_id":"steps","display_name":title,"title":title,
                        "measured_at":"2026-10-28T12:00:00Z","source":{"kind":"appleHealth"},
                        "value":{"literal":value.to_string(),"unit":"count"}}}))
                .send().await.unwrap().error_for_status().unwrap();
        }

        let mut patient = Browser::launch();
        patient.text_until(&format!("{base}/?mode=client"), "Shared");
        patient.click("nav.tabs button:nth-of-type(1)");
        patient.wait_for("First patient steps");
        click_text(&mut patient, "Settings");
        patient.wait_for("Sessions & account");
        click_text(&mut patient, "Sign out");
        patient.wait_for("Sign in to your Duet account.");
        assert!(!patient.body_text().unwrap().contains("First patient steps"));

        // Substitute the second synthetic identity at the transport, leaving the same
        // mounted account shell, tab, origin, storage and real API in place.
        patient.page().call_session("Network.enable", serde_json::json!({}));
        patient.page().call_session("Network.setExtraHTTPHeaders", serde_json::json!({
            "headers":{"Authorization":format!("Bearer {OTHER}")}
        }));
        patient.page().evaluate("window.previousPatientShown = false; new MutationObserver(() => { if (document.body.innerText.includes('First patient steps')) window.previousPatientShown = true }).observe(document.body, {subtree:true,childList:true,characterData:true,attributes:true}); true");
        click_text(&mut patient, "Retry saved sign-in");
        patient.wait_for("Shared");
        patient.click("nav.tabs button:nth-of-type(2)");
        patient.click("nav.tabs button:nth-of-type(1)");
        patient.wait_for("Second patient steps");
        assert_eq!(patient.page().evaluate("window.previousPatientShown"), serde_json::json!(false));
    }).await;
}

async fn with_durable_browser(
    check: impl AsyncFn(
        &str,
        &WorkspaceStore,
        &duet_api::identity::AccountContext,
        &str,
        &mut BackendProcess,
    ),
) {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("explicit disposable loopback PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&options.get_host()));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("duet_browser_{suffix}");
    let role = format!("duet_browser_role_{suffix}");
    crate::disposable::create_database_and_role(&mut admin, &name, &role).await;
    let owner_url = options.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = options
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    migrate(&owner_url, &role).await.unwrap();
    let database = AccountDatabase::connect(&runtime_url).await.unwrap();
    let identity = IdentityStore::new(database.clone(), Arc::new(SyntheticIdentity));
    let account = identity.authenticate(TOKEN, true).await.unwrap();
    #[expect(
        clippy::disallowed_methods,
        reason = "stages initial Terms acceptance as the database owner for the browser doctor harness"
    )]
    {
        let mut init_owner = sqlx::PgConnection::connect(&owner_url).await.unwrap();
        sqlx::query("update users set terms_version='2026-09-23' where id=$1")
            .bind(account.account_id)
            .execute(&mut init_owner)
            .await
            .unwrap();
    }
    let journal_path = std::env::temp_dir().join(format!("duet-browser-journal-{suffix}"));
    let workspace = WorkspaceStore::new(database.clone());
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let base = format!("http://{address}");
    let site = build_site_with_bearer(Some(TOKEN));
    let mut server = BackendProcess::start(&runtime_url, &journal_path, &site, &base);
    wait_until_ready(address).await;
    check(&base, &workspace, &account, &owner_url, &mut server).await;
    server.stop();
    drop(workspace);
    drop(database);
    crate::disposable::drop_database_and_role(&mut admin, &name, &role).await;
    std::fs::remove_dir_all(journal_path).unwrap();
}

/// Child-only test entrypoint. Running the parent qualification starts this executable
/// as a distinct process, kills it, and starts a new process against the same database.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "server helper launched by durable browser qualification"]
async fn backend_process() {
    let runtime_url = std::env::var("DUET_BROWSER_CHILD_DATABASE").expect("child configuration");
    let base = std::env::var("DUET_BROWSER_CHILD_ORIGIN").unwrap();
    let journal_path = PathBuf::from(std::env::var("DUET_BROWSER_CHILD_JOURNAL").unwrap());
    let site = PathBuf::from(std::env::var("DUET_BROWSER_CHILD_SITE").unwrap());
    let database = AccountDatabase::connect(&runtime_url).await.unwrap();
    let state = DurableState {
        identity: IdentityStore::new(database.clone(), Arc::new(SyntheticIdentity)),
        evidence: EvidenceStore::new(database.clone()),
        privacy: PrivacyStore::new(
            database.clone(),
            Arc::new(FileJournal::open(journal_path.clone()).await.unwrap()),
        ),
    };
    let workspace = WorkspaceStore::new(database.clone());
    let listener = tokio::net::TcpListener::bind(base.strip_prefix("http://").unwrap())
        .await
        .unwrap();
    let fmd_state = duet_api::for_my_doctor::ForMyDoctorState {
        durable: state.clone(),
        store: duet_api::for_my_doctor::ForMyDoctorStore::new(database.clone(), None),
    };
    let routes = duet_api::durable::router(state.clone())
        .merge(duet_api::evidence_routes::router(state.clone()))
        .merge(duet_api::doctor_routes::router(
            duet_api::doctor_routes::DoctorState {
                durable: state,
                workspace: workspace.clone(),
            },
        ))
        .merge(duet_api::for_my_doctor::router(fmd_state));
    let app =
        axum::Router::new().fallback(move |request: axum::http::Request<axum::body::Body>| {
            let mut routes = routes.clone();
            let site = site.clone();
            async move {
                use tower::Service;
                if request.uri().path().starts_with("/api/") {
                    routes.call(request).await.unwrap()
                } else {
                    static_file(&site, request.uri().path(), "")
                        .await
                        .unwrap_or_else(|_| {
                            axum::http::Response::builder()
                                .status(404)
                                .body(axum::body::Body::empty())
                                .unwrap()
                        })
                }
            }
        });
    axum::serve(listener, app).await.unwrap();
}

struct BackendProcess {
    child: Option<std::process::Child>,
    runtime: String,
    journal: PathBuf,
    site: PathBuf,
    base: String,
}
impl BackendProcess {
    fn start(runtime: &str, journal: &std::path::Path, site: &std::path::Path, base: &str) -> Self {
        let mut result = Self {
            child: None,
            runtime: runtime.into(),
            journal: journal.into(),
            site: site.into(),
            base: base.into(),
        };
        result.spawn();
        result
    }
    fn spawn(&mut self) {
        self.child = Some(
            std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--ignored",
                    "--exact",
                    "durable_doctor::backend_process",
                    "--nocapture",
                ])
                .env("DUET_BROWSER_CHILD_DATABASE", &self.runtime)
                .env("DUET_BROWSER_CHILD_JOURNAL", &self.journal)
                .env("DUET_BROWSER_CHILD_SITE", &self.site)
                .env("DUET_BROWSER_CHILD_ORIGIN", &self.base)
                .spawn()
                .unwrap(),
        );
    }
    fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            child.kill().unwrap();
            child.wait().unwrap();
        }
    }
    async fn restart(&mut self) {
        self.stop();
        self.spawn();
        wait_until_ready(self.base.strip_prefix("http://").unwrap().parse().unwrap()).await;
    }
}
impl Drop for BackendProcess {
    fn drop(&mut self) {
        self.stop();
    }
}

async fn run_browser_checks(
    base: &str,
    _workspace: &WorkspaceStore,
    _account: &duet_api::identity::AccountContext,
    _owner_url: &str,
    server: &mut BackendProcess,
) {
    let mut patient = Browser::launch();
    patient.resize(PHONE);
    patient.text_until(&format!("{base}/?mode=client"), "Shared");
    patient.click("nav.tabs button:nth-of-type(2)");
    patient.wait_for("TO SEND");
    let editor_text = patient.body_text().unwrap();
    for retired in [
        "Name",
        "Date of birth",
        "Cover note",
        "Saved page preview",
        "Check and share PDF",
    ] {
        assert!(
            !editor_text.contains(retired),
            "retired field or control {retired} shown:\n{editor_text}"
        );
    }
    patient.click("button[aria-label='Add note']");
    wait_for_selector(&mut patient, "textarea");
    patient.type_into("textarea", "What changed this week?");
    patient.click("button[aria-label='Save note']");
    patient.wait_for("What changed this week?");

    // Kill the actual Axum process, then rebind the same origin and verify durable state + retired routes.
    server.restart().await;
    patient.text_until(&format!("{base}/?mode=client"), "Shared");
    patient.click("nav.tabs button:nth-of-type(2)");
    patient.wait_for("What changed this week?");

    let http = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .unwrap();
    for retired_url in [
        format!("{base}/api/doctor-collection/pdf"),
        format!("{base}/api/doctor-collection/preview"),
        format!("{base}/api/doctor-collection/share-link"),
        format!("{base}/shared/for-my-doctor/retired-token"),
        format!("{base}/shared/for-my-doctor/retired-token/pdf"),
    ] {
        assert_eq!(
            http.get(&retired_url)
                .bearer_auth(TOKEN)
                .send()
                .await
                .unwrap()
                .status(),
            404,
            "{retired_url} must be retired"
        );
    }

    // Two editors: a stale browser save hits 409 version_conflict, displays the newer saved version,
    // and keeps the browser's unsaved words intact.
    click_text(&mut patient, "Edit");
    wait_for_selector(&mut patient, "textarea");
    patient.type_into("textarea", "Stale browser note words");
    let state: serde_json::Value = http
        .get(format!("{base}/api/for-my-doctor"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let note_id = state["notes"][0]["id"].as_str().unwrap();
    let response = http
        .put(format!("{base}/api/for-my-doctor/notes/{note_id}"))
        .bearer_auth(TOKEN)
        .json(&serde_json::json!({"body": "Newer saved note", "expected_version": 1}))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    patient.click("button[aria-label='Save note']");
    patient.wait_for("Newer saved note");
    let conflict_text = patient.body_text().unwrap();
    assert!(
        conflict_text.contains("Not saved yet") && conflict_text.contains("Newer saved note"),
        "expected conflict UI:\n{conflict_text}"
    );
    let kept_value = patient
        .page()
        .evaluate("document.querySelector('textarea')?.value ?? ''");
    assert_eq!(kept_value, serde_json::json!("Stale browser note words"));
    drop(patient);
}
