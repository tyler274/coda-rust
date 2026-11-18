//! Real PostgreSQL and fixed-provider integration tests for on-demand patient
//! source refresh (`runtime::refresh`).
//!
//! Runs automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` or
//! `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use local_product::physician_chat::{PhysicianSkills, recipe as chat_recipe};
use local_product::physician_sources::{
    ChartRequest, ChartScope, MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate,
    SEARCH_TOOL, SourcePatient, SourceRevision,
};
use local_product::runtime::jobs::{
    Assignment, PracticeAccessError, PracticeAuthority, PracticeJobError, PracticeJobHost,
    PracticeJobKind, PracticeScope,
};
use local_product::runtime::refresh::{
    FixedCheckStep, FixedSourceCheck, REFRESH_TOOL, RefreshCommandKey, RefreshError, RefreshReport,
    RefreshRequest, RefreshTarget, SavedSourceStatus, SourceCheckOutcome, SourceId,
    SourceRefreshOutcome,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store, WorkStatus,
    async_trait, engine,
};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};
use serde_json::json;
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

#[test]
fn refresh_tool_and_job_recipe_have_clean_model_text() {
    let skills = load_skills();
    let roles = distinct_roles();
    let tool = local_product::runtime::refresh::refresh_tool();
    model_text::assert_clean_model_text(&tool.description);
    model_text::assert_clean_model_text(&serde_json::to_string(&tool.function()).unwrap());

    let assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "model-text-check",
        "Check fresh records and summarize clinically material findings.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    let built = local_product::runtime::jobs::recipe(&roles, &assignment, &skills);
    model_text::assert_clean_model_text(&built.instructions);
    model_text::assert_clean_model_text(&serde_json::to_string(&built.tools).unwrap());
}

fn database_url() -> Option<String> {
    if let Ok(url) = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
    {
        let trimmed = url.trim().to_owned();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    assert!(
        std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
        "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL or PHYSICIAN_DATABASE_URL"
    );
    None
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills catalog")
}

fn distinct_roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "interactive-chat-lane".into(),
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "fixture".into(),
        },
        background: ModelBinding {
            model: "background-worker-lane".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        },
    }
}

fn record_for(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    date: &str,
    content: &str,
) -> SourceRevision {
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "DiagnosticReport".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: date.to_owned(),
        }],
        content: content.to_owned(),
        sha256: sha256_hex(content.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Summary".to_owned(),
            text: content.to_owned(),
        }],
        provenance: json!({
            "system": source_id,
            "native_id": candidate_id,
        }),
        envelope: None,
    }
}

fn source_patient(id: &str, name: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: id.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some("1979-08-21".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "mrn".to_owned(),
            value: format!("MRN-{id}"),
        }],
    }
}

async fn connect_stores(url: &str) -> (Store, Arc<PostgresAuthStore>) {
    let store = Store::connect(url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    (store, auth)
}

async fn read_exact_chart(
    store: &Store,
    workspace: &str,
    candidate_id: &str,
    revision: &str,
) -> String {
    let chart = ChartRequest::parse(
        READ_TOOL,
        &json!({
            "references": [{
                "candidate_id": candidate_id,
                "revision": revision,
            }],
            "max_result_bytes": 16_384,
        }),
        ChartScope::Chat,
    )
    .expect("valid exact chart read");
    let observation = store
        .chart_observation(workspace, &chart)
        .await
        .expect("load chart observation");
    match chart
        .run(workspace, observation.version, &observation.sources)
        .expect("run exact chart read")
    {
        engine::ToolOutcome::Returned(payload) => payload,
        other => panic!("expected Returned chart read, got {other:?}"),
    }
}

fn materialized_output<'a>(request: &'a engine::recipe::Request, id: &str) -> &'a str {
    request
        .items
        .iter()
        .find_map(|item| match item {
            engine::Item::MaterializedResult { call_id, output } if call_id == id => {
                Some(output.as_str())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("missing materialized result for call_id `{id}`"))
}

#[tokio::test]
async fn ac1_refresh_tool_refreshes_right_patient_and_source_and_later_read_returns_new_versions() {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-refresh-ac1-{}", Uuid::new_v4().simple());
    let pw_a = format!("pw-a-{}", Uuid::new_v4().simple());
    let pw_b = format!("pw-b-{}", Uuid::new_v4().simple());

    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &pw_a).await.unwrap();
    auth.register_patient(&practice, &pw_b).await.unwrap();

    let ehr_id = SourceId::new("ehr").unwrap();
    let labs_id = SourceId::new("labs").unwrap();
    let imaging_id = SourceId::new("imaging").unwrap();

    // Seed initial rev-1 records in the real evidence store across both patients and sources.
    base_store
        .record_chart(
            &pw_a,
            &[record_for(
                &pw_a,
                "ehr",
                "a1c-panel",
                "rev-1",
                "2026-09-01",
                "Hemoglobin A1c 7.4% collected 2026-09-01.",
            )],
            &SourcePull {
                source_id: "ehr".into(),
                source_label: "Metro EHR".into(),
                patient: source_patient("pt-a", "Maya Lin"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();
    base_store
        .record_chart(
            &pw_a,
            &[record_for(
                &pw_a,
                "labs",
                "lipid-panel",
                "rev-1",
                "2026-09-01",
                "LDL cholesterol 132 mg/dL collected 2026-09-01.",
            )],
            &SourcePull {
                source_id: "labs".into(),
                source_label: "Quest Labs".into(),
                patient: source_patient("pt-a", "Maya Lin"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();
    base_store
        .record_chart(
            &pw_a,
            &[record_for(
                &pw_a,
                "imaging",
                "chest-xray-baseline",
                "rev-1",
                "2026-08-15",
                "Baseline chest radiograph clear on 2026-08-15.",
            )],
            &SourcePull {
                source_id: "imaging".into(),
                source_label: "Radiology Archive".into(),
                patient: source_patient("pt-a", "Maya Lin"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();
    base_store
        .record_chart(
            &pw_b,
            &[record_for(
                &pw_b,
                "ehr",
                "other-patient-a1c",
                "rev-1",
                "2026-09-01",
                "Other patient A1c 5.5% collected 2026-09-01.",
            )],
            &SourcePull {
                source_id: "ehr".into(),
                source_label: "Metro EHR".into(),
                patient: source_patient("pt-b", "Jordan Cruz"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();

    // Configure FixedSourceCheck with newer versions for both patients/sources,
    // plus a PartlyUpdated outcome for `imaging` on `pw_a`.
    let checker = FixedSourceCheck::new();
    checker.set_source_outcome(
        &practice,
        &pw_a,
        ehr_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Metro EHR".into(),
            patient: source_patient("pt-a", "Maya Lin"),
            pulled_at: 1_700_000_400,
            records: vec![record_for(
                &pw_a,
                "ehr",
                "a1c-panel",
                "rev-2",
                "2026-11-10",
                "Hemoglobin A1c 6.2% collected 2026-11-10.",
            )],
        },
    );
    checker.set_source_outcome(
        &practice,
        &pw_a,
        labs_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Quest Labs".into(),
            patient: source_patient("pt-a", "Maya Lin"),
            pulled_at: 1_700_000_400,
            records: vec![record_for(
                &pw_a,
                "labs",
                "lipid-panel",
                "rev-2",
                "2026-11-10",
                "LDL cholesterol 88 mg/dL collected 2026-11-10.",
            )],
        },
    );
    checker.set_source_outcome(
        &practice,
        &pw_a,
        imaging_id.clone(),
        SourceCheckOutcome::PartlyUpdated {
            source_label: "Radiology Archive".into(),
            patient: source_patient("pt-a", "Maya Lin"),
            pulled_at: 1_700_000_420,
            records: vec![record_for(
                &pw_a,
                "imaging",
                "carotid-ultrasound",
                "rev-1",
                "2026-11-11",
                "Carotid duplex ultrasound without hemodynamically significant stenosis on 2026-11-11.",
            )],
            reason: "prior MRI DICOM study archive segment timed out".into(),
        },
    );
    checker.set_source_outcome(
        &practice,
        &pw_b,
        ehr_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Metro EHR".into(),
            patient: source_patient("pt-b", "Jordan Cruz"),
            pulled_at: 1_700_000_400,
            records: vec![record_for(
                &pw_b,
                "ehr",
                "other-patient-a1c",
                "rev-2",
                "2026-11-10",
                "Other patient A1c 8.9% collected 2026-11-10.",
            )],
        },
    );

    let store = base_store.with_source_refresh(Arc::new(checker.clone()));

    let scope = PracticeScope {
        practice: practice.clone(),
        workspace: pw_a.clone(),
        job_id: "job-refresh-ac1".into(),
    };
    let assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-ac1",
        "Refresh Metro EHR and Radiology Archive for this patient and read the updated A1c.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    assert_eq!(
        store
            .accept_practice_job(&scope, &assignment, &roles, &skills, auth.as_ref())
            .await
            .unwrap(),
        Accepted::New
    );

    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim job");

    struct SelectiveRefreshProvider {
        step: usize,
        expected_workspace: String,
    }

    impl engine::recipe::Provider for SelectiveRefreshProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "refresh-ehr-only".into(),
                        name: REFRESH_TOOL.into(),
                        arguments: json!({ "source_ids": ["ehr"] }).to_string(),
                    })],
                    receipt: json!({ "step": 1 }),
                    usage: Default::default(),
                }),
                2 => {
                    let report: RefreshReport =
                        serde_json::from_str(materialized_output(request, "refresh-ehr-only"))
                            .expect("parse RefreshReport JSON");
                    assert_eq!(report.workspace, self.expected_workspace);
                    assert_eq!(report.sources.len(), 1);
                    assert_eq!(report.sources[0].source_id, SourceId::new("ehr").unwrap());
                    assert_eq!(
                        report.sources[0].outcome,
                        SourceRefreshOutcome::Updated {
                            workspace_version: 4,
                            changed_records: 1,
                            pulled_at: 1_700_000_400,
                        }
                    );
                    Ok(engine::Turn {
                        items: vec![
                            engine::Item::Call(engine::Invocation {
                                id: "search-after-refresh".into(),
                                name: SEARCH_TOOL.into(),
                                arguments: json!({ "query": "Hemoglobin A1c" }).to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "refresh-imaging-partial".into(),
                                name: REFRESH_TOOL.into(),
                                arguments: json!({
                                    "source_ids": ["imaging"],
                                    "request_key": "imaging-pass"
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({ "step": 2 }),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let search_out = materialized_output(request, "search-after-refresh");
                    assert!(
                        search_out.contains("rev-2"),
                        "search after refresh must see rev-2: {search_out}"
                    );
                    let imaging_report: RefreshReport = serde_json::from_str(materialized_output(
                        request,
                        "refresh-imaging-partial",
                    ))
                    .expect("parse imaging RefreshReport");
                    assert_eq!(imaging_report.sources.len(), 1);
                    assert!(matches!(
                        &imaging_report.sources[0].outcome,
                        SourceRefreshOutcome::PartlyUpdated {
                            workspace_version: 5,
                            changed_records: 1,
                            pulled_at: 1_700_000_420,
                            reason,
                        } if reason.contains("DICOM")
                    ));
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-updated-a1c".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "a1c-panel",
                                    "revision": "rev-2",
                                }],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        })],
                        receipt: json!({ "step": 3 }),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let read_out = materialized_output(request, "read-updated-a1c");
                    assert!(
                        read_out.contains("Hemoglobin A1c 6.2% collected 2026-11-10."),
                        "later read during run must return rev-2 content: {read_out}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Metro EHR updated on 2026-11-10: Hemoglobin A1c improved to 6.2% (a1c-panel rev-2). Radiology Archive partly updated with carotid duplex from 2026-11-11.",
                        )],
                        receipt: json!({ "step": 4 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected provider step {other}"),
            }
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth.as_ref(),
    };
    let mut provider = SelectiveRefreshProvider {
        step: 0,
        expected_workspace: pw_a.clone(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("selective refresh run completes");

    // Verify ONLY `(pw_a, ehr)` and `(pw_a, imaging)` were refreshed;
    // `(pw_a, labs)` and foreign patient `(pw_b, ehr)` were never touched!
    assert_eq!(checker.call_count(&practice, &pw_a, &ehr_id), 1);
    assert_eq!(checker.call_count(&practice, &pw_a, &imaging_id), 1);
    assert_eq!(checker.call_count(&practice, &pw_a, &labs_id), 0);
    assert_eq!(checker.call_count(&practice, &pw_b, &ehr_id), 0);

    // Later reads return `rev-2` for `(pw_a, ehr)`, preserve both baseline + new
    // imaging records on `PartlyUpdated`, and keep `rev-1` for untouched `(pw_a, labs)`
    // and `(pw_b, ehr)`.
    let a1c_v2 = read_exact_chart(&store, &pw_a, "a1c-panel", "rev-2").await;
    assert!(a1c_v2.contains("Hemoglobin A1c 6.2% collected 2026-11-10."));

    let baseline_xray = read_exact_chart(&store, &pw_a, "chest-xray-baseline", "rev-1").await;
    assert!(baseline_xray.contains("Baseline chest radiograph clear on 2026-08-15."));
    let new_carotid = read_exact_chart(&store, &pw_a, "carotid-ultrasound", "rev-1").await;
    assert!(new_carotid.contains("Carotid duplex ultrasound"));

    let untouched_labs = read_exact_chart(&store, &pw_a, "lipid-panel", "rev-1").await;
    assert!(untouched_labs.contains("LDL cholesterol 132 mg/dL collected 2026-09-01."));
    let missing_labs_v2 = read_exact_chart(&store, &pw_a, "lipid-panel", "rev-2").await;
    assert!(missing_labs_v2.contains("unavailable"));

    let untouched_pw_b = read_exact_chart(&store, &pw_b, "other-patient-a1c", "rev-1").await;
    assert!(untouched_pw_b.contains("Other patient A1c 5.5%"));
    let missing_pw_b_v2 = read_exact_chart(&store, &pw_b, "other-patient-a1c", "rev-2").await;
    assert!(missing_pw_b_v2.contains("unavailable"));

    let saved_statuses = store
        .saved_source_statuses(&practice, &pw_a)
        .await
        .expect("read saved source statuses");
    assert_eq!(saved_statuses.len(), 2);
    assert_eq!(saved_statuses[0].source_id, ehr_id);
    assert!(matches!(
        saved_statuses[0].outcome,
        SourceRefreshOutcome::Updated {
            workspace_version: 4,
            changed_records: 1,
            pulled_at: 1_700_000_400,
        }
    ));
    assert_eq!(saved_statuses[1].source_id, imaging_id);
    assert!(matches!(
        saved_statuses[1].outcome,
        SourceRefreshOutcome::PartlyUpdated {
            workspace_version: 5,
            changed_records: 1,
            pulled_at: 1_700_000_420,
            ..
        }
    ));

    // `PartlyUpdated` delivers available records without advancing `source_pulls.pulled_at`.
    let pulls = store
        .source_pulls(std::slice::from_ref(&pw_a))
        .await
        .unwrap();
    let pw_a_pulls = pulls.get(&pw_a).expect("pw_a source pulls");
    assert_eq!(
        pw_a_pulls
            .iter()
            .find(|p| p.source_id == "ehr")
            .and_then(|p| p.pulled_at),
        Some(1_700_000_400)
    );
    assert_eq!(
        pw_a_pulls
            .iter()
            .find(|p| p.source_id == "imaging")
            .and_then(|p| p.pulled_at),
        Some(1_700_000_100)
    );

    let finished_job = store
        .practice_job(&scope)
        .await
        .unwrap()
        .expect("saved job");
    assert_eq!(finished_job.status, WorkStatus::Completed);
    assert_eq!(finished_job.result.unwrap().sources_version, 5);
}

#[tokio::test]
async fn ac2_mixed_source_refresh_names_both_records_failure_status_and_keeps_last_good_records() {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-refresh-ac2-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-mixed-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let ehr_id = SourceId::new("ehr").unwrap();
    let pharmacy_id = SourceId::new("pharmacy").unwrap();

    // Seed last good records at `t = 1_700_000_100` for both `ehr` and `pharmacy`.
    base_store
        .record_chart(
            &workspace,
            &[record_for(
                &workspace,
                "ehr",
                "bp-note",
                "rev-1",
                "2026-09-15",
                "Office blood pressure 144/90 mmHg on 2026-09-15.",
            )],
            &SourcePull {
                source_id: "ehr".into(),
                source_label: "Metro EHR".into(),
                patient: source_patient("pt-mixed", "Elena Rostova"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();
    base_store
        .record_chart(
            &workspace,
            &[record_for(
                &workspace,
                "pharmacy",
                "med-lisinopril",
                "rev-1",
                "2026-09-15",
                "Lisinopril 20 mg daily dispensed 2026-09-15.",
            )],
            &SourcePull {
                source_id: "pharmacy".into(),
                source_label: "SureScripts Pharmacy".into(),
                patient: source_patient("pt-mixed", "Elena Rostova"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();

    // `ehr` succeeds with `rev-2` at `t = 1_700_000_500`; `pharmacy` fails at `t = 1_700_000_505`.
    let checker = FixedSourceCheck::new();
    checker.set_source_outcome(
        &practice,
        &workspace,
        ehr_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Metro EHR".into(),
            patient: source_patient("pt-mixed", "Elena Rostova"),
            pulled_at: 1_700_000_500,
            records: vec![record_for(
                &workspace,
                "ehr",
                "bp-note",
                "rev-2",
                "2026-11-11",
                "Home blood pressure log average 124/78 mmHg on 2026-11-11.",
            )],
        },
    );
    checker.set_source_outcome(
        &practice,
        &workspace,
        pharmacy_id.clone(),
        SourceCheckOutcome::Failed {
            source_label: None,
            checked_at: 1_700_000_505,
            reason: "SureScripts upstream gateway timeout (HTTP 504)".into(),
        },
    );

    let store = base_store.with_source_refresh(Arc::new(checker));
    let scope = PracticeScope {
        practice: practice.clone(),
        workspace: workspace.clone(),
        job_id: "job-refresh-ac2".into(),
    };
    let assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-ac2",
        "Refresh all patient sources and verify current blood pressure and medication records.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    store
        .accept_practice_job(&scope, &assignment, &roles, &skills, auth.as_ref())
        .await
        .unwrap();
    let claim = store
        .claim_practice_job(&scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim mixed refresh job");

    struct MixedOutcomeProvider {
        step: usize,
    }

    impl engine::recipe::Provider for MixedOutcomeProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "refresh-all".into(),
                        name: REFRESH_TOOL.into(),
                        arguments: json!({}).to_string(),
                    })],
                    receipt: json!({ "step": 1 }),
                    usage: Default::default(),
                }),
                2 => {
                    let report_json = materialized_output(request, "refresh-all");
                    let report: RefreshReport =
                        serde_json::from_str(report_json).expect("decode mixed RefreshReport");
                    assert_eq!(report.sources.len(), 2, "result must name both sources");
                    assert_eq!(report.sources[0].source_id, SourceId::new("ehr").unwrap());
                    assert_eq!(
                        report.sources[0].outcome,
                        SourceRefreshOutcome::Updated {
                            workspace_version: 3,
                            changed_records: 1,
                            pulled_at: 1_700_000_500,
                        }
                    );
                    assert_eq!(
                        report.sources[1].source_id,
                        SourceId::new("pharmacy").unwrap()
                    );
                    assert_eq!(report.sources[1].source_label, "SureScripts Pharmacy");
                    assert_eq!(
                        report.sources[1].outcome,
                        SourceRefreshOutcome::Failed {
                            last_good_pulled_at: Some(1_700_000_100),
                            checked_at: 1_700_000_505,
                            reason: "SureScripts upstream gateway timeout (HTTP 504)".into(),
                        }
                    );

                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-both".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    { "candidate_id": "bp-note", "revision": "rev-2" },
                                    { "candidate_id": "med-lisinopril", "revision": "rev-1" },
                                ],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        })],
                        receipt: json!({ "step": 2 }),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let read_both = materialized_output(request, "read-both");
                    assert!(
                        read_both.contains("124/78 mmHg on 2026-11-11"),
                        "succeeded source must expose new record: {read_both}"
                    );
                    assert!(
                        read_both.contains("Lisinopril 20 mg daily dispensed 2026-09-15."),
                        "failed source must keep its last good record readable: {read_both}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Metro EHR refreshed with 2026-11-11 BP 124/78 mmHg (bp-note rev-2). SureScripts Pharmacy refresh failed at check; retained 2026-09-15 Lisinopril 20 mg record remains available.",
                        )],
                        receipt: json!({ "step": 3 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected provider step {other}"),
            }
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth.as_ref(),
    };
    host.run(
        &mut MixedOutcomeProvider { step: 0 },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("mixed refresh run completes");

    // Verify saved source status table durably recorded both outcomes, including the failure.
    let statuses = store
        .saved_source_statuses(&practice, &workspace)
        .await
        .unwrap();
    assert_eq!(
        statuses,
        vec![
            SavedSourceStatus {
                practice: practice.clone(),
                workspace: workspace.clone(),
                source_id: ehr_id,
                source_label: "Metro EHR".into(),
                outcome: SourceRefreshOutcome::Updated {
                    workspace_version: 3,
                    changed_records: 1,
                    pulled_at: 1_700_000_500,
                },
            },
            SavedSourceStatus {
                practice: practice.clone(),
                workspace: workspace.clone(),
                source_id: pharmacy_id,
                source_label: "SureScripts Pharmacy".into(),
                outcome: SourceRefreshOutcome::Failed {
                    last_good_pulled_at: Some(1_700_000_100),
                    checked_at: 1_700_000_505,
                    reason: "SureScripts upstream gateway timeout (HTTP 504)".into(),
                },
            },
        ]
    );

    // Verify failed source keeps its last good records AND its last good `pulled_at` in `source_pulls`.
    let observation = store.sources(&workspace).await.unwrap();
    assert_eq!(observation.version, 3);
    assert_eq!(observation.sources.len(), 2);
    let kept_pharmacy = read_exact_chart(&store, &workspace, "med-lisinopril", "rev-1").await;
    assert!(kept_pharmacy.contains("Lisinopril 20 mg daily dispensed 2026-09-15."));

    let pulls = store
        .source_pulls(std::slice::from_ref(&workspace))
        .await
        .unwrap();
    let ws_pulls = &pulls[&workspace];
    assert_eq!(ws_pulls.len(), 2);
    assert_eq!(ws_pulls[0].source_id, "ehr");
    assert_eq!(ws_pulls[0].pulled_at, Some(1_700_000_500));
    assert_eq!(ws_pulls[1].source_id, "pharmacy");
    assert_eq!(ws_pulls[1].pulled_at, Some(1_700_000_100));
}

#[tokio::test]
async fn ac3_retry_after_interrupted_source_check_saves_each_result_once_and_rechecks_patient_access()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-refresh-ac3-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-retry-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();

    let ehr_id = SourceId::new("ehr").unwrap();
    let labs_id = SourceId::new("labs").unwrap();

    // Seed initial `rev-1` records in the real evidence store (`version` = 2).
    base_store
        .record_chart(
            &workspace,
            &[record_for(
                &workspace,
                "ehr",
                "visit-note",
                "rev-1",
                "2026-09-01",
                "Initial hypertension visit note 2026-09-01.",
            )],
            &SourcePull {
                source_id: "ehr".into(),
                source_label: "Metro EHR".into(),
                patient: source_patient("pt-retry", "Samuel Okafor"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();
    base_store
        .record_chart(
            &workspace,
            &[record_for(
                &workspace,
                "labs",
                "potassium-lab",
                "rev-1",
                "2026-09-01",
                "Serum potassium 4.1 mEq/L on 2026-09-01.",
            )],
            &SourcePull {
                source_id: "labs".into(),
                source_label: "Quest Labs".into(),
                patient: source_patient("pt-retry", "Samuel Okafor"),
                pulled_at: Some(1_700_000_100),
            },
        )
        .await
        .unwrap();
    assert_eq!(base_store.sources(&workspace).await.unwrap().version, 2);

    // Prepare Attempt 1's cancellation token so `labs` interrupts Attempt 1 right
    // after `ehr` has already committed its new record to the real evidence store!
    let cancel_attempt_1 = engine::Cancellation::default();
    let cancel_for_hook = cancel_attempt_1.clone();

    let checker = FixedSourceCheck::new();
    checker.set_source_outcome(
        &practice,
        &workspace,
        ehr_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Metro EHR".into(),
            patient: source_patient("pt-retry", "Samuel Okafor"),
            pulled_at: 1_700_000_600,
            records: vec![record_for(
                &workspace,
                "ehr",
                "visit-note",
                "rev-2",
                "2026-11-11",
                "Follow-up hypertension visit note 2026-11-11.",
            )],
        },
    );
    checker.push_step(
        &practice,
        &workspace,
        labs_id.clone(),
        FixedCheckStep::Interrupt {
            reason: "worker connection dropped mid-pull".into(),
            on_interrupt: Some(Arc::new(move || {
                cancel_for_hook.cancel();
            })),
        },
    );
    checker.set_source_outcome(
        &practice,
        &workspace,
        labs_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Quest Labs".into(),
            patient: source_patient("pt-retry", "Samuel Okafor"),
            pulled_at: 1_700_000_610,
            records: vec![record_for(
                &workspace,
                "labs",
                "potassium-lab",
                "rev-2",
                "2026-11-11",
                "Serum potassium 4.4 mEq/L on 2026-11-11.",
            )],
        },
    );

    let store = base_store.with_source_refresh(Arc::new(checker.clone()));

    struct RevocablePracticeAuthority {
        inner: Arc<PostgresAuthStore>,
        revoked: AtomicBool,
    }

    #[async_trait]
    impl PracticeAuthority for RevocablePracticeAuthority {
        async fn check_practice_patient(
            &self,
            scope: &PracticeScope,
        ) -> Result<(), PracticeAccessError> {
            if self.revoked.load(Ordering::SeqCst) {
                return Err(PracticeAccessError::UnregisteredPatient {
                    practice: scope.practice.clone(),
                    workspace: scope.workspace.clone(),
                });
            }
            self.inner.check_practice_patient(scope).await
        }
    }

    let revocable_practice_auth = RevocablePracticeAuthority {
        inner: auth.clone(),
        revoked: AtomicBool::new(false),
    };

    let scope = PracticeScope {
        practice: practice.clone(),
        workspace: workspace.clone(),
        job_id: "job-refresh-ac3".into(),
    };
    let assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-ac3",
        "Refresh all patient sources, recovering cleanly if interrupted, and summarize updated records.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    store
        .accept_practice_job(
            &scope,
            &assignment,
            &roles,
            &skills,
            &revocable_practice_auth,
        )
        .await
        .unwrap();

    // --- Attempt 1: short 1-second lease; `ehr` commits, `labs` interrupts Attempt 1 ---
    let claim_1 = store
        .claim_practice_job(&scope, 1, &revocable_practice_auth)
        .await
        .unwrap()
        .expect("claim attempt 1");

    struct InterruptThenRetryProvider {
        attempt_phase: usize,
    }

    impl engine::recipe::Provider for InterruptThenRetryProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.attempt_phase += 1;
            match self.attempt_phase {
                // Attempt 1, Turn 1: request refresh of all sources (`ehr` + `labs`).
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "refresh-attempt-1".into(),
                        name: REFRESH_TOOL.into(),
                        arguments: json!({}).to_string(),
                    })],
                    receipt: json!({ "phase": 1 }),
                    usage: Default::default(),
                }),
                // Attempt 2, Turn 1 (after recovery): `refresh-attempt-1` had `ToolStarted`
                // with no settled command receipt yet, so harness reports `"outcome": "unknown"`.
                // Fixed provider retries `refresh_patient_sources({})` in the same work!
                2 => {
                    let unknown_out = materialized_output(request, "refresh-attempt-1");
                    assert!(
                        unknown_out.contains("unknown"),
                        "interrupted refresh without completed receipt must report unknown outcome: {unknown_out}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "refresh-attempt-2".into(),
                            name: REFRESH_TOOL.into(),
                            arguments: json!({}).to_string(),
                        })],
                        receipt: json!({ "phase": 2 }),
                        usage: Default::default(),
                    })
                }
                // Attempt 2, Turn 2: verify both `ehr` (reused from Attempt 1's atomic receipt!)
                // and `labs` (newly fetched once!) are reported, then read both `rev-2` records.
                3 => {
                    let report: RefreshReport =
                        serde_json::from_str(materialized_output(request, "refresh-attempt-2"))
                            .expect("decode retried RefreshReport");
                    assert_eq!(report.workspace_version, 4);
                    assert_eq!(report.sources.len(), 2);
                    assert_eq!(
                        report.sources[0].outcome,
                        SourceRefreshOutcome::Updated {
                            workspace_version: 3,
                            changed_records: 1,
                            pulled_at: 1_700_000_600,
                        }
                    );
                    assert_eq!(
                        report.sources[1].outcome,
                        SourceRefreshOutcome::Updated {
                            workspace_version: 4,
                            changed_records: 1,
                            pulled_at: 1_700_000_610,
                        }
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-after-retry".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    { "candidate_id": "visit-note", "revision": "rev-2" },
                                    { "candidate_id": "potassium-lab", "revision": "rev-2" },
                                ],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        })],
                        receipt: json!({ "phase": 3 }),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let read_out = materialized_output(request, "read-after-retry");
                    assert!(read_out.contains("Follow-up hypertension visit note 2026-11-11."));
                    assert!(read_out.contains("Serum potassium 4.4 mEq/L on 2026-11-11."));
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Both sources refreshed once across recovery: 2026-11-11 hypertension visit note and potassium 4.4 mEq/L.",
                        )],
                        receipt: json!({ "phase": 4 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected provider phase {other}"),
            }
        }
    }

    let mut provider = InterruptThenRetryProvider { attempt_phase: 0 };
    let mut host_1 = PracticeJobHost {
        store: store.clone(),
        claim: claim_1,
        authority: &revocable_practice_auth,
    };
    let interrupted = host_1
        .run(
            &mut provider,
            engine::RunLimits::default(),
            &cancel_attempt_1,
        )
        .await;
    assert!(
        interrupted.is_err(),
        "Attempt 1 must abort when interrupted mid-source-check"
    );

    // After Attempt 1 aborted mid-check:
    // - `ehr` ran once and committed `rev-2` (`version` 2 -> 3).
    // - `labs` ran once and aborted before committing (`potassium-lab` still `rev-1`).
    assert_eq!(checker.call_count(&practice, &workspace, &ehr_id), 1);
    assert_eq!(checker.call_count(&practice, &workspace, &labs_id), 1);
    assert_eq!(store.sources(&workspace).await.unwrap().version, 3);

    // Wait for Attempt 1's 1-second lease to expire before retrying.
    tokio::time::sleep(Duration::from_millis(1200)).await;

    // Revoke practice patient access BEFORE retry: both job claim and direct
    // refresh retry must recheck access first and refuse without touching any source!
    revocable_practice_auth
        .revoked
        .store(true, Ordering::SeqCst);
    assert!(matches!(
        store
            .claim_practice_job(&scope, 60, &revocable_practice_auth)
            .await,
        Err(PracticeJobError::Access(
            PracticeAccessError::UnregisteredPatient { .. }
        ))
    ));
    let direct_retry_request = RefreshRequest::new(
        RefreshCommandKey::new("manual-check").unwrap(),
        RefreshTarget::AllSources,
    );
    assert!(matches!(
        store
            .refresh_patient_sources_for_practice(
                &scope,
                &revocable_practice_auth,
                &direct_retry_request,
                &checker,
            )
            .await,
        Err(RefreshError::PracticeAccess(
            PracticeAccessError::UnregisteredPatient { .. }
        ))
    ));
    assert_eq!(
        checker.call_count(&practice, &workspace, &ehr_id),
        1,
        "revoked retry must never invoke ehr source check"
    );
    assert_eq!(
        checker.call_count(&practice, &workspace, &labs_id),
        1,
        "revoked retry must never invoke labs source check"
    );
    assert_eq!(store.sources(&workspace).await.unwrap().version, 3);

    // Restore patient access and claim Attempt 2 -> finishes cleanly!
    revocable_practice_auth
        .revoked
        .store(false, Ordering::SeqCst);
    let claim_2 = store
        .claim_practice_job(&scope, 60, &revocable_practice_auth)
        .await
        .unwrap()
        .expect("claim attempt 2 after access restored");
    let mut host_2 = PracticeJobHost {
        store: store.clone(),
        claim: claim_2,
        authority: &revocable_practice_auth,
    };
    host_2
        .run(
            &mut provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("Attempt 2 completes after recovering interrupted refresh");

    // Once-only proof:
    // - `ehr` was NEVER called a second time (`call_count == 1`)!
    // - `labs` was called once in Attempt 1 (interrupted) + once in Attempt 2 (`call_count == 2`)!
    // - `workspace_version` bumped once for `ehr` (3) and once for `labs` (4), never twice!
    assert_eq!(checker.call_count(&practice, &workspace, &ehr_id), 1);
    assert_eq!(checker.call_count(&practice, &workspace, &labs_id), 2);
    assert_eq!(store.sources(&workspace).await.unwrap().version, 4);

    // Also verify `saved_practice_refresh_outcome` reconciles the now-completed
    // command receipt without calling `checker` at all!
    let completed_job = store.practice_job(&scope).await.unwrap().unwrap();
    let reconciled = store
        .saved_practice_refresh_outcome(
            &scope,
            &completed_job.work,
            &engine::Invocation {
                id: "refresh-attempt-2".into(),
                name: REFRESH_TOOL.into(),
                arguments: json!({}).to_string(),
            },
        )
        .await
        .unwrap()
        .expect("completed refresh command has saved_outcome");
    let reconciled_report: RefreshReport =
        serde_json::from_str(reconciled.text().expect("returned text")).unwrap();
    assert_eq!(reconciled_report.workspace_version, 4);
    assert_eq!(checker.call_count(&practice, &workspace, &ehr_id), 1);
    assert_eq!(checker.call_count(&practice, &workspace, &labs_id), 2);

    // Also verify physician-scoped retry rechecks panel `Authority` before returning
    // even an already-committed once-only receipt, and detects idempotency collisions!
    struct TogglePhysicianAuthority {
        allowed: bool,
    }

    #[async_trait]
    impl Authority for TogglePhysicianAuthority {
        async fn check(
            &mut self,
            _: &PublicationTransaction,
            _: &Scope,
            session_hash: Option<&[u8]>,
        ) -> Result<(), String> {
            if session_hash != Some(&[7_u8; 32][..]) {
                return Err("session hash mismatch".into());
            }
            if !self.allowed {
                return Err("physician removed from patient panel".into());
            }
            Ok(())
        }

        async fn lock_publication(
            &mut self,
            tx: &PublicationTransaction,
            scope: &Scope,
            session_hash: Option<&[u8]>,
        ) -> Result<(), String> {
            self.check(tx, scope, session_hash).await
        }
    }

    let chat_scope = Scope {
        practice: practice.clone(),
        physician: "dr-chen".into(),
        workspace: workspace.clone(),
        chat: "chat-refresh-access".into(),
    };
    let built_chat_recipe = chat_recipe(&roles.chat, "Review patient chart.", &skills);
    store
        .accept_authorized(
            &chat_scope,
            &Input {
                key: "msg-1".into(),
                text: "Check fresh labs.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            vec![7_u8; 32],
            &skills,
            &built_chat_recipe,
            None,
        )
        .await
        .unwrap();

    let mut md_auth = TogglePhysicianAuthority { allowed: true };
    let md_request = RefreshRequest::new(
        RefreshCommandKey::new("md-refresh-1").unwrap(),
        RefreshTarget::selected([ehr_id.clone()]).unwrap(),
    );
    let first_md_report = store
        .refresh_patient_sources_for_physician(&chat_scope, &mut md_auth, &md_request, &checker)
        .await
        .unwrap();
    assert_eq!(first_md_report.sources.len(), 1);
    assert_eq!(checker.call_count(&practice, &workspace, &ehr_id), 2);

    // Exact retry while still authorized returns the saved report without calling `checker` again.
    let duplicate_md_report = store
        .refresh_patient_sources_for_physician(&chat_scope, &mut md_auth, &md_request, &checker)
        .await
        .unwrap();
    assert_eq!(duplicate_md_report, first_md_report);
    assert_eq!(
        checker.call_count(&practice, &workspace, &ehr_id),
        2,
        "once-only receipt must prevent duplicate source check on retry"
    );

    // Reusing `md-refresh-1` with a different target is rejected as `IdempotencyCollision`.
    let colliding_request = RefreshRequest::new(
        RefreshCommandKey::new("md-refresh-1").unwrap(),
        RefreshTarget::AllSources,
    );
    assert!(matches!(
        store
            .refresh_patient_sources_for_physician(
                &chat_scope,
                &mut md_auth,
                &colliding_request,
                &checker,
            )
            .await,
        Err(RefreshError::IdempotencyCollision { .. })
    ));

    // Revoking physician panel access before retry refuses immediately, even when a
    // committed once-only receipt already exists for `md-refresh-1`, and refuses a
    // claimed `PhysicianHost` retry as well!
    md_auth.allowed = false;
    assert!(matches!(
        store
            .refresh_patient_sources_for_physician(&chat_scope, &mut md_auth, &md_request, &checker)
            .await,
        Err(RefreshError::PhysicianAccess(err)) if err.to_string().contains("removed from patient panel")
    ));

    let chat_claim = store
        .claim(&chat_scope, 60)
        .await
        .unwrap()
        .expect("claim chat");
    let mut chat_host = PhysicianHost {
        store: store.clone(),
        claim: chat_claim,
        authority: TogglePhysicianAuthority { allowed: false },
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };
    struct NeverReachedProvider;
    impl engine::recipe::Provider for NeverReachedProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            panic!("provider must never be reached when physician access is revoked");
        }
    }
    let revoked_host_res = chat_host
        .run(
            &mut NeverReachedProvider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await;
    assert!(
        revoked_host_res.is_err(),
        "PhysicianHost retry must recheck authority before running"
    );
    assert_eq!(checker.call_count(&practice, &workspace, &ehr_id), 2);
}
