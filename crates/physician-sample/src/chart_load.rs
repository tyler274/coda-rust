//! Loads the synthetic charts into the synthetic practice.
//!
//! The adapter says who the source holds and hands over each chart; each chart
//! lands whole in the source store with its durable patient link and pull status
//! before its patient joins the practice, so a registered patient always has a
//! chart. A patient who fails is reported and the rest still load. Loading again
//! retries them, changes only the records the source changed and fills only blank
//! panel places, so a panel choice stays.

use std::fmt;
use std::path::Path;

use local_product::physician_sources::{PatientSource, workspace_id};
use local_product::runtime::Store;
use local_product::source_status::{
    LinkTarget, SourceCoordinator, SourcePatientKey, SourceStanding,
};
use physician_auth::{AuthStore, StoreUnavailable};

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

/// Why enrolling newly identified source patients into the practice directory failed.
#[derive(Debug)]
pub(crate) enum EnrollmentError {
    /// Reading retained source pulls from the runtime store failed.
    ReadSourcePulls {
        /// Underlying database error from `runtime.source_pulls`.
        source: sea_orm::DbErr,
    },
    /// Reading the staging panel accounts from the auth store failed.
    ReadStagingPanel(StoreUnavailable),
    /// Registering a patient workspace into the practice failed.
    RegisterPatient(StoreUnavailable),
    /// Placing a registered patient onto a physician's panel failed.
    PlaceOnPanel(StoreUnavailable),
}

impl fmt::Display for EnrollmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadSourcePulls { source } => write!(f, "read source pulls: {source}"),
            Self::ReadStagingPanel(_) => f.write_str("read the staging panel"),
            Self::RegisterPatient(_) => f.write_str("register the patient"),
            Self::PlaceOnPanel(_) => f.write_str("place the patient on the panel"),
        }
    }
}

impl std::error::Error for EnrollmentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadSourcePulls { source } => Some(source),
            Self::ReadStagingPanel(_) | Self::RegisterPatient(_) | Self::PlaceOnPanel(_) => None,
        }
    }
}

/// Registers any `workspaces` that hold patient directory identity in `runtime` into
/// `practice_id` (and fills blank panel places on the synthetic practice), withholding
/// workspaces whose source pulls have not yet delivered a [`local_product::physician_sources::SourcePatient`].
pub(crate) async fn enroll_identified_patients(
    practice_id: &str,
    auth: &dyn AuthStore,
    runtime: &Store,
    workspaces: &[String],
) -> Result<(), EnrollmentError> {
    if workspaces.is_empty() {
        return Ok(());
    }
    let pulls = runtime
        .source_pulls(workspaces)
        .await
        .map_err(|source| EnrollmentError::ReadSourcePulls { source })?;
    let panel = if practice_id == STAGING_PRACTICE_ID {
        staging_panel(auth)
            .await
            .map_err(EnrollmentError::ReadStagingPanel)?
    } else {
        Vec::new()
    };
    for workspace in workspaces {
        if pulls.get(workspace).is_none_or(Vec::is_empty) {
            continue;
        }
        auth.register_patient(practice_id, workspace)
            .await
            .map_err(EnrollmentError::RegisterPatient)?;
        for identity in &panel {
            auth.place_on_panel(identity, workspace)
                .await
                .map_err(EnrollmentError::PlaceOnPanel)?;
        }
    }
    Ok(())
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
    for id in source
        .patient_ids()
        .await
        .map_err(|error| error.to_string())?
    {
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
                SourceStanding::UpToDate { .. } => {}
                SourceStanding::Partial { .. } => {
                    let pulls = runtime
                        .source_pulls(std::slice::from_ref(&report.workspace_id))
                        .await
                        .map_err(|error| error.to_string())?;
                    if pulls.get(&report.workspace_id).is_none_or(Vec::is_empty) {
                        return Err("partial pull missing patient identity".to_owned());
                    }
                }
                SourceStanding::Failed { reason, .. }
                | SourceStanding::ReconnectNeeded { reason, .. } => {
                    return Err(reason.clone());
                }
                SourceStanding::NotYetRead => {
                    return Err("source check produced no records".to_owned());
                }
            }
            enroll_identified_patients(
                practice_id,
                auth,
                runtime,
                std::slice::from_ref(&report.workspace_id),
            )
            .await
            .map_err(|error| error.to_string())?;
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

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::*;

    #[test]
    fn enrollment_error_preserves_sea_orm_source_and_describes_auth_store_failures() {
        let db_err = EnrollmentError::ReadSourcePulls {
            source: sea_orm::DbErr::Custom("runtime pool closed".to_owned()),
        };
        assert!(
            db_err.source().is_some(),
            "ReadSourcePulls must retain sea_orm::DbErr as its Error::source()"
        );
        assert!(db_err.to_string().contains("runtime pool closed"));

        for (variant, expected) in [
            (
                EnrollmentError::ReadStagingPanel(StoreUnavailable),
                "read the staging panel",
            ),
            (
                EnrollmentError::RegisterPatient(StoreUnavailable),
                "register the patient",
            ),
            (
                EnrollmentError::PlaceOnPanel(StoreUnavailable),
                "place the patient on the panel",
            ),
        ] {
            assert_eq!(variant.to_string(), expected);
            assert!(variant.source().is_none());
        }
    }
}
