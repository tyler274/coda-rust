//! Physician-side Connected Care state: the patient link a patient page shows.
//! The patient product owns invitation acceptance and consent.

use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use serde_json::Value;

use crate::timestamp::format_rfc3339;
use physician_auth::{
    AuthStore, Membership, MemoryAuthStore, PatientLink, PhysicianIdentity, StoreUnavailable,
};
use physician_wire::PatientLinkWire;

use crate::auth::{AuthContext, STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK, now};
use crate::patients::{self, DirectoryError, MemoryPatients, Patient, PatientDirectory};

#[derive(Debug, thiserror::Error)]
pub(crate) enum PatientLookupError {
    #[error("physician auth store unavailable")]
    Auth,
    #[error(transparent)]
    Directory(#[from] DirectoryError),
}

/// Shared Connected Care state attached to every request on a physician router.
#[derive(Clone)]
pub(crate) struct ConnectedCare {
    fallback_auth: Arc<dyn AuthStore>,
    fallback_patients: Arc<dyn PatientDirectory>,
}

impl ConnectedCare {
    /// Creates a new Connected Care state with its own fallback memory stores,
    /// holding no patients.
    pub(crate) fn new() -> Self {
        Self::on(
            Arc::new(Self::memory_store()),
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
        Self::on(Arc::new(store), patients)
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

    fn on(fallback_auth: Arc<dyn AuthStore>, fallback_patients: Arc<dyn PatientDirectory>) -> Self {
        Self {
            fallback_auth,
            fallback_patients,
        }
    }

    pub(crate) fn auth_store(&self, extensions: &axum::http::Extensions) -> Arc<dyn AuthStore> {
        extensions
            .get::<Arc<dyn AuthStore>>()
            .cloned()
            .unwrap_or_else(|| Arc::clone(&self.fallback_auth))
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

    /// The signed-in practice's clinical "now", as the wire carries it.
    pub(crate) async fn practice_now(
        &self,
        extensions: &axum::http::Extensions,
    ) -> Result<String, StoreUnavailable> {
        let store = self.auth_store(extensions);
        let identity = Self::identity(extensions);
        let clock = store.practice_clock(&identity.practice_id).await?;
        Ok(format_rfc3339(
            UNIX_EPOCH + Duration::from_secs(clock.now(now())),
        ))
    }

    pub(crate) async fn link_wire(
        &self,
        store: &dyn AuthStore,
        practice_id: &str,
        workspace_id: &str,
    ) -> PatientLinkWire {
        match store.patient_link(practice_id, workspace_id).await {
            Some(link) => to_link_wire(link),
            None => PatientLinkWire {
                practice_id: practice_id.to_owned(),
                patient_workspace_id: workspace_id.to_owned(),
                duet_account_id: None,
                linked: false,
                sharing_consented: false,
                consent_version: None,
                consented_at: None,
                revoked_at: None,
            },
        }
    }
}

/// Whether `id` is a patient registered to the signed-in practice.
pub(crate) async fn on_roster(extensions: &axum::http::Extensions, id: &str) -> bool {
    let Some(care) = extensions.get::<ConnectedCare>() else {
        return false;
    };
    let store = care.auth_store(extensions);
    let identity = ConnectedCare::identity(extensions);
    matches!(
        store.patient_practice(id).await,
        Ok(Some(owner)) if owner == identity.practice_id
    )
}

/// Adds `patient_link` to a patient page.
pub(crate) async fn enrich_workspace(
    mut value: Value,
    workspace_id: &str,
    extensions: &axum::http::Extensions,
) -> Value {
    if let Some(care) = extensions.get::<ConnectedCare>() {
        let store = care.auth_store(extensions);
        let identity = ConnectedCare::identity(extensions);
        let link = care
            .link_wire(&*store, &identity.practice_id, workspace_id)
            .await;
        if let Some(obj) = value.as_object_mut() {
            obj.insert(
                "patient_link".to_owned(),
                serde_json::to_value(&link).unwrap_or(Value::Null),
            );
        }
    }
    value
}

fn to_link_wire(link: PatientLink) -> PatientLinkWire {
    PatientLinkWire {
        practice_id: link.practice_id,
        patient_workspace_id: link.patient_workspace_id,
        duet_account_id: link.duet_account_id,
        linked: link.linked,
        sharing_consented: link.sharing_consented,
        consent_version: link.consent_version,
        consented_at: link.consented_at,
        revoked_at: link.revoked_at,
    }
}
