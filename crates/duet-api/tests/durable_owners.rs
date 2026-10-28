//! Isolated real PostgreSQL proof; requires explicit loopback admin URL and never silently skips.
use async_trait::async_trait;
use chrono::{Duration, Utc};
use duet_api::{
    ApiError,
    database::{AccountDatabase, migrate},
    evidence::{
        EvidenceBatch, EvidenceItemOutcome, EvidenceMutation, EvidenceOperation,
        EvidenceRejectionCode, EvidenceStore, IngestAuthority, SOURCE_AUTHORITY_REFUSED,
    },
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore, ReceiptJournal, Restriction},
};
use duet_wire::models::addressed_record_revision::Entity;
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection, PgConnection};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use uuid::Uuid;

struct Verifier;
#[async_trait]
impl IdentityVerifier for Verifier {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        if !["a", "b"].contains(&bearer) {
            return Err(ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: format!("provider-subject-{bearer}"),
            authenticated_at: Utc::now() - Duration::minutes(5),
        })
    }
}
struct FaultJournal {
    inner: FileJournal,
    fail: AtomicBool,
    ambiguous: AtomicBool,
}
#[async_trait]
impl ReceiptJournal for FaultJournal {
    async fn put_if_absent(&self, id: Uuid, bytes: &[u8]) -> Result<(), ApiError> {
        if self.fail.load(Ordering::SeqCst) {
            return Err(ApiError::Unavailable("injected journal outage"));
        }
        self.inner.put_if_absent(id, bytes).await?;
        if self.ambiguous.load(Ordering::SeqCst) {
            return Err(ApiError::Unavailable("injected lost acknowledgement"));
        }
        Ok(())
    }
    async fn read(&self, id: Uuid) -> Result<Option<Vec<u8>>, ApiError> {
        if self.fail.load(Ordering::SeqCst) {
            return Err(ApiError::Unavailable("injected journal outage"));
        }
        self.inner.read(id).await
    }
}
#[test]
fn durable_identity_evidence_privacy_and_restore() {
    // Keep the large all-owner integration future and renderer on an explicitly
    // bounded test thread; production work uses the normal runtime.
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(run_durable_owners())
        })
        .unwrap()
        .join()
        .unwrap();
}
async fn run_durable_owners() {
    eprintln!("durable proof: database setup");
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    eprintln!("durable proof: parsing options");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    eprintln!("durable proof: connecting");
    assert!(["localhost", "127.0.0.1", "::1"].contains(&options.get_host()));
    let mut admin = PgConnection::connect(&url).await.unwrap();
    eprintln!("durable proof: connected");
    let suffix = Uuid::new_v4().simple().to_string();
    let role = format!("duet_owner_{suffix}");
    let name = format!("duet_durable_{suffix}");
    sqlx::raw_sql(AssertSqlSafe(format!(
        "create role {role} login; create database {name}"
    )))
    .execute(&mut admin)
    .await
    .unwrap_err();
    // CREATE DATABASE must not share a transaction with role creation.
    sqlx::raw_sql(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let owner_url = options.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = options
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    let mut owner = PgConnection::connect(&owner_url).await.unwrap();
    sqlx::raw_sql("revoke create on schema public from public")
        .execute(&mut owner)
        .await
        .unwrap();
    // An existing imported provider identity keeps exactly its existing users.id.
    let existing = Uuid::new_v4();
    sqlx::raw_sql(include_str!("../migrations/0001_account_boundary.sql"))
        .execute(&mut owner)
        .await
        .unwrap();
    sqlx::query("insert into users(id,auth_user_id,sharing_label) values($1,'provider-subject-a','Existing')").bind(existing).execute(&mut owner).await.unwrap();
    migrate(&owner_url, &role).await.unwrap();
    migrate(&owner_url, &role).await.unwrap();
    assert!(AccountDatabase::connect(&owner_url).await.is_err());
    eprintln!("durable proof: migrated");
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let identities = IdentityStore::new(db.clone(), Arc::new(Verifier));
    let a = identities.authenticate("a", true).await.unwrap();
    assert_eq!(a.account_id, existing);
    let b = identities.authenticate("b", true).await.unwrap();
    assert_ne!(a.account_id, b.account_id);
    let door = Uuid::new_v4();
    identities.map_front_door(&a, door).await.unwrap();
    assert!(identities.map_front_door(&a, Uuid::new_v4()).await.is_err());
    let db2 = AccountDatabase::connect(&runtime_url).await.unwrap();
    let second = IdentityStore::new(db2.clone(), Arc::new(Verifier));
    assert_eq!(
        second.authenticate("a", false).await.unwrap().account_id,
        a.account_id
    );
    eprintln!("durable proof: identities");
    let evidence = EvidenceStore::new(db.clone());
    let mut mutation = EvidenceMutation {
        operation_id: Uuid::new_v4(),
        canonical_record_id: Uuid::new_v4(),
        record_id: "source/non-uuid-original".to_owned(),
        entity: Entity::HealthEvidence,
        expected_revision: None,
        source_generation: None,
        operation: Default::default(),
        payload: serde_json::json!({"concept_id":"test-measurement","measured_at":"2026-10-20T00:00:00Z","source":{"kind":"healthRecords","record_id":"original-id","declared_dates":{"issued":"2026"},"projected_date_role":"issued"},"value":{"literal":"< 0.03","unit":"mg/dL","unknown":[null,{"custom":"preserve"}]},"status":"preliminary"}),
    };
    let first = evidence.ingest(&a, &mutation).await.unwrap();
    assert_eq!(evidence.ingest(&a, &mutation).await.unwrap(), first);
    assert_eq!(
        evidence.exact(&a, &first, true).await.unwrap().payload,
        mutation.payload
    );
    assert!(evidence.exact(&b, &first, false).await.is_err());
    let snapshot = evidence.snapshot(&a, 0).await.unwrap();
    assert_eq!(snapshot.records.len(), 1);
    mutation.payload["status"] = serde_json::json!("corrected");
    assert!(evidence.ingest(&a, &mutation).await.is_err());
    mutation.operation_id = Uuid::new_v4();
    mutation.expected_revision = Some(i64::from(first.revision));
    let corrected = evidence.ingest(&a, &mutation).await.unwrap();
    assert!(evidence.exact(&a, &first, true).await.is_err());
    assert!(!evidence.exact(&a, &first, false).await.unwrap().current);
    let restart = EvidenceStore::new(db2.clone());
    assert_eq!(
        restart.snapshot(&a, snapshot.cursor).await.unwrap().records[0].address,
        corrected
    );
    let mut competing = mutation.clone();
    competing.operation_id = Uuid::new_v4();
    competing.expected_revision = Some(i64::from(corrected.revision));
    competing.payload["status"] = serde_json::json!("final");
    let mut competing2 = competing.clone();
    competing2.operation_id = Uuid::new_v4();
    let (x, y) = tokio::join!(
        evidence.ingest(&a, &competing),
        restart.ingest(&a, &competing2)
    );
    assert_ne!(x.is_ok(), y.is_ok());
    let current = x.or(y).unwrap();
    // Runtime cannot mutate retained revisions or read another account without scope.
    let mut tx = db.begin(a.account_id).await.unwrap();
    assert!(
        sqlx::query("update canonical_evidence_revisions set payload='{}'")
            .execute(&mut *tx)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    let root = std::env::temp_dir().join(format!("duet-receipts-{suffix}"));
    let journal = Arc::new(FaultJournal {
        inner: FileJournal::open(root.clone()).await.unwrap(),
        fail: AtomicBool::new(true),
        ambiguous: AtomicBool::new(false),
    });
    let privacy = PrivacyStore::new(db.clone(), journal.clone());
    let event = Uuid::new_v4();
    assert!(
        privacy
            .restrict(&a, event, Restriction::LunaWithdraw, a.account_id, 0)
            .await
            .is_err()
    );
    let disabled: bool =
        sqlx::query_scalar("select not intelligence_enabled from users where id=$1")
            .bind(a.account_id)
            .fetch_one(&mut owner)
            .await
            .unwrap();
    assert!(disabled);
    let count: i64 = sqlx::query_scalar("select count(*) from privacy_recovery_events where id=$1")
        .bind(event)
        .fetch_one(&mut owner)
        .await
        .unwrap();
    assert_eq!(count, 1);
    journal.fail.store(false, Ordering::SeqCst);
    journal.ambiguous.store(true, Ordering::SeqCst);
    let receipt = privacy
        .restrict(&a, event, Restriction::LunaWithdraw, a.account_id, 0)
        .await
        .unwrap();
    let bytes = journal.read(event).await.unwrap().unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()["operation"],
        "luna_withdraw"
    );
    assert!(!String::from_utf8(bytes.clone()).unwrap().contains("mg/dL"));
    assert_eq!(
        privacy
            .restrict(&a, event, Restriction::LunaWithdraw, a.account_id, 0)
            .await
            .unwrap(),
        receipt
    );
    assert!(
        privacy
            .restrict(&a, event, Restriction::SessionsRevoke, a.account_id, 0)
            .await
            .is_err()
    );
    // Simulated SQL rollback restores pre-restriction permission; journal survives independently.
    sqlx::query("update users set intelligence_enabled=true,luna_consent_decision=null,luna_consent_at=null,luna_consent_version=null,intelligence_control_revision=0 where id=$1").bind(a.account_id).execute(&mut owner).await.unwrap();
    sqlx::query("delete from privacy_recovery_events where id=$1")
        .bind(event)
        .execute(&mut owner)
        .await
        .unwrap();
    let restored = PrivacyStore::new(db2.clone(), journal.clone());
    assert!(restored.replay(event).await.unwrap());
    assert!(!restored.replay(event).await.unwrap());
    // A later consent incarnation is not disabled by old recovery.
    sqlx::query(
        "update users set intelligence_enabled=true,intelligence_control_revision=2 where id=$1",
    )
    .bind(a.account_id)
    .execute(&mut owner)
    .await
    .unwrap();
    assert!(!restored.replay(event).await.unwrap());
    eprintln!("durable proof: publication and SHL");
    // Durable approved workspace, pinned publication, and independent SHL decoding.
    use aes_gcm::{
        Aes256Gcm, KeyInit,
        aead::{Aead, Payload},
    };
    use base64::Engine;
    use duet_api::{
        publication::{PublicationStore, PublishRequest},
        workspace::WorkspaceStore,
    };
    use duet_wire::models::{
        CanonicalRecordSelection, DoctorPageContent, DoctorPageSave,
        canonical_record_selection::Kind,
    };
    let workspace = WorkspaceStore::new(db.clone());
    let page = workspace
        .select_record(
            &a,
            &CanonicalRecordSelection::new(
                Kind::Record,
                current.canonical_record_id,
                current.revision,
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        page["items"][0]["record"]["record_id"],
        "source/non-uuid-original"
    );
    let item: Uuid = serde_json::from_value(page["items"][0]["id"].clone()).unwrap();
    let mut content = DoctorPageContent::new();
    content.name = Some(Some("Alex <script>alert(1)</script>".into()));
    content.questions = Some(vec!["What should we discuss?".into()]);
    content.cover_note = Some(Some("PRIVATE COVER NOTE".into()));
    let save = DoctorPageSave::new(
        page["note_generation"].as_i64().unwrap() as i32,
        vec![item],
        content,
    );
    let approved = workspace.save(&a, &save).await.unwrap();
    assert_eq!(workspace.save(&a, &save).await.unwrap(), approved);
    let publications = PublicationStore::new(
        db.clone(),
        "https://duet.example".into(),
        "https://viewer.example/".into(),
        [7; 32],
    )
    .unwrap();
    let request = PublishRequest {
        operation_id: Uuid::new_v4(),
        expected_generation: approved["note_generation"].as_i64().unwrap(),
        expires_in_seconds: 3600,
    };
    let view = publications.publish(&a, &request).await.unwrap();
    assert_eq!(
        publications.publish(&a, &request).await.unwrap().web_url,
        view.web_url
    );
    let web_token = view.web_url.rsplit('/').next().unwrap();
    let public = publications.retrieve(web_token, false).await.unwrap();
    // Real durable route composition: no auth required publicly, private workspace fails closed.
    use tower::ServiceExt;
    let routes = duet_api::doctor_routes::router(duet_api::doctor_routes::DoctorState {
        durable: duet_api::durable::DurableState {
            identity: identities.clone(),
            privacy: privacy.clone(),
            evidence: evidence.clone(),
        },
        workspace: workspace.clone(),
        publications: publications.clone(),
    });
    let response = routes
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri(format!("/shared/for-my-doctor/{web_token}"))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers()["cache-control"], "no-store, max-age=0");
    assert_eq!(response.headers()["referrer-policy"], "no-referrer");
    assert!(response.headers().contains_key("content-security-policy"));
    let unauthorized = routes
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/doctor-collection")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unauthorized.status(), 401);
    let html = duet_api::publication::html(&public.snapshot);
    assert!(!html.contains("<script>"));
    assert!(!html.contains("PRIVATE COVER NOTE"));
    let encoded = view.shl_url.split("#shlink:/").nth(1).unwrap();
    let decoder = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let shl: serde_json::Value = serde_json::from_slice(&decoder.decode(encoded).unwrap()).unwrap();
    let token = shl["url"].as_str().unwrap().split('/').nth(4).unwrap();
    let manifest = publications.manifest(token).await.unwrap();
    let location = manifest["files"][0]["location"]
        .as_str()
        .unwrap()
        .rsplit('/')
        .next()
        .unwrap();
    let mut tampered = location.as_bytes().to_vec();
    tampered[30] = if tampered[30] == b'A' { b'B' } else { b'A' };
    assert!(
        publications
            .shl_file(std::str::from_utf8(&tampered).unwrap())
            .await
            .is_err()
    );
    let jwe = publications.shl_file(location).await.unwrap();
    let parts: Vec<_> = jwe.split('.').collect();
    assert_eq!(parts.len(), 5);
    assert!(parts[1].is_empty());
    let header: serde_json::Value =
        serde_json::from_slice(&decoder.decode(parts[0]).unwrap()).unwrap();
    assert_eq!(header["alg"], "dir");
    assert_eq!(header["enc"], "A256GCM");
    let key = decoder.decode(shl["key"].as_str().unwrap()).unwrap();
    let nonce = decoder.decode(parts[2]).unwrap();
    let ciphertext = [
        decoder.decode(parts[3]).unwrap(),
        decoder.decode(parts[4]).unwrap(),
    ]
    .concat();
    let plaintext = Aes256Gcm::new_from_slice(&key)
        .unwrap()
        .decrypt(
            nonce.as_slice().into(),
            Payload {
                msg: &ciphertext,
                aad: parts[0].as_bytes(),
            },
        )
        .unwrap();
    let fhir: serde_json::Value = serde_json::from_slice(&plaintext).unwrap();
    assert_eq!(fhir["resourceType"], "DocumentReference");
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(fhir["content"][0]["attachment"]["data"].as_str().unwrap())
            .unwrap(),
        public.pdf
    );
    let mut changed = save.clone();
    changed.expected_generation = approved["note_generation"].as_i64().unwrap() as i32;
    changed.page.name = Some(Some("New wording".into()));
    workspace.save(&a, &changed).await.unwrap();
    assert_eq!(
        publications.retrieve(web_token, false).await.unwrap().pdf,
        public.pdf
    );
    sqlx::query("update duet_publications set expires_at=now()-interval '1 second' where id=$1")
        .bind(view.id)
        .execute(&mut owner)
        .await
        .unwrap();
    assert!(publications.retrieve(web_token, false).await.is_err());
    assert!(publications.shl_file(location).await.is_err());
    sqlx::query("update duet_publications set expires_at=now()+interval '1 hour' where id=$1")
        .bind(view.id)
        .execute(&mut owner)
        .await
        .unwrap();
    privacy
        .restrict(
            &a,
            Uuid::new_v4(),
            Restriction::PublicationRevoke,
            view.id,
            0,
        )
        .await
        .unwrap();
    assert!(publications.retrieve(web_token, false).await.is_err());
    assert!(publications.manifest(token).await.is_err());
    assert!(publications.shl_file(location).await.is_err());
    // Conditional publication revoke: real HTTP contract and shared account-lock race.
    let revoke_request = |operation: Uuid, target: Option<Uuid>| {
        let mut builder = axum::http::Request::builder()
            .method("DELETE")
            .uri("/api/doctor-collection/share-link")
            .header("Authorization", "Bearer a")
            .header("Idempotency-Key", operation.to_string());
        if let Some(target) = target {
            builder = builder.header("If-Match", target.to_string());
        }
        builder.body(axum::body::Body::empty()).unwrap()
    };
    let status = routes
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/doctor-collection/share-link")
                .header("Authorization", "Bearer a")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(status.headers()["duet-publication-precondition"], "uuid-v1");
    assert_eq!(
        status.headers()["access-control-expose-headers"],
        "Duet-Publication-Precondition"
    );
    let preflight = routes
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("OPTIONS")
                .uri("/api/doctor-collection/share-link")
                .header("Origin", "https://patient.example")
                .header("Access-Control-Request-Method", "DELETE")
                .header(
                    "Access-Control-Request-Headers",
                    "If-Match,Idempotency-Key,Authorization",
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(preflight.status(), 204);
    let allowed = preflight.headers()["access-control-allow-headers"]
        .to_str()
        .unwrap();
    assert!(allowed.contains("If-Match") && allowed.contains("Idempotency-Key"));
    assert_eq!(
        routes
            .clone()
            .oneshot(revoke_request(Uuid::new_v4(), None))
            .await
            .unwrap()
            .status(),
        428
    );
    let generation = workspace.read(&a).await.unwrap()["note_generation"]
        .as_i64()
        .unwrap();
    let publish_request = || PublishRequest {
        operation_id: Uuid::new_v4(),
        expected_generation: generation,
        expires_in_seconds: 3600,
    };
    let reviewed = publications.publish(&a, &publish_request()).await.unwrap();
    for invalid in [
        "*".to_owned(),
        format!("\"{}\"", reviewed.id),
        format!("W/\"{}\"", reviewed.id),
        "not-a-uuid".to_owned(),
    ] {
        let mut request = revoke_request(Uuid::new_v4(), None);
        request
            .headers_mut()
            .insert("If-Match", invalid.parse().unwrap());
        assert_eq!(routes.clone().oneshot(request).await.unwrap().status(), 422);
    }
    let mut missing_operation = revoke_request(Uuid::new_v4(), Some(reviewed.id));
    missing_operation.headers_mut().remove("Idempotency-Key");
    assert_eq!(
        routes
            .clone()
            .oneshot(missing_operation)
            .await
            .unwrap()
            .status(),
        422
    );
    let mut other_account = revoke_request(Uuid::new_v4(), Some(reviewed.id));
    other_account
        .headers_mut()
        .insert("Authorization", "Bearer b".parse().unwrap());
    assert_eq!(
        routes
            .clone()
            .oneshot(other_account)
            .await
            .unwrap()
            .status(),
        409
    );
    let op = Uuid::new_v4();
    // Independent journal outage leaves restriction + original operation binding
    // effective. A replacement must survive the later acknowledgement retry.
    journal.fail.store(true, Ordering::SeqCst);
    assert_eq!(
        routes
            .clone()
            .oneshot(revoke_request(op, Some(reviewed.id)))
            .await
            .unwrap()
            .status(),
        503
    );
    let replacement = publications.publish(&a, &publish_request()).await.unwrap();
    journal.fail.store(false, Ordering::SeqCst);
    let acknowledged = routes
        .clone()
        .oneshot(revoke_request(op, Some(reviewed.id)))
        .await
        .unwrap();
    assert_eq!(acknowledged.status(), 204);
    assert_eq!(
        acknowledged.headers()["duet-publication-precondition"],
        "uuid-v1"
    );
    assert_eq!(
        routes
            .clone()
            .oneshot(revoke_request(op, Some(replacement.id)))
            .await
            .unwrap()
            .status(),
        409
    );
    let stale_op = Uuid::new_v4();
    assert_eq!(
        routes
            .clone()
            .oneshot(revoke_request(stale_op, Some(reviewed.id)))
            .await
            .unwrap()
            .status(),
        409
    );
    let retained: i64 =
        sqlx::query_scalar("select count(*) from privacy_recovery_events where id=$1")
            .bind(stale_op)
            .fetch_one(&mut owner)
            .await
            .unwrap();
    assert_eq!(retained, 0);
    assert!(
        publications
            .retrieve(replacement.web_url.rsplit('/').next().unwrap(), false)
            .await
            .is_ok()
    );
    // Retry same successful operation after replacement still acknowledges it.
    assert_eq!(
        routes
            .clone()
            .oneshot(revoke_request(op, Some(reviewed.id)))
            .await
            .unwrap()
            .status(),
        204
    );
    // Two runtime pools race replacement against conditional revoke. Whichever
    // wins the account lock, the newly published target must never be revoked.
    let second_publications = PublicationStore::new(
        db2.clone(),
        "https://duet.example".into(),
        "https://viewer.example/".into(),
        [7; 32],
    )
    .unwrap();
    for _ in 0..3 {
        let reviewed = publications.status(&a).await.unwrap().unwrap();
        let race_op = Uuid::new_v4();
        let replace = publish_request();
        let (revoked, replaced) = tokio::join!(
            routes
                .clone()
                .oneshot(revoke_request(race_op, Some(reviewed.id))),
            second_publications.publish(&a, &replace)
        );
        let result = revoked.unwrap();
        let replaced = replaced.unwrap();
        assert!([204, 409].contains(&result.status().as_u16()));
        let active: bool =
            sqlx::query_scalar("select revoked_at is null from duet_publications where id=$1")
                .bind(replaced.id)
                .fetch_one(&mut owner)
                .await
                .unwrap();
        assert!(active);
        let retry = routes
            .clone()
            .oneshot(revoke_request(race_op, Some(reviewed.id)))
            .await
            .unwrap();
        assert_eq!(retry.status(), result.status());
        if result.status() == 204 {
            assert_eq!(
                routes
                    .clone()
                    .oneshot(revoke_request(race_op, Some(replaced.id)))
                    .await
                    .unwrap()
                    .status(),
                409
            );
        }
    }
    // Concurrent duplicate submissions bind just one receipt and both acknowledge.
    let duplicate_target = publications.status(&a).await.unwrap().unwrap();
    let duplicate_op = Uuid::new_v4();
    let (one, two) = tokio::join!(
        routes
            .clone()
            .oneshot(revoke_request(duplicate_op, Some(duplicate_target.id))),
        routes
            .clone()
            .oneshot(revoke_request(duplicate_op, Some(duplicate_target.id)))
    );
    assert_eq!(one.unwrap().status(), 204);
    assert_eq!(two.unwrap().status(), 204);
    let count: i64 = sqlx::query_scalar(
        "select count(*) from privacy_recovery_events where id=$1 and scope_id=$2",
    )
    .bind(duplicate_op)
    .bind(duplicate_target.id)
    .fetch_one(&mut owner)
    .await
    .unwrap();
    assert_eq!(count, 1);
    // Proposals are durable across retry; rejection and newer saves cannot resurrect them.
    sqlx::query("update users set intelligence_enabled=true,luna_consent_decision='accepted',luna_consent_version='2026-09-24',terms_version=$2 where id=$1").bind(a.account_id).bind("2026-09-23").execute(&mut owner).await.unwrap();
    let proposal_context = identities.authenticate("a", false).await.unwrap();
    let base = workspace.read(&proposal_context).await.unwrap();
    let proposal = serde_json::json!({"id":Uuid::new_v4(),"run_id":Uuid::new_v4(),"message_id":Uuid::new_v4(),"base_generation":base["note_generation"],"items":base["items"],"page":{"name":"Proposed name"},"note":"Proposed concern"});
    workspace
        .stage_proposal(&proposal_context, &proposal)
        .await
        .unwrap();
    assert_eq!(
        workspace.read(&proposal_context).await.unwrap()["page"],
        base["page"]
    );
    let proposal_id: Uuid = serde_json::from_value(proposal["id"].clone()).unwrap();
    workspace
        .reject_proposal(
            &proposal_context,
            proposal_id,
            base["note_generation"].as_i64().unwrap(),
        )
        .await
        .unwrap();
    workspace
        .stage_proposal(&proposal_context, &proposal)
        .await
        .unwrap();
    assert!(workspace.read(&proposal_context).await.unwrap()["proposal"].is_null());
    let mut rejected = changed.clone();
    rejected.expected_generation = base["note_generation"].as_i64().unwrap() as i32;
    rejected.proposal_id = Some(Some(proposal_id));
    assert!(workspace.save(&proposal_context, &rejected).await.is_err());
    // New device uploads leave the selected dated snapshot unchanged.
    let device_id = Uuid::new_v4();
    let device = serde_json::json!({"snapshot_id":device_id,"generated_at":"2026-10-20T12:00:00Z","facts":[{"id":"watch/steps","title":"Steps","value":1234,"unit":"count","range_start":"2026-10-19T00:00:00Z","range_end":"2026-10-20T00:00:00Z"}]});
    workspace.upload_device(&a, &device).await.unwrap();
    let selected = workspace.select_fact(&a, "watch/steps").await.unwrap();
    let frozen = selected["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["source_id"] == "watch/steps")
        .unwrap()["fact_snapshot"]
        .clone();
    let mut newer = device.clone();
    newer["snapshot_id"] = serde_json::json!(Uuid::new_v4());
    newer["generated_at"] = serde_json::json!("2026-10-21T12:00:00Z");
    newer["facts"][0]["value"] = serde_json::json!(9999);
    workspace.upload_device(&a, &newer).await.unwrap();
    assert_eq!(
        workspace.read(&a).await.unwrap()["items"]
            .as_array()
            .unwrap()
            .iter()
            .find(|i| i["source_id"] == "watch/steps")
            .unwrap()["fact_snapshot"],
        frozen
    );
    privacy
        .restrict(
            &a,
            Uuid::new_v4(),
            Restriction::DeviceWithdraw,
            a.account_id,
            0,
        )
        .await
        .unwrap();
    assert!(
        workspace.read(&a).await.unwrap()["items"]
            .as_array()
            .unwrap()
            .iter()
            .all(|i| i["source_id"] != "watch/steps")
    );
    // Unknown journal schemas keep the restore gate closed, rather than silently ignoring historical receipts.
    let plain = FileJournal::open(root.clone()).await.unwrap();
    let malformed = Uuid::new_v4();
    plain
        .put_if_absent(malformed, b"{\"schema_version\":99}")
        .await
        .unwrap();
    let gate = PrivacyStore::new(db.clone(), Arc::new(plain));
    assert!(gate.reconcile_inventory().await.is_err());
    tokio::fs::remove_file(root.join(format!("{malformed}.json")))
        .await
        .unwrap();
    let deletion = Uuid::new_v4();
    privacy
        .restrict(
            &a,
            deletion,
            Restriction::EvidenceDelete,
            current.canonical_record_id,
            i64::from(current.revision),
        )
        .await
        .unwrap();
    assert!(evidence.exact(&a, &first, false).await.is_err());
    let delta = evidence.snapshot(&a, snapshot.cursor).await.unwrap();
    assert!(delta.records.is_empty());
    assert!(delta.changes[0].deleted);
    // A restored backup may contain an older revision of the same deleted source.
    sqlx::query("insert into health_evidence(id,user_id,concept_id,source_kind,measured_at,received_at,payload) values($1,$2,'test-measurement','appleHealth',now(),now(),$3)")
        .bind(current.canonical_record_id).bind(a.account_id).bind(&mutation.payload).execute(&mut owner).await.unwrap();
    sqlx::query("insert into canonical_evidence_revisions(revision,user_id,evidence_id,payload,content_sha256) overriding system value values($1,$2,$3,$4,repeat('0',64))")
        .bind(i64::from(first.revision)).bind(a.account_id).bind(first.canonical_record_id).bind(&mutation.payload).execute(&mut owner).await.unwrap();
    assert!(restored.replay(deletion).await.unwrap());
    assert!(evidence.exact(&a, &first, false).await.is_err());
    let signed_out = Uuid::new_v4();
    privacy
        .restrict(&a, signed_out, Restriction::SessionsRevoke, a.account_id, 2)
        .await
        .unwrap();
    assert!(matches!(
        second.authenticate("a", false).await,
        Err(ApiError::SessionRevoked)
    ));
    assert!(second.authenticate("b", false).await.is_ok());
    let mut recent_b = b.clone();
    recent_b.authenticated_at = Utc::now();
    let removed_b = Uuid::new_v4();
    privacy
        .restrict(
            &recent_b,
            removed_b,
            Restriction::AccountDelete,
            b.account_id,
            0,
        )
        .await
        .unwrap();
    assert!(second.authenticate("b", true).await.is_err());
    let deleted_retry = identities.authenticate_receipt_retry("b").await.unwrap();
    privacy
        .restrict(
            &deleted_retry,
            removed_b,
            Restriction::AccountDelete,
            b.account_id,
            0,
        )
        .await
        .unwrap();
    // The old signed session can reconcile its exact revoke receipt, never create a new restriction.
    let retry = identities.authenticate_receipt_retry("a").await.unwrap();
    privacy
        .restrict(
            &retry,
            signed_out,
            Restriction::SessionsRevoke,
            a.account_id,
            2,
        )
        .await
        .unwrap();
    assert!(
        privacy
            .restrict(
                &retry,
                Uuid::new_v4(),
                Restriction::SessionsRevoke,
                a.account_id,
                2
            )
            .await
            .is_err()
    );
    drop(restored);
    drop(privacy);
    drop(restart);
    drop(evidence);
    drop(identities);
    drop(second);
    drop(db);
    drop(db2);
    owner.close().await.unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
    tokio::fs::remove_dir_all(root).await.unwrap();
}

fn upsert(entity: Entity, record_id: &str, payload: serde_json::Value) -> EvidenceMutation {
    EvidenceMutation {
        operation_id: Uuid::new_v4(),
        canonical_record_id: Uuid::new_v4(),
        record_id: record_id.to_owned(),
        entity,
        expected_revision: None,
        source_generation: None,
        operation: EvidenceOperation::Upsert,
        payload,
    }
}
fn removal(of: &EvidenceMutation, expected: i32, source: serde_json::Value) -> EvidenceMutation {
    EvidenceMutation {
        operation_id: Uuid::new_v4(),
        canonical_record_id: of.canonical_record_id,
        record_id: of.record_id.clone(),
        entity: of.entity,
        expected_revision: Some(i64::from(expected)),
        source_generation: of.source_generation,
        operation: EvidenceOperation::Delete,
        payload: serde_json::json!({ "source": source }),
    }
}
fn epic(connection: &str, record: &str) -> serde_json::Value {
    serde_json::json!({"concept_id":"lab:potassium","measured_at":"2026-09-30T08:15:00-07:00","source":{"kind":"epic","connection_id":connection,"record_id":record},"source_record":{"resourceType":"Observation","id":record,"valueQuantity":{"value":4.1,"unit":"mmol/L"}},"value":{"literal":"4.1","unit":"mmol/L"}})
}
#[track_caller]
fn assert_refused<T: std::fmt::Debug>(result: Result<T, ApiError>) {
    assert!(
        matches!(result, Err(ApiError::Conflict(SOURCE_AUTHORITY_REFUSED))),
        "{result:?}"
    );
}
#[track_caller]
fn assert_conflict<T: std::fmt::Debug>(result: Result<T, ApiError>) {
    assert!(
        matches!(&result, Err(ApiError::Conflict(m)) if *m != SOURCE_AUTHORITY_REFUSED),
        "{result:?}"
    );
}

/// Canonical ingestion authority, honest dates, explicit source removal versus
/// privacy erasure, per-item batch isolation and restart readback.
#[tokio::test]
async fn canonical_ingestion_authority_removal_and_batch() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&options.get_host()));
    let mut admin = PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let role = format!("duet_authority_{suffix}");
    let name = format!("duet_authority_{suffix}");
    sqlx::raw_sql(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let owner_url = options.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = options
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    let mut owner = PgConnection::connect(&owner_url).await.unwrap();
    sqlx::raw_sql("revoke create on schema public from public")
        .execute(&mut owner)
        .await
        .unwrap();
    migrate(&owner_url, &role).await.unwrap();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let identities = IdentityStore::new(db.clone(), Arc::new(Verifier));
    let a = identities.authenticate("a", true).await.unwrap();
    let b = identities.authenticate("b", true).await.unwrap();
    let evidence = EvidenceStore::new(db.clone());
    let connector = IngestAuthority::connector("epic", "epic-connection-1").unwrap();
    assert!(IngestAuthority::connector("appleHealth", "x").is_err());

    // Retained originals: literal values, units and time zones survive; an
    // undated or partially dated source is stored with an unknown date, not receipt time.
    let dated = upsert(
        Entity::HealthEvidence,
        "HKQuantitySample/abc",
        serde_json::json!({"concept_id":"vital:heart_rate","measured_at":"2026-10-20T07:30:00+09:00","timezone":"Asia/Tokyo","source":{"kind":"appleHealth","record_id":"abc"},"value":{"literal":"072","unit":"count/min"}}),
    );
    let undated = upsert(
        Entity::HealthEvidence,
        "fhir/Observation/undated",
        serde_json::json!({"concept_id":"lab:a1c","source":{"kind":"healthRecords","record_id":"undated","declared_dates":{"issued":"2026-09"}},"value":{"literal":"< 5.7","unit":"%"}}),
    );
    let partial = upsert(
        Entity::HealthEvidence,
        "fhir/Observation/partial",
        serde_json::json!({"concept_id":"lab:ldl","measured_interval":{"start":"2026"},"source":{"kind":"healthRecords"},"value":{"literal":"99","unit":"mg/dL"}}),
    );
    let dated_at = evidence.ingest(&a, &dated).await.unwrap();
    let undated_at = evidence.ingest(&a, &undated).await.unwrap();
    let partial_at = evidence.ingest(&a, &partial).await.unwrap();
    for (address, mutation) in [
        (&dated_at, &dated),
        (&undated_at, &undated),
        (&partial_at, &partial),
    ] {
        assert_eq!(
            evidence.exact(&a, address, true).await.unwrap().payload,
            mutation.payload
        );
    }
    let stored: Vec<(Uuid, Option<chrono::DateTime<Utc>>)> =
        sqlx::query_as("select id,measured_at from health_evidence where user_id=$1")
            .bind(a.account_id)
            .fetch_all(&mut owner)
            .await
            .unwrap();
    let date_of = |id: Uuid| stored.iter().find(|(x, _)| *x == id).unwrap().1;
    assert_eq!(
        date_of(dated.canonical_record_id),
        Some("2026-10-19T22:30:00Z".parse().unwrap())
    );
    assert_eq!(date_of(undated.canonical_record_id), None);
    assert_eq!(date_of(partial.canonical_record_id), None);
    // Patient-authored wording is attributed separately and must be dated.
    let mut patient = upsert(
        Entity::ClinicalNote,
        "duet/note/1",
        serde_json::json!({"source_label":"Added in Duet","title":"My words","body":"Felt dizzy after the new dose."}),
    );
    assert!(matches!(
        evidence.ingest(&a, &patient).await,
        Err(ApiError::Unprocessable(_))
    ));
    patient.operation_id = Uuid::new_v4();
    patient.payload["recorded_at"] = serde_json::json!("2026-10-21T21:05:00-04:00");
    let patient_at = evidence.ingest(&a, &patient).await.unwrap();
    let (label, recorded): (String, Option<chrono::DateTime<Utc>>) =
        sqlx::query_as("select source_label,recorded_at from clinical_notes where id=$1")
            .bind(patient.canonical_record_id)
            .fetch_one(&mut owner)
            .await
            .unwrap();
    assert_eq!(label, "Added in Duet");
    assert_eq!(recorded, Some("2026-10-22T01:05:00Z".parse().unwrap()));

    // Duplicate and uncertain acknowledgements.
    assert_eq!(evidence.ingest(&a, &dated).await.unwrap(), dated_at);
    let mut reused = dated.clone();
    reused.payload["value"]["literal"] = serde_json::json!("073");
    assert_conflict(evidence.ingest(&a, &reused).await);
    // A lost acknowledgement is recovered by retrying the same operation identity;
    // a new identity for the same create is a stale write, never a second record.
    let mut lost_ack = dated.clone();
    lost_ack.operation_id = Uuid::new_v4();
    assert_conflict(evidence.ingest(&a, &lost_ack).await);
    // Corrections are optimistic against the exact retained revision.
    let mut correction = reused.clone();
    correction.operation_id = Uuid::new_v4();
    assert_conflict(evidence.ingest(&a, &correction).await);
    correction.expected_revision = Some(i64::from(dated_at.revision));
    let corrected_at = evidence.ingest(&a, &correction).await.unwrap();
    assert!(corrected_at.revision > dated_at.revision);
    assert!(!evidence.exact(&a, &dated_at, false).await.unwrap().current);

    // Unauthorized-source mutation.
    let device_epic = upsert(
        Entity::HealthEvidence,
        "epic/obs/1",
        epic("epic-connection-1", "obs-1"),
    );
    assert_refused(evidence.ingest(&a, &device_epic).await);
    for kind in ["oracleHealth", "healow", "googleHealth"] {
        let mut other = device_epic.clone();
        other.operation_id = Uuid::new_v4();
        other.payload["source"]["kind"] = serde_json::json!(kind);
        assert_refused(evidence.ingest(&a, &other).await);
    }
    let portal_at = evidence
        .ingest_with_authority(&a, &connector, &device_epic)
        .await
        .unwrap();
    assert_eq!(
        evidence
            .ingest_with_authority(&a, &connector, &device_epic)
            .await
            .unwrap(),
        portal_at
    );
    // The same operation identity under device authority is not a replay of the connector write.
    assert_conflict(evidence.ingest(&a, &device_epic).await);
    let mut takeover = device_epic.clone();
    takeover.operation_id = Uuid::new_v4();
    takeover.expected_revision = Some(i64::from(portal_at.revision));
    takeover.payload["source"] = serde_json::json!({"kind":"healthRecords","record_id":"obs-1"});
    assert_refused(evidence.ingest(&a, &takeover).await);
    let other_connection = IngestAuthority::connector("epic", "epic-connection-2").unwrap();
    let mut moved = device_epic.clone();
    moved.operation_id = Uuid::new_v4();
    moved.expected_revision = Some(i64::from(portal_at.revision));
    moved.payload = epic("epic-connection-2", "obs-1");
    assert_refused(
        evidence
            .ingest_with_authority(&a, &other_connection, &moved)
            .await,
    );
    let mut mismatched = moved.clone();
    mismatched.operation_id = Uuid::new_v4();
    assert_refused(
        evidence
            .ingest_with_authority(&a, &connector, &mismatched)
            .await,
    );
    assert_refused(
        evidence
            .ingest(
                &a,
                &removal(
                    &device_epic,
                    portal_at.revision,
                    serde_json::json!({"kind":"epic"}),
                ),
            )
            .await,
    );
    assert_refused(
        evidence
            .ingest(
                &a,
                &removal(
                    &device_epic,
                    portal_at.revision,
                    serde_json::json!({"kind":"healthRecords"}),
                ),
            )
            .await,
    );
    // An older client cannot replace the richer retained source object.
    let mut poorer = device_epic.clone();
    poorer.operation_id = Uuid::new_v4();
    poorer.expected_revision = Some(i64::from(portal_at.revision));
    poorer
        .payload
        .as_object_mut()
        .unwrap()
        .remove("source_record");
    assert_conflict(
        evidence
            .ingest_with_authority(&a, &connector, &poorer)
            .await,
    );
    let mut portal_note = upsert(
        Entity::ClinicalNote,
        "epic/DocumentReference/9",
        serde_json::json!({"source_label":"Epic MyChart","source":{"kind":"epic","connection_id":"epic-connection-1"},"title":"Visit summary","body":"Literal portal note","recorded_at":"2026-08-01"}),
    );
    assert_refused(evidence.ingest(&a, &portal_note).await);
    portal_note.operation_id = Uuid::new_v4();
    let portal_note_at = evidence
        .ingest_with_authority(&a, &connector, &portal_note)
        .await
        .unwrap();
    let mut relabel = portal_note.clone();
    relabel.operation_id = Uuid::new_v4();
    relabel.expected_revision = Some(i64::from(portal_note_at.revision));
    relabel.payload = serde_json::json!({"source_label":"Health Records","title":"Visit summary"});
    assert_refused(evidence.ingest(&a, &relabel).await);
    // Stale source generation is refused for device sources.
    sqlx::query("update users set apple_health_generation=apple_health_generation+1 where id=$1")
        .bind(a.account_id)
        .execute(&mut owner)
        .await
        .unwrap();
    let mut stale = upsert(
        Entity::HealthEvidence,
        "HKQuantitySample/stale",
        serde_json::json!({"concept_id":"vital:steps","source":{"kind":"appleHealth"},"value":{"literal":"10"}}),
    );
    assert_conflict(evidence.ingest(&a, &stale).await);
    stale.operation_id = Uuid::new_v4();
    stale.source_generation = Some(1);
    let stale_at = evidence.ingest(&a, &stale).await.unwrap();

    // Cross-account refusal: no read, no correction, no removal of another account's record.
    assert!(matches!(
        evidence.exact(&b, &undated_at, false).await,
        Err(ApiError::NotFound)
    ));
    let mut foreign = undated.clone();
    foreign.operation_id = Uuid::new_v4();
    assert_conflict(evidence.ingest(&b, &foreign).await);
    assert!(matches!(
        evidence
            .ingest(
                &b,
                &removal(
                    &undated,
                    undated_at.revision,
                    serde_json::json!({"kind":"healthRecords"})
                )
            )
            .await,
        Err(ApiError::NotFound)
    ));
    assert!(evidence.snapshot(&b, 0).await.unwrap().records.is_empty());

    // Explicit source removal keeps citations, withdraws current membership.
    let before = evidence.snapshot(&a, 0).await.unwrap();
    assert_conflict(
        evidence
            .ingest(
                &a,
                &removal(
                    &undated,
                    undated_at.revision + 1000,
                    serde_json::json!({"kind":"healthRecords"}),
                ),
            )
            .await,
    );
    let mut missing = removal(
        &undated,
        undated_at.revision,
        serde_json::json!({"kind":"healthRecords"}),
    );
    missing.canonical_record_id = Uuid::new_v4();
    assert!(matches!(
        evidence.ingest(&a, &missing).await,
        Err(ApiError::NotFound)
    ));
    let remove_undated = removal(
        &undated,
        undated_at.revision,
        serde_json::json!({"kind":"healthRecords"}),
    );
    let removed_at = evidence.ingest(&a, &remove_undated).await.unwrap();
    assert_eq!(removed_at, undated_at);
    assert_eq!(
        evidence.ingest(&a, &remove_undated).await.unwrap(),
        removed_at
    );
    let mut again = remove_undated.clone();
    again.operation_id = Uuid::new_v4();
    assert!(matches!(
        evidence.ingest(&a, &again).await,
        Err(ApiError::NotFound)
    ));
    let cited = evidence.exact(&a, &undated_at, false).await.unwrap();
    assert!(!cited.current);
    assert_eq!(cited.payload, undated.payload);
    assert_conflict(evidence.exact(&a, &undated_at, true).await);
    let delta = evidence.snapshot(&a, before.cursor).await.unwrap();
    assert_eq!(delta.changes.len(), 1);
    assert!(delta.changes[0].deleted && delta.changes[0].source_removed);
    assert!(delta.records.is_empty());
    let full = evidence.snapshot(&a, 0).await.unwrap();
    assert!(
        full.records
            .iter()
            .all(|r| r.address.canonical_record_id != undated.canonical_record_id)
    );
    assert_eq!(full.records.len(), before.records.len() - 1);
    // A removed identity is not revived by a later upsert.
    let mut revive = undated.clone();
    revive.operation_id = Uuid::new_v4();
    revive.expected_revision = Some(i64::from(undated_at.revision));
    assert_conflict(evidence.ingest(&a, &revive).await);
    // Connector removal requires its own connection; a device cannot remove a patient note
    // and an Apple source cannot remove a clinical note.
    assert_refused(
        evidence
            .ingest_with_authority(
                &a,
                &other_connection,
                &removal(
                    &portal_note,
                    portal_note_at.revision,
                    serde_json::json!({"kind":"epic","connection_id":"epic-connection-2"}),
                ),
            )
            .await,
    );
    assert_refused(
        evidence
            .ingest(
                &a,
                &removal(
                    &patient,
                    patient_at.revision,
                    serde_json::json!({"kind":"healthRecords"}),
                ),
            )
            .await,
    );
    assert!(matches!(
        evidence
            .ingest(
                &a,
                &EvidenceMutation {
                    source_generation: Some(1),
                    ..removal(
                        &patient,
                        patient_at.revision,
                        serde_json::json!({"kind":"appleHealth"})
                    )
                },
            )
            .await,
        Err(ApiError::Unprocessable(_))
    ));
    let portal_removed = evidence
        .ingest_with_authority(
            &a,
            &connector,
            &removal(
                &portal_note,
                portal_note_at.revision,
                serde_json::json!({"kind":"epic","connection_id":"epic-connection-1"}),
            ),
        )
        .await
        .unwrap();
    assert!(
        !evidence
            .exact(&a, &portal_removed, false)
            .await
            .unwrap()
            .current
    );

    // Privacy erasure remains authoritative, including over a source-removed record.
    let root = std::env::temp_dir().join(format!("duet-authority-{suffix}"));
    let privacy = PrivacyStore::new(
        db.clone(),
        Arc::new(FileJournal::open(root.clone()).await.unwrap()),
    );
    privacy
        .restrict(
            &a,
            Uuid::new_v4(),
            Restriction::EvidenceDelete,
            undated.canonical_record_id,
            i64::from(undated_at.revision),
        )
        .await
        .unwrap();
    assert!(matches!(
        evidence.exact(&a, &undated_at, false).await,
        Err(ApiError::NotFound)
    ));
    privacy
        .restrict(
            &a,
            Uuid::new_v4(),
            Restriction::EvidenceDelete,
            partial.canonical_record_id,
            i64::from(partial_at.revision),
        )
        .await
        .unwrap();
    assert!(matches!(
        evidence.exact(&a, &partial_at, false).await,
        Err(ApiError::NotFound)
    ));
    let erased: Vec<(bool, bool)> = sqlx::query_as(
        "select deleted,source_removed from evidence_sync_changes where record_id=any($1) order by record_id",
    )
    .bind(vec![undated.canonical_record_id, partial.canonical_record_id])
    .fetch_all(&mut owner)
    .await
    .unwrap();
    assert_eq!(erased, vec![(true, false), (true, false)]);
    let retained: i64 = sqlx::query_scalar(
        "select count(*) from canonical_evidence_revisions where coalesce(evidence_id,note_id)=any($1)",
    )
    .bind(vec![undated.canonical_record_id, partial.canonical_record_id])
    .fetch_one(&mut owner)
    .await
    .unwrap();
    assert_eq!(retained, 0);

    // Batch: each item commits or is reported; nothing is skipped silently.
    let good = upsert(
        Entity::HealthEvidence,
        "HKQuantitySample/batch-good",
        serde_json::json!({"concept_id":"vital:spo2","source":{"kind":"appleHealth"},"value":{"literal":"98","unit":"%"}}),
    );
    let mut good = good;
    good.source_generation = Some(1);
    let bad_source = upsert(
        Entity::HealthEvidence,
        "epic/obs/batch",
        epic("epic-connection-1", "batch"),
    );
    let invalid = upsert(
        Entity::HealthEvidence,
        "manual/batch",
        serde_json::json!({"source":{"kind":"manual"},"value":{"literal":"1"}}),
    );
    let mut stale_item = upsert(
        Entity::HealthEvidence,
        "HKQuantitySample/batch-stale",
        serde_json::json!({"concept_id":"vital:steps","source":{"kind":"appleHealth"}}),
    );
    stale_item.source_generation = Some(0);
    let missing_item = removal(&good, 1, serde_json::json!({"kind":"appleHealth"}));
    let mut missing_item = missing_item;
    missing_item.canonical_record_id = Uuid::new_v4();
    let batch = EvidenceBatch {
        items: vec![
            good.clone(),
            bad_source,
            invalid,
            stale_item,
            missing_item,
            good.clone(),
        ],
    };
    let cursor_before: i64 = sqlx::query_scalar("select evidence_revision from users where id=$1")
        .bind(a.account_id)
        .fetch_one(&mut owner)
        .await
        .unwrap();
    let result = evidence.ingest_batch(&a, &batch).await.unwrap();
    assert_eq!(result.results.len(), 6);
    let codes: Vec<_> = result
        .results
        .iter()
        .map(|outcome| match outcome {
            EvidenceItemOutcome::Accepted { .. } => None,
            EvidenceItemOutcome::Rejected { rejection, .. } => Some(rejection.code),
        })
        .collect();
    assert_eq!(
        codes,
        vec![
            None,
            Some(EvidenceRejectionCode::SourceAuthorityRefused),
            Some(EvidenceRejectionCode::Invalid),
            Some(EvidenceRejectionCode::Conflict),
            Some(EvidenceRejectionCode::NotFound),
            None,
        ]
    );
    let (
        EvidenceItemOutcome::Accepted { address: first, .. },
        EvidenceItemOutcome::Accepted {
            address: replay, ..
        },
    ) = (&result.results[0], &result.results[5])
    else {
        panic!("accepted outcomes expected")
    };
    assert_eq!(first, replay);
    let cursor_after: i64 = sqlx::query_scalar("select evidence_revision from users where id=$1")
        .bind(a.account_id)
        .fetch_one(&mut owner)
        .await
        .unwrap();
    assert_eq!(cursor_after, cursor_before + 1);
    assert_eq!(evidence.ingest_batch(&a, &batch).await.unwrap(), result);
    assert!(matches!(
        evidence
            .ingest_batch(&a, &EvidenceBatch { items: vec![] })
            .await,
        Err(ApiError::Unprocessable(_))
    ));
    assert!(matches!(
        evidence
            .ingest_batch(
                &a,
                &EvidenceBatch {
                    items: vec![good.clone(); 101]
                }
            )
            .await,
        Err(ApiError::Unprocessable(_))
    ));

    // Restart readback: a new pool and owner observe the same committed state.
    let snapshot = evidence.snapshot(&a, 0).await.unwrap();
    let restarted = EvidenceStore::new(AccountDatabase::connect(&runtime_url).await.unwrap());
    let again = restarted.snapshot(&a, 0).await.unwrap();
    assert_eq!(again.cursor, snapshot.cursor);
    assert_eq!(
        serde_json::to_value(&again.records).unwrap(),
        serde_json::to_value(&snapshot.records).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&again.changes).unwrap(),
        serde_json::to_value(&snapshot.changes).unwrap()
    );
    assert!(!restarted.exact(&a, &dated_at, false).await.unwrap().current);
    assert!(
        restarted
            .exact(&a, &corrected_at, true)
            .await
            .unwrap()
            .current
    );
    assert!(restarted.exact(&a, &stale_at, true).await.unwrap().current);
    assert!(
        !restarted
            .exact(&a, &portal_removed, false)
            .await
            .unwrap()
            .current
    );
    assert_eq!(
        restarted
            .ingest_with_authority(&a, &connector, &device_epic)
            .await
            .unwrap(),
        portal_at
    );

    drop(restarted);
    drop(privacy);
    drop(evidence);
    drop(identities);
    drop(db);
    owner.close().await.unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
    tokio::fs::remove_dir_all(root).await.unwrap();
}
