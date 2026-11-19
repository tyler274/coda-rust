//! Acceptance proof for practice-scoped source patient links, pull status
//! preservation, and practice-wide outage summarization against real Postgres.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use local_product::physician_sources::{
    FailedPart, MedicalRecordNumber, PatientSource, PullOutcome, ReconnectReceipt, RecordDate,
    SourceAdapterError, SourceCommandKey, SourcePatient, SourcePullOutput, SourceRecordKind,
    SourceRevision, workspace_id,
};
use local_product::runtime::Store;
use local_product::sha256_hex;
use local_product::source_status::{
    CompleteDelivery, LinkTarget, PartialDelivery, PatientSourceRecordStatus, PracticeOutage,
    PracticeOutageKind, PracticeSourceSummary, PullAttempt, PullDisposition, ReconnectCommand,
    ReconnectCommandFence, ReconnectCommandOutcome, ReconnectLeasePolicy, SourceAccessState,
    SourceCoordinator, SourcePatientKey, SourceStanding, SourceStatusError, derive_standing,
    summarize_practice_source,
};
use serde_json::json;
use uuid::Uuid;

fn test_database_url() -> String {
    std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("PHYSICIAN_TEST_DATABASE_URL"))
        .expect("set PHYSICIAN_RUNTIME_TEST_DATABASE_URL or PHYSICIAN_TEST_DATABASE_URL")
}

async fn migrated_store() -> Store {
    let store = Store::connect(&test_database_url()).await.unwrap();
    store.migrate().await.unwrap();
    store
}

fn make_patient(source_patient_id: &str, name: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: source_patient_id.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some("1978-05-14".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "urn:clinic:mrn".to_owned(),
            value: source_patient_id.to_owned(),
        }],
    }
}

fn make_revision(
    workspace: &str,
    source_id: &str,
    record_type: &str,
    id: &str,
    body_text: &str,
) -> SourceRevision {
    let candidate_id = format!("{record_type}/{id}");
    let content = json!({
        "resourceType": record_type,
        "id": id,
        "text": body_text,
    })
    .to_string();
    let sha256 = sha256_hex(content.as_bytes());
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.clone(),
        revision: sha256.clone(),
        record_type: record_type.to_owned(),
        dates: vec![RecordDate {
            name: "recordedDate".to_owned(),
            value: "2026-08-20".to_owned(),
        }],
        content,
        sha256,
        readable_parts: Vec::new(),
        provenance: json!({ "native_record": candidate_id }),
        envelope: None,
    }
}

#[derive(Clone)]
enum ScriptedPull {
    Complete {
        name: String,
        records: Vec<(String, String, String)>,
    },
    Partial {
        name: Option<String>,
        succeeded_kinds: Vec<String>,
        records: Vec<(String, String, String)>,
        failed_parts: Vec<FailedPart>,
    },
    Failed {
        reason: String,
    },
    LapsedAccess {
        reason: String,
    },
}

struct ScriptedSource {
    source_id: String,
    source_label: String,
    pulls: Mutex<Vec<ScriptedPull>>,
    pull_invocations: Mutex<usize>,
    reconnected_for: Mutex<Vec<(String, i64)>>,
}

impl ScriptedSource {
    fn new(source_id: &str, scripted: Vec<ScriptedPull>) -> Arc<Self> {
        Arc::new(Self {
            source_id: source_id.to_owned(),
            source_label: "Scripted Clinic EHR".to_owned(),
            pulls: Mutex::new(scripted),
            pull_invocations: Mutex::new(0),
            reconnected_for: Mutex::new(Vec::new()),
        })
    }

    fn push(&self, next: ScriptedPull) {
        self.pulls.lock().unwrap().push(next);
    }

    fn pull_count(&self) -> usize {
        *self.pull_invocations.lock().unwrap()
    }
}

#[async_trait]
impl PatientSource for ScriptedSource {
    fn source_id(&self) -> &str {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        &self.source_label
    }

    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        Ok(vec!["pat-1".to_owned()])
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, SourceAdapterError> {
        Ok(make_patient(source_patient_id, "Elena Morales"))
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        _workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        Ok(Vec::new())
    }

    async fn pull_patient(
        &self,
        _practice_id: &str,
        source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        *self.pull_invocations.lock().unwrap() += 1;
        let mut queue = self.pulls.lock().unwrap();
        assert!(
            !queue.is_empty(),
            "scripted pull queue exhausted for {source_patient_id}"
        );
        let step = queue.remove(0);
        match step {
            ScriptedPull::Complete { name, records } => SourcePullOutput::Complete {
                patient: make_patient(source_patient_id, &name),
                records: records
                    .into_iter()
                    .map(|(rtype, id, text)| {
                        make_revision(workspace, &self.source_id, &rtype, &id, &text)
                    })
                    .collect(),
            },
            ScriptedPull::Partial {
                name,
                succeeded_kinds,
                records,
                failed_parts,
            } => {
                let mut kind_names = succeeded_kinds;
                for (rtype, _, _) in &records {
                    if !kind_names.contains(rtype) {
                        kind_names.push(rtype.clone());
                    }
                }
                SourcePullOutput::Partial {
                    patient: name.map(|n| make_patient(source_patient_id, &n)),
                    succeeded_kinds: kind_names
                        .into_iter()
                        .map(|kind| SourceRecordKind {
                            kind,
                            category: None,
                        })
                        .collect(),
                    records: records
                        .into_iter()
                        .map(|(rtype, id, text)| {
                            make_revision(workspace, &self.source_id, &rtype, &id, &text)
                        })
                        .collect(),
                    failed_parts,
                }
            }
            ScriptedPull::Failed { reason } => SourcePullOutput::Failed {
                error: SourceAdapterError::Unavailable { reason },
            },
            ScriptedPull::LapsedAccess { reason } => SourcePullOutput::LapsedAccess {
                error: SourceAdapterError::LapsedAccess { reason },
            },
        }
    }

    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        _command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        self.reconnected_for
            .lock()
            .unwrap()
            .push((practice_id.to_owned(), now));
        Ok(ReconnectReceipt {
            source_id: self.source_id.clone(),
            practice_id: practice_id.to_owned(),
            reconnected_at: now,
        })
    }
}

#[test]
fn pure_standing_and_outage_rules_cover_partial_failed_lapsed_and_majority_thresholds() {
    let connected = SourceAccessState::Connected {
        connected_at: 1_000,
    };
    let good_status = PatientSourceRecordStatus {
        practice_id: "prac-1".into(),
        workspace_id: "pw-1".into(),
        source_id: "epic".into(),
        last_good_read_at: Some(1_100),
        latest_attempt: Some(PullAttempt {
            attempted_at: 1_100,
            outcome: PullOutcome::Succeeded {
                missing_candidates: Vec::new(),
            },
        }),
    };
    assert_eq!(
        derive_standing(Some(&connected), Some(&good_status)),
        SourceStanding::UpToDate {
            last_good_read_at: 1_100,
            missing_candidates: Vec::new(),
        }
    );

    // 1 of 2 failing (exact half) stays per-patient, never a practice-wide outage.
    let one_of_two = vec![
        (
            "pw-1".to_owned(),
            SourceStanding::Failed {
                last_good_read_at: Some(1_100),
                attempted_at: 1_200,
                reason: "chart lock timeout".into(),
            },
        ),
        (
            "pw-2".to_owned(),
            SourceStanding::UpToDate {
                last_good_read_at: 1_200,
                missing_candidates: Vec::new(),
            },
        ),
    ];
    match summarize_practice_source("prac-1", "epic", Some(&connected), &one_of_two) {
        PracticeSourceSummary::PatientIssues {
            total_patients,
            issues,
            ..
        } => {
            assert_eq!(total_patients, 2);
            assert_eq!(issues.len(), 1);
            assert_eq!(issues[0].workspace_id, "pw-1");
        }
        other => panic!("expected PatientIssues for 1 of 2 failing, got {other:?}"),
    }

    // 3 of 5 failing (strict majority) collapses into one PracticeWideOutage with zero per-patient rows.
    let three_of_five = vec![
        (
            "pw-1".to_owned(),
            SourceStanding::Failed {
                last_good_read_at: Some(1_100),
                attempted_at: 1_300,
                reason: "503 upstream unavailable".into(),
            },
        ),
        (
            "pw-2".to_owned(),
            SourceStanding::Partial {
                last_good_read_at: Some(1_100),
                attempted_at: 1_300,
                failed_parts: vec![FailedPart {
                    part: "Observation".into(),
                    reason: "503 upstream unavailable".into(),
                }],
                missing_candidates: Vec::new(),
            },
        ),
        (
            "pw-3".to_owned(),
            SourceStanding::Failed {
                last_good_read_at: Some(1_100),
                attempted_at: 1_300,
                reason: "503 upstream unavailable".into(),
            },
        ),
        (
            "pw-4".to_owned(),
            SourceStanding::UpToDate {
                last_good_read_at: 1_300,
                missing_candidates: Vec::new(),
            },
        ),
        (
            "pw-5".to_owned(),
            SourceStanding::UpToDate {
                last_good_read_at: 1_300,
                missing_candidates: Vec::new(),
            },
        ),
    ];
    assert_eq!(
        summarize_practice_source("prac-1", "epic", Some(&connected), &three_of_five),
        PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
            practice_id: "prac-1".into(),
            source_id: "epic".into(),
            affected_patients: 3,
            total_patients: 5,
            kind: PracticeOutageKind::SourceDown {
                reason: "source reads are failing across most linked patients".into(),
            },
        })
    );
}

#[tokio::test]
async fn source_patient_resolves_to_one_chords_patient_across_pulls_and_refuses_other_practice_writes()
 {
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice_a = format!("practice-a-{run}");
    let practice_b = format!("practice-b-{run}");
    let source_id = format!("ehr-{run}");

    let source_a = ScriptedSource::new(
        &source_id,
        vec![
            ScriptedPull::Complete {
                name: "Elena Morales".into(),
                records: vec![("Condition".into(), "cond-1".into(), "Hypertension".into())],
            },
            ScriptedPull::Complete {
                name: "Elena M. Morales".into(),
                records: vec![
                    ("Condition".into(), "cond-1".into(), "Hypertension".into()),
                    ("Observation".into(), "obs-1".into(), "BP 126/78".into()),
                ],
            },
        ],
    );

    let chosen_workspace = format!("pw-elena-{run}");
    let first = SourceCoordinator::check_patient(
        &store,
        &*source_a,
        &practice_a,
        "src-pat-42",
        LinkTarget::ExistingOrDefault(&chosen_workspace),
        1_700_000_100,
    )
    .await
    .unwrap();
    assert_eq!(first.workspace_id, chosen_workspace);
    assert_eq!(first.changed, 1);

    // Second pull supplies a different fallback default workspace, yet the link resolves to the
    // exact same Chords patient (`chosen_workspace`).
    let different_default = workspace_id(&source_id, "src-pat-42");
    assert_ne!(different_default, chosen_workspace);
    let second = SourceCoordinator::check_patient(
        &store,
        &*source_a,
        &practice_a,
        "src-pat-42",
        LinkTarget::ExistingOrDefault(&different_default),
        1_700_000_200,
    )
    .await
    .unwrap();
    assert_eq!(second.workspace_id, chosen_workspace);
    assert_eq!(second.changed, 1);
    assert_eq!(
        store
            .sources(&chosen_workspace)
            .await
            .unwrap()
            .sources
            .len(),
        2
    );

    // Explicitly demanding a conflicting workspace for the same source patient in the same practice is refused.
    let conflict = SourceCoordinator::check_patient(
        &store,
        &*source_a,
        &practice_a,
        "src-pat-42",
        LinkTarget::Exact(&different_default),
        1_700_000_250,
    )
    .await
    .unwrap_err();
    assert!(
        matches!(conflict, SourceStatusError::ConflictingWorkspaceLink { .. }),
        "expected ConflictingWorkspaceLink, got {conflict:?}"
    );
    assert_eq!(
        source_a.pull_count(),
        2,
        "conflicting link must abort before calling the source adapter"
    );

    // Another practice (`practice_b`) attempting to pull or read into `chosen_workspace` aborts
    // with PracticeMismatch BEFORE calling the adapter or saving any records.
    let rogue_source = ScriptedSource::new(
        &source_id,
        vec![ScriptedPull::Complete {
            name: "Wrong Practice Patient".into(),
            records: vec![(
                "MedicationRequest".into(),
                "med-rogue".into(),
                "Should never be saved".into(),
            )],
        }],
    );
    let mismatch = SourceCoordinator::check_patient(
        &store,
        &*rogue_source,
        &practice_b,
        "src-pat-42",
        LinkTarget::ExistingOrDefault(&chosen_workspace),
        1_700_000_300,
    )
    .await
    .unwrap_err();
    assert!(
        matches!(
            mismatch,
            SourceStatusError::PracticeMismatch {
                ref workspace_id,
                ref owner_practice_id,
                ref requested_practice_id,
            } if workspace_id == &chosen_workspace
                && owner_practice_id == &practice_a
                && requested_practice_id == &practice_b
        ),
        "expected PracticeMismatch, got {mismatch:?}"
    );
    assert_eq!(
        rogue_source.pull_count(),
        0,
        "cross-practice attempt must not invoke pull_patient"
    );

    // Verify zero rogue records and zero links were written for `practice_b`.
    let saved_sources = store.sources(&chosen_workspace).await.unwrap().sources;
    assert_eq!(saved_sources.len(), 2);
    assert!(
        saved_sources
            .iter()
            .all(|rev| rev.candidate_id != "MedicationRequest/med-rogue")
    );
    let practice_b_link = store
        .source_patient_link(&SourcePatientKey {
            practice_id: practice_b.clone(),
            source_id: source_id.clone(),
            source_patient_id: "src-pat-42".into(),
        })
        .await
        .unwrap();
    assert!(practice_b_link.is_none());

    // Cross-practice status read on `chosen_workspace` is also refused.
    let read_mismatch = store
        .patient_source_statuses(&practice_b, &chosen_workspace)
        .await
        .unwrap_err();
    assert!(matches!(
        read_mismatch,
        SourceStatusError::PracticeMismatch { .. }
    ));
}

#[tokio::test]
async fn failed_partial_and_lapsed_pulls_keep_last_good_read_time_and_reconnect_recovers() {
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let source_id = format!("ehr-{run}");
    let workspace = format!("pw-pat-{run}");

    let source = ScriptedSource::new(
        &source_id,
        vec![
            // 1. Complete good pull at t = 1_000.
            ScriptedPull::Complete {
                name: "Arthur Pendelton".into(),
                records: vec![
                    ("Patient".into(), "pat-1".into(), "Arthur Pendelton".into()),
                    (
                        "DocumentReference".into(),
                        "note-1".into(),
                        "Initial consult note".into(),
                    ),
                ],
            },
            // 2. Whole failure at t = 2_000.
            ScriptedPull::Failed {
                reason: "connection reset by peer".into(),
            },
            // 3. Partial pull at t = 3_000: Encounter succeeds, DocumentReference fails.
            ScriptedPull::Partial {
                name: Some("Arthur Pendelton".into()),
                succeeded_kinds: vec!["Patient".into(), "Encounter".into()],
                records: vec![
                    ("Patient".into(), "pat-1".into(), "Arthur Pendelton".into()),
                    ("Encounter".into(), "enc-1".into(), "Follow-up visit".into()),
                ],
                failed_parts: vec![FailedPart {
                    part: "DocumentReference".into(),
                    reason: "notes service 502".into(),
                }],
            },
            // 4. Lapsed access at t = 4_000.
            ScriptedPull::LapsedAccess {
                reason: "oauth refresh token revoked".into(),
            },
            // 5. Complete recovery pull during reconnect at t = 5_000.
            ScriptedPull::Complete {
                name: "Arthur Pendelton".into(),
                records: vec![
                    ("Patient".into(), "pat-1".into(), "Arthur Pendelton".into()),
                    (
                        "DocumentReference".into(),
                        "note-1".into(),
                        "Initial consult note".into(),
                    ),
                    ("Encounter".into(), "enc-1".into(), "Follow-up visit".into()),
                ],
            },
        ],
    );

    // Step 1: Good read at t = 1_000.
    let good = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "pat-1",
        LinkTarget::ExistingOrDefault(&workspace),
        1_000,
    )
    .await
    .unwrap();
    assert_eq!(
        good.standing,
        SourceStanding::UpToDate {
            last_good_read_at: 1_000,
            missing_candidates: Vec::new(),
        }
    );

    // Step 2: Failed pull at t = 2_000 keeps last_good_read_at = 1_000.
    let failed = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "pat-1",
        LinkTarget::ExistingOrDefault(&workspace),
        2_000,
    )
    .await
    .unwrap();
    assert_eq!(
        failed.standing,
        SourceStanding::Failed {
            last_good_read_at: Some(1_000),
            attempted_at: 2_000,
            reason: "connection reset by peer".into(),
        }
    );

    // Step 3: Partial pull at t = 3_000 saves Encounter/enc-1, keeps DocumentReference/note-1,
    // names failed part `DocumentReference`, and STILL preserves last_good_read_at = 1_000.
    let partial = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "pat-1",
        LinkTarget::ExistingOrDefault(&workspace),
        3_000,
    )
    .await
    .unwrap();
    assert_eq!(partial.changed, 1);
    assert_eq!(
        partial.standing,
        SourceStanding::Partial {
            last_good_read_at: Some(1_000),
            attempted_at: 3_000,
            failed_parts: vec![FailedPart {
                part: "DocumentReference".into(),
                reason: "notes service 502".into(),
            }],
            missing_candidates: Vec::new(),
        }
    );
    let candidate_ids: Vec<String> = store
        .sources(&workspace)
        .await
        .unwrap()
        .sources
        .into_iter()
        .map(|rev| rev.candidate_id)
        .collect();
    assert_eq!(
        candidate_ids,
        vec![
            "DocumentReference/note-1".to_owned(),
            "Encounter/enc-1".to_owned(),
            "Patient/pat-1".to_owned(),
        ]
    );
    let pulls = store
        .source_pulls(std::slice::from_ref(&workspace))
        .await
        .unwrap();
    assert_eq!(
        pulls[&workspace][0].pulled_at,
        Some(1_000),
        "partial pull must not advance last complete pulled_at"
    );

    // Step 4: Lapsed access at t = 4_000 marks ReconnectNeeded while keeping last_good_read_at = 1_000.
    let lapsed = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "pat-1",
        LinkTarget::ExistingOrDefault(&workspace),
        4_000,
    )
    .await
    .unwrap();
    assert_eq!(
        lapsed.standing,
        SourceStanding::ReconnectNeeded {
            last_good_read_at: Some(1_000),
            attempted_at: Some(4_000),
            reason: "oauth refresh token revoked".into(),
        }
    );
    assert_eq!(
        store
            .practice_source_report(&practice, &source_id)
            .await
            .unwrap()
            .summary,
        PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
            practice_id: practice.clone(),
            source_id: source_id.clone(),
            affected_patients: 1,
            total_patients: 1,
            kind: PracticeOutageKind::ReconnectNeeded {
                reason: "source access requires reconnect across most linked patients".into(),
            },
        })
    );

    // Step 5: Reconnect at t = 5_000 restores Connected and advances last_good_read_at = 5_000.
    let reconnected = SourceCoordinator::reconnect_source(&store, &*source, &practice, 5_000)
        .await
        .unwrap();
    assert_eq!(reconnected.receipt.reconnected_at, 5_000);
    assert_eq!(
        reconnected.access,
        SourceAccessState::Connected {
            connected_at: 5_000
        }
    );
    assert_eq!(
        reconnected.check.summary,
        PracticeSourceSummary::Healthy {
            source_id: source_id.clone(),
            total_patients: 1,
        }
    );
    assert_eq!(
        reconnected.check.patients[0].standing,
        SourceStanding::UpToDate {
            last_good_read_at: 5_000,
            missing_candidates: Vec::new(),
        }
    );
}

#[tokio::test]
async fn practice_report_returns_one_outage_instead_of_patient_rows_when_majority_fails() {
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let source_id = format!("ehr-{run}");

    let source = ScriptedSource::new(&source_id, Vec::new());

    // Link 4 patients initially healthy at t = 1_000.
    for index in 1..=4 {
        source.push(ScriptedPull::Complete {
            name: format!("Patient {index}"),
            records: vec![(
                "Patient".into(),
                format!("p-{index}"),
                format!("Patient {index}"),
            )],
        });
        let ws = format!("pw-{index}-{run}");
        SourceCoordinator::check_patient(
            &store,
            &*source,
            &practice,
            &format!("p-{index}"),
            LinkTarget::Exact(&ws),
            1_000,
        )
        .await
        .unwrap();
    }

    // Fail 1 of 4 -> PatientIssues (1 row), not a practice outage.
    source.push(ScriptedPull::Failed {
        reason: "record locked".into(),
    });
    let ws1 = format!("pw-1-{run}");
    SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "p-1",
        LinkTarget::Exact(&ws1),
        2_000,
    )
    .await
    .unwrap();

    let minority_report = store
        .practice_source_report(&practice, &source_id)
        .await
        .unwrap();
    match minority_report.summary {
        PracticeSourceSummary::PatientIssues {
            total_patients,
            issues,
            ..
        } => {
            assert_eq!(total_patients, 4);
            assert_eq!(issues.len(), 1);
            assert_eq!(issues[0].workspace_id, ws1);
        }
        other => panic!("expected PatientIssues when 1 of 4 fails, got {other:?}"),
    }

    // Now fail patients 2 and 3 as well (3 of 4 failing = strict majority) -> ONE PracticeWideOutage!
    for index in [2, 3] {
        source.push(ScriptedPull::Failed {
            reason: "EHR gateway 503".into(),
        });
        let ws = format!("pw-{index}-{run}");
        SourceCoordinator::check_patient(
            &store,
            &*source,
            &practice,
            &format!("p-{index}"),
            LinkTarget::Exact(&ws),
            3_000,
        )
        .await
        .unwrap();
    }

    let outage_report = store
        .practice_source_report(&practice, &source_id)
        .await
        .unwrap();
    assert_eq!(
        outage_report.summary,
        PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
            practice_id: practice,
            source_id,
            affected_patients: 3,
            total_patients: 4,
            kind: PracticeOutageKind::SourceDown {
                reason: "source reads are failing across most linked patients".into(),
            },
        })
    );
    assert!(
        !serde_json::to_string(&outage_report.summary)
            .unwrap()
            .contains("record locked"),
        "practice-wide outage summary must not copy patient-specific failure diagnostics"
    );
}

#[tokio::test]
async fn single_patient_lapsed_access_stays_patient_scoped_and_later_success_recovers_cleanly() {
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let source_id = format!("ehr-{run}");
    let source = ScriptedSource::new(&source_id, Vec::new());

    for index in 1..=3 {
        source.push(ScriptedPull::Complete {
            name: format!("Patient {index}"),
            records: vec![(
                "Patient".into(),
                format!("p-{index}"),
                format!("Patient {index}"),
            )],
        });
        let ws = format!("pw-{index}-{run}");
        SourceCoordinator::check_patient(
            &store,
            &*source,
            &practice,
            &format!("p-{index}"),
            LinkTarget::Exact(&ws),
            1_000,
        )
        .await
        .unwrap();
    }

    // Only patient 1 hits LapsedAccess at t = 2_000 (1 of 3 = minority).
    let ws1 = format!("pw-1-{run}");
    source.push(ScriptedPull::LapsedAccess {
        reason: "patient portal grant expired".into(),
    });
    let p1_lapsed = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "p-1",
        LinkTarget::Exact(&ws1),
        2_000,
    )
    .await
    .unwrap();
    assert_eq!(
        p1_lapsed.standing,
        SourceStanding::ReconnectNeeded {
            last_good_read_at: Some(1_000),
            attempted_at: Some(2_000),
            reason: "patient portal grant expired".into(),
        }
    );

    // Practice-level access stays Connected and practice summary stays PatientIssues (only p-1), not a practice outage.
    assert_eq!(
        store
            .practice_source_access(&practice, &source_id)
            .await
            .unwrap(),
        Some(SourceAccessState::Connected {
            connected_at: 1_000
        })
    );
    let report = store
        .practice_source_report(&practice, &source_id)
        .await
        .unwrap();
    match report.summary {
        PracticeSourceSummary::PatientIssues { issues, .. } => {
            assert_eq!(issues.len(), 1);
            assert_eq!(issues[0].workspace_id, ws1);
        }
        other => panic!("expected single PatientIssues for 1/3 lapsed access, got {other:?}"),
    }

    // Even if practice_source_access was marked ReconnectNeeded, a subsequent Succeeded pull recovers UpToDate cleanly.
    store
        .set_practice_source_access(
            &practice,
            &source_id,
            &SourceAccessState::ReconnectNeeded {
                since: 2_500,
                reason: "temporary token rotation".into(),
            },
            2_500,
        )
        .await
        .unwrap();
    source.push(ScriptedPull::Complete {
        name: "Patient 1".into(),
        records: vec![("Patient".into(), "p-1".into(), "Patient 1".into())],
    });
    let recovered = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "p-1",
        LinkTarget::Exact(&ws1),
        3_000,
    )
    .await
    .unwrap();
    assert_eq!(
        recovered.standing,
        SourceStanding::UpToDate {
            last_good_read_at: 3_000,
            missing_candidates: Vec::new(),
        }
    );
    assert_eq!(
        store
            .practice_source_access(&practice, &source_id)
            .await
            .unwrap(),
        Some(SourceAccessState::Connected {
            connected_at: 3_000
        })
    );

    // During reconnect_source at t = 4_000, if p-1 still returns LapsedAccess while p-2 and p-3
    // succeed (keeping access = Connected { connected_at: 4_000 }), p-1 MUST still report
    // ReconnectNeeded rather than being masked as UpToDate.
    source.push(ScriptedPull::LapsedAccess {
        reason: "patient-1 consent still revoked".into(),
    });
    source.push(ScriptedPull::Complete {
        name: "Patient 2".into(),
        records: vec![("Patient".into(), "p-2".into(), "Patient 2".into())],
    });
    source.push(ScriptedPull::Complete {
        name: "Patient 3".into(),
        records: vec![("Patient".into(), "p-3".into(), "Patient 3".into())],
    });
    let reconnected = SourceCoordinator::reconnect_source(&store, &*source, &practice, 4_000)
        .await
        .unwrap();
    assert_eq!(
        reconnected.access,
        SourceAccessState::Connected {
            connected_at: 4_000
        }
    );
    let p1_after_reconnect = reconnected
        .check
        .patients
        .iter()
        .find(|p| p.workspace_id == ws1)
        .unwrap();
    assert_eq!(
        p1_after_reconnect.standing,
        SourceStanding::ReconnectNeeded {
            last_good_read_at: Some(3_000),
            attempted_at: Some(4_000),
            reason: "patient-1 consent still revoked".into(),
        }
    );
}

#[tokio::test]
async fn partial_pull_reports_missing_candidates_for_empty_succeeded_kinds_without_flagging_failed_kinds()
 {
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let source_id = format!("ehr-{run}");
    let workspace = format!("pw-empty-kind-{run}");

    let source = ScriptedSource::new(
        &source_id,
        vec![
            ScriptedPull::Complete {
                name: "Clara Oswald".into(),
                records: vec![
                    ("Patient".into(), "pat-1".into(), "Clara Oswald".into()),
                    ("Observation".into(), "obs-1".into(), "Potassium 4.2".into()),
                    (
                        "DocumentReference".into(),
                        "note-1".into(),
                        "Clinic note".into(),
                    ),
                ],
            },
            // Observation succeeded with ZERO Observation records delivered; DocumentReference failed.
            ScriptedPull::Partial {
                name: Some("Clara Oswald".into()),
                succeeded_kinds: vec!["Patient".into(), "Observation".into()],
                records: vec![("Patient".into(), "pat-1".into(), "Clara Oswald".into())],
                failed_parts: vec![FailedPart {
                    part: "DocumentReference".into(),
                    reason: "notes endpoint 503".into(),
                }],
            },
        ],
    );

    SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "pat-1",
        LinkTarget::Exact(&workspace),
        1_000,
    )
    .await
    .unwrap();

    let partial = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        "pat-1",
        LinkTarget::Exact(&workspace),
        2_000,
    )
    .await
    .unwrap();

    // Both previously saved records remain retained in Store.
    let saved_ids: Vec<String> = store
        .sources(&workspace)
        .await
        .unwrap()
        .sources
        .into_iter()
        .map(|rev| rev.candidate_id)
        .collect();
    assert_eq!(
        saved_ids,
        vec![
            "DocumentReference/note-1".to_owned(),
            "Observation/obs-1".to_owned(),
            "Patient/pat-1".to_owned(),
        ]
    );

    // Observation/obs-1 IS reported in missing_candidates (Observation succeeded empty),
    // while DocumentReference/note-1 is NOT (DocumentReference failed).
    assert_eq!(
        partial.missing_candidates,
        vec!["Observation/obs-1".to_owned()]
    );
    assert_eq!(
        partial.standing,
        SourceStanding::Partial {
            last_good_read_at: Some(1_000),
            attempted_at: 2_000,
            failed_parts: vec![FailedPart {
                part: "DocumentReference".into(),
                reason: "notes endpoint 503".into(),
            }],
            missing_candidates: vec!["Observation/obs-1".to_owned()],
        }
    );
}

#[tokio::test]
async fn initial_onboarding_handles_partial_identity_and_records_initial_failed_or_lapsed_attempts()
{
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let source_id = format!("ehr-{run}");

    let source = ScriptedSource::new(
        &source_id,
        vec![
            ScriptedPull::Partial {
                name: Some("Initial Partial With Identity".into()),
                succeeded_kinds: vec!["Patient".into()],
                records: vec![("Patient".into(), "p-id".into(), "Identity present".into())],
                failed_parts: vec![FailedPart {
                    part: "Observation".into(),
                    reason: "lab endpoint timeout".into(),
                }],
            },
            ScriptedPull::Partial {
                name: None,
                succeeded_kinds: vec!["Observation".into()],
                records: vec![("Observation".into(), "obs-1".into(), "BP 120/80".into())],
                failed_parts: vec![FailedPart {
                    part: "Patient".into(),
                    reason: "demographics 503".into(),
                }],
            },
            ScriptedPull::Failed {
                reason: "corrupt initial bundle".into(),
            },
            ScriptedPull::LapsedAccess {
                reason: "initial token rejected".into(),
            },
        ],
    );

    // 1. Initial Partial WITH patient identity: links patient, persists directory SourcePull
    // with pulled_at == None (no sentinel), and keeps last_good_read_at == None.
    let ws_with_id = format!("pw-init-partial-id-{run}");
    let r_with_id = SourceCoordinator::onboard_or_check_patient(
        &store,
        &*source,
        &practice,
        "p-with-id",
        LinkTarget::ExistingOrDefault(&ws_with_id),
        1_500,
    )
    .await
    .unwrap();
    assert_eq!(
        r_with_id.standing,
        SourceStanding::Partial {
            last_good_read_at: None,
            attempted_at: 1_500,
            failed_parts: vec![FailedPart {
                part: "Observation".into(),
                reason: "lab endpoint timeout".into(),
            }],
            missing_candidates: Vec::new(),
        }
    );
    let pulls = store
        .source_pulls(std::slice::from_ref(&ws_with_id))
        .await
        .unwrap();
    assert_eq!(pulls[&ws_with_id].len(), 1);
    assert_eq!(
        pulls[&ws_with_id][0].patient.display_name,
        "Initial Partial With Identity"
    );
    assert_eq!(pulls[&ws_with_id][0].pulled_at, None);

    // 2. Initial Partial WITHOUT patient identity: withholds SourcePull directory exposure while
    // retaining an addressable source_patient_link and recording Partial status.
    let ws_no_id = format!("pw-init-partial-noid-{run}");
    let r_no_id = SourceCoordinator::onboard_or_check_patient(
        &store,
        &*source,
        &practice,
        "p-no-id",
        LinkTarget::ExistingOrDefault(&ws_no_id),
        1_500,
    )
    .await
    .unwrap();
    assert!(matches!(
        r_no_id.standing,
        SourceStanding::Partial {
            last_good_read_at: None,
            ..
        }
    ));
    assert!(
        store
            .source_pulls(std::slice::from_ref(&ws_no_id))
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        store
            .source_patient_link(&SourcePatientKey {
                practice_id: practice.clone(),
                source_id: source_id.clone(),
                source_patient_id: "p-no-id".into(),
            })
            .await
            .unwrap()
            .unwrap()
            .workspace_id,
        ws_no_id
    );

    // 3. Initial Failed and LapsedAccess: withhold SourcePull directory exposure while retaining
    // addressable source_patient_links so check_practice / reconnect_source can retry and clear them.
    let ws_failed = format!("pw-init-failed-{run}");
    let r_failed = SourceCoordinator::onboard_or_check_patient(
        &store,
        &*source,
        &practice,
        "p-failed",
        LinkTarget::ExistingOrDefault(&ws_failed),
        1_500,
    )
    .await
    .unwrap();
    assert_eq!(
        r_failed.standing,
        SourceStanding::Failed {
            last_good_read_at: None,
            attempted_at: 1_500,
            reason: "corrupt initial bundle".into(),
        }
    );

    let ws_lapsed = format!("pw-init-lapsed-{run}");
    let r_lapsed = SourceCoordinator::onboard_or_check_patient(
        &store,
        &*source,
        &practice,
        "p-lapsed",
        LinkTarget::ExistingOrDefault(&ws_lapsed),
        1_500,
    )
    .await
    .unwrap();
    assert_eq!(
        r_lapsed.standing,
        SourceStanding::ReconnectNeeded {
            last_good_read_at: None,
            attempted_at: Some(1_500),
            reason: "initial token rejected".into(),
        }
    );
    assert!(
        store
            .source_pulls(&[ws_failed.clone(), ws_lapsed.clone()])
            .await
            .unwrap()
            .is_empty(),
        "unidentified failed/lapsed initial pulls stay hidden from patient directory"
    );

    // 4. Recovery via reconnect_source: check_practice visits all 4 linked patients (ordered by
    // source_patient_id: p-failed, p-lapsed, p-no-id, p-with-id), delivers Complete charts,
    // populates SourcePull directory identities, and clears stale statuses to Healthy!
    for (spid, label) in [
        ("p-failed", "Recovered Failed"),
        ("p-lapsed", "Recovered Lapsed"),
        ("p-no-id", "Recovered NoId"),
        ("p-with-id", "Recovered WithId"),
    ] {
        source.push(ScriptedPull::Complete {
            name: label.into(),
            records: vec![("Patient".into(), spid.into(), label.into())],
        });
    }
    let recovered = SourceCoordinator::reconnect_source(&store, &*source, &practice, 2_500)
        .await
        .unwrap();
    assert_eq!(recovered.check.patients.len(), 4);
    assert_eq!(
        recovered.check.summary,
        PracticeSourceSummary::Healthy {
            source_id: source_id.clone(),
            total_patients: 4,
        }
    );
    let recovered_pulls = store
        .source_pulls(&[
            ws_failed.clone(),
            ws_lapsed.clone(),
            ws_no_id.clone(),
            ws_with_id.clone(),
        ])
        .await
        .unwrap();
    assert_eq!(recovered_pulls.len(), 4);
    for ws in [&ws_failed, &ws_lapsed, &ws_no_id, &ws_with_id] {
        assert_eq!(recovered_pulls[ws][0].pulled_at, Some(2_500));
    }
}

#[tokio::test]
async fn same_timestamp_practice_check_keeps_third_patient_up_to_date_when_two_colleagues_lapse() {
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let source_id = format!("ehr-{run}");
    let source = ScriptedSource::new(&source_id, Vec::new());

    for index in 1..=3 {
        source.push(ScriptedPull::Complete {
            name: format!("Patient {index}"),
            records: vec![(
                "Patient".into(),
                format!("p-{index}"),
                format!("Patient {index}"),
            )],
        });
        let ws = format!("pw-{index}-{run}");
        SourceCoordinator::check_patient(
            &store,
            &*source,
            &practice,
            &format!("p-{index}"),
            LinkTarget::Exact(&ws),
            1_000,
        )
        .await
        .unwrap();
    }

    // At fixed timestamp t = 2_000, patients 1 and 2 return LapsedAccess while patient 3 succeeds.
    source.push(ScriptedPull::LapsedAccess {
        reason: "p-1 scope expired".into(),
    });
    source.push(ScriptedPull::LapsedAccess {
        reason: "p-2 scope expired".into(),
    });
    source.push(ScriptedPull::Complete {
        name: "Patient 3".into(),
        records: vec![("Patient".into(), "p-3".into(), "Patient 3".into())],
    });

    let check = SourceCoordinator::check_practice(&store, &*source, &practice, 2_000)
        .await
        .unwrap();
    assert_eq!(
        check.summary,
        PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
            practice_id: practice.clone(),
            source_id: source_id.clone(),
            affected_patients: 2,
            total_patients: 3,
            kind: PracticeOutageKind::ReconnectNeeded {
                reason: "source access requires reconnect across most linked patients".into(),
            },
        })
    );

    let ws3 = format!("pw-3-{run}");
    let p3 = check
        .patients
        .iter()
        .find(|p| p.workspace_id == ws3)
        .unwrap();
    assert_eq!(
        p3.standing,
        SourceStanding::UpToDate {
            last_good_read_at: 2_000,
            missing_candidates: Vec::new(),
        }
    );

    let standings = store
        .workspace_source_standings(std::slice::from_ref(&ws3))
        .await
        .unwrap();
    assert_eq!(
        standings[&ws3][&source_id],
        SourceStanding::UpToDate {
            last_good_read_at: 2_000,
            missing_candidates: Vec::new(),
        }
    );
}

struct GateControlledSource {
    source_id: String,
    reconnect_calls: std::sync::atomic::AtomicUsize,
    pull_calls: std::sync::atomic::AtomicUsize,
    reconnect_delay_ms: u64,
    pause_pull: std::sync::atomic::AtomicBool,
    pull_entered: tokio::sync::Notify,
    pull_release: tokio::sync::Notify,
}

impl GateControlledSource {
    fn new(source_id: &str, reconnect_delay_ms: u64) -> Arc<Self> {
        Arc::new(Self {
            source_id: source_id.to_owned(),
            reconnect_calls: std::sync::atomic::AtomicUsize::new(0),
            pull_calls: std::sync::atomic::AtomicUsize::new(0),
            reconnect_delay_ms,
            pause_pull: std::sync::atomic::AtomicBool::new(false),
            pull_entered: tokio::sync::Notify::new(),
            pull_release: tokio::sync::Notify::new(),
        })
    }
}

#[async_trait]
impl PatientSource for GateControlledSource {
    fn source_id(&self) -> &str {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        "Gate Controlled EHR"
    }

    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        Ok(vec!["pat-1".to_owned()])
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, SourceAdapterError> {
        Ok(make_patient(source_patient_id, "Elena Morales"))
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        _workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        Ok(Vec::new())
    }

    async fn pull_patient(
        &self,
        _practice_id: &str,
        source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        self.pull_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if self.pause_pull.load(std::sync::atomic::Ordering::SeqCst) {
            self.pull_entered.notify_waiters();
            self.pull_release.notified().await;
        }
        SourcePullOutput::Complete {
            patient: make_patient(source_patient_id, "Elena Morales"),
            records: vec![make_revision(
                workspace,
                &self.source_id,
                "Patient",
                source_patient_id,
                "Elena Morales",
            )],
        }
    }

    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        _command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        self.reconnect_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if self.reconnect_delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.reconnect_delay_ms)).await;
        }
        Ok(ReconnectReceipt {
            source_id: self.source_id.clone(),
            practice_id: practice_id.to_owned(),
            reconnected_at: now,
        })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reconnect_command_serializes_concurrent_duplicates_recovers_interrupted_check_and_preserves_original_receipt()
 {
    use std::sync::atomic::Ordering::SeqCst;

    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let physician = format!("dr-{run}");
    let source_a_id = format!("ehr-a-{run}");
    let source_b_id = format!("ehr-b-{run}");
    let source_a = GateControlledSource::new(&source_a_id, 60);
    let source_b = GateControlledSource::new(&source_b_id, 0);

    let ws = format!("pw-cmd-{run}");
    SourceCoordinator::check_patient(
        &store,
        &*source_a,
        &practice,
        "pat-1",
        LinkTarget::Exact(&ws),
        1_000,
    )
    .await
    .unwrap();
    assert_eq!(source_a.pull_calls.load(SeqCst), 1);

    // (a) Concurrent duplicate `reconnect_source_command` calls execute `reconnect()` and
    // `pull_patient()` ONCE total while returning identical Accepted receipts to both callers.
    let key_1 = format!("key-dup-{run}");
    let (out_a, out_b) = tokio::join!(
        SourceCoordinator::reconnect_source_command(
            &store,
            ReconnectCommand {
                practice_id: &practice,
                physician_id: &physician,
                idempotency_key: &key_1,
                source_id: &source_a_id,
                source: Some(&*source_a),
                now: 2_000,
            },
        ),
        SourceCoordinator::reconnect_source_command(
            &store,
            ReconnectCommand {
                practice_id: &practice,
                physician_id: &physician,
                idempotency_key: &key_1,
                source_id: &source_a_id,
                source: Some(&*source_a),
                now: 2_000,
            },
        ),
    );
    let out_a = out_a.unwrap();
    let out_b = out_b.unwrap();
    assert_eq!(out_a, out_b);
    assert_eq!(
        source_a.reconnect_calls.load(SeqCst),
        1,
        "concurrent duplicate reconnect commands must invoke source.reconnect() only once"
    );
    assert_eq!(
        source_a.pull_calls.load(SeqCst),
        2,
        "concurrent duplicate reconnect commands must run check_practice only once"
    );
    assert!(matches!(
        &out_a,
        ReconnectCommandOutcome::Accepted { receipt, .. } if receipt.reconnected_at == 2_000
    ));

    // (d) Conflicting key reuse against BOTH a second registered PatientSource AND an unregistered
    // source is rejected via the receipt owner without invoking the second adapter.
    let conflict_registered = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: &key_1,
            source_id: &source_b_id,
            source: Some(&*source_b),
            now: 2_500,
        },
    )
    .await
    .unwrap();
    assert_eq!(conflict_registered, ReconnectCommandOutcome::ConflictingKey);
    assert_eq!(
        source_b.reconnect_calls.load(SeqCst),
        0,
        "conflicting registered source must never execute reconnect()"
    );

    let conflict_unregistered = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: &key_1,
            source_id: "unregistered-ehr",
            source: None,
            now: 2_500,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        conflict_unregistered,
        ReconnectCommandOutcome::ConflictingKey
    );

    // Concurrent conflicting commands (`source_a` vs `source_b` under a fresh key): at most one
    // executes and the other rejects with `ConflictingKey` before touching its adapter.
    let key_race_conflict = format!("key-race-conflict-{run}");
    let (race_a, race_b) = tokio::join!(
        SourceCoordinator::reconnect_source_command(
            &store,
            ReconnectCommand {
                practice_id: &practice,
                physician_id: &physician,
                idempotency_key: &key_race_conflict,
                source_id: &source_a_id,
                source: Some(&*source_a),
                now: 3_000,
            },
        ),
        SourceCoordinator::reconnect_source_command(
            &store,
            ReconnectCommand {
                practice_id: &practice,
                physician_id: &physician,
                idempotency_key: &key_race_conflict,
                source_id: &source_b_id,
                source: Some(&*source_b),
                now: 3_000,
            },
        ),
    );
    let race_a = race_a.unwrap();
    let race_b = race_b.unwrap();
    let accepted_count = usize::from(matches!(race_a, ReconnectCommandOutcome::Accepted { .. }))
        + usize::from(matches!(race_b, ReconnectCommandOutcome::Accepted { .. }));
    let conflict_count = usize::from(race_a == ReconnectCommandOutcome::ConflictingKey)
        + usize::from(race_b == ReconnectCommandOutcome::ConflictingKey);
    assert_eq!((accepted_count, conflict_count), (1, 1));
    assert_eq!(
        source_a.reconnect_calls.load(SeqCst) + source_b.reconnect_calls.load(SeqCst),
        2,
        "only the winning side of a concurrent key conflict may call reconnect()"
    );

    // (b) Execute a later reconnect under `key_2` at t = 4_000 (advancing practice_source_access to 4_000),
    // compact receipts, and verify replaying `key_1` at t = 5_000 still returns its ORIGINAL `reconnected_at == 2_000`!
    let key_2 = format!("key-later-{run}");
    let later = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: &key_2,
            source_id: &source_a_id,
            source: Some(&*source_a),
            now: 4_000,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        later,
        ReconnectCommandOutcome::Accepted { receipt, .. } if receipt.reconnected_at == 4_000
    ));
    let _ = store.compact_receipts(0, 100).await.unwrap();

    let replayed_key_1 = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: &key_1,
            source_id: &source_a_id,
            source: Some(&*source_a),
            now: 5_000,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        replayed_key_1, out_a,
        "replaying an older key after a later reconnect and receipt compaction must return its original receipt"
    );

    // (c) Interrupted execution in `Reconnected { receipt }` state: pause `pull_patient`, abort
    // the task after `reconnect()` has committed `Reconnected`, then retry the same key at t = 7_000.
    // Retry must NOT call `source.reconnect()` again and must return the original `reconnected_at == 6_000`.
    let before_interrupted_reconnects = source_a.reconnect_calls.load(SeqCst);
    source_a.pause_pull.store(true, SeqCst);
    let key_interrupted = format!("key-interrupted-{run}");
    let entered = source_a.pull_entered.notified();
    let store_bg = store.clone();
    let practice_bg = practice.clone();
    let physician_bg = physician.clone();
    let key_bg = key_interrupted.clone();
    let source_a_id_bg = source_a_id.clone();
    let source_a_bg = Arc::clone(&source_a);

    let handle = tokio::spawn(async move {
        SourceCoordinator::reconnect_source_command(
            &store_bg,
            ReconnectCommand {
                practice_id: &practice_bg,
                physician_id: &physician_bg,
                idempotency_key: &key_bg,
                source_id: &source_a_id_bg,
                source: Some(&*source_a_bg),
                now: 6_000,
            },
        )
        .await
    });

    entered.await;
    assert_eq!(
        source_a.reconnect_calls.load(SeqCst),
        before_interrupted_reconnects + 1
    );
    handle.abort();
    let _ = handle.await;

    source_a.pause_pull.store(false, SeqCst);
    source_a.pull_release.notify_waiters();

    let recovered_cmd = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: &key_interrupted,
            source_id: &source_a_id,
            source: Some(&*source_a),
            now: 7_000,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        source_a.reconnect_calls.load(SeqCst),
        before_interrupted_reconnects + 1,
        "recovering from Reconnected state must complete check_practice without calling reconnect() again"
    );
    assert!(matches!(
        recovered_cmd,
        ReconnectCommandOutcome::Accepted { receipt, .. } if receipt.reconnected_at == 6_000
    ));
}

#[tokio::test]
async fn partial_and_complete_deliveries_commit_atomically_with_pull_status_and_nullable_pulled_at()
{
    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-{run}");
    let other_practice = format!("other-practice-{run}");
    let ws = format!("pw-atomic-{run}");
    let source_id = format!("ehr-{run}");

    let pat = make_patient("pat-atomic", "Atomic Patient");
    let valid_rev = make_revision(&ws, &source_id, "Observation", "obs-1", "Hemoglobin 13.5");

    // 1. Initial partial pull commits records, SourcePull (`pulled_at == None`), and
    // `PullOutcome::Partial` (`last_good_read_at == None`) together in one transaction.
    let (receipt, status) = store
        .record_partial_pull(PartialDelivery {
            practice_id: &practice,
            workspace_id: &ws,
            source_id: &source_id,
            source_label: "Atomic EHR",
            patient: Some(&pat),
            succeeded_kinds: &[SourceRecordKind {
                kind: "Observation".into(),
                category: None,
            }],
            records: std::slice::from_ref(&valid_rev),
            failed_parts: &[FailedPart {
                part: "DocumentReference".into(),
                reason: "notes 503".into(),
            }],
            attempted_at: 1_200,
            ticket: None,
        })
        .await
        .unwrap();
    assert_eq!(receipt.changed, 1);
    assert_eq!(status.last_good_read_at, None);
    let pulls = store.source_pulls(std::slice::from_ref(&ws)).await.unwrap();
    assert_eq!(pulls[&ws][0].pulled_at, None);

    // 2. If a subsequent partial or complete delivery fails mid-transaction (e.g. practice mismatch
    // or conflicting revision bytes under the same candidate/revision identity), NEITHER records,
    // `source_pull`, NOR `patient_source_status` commits!
    let mut conflicting_rev = valid_rev.clone();
    conflicting_rev.content =
        r#"{"resourceType":"Observation","id":"obs-1","text":"TAMPERED"}"#.into();
    conflicting_rev.sha256 = sha256_hex(conflicting_rev.content.as_bytes());
    // Keep the old `revision` string so `apply_source_delivery_tx` rejects conflicting revision identity!

    let err = store
        .record_complete_pull(CompleteDelivery {
            practice_id: &practice,
            workspace_id: &ws,
            source_id: &source_id,
            source_label: "Atomic EHR",
            patient: &pat,
            records: &[conflicting_rev],
            pulled_at: 2_000,
            ticket: None,
        })
        .await
        .unwrap_err();
    assert!(
        err.to_string()
            .contains("conflicting source revision identity"),
        "{err}"
    );

    // Verify complete rollback: `pulled_at` is STILL `None` and status is STILL `Partial` at 1_200!
    let pulls_after = store.source_pulls(std::slice::from_ref(&ws)).await.unwrap();
    assert_eq!(pulls_after[&ws][0].pulled_at, None);
    let statuses_after = store.patient_source_statuses(&practice, &ws).await.unwrap();
    assert_eq!(statuses_after[0].last_good_read_at, None);
    assert_eq!(
        statuses_after[0]
            .latest_attempt
            .as_ref()
            .unwrap()
            .attempted_at,
        1_200
    );

    // Cross-practice attempt also rolls back cleanly without altering records or status.
    let mismatch = store
        .record_partial_pull(PartialDelivery {
            practice_id: &other_practice,
            workspace_id: &ws,
            source_id: &source_id,
            source_label: "Atomic EHR",
            patient: Some(&pat),
            succeeded_kinds: &[SourceRecordKind {
                kind: "Observation".into(),
                category: None,
            }],
            records: &[],
            failed_parts: &[],
            attempted_at: 3_000,
            ticket: None,
        })
        .await
        .unwrap_err();
    assert!(matches!(
        mismatch,
        SourceStatusError::PracticeMismatch { .. }
    ));
}

struct LeaseControlledSource {
    source_id: String,
    reconnect_effects: std::sync::atomic::AtomicUsize,
    pull_calls: std::sync::atomic::AtomicUsize,
    reconnect_sleep_ms: std::sync::atomic::AtomicU64,
    pull_sleep_ms: std::sync::atomic::AtomicU64,
    pause_on_pull: std::sync::atomic::AtomicBool,
    pull_entered: tokio::sync::Notify,
    pull_release: tokio::sync::Notify,
    cached_receipts: Mutex<HashMap<(String, String, String), ReconnectReceipt>>,
}

impl LeaseControlledSource {
    fn new(source_id: &str) -> Arc<Self> {
        Arc::new(Self {
            source_id: source_id.to_owned(),
            reconnect_effects: std::sync::atomic::AtomicUsize::new(0),
            pull_calls: std::sync::atomic::AtomicUsize::new(0),
            reconnect_sleep_ms: std::sync::atomic::AtomicU64::new(0),
            pull_sleep_ms: std::sync::atomic::AtomicU64::new(0),
            pause_on_pull: std::sync::atomic::AtomicBool::new(false),
            pull_entered: tokio::sync::Notify::new(),
            pull_release: tokio::sync::Notify::new(),
            cached_receipts: Mutex::new(HashMap::new()),
        })
    }
}

#[async_trait]
impl PatientSource for LeaseControlledSource {
    fn source_id(&self) -> &str {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        "Lease Controlled EHR"
    }

    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        Ok(vec!["pat-1".to_owned(), "pat-2".to_owned()])
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, SourceAdapterError> {
        Ok(make_patient(source_patient_id, "Elena Morales"))
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        _workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        Ok(Vec::new())
    }

    async fn pull_patient(
        &self,
        _practice_id: &str,
        source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        use std::sync::atomic::Ordering::SeqCst;

        self.pull_calls.fetch_add(1, SeqCst);
        if self.pause_on_pull.load(SeqCst) {
            let release = self.pull_release.notified();
            self.pull_entered.notify_waiters();
            release.await;
        }
        let sleep_ms = self.pull_sleep_ms.load(SeqCst);
        if sleep_ms > 0 {
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
        }
        SourcePullOutput::Complete {
            patient: make_patient(source_patient_id, "Elena Morales"),
            records: vec![make_revision(
                workspace,
                &self.source_id,
                "Patient",
                source_patient_id,
                "Elena Morales",
            )],
        }
    }

    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        use std::sync::atomic::Ordering::SeqCst;

        if let Some(key) = command_key
            && let Some(cached) = self
                .cached_receipts
                .lock()
                .unwrap()
                .get(&key.scoped_key(practice_id))
                .cloned()
        {
            return Ok(cached);
        }
        let sleep_ms = self.reconnect_sleep_ms.load(SeqCst);
        if sleep_ms > 0 {
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
        }
        self.reconnect_effects.fetch_add(1, SeqCst);
        let receipt = ReconnectReceipt {
            source_id: self.source_id.clone(),
            practice_id: practice_id.to_owned(),
            reconnected_at: now,
        };
        if let Some(key) = command_key {
            self.cached_receipts
                .lock()
                .unwrap()
                .insert(key.scoped_key(practice_id), receipt.clone());
        }
        Ok(receipt)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn active_slow_reconnect_and_catch_up_renew_lease_while_superseded_fence_cannot_write() {
    use std::sync::atomic::Ordering::SeqCst;

    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-lease-{run}");
    let physician = format!("dr-lease-{run}");
    let source_id = format!("ehr-lease-{run}");
    let source = LeaseControlledSource::new(&source_id);

    let ws1 = format!("pw-lease-1-{run}");
    let ws2 = format!("pw-lease-2-{run}");
    for (pat_id, ws) in [("pat-1", &ws1), ("pat-2", &ws2)] {
        SourceCoordinator::check_patient(
            &store,
            &*source,
            &practice,
            pat_id,
            LinkTarget::Exact(ws),
            1_000,
        )
        .await
        .unwrap();
    }
    assert_eq!(source.pull_calls.load(SeqCst), 2);

    // Part A: Active slow reconnect (1.3s) + two slow sequential patient pulls (2 * 1.3s = 2.6s)
    // runs ~3.9s total — nearly 2x the initial 2s lease TTL. Heartbeat renewal every 250ms keeps
    // caller 1's lease alive so a duplicate caller arriving at 2.3s waits rather than stealing the
    // lease, executing 1 reconnect effect and 2 patient pulls total.
    source.reconnect_sleep_ms.store(1_300, SeqCst);
    source.pull_sleep_ms.store(1_300, SeqCst);
    let active_policy = ReconnectLeasePolicy {
        lease_seconds: 2,
        renew_interval: Duration::from_millis(250),
    };
    let slow_key = format!("key-slow-active-{run}");

    let store_c1 = store.clone();
    let practice_c1 = practice.clone();
    let physician_c1 = physician.clone();
    let key_c1 = slow_key.clone();
    let source_id_c1 = source_id.clone();
    let source_c1 = Arc::clone(&source);
    let caller_1 = tokio::spawn(async move {
        SourceCoordinator::reconnect_source_command_with_lease(
            &store_c1,
            ReconnectCommand {
                practice_id: &practice_c1,
                physician_id: &physician_c1,
                idempotency_key: &key_c1,
                source_id: &source_id_c1,
                source: Some(&*source_c1),
                now: 5_000,
            },
            active_policy,
        )
        .await
    });

    tokio::time::sleep(Duration::from_millis(2_300)).await;
    let caller_2 = SourceCoordinator::reconnect_source_command_with_lease(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: &slow_key,
            source_id: &source_id,
            source: Some(&*source),
            now: 5_500,
        },
        active_policy,
    )
    .await
    .unwrap();
    let caller_1 = caller_1.await.unwrap().unwrap();

    assert_eq!(caller_1, caller_2);
    assert_eq!(
        source.reconnect_effects.load(SeqCst),
        1,
        "heartbeat renewal must prevent duplicate reconnect effects past initial lease TTL"
    );
    assert_eq!(
        source.pull_calls.load(SeqCst),
        4,
        "each of the 2 linked patients must be pulled once during slow catch-up (2 baseline + 2 catch-up)"
    );
    assert!(matches!(
        &caller_1,
        ReconnectCommandOutcome::Accepted { receipt, .. } if receipt.reconnected_at == 5_000
    ));

    // Part B: Stalled runner loses its lease after TTL expires without heartbeat renewal; runner B
    // claims the expired lease, finishes catch-up at `now = 9_000`, and marks the command Completed.
    // When stalled runner A wakes up with stale `now = 8_000`, write fencing rejects its delivery
    // with `CommandLeaseLost` and leaves patient records/statuses at `9_000`.
    source.reconnect_sleep_ms.store(0, SeqCst);
    source.pull_sleep_ms.store(0, SeqCst);
    source.pause_on_pull.store(true, SeqCst);

    let stalled_key = format!("key-stalled-fence-{run}");
    let stalled_policy = ReconnectLeasePolicy {
        lease_seconds: 2,
        renew_interval: Duration::from_secs(3_600),
    };
    let entered = source.pull_entered.notified();
    let store_stalled = store.clone();
    let practice_stalled = practice.clone();
    let physician_stalled = physician.clone();
    let key_stalled = stalled_key.clone();
    let source_id_stalled = source_id.clone();
    let source_stalled = Arc::clone(&source);

    let runner_a = tokio::spawn(async move {
        SourceCoordinator::reconnect_source_command_with_lease(
            &store_stalled,
            ReconnectCommand {
                practice_id: &practice_stalled,
                physician_id: &physician_stalled,
                idempotency_key: &key_stalled,
                source_id: &source_id_stalled,
                source: Some(&*source_stalled),
                now: 8_000,
            },
            stalled_policy,
        )
        .await
    });

    entered.await;
    let effects_after_a = source.reconnect_effects.load(SeqCst);
    assert_eq!(effects_after_a, 2);

    // Wait past the 2s integer-second lease window so runner A's unrenewed lease expires in Postgres.
    tokio::time::sleep(Duration::from_millis(2_500)).await;
    source.pause_on_pull.store(false, SeqCst);

    let runner_b_out = SourceCoordinator::reconnect_source_command_with_lease(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: &stalled_key,
            source_id: &source_id,
            source: Some(&*source),
            now: 9_000,
        },
        ReconnectLeasePolicy {
            lease_seconds: 10,
            renew_interval: Duration::from_millis(250),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        source.reconnect_effects.load(SeqCst),
        effects_after_a,
        "runner B claiming an expired Reconnected command must not re-run source.reconnect()"
    );
    assert!(matches!(
        &runner_b_out,
        ReconnectCommandOutcome::Accepted { receipt, .. } if receipt.reconnected_at == 8_000
    ));

    // Release stalled runner A so it tries to commit its stale `now = 8_000` pull output.
    source.pull_release.notify_waiters();
    let runner_a_err = runner_a.await.unwrap().unwrap_err();
    assert!(
        matches!(runner_a_err, SourceStatusError::CommandLeaseLost { .. }),
        "superseded runner A must fail closed with CommandLeaseLost, got {runner_a_err:?}"
    );

    // Verify neither patient status reverted from `9_000` to stale `8_000`, and a direct fenced
    // check/link resolution using a stale lease token is also fenced before touching the adapter.
    for ws in [&ws1, &ws2] {
        let statuses = store.patient_source_statuses(&practice, ws).await.unwrap();
        assert_eq!(statuses[0].last_good_read_at, Some(9_000));
    }

    let pulls_before_stale = source.pull_calls.load(SeqCst);
    let stale_fence = ReconnectCommandFence {
        practice_id: &practice,
        physician_id: &physician,
        idempotency_key: &stalled_key,
        lease_owner: "stale-superseded-owner",
        lease_seconds: 10,
    };
    let ws3 = format!("pw-lease-3-{run}");
    let stale_link_err = SourceCoordinator::check_patient_fenced(
        &store,
        &*source,
        &practice,
        "pat-3",
        LinkTarget::Exact(&ws3),
        7_000,
        Some(stale_fence),
    )
    .await
    .unwrap_err();
    assert!(matches!(
        stale_link_err,
        SourceStatusError::CommandLeaseLost { .. }
    ));
    assert_eq!(
        source.pull_calls.load(SeqCst),
        pulls_before_stale,
        "fenced patient link check must fail before pulling from the adapter"
    );
    assert!(
        store
            .source_patient_link(&SourcePatientKey {
                practice_id: practice.clone(),
                source_id: source_id.clone(),
                source_patient_id: "pat-3".to_owned(),
            })
            .await
            .unwrap()
            .is_none(),
        "fenced link resolution must write zero patient link rows"
    );
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct PersistedPreCheckpointLedger {
    underlying_effects: usize,
    receipts: Vec<((String, String, String), ReconnectReceipt)>,
}

struct PreCheckpointSource {
    source_id: String,
    store: Store,
    hang_after_effect: std::sync::atomic::AtomicBool,
    effect_committed: tokio::sync::Notify,
}

impl PreCheckpointSource {
    fn open(store: Store, source_id: &str) -> Arc<Self> {
        Arc::new(Self {
            source_id: source_id.to_owned(),
            store,
            hang_after_effect: std::sync::atomic::AtomicBool::new(false),
            effect_committed: tokio::sync::Notify::new(),
        })
    }

    async fn underlying_effect_count(&self, practice_id: &str) -> usize {
        self.store
            .read_source_adapter_state::<PersistedPreCheckpointLedger>(practice_id, &self.source_id)
            .await
            .unwrap()
            .underlying_effects
    }
}

#[async_trait]
impl PatientSource for PreCheckpointSource {
    fn source_id(&self) -> &str {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        "Pre-Checkpoint EHR"
    }

    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        Ok(vec!["pat-1".to_owned()])
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, SourceAdapterError> {
        Ok(make_patient(source_patient_id, "Elena Morales"))
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        _workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        Ok(Vec::new())
    }

    async fn pull_patient(
        &self,
        _practice_id: &str,
        source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        SourcePullOutput::Complete {
            patient: make_patient(source_patient_id, "Elena Morales"),
            records: vec![make_revision(
                workspace,
                &self.source_id,
                "Patient",
                source_patient_id,
                "Elena Morales",
            )],
        }
    }

    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        use std::sync::atomic::Ordering::SeqCst;

        let (receipt, was_cached) = self
            .store
            .mutate_source_adapter_state(
                practice_id,
                &self.source_id,
                |ledger: &mut PersistedPreCheckpointLedger| {
                    if let Some(key) = command_key {
                        let scoped = key.scoped_key(practice_id);
                        if let Some((_, cached)) =
                            ledger.receipts.iter().find(|(k, _)| k == &scoped)
                        {
                            return Ok((cached.clone(), true));
                        }
                    }

                    ledger.underlying_effects += 1;
                    let receipt = ReconnectReceipt {
                        source_id: self.source_id.clone(),
                        practice_id: practice_id.to_owned(),
                        reconnected_at: now,
                    };
                    if let Some(key) = command_key {
                        ledger
                            .receipts
                            .push((key.scoped_key(practice_id), receipt.clone()));
                    }
                    Ok((receipt, false))
                },
            )
            .await?;

        if !was_cached {
            self.effect_committed.notify_waiters();
            if self.hang_after_effect.load(SeqCst) {
                std::future::pending::<()>().await;
            }
        }
        Ok(receipt)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn interrupted_reconnect_before_db_checkpoint_recovers_cached_adapter_receipt_without_second_effect()
 {
    use std::sync::atomic::Ordering::SeqCst;

    let store = migrated_store().await;
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-prechk-{run}");
    let physician_a = format!("dr-a-{run}");
    let physician_b = format!("dr-b-{run}");
    let source_id = format!("ehr-prechk-{run}");
    let instance_a = PreCheckpointSource::open(store.clone(), &source_id);

    let ws = format!("pw-prechk-{run}");
    SourceCoordinator::check_patient(
        &store,
        &*instance_a,
        &practice,
        "pat-1",
        LinkTarget::Exact(&ws),
        1_000,
    )
    .await
    .unwrap();

    // Mark access as lapsed at t = 6_500 before issuing the reconnect command.
    store
        .set_practice_source_access(
            &practice,
            &source_id,
            &SourceAccessState::ReconnectNeeded {
                since: 6_500,
                reason: "refresh token expired".into(),
            },
            6_500,
        )
        .await
        .unwrap();

    instance_a.hang_after_effect.store(true, SeqCst);
    let key = format!("key-prechk-{run}");
    let committed = instance_a.effect_committed.notified();

    let store_bg = store.clone();
    let practice_bg = practice.clone();
    let physician_bg = physician_a.clone();
    let key_bg = key.clone();
    let source_id_bg = source_id.clone();
    let source_bg = Arc::clone(&instance_a);

    let attempt_1 = tokio::spawn(async move {
        SourceCoordinator::reconnect_source_command(
            &store_bg,
            ReconnectCommand {
                practice_id: &practice_bg,
                physician_id: &physician_bg,
                idempotency_key: &key_bg,
                source_id: &source_id_bg,
                source: Some(&*source_bg),
                now: 7_000,
            },
        )
        .await
    });

    // Wait until `source.reconnect(..., Some(key))` has committed its underlying effect and
    // receipt in Postgres (`physician_runtime_source_adapter_state`), then abort caller 1 BEFORE
    // `store.record_reconnect_command_reconnected` runs (DB command row stays `Claimed`) and drop
    // `instance_a` completely so retry must recover from shared Postgres state on a freshly
    // constructed adapter instance with zero shared filesystem.
    committed.await;
    assert_eq!(instance_a.underlying_effect_count(&practice).await, 1);
    attempt_1.abort();
    let _ = attempt_1.await;
    drop(instance_a);

    // Access in Postgres was NOT updated yet because the DB checkpoint never ran!
    let report_before_retry = store
        .practice_source_report(&practice, &source_id)
        .await
        .unwrap();
    assert!(matches!(
        report_before_retry.access,
        Some(SourceAccessState::ReconnectNeeded { since: 6_500, .. })
    ));

    // Construct a brand-new adapter instance sharing ONLY Postgres (`store.clone()`) and retry
    // with the same `(practice, physician_a, key)` at `now = 7_500`: adapter recovers the cached
    // receipt (`reconnected_at == 7_000`) without repeating the effect, checkpoints `Reconnected`
    // + `Completed`, and restores practice access at `7_000`.
    let instance_b = PreCheckpointSource::open(store.clone(), &source_id);
    let recovered = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician_a,
            idempotency_key: &key,
            source_id: &source_id,
            source: Some(&*instance_b),
            now: 7_500,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        instance_b.underlying_effect_count(&practice).await,
        1,
        "retry on freshly constructed adapter instance after pre-checkpoint interruption must recover cached receipt from Postgres without a second effect"
    );
    assert!(matches!(
        &recovered,
        ReconnectCommandOutcome::Accepted {
            receipt,
            access: SourceAccessState::Connected { connected_at: 7_000 },
            ..
        } if receipt.reconnected_at == 7_000
    ));

    // A different physician in the same practice reusing the same `idempotency_key` string after a
    // later lapse is scoped independently (`(practice_id, physician_id, idempotency_key)`), so it
    // executes a fresh reconnect effect at `8_500`.
    let second_physician_out = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician_b,
            idempotency_key: &key,
            source_id: &source_id,
            source: Some(&*instance_b),
            now: 8_500,
        },
    )
    .await
    .unwrap();
    assert_eq!(instance_b.underlying_effect_count(&practice).await, 2);
    assert!(matches!(
        &second_physician_out,
        ReconnectCommandOutcome::Accepted { receipt, .. } if receipt.reconnected_at == 8_500
    ));
}

struct BarrierHold {
    entered: Arc<tokio::sync::Notify>,
    release: Arc<tokio::sync::Notify>,
}

type PullProducer = Box<dyn Fn(&str) -> SourcePullOutput + Send + Sync>;
type PlannedPull = (PullProducer, Option<BarrierHold>);

struct InterleavingPullSource {
    source_id: String,
    calls: std::sync::atomic::AtomicUsize,
    plans: Mutex<HashMap<usize, PlannedPull>>,
    default_plan: Mutex<PullProducer>,
}

impl InterleavingPullSource {
    fn new(
        source_id: &str,
        default_plan: impl Fn(&str) -> SourcePullOutput + Send + Sync + 'static,
    ) -> Arc<Self> {
        Arc::new(Self {
            source_id: source_id.to_owned(),
            calls: std::sync::atomic::AtomicUsize::new(0),
            plans: Mutex::new(HashMap::new()),
            default_plan: Mutex::new(Box::new(default_plan)),
        })
    }

    fn set_default_plan(&self, plan: impl Fn(&str) -> SourcePullOutput + Send + Sync + 'static) {
        *self.default_plan.lock().unwrap() = Box::new(plan);
    }

    fn hold_call(
        &self,
        call_index: usize,
        plan: impl Fn(&str) -> SourcePullOutput + Send + Sync + 'static,
    ) -> (Arc<tokio::sync::Notify>, Arc<tokio::sync::Notify>) {
        let entered = Arc::new(tokio::sync::Notify::new());
        let release = Arc::new(tokio::sync::Notify::new());
        self.plans.lock().unwrap().insert(
            call_index,
            (
                Box::new(plan),
                Some(BarrierHold {
                    entered: Arc::clone(&entered),
                    release: Arc::clone(&release),
                }),
            ),
        );
        (entered, release)
    }
}

#[async_trait]
impl PatientSource for InterleavingPullSource {
    fn source_id(&self) -> &str {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        "Interleaving EHR"
    }

    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        Ok(vec!["pat-order-1".to_owned()])
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, SourceAdapterError> {
        Ok(make_patient(source_patient_id, "Order Patient"))
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        _workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        Ok(Vec::new())
    }

    async fn pull_patient(
        &self,
        _practice_id: &str,
        _source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        let idx = self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        let planned = self.plans.lock().unwrap().remove(&idx);
        match planned {
            Some((produce, Some(hold))) => {
                let out = produce(workspace);
                hold.entered.notify_waiters();
                hold.release.notified().await;
                out
            }
            Some((produce, None)) => produce(workspace),
            None => (self.default_plan.lock().unwrap())(workspace),
        }
    }

    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        _command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        Ok(ReconnectReceipt {
            source_id: self.source_id.clone(),
            practice_id: practice_id.to_owned(),
            reconnected_at: now,
        })
    }
}

#[tokio::test]
async fn late_lapsed_check_held_across_reconnect_command_does_not_overwrite_recovery() {
    let store = migrated_store().await;
    let suffix = Uuid::new_v4().simple().to_string();
    let practice = format!("practice-order-lapsed-{suffix}");
    let physician = format!("dr-order-lapsed-{suffix}");
    let source_id = format!("ehr-order-lapsed-{suffix}");
    let spid = "pat-order-1";
    let ws = workspace_id(&source_id, spid);

    let sid_initial = source_id.clone();
    let source =
        InterleavingPullSource::new(&source_id, move |workspace| SourcePullOutput::Complete {
            patient: make_patient(spid, "Order Patient"),
            records: vec![make_revision(
                workspace,
                &sid_initial,
                "Observation",
                "obs-1",
                "Initial A1c 6.4",
            )],
        });

    // Call #1: baseline complete pull at t = 100.
    let initial = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        spid,
        LinkTarget::ExistingOrDefault(&ws),
        100,
    )
    .await
    .unwrap();
    assert_eq!(initial.applied, PullDisposition::Applied);
    assert!(matches!(
        initial.standing,
        SourceStanding::UpToDate {
            last_good_read_at: 100,
            ..
        }
    ));

    // Hold Call #2 (ordinary Check A): observes LapsedAccess at t = 150 and pauses before DB commit.
    let (check_a_entered, check_a_release) =
        source.hold_call(2, |_workspace| SourcePullOutput::LapsedAccess {
            error: SourceAdapterError::LapsedAccess {
                reason: "stale refresh token before reconnect".into(),
            },
        });
    let entered_wait = check_a_entered.notified();

    let store_a = store.clone();
    let source_a = Arc::clone(&source);
    let practice_a = practice.clone();
    let ws_a = ws.clone();
    let check_a_task = tokio::spawn(async move {
        SourceCoordinator::check_patient(
            &store_a,
            &*source_a,
            &practice_a,
            spid,
            LinkTarget::Exact(&ws_a),
            150,
        )
        .await
    });

    entered_wait.await;

    // While Check A is paused holding LapsedAccess, `reconnect_source_command` B restores access
    // and commits a fresh Complete pull (Call #3) at t = 300.
    let sid_recovered = source_id.clone();
    source.set_default_plan(move |workspace| SourcePullOutput::Complete {
        patient: make_patient(spid, "Order Patient"),
        records: vec![make_revision(
            workspace,
            &sid_recovered,
            "Observation",
            "obs-1",
            "Recovered A1c 5.9",
        )],
    });

    let reconnect_out = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: "idem-order-recover",
            source_id: &source_id,
            source: Some(&*source),
            now: 300,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        reconnect_out,
        ReconnectCommandOutcome::Accepted {
            summary: PracticeSourceSummary::Healthy { .. },
            ..
        }
    ));

    // Release delayed Check A: its stale LapsedAccess must be marked Superseded and leave B's recovery intact.
    check_a_release.notify_waiters();
    let report_a = check_a_task.await.unwrap().unwrap();
    assert_eq!(report_a.applied, PullDisposition::Superseded);
    assert!(matches!(
        report_a.standing,
        SourceStanding::UpToDate {
            last_good_read_at: 300,
            ..
        }
    ));

    let statuses = store.patient_source_statuses(&practice, &ws).await.unwrap();
    assert_eq!(statuses[0].last_good_read_at, Some(300));
    assert!(matches!(
        statuses[0].latest_attempt.as_ref().unwrap(),
        PullAttempt {
            attempted_at: 300,
            outcome: PullOutcome::Succeeded { .. },
        }
    ));
    let access = store
        .practice_source_access(&practice, &source_id)
        .await
        .unwrap();
    assert_eq!(
        access,
        Some(SourceAccessState::Connected { connected_at: 300 })
    );
    let practice_report = store
        .practice_source_report(&practice, &source_id)
        .await
        .unwrap();
    assert!(matches!(
        practice_report.summary,
        PracticeSourceSummary::Healthy { .. }
    ));
}

#[tokio::test]
async fn older_complete_pulls_held_across_newer_check_and_distinct_key_reconnect_do_not_regress_chart_or_timestamps()
 {
    let store = migrated_store().await;
    let suffix = Uuid::new_v4().simple().to_string();
    let practice = format!("practice-order-complete-{suffix}");
    let physician = format!("dr-order-complete-{suffix}");
    let source_id = format!("ehr-order-complete-{suffix}");
    let spid = "pat-order-complete";
    let ws = workspace_id(&source_id, spid);

    let sid_base = source_id.clone();
    let source =
        InterleavingPullSource::new(&source_id, move |workspace| SourcePullOutput::Complete {
            patient: make_patient(spid, "Complete Order Patient"),
            records: vec![make_revision(
                workspace,
                &sid_base,
                "Observation",
                "obs-order",
                "v0 baseline",
            )],
        });

    // Call #1: seed link + v0 chart at t = 100.
    SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        spid,
        LinkTarget::ExistingOrDefault(&ws),
        100,
    )
    .await
    .unwrap();

    // Part 1: Hold ordinary Check A (Call #2) delivering v1 at t = 200 while newer Check B (Call #3)
    // commits v2 at t = 300.
    let sid_v1 = source_id.clone();
    let (hold_a_entered, hold_a_release) =
        source.hold_call(2, move |workspace| SourcePullOutput::Complete {
            patient: make_patient(spid, "Complete Order Patient v1"),
            records: vec![make_revision(
                workspace,
                &sid_v1,
                "Observation",
                "obs-order",
                "v1 STALE CHART",
            )],
        });
    let entered_a = hold_a_entered.notified();

    let store_a = store.clone();
    let source_a = Arc::clone(&source);
    let practice_a = practice.clone();
    let ws_a = ws.clone();
    let task_a = tokio::spawn(async move {
        SourceCoordinator::check_patient(
            &store_a,
            &*source_a,
            &practice_a,
            spid,
            LinkTarget::Exact(&ws_a),
            200,
        )
        .await
    });
    entered_a.await;

    let sid_v2 = source_id.clone();
    source.set_default_plan(move |workspace| SourcePullOutput::Complete {
        patient: make_patient(spid, "Complete Order Patient v2"),
        records: vec![make_revision(
            workspace,
            &sid_v2,
            "Observation",
            "obs-order",
            "v2 AUTHORITATIVE CHART",
        )],
    });
    let report_b = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        spid,
        LinkTarget::Exact(&ws),
        300,
    )
    .await
    .unwrap();
    assert_eq!(report_b.applied, PullDisposition::Applied);

    hold_a_release.notify_waiters();
    let report_a = task_a.await.unwrap().unwrap();
    assert_eq!(report_a.applied, PullDisposition::Superseded);
    assert_eq!(report_a.changed, 0);

    let current_sources = store.sources(&ws).await.unwrap().sources;
    let obs_rev = current_sources
        .iter()
        .find(|rev| rev.candidate_id == "Observation/obs-order")
        .expect("Observation/obs-order stored");
    assert!(
        obs_rev.content.contains("v2 AUTHORITATIVE CHART"),
        "older complete check must not overwrite v2 chart records"
    );
    let pulls = store.source_pulls(std::slice::from_ref(&ws)).await.unwrap();
    assert_eq!(pulls[&ws][0].pulled_at, Some(300));
    assert_eq!(
        pulls[&ws][0].patient.display_name,
        "Complete Order Patient v2"
    );
    let statuses = store.patient_source_statuses(&practice, &ws).await.unwrap();
    assert_eq!(statuses[0].last_good_read_at, Some(300));
    assert_eq!(
        statuses[0].latest_attempt.as_ref().unwrap().attempted_at,
        300
    );

    // Part 2: Hold distinct-key `reconnect_source_command` ("idem-old", Call #4) delivering v3-stale
    // at t = 400 while newer distinct-key `reconnect_source_command` ("idem-new", Call #5) commits
    // v4-newest at t = 500.
    let sid_v3 = source_id.clone();
    let (hold_old_entered, hold_old_release) =
        source.hold_call(4, move |workspace| SourcePullOutput::Complete {
            patient: make_patient(spid, "Complete Order Patient v3"),
            records: vec![make_revision(
                workspace,
                &sid_v3,
                "Observation",
                "obs-order",
                "v3 STALE RECONNECT",
            )],
        });
    let entered_old = hold_old_entered.notified();

    let store_old = store.clone();
    let source_old = Arc::clone(&source);
    let practice_old = practice.clone();
    let physician_old = physician.clone();
    let sid_cmd_old = source_id.clone();
    let task_old = tokio::spawn(async move {
        SourceCoordinator::reconnect_source_command(
            &store_old,
            ReconnectCommand {
                practice_id: &practice_old,
                physician_id: &physician_old,
                idempotency_key: "idem-distinct-old",
                source_id: &sid_cmd_old,
                source: Some(&*source_old),
                now: 400,
            },
        )
        .await
    });
    entered_old.await;

    let sid_v4 = source_id.clone();
    source.set_default_plan(move |workspace| SourcePullOutput::Complete {
        patient: make_patient(spid, "Complete Order Patient v4"),
        records: vec![make_revision(
            workspace,
            &sid_v4,
            "Observation",
            "obs-order",
            "v4 NEWEST RECONNECT",
        )],
    });
    let out_new = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: &physician,
            idempotency_key: "idem-distinct-new",
            source_id: &source_id,
            source: Some(&*source),
            now: 500,
        },
    )
    .await
    .unwrap();
    assert!(matches!(out_new, ReconnectCommandOutcome::Accepted { .. }));

    hold_old_release.notify_waiters();
    let out_old = task_old.await.unwrap().unwrap();
    assert!(matches!(out_old, ReconnectCommandOutcome::Accepted { .. }));

    let final_sources = store.sources(&ws).await.unwrap().sources;
    let final_obs_rev = final_sources
        .iter()
        .find(|rev| rev.candidate_id == "Observation/obs-order")
        .expect("Observation/obs-order stored");
    assert!(
        final_obs_rev.content.contains("v4 NEWEST RECONNECT"),
        "older distinct-key reconnect must not overwrite v4 chart records"
    );
    let final_pulls = store.source_pulls(std::slice::from_ref(&ws)).await.unwrap();
    assert_eq!(final_pulls[&ws][0].pulled_at, Some(500));
    assert_eq!(
        final_pulls[&ws][0].patient.display_name,
        "Complete Order Patient v4"
    );
    let final_statuses = store.patient_source_statuses(&practice, &ws).await.unwrap();
    assert_eq!(final_statuses[0].last_good_read_at, Some(500));
    assert_eq!(
        final_statuses[0]
            .latest_attempt
            .as_ref()
            .unwrap()
            .attempted_at,
        500
    );
}

#[tokio::test]
async fn pre_reconnect_lapsed_pull_held_across_practice_access_restore_is_superseded_before_catch_up()
 {
    let store = migrated_store().await;
    let suffix = Uuid::new_v4().simple().to_string();
    let practice = format!("practice-access-seq-{suffix}");
    let source_id = format!("ehr-access-seq-{suffix}");
    let spid = "pat-access-seq";
    let ws = workspace_id(&source_id, spid);

    let sid_init = source_id.clone();
    let source =
        InterleavingPullSource::new(&source_id, move |workspace| SourcePullOutput::Complete {
            patient: make_patient(spid, "Access Seq Patient"),
            records: vec![make_revision(
                workspace,
                &sid_init,
                "Observation",
                "obs-seq",
                "Baseline normal",
            )],
        });

    // Call #1: baseline complete pull at t = 100.
    SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        spid,
        LinkTarget::ExistingOrDefault(&ws),
        100,
    )
    .await
    .unwrap();

    // Hold Call #2: Check A starts BEFORE reconnect (`access_seq == 0`), observes LapsedAccess at t = 150,
    // and pauses before writing.
    let (entered_a, release_a) = source.hold_call(2, |_workspace| SourcePullOutput::LapsedAccess {
        error: SourceAdapterError::LapsedAccess {
            reason: "pre-reconnect expired grant".into(),
        },
    });
    let wait_entered = entered_a.notified();

    let store_a = store.clone();
    let source_a = Arc::clone(&source);
    let practice_a = practice.clone();
    let ws_a = ws.clone();
    let task_a = tokio::spawn(async move {
        SourceCoordinator::check_patient(
            &store_a,
            &*source_a,
            &practice_a,
            spid,
            LinkTarget::Exact(&ws_a),
            150,
        )
        .await
    });
    wait_entered.await;

    // Practice restores `Connected` at t = 200 BEFORE any patient catch-up pull has run for `ws`
    // (`applied_pull_seq` is still 1 < `ticket.seq` 2, while `access_seq` advances to 1!).
    store
        .set_practice_source_access(
            &practice,
            &source_id,
            &SourceAccessState::Connected { connected_at: 200 },
            200,
        )
        .await
        .unwrap();

    release_a.notify_waiters();
    let report_a = task_a.await.unwrap().unwrap();
    assert_eq!(report_a.applied, PullDisposition::Superseded);
    assert!(matches!(
        report_a.standing,
        SourceStanding::UpToDate {
            last_good_read_at: 100,
            ..
        }
    ));

    let statuses = store.patient_source_statuses(&practice, &ws).await.unwrap();
    assert_eq!(statuses[0].last_good_read_at, Some(100));
    assert!(matches!(
        statuses[0].latest_attempt.as_ref().unwrap(),
        PullAttempt {
            attempted_at: 100,
            outcome: PullOutcome::Succeeded { .. },
        }
    ));
}

#[tokio::test]
async fn overdue_scheduled_fetch_reports_couldn_t_update_standing() {
    use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};

    let store = migrated_store().await;
    let auth = PostgresAuthStore::connect_with_migrations(&test_database_url())
        .await
        .expect("migrate auth store");
    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-overdue-{run}");
    let source_id = format!("ehr-overdue-{run}");
    let ws = format!("pw-overdue-{run}");
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let source = ScriptedSource::new(
        &source_id,
        vec![ScriptedPull::Complete {
            name: "Elena Vance".into(),
            records: vec![("Patient".into(), "pat-overdue".into(), "Elena Vance".into())],
        }],
    );
    store
        .set_practice_source_access(
            &practice,
            &source_id,
            &SourceAccessState::Connected {
                connected_at: 1_000,
            },
            1_000,
        )
        .await
        .unwrap();
    let spid = format!("src-overdue-{run}");
    let report = SourceCoordinator::check_patient(
        &store,
        &*source,
        &practice,
        &spid,
        LinkTarget::ExistingOrDefault(&ws),
        1_000,
    )
    .await
    .unwrap();
    assert_eq!(report.workspace_id, ws);

    auth.register_patient(&practice, &ws).await.unwrap();
    store
        .register_patient_dispatch(&auth, &practice, &ws, 1_000)
        .await
        .unwrap();

    // Daily cadence check_every is 86_400s from last_good_read_at = 1_000 (due at 87_400),
    // which is long past current wall clock now -> missed scheduled fetch must NOT read UpToDate.
    let standings = store
        .workspace_source_standings(std::slice::from_ref(&ws))
        .await
        .unwrap();
    let standing = &standings[&ws][&source_id];
    assert!(
        matches!(
            standing,
            SourceStanding::Overdue {
                last_good_read_at: Some(1_000),
                scheduled_at: 87_400,
            }
        ),
        "overdue scheduled fetch must report couldn't update / overdue with last_good_read_at and scheduled_at, got: {standing:?}"
    );
}
