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
use axum::routing::{get, post};
use duet_wire::models::conversation_message::Role;
use duet_wire::models::{
    ConversationAcceptance, ConversationMessage, ConversationMessageState, ConversationPage,
    ConversationRun, ConversationRunState, ConversationSendRequest, EvidenceLibraryPage,
    FollowUpPage, InsightCyclePage, IntelligenceSettings, SavedRoutineList,
    SharingOverviewResponse,
};
use serde::de::DeserializeOwned;

const CONVERSATION: &str = include_str!("../fixtures/conversation.json");
const INSIGHTS: &str = include_str!("../fixtures/insights.json");
const SETTINGS: &str = include_str!("../fixtures/settings.json");
const LIBRARY: &str = include_str!("../fixtures/library.json");
const ROUTINES: &str = include_str!("../fixtures/routines.json");
const FOLLOW_UPS: &str = include_str!("../fixtures/follow-ups.json");
const SHARING: &str = include_str!("../fixtures/sharing.json");
const RUN_ID: &str = "00000000-0000-0000-0000-000000000099";

/// Accepted messages kept beside the recorded conversation.
pub struct Store {
    path: PathBuf,
    sent: Vec<ConversationMessage>,
}

impl Store {
    /// Loads accepted messages from `path`. A missing file starts empty.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, String> {
        let path = path.into();
        let sent = if path.is_file() {
            let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?
        } else {
            Vec::new()
        };
        Ok(Self { path, sent })
    }

    fn save(&self) -> Result<(), String> {
        if let Some(parent) = self.path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let text = serde_json::to_string_pretty(&self.sent).map_err(|error| error.to_string())?;
        let temporary = self.path.with_extension("json.tmp");
        fs::write(&temporary, text).map_err(|error| error.to_string())?;
        fs::rename(&temporary, &self.path).map_err(|error| error.to_string())
    }
}

/// Builds the stand-in router. `store` is shared across requests and written on each new send.
pub fn router(store: Arc<Mutex<Store>>) -> Router {
    Router::new()
        .route("/api/conversation/messages", get(messages).post(send))
        .route("/api/insight-cycles", get(insights))
        .route("/api/intelligence/settings", get(settings))
        .route("/api/evidence/library", post(library))
        .route("/api/saved-routines", get(routines))
        .route("/api/follow-ups", get(follow_ups))
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
    if let Err(_error) = store.save() {
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

async fn library() -> Json<EvidenceLibraryPage> {
    Json(fixture("library", LIBRARY))
}

async fn routines() -> Json<SavedRoutineList> {
    Json(fixture("routines", ROUTINES))
}

async fn follow_ups() -> Json<FollowUpPage> {
    Json(fixture("follow-ups", FOLLOW_UPS))
}

async fn sharing() -> Json<SharingOverviewResponse> {
    Json(fixture("sharing", SHARING))
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
        let sharing: SharingOverviewResponse = fixture("sharing", SHARING);
        assert_eq!(page.messages.len(), 3);
        assert_eq!(routines.items[0].title, "Evening walk");
        assert_eq!(
            follow_ups.items[0].display_title,
            Some(Some("Evening walk".into()))
        );
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
}
