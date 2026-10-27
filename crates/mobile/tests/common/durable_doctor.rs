//! Real PostgreSQL + durable routes + compiled patient UI; no stub or live model.
use super::*;
use duet_api::{
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore},
    publication::PublicationStore,
    workspace::WorkspaceStore,
};
use sqlx::{ConnectOptions, Connection};
use uuid::Uuid;

const TOKEN: &str = "synthetic-durable-browser-only";
const PROPOSAL_REQUEST: &str = "Prepare a doctor page proposal for review: set my name to Explicitly approved proposal and add the question Review this proposal? Keep my existing question and do not save or share it.";
const PROPOSAL_REPLY: &str =
    "Your proposed doctor page is ready for review. Nothing has been saved or shared.";

/// Only the test server impersonates the model; admission, tool dispatch, receipts,
/// proposal staging and approval still belong to the real conversation/workspace owners.
fn controlled_proposal(input: &[serde_json::Value]) -> serde_json::Value {
    use serde_json::json;
    let result = |id: &str| {
        input
            .iter()
            .find(|v| v["type"] == "function_call_output" && v["call_id"] == id)
            .map(|v| {
                serde_json::from_str::<serde_json::Value>(v["output"].as_str().unwrap()).unwrap()
            })
    };
    let Some(workspace) = result("browser-workspace") else {
        return json!([{"type":"function_call","call_id":"browser-workspace","name":"read_workspace","arguments":"{}"}]);
    };
    if let Some(proposal) = result("browser-proposal") {
        assert_eq!(proposal["base_generation"], workspace["note_generation"]);
        assert_eq!(proposal["page"]["name"], "Explicitly approved proposal");
        return json!([{"type":"message","content":[{"type":"output_text","text":PROPOSAL_REPLY}]}]);
    }
    // Compose from the actual tool result, never a hard-coded generation or SQL seed.
    let mut page = workspace["page"].clone();
    page["name"] = json!("Explicitly approved proposal");
    page["questions"]
        .as_array_mut()
        .unwrap()
        .push(json!("Review this proposal?"));
    let proposal = json!({"base_generation":workspace["note_generation"],"note":workspace["note"],"items":workspace["items"],"page":page});
    json!([{"type":"function_call","call_id":"browser-proposal","name":"stage_proposal","arguments":proposal.to_string()}])
}
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
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("explicit disposable loopback PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&options.get_host()));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("duet_browser_{suffix}");
    let role = format!("duet_browser_role_{suffix}");
    // `role` and `name` are generated from a UUID simple string ([0-9a-f]).
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
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
    let journal_path = std::env::temp_dir().join(format!("duet-browser-journal-{suffix}"));
    let workspace = WorkspaceStore::new(database.clone());
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let base = format!("http://{address}");
    let site = build_site_with_bearer(Some(TOKEN));
    let mut server = BackendProcess::start(&runtime_url, &journal_path, &site, &base);
    wait_until_ready(address).await;
    run_browser_checks(
        &base,
        &workspace,
        &account,
        &owner_url,
        &runtime_url,
        &mut server,
    )
    .await;
    server.stop();
    drop(workspace);
    drop(database);
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!(
        "drop database {name} with (force)"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    sqlx::raw_sql(sqlx::AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
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
    let publications = PublicationStore::new(
        database.clone(),
        base.clone(),
        format!("{base}/viewer"),
        [42; 32],
    )
    .unwrap();
    let routes = duet_api::durable::router(state.clone())
        .merge(duet_api::evidence_routes::router(state.clone()))
        .merge(duet_api::doctor_routes::router(
            duet_api::doctor_routes::DoctorState {
                durable: state.clone(),
                workspace: workspace.clone(),
                publications,
            },
        ))
        .merge(duet_api::conversation_routes::router(
            duet_api::conversation_routes::ConversationState {
                durable: state,
                store: duet_api::conversation::ConversationStore::new(database.clone()),
                model: "no-live-provider".into(),
                provider: Some((
                    format!("{base}/controlled-response"),
                    "local-synthetic".into(),
                )),
            },
        ));
    let app =
        axum::Router::new().fallback(move |request: axum::http::Request<axum::body::Body>| {
            let mut routes = routes.clone();
            let site = site.clone();
            async move {
                use tower::Service;
                if request.uri().path() == "/controlled-response" {
                    let bytes = axum::body::to_bytes(request.into_body(), 1_000_000).await.unwrap();
                    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                    let input = body["input"].as_array().unwrap();
                    let proposal_request = input.iter().rev().find(|v| v["role"] == "user").is_some_and(|v| v["content"] == PROPOSAL_REQUEST);
                    let output = if proposal_request {
                        controlled_proposal(input)
                    } else if !input.iter().any(|v|v["call_id"] == "browser-read") {
                        let context = input.iter().filter_map(|v|v["content"].as_str()).find(|s|s.starts_with("Selected context and reply reference")).unwrap();
                        let admission: serde_json::Value = serde_json::from_str(context.split_once("): ").unwrap().1).unwrap();
                        if admission["source"]["kind"].is_string() {
                            serde_json::json!([{"type":"function_call","call_id":"browser-read","name":"read_selected_source","arguments":"{}"}])
                        } else {
                            serde_json::json!([{"type":"function_call","call_id":"browser-read","name":"read_health_source","arguments":serde_json::json!({"address":admission["source"]["source"]["address"]}).to_string()}])
                        }
                    } else if !input.iter().any(|v|v["call_id"] == "browser-citation") {
                        serde_json::json!([{"type":"function_call","call_id":"browser-citation","name":"cite_health_sources","arguments":"{\"read_ids\":[\"browser-read\"]}"}])
                    } else {
                        serde_json::json!([{"type":"message","content":[{"type":"output_text","text":"Here is the original record you selected. Nothing has been saved or shared."}]}])
                    };
                    let event = serde_json::json!({"type":"response.completed","response":{"status":"completed","output":output,"usage":{"input_tokens":3,"output_tokens":2}}});
                    if proposal_request && output[0]["type"] == "message" {
                        let chunks = futures_util::stream::unfold(0, move |step| {
                            let event = event.clone();
                            async move {
                                match step {
                                    0 => Some((Ok::<_, std::io::Error>(format!("data: {}\n\n", serde_json::json!({"type":"response.output_text.delta","delta":"Controlled provisional preview"}))), 1)),
                                    1 => {
                                        tokio::time::sleep(Duration::from_secs(4)).await;
                                        Some((Ok(format!("data: {event}\n\n")), 2))
                                    },
                                    _ => None,
                                }
                            }
                        });
                        return axum::http::Response::builder().header("content-type", "text/event-stream").body(axum::body::Body::from_stream(chunks)).unwrap();
                    }
                    return axum::http::Response::builder().header("content-type", "text/event-stream").body(axum::body::Body::from(format!("data: {event}\n\n"))).unwrap();
                }
                if request.uri().path() == "/independent-reader" {
                    return axum::http::Response::builder().header("content-type", "text/html").body(axum::body::Body::from("<!doctype html><title>Independent SHL reader</title><p>Independent reader</p>")).unwrap();
                }
                if request.uri().path().starts_with("/api/")
                    || request.uri().path().starts_with("/shared/")
                    || request.uri().path().starts_with("/shl/")
                    || request.uri().path().starts_with("/shl-file/")
                {
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

async fn prove_conversation_source(
    base: &str,
    account: &duet_api::identity::AccountContext,
    owner_url: &str,
    patient: &mut Browser,
) {
    use serde_json::json;
    let mut admin = sqlx::PgConnection::connect(owner_url).await.unwrap();
    sqlx::query("update users set intelligence_enabled=true,luna_consent_decision='accepted',luna_consent_version='2026-09-24',terms_version='2026-09-23' where id=$1").bind(account.account_id).execute(&mut admin).await.unwrap();
    let http = reqwest::Client::new();
    let record = Uuid::new_v4();
    let mut mutation = json!({"operation_id":Uuid::new_v4(),"canonical_record_id":record,"record_id":"browser-literal-note","entity":"clinical_note","payload":{"title":"Retained browser note","body":"Original exact browser wording","source_label":"Synthetic records"}});
    let address: serde_json::Value = http
        .post(format!("{base}/api/canonical-evidence"))
        .bearer_auth(TOKEN)
        .json(&mutation)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let id = Uuid::new_v4();
    let submission = json!({"id":id,"text":"Explain the selected original note","timezone":"UTC","selected_context":{"kind":"record","record_id":record,"revision":address["revision"]}});
    let accepted = http
        .post(format!("{base}/api/conversation/messages"))
        .bearer_auth(TOKEN)
        .json(&submission)
        .send()
        .await
        .unwrap();
    assert_eq!(accepted.status(), 202, "{}", accepted.text().await.unwrap());
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let page = loop {
        let page: serde_json::Value = http
            .get(format!("{base}/api/conversation/messages"))
            .bearer_auth(TOKEN)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        if page["messages"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["role"] == "assistant")
        {
            break page;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "controlled conversation never completed: {page}"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    };
    let assistant = page["messages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["role"] == "assistant")
        .unwrap();
    let detail_url = format!(
        "{base}/api/conversation/messages/{}/sources/browser-read",
        assistant["id"].as_str().unwrap()
    );
    assert_eq!(http.get(&detail_url).send().await.unwrap().status(), 401);
    http.put(format!("{base}/api/auth/account"))
        .json(&json!({}))
        .bearer_auth("synthetic-foreign-browser-only")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    assert_eq!(
        http.get(&detail_url)
            .bearer_auth("synthetic-foreign-browser-only")
            .send()
            .await
            .unwrap()
            .status(),
        404
    );
    let original: serde_json::Value = http
        .get(&detail_url)
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(original["superseded"], false);
    mutation["operation_id"] = json!(Uuid::new_v4());
    mutation["expected_revision"] = address["revision"].clone();
    mutation["payload"]["body"] = json!("Corrected current browser wording");
    http.post(format!("{base}/api/canonical-evidence"))
        .bearer_auth(TOKEN)
        .json(&mutation)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    let old: serde_json::Value = http
        .get(&detail_url)
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(old["superseded"], true);
    assert_eq!(old["text"], original["text"]);
    assert!(old["current_revision"].as_i64().unwrap() > address["revision"].as_i64().unwrap());
    // Exact retry is retained despite correction; changed submission is rejected.
    assert_eq!(
        http.post(format!("{base}/api/conversation/messages"))
            .bearer_auth(TOKEN)
            .json(&submission)
            .send()
            .await
            .unwrap()
            .status(),
        202
    );
    let mut changed = submission.clone();
    changed["timezone"] = json!("America/New_York");
    assert_eq!(
        http.post(format!("{base}/api/conversation/messages"))
            .bearer_auth(TOKEN)
            .json(&changed)
            .send()
            .await
            .unwrap()
            .status(),
        409
    );
    changed["id"] = json!(Uuid::new_v4());
    assert_eq!(
        http.post(format!("{base}/api/conversation/messages"))
            .bearer_auth(TOKEN)
            .json(&changed)
            .send()
            .await
            .unwrap()
            .status(),
        409
    );
    patient.text_until(&format!("{base}/?mode=client"), "Today");
    patient.click("nav.tabs button:nth-of-type(1)");
    patient.wait_for("Here is the original record you selected.");
    // The final source chip belongs to the assistant, not the selected user attachment.
    patient.page().evaluate("(() => { const chips = [...document.querySelectorAll('button[aria-label=\"Open source Retained browser note\"]')]; chips[ chips.length-1 ].click(); return true; })()");
    patient.wait_for("Original exact browser wording");
    patient.wait_for("A newer version of this record is available.");
    assert!(
        !patient
            .body_text()
            .unwrap()
            .contains("Corrected current browser wording")
    );
    patient.click("button[aria-label='Close source']");
    let boundary = Uuid::new_v4();
    let snapshot = Uuid::new_v4();
    let cycle = Uuid::new_v4();
    let insight = Uuid::new_v4();
    let fact = json!({"id":"steps:day","kind":"current_value","window":"day","concept_ids":["steps"],"title":"Frozen browser measurement","range_start":"2026-10-19T00:00:00Z","range_end":"2026-10-20T00:00:00Z","value":7654.0,"evidence":[{"id":"stat:steps","kind":"stat","title":"Steps","source_kind":"appleHealth"}]});
    sqlx::query("insert into health_contexts(user_id,snapshot_id,source_boundary_id,generated_at,timezone,projections) values($1,$2,$3,now(),'UTC','[{\"kind\":\"daily\"}]')").bind(account.account_id).bind(snapshot).bind(boundary).execute(&mut admin).await.unwrap();
    sqlx::query("insert into insight_cycles(id,user_id,context_snapshot_id,kind,window_start,window_end,timezone,first_weekday,status,insights,supporting_facts,observed_through,generated_at) values($1,$2,$3,'daily','2026-10-19','2026-10-20','UTC',1,'ready',$4,$5,now(),now())").bind(cycle).bind(account.account_id).bind(snapshot).bind(json!([{"id":insight,"title":"Frozen browser observation","detail":"Original published browser observation","evidence":[],"fact_ids":["steps:day"]}])).bind(json!([fact])).execute(&mut admin).await.unwrap();
    for (selected, title, text) in [
        (
            json!({"kind":"measurement","source_boundary_id":boundary,"health_context_revision":1,"source":{"kind":"appleHealth"},"measurement_id":"steps","period_start":"2026-10-19T00:00:00Z","period_end":"2026-10-20T00:00:00Z","fact":fact}),
            "Frozen browser measurement",
            "7654",
        ),
        (
            json!({"kind":"published_insight","cycle_id":cycle,"insight_id":insight}),
            "Frozen browser observation",
            "Original published browser observation",
        ),
    ] {
        let id = Uuid::new_v4();
        http.post(format!("{base}/api/conversation/messages")).bearer_auth(TOKEN).json(&json!({"id":id,"text":"Explain this retained context","timezone":"UTC","selected_context":selected})).send().await.unwrap().error_for_status().unwrap();
        let assistant_id = Uuid::from_u128(id.as_u128() ^ (1u128 << 127));
        let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
        loop {
            let page: serde_json::Value = http
                .get(format!("{base}/api/conversation/messages"))
                .bearer_auth(TOKEN)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            if page["messages"]
                .as_array()
                .unwrap()
                .iter()
                .any(|m| m["id"] == json!(assistant_id))
            {
                break;
            }
            assert!(
                tokio::time::Instant::now() < deadline,
                "retained context completion: {page}"
            );
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        patient.text_until(&format!("{base}/?mode=client"), "Today");
        patient.click("nav.tabs button:nth-of-type(1)");
        patient.wait_for(title);
        let label = format!("Open source {title}");
        patient.page().evaluate(&format!("(() => {{ const chips=[...document.querySelectorAll('button')].filter(b=>b.getAttribute('aria-label')==={label:?}); chips[chips.length-1].click();return true; }})()"));
        patient.wait_for(text);
        patient.click("button[aria-label='Close source']");
        let detail =
            format!("{base}/api/conversation/messages/{assistant_id}/sources/browser-read");
        assert_eq!(
            http.get(&detail)
                .bearer_auth("synthetic-foreign-browser-only")
                .send()
                .await
                .unwrap()
                .status(),
            404
        );
    }
    let workspace: serde_json::Value = http
        .get(format!("{base}/api/doctor-collection"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(workspace["note_generation"], 0);
    assert!(workspace["items"].as_array().unwrap().is_empty());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("select count(*) from duet_publications")
            .fetch_one(&mut admin)
            .await
            .unwrap(),
        0
    );
    // Withdraw through the actual owner: persisted delivered prose must not reopen.
    let restriction = json!({"event_id":Uuid::new_v4(),"operation":"evidence_delete","scope_id":record,"scope_version":old["current_revision"]});
    let deleted = http
        .post(format!("{base}/api/auth/privacy-restrictions"))
        .bearer_auth(TOKEN)
        .json(&restriction)
        .send()
        .await
        .unwrap();
    assert!(
        deleted.status().is_success(),
        "{}",
        deleted.text().await.unwrap()
    );
    assert_eq!(
        http.get(&detail_url)
            .bearer_auth(TOKEN)
            .send()
            .await
            .unwrap()
            .status(),
        404
    );
    patient.page().evaluate("(() => { const chips = [...document.querySelectorAll('button[aria-label=\"Open source Retained browser note\"]')]; chips[chips.length-1].click(); return true; })()");
    patient.wait_for("couldn’t");
    patient.click("button[aria-label='Close source']");
}

async fn run_browser_checks(
    base: &str,
    workspace: &WorkspaceStore,
    account: &duet_api::identity::AccountContext,
    owner_url: &str,
    _runtime_url: &str,
    server: &mut BackendProcess,
) {
    let mut patient = Browser::launch();
    patient.resize(PHONE);
    prove_conversation_source(base, account, owner_url, &mut patient).await;
    patient.text_until(&format!("{base}/?mode=client"), "Today");
    patient.click("nav.tabs button:nth-of-type(1)");
    patient.wait_for("Open For my doctor");
    click_text(&mut patient, "Open For my doctor");
    patient.click("button[aria-label='Edit page']");
    patient.type_into("input[aria-label='Name']", "Synthetic Ada");
    patient.click("button[aria-label='Add a question']");
    wait_for_selector(&mut patient, "input[aria-label='Question']");
    patient.type_into("input[aria-label='Question']", "What changed this week?");
    patient.click("button[aria-label='Save page']");
    patient.wait_for("Saved page preview");
    let saved = workspace.read(account).await.unwrap();
    assert_eq!(saved["page"]["name"], "Synthetic Ada");
    patient.text_until(&format!("{base}/?mode=client"), "Today");
    patient.click("nav.tabs button:nth-of-type(1)");
    patient.wait_for("Open For my doctor");
    click_text(&mut patient, "Open For my doctor");
    patient.wait_for("Saved page preview");
    patient.page().evaluate("(() => { const s = [...document.querySelectorAll('summary')].find(s => s.textContent === 'Read complete page text'); s.click(); return true; })()");
    patient.wait_for("Synthetic Ada");
    // Observe the normal browser download boundary without writing PHI to disk.
    patient.page().evaluate("(() => { const original = HTMLAnchorElement.prototype.click; HTMLAnchorElement.prototype.click = function() { if (this.download === 'for-my-doctor.pdf') { window.__doctorDownload = this.download; return; } return original.call(this); }; return true; })()");
    click_text(&mut patient, "Check and share PDF");
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    loop {
        if patient
            .page()
            .evaluate("window.__doctorDownload || ''")
            .as_str()
            == Some("for-my-doctor.pdf")
        {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "PDF export never reached download: {:?}",
            patient.body_text()
        );
        std::thread::sleep(Duration::from_millis(100));
    }
    patient.wait_for("Publish reviewed page…");
    click_text(&mut patient, "Publish reviewed page…");
    click_text(&mut patient, "Confirm publish");
    patient.wait_for("Revoke link…");
    // This proof kills the backend between reads; do not reuse sockets owned
    // by the old process when asserting durable revocation after restart.
    let http = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .unwrap();
    let status: serde_json::Value = http
        .get(format!("{base}/api/doctor-collection/share-link"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let link = status["web_url"].as_str().expect("published web URL");
    let original_pdf = http
        .get(format!("{link}/pdf"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .bytes()
        .await
        .unwrap();
    // Editing the workspace must not silently replace the approved publication.
    patient.click("button[aria-label='Edit page']");
    patient.type_into("input[aria-label='Name']", "Later patient wording");
    patient.click("button[aria-label='Save page']");
    patient.wait_for("Saved page preview");
    assert_eq!(
        workspace.read(account).await.unwrap()["page"]["name"],
        "Later patient wording"
    );
    assert_eq!(
        http.get(format!("{link}/pdf"))
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap(),
        original_pdf
    );
    // Kill the actual Axum process after approval/edit, then rebind the same origin.
    server.restart().await;
    assert_eq!(
        http.get(format!("{link}/pdf"))
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap(),
        original_pdf
    );
    patient.text_until(&format!("{base}/?mode=client"), "Today");
    patient.click("nav.tabs button:nth-of-type(1)");
    patient.wait_for("Open For my doctor");
    click_text(&mut patient, "Open For my doctor");
    patient.wait_for("Revoke link…");
    let mut doctor = Browser::launch();
    let text = doctor.text_until(link, "Synthetic Ada");
    assert!(text.contains("What changed this week?"));
    // The public HTML is intentionally sandboxed. Use a separate no-login reader origin page,
    // not a weakening of the publication CSP, to exercise the independent decoder.
    doctor.text_until(&format!("{base}/independent-reader"), "Independent reader");
    // A separate browser decrypts using WebCrypto, independently of Rust's JWE implementation.
    let shl = status["shl_url"].as_str().unwrap();
    let expression = format!(
        r#"(async () => {{
        const decode = s => Uint8Array.from(atob(s.replace(/-/g, '+').replace(/_/g, '/')), c => c.charCodeAt(0));
        const link = JSON.parse(new TextDecoder().decode(decode({shl:?}.split('#shlink:/')[1])));
        const manifestResponse = await fetch(link.url, {{method:'POST', headers:{{'Content-Type':'application/json'}}, body:JSON.stringify({{recipient:'Synthetic doctor'}})}});
        if (!manifestResponse.ok) throw new Error('manifest ' + manifestResponse.status);
        const manifest = await manifestResponse.json();
        const file = manifest.files[0];
        const parts = (await (await fetch(file.location)).text()).split('.');
        const header = JSON.parse(new TextDecoder().decode(decode(parts[0])));
        if (parts.length !== 5 || parts[1] !== '' || header.alg !== 'dir' || header.enc !== 'A256GCM') throw new Error('JWE header');
        const key = await crypto.subtle.importKey('raw', decode(link.key), 'AES-GCM', false, ['decrypt']);
        const encrypted = new Uint8Array([...decode(parts[3]), ...decode(parts[4])]);
        const clear = await crypto.subtle.decrypt({{name:'AES-GCM', iv:decode(parts[2]), additionalData:new TextEncoder().encode(parts[0]), tagLength:128}}, key, encrypted);
        const resource = JSON.parse(new TextDecoder().decode(clear));
        const attachment = resource.content[0].attachment;
        const pdf = decode(attachment.data);
        const publicPdf = new Uint8Array(await (await fetch({link:?} + '/pdf')).arrayBuffer());
        return {{kind:resource.resourceType, mime:attachment.contentType, same:pdf.length === publicPdf.length && pdf.every((b,i) => b === publicPdf[i]), file:file.location}};
    }})()"#
    );
    let decoded = doctor.page().call_session(
        "Runtime.evaluate",
        serde_json::json!({"expression": expression, "awaitPromise":true, "returnByValue":true}),
    );
    assert!(
        decoded.pointer("/result/exceptionDetails").is_none(),
        "independent SHL decode: {decoded}"
    );
    let decoded = &decoded["result"]["result"]["value"];
    assert_eq!(decoded["kind"], "DocumentReference");
    assert_eq!(decoded["mime"], "application/pdf");
    assert_eq!(decoded["same"], true);
    let file_url = decoded["file"].as_str().unwrap().to_owned();
    assert_eq!(
        http.get(format!("{base}/api/doctor-collection"))
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    click_text(&mut patient, "Revoke link…");
    click_text(&mut patient, "Confirm revoke");
    patient.wait_for("This publication was revoked.");
    assert!(!http.get(link).send().await.unwrap().status().is_success());
    assert!(
        !http
            .get(&file_url)
            .send()
            .await
            .unwrap()
            .status()
            .is_success(),
        "file dereference must recheck revoke after manifest retrieval"
    );
    server.restart().await;
    assert!(!http.get(link).send().await.unwrap().status().is_success());
    assert!(
        !http
            .get(&file_url)
            .send()
            .await
            .unwrap()
            .status()
            .is_success()
    );
    // Publish another generation, retrieve its manifest, then advance only the
    // disposable database expiry (not wall-clock or production policy).
    let current = workspace.read(account).await.unwrap();
    let expired: serde_json::Value = http.post(format!("{base}/api/doctor-collection/share-link"))
        .bearer_auth(TOKEN).header("Idempotency-Key", Uuid::new_v4().to_string())
        .json(&serde_json::json!({"expected_generation":current["note_generation"],"expires_in_seconds":60,"operation_id":Uuid::new_v4()}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    let expired_web = expired["web_url"].as_str().unwrap();
    doctor.text_until(expired_web, "Later patient wording");
    doctor.text_until(&format!("{base}/independent-reader"), "Independent reader");
    let expired_shl = expired["shl_url"].as_str().unwrap();
    let manifest_url = doctor.page().evaluate(&format!("JSON.parse(atob({expired_shl:?}.split('#shlink:/')[1].replace(/-/g,'+').replace(/_/g,'/'))).url"));
    let manifest_url = manifest_url.as_str().unwrap();
    let manifest: serde_json::Value = http
        .post(manifest_url)
        .json(&serde_json::json!({"recipient":"Synthetic doctor"}))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let expired_file = manifest["files"][0]["location"].as_str().unwrap();
    let mut expiry_owner = sqlx::PgConnection::connect(owner_url).await.unwrap();
    sqlx::query("update duet_publications set expires_at=now()-interval '1 second' where id=$1")
        .bind(expired["id"].as_str().unwrap().parse::<Uuid>().unwrap())
        .execute(&mut expiry_owner)
        .await
        .unwrap();
    server.restart().await;
    let result = doctor.page().call_session("Runtime.evaluate", serde_json::json!({"expression":format!("Promise.all([fetch({expired_web:?}),fetch({expired_web:?}+'/pdf'),fetch({expired_file:?}),fetch({manifest_url:?},{{method:'POST',headers:{{'Content-Type':'application/json'}},body:JSON.stringify({{recipient:'Synthetic doctor'}})}})].map(async r => (await r).status))"),"awaitPromise":true,"returnByValue":true}));
    let statuses = result["result"]["result"]["value"]
        .as_array()
        .expect("expiry browser fetch results");
    assert_eq!(statuses.len(), 4);
    assert!(
        statuses
            .iter()
            .all(|s| s.as_u64().is_some_and(|n| n == 404 || n == 410)),
        "expired capabilities: {result}"
    );
    drop(expiry_owner);
    // A person message must traverse ConversationOwner::execute and the local
    // Responses transport; no direct stage_proposal call or proposal SQL insertion.
    let mut owner = sqlx::PgConnection::connect(owner_url).await.unwrap();
    let current = workspace.read(account).await.unwrap();
    let publication_count: i64 = sqlx::query_scalar("select count(*) from duet_publications")
        .fetch_one(&mut owner)
        .await
        .unwrap();
    let message_id = Uuid::new_v4();
    let assistant_id = Uuid::from_u128(message_id.as_u128() ^ (1u128 << 127));
    let submission = serde_json::json!({"id":message_id,"text":PROPOSAL_REQUEST,"timezone":"UTC"});
    let accepted = http
        .post(format!("{base}/api/conversation/messages"))
        .bearer_auth(TOKEN)
        .json(&submission)
        .send()
        .await
        .unwrap();
    assert_eq!(accepted.status(), 202, "{}", accepted.text().await.unwrap());
    patient.text_until(&format!("{base}/?mode=client"), "Today");
    patient.click("nav.tabs button:nth-of-type(1)");
    patient.wait_for("Controlled provisional preview");
    patient.wait_for("Luna — drafting, not yet saved");
    let pending: serde_json::Value = http
        .get(format!("{base}/api/conversation/messages"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(
        !pending["messages"]
            .to_string()
            .contains("Controlled provisional preview")
    );
    patient.wait_for(PROPOSAL_REPLY);
    assert!(
        !patient
            .body_text()
            .unwrap()
            .contains("Controlled provisional preview")
    );
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    loop {
        let messages: serde_json::Value = http
            .get(format!("{base}/api/conversation/messages"))
            .bearer_auth(TOKEN)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        if let Some(assistant) = messages["messages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["id"] == serde_json::json!(assistant_id))
        {
            assert_eq!(assistant["role"], "assistant");
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "executor proposal never completed: {messages}"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    let effects: Vec<(String, serde_json::Value)> = sqlx::query_as(
        "select call->>'name',result from duet_tool_effects where user_id=$1 and command_id=$2 order by call->>'name'"
    ).bind(account.account_id).bind(message_id).fetch_all(&mut owner).await.unwrap();
    assert_eq!(
        effects.len(),
        2,
        "exactly the executor read and stage effects"
    );
    assert_eq!(effects[0].0, "read_workspace");
    assert_eq!(effects[0].1, current);
    assert_eq!(effects[1].0, "stage_proposal");
    let proposal = &effects[1].1;
    assert_eq!(proposal["run_id"], serde_json::json!(message_id));
    assert_eq!(proposal["message_id"], serde_json::json!(message_id));
    assert_eq!(proposal["base_generation"], current["note_generation"]);
    assert_eq!(
        proposal["page"]["questions"],
        serde_json::json!(["What changed this week?", "Review this proposal?"])
    );
    assert_eq!(
        sqlx::query_scalar::<_, String>("select state from duet_commands where id=$1")
            .bind(message_id)
            .fetch_one(&mut owner)
            .await
            .unwrap(),
        "complete"
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "select count(*) from duet_doctor_proposals where user_id=$1 and run_id=$2"
        )
        .bind(account.account_id)
        .bind(message_id)
        .fetch_one(&mut owner)
        .await
        .unwrap(),
        1
    );
    let staged = workspace.read(account).await.unwrap();
    assert_eq!(staged["proposal"], *proposal);
    for key in ["note_generation", "page", "items", "note"] {
        assert_eq!(staged[key], current[key], "staging must not save {key}");
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("select count(*) from duet_publications")
            .fetch_one(&mut owner)
            .await
            .unwrap(),
        publication_count
    );
    patient.text_until(&format!("{base}/?mode=client"), "Today");
    patient.click("nav.tabs button:nth-of-type(1)");
    patient.wait_for(PROPOSAL_REPLY);
    patient.wait_for("Review proposed doctor page");
    click_text(&mut patient, "Review proposed doctor page");
    patient.wait_for("Proposed page — not saved");
    patient.wait_for("Explicitly approved proposal");
    patient.wait_for("What changed this week?");
    patient.wait_for("Review this proposal?");
    assert_eq!(
        workspace.read(account).await.unwrap()["note_generation"],
        current["note_generation"]
    );
    assert_eq!(
        workspace.read(account).await.unwrap()["page"]["name"],
        "Later patient wording"
    );
    click_text(&mut patient, "Approve and save reviewed proposal");
    wait_until(&mut patient, |text| {
        !text.contains("Proposed page — not saved")
    });
    click_text(&mut patient, "Open For my doctor");
    patient.wait_for("Saved page preview");
    assert_eq!(
        workspace.read(account).await.unwrap()["page"]["name"],
        "Explicitly approved proposal"
    );
    let approved = workspace.read(account).await.unwrap();
    assert_eq!(
        approved["note_generation"].as_i64().unwrap(),
        current["note_generation"].as_i64().unwrap() + 1
    );
    assert!(approved["proposal"].is_null());
    assert_eq!(approved["page"], proposal["page"]);
    assert_eq!(approved["items"], current["items"]);
    assert_eq!(approved["note"], current["note"]);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("select count(*) from duet_publications")
            .fetch_one(&mut owner)
            .await
            .unwrap(),
        publication_count,
        "approval is not publication"
    );
    // Reload from the server: the approved proposal is now the saved page, not
    // a browser-local preview, and both the existing and new question survive.
    patient.text_until(&format!("{base}/?mode=client"), "Today");
    patient.click("nav.tabs button:nth-of-type(1)");
    patient.wait_for("Open For my doctor");
    assert!(
        !patient
            .body_text()
            .unwrap()
            .contains("Review proposed doctor page")
    );
    click_text(&mut patient, "Open For my doctor");
    patient.wait_for("Saved page preview");
    patient.page().evaluate("(() => { const s = [...document.querySelectorAll('summary')].find(s => s.textContent === 'Read complete page text'); s.click(); return true; })()");
    patient.wait_for("Explicitly approved proposal");
    patient.wait_for("What changed this week?");
    patient.wait_for("Review this proposal?");
    assert!(
        !patient
            .body_text()
            .unwrap()
            .contains("Proposed page — not saved")
    );
    assert_eq!(workspace.read(account).await.unwrap(), approved);
    // Oversize content saves successfully but remains unexportable and unpublishable.
    patient.click("button[aria-label='Edit page']");
    patient.type_into(
        "input[aria-label='Question']",
        &"An intentionally long full question. ".repeat(600),
    );
    patient.click("button[aria-label='Save page']");
    patient.wait_for("The complete page cannot be exported faithfully on one page.");
    assert_eq!(patient.page().evaluate("[...document.querySelectorAll('button')].find(b => b.textContent === 'Check and share PDF').disabled"), serde_json::json!(true));
    assert!(
        !http
            .get(format!("{base}/api/doctor-collection/pdf"))
            .bearer_auth(TOKEN)
            .send()
            .await
            .unwrap()
            .status()
            .is_success()
    );
    // Two editors: a stale browser save cannot overwrite a newer patient-approved generation.
    patient.click("button[aria-label='Edit page']");
    patient.type_into("input[aria-label='Name']", "Stale browser wording");
    let current = workspace.read(account).await.unwrap();
    let response = http.put(format!("{base}/api/doctor-collection/page")).bearer_auth(TOKEN)
        .json(&serde_json::json!({"expected_generation":current["note_generation"], "item_ids":[], "page":{"name":"Newer approved wording","questions":["Current question"]}}))
        .send().await.unwrap();
    assert!(response.status().is_success());
    patient.click("button[aria-label='Save page']");
    patient.wait_for("The last save may have succeeded.");
    assert_eq!(
        workspace.read(account).await.unwrap()["page"]["name"],
        "Newer approved wording"
    );
    assert!(
        patient
            .body_text()
            .unwrap()
            .contains("Your exact draft is retained")
    );
    drop(owner);
    drop(patient);
    drop(doctor);
}
