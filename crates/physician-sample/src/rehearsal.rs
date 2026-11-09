//! The rehearsal practice: synthetic staging traffic that telemetry labels
//! `simulation`.
//!
//! Real staging accounts stay in the synthetic practice
//! ([`crate::STAGING_PRACTICE_ID`]) and their chats stay `real`. Rehearsal
//! chats run in this separate practice, with its own physician and its own
//! fixture panel on the same fixed clock, so naming the practice in
//! `PHYSICIAN_SIMULATION_PRACTICES` relabels only rehearsal traffic. Its
//! patients are the fixture patients under rehearsal workspace ids: a
//! workspace belongs to one practice, so the two practices never share one.

use local_product::physician_sources::workspace_id;
use local_product::runtime::Store;
use physician_auth::{AuthStore, PostgresAuthStore};

use crate::auth::SYNTHETIC_PRACTICE_CLOCK;

/// The rehearsal practice id.
pub const REHEARSAL_PRACTICE_ID: &str = "coda-rehearsal-practice";

/// The one physician who works in the rehearsal practice.
pub const REHEARSAL_PHYSICIAN_ID: &str = "coda-rehearsal-physician";

/// The rehearsal physician's account email. Nobody signs in with it by link;
/// rehearsal sessions come from the rehearsal sign-in route.
pub const REHEARSAL_EMAIL: &str = "rehearsal@codahealth.test";

/// The source the rehearsal panel's charts come from.
const REHEARSAL_SOURCE: &str = "rehearsal-fixture";

/// The rehearsal panel's workspace ids, one per fixture patient, in fixture
/// order.
pub fn rehearsal_workspaces() -> Vec<String> {
    crate::WORKSPACE_IDS
        .iter()
        .map(|fixture| workspace_id(REHEARSAL_SOURCE, fixture))
        .collect()
}

/// Creates the rehearsal practice on the synthetic clock, its physician
/// account, and its fixture panel. Running it again changes nothing: an
/// existing account is left as an administrator set it, and a patient already
/// on the panel stays.
pub async fn seed_rehearsal_practice(
    auth: &PostgresAuthStore,
    runtime: &Store,
    now: u64,
) -> Result<(), String> {
    auth.ensure_practice(REHEARSAL_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
        .map_err(|_| "create the rehearsal practice".to_owned())?;
    auth.ensure_account(
        REHEARSAL_PRACTICE_ID,
        REHEARSAL_PHYSICIAN_ID,
        REHEARSAL_EMAIL,
        "Rehearsal Physician",
        false,
        now,
    )
    .await
    .map_err(|error| format!("rehearsal account: {error}"))?;
    let physician = physician_auth::PhysicianIdentity {
        practice_id: REHEARSAL_PRACTICE_ID.to_owned(),
        physician_id: REHEARSAL_PHYSICIAN_ID.to_owned(),
    };
    for (fixture, workspace) in crate::WORKSPACE_IDS.iter().zip(rehearsal_workspaces()) {
        let owner = auth
            .patient_practice(&workspace)
            .await
            .map_err(|_| format!("{workspace}: read the patient's practice"))?;
        if owner
            .as_deref()
            .is_some_and(|id| id != REHEARSAL_PRACTICE_ID)
        {
            return Err(format!("{workspace}: belongs to another practice"));
        }
        let mut pull = crate::patients::fixture_pull(fixture);
        pull.source_id = REHEARSAL_SOURCE.to_owned();
        pull.source_label = "Rehearsal fixture".to_owned();
        runtime
            .record_chart(&workspace, &[], &pull)
            .await
            .map_err(|error| format!("{workspace}: {error}"))?;
        auth.register_patient(REHEARSAL_PRACTICE_ID, &workspace)
            .await
            .map_err(|_| format!("{workspace}: register the patient"))?;
        auth.place_on_panel(&physician, &workspace)
            .await
            .map_err(|_| format!("{workspace}: place the patient on the panel"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rehearsal_workspaces_never_collide_with_the_staging_panel() {
        let rehearsal = rehearsal_workspaces();
        assert_eq!(rehearsal.len(), crate::WORKSPACE_IDS.len());
        for workspace in &rehearsal {
            assert!(workspace.starts_with("pw-"));
            assert!(!crate::WORKSPACE_IDS.contains(&workspace.as_str()));
        }
    }
}
