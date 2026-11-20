//! Real-Postgres + live `front-door` HTTP proof for `COD-61`:
//! - Streaming Duet portal records, phone readings, and medications to `PUT /front-door/shared-records`
//!   on join and live updates with exact `"from <source>, via the patient's Duet"` attribution.
//! - Excluding private AI notes and unsent `For my doctor` drafts (`Rule 10` / `Rule 11`).
//! - Propagating source corrections and explicit source deletions as distinct revisions (`Rule 13`).
//! - Failing closed on upload outage and surfacing visible `SharingSyncStanding::Degraded` +
//!   content-free operational alerts (`P15`, `C1.3.8`).

#![allow(
    clippy::disallowed_methods,
    reason = "integration test provisions an isolated disposable Postgres database"
)]

use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use chrono::Utc;
use duet_api::ApiError;
use duet_api::connected_care_sync::{
    ConnectedCareSyncState, ConnectedCareUploader, DirectFrontDoorSyncTransport,
    FRONT_DOOR_SESSION_HEADER, HttpFrontDoorSyncTransport, SharingFailureReason, SharingSyncReport,
    SharingSyncStanding,
};
use duet_api::database::{AccountDatabase, migrate};
use duet_api::durable::DurableState;
use duet_api::evidence::{EvidenceMutation, EvidenceOperation, EvidenceStore, IngestAuthority};
use duet_api::identity::{IdentityStore, IdentityVerifier, VerifiedIdentity};
use duet_api::privacy::{FileJournal, PrivacyStore};
use duet_wire::models::addressed_record_revision::Entity as EvidenceEntity;
use front_door::{FrontDoorStore, MemoryFrontDoorStore, RosterPatient};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection};
use tower::ServiceExt;
use uuid::Uuid;

struct StaticIdentity;

#[async_trait]
impl IdentityVerifier for StaticIdentity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        let subject = match bearer {
            "duet-patient-token" => "cod61-patient-subject",
            "duet-patient-b-token" => "cod61-patient-b-subject",
            _ => return Err(ApiError::Unauthorized),
        };
        Ok(VerifiedIdentity {
            subject: subject.to_owned(),
            authenticated_at: Utc::now(),
        })
    }
}

async fn http_as(
    app: &Router,
    duet_bearer: &str,
    method: &str,
    path: &str,
    front_door_session: Option<&str>,
    body: Value,
) -> (u16, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header("authorization", format!("Bearer {duet_bearer}"))
        .header("content-type", "application/json");
    if let Some(session) = front_door_session {
        builder = builder.header(FRONT_DOOR_SESSION_HEADER, session);
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
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

async fn http(
    app: &Router,
    method: &str,
    path: &str,
    front_door_session: Option<&str>,
    body: Value,
) -> (u16, Value) {
    http_as(
        app,
        "duet-patient-token",
        method,
        path,
        front_door_session,
        body,
    )
    .await
}

#[tokio::test]
async fn cod61_live_uploader_streams_to_front_door_excludes_private_notes_and_surfaces_failures() {
    let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("real disposable loopback PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let suffix = Uuid::new_v4().simple().to_string();
    let db_name = format!("cod61_sync_{suffix}");
    let runtime_role = format!("cod61_rt_{suffix}");
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
    let journal_dir = std::env::temp_dir().join(format!("cod61-journal-{suffix}"));

    let durable = DurableState {
        identity: IdentityStore::new(database.clone(), Arc::new(StaticIdentity)),
        privacy: PrivacyStore::new(
            database.clone(),
            Arc::new(FileJournal::open(journal_dir.clone()).await.unwrap()),
        ),
        evidence: EvidenceStore::new(database.clone()),
    };

    // Start a real loopback HTTP `front-door` server serving `PUT /front-door/shared-records`.
    let front_door_store = Arc::new(MemoryFrontDoorStore::new());
    let now_unix: u64 = 1_763_217_000;
    let roster = RosterPatient {
        practice_id: "practice-cod61".to_owned(),
        patient_id: "ws-cod61".to_owned(),
    };
    let invite_token = front_door_store
        .invite(&roster, "pat@cod61.test", "Pat River", now_unix)
        .await
        .unwrap();
    let (fd_account_id, fd_session) = front_door_store
        .redeem_invitation("pat@cod61.test", &invite_token, now_unix + 10)
        .await
        .unwrap();

    let mut fd_state = front_door::http::FrontDoorState::new(front_door_store.clone());
    fd_state.now = Arc::new(move || now_unix + 20);
    let fd_router = front_door::http::router_with(fd_state, true);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, fd_router).await.unwrap();
    });

    let transport = Arc::new(HttpFrontDoorSyncTransport::new(&format!("http://{addr}")).unwrap());
    let uploader =
        ConnectedCareUploader::new(database.clone(), durable.evidence.clone(), transport);
    let app = duet_api::durable::router(durable.clone()).merge(
        duet_api::connected_care_sync::router(ConnectedCareSyncState {
            durable: durable.clone(),
            uploader: uploader.clone(),
        }),
    );

    let (status, _) = http(&app, "PUT", "/api/auth/account", None, json!({})).await;
    assert_eq!(status, StatusCode::OK.as_u16());
    let ctx = durable
        .identity
        .authenticate("duet-patient-token", false)
        .await
        .unwrap();

    // Rule 21: unmapped Duet account is refused before any records touch front-door.
    let (unmapped_status, _) = http(
        &app,
        "POST",
        "/api/connected-care/sync",
        Some(&fd_session),
        json!({}),
    )
    .await;
    assert_eq!(unmapped_status, StatusCode::UNAUTHORIZED.as_u16());
    assert!(
        front_door_store
            .shared_records_for_practice_patient("practice-cod61", "ws-cod61")
            .await
            .unwrap()
            .is_empty(),
        "unmapped Duet account must never write shared records to front-door"
    );

    // Explicit identity mapping authorizes Account A -> fd_account_id only.
    durable
        .identity
        .map_front_door(&ctx, fd_account_id)
        .await
        .unwrap();

    // Seed Epic portal lab, Apple Health reading, patient medication, plus private/draft notes.
    let epic_record_id = Uuid::new_v4();
    durable
        .evidence
        .ingest_with_authority(
            &ctx,
            &IngestAuthority::Connector {
                source_kind: "epic".to_owned(),
                connection_id: "epic-org-1".to_owned(),
            },
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                operation: EvidenceOperation::Upsert,
                canonical_record_id: epic_record_id,
                record_id: "epic-a1c-001".to_owned(),
                entity: EvidenceEntity::HealthEvidence,
                expected_revision: None,
                source_generation: None,
                payload: json!({
                    "id": epic_record_id,
                    "concept_id": "hba1c",
                    "display_name": "Hemoglobin A1c",
                    "verification": "verified",
                    "method": "point",
                    "measured_interval": { "start": "2026-11-15T14:30:00Z" },
                    "source": { "kind": "epic", "connection_id": "epic-org-1", "issuer": "UCSF", "provenance": "portal" },
                    "value": { "type": "quantity", "quantity": { "canonical_value": 6.4, "canonical_unit": "%" } }
                }),
            },
        )
        .await
        .unwrap();

    let apple_record_id = Uuid::new_v4();
    let apple_addr = durable
        .evidence
        .ingest(
            &ctx,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                operation: EvidenceOperation::Upsert,
                canonical_record_id: apple_record_id,
                record_id: "apple-bp-001".to_owned(),
                entity: EvidenceEntity::HealthEvidence,
                expected_revision: None,
                source_generation: Some(0),
                payload: json!({
                    "id": apple_record_id,
                    "concept_id": "blood_pressure",
                    "display_name": "Blood Pressure",
                    "verification": "verified",
                    "method": "point",
                    "measured_interval": { "start": "2026-11-16T08:00:00Z" },
                    "source": { "kind": "appleHealth", "connection_id": "phone-1", "issuer": "Apple Watch", "provenance": "native" },
                    "value": { "type": "quantity", "quantity": { "canonical_value": 138.0, "canonical_unit": "mmHg" } }
                }),
            },
        )
        .await
        .unwrap();

    let med_record_id = Uuid::new_v4();
    durable
        .evidence
        .ingest(
            &ctx,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                operation: EvidenceOperation::Upsert,
                canonical_record_id: med_record_id,
                record_id: "med-lisinopril-1".to_owned(),
                entity: EvidenceEntity::HealthEvidence,
                expected_revision: None,
                source_generation: None,
                payload: json!({
                    "id": med_record_id,
                    "concept_id": "medication",
                    "display_name": "Lisinopril 10 mg daily",
                    "verification": "patient_reported",
                    "method": "point",
                    "measured_interval": { "start": "2026-11-10T09:00:00Z" },
                    "source": { "kind": "manual", "connection_id": "duet", "issuer": "patient", "provenance": "manual" },
                    "summary": "Lisinopril 10 mg PO once daily"
                }),
            },
        )
        .await
        .unwrap();

    let draft_note_id = Uuid::new_v4();
    durable
        .evidence
        .ingest(
            &ctx,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                operation: EvidenceOperation::Upsert,
                canonical_record_id: draft_note_id,
                record_id: "fmd-draft-1".to_owned(),
                entity: EvidenceEntity::ClinicalNote,
                expected_revision: None,
                source_generation: None,
                payload: json!({
                    "id": draft_note_id,
                    "title": "Unsent For my doctor note",
                    "source_label": "Added in Duet",
                    "body": "Private draft not yet sent",
                    "recorded_at": "2026-11-16T09:00:00Z"
                }),
            },
        )
        .await
        .unwrap();

    // 1. Trigger live join sync via HTTP -> PUT /front-door/shared-records.
    let (sync_status, sync_body) = http(
        &app,
        "POST",
        "/api/connected-care/sync",
        Some(&fd_session),
        json!({}),
    )
    .await;
    assert_eq!(
        sync_status,
        StatusCode::OK.as_u16(),
        "sync failed: {sync_body}"
    );
    let report: SharingSyncReport = serde_json::from_value(sync_body).unwrap();
    assert_eq!(report.uploaded_count, 3);
    assert_eq!(report.excluded_private_count, 1);

    let shared = front_door_store
        .shared_records_for_practice_patient("practice-cod61", "ws-cod61")
        .await
        .unwrap();
    assert_eq!(shared.len(), 3);
    let epic_shared = shared
        .iter()
        .find(|r| r.record_id == epic_record_id.to_string())
        .expect("Epic lab uploaded");
    assert!(
        epic_shared
            .content_text
            .contains("Origin: from Epic, via the patient's Duet")
    );
    let apple_shared = shared
        .iter()
        .find(|r| r.record_id == apple_record_id.to_string())
        .expect("Apple Health reading uploaded");
    assert!(
        apple_shared
            .content_text
            .contains("Origin: from Apple Health, via the patient's Duet")
    );
    let med_shared = shared
        .iter()
        .find(|r| r.record_id == med_record_id.to_string())
        .expect("Patient medication uploaded");
    assert!(
        med_shared
            .content_text
            .contains("Origin: from patient medications, via the patient's Duet")
    );
    assert!(
        !shared
            .iter()
            .any(|r| r.record_id == draft_note_id.to_string()),
        "unsent For my doctor draft must never stream automatically"
    );

    // Status route reports Current.
    let (st_code, st_body) =
        http(&app, "GET", "/api/connected-care/status", None, json!(null)).await;
    assert_eq!(st_code, StatusCode::OK.as_u16());
    let standing: SharingSyncStanding = serde_json::from_value(st_body).unwrap();
    assert!(matches!(
        standing,
        SharingSyncStanding::Current {
            record_count: 3,
            ..
        }
    ));

    // 2. Source deletion propagates as a distinct removal revision.
    durable
        .evidence
        .ingest(
            &ctx,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                operation: EvidenceOperation::Delete,
                canonical_record_id: apple_record_id,
                record_id: "apple-bp-001".to_owned(),
                entity: EvidenceEntity::HealthEvidence,
                expected_revision: Some(i64::from(apple_addr.revision)),
                source_generation: Some(0),
                payload: json!({ "source": { "kind": "appleHealth", "connection_id": "phone-1" } }),
            },
        )
        .await
        .unwrap();
    let (sync2_status, _) = http(
        &app,
        "POST",
        "/api/connected-care/sync",
        Some(&fd_session),
        json!({}),
    )
    .await;
    assert_eq!(sync2_status, StatusCode::OK.as_u16());
    let shared_after = front_door_store
        .shared_records_for_practice_patient("practice-cod61", "ws-cod61")
        .await
        .unwrap();
    let removed_apple = shared_after
        .iter()
        .find(|r| r.record_id == apple_record_id.to_string())
        .expect("removed Apple Health record produces a retraction revision");
    assert!(removed_apple.revision.starts_with("removed-"));
    assert!(
        removed_apple
            .content_text
            .contains("Source-Status: removed")
    );

    // 3. Failure visibility (`P15`): invalid session fails closed and surfaces Degraded standing + ops alert.
    let (fail_status, _) = http(
        &app,
        "POST",
        "/api/connected-care/sync",
        Some("codaf-invalid-session"),
        json!({}),
    )
    .await;
    assert_eq!(fail_status, StatusCode::UNAUTHORIZED.as_u16());

    let (deg_code, deg_body) =
        http(&app, "GET", "/api/connected-care/status", None, json!(null)).await;
    assert_eq!(deg_code, StatusCode::OK.as_u16());
    let degraded: SharingSyncStanding = serde_json::from_value(deg_body).unwrap();
    match degraded {
        SharingSyncStanding::Degraded {
            last_succeeded_at,
            consecutive_failures,
            reason,
            ..
        } => {
            assert!(last_succeeded_at.is_some());
            assert_eq!(consecutive_failures, 1);
            assert_eq!(reason, SharingFailureReason::Unauthenticated);
        }
        other => panic!("expected Degraded standing, got {other:?}"),
    }
    let alerts = uploader.degraded_alerts();
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].account_id, ctx.account_id);
    assert_eq!(alerts[0].reason, SharingFailureReason::Unauthenticated);

    // Succeeding again clears the ops alert and restores Current standing.
    let (recover_status, _) = http(
        &app,
        "POST",
        "/api/connected-care/sync",
        Some(&fd_session),
        json!({}),
    )
    .await;
    assert_eq!(recover_status, StatusCode::OK.as_u16());
    assert!(uploader.degraded_alerts().is_empty());

    // 4. Cross-account PHI misrouting (`Rule 21`): pairing Account A's Duet bearer with
    //    Account B's front-door session (or an unmapped Account B with Account A's session)
    //    must fail closed with 401 and write ZERO records into the other account's chart.
    let roster_b = RosterPatient {
        practice_id: "practice-cod61".to_owned(),
        patient_id: "ws-cod61-b".to_owned(),
    };
    let invite_b = front_door_store
        .invite(&roster_b, "pat-b@cod61.test", "Pat B", now_unix)
        .await
        .unwrap();
    let (fd_account_b, fd_session_b) = front_door_store
        .redeem_invitation("pat-b@cod61.test", &invite_b, now_unix + 10)
        .await
        .unwrap();

    // Provision Account B in Duet (unmapped first) with distinct PHI.
    let (b_acct_status, _) = http_as(
        &app,
        "duet-patient-b-token",
        "PUT",
        "/api/auth/account",
        None,
        json!({}),
    )
    .await;
    assert_eq!(b_acct_status, StatusCode::OK.as_u16());
    let ctx_b = durable
        .identity
        .authenticate("duet-patient-b-token", false)
        .await
        .unwrap();
    let b_record_id = Uuid::new_v4();
    durable
        .evidence
        .ingest(
            &ctx_b,
            &EvidenceMutation {
                operation_id: Uuid::new_v4(),
                operation: EvidenceOperation::Upsert,
                canonical_record_id: b_record_id,
                record_id: "b-secret-lab-1".to_owned(),
                entity: EvidenceEntity::HealthEvidence,
                expected_revision: None,
                source_generation: None,
                payload: json!({
                    "id": b_record_id,
                    "concept_id": "tsh",
                    "display_name": "Patient B Private Lab",
                    "verification": "verified",
                    "method": "point",
                    "measured_interval": { "start": "2026-11-16T09:00:00Z" },
                    "source": { "kind": "manual", "connection_id": "patient", "issuer": "Patient", "provenance": "patient" },
                    "value": { "type": "quantity", "quantity": { "canonical_value": 2.1, "canonical_unit": "mIU/L" } }
                }),
            },
        )
        .await
        .unwrap();

    // 4a. Unmapped Account B presenting Account A's valid front-door session is refused.
    let (unmapped_b_status, _) = http_as(
        &app,
        "duet-patient-b-token",
        "POST",
        "/api/connected-care/sync",
        Some(&fd_session),
        json!({}),
    )
    .await;
    assert_eq!(unmapped_b_status, StatusCode::UNAUTHORIZED.as_u16());
    let account_a_records_after_b_attempt = front_door_store
        .shared_records_for_practice_patient("practice-cod61", "ws-cod61")
        .await
        .unwrap();
    assert!(
        account_a_records_after_b_attempt
            .iter()
            .all(|r| r.record_id != b_record_id.to_string()),
        "unmapped Account B's PHI must never land in Account A's front-door store"
    );

    // 4b. Mapped Account A presenting Account B's front-door session is refused over HTTP.
    durable
        .identity
        .map_front_door(&ctx_b, fd_account_b)
        .await
        .unwrap();
    let (cross_http_status, _) = http(
        &app,
        "POST",
        "/api/connected-care/sync",
        Some(&fd_session_b),
        json!({}),
    )
    .await;
    assert_eq!(cross_http_status, StatusCode::UNAUTHORIZED.as_u16());
    assert!(
        front_door_store
            .shared_records_for_practice_patient("practice-cod61", "ws-cod61-b")
            .await
            .unwrap()
            .is_empty(),
        "Account A's PHI must never be written to Account B's front-door store over HTTP"
    );

    // 4c. Same cross-account guard enforced on DirectFrontDoorSyncTransport.
    let direct_uploader = ConnectedCareUploader::new(
        database.clone(),
        durable.evidence.clone(),
        Arc::new(DirectFrontDoorSyncTransport::new(
            front_door_store.clone(),
            Arc::new(move || now_unix + 20),
        )),
    );
    let cross_direct_err = direct_uploader
        .sync_account(&ctx, &fd_session_b, Utc::now())
        .await
        .unwrap_err();
    assert_eq!(
        cross_direct_err.reason,
        SharingFailureReason::Unauthenticated
    );
    assert!(
        front_door_store
            .shared_records_for_practice_patient("practice-cod61", "ws-cod61-b")
            .await
            .unwrap()
            .is_empty(),
        "Account A's PHI must never be written to Account B's front-door store via Direct transport"
    );

    server.abort();
    drop(app);
    drop(uploader);
    drop(durable);
    drop(database);
    let _ = std::fs::remove_dir_all(&journal_dir);
    let _ = sqlx::query(AssertSqlSafe(format!(
        "drop database if exists {db_name} with (force)"
    )))
    .execute(&mut admin)
    .await;
    let _ = sqlx::query(AssertSqlSafe(format!("drop role if exists {runtime_role}")))
        .execute(&mut admin)
        .await;
}
