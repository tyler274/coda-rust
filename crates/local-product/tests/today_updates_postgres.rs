//! Real PostgreSQL integration tests for the Today and Updates read model,
//! per-physician seen state, live panel authorization, practice-wide source
//! outage collapsing, and partial-failure completeness rules.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, Weak};

use local_product::initial_understanding::SummaryClaimInput;
use local_product::patient_reviews::{
    QuietReviewReason, ReviewCitation, ReviewOutcome, ReviewTrigger, UnfinishedReviewWork,
};
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::concerns::{
    CommandKey, ConcernUrgency, ExactRecordReference, SourceWork,
};
use local_product::runtime::reviews::ReviewLifecycleState;
use local_product::runtime::sources::SourcePull;
use local_product::runtime::today_updates::{
    ClearItemDoneRequest, OpenItemRequest, ProducerKind, ReadCompleteness, RecognizableRunKind,
    SeenItemKey, SeenStanding, TodayDoneResult, TodayRowPayload, TodayUpdatesError,
    TodayUpdatesProbe, TodayVisitSlot, UnhandledWorkKind, UpdateEntryKind,
    fixture::{
        SeedConcern, SeedConcernStanding, SeedReview, SeedRunningConversation, SeedSourceOutcome,
        SeedVisitBriefing, SeedVisitPrepState, SeedVisitPreparation, revise_seeded_concern,
        seed_concern, seed_failed_initial_understanding, seed_patient_summary, seed_review,
        seed_running_conversation, seed_source_status, seed_visit_briefing, seed_visit_preparation,
    },
};
use local_product::runtime::understanding::{PatientSummary, SummaryCitation, SummarySegment};
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, async_trait};
use local_product::sha256_hex;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::json;
use uuid::Uuid;

/// Fixed clinical reference time (`2026-11-12T15:00:00Z`).
const BASE_NOW_UNIX: u64 = 1_794_495_600;
const DAY_SECONDS: i64 = 86_400;

struct LocalCluster {
    cluster_dir: PathBuf,
    data_dir: PathBuf,
    admin_url: String,
}

impl Drop for LocalCluster {
    fn drop(&mut self) {
        let _ = Command::new("pg_ctl")
            .args([
                "-D",
                self.data_dir.to_str().unwrap_or_default(),
                "-m",
                "immediate",
                "stop",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = std::fs::remove_dir_all(&self.cluster_dir);
    }
}

static LOCAL_CLUSTER: StdMutex<Weak<LocalCluster>> = StdMutex::new(Weak::new());
static MIGRATE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn acquire_database() -> Option<(String, Option<Arc<LocalCluster>>)> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(url) = std::env::var(key) {
            let trimmed = url.trim().to_owned();
            if !trimmed.is_empty() {
                return Some((trimmed, None));
            }
        }
    }

    let mut slot = LOCAL_CLUSTER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.upgrade() {
        return Some((existing.admin_url.clone(), Some(existing)));
    }

    let port = {
        use std::net::TcpListener;
        (0..500)
            .find_map(|_| {
                let candidate = 22_000 + (Uuid::new_v4().as_u128() % 9_000) as u16;
                TcpListener::bind(("127.0.0.1", candidate))
                    .ok()
                    .map(|_| candidate)
            })
            .expect("free loopback port for local postgres")
    };
    let cluster_dir =
        std::env::temp_dir().join(format!("coda-tu-pg-{}-{}", std::process::id(), port));
    let _ = std::fs::remove_dir_all(&cluster_dir);
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    std::fs::create_dir_all(&data_dir).ok()?;
    std::fs::create_dir_all(&sock_dir).ok()?;

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str()?,
            "-U",
            "postgres",
            "-A",
            "trust",
            "--no-locale",
            "-E",
            "UTF8",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !init.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let log_file = cluster_dir.join("postgres.log");
    let opts = format!("-F -p {port} -h 127.0.0.1 -k {}", sock_dir.to_str()?);
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str()?,
            "-l",
            log_file.to_str()?,
            "-o",
            &opts,
            "-w",
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !start.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
    });
    *slot = Arc::downgrade(&cluster);
    Some((cluster.admin_url.clone(), Some(cluster)))
}

async fn connect_stores() -> Option<(
    Store,
    Arc<PostgresAuthStore>,
    String,
    Option<Arc<LocalCluster>>,
)> {
    let Some((url, cluster)) = acquire_database() else {
        assert!(
            std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
            "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
        );
        return None;
    };
    let _guard = MIGRATE_LOCK.lock().await;
    let store = Store::connect(&url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    drop(_guard);
    Some((store, auth, url, cluster))
}

struct PostgresWorkspaceAuthority;

#[async_trait]
impl Authority for PostgresWorkspaceAuthority {
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::check_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
    now_unix: u64,
) -> (PhysicianIdentity, String) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let invite = auth
        .invite(practice_id, &email, label, true, now_unix)
        .await
        .expect("invite physician");
    auth.redeem_invitation(&email, &invite, now_unix)
        .await
        .expect("redeem invitation")
}

async fn seed_chart_record(
    store: &Store,
    workspace: &str,
    patient_name: &str,
    candidate_id: &str,
    revision: &str,
    excerpt: &str,
    pulled_at: i64,
) -> ExactRecordReference {
    let content = json!({
        "resourceType": "Observation",
        "id": candidate_id,
        "note": excerpt,
    })
    .to_string();
    let pull = SourcePull {
        source_id: "regional-ehr".to_owned(),
        source_label: "Regional EHR".to_owned(),
        patient: SourcePatient {
            source_patient_id: workspace.to_owned(),
            display_name: patient_name.to_owned(),
            birth_date: Some("1968-05-14".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "MRN".to_owned(),
                value: format!("MRN-{workspace}"),
            }],
        },
        pulled_at,
    };
    let rev = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "regional-ehr".to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "Laboratory result".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDate".to_owned(),
            value: "2026-11-12".to_owned(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: vec![ReadablePart {
            label: "Result summary".to_owned(),
            text: excerpt.to_owned(),
        }],
        provenance: json!({"system": "regional-ehr", "id": candidate_id}),
    };
    store
        .record_chart(workspace, &[rev], &pull)
        .await
        .expect("record chart");
    ExactRecordReference::new(candidate_id, revision).expect("valid ref")
}

struct ForcedFaultProbe {
    fail_producers: BTreeSet<ProducerKind>,
    fail_open_keys: BTreeSet<String>,
}

impl TodayUpdatesProbe for ForcedFaultProbe {
    fn check_producer(&self, producer: ProducerKind) -> Result<(), String> {
        if self.fail_producers.contains(&producer) {
            return Err(format!("simulated {} storage timeout", producer.label()));
        }
        Ok(())
    }

    fn check_open_item(&self, _workspace: &str, item_key: &str) -> Result<(), String> {
        if self.fail_open_keys.contains(item_key) {
            return Err("simulated detail query timeout".to_owned());
        }
        Ok(())
    }
}

#[tokio::test]
async fn ac1_every_producer_kind_reaches_the_right_list_and_quiet_or_clean_failed_reviews_create_no_today_row()
 {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };

    let now_u64 = BASE_NOW_UNIX;
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token) = invite_and_sign_in(&auth, &practice, "Dr. Rivera", now_u64).await;
    let ws_now = format!("pw-now-{}", Uuid::new_v4().simple());
    let ws_nudge = format!("pw-nudge-{}", Uuid::new_v4().simple());
    let ws_quiet = format!("pw-quiet-{}", Uuid::new_v4().simple());
    let ws_unhandled = format!("pw-unhandled-{}", Uuid::new_v4().simple());

    for ws in [&ws_now, &ws_nudge, &ws_quiet, &ws_unhandled] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let ref_now = seed_chart_record(
        &store,
        &ws_now,
        "Elena Vance",
        "obs-k-1",
        "rev-1",
        "Serum potassium 6.1 mmol/L (critical high).",
        now_i64 - 600,
    )
    .await;
    let ref_nudge = seed_chart_record(
        &store,
        &ws_nudge,
        "Marcus Sterling",
        "obs-a1c-1",
        "rev-1",
        "Hemoglobin A1c 8.7%, up from 7.2% six months ago.",
        now_i64 - 3_600,
    )
    .await;
    let ref_quiet = seed_chart_record(
        &store,
        &ws_quiet,
        "Hannah Cho",
        "obs-bp-1",
        "rev-1",
        "Home blood pressure log averaging 118/74 mmHg on current regimen.",
        now_i64 - 1_800,
    )
    .await;
    let _ref_unhandled = seed_chart_record(
        &store,
        &ws_unhandled,
        "Devon Brooks",
        "obs-renal-1",
        "rev-1",
        "External nephrology consult pending scanned attachment.",
        now_i64 - 7_200,
    )
    .await;

    let origin_work = SourceWork::new("patient_review", "work-origin-1", "attempt-1").unwrap();

    // 1. Open Now concern (waited 5 minutes -> still ranks FIRST ahead of 2-hour Today rows!).
    seed_concern(
        &store,
        SeedConcern {
            practice: practice.clone(),
            workspace: ws_now.clone(),
            concern_id: "concern-now-k".to_owned(),
            urgency: ConcernUrgency::Now,
            standing: SeedConcernStanding::Open,
            reason: "Critical hyperkalemia (K 6.1 mmol/L) on morning BMP.".to_owned(),
            records: vec![ref_now.clone()],
            source_work: origin_work.clone(),
            created_at: now_i64 - 300,
            updated_at: now_i64 - 300,
            revision: 1,
            visit_slot: None,
        },
    )
    .await
    .unwrap();

    // 2. Open Today pre-visit nudge (waited 1 hour) + saved VisitBriefing.
    seed_visit_briefing(
        &store,
        SeedVisitBriefing {
            practice: practice.clone(),
            workspace: ws_nudge.clone(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-sterling-1".to_owned(),
            starts_at_unix: now_i64 + 3_600,
            ends_at_unix: now_i64 + 5_400,
            briefing_id: "briefing-sterling-today".to_owned(),
            orientation: "Follow-up for rising A1c (8.7%) ahead of afternoon visit.".to_owned(),
            narrative: "A1c rose from 7.2% to 8.7% over 6 months despite metformin.".to_owned(),
            basis_and_limitations: "Regional EHR labs through 2026-11-12.".to_owned(),
            citations: vec![SummaryCitation {
                reference: ref_nudge.clone(),
                source_id: "regional-ehr".to_owned(),
                source_label: "Regional EHR".to_owned(),
                sha256: "deadbeef".to_owned(),
                display_label: "Hemoglobin A1c".to_owned(),
                as_of: Some("2026-11-12".to_owned()),
                excerpt: "Hemoglobin A1c 8.7%.".to_owned(),
                native_uri: None,
            }],
            prepared_at: now_i64 - 3_600,
            source_version: 1,
        },
    )
    .await
    .unwrap();

    seed_concern(
        &store,
        SeedConcern {
            practice: practice.clone(),
            workspace: ws_nudge.clone(),
            concern_id: "concern-visit-a1c".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: SeedConcernStanding::Open,
            reason: "A1c rose to 8.7% ahead of today's diabetes follow-up.".to_owned(),
            records: vec![ref_nudge.clone()],
            source_work: origin_work.clone(),
            created_at: now_i64 - 3_600,
            updated_at: now_i64 - 3_600,
            revision: 1,
            visit_slot: Some(TodayVisitSlot {
                source_id: "practice-sched".to_owned(),
                appointment_id: "appt-sterling-1".to_owned(),
                starts_at_unix: now_i64 + 3_600,
                ends_at_unix: now_i64 + 5_400,
            }),
        },
    )
    .await
    .unwrap();

    // 3. Retired concern (should appear ONLY in Updates with one-line reason, NEVER on Today).
    seed_concern(
        &store,
        SeedConcern {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            concern_id: "concern-retired-bp".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: SeedConcernStanding::Retired {
                reason: "Repeat clinic and home blood pressures normalized.".to_owned(),
                retired_at: now_i64 - 900,
            },
            reason: "Borderline elevated office blood pressure last month.".to_owned(),
            records: vec![ref_quiet.clone()],
            source_work: origin_work,
            created_at: now_i64 - 10_000,
            updated_at: now_i64 - 900,
            revision: 2,
            visit_slot: None,
        },
    )
    .await
    .unwrap();

    // 4. Quiet completed review today -> NO Today row; reaches Updates `reviewed_nothing_for_you`!
    seed_review(
        &store,
        SeedReview {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            review_id: "rev-quiet-today".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Quiet {
                    reason: QuietReviewReason::NothingToDo,
                    explanation: "Reviewed home BP log; values remain at goal.".to_owned(),
                    citations: vec![ReviewCitation {
                        reference: ref_quiet.clone(),
                        source_id: "regional-ehr".to_owned(),
                        source_label: "Regional EHR".to_owned(),
                        sha256: "abcd".to_owned(),
                        display_label: "Home BP Log".to_owned(),
                        as_of: Some("2026-11-12".to_owned()),
                        excerpt: "118/74 mmHg average.".to_owned(),
                    }],
                    basis_and_limitations: "Regional EHR flowsheet.".to_owned(),
                },
                completed_at: now_i64 - 1_000,
                completed_source_version: 1,
            },
            evidence_read: vec![ref_quiet.clone()],
            queued_at: now_i64 - 1_200,
            updated_at: now_i64 - 1_000,
        },
    )
    .await
    .unwrap();

    // 5. Assessed completed review today -> reaches Updates `today_items`, no duplicate Today row.
    seed_review(
        &store,
        SeedReview {
            practice: practice.clone(),
            workspace: ws_now.clone(),
            review_id: "rev-assessed-today".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-k-1".to_owned()],
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Assessed {
                    summary: "Flagged critical potassium 6.1 mmol/L and raised Now concern."
                        .to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Regional EHR lab result.".to_owned(),
                },
                completed_at: now_i64 - 290,
                completed_source_version: 1,
            },
            evidence_read: vec![ref_now.clone()],
            queued_at: now_i64 - 310,
            updated_at: now_i64 - 290,
        },
    )
    .await
    .unwrap();

    // 6. Earlier-day completed review (Yesterday) -> reaches Updates `earlier_days`.
    seed_review(
        &store,
        SeedReview {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            review_id: "rev-yesterday-1".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Assessed {
                    summary: "Updated lipid trend note after fasting panel.".to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Fasting lipid panel.".to_owned(),
                },
                completed_at: now_i64 - DAY_SECONDS,
                completed_source_version: 1,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - DAY_SECONDS - 600,
            updated_at: now_i64 - DAY_SECONDS,
        },
    )
    .await
    .unwrap();

    // 7. Failed background review WITHOUT unfinished_work -> NO Today row!
    seed_review(
        &store,
        SeedReview {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            review_id: "rev-failed-clean".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Failed {
                reason: "Transient provider error on routine check".to_owned(),
                failed_at: now_i64 - 1_900,
                attempts: 3,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 2_000,
            updated_at: now_i64 - 1_900,
        },
    )
    .await
    .unwrap();

    // 8. Failed review WITH unfinished_work (waited 2 hours) -> creates Today UnhandledWork row!
    seed_review(
        &store,
        SeedReview {
            practice: practice.clone(),
            workspace: ws_unhandled.clone(),
            review_id: "rev-failed-unhandled".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-renal-1".to_owned()],
            }],
            unfinished_work: vec![UnfinishedReviewWork {
                topic: "External nephrology consult".to_owned(),
                detail: "Verify potassium binder dose in scanned consult attachment".to_owned(),
                limit: None,
            }],
            state: ReviewLifecycleState::Failed {
                reason: "Provider budget exhausted".to_owned(),
                failed_at: now_i64 - 7_200,
                attempts: 3,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 7_300,
            updated_at: now_i64 - 7_200,
        },
    )
    .await
    .unwrap();

    // 9. Routine background running review -> Updates `active_runs` ONLY (not Today).
    seed_review(
        &store,
        SeedReview {
            practice: practice.clone(),
            workspace: ws_nudge.clone(),
            review_id: "rev-routine-running".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Running {
                attempt_id: "att-bg-1".to_owned(),
                started_at: now_i64 - 100,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 120,
            updated_at: now_i64 - 100,
        },
    )
    .await
    .unwrap();

    // 10. Visit preparation CouldNotFinish (upcoming) -> Today UnhandledWork; Skipped -> Updates EndedNothingToDo.
    seed_visit_preparation(
        &store,
        SeedVisitPreparation {
            practice: practice.clone(),
            workspace: ws_unhandled.clone(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-brooks-upcoming".to_owned(),
            starts_at_unix: now_i64 + 7_200,
            ends_at_unix: now_i64 + 9_000,
            state: SeedVisitPrepState::CouldNotFinish {
                reason: "Could not synthesize external nephrology note before visit".to_owned(),
            },
            updated_at: now_i64 - 5_400,
        },
    )
    .await
    .unwrap();

    seed_visit_preparation(
        &store,
        SeedVisitPreparation {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-cho-flu".to_owned(),
            starts_at_unix: now_i64 + 10_800,
            ends_at_unix: now_i64 + 11_700,
            state: SeedVisitPrepState::Skipped {
                reason: "Nurse-only influenza vaccination visit needs no chart briefing."
                    .to_owned(),
            },
            updated_at: now_i64 - 800,
        },
    )
    .await
    .unwrap();

    // 11. Saved patient summary on ws_quiet + failed InitialUnderstanding on ws_unhandled.
    seed_patient_summary(
        &store,
        &practice,
        &ws_quiet,
        1,
        1,
        &PatientSummary {
            narrative: "Well-controlled hypertension on lisinopril 10 mg daily.".to_owned(),
            claims: vec![SummaryClaimInput {
                text: "Home BP averaging 118/74 mmHg.".to_owned(),
                citations: vec![ref_quiet],
            }],
            material_gaps: Vec::new(),
            scope_note: None,
            citations: Vec::new(),
            segments: vec![SummarySegment::Prose {
                text: "Well-controlled hypertension on lisinopril 10 mg daily.".to_owned(),
            }],
        },
        now_i64 - 700,
    )
    .await
    .unwrap();

    seed_failed_initial_understanding(
        &store,
        &practice,
        &ws_unhandled,
        "init-understanding-brooks",
        1,
        now_i64 - 4_500,
    )
    .await
    .unwrap();

    // 12. Active physician chat response on ws_now -> RecognizableRunKind::PhysicianChatResponse on Today.
    let mut authority = PostgresWorkspaceAuthority;
    seed_running_conversation(
        &store,
        SeedRunningConversation {
            practice: practice.clone(),
            physician: dr.physician_id.clone(),
            workspace: ws_now.clone(),
            chat_id: "chat-hyperkalemia-1".to_owned(),
            question: "Did Elena take patiromer this week?".to_owned(),
            activity_at: now_i64 - 120,
        },
    )
    .await
    .expect("seed running chat");

    // --- READ TODAY ---
    let names = BTreeMap::new();
    let today = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .expect("read today");

    assert_eq!(today.completeness, ReadCompleteness::Complete);
    assert!(!today.nothing_needs_physician());
    assert_eq!(
        today.also_reviewed_today_count, 1,
        "quiet review today counts in also_reviewed_today_count without adding a Today row"
    );

    let today_keys: Vec<&str> = today.rows.iter().map(|r| r.item_key.as_str()).collect();
    // Verify quiet review, clean-failed review, retired concern, and routine running review NEVER appear on Today:
    assert!(
        !today_keys.iter().any(|k| k.contains("rev-quiet-today")
            || k.contains("rev-failed-clean")
            || k.contains("concern-retired-bp")
            || k.contains("rev-routine-running")),
        "quiet/clean-failed/retired/routine-running items must never create Today rows: {today_keys:?}"
    );

    // Verify Now comes FIRST, then remaining rows ordered by longest wait (`waiting_since_unix` ascending):
    assert_eq!(
        today.rows[0].item_key.as_str(),
        format!("concern:{ws_now}:concern-now-k")
    );
    assert!(today.rows[0].is_now());
    assert_eq!(today.rows[0].patient_display_name, "Elena Vance");

    // Remaining non-Now rows should be ordered oldest waiting_since_unix first:
    // -7200 (unhandled review) -> -5400 (unhandled visit prep) -> -4500 (unhandled understanding) -> -3600 (visit nudge) -> running chat
    assert!(matches!(
        today.rows[1].payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::PatientReview,
            ..
        }
    ));
    assert!(matches!(
        today.rows[2].payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::VisitPreparation,
            ..
        }
    ));
    assert!(matches!(
        today.rows[3].payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::InitialUnderstanding,
            ..
        }
    ));
    assert!(matches!(
        today.rows[4].payload,
        TodayRowPayload::OpenConcern {
            urgency: ConcernUrgency::Today,
            visit_slot: Some(_),
            ..
        }
    ));
    assert!(matches!(
        today.rows[5].payload,
        TodayRowPayload::RunningWork {
            run_kind: RecognizableRunKind::PhysicianChatResponse,
            ..
        }
    ));

    // Verify opening a Today concern returns its verified chart evidence and citations:
    let open_key = SeenItemKey::new(format!("concern:{ws_now}:concern-now-k")).unwrap();
    let evidence = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_now,
                item_key: &open_key,
                command_key: &CommandKey::new("cmd-open-now-1").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .expect("open Now concern");
    assert_eq!(evidence.citations.len(), 1);
    assert!(evidence.citations[0].excerpt.contains("6.1 mmol/L"));
    assert!(matches!(evidence.seen_standing, SeenStanding::Seen { .. }));

    // Verify clearing an unhandled work item with `Done` removes ONLY that row across reads:
    let unhandled_row = &today.rows[1];
    let done_res = store
        .clear_today_item_done(
            auth.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &unhandled_row.workspace,
                item_key: &unhandled_row.item_key,
                version_id: &unhandled_row.version_id,
                command_key: &CommandKey::new("cmd-done-unhandled-1").unwrap(),
            },
        )
        .await
        .expect("clear unhandled work Done");
    assert!(matches!(done_res, TodayDoneResult::Cleared { .. }));

    let today_after_done = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(
        !today_after_done
            .rows
            .iter()
            .any(|r| r.item_key == unhandled_row.item_key),
        "cleared unhandled work must leave Today"
    );

    // --- READ UPDATES ---
    let updates = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .expect("read updates");
    assert_eq!(updates.completeness, ReadCompleteness::Complete);
    assert_eq!(
        updates.active_runs.len(),
        1,
        "routine running review surfaces in Updates active_runs"
    );
    assert_eq!(
        updates.reviewed_nothing_for_you.len(),
        1,
        "quiet review today reaches Reviewed, nothing for you"
    );
    assert!(matches!(
        updates.reviewed_nothing_for_you[0].kind,
        UpdateEntryKind::QuietReview { .. }
    ));

    // Check every producer kind in `today_items`: AssessedReview, VisitBriefing, PatientSummary, ConcernRaised, ConcernRetired, EndedNothingToDo!
    let has_assessed = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::AssessedReview { .. }));
    let has_briefing = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::VisitBriefing { .. }));
    let has_summary = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::PatientSummary { .. }));
    let has_raised = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::ConcernRaised { .. }));
    let has_retired = updates.today_items.iter().any(|i| {
        matches!(
            &i.kind,
            UpdateEntryKind::ConcernRetired {
                retirement_reason,
                ..
            } if retirement_reason.contains("normalized")
        )
    });
    let has_skipped = updates.today_items.iter().any(|i| {
        matches!(
            &i.kind,
            UpdateEntryKind::EndedNothingToDo { reason, .. } if reason.contains("influenza vaccination")
        )
    });
    assert!(
        has_assessed && has_briefing && has_summary && has_raised && has_retired && has_skipped,
        "every producer kind must reach Updates today_items: {:?}",
        updates.today_items
    );

    // Verify OperatingIndex sentinel never leaks into Updates JSON:
    let updates_serialized = serde_json::to_string(&updates).unwrap();
    assert!(
        !updates_serialized.contains("INTERNAL-OPERATING-INDEX-NEVER-SHOWN"),
        "OperatingIndex must never leak into Updates projection"
    );

    // Verify Earlier days group contains yesterday's review:
    assert_eq!(updates.earlier_days.len(), 1);
    assert_eq!(updates.earlier_days[0].day_label, "Yesterday");
    assert_eq!(updates.earlier_days[0].items.len(), 1);
    assert_eq!(
        updates.earlier_days[0].items[0].item_key.as_str(),
        format!("review:{ws_quiet}:rev-yesterday-1")
    );
}

#[tokio::test]
async fn ac2_seen_state_is_saved_per_physician_survives_restart_and_changed_item_returns_as_new_with_what_changed()
 {
    let Some((store, auth, url, _cluster)) = connect_stores().await else {
        return;
    };
    let now_u64 = BASE_NOW_UNIX;
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr_a, token_a) = invite_and_sign_in(&auth, &practice, "Dr. Patel", now_u64).await;
    let (dr_b, token_b) = invite_and_sign_in(&auth, &practice, "Dr. Nguyen", now_u64).await;
    let ws = format!("pw-shared-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &ws).await.unwrap();
    auth.set_membership(&dr_a, &ws, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr_b, &ws, Membership::Allowed)
        .await
        .unwrap();

    let rec_ref = seed_chart_record(
        &store,
        &ws,
        "Samira Al-Mansoor",
        "obs-cr-1",
        "rev-1",
        "Serum creatinine 1.6 mg/dL (baseline 1.0 mg/dL).",
        now_i64 - 1_200,
    )
    .await;

    seed_concern(
        &store,
        SeedConcern {
            practice: practice.clone(),
            workspace: ws.clone(),
            concern_id: "concern-creatinine".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: SeedConcernStanding::Open,
            reason: "Creatinine rose from 1.0 to 1.6 mg/dL.".to_owned(),
            records: vec![rec_ref],
            source_work: SourceWork::new("patient_review", "work-cr-1", "att-1").unwrap(),
            created_at: now_i64 - 1_000,
            updated_at: now_i64 - 1_000,
            revision: 1,
            visit_slot: None,
        },
    )
    .await
    .unwrap();

    let names = BTreeMap::new();
    let item_key = SeenItemKey::new(format!("concern:{ws}:concern-creatinine")).unwrap();

    // Initially Unseen for both physicians.
    let today_a0 = store
        .read_today(auth.as_ref(), &token_a, now_u64, &names)
        .await
        .unwrap();
    let today_b0 = store
        .read_today(auth.as_ref(), &token_b, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_a0.rows[0].seen_standing, SeenStanding::Unseen);
    assert_eq!(today_b0.rows[0].seen_standing, SeenStanding::Unseen);

    // Dr. Patel opens the item -> Seen for Dr. Patel, still Unseen for Dr. Nguyen.
    let mut authority = PostgresWorkspaceAuthority;
    store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token_a,
                real_now_unix: now_u64,
                workspace: &ws,
                item_key: &item_key,
                command_key: &CommandKey::new("cmd-patel-open-1").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .unwrap();

    // Simulate full process restart by dropping `store` and reconnecting a fresh `Store`.
    drop(store);
    let restarted_store = Store::connect(&url).await.expect("reconnect runtime store");

    let today_a1 = restarted_store
        .read_today(auth.as_ref(), &token_a, now_u64 + 10, &names)
        .await
        .unwrap();
    let today_b1 = restarted_store
        .read_today(auth.as_ref(), &token_b, now_u64 + 10, &names)
        .await
        .unwrap();
    assert!(matches!(
        today_a1.rows[0].seen_standing,
        SeenStanding::Seen { .. }
    ));
    assert_eq!(
        today_b1.rows[0].seen_standing,
        SeenStanding::Unseen,
        "Dr. Nguyen's seen state must remain independent of Dr. Patel"
    );

    // Now Coda revises the concern (escalates to Now + new repeat creatinine 2.3 mg/dL).
    revise_seeded_concern(
        &restarted_store,
        &practice,
        &ws,
        "concern-creatinine",
        ConcernUrgency::Now,
        "Repeat creatinine 2.3 mg/dL with hyperkalemia.",
        now_i64 + 30,
    )
    .await
    .unwrap();

    let today_a2 = restarted_store
        .read_today(auth.as_ref(), &token_a, now_u64 + 40, &names)
        .await
        .unwrap();
    let today_b2 = restarted_store
        .read_today(auth.as_ref(), &token_b, now_u64 + 40, &names)
        .await
        .unwrap();

    assert!(
        today_a2.rows[0].seen_standing.needs_physician_eye(),
        "changed item must return as needing physician eye"
    );
    match &today_a2.rows[0].seen_standing {
        SeenStanding::ChangedSinceSeen { what_changed, .. } => {
            assert!(
                what_changed.contains("Urgency raised to Now"),
                "expected urgency escalation in what_changed, got: {what_changed}"
            );
            assert!(
                what_changed.contains("2.3 mg/dL"),
                "expected updated clinical finding in what_changed, got: {what_changed}"
            );
        }
        other => panic!("expected ChangedSinceSeen for Dr. Patel, got {other:?}"),
    }
    assert_eq!(today_b2.rows[0].seen_standing, SeenStanding::Unseen);

    // Updates also reflects the exact same ChangedSinceSeen standing for Dr. Patel:
    let updates_a2 = restarted_store
        .read_updates(auth.as_ref(), &token_a, now_u64 + 40, &names)
        .await
        .unwrap();
    assert!(matches!(
        updates_a2.today_items[0].seen_standing,
        SeenStanding::ChangedSinceSeen { .. }
    ));
}

#[tokio::test]
async fn ac3_physician_sees_only_authorized_patients_including_after_access_changes() {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let now_u64 = BASE_NOW_UNIX;
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac3-{}", Uuid::new_v4().simple());
    let other_practice = format!("prac-other-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();
    auth.ensure_practice(&other_practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token) = invite_and_sign_in(&auth, &practice, "Dr. Lin", now_u64).await;
    let ws_allowed = format!("pw-allowed-{}", Uuid::new_v4().simple());
    let ws_off_panel = format!("pw-offpanel-{}", Uuid::new_v4().simple());
    let ws_other_prac = format!("pw-otherprac-{}", Uuid::new_v4().simple());

    auth.register_patient(&practice, &ws_allowed).await.unwrap();
    auth.register_patient(&practice, &ws_off_panel)
        .await
        .unwrap();
    auth.register_patient(&other_practice, &ws_other_prac)
        .await
        .unwrap();

    auth.set_membership(&dr, &ws_allowed, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr, &ws_off_panel, Membership::ClinicDemographic)
        .await
        .unwrap();

    for (target_prac, ws, name) in [
        (&practice, &ws_allowed, "Panel Patient"),
        (&practice, &ws_off_panel, "Off-Panel Clinic Patient"),
        (&other_practice, &ws_other_prac, "Other Practice Patient"),
    ] {
        let rec =
            seed_chart_record(&store, ws, name, "obs-1", "rev-1", "Lab finding", now_i64).await;
        seed_concern(
            &store,
            SeedConcern {
                practice: target_prac.clone(),
                workspace: ws.clone(),
                concern_id: "concern-1".to_owned(),
                urgency: ConcernUrgency::Now,
                standing: SeedConcernStanding::Open,
                reason: format!("Urgent finding for {name}"),
                records: vec![rec],
                source_work: SourceWork::new("patient_review", "work-1", "att-1").unwrap(),
                created_at: now_i64 - 100,
                updated_at: now_i64 - 100,
                revision: 1,
                visit_slot: None,
            },
        )
        .await
        .unwrap();
    }

    let names = BTreeMap::new();
    let today_initial = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_initial.rows.len(), 1);
    assert_eq!(today_initial.rows[0].workspace, ws_allowed);

    let updates_initial = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(updates_initial.today_items.len(), 1);
    assert_eq!(updates_initial.today_items[0].workspace, ws_allowed);

    // Direct patient-page Today read on off-panel or other-practice patient is refused:
    assert!(matches!(
        store
            .read_today_for_workspace(auth.as_ref(), &token, now_u64, &ws_off_panel, &names)
            .await,
        Err(TodayUpdatesError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .read_today_for_workspace(auth.as_ref(), &token, now_u64, &ws_other_prac, &names)
            .await,
        Err(TodayUpdatesError::AccessDenied { .. })
    ));

    // Grant access to `ws_off_panel` -> immediately appears in Today and Updates!
    auth.set_membership(&dr, &ws_off_panel, Membership::Allowed)
        .await
        .unwrap();
    let today_granted = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_granted.rows.len(), 2);

    // Revoke access from `ws_allowed` -> immediately disappears, and opening its item is refused!
    auth.set_membership(&dr, &ws_allowed, Membership::ClinicDemographic)
        .await
        .unwrap();
    let today_revoked = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_revoked.rows.len(), 1);
    assert_eq!(today_revoked.rows[0].workspace, ws_off_panel);

    let updates_revoked = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(updates_revoked.today_items.len(), 1);
    assert_eq!(updates_revoked.today_items[0].workspace, ws_off_panel);

    let mut authority = PostgresWorkspaceAuthority;
    let revoked_key = SeenItemKey::new(format!("concern:{ws_allowed}:concern-1")).unwrap();
    let open_err = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_allowed,
                item_key: &revoked_key,
                command_key: &CommandKey::new("cmd-open-revoked").unwrap(),
                patient_names: &names,
            },
        )
        .await;
    assert!(matches!(
        open_err,
        Err(TodayUpdatesError::AccessDenied { .. })
    ));
}

#[tokio::test]
async fn ac4_partial_producer_failure_practice_source_outage_collapse_and_empty_completeness_rule()
{
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let now_u64 = BASE_NOW_UNIX;
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac4-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token) = invite_and_sign_in(&auth, &practice, "Dr. Okafor", now_u64).await;
    let ws_1 = format!("pw-p1-{}", Uuid::new_v4().simple());
    let ws_2 = format!("pw-p2-{}", Uuid::new_v4().simple());
    for ws in [&ws_1, &ws_2] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let names = BTreeMap::new();

    // 1. Clean empty state when EVERY producer answers -> `nothing_needs_physician() == true`.
    let clean_empty = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(clean_empty.completeness, ReadCompleteness::Complete);
    assert!(clean_empty.rows.is_empty());
    assert!(clean_empty.practice_outages.is_empty());
    assert!(clean_empty.nothing_needs_physician());

    // 2. Empty rows when ONE producer fails -> `nothing_needs_physician()` MUST be FALSE!
    let concerns_down_probe = ForcedFaultProbe {
        fail_producers: BTreeSet::from([ProducerKind::Concerns]),
        fail_open_keys: BTreeSet::new(),
    };
    let partial_empty = store
        .read_today_with_probe(
            auth.as_ref(),
            &token,
            now_u64,
            None,
            &names,
            &concerns_down_probe,
        )
        .await
        .unwrap();
    assert!(!partial_empty.completeness.is_complete());
    assert!(!partial_empty.nothing_needs_physician());
    assert_eq!(
        partial_empty.completeness.failed_parts()[0].producer,
        ProducerKind::Concerns
    );

    // 3. Seed an open concern on `ws_1`, then force `PatientReviews` to fail:
    //    Concerns still return, and `failed_parts` names `PatientReviews`.
    let rec = seed_chart_record(
        &store,
        &ws_1,
        "Patient One",
        "obs-1",
        "rev-1",
        "INR 4.8",
        now_i64,
    )
    .await;
    seed_concern(
        &store,
        SeedConcern {
            practice: practice.clone(),
            workspace: ws_1.clone(),
            concern_id: "concern-inr".to_owned(),
            urgency: ConcernUrgency::Now,
            standing: SeedConcernStanding::Open,
            reason: "Elevated INR 4.8 on warfarin.".to_owned(),
            records: vec![rec],
            source_work: SourceWork::new("patient_review", "work-inr", "att-1").unwrap(),
            created_at: now_i64 - 200,
            updated_at: now_i64 - 200,
            revision: 1,
            visit_slot: None,
        },
    )
    .await
    .unwrap();

    let reviews_down_probe = ForcedFaultProbe {
        fail_producers: BTreeSet::from([ProducerKind::PatientReviews]),
        fail_open_keys: BTreeSet::new(),
    };
    let partial_with_concern = store
        .read_today_with_probe(
            auth.as_ref(),
            &token,
            now_u64,
            None,
            &names,
            &reviews_down_probe,
        )
        .await
        .unwrap();
    assert_eq!(
        partial_with_concern.rows.len(),
        1,
        "healthy Concerns producer still returns its row when PatientReviews fails"
    );
    assert_eq!(partial_with_concern.completeness.failed_parts().len(), 1);
    assert_eq!(
        partial_with_concern.completeness.failed_parts()[0].producer,
        ProducerKind::PatientReviews
    );

    // 4. Single-patient source failure (`ws_1` failed, `ws_2` updated) -> NO practice outage line!
    seed_source_status(
        &store,
        &practice,
        &ws_1,
        "quest-labs",
        "Quest Diagnostics",
        SeedSourceOutcome::Failed {
            last_good_pulled_at: Some(now_i64 - 3_600),
            reason: "patient match timeout".to_owned(),
        },
        now_i64 - 120,
    )
    .await
    .unwrap();
    seed_source_status(
        &store,
        &practice,
        &ws_2,
        "quest-labs",
        "Quest Diagnostics",
        SeedSourceOutcome::Updated {
            pulled_at: now_i64 - 60,
        },
        now_i64 - 60,
    )
    .await
    .unwrap();

    let single_fail_today = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(
        single_fail_today.practice_outages.is_empty(),
        "single-patient source failure must not report a practice-wide outage"
    );

    // 5. Whole source down across the practice (`ws_1` AND `ws_2` failed on `quest-labs`) -> ONE outage line!
    seed_source_status(
        &store,
        &practice,
        &ws_2,
        "quest-labs",
        "Quest Diagnostics",
        SeedSourceOutcome::Failed {
            last_good_pulled_at: Some(now_i64 - 3_600),
            reason: "upstream gateway 503".to_owned(),
        },
        now_i64 - 30,
    )
    .await
    .unwrap();

    let practice_outage_today = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(
        practice_outage_today.practice_outages.len(),
        1,
        "practice-wide source failure must collapse into one outage line"
    );
    assert_eq!(
        practice_outage_today.practice_outages[0].source_id,
        "quest-labs"
    );
    assert!(
        practice_outage_today.practice_outages[0]
            .failure_detail
            .contains("Quest Diagnostics")
    );
}
