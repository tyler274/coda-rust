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

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use duet_wire::models::canonical_record_revision::Entity as RevisionEntity;
use duet_wire::models::conversation_message::Role;
use duet_wire::models::evidence_item::Entity as ItemEntity;
use duet_wire::models::evidence_library_entry::Entity as LibraryEntity;
use duet_wire::models::{
    CanonicalRecordRevision, ConversationAcceptance, ConversationMessage, ConversationMessageState,
    ConversationPage, ConversationRun, ConversationRunState, ConversationSendRequest,
    DoctorCollection, DoctorPageSave, EvidenceBatchRequest, EvidenceBatchResponse, EvidenceItem,
    EvidenceLibraryEntry, EvidenceLibraryPage, FollowUpPage, InsightCyclePage,
    IntelligenceSettings, SavedRoutineList, SharingOverviewResponse,
};
use serde::de::DeserializeOwned;

const CONVERSATION: &str = include_str!("../fixtures/conversation.json");
const INSIGHTS: &str = include_str!("../fixtures/insights.json");
const SETTINGS: &str = include_str!("../fixtures/settings.json");
const LIBRARY: &str = include_str!("../fixtures/library.json");
const ROUTINES: &str = include_str!("../fixtures/routines.json");
const FOLLOW_UPS: &str = include_str!("../fixtures/follow-ups.json");
const DOCTOR: &str = include_str!("../fixtures/doctor.json");
const SHARING: &str = include_str!("../fixtures/sharing.json");
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
    doctor: Option<DoctorCollection>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Writes {
    records: Vec<EvidenceItem>,
    doctor: Option<DoctorCollection>,
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
        let writes_path = path.with_file_name("writes.json");
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
            doctor: writes.doctor,
        })
    }

    fn save_messages(&self) -> Result<(), String> {
        write_json(&self.path, &self.sent)
    }

    fn save_writes(&self) -> Result<(), String> {
        write_json(
            &self.writes_path,
            &Writes {
                records: self.records.clone(),
                doctor: self.doctor.clone(),
            },
        )
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
        .route("/api/insight-cycles", get(insights))
        .route("/api/intelligence/settings", get(settings))
        .route("/api/evidence/library", post(library))
        .route("/api/evidence", post(ingest))
        .route("/api/saved-routines", get(routines))
        .route("/api/follow-ups", get(follow_ups))
        .route("/api/doctor-collection", get(doctor))
        .route("/api/doctor-collection/page", put(save_doctor))
        .route("/api/sharing/overview", get(sharing))
        .with_state(store)
}

fn fixture<T: DeserializeOwned>(name: &str, raw: &str) -> T {
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
    let mut page: EvidenceLibraryPage = fixture("library", LIBRARY);
    for item in &store.records {
        page.entries.push(library_entry(item));
    }
    Json(page)
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
    for item in items {
        if item.id.trim().is_empty() || record_title(&item).is_none() {
            return Err(StatusCode::BAD_REQUEST);
        }
        let entity = revision_entity(item.entity);
        let record_id = uuid::Uuid::parse_str(&item.id).unwrap_or(uuid::Uuid::nil());
        revisions.push(CanonicalRecordRevision::new(entity, record_id, 1));
        stored.push(record_id);
        if store.records.iter().any(|kept| kept.id == item.id) {
            continue;
        }
        let mut item = item;
        stamp_recorded_at(&mut item);
        added.push(item);
    }
    let previous = store.records.len();
    store.records.extend(added);
    if store.records.len() != previous && store.save_writes().is_err() {
        store.records.truncate(previous);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut response = EvidenceBatchResponse::new(revisions);
    response.stored = Some(stored);
    Ok(Json(response))
}

async fn routines() -> Json<SavedRoutineList> {
    Json(fixture("routines", ROUTINES))
}

async fn follow_ups() -> Json<FollowUpPage> {
    Json(fixture("follow-ups", FOLLOW_UPS))
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

async fn sharing() -> Json<SharingOverviewResponse> {
    Json(fixture("sharing", SHARING))
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
        let sharing: SharingOverviewResponse = fixture("sharing", SHARING);
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
        assert_eq!(sharing.account_label, "me");
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
        let _ = fs::remove_file(path.with_file_name("writes.json"));
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
}
