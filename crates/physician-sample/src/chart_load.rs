//! Loads the synthetic charts into the synthetic practice.
//!
//! The adapter says who the source holds and hands over each chart; each chart
//! lands whole in the source store before its patient joins the practice, so a
//! registered patient always has a chart. A patient who fails is reported and the
//! rest still load. Loading again retries them, changes only the records the
//! source changed and fills only blank panel places, so a panel choice stays.

use std::path::Path;

use local_product::physician_sources::{PatientSource, workspace_id};
use local_product::runtime::Store;
use local_product::runtime::sources::SourcePull;
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
    auth.ensure_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
        .map_err(|_| "create the synthetic practice".to_owned())?;
    let panel = staging_panel(auth)
        .await
        .map_err(|_| "read the staging panel".to_owned())?;
    let source = SyntheticCharts::open(bank);
    let mut summary = LoadSummary {
        patients: 0,
        records: 0,
        changed: 0,
        failed: Vec::new(),
    };
    let clock = auth
        .practice_clock(STAGING_PRACTICE_ID)
        .await
        .map_err(|_| "read the synthetic practice's clock".to_owned())?;
    for id in source
        .patient_ids()
        .await
        .map_err(|error| error.to_string())?
    {
        let workspace = workspace_id(source.source_id(), &id);
        let loaded: Result<(), String> = async {
            let owner = auth
                .patient_practice(&workspace)
                .await
                .map_err(|_| "read the patient's practice".to_owned())?;
            if owner.as_deref().is_some_and(|id| id != STAGING_PRACTICE_ID) {
                return Err("belongs to another practice".to_owned());
            }
            let patient = source
                .patient(&id)
                .await
                .map_err(|error| error.to_string())?;
            let records = source
                .records(&patient, &workspace)
                .await
                .map_err(|error| error.to_string())?;
            let pull = SourcePull {
                source_id: source.source_id().to_owned(),
                source_label: source.source_label().to_owned(),
                patient,
                pulled_at: clock.now(now()) as i64,
            };
            summary.changed += runtime
                .record_chart(&workspace, &records, &pull)
                .await
                .map_err(|error| error.to_string())?;
            auth.register_patient(STAGING_PRACTICE_ID, &workspace)
                .await
                .map_err(|_| "register the patient".to_owned())?;
            for identity in &panel {
                auth.place_on_panel(identity, &workspace)
                    .await
                    .map_err(|_| "place the patient on the panel".to_owned())?;
            }
            summary.records += records.len();
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
