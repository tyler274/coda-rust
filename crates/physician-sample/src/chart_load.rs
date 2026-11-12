//! Loads the synthetic charts into the synthetic practice.
//!
//! The adapter says who the source holds and hands over each chart; each chart
//! lands whole in the source store with its durable patient link and pull status
//! before its patient joins the practice, so a registered patient always has a
//! chart. A patient who fails is reported and the rest still load. Loading again
//! retries them, changes only the records the source changed and fills only blank
//! panel places, so a panel choice stays.

use std::path::Path;

use local_product::physician_sources::{PatientSource, workspace_id};
use local_product::runtime::Store;
use local_product::source_status::{
    LinkTarget, SourceCoordinator, SourcePatientKey, SourceStanding,
};
use physician_auth::AuthStore;

use crate::auth::{STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK, now, staging_panel};
use crate::synthetic_chart::SyntheticCharts;

/// What one load did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadSummary {
    /// Patients who finished loading.
    pub patients: usize,
    /// Records those patients' charts delivered.
    pub records: usize,
    /// Records that changed in the store, including a chart saved for a patient
    /// who then failed.
    pub changed: usize,
    /// Each patient who did not finish loading, with the step that failed.
    pub failed: Vec<String>,
}

/// Creates the synthetic practice on its clock and loads the charts built at
/// `bank` into it.
pub async fn load_synthetic_practice(
    bank: &Path,
    auth: &dyn AuthStore,
    runtime: &Store,
) -> Result<LoadSummary, String> {
    let source = SyntheticCharts::open(bank);
    load_practice_from_source(STAGING_PRACTICE_ID, &source, auth, runtime).await
}

/// Loads all patients exposed by `source` into `practice_id`, recording durable
/// source-patient links and pull status in `runtime`.
pub async fn load_practice_from_source(
    practice_id: &str,
    source: &dyn PatientSource,
    auth: &dyn AuthStore,
    runtime: &Store,
) -> Result<LoadSummary, String> {
    if practice_id == STAGING_PRACTICE_ID {
        auth.ensure_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
            .await
            .map_err(|_| "create the synthetic practice".to_owned())?;
    }
    let panel = if practice_id == STAGING_PRACTICE_ID {
        staging_panel(auth)
            .await
            .map_err(|_| "read the staging panel".to_owned())?
    } else {
        Vec::new()
    };
    let mut summary = LoadSummary {
        patients: 0,
        records: 0,
        changed: 0,
        failed: Vec::new(),
    };
    let clock = auth
        .practice_clock(practice_id)
        .await
        .map_err(|_| "read the practice's clock".to_owned())?;
    for id in source.patient_ids().await? {
        let default_workspace = workspace_id(source.source_id(), &id);
        let key = SourcePatientKey {
            practice_id: practice_id.to_owned(),
            source_id: source.source_id().to_owned(),
            source_patient_id: id.clone(),
        };
        let existing_link = runtime
            .source_patient_link(&key)
            .await
            .map_err(|error| error.to_string())?;
        let workspace = existing_link
            .as_ref()
            .map(|link| link.workspace_id.clone())
            .unwrap_or(default_workspace);

        let loaded: Result<(), String> = async {
            let owner = auth
                .patient_practice(&workspace)
                .await
                .map_err(|_| "read the patient's practice".to_owned())?;
            if owner.as_deref().is_some_and(|owner| owner != practice_id) {
                return Err("belongs to another practice".to_owned());
            }
            let pulled_at = clock.now(now()) as i64;
            let report = SourceCoordinator::onboard_or_check_patient(
                runtime,
                source,
                practice_id,
                &id,
                LinkTarget::ExistingOrDefault(&workspace),
                pulled_at,
            )
            .await
            .map_err(|error| error.to_string())?;
            summary.changed += report.changed;
            match &report.standing {
                SourceStanding::UpToDate { .. } | SourceStanding::Partial { .. } => {}
                SourceStanding::Failed { reason, .. }
                | SourceStanding::ReconnectNeeded { reason, .. } => {
                    return Err(reason.clone());
                }
                SourceStanding::NotYetRead => {
                    return Err("source check produced no records".to_owned());
                }
            }
            auth.register_patient(practice_id, &report.workspace_id)
                .await
                .map_err(|_| "register the patient".to_owned())?;
            for identity in &panel {
                auth.place_on_panel(identity, &report.workspace_id)
                    .await
                    .map_err(|_| "place the patient on the panel".to_owned())?;
            }
            summary.records += report.records_delivered;
            summary.patients += 1;
            Ok(())
        }
        .await;
        if let Err(error) = loaded {
            summary.failed.push(format!("{workspace}: {error}"));
        }
    }
    Ok(summary)
}
