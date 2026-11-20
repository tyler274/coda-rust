//! Client-side HTTP and outbox proofs for `[COD-63]` and `[COD-64]`.
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::http::StatusCode;
use axum::routing::{get, post};
use duet_client::DuetClient;
use duet_client::apple_health::{AppleHealth, AppleHealthDevice};
use duet_client::health_connect::{HealthConnect, HealthConnectDevice};
use duet_client::portals::{ConnectorSetupState, PhoneSourceKind, PortalClientError, PortalVendor};
use duet_client::retention::{ProtectedStore, RetainedIdentity, RetentionOwner};
use duet_health::AuthorizationError;
use duet_health::native_adapters::{
    BridgeError, HealthConnectBridge, HealthConnectChanges, HealthKitBridge, HealthKitChanges,
    HealthKitQuery,
};
use serde_json::json;
use std::collections::HashMap;

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

#[derive(Clone, Default)]
struct StubHealthConnect;
impl HealthConnectBridge for StubHealthConnect {
    fn changes_token(&self, _record_types: &[String]) -> Result<String, BridgeError> {
        Ok("tok-0".into())
    }
    fn changes(&self, _token: &str) -> Result<HealthConnectChanges, BridgeError> {
        Ok(HealthConnectChanges {
            next_token: "tok-1".into(),
            ..HealthConnectChanges::default()
        })
    }
}
impl HealthConnectDevice for StubHealthConnect {
    async fn authorize(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Clone, Default)]
struct StubAppleHealth;
impl HealthKitBridge for StubAppleHealth {
    fn available(&self) -> bool {
        true
    }
    fn read_type(&self, _query: &HealthKitQuery) -> Result<HealthKitChanges, BridgeError> {
        Ok(HealthKitChanges {
            anchor: "a1".into(),
            ..HealthKitChanges::default()
        })
    }
}
impl AppleHealthDevice for StubAppleHealth {
    async fn authorize(&self) -> Result<(), AuthorizationError> {
        Ok(())
    }
}

#[tokio::test]
async fn client_handles_portals_vendor_disabled_not_now_and_phone_disconnect_sync() {
    let disconnects: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let disc_apple = disconnects.clone();
    let disc_hc = disconnects.clone();

    let app = axum::Router::new()
        .route(
            "/api/portals/organizations",
            get(|| async {
                Json(json!({
                    "vendors": [{"vendor": "epic", "brand": "Epic / MyChart"}],
                    "organizations": [{
                        "id": "epic-ucsf-health",
                        "name": "UCSF Health",
                        "vendor": "epic",
                        "brand": "Epic / MyChart",
                        "location": "San Francisco, CA",
                        "fhir_base_url": "https://unified-api.ucsf.edu/clinical/apex/api/FHIR/R4",
                        "authorize_url": "https://unified-api.ucsf.edu/clinical/apex/oauth2/authorize",
                        "token_url": "https://unified-api.ucsf.edu/clinical/apex/oauth2/token"
                    }]
                }))
            }),
        )
        .route(
            "/api/portals/setup/not-now",
            post(|| async {
                Json(json!({
                    "setup_state": "not_now",
                    "setup_complete": true
                }))
            }),
        )
        .route(
            "/api/portals/oauth/start",
            post(|| async {
                (
                    StatusCode::CONFLICT,
                    Json(json!({"detail": {"code": "stale_state", "message": "portal_vendor_disabled"}})),
                )
            }),
        )
        .route(
            "/api/apple-health/generation",
            get(|| async { Json(0i32) }),
        )
        .route(
            "/api/phone-sources/healthConnect/generation",
            get(|| async { Json(0i32) }),
        )
        .route(
            "/api/phone-sources/healthConnect/connect",
            post(|| async {
                Json(json!({
                    "source": "healthConnect",
                    "generation": 0,
                    "disconnected_at": null,
                    "guidance": duet_client::HEALTH_CONNECT_BACKGROUND_LIMIT_NOTICE
                }))
            }),
        )
        .route(
            "/api/phone-sources/healthConnect/disconnect",
            post(move || {
                let disc = disc_hc.clone();
                async move {
                    disc.lock().unwrap().push("healthConnect".into());
                    Json(json!({
                        "source": "healthConnect",
                        "generation": 0,
                        "disconnected_at": "2026-11-19T12:00:00Z",
                        "guidance": duet_client::HEALTH_CONNECT_BACKGROUND_LIMIT_NOTICE
                    }))
                }
            }),
        )
        .route(
            "/api/phone-sources/appleHealth/disconnect",
            post(move || {
                let disc = disc_apple.clone();
                async move {
                    disc.lock().unwrap().push("appleHealth".into());
                    Json(json!({
                        "source": "appleHealth",
                        "generation": 0,
                        "disconnected_at": "2026-11-19T12:00:00Z",
                        "guidance": duet_client::APPLE_HEALTH_WEARABLE_INSTRUCTION
                    }))
                }
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let client = DuetClient::new(format!("http://{addr}"), "tok", "200", "1.0.0").unwrap();

    let orgs = client.search_portal_organizations("ucsf").await.unwrap();
    assert_eq!(orgs.organizations.len(), 1);
    assert_eq!(orgs.organizations[0].vendor, PortalVendor::Epic);

    let skipped = client.skip_connector_setup_not_now().await.unwrap();
    assert_eq!(skipped.setup_state, ConnectorSetupState::NotNow);
    assert!(skipped.setup_complete);

    assert!(matches!(
        client
            .start_portal_oauth("oracle-adventhealth", "duet://oauth/callback")
            .await,
        Err(PortalClientError::VendorDisabled)
    ));

    let retention = RetentionOwner::new(Arc::new(MemoryProtectedStore::default()));
    let fence = retention
        .activate(RetainedIdentity {
            subject: "subject-patient-1".into(),
            owner: "patient-1".into(),
            refresh_token: "secret".into(),
        })
        .unwrap();
    let hc = HealthConnect::open(&retention, &fence, StubHealthConnect).unwrap();
    hc.connect(&client).await.unwrap();
    assert!(hc.connected().unwrap());
    hc.disconnect_and_sync(&client).await.unwrap();
    assert!(!hc.connected().unwrap());

    let apple = AppleHealth::open(&retention, &fence, StubAppleHealth).unwrap();
    apple.connect(&client).await.unwrap();
    assert!(apple.connected().unwrap());
    apple.disconnect_and_sync(&client).await.unwrap();
    assert!(!apple.connected().unwrap());

    assert_eq!(
        *disconnects.lock().unwrap(),
        vec!["healthConnect".to_string(), "appleHealth".to_string()]
    );
    assert_eq!(PhoneSourceKind::HealthConnect.as_str(), "healthConnect");
    assert!(duet_client::APPLE_HEALTH_WEARABLE_INSTRUCTION.contains("HealthKit"));
    assert!(duet_client::HEALTH_CONNECT_BACKGROUND_LIMIT_NOTICE.contains("background"));
}
