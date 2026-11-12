//! Real PostgreSQL integration tests for practice schedule reads, patient link
//! matching, reschedule/cancel versioning, mid-read restart safety, and failed
//! or partial read handling.
//!
//! Runs whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`, `PHYSICIAN_DATABASE_URL`,
//! or `PHYSICIAN_TEST_DATABASE_URL` is set; panics if unset when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::Store;
use local_product::runtime::schedule::{
    AppointmentId, AppointmentStatus, AppointmentWindow, PatientMatch, ScheduleDelivery,
    ScheduleError, ScheduleFreshness, ScheduleReadFailures, ScheduleReadIssue, ScheduleSource,
    ScheduleSourceError, ScheduleStanding, ScheduleSyncOutcome, SourceAppointment,
    SourceAppointmentParts, SourceId, SourcePatientId,
};
use local_product::runtime::sources::SourcePull;
use local_product::sha256_hex;
use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};
use serde_json::json;
use tokio::sync::Notify;
use uuid::Uuid;

fn database_url() -> Option<String> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(url) = std::env::var(key) {
            let trimmed = url.trim().to_owned();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    assert!(
        std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
        "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
    );
    None
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

fn chart_record(workspace: &str, mrn: &str) -> SourceRevision {
    let content = format!(r#"{{"resourceType":"Patient","id":"{mrn}"}}"#);
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "synthetic-charts".to_owned(),
        candidate_id: format!("Patient/{mrn}"),
        revision: sha256_hex(content.as_bytes()),
        record_type: "Patient".to_owned(),
        dates: vec![],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: vec![ReadablePart {
            label: "Patient".to_owned(),
            text: mrn.to_owned(),
        }],
        provenance: json!({ "native_record": format!("Patient/{mrn}") }),
    }
}

async fn register_roster_patient_with_pull(
    store: &Store,
    auth: &PostgresAuthStore,
    practice: &str,
    workspace: &str,
    mrn: &str,
    display_name: &str,
    pulled_at: i64,
) {
    let record = chart_record(workspace, mrn);
    let pull = SourcePull {
        source_id: "synthetic-charts".to_owned(),
        source_label: "Synthetic chart".to_owned(),
        patient: SourcePatient {
            source_patient_id: mrn.to_owned(),
            display_name: display_name.to_owned(),
            birth_date: Some("1978-05-14".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "urn:mrn".to_owned(),
                value: mrn.to_owned(),
            }],
        },
        pulled_at,
    };
    store
        .record_chart(workspace, &[record], &pull)
        .await
        .expect("record chart");
    auth.register_patient(practice, workspace)
        .await
        .expect("register roster patient");
}

fn make_appointment(
    id: &str,
    status: AppointmentStatus,
    source_status: &str,
    label: &str,
    starts_at: &str,
    ends_at: &str,
    mrn: Option<&str>,
) -> SourceAppointment {
    let content = json!({
        "resourceType": "Appointment",
        "id": id,
        "meta": { "versionId": "1", "lastUpdated": "2026-08-27T00:00:00+00:00" },
        "status": source_status,
        "description": label,
        "start": starts_at,
        "end": ends_at,
        "participant": mrn.map(|m| json!([{ "actor": { "reference": format!("Patient/{m}") }, "status": "accepted" }])).unwrap_or_else(|| json!([])),
    })
    .to_string();

    SourceAppointment::new(SourceAppointmentParts {
        appointment_id: AppointmentId::new(id).expect("appointment id"),
        source_revision: None,
        window: AppointmentWindow::parse(starts_at, ends_at).expect("appointment window"),
        status,
        source_status: source_status.to_owned(),
        label: label.to_owned(),
        source_patient_id: mrn.map(|m| SourcePatientId::new(m).expect("source patient id")),
        dates: vec![
            RecordDate {
                name: "start".to_owned(),
                value: starts_at.to_owned(),
            },
            RecordDate {
                name: "end".to_owned(),
                value: ends_at.to_owned(),
            },
        ],
        content,
        provenance: json!({ "native_record": format!("Appointment/{id}") }),
    })
    .expect("source appointment")
}

#[derive(Clone)]
enum StubSchedulePlan {
    Complete(Vec<SourceAppointment>),
    Partial {
        appointments: Vec<SourceAppointment>,
        issues: Vec<ScheduleReadIssue>,
    },
    FailIo(String),
}

struct StubScheduleSource {
    source_id: SourceId,
    label: String,
    plan: Mutex<StubSchedulePlan>,
}

impl StubScheduleSource {
    fn new(plan: StubSchedulePlan) -> Self {
        Self {
            source_id: SourceId::new("synthetic-practice-calendar").unwrap(),
            label: "Synthetic practice calendar".to_owned(),
            plan: Mutex::new(plan),
        }
    }

    fn set_plan(&self, plan: StubSchedulePlan) {
        *self.plan.lock().unwrap() = plan;
    }
}

#[async_trait]
impl ScheduleSource for StubScheduleSource {
    fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        &self.label
    }

    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError> {
        let plan = self.plan.lock().unwrap().clone();
        match plan {
            StubSchedulePlan::Complete(appointments) => {
                Ok(ScheduleDelivery::Complete { appointments })
            }
            StubSchedulePlan::Partial {
                appointments,
                issues,
            } => Ok(ScheduleDelivery::Partial {
                appointments,
                failures: ScheduleReadFailures::new(issues).unwrap(),
            }),
            StubSchedulePlan::FailIo(message) => Err(ScheduleSourceError::Io {
                source_id: self.source_id.clone(),
                source: std::io::Error::new(std::io::ErrorKind::NotFound, message),
            }),
        }
    }
}

struct GatedScheduleSource {
    source_id: SourceId,
    entered_read: Arc<Notify>,
    release_read: Arc<Notify>,
    appointments: Vec<SourceAppointment>,
}

#[async_trait]
impl ScheduleSource for GatedScheduleSource {
    fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        "Synthetic practice calendar"
    }

    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError> {
        self.entered_read.notify_one();
        self.release_read.notified().await;
        Ok(ScheduleDelivery::Complete {
            appointments: self.appointments.clone(),
        })
    }
}

#[tokio::test]
async fn ac1_matches_linked_patients_keeps_unmatched_visible_and_never_adds_to_roster() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let practice = format!("practice-sched-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(1_787_846_400))
        .await
        .unwrap();

    let alice_ws = format!("pw-alice-{}", Uuid::new_v4().simple());
    let bob_ws = format!("pw-bob-{}", Uuid::new_v4().simple());
    let off_roster_ws = format!("pw-off-roster-{}", Uuid::new_v4().simple());

    register_roster_patient_with_pull(
        &store,
        &auth,
        &practice,
        &alice_ws,
        "MRN-ALICE",
        "Alice Carter",
        1_787_846_400,
    )
    .await;
    register_roster_patient_with_pull(
        &store,
        &auth,
        &practice,
        &bob_ws,
        "MRN-BOB",
        "Bob Diaz",
        1_787_846_400,
    )
    .await;

    let source_id = SourceId::new("synthetic-practice-calendar").unwrap();
    // Seeding a link for a workspace not on the practice roster must be refused.
    let refused = store
        .seed_schedule_patient_link(
            &*auth,
            &practice,
            &source_id,
            &SourcePatientId::new("MRN-GHOST").unwrap(),
            &off_roster_ws,
            1_787_846_400,
        )
        .await;
    assert!(matches!(
        refused,
        Err(ScheduleError::PatientNotOnRoster { .. })
    ));

    // Seed schedule links from already-registered roster pulls.
    let seeded = store
        .seed_schedule_links_from_pulls(&*auth, &practice, &source_id, 1_787_846_400)
        .await
        .unwrap();
    assert_eq!(seeded, 2);

    let calendar = StubScheduleSource::new(StubSchedulePlan::Complete(vec![
        make_appointment(
            "appt-1",
            AppointmentStatus::Scheduled,
            "booked",
            "Medication follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        make_appointment(
            "appt-2",
            AppointmentStatus::Scheduled,
            "booked",
            "Care-plan follow-up",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            Some("MRN-BOB"),
        ),
        make_appointment(
            "appt-3-no-participant",
            AppointmentStatus::Scheduled,
            "booked",
            "New patient visit",
            "2026-08-27T19:00:00+00:00",
            "2026-08-27T19:30:00+00:00",
            None,
        ),
        make_appointment(
            "appt-4-unlinked-mrn",
            AppointmentStatus::Scheduled,
            "booked",
            "Walk-in consult",
            "2026-08-27T20:00:00+00:00",
            "2026-08-27T20:30:00+00:00",
            Some("MRN-STRANGER"),
        ),
    ]));

    let outcome = store
        .sync_practice_schedule(&*auth, &practice, &calendar, 1_787_846_400)
        .await
        .unwrap();
    let ScheduleSyncOutcome::Applied(summary) = outcome else {
        panic!("expected applied sync outcome");
    };
    assert_eq!(summary.saved_appointments, 4);
    assert_eq!(summary.changed_appointments, 4);
    assert_eq!(summary.matched_appointments, 2);
    assert_eq!(summary.unmatched_appointments, 2);

    // Roster invariant: schedule sync NEVER puts new patients on the practice roster.
    let roster_after = auth.practice_patients(&practice).await.unwrap();
    assert_eq!(roster_after.len(), 2);
    assert!(roster_after.contains(&alice_ws));
    assert!(roster_after.contains(&bob_ws));

    let schedule = store.practice_schedule(&practice).await.unwrap();
    assert_eq!(
        schedule.standing,
        ScheduleStanding::Usable {
            source_id: source_id.clone(),
            source_label: "Synthetic practice calendar".to_owned(),
            schedule_version: 1,
            as_of: 1_787_846_400,
            freshness: ScheduleFreshness::Current,
        }
    );
    assert_eq!(schedule.appointments.len(), 4);
    assert_eq!(
        schedule.appointments[0].patient_match,
        PatientMatch::Matched {
            workspace_id: alice_ws.clone(),
            source_patient_id: SourcePatientId::new("MRN-ALICE").unwrap(),
        }
    );
    assert_eq!(
        schedule.appointments[1].patient_match,
        PatientMatch::Matched {
            workspace_id: bob_ws.clone(),
            source_patient_id: SourcePatientId::new("MRN-BOB").unwrap(),
        }
    );
    assert_eq!(
        schedule.appointments[2].patient_match,
        PatientMatch::Unmatched {
            source_patient_id: None,
        }
    );
    assert_eq!(
        schedule.appointments[3].patient_match,
        PatientMatch::Unmatched {
            source_patient_id: Some(SourcePatientId::new("MRN-STRANGER").unwrap()),
        }
    );

    // If MRN-STRANGER later joins the practice roster and has a patient link seeded,
    // syncing the unchanged calendar matches appt-4 and bumps ONLY appt-4's saved_version.
    let stranger_ws = format!("pw-stranger-{}", Uuid::new_v4().simple());
    register_roster_patient_with_pull(
        &store,
        &auth,
        &practice,
        &stranger_ws,
        "MRN-STRANGER",
        "Sam Stranger",
        1_787_846_500,
    )
    .await;
    store
        .seed_schedule_patient_link(
            &*auth,
            &practice,
            &source_id,
            &SourcePatientId::new("MRN-STRANGER").unwrap(),
            &stranger_ws,
            1_787_846_500,
        )
        .await
        .unwrap();

    let relink_outcome = store
        .sync_practice_schedule(&*auth, &practice, &calendar, 1_787_846_600)
        .await
        .unwrap();
    let ScheduleSyncOutcome::Applied(relink_summary) = relink_outcome else {
        panic!("expected applied outcome");
    };
    assert_eq!(relink_summary.changed_appointments, 1);
    assert_eq!(relink_summary.matched_appointments, 3);
    assert_eq!(relink_summary.unmatched_appointments, 1);

    let updated = store.practice_schedule(&practice).await.unwrap();
    assert_eq!(updated.appointments[0].saved_version, 1);
    assert_eq!(updated.appointments[3].saved_version, 2);
    assert_eq!(
        updated.appointments[3].patient_match,
        PatientMatch::Matched {
            workspace_id: stranger_ws,
            source_patient_id: SourcePatientId::new("MRN-STRANGER").unwrap(),
        }
    );
}

#[tokio::test]
async fn ac2_reschedule_and_cancel_update_saved_versions_with_idempotent_replay() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let practice = format!("practice-sched-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(1_787_846_400))
        .await
        .unwrap();

    let ws_1 = format!("pw-1-{}", Uuid::new_v4().simple());
    let ws_2 = format!("pw-2-{}", Uuid::new_v4().simple());
    register_roster_patient_with_pull(&store, &auth, &practice, &ws_1, "MRN-1", "Pat One", 1_000)
        .await;
    register_roster_patient_with_pull(&store, &auth, &practice, &ws_2, "MRN-2", "Pat Two", 1_000)
        .await;
    let source_id = SourceId::new("synthetic-practice-calendar").unwrap();
    store
        .seed_schedule_links_from_pulls(&*auth, &practice, &source_id, 1_000)
        .await
        .unwrap();

    let calendar = StubScheduleSource::new(StubSchedulePlan::Complete(vec![
        make_appointment(
            "appt-resched",
            AppointmentStatus::Scheduled,
            "booked",
            "Follow-up visit",
            "2026-08-27T14:00:00+00:00",
            "2026-08-27T14:30:00+00:00",
            Some("MRN-1"),
        ),
        make_appointment(
            "appt-cancel-explicit",
            AppointmentStatus::Scheduled,
            "booked",
            "Medication check",
            "2026-08-27T15:00:00+00:00",
            "2026-08-27T15:30:00+00:00",
            Some("MRN-2"),
        ),
        make_appointment(
            "appt-withdraw",
            AppointmentStatus::Scheduled,
            "booked",
            "Lab review",
            "2026-08-27T16:00:00+00:00",
            "2026-08-27T16:30:00+00:00",
            None,
        ),
    ]));

    store
        .sync_practice_schedule(&*auth, &practice, &calendar, 1_000)
        .await
        .unwrap();

    // Idempotent replay of identical schedule changes 0 rows and duplicates nothing.
    let replay = store
        .sync_practice_schedule(&*auth, &practice, &calendar, 1_100)
        .await
        .unwrap();
    let ScheduleSyncOutcome::Applied(replay_summary) = replay else {
        panic!("expected applied replay");
    };
    assert_eq!(replay_summary.changed_appointments, 0);
    assert_eq!(replay_summary.saved_appointments, 3);
    let after_replay = store.practice_schedule(&practice).await.unwrap();
    assert!(
        after_replay
            .appointments
            .iter()
            .all(|appt| appt.saved_version == 1 && appt.as_of == 1_100)
    );

    // Reschedule appt-resched, explicitly cancel appt-cancel-explicit, and omit appt-withdraw.
    calendar.set_plan(StubSchedulePlan::Complete(vec![
        make_appointment(
            "appt-resched",
            AppointmentStatus::Scheduled,
            "booked",
            "Follow-up visit",
            "2026-08-27T17:30:00+00:00",
            "2026-08-27T18:00:00+00:00",
            Some("MRN-1"),
        ),
        make_appointment(
            "appt-cancel-explicit",
            AppointmentStatus::Cancelled,
            "cancelled",
            "Medication check",
            "2026-08-27T15:00:00+00:00",
            "2026-08-27T15:30:00+00:00",
            Some("MRN-2"),
        ),
    ]));

    let mutated = store
        .sync_practice_schedule(&*auth, &practice, &calendar, 1_200)
        .await
        .unwrap();
    let ScheduleSyncOutcome::Applied(mutated_summary) = mutated else {
        panic!("expected applied mutation");
    };
    assert_eq!(mutated_summary.changed_appointments, 3);
    assert_eq!(mutated_summary.saved_appointments, 3);

    let saved = store.practice_schedule(&practice).await.unwrap();
    assert_eq!(saved.appointments.len(), 3);
    let by_id: std::collections::BTreeMap<_, _> = saved
        .appointments
        .iter()
        .map(|a| (a.appointment_id.as_str(), a))
        .collect();

    let resched = by_id["appt-resched"];
    assert_eq!(resched.window.starts_at(), "2026-08-27T17:30:00.000Z");
    assert_eq!(resched.window.ends_at(), "2026-08-27T18:00:00.000Z");
    assert_eq!(resched.status, AppointmentStatus::Scheduled);
    assert_eq!(resched.saved_version, 2);

    let cancelled = by_id["appt-cancel-explicit"];
    assert_eq!(cancelled.status, AppointmentStatus::Cancelled);
    assert_eq!(cancelled.source_status, "cancelled");
    assert_eq!(cancelled.saved_version, 2);

    let withdrawn = by_id["appt-withdraw"];
    assert_eq!(withdrawn.status, AppointmentStatus::Cancelled);
    assert_eq!(withdrawn.source_status, "withdrawn");
    assert_eq!(withdrawn.saved_version, 2);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac2_restart_mid_read_and_mid_transaction_loses_and_duplicates_nothing() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let practice = format!("practice-sched-restart-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(1_787_846_400))
        .await
        .unwrap();

    let ws = format!("pw-restart-{}", Uuid::new_v4().simple());
    register_roster_patient_with_pull(&store, &auth, &practice, &ws, "MRN-R1", "Rosa Lee", 2_000)
        .await;
    let source_id = SourceId::new("synthetic-practice-calendar").unwrap();
    store
        .seed_schedule_links_from_pulls(&*auth, &practice, &source_id, 2_000)
        .await
        .unwrap();

    let initial = StubScheduleSource::new(StubSchedulePlan::Complete(vec![
        make_appointment(
            "appt-a",
            AppointmentStatus::Scheduled,
            "booked",
            "Initial visit",
            "2026-08-27T13:00:00+00:00",
            "2026-08-27T13:30:00+00:00",
            Some("MRN-R1"),
        ),
        make_appointment(
            "appt-b",
            AppointmentStatus::Scheduled,
            "booked",
            "Follow-up visit",
            "2026-08-27T14:00:00+00:00",
            "2026-08-27T14:30:00+00:00",
            None,
        ),
    ]));
    store
        .sync_practice_schedule(&*auth, &practice, &initial, 2_000)
        .await
        .unwrap();

    // 1. Mid-source-read crash: abort an in-flight sync task while it is reading.
    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let next_appointments = vec![
        make_appointment(
            "appt-a",
            AppointmentStatus::Scheduled,
            "booked",
            "Initial visit",
            "2026-08-27T15:00:00+00:00",
            "2026-08-27T15:30:00+00:00",
            Some("MRN-R1"),
        ),
        make_appointment(
            "appt-b",
            AppointmentStatus::Cancelled,
            "cancelled",
            "Follow-up visit",
            "2026-08-27T14:00:00+00:00",
            "2026-08-27T14:30:00+00:00",
            None,
        ),
        make_appointment(
            "appt-c",
            AppointmentStatus::Scheduled,
            "booked",
            "Added visit",
            "2026-08-27T16:00:00+00:00",
            "2026-08-27T16:30:00+00:00",
            Some("MRN-R1"),
        ),
    ];
    let gated = Arc::new(GatedScheduleSource {
        source_id: source_id.clone(),
        entered_read: Arc::clone(&entered),
        release_read: Arc::clone(&release),
        appointments: next_appointments.clone(),
    });
    let store_in_flight = store.clone();
    let auth_in_flight = Arc::clone(&auth);
    let practice_in_flight = practice.clone();
    let in_flight = tokio::spawn(async move {
        store_in_flight
            .sync_practice_schedule(&*auth_in_flight, &practice_in_flight, &*gated, 2_100)
            .await
    });
    entered.notified().await;
    in_flight.abort();
    let _ = in_flight.await;

    // Reconnect a brand-new Store ("restart"): prior schedule at t=2000 is completely intact.
    let restarted_store = Store::connect(&url).await.unwrap();
    let after_abort = restarted_store.practice_schedule(&practice).await.unwrap();
    assert_eq!(after_abort.appointments.len(), 2);
    assert_eq!(
        after_abort.appointments[0].window.starts_at(),
        "2026-08-27T13:00:00.000Z"
    );
    assert_eq!(after_abort.appointments[0].saved_version, 1);
    assert_eq!(
        after_abort.appointments[1].status,
        AppointmentStatus::Scheduled
    );

    // 2. Mid-transaction crash after writing the first two rows: inject a one-practice
    //    Postgres trigger that aborts on inserting `appt-c`, proving atomic rollback.
    let trigger_fn = format!("abort_mid_sched_{}", Uuid::new_v4().simple());
    let trigger_sql = format!(
        "CREATE FUNCTION {trigger_fn}() RETURNS trigger AS $$ BEGIN
           IF NEW.practice = '{practice}' AND NEW.appointment_id = 'appt-c' THEN
             RAISE EXCEPTION 'simulated mid-transaction restart';
           END IF;
           RETURN NEW;
         END $$ LANGUAGE plpgsql;
         CREATE TRIGGER {trigger_fn} BEFORE INSERT ON physician_runtime_appointments
           FOR EACH ROW EXECUTE FUNCTION {trigger_fn}();"
    );
    sea_orm::sqlx::raw_sql(sea_orm::sqlx::AssertSqlSafe(trigger_sql))
        .execute(auth.pool())
        .await
        .unwrap();

    let next_source = StubScheduleSource::new(StubSchedulePlan::Complete(next_appointments));
    let tx_err = restarted_store
        .sync_practice_schedule(&*auth, &practice, &next_source, 2_150)
        .await
        .expect_err("mid-transaction crash must abort");
    assert!(matches!(tx_err, ScheduleError::Database(_)));

    // Remove the simulated mid-transaction fault and verify rollback left t=2000 untouched.
    let cleanup_sql = format!(
        "DROP TRIGGER {trigger_fn} ON physician_runtime_appointments;
         DROP FUNCTION {trigger_fn}();"
    );
    sea_orm::sqlx::raw_sql(sea_orm::sqlx::AssertSqlSafe(cleanup_sql))
        .execute(auth.pool())
        .await
        .unwrap();

    let after_rollback = restarted_store.practice_schedule(&practice).await.unwrap();
    assert_eq!(after_rollback.appointments.len(), 2);
    assert_eq!(
        after_rollback.appointments[0].window.starts_at(),
        "2026-08-27T13:00:00.000Z"
    );
    assert_eq!(after_rollback.appointments[0].saved_version, 1);
    assert_eq!(
        after_rollback.appointments[1].status,
        AppointmentStatus::Scheduled
    );
    assert_eq!(after_rollback.appointments[1].saved_version, 1);

    // Resume sync after restart: applies cleanly once, losing and duplicating nothing.
    let recovered = restarted_store
        .sync_practice_schedule(&*auth, &practice, &next_source, 2_200)
        .await
        .unwrap();
    let ScheduleSyncOutcome::Applied(recovered_summary) = recovered else {
        panic!("expected applied recovery");
    };
    assert_eq!(recovered_summary.saved_appointments, 3);
    assert_eq!(recovered_summary.changed_appointments, 3);

    let final_sched = restarted_store.practice_schedule(&practice).await.unwrap();
    assert_eq!(final_sched.appointments.len(), 3);
    let by_id: std::collections::BTreeMap<_, _> = final_sched
        .appointments
        .iter()
        .map(|a| (a.appointment_id.as_str(), a))
        .collect();
    assert_eq!(by_id["appt-a"].saved_version, 2);
    assert_eq!(
        by_id["appt-a"].window.starts_at(),
        "2026-08-27T15:00:00.000Z"
    );
    assert_eq!(by_id["appt-b"].saved_version, 2);
    assert_eq!(by_id["appt-b"].status, AppointmentStatus::Cancelled);
    assert_eq!(by_id["appt-c"].saved_version, 1);
}

#[tokio::test]
async fn ac3_failed_read_keeps_last_good_schedule_and_time_and_first_failure_is_unavailable() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;

    // Practice with no prior good read reports Unavailable (no fabricated as_of).
    let fresh_practice = format!("practice-sched-unavail-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&fresh_practice, PracticeClock::Fixed(1_787_846_400))
        .await
        .unwrap();
    let failing = StubScheduleSource::new(StubSchedulePlan::FailIo(
        "calendar endpoint connection reset".to_owned(),
    ));
    let first_fail = store
        .sync_practice_schedule(&*auth, &fresh_practice, &failing, 3_000)
        .await
        .unwrap();
    assert!(matches!(
        first_fail,
        ScheduleSyncOutcome::FailedRead {
            standing: ScheduleStanding::Unavailable { .. },
            retained_appointments: 0,
        }
    ));
    let unavail_readback = store.practice_schedule(&fresh_practice).await.unwrap();
    assert!(unavail_readback.appointments.is_empty());
    assert!(matches!(
        unavail_readback.standing,
        ScheduleStanding::Unavailable { .. }
    ));

    // Practice with a saved good schedule preserves its visits and exact as_of when a refresh fails.
    let practice = format!("practice-sched-fail-keep-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(1_787_846_400))
        .await
        .unwrap();
    let ws = format!("pw-keep-{}", Uuid::new_v4().simple());
    register_roster_patient_with_pull(&store, &auth, &practice, &ws, "MRN-K1", "Kim Park", 4_000)
        .await;
    let source_id = SourceId::new("synthetic-practice-calendar").unwrap();
    store
        .seed_schedule_links_from_pulls(&*auth, &practice, &source_id, 4_000)
        .await
        .unwrap();

    let source = StubScheduleSource::new(StubSchedulePlan::Complete(vec![
        make_appointment(
            "appt-keep-1",
            AppointmentStatus::Scheduled,
            "booked",
            "Follow-up visit",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-K1"),
        ),
        make_appointment(
            "appt-keep-2",
            AppointmentStatus::Scheduled,
            "booked",
            "New patient visit",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            None,
        ),
    ]));
    store
        .sync_practice_schedule(&*auth, &practice, &source, 4_000)
        .await
        .unwrap();

    source.set_plan(StubSchedulePlan::FailIo("upstream 503 timeout".to_owned()));
    let failed_refresh = store
        .sync_practice_schedule(&*auth, &practice, &source, 4_500)
        .await
        .unwrap();
    let ScheduleSyncOutcome::FailedRead {
        standing,
        retained_appointments,
    } = failed_refresh
    else {
        panic!("expected failed read outcome");
    };
    assert_eq!(retained_appointments, 2);
    match standing {
        ScheduleStanding::Usable {
            schedule_version,
            as_of,
            freshness: ScheduleFreshness::RefreshFailed { failure },
            ..
        } => {
            assert_eq!(schedule_version, 1);
            assert_eq!(as_of, 4_000);
            assert_eq!(failure.failed_at, 4_500);
            assert!(failure.reason.contains("upstream 503 timeout"));
        }
        other => panic!("unexpected standing: {other:?}"),
    }

    let preserved = store.practice_schedule(&practice).await.unwrap();
    assert_eq!(preserved.appointments.len(), 2);
    assert_eq!(preserved.appointments[0].as_of, 4_000);
    assert_eq!(preserved.appointments[1].as_of, 4_000);
}

#[tokio::test]
async fn ac3_partial_read_keeps_delivered_and_prior_visits_and_names_what_failed() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let practice = format!("practice-sched-partial-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(1_787_846_400))
        .await
        .unwrap();

    let ws = format!("pw-partial-{}", Uuid::new_v4().simple());
    register_roster_patient_with_pull(&store, &auth, &practice, &ws, "MRN-P1", "Priya Nair", 5_000)
        .await;
    let source_id = SourceId::new("synthetic-practice-calendar").unwrap();
    store
        .seed_schedule_links_from_pulls(&*auth, &practice, &source_id, 5_000)
        .await
        .unwrap();

    // 1. Initial partial read saves the visits it got and names what failed.
    let source = StubScheduleSource::new(StubSchedulePlan::Partial {
        appointments: vec![make_appointment(
            "appt-p1",
            AppointmentStatus::Scheduled,
            "booked",
            "Medication follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-P1"),
        )],
        issues: vec![
            ScheduleReadIssue::new("Appointment/appt-corrupt", "invalid start timestamp").unwrap(),
        ],
    });

    let first = store
        .sync_practice_schedule(&*auth, &practice, &source, 5_000)
        .await
        .unwrap();
    let ScheduleSyncOutcome::Applied(first_summary) = first else {
        panic!("expected applied partial sync");
    };
    assert_eq!(first_summary.saved_appointments, 1);

    let after_first = store.practice_schedule(&practice).await.unwrap();
    assert_eq!(after_first.appointments.len(), 1);
    match &after_first.standing {
        ScheduleStanding::Usable {
            as_of,
            freshness:
                ScheduleFreshness::Partial {
                    failures,
                    last_complete_as_of,
                },
            ..
        } => {
            assert_eq!(*as_of, 5_000);
            assert_eq!(*last_complete_as_of, None);
            assert_eq!(failures.len(), 1);
            assert_eq!(failures.as_slice()[0].target(), "Appointment/appt-corrupt");
            assert_eq!(failures.as_slice()[0].reason(), "invalid start timestamp");
        }
        other => panic!("expected partial usable standing, got {other:?}"),
    }

    // 2. Complete read adds appt-p2 and clears partial issues.
    source.set_plan(StubSchedulePlan::Complete(vec![
        make_appointment(
            "appt-p1",
            AppointmentStatus::Scheduled,
            "booked",
            "Medication follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-P1"),
        ),
        make_appointment(
            "appt-p2",
            AppointmentStatus::Scheduled,
            "booked",
            "Annual physical",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            None,
        ),
    ]));
    store
        .sync_practice_schedule(&*auth, &practice, &source, 5_100)
        .await
        .unwrap();

    // 3. Subsequent partial read where appt-p1 is rescheduled and appt-p2 failed to read:
    //    appt-p1 updates, appt-p2 is preserved (NOT withdrawn!), and appt-p2's failure is named.
    source.set_plan(StubSchedulePlan::Partial {
        appointments: vec![make_appointment(
            "appt-p1",
            AppointmentStatus::Scheduled,
            "booked",
            "Medication follow-up",
            "2026-08-27T19:30:00+00:00",
            "2026-08-27T20:00:00+00:00",
            Some("MRN-P1"),
        )],
        issues: vec![
            ScheduleReadIssue::new("Appointment/appt-p2", "corrupt upstream payload").unwrap(),
        ],
    });
    store
        .sync_practice_schedule(&*auth, &practice, &source, 5_200)
        .await
        .unwrap();

    let after_partial = store.practice_schedule(&practice).await.unwrap();
    assert_eq!(after_partial.appointments.len(), 2);
    let by_id: std::collections::BTreeMap<_, _> = after_partial
        .appointments
        .iter()
        .map(|a| (a.appointment_id.as_str(), a))
        .collect();
    assert_eq!(
        by_id["appt-p1"].window.starts_at(),
        "2026-08-27T19:30:00.000Z"
    );
    assert_eq!(by_id["appt-p1"].saved_version, 2);
    assert_eq!(by_id["appt-p2"].status, AppointmentStatus::Scheduled);
    assert_eq!(by_id["appt-p2"].saved_version, 1);
    assert_eq!(by_id["appt-p2"].as_of, 5_100);

    match &after_partial.standing {
        ScheduleStanding::Usable {
            as_of,
            freshness:
                ScheduleFreshness::Partial {
                    failures,
                    last_complete_as_of,
                },
            ..
        } => {
            assert_eq!(*as_of, 5_200);
            assert_eq!(*last_complete_as_of, Some(5_100));
            assert_eq!(failures.len(), 1);
            assert_eq!(failures.as_slice()[0].target(), "Appointment/appt-p2");
            assert_eq!(failures.as_slice()[0].reason(), "corrupt upstream payload");
        }
        other => panic!("expected partial usable standing, got {other:?}"),
    }
}
