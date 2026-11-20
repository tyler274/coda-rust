//! Real PostgreSQL + HTTP end-to-end proofs for `[COD-63]` (Epic/MyChart & Oracle Health
//! portals, `"Doctors & hospitals"` search, multi-org independent daily refresh, partial
//! failure isolation, `"Couldn't update since <day>"` + `Reconnect`, vendor kill-switch,
//! setup `"Not now"`, and overdue diagnostics) and `[COD-64]` (Android Health Connect
//! outbox -> Postgres ingestion, Apple Health & Health Connect server disconnect sync
//! preserving history, and automatic Apple Health deletion day-total recomputation).
//!
//! Requires `DUET_TEST_DATABASE_URL` pointing at disposable loopback PostgreSQL.
#![allow(
    clippy::disallowed_methods,
    reason = "test provisions disposable PostgreSQL databases and asserts stored columns"
)]

use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use duet_api::database::{AccountDatabase, migrate};
use duet_api::durable::{DurableState, router as durable_router};
use duet_api::evidence::{EvidenceMutation, EvidenceOperation, EvidenceStore};
use duet_api::health_context::{HealthContextProducer, Upload};
use duet_api::identity::{IdentityStore, IdentityVerifier, VerifiedIdentity};
use duet_api::portals::{
    ConnectorSetupState as ApiSetupState, OverduePortalConnection, PortalFetchError,
    PortalFetchPage, PortalFetchedItem, PortalFhirClient, PortalOrganization, PortalService,
    PortalState, PortalTokenSet, PortalVendor, VendorSwitches, VendorToggle,
    router as portals_router, stable_portal_uuid,
};
use duet_api::privacy::{FileJournal, PrivacyStore};
use duet_client::DuetClient;
use duet_client::apple_health::{AppleHealth, AppleHealthDevice};
use duet_client::health_connect::{HealthConnect, HealthConnectDevice};
use duet_client::portals::{
    ConnectorSetupState, ImportOutcome, PhoneSourceKind, PortalClientError, PortalConnectionState,
    PortalVendor as ClientVendor,
};
use duet_client::retention::{ProtectedStore, RetainedIdentity, RetentionOwner};
use duet_health::AuthorizationError;
use duet_health::native_adapters::{
    BridgeError, HealthConnectBridge, HealthConnectChanges, HealthConnectRecord,
    HealthConnectScope, HealthKitBridge, HealthKitChanges, HealthKitQuery, NativeSample, TypeSpec,
};
use duet_wire::models::addressed_record_revision::Entity;
use serde_json::json;
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, PgConnection};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
struct MemoryProtectedStore {
    values: Mutex<HashMap<String, Vec<u8>>>,
    cleanup: Mutex<bool>,
}
impl ProtectedStore for MemoryProtectedStore {
    fn cleanup_pending(&self) -> Result<bool, String> {
        Ok(*self.cleanup.lock().unwrap())
    }
    fn set_cleanup_pending(&self, value: bool) -> Result<(), String> {
        *self.cleanup.lock().unwrap() = value;
        Ok(())
    }
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        Ok(self.values.lock().unwrap().get(key).cloned())
    }
    fn write(&self, key: &str, value: &[u8]) -> Result<(), String> {
        self.values.lock().unwrap().insert(key.into(), value.into());
        Ok(())
    }
    fn remove(&self, key: &str) -> Result<(), String> {
        self.values.lock().unwrap().remove(key);
        Ok(())
    }
}

struct StaticVerifier;
#[async_trait]
impl IdentityVerifier for StaticVerifier {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, duet_api::ApiError> {
        Ok(VerifiedIdentity {
            subject: format!("subject-{bearer}"),
            authenticated_at: Utc::now() - Duration::minutes(1),
        })
    }
}

#[derive(Clone)]
struct ScriptedClock(Arc<Mutex<DateTime<Utc>>>);
impl ScriptedClock {
    fn new(at: DateTime<Utc>) -> Self {
        Self(Arc::new(Mutex::new(at)))
    }
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
    fn advance(&self, delta: Duration) {
        *self.0.lock().unwrap() += delta;
    }
}

#[derive(Default)]
struct ScriptedFhirClient {
    pages_by_org: Mutex<BTreeMap<String, VecDeque<Result<PortalFetchPage, PortalFetchError>>>>,
    refresh_calls: Mutex<Vec<String>>,
}

impl ScriptedFhirClient {
    fn push(&self, org_id: &str, reply: Result<PortalFetchPage, PortalFetchError>) {
        self.pages_by_org
            .lock()
            .unwrap()
            .entry(org_id.to_owned())
            .or_default()
            .push_back(reply);
    }
}

#[async_trait]
impl PortalFhirClient for ScriptedFhirClient {
    async fn exchange_code(
        &self,
        organization: &PortalOrganization,
        code: &str,
        _code_verifier: &str,
        _redirect_uri: &str,
    ) -> Result<PortalTokenSet, PortalFetchError> {
        Ok(PortalTokenSet {
            access_token: format!("access-{}-{code}", organization.id),
            refresh_token: Some(format!("refresh-{}-{code}", organization.id)),
            expires_in_seconds: Some(3600),
            granted_scopes: vec!["patient/Observation.read".into()],
        })
    }

    async fn refresh_token(
        &self,
        organization: &PortalOrganization,
        _refresh_token: &str,
    ) -> Result<PortalTokenSet, PortalFetchError> {
        self.refresh_calls
            .lock()
            .unwrap()
            .push(organization.id.clone());
        Ok(PortalTokenSet {
            access_token: format!("access-refreshed-{}", organization.id),
            refresh_token: Some(format!("refresh-next-{}", organization.id)),
            expires_in_seconds: Some(3600),
            granted_scopes: vec!["patient/Observation.read".into()],
        })
    }

    async fn fetch_records(
        &self,
        organization: &PortalOrganization,
        _connection_id: Uuid,
        _access_token: &str,
        _cursor: Option<&str>,
    ) -> Result<PortalFetchPage, PortalFetchError> {
        self.pages_by_org
            .lock()
            .unwrap()
            .get_mut(&organization.id)
            .and_then(VecDeque::pop_front)
            .unwrap_or_else(|| Ok(PortalFetchPage::default()))
    }
}

fn portal_mutation(
    vendor: PortalVendor,
    connection_id: Uuid,
    source_record_id: &str,
    org_name: &str,
    value: &str,
) -> EvidenceMutation {
    EvidenceMutation {
        operation_id: Uuid::new_v4(),
        canonical_record_id: stable_portal_uuid(vendor, connection_id, source_record_id),
        record_id: source_record_id.to_owned(),
        entity: Entity::HealthEvidence,
        expected_revision: None,
        source_generation: None,
        operation: EvidenceOperation::Upsert,
        payload: json!({
            "concept_id": "lab:hemoglobin_a1c",
            "display_name": "Hemoglobin A1c",
            "measured_at": "2026-11-18T15:00:00Z",
            "value": { "literal": value, "unit": "%" },
            "source": {
                "kind": vendor.source_kind(),
                "connection_id": connection_id.to_string(),
                "record_id": source_record_id,
                "issuer": org_name,
            }
        }),
    }
}

struct TestDb {
    owner_url: String,
    runtime_url: String,
    writer_url: String,
    db: AccountDatabase,
    producer: HealthContextProducer,
}

async fn provision_test_db(tag: &str) -> TestDb {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let role = format!("duet_{tag}_rt_{suffix}");
    let writer = format!("duet_{tag}_hc_{suffix}");
    let name = format!("duet_{tag}_{suffix}");
    let password = Uuid::new_v4().simple().to_string();
    sqlx::raw_sql(AssertSqlSafe(format!(
        "create role {role} login password '{password}'; create role {writer} login password '{password}';"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let base = options.database(&name);
    let owner_url = base.to_url_lossy().to_string();
    let runtime_url = base
        .clone()
        .username(&role)
        .password(&password)
        .to_url_lossy()
        .to_string();
    let writer_url = base
        .username(&writer)
        .password(&password)
        .to_url_lossy()
        .to_string();
    let mut owner = PgConnection::connect(&owner_url).await.unwrap();
    sqlx::raw_sql("revoke create on schema public from public")
        .execute(&mut owner)
        .await
        .unwrap();
    migrate(&owner_url, &role).await.unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!(
        "grant usage on schema public to {writer}; \
         grant select on users,evidence_sync_changes,canonical_evidence_revisions,health_contexts,duet_health_context_operations,privacy_recovery_events to {writer}; \
         grant update(id) on users to {writer}; \
         grant insert,update on health_contexts to {writer}; \
         grant insert on duet_health_context_operations to {writer}; \
         grant execute on function acting_account() to {writer};"
    )))
    .execute(&mut owner)
    .await
    .unwrap();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let producer = HealthContextProducer::connect(&writer_url).await.unwrap();
    TestDb {
        owner_url,
        runtime_url,
        writer_url,
        db,
        producer,
    }
}

#[tokio::test]
async fn cod63_portals_multi_org_daily_refresh_partial_broken_kill_switch_not_now_and_overdue() {
    let env = provision_test_db("cod63").await;
    let _ = (&env.owner_url, &env.runtime_url, &env.writer_url);
    let journal = FileJournal::open(
        std::env::temp_dir().join(format!("duet-cod63-journal-{}", Uuid::new_v4().simple())),
    )
    .await
    .unwrap();
    let identities = IdentityStore::new(env.db.clone(), Arc::new(StaticVerifier));
    let evidence = EvidenceStore::new(env.db.clone());
    let durable = DurableState {
        identity: identities.clone(),
        privacy: PrivacyStore::new(env.db.clone(), Arc::new(journal)),
        evidence: evidence.clone(),
    };
    let patient_a = identities.authenticate("alice", true).await.unwrap();
    let _patient_b = identities.authenticate("bob", true).await.unwrap();

    let day1: DateTime<Utc> = "2026-11-19T10:00:00Z".parse().unwrap();
    let clock = ScriptedClock::new(day1);
    let clock_fn = {
        let clock = clock.clone();
        Arc::new(move || clock.now())
    };
    let fhir = Arc::new(ScriptedFhirClient::default());
    let service = PortalService::new(
        env.db.clone(),
        evidence.clone(),
        VendorSwitches::default(),
        "epic-client-test",
        "oracle-client-test",
    )
    .with_client(fhir.clone())
    .with_clock(clock_fn);

    let diag_token = "ops-secret-token";
    let app = durable_router(durable.clone()).merge(portals_router(PortalState {
        durable,
        service: service.clone(),
        diagnostics_token: Some(diag_token.to_owned()),
    }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let base = format!("http://{addr}");

    let alice = DuetClient::new(&base, "alice", "200", "1.0.0").unwrap();
    let bob = DuetClient::new(&base, "bob", "200", "1.0.0").unwrap();

    // 1. Initial setup snapshot + "Not now" persistence (`D1.4`)
    let initial = alice.portals().await.unwrap();
    assert_eq!(initial.setup_state, ConnectorSetupState::Pending);
    assert!(!initial.setup_complete);
    assert_eq!(initial.vendors.len(), 2);
    assert_eq!(initial.phone_sources.len(), 2);
    assert!(
        initial
            .phone_sources
            .iter()
            .any(|p| p.source == PhoneSourceKind::AppleHealth && p.guidance.contains("HealthKit"))
    );
    assert!(
        initial.phone_sources.iter().any(
            |p| p.source == PhoneSourceKind::HealthConnect && p.guidance.contains("background")
        )
    );

    let skipped = alice.skip_connector_setup_not_now().await.unwrap();
    assert_eq!(skipped.setup_state, ConnectorSetupState::NotNow);
    assert!(skipped.setup_complete);
    let after_skip = alice.portals().await.unwrap();
    assert_eq!(after_skip.setup_state, ConnectorSetupState::NotNow);
    assert!(after_skip.setup_complete);

    // 2. "Doctors & hospitals" search resolves Epic/MyChart and Oracle Health (`D2.1`)
    let ucsf_search = alice.search_portal_organizations("ucsf").await.unwrap();
    assert_eq!(ucsf_search.organizations.len(), 1);
    assert_eq!(ucsf_search.organizations[0].vendor, ClientVendor::Epic);
    let advent_search = alice
        .search_portal_organizations("adventhealth")
        .await
        .unwrap();
    assert_eq!(advent_search.organizations.len(), 1);
    assert_eq!(advent_search.organizations[0].vendor, ClientVendor::Oracle);

    // 3. Connect TWO Epic/MyChart organizations + ONE Oracle Health organization (`D2.2`, `D2.4`)
    let redirect = "duet://oauth/callback";
    let ucsf_start = alice
        .start_portal_oauth("epic-ucsf-health", redirect)
        .await
        .unwrap();
    assert!(
        ucsf_start
            .authorize_url
            .contains("code_challenge_method=S256")
    );
    // Bob cannot steal Alice's OAuth state (RLS + account check).
    assert!(matches!(
        bob.complete_portal_oauth(&ucsf_start.state, "code-1", redirect)
            .await,
        Err(PortalClientError::Conflict(_))
    ));

    // Script UCSF initial import with 1 readable item + 1 unreadable item -> Partial!
    // We don't know connection_id before complete_oauth, so push an unreadable item + a readable
    // item via a custom dynamic page? Actually let's complete UCSF with 1 unreadable item first,
    // verify Partial, then push a readable item using the returned connection_id!
    fhir.push(
        "epic-ucsf-health",
        Ok(PortalFetchPage {
            items: vec![PortalFetchedItem::Unreadable {
                record_id: "Observation/corrupt-1".into(),
                reason: "corrupt FHIR Observation".into(),
            }],
            next_cursor: None,
            has_more: false,
        }),
    );
    let ucsf_sync = alice
        .complete_portal_oauth(&ucsf_start.state, "code-ucsf", redirect)
        .await
        .unwrap();
    assert!(matches!(
        ucsf_sync.import,
        ImportOutcome::Partial { accepted: 0, ref failed } if failed.len() == 1
    ));
    assert!(matches!(
        ucsf_sync.connection.status,
        PortalConnectionState::Partial {
            failed_items: 1,
            ..
        }
    ));
    let ucsf_id = ucsf_sync.connection.id;

    // Now push 1 valid record + 1 unreadable record on UCSF refresh to prove readable publishes alone!
    fhir.push(
        "epic-ucsf-health",
        Ok(PortalFetchPage {
            items: vec![
                PortalFetchedItem::Readable(portal_mutation(
                    PortalVendor::Epic,
                    ucsf_id,
                    "Observation/a1c-ucsf-1",
                    "UCSF Health",
                    "6.2",
                )),
                PortalFetchedItem::Unreadable {
                    record_id: "Observation/corrupt-2".into(),
                    reason: "missing valueQuantity".into(),
                },
            ],
            next_cursor: None,
            has_more: false,
        }),
    );
    let ucsf_partial = alice.refresh_portal_connection(ucsf_id).await.unwrap();
    assert!(matches!(
        ucsf_partial.import,
        ImportOutcome::Partial { accepted: 1, ref failed } if failed.len() == 1
    ));
    let snap = evidence.snapshot(&patient_a, 0).await.unwrap();
    assert_eq!(snap.records.len(), 1);
    assert_eq!(snap.records[0].address.record_id, "Observation/a1c-ucsf-1");

    // Connect second MyChart org (Stanford) and Oracle Health org (AdventHealth)
    let stanford_start = alice
        .start_portal_oauth("epic-stanford-health-care", redirect)
        .await
        .unwrap();
    let stanford_sync = alice
        .complete_portal_oauth(&stanford_start.state, "code-stan", redirect)
        .await
        .unwrap();
    let stanford_id = stanford_sync.connection.id;
    assert_ne!(stanford_id, ucsf_id);

    let advent_start = alice
        .start_portal_oauth("oracle-adventhealth", redirect)
        .await
        .unwrap();
    let advent_sync = alice
        .complete_portal_oauth(&advent_start.state, "code-adv", redirect)
        .await
        .unwrap();
    let advent_id = advent_sync.connection.id;
    assert_eq!(advent_sync.connection.vendor, ClientVendor::Oracle);

    // Setup state advanced to Completed once a portal connected.
    let portals_now = alice.portals().await.unwrap();
    assert_eq!(portals_now.setup_state, ConnectorSetupState::Completed);
    assert_eq!(portals_now.connections.len(), 3);
    assert!(bob.portals().await.unwrap().connections.is_empty());

    // 4. Independent daily refresh (`D2.7`) + `"Couldn't update since <day>"` (`D2.8`)
    clock.advance(Duration::hours(24)); // 2026-11-20T10:00:00Z
    fhir.push(
        "epic-ucsf-health",
        Ok(PortalFetchPage {
            items: vec![PortalFetchedItem::Readable(portal_mutation(
                PortalVendor::Epic,
                ucsf_id,
                "Observation/a1c-ucsf-2",
                "UCSF Health",
                "6.0",
            ))],
            next_cursor: None,
            has_more: false,
        }),
    );
    fhir.push(
        "epic-stanford-health-care",
        Err(PortalFetchError::ReauthorizationRequired {
            reason: "MyChart token revoked".into(),
        }),
    );
    fhir.push(
        "oracle-adventhealth",
        Ok(PortalFetchPage {
            items: vec![PortalFetchedItem::Readable(portal_mutation(
                PortalVendor::Oracle,
                advent_id,
                "Observation/a1c-oracle-1",
                "AdventHealth",
                "5.9",
            ))],
            next_cursor: None,
            has_more: false,
        }),
    );

    let sweep = service.run_due_refreshes(64).await.unwrap();
    assert_eq!(sweep.attempted, 3);
    assert_eq!(sweep.complete, 2);
    assert_eq!(sweep.failed, 1);

    let after_sweep = alice.portals().await.unwrap();
    let stanford_view = after_sweep
        .connections
        .iter()
        .find(|c| c.id == stanford_id)
        .unwrap();
    assert_eq!(
        stanford_view.status_copy.as_deref(),
        Some("Couldn't update since Nov 20")
    );
    assert!(stanford_view.can_reconnect);
    assert!(matches!(
        stanford_view.status,
        PortalConnectionState::Broken {
            requires_reconnect: true,
            ..
        }
    ));

    // Another failed refresh on Nov 21 preserves the ORIGINAL failure day ("Nov 20").
    clock.advance(Duration::hours(24)); // 2026-11-21
    fhir.push(
        "epic-stanford-health-care",
        Err(PortalFetchError::ReauthorizationRequired {
            reason: "Still revoked".into(),
        }),
    );
    let _ = service.run_due_refreshes(64).await.unwrap();
    let stanford_day2 = alice
        .portals()
        .await
        .unwrap()
        .connections
        .into_iter()
        .find(|c| c.id == stanford_id)
        .unwrap();
    assert_eq!(
        stanford_day2.status_copy.as_deref(),
        Some("Couldn't update since Nov 20")
    );

    // 5. Engineering overdue diagnostics (`AC4`) — content-free & operator-authenticated
    let http = reqwest::Client::new();
    let unauth = http
        .get(format!("{base}/api/portals/diagnostics/overdue"))
        .bearer_auth("alice")
        .send()
        .await
        .unwrap();
    assert_eq!(unauth.status(), reqwest::StatusCode::UNAUTHORIZED);

    let diag_resp = http
        .get(format!("{base}/api/portals/diagnostics/overdue"))
        .bearer_auth(diag_token)
        .send()
        .await
        .unwrap();
    assert_eq!(diag_resp.status(), reqwest::StatusCode::OK);
    let overdue: Vec<OverduePortalConnection> = diag_resp.json().await.unwrap();
    let stanford_diag = overdue
        .iter()
        .find(|row| row.connection_id == stanford_id)
        .expect("broken Stanford connection must surface in overdue diagnostics");
    assert_eq!(stanford_diag.state, "broken");
    assert!(stanford_diag.reason.contains("revoked") || stanford_diag.reason.contains("Still"));

    // 6. Disconnect preserves imported history; reconnect reuses same connection_id (`D2.11`)
    let disconnected = alice.disconnect_portal_connection(ucsf_id).await.unwrap();
    assert!(matches!(
        disconnected.status,
        PortalConnectionState::Disconnected { .. }
    ));
    assert!(!disconnected.can_disconnect);
    assert!(disconnected.can_reconnect);
    let snap_after_disconnect = evidence.snapshot(&patient_a, 0).await.unwrap();
    assert!(
        snap_after_disconnect
            .records
            .iter()
            .any(|r| r.address.record_id == "Observation/a1c-ucsf-1"),
        "disconnect must keep imported history"
    );

    let reconnect_start = alice
        .start_portal_oauth("epic-ucsf-health", redirect)
        .await
        .unwrap();
    let reconnected = alice
        .complete_portal_oauth(&reconnect_start.state, "code-reconn", redirect)
        .await
        .unwrap();
    assert_eq!(
        reconnected.connection.id, ucsf_id,
        "reconnecting same organization must reuse its connection_id and history"
    );
    assert!(matches!(
        reconnected.connection.status,
        PortalConnectionState::Connected { .. }
    ));

    // 7. Vendor kill-switch (`D2.12`): switch Oracle Health OFF
    service.set_switches(VendorSwitches {
        epic: VendorToggle::Enabled,
        oracle: VendorToggle::Disabled,
    });
    let search_after_off = alice.search_portal_organizations("").await.unwrap();
    assert!(
        search_after_off
            .organizations
            .iter()
            .all(|org| org.vendor == ClientVendor::Epic),
        "switched-off Oracle Health must disappear from Doctors & hospitals search"
    );
    assert_eq!(search_after_off.vendors.len(), 1);
    assert!(matches!(
        alice
            .start_portal_oauth("oracle-intermountain-health", redirect)
            .await,
        Err(PortalClientError::VendorDisabled)
    ));

    // Existing AdventHealth connection STILL refreshes on daily sweep AND can disconnect!
    clock.advance(Duration::hours(24));
    fhir.push(
        "oracle-adventhealth",
        Ok(PortalFetchPage {
            items: vec![PortalFetchedItem::Readable(portal_mutation(
                PortalVendor::Oracle,
                advent_id,
                "Observation/a1c-oracle-after-off",
                "AdventHealth",
                "5.8",
            ))],
            next_cursor: None,
            has_more: false,
        }),
    );
    let sweep_after_off = service.run_due_refreshes(64).await.unwrap();
    assert!(sweep_after_off.complete >= 1);
    let snap_after_off = evidence.snapshot(&patient_a, 0).await.unwrap();
    assert!(
        snap_after_off
            .records
            .iter()
            .any(|r| r.address.record_id == "Observation/a1c-oracle-after-off"),
        "existing Oracle Health connection must keep refreshing after vendor kill-switch"
    );
    let advent_disconnected = alice.disconnect_portal_connection(advent_id).await.unwrap();
    assert!(matches!(
        advent_disconnected.status,
        PortalConnectionState::Disconnected { .. }
    ));
    let _ = ApiSetupState::Completed;
}

#[derive(Clone, Default)]
struct FakeAndroidDevice {
    granted: Arc<Mutex<Vec<String>>>,
    initial: Arc<Mutex<BTreeMap<String, Vec<HealthConnectRecord>>>>,
    changes: Arc<Mutex<VecDeque<HealthConnectChanges>>>,
}

impl HealthConnectBridge for FakeAndroidDevice {
    fn changes_token(&self, _record_types: &[String]) -> Result<String, BridgeError> {
        Ok("hc-tok-0".into())
    }
    fn changes(&self, _token: &str) -> Result<HealthConnectChanges, BridgeError> {
        Ok(self
            .changes
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| HealthConnectChanges {
                next_token: "hc-tok-1".into(),
                ..HealthConnectChanges::default()
            }))
    }
    fn granted_record_types(&self) -> Result<Option<Vec<String>>, BridgeError> {
        Ok(Some(self.granted.lock().unwrap().clone()))
    }
    fn read_initial_records(
        &self,
        record_type: &str,
        limit: usize,
    ) -> Result<Vec<HealthConnectRecord>, BridgeError> {
        Ok(self
            .initial
            .lock()
            .unwrap()
            .get(record_type)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .take(limit)
            .collect())
    }
}

impl HealthConnectDevice for FakeAndroidDevice {
    async fn authorize(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Clone, Default)]
struct FakeIosDevice {
    replies: Arc<Mutex<BTreeMap<String, VecDeque<HealthKitChanges>>>>,
}

impl FakeIosDevice {
    fn script(&self, data_type: &str, changes: HealthKitChanges) {
        self.replies
            .lock()
            .unwrap()
            .entry(data_type.to_owned())
            .or_default()
            .push_back(changes);
    }
}

impl HealthKitBridge for FakeIosDevice {
    fn available(&self) -> bool {
        true
    }
    fn read_type(&self, query: &HealthKitQuery) -> Result<HealthKitChanges, BridgeError> {
        Ok(self
            .replies
            .lock()
            .unwrap()
            .get_mut(&query.data_type)
            .and_then(VecDeque::pop_front)
            .unwrap_or_else(|| HealthKitChanges {
                anchor: query.anchor.clone().unwrap_or_else(|| "a0".into()),
                ..HealthKitChanges::default()
            }))
    }
}

impl AppleHealthDevice for FakeIosDevice {
    async fn authorize(&self) -> Result<(), AuthorizationError> {
        Ok(())
    }
}

#[tokio::test]
async fn cod64_health_connect_outbox_phone_disconnect_and_apple_health_deletion_recompute() {
    let env = provision_test_db("cod64").await;
    let journal = FileJournal::open(
        std::env::temp_dir().join(format!("duet-cod64-journal-{}", Uuid::new_v4().simple())),
    )
    .await
    .unwrap();
    let identities = IdentityStore::new(env.db.clone(), Arc::new(StaticVerifier));
    let evidence = EvidenceStore::new(env.db.clone()).with_health_context(env.producer.clone());
    let durable = DurableState {
        identity: identities.clone(),
        privacy: PrivacyStore::new(env.db.clone(), Arc::new(journal)),
        evidence: evidence.clone(),
    };
    let service = PortalService::new(
        env.db.clone(),
        evidence.clone(),
        VendorSwitches::default(),
        "epic-test",
        "oracle-test",
    );
    let patient = identities.authenticate("sam", true).await.unwrap();

    let app = durable_router(durable.clone()).merge(portals_router(PortalState {
        durable,
        service,
        diagnostics_token: None,
    }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let client = DuetClient::new(format!("http://{addr}"), "sam", "200", "1.0.0").unwrap();

    // 1. Android Health Connect acquisition -> durable outbox -> duet-api Postgres (`COD-64` AC1)
    let android_device = FakeAndroidDevice::default();
    *android_device.granted.lock().unwrap() = vec!["StepsRecord".into()];
    let today = Utc::now().date_naive();
    let sample_time = (today - Duration::days(1))
        .and_hms_opt(14, 0, 0)
        .unwrap()
        .and_utc()
        .to_rfc3339();
    android_device.initial.lock().unwrap().insert(
        "StepsRecord".into(),
        vec![HealthConnectRecord {
            record_type: "StepsRecord".into(),
            sample: NativeSample {
                id: "hc-steps-1".into(),
                value: "4100".into(),
                unit: Some("count".into()),
                start: Some(sample_time.clone()),
                end: Some(sample_time.clone()),
                time_zone: Some("UTC".into()),
                source_name: None,
                writer_id: Some("com.sec.android.app.shealth".into()),
                device: Some("Galaxy Watch".into()),
                metadata: None,
            },
        }],
    );

    let retention = RetentionOwner::new(Arc::new(MemoryProtectedStore::default()));
    let fence = retention
        .activate(RetainedIdentity {
            subject: "subject-sam".into(),
            owner: "sam".into(),
            refresh_token: "secret".into(),
        })
        .unwrap();
    let hc = HealthConnect::open(&retention, &fence, android_device.clone())
        .unwrap()
        .with_scope(HealthConnectScope::Enabled {
            types: vec![TypeSpec::new("StepsRecord", "activity:steps", "count")],
        });
    hc.connect(&client).await.unwrap();
    hc.refresh(&client).await.unwrap();

    let snap_hc = evidence.snapshot(&patient, 0).await.unwrap();
    assert_eq!(snap_hc.records.len(), 1);
    assert_eq!(
        snap_hc.records[0].address.record_id,
        "HealthConnect/hc-steps-1"
    );
    assert_eq!(
        snap_hc.records[0].payload["source"]["kind"],
        "healthConnect"
    );
    assert_eq!(snap_hc.records[0].payload["value"]["literal"], "4100");

    // Disconnect Health Connect synced to server without erasing history (`COD-64` AC3)
    hc.disconnect_and_sync(&client).await.unwrap();
    assert!(!hc.connected().unwrap());
    let portals = client.portals().await.unwrap();
    let hc_status = portals
        .phone_sources
        .iter()
        .find(|p| p.source == PhoneSourceKind::HealthConnect)
        .unwrap();
    assert!(hc_status.disconnected_at.is_some());
    assert_eq!(hc_status.generation, 0);
    assert_eq!(
        evidence.snapshot(&patient, 0).await.unwrap().records.len(),
        1,
        "Health Connect disconnect must keep imported history"
    );

    // 2. Apple Health deletion recomputation (`COD-64` AC2 / `D2.10`):
    //    Seed initial health_context upload, ingest 2 Apple Health step samples on the same day,
    //    delete one -> day total recalculates automatically, delete the second -> empty day drops
    //    from current evidence while retained revisions remain readable for prior sends!
    let ios_device = FakeIosDevice::default();
    let day = today - Duration::days(1);
    let t1 = day.and_hms_opt(9, 0, 0).unwrap().and_utc().to_rfc3339();
    let t2 = day.and_hms_opt(16, 0, 0).unwrap().and_utc().to_rfc3339();
    ios_device.script(
        "HKQuantityTypeIdentifierStepCount",
        HealthKitChanges {
            samples: vec![
                NativeSample {
                    id: "hk-step-1".into(),
                    value: "3000".into(),
                    unit: Some("count".into()),
                    start: Some(t1.clone()),
                    end: Some(t1),
                    time_zone: Some("UTC".into()),
                    source_name: Some("Apple Watch".into()),
                    writer_id: Some("com.apple.health".into()),
                    device: None,
                    metadata: None,
                },
                NativeSample {
                    id: "hk-step-2".into(),
                    value: "2000".into(),
                    unit: Some("count".into()),
                    start: Some(t2.clone()),
                    end: Some(t2),
                    time_zone: Some("UTC".into()),
                    source_name: Some("Apple Watch".into()),
                    writer_id: Some("com.apple.health".into()),
                    device: None,
                    metadata: None,
                },
            ],
            deleted: vec![],
            anchor: "hk-a1".into(),
            has_more: false,
        },
    );

    let apple = AppleHealth::open(&retention, &fence, ios_device.clone()).unwrap();
    apple.connect(&client).await.unwrap();
    apple.refresh(&client).await.unwrap();

    // Publish initial health context over the two Apple Health readings (3000 + 2000 = 5000).
    let initial_upload: Upload = serde_json::from_value(json!({
        "expected_revision": null,
        "snapshot": {
            "id": Uuid::new_v4(),
            "source_boundary_id": Uuid::new_v4(),
            "schema_version": 1,
            "generated_at": Utc::now(),
            "timezone": "UTC",
            "source_generations": {},
            "projections": [{
                "kind": "daily",
                "window_start": today.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                "window_end": (today + Duration::days(1)).and_hms_opt(0, 0, 0).unwrap().and_utc(),
                "timezone": "UTC",
                "first_weekday": 2,
                "facts": []
            }]
        }
    }))
    .unwrap();
    let stored = env
        .producer
        .upload(&patient, &initial_upload)
        .await
        .unwrap();
    let apple_steps_value = |ctx: &duet_api::health_context::StoredContext| -> Option<f64> {
        ctx.snapshot.projections[0]
            .facts
            .iter()
            .find(|f| {
                f.kind == duet_wire::models::HealthFactKind::CurrentValue
                    && f.concept_ids.iter().any(|c| c == "activity:steps")
                    && f.evidence.iter().any(|e| {
                        e.source_kind.as_ref().and_then(|s| s.as_deref()) == Some("appleHealth")
                    })
            })
            .and_then(|f| f.value.flatten())
    };
    assert_eq!(apple_steps_value(&stored), Some(5000.0));

    let snap_before_delete = evidence.snapshot(&patient, 0).await.unwrap();
    let step1_addr = snap_before_delete
        .records
        .iter()
        .find(|r| r.address.record_id == "HKObject/hk-step-1")
        .unwrap()
        .address
        .clone();
    let step2_addr = snap_before_delete
        .records
        .iter()
        .find(|r| r.address.record_id == "HKObject/hk-step-2")
        .unwrap()
        .address
        .clone();

    // Delete `hk-step-2` (2000) in Apple Health -> outbox uploads Delete -> EvidenceStore
    // AUTOMATICALLY recomputes HealthContextProducer from 5000 -> 3000 without any manual refresh!
    ios_device.script(
        "HKQuantityTypeIdentifierStepCount",
        HealthKitChanges {
            samples: vec![],
            deleted: vec!["hk-step-2".into()],
            anchor: "hk-a2".into(),
            has_more: false,
        },
    );
    apple.refresh(&client).await.unwrap();
    let after_first_delete = env.producer.read(&patient).await.unwrap().unwrap();
    assert_eq!(
        apple_steps_value(&after_first_delete),
        Some(3000.0),
        "deleting one Apple Health reading must automatically recalculate that day's total"
    );
    // Retained revision for hk-step-2 stays readable for prior sends (`current = false`).
    let retained_step2 = evidence.exact(&patient, &step2_addr, false).await.unwrap();
    assert!(!retained_step2.current);
    assert_eq!(retained_step2.payload["value"]["literal"], "2000");

    // Delete remaining `hk-step-1` (3000) -> day has no readings left -> dropped from current evidence!
    ios_device.script(
        "HKQuantityTypeIdentifierStepCount",
        HealthKitChanges {
            samples: vec![],
            deleted: vec!["hk-step-1".into()],
            anchor: "hk-a3".into(),
            has_more: false,
        },
    );
    apple.refresh(&client).await.unwrap();
    let after_second_delete = env.producer.read(&patient).await.unwrap().unwrap();
    assert_eq!(
        apple_steps_value(&after_second_delete),
        None,
        "deleting the final reading of a day must drop that day from current evidence"
    );
    let retained_step1 = evidence.exact(&patient, &step1_addr, false).await.unwrap();
    assert!(!retained_step1.current);
    assert_eq!(retained_step1.payload["value"]["literal"], "3000");

    // Sync Apple Health disconnect to server (`COD-64` AC3) while keeping retained history.
    apple.disconnect_and_sync(&client).await.unwrap();
    assert!(!apple.connected().unwrap());
    let portals_after = client.portals().await.unwrap();
    let apple_status = portals_after
        .phone_sources
        .iter()
        .find(|p| p.source == PhoneSourceKind::AppleHealth)
        .unwrap();
    assert!(apple_status.disconnected_at.is_some());
    assert_eq!(apple_status.generation, 0);
}
