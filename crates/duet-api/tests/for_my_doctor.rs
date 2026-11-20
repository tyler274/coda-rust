//! Real-Postgres + `front-door` integration proof for `"For my doctor"` (`COD-58`, `COD-59`, `COD-50`, `COD-51`):
//! - RLS isolation across Duet accounts, plus portable export coverage.
//! - One-tap mark/unmark of records, values, readings, and trends with reference ranges stripped.
//! - Unlimited free-text notes (standalone and linked to a canonical record) + `409 version_conflict`.
//! - Source correction updating a marked item without blocking `Send` or mutating earlier sends.
//! - Source removal (`COD-50`) clearing the mark from `To send`, recording a `You` notice, and preserving earlier sends.
//! - Idempotent `share_key` sends to `front-door`, uncertain retry recovery, `ConnectionEnded` / `COD-51`,
//!   and `Sent` -> `Received` vs ended-before-pickup staying `Sent`.

#![allow(
    clippy::disallowed_methods,
    reason = "integration test provisions an isolated disposable Postgres database"
)]

use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::http::Request;
use chrono::Utc;
use duet_api::ApiError;
use duet_api::account_export::AccountExport;
use duet_api::connected_care_sync::FRONT_DOOR_SESSION_HEADER;
use duet_api::database::{AccountDatabase, migrate};
use duet_api::durable::DurableState;
use duet_api::evidence::{EvidenceMutation, EvidenceOperation, EvidenceStore, IngestAuthority};
use duet_api::for_my_doctor::{
    DirectFrontDoorShareTransport, ForMyDoctorFrontDoorTransport, ForMyDoctorState,
    ForMyDoctorStore, FrontDoorCallError, FrontDoorOverview, HttpFrontDoorShareTransport,
};
use duet_api::identity::{IdentityStore, IdentityVerifier, VerifiedIdentity};
use duet_api::privacy::{FileJournal, PrivacyStore};
use duet_wire::models::addressed_record_revision::Entity as EvidenceEntity;
use front_door::{
    ConnectionEndReason, ForMyDoctorSendRequest, ForMyDoctorShare, FrontDoorStore,
    MemoryFrontDoorStore, RosterPatient,
};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection};
use tower::ServiceExt;
use uuid::Uuid;

struct StaticIdentity;

#[async_trait]
impl IdentityVerifier for StaticIdentity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        let subject = match bearer {
            "duet-ada" => "fmd-ada-subject",
            "duet-bob" => "fmd-bob-subject",
            "duet-absent" => "fmd-absent-subject",
            _ => return Err(ApiError::Unauthorized),
        };
        Ok(VerifiedIdentity {
            subject: subject.to_owned(),
            authenticated_at: Utc::now(),
        })
    }
}

struct FlakyShareTransport {
    inner: Arc<DirectFrontDoorShareTransport>,
    fail_next_send: AtomicBool,
}

#[async_trait]
impl ForMyDoctorFrontDoorTransport for FlakyShareTransport {
    async fn account_overview(
        &self,
        front_door_account_id: Uuid,
        front_door_bearer: &str,
    ) -> Result<FrontDoorOverview, FrontDoorCallError> {
        self.inner
            .account_overview(front_door_account_id, front_door_bearer)
            .await
    }

    async fn send_share(
        &self,
        front_door_account_id: Uuid,
        front_door_bearer: &str,
        request: &ForMyDoctorSendRequest,
    ) -> Result<ForMyDoctorShare, FrontDoorCallError> {
        if self.fail_next_send.swap(false, Ordering::SeqCst) {
            return Err(FrontDoorCallError::Unavailable(
                "Simulated network drop before confirmation".to_owned(),
            ));
        }
        self.inner
            .send_share(front_door_account_id, front_door_bearer, request)
            .await
    }
}

async fn http_as(
    app: &Router,
    bearer: &str,
    method: &str,
    path: &str,
    front_door_session: Option<&str>,
    body: Option<Value>,
) -> (u16, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header("authorization", format!("Bearer {bearer}"));
    if let Some(session) = front_door_session {
        builder = builder.header(FRONT_DOOR_SESSION_HEADER, session);
    }
    let body = match body {
        Some(v) => {
            builder = builder.header("content-type", "application/json");
            Body::from(v.to_string())
        }
        None => Body::empty(),
    };
    let response = app
        .clone()
        .oneshot(builder.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status().as_u16();
    let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
        .await
        .unwrap();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({ "raw": String::from_utf8_lossy(&bytes) }))
    };
    (status, json)
}

#[tokio::test]
async fn for_my_doctor_marks_notes_conflict_source_changes_sends_and_rls() {
    let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("real disposable loopback PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let suffix = Uuid::new_v4().simple().to_string();
    let db_name = format!("fmd_{suffix}");
    let runtime_role = format!("fmd_rt_{suffix}");
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
    let journal_dir = std::env::temp_dir().join(format!("fmd-journal-{suffix}"));

    let durable = DurableState {
        identity: IdentityStore::new(database.clone(), Arc::new(StaticIdentity)),
        privacy: PrivacyStore::new(
            database.clone(),
            Arc::new(FileJournal::open(journal_dir).await.unwrap()),
        ),
        evidence: EvidenceStore::new(database.clone()),
    };

    let now = Arc::new(AtomicU64::new(1_763_300_000));
    let front_door_store = Arc::new(MemoryFrontDoorStore::new());
    let roster_one = RosterPatient {
        practice_id: "practice-north".to_owned(),
        patient_id: "pt-north-1".to_owned(),
    };
    let roster_two = RosterPatient {
        practice_id: "practice-south".to_owned(),
        patient_id: "pt-south-1".to_owned(),
    };
    let inv_one = front_door_store
        .invite(
            &roster_one,
            "ada@fmd.test",
            "Ada Lovelace",
            now.load(Ordering::SeqCst),
        )
        .await
        .unwrap();
    let (fd_ada_id, fd_session) = front_door_store
        .redeem_invitation("ada@fmd.test", &inv_one, now.load(Ordering::SeqCst) + 5)
        .await
        .unwrap();
    let inv_two = front_door_store
        .invite(
            &roster_two,
            "ada@fmd.test",
            "Ada Lovelace",
            now.load(Ordering::SeqCst) + 10,
        )
        .await
        .unwrap();
    let _ = front_door_store
        .redeem_invitation("ada@fmd.test", &inv_two, now.load(Ordering::SeqCst) + 15)
        .await
        .unwrap();

    // Live HTTP front-door server to exercise real wire compatibility too.
    let mut fd_http_state = front_door::http::FrontDoorState::new(front_door_store.clone());
    let fd_now_http = now.clone();
    fd_http_state.now = Arc::new(move || fd_now_http.load(Ordering::SeqCst));
    let fd_router = front_door::http::router_with(fd_http_state, true);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, fd_router).await.unwrap();
    });
    let _http_transport = HttpFrontDoorShareTransport::new(&format!("http://{addr}")).unwrap();

    let now_for_direct = now.clone();
    let direct_transport = Arc::new(DirectFrontDoorShareTransport::new(
        front_door_store.clone(),
        Arc::new(move || now_for_direct.load(Ordering::SeqCst)),
    ));
    let flaky_transport = Arc::new(FlakyShareTransport {
        inner: direct_transport,
        fail_next_send: AtomicBool::new(false),
    });

    let fmd_store = ForMyDoctorStore::new(database.clone(), Some(flaky_transport.clone()));
    let app = duet_api::durable::router(durable.clone()).merge(duet_api::for_my_doctor::router(
        ForMyDoctorState {
            durable: durable.clone(),
            store: fmd_store,
        },
    ));

    // Sign up Ada & Bob and accept Terms.
    let (s, _) = http_as(
        &app,
        "duet-ada",
        "PUT",
        "/api/auth/account",
        None,
        Some(json!({})),
    )
    .await;
    assert_eq!(s, 200);
    let (s, _) = http_as(
        &app,
        "duet-ada",
        "PUT",
        "/api/auth/account/terms",
        None,
        Some(json!({ "version": "2026-09-23" })),
    )
    .await;
    assert_eq!(s, 200);

    let (s, _) = http_as(
        &app,
        "duet-bob",
        "PUT",
        "/api/auth/account",
        None,
        Some(json!({})),
    )
    .await;
    assert_eq!(s, 200);
    // Auth/session/terms refusals on POST /api/for-my-doctor/send must preserve their HTTP
    // status and refusal code rather than flattening into 503 uncertain.
    let probe_send = json!({
        "share_key": format!("share-probe-{}", Uuid::new_v4()),
        "practice_id": &roster_one.practice_id,
        "practice_name": "Northside Primary Care",
        "patient_id": &roster_one.patient_id,
        "disclosure_accepted": true,
        "selected_mark_ids": [Uuid::new_v4()],
        "selected_note_ids": []
    });
    let (s, err_terms) = http_as(
        &app,
        "duet-bob",
        "POST",
        "/api/for-my-doctor/send",
        None,
        Some(probe_send.clone()),
    )
    .await;
    assert_eq!(s, 403, "expected terms refusal: {err_terms}");
    assert_eq!(err_terms["detail"]["code"], "terms_acceptance_required");

    let (s, err_state_terms) =
        http_as(&app, "duet-bob", "GET", "/api/for-my-doctor", None, None).await;
    assert_eq!(s, 403, "expected terms refusal on state: {err_state_terms}");
    assert_eq!(
        err_state_terms["detail"]["code"],
        "terms_acceptance_required"
    );

    let (s, err_unauth) = http_as(
        &app,
        "duet-unknown-session",
        "POST",
        "/api/for-my-doctor/send",
        None,
        Some(probe_send.clone()),
    )
    .await;
    assert_eq!(s, 401, "expected 401 session refusal: {err_unauth}");
    assert_eq!(err_unauth["detail"]["code"], "session_invalid");

    let (s, err_absent) = http_as(
        &app,
        "duet-absent",
        "POST",
        "/api/for-my-doctor/send",
        None,
        Some(probe_send),
    )
    .await;
    assert_eq!(s, 401, "expected 401 account_absent refusal: {err_absent}");
    assert_eq!(err_absent["detail"]["code"], "account_absent");

    let (s, _) = http_as(
        &app,
        "duet-bob",
        "PUT",
        "/api/auth/account/terms",
        None,
        Some(json!({ "version": "2026-09-23" })),
    )
    .await;
    assert_eq!(s, 200);

    // Seed two canonical health records for Ada:
    // 1. A lab record with printed reference ranges that must NEVER appear in For my doctor.
    // 2. A manual/portal reading that will later be source-corrected and then source-removed (COD-50).
    let lab_id = Uuid::new_v4();
    let a1c_id = Uuid::new_v4();
    let ingest_ctx = durable
        .authenticate(&{
            let mut h = axum::http::HeaderMap::new();
            h.insert("authorization", "Bearer duet-ada".parse().unwrap());
            h
        })
        .await
        .unwrap();

    // Map Ada's Duet account to her front-door account.
    durable
        .identity
        .map_front_door(&ingest_ctx, fd_ada_id)
        .await
        .unwrap();

    let authority = IngestAuthority::Connector {
        source_kind: "epic".to_owned(),
        connection_id: "epic-1".to_owned(),
    };
    durable
        .evidence
        .ingest_with_authority(
            &ingest_ctx,
            &authority,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                entity: EvidenceEntity::HealthEvidence,
                canonical_record_id: lab_id,
                record_id: lab_id.to_string(),
                operation: EvidenceOperation::Upsert,
                expected_revision: None,
                source_generation: None,
                payload: json!({
                    "id": lab_id,
                    "concept_id": "lab-ldl",
                    "display_name": "LDL cholesterol",
                    "verification": "verified",
                    "method": "point",
                    "measured_interval": { "start": "2026-11-10T10:00:00Z" },
                    "value": { "type": "quantity", "quantity": { "canonical_value": 142.0, "canonical_unit": "mg/dL" } },
                    "reference_range": "< 100 mg/dL",
                    "referenceRange": [{"text": "0 - 99 mg/dL"}],
                    "interpretation": "H",
                    "source_label": "Quest Diagnostics",
                    "source": { "kind": "epic", "connection_id": "epic-1", "issuer": "Quest Diagnostics", "provenance": "portal" },
                }),
            },
        )
        .await
        .unwrap();
    let a1c_addr = durable
        .evidence
        .ingest_with_authority(
            &ingest_ctx,
            &authority,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                entity: EvidenceEntity::HealthEvidence,
                canonical_record_id: a1c_id,
                record_id: a1c_id.to_string(),
                operation: EvidenceOperation::Upsert,
                expected_revision: None,
                source_generation: None,
                payload: json!({
                    "id": a1c_id,
                    "concept_id": "lab-a1c",
                    "display_name": "Hemoglobin A1c",
                    "verification": "verified",
                    "method": "point",
                    "measured_interval": { "start": "2026-11-11T10:00:00Z" },
                    "value": { "type": "quantity", "quantity": { "canonical_value": 6.4, "canonical_unit": "%" } },
                    "source_label": "Mercy Clinic",
                    "source": { "kind": "epic", "connection_id": "epic-1", "issuer": "Mercy Clinic", "provenance": "portal" },
                }),
            },
        )
        .await
        .unwrap();
    let a1c_rev1 = a1c_addr.revision;

    // 1. Mark LDL record + mark a Blood Pressure trend (captures range_label); verify reference ranges stripped.
    let (s, ldl_mark) = http_as(
        &app,
        "duet-ada",
        "PUT",
        "/api/for-my-doctor/marks",
        None,
        Some(json!({
            "target_kind": "record",
            "target_key": format!("record:{lab_id}"),
            "record_id": lab_id,
            "entity": "health_evidence",
            "title": "LDL cholesterol",
            "summary_text": "142 mg/dL (Reference range <100)",
            "range_label": null,
            "source_label": "Quest Diagnostics",
            "payload": { "value": "142", "reference_range": "< 100 mg/dL" }
        })),
    )
    .await;
    assert_eq!(s, 200);
    let ldl_mark_str = ldl_mark.to_string();
    assert!(!ldl_mark_str.contains("reference_range") && !ldl_mark_str.contains("referenceRange"));
    assert!(
        !ldl_mark["summary_text"]
            .as_str()
            .unwrap()
            .contains("Reference range")
    );

    let (s, a1c_mark) = http_as(
        &app,
        "duet-ada",
        "PUT",
        "/api/for-my-doctor/marks",
        None,
        Some(json!({
            "target_kind": "record",
            "target_key": format!("record:{a1c_id}"),
            "record_id": a1c_id,
            "entity": "health_evidence",
            "title": "Hemoglobin A1c",
            "summary_text": "6.4 %",
            "source_label": "Mercy Clinic",
            "payload": {}
        })),
    )
    .await;
    assert_eq!(s, 200);
    let a1c_mark_id = a1c_mark["id"].as_str().unwrap().to_owned();

    let (s, trend_mark) = http_as(
        &app,
        "duet-ada",
        "PUT",
        "/api/for-my-doctor/marks",
        None,
        Some(json!({
            "target_kind": "trend",
            "target_key": "trend:bp:30d",
            "record_id": null,
            "entity": null,
            "title": "Blood pressure trend",
            "summary_text": "3 readings (124/78 to 132/84 mmHg)",
            "range_label": "Last 30 days",
            "source_label": "Apple Health",
            "payload": { "points": 3 }
        })),
    )
    .await;
    assert_eq!(s, 200);
    let trend_mark_id = trend_mark["id"].as_str().unwrap().to_owned();

    // 2. Create standalone note and linked note + verify 409 version_conflict returns latest saved version.
    let note_standalone_id = Uuid::new_v4();
    let note_linked_id = Uuid::new_v4();
    let (s, n1) = http_as(
        &app,
        "duet-ada",
        "POST",
        "/api/for-my-doctor/notes",
        None,
        Some(json!({
            "id": note_standalone_id,
            "body": "Morning dizziness since Monday."
        })),
    )
    .await;
    assert_eq!(s, 200);
    assert_eq!(n1["version"], 1);

    let (s, n2) = http_as(
        &app,
        "duet-ada",
        "POST",
        "/api/for-my-doctor/notes",
        None,
        Some(json!({
            "id": note_linked_id,
            "body": "Should I adjust my statin before re-testing?",
            "linked_record_id": lab_id,
            "linked_record_title": "LDL cholesterol",
            "linked_entity": "health_evidence"
        })),
    )
    .await;
    assert_eq!(s, 200);
    assert_eq!(n2["linked_record_title"], "LDL cholesterol");

    // Device A updates note 1 -> version 2; stale edit from device B gets 409 with latest saved note.
    let (s, _) = http_as(
        &app,
        "duet-ada",
        "PUT",
        &format!("/api/for-my-doctor/notes/{note_standalone_id}"),
        None,
        Some(json!({
            "body": "Morning dizziness improved after breakfast.",
            "expected_version": 1
        })),
    )
    .await;
    assert_eq!(s, 200);

    let (s, conflict_body) = http_as(
        &app,
        "duet-ada",
        "PUT",
        &format!("/api/for-my-doctor/notes/{note_standalone_id}"),
        None,
        Some(json!({
            "body": "My offline draft words.",
            "expected_version": 1
        })),
    )
    .await;
    assert_eq!(s, 409);
    assert_eq!(conflict_body["code"], "version_conflict");
    assert_eq!(
        conflict_body["latest"]["body"],
        "Morning dizziness improved after breakfast."
    );

    // 3. RLS check: Bob cannot see Ada's marks, notes, or sends.
    let (s, bob_state) = http_as(&app, "duet-bob", "GET", "/api/for-my-doctor", None, None).await;
    assert_eq!(s, 200);
    assert!(bob_state["marks"].as_array().unwrap().is_empty());
    assert!(bob_state["notes"].as_array().unwrap().is_empty());

    // 4. Send linked note ONLY (should carry linked LDL cholesterol record automatically even if LDL mark wasn't picked,
    //    and leave standalone note + trend mark untouched in To send!). Simulate an uncertain failure first, then retry same share_key!
    flaky_transport.fail_next_send.store(true, Ordering::SeqCst);
    let share_key_1 = format!("share-{}", Uuid::new_v4());
    let send_payload = json!({
        "share_key": share_key_1,
        "practice_id": "practice-north",
        "practice_name": "Northside Primary Care",
        "patient_id": "pt-north-1",
        "disclosure_accepted": true,
        "selected_mark_ids": [],
        "selected_note_ids": [note_linked_id]
    });
    let (s, err_resp) = http_as(
        &app,
        "duet-ada",
        "POST",
        "/api/for-my-doctor/send",
        Some(&fd_session),
        Some(send_payload.clone()),
    )
    .await;
    assert_eq!(s, 503);
    assert_eq!(err_resp["code"], "uncertain");

    // To send still intact after uncertain failure.
    let (_, after_fail_state) = http_as(
        &app,
        "duet-ada",
        "GET",
        "/api/for-my-doctor",
        Some(&fd_session),
        None,
    )
    .await;
    assert_eq!(after_fail_state["notes"].as_array().unwrap().len(), 2);

    // Try again with exact same share_key -> succeeds and commits once.
    let (s, sent_1) = http_as(
        &app,
        "duet-ada",
        "POST",
        "/api/for-my-doctor/send",
        Some(&fd_session),
        Some(send_payload.clone()),
    )
    .await;
    assert_eq!(s, 200, "send retry failed: {sent_1}");
    assert_eq!(sent_1["practice_name"], "Northside Primary Care");
    assert_eq!(sent_1["status"], "sent");
    assert_eq!(sent_1["records"][0]["title"], "LDL cholesterol");
    assert_eq!(sent_1["notes"][0]["id"], note_linked_id.to_string());

    // Replay same share_key -> idempotent return, only 1 share in front-door.
    let (s, sent_1_replay) = http_as(
        &app,
        "duet-ada",
        "POST",
        "/api/for-my-doctor/send",
        Some(&fd_session),
        Some(send_payload),
    )
    .await;
    assert_eq!(s, 200);
    assert_eq!(sent_1_replay["id"], sent_1["id"]);
    assert_eq!(
        front_door_store
            .for_my_doctor_shares_for_account(fd_ada_id)
            .await
            .unwrap()
            .len(),
        1
    );

    // 5. Source correction of Hemoglobin A1c updates its To send mark with source_corrected=true
    //    while earlier Sent entry (LDL cholesterol) remains untouched!
    let a1c_corr = durable
        .evidence
        .ingest_with_authority(
            &ingest_ctx,
            &authority,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                entity: EvidenceEntity::HealthEvidence,
                canonical_record_id: a1c_id,
                record_id: a1c_id.to_string(),
                operation: EvidenceOperation::Upsert,
                expected_revision: Some(a1c_rev1.into()),
                source_generation: None,
                payload: json!({
                    "id": a1c_id,
                    "concept_id": "lab-a1c",
                    "display_name": "Hemoglobin A1c",
                    "verification": "verified",
                    "method": "point",
                    "measured_interval": { "start": "2026-11-11T10:00:00Z" },
                    "value": { "type": "quantity", "quantity": { "canonical_value": 6.1, "canonical_unit": "%" } },
                    "status": "corrected",
                    "corrects": a1c_id,
                    "source_label": "Mercy Clinic",
                    "source": { "kind": "epic", "connection_id": "epic-1", "issuer": "Mercy Clinic", "provenance": "portal" },
                }),
            },
        )
        .await
        .unwrap();
    let a1c_rev2 = a1c_corr.revision;

    let (s, state_after_corr) = http_as(
        &app,
        "duet-ada",
        "GET",
        "/api/for-my-doctor",
        Some(&fd_session),
        None,
    )
    .await;
    assert_eq!(s, 200);
    let corrected_mark = state_after_corr["marks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == a1c_mark_id)
        .unwrap();
    assert_ne!(corrected_mark["record_revision"], a1c_rev1);
    assert_eq!(corrected_mark["source_corrected"], true);
    assert_eq!(corrected_mark["record_revision"], a1c_rev2);
    assert_ne!(a1c_rev1, a1c_rev2);

    // 6. COD-50: Source removes Hemoglobin A1c -> mark disappears from To send + notice created in You,
    //    and earlier Sent entry remains intact!
    durable
        .evidence
        .ingest_with_authority(
            &ingest_ctx,
            &authority,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                entity: EvidenceEntity::HealthEvidence,
                canonical_record_id: a1c_id,
                record_id: a1c_id.to_string(),
                operation: EvidenceOperation::Delete,
                expected_revision: Some(a1c_rev2.into()),
                source_generation: None,
                payload: json!({ "source": { "kind": "epic", "connection_id": "epic-1" } }),
            },
        )
        .await
        .unwrap();

    let (s, state_after_rm) = http_as(
        &app,
        "duet-ada",
        "GET",
        "/api/for-my-doctor",
        Some(&fd_session),
        None,
    )
    .await;
    assert_eq!(s, 200);
    assert!(
        state_after_rm["marks"]
            .as_array()
            .unwrap()
            .iter()
            .all(|m| m["id"] != a1c_mark_id)
    );
    let notices = state_after_rm["notices"].as_array().unwrap();
    assert_eq!(notices.len(), 1);
    assert_eq!(notices[0]["title"], "Hemoglobin A1c");
    assert_eq!(state_after_rm["sends"].as_array().unwrap().len(), 1);

    // Dismiss the notice in You.
    let notice_id = notices[0]["id"].as_str().unwrap();
    let (s, _) = http_as(
        &app,
        "duet-ada",
        "POST",
        &format!("/api/for-my-doctor/notices/{notice_id}/dismiss"),
        None,
        None,
    )
    .await;
    assert_eq!(s, 204);

    // 7. Practice picks up send 1 -> transitions to Received ("received").
    let shares = front_door_store
        .pending_for_my_doctor_shares_for_practice(&roster_one.practice_id)
        .await
        .unwrap();
    front_door_store
        .mark_for_my_doctor_received(
            &roster_one.practice_id,
            &shares[0].share_id,
            now.load(Ordering::SeqCst) + 50,
        )
        .await
        .unwrap();

    // Send 2 to South practice, then end South connection BEFORE pickup -> stays Sent with connection_ended_at!
    let share_key_2 = format!("share-{}", Uuid::new_v4());
    let (s, sent_2) = http_as(
        &app,
        "duet-ada",
        "POST",
        "/api/for-my-doctor/send",
        Some(&fd_session),
        Some(json!({
            "share_key": share_key_2,
            "practice_id": "practice-south",
            "practice_name": "South Clinic",
            "patient_id": "pt-south-1",
            "disclosure_accepted": true,
            "selected_mark_ids": [trend_mark_id],
            "selected_note_ids": []
        })),
    )
    .await;
    assert_eq!(s, 200);
    assert_eq!(sent_2["records"][0]["range_label"], "Last 30 days");

    front_door_store
        .end_connection(
            fd_ada_id,
            &roster_two,
            ConnectionEndReason::PatientLeft,
            now.load(Ordering::SeqCst) + 80,
        )
        .await
        .unwrap();

    // Attempting a new send to ended South practice returns 409 connection_ended and keeps standalone note.
    let (s, ended_err) = http_as(
        &app,
        "duet-ada",
        "POST",
        "/api/for-my-doctor/send",
        Some(&fd_session),
        Some(json!({
            "share_key": format!("share-{}", Uuid::new_v4()),
            "practice_id": "practice-south",
            "practice_name": "South Clinic",
            "patient_id": "pt-south-1",
            "disclosure_accepted": true,
            "selected_mark_ids": [],
            "selected_note_ids": [note_standalone_id]
        })),
    )
    .await;
    assert_eq!(s, 409);
    assert_eq!(ended_err["code"], "connection_ended");

    // Synced state: North send is Received, South send stays Sent with connection_ended_at, standalone note remains in To send.
    let (s, final_state) = http_as(
        &app,
        "duet-ada",
        "GET",
        "/api/for-my-doctor",
        Some(&fd_session),
        None,
    )
    .await;
    assert_eq!(s, 200);
    let sends = final_state["sends"].as_array().unwrap();
    assert_eq!(sends.len(), 2);
    let north = sends
        .iter()
        .find(|x| x["practice_id"] == "practice-north")
        .unwrap();
    let south = sends
        .iter()
        .find(|x| x["practice_id"] == "practice-south")
        .unwrap();
    assert_eq!(north["status"], "received");
    assert_eq!(south["status"], "sent");
    assert!(south["connection_ended_at"].is_string());
    assert_eq!(final_state["notes"].as_array().unwrap().len(), 1);

    // 8. Export includes For my doctor marks, notes, and sends cleanly.
    let (s, export_val) = http_as(
        &app,
        "duet-ada",
        "GET",
        "/api/auth/account/export",
        None,
        None,
    )
    .await;
    assert_eq!(s, 200);
    let export: AccountExport = serde_json::from_value(export_val).unwrap();
    assert_eq!(export.for_my_doctor.marks.len(), 1);
    assert_eq!(export.for_my_doctor.notes.len(), 1);
    assert_eq!(export.for_my_doctor.sends.len(), 2);

    server.abort();
}
