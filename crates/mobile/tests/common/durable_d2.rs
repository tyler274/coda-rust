//! Durable-service D2 proof: isolated PostgreSQL, the real durable evidence and
//! privacy owners mounted in-process, and the real `duet-client` calls the
//! patient app makes. No installed phone and no portal provider participate.
use duet_api::{
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore, Restriction},
};
use duet_client::DuetClient;
use duet_wire::models::{
    CanonicalSourceAddress, evidence_browse_record::Status as Browse,
    evidence_revision_view::Status as Revision,
};
use serde_json::{Value, json};
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;
use uuid::Uuid;

const PATIENT: &str = "synthetic-d2-patient";
const FOREIGN: &str = "synthetic-d2-foreign";
/// Large enough to need several bounded pages (fifty records per page).
const IMPORTED: usize = 260;
const UNDATED: usize = 7;

struct Verifier;
#[async_trait::async_trait]
impl IdentityVerifier for Verifier {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, duet_api::ApiError> {
        if ![PATIENT, FOREIGN].contains(&bearer) {
            return Err(duet_api::ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: bearer.into(),
            authenticated_at: chrono::Utc::now() - chrono::Duration::seconds(5),
        })
    }
}

fn health(record: &str, payload: Value) -> Value {
    json!({"operation_id":Uuid::new_v4(),"canonical_record_id":Uuid::new_v4(),"record_id":record,"entity":"health_evidence","payload":payload})
}

async fn post(base: &str, bearer: &str, path: &str, body: &Value) -> Value {
    let response = reqwest::Client::new()
        .post(format!("{base}{path}"))
        .bearer_auth(bearer)
        .json(body)
        .send()
        .await
        .unwrap();
    let status = response.status();
    let text = response.text().await.unwrap();
    assert!(status.is_success(), "{path} answered {status}: {text}");
    serde_json::from_str(&text).unwrap()
}

fn address(value: &Value) -> CanonicalSourceAddress {
    serde_json::from_value(value.clone()).unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn d2_records_browse_correct_remove_and_erase_on_durable_services() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&options.get_host()));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("eng162_d2_{suffix}");
    let role = format!("eng162_d2_role_{suffix}");
    crate::disposable::create_database_and_role(&mut admin, &name, &role).await;
    let owner_url = options.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = options
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    migrate(&owner_url, &role).await.unwrap();
    let journal = std::env::temp_dir().join(format!("eng162-d2-journal-{suffix}"));
    run(&owner_url, &runtime_url, &journal).await;
    crate::disposable::drop_database_and_role(&mut admin, &name, &role).await;
    let _ = std::fs::remove_dir_all(journal);
}

#[expect(
    clippy::disallowed_methods,
    reason = "stages consent as the database owner, outside the API under proof"
)]
async fn run(owner_url: &str, runtime_url: &str, journal: &std::path::Path) {
    let database = AccountDatabase::connect(runtime_url).await.unwrap();
    let identity = IdentityStore::new(database.clone(), Arc::new(Verifier));
    let patient = identity.authenticate(PATIENT, true).await.unwrap();
    let foreign = identity.authenticate(FOREIGN, true).await.unwrap();
    let mut owner = sqlx::PgConnection::connect(owner_url).await.unwrap();
    sqlx::query("update users set intelligence_enabled=true,luna_consent_decision='accepted',luna_consent_version='2026-09-24',terms_version='2026-09-23' where id=$1")
        .bind(patient.account_id)
        .execute(&mut owner)
        .await
        .unwrap();
    sqlx::query("update users set terms_version='2026-09-23' where id=$1")
        .bind(foreign.account_id)
        .execute(&mut owner)
        .await
        .unwrap();
    let privacy = PrivacyStore::new(
        database.clone(),
        Arc::new(FileJournal::open(journal.to_path_buf()).await.unwrap()),
    );
    let state = DurableState {
        identity,
        evidence: EvidenceStore::new(database.clone()),
        privacy: privacy.clone(),
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let routes =
        duet_api::durable::router(state.clone()).merge(duet_api::evidence_routes::router(state));
    tokio::spawn(async move { axum::serve(listener, routes).await.unwrap() });
    let client = DuetClient::new(&base, PATIENT, "1", "0.1.0").unwrap();
    let outsider = DuetClient::new(&base, FOREIGN, "1", "0.1.0").unwrap();

    // Import at a realistic size: dated, undated and portal-shaped device records.
    let mut items = Vec::new();
    for i in 0..IMPORTED {
        let payload = if i < UNDATED {
            json!({"concept_id":"lab:undated","title":format!("Undated result {i}"),"source":{"kind":"healthRecords","record_id":format!("undated-{i}")},"value":{"literal":"< 0.03","unit":"mg/dL"}})
        } else if i + 1 == IMPORTED {
            // Interval-only shape of AcquisitionRecord::payload: no measured_at and no title.
            json!({
                "concept_id": "vital:steps",
                "timezone": "America/Los_Angeles",
                "source": {
                    "kind": "appleHealth",
                    "record_id": "HKQuantitySample/steps-day",
                    "data_type": "HKQuantityTypeIdentifierStepCount",
                    "source_name": "Apple Watch",
                    "writer_id": "com.apple.health.ABC",
                    "device": "Watch7,1"
                },
                "value": {"literal": "7654", "unit": "count"},
                "measured_interval": {
                    "start": "2026-10-28T00:00:00-07:00",
                    "end": "2026-10-29T00:00:00-07:00"
                },
                "status": "entered-in-error"
            })
        } else {
            json!({"concept_id":"lab:glucose","title":format!("Glucose {i}"),"measured_at":format!("2026-{:02}-{:02}T08:{:02}:00-07:00", 1 + i % 9, 1 + i % 27, i % 60),"source":{"kind":"healthRecords","record_id":format!("glucose-{i}")},"value":{"literal":format!("{}", 80 + i % 40),"unit":"mg/dL"},"status":"final"})
        };
        items.push(health(&format!("import/{i}"), payload));
    }
    let mut accepted = Vec::new();
    for batch in items.chunks(100) {
        let result = post(
            &base,
            PATIENT,
            "/api/canonical-evidence/batch",
            &json!({"items":batch}),
        )
        .await;
        for outcome in result["results"].as_array().unwrap() {
            assert_eq!(outcome["status"], "accepted", "{outcome}");
            accepted.push(outcome["address"].clone());
        }
    }
    assert_eq!(accepted.len(), IMPORTED);

    // Discovery is complete and bounded: every page is at most fifty, the cursor
    // chain reaches every record exactly once, and nothing is truncated.
    let mut cursor = None;
    let mut pages = 0;
    let mut walked = Vec::new();
    loop {
        let page = client.browse_records(cursor.take()).await.unwrap();
        assert!(page.records.len() <= 50);
        assert_eq!(page.total_count as usize, IMPORTED);
        walked.extend(page.records);
        pages += 1;
        match page.next_cursor {
            Some(next) => cursor = Some(next),
            None => break,
        }
    }
    assert!(pages >= 6, "{pages} pages");
    let ids: std::collections::HashSet<_> = walked
        .iter()
        .map(|r| r.address.canonical_record_id)
        .collect();
    assert_eq!(ids.len(), IMPORTED, "every record exactly once");
    assert_eq!(client.all_records().await.unwrap().len(), IMPORTED);
    // Undated records have no invented date and sort after every dated record.
    let undated: Vec<_> = walked.iter().filter(|r| r.date.is_none()).collect();
    assert_eq!(undated.len(), UNDATED);
    assert!(
        walked[IMPORTED - UNDATED..]
            .iter()
            .all(|r| r.date.is_none())
    );
    assert_eq!(undated[0].value.as_deref(), Some("< 0.03"));
    assert_eq!(undated[0].unit.as_deref(), Some("mg/dL"));
    // The interval-only acquisition record sorts on its start and keeps the end.
    let steps = &walked[0];
    assert_eq!(steps.title, "Health record");
    assert_eq!(steps.date.as_deref(), Some("2026-10-28T00:00:00-07:00"));
    assert_eq!(steps.date_end.as_deref(), Some("2026-10-29T00:00:00-07:00"));
    assert_eq!(steps.time_zone.as_deref(), Some("America/Los_Angeles"));
    assert_eq!(steps.value.as_deref(), Some("7654"));
    assert_eq!(steps.unit.as_deref(), Some("count"));
    assert_eq!(steps.source_status.as_deref(), Some("entered-in-error"));
    assert_eq!(
        steps.payload.as_ref().unwrap()["source"]["record_id"],
        "HKQuantitySample/steps-day"
    );
    let dated = walked
        .iter()
        .find(|r| r.provenance.source_kind == "healthRecords" && r.date.is_some())
        .unwrap();
    assert_eq!(dated.time_zone.as_deref(), Some("-07:00"));
    assert_eq!(dated.provenance.source_kind, "healthRecords");
    assert!(dated.payload.as_ref().unwrap().contains_key("source"));
    // A stale or foreign cursor never silently splices pages.
    let first = client.browse_records(None).await.unwrap();
    let stale = first.next_cursor.clone().unwrap();
    assert!(outsider.browse_records(Some(stale.clone())).await.is_err());

    // One exact record revision reads back with its status.
    let note = health(
        "portal/a1c",
        json!({"concept_id":"lab:a1c","title":"Hemoglobin A1c","measured_at":"2026-10-01T09:00:00-04:00","source":{"kind":"healthRecords","record_id":"a1c-original"},"value":{"literal":"5.4","unit":"%"},"status":"final"}),
    );
    let original = post(&base, PATIENT, "/api/canonical-evidence", &note).await;
    let record: Uuid = original["canonical_record_id"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let first_revision = original["revision"].as_i64().unwrap() as i32;
    let exact = client.record_revision(address(&original)).await.unwrap();
    assert_eq!(exact.status, Revision::Current);
    assert_eq!(exact.address.revision, first_revision);
    // A browse page read now carries the record's exact revision.
    let listed = client.all_records().await.unwrap();
    assert!(
        listed
            .iter()
            .any(|r| r.address.canonical_record_id == record
                && r.address.revision == first_revision
                && r.status == Browse::Current)
    );

    // Correction: the original revision reopens with a newer-version notice and
    // keeps its own value; the current revision carries the correction.
    let mut correction = note.clone();
    correction["operation_id"] = json!(Uuid::new_v4());
    correction["expected_revision"] = json!(first_revision);
    correction["payload"]["value"]["literal"] = json!("5.6");
    let corrected = post(&base, PATIENT, "/api/canonical-evidence", &correction).await;
    let second_revision = corrected["revision"].as_i64().unwrap() as i32;
    assert!(second_revision > first_revision);
    let reopened = client.record_revision(address(&original)).await.unwrap();
    assert_eq!(reopened.status, Revision::Superseded);
    assert_eq!(reopened.newer_revision_exists, Some(true));
    assert_eq!(reopened.address.revision, first_revision);
    let body = reopened.payload.clone().flatten().unwrap();
    assert_eq!(
        body["value"]["literal"], "5.4",
        "the original, not the correction"
    );
    let current = client.record_revision(address(&corrected)).await.unwrap();
    assert_eq!(current.status, Revision::Current);
    assert_eq!(
        current.payload.flatten().unwrap()["value"]["literal"],
        "5.6"
    );

    // Source removal: still listed and labelled, and both revisions stay readable.
    let removal = json!({"operation_id":Uuid::new_v4(),"canonical_record_id":record,"record_id":"portal/a1c","entity":"health_evidence","expected_revision":second_revision,"operation":"delete","payload":{"source":{"kind":"healthRecords"}}});
    post(&base, PATIENT, "/api/canonical-evidence", &removal).await;
    let browse = client.browse_records(None).await.unwrap();
    assert_eq!(browse.removed_count, Some(1));
    let listed = client.all_records().await.unwrap();
    let removed = listed
        .iter()
        .find(|r| r.address.canonical_record_id == record)
        .expect("source-removed records stay discoverable");
    assert_eq!(removed.status, Browse::RemovedBySource);
    let old = client.record_revision(address(&original)).await.unwrap();
    assert_eq!(old.status, Revision::RemovedBySource);
    assert_eq!(old.payload.flatten().unwrap()["value"]["literal"], "5.4");
    let latest = client.record_revision(address(&corrected)).await.unwrap();
    assert_eq!(latest.status, Revision::RemovedBySource);

    // Cross-account refusal: no browse membership and no exact read.
    assert_eq!(outsider.all_records().await.unwrap().len(), 0);
    assert!(outsider.record_revision(address(&original)).await.is_err());
    let _ = foreign;

    // Privacy erasure: no exact read and no listing.
    let erasable = &accepted[UNDATED];
    let erased: Uuid = erasable["canonical_record_id"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let erased_revision = erasable["revision"].as_i64().unwrap() as i32;
    assert!(client.record_revision(address(erasable)).await.is_ok());
    privacy
        .restrict(
            &patient,
            Uuid::new_v4(),
            Restriction::EvidenceDelete,
            erased,
            i64::from(erased_revision),
        )
        .await
        .unwrap();
    assert!(client.record_revision(address(erasable)).await.is_err());
    let listed = client.all_records().await.unwrap();
    assert!(
        listed
            .iter()
            .all(|r| r.address.canonical_record_id != erased)
    );
    assert_eq!(listed.len(), IMPORTED); // 260 imports + A1c (removed) − 1 erased
}
