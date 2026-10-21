//! Real PostgreSQL, authenticated HTTP and actual patient inventory consumer.
use async_trait::async_trait;
use duet_api::{
    ApiError,
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::{EvidenceMutation, EvidenceStore},
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore, ReceiptJournal},
};
use duet_wire::models::addressed_record_revision::Entity;
use serde_json::{Value, json};
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;
use uuid::Uuid;
struct Identity;
#[async_trait]
impl IdentityVerifier for Identity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        Ok(VerifiedIdentity {
            subject: format!("cohort-{bearer}"),
            authenticated_at: chrono::Utc::now(),
        })
    }
}
#[tokio::test]
async fn paginated_exact_membership_account_revision_and_restore_fences() {
    let url = std::env::var("DUET_TEST_DATABASE_URL").expect("real local PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "127.0.0.1" | "localhost" | "::1"
    ));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("cohort_{suffix}");
    let role = format!("cohort_role_{suffix}");
    sqlx::query(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    let mut owner = sqlx::PgConnection::connect(options.to_url_lossy().as_str())
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("../migrations/0001_account_boundary.sql"))
        .execute(&mut owner)
        .await
        .unwrap();
    sqlx::query("insert into users(auth_user_id,sharing_label) values('cohort-a','Synthetic legacy account')").execute(&mut owner).await.unwrap();
    migrate(options.to_url_lossy().as_str(), &role)
        .await
        .unwrap();
    duet_api::producer_database::grant(
        options.to_url_lossy().as_str(),
        &role,
        duet_api::producer_database::ProducerKind::AcceptedWork,
    )
    .await
    .unwrap();
    let db = AccountDatabase::connect(options.username(&role).to_url_lossy().as_str())
        .await
        .unwrap();
    let identity = IdentityStore::new(db.clone(), Arc::new(Identity));
    let a = identity.authenticate("a", true).await.unwrap();
    let _b = identity.authenticate("b", true).await.unwrap();
    let evidence = EvidenceStore::new(db.clone());
    let mut addresses = Vec::new();
    for i in 0..55 {
        let payload = json!({"concept_id":"clinical:condition","clinical_record":{"kind":"condition","clinical_status":if i%2==0{"active"}else{"resolved"}},"category":if i%2==0{"currentCondition"}else{"pastCondition"},"concept_codings":if i==0{json!([{"system":"http://snomed.info/sct","code":"A"}])}else if i==1{json!([{"system":"http://snomed.info/sct","code":"A"},{"system":"http://snomed.info/sct","code":"B"}])}else{json!([{"system":"http://snomed.info/sct","code":"B"}])},"measured_at":"2026-10-20T00:00:00Z","source":{"kind":"manual"},"display_name":format!("Literal {i}")});
        addresses.push(
            evidence
                .ingest(
                    &a,
                    &EvidenceMutation {
                        operation_id: Uuid::new_v4(),
                        canonical_record_id: Uuid::new_v4(),
                        record_id: format!("original/{i}"),
                        entity: Entity::HealthEvidence,
                        expected_revision: None,
                        source_generation: None,
                        payload,
                    },
                )
                .await
                .unwrap(),
        );
    }
    let dir = std::env::temp_dir().join(format!("duet-cohort-{suffix}"));
    let journal = Arc::new(FileJournal::open(dir.clone()).await.unwrap());
    let privacy = PrivacyStore::new(db.clone(), journal.clone());
    let app = duet_api::evidence_routes::router(DurableState {
        identity: identity.clone(),
        privacy: privacy.clone(),
        evidence: evidence.clone(),
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let http = reqwest::Client::new();
    let first: Value = http
        .get(format!("{base}/api/evidence/records"))
        .bearer_auth("a")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(first["canonical_revisions"].as_array().unwrap().len(), 50);
    assert_eq!(first["total_count"], 55);
    let cursor = first["next_cursor"].as_str().unwrap();
    let second: Value = http
        .get(format!("{base}/api/evidence/records"))
        .query(&[("cursor", cursor)])
        .bearer_auth("a")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(second["canonical_revisions"].as_array().unwrap().len(), 5);
    assert!(second["next_cursor"].is_null());
    for (token, path) in [("b", "records"), ("a", "state")] {
        assert_eq!(
            http.get(format!("{base}/api/evidence/{path}"))
                .query(&[("cursor", cursor)])
                .bearer_auth(token)
                .send()
                .await
                .unwrap()
                .status(),
            409
        );
    }
    assert_eq!(
        http.get(format!("{base}/api/evidence/records"))
            .query(&[("cursor", cursor), ("source_kind", "manual")])
            .bearer_auth("a")
            .send()
            .await
            .unwrap()
            .status(),
        409
    );
    let client = duet_client::DuetClient::new(&base, "a", "test", "test").unwrap();
    let library = client.evidence_library().await.unwrap();
    assert_eq!(library.entries.len(), 1);
    assert_eq!(library.entries[0].member_count, 55);
    assert!(library.entries[0].status_conflict);
    let inventory = client.doctor_source_inventory().await.unwrap();
    assert_eq!(inventory.canonical_revisions.len(), 55);
    assert_eq!(inventory.health_evidence.len(), 55);
    assert!(inventory.next_cursor.is_none());
    let grouped = client
        .doctor_group_inventory("condition", addresses[0].canonical_record_id)
        .await
        .unwrap();
    assert_eq!(grouped.canonical_revisions.len(), 55);
    assert_eq!(grouped.health_evidence, inventory.health_evidence);
    let group: Value = http
        .get(format!("{base}/api/evidence/library"))
        .query(&[
            ("scope", "completeRecord"),
            ("group_kind", "condition"),
            (
                "anchor_record_id",
                addresses[0].canonical_record_id.to_string().as_str(),
            ),
        ])
        .bearer_auth("a")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(group["total_count"], 55);
    assert_eq!(group["entries"].as_array().unwrap().len(), 50);
    let key = duet_api::evidence_paging::query_key("records", None).unwrap();
    let snap = evidence.snapshot(&a, 0).await.unwrap();
    assert!(duet_api::evidence_paging::offset(Some(cursor), &a, &snap, &key).is_ok());
    let receipt = privacy
        .restrict(
            &a,
            Uuid::new_v4(),
            duet_api::privacy::Restriction::EvidenceDelete,
            addresses[0].canonical_record_id,
            i64::from(addresses[0].revision),
        )
        .await
        .unwrap();
    assert_eq!(
        http.get(format!("{base}/api/evidence/records"))
            .query(&[("cursor", cursor)])
            .bearer_auth("a")
            .send()
            .await
            .unwrap()
            .status(),
        409
    );
    assert!(!privacy.replay(receipt.event_id).await.unwrap());
    // Legacy account UUIDs are admitted only by migration, never at runtime.
    for (account, accepted) in [(a.account_id, true), (_b.account_id, false)] {
        let event = Uuid::new_v4();
        let legacy = json!({"schema_version":1,"event_id":event,"account_id":account,"operation":"luna_withdraw","scope_id":account,"scope_version":"","occurred_at":chrono::Utc::now(),"state":"restriction_committed"});
        journal
            .put_if_absent(event, &serde_json::to_vec(&legacy).unwrap())
            .await
            .unwrap();
        assert_eq!(privacy.replay(event).await.is_ok(), accepted);
        if !accepted {
            tokio::fs::remove_file(dir.join(format!("{event}.json")))
                .await
                .unwrap();
        }
    }
    // Imported follow-up incarnations are restricted by their original stable IDs.
    let follow = Uuid::new_v4();
    sqlx::query("insert into follow_ups(id,user_id,accepted_instruction,schedule,schedule_summary,state,source_message_ids) values($1,$2,'Synthetic retained work','{}','Retained','active','{}')").bind(follow).bind(a.account_id).execute(&mut owner).await.unwrap();
    for (operation, scope, state) in [
        ("follow_up_pause", follow, "paused"),
        ("follow_up_cancel", follow, "cancelled"),
        ("proactive_withdraw", a.account_id, "cancelled"),
    ] {
        let event = Uuid::new_v4();
        let bytes=serde_json::to_vec(&json!({"schema_version":1,"event_id":event,"account_id":a.account_id,"operation":operation,"scope_id":scope,"scope_version":"","occurred_at":chrono::Utc::now(),"state":"restriction_committed"})).unwrap();
        journal.put_if_absent(event, &bytes).await.unwrap();
        privacy.replay(event).await.unwrap();
        assert_eq!(journal.read(event).await.unwrap().unwrap(), bytes);
        let current: String = sqlx::query_scalar("select state from follow_ups where id=$1")
            .bind(follow)
            .fetch_one(&mut owner)
            .await
            .unwrap();
        assert_eq!(current, state);
    }
    // Full restore withdraws later positive grants; targeted replay retains its revision fence.
    sqlx::query(
        "update users set intelligence_enabled=true,intelligence_control_revision=99 where id=$1",
    )
    .bind(a.account_id)
    .execute(&mut owner)
    .await
    .unwrap();
    privacy.reconcile_inventory().await.unwrap();
    let enabled: bool = sqlx::query_scalar("select intelligence_enabled from users where id=$1")
        .bind(a.account_id)
        .fetch_one(&mut owner)
        .await
        .unwrap();
    assert!(!enabled);
    // Recreated account incarnation is not silently rebound to old legacy receipts.
    sqlx::query("update users set duet_incarnation=$2 where id=$1")
        .bind(a.account_id)
        .bind(Uuid::new_v4())
        .execute(&mut owner)
        .await
        .unwrap();
    assert!(privacy.reconcile_inventory().await.is_err());
    sqlx::query("update users set duet_incarnation=$2 where id=$1")
        .bind(a.account_id)
        .bind(a.incarnation)
        .execute(&mut owner)
        .await
        .unwrap();
    // Unknown active owners are never silently marked migrated.
    let event = Uuid::new_v4();
    let legacy = json!({"schema_version":1,"event_id":event,"account_id":a.account_id,"operation":"source_withdraw","scope_id":Uuid::new_v4(),"scope_version":"","occurred_at":chrono::Utc::now(),"state":"restriction_committed"});
    journal
        .put_if_absent(event, &serde_json::to_vec(&legacy).unwrap())
        .await
        .unwrap();
    assert!(privacy.reconcile_inventory().await.is_err());
    server.abort();
    drop(client);
    drop(identity);
    drop(evidence);
    drop(privacy);
    drop(db);
    sqlx::query(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
}
