//! Durable HTTP contract and Chromium browser qualification for patient-page
//! reviews, lifecycle controls (`Stop`, `Resume`, `Try again`), reload
//! persistence, panel access checks, and desktop/phone viewports.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use base64::Engine as _;
use local_product::initial_understanding::{
    HealthSystemStanding, IndexedAnchor, KnownHealthSystem, OperatingIndex,
    SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs, SummaryClaimInput, job_key,
};
use local_product::patient_reviews::{
    COMPLETE_PATIENT_REVIEW_TOOL, CompleteReviewArgs, CompleteReviewDisposition, ReviewTrigger,
    UnfinishedReviewWork,
};
use local_product::physician_sources::{READ_TOOL, SEARCH_TOOL, workspace_id};
use local_product::runtime::chat_operations::{PATIENT_MEMORY_TOOL, RecordLimit, RecordLimitKind};
use local_product::runtime::concerns::ExactRecordReference;
use local_product::runtime::engine;
use local_product::runtime::jobs::{PracticeJobHost, PracticeScope};
use local_product::runtime::reviews::AdmittedPatientReview;
use physician_auth::{AuthStore, Membership, PhysicianIdentity};
use physician_wire::{
    PatientReviewWire, PatientReviewsResponse, ReviewControlActionWire, ReviewControlRefusalReason,
    ReviewControlRequest, ReviewControlResponse, ReviewVersion,
};
use tokio::sync::{Mutex, Notify};

mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[path = "scale/mod.rs"]
mod scale;

use devtools::{Browser, Key};

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static BUILT_SITE: OnceLock<PathBuf> = OnceLock::new();

fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

struct SyntheticSeed {
    bank_dir: PathBuf,
    workspace: String,
    note_revision: String,
    lab_revision: String,
}

impl Drop for SyntheticSeed {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.bank_dir);
    }
}

async fn seed_synthetic_patient(
    patient_id: &str,
    auth: &physician_auth::PostgresAuthStore,
    store: &local_product::runtime::Store,
    roles: &local_product::ModelRoles,
    skills: &local_product::physician_chat::PhysicianSkills,
) -> SyntheticSeed {
    let bank_dir = env::temp_dir().join(format!(
        "coda-patient-reviews-bank-{patient_id}-{}",
        uuid::Uuid::new_v4()
    ));
    let _ = fs::remove_dir_all(&bank_dir);
    let chart_dir = bank_dir.join(format!("patients/{patient_id}"));
    fs::create_dir_all(&chart_dir).expect("create synthetic chart dir");

    let patient_ndjson = serde_json::json!({
        "resourceType": "Patient",
        "id": patient_id,
        "meta": { "lastUpdated": "2026-08-25T10:00:00Z" },
        "identifier": [{ "system": "urn:oid:2.16.840.1.113883.19.5", "value": "00091402" }],
        "name": [{ "family": "Navarro", "given": ["Sofia"] }],
        "gender": "female",
        "birthDate": "1968-05-19"
    });
    let note_text = "CONSULTATION NOTE (2026-08-20): Persistent atrial fibrillation anticoagulated with apixaban 5 mg twice daily. Prior ablation operative report from Harborview Electrophysiology Center has not yet been received.";
    let note_ndjson = serde_json::json!({
        "resourceType": "DocumentReference",
        "id": "consult-afib-2026",
        "date": "2026-08-20T14:30:00Z",
        "content": [{
            "attachment": {
                "contentType": "text/plain; charset=utf-8",
                "data": base64::engine::general_purpose::STANDARD.encode(note_text.as_bytes()),
            }
        }]
    });
    let lab_ndjson = serde_json::json!({
        "resourceType": "Observation",
        "id": "creatinine-2026-08",
        "effectiveDateTime": "2026-08-22T09:15:00Z",
        "code": { "text": "Serum Creatinine" },
        "valueQuantity": { "value": 1.12, "unit": "mg/dL" }
    });

    fs::write(
        chart_dir.join("Patient.ndjson"),
        format!("{}\n", serde_json::to_string(&patient_ndjson).unwrap()),
    )
    .unwrap();
    fs::write(
        chart_dir.join("DocumentReference.ndjson"),
        format!("{}\n", serde_json::to_string(&note_ndjson).unwrap()),
    )
    .unwrap();
    fs::write(
        chart_dir.join("Observation.ndjson"),
        format!("{}\n", serde_json::to_string(&lab_ndjson).unwrap()),
    )
    .unwrap();

    let summary = physician_sample::chart_load::load_synthetic_practice_with_understanding(
        &bank_dir, auth, store, roles, skills,
    )
    .await
    .expect("load synthetic practice");
    assert_eq!(summary.patients, 1);
    assert_eq!(summary.records, 3);
    assert!(summary.failed.is_empty(), "{:?}", summary.failed);

    let workspace = workspace_id(physician_sample::synthetic_chart::SOURCE_ID, patient_id);
    let initial_scope = PracticeScope {
        practice: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        workspace: workspace.clone(),
        job_id: job_key(&workspace, 1),
    };
    let claim = store
        .claim_practice_job(&initial_scope, 60, auth)
        .await
        .unwrap()
        .expect("claim initial understanding job");

    struct InitialSummaryProvider {
        turn: usize,
        note_rev: Option<String>,
        lab_rev: Option<String>,
    }

    impl engine::recipe::Provider for InitialSummaryProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "browse-chart".into(),
                        name: SEARCH_TOOL.into(),
                        arguments: serde_json::json!({
                            "query": "",
                            "record_type": null,
                            "after": null
                        })
                        .to_string(),
                    })],
                    receipt: serde_json::json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let search_json: serde_json::Value = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "browse-chart" =>
                            {
                                serde_json::from_str(output).ok()
                            }
                            _ => None,
                        })
                        .expect("search results materialized");
                    let results = search_json["results"].as_array().expect("results array");
                    let find_rev = |cand: &str| -> String {
                        results
                            .iter()
                            .find(|r| r["reference"]["candidate_id"] == cand)
                            .and_then(|r| r["reference"]["revision"].as_str())
                            .unwrap_or_else(|| panic!("missing candidate {cand} in {results:?}"))
                            .to_owned()
                    };
                    let note_rev = find_rev("DocumentReference/consult-afib-2026");
                    let lab_rev = find_rev("Observation/creatinine-2026-08");
                    self.note_rev = Some(note_rev.clone());
                    self.lab_rev = Some(lab_rev.clone());
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-sources".into(),
                            name: READ_TOOL.into(),
                            arguments: serde_json::json!({
                                "references": [
                                    {
                                        "candidate_id": "DocumentReference/consult-afib-2026",
                                        "revision": note_rev,
                                        "cursor": null
                                    },
                                    {
                                        "candidate_id": "Observation/creatinine-2026-08",
                                        "revision": lab_rev,
                                        "cursor": null
                                    }
                                ],
                                "max_result_bytes": 32_768
                            })
                            .to_string(),
                        })],
                        receipt: serde_json::json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let note_rev = self.note_rev.clone().unwrap();
                    let args = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "Persistent AFib on apixaban; external EP ablation report missing.".into(),
                            known_health_systems: vec![KnownHealthSystem {
                                name: "Synthetic chart".into(),
                                standing: HealthSystemStanding::Connected {
                                    source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                                },
                                note: "Primary connected chart".into(),
                            }],
                            open_threads: vec!["Obtain Harborview EP operative report".into()],
                            anchors: vec![IndexedAnchor {
                                reference: ExactRecordReference {
                                    candidate_id: "DocumentReference/consult-afib-2026".into(),
                                    revision: note_rev.clone(),
                                },
                                source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                                topic: "Persistent atrial fibrillation".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Persistent atrial fibrillation anticoagulated with apixaban 5 mg twice daily".into(),
                            citations: vec![ExactRecordReference {
                                candidate_id: "DocumentReference/consult-afib-2026".into(),
                                revision: note_rev,
                            }],
                        }],
                        material_gaps: vec![
                            "Harborview Electrophysiology Center ablation operative report not yet received".into(),
                        ],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "save-understanding".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&args).unwrap(),
                        })],
                        receipt: serde_json::json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Initial patient understanding saved.",
                    )],
                    receipt: serde_json::json!({"turn": 4}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected initial summary turn {other}"),
            }
        }
    }

    let mut provider = InitialSummaryProvider {
        turn: 0,
        note_rev: None,
        lab_rev: None,
    };
    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth,
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("complete initial summary job");

    SyntheticSeed {
        bank_dir,
        workspace,
        note_revision: provider.note_rev.expect("note revision"),
        lab_revision: provider.lab_rev.expect("lab revision"),
    }
}

async fn run_assessed_review(
    store: &local_product::runtime::Store,
    auth: &physician_auth::PostgresAuthStore,
    roles: &local_product::ModelRoles,
    skills: &local_product::physician_chat::PhysicianSkills,
    seed: &SyntheticSeed,
    command_key: &str,
) -> String {
    let obs = store.sources(&seed.workspace).await.unwrap();
    let record_versions: BTreeMap<String, String> = obs
        .sources
        .iter()
        .map(|s| (s.candidate_id.clone(), s.revision.clone()))
        .collect();
    let trigger = ReviewTrigger::ChangedRecords {
        workspace_version: obs.version,
        record_versions,
        changed_candidates: vec![
            "Observation/creatinine-2026-08".into(),
            "DocumentReference/consult-afib-2026".into(),
        ],
    };
    let admitted = store
        .admit_patient_review_with_trigger(
            physician_sample::STAGING_PRACTICE_ID,
            &seed.workspace,
            command_key,
            trigger,
            roles,
            skills,
            auth,
        )
        .await
        .expect("admit assessed review");
    let review = match admitted {
        AdmittedPatientReview::New { review, .. }
        | AdmittedPatientReview::Duplicate { review, .. } => *review,
    };
    let claim = store
        .claim_practice_job(&review.practice_scope(), 60, auth)
        .await
        .unwrap()
        .expect("claim assessed review");

    struct AssessedReviewProvider {
        turn: usize,
        note_rev: String,
        lab_rev: String,
    }

    impl engine::recipe::Provider for AssessedReviewProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-both".into(),
                        name: READ_TOOL.into(),
                        arguments: serde_json::json!({
                            "references": [
                                {
                                    "candidate_id": "DocumentReference/consult-afib-2026",
                                    "revision": self.note_rev,
                                    "cursor": null
                                },
                                {
                                    "candidate_id": "Observation/creatinine-2026-08",
                                    "revision": self.lab_rev,
                                    "cursor": null
                                }
                            ],
                            "max_result_bytes": 32_768
                        })
                        .to_string(),
                    })],
                    receipt: serde_json::json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "record-mem".into(),
                        name: PATIENT_MEMORY_TOOL.into(),
                        arguments: serde_json::json!({
                            "operation": "record_correction",
                            "memory_id": "mem-renal-dosing",
                            "topic": "Apixaban renal monitoring",
                            "statement": "Baseline creatinine 1.12 mg/dL on 2026-08-22 supports continuing standard apixaban 5 mg twice daily."
                        })
                        .to_string(),
                    })],
                    receipt: serde_json::json!({"turn": 2}),
                    usage: Default::default(),
                }),
                3 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Serum creatinine is 1.12 mg/dL [2], supporting continuation of standard-dose apixaban 5 mg twice daily for persistent atrial fibrillation [1].".into(),
                        },
                        citations: vec![
                            ExactRecordReference {
                                candidate_id: "DocumentReference/consult-afib-2026".into(),
                                revision: self.note_rev.clone(),
                            },
                            ExactRecordReference {
                                candidate_id: "Observation/creatinine-2026-08".into(),
                                revision: self.lab_rev.clone(),
                            },
                        ],
                        basis_and_limitations: "Reviewed cardiology consultation note [1] and serum creatinine lab [2]; external Harborview EP operative report remains missing.".into(),
                        unfinished_work: vec![UnfinishedReviewWork {
                            topic: "Harborview EP ablation operative report".into(),
                            detail: "Verify pulmonary vein isolation details once external records arrive.".into(),
                            limit: Some(RecordLimit {
                                kind: RecordLimitKind::Missing,
                                subject: "DocumentReference/harborview-ablation-2025".into(),
                                detail: "External operative report is not present in connected sources.".into(),
                            }),
                        }],
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-assessed".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: serde_json::json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Completed assessed review.")],
                    receipt: serde_json::json!({"turn": 4}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected assessed review turn {other}"),
            }
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth,
    };
    host.run(
        &mut AssessedReviewProvider {
            turn: 0,
            note_rev: seed.note_revision.clone(),
            lab_rev: seed.lab_revision.clone(),
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("run assessed review");

    review.review_id
}

async fn run_quiet_review(
    store: &local_product::runtime::Store,
    auth: &physician_auth::PostgresAuthStore,
    roles: &local_product::ModelRoles,
    skills: &local_product::physician_chat::PhysicianSkills,
    seed: &SyntheticSeed,
    command_key: &str,
) -> String {
    let obs = store.sources(&seed.workspace).await.unwrap();
    let record_versions: BTreeMap<String, String> = obs
        .sources
        .iter()
        .map(|s| (s.candidate_id.clone(), s.revision.clone()))
        .collect();
    let trigger = ReviewTrigger::Periodic {
        workspace_version: obs.version,
        record_versions,
        last_reviewed_at: Some(1_756_200_000),
    };
    let admitted = store
        .admit_patient_review_with_trigger(
            physician_sample::STAGING_PRACTICE_ID,
            &seed.workspace,
            command_key,
            trigger,
            roles,
            skills,
            auth,
        )
        .await
        .expect("admit quiet review");
    let review = match admitted {
        AdmittedPatientReview::New { review, .. }
        | AdmittedPatientReview::Duplicate { review, .. } => *review,
    };
    let claim = store
        .claim_practice_job(&review.practice_scope(), 60, auth)
        .await
        .unwrap()
        .expect("claim quiet review");

    struct QuietReviewProvider {
        turn: usize,
        lab_rev: String,
    }

    impl engine::recipe::Provider for QuietReviewProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "read-lab".into(),
                        name: READ_TOOL.into(),
                        arguments: serde_json::json!({
                            "references": [{
                                "candidate_id": "Observation/creatinine-2026-08",
                                "revision": self.lab_rev,
                                "cursor": null
                            }],
                            "max_result_bytes": 32_768
                        })
                        .to_string(),
                    })],
                    receipt: serde_json::json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Quiet {
                            explanation: "Renal function and anticoagulation standing remain unchanged since the last assessment [1].".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "Observation/creatinine-2026-08".into(),
                            revision: self.lab_rev.clone(),
                        }],
                        basis_and_limitations:
                            "Scheduled periodic review against connected chart lab [1].".into(),
                        unfinished_work: Vec::new(),
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "complete-quiet".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: serde_json::json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Completed quiet periodic review.")],
                    receipt: serde_json::json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected quiet review turn {other}"),
            }
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth,
    };
    host.run(
        &mut QuietReviewProvider {
            turn: 0,
            lab_rev: seed.lab_revision.clone(),
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("run quiet review");

    review.review_id
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_endpoints_enforce_versions_idempotency_and_panel_access() {
    let _guard = test_lock().lock().await;
    let Some(database) = scale::Database::create("reviews_http").await else {
        return;
    };
    scale::staging_account(&database.url).await;

    let store = local_product::runtime::Store::connect(&database.url)
        .await
        .unwrap();
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .unwrap();
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    let roles = scale::models::roles();

    let seed = seed_synthetic_patient("SYN-REV-HTTP-01", &auth, &store, &roles, &skills).await;
    let assessed_id =
        run_assessed_review(&store, &auth, &roles, &skills, &seed, "rev-http-assessed").await;

    let staging_identity = PhysicianIdentity {
        practice_id: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        physician_id: "staging-physician".to_owned(),
    };
    let admitted_running = store
        .submit_physician_patient_review(
            &auth,
            &staging_identity,
            &seed.workspace,
            "rev-http-running",
            "Re-check apixaban renal threshold before refill",
            &roles,
            &skills,
        )
        .await
        .expect("admit running review");
    let running_saved = match admitted_running {
        AdmittedPatientReview::New { review, .. }
        | AdmittedPatientReview::Duplicate { review, .. } => *review,
    };
    let running_id = running_saved.review_id.clone();
    let _claim = store
        .claim_practice_job(&running_saved.practice_scope(), 60, &auth)
        .await
        .unwrap()
        .expect("claim running review");

    auth.ensure_account(
        "other-practice",
        "outside-physician",
        "outside@other-clinic.test",
        "Outside Physician",
        false,
        scale::unix_now(),
    )
    .await
    .unwrap();

    let db_url = database.url.clone();
    let server = common::serve_isolated(|| {
        // SAFETY: `test_lock()` and `serve_isolated`'s start lock serialize env changes.
        unsafe {
            env::remove_var("PHYSICIAN_SITE_DIR");
            env::set_var("PHYSICIAN_DATABASE_URL", &db_url);
            env::set_var("PHYSICIAN_AUTH", "required");
            env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let db_url_b = database.url.clone();
    let server_b = common::serve_isolated(|| {
        // SAFETY: `test_lock()` and `serve_isolated`'s start lock serialize env changes.
        unsafe {
            env::remove_var("PHYSICIAN_SITE_DIR");
            env::set_var("PHYSICIAN_DATABASE_URL", &db_url_b);
            env::set_var("PHYSICIAN_AUTH", "required");
            env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let host = server.address.to_string();
    let base = format!("http://{host}");
    let base_b = format!("http://{}", server_b.address);
    let bearer = scale::sign_in(&host).await;
    let (outside_status, outside_login) = scale::post(
        &host,
        "/auth/sign-in",
        None,
        &serde_json::json!({ "email": "outside@other-clinic.test" }),
    )
    .await;
    assert_eq!(outside_status, 200, "{outside_login}");
    let outside_token = outside_login["token"].as_str().unwrap();
    let (verify_status, outside_verify) = scale::post(
        &host,
        "/auth/verify",
        None,
        &serde_json::json!({ "email": "outside@other-clinic.test", "token": outside_token }),
    )
    .await;
    assert_eq!(verify_status, 200, "{outside_verify}");
    let outside_bearer = outside_verify["session"].as_str().unwrap().to_owned();

    let http = reqwest::Client::new();

    // 1. Unauthenticated and cross-practice reads are refused (`401` and `404`).
    let unauth = http
        .get(format!(
            "{base}/patient-workspaces/{}/reviews",
            seed.workspace
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(unauth.status(), reqwest::StatusCode::UNAUTHORIZED);

    let cross_practice = http
        .get(format!(
            "{base}/patient-workspaces/{}/reviews",
            seed.workspace
        ))
        .bearer_auth(&outside_bearer)
        .send()
        .await
        .unwrap();
    assert_eq!(cross_practice.status(), reqwest::StatusCode::NOT_FOUND);

    // 2. Authorized list & detail return typed `PatientReviewsResponse` / `PatientReviewWire`.
    let list_resp = http
        .get(format!(
            "{base}/patient-workspaces/{}/reviews",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.status(), reqwest::StatusCode::OK);
    let list_body: PatientReviewsResponse = list_resp.json().await.unwrap();
    assert_eq!(list_body.reviews.len(), 2);
    let running_wire = list_body
        .reviews
        .iter()
        .find(|r| r.review_id == running_id)
        .expect("running review in list")
        .clone();
    let assessed_wire = list_body
        .reviews
        .iter()
        .find(|r| r.review_id == assessed_id)
        .expect("assessed review in list")
        .clone();
    assert_eq!(assessed_wire.evidence.len(), 2);
    assert_eq!(assessed_wire.actions.len(), 1);
    assert_eq!(assessed_wire.unfinished_work.len(), 1);

    let single_resp = http
        .get(format!(
            "{base}/patient-workspaces/{}/reviews/{assessed_id}",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .send()
        .await
        .unwrap();
    assert_eq!(single_resp.status(), reqwest::StatusCode::OK);
    let single_wire: PatientReviewWire = single_resp.json().await.unwrap();
    assert_eq!(single_wire, assessed_wire);

    // 3. InvalidState refusal (`409`) on completed review and StaleVersion refusal (`409`) on running review.
    let invalid_state_resp = http
        .post(format!(
            "{base}/patient-workspaces/{}/reviews/{assessed_id}/control",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .json(&ReviewControlRequest {
            review_id: assessed_id.clone(),
            version: assessed_wire.version.clone(),
            action: ReviewControlActionWire::Stop,
            idempotency_key: "invalid-stop-completed".into(),
        })
        .send()
        .await
        .unwrap();
    assert_eq!(invalid_state_resp.status(), reqwest::StatusCode::CONFLICT);
    let invalid_state_body: ReviewControlResponse = invalid_state_resp.json().await.unwrap();
    assert!(matches!(
        invalid_state_body,
        ReviewControlResponse::Refused {
            reason: ReviewControlRefusalReason::InvalidState,
            review: Some(_),
            ..
        }
    ));

    let stale_resp = http
        .post(format!(
            "{base}/patient-workspaces/{}/reviews/{running_id}/control",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .json(&ReviewControlRequest {
            review_id: running_id.clone(),
            version: ReviewVersion("rv-0000000000000000".into()),
            action: ReviewControlActionWire::Stop,
            idempotency_key: "stale-stop".into(),
        })
        .send()
        .await
        .unwrap();
    assert_eq!(stale_resp.status(), reqwest::StatusCode::CONFLICT);
    let stale_body: ReviewControlResponse = stale_resp.json().await.unwrap();
    assert!(matches!(
        stale_body,
        ReviewControlResponse::Refused {
            reason: ReviewControlRefusalReason::StaleVersion,
            review: Some(_),
            ..
        }
    ));

    // 4. Stop with matching version succeeds (`200 Accepted`), advances version, and replays
    //    idempotently both on the same instance and across a separate server instance (`base_b`)!
    let stop_req = ReviewControlRequest {
        review_id: running_id.clone(),
        version: running_wire.version.clone(),
        action: ReviewControlActionWire::Stop,
        idempotency_key: "stop-idempotent-1".into(),
    };
    let stop_resp = http
        .post(format!(
            "{base}/patient-workspaces/{}/reviews/{running_id}/control",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .json(&stop_req)
        .send()
        .await
        .unwrap();
    assert_eq!(stop_resp.status(), reqwest::StatusCode::OK);
    let stopped_wire = match stop_resp.json::<ReviewControlResponse>().await.unwrap() {
        ReviewControlResponse::Accepted { review } => *review,
        other => panic!("expected Accepted on Stop, got {other:?}"),
    };
    assert_ne!(stopped_wire.version, running_wire.version);

    for target_base in [&base, &base_b] {
        let replay_resp = http
            .post(format!(
                "{target_base}/patient-workspaces/{}/reviews/{running_id}/control",
                seed.workspace
            ))
            .bearer_auth(&bearer)
            .json(&stop_req)
            .send()
            .await
            .unwrap();
        assert_eq!(replay_resp.status(), reqwest::StatusCode::OK);
        let replayed = match replay_resp.json::<ReviewControlResponse>().await.unwrap() {
            ReviewControlResponse::Accepted { review } => *review,
            other => panic!("expected cross-instance Accepted replay, got {other:?}"),
        };
        assert_eq!(replayed.version, stopped_wire.version);
    }

    // Reusing `stop-idempotent-1` for a different action on Instance B returns `409 IdempotencyCollision`.
    let collision_resp = http
        .post(format!(
            "{base_b}/patient-workspaces/{}/reviews/{running_id}/control",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .json(&ReviewControlRequest {
            review_id: running_id.clone(),
            version: stopped_wire.version.clone(),
            action: ReviewControlActionWire::Resume,
            idempotency_key: "stop-idempotent-1".into(),
        })
        .send()
        .await
        .unwrap();
    assert_eq!(collision_resp.status(), reqwest::StatusCode::CONFLICT);
    assert!(matches!(
        collision_resp
            .json::<ReviewControlResponse>()
            .await
            .unwrap(),
        ReviewControlResponse::Refused {
            reason: ReviewControlRefusalReason::IdempotencyCollision,
            review: Some(_),
            ..
        }
    ));

    // 5. Resume with updated version succeeds (`200 Accepted`) and replays idempotently across `base_b`.
    let resume_req = ReviewControlRequest {
        review_id: running_id.clone(),
        version: stopped_wire.version.clone(),
        action: ReviewControlActionWire::Resume,
        idempotency_key: "resume-1".into(),
    };
    let resume_resp = http
        .post(format!(
            "{base}/patient-workspaces/{}/reviews/{running_id}/control",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .json(&resume_req)
        .send()
        .await
        .unwrap();
    assert_eq!(resume_resp.status(), reqwest::StatusCode::OK);
    let resumed_wire = match resume_resp.json::<ReviewControlResponse>().await.unwrap() {
        ReviewControlResponse::Accepted { review } => *review,
        other => panic!("expected Accepted on Resume, got {other:?}"),
    };
    assert!(resumed_wire.state.is_working());

    let resume_replay_b = http
        .post(format!(
            "{base_b}/patient-workspaces/{}/reviews/{running_id}/control",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .json(&resume_req)
        .send()
        .await
        .unwrap();
    assert_eq!(resume_replay_b.status(), reqwest::StatusCode::OK);
    assert!(matches!(
        resume_replay_b
            .json::<ReviewControlResponse>()
            .await
            .unwrap(),
        ReviewControlResponse::Accepted { .. }
    ));

    // 6. Panel removal (`ClinicDemographic`) immediately blocks review list, detail, and control with `404`.
    auth.set_membership(&staging_identity, &seed.workspace, Membership::Denied)
        .await
        .unwrap();
    for res in [
        http.get(format!(
            "{base}/patient-workspaces/{}/reviews",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .send()
        .await
        .unwrap(),
        http.get(format!(
            "{base}/patient-workspaces/{}/reviews/{assessed_id}",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .send()
        .await
        .unwrap(),
        http.post(format!(
            "{base}/patient-workspaces/{}/reviews/{running_id}/control",
            seed.workspace
        ))
        .bearer_auth(&bearer)
        .json(&ReviewControlRequest {
            review_id: running_id,
            version: resumed_wire.version,
            action: ReviewControlActionWire::Stop,
            idempotency_key: "blocked-stop".into(),
        })
        .send()
        .await
        .unwrap(),
    ] {
        assert_eq!(res.status(), reqwest::StatusCode::NOT_FOUND);
    }

    unsafe {
        env::remove_var("PHYSICIAN_DATABASE_URL");
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn patient_page_reviews_controls_reload_access_and_viewports() {
    let _guard = test_lock().lock().await;
    let database = scale::Database::create("reviews_browser")
        .await
        .expect("browser qualification requires PHYSICIAN_DATABASE_URL or PHYSICIAN_RUNTIME_TEST_DATABASE_URL");
    scale::staging_account(&database.url).await;
    let site = build_site();
    let shots_dir = screenshot_folder();
    fs::create_dir_all(&shots_dir).expect("create screenshot folder");

    let store = local_product::runtime::Store::connect(&database.url)
        .await
        .unwrap();
    let auth = Arc::new(
        physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
            .await
            .unwrap(),
    );
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    let roles = scale::models::roles();

    let seed = seed_synthetic_patient("SYN-REV-UI-01", &auth, &store, &roles, &skills).await;

    // 1. Seed Completed(Assessed) and Completed(Quiet) routine reviews on Sofia Navarro.
    let assessed_id =
        run_assessed_review(&store, &auth, &roles, &skills, &seed, "rev-ui-assessed").await;
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let quiet_id = run_quiet_review(&store, &auth, &roles, &skills, &seed, "rev-ui-quiet").await;
    tokio::time::sleep(Duration::from_millis(1100)).await;

    let db_url = database.url.clone();
    let server = common::serve_isolated(|| {
        // SAFETY: `test_lock()` and `serve_isolated`'s start lock serialize env changes.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::set_var("PHYSICIAN_DATABASE_URL", &db_url);
            env::set_var("PHYSICIAN_AUTH", "required");
            env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let base = format!("http://{}", server.address);

    // 2. Sign in and verify routine patient reviews do NOT show on Today!
    let mut tab = Tab::open(&base);
    tab.set_viewport(1440, 900, true);
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    let today_dom = tab.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    assert!(
        !today_dom.contains("data-patient-reviews")
            && !today_dom.contains("Serum creatinine is 1.12 mg/dL")
            && !today_dom.contains("Harborview EP ablation operative report"),
        "routine patient reviews must not appear on Today:\n{today_dom}"
    );

    // 3. Admit a 3rd review (`PhysicianRequest`), claim it, execute 1 chart search step so its
    //    live trace has recorded steps, and hold Turn 2 open so it stays genuinely `Running`.
    let staging_identity = PhysicianIdentity {
        practice_id: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        physician_id: "staging-physician".to_owned(),
    };
    let admitted_running = store
        .submit_physician_patient_review(
            auth.as_ref(),
            &staging_identity,
            &seed.workspace,
            "rev-ui-running",
            "Re-check renal dosing threshold for apixaban before refill",
            &roles,
            &skills,
        )
        .await
        .expect("admit running review");
    let running_saved = match admitted_running {
        AdmittedPatientReview::New { review, .. }
        | AdmittedPatientReview::Duplicate { review, .. } => *review,
    };
    let running_id = running_saved.review_id.clone();
    let running_scope = running_saved.practice_scope();
    let claim = store
        .claim_practice_job(&running_scope, 120, auth.as_ref())
        .await
        .unwrap()
        .expect("claim running review");

    let step_recorded = Arc::new(Notify::new());
    let release_turn2 = Arc::new(Notify::new());

    struct LiveRunningProvider {
        turn: usize,
        step_recorded: Arc<Notify>,
        release_turn2: Arc<Notify>,
    }

    impl engine::recipe::Provider for LiveRunningProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "live-search-1".into(),
                        name: SEARCH_TOOL.into(),
                        arguments: serde_json::json!({
                            "query": "creatinine",
                            "record_type": null,
                            "after": null
                        })
                        .to_string(),
                    })],
                    receipt: serde_json::json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    self.step_recorded.notify_waiters();
                    self.release_turn2.notified().await;
                    Err(std::io::Error::other("cancelled after stop").into())
                }
                other => panic!("unexpected live running turn {other}"),
            }
        }
    }

    let bg_store = store.clone();
    let bg_auth = Arc::clone(&auth);
    let step_recorded_wait = Arc::clone(&step_recorded);
    let release_turn2_send = Arc::clone(&release_turn2);
    let running_task = tokio::spawn(async move {
        let mut host = PracticeJobHost {
            store: bg_store,
            claim,
            authority: bg_auth.as_ref(),
        };
        let _ = host
            .run(
                &mut LiveRunningProvider {
                    turn: 0,
                    step_recorded,
                    release_turn2,
                },
                engine::RunLimits::default(),
                &engine::Cancellation::default(),
            )
            .await;
    });
    step_recorded_wait.notified().await;

    // Physician-requested running review shows as recognizable running work on Today while
    // keeping the patient-reviews workspace surface exclusively on the patient page.
    tab.reload();
    let today_running_dom = tab.wait_for_text(
        "Reviewing patient chart as requested",
        Duration::from_secs(20),
    );
    assert!(
        !today_running_dom.contains("data-patient-reviews"),
        "patient reviews panel must stay on the patient page:\n{today_running_dom}"
    );

    // 4. Open patient page -> verify Running, Completed(Quiet), and Completed(Assessed) rows!
    let patient_url = format!("{base}/?patient={}", seed.workspace);
    tab.navigate(&patient_url);
    tab.wait_for_text("Sofia Navarro", Duration::from_secs(20));
    tab.wait_for_text("SELECTED REVIEW", Duration::from_secs(15));
    tab.wait_for_text("Searched patient chart", Duration::from_secs(15));
    assert_eq!(tab.page_errors(), Vec::<String>::new());

    // Running review is newest, so it opens selected by default with `Stop` and no other control.
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-selected-review-id"),
        running_id
    );
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "running"
    );
    assert_eq!(
        tab.control_slugs(),
        vec!["stop".to_owned()],
        "running review must offer only Stop"
    );
    let running_row_text = tab.text(&format!("[data-review-row=\"{running_id}\"]"));
    assert!(
        running_row_text.contains("Reviewing · Re-check renal dosing threshold"),
        "running row must use Reviewing style: {running_row_text}"
    );

    // Select Completed(Quiet) review -> verify `"Reviewed, nothing to do"` and zero controls.
    tab.click(&format!("[data-review-row=\"{quiet_id}\"]"));
    tab.wait_for_text("Reviewed, nothing to do", Duration::from_secs(10));
    assert_eq!(tab.query_param("review"), quiet_id);
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "completed_quiet"
    );
    assert!(
        tab.control_slugs().is_empty(),
        "completed quiet review must not render lifecycle controls"
    );

    // Select Completed(Assessed) review -> verify Trigger, Evidence (with ReaderDialog!), Outcome, and Unfinished work!
    tab.click(&format!("[data-review-row=\"{assessed_id}\"]"));
    tab.wait_for_text(
        "Harborview EP ablation operative report",
        Duration::from_secs(10),
    );
    assert_eq!(tab.query_param("review"), assessed_id);
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "completed_assessed"
    );
    assert!(
        tab.control_slugs().is_empty(),
        "completed assessed review must not render lifecycle controls"
    );

    let trigger_text = tab.text("[data-review-trigger=\"true\"]");
    assert!(
        trigger_text
            .contains("New records: Observation (2026-08-22), Document reference (2026-08-20)")
            && !trigger_text.contains("T09:15:00Z"),
        "selected review must show clean physician-worded trigger dates and record types: {trigger_text}"
    );
    let evidence_text = tab.text("[data-review-evidence=\"true\"]");
    assert!(
        evidence_text.contains("Serum Creatinine")
            && evidence_text.contains("CONSULTATION NOTE (2026-08-20)"),
        "selected review must show its evidence references: {evidence_text}"
    );
    let outcome_text = tab.text("[data-review-outcome=\"completed_assessed\"]");
    assert!(
        outcome_text.contains("Serum creatinine is 1.12 mg/dL")
            && outcome_text.contains("Updated clinical memory: Apixaban renal monitoring"),
        "selected review must show its outcome and recorded actions: {outcome_text}"
    );
    let unfinished_text = tab.text("[data-review-unfinished=\"true\"]");
    assert!(
        unfinished_text.contains("Harborview EP ablation operative report")
            && unfinished_text.contains("Missing record")
            && unfinished_text.contains("Verify pulmonary vein isolation details"),
        "selected review must show its unfinished work and limit pill: {unfinished_text}"
    );

    // Expand Basis and limitations disclosure on the assessed outcome.
    tab.click("[data-review-outcome=\"completed_assessed\"] .basis-disclosure button");
    tab.wait_for_text(
        "external Harborview EP operative report remains missing",
        Duration::from_secs(10),
    );

    // Click citation chip in Evidence -> ReaderDialog opens with the cited record quote!
    tab.click("[data-review-evidence=\"true\"] button[aria-label*=\"Source 1:\"]");
    tab.wait_for_text(
        "Persistent atrial fibrillation anticoagulated with apixaban 5 mg twice daily.",
        Duration::from_secs(10),
    );
    tab.press_key(Key::ESCAPE);
    std::thread::sleep(Duration::from_millis(250));

    tab.scroll_into_view("[data-selected-review-id]");
    std::thread::sleep(Duration::from_millis(250));
    tab.save_shot(&shots_dir, "patient-reviews-completed-desktop.png");

    // 5. AC1 + AC3: Reload reopens the same patient AND selected review (`?review=`)!
    tab.reload();
    tab.wait_for_text(
        "Harborview EP ablation operative report",
        Duration::from_secs(20),
    );
    assert_eq!(tab.query_param("patient"), seed.workspace);
    assert_eq!(tab.query_param("review"), assessed_id);
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-selected-review-id"),
        assessed_id
    );
    assert!(
        tab.text("[data-review-trigger=\"true\"]")
            .contains("New records:")
            && tab
                .text("[data-review-unfinished=\"true\"]")
                .contains("Harborview EP ablation operative report"),
        "reloaded selected review must keep trigger, evidence, outcome, and unfinished work"
    );

    // 6. AC2: Select Running review -> intercept `POST .../control` -> verify ID + version sent,
    //    `pending` state without optimistic flip, forced failure (`failed` alert), and `done`!
    tab.click(&format!("[data-review-row=\"{running_id}\"]"));
    tab.wait_for_text("Searched patient chart", Duration::from_secs(10));
    let running_version = tab.attr("[data-selected-review-id]", "data-review-version");
    assert!(
        running_version.starts_with("rv-"),
        "expected non-empty review version, got {running_version:?}"
    );

    tab.intercept_controls();
    tab.click("[data-review-control=\"stop\"]");
    let paused_stop = tab.held_post_control();
    let stop_payload = held_body(&paused_stop);
    assert_eq!(stop_payload["review_id"], running_id);
    assert_eq!(stop_payload["version"], running_version);
    assert_eq!(stop_payload["action"], "stop");
    assert!(
        !stop_payload["idempotency_key"]
            .as_str()
            .unwrap_or_default()
            .is_empty()
    );

    // While request is paused in flight, UI shows `pending` and does NOT optimistically flip status!
    tab.wait_for_text("Stopping review…", Duration::from_secs(5));
    assert_eq!(
        tab.attr("[data-review-control-state]", "data-review-control-state"),
        "pending"
    );
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "running"
    );

    // Fulfill forced 503 failure -> UI shows `failed` alert and keeps review `running`!
    tab.fulfill_status(
        &paused_stop,
        503,
        &serde_json::json!({
            "message": "Forced control failure: Coda couldn’t confirm stopping this review."
        })
        .to_string(),
    );
    tab.wait_for_text(
        "Forced control failure: Coda couldn’t confirm stopping this review.",
        Duration::from_secs(10),
    );
    assert_eq!(
        tab.attr("[data-review-control-state]", "data-review-control-state"),
        "failed"
    );
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "running"
    );
    assert!(
        tab.exists("[data-review-control-state=\"failed\"] [role=\"alert\"]"),
        "failed control answer must carry role=alert"
    );
    tab.scroll_into_view("[data-selected-review-id]");
    std::thread::sleep(Duration::from_millis(200));
    tab.save_shot(&shots_dir, "patient-reviews-failed-control-desktop.png");

    // Disable interception and click `Check again` (reusing exact ReviewControlRequest!) -> server returns `done`!
    tab.stop_intercepting();
    tab.click_named(
        "[data-review-control-state=\"failed\"] button",
        "Check again",
    );
    tab.wait_for_text("Review stopped.", Duration::from_secs(15));
    tab.wait_for_text(
        "You stopped Coda before it finished.",
        Duration::from_secs(10),
    );
    assert_eq!(
        tab.attr("[data-review-control-state]", "data-review-control-state"),
        "done"
    );
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "held_stopped"
    );
    assert_eq!(
        tab.control_slugs(),
        vec!["resume".to_owned()],
        "held stopped review must offer only Resume"
    );

    // Let the cancelled background host finish cleanly.
    release_turn2_send.notify_waiters();
    let _ = running_task.await;

    // Expand `"You stopped Coda before it finished."` disclosure and verify recorded steps appear!
    assert_eq!(
        tab.attr(
            "[data-review-outcome=\"held_stopped\"] button.work-trace-line",
            "aria-expanded"
        ),
        "false"
    );
    tab.click("[data-review-outcome=\"held_stopped\"] button.work-trace-line");
    assert_eq!(
        tab.attr(
            "[data-review-outcome=\"held_stopped\"] button.work-trace-line",
            "aria-expanded"
        ),
        "true"
    );
    tab.wait_for_text("Searched patient chart", Duration::from_secs(5));
    std::thread::sleep(Duration::from_millis(200));
    tab.save_shot(&shots_dir, "patient-reviews-stopped-desktop.png");

    // 7. AC1 + AC2: Click `Resume` -> verify ID + updated version sent, `pending`, then `done` + working state!
    let stopped_version = tab.attr("[data-selected-review-id]", "data-review-version");
    assert_ne!(stopped_version, running_version);

    tab.intercept_controls();
    tab.click("[data-review-control=\"resume\"]");
    let paused_resume = tab.held_post_control();
    let resume_payload = held_body(&paused_resume);
    assert_eq!(resume_payload["review_id"], running_id);
    assert_eq!(resume_payload["version"], stopped_version);
    assert_eq!(resume_payload["action"], "resume");
    tab.wait_for_text("Resuming review…", Duration::from_secs(5));
    assert_eq!(
        tab.attr("[data-review-control-state]", "data-review-control-state"),
        "pending"
    );
    tab.release(&paused_resume);
    tab.stop_intercepting();

    tab.wait_for_text("Review resumed.", Duration::from_secs(15));
    assert_eq!(
        tab.attr("[data-review-control-state]", "data-review-control-state"),
        "done"
    );
    assert_eq!(
        tab.control_slugs(),
        vec!["stop".to_owned()],
        "resumed review must return to working state with Stop"
    );
    assert!(
        tab.text(&format!("[data-review-row=\"{running_id}\"]"))
            .contains("Reviewing · Re-check renal dosing threshold"),
        "resumed review row must return to working Reviewing style"
    );

    // 8. AC1 + AC2: Settle resumed review at run limit (`"Couldn’t finish"`) -> verify `Try again`!
    let resumed_claim = store
        .claim_practice_job(&running_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim resumed review");

    struct LimitProvider;
    impl engine::recipe::Provider for LimitProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            Ok(engine::Turn {
                items: vec![engine::Item::Call(engine::Invocation {
                    id: "limit-search".into(),
                    name: SEARCH_TOOL.into(),
                    arguments: serde_json::json!({
                        "query": "apixaban",
                        "record_type": null,
                        "after": null
                    })
                    .to_string(),
                })],
                receipt: serde_json::json!({"turn": "limit"}),
                usage: Default::default(),
            })
        }
    }

    {
        let mut host = PracticeJobHost {
            store: store.clone(),
            claim: resumed_claim,
            authority: auth.as_ref(),
        };
        host.run(
            &mut LimitProvider,
            engine::RunLimits {
                max_calls: 1,
                ..engine::RunLimits::default()
            },
            &engine::Cancellation::default(),
        )
        .await
        .expect("settle review at turn limit");
    }

    // Working background poll picks up the `"Couldn’t finish"` transition automatically!
    tab.wait_for_text(
        "Coda couldn’t finish this review before reaching its run limit.",
        Duration::from_secs(15),
    );
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "held_limit"
    );
    assert_eq!(
        tab.control_slugs(),
        vec!["try_again".to_owned()],
        "couldn't-finish review must offer Try again"
    );
    tab.scroll_into_view("[data-patient-reviews=\"true\"]");
    std::thread::sleep(Duration::from_millis(200));
    tab.save_shot(&shots_dir, "patient-reviews-couldnt-finish-desktop.png");

    let limit_version = tab.attr("[data-selected-review-id]", "data-review-version");
    tab.intercept_controls();
    tab.click("[data-review-control=\"try_again\"]");
    let paused_retry = tab.held_post_control();
    let retry_payload = held_body(&paused_retry);
    assert_eq!(retry_payload["review_id"], running_id);
    assert_eq!(retry_payload["version"], limit_version);
    assert_eq!(retry_payload["action"], "try_again");
    tab.wait_for_text("Retrying review…", Duration::from_secs(5));
    assert_eq!(
        tab.attr("[data-review-control-state]", "data-review-control-state"),
        "pending"
    );
    tab.release(&paused_retry);
    tab.stop_intercepting();

    tab.wait_for_text("Review started again.", Duration::from_secs(15));
    assert_eq!(
        tab.attr("[data-review-control-state]", "data-review-control-state"),
        "done"
    );
    assert_eq!(
        tab.control_slugs(),
        vec!["stop".to_owned()],
        "retried review must return to working state with Stop"
    );

    // Also exhaust retry allowance to verify `Failed` renders `"Couldn’t finish"` + `Try again` too!
    for _ in 0..5 {
        let Some(failed_claim) = store
            .claim_practice_job(&running_scope, 60, auth.as_ref())
            .await
            .unwrap()
        else {
            break;
        };
        store
            .settle_practice_job_attempt(
                &failed_claim,
                engine::AttemptFailure::Failed,
                local_product::runtime::delivery::Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
    }
    tab.wait_for_text("Coda couldn’t finish this review.", Duration::from_secs(15));
    assert_eq!(
        tab.attr("[data-selected-review-id]", "data-review-status"),
        "failed"
    );
    assert_eq!(tab.control_slugs(), vec!["try_again".to_owned()]);

    // 9. AC3: Desktop (`1440x900`) and Phone (`390x844`) layout + horizontal overflow checks!
    tab.click(&format!("[data-review-row=\"{assessed_id}\"]"));
    tab.wait_for_text(
        "Harborview EP ablation operative report",
        Duration::from_secs(10),
    );
    tab.set_viewport(1440, 900, true);
    std::thread::sleep(Duration::from_millis(250));
    tab.assert_no_horizontal_overflow("desktop 1440x900");

    tab.set_viewport(390, 844, true);
    std::thread::sleep(Duration::from_millis(300));
    tab.assert_no_horizontal_overflow("phone 390x844");
    tab.scroll_into_view("[data-patient-reviews=\"true\"]");
    std::thread::sleep(Duration::from_millis(250));
    tab.save_shot(&shots_dir, "patient-reviews-phone.png");

    // 10. AC3: Reload re-checks panel & practice access and never leaks clinical review content!
    tab.set_viewport(1440, 900, true);
    auth.set_membership(&staging_identity, &seed.workspace, Membership::Denied)
        .await
        .unwrap();
    tab.reload();
    let demographic_only_dom =
        tab.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(20));
    assert!(
        !demographic_only_dom.contains("data-patient-reviews")
            && !demographic_only_dom.contains("Serum creatinine is 1.12 mg/dL")
            && !demographic_only_dom.contains("Harborview EP ablation operative report"),
        "reload after panel removal must hide reviews completely:\n{demographic_only_dom}"
    );
    std::thread::sleep(Duration::from_millis(200));
    tab.save_shot(&shots_dir, "patient-reviews-access-revoked-desktop.png");

    // Also check outside-practice `Denied` (`404`) reload -> `"That patient is not available."`.
    tab.navigate(&format!(
        "{base}/?patient=pw-outside-practice-denied&review={assessed_id}"
    ));
    let denied_dom = tab.wait_for_text("That patient is not available.", Duration::from_secs(20));
    assert!(
        !denied_dom.contains("data-patient-reviews") && !denied_dom.contains("Sofia Navarro"),
        "outside-practice patient reload must show unavailable notice only:\n{denied_dom}"
    );
    assert_eq!(tab.page_errors(), Vec::<String>::new());

    drop(tab);
    unsafe {
        env::remove_var("PHYSICIAN_DATABASE_URL");
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
    }
}

fn screenshot_folder() -> PathBuf {
    if let Some(folder) = env::var_os("PHYSICIAN_PROOF_SHOTS") {
        return PathBuf::from(folder);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/screenshots/patient-reviews")
}

fn build_site() -> PathBuf {
    BUILT_SITE
        .get_or_init(|| {
            let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            let target = workspace.join("target/coda-physician-wasm");
            let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
            let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            cargo_without_parent_package(&mut cargo);
            let status = cargo
                .current_dir(&workspace)
                .env("CARGO_TARGET_DIR", &target)
                .env_remove("PHYSICIAN_API_URL")
                .args([
                    "build",
                    "-p",
                    "physician-app",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--features",
                    "web",
                ])
                .status()
                .expect("cargo build wasm");
            assert!(status.success(), "physician-app wasm build failed");
            let site = workspace.join("target/physician-web-reviews");
            fs::create_dir_all(&site).unwrap();
            let status = Command::new(wasm_bindgen())
                .args([
                    "--target",
                    "web",
                    "--out-dir",
                    site.to_str().unwrap(),
                    wasm.to_str().unwrap(),
                ])
                .status()
                .expect("wasm-bindgen");
            assert!(status.success(), "wasm-bindgen failed");
            site
        })
        .clone()
}

fn cargo_without_parent_package(command: &mut Command) {
    command.env_clear();
    for key in [
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
    ] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
    if let Some(paths) = env::var_os("PATH") {
        let filtered = env::join_paths(
            env::split_paths(&paths)
                .filter(|dir| !dir.to_string_lossy().contains(".local/share/swiftly")),
        )
        .expect("join PATH");
        command.env("PATH", filtered);
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(path) = find_on_path("wasm-bindgen") {
        return path;
    }
    let local = env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen is required to build the physician page");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

struct Tab {
    browser: Browser,
    held: Vec<serde_json::Value>,
}

impl Tab {
    fn open(url: &str) -> Self {
        let mut browser = Browser::launch();
        browser
            .page()
            .call_session("Page.enable", serde_json::json!({}));
        browser.watch_page_errors();
        browser.page().navigate(url);
        Self {
            browser,
            held: Vec::new(),
        }
    }

    fn eval(&mut self, expression: &str) -> serde_json::Value {
        self.browser.page().evaluate(expression)
    }

    fn call(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        self.browser.page().call_session(method, params)
    }

    fn step(&mut self, name: &str, args: serde_json::Value) -> serde_json::Value {
        let mut arguments = vec![serde_json::json!(name)];
        arguments.extend(args.as_array().expect("step args").iter().cloned());
        self.browser
            .page()
            .call_function(include_str!("js/site.js"), &arguments)
    }

    fn navigate(&mut self, url: &str) {
        self.browser.page().navigate(url);
    }

    fn click(&mut self, selector: &str) {
        self.browser.click(selector);
    }

    fn click_named(&mut self, selector: &str, words: &str) {
        self.step("clickNamed", serde_json::json!([selector, words]));
    }

    fn press_key(&mut self, key: Key) {
        self.browser.press_key(key);
    }

    fn request_sign_in(&mut self, email: &str) {
        self.browser.type_into("input[type=\"email\"]", email);
        std::thread::sleep(Duration::from_millis(50));
        self.click("button.control-primary");
    }

    fn wait_for_text(&mut self, expected: &str, timeout: Duration) -> String {
        let started = Instant::now();
        loop {
            let snapshot = self.eval(
                "({text: document.body?.innerText ?? '', html: document.documentElement?.outerHTML ?? ''})",
            );
            let html = snapshot["html"].as_str().unwrap_or_default();
            if snapshot["text"]
                .as_str()
                .is_some_and(|text| text.contains(expected))
            {
                return html.to_owned();
            }
            if started.elapsed() > timeout {
                panic!("timed out waiting for {expected:?} in DOM:\n{html}");
            }
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn reload(&mut self) {
        let previous = self.document_loader();
        self.call("Page.reload", serde_json::json!({}));
        let started = Instant::now();
        while self.document_loader() == previous {
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "reload did not replace the document"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn document_loader(&mut self) -> String {
        self.call("Page.getFrameTree", serde_json::json!({}))
            .pointer("/result/frameTree/frame/loaderId")
            .and_then(|id| id.as_str())
            .expect("main frame document loader")
            .to_owned()
    }

    fn query_param(&mut self, key: &str) -> String {
        self.browser
            .page()
            .call_function(
                "function(k) { return new URL(location.href).searchParams.get(k) ?? ''; }",
                &[serde_json::json!(key)],
            )
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }

    fn attr(&mut self, selector: &str, name: &str) -> String {
        self.browser
            .page()
            .call_function(
                "function(sel, attr) { return document.querySelector(sel)?.getAttribute(attr) ?? ''; }",
                &[serde_json::json!(selector), serde_json::json!(name)],
            )
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }

    fn text(&mut self, selector: &str) -> String {
        self.browser
            .page()
            .call_function(
                "function(sel) { return document.querySelector(sel)?.innerText ?? ''; }",
                &[serde_json::json!(selector)],
            )
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }

    fn exists(&mut self, selector: &str) -> bool {
        self.browser
            .page()
            .call_function(
                "function(sel) { return document.querySelector(sel) !== null; }",
                &[serde_json::json!(selector)],
            )
            .as_bool()
            .unwrap_or(false)
    }

    fn control_slugs(&mut self) -> Vec<String> {
        self.eval(
            "[...document.querySelectorAll('[data-review-control]')].map(b => b.getAttribute('data-review-control') ?? '')",
        )
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
    }

    fn scroll_into_view(&mut self, selector: &str) {
        let _ = self.browser.page().call_function(
            "function(sel) { document.querySelector(sel)?.scrollIntoView({ block: 'start', behavior: 'instant' }); return true; }",
            &[serde_json::json!(selector)],
        );
    }

    fn assert_no_horizontal_overflow(&mut self, label: &str) {
        let metrics = self.eval(
            "({ scrollWidth: document.documentElement.scrollWidth, innerWidth: window.innerWidth })",
        );
        let scroll_width = metrics["scrollWidth"].as_f64().unwrap_or(0.0);
        let inner_width = metrics["innerWidth"].as_f64().unwrap_or(0.0);
        assert!(
            scroll_width <= inner_width + 1.0,
            "horizontal overflow at {label}: scrollWidth={scroll_width} > innerWidth={inner_width}"
        );
    }

    fn intercept_controls(&mut self) {
        self.call(
            "Fetch.enable",
            serde_json::json!({
                "patterns": [{
                    "urlPattern": "*/patient-workspaces/*/reviews/*/control*",
                    "requestStage": "Request"
                }]
            }),
        );
    }

    fn stop_intercepting(&mut self) {
        self.held.extend(self.browser.take_paused_requests());
        for request in std::mem::take(&mut self.held) {
            self.release(&request);
        }
        let _ = self.call("Fetch.disable", serde_json::json!({}));
    }

    fn held_post_control(&mut self) -> serde_json::Value {
        let started = Instant::now();
        loop {
            self.held.extend(self.browser.take_paused_requests());
            let mut found = None;
            for request in std::mem::take(&mut self.held) {
                if found.is_none() && request["request"]["method"] == "POST" {
                    found = Some(request);
                } else {
                    self.held.push(request);
                }
            }
            if let Some(request) = found {
                return request;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "no POST control request was paused by Fetch.enable"
            );
            std::thread::sleep(Duration::from_millis(25));
        }
    }

    fn release(&mut self, request: &serde_json::Value) {
        let _ = self.browser.page().try_call_session(
            "Fetch.continueRequest",
            serde_json::json!({ "requestId": request["requestId"] }),
        );
    }

    fn fulfill_status(&mut self, request: &serde_json::Value, status: u16, body: &str) {
        let answer = self.browser.page().try_call_session(
            "Fetch.fulfillRequest",
            serde_json::json!({
                "requestId": request["requestId"],
                "responseCode": status,
                "responseHeaders": [{ "name": "content-type", "value": "application/json" }],
                "body": base64::engine::general_purpose::STANDARD.encode(body),
            }),
        );
        if let Err(error) = answer
            && error["message"] != "Invalid InterceptionId."
        {
            panic!("Fetch.fulfillRequest failed: {error}");
        }
    }

    fn set_viewport(&mut self, width: u32, height: u32, reduced_motion: bool) {
        self.call(
            "Emulation.setDeviceMetricsOverride",
            serde_json::json!({
                "width": width,
                "height": height,
                "deviceScaleFactor": 1,
                "mobile": width < 600,
            }),
        );
        self.call(
            "Emulation.setEmulatedMedia",
            serde_json::json!({
                "features": [{
                    "name": "prefers-reduced-motion",
                    "value": if reduced_motion { "reduce" } else { "no-preference" },
                }]
            }),
        );
    }

    fn save_shot(&mut self, dir: &Path, filename: &str) {
        let shot = self.call(
            "Page.captureScreenshot",
            serde_json::json!({ "format": "png" }),
        );
        let encoded = shot["result"]["data"]
            .as_str()
            .expect("base64 PNG screenshot");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .expect("decode PNG screenshot");
        fs::write(dir.join(filename), bytes).expect("write PNG screenshot");
    }

    fn page_errors(&mut self) -> Vec<String> {
        self.browser
            .take_page_errors()
            .into_iter()
            // Ignore expected HTTP 404/409/503 network log entries Chrome emits on forced refusals.
            .filter(|err| {
                !err.starts_with(
                    "browser: Failed to load resource: the server responded with a status of ",
                )
            })
            .collect()
    }
}

fn held_body(request: &serde_json::Value) -> serde_json::Value {
    let text = match request["request"]["postData"].as_str() {
        Some(text) => text.to_owned(),
        None => request["request"]["postDataEntries"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|entry| entry["bytes"].as_str())
            .filter_map(|bytes| base64::engine::general_purpose::STANDARD.decode(bytes).ok())
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .collect(),
    };
    serde_json::from_str(&text).unwrap_or(serde_json::Value::Null)
}
