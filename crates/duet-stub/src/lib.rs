//! Local Duet stand-in for the emulator and tests.
//!
//! ```text
//! cargo run -p duet-stub
//! cargo run -p duet-stub -- --listen 127.0.0.1:8000 --state ~/.local/state/duet-stub/messages.json
//! ```
//!
//! The four tab pages are the recorded fixtures. A send is accepted and appended
//! to the state file, so the next process still has that line. The stand-in does
//! not invent an assistant reply.
#![deny(missing_docs)]

pub mod care;

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use duet_wire::models::canonical_record_revision::Entity as RevisionEntity;
use duet_wire::models::conversation_message::Role;
use duet_wire::models::evidence_item::{Entity as ItemEntity, Operation};
use duet_wire::models::evidence_library_entry::Entity as LibraryEntity;
use duet_wire::models::{
    CanonicalRecordRevision, ConversationAcceptance, ConversationMessage, ConversationMessageState,
    ConversationPage, ConversationRun, ConversationRunState, ConversationSendRequest,
    ConversationSourceDetail, DoctorCollection, DoctorPageSave, EvidenceBatchRequest,
    EvidenceBatchResponse, EvidenceItem, EvidenceLibraryEntry, EvidenceLibraryPage, FollowUp,
    FollowUpPage, FollowUpState, FollowUpVersionRequest, InsightCyclePage, IntelligenceSettings,
    SavedRoutine, SavedRoutineList, SavedRoutineUpdateRequest,
};
use serde::de::DeserializeOwned;

const CONVERSATION: &str = include_str!("../fixtures/conversation.json");
const INSIGHTS: &str = include_str!("../fixtures/insights.json");
const SETTINGS: &str = include_str!("../fixtures/settings.json");
const LIBRARY: &str = include_str!("../fixtures/library.json");
const ROUTINES: &str = include_str!("../fixtures/routines.json");
const FOLLOW_UPS: &str = include_str!("../fixtures/follow-ups.json");
const DOCTOR: &str = include_str!("../fixtures/doctor.json");
const RUN_ID: &str = "00000000-0000-0000-0000-000000000099";

/// Accepted messages kept beside the recorded conversation.
///
/// Medications, notes, and a saved For my doctor page live in `writes.json`
/// next to the message file, so a restart keeps both.
pub struct Store {
    path: PathBuf,
    writes_path: PathBuf,
    sent: Vec<ConversationMessage>,
    records: Vec<EvidenceItem>,
    removed_evidence: Vec<String>,
    corrections: Vec<EvidenceItem>,
    doctor: Option<DoctorCollection>,
    routines: Vec<SavedRoutine>,
    removed_routines: Vec<uuid::Uuid>,
    follow_ups: Vec<FollowUp>,
    /// Connected Care state (`ENG-44`, `ENG-39`, `ENG-45`, `ENG-46`).
    pub care: care::ConnectedCareStore,
    /// Structurally unexportable private operating index (`ENG-52`).
    pub private_operating_index: care::PrivateOperatingIndex,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Writes {
    records: Vec<EvidenceItem>,
    #[serde(default)]
    removed_evidence: Vec<String>,
    #[serde(default)]
    corrections: Vec<EvidenceItem>,
    doctor: Option<DoctorCollection>,
    #[serde(default)]
    routines: Vec<SavedRoutine>,
    #[serde(default)]
    removed_routines: Vec<uuid::Uuid>,
    #[serde(default)]
    follow_ups: Vec<FollowUp>,
    #[serde(default)]
    care: care::ConnectedCareStore,
}

impl Store {
    /// Loads accepted messages from `path`. A missing file starts empty.
    /// Writes load from `writes.json` in the same directory.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, String> {
        let path = path.into();
        let sent = if path.is_file() {
            let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?
        } else {
            Vec::new()
        };
        let writes_path = writes_beside(&path);
        let writes = if writes_path.is_file() {
            let text = fs::read_to_string(&writes_path).map_err(|error| error.to_string())?;
            serde_json::from_str(&text)
                .map_err(|error| format!("{}: {error}", writes_path.display()))?
        } else {
            Writes::default()
        };
        Ok(Self {
            path,
            writes_path,
            sent,
            records: writes.records,
            removed_evidence: writes.removed_evidence,
            corrections: writes.corrections,
            doctor: writes.doctor,
            routines: writes.routines,
            removed_routines: writes.removed_routines,
            follow_ups: writes.follow_ups,
            care: writes.care,
            private_operating_index: care::PrivateOperatingIndex::default(),
        })
    }

    fn save_messages(&self) -> Result<(), String> {
        write_json(&self.path, &self.sent)
    }

    pub(crate) fn save_writes(&self) -> Result<(), String> {
        write_json(
            &self.writes_path,
            &Writes {
                records: self.records.clone(),
                removed_evidence: self.removed_evidence.clone(),
                corrections: self.corrections.clone(),
                doctor: self.doctor.clone(),
                routines: self.routines.clone(),
                removed_routines: self.removed_routines.clone(),
                follow_ups: self.follow_ups.clone(),
                care: self.care.clone(),
            },
        )
    }
}

fn writes_beside(path: &std::path::Path) -> PathBuf {
    match path.file_name().and_then(|name| name.to_str()) {
        Some("messages.json") => path.with_file_name("writes.json"),
        Some(name) => path.with_file_name(format!("{name}.writes.json")),
        None => path.with_file_name("writes.json"),
    }
}

fn write_json(path: &std::path::Path, value: &impl serde::Serialize) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, text).map_err(|error| error.to_string())?;
    fs::rename(&temporary, path).map_err(|error| error.to_string())
}

/// Builds the stand-in router. `store` is shared across requests and written on each new send.
pub fn router(store: Arc<Mutex<Store>>) -> Router {
    Router::new()
        .route("/api/conversation/messages", get(messages).post(send))
        .route(
            "/api/conversation/messages/{message_id}/sources/{source_id}",
            get(message_source),
        )
        .route(
            "/api/conversation/messages/{message_id}/sources/{source_id}/revision",
            get(cited_revision),
        )
        .route("/api/insight-cycles", get(insights))
        .route("/api/intelligence/settings", get(settings))
        .route("/api/evidence/library", post(library))
        .route("/api/evidence/browse", post(browse))
        .route("/api/evidence", post(ingest))
        .route("/api/saved-routines", get(routines))
        .route(
            "/api/saved-routines/{routine_id}",
            put(update_routine).delete(delete_routine),
        )
        .route("/api/follow-ups", get(follow_ups))
        .route(
            "/api/follow-ups/{follow_up_id}/pause",
            post(pause_follow_up),
        )
        .route(
            "/api/follow-ups/{follow_up_id}/resume",
            post(resume_follow_up),
        )
        .route("/api/doctor-collection", get(doctor).delete(clear_doctor))
        .route("/api/doctor-collection/page", put(save_doctor))
        .route(
            "/api/doctor-collection/share-link",
            post(care::post_create_share_link).delete(care::delete_share_link),
        )
        .route(
            "/shared/for-my-doctor/{token}",
            get(care::get_shared_for_my_doctor),
        )
        .route(
            "/api/physician-sharing/status",
            get(care::get_sharing_status),
        )
        .route(
            "/api/physician-sharing/accept-invitation",
            post(care::post_accept_invitation),
        )
        .route(
            "/api/physician-sharing/consent",
            put(care::put_grant_consent),
        )
        .route(
            "/api/physician-sharing/stop-sharing",
            post(care::post_stop_sharing),
        )
        .route(
            "/api/physician-sharing/records",
            post(care::post_patient_record),
        )
        .route(
            "/api/physician-sharing/consented-records",
            get(care::get_consented_records),
        )
        .route(
            "/api/luna-notes/propose",
            post(care::post_propose_luna_note),
        )
        .route(
            "/api/luna-notes/proposals",
            get(care::get_luna_note_proposals),
        )
        .route(
            "/api/luna-notes/{id}/approve",
            post(care::post_approve_luna_note),
        )
        .route(
            "/api/doctor-plan",
            get(care::get_doctor_plan).put(care::put_doctor_plan),
        )
        .with_state(store)
}

pub(crate) fn fixture<T: DeserializeOwned>(name: &str, raw: &str) -> T {
    serde_json::from_str(raw).unwrap_or_else(|error| panic!("{name} fixture: {error}"))
}

fn base_page() -> ConversationPage {
    fixture("conversation", CONVERSATION)
}

async fn messages(State(store): State<Arc<Mutex<Store>>>) -> Json<ConversationPage> {
    let store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut page = base_page();
    let extra = store.sent.len() as i32;
    page.messages.extend(store.sent.iter().cloned());
    page.revision += extra;
    Json(page)
}

async fn send(
    State(store): State<Arc<Mutex<Store>>>,
    Json(request): Json<ConversationSendRequest>,
) -> Result<(StatusCode, Json<ConversationAcceptance>), StatusCode> {
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    if let Some(existing) = find_message(&store, request.id) {
        return Ok((
            StatusCode::ACCEPTED,
            Json(acceptance(existing, base_page().revision)),
        ));
    }
    let sequence = next_sequence(&store);
    let now = chrono::Utc::now().fixed_offset();
    let message = ConversationMessage::new(
        now,
        request.id,
        1,
        Role::User,
        sequence,
        ConversationMessageState::Complete,
        request.text,
        now,
    );
    store.sent.push(message.clone());
    if let Err(_error) = store.save_messages() {
        store.sent.pop();
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let revision = base_page().revision + store.sent.len() as i32;
    Ok((StatusCode::ACCEPTED, Json(acceptance(message, revision))))
}

fn find_message(store: &Store, id: uuid::Uuid) -> Option<ConversationMessage> {
    base_page()
        .messages
        .into_iter()
        .chain(store.sent.iter().cloned())
        .find(|message| message.id == id)
}

fn next_sequence(store: &Store) -> i32 {
    base_page()
        .messages
        .iter()
        .chain(store.sent.iter())
        .map(|message| message.sequence)
        .max()
        .unwrap_or(0)
        + 1
}

async fn message_source(
    State(store): State<Arc<Mutex<Store>>>,
    Path((message_id, source_id)): Path<(uuid::Uuid, String)>,
) -> Result<Json<ConversationSourceDetail>, StatusCode> {
    let store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let message = find_message(&store, message_id).ok_or(StatusCode::NOT_FOUND)?;
    let source = message
        .sources
        .iter()
        .flatten()
        .find(|source| source.id == source_id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;
    let text = match source.id.as_str() {
        "sleep-note" => "About seven hours, from the sleep note.".to_owned(),
        _ => source.title.clone(),
    };
    Ok(Json(ConversationSourceDetail::new(source, text)))
}

/// Stand-in for the exact cited revision. The sleep note's payload stays on the
/// revision; the client renders person-facing lines and does not dump JSON.
async fn cited_revision(
    Path((_message_id, source_id)): Path<(uuid::Uuid, String)>,
) -> Result<Json<duet_wire::models::EvidenceRevisionView>, StatusCode> {
    if source_id != "sleep-note" {
        return Err(StatusCode::NOT_FOUND);
    }
    use duet_wire::models::{
        CanonicalSourceAddress, EvidenceRevisionView, canonical_source_address::Entity,
        evidence_revision_view::Status,
    };
    let mut view = EvidenceRevisionView::new(
        CanonicalSourceAddress::new(
            uuid::Uuid::nil(),
            "sleep-note".into(),
            Entity::ClinicalNote,
            1,
        ),
        Status::Current,
        true,
    );
    view.payload = Some(Some(
        [
            ("title".to_owned(), serde_json::json!("Sleep note")),
            (
                "body".to_owned(),
                serde_json::json!("About seven hours, from the sleep note."),
            ),
            ("source_label".to_owned(), serde_json::json!("Sleep note")),
        ]
        .into(),
    ));
    Ok(Json(view))
}

fn acceptance(message: ConversationMessage, revision: i32) -> ConversationAcceptance {
    let stamp = message.created_at;
    let run = ConversationRun::new(
        None,
        stamp,
        uuid::Uuid::parse_str(RUN_ID).expect("run id"),
        Some(message.id),
        1,
        ConversationRunState::Queued,
        stamp,
    );
    ConversationAcceptance::new(message, revision, run)
}

async fn insights() -> Json<InsightCyclePage> {
    Json(fixture("insights", INSIGHTS))
}

async fn settings() -> Json<IntelligenceSettings> {
    Json(fixture("settings", SETTINGS))
}

async fn library(State(store): State<Arc<Mutex<Store>>>) -> Json<EvidenceLibraryPage> {
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut stamped = false;
    for item in &mut store.records {
        stamped |= stamp_recorded_at(item);
    }
    if stamped {
        let _ = store.save_writes();
    }
    Json(library_page(&store))
}

async fn ingest(
    State(store): State<Arc<Mutex<Store>>>,
    Json(request): Json<EvidenceBatchRequest>,
) -> Result<Json<EvidenceBatchResponse>, StatusCode> {
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let items = request.items.unwrap_or_default();
    let mut stored = Vec::new();
    let mut revisions = Vec::new();
    let mut added = Vec::new();
    let mut corrections = Vec::new();
    let mut removed = Vec::new();
    for item in items {
        if item.id.trim().is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        let entity = revision_entity(item.entity);
        let record_id = uuid::Uuid::parse_str(&item.id).unwrap_or(uuid::Uuid::nil());
        revisions.push(CanonicalRecordRevision::new(entity, record_id, 1));
        stored.push(record_id);
        match item.operation {
            Some(Operation::Delete) => {
                if !evidence_known(&store, &item.id) {
                    return Err(StatusCode::NOT_FOUND);
                }
                removed.push(item.id);
            }
            _ if payload_text(&item, &["corrects"]).is_some() => {
                if record_title(&item).is_none() {
                    return Err(StatusCode::BAD_REQUEST);
                }
                let target = payload_text(&item, &["corrects"]).expect("corrects");
                if !evidence_known(&store, &target) {
                    return Err(StatusCode::NOT_FOUND);
                }
                corrections.push(item);
            }
            _ => {
                if record_title(&item).is_none() {
                    return Err(StatusCode::BAD_REQUEST);
                }
                if store.records.iter().any(|kept| kept.id == item.id) {
                    continue;
                }
                let mut item = item;
                stamp_recorded_at(&mut item);
                added.push(item);
            }
        }
    }
    let previous_records = store.records.clone();
    let previous_removed = store.removed_evidence.clone();
    let previous_corrections = store.corrections.clone();
    for id in &removed {
        store.records.retain(|kept| &kept.id != id);
        store
            .corrections
            .retain(|item| payload_text(item, &["corrects"]).as_deref() != Some(id));
        if !store.removed_evidence.iter().any(|kept| kept == id) {
            store.removed_evidence.push(id.clone());
        }
    }
    for item in corrections {
        let target = payload_text(&item, &["corrects"]);
        store
            .corrections
            .retain(|kept| payload_text(kept, &["corrects"]) != target);
        store.corrections.push(item);
    }
    store.records.extend(added);
    if store.save_writes().is_err() {
        store.records = previous_records;
        store.removed_evidence = previous_removed;
        store.corrections = previous_corrections;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut response = EvidenceBatchResponse::new(revisions);
    response.stored = Some(stored);
    Ok(Json(response))
}

async fn routines(State(store): State<Arc<Mutex<Store>>>) -> Json<SavedRoutineList> {
    let store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut page: SavedRoutineList = fixture("routines", ROUTINES);
    page.items = listed_routines(&store);
    Json(page)
}

async fn update_routine(
    State(store): State<Arc<Mutex<Store>>>,
    Path(id): Path<uuid::Uuid>,
    Json(request): Json<SavedRoutineUpdateRequest>,
) -> Result<Json<SavedRoutine>, StatusCode> {
    if request.title.trim().is_empty() || request.body.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let Some(mut routine) = listed_routines(&store)
        .into_iter()
        .find(|item| item.id == id)
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    if routine.revision != request.expected_revision {
        return Err(StatusCode::CONFLICT);
    }
    routine.title = request.title;
    routine.body = request.body;
    routine.revision += 1;
    routine.updated_at = chrono::Utc::now().fixed_offset();
    let previous = store.routines.clone();
    remember_routine(&mut store, routine.clone());
    if store.save_writes().is_err() {
        store.routines = previous;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(routine))
}

#[derive(serde::Deserialize)]
struct ExpectedRevision {
    expected_revision: i32,
}

async fn delete_routine(
    State(store): State<Arc<Mutex<Store>>>,
    Path(id): Path<uuid::Uuid>,
    Query(query): Query<ExpectedRevision>,
) -> Result<StatusCode, StatusCode> {
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let Some(routine) = listed_routines(&store)
        .into_iter()
        .find(|item| item.id == id)
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    if routine.revision != query.expected_revision {
        return Err(StatusCode::CONFLICT);
    }
    let previous_routines = store.routines.clone();
    let previous_removed = store.removed_routines.clone();
    store.routines.retain(|item| item.id != id);
    store.removed_routines.push(id);
    if store.save_writes().is_err() {
        store.routines = previous_routines;
        store.removed_routines = previous_removed;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn follow_ups(State(store): State<Arc<Mutex<Store>>>) -> Json<FollowUpPage> {
    let store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut page: FollowUpPage = fixture("follow-ups", FOLLOW_UPS);
    page.items = listed_follow_ups(&store);
    Json(page)
}

async fn pause_follow_up(
    State(store): State<Arc<Mutex<Store>>>,
    Path(id): Path<uuid::Uuid>,
    Json(request): Json<FollowUpVersionRequest>,
) -> Result<Json<FollowUp>, StatusCode> {
    set_follow_up_state(store, id, request.revision, FollowUpState::Paused)
}

async fn resume_follow_up(
    State(store): State<Arc<Mutex<Store>>>,
    Path(id): Path<uuid::Uuid>,
    Json(request): Json<FollowUpVersionRequest>,
) -> Result<Json<FollowUp>, StatusCode> {
    set_follow_up_state(store, id, request.revision, FollowUpState::Active)
}

fn set_follow_up_state(
    store: Arc<Mutex<Store>>,
    id: uuid::Uuid,
    revision: i32,
    state: FollowUpState,
) -> Result<Json<FollowUp>, StatusCode> {
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let Some(mut follow_up) = listed_follow_ups(&store)
        .into_iter()
        .find(|item| item.id == id)
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    if follow_up.revision != revision {
        return Err(StatusCode::CONFLICT);
    }
    follow_up.state = state;
    follow_up.pause_reason = match state {
        FollowUpState::Paused => Some(Some(duet_wire::models::follow_up::PauseReason::User)),
        _ => None,
    };
    follow_up.revision += 1;
    follow_up.updated_at = chrono::Utc::now().fixed_offset();
    let previous = store.follow_ups.clone();
    remember_follow_up(&mut store, follow_up.clone());
    if store.save_writes().is_err() {
        store.follow_ups = previous;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(follow_up))
}

async fn doctor(State(store): State<Arc<Mutex<Store>>>) -> Json<DoctorCollection> {
    let store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    Json(
        store
            .doctor
            .clone()
            .unwrap_or_else(|| fixture("doctor", DOCTOR)),
    )
}

async fn clear_doctor(State(store): State<Arc<Mutex<Store>>>) -> Result<StatusCode, StatusCode> {
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let previous = store.doctor.clone();
    store.doctor = Some(DoctorCollection::new(-1));
    if store.save_writes().is_err() {
        store.doctor = previous;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn save_doctor(
    State(store): State<Arc<Mutex<Store>>>,
    Json(save): Json<DoctorPageSave>,
) -> Result<Json<DoctorCollection>, StatusCode> {
    let mut store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let base = store
        .doctor
        .clone()
        .unwrap_or_else(|| fixture("doctor", DOCTOR));
    let saved = apply_doctor_save(base, save);
    let previous = store.doctor.clone();
    store.doctor = Some(saved.clone());
    if store.save_writes().is_err() {
        store.doctor = previous;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(saved))
}

fn stamp_recorded_at(item: &mut EvidenceItem) -> bool {
    let payload = item.payload.get_or_insert_with(Default::default);
    if payload.contains_key("recorded_at") {
        return false;
    }
    payload.insert(
        "recorded_at".to_owned(),
        serde_json::Value::String(chrono::Local::now().to_rfc3339()),
    );
    true
}

fn recorded_day(item: &EvidenceItem) -> chrono::DateTime<chrono::FixedOffset> {
    item.payload
        .as_ref()
        .and_then(|payload| payload.get("recorded_at"))
        .and_then(|value| value.as_str())
        .and_then(|text| chrono::DateTime::parse_from_rfc3339(text).ok())
        .unwrap_or_else(|| chrono::Local::now().fixed_offset())
}

fn listed_routines(store: &Store) -> Vec<SavedRoutine> {
    let mut items = fixture::<SavedRoutineList>("routines", ROUTINES).items;
    items.retain(|item| !store.removed_routines.contains(&item.id));
    for saved in &store.routines {
        if store.removed_routines.contains(&saved.id) {
            continue;
        }
        if let Some(existing) = items.iter_mut().find(|item| item.id == saved.id) {
            *existing = saved.clone();
        } else {
            items.push(saved.clone());
        }
    }
    items
}

fn remember_routine(store: &mut Store, routine: SavedRoutine) {
    if let Some(existing) = store.routines.iter_mut().find(|item| item.id == routine.id) {
        *existing = routine;
    } else {
        store.routines.push(routine);
    }
}

fn listed_follow_ups(store: &Store) -> Vec<FollowUp> {
    let mut items = fixture::<FollowUpPage>("follow-ups", FOLLOW_UPS).items;
    for saved in &store.follow_ups {
        if let Some(existing) = items.iter_mut().find(|item| item.id == saved.id) {
            *existing = saved.clone();
        } else {
            items.push(saved.clone());
        }
    }
    items
}

fn remember_follow_up(store: &mut Store, follow_up: FollowUp) {
    if let Some(existing) = store
        .follow_ups
        .iter_mut()
        .find(|item| item.id == follow_up.id)
    {
        *existing = follow_up;
    } else {
        store.follow_ups.push(follow_up);
    }
}

fn record_title(item: &EvidenceItem) -> Option<String> {
    match item.entity {
        ItemEntity::HealthEvidence => payload_text(item, &["display_name", "displayName"]),
        ItemEntity::ClinicalNote => payload_text(item, &["title"]),
    }
}

fn payload_text(item: &EvidenceItem, keys: &[&str]) -> Option<String> {
    let payload = item.payload.as_ref()?;
    keys.iter().find_map(|key| {
        payload
            .get(*key)
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(str::to_owned)
    })
}

fn evidence_known(store: &Store, id: &str) -> bool {
    if store.removed_evidence.iter().any(|kept| kept == id) {
        return false;
    }
    if store.records.iter().any(|item| item.id == id) {
        return true;
    }
    let page: EvidenceLibraryPage = fixture("library", LIBRARY);
    page.entries
        .iter()
        .any(|entry| entry.id == id || entry.record_id.to_string() == id)
}

fn library_page(store: &Store) -> EvidenceLibraryPage {
    let mut page: EvidenceLibraryPage = fixture("library", LIBRARY);
    let changed = !store.removed_evidence.is_empty() || !store.corrections.is_empty();
    if changed {
        page.entries.retain(|entry| {
            !store
                .removed_evidence
                .iter()
                .any(|id| id == &entry.id || id == &entry.record_id.to_string())
        });
    }
    for item in &store.records {
        if store.removed_evidence.iter().any(|id| id == &item.id) {
            continue;
        }
        if page.entries.iter().any(|entry| entry.id == item.id) {
            continue;
        }
        page.entries.push(library_entry(item));
    }
    if changed {
        for correction in &store.corrections {
            let Some(target) = payload_text(correction, &["corrects"]) else {
                continue;
            };
            if let Some(entry) = page
                .entries
                .iter_mut()
                .find(|entry| entry.id == target || entry.record_id.to_string() == target)
            {
                if let Some(title) = record_title(correction) {
                    entry.title = title;
                }
                if let Some(summary) = payload_text(correction, &["summary", "body"]) {
                    entry.summary = Some(Some(summary));
                }
            }
        }
        page.counts.clear();
        for entry in &page.entries {
            *page.counts.entry(entry.category.clone()).or_insert(0) += 1;
        }
        page.total_count = page.entries.len() as i32;
    }
    page
}

fn library_entry(item: &EvidenceItem) -> EvidenceLibraryEntry {
    let title = record_title(item).unwrap_or_default();
    let (category, entity, summary) = match item.entity {
        ItemEntity::HealthEvidence => ("medication", LibraryEntity::HealthEvidence, None),
        ItemEntity::ClinicalNote => (
            "clinicalDocument",
            LibraryEntity::ClinicalNote,
            payload_text(item, &["body"]).map(Some),
        ),
    };
    let mut entry = EvidenceLibraryEntry::new(
        category.to_owned(),
        Vec::new(),
        false,
        Some(recorded_day(item)),
        entity,
        None,
        item.id.clone(),
        1,
        None,
        uuid::Uuid::parse_str(&item.id).unwrap_or(uuid::Uuid::nil()),
        false,
        title,
    );
    entry.source_label = Some(Some("Added in Duet".to_owned()));
    entry.summary = summary;
    entry
}

fn revision_entity(entity: ItemEntity) -> RevisionEntity {
    match entity {
        ItemEntity::HealthEvidence => RevisionEntity::HealthEvidence,
        ItemEntity::ClinicalNote => RevisionEntity::ClinicalNote,
    }
}

fn apply_doctor_save(base: DoctorCollection, save: DoctorPageSave) -> DoctorCollection {
    let current = base.items.clone().unwrap_or_default();
    let items = save
        .item_ids
        .iter()
        .filter_map(|id| current.iter().find(|item| item.id == *id).cloned())
        .collect();
    DoctorCollection {
        items: Some(items),
        note: save.note.clone().or(base.note),
        note_generation: base.note_generation,
        page: Some(save.page),
        page_updated_at: Some(Some(chrono::Utc::now().fixed_offset())),
        proposal: base.proposal,
    }
}

/// Stand-in for the durable record browser: the stub library as exact records, paged
/// fifty at a time by an offset cursor. Stub records carry no provenance connection.
async fn browse(
    State(store): State<Arc<Mutex<Store>>>,
    Json(query): Json<duet_wire::models::EvidenceBrowseQuery>,
) -> Result<Json<duet_wire::models::EvidenceBrowsePage>, StatusCode> {
    use duet_wire::models::{
        CanonicalSourceAddress, EvidenceBrowsePage, EvidenceBrowseRecord, EvidenceRecordProvenance,
        canonical_source_address::Entity as AddressEntity, evidence_browse_record::Status,
        evidence_record_provenance::Writer,
    };
    let store = store.lock().unwrap_or_else(|poison| poison.into_inner());
    let page = library_page(&store);
    let records: Vec<EvidenceBrowseRecord> = page
        .entries
        .iter()
        .map(|entry| {
            let entity = match entry.entity {
                LibraryEntity::ClinicalNote => AddressEntity::ClinicalNote,
                LibraryEntity::HealthEvidence => AddressEntity::HealthEvidence,
            };
            let mut record = EvidenceBrowseRecord::new(
                CanonicalSourceAddress::new(entry.record_id, entry.id.clone(), entity, 1),
                Status::Current,
                entry.title.clone(),
                EvidenceRecordProvenance::new("manual".into(), Writer::Device),
                Some(entry.preview.clone().unwrap_or_default()),
            );
            record.provenance.source_label = entry.source_label.clone().flatten();
            record.date = entry.date.map(|date| date.to_rfc3339());
            record.date_role = record.date.as_ref().map(|_| "recorded_at".to_owned());
            record
        })
        .collect();
    let offset = match query.cursor.as_deref() {
        None => 0,
        Some(cursor) => cursor.parse().map_err(|_| StatusCode::CONFLICT)?,
    };
    if offset > records.len() {
        return Err(StatusCode::CONFLICT);
    }
    let end = (offset + 50).min(records.len());
    let mut out =
        EvidenceBrowsePage::new(0, 0, records.len() as i32, records[offset..end].to_vec());
    out.removed_count = Some(0);
    out.next_cursor = (end < records.len()).then(|| end.to_string());
    Ok(Json(out))
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use std::sync::atomic::{AtomicU64, Ordering};
    use tower::ServiceExt;

    fn temp_store() -> (PathBuf, Arc<Mutex<Store>>) {
        static COUNT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "duet-stub-{}-{}.json",
            std::process::id(),
            COUNT.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_file(&path);
        let store = Store::open(&path).unwrap();
        (path, Arc::new(Mutex::new(store)))
    }

    async fn body(response: axum::response::Response) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
            .await
            .unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[test]
    fn fixtures_match_the_wire() {
        let page: ConversationPage = fixture("conversation", CONVERSATION);
        let insights: InsightCyclePage = fixture("insights", INSIGHTS);
        let settings: IntelligenceSettings = fixture("settings", SETTINGS);
        let library: EvidenceLibraryPage = fixture("library", LIBRARY);
        let routines: SavedRoutineList = fixture("routines", ROUTINES);
        let follow_ups: FollowUpPage = fixture("follow-ups", FOLLOW_UPS);
        let doctor: DoctorCollection = fixture("doctor", DOCTOR);
        assert_eq!(page.messages.len(), 3);
        assert_eq!(routines.items[0].title, "Evening walk");
        assert_eq!(
            follow_ups.items[0].display_title,
            Some(Some("Evening walk".into()))
        );
        assert_eq!(doctor.items.as_ref().unwrap()[0].title, "Ask about sleep");
        assert_eq!(
            insights.items[0].insights.as_ref().unwrap()[0].title,
            "Steady walks"
        );
        assert!(settings.enabled);
        assert_eq!(library.entries[0].title, "Lisinopril");
        assert_eq!(library.entries[1].title, "LDL cholesterol");
        assert_eq!(
            page.messages[1].sources.as_ref().unwrap()[0].title,
            "Sleep note"
        );
    }

    #[tokio::test]
    async fn a_message_source_opens_and_evidence_can_be_corrected_or_deleted() {
        let (path, store) = temp_store();
        let app = router(store);
        let opened = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/conversation/messages/00000000-0000-0000-0000-000000000002/sources/sleep-note")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(opened.status().is_success());
        let detail: ConversationSourceDetail = serde_json::from_str(&body(opened).await).unwrap();
        assert_eq!(detail.text, "About seven hours, from the sleep note.");
        assert_eq!(detail.reference.title, "Sleep note");
        let revision = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/conversation/messages/00000000-0000-0000-0000-000000000002/sources/sleep-note/revision")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(revision.status().is_success());
        let view: duet_wire::models::EvidenceRevisionView =
            serde_json::from_str(&body(revision).await).unwrap();
        assert_eq!(
            view.payload.flatten().unwrap()["body"],
            "About seven hours, from the sleep note."
        );
        let missing = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/conversation/messages/00000000-0000-0000-0000-000000000002/sources/missing")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);

        let correction = serde_json::json!({
            "items": [{
                "id": "00000000-0000-0000-0000-000000000071",
                "entity": "health_evidence",
                "operation": "upsert",
                "payload": {
                    "display_name": "Lisinopril 5 mg",
                    "corrects": "med-1",
                    "source_label": "Added in Duet"
                }
            }]
        });
        let corrected = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/evidence")
                    .header("content-type", "application/json")
                    .body(Body::from(correction.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(corrected.status().is_success(), "{}", body(corrected).await);
        let deletion = serde_json::json!({
            "items": [{
                "id": "lab-1",
                "entity": "health_evidence",
                "operation": "delete"
            }]
        });
        let deleted = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/evidence")
                    .header("content-type", "application/json")
                    .body(Body::from(deletion.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(deleted.status().is_success(), "{}", body(deleted).await);
        let library = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/evidence/library")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let library: EvidenceLibraryPage = serde_json::from_str(&body(library).await).unwrap();
        assert_eq!(library.entries[0].title, "Lisinopril 5 mg");
        assert!(
            library
                .entries
                .iter()
                .all(|entry| entry.title != "LDL cholesterol")
        );
        assert_eq!(
            library
                .entries
                .iter()
                .filter(|entry| entry.id == "med-1")
                .count(),
            1
        );
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(writes_beside(&path));
    }

    #[tokio::test]
    async fn a_send_survives_reopening_the_state_file() {
        let (path, store) = temp_store();
        let app = router(store);
        let request = ConversationSendRequest::new(
            uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000030").unwrap(),
            "keys for \"hello\"".into(),
            "US/Arizona".into(),
        );
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/conversation/messages")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let acceptance: ConversationAcceptance =
            serde_json::from_str(&body(response).await).unwrap();
        assert_eq!(acceptance.message.text, "keys for \"hello\"");

        let reopened = Arc::new(Mutex::new(Store::open(&path).unwrap()));
        let page = router(reopened)
            .oneshot(
                Request::builder()
                    .uri("/api/conversation/messages")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page: ConversationPage = serde_json::from_str(&body(page).await).unwrap();
        assert_eq!(page.messages.len(), 4);
        assert_eq!(page.messages[3].text, "keys for \"hello\"");
        let _ = fs::remove_file(path);
    }

    #[tokio::test]
    async fn the_same_id_is_not_stored_twice() {
        let (path, store) = temp_store();
        let id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000031").unwrap();
        let first = ConversationSendRequest::new(id, "Ping".into(), "US/Arizona".into());
        let second = ConversationSendRequest::new(id, "Other".into(), "US/Arizona".into());
        let app = router(store);
        for request in [first, second] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/api/conversation/messages")
                        .header("content-type", "application/json")
                        .body(Body::from(serde_json::to_vec(&request).unwrap()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::ACCEPTED);
        }
        let saved: Vec<ConversationMessage> =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].text, "Ping");
        let _ = fs::remove_file(path);
    }

    #[tokio::test]
    async fn a_medication_and_a_note_stay_in_the_library() {
        let (path, store) = temp_store();
        let app = router(store);
        let medication = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000061").unwrap();
        let note = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000062").unwrap();
        post_evidence(
            &app,
            medication,
            "health_evidence",
            "Atorvastatin 20 mg",
            None,
        )
        .await;
        post_evidence(&app, medication, "health_evidence", "Other name", None).await;
        post_evidence(
            &app,
            note,
            "clinical_note",
            "Sleep this week",
            Some("Walks have been steady."),
        )
        .await;
        let library = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/evidence/library")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let library: EvidenceLibraryPage = serde_json::from_str(&body(library).await).unwrap();
        assert_eq!(library.entries[0].title, "Lisinopril");
        assert_eq!(library.entries[2].title, "Atorvastatin 20 mg");
        assert_eq!(
            library.entries[2].date.unwrap().date_naive(),
            chrono::Local::now().date_naive()
        );
        assert_eq!(
            library.entries[2].source_label,
            Some(Some("Added in Duet".into()))
        );
        assert_eq!(
            library
                .entries
                .iter()
                .filter(|entry| entry.title == "Atorvastatin 20 mg")
                .count(),
            1
        );
        assert_eq!(library.entries[3].title, "Sleep this week");
        assert_eq!(library.entries[3].category, "clinicalDocument");

        let item = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000046").unwrap();
        let save = DoctorPageSave::new(1, vec![item], duet_wire::models::DoctorPageContent::new());
        let saved = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/doctor-collection/page")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&save).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let saved: DoctorCollection = serde_json::from_str(&body(saved).await).unwrap();
        assert_eq!(saved.items.as_ref().unwrap()[0].title, "Ask about sleep");
        let again = app
            .oneshot(
                Request::builder()
                    .uri("/api/doctor-collection")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let again: DoctorCollection = serde_json::from_str(&body(again).await).unwrap();
        assert_eq!(again.items.as_ref().unwrap()[0].title, "Ask about sleep");
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(writes_beside(&path));
    }

    #[tokio::test]
    async fn an_edited_doctor_page_stays_and_start_fresh_clears_it() {
        let (path, store) = temp_store();
        let app = router(store);
        let item = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000046").unwrap();
        let mut page = duet_wire::models::DoctorPageContent::new();
        page.name = Some(Some("Ada".into()));
        page.date_of_birth = Some(Some(chrono::NaiveDate::from_ymd_opt(1990, 4, 2).unwrap()));
        page.cover_note = Some(Some("Bring the sleep note.".into()));
        page.questions = Some(vec!["How is my sleep?".into()]);
        let mut save = DoctorPageSave::new(1, vec![item], page.clone());
        save.note = Some(Some("I want to talk about sleep.".into()));
        let saved = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/doctor-collection/page")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&save).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let saved: DoctorCollection = serde_json::from_str(&body(saved).await).unwrap();
        assert_eq!(saved.items.as_ref().unwrap()[0].title, "Ask about sleep");
        assert_eq!(saved.note, Some(Some("I want to talk about sleep.".into())));
        let saved_page = saved.page.unwrap();
        assert_eq!(saved_page.name, Some(Some("Ada".into())));
        assert_eq!(
            saved_page.date_of_birth,
            Some(Some(chrono::NaiveDate::from_ymd_opt(1990, 4, 2).unwrap()))
        );
        assert_eq!(saved_page.questions, Some(vec!["How is my sleep?".into()]));
        assert_eq!(
            saved_page.cover_note,
            Some(Some("Bring the sleep note.".into()))
        );

        let mut omitted = DoctorPageSave::new(1, Vec::new(), page);
        omitted.note = Some(Some("I want to talk about sleep.".into()));
        let omitted = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/doctor-collection/page")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&omitted).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let omitted: DoctorCollection = serde_json::from_str(&body(omitted).await).unwrap();
        assert!(omitted.items.unwrap_or_default().is_empty());
        assert_eq!(
            omitted.note,
            Some(Some("I want to talk about sleep.".into()))
        );
        assert_eq!(
            omitted.page.unwrap().questions,
            Some(vec!["How is my sleep?".into()])
        );

        let cleared = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/doctor-collection")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(cleared.status().is_success());
        let again = app
            .oneshot(
                Request::builder()
                    .uri("/api/doctor-collection")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let again: DoctorCollection = serde_json::from_str(&body(again).await).unwrap();
        assert!(again.items.unwrap_or_default().is_empty());
        assert!(again.note.flatten().is_none());
        assert!(again.page.is_none());
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(writes_beside(&path));
    }

    #[tokio::test]
    async fn a_plan_edit_and_a_paused_reminder_stay() {
        let (path, store) = temp_store();
        let app = router(store);
        let routine = "00000000-0000-0000-0000-000000000041";
        let follow_up = "00000000-0000-0000-0000-000000000042";
        let save =
            SavedRoutineUpdateRequest::new("A longer walk.".into(), 1, "Morning walk".into());
        let saved = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/api/saved-routines/{routine}"))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&save).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = saved.status();
        let text = body(saved).await;
        assert!(status.is_success(), "plan save {status}: {text}");
        let saved: SavedRoutine = serde_json::from_str(&text).unwrap();
        assert_eq!(saved.title, "Morning walk");
        assert_eq!(saved.revision, 2);
        let stale = SavedRoutineUpdateRequest::new("Other".into(), 1, "Other".into());
        let rejected = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/api/saved-routines/{routine}"))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&stale).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(rejected.status(), StatusCode::CONFLICT);
        let listed = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/saved-routines")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let listed: SavedRoutineList = serde_json::from_str(&body(listed).await).unwrap();
        assert_eq!(listed.items[0].title, "Morning walk");
        assert_eq!(listed.items[0].body, "A longer walk.");
        let removed = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("/api/saved-routines/{routine}?expected_revision=2"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(removed.status(), StatusCode::NO_CONTENT);
        let after = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/saved-routines")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let after: SavedRoutineList = serde_json::from_str(&body(after).await).unwrap();
        assert!(after.items.iter().all(|item| item.title != "Morning walk"));
        assert!(after.items.iter().all(|item| item.title != "Evening walk"));

        let pause = FollowUpVersionRequest::new(1);
        let paused = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/follow-ups/{follow_up}/pause"))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&pause).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let paused: FollowUp = serde_json::from_str(&body(paused).await).unwrap();
        assert_eq!(paused.state, FollowUpState::Paused);
        let resume = FollowUpVersionRequest::new(paused.revision);
        let resumed = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/follow-ups/{follow_up}/resume"))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&resume).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let resumed: FollowUp = serde_json::from_str(&body(resumed).await).unwrap();
        assert_eq!(resumed.state, FollowUpState::Active);
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(writes_beside(&path));
    }

    async fn post_evidence(
        app: &Router,
        id: uuid::Uuid,
        entity: &str,
        title: &str,
        body_text: Option<&str>,
    ) {
        let mut payload = serde_json::Map::new();
        if entity == "clinical_note" {
            payload.insert("title".into(), serde_json::json!(title));
            if let Some(body_text) = body_text {
                payload.insert("body".into(), serde_json::json!(body_text));
            }
        } else {
            payload.insert("display_name".into(), serde_json::json!(title));
            payload.insert("concept_id".into(), serde_json::json!("medication:manual"));
        }
        payload.insert("source_label".into(), serde_json::json!("Added in Duet"));
        let item = serde_json::json!({
            "id": id,
            "entity": entity,
            "operation": "upsert",
            "payload": payload,
        });
        let request = serde_json::json!({ "items": [item] });
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/evidence")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn connected_care_consent_approved_luna_notes_share_link_and_read_only_doctors_plan() {
        let (path, store) = temp_store();
        let app = router(store);

        // 1. Before consent, consented-records is forbidden.
        let unconsented = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/physician-sharing/consented-records")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unconsented.status(), StatusCode::FORBIDDEN);

        // 2. Accept explicit practice invitation.
        let accept = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/physician-sharing/accept-invitation")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"practice_id":"practice-coda-1","practice_name":"Coda Internal Medicine","patient_workspace_id":"pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e","invitation_token":"inv-1"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(accept.status(), StatusCode::OK);

        // 3. Propose two Luna notes: one stays unapproved with private operating_index reasoning,
        //    and one is approved by the patient and saved to You.
        let unapproved = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/luna-notes/propose")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"date":"2026-10-12","title":"Private hypothesis","summary":"Unapproved draft","private_operating_note":"private_operating_index_secret_never_export"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unapproved.status(), StatusCode::OK);

        let proposed = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/luna-notes/propose")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"date":"2026-10-13","title":"Morning walking log","summary":"Walked 30 minutes daily without chest tightness.","private_operating_note":"private_operating_index_secret_never_export"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(proposed.status(), StatusCode::OK);
        let proposal: serde_json::Value = serde_json::from_str(&body(proposed).await).unwrap();
        let proposal_id = proposal["id"].as_str().unwrap();

        let approved = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/luna-notes/{proposal_id}/approve"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(approved.status(), StatusCode::OK);

        // Also add a consented Quest lab record.
        let add_lab = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/physician-sharing/records")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"record_id":"rec-quest-ldl","category":"lab","title":"LDL Cholesterol","summary":"98 mg/dL","recorded_at":"2026-10-13T08:00:00Z","origin_label":"from Quest","source_identity":"quest:ldl:2026-10-13"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(add_lab.status(), StatusCode::OK);

        // 4. Grant versioned "share with my physician" consent.
        let consent = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/physician-sharing/consent")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"consent_version":"v1"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(consent.status(), StatusCode::OK);

        // 5. Read consented records: verify sha256 cursors and zero private-note leakage.
        let read_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/physician-sharing/consented-records")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(read_resp.status(), StatusCode::OK);
        let read_text = body(read_resp).await;
        assert!(!read_text.contains("private_operating_index_secret_never_export"));
        assert!(!read_text.contains("operating_index"));
        assert!(!read_text.contains("Unapproved draft"));
        let read_json: serde_json::Value = serde_json::from_str(&read_text).unwrap();
        assert_eq!(read_json["consent_version"], "v1");
        let items = read_json["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        let cursor = read_json["latest_cursor"].as_str().unwrap();
        assert_eq!(cursor.len(), 64);

        // Incremental cursor read returns 0 items when nothing changed after `cursor`.
        let incremental = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/api/physician-sharing/consented-records?after_cursor={cursor}"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(incremental.status(), StatusCode::OK);
        let inc_json: serde_json::Value = serde_json::from_str(&body(incremental).await).unwrap();
        assert_eq!(inc_json["items"].as_array().unwrap().len(), 0);

        // 6. Expiring, revocable read-only "For my doctor" share link.
        let share_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/doctor-collection/share-link")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"ttl_seconds":3600}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(share_resp.status(), StatusCode::OK);
        let share_json: serde_json::Value = serde_json::from_str(&body(share_resp).await).unwrap();
        let share_path = share_json["share_path"].as_str().unwrap();

        let shared_page = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(share_path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(shared_page.status(), StatusCode::OK);
        let shared_page_json: serde_json::Value =
            serde_json::from_str(&body(shared_page).await).unwrap();
        assert_eq!(shared_page_json["read_only"], true);

        let revoked_link = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/doctor-collection/share-link")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(revoked_link.status(), StatusCode::OK);

        let gone = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(share_path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(gone.status(), StatusCode::NOT_FOUND);

        // 7. Read-only "your doctor's plan" in Duet.
        let publish_plan = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/doctor-plan")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"practice_id":"practice-coda-1","patient_workspace_id":"pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e","physician_name":"Dr. Rivera","version":1,"approved_at":"2026-10-13T12:00:00Z","items":[{"id":"item-1","title":"Continue statin","body":"Take rosuvastatin 10 mg nightly."}]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(publish_plan.status(), StatusCode::OK);

        let get_plan = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/doctor-plan")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_plan.status(), StatusCode::OK);
        let plan_json: serde_json::Value = serde_json::from_str(&body(get_plan).await).unwrap();
        assert_eq!(plan_json["label"], "your doctor's plan");
        assert_eq!(plan_json["read_only"], true);
        assert_eq!(plan_json["version"], 1);

        // 8. One-step "stop sharing" immediately revokes physician read access.
        let stop = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/physician-sharing/stop-sharing")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(stop.status(), StatusCode::OK);

        let blocked = app
            .oneshot(
                Request::builder()
                    .uri("/api/physician-sharing/consented-records")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(blocked.status(), StatusCode::FORBIDDEN);

        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(writes_beside(&path));
    }
}
