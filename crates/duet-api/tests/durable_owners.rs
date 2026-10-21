//! Isolated real PostgreSQL proof; requires explicit loopback admin URL and never silently skips.
use async_trait::async_trait;
use chrono::{Duration, Utc};
use duet_api::{
    ApiError,
    database::{AccountDatabase, migrate},
    evidence::{EvidenceMutation, EvidenceStore},
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
