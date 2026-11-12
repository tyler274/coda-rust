//! Acceptance proof for practice-scoped source patient links, pull status
//! preservation, and practice-wide outage summarization against real Postgres.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use local_product::physician_sources::{
    FailedPart, MedicalRecordNumber, PatientSource, PullOutcome, ReconnectReceipt, RecordDate,
    SourceAdapterError, SourcePatient, SourcePullOutput, SourceRecordKind, SourceRevision,
    workspace_id,
};
use local_product::runtime::Store;
use local_product::sha256_hex;
use local_product::source_status::{
    CompleteDelivery, LinkTarget, PartialDelivery, PatientSourceRecordStatus, PracticeOutage,
    PracticeOutageKind, PracticeSourceSummary, PullAttempt, ReconnectCommand,
    ReconnectCommandOutcome, SourceAccessState, SourceCoordinator, SourcePatientKey,
    SourceStanding, SourceStatusError, derive_standing, summarize_practice_source,
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
        })
        .await
        .unwrap_err();
    assert!(matches!(
        mismatch,
        SourceStatusError::PracticeMismatch { .. }
    ));
}
