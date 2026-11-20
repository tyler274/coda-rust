//! Physician-side Connected Care state: the patient link a patient page shows.
//! The patient product owns invitation acceptance and consent.

use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use serde_json::Value;

use crate::timestamp::format_rfc3339;
use front_door::{FrontDoorStore, MemoryFrontDoorStore, RosterConnectionStatus, RosterPatient};
use physician_auth::{
    AuthStore, Membership, MemoryAuthStore, PhysicianIdentity, PracticeClock, PracticeTimeZone,
    StoreUnavailable,
};
use physician_wire::{ConnectionStatusWire, PatientLinkWire};

use crate::auth::{AuthContext, STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK, now};
use crate::patients::{self, DirectoryError, MemoryPatients, Patient, PatientDirectory};

#[derive(Debug, thiserror::Error)]
pub(crate) enum PatientLookupError {
    #[error("physician auth store unavailable")]
    Auth,
    #[error(transparent)]
    Directory(#[from] DirectoryError),
}

/// Authoritative practice clock and time-zone snapshot for a request.
#[derive(Debug, Clone)]
pub(crate) struct PracticeMoment {
    /// Practice clinical "now" in Unix seconds.
    pub(crate) now_unix: u64,
    /// Practice clinical "now" formatted as RFC 3339 (`YYYY-MM-DDTHH:MM:SSZ`).
    pub(crate) now_rfc3339: String,
    /// Local calendar date (`YYYY-MM-DD`) in the practice's saved IANA time zone.
    pub(crate) date_iso: String,
    /// Saved practice IANA time zone (defaulting to `America/New_York` only when no row exists).
    pub(crate) time_zone: PracticeTimeZone,
}

/// Shared Connected Care state attached to every request on a physician router.
#[derive(Clone)]
pub(crate) struct ConnectedCare {
    fallback_auth: Arc<dyn AuthStore>,
    fallback_front_door: Arc<dyn FrontDoorStore>,
    fallback_patients: Arc<dyn PatientDirectory>,
}

impl ConnectedCare {
    /// Creates a new Connected Care state with its own fallback memory stores,
    /// holding no patients.
    pub(crate) fn new() -> Self {
        Self::on(
            Arc::new(Self::memory_store()),
            Arc::new(MemoryFrontDoorStore::new()),
            Arc::new(MemoryPatients::default()),
        )
    }

    /// The fallback memory stores over the fixture patients, on the preview
    /// identity's panel.
    pub(crate) fn fixture() -> Self {
        let store = Self::memory_store();
        let patients = MemoryPatients::fixture();
        let preview = Self::identity(&axum::http::Extensions::new());
        for workspace in crate::WORKSPACE_IDS {
            store.seed_patient(STAGING_PRACTICE_ID, workspace);
            store.seed_membership(&preview, workspace, Membership::Allowed);
        }
        Self::on(
            Arc::new(store),
            Arc::new(MemoryFrontDoorStore::new()),
            patients,
        )
    }

    fn memory_store() -> MemoryAuthStore {
        let store = MemoryAuthStore::new();
        store.seed_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK);
        store.seed_account(
            STAGING_PRACTICE_ID,
            "staging-physician",
            "physician@codahealth.test",
            "Staging Physician",
            true,
        );
        store
    }

    fn on(
        fallback_auth: Arc<dyn AuthStore>,
        fallback_front_door: Arc<dyn FrontDoorStore>,
        fallback_patients: Arc<dyn PatientDirectory>,
    ) -> Self {
        Self {
            fallback_auth,
            fallback_front_door,
            fallback_patients,
        }
    }

    pub(crate) fn auth_store(&self, extensions: &axum::http::Extensions) -> Arc<dyn AuthStore> {
        extensions
            .get::<Arc<dyn AuthStore>>()
            .cloned()
            .unwrap_or_else(|| Arc::clone(&self.fallback_auth))
    }

    pub(crate) fn front_door(
        &self,
        extensions: &axum::http::Extensions,
    ) -> Arc<dyn FrontDoorStore> {
        extensions
            .get::<Arc<dyn FrontDoorStore>>()
            .cloned()
            .unwrap_or_else(|| Arc::clone(&self.fallback_front_door))
    }

    pub(crate) fn patients(
        &self,
        extensions: &axum::http::Extensions,
    ) -> Arc<dyn PatientDirectory> {
        extensions
            .get::<Arc<dyn PatientDirectory>>()
            .cloned()
            .unwrap_or_else(|| Arc::clone(&self.fallback_patients))
    }

    /// `id` if it is a patient of the signed-in practice a source holds.
    pub(crate) async fn patient(
        &self,
        extensions: &axum::http::Extensions,
        id: &str,
    ) -> Result<Option<Patient>, PatientLookupError> {
        let practice = Self::identity(extensions).practice_id;
        let store = self.auth_store(extensions);
        if store
            .patient_practice(id)
            .await
            .map_err(|StoreUnavailable| PatientLookupError::Auth)?
            .as_deref()
            != Some(practice.as_str())
        {
            return Ok(None);
        }
        Ok(patients::read_patient(&*self.patients(extensions), id).await?)
    }

    pub(crate) fn identity(extensions: &axum::http::Extensions) -> PhysicianIdentity {
        extensions
            .get::<AuthContext>()
            .map(|context| context.identity.clone())
            .unwrap_or_else(|| PhysicianIdentity {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                physician_id: "staging-physician".to_owned(),
            })
    }

    /// Resolves the signed-in practice's clinical clock, local calendar date, and saved IANA time zone.
    pub(crate) async fn practice_moment(
        &self,
        extensions: &axum::http::Extensions,
    ) -> Result<PracticeMoment, StoreUnavailable> {
        let store = self.auth_store(extensions);
        let identity = Self::identity(extensions);
        let (clock, time_zone) = match store.practice(&identity.practice_id).await? {
            Some(record) => (record.clock, record.time_zone),
            None => (PracticeClock::RealTime, PracticeTimeZone::default()),
        };
        let now_unix = clock.now(now());
        let now_rfc3339 = format_rfc3339(UNIX_EPOCH + Duration::from_secs(now_unix));
        let date_iso = time_zone.calendar_date_iso(now_unix);
        Ok(PracticeMoment {
            now_unix,
            now_rfc3339,
            date_iso,
            time_zone,
        })
    }

    /// The signed-in practice's clinical "now", as the wire carries it.
    pub(crate) async fn practice_now(
        &self,
        extensions: &axum::http::Extensions,
    ) -> Result<String, StoreUnavailable> {
        Ok(self.practice_moment(extensions).await?.now_rfc3339)
    }

    pub(crate) async fn link_wire(
        &self,
        front_door: &dyn FrontDoorStore,
        practice_id: &str,
        workspace_id: &str,
    ) -> PatientLinkWire {
        let status = front_door
            .roster_connection(&RosterPatient {
                practice_id: practice_id.to_owned(),
                patient_id: workspace_id.to_owned(),
            })
            .await;
        status_to_link_wire(practice_id, workspace_id, &status)
    }
}

/// Maps front-door `RosterConnectionStatus` into wire `ConnectionStatusWire`.
pub(crate) fn to_wire_status(status: &RosterConnectionStatus) -> ConnectionStatusWire {
    match status {
        RosterConnectionStatus::NotInvited => ConnectionStatusWire::NotInvited,
        RosterConnectionStatus::Invited { .. } => ConnectionStatusWire::Invited,
        RosterConnectionStatus::Connected { .. } => ConnectionStatusWire::Connected,
        RosterConnectionStatus::Ended { .. } => ConnectionStatusWire::Ended,
    }
}

/// Projects `RosterConnectionStatus` onto `PatientLinkWire`.
pub(crate) fn status_to_link_wire(
    practice_id: &str,
    workspace_id: &str,
    status: &RosterConnectionStatus,
) -> PatientLinkWire {
    let connection_status = to_wire_status(status);
    match status {
        RosterConnectionStatus::Connected { account_id, .. } => PatientLinkWire {
            practice_id: practice_id.to_owned(),
            patient_workspace_id: workspace_id.to_owned(),
            duet_account_id: Some(account_id.to_string()),
            linked: true,
            sharing_consented: true,
            connection_status,
            consent_version: None,
            consented_at: None,
            revoked_at: None,
        },
        RosterConnectionStatus::Ended { account_id, .. } => PatientLinkWire {
            practice_id: practice_id.to_owned(),
            patient_workspace_id: workspace_id.to_owned(),
            duet_account_id: Some(account_id.to_string()),
            linked: false,
            sharing_consented: false,
            connection_status,
            consent_version: None,
            consented_at: None,
            revoked_at: None,
        },
        RosterConnectionStatus::NotInvited | RosterConnectionStatus::Invited { .. } => {
            PatientLinkWire {
                practice_id: practice_id.to_owned(),
                patient_workspace_id: workspace_id.to_owned(),
                duet_account_id: None,
                linked: false,
                sharing_consented: false,
                connection_status,
                consent_version: None,
                consented_at: None,
                revoked_at: None,
            }
        }
    }
}

/// Whether `id` is a patient registered to the signed-in practice.
pub(crate) async fn try_on_roster(
    extensions: &axum::http::Extensions,
    id: &str,
) -> Result<bool, axum::http::StatusCode> {
    let Some(care) = extensions.get::<ConnectedCare>() else {
        return Ok(false);
    };
    let store = care.auth_store(extensions);
    let identity = ConnectedCare::identity(extensions);
    let owner = store
        .patient_practice(id)
        .await
        .map_err(|_| axum::http::StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(owner.as_deref() == Some(identity.practice_id.as_str()))
}

pub(crate) async fn on_roster(extensions: &axum::http::Extensions, id: &str) -> bool {
    try_on_roster(extensions, id).await.unwrap_or(false)
}

/// Adds `connection_status` and `patient_link` to a patient page.
pub(crate) async fn enrich_workspace(
    mut value: Value,
    workspace_id: &str,
    extensions: &axum::http::Extensions,
) -> Value {
    if let Some(care) = extensions.get::<ConnectedCare>() {
        let store = care.auth_store(extensions);
        let front_door = care.front_door(extensions);
        let identity = ConnectedCare::identity(extensions);
        let link = care
            .link_wire(&*front_door, &identity.practice_id, workspace_id)
            .await;
        let practice_tz = store
            .practice(&identity.practice_id)
            .await
            .ok()
            .flatten()
            .map(|rec| rec.time_zone);
        if let Some(obj) = value.as_object_mut() {
            obj.insert(
                "connection_status".to_owned(),
                serde_json::to_value(link.connection_status).unwrap_or(Value::Null),
            );
            obj.insert(
                "patient_link".to_owned(),
                serde_json::to_value(&link).unwrap_or(Value::Null),
            );
            if let Some(zone) = practice_tz {
                obj.insert(
                    "time_zone".to_owned(),
                    Value::String(zone.as_str().to_owned()),
                );
            }
        }
    }
    value
}
