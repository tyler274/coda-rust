//! Physician-side Connected Care state: the patient link a patient page shows
//! and the doctor's plan routes. The patient product owns invitation
//! acceptance and consent.

use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};

use axum::Json;
use axum::body::Body;
use axum::extract::{Path, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::timestamp::format_rfc3339;
use local_product::{ApprovedDoctorPlan, DoctorPlanDraft, DoctorPlanStore, PlanItemInput};
use physician_auth::{
    AuthStore, Cadence, Membership, MemoryAuthStore, PatientLink, PhysicianIdentity,
    StoreUnavailable,
};
use physician_wire::{
    ApprovedDoctorPlanWire, DoctorPlanDraftWire, DoctorPlanItemWire, PatientLinkWire,
    PlanDiffEntryWire, PlanItemWire,
};

use crate::auth::{AuthContext, STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK, now};
use crate::patients::{self, MemoryPatients, Patient, PatientDirectory};

/// Shared Connected Care state attached to every request on a physician router.
#[derive(Clone)]
pub(crate) struct ConnectedCare {
    fallback_auth: Arc<dyn AuthStore>,
    fallback_patients: Arc<dyn PatientDirectory>,
    pub(crate) doctor_plans: Arc<Mutex<DoctorPlanStore>>,
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
            doctor_plans: Arc::new(Mutex::new(DoctorPlanStore::new())),
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
    ) -> Result<Option<Patient>, StoreUnavailable> {
        let practice = Self::identity(extensions).practice_id;
        let store = self.auth_store(extensions);
        if store.patient_practice(id).await?.as_deref() != Some(practice.as_str()) {
            return Ok(None);
        }
        patients::read_patient(&*self.patients(extensions), id).await
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
                cadence: Cadence::Weekly.as_str().to_owned(),
            },
        }
    }

    pub(crate) fn plan_draft_wire(&self, workspace_id: &str) -> Option<DoctorPlanDraftWire> {
        self.doctor_plans
            .lock()
            .expect("doctor plans lock")
            .draft(workspace_id)
            .map(to_plan_draft_wire)
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

/// Reads the current doctor's plan draft and approval state (`ENG-53`).
pub(crate) async fn get_plan(Path(id): Path<String>, request: Request<Body>) -> Response {
    if !on_roster(request.extensions(), &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Some(care) = request.extensions().get::<ConnectedCare>().cloned() else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let identity = ConnectedCare::identity(request.extensions());
    let draft = care.plan_draft_wire(&id).unwrap_or(DoctorPlanDraftWire {
        patient_workspace_id: id,
        practice_id: identity.practice_id,
        physician_name: "Staging Physician".to_owned(),
        approved_version: 0,
        currently_approved: false,
        items: Vec::new(),
        latest_approved: None,
    });
    Json(draft).into_response()
}

#[derive(Deserialize)]
struct SavePlanDraftBody {
    items: Vec<PlanItemWire>,
    #[serde(default)]
    physician_name: Option<String>,
}

/// Saves or edits the doctor's plan draft (`ENG-53`). Any edit invalidates prior approval.
pub(crate) async fn put_plan_draft(Path(id): Path<String>, request: Request<Body>) -> Response {
    if !on_roster(request.extensions(), &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Some(care) = request.extensions().get::<ConnectedCare>().cloned() else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let store = care.auth_store(request.extensions());
    let identity = ConnectedCare::identity(request.extensions());
    let default_physician_name = store
        .account_by_identity(&identity)
        .await
        .map(|account| account.display_name)
        .unwrap_or_else(|| "Staging Physician".to_owned());
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), 256 * 1024).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<SavePlanDraftBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let physician_name = body.physician_name.unwrap_or(default_physician_name);
    let items: Vec<PlanItemInput> = body
        .items
        .into_iter()
        .map(|item| PlanItemInput {
            id: item.id,
            title: item.title,
            body: item.body,
        })
        .collect();
    let result = care
        .doctor_plans
        .lock()
        .expect("doctor plans lock")
        .save_draft(&id, &identity.practice_id, &physician_name, items);
    match result {
        Ok(draft) => Json(to_plan_draft_wire(draft)).into_response(),
        Err(detail) => (StatusCode::BAD_REQUEST, Json(json!({ "detail": detail }))).into_response(),
    }
}

#[derive(Deserialize)]
struct ApprovePlanBody {
    approved_items: Vec<PlanItemWire>,
    #[serde(default)]
    approved_at: Option<String>,
}

/// Approves the exact visible text of the doctor's plan (`ENG-53`).
pub(crate) async fn post_approve_plan(Path(id): Path<String>, request: Request<Body>) -> Response {
    if !on_roster(request.extensions(), &id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Some(care) = request.extensions().get::<ConnectedCare>().cloned() else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), 256 * 1024).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<ApprovePlanBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let approved_at = body
        .approved_at
        .unwrap_or_else(|| "2026-10-13T12:00:00Z".to_owned());
    let approved_items: Vec<PlanItemInput> = body
        .approved_items
        .into_iter()
        .map(|item| PlanItemInput {
            id: item.id,
            title: item.title,
            body: item.body,
        })
        .collect();
    let result = care
        .doctor_plans
        .lock()
        .expect("doctor plans lock")
        .approve_exact_text(&id, &approved_items, &approved_at);
    match result {
        Ok(approved) => Json(to_approved_plan_wire(approved)).into_response(),
        Err(detail) => (StatusCode::BAD_REQUEST, Json(json!({ "detail": detail }))).into_response(),
    }
}

/// Adds `patient_link` and `doctor_plan` to a patient page.
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
        let doctor_plan = care.plan_draft_wire(workspace_id);
        if let Some(obj) = value.as_object_mut() {
            obj.insert(
                "patient_link".to_owned(),
                serde_json::to_value(&link).unwrap_or(Value::Null),
            );
            obj.insert(
                "doctor_plan".to_owned(),
                serde_json::to_value(&doctor_plan).unwrap_or(Value::Null),
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
        cadence: link.cadence.as_str().to_owned(),
    }
}

fn to_approved_plan_wire(plan: ApprovedDoctorPlan) -> ApprovedDoctorPlanWire {
    ApprovedDoctorPlanWire {
        patient_workspace_id: plan.patient_workspace_id,
        practice_id: plan.practice_id,
        physician_name: plan.physician_name,
        label: plan.label,
        version: plan.version,
        approved_at: plan.approved_at,
        items: plan
            .items
            .into_iter()
            .map(|item| PlanItemWire {
                id: item.id,
                title: item.title,
                body: item.body,
            })
            .collect(),
        last_diff: plan
            .last_diff
            .into_iter()
            .map(|diff| PlanDiffEntryWire {
                item_id: diff.item_id,
                title: diff.title,
                previous_body: diff.previous_body,
                approved_body: diff.approved_body,
            })
            .collect(),
    }
}

fn to_plan_draft_wire(draft: DoctorPlanDraft) -> DoctorPlanDraftWire {
    DoctorPlanDraftWire {
        patient_workspace_id: draft.patient_workspace_id,
        practice_id: draft.practice_id,
        physician_name: draft.physician_name,
        approved_version: draft.approved_version,
        currently_approved: draft.currently_approved,
        items: draft
            .items
            .into_iter()
            .map(|item| DoctorPlanItemWire {
                id: item.id,
                title: item.title,
                body: item.body,
                approved_title: item.approved_title,
                approved_body: item.approved_body,
                needs_approval: item.needs_approval,
            })
            .collect(),
        latest_approved: draft.latest_approved.map(to_approved_plan_wire),
    }
}
