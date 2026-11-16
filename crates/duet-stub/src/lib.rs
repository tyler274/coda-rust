//! Local Duet stand-in for the emulator and tests.
//!
//! ```text
//! cargo run -p duet-stub
//! cargo run -p duet-stub -- --listen 127.0.0.1:8000 --state ~/.local/state/duet-stub/messages.json
//! ```
//!
//! The library and For my doctor pages start from the recorded fixtures. Writes
//! go to `writes.json` beside the state path, so the next process still has them.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use duet_wire::models::canonical_record_revision::Entity as RevisionEntity;
use duet_wire::models::evidence_item::{Entity as ItemEntity, Operation};
use duet_wire::models::evidence_library_entry::Entity as LibraryEntity;
use duet_wire::models::{
    CanonicalRecordRevision, DoctorCollection, DoctorPageSave, EvidenceBatchRequest,
    EvidenceBatchResponse, EvidenceItem, EvidenceLibraryEntry, EvidenceLibraryPage,
};
use serde::de::DeserializeOwned;

const LIBRARY: &str = include_str!("../fixtures/library.json");
const DOCTOR: &str = include_str!("../fixtures/doctor.json");

/// Writes kept beside the recorded fixtures.
///
/// Medications, notes, and a saved For my doctor page live in `writes.json`
/// next to the state path, so a restart keeps them.
pub struct Store {
    writes_path: PathBuf,
    records: Vec<EvidenceItem>,
    removed_evidence: Vec<String>,
    corrections: Vec<EvidenceItem>,
    doctor: Option<DoctorCollection>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Writes {
    records: Vec<EvidenceItem>,
    #[serde(default)]
    removed_evidence: Vec<String>,
    #[serde(default)]
    corrections: Vec<EvidenceItem>,
    doctor: Option<DoctorCollection>,
}

impl Store {
    /// Loads writes from `writes.json` beside `path`. A missing file starts empty.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, String> {
        let path = path.into();
        let writes_path = writes_beside(&path);
        let writes = if writes_path.is_file() {
            let text = fs::read_to_string(&writes_path).map_err(|error| error.to_string())?;
            serde_json::from_str(&text)
                .map_err(|error| format!("{}: {error}", writes_path.display()))?
        } else {
            Writes::default()
        };
        Ok(Self {
            writes_path,
            records: writes.records,
            removed_evidence: writes.removed_evidence,
            corrections: writes.corrections,
            doctor: writes.doctor,
        })
    }

    pub(crate) fn save_writes(&self) -> Result<(), String> {
        write_json(
            &self.writes_path,
            &Writes {
                records: self.records.clone(),
                removed_evidence: self.removed_evidence.clone(),
                corrections: self.corrections.clone(),
                doctor: self.doctor.clone(),
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

/// Builds the stand-in router. `store` is shared across requests and written on each write.
pub fn router(store: Arc<Mutex<Store>>) -> Router {
    Router::new()
        .route("/api/evidence/library", post(library))
        .route("/api/evidence/browse", post(browse))
        .route("/api/evidence", post(ingest))
        .route("/api/doctor-collection", get(doctor).delete(clear_doctor))
        .route("/api/doctor-collection/page", put(save_doctor))
        .with_state(store)
}

pub(crate) fn fixture<T: DeserializeOwned>(name: &str, raw: &str) -> T {
    serde_json::from_str(raw).unwrap_or_else(|error| panic!("{name} fixture: {error}"))
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
        let library: EvidenceLibraryPage = fixture("library", LIBRARY);
        let doctor: DoctorCollection = fixture("doctor", DOCTOR);
        assert_eq!(doctor.items.as_ref().unwrap()[0].title, "Ask about sleep");
        assert_eq!(library.entries[0].title, "Lisinopril");
        assert_eq!(library.entries[1].title, "LDL cholesterol");
    }

    #[tokio::test]
    async fn evidence_can_be_corrected_or_deleted() {
        let (path, store) = temp_store();
        let app = router(store);
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
