//! Real-Postgres verification of multi-device generic push notifications,
//! Duet AI foundations on `crates/harness` + `crates/clinical-library`,
//! 30-day memory version retention, default-off AI consent, separate research
//! consent, and parked legacy route quarantine.

#![allow(
    clippy::disallowed_methods,
    reason = "integration test provisions isolated disposable databases and verifies final SQL state"
)]

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use chrono::{Duration, Utc};
use clinical_library::corpus::{
    LatencySummary, LifecycleState, ReadEvidenceUnit, ReadItemOutcome, SearchCandidate, SearchPage,
    SearchResult, sha256_hex,
};
use clinical_library::freshness::{LibraryFreshnessReport, LibraryStanding};
use duet_api::ApiError;
use duet_api::assistant_harness::{
    AssistantBlockReason, AssistantEligibility, AssistantFoundationSnapshot,
    AssistantFoundationStore, AssistantOperatorSwitch, AssistantState, AssistantTurnInput,
    ClinicalLibraryReadError, ClinicalLibraryReader, ParkedRoutesPolicy, ResearchConsentState,
    STANDING_AI_DISCLOSURE, TOOL_CLINICAL_LIBRARY_SEARCH,
};
use duet_api::database::{AccountDatabase, migrate};
use duet_api::durable::DurableState;
use duet_api::evidence::EvidenceStore;
use duet_api::identity::{IdentityStore, IdentityVerifier, VerifiedIdentity};
use duet_api::privacy::{FileJournal, PrivacyStore, Restriction};
use duet_api::push::{
    GenericPushDispatch, PushState, PushStore, PushTransport, PushTransportError,
};
use duet_notify::GenericAlertKind;
use harness::engine::recipe::{Observer, Provider, Request as HarnessRequest, Usage};
use harness::engine::runs::RunOutcome;
use harness::engine::{Cancellation, Invocation, Item, RunLimits, Turn};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection};
use tower::ServiceExt;
use uuid::Uuid;

#[derive(Clone)]
struct FixedIdentity {
    auth_time: Arc<Mutex<chrono::DateTime<Utc>>>,
}

#[async_trait]
impl IdentityVerifier for FixedIdentity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        let subject = match bearer {
            "a" => "cod67-user-a",
            "b" => "cod67-user-b",
            _ => return Err(ApiError::Unauthorized),
        };
        let authenticated_at = *self.auth_time.lock().unwrap();
        Ok(VerifiedIdentity {
            subject: subject.to_owned(),
            authenticated_at,
        })
    }
}

#[derive(Default)]
struct RecordingPushTransport {
    sent: Mutex<Vec<GenericPushDispatch>>,
    fail_next: Mutex<Option<PushTransportError>>,
}

impl RecordingPushTransport {
    fn dispatches(&self) -> Vec<GenericPushDispatch> {
        self.sent.lock().unwrap().clone()
    }

    fn set_next_error(&self, error: PushTransportError) {
        *self.fail_next.lock().unwrap() = Some(error);
    }
}

#[async_trait]
impl PushTransport for RecordingPushTransport {
    async fn send(&self, dispatch: &GenericPushDispatch) -> Result<(), PushTransportError> {
        if let Some(err) = self.fail_next.lock().unwrap().take() {
            return Err(err);
        }
        self.sent.lock().unwrap().push(dispatch.clone());
        Ok(())
    }
}

struct StubClinicalLibrary;

#[async_trait]
impl ClinicalLibraryReader for StubClinicalLibrary {
    async fn search(
        &self,
        _query: &str,
        _cursor: Option<&str>,
    ) -> Result<SearchResult, ClinicalLibraryReadError> {
        Ok(SearchResult {
            candidates: vec![SearchCandidate {
                candidate_id: "11111111-1111-1111-1111-111111111111".into(),
                source_code: "pubmed".into(),
                source_display_name: "PubMed".into(),
                native_document_id: "12345678".into(),
                title: "Glycemic targets in adults".into(),
            }],
            page: SearchPage {
                examined_candidate_count: 1,
                returned_candidate_count: 1,
                unavailable_candidate_count: 0,
                truncated: false,
            },
            next_cursor: None,
        })
    }

    async fn read_items(
        &self,
        candidate_ids: &[String],
    ) -> Result<Vec<ReadItemOutcome>, ClinicalLibraryReadError> {
        let passage = "HbA1c targets are individualized with a clinician.";
        Ok(candidate_ids
            .iter()
            .map(|id| ReadItemOutcome::Available {
                candidate_id: id.clone(),
                unit: Box::new(ReadEvidenceUnit {
                    bibliographic_observations: None,
                    unit_id: id.clone(),
                    version_id: "v-2026-11-19".into(),
                    source_code: "pubmed".into(),
                    source_display_name: "PubMed".into(),
                    native_document_id: "12345678".into(),
                    native_version_id: None,
                    source_version_label: None,
                    title: "Glycemic targets in adults".into(),
                    document_type: "guideline".into(),
                    publication_date: Some("2026-11-19".into()),
                    version_date: Some("2026-11-19".into()),
                    acquired_at: "2026-11-19T00:00:00Z".into(),
                    lifecycle_state: LifecycleState::Current,
                    source_status_code: None,
                    native_unit_id: "u1".into(),
                    native_locator: "sec-1".into(),
                    unit_kind: "abstract".into(),
                    ordinal: 0,
                    label: Some("Summary".into()),
                    content: passage.into(),
                    content_sha256: sha256_hex(passage.as_bytes()),
                    native_destination_uri: "https://pubmed.ncbi.nlm.nih.gov/12345678/".into(),
                    version_destination_uri: "https://pubmed.ncbi.nlm.nih.gov/12345678/".into(),
                    rights_decision_id: "rd-1".into(),
                    license_identifier: None,
                    attribution_text: None,
                    may_display_excerpt: true,
                    identifiers: Vec::new(),
                    relationships: Vec::new(),
                }),
            })
            .collect())
    }

    async fn freshness(&self) -> Result<LibraryFreshnessReport, ClinicalLibraryReadError> {
        Ok(LibraryFreshnessReport {
            standing: LibraryStanding::Current,
            active_version: Some("v-2026-11-19".into()),
            last_night_recorded_at: Some("2026-11-19T00:00:00Z".into()),
            last_night_outcome: Some("succeeded".into()),
            sources: Vec::new(),
            open_alerts: 0,
            search_latency: LatencySummary {
                endpoint: "search".into(),
                sample_count: 1,
                p50_ms: 12.0,
                p95_ms: 15.0,
                p99_ms: 18.0,
                within_targets: Some(true),
            },
        })
    }
}

struct ScriptedProvider {
    turns: VecDeque<Turn>,
}

impl Provider for ScriptedProvider {
    fn exchange(
        &mut self,
        request: &HarnessRequest,
        _cancel: &Cancellation,
        _limits: &RunLimits,
        _observations: &mut dyn Observer,
    ) -> impl std::future::Future<
        Output = Result<Turn, Box<dyn std::error::Error + Send + Sync + 'static>>,
    > + Send {
        let standing_ok = request.instructions.contains(STANDING_AI_DISCLOSURE);
        let next = self.turns.pop_front();
        async move {
            if !standing_ok {
                return Err("missing standing disclosure in harness request".into());
            }
            next.ok_or_else(|| "no scripted provider turn remaining".into())
        }
    }
}

struct HarnessFixture {
    db_name: String,
    runtime_role: String,
    admin_url: String,
    owner_url: String,
    database: AccountDatabase,
    durable: DurableState,
    journal_dir: std::path::PathBuf,
    auth_time: Arc<Mutex<chrono::DateTime<Utc>>>,
}

impl HarnessFixture {
    async fn setup(prefix: &str) -> Self {
        let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
            .expect("real disposable loopback PostgreSQL required");
        let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
        assert!(matches!(
            options.get_host(),
            "localhost" | "127.0.0.1" | "::1"
        ));
        let suffix = Uuid::new_v4().simple().to_string();
        let db_name = format!("{prefix}_{suffix}");
        let runtime_role = format!("{prefix}_rt_{suffix}");
        let mut admin = sqlx::PgConnection::connect(&admin_url).await.unwrap();
        sqlx::query(AssertSqlSafe(format!("create role {runtime_role} login")))
            .execute(&mut admin)
            .await
            .unwrap();
        sqlx::query(AssertSqlSafe(format!("create database {db_name}")))
            .execute(&mut admin)
            .await
            .unwrap();
        let db_options = options.database(&db_name);
        let owner_url = db_options.to_url_lossy().to_string();
        migrate(&owner_url, &runtime_role).await.unwrap();
        let runtime_url = db_options
            .username(&runtime_role)
            .to_url_lossy()
            .to_string();
        let database = AccountDatabase::connect(&runtime_url).await.unwrap();
        let journal_dir = std::env::temp_dir().join(format!("{prefix}-journal-{suffix}"));
        let auth_time = Arc::new(Mutex::new(Utc::now() - Duration::seconds(30)));
        let durable = DurableState {
            identity: IdentityStore::new(
                database.clone(),
                Arc::new(FixedIdentity {
                    auth_time: auth_time.clone(),
                }),
            ),
            privacy: PrivacyStore::new(
                database.clone(),
                Arc::new(FileJournal::open(journal_dir.clone()).await.unwrap()),
            ),
            evidence: EvidenceStore::new(database.clone()),
        };
        Self {
            db_name,
            runtime_role,
            admin_url,
            owner_url,
            database,
            durable,
            journal_dir,
            auth_time,
        }
    }

    async fn cleanup(self) {
        drop(self.durable);
        drop(self.database);
        let _ = std::fs::remove_dir_all(&self.journal_dir);
        let mut admin = sqlx::PgConnection::connect(&self.admin_url).await.unwrap();
        let _ = sqlx::query(AssertSqlSafe(format!(
            "drop database if exists {} with (force)",
            self.db_name
        )))
        .execute(&mut admin)
        .await;
        let _ = sqlx::query(AssertSqlSafe(format!(
            "drop role if exists {}",
            self.runtime_role
        )))
        .execute(&mut admin)
        .await;
    }
}

async fn http(app: &Router, method: &str, path: &str, who: &str, body: Value) -> (u16, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("authorization", format!("Bearer {who}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status().as_u16();
    let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({ "text": String::from_utf8_lossy(&bytes) })),
    )
}

#[tokio::test]
async fn cod67_multi_device_generic_push_and_sign_out_cleanup() {
    let fx = HarnessFixture::setup("cod67_push").await;
    let transport = Arc::new(RecordingPushTransport::default());
    let push_store = PushStore::new(fx.database.clone(), transport.clone());
    let app =
        duet_api::durable::router(fx.durable.clone()).merge(duet_api::push::router(PushState {
            durable: fx.durable.clone(),
            push: push_store.clone(),
        }));

    for who in ["a", "b"] {
        let (status, _) = http(&app, "PUT", "/api/auth/account", who, json!({})).await;
        assert_eq!(status, StatusCode::OK.as_u16());
    }

    // Register two devices for user A (iOS APNs + Android FCM) and one for user B.
    let (s1, _) = http(
        &app,
        "PUT",
        "/api/push/devices",
        "a",
        json!({
            "device_id": "iphone-a",
            "platform": "apns",
            "push_token": "apns-token-user-a-device-1"
        }),
    )
    .await;
    assert_eq!(s1, StatusCode::OK.as_u16());
    let (s2, _) = http(
        &app,
        "PUT",
        "/api/push/devices",
        "a",
        json!({
            "device_id": "android-a",
            "platform": "fcm",
            "push_token": "fcm-token-user-a-device-2"
        }),
    )
    .await;
    assert_eq!(s2, StatusCode::OK.as_u16());
    let (s3, _) = http(
        &app,
        "PUT",
        "/api/push/devices",
        "b",
        json!({
            "device_id": "iphone-b",
            "platform": "apns",
            "push_token": "apns-token-user-b-device-1"
        }),
    )
    .await;
    assert_eq!(s3, StatusCode::OK.as_u16());

    // Listing devices enforces RLS isolation and never exposes raw push tokens.
    let (list_status, list_a) = http(&app, "GET", "/api/push/devices", "a", json!(null)).await;
    assert_eq!(list_status, StatusCode::OK.as_u16());
    let list_a_arr = list_a.as_array().unwrap();
    assert_eq!(list_a_arr.len(), 2);
    assert!(!list_a.to_string().contains("apns-token-user-a-device-1"));
    assert!(!list_a.to_string().contains("iphone-b"));

    let ctx_a = fx.durable.identity.authenticate("a", false).await.unwrap();
    let ctx_b = fx.durable.identity.authenticate("b", false).await.unwrap();

    // Fan out one generic lock-screen alert to every active device on A's account.
    let summary = push_store
        .deliver_generic_alert(ctx_a.account_id, GenericAlertKind::DoctorMessage)
        .await
        .unwrap();
    assert_eq!(summary.delivered_devices, vec!["iphone-a", "android-a"]);
    assert!(summary.pending_devices.is_empty());
    let dispatches = transport.dispatches();
    assert_eq!(dispatches.len(), 2);
    for dispatch in &dispatches {
        assert_eq!(dispatch.title, "Duet");
        assert_eq!(dispatch.body, "You have a new message in Duet.");
        assert_ne!(dispatch.token.as_str(), "apns-token-user-b-device-1");
    }

    // Queue a pending alert on iphone-a (transient failure), then sign out of iphone-a.
    transport.set_next_error(PushTransportError::Unavailable("gateway timeout".into()));
    let partial = push_store
        .deliver_generic_alert(ctx_a.account_id, GenericAlertKind::NewRecord)
        .await
        .unwrap();
    assert_eq!(partial.pending_devices, vec!["iphone-a"]);
    assert_eq!(partial.delivered_devices, vec!["android-a"]);
    assert_eq!(
        push_store
            .pending_alert_count(ctx_a.account_id)
            .await
            .unwrap(),
        1
    );

    let (del_status, cleanup) = http(
        &app,
        "DELETE",
        "/api/push/devices/iphone-a",
        "a",
        json!(null),
    )
    .await;
    assert_eq!(del_status, StatusCode::OK.as_u16());
    assert_eq!(cleanup["cleared_devices"], 1);
    assert_eq!(cleanup["cancelled_pending_alerts"], 1);
    assert_eq!(
        push_store
            .pending_alert_count(ctx_a.account_id)
            .await
            .unwrap(),
        0
    );

    // Queue a pending alert on android-a, then revoke all sessions via PrivacyStore::SessionsRevoke.
    transport.set_next_error(PushTransportError::Unavailable("transient".into()));
    let _ = push_store
        .deliver_generic_alert(ctx_a.account_id, GenericAlertKind::AppointmentReminder)
        .await
        .unwrap();
    assert_eq!(
        push_store
            .pending_alert_count(ctx_a.account_id)
            .await
            .unwrap(),
        1
    );
    fx.durable
        .privacy
        .restrict(
            &ctx_a,
            Uuid::new_v4(),
            Restriction::SessionsRevoke,
            ctx_a.account_id,
            0,
        )
        .await
        .unwrap();
    assert_eq!(
        push_store
            .pending_alert_count(ctx_a.account_id)
            .await
            .unwrap(),
        0
    );

    // After a fresh sign-in, register a device and verify AccountDelete cascades everything for A
    // while leaving B's registered device intact.
    *fx.auth_time.lock().unwrap() = Utc::now() + Duration::seconds(5);
    let fresh_a = fx.durable.identity.authenticate("a", false).await.unwrap();
    let (s_new, _) = http(
        &app,
        "PUT",
        "/api/push/devices",
        "a",
        json!({
            "device_id": "iphone-a-relogin",
            "platform": "apns",
            "push_token": "apns-token-relogin"
        }),
    )
    .await;
    assert_eq!(s_new, StatusCode::OK.as_u16());
    fx.durable
        .privacy
        .restrict(
            &fresh_a,
            Uuid::new_v4(),
            Restriction::AccountDelete,
            fresh_a.account_id,
            0,
        )
        .await
        .unwrap();

    let mut owner = sqlx::PgConnection::connect(&fx.owner_url).await.unwrap();
    let remaining_a: i64 =
        sqlx::query_scalar("select count(*) from public.duet_push_devices where user_id = $1")
            .bind(fresh_a.account_id)
            .fetch_one(&mut owner)
            .await
            .unwrap();
    let remaining_b: i64 =
        sqlx::query_scalar("select count(*) from public.duet_push_devices where user_id = $1")
            .bind(ctx_b.account_id)
            .fetch_one(&mut owner)
            .await
            .unwrap();
    assert_eq!(remaining_a, 0);
    assert_eq!(remaining_b, 1);

    fx.cleanup().await;
}

#[tokio::test]
async fn cod67_harness_clinical_library_consent_and_memory_retention() {
    let fx = HarnessFixture::setup("cod67_ai").await;
    let assistant_store = AssistantFoundationStore::new(
        fx.database.clone(),
        AssistantOperatorSwitch::On,
        ParkedRoutesPolicy::Quarantined,
        Some(Arc::new(StubClinicalLibrary)),
    );
    let app = duet_api::durable::router(fx.durable.clone()).merge(
        duet_api::assistant_harness::router(AssistantState {
            durable: fx.durable.clone(),
            assistant: assistant_store.clone(),
        }),
    );

    // 1. Fresh account defaults intelligence_enabled = false and research consent = off.
    let (status, account_body) = http(&app, "PUT", "/api/auth/account", "a", json!({})).await;
    assert_eq!(status, StatusCode::OK.as_u16());
    assert_eq!(account_body["intelligence_enabled"], false);

    let (f_status, f_body) = http(&app, "GET", "/api/assistant/foundation", "a", json!(null)).await;
    assert_eq!(f_status, StatusCode::OK.as_u16());
    let foundation: AssistantFoundationSnapshot = serde_json::from_value(f_body).unwrap();
    assert_eq!(foundation.standing_disclosure, STANDING_AI_DISCLOSURE);
    assert!(foundation.ai_processing_disclosed);
    assert!(!foundation.intelligence_enabled);
    assert_eq!(
        foundation.eligibility,
        AssistantEligibility::Blocked {
            reason: AssistantBlockReason::TermsNotAccepted,
        }
    );
    assert_eq!(foundation.research_consent, ResearchConsentState::Off);
    assert_eq!(foundation.memory_version_retention_days, 30);
    assert_eq!(foundation.parked_routes, ParkedRoutesPolicy::Quarantined);

    // Accept Terms: intelligence_enabled stays false until explicit AI consent.
    let (t_status, _) = http(
        &app,
        "PUT",
        "/api/auth/account/terms",
        "a",
        json!({ "version": "2026-09-23" }),
    )
    .await;
    assert_eq!(t_status, StatusCode::OK.as_u16());

    let ctx_a = fx.durable.identity.authenticate("a", false).await.unwrap();
    let after_terms = assistant_store.snapshot(&ctx_a).await.unwrap();
    assert_eq!(
        after_terms.eligibility,
        AssistantEligibility::Blocked {
            reason: AssistantBlockReason::IntelligenceConsentRequired,
        }
    );

    // 2. Separate opt-in research consent toggles independently without enabling AI intelligence.
    let (r_grant, grant_body) = http(
        &app,
        "PUT",
        "/api/assistant/research-consent",
        "a",
        json!({ "action": "grant", "expected_revision": 0 }),
    )
    .await;
    assert_eq!(r_grant, StatusCode::OK.as_u16());
    let granted: AssistantFoundationSnapshot = serde_json::from_value(grant_body).unwrap();
    assert!(matches!(
        granted.research_consent,
        ResearchConsentState::Granted { .. }
    ));
    assert_eq!(granted.research_consent_revision, 1);
    assert!(!granted.intelligence_enabled);

    // Stale revision conflicts honestly.
    let (r_conflict, _) = http(
        &app,
        "PUT",
        "/api/assistant/research-consent",
        "a",
        json!({ "action": "withdraw", "expected_revision": 0 }),
    )
    .await;
    assert_eq!(r_conflict, StatusCode::CONFLICT.as_u16());

    let (r_withdraw, withdraw_body) = http(
        &app,
        "PUT",
        "/api/assistant/research-consent",
        "a",
        json!({ "action": "withdraw", "expected_revision": 1 }),
    )
    .await;
    assert_eq!(r_withdraw, StatusCode::OK.as_u16());
    let withdrawn: AssistantFoundationSnapshot = serde_json::from_value(withdraw_body).unwrap();
    assert!(matches!(
        withdrawn.research_consent,
        ResearchConsentState::Withdrawn { .. }
    ));

    // 3. Harness execution fails closed before explicit AI consent.
    let mut blocked_provider = ScriptedProvider {
        turns: VecDeque::new(),
    };
    let denied = assistant_store
        .run_harness_turn(
            &ctx_a,
            AssistantTurnInput {
                run_id: "run-before-consent",
                model: "gpt-5.4",
                prompt: "What does HbA1c mean?",
            },
            &mut blocked_provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await;
    assert!(matches!(denied, Err(ApiError::Forbidden)));

    // Grant explicit AI consent in DB (simulating future AI consent surface), then run harness + clinical-library.
    let mut owner = sqlx::PgConnection::connect(&fx.owner_url).await.unwrap();
    sqlx::query(
        "update public.users set intelligence_enabled = true, luna_consent_decision = 'accepted', \
         luna_consent_version = '2026-09-24', luna_consent_at = now() where id = $1",
    )
    .bind(ctx_a.account_id)
    .execute(&mut owner)
    .await
    .unwrap();

    let mut provider = ScriptedProvider {
        turns: VecDeque::from([
            Turn {
                items: vec![Item::Call(Invocation {
                    id: "call-lib-1".into(),
                    name: TOOL_CLINICAL_LIBRARY_SEARCH.into(),
                    arguments: json!({ "query": "HbA1c targets" }).to_string(),
                })],
                receipt: json!({ "response_id": "resp-1" }),
                usage: Usage {
                    input_tokens: Some(120),
                    cached_input_tokens: None,
                    output_tokens: Some(30),
                },
            },
            Turn {
                items: vec![Item::Message {
                    role: "assistant".into(),
                    text: "HbA1c reflects average blood glucose over roughly three months; discuss personal targets with your clinician.".into(),
                }],
                receipt: json!({ "response_id": "resp-2" }),
                usage: Usage {
                    input_tokens: Some(180),
                    cached_input_tokens: Some(60),
                    output_tokens: Some(45),
                },
            },
        ]),
    };

    let run = assistant_store
        .run_harness_turn(
            &ctx_a,
            AssistantTurnInput {
                run_id: "run-grounded-1",
                model: "gpt-5.4",
                prompt: "What does HbA1c mean?",
            },
            &mut provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(run.outcome, RunOutcome::Completed);
    assert!(run.reply_text.as_deref().unwrap().contains("three months"));
    assert_eq!(run.standing_disclosure, STANDING_AI_DISCLOSURE);
    assert!(run.event_count >= 4);

    let persisted = assistant_store
        .read_harness_run(&ctx_a, "run-grounded-1")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(persisted.0, "completed");
    assert_eq!(persisted.1, run.event_count);

    // 4. 30-day rolling retention for superseded private memory versions (`R.Leave.7`).
    // Set t0 = now - 45d so v1 (expires t0 + 40d = now - 5d) is due in duet_memory_retention_due
    // while v2 (expires t0 + 55d = now + 10d) remains within its 30-day retention window.
    let t0 = Utc::now() - Duration::days(45);
    assert_eq!(
        assistant_store
            .write_memory(&ctx_a, json!({ "note": "version 1" }), t0)
            .await
            .unwrap(),
        1
    );
    // Writing v2 at t0 + 10d archives v1 with expires_at = t0 + 40d.
    assert_eq!(
        assistant_store
            .write_memory(
                &ctx_a,
                json!({ "note": "version 2" }),
                t0 + Duration::days(10)
            )
            .await
            .unwrap(),
        2
    );
    // Writing v3 at t0 + 25d archives v2 with expires_at = t0 + 55d.
    assert_eq!(
        assistant_store
            .write_memory(
                &ctx_a,
                json!({ "note": "version 3" }),
                t0 + Duration::days(25)
            )
            .await
            .unwrap(),
        3
    );

    // At t0 + 39d: both historical versions are < 30 days old; pruning deletes 0.
    assert_eq!(
        assistant_store
            .prune_expired_memory_versions(ctx_a.account_id, t0 + Duration::days(39))
            .await
            .unwrap(),
        0
    );
    let kept_at_39 = assistant_store
        .retained_memory_versions(&ctx_a, t0 + Duration::days(39))
        .await
        .unwrap();
    assert_eq!(kept_at_39.len(), 2);

    // Content-free operational directory exposes expired accounts for background pruning.
    let due_before = fx.database.accounts_with_expired_memory().await.unwrap();
    assert!(
        due_before.contains(&ctx_a.account_id),
        "expired memory version (t0 + 40d < now) must appear in duet_memory_retention_due"
    );

    // At t0 + 41d: v1 expired (31 days after archive), v2 and current v3 stay intact.
    assert_eq!(
        assistant_store
            .prune_expired_memory_versions(ctx_a.account_id, t0 + Duration::days(41))
            .await
            .unwrap(),
        1
    );
    let due_after = fx.database.accounts_with_expired_memory().await.unwrap();
    assert!(
        !due_after.contains(&ctx_a.account_id),
        "pruned account must leave duet_memory_retention_due once no expired rows remain"
    );
    let kept_at_41 = assistant_store
        .retained_memory_versions(&ctx_a, t0 + Duration::days(41))
        .await
        .unwrap();
    assert_eq!(kept_at_41.len(), 1);
    assert_eq!(kept_at_41[0].revision, 2);
    let current = assistant_store
        .current_memory(&ctx_a)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(current.0, 3);
    assert_eq!(current.1, json!({ "note": "version 3" }));

    // Account deletion cascades private memory, archived versions, AI preferences, and harness runs.
    fx.durable
        .privacy
        .restrict(
            &ctx_a,
            Uuid::new_v4(),
            Restriction::AccountDelete,
            ctx_a.account_id,
            0,
        )
        .await
        .unwrap();
    for table in [
        "duet_private_memory",
        "duet_private_memory_versions",
        "duet_ai_preferences",
        "duet_harness_runs",
    ] {
        let count: i64 = sqlx::query_scalar(AssertSqlSafe(format!(
            "select count(*) from public.{table} where user_id = $1"
        )))
        .bind(ctx_a.account_id)
        .fetch_one(&mut owner)
        .await
        .unwrap();
        assert_eq!(count, 0, "{table} must cascade on AccountDelete");
    }

    fx.cleanup().await;
}

#[tokio::test]
async fn cod67_parked_legacy_routes_are_quarantined_by_default() {
    let fx = HarnessFixture::setup("cod67_parked").await;
    let app = duet_api::durable::router(fx.durable.clone());

    let (s_account, _) = http(&app, "PUT", "/api/auth/account", "a", json!({})).await;
    assert_eq!(s_account, StatusCode::OK.as_u16());

    for (method, path, body) in [
        (
            "PUT",
            "/api/auth/account/today-stats",
            json!({ "stat_ids": ["steps"] }),
        ),
        (
            "PUT",
            "/api/auth/account/luna-consent",
            json!({ "version": "2026-09-24", "expected_revision": 0 }),
        ),
        ("GET", "/api/health-context", json!(null)),
        ("POST", "/api/health-context", json!({})),
        ("POST", "/api/health-context/refresh", json!({})),
    ] {
        let (status, _) = http(&app, method, path, "a", body).await;
        assert_eq!(
            status,
            StatusCode::NOT_FOUND.as_u16(),
            "{method} {path} must be quarantined out of default production routes"
        );
    }

    // Legacy `genome_withdraw` privacy receipts fail closed instead of silently retiring.
    let event_id = Uuid::new_v4();
    let legacy_genome = json!({
        "schema_version": 1,
        "event_id": event_id,
        "account_id": Uuid::new_v4(),
        "operation": "genome_withdraw",
        "scope_id": Uuid::new_v4(),
        "scope_version": "",
        "occurred_at": Utc::now(),
        "state": "restriction_committed"
    });
    std::fs::write(
        fx.journal_dir.join(format!("{event_id}.json")),
        serde_json::to_vec(&legacy_genome).unwrap(),
    )
    .unwrap();
    assert!(
        fx.durable.privacy.reconcile_inventory().await.is_err(),
        "retired genome_withdraw receipt must fail closed"
    );

    fx.cleanup().await;
}
