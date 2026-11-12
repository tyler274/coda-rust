//! A practice's patients as a physician reads them.
//!
//! The auth store says who belongs to a practice and whose panel they are on.
//! Each source's last pull says who the patient is and how current their
//! records are, while [`SourceStanding`] reflects whether a later pull failed,
//! partially degraded, or lost source access. Search, the patient list and the
//! patient page all read from here, so a patient exists on every surface or on
//! none: a registered patient no source holds reads as not found.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};

use async_trait::async_trait;
use local_product::physician_sources::{MedicalRecordNumber, SourcePatient};
use local_product::runtime::Store;
use local_product::runtime::sources::SourcePull;
use local_product::source_status::{SourceStanding, SourceStatusError};
use physician_auth::{AuthStore, Membership, MemoryAuthStore};
use serde_json::{Value, json};

use crate::auth::{STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK, staging_panel};
use crate::sources::SourceRegistry;
use crate::timestamp::format_rfc3339;

/// The zone the practice's times are worded in.
pub(crate) const PRACTICE_TIME_ZONE: &str = "America/New_York";

/// The most patients one search answers with.
const SEARCH_LIMIT: usize = 50;

/// Failure reading patient pulls or source standings from [`PatientDirectory`].
#[derive(Debug, thiserror::Error)]
pub enum DirectoryError {
    /// Reading patient source pulls failed.
    #[error("failed to read patient source pulls")]
    Pulls(#[source] sea_orm::DbErr),
    /// Reading workspace source standings failed.
    #[error("failed to read workspace source standings")]
    Standings(#[from] SourceStatusError),
}

/// Reads each source's last pull and effective source standing for a set of patients.
#[async_trait]
pub trait PatientDirectory: Send + Sync {
    /// The pulls of `workspaces`, keyed by workspace, each ordered by source.
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError>;

    /// Effective source standings for `workspaces`, keyed by workspace then `source_id`.
    async fn standings(
        &self,
        _workspaces: &[String],
    ) -> Result<BTreeMap<String, BTreeMap<String, SourceStanding>>, DirectoryError> {
        Ok(BTreeMap::new())
    }
}

#[async_trait]
impl PatientDirectory for Store {
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError> {
        self.source_pulls(workspaces)
            .await
            .map_err(DirectoryError::Pulls)
    }

    async fn standings(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, BTreeMap<String, SourceStanding>>, DirectoryError> {
        Ok(self.workspace_source_standings(workspaces).await?)
    }
}

/// The directory of a site with no database, and of tests.
#[derive(Default)]
pub struct MemoryPatients {
    pulls: Mutex<BTreeMap<String, Vec<SourcePull>>>,
}

impl MemoryPatients {
    /// Records one source's pull for `workspace`.
    pub fn add(&self, workspace: &str, pull: SourcePull) {
        self.pulls
            .lock()
            .expect("patient pulls")
            .entry(workspace.to_owned())
            .or_default()
            .push(pull);
    }

    /// The five patients the tests read, held in memory.
    #[doc(hidden)]
    pub fn fixture() -> Arc<Self> {
        let patients = Self::default();
        for workspace in crate::WORKSPACE_IDS {
            patients.add(workspace, fixture_pull(workspace));
        }
        Arc::new(patients)
    }

    fn workspaces(&self) -> Vec<String> {
        self.pulls
            .lock()
            .expect("patient pulls")
            .keys()
            .cloned()
            .collect()
    }
}

/// A fixture patient as one source's pull.
pub(crate) fn fixture_pull(workspace: &str) -> SourcePull {
    let body = workspace_body(workspace).expect("fixture patient");
    let body: Value = serde_json::from_str(body).expect("fixture patient");
    let identity = &body["identity"];
    let text = |value: &Value| value.as_str().map(str::to_owned);
    SourcePull {
        source_id: "fixture-chart".to_owned(),
        source_label: "Fixture chart".to_owned(),
        patient: SourcePatient {
            source_patient_id: workspace.to_owned(),
            display_name: text(&body["display_name"]).expect("fixture name"),
            birth_date: text(&identity["birth_date"]),
            sex_or_gender: text(&identity["sex_or_gender"]["value"]),
            medical_record_numbers: identity["medical_record_numbers"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|number| {
                    Some(MedicalRecordNumber {
                        system: text(&number["system"])?,
                        value: text(&number["value"])?,
                    })
                })
                .collect(),
        },
        pulled_at: Some(1_787_846_400),
    }
}

#[async_trait]
impl PatientDirectory for MemoryPatients {
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError> {
        let pulls = self.pulls.lock().expect("patient pulls");
        Ok(workspaces
            .iter()
            .filter_map(|workspace| {
                pulls
                    .get(workspace)
                    .map(|found| (workspace.clone(), found.clone()))
            })
            .collect())
    }
}

/// Creates the synthetic practice in memory, registers `patients` to it and
/// places them on its staging panel, as a load does with a database.
pub(crate) async fn enroll_in_memory(store: &MemoryAuthStore, patients: &MemoryPatients) {
    store.seed_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK);
    let panel = staging_panel(store)
        .await
        .expect("the memory store always reads");
    for workspace in patients.workspaces() {
        store.seed_patient(STAGING_PRACTICE_ID, &workspace);
        for identity in &panel {
            store
                .place_on_panel(identity, &workspace)
                .await
                .expect("the memory store always writes");
        }
    }
}

/// Registers the fixture patients to the synthetic practice in Postgres, with
/// a pull for each and the staging panel on each, for tests that start a real
/// server.
#[doc(hidden)]
pub async fn enroll_fixture(auth: &dyn AuthStore, runtime: &Store) -> Result<(), String> {
    auth.ensure_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
        .map_err(|_| "create the synthetic practice".to_owned())?;
    let panel = staging_panel(auth)
        .await
        .map_err(|_| "read the staging panel".to_owned())?;
    for workspace in crate::WORKSPACE_IDS {
        runtime
            .record_chart(workspace, &[], &fixture_pull(workspace))
            .await
            .map_err(|error| format!("{workspace}: {error}"))?;
        auth.register_patient(STAGING_PRACTICE_ID, workspace)
            .await
            .map_err(|_| format!("{workspace}: register the patient"))?;
        for identity in &panel {
            auth.set_membership(identity, workspace, Membership::Allowed)
                .await
                .map_err(|_| format!("{workspace}: place the patient on the panel"))?;
        }
    }
    Ok(())
}

/// One patient: their workspace and what the sources say.
#[derive(Clone, Debug)]
pub(crate) struct Patient {
    workspace: String,
    pulls: Vec<SourcePull>,
    standings: BTreeMap<String, SourceStanding>,
}

fn unix_to_rfc3339(seconds: i64) -> String {
    format_rfc3339(UNIX_EPOCH + Duration::from_secs(seconds.unsigned_abs()))
}

impl Patient {
    fn from_entry(
        (workspace, pulls): (String, Vec<SourcePull>),
        standings: BTreeMap<String, SourceStanding>,
    ) -> Option<Self> {
        (!pulls.is_empty()).then_some(Self {
            workspace,
            pulls,
            standings,
        })
    }

    fn holder(&self) -> &SourcePatient {
        &self.pulls[0].patient
    }

    fn sort_key(&self) -> (String, &str) {
        (
            self.holder().display_name.to_lowercase(),
            self.workspace.as_str(),
        )
    }

    /// A row of the physician's patient list.
    pub(crate) fn list_row(&self) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "display_name": self.holder().display_name,
        })
    }

    /// A row of roster search.
    pub(crate) fn roster_row(&self) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "patient_display_name": self.holder().display_name,
            "birth_date": self.holder().birth_date,
        })
    }

    fn matches(&self, words: &[String]) -> bool {
        let birth = self.holder().birth_date.as_deref().unwrap_or_default();
        let haystack = format!("{} {birth}", self.holder().display_name).to_lowercase();
        words.iter().all(|word| haystack.contains(word))
    }

    fn identity(&self, now: &str) -> Value {
        let patient = self.holder();
        let today = now.get(..10).unwrap_or_default();
        let sex_or_gender = patient
            .sex_or_gender
            .as_ref()
            .map(|value| json!({ "label": "Administrative gender", "value": value }));
        let numbers: Vec<Value> = patient
            .medical_record_numbers
            .iter()
            .map(|number| json!({ "system": number.system, "value": number.value }))
            .collect();
        json!({
            "state": "available",
            "display_name": patient.display_name,
            "birth_date": patient.birth_date,
            "age_years": patient.birth_date.as_deref().and_then(|birth| age_on(birth, today)),
            "age_as_of": today,
            "sex_or_gender": sex_or_gender,
            "medical_record_numbers": numbers,
        })
    }

    fn connected_sources(&self, registry: Option<&SourceRegistry>) -> Vec<Value> {
        let pulls_by_source: BTreeMap<&str, &SourcePull> = self
            .pulls
            .iter()
            .map(|pull| (pull.source_id.as_str(), pull))
            .collect();
        let source_ids: BTreeSet<&str> = pulls_by_source
            .keys()
            .copied()
            .chain(self.standings.keys().map(String::as_str))
            .collect();

        let mut sources = Vec::with_capacity(source_ids.len());
        for source_id in source_ids {
            let pull = pulls_by_source.get(source_id).copied();
            let standing = self.standings.get(source_id);
            let source_label = match pull {
                Some(pull) => pull.source_label.clone(),
                None => registry
                    .and_then(|reg| reg.adapter(source_id))
                    .map(|adapter| adapter.source_label().to_owned())
                    .unwrap_or_else(|| source_id.to_owned()),
            };

            let entry = match standing {
                None | Some(SourceStanding::NotYetRead) => {
                    let Some(pull) = pull else {
                        continue;
                    };
                    match pull.pulled_at {
                        Some(pulled_at) => {
                            let synchronized_at = unix_to_rfc3339(pulled_at);
                            json!({
                                "source_id": source_id,
                                "source_label": source_label,
                                "standing": "up_to_date",
                                "reconnect_needed": false,
                                "last_good_read_at": synchronized_at,
                                "state": {
                                    "state": "usable",
                                    "synchronized_at": synchronized_at,
                                },
                            })
                        }
                        None => json!({
                            "source_id": source_id,
                            "source_label": source_label,
                            "standing": "partial",
                            "reconnect_needed": false,
                            "last_good_read_at": null,
                            "state": {
                                "state": "failed",
                                "synchronized_at": null,
                                "usable_snapshot": null,
                            },
                        }),
                    }
                }
                Some(SourceStanding::UpToDate {
                    last_good_read_at,
                    missing_candidates,
                }) => {
                    let synchronized_at = unix_to_rfc3339(*last_good_read_at);
                    json!({
                        "source_id": source_id,
                        "source_label": source_label,
                        "standing": "up_to_date",
                        "reconnect_needed": false,
                        "last_good_read_at": synchronized_at,
                        "missing_candidates": missing_candidates,
                        "state": {
                            "state": "usable",
                            "synchronized_at": synchronized_at,
                        },
                    })
                }
                Some(SourceStanding::Partial {
                    last_good_read_at,
                    attempted_at,
                    failed_parts,
                    missing_candidates,
                }) => {
                    let good = last_good_read_at.map(unix_to_rfc3339);
                    let attempted = unix_to_rfc3339(*attempted_at);
                    let usable_snapshot = good.as_ref().map(|ts| json!({ "synchronized_at": ts }));
                    json!({
                        "source_id": source_id,
                        "source_label": source_label,
                        "standing": "partial",
                        "reconnect_needed": false,
                        "last_good_read_at": good,
                        "attempted_at": attempted,
                        "failed_parts": failed_parts,
                        "missing_candidates": missing_candidates,
                        "state": {
                            "state": "failed",
                            "synchronized_at": attempted,
                            "usable_snapshot": usable_snapshot,
                        },
                    })
                }
                Some(SourceStanding::Failed {
                    last_good_read_at,
                    attempted_at,
                    reason,
                }) => {
                    let good = last_good_read_at.map(unix_to_rfc3339);
                    let attempted = unix_to_rfc3339(*attempted_at);
                    let usable_snapshot = good.as_ref().map(|ts| json!({ "synchronized_at": ts }));
                    json!({
                        "source_id": source_id,
                        "source_label": source_label,
                        "standing": "failed",
                        "reconnect_needed": false,
                        "reason": reason,
                        "last_good_read_at": good,
                        "attempted_at": attempted,
                        "state": {
                            "state": "failed",
                            "synchronized_at": attempted,
                            "usable_snapshot": usable_snapshot,
                        },
                    })
                }
                Some(SourceStanding::ReconnectNeeded {
                    last_good_read_at,
                    attempted_at,
                    reason,
                }) => {
                    let good = last_good_read_at.map(unix_to_rfc3339);
                    let attempted = attempted_at.map(unix_to_rfc3339);
                    let usable_snapshot = good.as_ref().map(|ts| json!({ "synchronized_at": ts }));
                    json!({
                        "source_id": source_id,
                        "source_label": source_label,
                        "standing": "reconnect_needed",
                        "reconnect_needed": true,
                        "reason": reason,
                        "last_good_read_at": good,
                        "attempted_at": attempted,
                        "state": {
                            "state": "failed",
                            "synchronized_at": attempted,
                            "usable_snapshot": usable_snapshot,
                        },
                    })
                }
            };
            sources.push(entry);
        }
        sources
    }

    /// The patient page: identity and how current the records are.
    pub(crate) fn workspace_body(&self, now: &str, registry: Option<&SourceRegistry>) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "display_name": self.holder().display_name,
            "time_zone": PRACTICE_TIME_ZONE,
            "current_time": now,
            "identity": self.identity(now),
            "connected_sources": self.connected_sources(registry),
            "chats": [],
            "active_follow_ups": [],
            "has_past_work": false,
        })
    }

    /// What a physician in the clinic reads of a patient not on their panel.
    pub(crate) fn clinic_demographics(&self, now: &str) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "display_name": self.holder().display_name,
            "membership": "clinic_demographic",
            "time_zone": PRACTICE_TIME_ZONE,
            "current_time": now,
            "identity": self.identity(now),
            "chats": [],
            "patient_synopsis": null,
            "automation_run": null,
            "active_follow_ups": [],
            "has_past_work": false,
            "connected_sources": [],
        })
    }
}

/// Whole years between two `YYYY-MM-DD` dates.
fn age_on(birth: &str, today: &str) -> Option<i64> {
    let parts = |date: &str| -> Option<(i64, i64, i64)> {
        let mut pieces = date.split('-');
        Some((
            pieces.next()?.parse().ok()?,
            pieces.next()?.parse().ok()?,
            pieces.next()?.parse().ok()?,
        ))
    };
    let (born_year, born_month, born_day) = parts(birth)?;
    let (year, month, day) = parts(today)?;
    let before_birthday = (month, day) < (born_month, born_day);
    Some(year - born_year - i64::from(before_birthday))
}

/// The patients of `workspaces` any source holds, by name.
pub(crate) async fn read_patients(
    directory: &dyn PatientDirectory,
    workspaces: &[String],
) -> Result<Vec<Patient>, DirectoryError> {
    let (pulls, mut standings) =
        tokio::try_join!(directory.pulls(workspaces), directory.standings(workspaces))?;
    let mut patients: Vec<Patient> = pulls
        .into_iter()
        .filter_map(|(workspace, ws_pulls)| {
            let ws_standings = standings.remove(&workspace).unwrap_or_default();
            Patient::from_entry((workspace, ws_pulls), ws_standings)
        })
        .collect();
    patients.sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
    Ok(patients)
}

/// One patient any source holds.
pub(crate) async fn read_patient(
    directory: &dyn PatientDirectory,
    workspace: &str,
) -> Result<Option<Patient>, DirectoryError> {
    Ok(read_patients(directory, &[workspace.to_owned()])
        .await?
        .pop())
}

/// The patients of `workspaces` whose name and birth date hold every word of
/// `query`, first by name. A blank query finds no one.
pub(crate) async fn search(
    directory: &dyn PatientDirectory,
    workspaces: &[String],
    query: &str,
) -> Result<Vec<Patient>, DirectoryError> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    if words.is_empty() {
        return Ok(Vec::new());
    }
    let mut found = read_patients(directory, workspaces).await?;
    found.retain(|patient| patient.matches(&words));
    found.truncate(SEARCH_LIMIT);
    Ok(found)
}

pub(crate) fn workspace_body(id: &str) -> Option<&'static str> {
    Some(match id {
        "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee" => {
            include_str!("../fixtures/workspaces/pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee.json")
        }
        "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" => {
            include_str!("../fixtures/workspaces/pw-82c44e9b-58c0-4af1-bd21-771bf77e101d.json")
        }
        "pw-f5404d89-acde-4bb5-8c42-2fa191766811" => {
            include_str!("../fixtures/workspaces/pw-f5404d89-acde-4bb5-8c42-2fa191766811.json")
        }
        "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971" => {
            include_str!("../fixtures/workspaces/pw-d11a35f7-0d8d-466f-a48e-5812df0a4971.json")
        }
        "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e" => {
            include_str!("../fixtures/workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json")
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::*;

    struct FailingStandingsDirectory;

    #[async_trait]
    impl PatientDirectory for FailingStandingsDirectory {
        async fn pulls(
            &self,
            _workspaces: &[String],
        ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError> {
            Ok(BTreeMap::new())
        }

        async fn standings(
            &self,
            _workspaces: &[String],
        ) -> Result<BTreeMap<String, BTreeMap<String, SourceStanding>>, DirectoryError> {
            Err(DirectoryError::from(SourceStatusError::EmptyIdentity {
                field: "workspace_id",
            }))
        }
    }

    #[test]
    fn age_turns_on_the_birthday() {
        assert_eq!(age_on("1972-08-27", "2026-08-26"), Some(53));
        assert_eq!(age_on("1972-08-27", "2026-08-27"), Some(54));
        assert_eq!(age_on("unknown", "2026-08-27"), None);
    }

    #[tokio::test]
    async fn directory_error_preserves_source_status_cause() {
        let err = read_patient(&FailingStandingsDirectory, "pw-test")
            .await
            .expect_err("standings failure must surface");
        let source = err
            .source()
            .expect("DirectoryError::Standings preserves SourceStatusError cause");
        let typed = source
            .downcast_ref::<SourceStatusError>()
            .expect("cause downcasts to SourceStatusError");
        assert!(matches!(
            typed,
            SourceStatusError::EmptyIdentity {
                field: "workspace_id"
            }
        ));
    }

    #[test]
    fn connected_sources_projects_pull_less_linked_standings_and_skips_not_yet_read() {
        let mut standings = BTreeMap::new();
        standings.insert(
            "fixture-chart".to_owned(),
            SourceStanding::UpToDate {
                last_good_read_at: 1_787_846_400,
                missing_candidates: Vec::new(),
            },
        );
        standings.insert(
            "regional-labs".to_owned(),
            SourceStanding::Failed {
                last_good_read_at: None,
                attempted_at: 1_787_846_700,
                reason: "upstream timeout".to_owned(),
            },
        );
        standings.insert("never-attempted".to_owned(), SourceStanding::NotYetRead);

        let ws = crate::WORKSPACE_IDS[0];
        let patient =
            Patient::from_entry((ws.to_owned(), vec![fixture_pull(ws)]), standings).unwrap();
        let connected = patient.connected_sources(None);
        assert_eq!(connected.len(), 2);
        assert_eq!(connected[0]["source_id"], "fixture-chart");
        assert_eq!(connected[0]["standing"], "up_to_date");
        assert_eq!(connected[1]["source_id"], "regional-labs");
        assert_eq!(connected[1]["source_label"], "regional-labs");
        assert_eq!(connected[1]["standing"], "failed");
        assert_eq!(connected[1]["reason"], "upstream timeout");
        assert_eq!(connected[1]["last_good_read_at"], Value::Null);
    }
}
