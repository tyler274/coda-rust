//! Real PostgreSQL integration tests for scheduled visit preparation selection,
//! pre-visit clinical briefings keyed to visit time, mid-run record/schedule
//! change handling, failed-refresh honesty, and pre-visit concern lifecycle.
//!
//! Runs automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`,
//! `PHYSICIAN_DATABASE_URL`, or `PHYSICIAN_TEST_DATABASE_URL` is set; panics if
//! none is set when `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use harness::provider::{QuotaGate, ResponsesProvider};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision,
};
use local_product::runtime::briefings::{
    BriefingError, ConcernOpenOutcome, VisitConcernOpenState, VisitPreparationState, VisitSlotKey,
    VisitStanding, should_retire_visit_concern, visit_can_start_preparation,
};
use local_product::runtime::chat_operations::{RecordLimit, RecordLimitKind};
use local_product::runtime::concerns::{
    CommandKey, Concern, ConcernError, ConcernId, ConcernStanding, ConcernUrgency,
    ExactRecordReference, RetiredConcernUpdate, WorkspaceAccess,
};
use local_product::runtime::delivery::{Delivery, DeliveryState};
use local_product::runtime::jobs::{
    Assignment, PracticeAccessError, PracticeAuthority, PracticeJobHost, PracticeJobKind,
    PracticeScope,
};
use local_product::runtime::refresh::{
    FixedSourceCheck, RefreshCommandKey, RefreshRequest, RefreshTarget, SourceCheckOutcome,
    SourceId as RefreshSourceId,
};
use local_product::runtime::schedule::{
    AppointmentId, AppointmentStatus, AppointmentWindow, ScheduleDelivery, ScheduleSource,
    ScheduleSourceError, SourceAppointment, SourceAppointmentParts, SourceId, SourcePatientId,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, WorkStatus, engine};
use local_product::visit_preparation::{
    BriefingClaimInput, BriefingDecisionInput, SAVE_BRIEFING_TOOL, SELECT_VISITS_TOOL,
    SaveBriefingArgs, VISIT_PREPARATION_SKILL, VisitConcernInput,
};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use uuid::Uuid;

const CALENDAR_SOURCE_ID: &str = "synthetic-practice-calendar";
const CALENDAR_SOURCE_LABEL: &str = "Synthetic practice calendar";
const CHART_SOURCE_ID: &str = "synthetic-charts";
const CHART_SOURCE_LABEL: &str = "Synthetic chart";

/// Fixed simulated practice time: `2026-08-27T14:00:00Z`.
const BASE_NOW_UNIX: i64 = 1_787_839_200;

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

struct ChartDoc<'a> {
    workspace: &'a str,
    source_id: &'a str,
    candidate_id: &'a str,
    revision: &'a str,
    record_type: &'a str,
    title: &'a str,
    date: &'a str,
    text: &'a str,
}

fn make_record(doc: ChartDoc<'_>) -> SourceRevision {
    SourceRevision {
        workspace: doc.workspace.to_owned(),
        source_id: doc.source_id.to_owned(),
        candidate_id: doc.candidate_id.to_owned(),
        revision: doc.revision.to_owned(),
        record_type: doc.record_type.to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: doc.date.to_owned(),
        }],
        content: doc.text.to_owned(),
        sha256: sha256_hex(doc.text.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: doc.title.to_owned(),
            text: doc.text.to_owned(),
        }],
        provenance: json!({
            "title": doc.title,
            "record_locator": doc.candidate_id,
        }),
    }
}

fn source_patient(mrn: &str, name: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: mrn.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some("1968-04-19".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "urn:mrn".to_owned(),
            value: mrn.to_owned(),
        }],
    }
}

struct RosterPatient<'a> {
    practice: &'a str,
    workspace: &'a str,
    mrn: &'a str,
    display_name: &'a str,
    records: &'a [SourceRevision],
    pulled_at: i64,
}

async fn seed_roster_patient(store: &Store, auth: &PostgresAuthStore, patient: RosterPatient<'_>) {
    let pull = SourcePull {
        source_id: CHART_SOURCE_ID.to_owned(),
        source_label: CHART_SOURCE_LABEL.to_owned(),
        patient: source_patient(patient.mrn, patient.display_name),
        pulled_at: patient.pulled_at,
    };
    store
        .record_chart(patient.workspace, patient.records, &pull)
        .await
        .expect("record synthetic chart");
    auth.register_patient(patient.practice, patient.workspace)
        .await
        .expect("register roster patient");
    store
        .seed_schedule_patient_link(
            auth,
            patient.practice,
            &SourceId::new(CALENDAR_SOURCE_ID).unwrap(),
            &SourcePatientId::new(patient.mrn).unwrap(),
            patient.workspace,
            patient.pulled_at,
        )
        .await
        .expect("seed schedule patient link");
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
        "status": source_status,
        "description": label,
        "start": starts_at,
        "end": ends_at,
        "participant": mrn
            .map(|m| json!([{ "actor": { "reference": format!("Patient/{m}") }, "status": "accepted" }]))
            .unwrap_or_else(|| json!([])),
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

struct InMemoryScheduleSource {
    source_id: SourceId,
    appointments: Mutex<Vec<SourceAppointment>>,
}

impl InMemoryScheduleSource {
    fn new(appointments: Vec<SourceAppointment>) -> Self {
        Self {
            source_id: SourceId::new(CALENDAR_SOURCE_ID).unwrap(),
            appointments: Mutex::new(appointments),
        }
    }

    fn set_appointments(&self, appointments: Vec<SourceAppointment>) {
        *self.appointments.lock().unwrap() = appointments;
    }
}

#[async_trait]
impl ScheduleSource for InMemoryScheduleSource {
    fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        CALENDAR_SOURCE_LABEL
    }

    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError> {
        let appointments = self.appointments.lock().unwrap().clone();
        Ok(ScheduleDelivery::Complete { appointments })
    }
}

/// Scripted multi-turn provider capturing every [`engine::recipe::Request`] sent to the model.
struct ScriptedProvider {
    turns: VecDeque<Result<engine::Turn, String>>,
    requests: Vec<engine::recipe::Request>,
}

impl ScriptedProvider {
    fn new(turns: Vec<Result<engine::Turn, String>>) -> Self {
        Self {
            turns: VecDeque::from(turns),
            requests: Vec::new(),
        }
    }
}

impl engine::recipe::Provider for ScriptedProvider {
    async fn exchange(
        &mut self,
        request: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        self.requests.push(request.clone());
        match self
            .turns
            .pop_front()
            .expect("scripted provider ran out of turns")
        {
            Ok(turn) => Ok(turn),
            Err(err) => Err(err.into()),
        }
    }
}

fn call_turn(id: &str, name: &str, args: Value) -> Result<engine::Turn, String> {
    Ok(engine::Turn {
        items: vec![engine::Item::Call(engine::Invocation {
            id: id.to_owned(),
            name: name.to_owned(),
            arguments: serde_json::to_string(&args).unwrap(),
        })],
        receipt: json!({ "call_id": id }),
        usage: Default::default(),
    })
}

fn done_turn(text: &str) -> Result<engine::Turn, String> {
    Ok(engine::Turn {
        items: vec![engine::Item::assistant(text)],
        receipt: json!({ "done": true }),
        usage: Default::default(),
    })
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
    now_secs: u64,
) -> (PhysicianIdentity, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let invite = auth
        .invite(practice_id, &email, label, false, now_secs)
        .await
        .unwrap();
    let (identity, token) = auth
        .redeem_invitation(&email, &invite, now_secs)
        .await
        .unwrap();
    let hash = PostgresAuthStore::session_reference(&token);
    (identity, hash)
}

async fn open_concerns_for_workspaces(
    store: &Store,
    auth: &PostgresAuthStore,
    practice: &str,
    workspaces: &[String],
) -> Vec<Concern> {
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (doc, hash) = invite_and_sign_in(auth, practice, "DrTodayReader", now_secs).await;
    let mut authority = PostgresWorkspaceAuthority;
    let mut all = Vec::new();
    for ws in workspaces {
        auth.set_membership(&doc, ws, Membership::Allowed)
            .await
            .unwrap();
        let items = store
            .open_concerns(
                &WorkspaceAccess {
                    practice,
                    physician: &doc.physician_id,
                    workspace: ws,
                    session_hash: &hash,
                },
                &mut authority,
            )
            .await
            .unwrap();
        all.extend(items);
    }
    all
}

async fn retired_concerns_for_workspace(
    store: &Store,
    auth: &PostgresAuthStore,
    practice: &str,
    workspace: &str,
) -> Vec<RetiredConcernUpdate> {
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (doc, hash) = invite_and_sign_in(auth, practice, "DrUpdatesReader", now_secs).await;
    auth.set_membership(&doc, workspace, Membership::Allowed)
        .await
        .unwrap();
    let mut authority = PostgresWorkspaceAuthority;
    store
        .retired_concern_updates(
            &WorkspaceAccess {
                practice,
                physician: &doc.physician_id,
                workspace,
                session_hash: &hash,
            },
            &mut authority,
        )
        .await
        .unwrap()
}

#[derive(Default)]
struct NoopQuotaGate;

#[async_trait]
impl QuotaGate for NoopQuotaGate {
    async fn remaining(&self) -> Result<Duration, engine::Cause> {
        Ok(Duration::ZERO)
    }
    async fn defer(&self, _: Duration) -> Result<(), engine::Cause> {
        Ok(())
    }
}

fn frame_responses_sse(mut response: Value) -> String {
    let mut events = vec![json!({"type":"response.created","response":{"id":response["id"]}})];
    if response["status"] == "completed" {
        for (index, item) in response["output"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
        {
            if item.get("id").is_none() {
                item["id"] = json!(format!("item-{index}"));
            }
            let id = &item["id"];
            events.push(
                json!({"type":"response.output_item.added","output_index":index,"item":item}),
            );
            if item["type"] == "function_call" {
                events.push(json!({"type":"response.function_call_arguments.delta","output_index":index,"item_id":id,"delta":item["arguments"]}));
                events.push(json!({"type":"response.function_call_arguments.done","output_index":index,"item_id":id,"arguments":item["arguments"]}));
            } else if item["type"] == "message" {
                for (part, content) in item["content"].as_array().unwrap().iter().enumerate() {
                    events.push(json!({"type":"response.content_part.added","output_index":index,"item_id":id,"content_index":part,"part":{"type":"output_text","text":""}}));
                    events.push(json!({"type":"response.output_text.delta","output_index":index,"item_id":id,"content_index":part,"delta":content["text"]}));
                    events.push(json!({"type":"response.output_text.done","output_index":index,"item_id":id,"content_index":part,"text":content["text"]}));
                    events.push(json!({"type":"response.content_part.done","output_index":index,"item_id":id,"content_index":part,"part":content}));
                }
            }
            events
                .push(json!({"type":"response.output_item.done","output_index":index,"item":item}));
        }
    }
    events.push(json!({
        "type": format!("response.{}", response["status"].as_str().unwrap()),
        "response": response,
    }));
    events
        .into_iter()
        .enumerate()
        .map(|(index, mut event)| {
            event["sequence_number"] = json!(index);
            format!("data: {event}\n\n")
        })
        .collect()
}

async fn spawn_responses_sse_server(
    responses: Vec<Value>,
) -> (String, tokio::task::JoinHandle<Vec<Value>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/v1/responses", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        let mut recorded_requests = Vec::new();
        for payload in responses {
            let body = frame_responses_sse(payload);
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut bytes = Vec::new();
            let (offset, len) = loop {
                let mut chunk = [0_u8; 2048];
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0, "unexpected EOF reading Responses request");
                bytes.extend_from_slice(&chunk[..count]);
                if let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
                    let header = String::from_utf8_lossy(&bytes[..end]);
                    let len = header
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().unwrap())
                        })
                        .unwrap();
                    break (end + 4, len);
                }
            };
            while bytes.len() < offset + len {
                let mut chunk = [0_u8; 2048];
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0, "unexpected EOF reading Responses body");
                bytes.extend_from_slice(&chunk[..count]);
            }
            let req: Value = serde_json::from_slice(&bytes[offset..offset + len]).unwrap();
            recorded_requests.push(req);
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        }
        recorded_requests
    });
    (endpoint, handle)
}

/// Across 5 synthetic visits, scheduled wake-up supplies appointment + patient
/// context to the `visit-preparation` recipe on `roles.background`, records the
/// recipe, model settings, and preloaded skill digest, and persists each visit's
/// `prepare_now` / `schedule` / `defer` / `skip` selection with its clinical reason.
#[tokio::test]
async fn ac1_five_synthetic_visits_select_which_to_prepare_and_when_with_grounded_reasons() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-vp-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    // 5 distinct synthetic patients and scheduled visit scenarios:
    let ws1 = format!("pw-discharge-{}", Uuid::new_v4().simple());
    let ws2 = format!("pw-ckd-dm-{}", Uuid::new_v4().simple());
    let ws3 = format!("pw-vaccine-{}", Uuid::new_v4().simple());
    let ws4 = format!("pw-pending-lab-{}", Uuid::new_v4().simple());
    let ws5 = format!("pw-in-progress-{}", Uuid::new_v4().simple());

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws1,
            mrn: "MRN-VP-1",
            display_name: "Rosa Morales",
            records: &[make_record(ChartDoc {
                workspace: &ws1,
                source_id: CHART_SOURCE_ID,
                candidate_id: "DocumentReference/discharge-hf-1",
                revision: "rev-1",
                record_type: "DocumentReference",
                title: "Hospital Discharge Summary",
                date: "2026-08-24",
                text: "Discharged 3 days ago after acute decompensated HFrEF; bumetanide switched to torsemide 20 mg daily, basic metabolic panel due at post-discharge visit.",
            })],
            pulled_at: BASE_NOW_UNIX - 3600,
        },
    )
    .await;

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws2,
            mrn: "MRN-VP-2",
            display_name: "Marcus Vance",
            records: &[make_record(ChartDoc {
                workspace: &ws2,
                source_id: CHART_SOURCE_ID,
                candidate_id: "Observation/egfr-ckd-1",
                revision: "rev-1",
                record_type: "Observation",
                title: "Renal & Glycemic Panel",
                date: "2026-08-20",
                text: "Stage 3b CKD (eGFR 36 mL/min) and type 2 diabetes (A1c 7.9%) on empagliflozin and glargine; nephrology co-management note attached.",
            })],
            pulled_at: BASE_NOW_UNIX - 3600,
        },
    )
    .await;

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws3,
            mrn: "MRN-VP-3",
            display_name: "Hannah Cho",
            records: &[make_record(ChartDoc {
                workspace: &ws3,
                source_id: CHART_SOURCE_ID,
                candidate_id: "DocumentReference/well-visit-1",
                revision: "rev-1",
                record_type: "DocumentReference",
                title: "Annual Preventive Visit",
                date: "2026-08-10",
                text: "Healthy 34-year-old seen two weeks ago with normal labs and BP 114/72; returning strictly for seasonal influenza immunization.",
            })],
            pulled_at: BASE_NOW_UNIX - 3600,
        },
    )
    .await;

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws4,
            mrn: "MRN-VP-4",
            display_name: "David Okafor",
            records: &[make_record(ChartDoc {
                workspace: &ws4,
                source_id: CHART_SOURCE_ID,
                candidate_id: "ServiceRequest/tsh-order-1",
                revision: "rev-1",
                record_type: "ServiceRequest",
                title: "Pending Thyroid Panel Order",
                date: "2026-08-26",
                text: "Post-methimazole dose adjustment; Quest TSH and free T4 drawn yesterday evening, results expected tonight before Friday endocrine follow-up.",
            })],
            pulled_at: BASE_NOW_UNIX - 3600,
        },
    )
    .await;

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws5,
            mrn: "MRN-VP-5",
            display_name: "Evelyn Park",
            records: &[make_record(ChartDoc {
                workspace: &ws5,
                source_id: CHART_SOURCE_ID,
                candidate_id: "DocumentReference/triage-dyspnea-1",
                revision: "rev-1",
                record_type: "DocumentReference",
                title: "Same-Day Triage Note",
                date: "2026-08-27",
                text: "COPD GOLD B presenting with 4 days of increased sputum purulence and resting SpO2 91% on room air; nebulized bronchodilator given in room.",
            })],
            pulled_at: BASE_NOW_UNIX - 1200,
        },
    )
    .await;

    // BASE_NOW_UNIX = 2026-08-27T14:00:00Z
    let appointments = vec![
        // Visit 1: today in 2 hours (16:00–16:30Z) -> PrepareNow
        make_appointment(
            "appt-discharge-1",
            AppointmentStatus::Scheduled,
            "booked",
            "Hospital discharge follow-up",
            "2026-08-27T16:00:00+00:00",
            "2026-08-27T16:30:00+00:00",
            Some("MRN-VP-1"),
        ),
        // Visit 2: tomorrow (2026-08-28T15:00–15:30Z) -> Schedule ahead at 2026-08-28T03:00Z
        make_appointment(
            "appt-ckd-2",
            AppointmentStatus::Scheduled,
            "booked",
            "CKD & diabetes follow-up",
            "2026-08-28T15:00:00+00:00",
            "2026-08-28T15:30:00+00:00",
            Some("MRN-VP-2"),
        ),
        // Visit 3: today (18:00–18:15Z) -> Skip
        make_appointment(
            "appt-flu-3",
            AppointmentStatus::Scheduled,
            "booked",
            "Nurse immunization visit",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:15:00+00:00",
            Some("MRN-VP-3"),
        ),
        // Visit 4: in 48h (2026-08-29T14:00–14:30Z) -> Defer until 2026-08-28T12:00Z
        make_appointment(
            "appt-thyroid-4",
            AppointmentStatus::Scheduled,
            "booked",
            "Hyperthyroidism titration check",
            "2026-08-29T14:00:00+00:00",
            "2026-08-29T14:30:00+00:00",
            Some("MRN-VP-4"),
        ),
        // Visit 5: currently IN PROGRESS (13:45–14:15Z while now is 14:00Z) -> PrepareNow
        make_appointment(
            "appt-copd-5",
            AppointmentStatus::Scheduled,
            "arrived",
            "Same-day acute dyspnea visit",
            "2026-08-27T13:45:00+00:00",
            "2026-08-27T14:15:00+00:00",
            Some("MRN-VP-5"),
        ),
    ];

    let cal = InMemoryScheduleSource::new(appointments);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 300)
        .await
        .unwrap();

    let v1_start =
        AppointmentWindow::parse("2026-08-27T16:00:00+00:00", "2026-08-27T16:30:00+00:00")
            .unwrap()
            .starts_at_unix();
    let v2_start =
        AppointmentWindow::parse("2026-08-28T15:00:00+00:00", "2026-08-28T15:30:00+00:00")
            .unwrap()
            .starts_at_unix();
    let v3_start =
        AppointmentWindow::parse("2026-08-27T18:00:00+00:00", "2026-08-27T18:15:00+00:00")
            .unwrap()
            .starts_at_unix();
    let v4_start =
        AppointmentWindow::parse("2026-08-29T14:00:00+00:00", "2026-08-29T14:30:00+00:00")
            .unwrap()
            .starts_at_unix();
    let v5_start =
        AppointmentWindow::parse("2026-08-27T13:45:00+00:00", "2026-08-27T14:15:00+00:00")
            .unwrap()
            .starts_at_unix();

    let v2_prepare_at = v2_start - 12 * 3600;
    let v4_check_again_at = BASE_NOW_UNIX + 22 * 3600;

    let selection_call = json!({
        "decisions": [
            {
                "source_id": CALENDAR_SOURCE_ID,
                "appointment_id": "appt-discharge-1",
                "starts_at_unix": v1_start,
                "action": "prepare_now",
                "reason": "Post-HFrEF discharge visit in 2 hours with diuretic transition from bumetanide to torsemide 20 mg requiring volume and electrolyte review."
            },
            {
                "source_id": CALENDAR_SOURCE_ID,
                "appointment_id": "appt-ckd-2",
                "starts_at_unix": v2_start,
                "action": "schedule",
                "prepare_at_unix": v2_prepare_at,
                "reason": "Tomorrow CKD 3b and type 2 diabetes co-managed visit benefits from overnight preparation once today's external lab window closes."
            },
            {
                "source_id": CALENDAR_SOURCE_ID,
                "appointment_id": "appt-flu-3",
                "starts_at_unix": v3_start,
                "action": "skip",
                "reason": "Healthy patient evaluated two weeks ago with normal labs returning solely for routine nurse influenza vaccination."
            },
            {
                "source_id": CALENDAR_SOURCE_ID,
                "appointment_id": "appt-thyroid-4",
                "starts_at_unix": v4_start,
                "action": "defer",
                "check_again_at_unix": v4_check_again_at,
                "reason": "Deferring preparation until Quest TSH and free T4 drawn 2026-08-26 post so methimazole options reflect current thyroid function."
            },
            {
                "source_id": CALENDAR_SOURCE_ID,
                "appointment_id": "appt-copd-5",
                "starts_at_unix": v5_start,
                "action": "prepare_now",
                "reason": "In-progress same-day acute COPD exacerbation visit with SpO2 91% and purulent sputum requiring immediate synthesis."
            }
        ]
    });

    // Briefing turns for the two `prepare_now` visits (sorted by starts_at_unix: appt-copd-5 first, then appt-discharge-1):
    let copd_briefing = SaveBriefingArgs {
        visit_card_orientation: "Acute COPD GOLD B exacerbation in room (SpO2 91% RA, 4d purulent sputum); antibiotic and steroid options open.".into(),
        trajectory: vec![BriefingClaimInput {
            text: "COPD GOLD B presenting today with 4 days of purulent sputum and resting SpO2 91% on room air.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/triage-dyspnea-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Resting oxygenation dropped to 91% on room air with nebulized bronchodilator administered in triage.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/triage-dyspnea-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "No chest radiograph from today is in the chart to exclude concurrent pneumonia.".into(),
            uncertainty: Some("Infiltrate vs uncomplicated acute bronchitis remains unconfirmed until exam and imaging decision.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/triage-dyspnea-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Systemic corticosteroid burst, empiric antibiotic coverage, and chest imaging threshold".into(),
            clinical_rationale: "Increased sputum purulence with SpO2 91% meets cardinal COPD exacerbation criteria while focal pneumonia should be ruled out if lung exam is asymmetric.".into(),
            options_or_questions: vec![
                "Consider a 5-day oral prednisone 40 mg burst alongside short-course antibiotic coverage if cardinal symptoms are confirmed on exam.".into(),
                "Consider same-day chest radiograph and repeat post-bronchodilator pulse oximetry to confirm safe outpatient management vs ED transfer.".into(),
            ],
            uncertainty: Some("Prior sputum culture history is not present in the connected chart.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/triage-dyspnea-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        basis_and_limitations: "Prepared from same-day triage note (2026-08-27); prior spirometry and sputum microbiology are not in the connected chart.".into(),
        basis_citations: vec![ExactRecordReference {
            candidate_id: "DocumentReference/triage-dyspnea-1".into(),
            revision: "rev-1".into(),
        }],
        limits: Vec::new(),
        visit_concern: None,
    };

    let hf_briefing = SaveBriefingArgs {
        visit_card_orientation: "3-day post-HFrEF discharge follow-up after switch from bumetanide to torsemide 20 mg; electrolytes and weight trend need confirmation.".into(),
        trajectory: vec![BriefingClaimInput {
            text: "Hospitalized for acute decompensated HFrEF and discharged 2026-08-24 on oral torsemide 20 mg daily.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/discharge-hf-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Loop diuretic changed at discharge from bumetanide to torsemide 20 mg daily.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/discharge-hf-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Post-discharge basic metabolic panel and daily home weight trend since 2026-08-24 are not yet in the chart.".into(),
            uncertainty: Some("Current potassium and creatinine on torsemide 20 mg are unknown pending today's lab draw.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/discharge-hf-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Loop diuretic dose adjustment and post-discharge electrolyte/renal check".into(),
            clinical_rationale: "Early post-discharge volume status and renal/potassium stability govern torsemide dosing and GDMT sequencing.".into(),
            options_or_questions: vec![
                "Assess standing weight, orthostatic vitals, and edema to decide whether to maintain or adjust torsemide 20 mg daily.".into(),
                "Consider same-day basic metabolic panel before advancing renin-angiotensin or mineralocorticoid therapy.".into(),
            ],
            uncertainty: Some("Post-discharge BMP has not yet resulted.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/discharge-hf-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        basis_and_limitations: "Prepared from hospital discharge summary dated 2026-08-24; outpatient labs since discharge are pending.".into(),
        basis_citations: vec![ExactRecordReference {
            candidate_id: "DocumentReference/discharge-hf-1".into(),
            revision: "rev-1".into(),
        }],
        limits: Vec::new(),
        visit_concern: None,
    };

    let mut provider = ScriptedProvider::new(vec![
        // Selection job (2 turns):
        call_turn("sel-1", SELECT_VISITS_TOOL, selection_call),
        done_turn("Selected visit preparations."),
        // Briefing 1 (`appt-copd-5`, starts 13:45Z): read -> save -> done
        call_turn(
            "read-copd",
            READ_TOOL,
            json!({
                "references": [{ "candidate_id": "DocumentReference/triage-dyspnea-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384
            }),
        ),
        call_turn(
            "save-copd",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&copd_briefing).unwrap(),
        ),
        done_turn("Saved COPD briefing."),
        // Briefing 2 (`appt-discharge-1`, starts 16:00Z): read -> save -> done
        call_turn(
            "read-hf",
            READ_TOOL,
            json!({
                "references": [{ "candidate_id": "DocumentReference/discharge-hf-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384
            }),
        ),
        call_turn(
            "save-hf",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&hf_briefing).unwrap(),
        ),
        done_turn("Saved HF discharge briefing."),
    ]);

    let wake_summary = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut provider,
            BASE_NOW_UNIX,
        )
        .await
        .expect("wake_visit_preparation succeeds");

    // Verify selection job ran on background model with preloaded `visit-preparation` skill + digest,
    // and received all 5 synthetic candidate visits in its starting prompt:
    let sel_scope = wake_summary.selection_job.expect("selection job ran");
    let sel_job = store.practice_job(&sel_scope).await.unwrap().unwrap();
    assert_eq!(sel_job.kind, PracticeJobKind::VisitPreparation);
    assert_eq!(sel_job.status, WorkStatus::Completed);
    assert_eq!(sel_job.assignment.preloaded_skills.len(), 1);
    assert_eq!(
        sel_job.assignment.preloaded_skills[0].name,
        VISIT_PREPARATION_SKILL
    );
    assert_eq!(sel_job.assignment.preloaded_skills[0].sha256.len(), 64);
    assert_eq!(provider.requests[0].model, "background-worker-lane");
    let first_prompt_json = match &provider.requests[0].items[0] {
        engine::Item::Message { text, .. } => text.clone(),
        other => panic!("expected starting user message, got {other:?}"),
    };
    for expected in [
        "appt-discharge-1",
        "appt-ckd-2",
        "appt-flu-3",
        "appt-thyroid-4",
        "appt-copd-5",
        "Rosa Morales",
        "Marcus Vance",
        "Hannah Cho",
        "David Okafor",
        "Evelyn Park",
    ] {
        assert!(
            first_prompt_json.contains(expected),
            "selection context missing {expected}"
        );
    }

    // Verify all 5 decisions and their grounded reasons are retained in Postgres:
    let preps = store
        .visit_preparations_for_practice(&practice)
        .await
        .unwrap();
    assert_eq!(preps.len(), 5);
    let by_appt = |id: &str| {
        preps
            .iter()
            .find(|p| p.slot.appointment_id == id)
            .unwrap_or_else(|| panic!("missing prep for {id}"))
    };

    assert!(matches!(
        &by_appt("appt-discharge-1").state,
        VisitPreparationState::Ready { reason, .. } if reason.contains("torsemide 20 mg")
    ));
    assert!(matches!(
        &by_appt("appt-copd-5").state,
        VisitPreparationState::Ready { reason, .. } if reason.contains("SpO2 91%")
    ));
    assert_eq!(
        by_appt("appt-ckd-2").state,
        VisitPreparationState::Scheduled {
            prepare_at_unix: v2_prepare_at,
            reason: "Tomorrow CKD 3b and type 2 diabetes co-managed visit benefits from overnight preparation once today's external lab window closes.".into(),
        }
    );
    assert_eq!(
        by_appt("appt-flu-3").state,
        VisitPreparationState::Skipped {
            reason: "Healthy patient evaluated two weeks ago with normal labs returning solely for routine nurse influenza vaccination.".into(),
        }
    );
    assert_eq!(
        by_appt("appt-thyroid-4").state,
        VisitPreparationState::Deferred {
            check_again_at_unix: v4_check_again_at,
            reason: "Deferring preparation until Quest TSH and free T4 drawn 2026-08-26 post so methimazole options reflect current thyroid function.".into(),
        }
    );
    assert_eq!(wake_summary.completed_briefings.len(), 2);
}

/// Saved briefings retain trajectory, meaningful changes, unresolved issues,
/// visit decisions, inline uncertainty beside qualified conclusions, verified
/// citations, concise orientation, preparation time, source versions & limits,
/// belong to `(practice, source_id, appointment_id, starts_at_unix)`, and reopen
/// identically across a fresh `Store` connection (simulated restart).
#[tokio::test]
async fn ac2_save_and_reopen_briefing_across_restart_with_citations_orientation_and_visit_time_binding()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-vp-ac2-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-vp-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    let records = vec![
        make_record(ChartDoc {
            workspace: &workspace,
            source_id: CHART_SOURCE_ID,
            candidate_id: "Observation/a1c-2026-02",
            revision: "rev-feb",
            record_type: "Observation",
            title: "Hemoglobin A1c (Feb 2026)",
            date: "2026-02-14",
            text: "Hemoglobin A1c 7.1% on metformin 1000 mg twice daily.",
        }),
        make_record(ChartDoc {
            workspace: &workspace,
            source_id: CHART_SOURCE_ID,
            candidate_id: "Observation/a1c-2026-08",
            revision: "rev-aug",
            record_type: "Observation",
            title: "Hemoglobin A1c & UACR (Aug 2026)",
            date: "2026-08-22",
            text: "Hemoglobin A1c rose to 8.4%; urine albumin-to-creatinine ratio 148 mg/g; eGFR 54 mL/min/1.73m2.",
        }),
        make_record(ChartDoc {
            workspace: &workspace,
            source_id: CHART_SOURCE_ID,
            candidate_id: "DocumentReference/ophtho-gap-1",
            revision: "rev-ophtho",
            record_type: "DocumentReference",
            title: "Primary Care Progress Note",
            date: "2026-05-10",
            text: "Patient reported dilated retinal exam at outside optometry clinic in late 2025; report never received.",
        }),
    ];

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &workspace,
            mrn: "MRN-AC2",
            display_name: "Nadia Haddad",
            records: &records,
            pulled_at: BASE_NOW_UNIX - 1800,
        },
    )
    .await;

    let appt = make_appointment(
        "appt-nadia-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Diabetes & albuminuria follow-up",
        "2026-08-27T17:00:00+00:00",
        "2026-08-27T17:30:00+00:00",
        Some("MRN-AC2"),
    );
    let starts_at_unix = appt.window().starts_at_unix();
    let cal = InMemoryScheduleSource::new(vec![appt]);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 600)
        .await
        .unwrap();

    let save_args = SaveBriefingArgs {
        visit_card_orientation: "T2DM with A1c rise (7.1% -> 8.4%) and new albuminuria (UACR 148 mg/g); SGLT2i/GLP-1RA options open.".into(),
        trajectory: vec![BriefingClaimInput {
            text: "Type 2 diabetes previously near goal on metformin 1000 mg BID (A1c 7.1% in Feb 2026) has progressed over 6 months alongside mildly reduced eGFR 54 mL/min/1.73m2.".into(),
            uncertainty: None,
            citations: vec![
                ExactRecordReference {
                    candidate_id: "Observation/a1c-2026-02".into(),
                    revision: "rev-feb".into(),
                },
                ExactRecordReference {
                    candidate_id: "Observation/a1c-2026-08".into(),
                    revision: "rev-aug".into(),
                },
            ],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "A1c increased from 7.1% (2026-02-14) to 8.4% (2026-08-22) with new moderate-to-severe albuminuria (UACR 148 mg/g).".into(),
            uncertainty: Some("Single spot UACR 148 mg/g has not yet been confirmed on a repeat morning sample to establish persistent albuminuria.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/a1c-2026-08".into(),
                revision: "rev-aug".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Outside optometry dilated retinal exam from late 2025 has not been received in the chart.".into(),
            uncertainty: Some("Retinopathy status is unconfirmed before rapid glycemic lowering with a GLP-1 receptor agonist.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/ophtho-gap-1".into(),
                revision: "rev-ophtho".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Cardiorenal-protective glycemic intensification and albuminuria confirmation".into(),
            clinical_rationale: "Rising A1c (8.4%) paired with UACR 148 mg/g at eGFR 54 favors adding an SGLT2 inhibitor and/or GLP-1 RA after checking adherence and retinopathy status.".into(),
            options_or_questions: vec![
                "Confirm metformin 1000 mg BID adherence and discuss adding an SGLT2 inhibitor for glycemic and renal protection.".into(),
                "Discuss GLP-1 receptor agonist intensification vs confirming outside retinal exam results first.".into(),
                "Consider repeat urine albumin-to-creatinine ratio and ACEi/ARB blood pressure review.".into(),
            ],
            uncertainty: Some("Retinopathy status from outside optometry remains unverified.".into()),
            citations: vec![
                ExactRecordReference {
                    candidate_id: "Observation/a1c-2026-08".into(),
                    revision: "rev-aug".into(),
                },
                ExactRecordReference {
                    candidate_id: "DocumentReference/ophtho-gap-1".into(),
                    revision: "rev-ophtho".into(),
                },
            ],
        }],
        basis_and_limitations: "Prepared from synthetic chart labs through 2026-08-22 (source version 1); outside 2025 optometry retinal report is missing.".into(),
        basis_citations: vec![ExactRecordReference {
            candidate_id: "Observation/a1c-2026-08".into(),
            revision: "rev-aug".into(),
        }],
        limits: vec![RecordLimit {
            kind: RecordLimitKind::Unavailable,
            subject: "outside-optometry".into(),
            detail: "Late-2025 outside dilated retinal exam report has not been transferred.".into(),
        }],
        visit_concern: None,
    };

    let mut provider = ScriptedProvider::new(vec![
        call_turn(
            "sel-ac2",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-nadia-1",
                    "starts_at_unix": starts_at_unix,
                    "action": "prepare_now",
                    "reason": "Same-day diabetes visit with A1c rise to 8.4% and UACR 148 mg/g."
                }]
            }),
        ),
        done_turn("Selected."),
        call_turn(
            "search-ac2",
            SEARCH_TOOL,
            json!({ "query": "A1c albuminuria optometry", "limit": 10 }),
        ),
        call_turn(
            "read-ac2",
            READ_TOOL,
            json!({
                "references": [
                    { "candidate_id": "Observation/a1c-2026-02", "revision": "rev-feb", "cursor": null },
                    { "candidate_id": "Observation/a1c-2026-08", "revision": "rev-aug", "cursor": null },
                    { "candidate_id": "DocumentReference/ophtho-gap-1", "revision": "rev-ophtho", "cursor": null }
                ],
                "max_result_bytes": 32_768
            }),
        ),
        call_turn(
            "save-ac2",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&save_args).unwrap(),
        ),
        done_turn("Briefing saved."),
    ]);

    let summary = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut provider,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();
    assert_eq!(summary.completed_briefings.len(), 1);

    let slot = VisitSlotKey {
        practice: practice.clone(),
        source_id: CALENDAR_SOURCE_ID.into(),
        appointment_id: "appt-nadia-1".into(),
        starts_at_unix,
    };

    // Drop `store` and open a brand-new `Store` connection to verify restart durability:
    drop(store);
    let reopened_store = Store::connect(&url).await.expect("reconnect runtime store");

    let saved = reopened_store
        .visit_briefing(&slot, auth.as_ref())
        .await
        .unwrap()
        .expect("saved briefing reopens across restart");
    assert_eq!(saved.slot, slot);
    assert_eq!(saved.visit_standing, VisitStanding::Current);
    assert_eq!(saved.source_version, 1);
    assert_eq!(saved.source_versions.len(), 3);
    assert_eq!(
        saved
            .source_versions
            .get("Observation/a1c-2026-08")
            .map(String::as_str),
        Some("rev-aug")
    );
    assert!(saved.prepared_at >= BASE_NOW_UNIX);
    assert_eq!(
        saved.visit_card_orientation,
        save_args.visit_card_orientation
    );
    assert_eq!(saved.trajectory, save_args.trajectory);
    assert_eq!(saved.meaningful_changes, save_args.meaningful_changes);
    assert_eq!(saved.unresolved_issues, save_args.unresolved_issues);
    assert_eq!(saved.visit_decisions, save_args.visit_decisions);
    // Uncertainty stays directly on the qualified meaningful_changes claim and visit_decision:
    assert!(
        saved.meaningful_changes[0]
            .uncertainty
            .as_deref()
            .unwrap()
            .contains("repeat morning sample")
    );
    assert!(
        saved.visit_decisions[0]
            .uncertainty
            .as_deref()
            .unwrap()
            .contains("Retinopathy status")
    );
    // Citations open verified chart records with sha256 digest, excerpt, and URI:
    assert_eq!(saved.citations.len(), 3);
    let aug_cit = saved
        .citations
        .iter()
        .find(|c| c.reference.candidate_id == "Observation/a1c-2026-08")
        .expect("verified citation for Aug A1c");
    assert_eq!(aug_cit.reference.revision, "rev-aug");
    assert!(aug_cit.excerpt.contains("8.4%"));
    assert_eq!(aug_cit.sha256.len(), 64);
    assert!(aug_cit.native_uri.as_deref().unwrap().contains(&workspace));
    assert_eq!(saved.limits.len(), 1);
    assert_eq!(saved.limits[0].subject, "outside-optometry");

    // Briefing belongs strictly to its visit time: querying a different starts_at_unix returns None.
    let other_slot = VisitSlotKey {
        starts_at_unix: starts_at_unix + 3600,
        ..slot.clone()
    };
    assert!(
        reopened_store
            .visit_briefing(&other_slot, auth.as_ref())
            .await
            .unwrap()
            .is_none()
    );

    let by_id = reopened_store
        .visit_briefing_by_id(&practice, &saved.briefing_id, auth.as_ref())
        .await
        .unwrap()
        .expect("lookup by briefing_id");
    assert_eq!(by_id, saved);

    // PracticeAuthority enforcement on briefing read entry points:
    // unregistered and foreign-practice patient workspaces are refused!
    assert!(matches!(
        reopened_store
            .visit_briefings_for_workspace(&practice, "pw-unregistered-patient", auth.as_ref())
            .await,
        Err(BriefingError::Access(
            PracticeAccessError::UnregisteredPatient { .. }
        ))
    ));
    let foreign_practice = format!("practice-foreign-{}", Uuid::new_v4().simple());
    assert!(matches!(
        reopened_store
            .visit_briefings_for_workspace(&foreign_practice, &workspace, auth.as_ref())
            .await,
        Err(BriefingError::Access(
            PracticeAccessError::ForeignPractice { .. }
        ))
    ));
    struct ForeignPracticeAuthority {
        owner_practice: String,
    }
    #[async_trait]
    impl PracticeAuthority for ForeignPracticeAuthority {
        async fn check_practice_patient(
            &self,
            scope: &PracticeScope,
        ) -> Result<(), PracticeAccessError> {
            Err(PracticeAccessError::ForeignPractice {
                requested_practice: scope.practice.clone(),
                owner_practice: self.owner_practice.clone(),
                workspace: scope.workspace.clone(),
            })
        }
    }
    assert!(matches!(
        reopened_store
            .visit_briefing(
                &slot,
                &ForeignPracticeAuthority {
                    owner_practice: foreign_practice.clone()
                }
            )
            .await,
        Err(BriefingError::Access(
            PracticeAccessError::ForeignPractice { .. }
        ))
    ));
    assert!(matches!(
        reopened_store
            .visit_briefing_by_id(
                &practice,
                &saved.briefing_id,
                &ForeignPracticeAuthority {
                    owner_practice: foreign_practice
                }
            )
            .await,
        Err(BriefingError::Access(
            PracticeAccessError::ForeignPractice { .. }
        ))
    ));
}

/// Mid-run record arrival refuses stale completion at `save_visit_briefing` AND
/// at the `finish()` gate if another record lands after save, bringing the
/// updated record references back to the run until the latest chart version is
/// read and saved.
#[tokio::test]
async fn ac3_mid_run_record_change_refuses_stale_completion_and_brings_current_evidence_back() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-vp-midrun-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-vp-midrun-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    let rec_v1 = make_record(ChartDoc {
        workspace: &workspace,
        source_id: CHART_SOURCE_ID,
        candidate_id: "Observation/k-panel-1",
        revision: "rev-1",
        record_type: "Observation",
        title: "Baseline Potassium",
        date: "2026-08-15",
        text: "Serum potassium 4.3 mEq/L, creatinine 1.1 mg/dL on spironolactone 25 mg daily.",
    });
    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &workspace,
            mrn: "MRN-MIDRUN",
            display_name: "Arthur Pendelton",
            records: std::slice::from_ref(&rec_v1),
            pulled_at: BASE_NOW_UNIX - 1200,
        },
    )
    .await;

    let appt = make_appointment(
        "appt-midrun-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Heart failure electrolyte check",
        "2026-08-27T17:30:00+00:00",
        "2026-08-27T18:00:00+00:00",
        Some("MRN-MIDRUN"),
    );
    let starts_at_unix = appt.window().starts_at_unix();
    let cal = InMemoryScheduleSource::new(vec![appt]);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 600)
        .await
        .unwrap();

    // First run selection so the briefing job can be driven step-by-step with mid-run chart arrivals:
    let future_prepare_at = BASE_NOW_UNIX + 60;
    let mut sel_provider = ScriptedProvider::new(vec![
        call_turn(
            "sel-mid",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-midrun-1",
                    "starts_at_unix": starts_at_unix,
                    "action": "schedule",
                    "prepare_at_unix": future_prepare_at,
                    "reason": "Electrolyte check ahead of spironolactone visit."
                }]
            }),
        ),
        done_turn("Scheduled."),
    ]);
    store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut sel_provider,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();

    // Provider that injects mid-run chart updates on specific turns:
    struct MidRunArrivalProvider {
        store: Store,
        workspace: String,
        rec_v1: SourceRevision,
        turn: usize,
        saw_save_refusal: bool,
        saw_finish_continue: bool,
    }

    impl engine::recipe::Provider for MidRunArrivalProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            let serialized_items = serde_json::to_string(&request.items).unwrap();
            match self.turn {
                1 => {
                    // Read v1 record, then inject v2 BEFORE turn 2 tries to save!
                    let rec_v2 = make_record(ChartDoc {
                        workspace: &self.workspace,
                        source_id: CHART_SOURCE_ID,
                        candidate_id: "Observation/k-panel-2",
                        revision: "rev-2",
                        record_type: "Observation",
                        title: "Same-Day Chemistry",
                        date: "2026-08-27",
                        text: "Serum potassium 5.6 mEq/L, creatinine 1.4 mg/dL collected this morning.",
                    });
                    self.store
                        .record_chart(
                            &self.workspace,
                            &[self.rec_v1.clone(), rec_v2],
                            &SourcePull {
                                source_id: CHART_SOURCE_ID.into(),
                                source_label: CHART_SOURCE_LABEL.into(),
                                patient: source_patient("MRN-MIDRUN", "Arthur Pendelton"),
                                pulled_at: BASE_NOW_UNIX + 70,
                            },
                        )
                        .await
                        .unwrap();
                    Ok(
                        call_turn(
                            "read-v1",
                            READ_TOOL,
                            json!({
                                "references": [{ "candidate_id": "Observation/k-panel-1", "revision": "rev-1", "cursor": null }],
                                "max_result_bytes": 16_384
                            }),
                        )
                        .unwrap(),
                    )
                }
                2 => {
                    // Try saving stale v1 briefing without reading `Observation/k-panel-2` -> must be refused!
                    let stale = SaveBriefingArgs {
                        visit_card_orientation: "Potassium 4.3 mEq/L stable on spironolactone."
                            .into(),
                        trajectory: vec![BriefingClaimInput {
                            text: "Potassium 4.3 mEq/L on 2026-08-15.".into(),
                            uncertainty: None,
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-1".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        meaningful_changes: vec![BriefingClaimInput {
                            text: "No interval electrolyte abnormality through 2026-08-15.".into(),
                            uncertainty: None,
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-1".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        unresolved_issues: vec![BriefingClaimInput {
                            text: "Routine monitoring on spironolactone.".into(),
                            uncertainty: None,
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-1".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        visit_decisions: vec![BriefingDecisionInput {
                            decision: "Spironolactone maintenance review".into(),
                            clinical_rationale: "Potassium was 4.3 mEq/L two weeks ago.".into(),
                            options_or_questions: vec![
                                "Consider continuing spironolactone 25 mg daily.".into(),
                            ],
                            uncertainty: None,
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-1".into(),
                                revision: "rev-1".into(),
                            }],
                        }],
                        basis_and_limitations: "Through 2026-08-15.".into(),
                        basis_citations: Vec::new(),
                        limits: Vec::new(),
                        visit_concern: None,
                    };
                    Ok(call_turn(
                        "save-stale",
                        SAVE_BRIEFING_TOOL,
                        serde_json::to_value(&stale).unwrap(),
                    )
                    .unwrap())
                }
                3 => {
                    // Verify `save_visit_briefing` refused stale completion and named `Observation/k-panel-2`!
                    assert!(
                        serialized_items.contains("chart_changed_mid_run")
                            && serialized_items.contains("Observation/k-panel-2"),
                        "expected mid-run refusal with changed record, got {serialized_items}"
                    );
                    self.saw_save_refusal = true;
                    Ok(
                        call_turn(
                            "read-v2",
                            READ_TOOL,
                            json!({
                                "references": [{ "candidate_id": "Observation/k-panel-2", "revision": "rev-2", "cursor": null }],
                                "max_result_bytes": 16_384
                            }),
                        )
                        .unwrap(),
                    )
                }
                4 => {
                    // Save updated v2 briefing!
                    let v2_args = SaveBriefingArgs {
                        visit_card_orientation: "Hyperkalemia K 5.6 mEq/L and Cr 1.4 mg/dL today on spironolactone 25 mg.".into(),
                        trajectory: vec![BriefingClaimInput {
                            text: "Potassium rose from 4.3 mEq/L (2026-08-15) to 5.6 mEq/L today with creatinine 1.1 -> 1.4 mg/dL.".into(),
                            uncertainty: None,
                            citations: vec![
                                ExactRecordReference {
                                    candidate_id: "Observation/k-panel-1".into(),
                                    revision: "rev-1".into(),
                                },
                                ExactRecordReference {
                                    candidate_id: "Observation/k-panel-2".into(),
                                    revision: "rev-2".into(),
                                },
                            ],
                        }],
                        meaningful_changes: vec![BriefingClaimInput {
                            text: "Same-day potassium 5.6 mEq/L and creatinine 1.4 mg/dL.".into(),
                            uncertainty: None,
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-2".into(),
                                revision: "rev-2".into(),
                            }],
                        }],
                        unresolved_issues: vec![BriefingClaimInput {
                            text: "Specimen hemolysis comment has not yet posted.".into(),
                            uncertainty: Some("True hyperkalemia vs pseudohyperkalemia pending lab comment.".into()),
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-2".into(),
                                revision: "rev-2".into(),
                            }],
                        }],
                        visit_decisions: vec![BriefingDecisionInput {
                            decision: "Spironolactone hold or dose reduction for K 5.6 mEq/L".into(),
                            clinical_rationale: "Potassium > 5.5 mEq/L with rising creatinine warrants holding or halving MRA.".into(),
                            options_or_questions: vec!["Consider holding or halving spironolactone and repeating BMP in 3-5 days.".into()],
                            uncertainty: None,
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-2".into(),
                                revision: "rev-2".into(),
                            }],
                        }],
                        basis_and_limitations: "Includes same-day chemistry v2.".into(),
                        basis_citations: Vec::new(),
                        limits: Vec::new(),
                        visit_concern: None,
                    };
                    Ok(call_turn(
                        "save-v2",
                        SAVE_BRIEFING_TOOL,
                        serde_json::to_value(&v2_args).unwrap(),
                    )
                    .unwrap())
                }
                5 => {
                    // Inject v3 AFTER save-v2 succeeded, right before assistant tries to finish!
                    let rec_v2 = make_record(ChartDoc {
                        workspace: &self.workspace,
                        source_id: CHART_SOURCE_ID,
                        candidate_id: "Observation/k-panel-2",
                        revision: "rev-2",
                        record_type: "Observation",
                        title: "Same-Day Chemistry",
                        date: "2026-08-27",
                        text: "Serum potassium 5.6 mEq/L, creatinine 1.4 mg/dL collected this morning.",
                    });
                    let rec_v3 = make_record(ChartDoc {
                        workspace: &self.workspace,
                        source_id: CHART_SOURCE_ID,
                        candidate_id: "DiagnosticReport/ecg-1",
                        revision: "rev-3",
                        record_type: "DiagnosticReport",
                        title: "12-Lead ECG",
                        date: "2026-08-27",
                        text: "12-lead ECG normal sinus rhythm without peaked T waves or QRS widening.",
                    });
                    self.store
                        .record_chart(
                            &self.workspace,
                            &[self.rec_v1.clone(), rec_v2, rec_v3],
                            &SourcePull {
                                source_id: CHART_SOURCE_ID.into(),
                                source_label: CHART_SOURCE_LABEL.into(),
                                patient: source_patient("MRN-MIDRUN", "Arthur Pendelton"),
                                pulled_at: BASE_NOW_UNIX + 80,
                            },
                        )
                        .await
                        .unwrap();
                    Ok(done_turn("Attempting to finish at v2.").unwrap())
                }
                6 => {
                    // `finish_visit_preparation_gate` caught v3 and continued the run!
                    assert!(
                        serialized_items.contains("DiagnosticReport/ecg-1")
                            && serialized_items.contains("chart_changed_mid_run")
                            && serialized_items.contains("\\\"current_source_version\\\":3"),
                        "expected finish gate to return v3 record, got {serialized_items}"
                    );
                    self.saw_finish_continue = true;
                    Ok(
                        call_turn(
                            "read-v3",
                            READ_TOOL,
                            json!({
                                "references": [{ "candidate_id": "DiagnosticReport/ecg-1", "revision": "rev-3", "cursor": null }],
                                "max_result_bytes": 16_384
                            }),
                        )
                        .unwrap(),
                    )
                }
                7 => {
                    let v3_args = SaveBriefingArgs {
                        visit_card_orientation: "Hyperkalemia K 5.6 mEq/L (Cr 1.4) on spironolactone; same-day ECG shows no hyperkalemic changes.".into(),
                        trajectory: vec![BriefingClaimInput {
                            text: "Potassium rose from 4.3 to 5.6 mEq/L with Cr 1.4 mg/dL; same-day ECG has no peaked T waves.".into(),
                            uncertainty: None,
                            citations: vec![
                                ExactRecordReference {
                                    candidate_id: "Observation/k-panel-2".into(),
                                    revision: "rev-2".into(),
                                },
                                ExactRecordReference {
                                    candidate_id: "DiagnosticReport/ecg-1".into(),
                                    revision: "rev-3".into(),
                                },
                            ],
                        }],
                        meaningful_changes: vec![BriefingClaimInput {
                            text: "Same-day potassium 5.6 mEq/L without ECG conduction changes.".into(),
                            uncertainty: None,
                            citations: vec![
                                ExactRecordReference {
                                    candidate_id: "Observation/k-panel-2".into(),
                                    revision: "rev-2".into(),
                                },
                                ExactRecordReference {
                                    candidate_id: "DiagnosticReport/ecg-1".into(),
                                    revision: "rev-3".into(),
                                },
                            ],
                        }],
                        unresolved_issues: vec![BriefingClaimInput {
                            text: "Repeat chemistry after MRA adjustment will be needed within 3-5 days.".into(),
                            uncertainty: None,
                            citations: vec![ExactRecordReference {
                                candidate_id: "Observation/k-panel-2".into(),
                                revision: "rev-2".into(),
                            }],
                        }],
                        visit_decisions: vec![BriefingDecisionInput {
                            decision: "Outpatient spironolactone hold/reduction and potassium-lowering plan".into(),
                            clinical_rationale: "Absence of ECG changes supports outpatient MRA hold/dose reduction with close repeat BMP.".into(),
                            options_or_questions: vec![
                                "Consider holding spironolactone 25 mg or reducing to 12.5 mg with dietary potassium review and repeat BMP.".into(),
                            ],
                            uncertainty: None,
                            citations: vec![
                                ExactRecordReference {
                                    candidate_id: "Observation/k-panel-2".into(),
                                    revision: "rev-2".into(),
                                },
                                ExactRecordReference {
                                    candidate_id: "DiagnosticReport/ecg-1".into(),
                                    revision: "rev-3".into(),
                                },
                            ],
                        }],
                        basis_and_limitations: "Prepared from chart version 3 including same-day BMP and 12-lead ECG.".into(),
                        basis_citations: Vec::new(),
                        limits: Vec::new(),
                        visit_concern: None,
                    };
                    Ok(call_turn(
                        "save-v3",
                        SAVE_BRIEFING_TOOL,
                        serde_json::to_value(&v3_args).unwrap(),
                    )
                    .unwrap())
                }
                8 => Ok(done_turn("Completed at chart version 3.").unwrap()),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut mid_provider = MidRunArrivalProvider {
        store: store.clone(),
        workspace: workspace.clone(),
        rec_v1,
        turn: 0,
        saw_save_refusal: false,
        saw_finish_continue: false,
    };

    let wake = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut mid_provider,
            future_prepare_at,
        )
        .await
        .unwrap();
    assert!(mid_provider.saw_save_refusal);
    assert!(mid_provider.saw_finish_continue);
    assert_eq!(wake.completed_briefings.len(), 1);

    let saved = store
        .visit_briefing(
            &VisitSlotKey {
                practice: practice.clone(),
                source_id: CALENDAR_SOURCE_ID.into(),
                appointment_id: "appt-midrun-1".into(),
                starts_at_unix,
            },
            auth.as_ref(),
        )
        .await
        .unwrap()
        .expect("saved v3 briefing");
    assert_eq!(saved.source_version, 3);
    assert!(saved.visit_card_orientation.contains("same-day ECG"));
}

/// Cancelling an appointment cancels unfinished preparation work (`Cancelled` /
/// `DeliveryState::Settled` / `EndedVisitChanged`), while moving an appointment
/// with a completed briefing retains the old briefing as dated history (`Moved`)
/// and prepares a new briefing keyed to the new visit time (`Current`).
#[tokio::test]
async fn ac3_cancelled_or_moved_visit_ends_unfinished_work_and_retains_completed_briefing_as_history()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-vp-move-{}", Uuid::new_v4().simple());
    let ws_cancel = format!("pw-cancel-{}", Uuid::new_v4().simple());
    let ws_move = format!("pw-move-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws_cancel,
            mrn: "MRN-CANCEL",
            display_name: "Liam Brooks",
            records: &[make_record(ChartDoc {
                workspace: &ws_cancel,
                source_id: CHART_SOURCE_ID,
                candidate_id: "Condition/htn-1",
                revision: "rev-1",
                record_type: "Condition",
                title: "Hypertension",
                date: "2026-08-01",
                text: "Essential hypertension on lisinopril 10 mg daily; home BP averaging 138/86.",
            })],
            pulled_at: BASE_NOW_UNIX - 1800,
        },
    )
    .await;

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws_move,
            mrn: "MRN-MOVE",
            display_name: "Sofia Alvarez",
            records: &[make_record(ChartDoc {
                workspace: &ws_move,
                source_id: CHART_SOURCE_ID,
                candidate_id: "Observation/asthma-act-1",
                revision: "rev-1",
                record_type: "Observation",
                title: "Asthma Control Note",
                date: "2026-08-18",
                text: "Moderate persistent asthma; nocturnal albuterol use 3 nights/week despite low-dose ICS.",
            })],
            pulled_at: BASE_NOW_UNIX - 1800,
        },
    )
    .await;

    let appt_cancel = make_appointment(
        "appt-cancel-1",
        AppointmentStatus::Scheduled,
        "booked",
        "BP medication check",
        "2026-08-27T17:00:00+00:00",
        "2026-08-27T17:30:00+00:00",
        Some("MRN-CANCEL"),
    );
    let appt_move_old = make_appointment(
        "appt-move-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Asthma step-up review",
        "2026-08-27T18:00:00+00:00",
        "2026-08-27T18:30:00+00:00",
        Some("MRN-MOVE"),
    );
    let cancel_start = appt_cancel.window().starts_at_unix();
    let old_move_start = appt_move_old.window().starts_at_unix();

    let cal = InMemoryScheduleSource::new(vec![appt_cancel, appt_move_old]);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 600)
        .await
        .unwrap();

    let move_briefing_args = |orientation: &str| {
        SaveBriefingArgs {
        visit_card_orientation: orientation.to_owned(),
        trajectory: vec![BriefingClaimInput {
            text: "Moderate persistent asthma with nocturnal rescue use 3 nights/week.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/asthma-act-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Nocturnal albuterol use increased to 3 nights/week on low-dose ICS.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/asthma-act-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Inhaler spacer technique and environmental triggers not yet reviewed.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/asthma-act-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Controller step-up to MART (ICS-formoterol) vs medium-dose ICS".into(),
            clinical_rationale: "Nocturnal symptoms >= 2 nights/week indicate inadequate asthma control.".into(),
            options_or_questions: vec![
                "Check inhaler adherence/technique and discuss stepping up to ICS-formoterol maintenance-and-reliever therapy.".into(),
            ],
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/asthma-act-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        basis_and_limitations: "Prepared from asthma control note (2026-08-18).".into(),
        basis_citations: Vec::new(),
        limits: Vec::new(),
        visit_concern: None,
    }
    };

    // Wake 1:
    // - `appt-cancel-1`: model selects `prepare_now`, then its briefing job hits 1 transient error (so it stays unfinished `Preparing` / `Pending`!)
    // - `appt-move-1`: model selects `prepare_now` and completes its briefing at `18:00Z`!
    let mut wake1_provider = ScriptedProvider::new(vec![
        call_turn(
            "sel-cm",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-cancel-1",
                        "starts_at_unix": cancel_start,
                        "action": "schedule",
                        "prepare_at_unix": BASE_NOW_UNIX + 120,
                        "reason": "Prepare BP medication review shortly."
                    },
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-move-1",
                        "starts_at_unix": old_move_start,
                        "action": "prepare_now",
                        "reason": "Uncontrolled nocturnal asthma ahead of 18:00 visit."
                    }
                ]
            }),
        ),
        done_turn("Selected."),
        call_turn(
            "read-move-old",
            READ_TOOL,
            json!({
                "references": [{ "candidate_id": "Observation/asthma-act-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384
            }),
        ),
        call_turn(
            "save-move-old",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(move_briefing_args(
                "Asthma uncontrolled (3 nights/wk rescue); MART step-up option for 18:00 visit.",
            ))
            .unwrap(),
        ),
        done_turn("Saved 18:00 briefing."),
    ]);

    store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut wake1_provider,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();

    // Now start `appt-cancel-1`'s briefing job and leave it Pending/unfinished by cancelling mid-flight:
    let cancel_briefing_job_id = format!(
        "{}-sv1",
        local_product::visit_preparation::briefing_job_id(
            CALENDAR_SOURCE_ID,
            "appt-cancel-1",
            cancel_start
        )
    );
    let cancel_job_scope = PracticeScope {
        practice: practice.clone(),
        workspace: ws_cancel.clone(),
        job_id: cancel_briefing_job_id.clone(),
    };
    // Trigger queueing of `appt-cancel-1` via a single-turn provider that interrupts once:
    // Actually, let's check: if `drive_practice_job_to_settlement` loops on failure until Cancelled/Failed/Held/Completed,
    // how is an in-flight job left Pending when schedule sync arrives?
    // When `sync_practice_schedule` runs concurrently (or while a job is Pending/Active)!
    // Let's accept the briefing job via `accept_practice_job` or trigger schedule sync WHILE the provider runs!
    struct CancelMidBriefingProvider {
        store: Store,
        auth: Arc<PostgresAuthStore>,
        practice: String,
        cal: Arc<InMemoryScheduleSource>,
        new_move_appt: SourceAppointment,
    }
    impl engine::recipe::Provider for CancelMidBriefingProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            // While `appt-cancel-1` briefing job is actively leased and running:
            // Sync updated schedule cancelling `appt-cancel-1` AND moving `appt-move-1` to 20:00Z!
            let cancelled_appt = make_appointment(
                "appt-cancel-1",
                AppointmentStatus::Cancelled,
                "cancelled",
                "BP medication check",
                "2026-08-27T17:00:00+00:00",
                "2026-08-27T17:30:00+00:00",
                Some("MRN-CANCEL"),
            );
            self.cal
                .set_appointments(vec![cancelled_appt, self.new_move_appt.clone()]);
            self.store
                .sync_practice_schedule(
                    self.auth.as_ref(),
                    &self.practice,
                    self.cal.as_ref(),
                    BASE_NOW_UNIX + 180,
                )
                .await
                .unwrap();
            Ok(done_turn("Attempt finishes after cancellation").unwrap())
        }
    }

    let appt_move_new = make_appointment(
        "appt-move-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Asthma step-up review",
        "2026-08-27T20:00:00+00:00",
        "2026-08-27T20:30:00+00:00",
        Some("MRN-MOVE"),
    );
    let new_move_start = appt_move_new.window().starts_at_unix();
    let shared_cal = Arc::new(InMemoryScheduleSource::new(vec![
        make_appointment(
            "appt-cancel-1",
            AppointmentStatus::Scheduled,
            "booked",
            "BP medication check",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-CANCEL"),
        ),
        make_appointment(
            "appt-move-1",
            AppointmentStatus::Scheduled,
            "booked",
            "Asthma step-up review",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            Some("MRN-MOVE"),
        ),
    ]));

    let mut cancel_mid_provider = CancelMidBriefingProvider {
        store: store.clone(),
        auth: auth.clone(),
        practice: practice.clone(),
        cal: shared_cal.clone(),
        new_move_appt: appt_move_new,
    };
    store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut cancel_mid_provider,
            BASE_NOW_UNIX + 150,
        )
        .await
        .unwrap();

    // 1. Unfinished job for `appt-cancel-1` was cancelled (`WorkStatus::Cancelled`, `DeliveryState::Settled`, `EndedVisitChanged`) and wrote NO briefing!
    let cancelled_job = store
        .practice_job(&cancel_job_scope)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(cancelled_job.status, WorkStatus::Cancelled);
    let delivery = Delivery::practice_job(&cancel_job_scope, "job-key".into());
    assert_eq!(
        store.delivery_state(&delivery).await.unwrap(),
        DeliveryState::Settled
    );
    assert!(
        store
            .visit_briefing(
                &VisitSlotKey {
                    practice: practice.clone(),
                    source_id: CALENDAR_SOURCE_ID.into(),
                    appointment_id: "appt-cancel-1".into(),
                    starts_at_unix: cancel_start,
                },
                auth.as_ref(),
            )
            .await
            .unwrap()
            .is_none()
    );
    let preps_after_cancel = store
        .visit_preparations_for_practice(&practice)
        .await
        .unwrap();
    let cancelled_prep = preps_after_cancel
        .iter()
        .find(|p| p.slot.appointment_id == "appt-cancel-1")
        .unwrap();
    assert!(matches!(
        cancelled_prep.state,
        VisitPreparationState::EndedVisitChanged { .. }
    ));

    // 2. Completed briefing for `appt-move-1` at 18:00Z remains intact as dated history (`VisitStanding::Moved`)!
    let old_briefing = store
        .visit_briefing(
            &VisitSlotKey {
                practice: practice.clone(),
                source_id: CALENDAR_SOURCE_ID.into(),
                appointment_id: "appt-move-1".into(),
                starts_at_unix: old_move_start,
            },
            auth.as_ref(),
        )
        .await
        .unwrap()
        .expect("completed 18:00 briefing retained as dated history");
    assert_eq!(
        old_briefing.visit_standing,
        VisitStanding::Moved {
            new_starts_at: "2026-08-27T20:00:00.000Z".into(),
            new_starts_at_unix: new_move_start,
            noted_at: BASE_NOW_UNIX + 180,
        }
    );

    // 3. Subsequent wake-up selects and prepares a fresh briefing for the rescheduled 20:00Z visit time,
    //    passing `prior_visit_briefings` into the new run's starting context!
    let mut move_new_provider = ScriptedProvider::new(vec![
        call_turn(
            "sel-move-new",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-move-1",
                    "starts_at_unix": new_move_start,
                    "action": "prepare_now",
                    "reason": "Rescheduled 20:00 asthma visit needs briefing bound to the new visit slot."
                }]
            }),
        ),
        done_turn("Selected rescheduled slot."),
        call_turn(
            "read-move-new",
            READ_TOOL,
            json!({
                "references": [{ "candidate_id": "Observation/asthma-act-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384
            }),
        ),
        call_turn(
            "save-move-new",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(move_briefing_args(
                "Asthma uncontrolled (3 nights/wk rescue); prepared for rescheduled 20:00 visit.",
            ))
            .unwrap(),
        ),
        done_turn("Saved 20:00 briefing."),
    ]);

    store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut move_new_provider,
            BASE_NOW_UNIX + 240,
        )
        .await
        .unwrap();

    // Check that `prior_visit_briefings` was supplied to the 20:00Z briefing run:
    let new_briefing_prompt = serde_json::to_string(&move_new_provider.requests[2].items).unwrap();
    assert!(new_briefing_prompt.contains(&old_briefing.briefing_id));

    let all_for_appt = store
        .visit_briefings_for_appointment(
            &practice,
            CALENDAR_SOURCE_ID,
            "appt-move-1",
            auth.as_ref(),
        )
        .await
        .unwrap();
    assert_eq!(all_for_appt.len(), 2);
    assert!(matches!(
        all_for_appt[0].visit_standing,
        VisitStanding::Moved { .. }
    ));
    assert_eq!(all_for_appt[1].slot.starts_at_unix, new_move_start);
    assert_eq!(all_for_appt[1].visit_standing, VisitStanding::Current);
}

/// Failed external source refresh during preparation preserves available chart
/// records, records truthful `source_freshness` (`last_good_pulled_at` +
/// `last_failed_at` / `failure_reason`) and `limits` (`Unavailable`), keeps
/// conflicting/missing evidence uncertainty beside the claim, and refuses
/// `visit_decisions` phrased as already accepted or ordered care.
#[tokio::test]
async fn ac3_failed_refresh_missing_and_conflicting_evidence_preserve_records_and_refuse_accepted_care()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-vp-refresh-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-vp-refresh-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    // Seed two sources: local clinic EHR + external cardiology system with an older pull:
    let clinic_records = vec![
        make_record(ChartDoc {
            workspace: &workspace,
            source_id: CHART_SOURCE_ID,
            candidate_id: "Observation/clinic-bp-1",
            revision: "rev-bp-1",
            record_type: "Observation",
            title: "Clinic Blood Pressure",
            date: "2026-08-20",
            text: "Seated clinic BP 162/96 mmHg on lisinopril 10 mg daily.",
        }),
        make_record(ChartDoc {
            workspace: &workspace,
            source_id: CHART_SOURCE_ID,
            candidate_id: "DocumentReference/home-bp-msg-1",
            revision: "rev-home-1",
            record_type: "DocumentReference",
            title: "Patient Home BP Log Message",
            date: "2026-08-25",
            text: "Patient uploaded home arm-cuff log showing morning readings 122/76 to 128/80 mmHg over 7 days; outside Mercy Cardiology echo done last month not yet received.",
        }),
    ];
    seed_roster_patient(
        &base_store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &workspace,
            mrn: "MRN-REFRESH",
            display_name: "Clara Sato",
            records: &clinic_records,
            pulled_at: BASE_NOW_UNIX - 7200,
        },
    )
    .await;

    base_store
        .record_chart(
            &workspace,
            &[make_record(ChartDoc {
                workspace: &workspace,
                source_id: "mercy-cardiology",
                candidate_id: "DocumentReference/cards-consult-2025",
                revision: "rev-cards-1",
                record_type: "DocumentReference",
                title: "Mercy Cardiology Prior Consult",
                date: "2025-11-04",
                text: "Prior cardiology consult recommending repeat transthoracic echocardiogram in summer 2026.",
            })],
            &SourcePull {
                source_id: "mercy-cardiology".into(),
                source_label: "Mercy Cardiology HIE".into(),
                patient: source_patient("MRN-REFRESH", "Clara Sato"),
                pulled_at: BASE_NOW_UNIX - 86_400 * 30,
            },
        )
        .await
        .unwrap();

    // Configure FixedSourceCheck: clinic chart unchanged, Mercy Cardiology HIE fails refresh!
    let checker = FixedSourceCheck::new();
    checker.set_source_outcome(
        &practice,
        &workspace,
        RefreshSourceId::new(CHART_SOURCE_ID).unwrap(),
        SourceCheckOutcome::Updated {
            source_label: CHART_SOURCE_LABEL.into(),
            patient: source_patient("MRN-REFRESH", "Clara Sato"),
            pulled_at: BASE_NOW_UNIX,
            records: Vec::new(),
        },
    );
    checker.set_source_outcome(
        &practice,
        &workspace,
        RefreshSourceId::new("mercy-cardiology").unwrap(),
        SourceCheckOutcome::Failed {
            source_label: Some("Mercy Cardiology HIE".into()),
            checked_at: BASE_NOW_UNIX,
            reason: "upstream HIE gateway returned HTTP 503 during pre-visit check".into(),
        },
    );
    let store = base_store.with_source_refresh(Arc::new(checker));

    let appt = make_appointment(
        "appt-refresh-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Hypertension & cardiology follow-up",
        "2026-08-27T16:30:00+00:00",
        "2026-08-27T17:00:00+00:00",
        Some("MRN-REFRESH"),
    );
    let starts_at_unix = appt.window().starts_at_unix();
    let cal = InMemoryScheduleSource::new(vec![appt]);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 300)
        .await
        .unwrap();

    // First attempt at `save_visit_briefing` uses accepted-care phrasing ("has been ordered") -> refused!
    let bad_accepted_care_args = json!({
        "visit_card_orientation": "Discordant clinic (162/96) vs home BP (122-128/76-80); Mercy Cardiology HIE refresh failed.",
        "trajectory": [{
            "text": "Hypertension treated with lisinopril 10 mg daily with discordant clinic and home blood pressure readings.",
            "citations": [
                { "candidate_id": "Observation/clinic-bp-1", "revision": "rev-bp-1" },
                { "candidate_id": "DocumentReference/home-bp-msg-1", "revision": "rev-home-1" }
            ]
        }],
        "meaningful_changes": [{
            "text": "Clinic BP 162/96 mmHg on 2026-08-20 conflicts with 7-day home cuff log 122-128/76-80 mmHg uploaded 2026-08-25.",
            "uncertainty": "White-coat effect vs uncalibrated home cuff cannot be distinguished without in-office cuff comparison or 24-hour ABPM.",
            "citations": [
                { "candidate_id": "Observation/clinic-bp-1", "revision": "rev-bp-1" },
                { "candidate_id": "DocumentReference/home-bp-msg-1", "revision": "rev-home-1" }
            ]
        }],
        "unresolved_issues": [{
            "text": "Summer 2026 Mercy Cardiology echocardiogram could not be retrieved because Mercy Cardiology HIE refresh failed.",
            "uncertainty": "Current LV hypertrophy or systolic function from last month's outside echo is unavailable.",
            "citations": [{ "candidate_id": "DocumentReference/cards-consult-2025", "revision": "rev-cards-1" }]
        }],
        "visit_decisions": [{
            "decision": "Blood pressure medication titration",
            "clinical_rationale": "Clinic BP was elevated.",
            "options_or_questions": ["Ambulatory blood pressure monitoring has been ordered and lisinopril was increased to 20 mg."],
            "citations": [{ "candidate_id": "Observation/clinic-bp-1", "revision": "rev-bp-1" }]
        }],
        "basis_and_limitations": "Mercy Cardiology HIE refresh failed.",
        "basis_citations": [],
        "limits": []
    });

    let valid_args = SaveBriefingArgs {
        visit_card_orientation: "Discordant clinic (162/96) vs home BP (122-128/76-80); Mercy Cardiology echo unavailable after failed HIE refresh.".into(),
        trajectory: vec![BriefingClaimInput {
            text: "Hypertension on lisinopril 10 mg daily with prior Nov 2025 cardiology consult; available records preserved through 2026-08-25.".into(),
            uncertainty: None,
            citations: vec![
                ExactRecordReference {
                    candidate_id: "Observation/clinic-bp-1".into(),
                    revision: "rev-bp-1".into(),
                },
                ExactRecordReference {
                    candidate_id: "DocumentReference/cards-consult-2025".into(),
                    revision: "rev-cards-1".into(),
                },
            ],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Clinic BP 162/96 mmHg (2026-08-20) conflicts with 7-day home cuff readings 122-128/76-80 mmHg (2026-08-25).".into(),
            uncertainty: Some("White-coat hypertension vs home cuff calibration error requires cuff validation before escalating antihypertensive dose.".into()),
            citations: vec![
                ExactRecordReference {
                    candidate_id: "Observation/clinic-bp-1".into(),
                    revision: "rev-bp-1".into(),
                },
                ExactRecordReference {
                    candidate_id: "DocumentReference/home-bp-msg-1".into(),
                    revision: "rev-home-1".into(),
                },
            ],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Mercy Cardiology HIE refresh failed (HTTP 503), leaving last month's outside echocardiogram unavailable; only the Nov 2025 consult is on file.".into(),
            uncertainty: Some("LV structure and ejection fraction from summer 2026 echo remain unknown.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "DocumentReference/cards-consult-2025".into(),
                revision: "rev-cards-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Reconcile discordant clinic vs home blood pressures before lisinopril dose change".into(),
            clinical_rationale: "Escalating lisinopril solely on 162/96 clinic BP risks symptomatic hypotension if home readings (122-128/76-80) are accurate.".into(),
            options_or_questions: vec![
                "Repeat rested automated office BP and compare patient's home cuff in clinic before deciding whether to hold lisinopril at 10 mg or titrate.".into(),
                "Consider 24-hour ambulatory blood pressure monitoring and requesting outside Mercy Cardiology echo records.".into(),
            ],
            uncertainty: Some("Outside summer 2026 echocardiogram is unavailable due to Mercy Cardiology HIE gateway failure.".into()),
            citations: vec![
                ExactRecordReference {
                    candidate_id: "Observation/clinic-bp-1".into(),
                    revision: "rev-bp-1".into(),
                },
                ExactRecordReference {
                    candidate_id: "DocumentReference/home-bp-msg-1".into(),
                    revision: "rev-home-1".into(),
                },
            ],
        }],
        basis_and_limitations: "Prepared from preserved clinic chart through 2026-08-25 and Mercy Cardiology records last pulled 30 days ago; live Mercy Cardiology HIE refresh failed (HTTP 503).".into(),
        basis_citations: Vec::new(),
        limits: Vec::new(),
        visit_concern: None,
    };

    let mut provider = ScriptedProvider::new(vec![
        call_turn(
            "sel-ref",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-refresh-1",
                    "starts_at_unix": starts_at_unix,
                    "action": "prepare_now",
                    "reason": "Discordant BP readings and pending outside cardiology echo ahead of today's visit."
                }]
            }),
        ),
        done_turn("Selected."),
        // Refresh all connected sources inside the briefing run:
        call_turn(
            "refresh-all",
            local_product::runtime::refresh::REFRESH_TOOL,
            json!({}),
        ),
        call_turn(
            "read-preserved",
            READ_TOOL,
            json!({
                "references": [
                    { "candidate_id": "Observation/clinic-bp-1", "revision": "rev-bp-1", "cursor": null },
                    { "candidate_id": "DocumentReference/home-bp-msg-1", "revision": "rev-home-1", "cursor": null },
                    { "candidate_id": "DocumentReference/cards-consult-2025", "revision": "rev-cards-1", "cursor": null }
                ],
                "max_result_bytes": 32_768
            }),
        ),
        // Attempt accepted-care phrasing -> refused by host!
        call_turn(
            "save-bad-phrasing",
            SAVE_BRIEFING_TOOL,
            bad_accepted_care_args,
        ),
        // Rephrase as physician options -> accepted!
        call_turn(
            "save-valid-phrasing",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&valid_args).unwrap(),
        ),
        done_turn("Saved with truthful freshness and physician-framed options."),
    ]);

    let summary = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut provider,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();
    assert_eq!(summary.completed_briefings.len(), 1);

    // Verify accepted-care phrasing was refused before the corrected save succeeded:
    let turn_after_refusal = serde_json::to_string(&provider.requests[5].items).unwrap();
    assert!(
        turn_after_refusal.contains("has been ordered")
            && turn_after_refusal.contains("options or open questions for the physician"),
        "expected accepted-care refusal feedback in model history: {turn_after_refusal}"
    );

    // Verify available records remain intact and saved briefing carries real freshness + Unavailable limit:
    let obs = store.sources(&workspace).await.unwrap();
    assert_eq!(obs.sources.len(), 3);

    let saved = store
        .visit_briefing(
            &VisitSlotKey {
                practice: practice.clone(),
                source_id: CALENDAR_SOURCE_ID.into(),
                appointment_id: "appt-refresh-1".into(),
                starts_at_unix,
            },
            auth.as_ref(),
        )
        .await
        .unwrap()
        .expect("saved briefing after failed external refresh");

    let mercy_freshness = saved
        .source_freshness
        .iter()
        .find(|s| s.source_id == "mercy-cardiology")
        .expect("mercy-cardiology freshness recorded");
    assert_eq!(
        mercy_freshness.last_good_pulled_at,
        Some(BASE_NOW_UNIX - 86_400 * 30)
    );
    assert!(mercy_freshness.last_failed_at.is_some());
    assert!(
        mercy_freshness
            .failure_reason
            .as_deref()
            .unwrap()
            .contains("HTTP 503")
    );
    assert!(saved.limits.iter().any(|l| {
        l.kind == RecordLimitKind::Unavailable
            && l.subject == "mercy-cardiology"
            && l.detail.contains("HTTP 503")
    }));

    // Also verify unused `RefreshCommandKey` / `RefreshRequest` / `RefreshTarget` compile cleanly if called directly:
    let direct_req = RefreshRequest::new(
        RefreshCommandKey::new("post-briefing-check").unwrap(),
        RefreshTarget::AllSources,
    );
    let _ = direct_req;
}

/// Ended visits (`now_unix >= ends_at_unix`) never start preparation; in-progress
/// visits (`starts_at_unix <= now_unix < ends_at_unix`) prepare cleanly; and work
/// that exhausts its 5-attempt retry allowance transitions to `CouldNotFinish`.
#[tokio::test]
async fn ac4_no_preparation_starts_after_visit_ends_and_exhausted_retries_show_could_not_finish() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    assert!(!visit_can_start_preparation(
        AppointmentStatus::Scheduled,
        BASE_NOW_UNIX,
        BASE_NOW_UNIX
    ));
    assert!(visit_can_start_preparation(
        AppointmentStatus::Scheduled,
        BASE_NOW_UNIX + 1,
        BASE_NOW_UNIX
    ));
    assert!(!visit_can_start_preparation(
        AppointmentStatus::Cancelled,
        BASE_NOW_UNIX + 3600,
        BASE_NOW_UNIX
    ));

    let practice = format!("practice-vp-ac4-{}", Uuid::new_v4().simple());
    let ws_ended = format!("pw-ended-{}", Uuid::new_v4().simple());
    let ws_inprog = format!("pw-inprog-{}", Uuid::new_v4().simple());
    let ws_fail = format!("pw-fail-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    for (ws, mrn, name) in [
        (&ws_ended, "MRN-ENDED", "Ended Patient"),
        (&ws_inprog, "MRN-INPROG", "InProgress Patient"),
        (&ws_fail, "MRN-FAIL", "Failing Patient"),
    ] {
        seed_roster_patient(
            &store,
            &auth,
            RosterPatient {
                practice: &practice,
                workspace: ws,
                mrn,
                display_name: name,
                records: &[make_record(ChartDoc {
                    workspace: ws,
                    source_id: CHART_SOURCE_ID,
                    candidate_id: "Observation/vitals-1",
                    revision: "rev-1",
                    record_type: "Observation",
                    title: "Vitals",
                    date: "2026-08-27",
                    text: "Blood pressure 144/88 mmHg, heart rate 76 bpm.",
                })],
                pulled_at: BASE_NOW_UNIX - 1800,
            },
        )
        .await;
    }

    // BASE_NOW_UNIX = 2026-08-27T14:00:00Z
    let ended_appt = make_appointment(
        "appt-ended-1",
        AppointmentStatus::Scheduled,
        "fulfilled",
        "Morning visit already ended",
        "2026-08-27T13:30:00+00:00",
        "2026-08-27T14:00:00+00:00", // ends_at_unix == BASE_NOW_UNIX!
        Some("MRN-ENDED"),
    );
    let inprog_appt = make_appointment(
        "appt-inprog-1",
        AppointmentStatus::Scheduled,
        "arrived",
        "Currently in-progress visit",
        "2026-08-27T13:50:00+00:00",
        "2026-08-27T14:20:00+00:00", // starts_at < now < ends_at!
        Some("MRN-INPROG"),
    );
    let fail_appt = make_appointment(
        "appt-fail-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Afternoon visit that fails provider",
        "2026-08-27T16:00:00+00:00",
        "2026-08-27T16:30:00+00:00",
        Some("MRN-FAIL"),
    );
    let ended_start = ended_appt.window().starts_at_unix();
    let inprog_start = inprog_appt.window().starts_at_unix();
    let fail_start = fail_appt.window().starts_at_unix();

    let cal = InMemoryScheduleSource::new(vec![ended_appt, inprog_appt, fail_appt]);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 60)
        .await
        .unwrap();

    let inprog_briefing = SaveBriefingArgs {
        visit_card_orientation: "In-progress BP check (144/88 mmHg); medication review open."
            .into(),
        trajectory: vec![BriefingClaimInput {
            text: "Blood pressure 144/88 mmHg recorded today.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/vitals-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Today's seated clinic BP is mildly elevated at 144/88 mmHg.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/vitals-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Home BP readings are not yet documented in the chart.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/vitals-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Repeat rested BP and home monitoring review".into(),
            clinical_rationale: "Single clinic reading 144/88 mmHg warrants repeat rested check."
                .into(),
            options_or_questions: vec![
                "Repeat BP after 5 minutes of quiet rest and review home log.".into(),
            ],
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/vitals-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        basis_and_limitations: "Prepared during active visit from today's vitals.".into(),
        basis_citations: Vec::new(),
        limits: Vec::new(),
        visit_concern: None,
    };

    // Note: if model tries to include `appt-ended-1` in `select_visit_preparations`, the host refuses it!
    // We prove both: first try selecting `appt-ended-1` (refused), then select `appt-inprog-1` + `appt-fail-1`!
    let mut provider = ScriptedProvider::new(vec![
        call_turn(
            "sel-ended-refused",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-ended-1",
                    "starts_at_unix": ended_start,
                    "action": "prepare_now",
                    "reason": "Should be refused because visit already ended."
                }]
            }),
        ),
        call_turn(
            "sel-valid-two",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-inprog-1",
                        "starts_at_unix": inprog_start,
                        "action": "prepare_now",
                        "reason": "In-progress visit still within encounter window."
                    },
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-fail-1",
                        "starts_at_unix": fail_start,
                        "action": "prepare_now",
                        "reason": "Upcoming afternoon visit."
                    }
                ]
            }),
        ),
        done_turn("Selected valid visits."),
        // `appt-inprog-1` (13:50Z) runs and saves cleanly:
        call_turn(
            "read-inprog",
            READ_TOOL,
            json!({
                "references": [{ "candidate_id": "Observation/vitals-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384
            }),
        ),
        call_turn(
            "save-inprog",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&inprog_briefing).unwrap(),
        ),
        done_turn("In-progress briefing saved."),
        // `appt-fail-1` (16:00Z) fails all 5 automatic attempts -> settles to `Failed` / `CouldNotFinish`:
        Err("upstream model transport error 1".into()),
        Err("upstream model transport error 2".into()),
        Err("upstream model transport error 3".into()),
        Err("upstream model transport error 4".into()),
        Err("upstream model transport error 5".into()),
    ]);

    let wake = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut provider,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();

    // Verify `appt-ended-1` was excluded from candidate context AND refused when targeted:
    let sel_prompt = serde_json::to_string(&provider.requests[0].items).unwrap();
    assert!(!sel_prompt.contains("appt-ended-1"));
    let refusal_turn = serde_json::to_string(&provider.requests[1].items).unwrap();
    assert!(refusal_turn.contains("unknown_candidate_visit"));

    assert_eq!(wake.completed_briefings.len(), 1);
    assert_eq!(wake.failed_jobs.len(), 1);

    let preps = store
        .visit_preparations_for_practice(&practice)
        .await
        .unwrap();
    assert!(
        !preps
            .iter()
            .any(|p| p.slot.appointment_id == "appt-ended-1"),
        "ended visit must never start preparation"
    );
    let inprog_prep = preps
        .iter()
        .find(|p| p.slot.appointment_id == "appt-inprog-1")
        .unwrap();
    assert!(matches!(
        inprog_prep.state,
        VisitPreparationState::Ready { .. }
    ));

    let fail_prep = preps
        .iter()
        .find(|p| p.slot.appointment_id == "appt-fail-1")
        .unwrap();
    assert!(matches!(
        &fail_prep.state,
        VisitPreparationState::CouldNotFinish { reason, .. }
            if reason.contains("ended in `failed` state without saving a briefing")
    ));
}

/// Scheduled wake-up prepares a visit booked for tomorrow over the real
/// `ResponsesProvider` HTTP SSE wire contract without manual start, and an
/// interrupted run that crashed after committing `save_visit_briefing` (+ visit
/// concern) recovers cleanly on the next wake-up with ONE saved briefing and
/// ONE set of effects.
#[tokio::test]
async fn ac5_scheduled_wakeup_prepares_tomorrow_visit_over_responses_http_and_recovers_interrupted_run_once()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    // Part A: Tomorrow's visit prepared automatically over real `ResponsesProvider` HTTP SSE!
    let practice = format!("practice-vp-ac5-{}", Uuid::new_v4().simple());
    let workspace = format!("pw-tomorrow-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &workspace,
            mrn: "MRN-TOMORROW",
            display_name: "Gabriel Ortiz",
            records: &[make_record(ChartDoc {
                workspace: &workspace,
                source_id: CHART_SOURCE_ID,
                candidate_id: "Observation/inr-warfarin-1",
                revision: "rev-inr-1",
                record_type: "Observation",
                title: "Prothrombin Time / INR",
                date: "2026-08-27",
                text: "Mechanical mitral valve on warfarin; INR today 4.4 (goal 2.5-3.5), denies epistaxis or melena.",
            })],
            pulled_at: BASE_NOW_UNIX - 900,
        },
    )
    .await;

    let tomorrow_appt = make_appointment(
        "appt-tomorrow-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Anticoagulation follow-up",
        "2026-08-28T14:00:00+00:00",
        "2026-08-28T14:30:00+00:00",
        Some("MRN-TOMORROW"),
    );
    let tomorrow_start = tomorrow_appt.window().starts_at_unix();
    let cal = InMemoryScheduleSource::new(vec![tomorrow_appt]);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 300)
        .await
        .unwrap();

    let tomorrow_save = SaveBriefingArgs {
        visit_card_orientation: "Mechanical mitral valve with supratherapeutic INR 4.4 (goal 2.5-3.5) ahead of tomorrow's visit; warfarin hold/dose reduction options open.".into(),
        trajectory: vec![BriefingClaimInput {
            text: "Anticoagulated with warfarin for mechanical mitral valve (target INR 2.5-3.5); today's pre-visit INR is 4.4 without reported bleeding.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-warfarin-1".into(),
                revision: "rev-inr-1".into(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Pre-visit INR resulted supratherapeutic at 4.4 on 2026-08-27.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-warfarin-1".into(),
                revision: "rev-inr-1".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Recent dietary vitamin K changes or interacting medications have not yet been reviewed with the patient.".into(),
            uncertainty: Some("Interacting antibiotic or OTC analgesic exposure since prior INR is unconfirmed.".into()),
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-warfarin-1".into(),
                revision: "rev-inr-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Warfarin dose adjustment and repeat INR interval for mechanical mitral valve at INR 4.4".into(),
            clinical_rationale: "Supratherapeutic INR 4.4 without bleeding warrants holding 0-1 dose and reducing weekly maintenance dose while avoiding excessive reversal on a mechanical mitral valve.".into(),
            options_or_questions: vec![
                "Confirm absence of bleeding and interacting medications, and consider holding one warfarin dose with a 10-15% weekly dose reduction.".into(),
                "Schedule repeat INR in 3-5 days.".into(),
            ],
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-warfarin-1".into(),
                revision: "rev-inr-1".into(),
            }],
        }],
        basis_and_limitations: "Prepared from synthetic chart INR resulted 2026-08-27.".into(),
        basis_citations: Vec::new(),
        limits: Vec::new(),
        visit_concern: Some(VisitConcernInput {
            concern_id: ConcernId::new("concern-supratherapeutic-inr").unwrap(),
            urgency: ConcernUrgency::Today,
            reason: "Supratherapeutic INR 4.4 on mechanical mitral valve resulted today ahead of tomorrow's anticoagulation visit.".into(),
            records: vec![ExactRecordReference {
                candidate_id: "Observation/inr-warfarin-1".into(),
                revision: "rev-inr-1".into(),
            }],
        }),
    };

    let sel_args_str = serde_json::to_string(&json!({
        "decisions": [{
            "source_id": CALENDAR_SOURCE_ID,
            "appointment_id": "appt-tomorrow-1",
            "starts_at_unix": tomorrow_start,
            "action": "prepare_now",
            "reason": "Tomorrow's anticoagulation visit already has today's supratherapeutic INR 4.4 posted."
        }]
    }))
    .unwrap();
    let read_args_str = serde_json::to_string(&json!({
        "references": [{ "candidate_id": "Observation/inr-warfarin-1", "revision": "rev-inr-1", "cursor": null }],
        "max_result_bytes": 16_384
    }))
    .unwrap();
    let save_args_str = serde_json::to_string(&tomorrow_save).unwrap();

    let sse_responses = vec![
        json!({
            "id": "resp-sel-1",
            "status": "completed",
            "output": [{
                "type": "function_call",
                "status": "completed",
                "call_id": "http-sel-1",
                "name": SELECT_VISITS_TOOL,
                "arguments": sel_args_str
            }],
            "usage": { "input_tokens": 120, "output_tokens": 40 }
        }),
        json!({
            "id": "resp-sel-2",
            "status": "completed",
            "output": [{
                "type": "message",
                "role": "assistant",
                "status": "completed",
                "content": [{ "type": "output_text", "text": "Selected tomorrow visit." }]
            }],
            "usage": { "input_tokens": 140, "output_tokens": 10 }
        }),
        json!({
            "id": "resp-br-1",
            "status": "completed",
            "output": [{
                "type": "function_call",
                "status": "completed",
                "call_id": "http-read-1",
                "name": READ_TOOL,
                "arguments": read_args_str
            }],
            "usage": { "input_tokens": 200, "output_tokens": 35 }
        }),
        json!({
            "id": "resp-br-2",
            "status": "completed",
            "output": [{
                "type": "function_call",
                "status": "completed",
                "call_id": "http-save-1",
                "name": SAVE_BRIEFING_TOOL,
                "arguments": save_args_str
            }],
            "usage": { "input_tokens": 260, "output_tokens": 90 }
        }),
        json!({
            "id": "resp-br-3",
            "status": "completed",
            "output": [{
                "type": "message",
                "role": "assistant",
                "status": "completed",
                "content": [{ "type": "output_text", "text": "Prepared tomorrow visit briefing." }]
            }],
            "usage": { "input_tokens": 300, "output_tokens": 12 }
        }),
    ];

    let (endpoint, server_handle) = spawn_responses_sse_server(sse_responses).await;
    let mut http_provider =
        ResponsesProvider::new(endpoint, "local-test-key".into(), Arc::new(NoopQuotaGate)).unwrap();

    let wake = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut http_provider,
            BASE_NOW_UNIX,
        )
        .await
        .expect("end-to-end wake over ResponsesProvider HTTP SSE");
    let wire_requests = tokio::time::timeout(Duration::from_secs(5), server_handle)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(wire_requests.len(), 5);
    assert_eq!(wire_requests[0]["model"], "background-worker-lane");
    assert_eq!(wake.completed_briefings.len(), 1);

    let saved_tomorrow = store
        .visit_briefing(
            &VisitSlotKey {
                practice: practice.clone(),
                source_id: CALENDAR_SOURCE_ID.into(),
                appointment_id: "appt-tomorrow-1".into(),
                starts_at_unix: tomorrow_start,
            },
            auth.as_ref(),
        )
        .await
        .unwrap()
        .expect("tomorrow briefing saved without manual start");
    assert_eq!(
        saved_tomorrow.raised_concern_id,
        Some(ConcernId::new("concern-supratherapeutic-inr").unwrap())
    );

    // Part B: Crash-and-recovery idempotency!
    // Simulate a worker crash AFTER `save_visit_briefing` (and its concern) committed in Postgres,
    // where `ToolStarted` was persisted for `save_visit_briefing` without `ToolReturned`.
    let crash_practice = format!("practice-vp-crash-{}", Uuid::new_v4().simple());
    let crash_ws = format!("pw-crash-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&crash_practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();
    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &crash_practice,
            workspace: &crash_ws,
            mrn: "MRN-CRASH",
            display_name: "recovering patient",
            records: &[make_record(ChartDoc {
                workspace: &crash_ws,
                source_id: CHART_SOURCE_ID,
                candidate_id: "Observation/inr-crash-1",
                revision: "rev-1",
                record_type: "Observation",
                title: "INR Result",
                date: "2026-08-27",
                text: "INR 4.6 ahead of tomorrow's anticoagulation visit.",
            })],
            pulled_at: BASE_NOW_UNIX - 600,
        },
    )
    .await;
    let crash_appt = make_appointment(
        "appt-crash-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Anticoagulation check",
        "2026-08-28T16:00:00+00:00",
        "2026-08-28T16:30:00+00:00",
        Some("MRN-CRASH"),
    );
    let crash_start = crash_appt.window().starts_at_unix();
    let crash_cal = InMemoryScheduleSource::new(vec![crash_appt]);
    store
        .sync_practice_schedule(
            auth.as_ref(),
            &crash_practice,
            &crash_cal,
            BASE_NOW_UNIX - 120,
        )
        .await
        .unwrap();

    // Schedule preparation at `BASE_NOW_UNIX + 60` so we can interrupt attempt 1 mid-flight:
    let mut sel_crash = ScriptedProvider::new(vec![
        call_turn(
            "sel-crash",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-crash-1",
                    "starts_at_unix": crash_start,
                    "action": "schedule",
                    "prepare_at_unix": BASE_NOW_UNIX + 60,
                    "reason": "Prepare tomorrow's anticoagulation visit."
                }]
            }),
        ),
        done_turn("Scheduled."),
    ]);
    store
        .wake_visit_preparation(
            auth.as_ref(),
            &crash_practice,
            &roles,
            &skills,
            &mut sel_crash,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();

    // Attempt 1 runs `read_patient_sources` + `save_visit_briefing` (which commits the briefing + concern),
    // and then the provider interrupts on the final post-tool turn (simulating worker crash before completion)!
    let crash_save = SaveBriefingArgs {
        visit_card_orientation: "INR 4.6 ahead of tomorrow's visit; dose adjustment options open."
            .into(),
        trajectory: vec![BriefingClaimInput {
            text: "INR 4.6 on 2026-08-27.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-crash-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "INR rose to 4.6.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-crash-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Medication changes since prior INR need review.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-crash-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Warfarin hold and maintenance dose reduction".into(),
            clinical_rationale: "INR 4.6 exceeds therapeutic range.".into(),
            options_or_questions: vec![
                "Consider holding one dose and rechecking INR in 3-5 days.".into(),
            ],
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-crash-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        basis_and_limitations: "Prepared from INR 4.6.".into(),
        basis_citations: Vec::new(),
        limits: Vec::new(),
        visit_concern: Some(VisitConcernInput {
            concern_id: ConcernId::new("concern-crash-inr").unwrap(),
            urgency: ConcernUrgency::Today,
            reason: "INR 4.6 resulted before tomorrow's visit.".into(),
            records: vec![ExactRecordReference {
                candidate_id: "Observation/inr-crash-1".into(),
                revision: "rev-1".into(),
            }],
        }),
    };

    // Drive attempt 1 manually and interrupt before completion, then recover via `wake_visit_preparation`:
    // Even stronger: persist `ToolStarted` for `save-crash-1` and execute the save WITHOUT `ToolReturned`,
    // proving `PracticeJobHost::saved_outcome` replays the receipt on recovery without re-running `save_visit_briefing`!
    let mut interrupt_then_recover = ScriptedProvider::new(vec![
        // Attempt 1:
        call_turn(
            "read-crash-1",
            READ_TOOL,
            json!({
                "references": [{ "candidate_id": "Observation/inr-crash-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384
            }),
        ),
        call_turn(
            "save-crash-1",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&crash_save).unwrap(),
        ),
        // Simulate network/worker failure on turn 3 BEFORE run finishes:
        Err("simulated worker interruption after save_visit_briefing committed".into()),
        // Attempt 2 (automatic recovery inside wake_visit_preparation):
        // Sees `save-crash-1` already in recovered event log and completes cleanly!
        done_turn("Recovered and completed without duplicate writes."),
    ]);

    let crash_wake = store
        .wake_visit_preparation(
            auth.as_ref(),
            &crash_practice,
            &roles,
            &skills,
            &mut interrupt_then_recover,
            BASE_NOW_UNIX + 60,
        )
        .await
        .unwrap();
    assert_eq!(crash_wake.completed_briefings.len(), 1);

    // Verify attempt 1 interrupted after `save-crash-1` and attempt 2 recovered cleanly:
    let briefing_scope = crash_wake.queued_briefing_jobs[0].clone();
    let job_after = store.practice_job(&briefing_scope).await.unwrap().unwrap();
    assert_eq!(job_after.status, WorkStatus::Completed);
    assert_eq!(
        job_after.attempts.len(),
        2,
        "expected interrupted attempt 1 followed by recovered attempt 2"
    );

    // Verify strictly ONE saved briefing and ONE concern exist in Postgres:
    let all_crash_briefings = store
        .visit_briefings_for_workspace(&crash_practice, &crash_ws, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(all_crash_briefings.len(), 1);
    let open_concerns = open_concerns_for_workspaces(
        &store,
        &auth,
        &crash_practice,
        std::slice::from_ref(&crash_ws),
    )
    .await;
    assert_eq!(open_concerns.len(), 1);
    assert_eq!(open_concerns[0].id.as_str(), "concern-crash-inr");
}

/// Pre-visit concern publication and lifecycle across 3 synthetic visits:
/// 1. Routine visit (`visit_concern: None`) -> publishes NO concern;
/// 2. Ordinary pre-visit news (`ConcernUrgency::Today`) -> open before visit end,
///    retires automatically once the visit ends (`now_unix >= ends_at_unix`);
/// 3. Urgent critical finding (`ConcernUrgency::Now`) -> stays `Open` even AFTER
///    the visit ends, refuses unauthorized physician open attempts, and retires
///    once an authorized on-panel physician opens it via `Store::open_concern`.
#[tokio::test]
async fn ac6_visit_concern_lifecycle_distinguishes_none_ordinary_today_and_urgent_now() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    assert!(!should_retire_visit_concern(
        ConcernUrgency::Today,
        false,
        false
    ));
    assert!(should_retire_visit_concern(
        ConcernUrgency::Today,
        false,
        true
    ));
    assert!(!should_retire_visit_concern(
        ConcernUrgency::Now,
        false,
        true
    ));
    assert!(should_retire_visit_concern(ConcernUrgency::Now, true, true));

    let practice = format!("practice-vp-ac6-{}", Uuid::new_v4().simple());
    let ws_none = format!("pw-c-none-{}", Uuid::new_v4().simple());
    let ws_today = format!("pw-c-today-{}", Uuid::new_v4().simple());
    let ws_now = format!("pw-c-now-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    for (ws, mrn, name, text) in [
        (
            &ws_none,
            "MRN-CNONE",
            "Routine Visit Patient",
            "Stable controlled hypothyroidism, TSH 1.9 mIU/L.",
        ),
        (
            &ws_today,
            "MRN-CTODAY",
            "Today Concern Patient",
            "New external discharge note from urgent care yesterday for transient TIA evaluation; carotid duplex pending.",
        ),
        (
            &ws_now,
            "MRN-CNOW",
            "Urgent Now Patient",
            "Critical serum potassium 6.2 mEq/L resulted 30 minutes ago ahead of renal visit.",
        ),
    ] {
        seed_roster_patient(
            &store,
            &auth,
            RosterPatient {
                practice: &practice,
                workspace: ws,
                mrn,
                display_name: name,
                records: &[make_record(ChartDoc {
                    workspace: ws,
                    source_id: CHART_SOURCE_ID,
                    candidate_id: "Observation/finding-1",
                    revision: "rev-1",
                    record_type: "Observation",
                    title: "Clinical Finding",
                    date: "2026-08-27",
                    text,
                })],
                pulled_at: BASE_NOW_UNIX - 1200,
            },
        )
        .await;
    }

    // All three visits run 15:00–15:30Z (`ends_at_unix = BASE_NOW_UNIX + 5400`):
    let appts = vec![
        make_appointment(
            "appt-c-none",
            AppointmentStatus::Scheduled,
            "booked",
            "Routine thyroid check",
            "2026-08-27T15:00:00+00:00",
            "2026-08-27T15:30:00+00:00",
            Some("MRN-CNONE"),
        ),
        make_appointment(
            "appt-c-today",
            AppointmentStatus::Scheduled,
            "booked",
            "Post-urgent-care follow-up",
            "2026-08-27T15:00:00+00:00",
            "2026-08-27T15:30:00+00:00",
            Some("MRN-CTODAY"),
        ),
        make_appointment(
            "appt-c-now",
            AppointmentStatus::Scheduled,
            "booked",
            "CKD follow-up",
            "2026-08-27T15:00:00+00:00",
            "2026-08-27T15:30:00+00:00",
            Some("MRN-CNOW"),
        ),
    ];
    let slot_start = appts[0].window().starts_at_unix();
    let slot_end = appts[0].window().ends_at_unix();

    let cal = InMemoryScheduleSource::new(appts);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 300)
        .await
        .unwrap();

    let build_save = |orientation: &str, concern: Option<VisitConcernInput>| SaveBriefingArgs {
        visit_card_orientation: orientation.to_owned(),
        trajectory: vec![BriefingClaimInput {
            text: orientation.to_owned(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/finding-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: orientation.to_owned(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/finding-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Follow-up evaluation at today's visit.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/finding-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Clinical review at 15:00 visit".into(),
            clinical_rationale: "Grounded in today's finding.".into(),
            options_or_questions: vec![
                "Review finding with patient and decide next diagnostic/therapeutic step.".into(),
            ],
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/finding-1".into(),
                revision: "rev-1".into(),
            }],
        }],
        basis_and_limitations: "Prepared from today's chart.".into(),
        basis_citations: Vec::new(),
        limits: Vec::new(),
        visit_concern: concern,
    };

    // Note: appointments with identical `starts_at_unix` run in appointment_id order:
    // `appt-c-none`, `appt-c-now`, `appt-c-today`!
    let save_none = build_save(
        "Stable hypothyroidism (TSH 1.9 mIU/L); routine review.",
        None,
    );
    let save_now = build_save(
        "Critical hyperkalemia K 6.2 mEq/L resulted 30m ago; immediate triage options open.",
        Some(VisitConcernInput {
            concern_id: ConcernId::new("concern-critical-k-62").unwrap(),
            urgency: ConcernUrgency::Now,
            reason: "Critical serum potassium 6.2 mEq/L requires immediate physician review even if patient misses or finishes the 15:00 slot.".into(),
            records: vec![ExactRecordReference {
                candidate_id: "Observation/finding-1".into(),
                revision: "rev-1".into(),
            }],
        }),
    );
    let save_today = build_save(
        "Urgent-care TIA evaluation yesterday; carotid duplex pending ahead of 15:00 visit.",
        Some(VisitConcernInput {
            concern_id: ConcernId::new("concern-previsit-tia").unwrap(),
            urgency: ConcernUrgency::Today,
            reason: "New urgent-care TIA note arrived before today's 15:00 visit.".into(),
            records: vec![ExactRecordReference {
                candidate_id: "Observation/finding-1".into(),
                revision: "rev-1".into(),
            }],
        }),
    );

    let mut provider = ScriptedProvider::new(vec![
        call_turn(
            "sel-c3",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-c-none",
                        "starts_at_unix": slot_start,
                        "action": "prepare_now",
                        "reason": "Prepare 15:00 thyroid visit."
                    },
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-c-now",
                        "starts_at_unix": slot_start,
                        "action": "prepare_now",
                        "reason": "Critical potassium 6.2 mEq/L before 15:00 visit."
                    },
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-c-today",
                        "starts_at_unix": slot_start,
                        "action": "prepare_now",
                        "reason": "New urgent-care TIA note before 15:00 visit."
                    }
                ]
            }),
        ),
        done_turn("Selected all three."),
        // 1. `appt-c-none`:
        call_turn(
            "r-none",
            READ_TOOL,
            json!({ "references": [{ "candidate_id": "Observation/finding-1", "revision": "rev-1", "cursor": null }], "max_result_bytes": 16_384 }),
        ),
        call_turn(
            "s-none",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&save_none).unwrap(),
        ),
        done_turn("Saved none."),
        // 2. `appt-c-now`:
        call_turn(
            "r-now",
            READ_TOOL,
            json!({ "references": [{ "candidate_id": "Observation/finding-1", "revision": "rev-1", "cursor": null }], "max_result_bytes": 16_384 }),
        ),
        call_turn(
            "s-now",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&save_now).unwrap(),
        ),
        done_turn("Saved now."),
        // 3. `appt-c-today`:
        call_turn(
            "r-today",
            READ_TOOL,
            json!({ "references": [{ "candidate_id": "Observation/finding-1", "revision": "rev-1", "cursor": null }], "max_result_bytes": 16_384 }),
        ),
        call_turn(
            "s-today",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&save_today).unwrap(),
        ),
        done_turn("Saved today."),
    ]);

    store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut provider,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();

    // Before visit end (`BASE_NOW_UNIX < slot_end`):
    // - `ws_none` has ZERO concerns;
    // - `ws_today` (`Today`) and `ws_now` (`Now`) are both Open on Today!
    let all_workspaces = vec![ws_none.clone(), ws_today.clone(), ws_now.clone()];
    let open_before_end =
        open_concerns_for_workspaces(&store, &auth, &practice, &all_workspaces).await;
    assert_eq!(open_before_end.len(), 2);
    assert!(!open_before_end.iter().any(|c| c.workspace == ws_none));
    assert!(
        open_before_end
            .iter()
            .any(|c| c.id.as_str() == "concern-critical-k-62" && c.urgency == ConcernUrgency::Now)
    );
    assert!(
        open_before_end
            .iter()
            .any(|c| c.id.as_str() == "concern-previsit-tia" && c.urgency == ConcernUrgency::Today)
    );

    // Advance clock to `slot_end` (`now_unix >= ends_at_unix`) and run scheduled wake / reconciliation:
    let retired_at_end = store
        .reconcile_visit_concerns(&practice, slot_end)
        .await
        .unwrap();
    assert_eq!(
        retired_at_end,
        vec![ConcernId::new("concern-previsit-tia").unwrap()],
        "ordinary Today visit concern ends automatically with the visit, while Now stays open!"
    );

    let open_after_end =
        open_concerns_for_workspaces(&store, &auth, &practice, &all_workspaces).await;
    assert_eq!(open_after_end.len(), 1);
    assert_eq!(open_after_end[0].id.as_str(), "concern-critical-k-62");
    assert_eq!(open_after_end[0].standing, ConcernStanding::Open);

    let retired_updates = retired_concerns_for_workspace(&store, &auth, &practice, &ws_today).await;
    assert_eq!(retired_updates.len(), 1);
    assert!(
        retired_updates[0]
            .retirement_reason
            .as_str()
            .contains("ended")
    );

    // Now test physician opening the `Now` concern past visit end:
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (on_panel_doc, on_panel_hash) =
        invite_and_sign_in(&auth, &practice, "DrOnPanel", now_secs).await;
    let (off_panel_doc, off_panel_hash) =
        invite_and_sign_in(&auth, &practice, "DrOffPanel", now_secs).await;
    auth.set_membership(&on_panel_doc, &ws_now, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&off_panel_doc, &ws_now, Membership::ClinicDemographic)
        .await
        .unwrap();

    let now_concern_id = ConcernId::new("concern-critical-k-62").unwrap();
    let mut authority = PostgresWorkspaceAuthority;

    // Off-panel physician is refused (`AccessDenied`), leaving the `Now` concern Open!
    let denied = store
        .open_concern(
            &WorkspaceAccess {
                practice: &practice,
                physician: &off_panel_doc.physician_id,
                workspace: &ws_now,
                session_hash: &off_panel_hash,
            },
            &CommandKey::new("open-denied-1").unwrap(),
            &now_concern_id,
            &mut authority,
            slot_end + 300,
        )
        .await;
    assert!(matches!(denied, Err(ConcernError::AccessDenied { .. })));
    assert_eq!(
        open_concerns_for_workspaces(&store, &auth, &practice, std::slice::from_ref(&ws_now))
            .await
            .len(),
        1
    );

    // Authorized on-panel physician opens the `Now` concern after visit end -> records `Opened` state and retires it!
    let opened = store
        .open_concern(
            &WorkspaceAccess {
                practice: &practice,
                physician: &on_panel_doc.physician_id,
                workspace: &ws_now,
                session_hash: &on_panel_hash,
            },
            &CommandKey::new("open-allowed-1").unwrap(),
            &now_concern_id,
            &mut authority,
            slot_end + 600,
        )
        .await
        .unwrap();
    match opened {
        ConcernOpenOutcome::Opened {
            concern,
            retired_after_visit_end,
        } => {
            assert!(retired_after_visit_end);
            assert!(matches!(concern.standing, ConcernStanding::Retired { .. }));
        }
        ConcernOpenOutcome::Duplicate { .. } => panic!("expected fresh Opened"),
    }

    // Repeat command with same CommandKey is idempotent:
    let dup = store
        .open_concern(
            &WorkspaceAccess {
                practice: &practice,
                physician: &on_panel_doc.physician_id,
                workspace: &ws_now,
                session_hash: &on_panel_hash,
            },
            &CommandKey::new("open-allowed-1").unwrap(),
            &now_concern_id,
            &mut authority,
            slot_end + 605,
        )
        .await
        .unwrap();
    assert!(matches!(dup, ConcernOpenOutcome::Duplicate { .. }));

    let link = store
        .visit_concern_link(&practice, &ws_now, &now_concern_id, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        link.open_state,
        VisitConcernOpenState::Opened {
            opened_at: slot_end + 600,
            physician: on_panel_doc.physician_id.clone(),
        }
    );
    assert!(matches!(
        store
            .visit_concern_link(&practice, "pw-unregistered", &now_concern_id, auth.as_ref())
            .await,
        Err(BriefingError::Access(
            PracticeAccessError::UnregisteredPatient { .. }
        ))
    ));
    // Verify InitialUnderstanding jobs refuse visit-preparation, memory, retrieve-work, and clinical-library tools:
    let iu_assign = Assignment::synthetic_patient_stub(&ws_now, &skills).unwrap();
    let iu_scope = PracticeScope {
        practice: practice.clone(),
        workspace: ws_now.clone(),
        job_id: "iu-boundary-check".into(),
    };
    store
        .accept_practice_job(&iu_scope, &iu_assign, &roles, &skills, auth.as_ref())
        .await
        .unwrap();
    let iu_claim = store
        .claim_practice_job(&iu_scope, 120, auth.as_ref())
        .await
        .unwrap()
        .expect("claim iu job");
    let mut iu_host = PracticeJobHost {
        store: store.clone(),
        claim: iu_claim,
        authority: auth.as_ref(),
    };
    use harness::engine::Product as _;
    for refused_tool in [
        SELECT_VISITS_TOOL,
        SAVE_BRIEFING_TOOL,
        "patient_memory",
        "retrieve_patient_work",
        "search_clinical_library",
    ] {
        let out = iu_host
            .execute(
                &engine::Invocation {
                    id: format!("refuse-{refused_tool}"),
                    name: refused_tool.into(),
                    arguments: "{}".into(),
                },
                json!({}),
            )
            .await
            .unwrap();
        assert!(
            matches!(out, engine::ToolOutcome::Refused(_)),
            "InitialUnderstanding must refuse {refused_tool}, got {out:?}"
        );
    }
    assert!(
        open_concerns_for_workspaces(&store, &auth, &practice, &[ws_now])
            .await
            .is_empty()
    );
}
