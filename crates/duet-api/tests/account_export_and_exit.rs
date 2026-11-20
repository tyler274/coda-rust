//! Isolated PostgreSQL proof for Rights-never-wait full personal data export (D5.1),
//! interrupted account deletion recovery (D5.2), backup-restore replay wiping deleted data (D5.2),
//! and clean re-signup under the same email yielding an empty new account (D5.3).
#![allow(
    clippy::disallowed_methods,
    reason = "provisions disposable databases, simulates backup restores, and runs SQL setup for typed owners"
)]

use async_trait::async_trait;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use duet_api::{
    ApiError,
    account_export::AccountExport,
    database::{AccountDatabase, migrate},
    doctor_routes::{self, DoctorState},
    durable::{self, DurableState},
    email_auth::{ChallengeCredential, EmailAuthStore, MemoryMailer},
    evidence::EvidenceStore,
    evidence_routes,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore, ReceiptJournal},
    workspace::WorkspaceStore,
};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, PgConnection};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tower::ServiceExt;
use uuid::Uuid;

struct UnusedExternalVerifier;
#[async_trait]
impl IdentityVerifier for UnusedExternalVerifier {
    async fn verify(&self, _bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        Err(ApiError::Unauthorized)
    }
}

struct FlakyJournal {
    inner: FileJournal,
    fail_next_put: AtomicBool,
}

#[async_trait]
impl ReceiptJournal for FlakyJournal {
    async fn put_if_absent(&self, id: Uuid, bytes: &[u8]) -> Result<(), ApiError> {
        if self.fail_next_put.load(Ordering::SeqCst) {
            return Err(ApiError::Unavailable(
                "Simulated network interruption before journal receipt confirmation",
            ));
        }
        self.inner.put_if_absent(id, bytes).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<Vec<u8>>, ApiError> {
        self.inner.read(id).await
    }

    async fn inventory(&self) -> Result<Vec<Uuid>, ApiError> {
        self.inner.inventory().await
    }
}

async fn http_full(
    app: &Router,
    method: &str,
    path: &str,
    bearer: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, axum::http::HeaderMap, Value) {
    let mut builder = Request::builder().method(method).uri(path);
    if let Some(token) = bearer {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let body = match body {
        Some(val) => {
            builder = builder.header("content-type", "application/json");
            Body::from(val.to_string())
        }
        None => Body::empty(),
    };
    let response = app
        .clone()
        .oneshot(builder.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({ "raw": String::from_utf8_lossy(&bytes) }))
    };
    (status, headers, json)
}

async fn email_sign_in(app: &Router, mailer: &MemoryMailer, email: &str) -> (String, Uuid, Uuid) {
    let (status, _, _) = http_full(
        app,
        "POST",
        "/api/auth/email/start",
        None,
        Some(json!({ "email": email })),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let sent = mailer.take().pop().unwrap();
    let (status, _, verified) = http_full(
        app,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "email": email,
            "credential": ChallengeCredential::ShortCode(sent.short_code.clone()),
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "email verify failed: {verified}");
    (
        verified["bearer"].as_str().unwrap().to_owned(),
        Uuid::parse_str(verified["account_id"].as_str().unwrap()).unwrap(),
        Uuid::parse_str(verified["incarnation"].as_str().unwrap()).unwrap(),
    )
}

#[tokio::test]
async fn full_personal_export_interrupted_delete_restore_replay_and_clean_resignup() {
    let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = PgConnection::connect(&admin_url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let db_name = format!("duet_export_{suffix}");
    let role = format!("duet_export_role_{suffix}");
    let password = Uuid::new_v4().simple().to_string();
    sqlx::raw_sql(AssertSqlSafe(format!(
        "create role {role} login password '{password}'"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {db_name}")))
        .execute(&mut admin)
        .await
        .unwrap();

    let owner_url = options
        .clone()
        .database(&db_name)
        .to_url_lossy()
        .to_string();
    let runtime_url = options
        .clone()
        .database(&db_name)
        .username(&role)
        .password(&password)
        .to_url_lossy()
        .to_string();
    migrate(&owner_url, &role).await.unwrap();

    let mailer = Arc::new(MemoryMailer::new());
    let journal_dir = std::env::temp_dir().join(format!("duet-export-journal-{suffix}"));
    let journal = Arc::new(FlakyJournal {
        inner: FileJournal::open(journal_dir.clone()).await.unwrap(),
        fail_next_put: AtomicBool::new(false),
    });

    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let email_auth = EmailAuthStore::new(db.clone(), mailer.clone(), "https://codahealth.ai");
    let privacy = PrivacyStore::new(db.clone(), journal.clone());
    let state = DurableState {
        identity: IdentityStore::with_email_auth(
            db.clone(),
            Arc::new(UnusedExternalVerifier),
            email_auth,
        ),
        privacy: privacy.clone(),
        evidence: EvidenceStore::new(db.clone()),
    };
    let doctor = DoctorState {
        durable: state.clone(),
        workspace: WorkspaceStore::new(db),
    };
    let app = durable::router(state.clone())
        .merge(evidence_routes::router(state))
        .merge(doctor_routes::router(doctor));

    // Create two isolated accounts; neither has accepted Terms yet (`AccessLevel::RightsOnly`).
    let (ada_bearer, ada_id, ada_incarnation) =
        email_sign_in(&app, &mailer, "Ada@Example.Test").await;
    let (_bob_bearer, bob_id, _) = email_sign_in(&app, &mailer, "bob@example.test").await;

    let mut owner_conn = PgConnection::connect(&owner_url).await.unwrap();
    let ev_id = Uuid::new_v4();
    let bob_ev_id = Uuid::new_v4();
    let note_id = Uuid::new_v4();
    let snap_id = Uuid::new_v4();
    let doctor_item_id = Uuid::new_v4();
    let portal_id = Uuid::new_v4();
    let conv_id = Uuid::new_v4();
    let cmd_id = Uuid::new_v4();

    // Seed Ada's data across every export category (with intelligence_enabled = false and no Terms).
    sqlx::query("update public.users set display_name='Ada Lovelace', intelligence_enabled=false, doctor_note='Check systolic trend', doctor_note_generation=4 where id=$1")
        .bind(ada_id)
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query("insert into public.health_evidence(id, user_id, concept_id, source_kind, source_connection_id, source_record_id, measured_at, received_at, payload) values ($1,$2,'bp','appleHealth','conn-ah-1','ah-rec-1','2026-10-01T08:00:00Z','2026-10-01T08:05:00Z',$3)")
        .bind(ev_id)
        .bind(ada_id)
        .bind(json!({"systolic": 118, "diastolic": 76}))
        .execute(&mut owner_conn)
        .await
        .unwrap();
    sqlx::query("insert into public.canonical_evidence_revisions(user_id, evidence_id, payload, content_sha256) values ($1,$2,$3,repeat('a',64))")
        .bind(ada_id)
        .bind(ev_id)
        .bind(json!({"systolic": 118, "diastolic": 76}))
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query("insert into public.health_evidence(id, user_id, concept_id, source_kind, source_record_id, measured_at, received_at, payload) values ($1,$2,'weight','manual','bob-secret-record','2026-10-02T09:00:00Z','2026-10-02T09:00:00Z',$3)")
        .bind(bob_ev_id)
        .bind(bob_id)
        .bind(json!({"kg": 80}))
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query("insert into public.clinical_notes(id, user_id, source_id, title, preview, body, recorded_at, source_label, duet_source_payload) values ($1,$2,'note-1','Cardiology consult','BP stable','Continue regimen','2026-09-15T14:00:00Z','Epic MyChart',$3)")
        .bind(note_id)
        .bind(ada_id)
        .bind(json!({"title":"Cardiology consult","author":"Dr. Patel"}))
        .execute(&mut owner_conn)
        .await
        .unwrap();
    sqlx::query("insert into public.canonical_evidence_revisions(user_id, note_id, payload, content_sha256) values ($1,$2,$3,repeat('b',64))")
        .bind(ada_id)
        .bind(note_id)
        .bind(json!({"title":"Cardiology consult"}))
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query("insert into public.duet_device_snapshots(user_id, id, generated_at, payload) values ($1,$2,'2026-10-01T08:02:00Z',$3)")
        .bind(ada_id)
        .bind(snap_id)
        .bind(json!({"facts":[{"id":"bp-latest"}]}))
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query("insert into public.doctor_collection_items(id, user_id, kind, source_id, title, detail, evidence, position, medication_confirmed) values ($1,$2,'measurement','bp-main','Blood pressure','118/76 mmHg',$3,0,true)")
        .bind(doctor_item_id)
        .bind(ada_id)
        .bind(json!([{"id": ev_id}]))
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query("insert into public.duet_portal_connections(id, user_id, organization_id, organization_name, vendor, brand, fhir_base_url, state, access_token, refresh_token, last_synced_at) values ($1,$2,'org-1','Mercy Clinic','epic','MyChart','https://fhir.example.test','connected','secret-oauth-access-token','secret-oauth-refresh-token','2026-10-03T10:00:00Z')")
        .bind(portal_id)
        .bind(ada_id)
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query("insert into public.duet_conversations(user_id, id, revision) values ($1,$2,1)")
        .bind(ada_id)
        .bind(conv_id)
        .execute(&mut owner_conn)
        .await
        .unwrap();
    sqlx::query("insert into public.duet_commands(user_id, id, conversation_id, context, input, base_revision, request, state, reply) values ($1,$2,$3,$4,'How are my labs?',0,$5,'complete',$6)")
        .bind(ada_id)
        .bind(cmd_id)
        .bind(conv_id)
        .bind(json!({}))
        .bind(json!({"prompt":"How are my labs?"}))
        .bind(json!({"text":"Your recent pressure looks steady."}))
        .execute(&mut owner_conn)
        .await
        .unwrap();

    sqlx::query(
        "insert into public.duet_private_memory(user_id, revision, content) values ($1, 3, $2)",
    )
    .bind(ada_id)
    .bind(json!({"summary":"Tracks morning blood pressure."}))
    .execute(&mut owner_conn)
    .await
    .unwrap();

    // Record a non-destructive privacy receipt on Ada's account.
    let (status, _, receipt_resp) = http_full(
        &app,
        "POST",
        "/api/auth/privacy-restrictions",
        Some(&ada_bearer),
        Some(json!({
            "event_id": Uuid::new_v4(),
            "operation": "luna_withdraw",
            "scope_id": null,
            "scope_version": 0,
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "receipt failed: {receipt_resp}");

    // 1. D5.1 Export works BEFORE Terms acceptance (`AccessLevel::RightsOnly`) and with AI disabled,
    // returns every personal category with sources & dates, and omits secrets + Bob's rows.
    let (status, headers, export_json) = http_full(
        &app,
        "GET",
        "/api/auth/account/export",
        Some(&ada_bearer),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        headers
            .get(header::CACHE_CONTROL)
            .and_then(|v| v.to_str().ok()),
        Some("no-store")
    );
    let export_text = export_json.to_string();
    assert!(
        !export_text.contains("secret-oauth-access-token")
            && !export_text.contains("secret-oauth-refresh-token")
            && !export_text.contains("bob-secret-record")
            && !export_text.contains("bob@example.test"),
        "export must never leak OAuth credentials or another account's data: {export_text}"
    );
    let export: AccountExport = serde_json::from_value(export_json).unwrap();
    assert_eq!(export.account.account_id, ada_id);
    assert_eq!(export.account.email.as_deref(), Some("ada@example.test"));
    assert!(!export.account.ai_enabled);
    assert_eq!(export.health_evidence.len(), 1);
    assert_eq!(export.health_evidence[0].source_kind, "appleHealth");
    assert!(export.health_evidence[0].measured_at.is_some());
    assert_eq!(export.health_evidence[0].revisions.len(), 1);
    assert_eq!(export.clinical_notes.len(), 1);
    assert_eq!(export.clinical_notes[0].source_label, "Epic MyChart");
    assert!(export.clinical_notes[0].recorded_at.is_some());
    assert_eq!(export.device_snapshots.len(), 1);
    assert_eq!(
        export.doctor_page.note.as_deref(),
        Some("Check systolic trend")
    );
    assert_eq!(export.doctor_page.items.len(), 1);
    assert_eq!(export.portal_connections.len(), 1);
    assert_eq!(
        export.portal_connections[0].organization_name,
        "Mercy Clinic"
    );
    assert_eq!(export.conversations.len(), 1);
    assert_eq!(export.conversations[0].turns.len(), 1);
    assert!(export.private_memory.is_some());
    assert!(!export.privacy_receipts.is_empty());

    // 2. D5.2 Interrupted AccountDelete fails loudly (`503`) before journal confirmation,
    // and idempotent retry with the same `event_id` finishes cleanly.
    let delete_event_id = Uuid::new_v4();
    journal.fail_next_put.store(true, Ordering::SeqCst);
    let (status, _, _) = http_full(
        &app,
        "POST",
        "/api/auth/privacy-restrictions",
        Some(&ada_bearer),
        Some(json!({
            "event_id": delete_event_id,
            "operation": "account_delete",
            "scope_id": null,
            "scope_version": 0,
        })),
    )
    .await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);

    // Retry with the exact same event_id after journal recovery.
    journal.fail_next_put.store(false, Ordering::SeqCst);
    let (status, _, _) = http_full(
        &app,
        "POST",
        "/api/auth/privacy-restrictions",
        Some(&ada_bearer),
        Some(json!({
            "event_id": delete_event_id,
            "operation": "account_delete",
            "scope_id": null,
            "scope_version": 0,
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // Old bearer cannot export or access account anymore.
    let (status, _, _) = http_full(
        &app,
        "GET",
        "/api/auth/account/export",
        Some(&ada_bearer),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // 3. D5.2 Backup-restore simulation: restore pre-deletion SQL user + patient data
    // while keeping independent FileJournal intact, then run `reconcile_inventory()`.
    sqlx::query("delete from public.duet_deleted_identities where account_id=$1")
        .bind(ada_id)
        .execute(&mut owner_conn)
        .await
        .unwrap();
    sqlx::query("delete from public.privacy_recovery_events where account_id=$1")
        .bind(ada_id)
        .execute(&mut owner_conn)
        .await
        .unwrap();
    sqlx::query("insert into public.users(id, auth_user_id, email, display_name, sharing_label, duet_incarnation) values ($1, 'email:' || $1::text, 'ada@example.test', 'Ada Restored', 'Ada Restored', $2)")
        .bind(ada_id)
        .bind(ada_incarnation)
        .execute(&mut owner_conn)
        .await
        .unwrap();
    sqlx::query("insert into public.health_evidence(id, user_id, concept_id, source_kind, measured_at, received_at, payload) values ($1,$2,'bp','appleHealth','2026-10-01T08:00:00Z','2026-10-01T08:05:00Z',$3)")
        .bind(ev_id)
        .bind(ada_id)
        .bind(json!({"systolic": 118}))
        .execute(&mut owner_conn)
        .await
        .unwrap();
    sqlx::query(
        "insert into public.duet_private_memory(user_id, revision, content) values ($1, 3, $2)",
    )
    .bind(ada_id)
    .bind(json!({"summary":"Restored secret"}))
    .execute(&mut owner_conn)
    .await
    .unwrap();

    let replayed = privacy.reconcile_inventory().await.unwrap();
    assert!(replayed > 0);
    let remaining_rows: i64 = sqlx::query_scalar(
        "select (select count(*) from public.users where id=$1) + (select count(*) from public.health_evidence where user_id=$1) + (select count(*) from public.duet_private_memory where user_id=$1)",
    )
    .bind(ada_id)
    .fetch_one(&mut owner_conn)
    .await
    .unwrap();
    assert_eq!(
        remaining_rows, 0,
        "backup-restore inventory replay must erase restored user and health records"
    );

    // 4. D5.3 Clean re-signup with the exact same email gets a fresh empty account.
    let (new_bearer, new_id, new_incarnation) =
        email_sign_in(&app, &mailer, "ada@example.test").await;
    assert_ne!(new_id, ada_id);
    assert_ne!(new_incarnation, ada_incarnation);
    let (status, _, new_export_json) = http_full(
        &app,
        "GET",
        "/api/auth/account/export",
        Some(&new_bearer),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let new_export: AccountExport = serde_json::from_value(new_export_json).unwrap();
    assert_eq!(new_export.account.account_id, new_id);
    assert!(new_export.health_evidence.is_empty());
    assert!(new_export.clinical_notes.is_empty());
    assert!(new_export.device_snapshots.is_empty());
    assert!(new_export.doctor_page.items.is_empty());
    assert!(new_export.doctor_page.note.is_none());
    assert!(new_export.portal_connections.is_empty());
    assert!(new_export.conversations.is_empty());
    assert!(new_export.private_memory.is_none());
    assert!(new_export.privacy_receipts.is_empty());

    let _ = tokio::fs::remove_dir_all(&journal_dir).await;
    drop(owner_conn);
    let _ = sqlx::raw_sql(AssertSqlSafe(format!(
        "drop database if exists {db_name} with (force)"
    )))
    .execute(&mut admin)
    .await;
    let _ = sqlx::raw_sql(AssertSqlSafe(format!("drop role if exists {role}")))
        .execute(&mut admin)
        .await;
}
