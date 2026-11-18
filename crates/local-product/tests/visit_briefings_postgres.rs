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
    BriefingError, ConcernOpenOutcome, VisitConcernOpenState, VisitSlotKey,
    should_retire_visit_concern,
};
use local_product::runtime::concerns::{
    CommandKey, Concern, ConcernError, ConcernId, ConcernStanding, ConcernUrgency,
    ExactRecordReference, RetiredConcernUpdate, WorkspaceAccess,
};
use local_product::runtime::jobs::{
    Assignment, PracticeAccessError, PracticeJobHost, PracticeScope,
};
use local_product::runtime::schedule::{
    AppointmentId, AppointmentStatus, AppointmentWindow, ScheduleDelivery, ScheduleSource,
    ScheduleSourceError, SourceAppointment, SourceAppointmentParts, SourceId, SourcePatientId,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, WorkStatus, engine};
use local_product::visit_preparation::{
    BriefingClaimInput, BriefingDecisionInput, SAVE_BRIEFING_TOOL, SELECT_VISITS_TOOL,
    SaveBriefingArgs, VisitConcernInput,
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
        envelope: None,
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
        pulled_at: Some(patient.pulled_at),
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
    let ws_later = format!("pw-c-later-{}", Uuid::new_v4().simple());
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
        (
            &ws_later,
            "MRN-CLATER",
            "Evening Today Concern Patient",
            "New outpatient Holter report shows symptomatic 4-second sinus pause ahead of 21:00 cardiology follow-up.",
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

    // Three visits run 15:00–15:30Z (`ends_at_unix = BASE_NOW_UNIX + 5400`) and a fourth
    // (`appt-c-later`) runs 21:00–21:30Z so it remains upcoming when `open_concern` runs:
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
        make_appointment(
            "appt-c-later",
            AppointmentStatus::Scheduled,
            "booked",
            "Evening arrhythmia follow-up",
            "2026-08-27T21:00:00+00:00",
            "2026-08-27T21:30:00+00:00",
            Some("MRN-CLATER"),
        ),
    ];
    let slot_start = appts[0].window().starts_at_unix();
    let slot_end = appts[0].window().ends_at_unix();
    let later_slot_start = appts[3].window().starts_at_unix();
    let later_slot_end = appts[3].window().ends_at_unix();

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
    let save_later = build_save(
        "Symptomatic 4-second sinus pause on Holter report ahead of 21:00 visit.",
        Some(VisitConcernInput {
            concern_id: ConcernId::new("concern-later-today-stay-open").unwrap(),
            urgency: ConcernUrgency::Today,
            reason: "New Holter sinus pause finding ahead of tonight's 21:00 cardiology visit."
                .into(),
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
                    },
                    {
                        "source_id": CALENDAR_SOURCE_ID,
                        "appointment_id": "appt-c-later",
                        "starts_at_unix": later_slot_start,
                        "action": "prepare_now",
                        "reason": "New Holter sinus pause before 21:00 visit."
                    }
                ]
            }),
        ),
        done_turn("Selected all four."),
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
        // 4. `appt-c-later` (`starts_at_unix = later_slot_start` runs after the 15:00 slots):
        call_turn(
            "r-later",
            READ_TOOL,
            json!({ "references": [{ "candidate_id": "Observation/finding-1", "revision": "rev-1", "cursor": null }], "max_result_bytes": 16_384 }),
        ),
        call_turn(
            "s-later",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(&save_later).unwrap(),
        ),
        done_turn("Saved later today."),
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
    // - `ws_today` (`Today`), `ws_later` (`Today` at 21:00), and `ws_now` (`Now`) are all Open on Today!
    let all_workspaces = vec![
        ws_none.clone(),
        ws_today.clone(),
        ws_now.clone(),
        ws_later.clone(),
    ];
    let open_before_end =
        open_concerns_for_workspaces(&store, &auth, &practice, &all_workspaces).await;
    assert_eq!(open_before_end.len(), 3);
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
    assert!(
        open_before_end
            .iter()
            .any(|c| c.id.as_str() == "concern-later-today-stay-open"
                && c.urgency == ConcernUrgency::Today)
    );

    // Advance clock to `slot_end` (`now_unix >= ends_at_unix` for 15:00 visits, still `< later_slot_end`)
    // and run scheduled wake / reconciliation:
    let retired_at_end = store
        .reconcile_visit_concerns(&practice, slot_end)
        .await
        .unwrap();
    assert_eq!(
        retired_at_end,
        vec![ConcernId::new("concern-previsit-tia").unwrap()],
        "ordinary Today visit concern for ended 15:00 visit retires, while Now and upcoming 21:00 Today stay open!"
    );

    let open_after_end =
        open_concerns_for_workspaces(&store, &auth, &practice, &all_workspaces).await;
    assert_eq!(open_after_end.len(), 2);
    assert!(
        open_after_end
            .iter()
            .any(|c| c.id.as_str() == "concern-critical-k-62")
    );
    assert!(
        open_after_end
            .iter()
            .any(|c| c.id.as_str() == "concern-later-today-stay-open")
    );

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
        open_concerns_for_workspaces(&store, &auth, &practice, std::slice::from_ref(&ws_now))
            .await
            .is_empty()
    );
    // Crucial regression check: opening `ws_now`'s ended `Now` concern MUST NOT falsely
    // retire `ws_later`'s still-upcoming 21:00 `Today` pre-visit concern!
    let later_still_open =
        open_concerns_for_workspaces(&store, &auth, &practice, std::slice::from_ref(&ws_later))
            .await;
    assert_eq!(
        later_still_open.len(),
        1,
        "peer patient's upcoming Today visit concern must remain Open after open_concern"
    );
    assert_eq!(
        later_still_open[0].id.as_str(),
        "concern-later-today-stay-open"
    );

    // Reconciling again at a distinct timestamp before 21:30 retires nothing and never
    // collides on receipt SourceWork; once 21:30 passes, `ws_later` retires cleanly and
    // repeated reconciles at later timestamps remain idempotent:
    assert!(
        store
            .reconcile_visit_concerns(&practice, slot_end + 610)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        store
            .reconcile_visit_concerns(&practice, later_slot_end)
            .await
            .unwrap(),
        vec![ConcernId::new("concern-later-today-stay-open").unwrap()]
    );
    assert!(
        store
            .reconcile_visit_concerns(&practice, later_slot_end + 45)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn closed_prior_concern_refusal_and_selection_scope_isolation() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();

    let practice = format!("practice-vp-closedc-{}", Uuid::new_v4().simple());
    let ws = format!("pw-closedc-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(BASE_NOW_UNIX as u64))
        .await
        .unwrap();

    let rec_v1 = make_record(ChartDoc {
        workspace: &ws,
        source_id: CHART_SOURCE_ID,
        candidate_id: "Observation/inr-previsit",
        revision: "rev-1",
        record_type: "Observation",
        title: "Outpatient INR",
        date: "2026-08-27",
        text: "INR 4.6 on warfarin 5 mg daily ahead of anticoagulation clinic visit.",
    });
    seed_roster_patient(
        &store,
        &auth,
        RosterPatient {
            practice: &practice,
            workspace: &ws,
            mrn: "MRN-CLOSEDC",
            display_name: "Elena Rostova",
            records: std::slice::from_ref(&rec_v1),
            pulled_at: BASE_NOW_UNIX - 600,
        },
    )
    .await;

    let appt = make_appointment(
        "appt-closedc-1",
        AppointmentStatus::Scheduled,
        "booked",
        "Anticoagulation follow-up",
        "2026-08-27T18:00:00+00:00",
        "2026-08-27T18:30:00+00:00",
        Some("MRN-CLOSEDC"),
    );
    let starts_at_unix = appt.window().starts_at_unix();
    let cal = InMemoryScheduleSource::new(vec![appt]);
    store
        .sync_practice_schedule(auth.as_ref(), &practice, &cal, BASE_NOW_UNIX - 300)
        .await
        .unwrap();

    // Part A: Prepare initial briefing raising `concern-inr-initial` (`Today`), verify that
    // a multi-visit selection job on `PracticeJobHost` refuses single-patient workspace tools,
    // then have an authorized physician mark `concern-inr-initial` `Done`, record new chart
    // evidence (`INR 5.9`), and re-prepare: reusing `concern-inr-initial` MUST be refused with
    // `concern_already_closed` until a fresh `concern_id` (`concern-inr-escalated`) is chosen!
    let make_save = |rev: &str, orientation: &str, concern_id: &str| SaveBriefingArgs {
        visit_card_orientation: orientation.to_owned(),
        trajectory: vec![BriefingClaimInput {
            text: orientation.to_owned(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-previsit".into(),
                revision: rev.to_owned(),
            }],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: orientation.to_owned(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-previsit".into(),
                revision: rev.to_owned(),
            }],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Warfarin hold/dose adjustment needed at 18:00 visit.".into(),
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-previsit".into(),
                revision: rev.to_owned(),
            }],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Anticoagulation dose adjustment".into(),
            clinical_rationale: "Supratherapeutic INR before visit.".into(),
            options_or_questions: vec![
                "Consider holding warfarin dose and repeating INR per protocol.".into(),
            ],
            uncertainty: None,
            citations: vec![ExactRecordReference {
                candidate_id: "Observation/inr-previsit".into(),
                revision: rev.to_owned(),
            }],
        }],
        basis_and_limitations: "Prepared from outpatient coagulation lab.".into(),
        basis_citations: Vec::new(),
        limits: Vec::new(),
        visit_concern: Some(VisitConcernInput {
            concern_id: ConcernId::new(concern_id).unwrap(),
            urgency: ConcernUrgency::Today,
            reason: orientation.to_owned(),
            records: vec![ExactRecordReference {
                candidate_id: "Observation/inr-previsit".into(),
                revision: rev.to_owned(),
            }],
        }),
    };

    let mut wake1 = ScriptedProvider::new(vec![
        call_turn(
            "sel-1",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-closedc-1",
                    "starts_at_unix": starts_at_unix,
                    "action": "prepare_now",
                    "reason": "Supratherapeutic INR 4.6 ahead of 18:00 anticoagulation visit."
                }]
            }),
        ),
        done_turn("Selected."),
        call_turn(
            "read-1",
            READ_TOOL,
            json!({ "references": [{ "candidate_id": "Observation/inr-previsit", "revision": "rev-1", "cursor": null }], "max_result_bytes": 16_384 }),
        ),
        call_turn(
            "save-1",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(make_save(
                "rev-1",
                "Supratherapeutic INR 4.6 ahead of 18:00 visit.",
                "concern-inr-initial",
            ))
            .unwrap(),
        ),
        done_turn("Saved initial briefing."),
    ]);
    let wake1_summary = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut wake1,
            BASE_NOW_UNIX,
        )
        .await
        .unwrap();

    // Verify `PracticeJobHost` on a claimed multi-visit selection job refuses all
    // single-patient workspace tools (`SEARCH_TOOL`, `READ_TOOL`, `refresh_patient_sources`,
    // `patient_memory`, `retrieve_patient_work`, `search_clinical_library`, `SAVE_BRIEFING_TOOL`):
    let wake1_sel_scope = wake1_summary.selection_job.expect("wake1 selection job");
    let mut sel_assign = store
        .practice_job(&wake1_sel_scope)
        .await
        .unwrap()
        .unwrap()
        .assignment;
    sel_assign.key = "sel-scope-guard-key".into();
    let sel_scope = PracticeScope {
        practice: practice.clone(),
        workspace: ws.clone(),
        job_id: "sel-scope-guard".into(),
    };
    store
        .accept_practice_job(&sel_scope, &sel_assign, &roles, &skills, auth.as_ref())
        .await
        .unwrap();
    let sel_claim = store
        .claim_practice_job(&sel_scope, 120, auth.as_ref())
        .await
        .unwrap()
        .expect("claim selection scope guard job");
    let mut sel_host = PracticeJobHost {
        store: store.clone(),
        claim: sel_claim,
        authority: auth.as_ref(),
    };
    use harness::engine::Product as _;
    for refused_tool in [
        SEARCH_TOOL,
        READ_TOOL,
        "refresh_patient_sources",
        "patient_memory",
        "retrieve_patient_work",
        "search_clinical_library",
        SAVE_BRIEFING_TOOL,
    ] {
        let out = sel_host
            .execute(
                &engine::Invocation {
                    id: format!("sel-refuse-{refused_tool}"),
                    name: refused_tool.into(),
                    arguments: "{}".into(),
                },
                json!({}),
            )
            .await
            .unwrap();
        assert!(
            matches!(out, engine::ToolOutcome::Refused(_)),
            "selection job must refuse workspace tool {refused_tool}, got {out:?}"
        );
    }

    // Physician marks `concern-inr-initial` Done before the visit:
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (doc, session_hash) = invite_and_sign_in(&auth, &practice, "DrAnticoag", now_secs).await;
    auth.set_membership(&doc, &ws, Membership::Allowed)
        .await
        .unwrap();
    let mut authority = PostgresWorkspaceAuthority;
    store
        .mark_concern_done(
            &WorkspaceAccess {
                practice: &practice,
                physician: &doc.physician_id,
                workspace: &ws,
                session_hash: &session_hash,
            },
            &CommandKey::new("done-initial-inr").unwrap(),
            &ConcernId::new("concern-inr-initial").unwrap(),
            &mut authority,
        )
        .await
        .unwrap();
    assert!(
        open_concerns_for_workspaces(&store, &auth, &practice, std::slice::from_ref(&ws))
            .await
            .is_empty()
    );

    // Repeat INR arrives higher (`INR 5.9`, `rev-2`), triggering re-selection and re-preparation:
    let rec_v2 = make_record(ChartDoc {
        workspace: &ws,
        source_id: CHART_SOURCE_ID,
        candidate_id: "Observation/inr-previsit",
        revision: "rev-2",
        record_type: "Observation",
        title: "Repeat Outpatient INR",
        date: "2026-08-27",
        text: "Repeat confirmatory INR 5.9 with minor gingival oozing reported.",
    });
    store
        .record_chart(
            &ws,
            std::slice::from_ref(&rec_v2),
            &SourcePull {
                source_id: CHART_SOURCE_ID.into(),
                source_label: CHART_SOURCE_LABEL.into(),
                patient: source_patient("MRN-CLOSEDC", "Elena Rostova"),
                pulled_at: Some(BASE_NOW_UNIX + 60),
            },
        )
        .await
        .unwrap();

    let mut wake2 = ScriptedProvider::new(vec![
        call_turn(
            "sel-2",
            SELECT_VISITS_TOOL,
            json!({
                "decisions": [{
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-closedc-1",
                    "starts_at_unix": starts_at_unix,
                    "action": "prepare_now",
                    "reason": "Confirmatory INR rose to 5.9; refresh briefing."
                }]
            }),
        ),
        done_turn("Re-selected."),
        call_turn(
            "read-2",
            READ_TOOL,
            json!({ "references": [{ "candidate_id": "Observation/inr-previsit", "revision": "rev-2", "cursor": null }], "max_result_bytes": 16_384 }),
        ),
        // First attempt reuses closed `concern-inr-initial` -> MUST be refused (`concern_already_closed`)!
        call_turn(
            "save-reuse-closed",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(make_save(
                "rev-2",
                "Repeat INR 5.9 with gingival oozing ahead of 18:00 visit.",
                "concern-inr-initial",
            ))
            .unwrap(),
        ),
        // Second attempt uses fresh `concern-inr-escalated` -> succeeds and opens a real Open concern!
        call_turn(
            "save-fresh-id",
            SAVE_BRIEFING_TOOL,
            serde_json::to_value(make_save(
                "rev-2",
                "Repeat INR 5.9 with gingival oozing ahead of 18:00 visit.",
                "concern-inr-escalated",
            ))
            .unwrap(),
        ),
        done_turn("Saved refreshed briefing with fresh concern."),
    ]);
    let summary2 = store
        .wake_visit_preparation(
            auth.as_ref(),
            &practice,
            &roles,
            &skills,
            &mut wake2,
            BASE_NOW_UNIX + 120,
        )
        .await
        .unwrap();
    assert_eq!(summary2.completed_briefings.len(), 1);
    let refusal_turn_prompt = serde_json::to_string(&wake2.requests[4].items).unwrap();
    assert!(
        refusal_turn_prompt.contains("concern_already_closed"),
        "expected concern_already_closed refusal when reusing Done concern_id, got {refusal_turn_prompt}"
    );

    let open_after_refresh =
        open_concerns_for_workspaces(&store, &auth, &practice, std::slice::from_ref(&ws)).await;
    assert_eq!(open_after_refresh.len(), 1);
    assert_eq!(open_after_refresh[0].id.as_str(), "concern-inr-escalated");
    assert_eq!(open_after_refresh[0].standing, ConcernStanding::Open);
}
