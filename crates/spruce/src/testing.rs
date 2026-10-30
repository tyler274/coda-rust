//! A loopback Spruce API for tests.
//!
//! It speaks the reads [`crate::api::Client`] makes. It does not prove the live
//! Spruce service. Item `startFrom` is exclusive, matching the stand-in the
//! TypeScript adapter tests against; the change reader also stops at its own
//! checkpoint, so that difference cannot present history as a new message.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::api::Client;

/// Bearer credential the stand-in accepts.
pub const CREDENTIAL: &str = "test-spruce-credential";
/// Organization id `GET /organization` returns.
pub const ORGANIZATION_ID: &str = "entity_FAKE";

/// One message to place in the stand-in.
#[derive(Debug, Clone)]
pub struct Delivery {
    /// Conversation id. Created if it does not exist.
    pub conversation: String,
    /// Item id.
    pub id: String,
    /// Body text.
    pub text: String,
    /// When Spruce created the item.
    pub created_at: String,
    /// When Spruce last changed the item.
    pub modified_at: String,
    /// Conversation `lastMessageAt` after this delivery. Not derived from `created_at`.
    pub activity_at: String,
    /// Voicemail transcription id, when this item has one.
    pub transcription_id: Option<String>,
    /// Transcript text. `None` with a `transcription_id` makes that GET answer 404.
    pub transcription_text: Option<String>,
}

struct StoredConversation {
    id: String,
    last_message_at: String,
}

#[derive(Clone)]
struct StoredItem {
    id: String,
    conversation: String,
    text: String,
    created_at: String,
    modified_at: String,
    transcription_id: Option<String>,
}

struct FakeState {
    page_size: usize,
    /// Every route answers 401, including with the right credential.
    unauthorized: bool,
    /// Every route answers 403, including the organization read.
    forbidden: bool,
    failing: HashSet<String>,
    /// Repeat this item once on the next item page that contains it.
    duplicate: Option<String>,
    conversations: Vec<StoredConversation>,
    items: Vec<StoredItem>,
    transcriptions: HashMap<String, String>,
}

impl FakeState {
    fn fresh() -> Self {
        Self {
            page_size: crate::payload::MAX_PAGE_SIZE,
            unauthorized: false,
            forbidden: false,
            failing: HashSet::new(),
            duplicate: None,
            conversations: Vec::new(),
            items: Vec::new(),
            transcriptions: HashMap::new(),
        }
    }
}

type Shared = Arc<Mutex<FakeState>>;

/// The stand-in server and the handles that change what it returns.
pub struct FakeSpruce {
    /// Version root, ending in `/v1/`.
    pub base: String,
    state: Shared,
}

impl FakeSpruce {
    /// Binds a loopback server and serves it until the process ends.
    pub async fn spawn() -> Self {
        let state = Arc::new(Mutex::new(FakeState::fresh()));
        let app = Router::new()
            .route("/v1/organization", get(organization))
            .route("/v1/internalendpoints", get(endpoints))
            .route("/v1/conversations", get(conversations))
            .route("/v1/conversations/{id}", get(conversation))
            .route("/v1/conversations/{id}/items", get(items))
            .route("/v1/conversationItems/{id}", get(item))
            .route("/v1/transcriptions/{id}", get(transcription))
            .with_state(Arc::clone(&state));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("spruce stand-in port");
        let address = listener.local_addr().expect("spruce stand-in address");
        tokio::spawn(async move {
            axum::serve(listener, app).await.expect("spruce stand-in");
        });
        Self {
            base: format!("http://{address}/v1/"),
            state,
        }
    }

    /// A client aimed at this stand-in, with its credential.
    pub fn client(&self) -> Client {
        Client::with_base(CREDENTIAL, &self.base).expect("stand-in base")
    }

    /// Adds or replaces an item and sets that conversation's last activity.
    pub fn deliver(&self, delivery: Delivery) {
        let mut state = self.state.lock().expect("spruce stand-in lock");
        if let Some(text) = delivery.transcription_text.clone()
            && let Some(id) = &delivery.transcription_id
        {
            state.transcriptions.insert(id.clone(), text);
        }
        if let Some(conversation) = state
            .conversations
            .iter_mut()
            .find(|conversation| conversation.id == delivery.conversation)
        {
            conversation.last_message_at = delivery.activity_at.clone();
        } else {
            state.conversations.push(StoredConversation {
                id: delivery.conversation.clone(),
                last_message_at: delivery.activity_at,
            });
        }
        let stored = StoredItem {
            id: delivery.id.clone(),
            conversation: delivery.conversation,
            text: delivery.text,
            created_at: delivery.created_at,
            modified_at: delivery.modified_at,
            transcription_id: delivery.transcription_id,
        };
        if let Some(existing) = state.items.iter_mut().find(|item| item.id == stored.id) {
            *existing = stored;
        } else {
            state.items.push(stored);
        }
    }

    /// Changes an item's text and `modifiedAt`. Does not move `lastMessageAt`.
    pub fn edit(&self, id: &str, text: &str, modified_at: &str) {
        let mut state = self.state.lock().expect("spruce stand-in lock");
        let item = state
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .expect("item to edit");
        item.text = text.to_owned();
        item.modified_at = modified_at.to_owned();
    }

    /// Removes an item. The conversation stays, and `lastMessageAt` stays.
    pub fn delete(&self, id: &str) {
        let mut state = self.state.lock().expect("spruce stand-in lock");
        state.items.retain(|item| item.id != id);
    }

    /// The next item page that contains `id` lists that item twice.
    pub fn show_twice(&self, id: &str) {
        self.state.lock().expect("spruce stand-in lock").duplicate = Some(id.to_owned());
    }

    /// Reads of this conversation answer 500.
    pub fn fail(&self, conversation: &str) {
        self.state
            .lock()
            .expect("spruce stand-in lock")
            .failing
            .insert(conversation.to_owned());
    }

    /// Every route answers 401.
    pub fn reject_unauthorized(&self) {
        self.state
            .lock()
            .expect("spruce stand-in lock")
            .unauthorized = true;
    }

    /// Every route answers 403, including `GET /organization`.
    pub fn forbid(&self) {
        self.state.lock().expect("spruce stand-in lock").forbidden = true;
    }

    /// Caps how many conversations or items one page returns.
    pub fn set_page_size(&self, page_size: usize) {
        self.state.lock().expect("spruce stand-in lock").page_size = page_size.max(1);
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PageQuery {
    page_size: Option<u32>,
    start_from: Option<String>,
    pagination_token: Option<String>,
    order: Option<String>,
}

fn rejected(state: &FakeState, headers: &HeaderMap) -> Option<StatusCode> {
    if state.unauthorized {
        return Some(StatusCode::UNAUTHORIZED);
    }
    if state.forbidden {
        return Some(StatusCode::FORBIDDEN);
    }
    let presented = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    (presented != Some(&format!("Bearer {CREDENTIAL}"))).then_some(StatusCode::UNAUTHORIZED)
}

fn deny(state: &Shared, headers: &HeaderMap) -> Option<Response> {
    let state = state.lock().expect("spruce stand-in lock");
    rejected(&state, headers).map(|status| (status, Json(json!({}))).into_response())
}

async fn organization(State(state): State<Shared>, headers: HeaderMap) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    Json(json!({"organization": {"id": ORGANIZATION_ID, "name": "Fake Practice"}})).into_response()
}

async fn endpoints(State(state): State<Shared>, headers: HeaderMap) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    Json(json!({"internalEndpoints": [{"endpoint": {"channel": "phone", "rawValue": "+12065551000", "displayValue": "+1 206-555-1000"}}]})).into_response()
}

async fn conversations(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(query): Query<PageQuery>,
) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    let state = state.lock().expect("spruce stand-in lock");
    let mut rows: Vec<&StoredConversation> = state.conversations.iter().collect();
    if let Some(start) = query.start_from.as_deref() {
        rows.retain(|conversation| at_or_after(&conversation.last_message_at, start));
    }
    rows.sort_by(|left, right| {
        instant(&right.last_message_at).cmp(&instant(&left.last_message_at))
    });
    let (page, has_more, token) = slice(rows, &state, &query);
    let conversations: Vec<Value> = page
        .iter()
        .map(|conversation| conversation_json(conversation))
        .collect();
    Json(json!({
        "conversations": conversations,
        "hasMore": has_more,
        "paginationToken": token,
    }))
    .into_response()
}

async fn conversation(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    let state = state.lock().expect("spruce stand-in lock");
    if state.failing.contains(&id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response();
    }
    let Some(conversation) = state
        .conversations
        .iter()
        .find(|conversation| conversation.id == id)
    else {
        return (StatusCode::NOT_FOUND, Json(json!({}))).into_response();
    };
    Json(json!({"conversation": conversation_json(conversation)})).into_response()
}

async fn items(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Query(query): Query<PageQuery>,
) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    let mut state = state.lock().expect("spruce stand-in lock");
    if state.failing.contains(&id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response();
    }
    if !state
        .conversations
        .iter()
        .any(|conversation| conversation.id == id)
    {
        return (StatusCode::NOT_FOUND, Json(json!({}))).into_response();
    }
    let mut rows: Vec<StoredItem> = state
        .items
        .iter()
        .filter(|item| item.conversation == id)
        .cloned()
        .collect();
    if let Some(start) = query.start_from.as_deref() {
        rows.retain(|item| after(&item.created_at, start));
    }
    let newest_first = query.order.as_deref() != Some("oldest_first");
    rows.sort_by(|left, right| {
        let order = instant(&left.created_at).cmp(&instant(&right.created_at));
        if newest_first { order.reverse() } else { order }
    });
    if let Some(duplicate) = state.duplicate.clone()
        && let Some(position) = rows.iter().position(|item| item.id == duplicate)
    {
        let copy = rows[position].clone();
        rows.insert(position, copy);
        state.duplicate = None;
    }
    let (page, has_more, token) = slice(rows, &state, &query);
    let conversation_items: Vec<Value> = page.iter().map(item_json).collect();
    Json(json!({
        "conversationItems": conversation_items,
        "hasMore": has_more,
        "paginationToken": token,
    }))
    .into_response()
}

async fn item(State(state): State<Shared>, headers: HeaderMap, Path(id): Path<String>) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    let state = state.lock().expect("spruce stand-in lock");
    let Some(item) = state.items.iter().find(|item| item.id == id) else {
        return (StatusCode::NOT_FOUND, Json(json!({}))).into_response();
    };
    Json(json!({"conversationItem": item_json(item)})).into_response()
}

async fn transcription(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    let state = state.lock().expect("spruce stand-in lock");
    let Some(text) = state.transcriptions.get(&id) else {
        return (StatusCode::NOT_FOUND, Json(json!({}))).into_response();
    };
    Json(json!({"transcription": {"id": id, "status": "completed", "text": text}})).into_response()
}

fn conversation_json(conversation: &StoredConversation) -> Value {
    json!({
        "id": conversation.id,
        "type": "secure",
        "lastMessageAt": conversation.last_message_at,
        "externalParticipants": [{"displayName": "Synthetic", "endpoint": {"channel": "phone", "rawValue": "+12065550100"}}],
        "internalEndpoint": {"channel": "phone", "rawValue": "+12065551000"},
    })
}

fn item_json(item: &StoredItem) -> Value {
    let mut body = json!({
        "id": item.id,
        "conversationId": item.conversation,
        "direction": "inbound",
        "isInternalNote": false,
        "author": {"displayName": "Synthetic"},
        "createdAt": item.created_at,
        "modifiedAt": item.modified_at,
        "appURL": format!("https://app.sprucehealth.com/org/{ORGANIZATION_ID}/thread/{}/message/{}", item.conversation, item.id),
        "text": item.text,
    });
    if let Some(id) = &item.transcription_id {
        body["event"] = json!({
            "type": "voicemail",
            "data": {
                "signedUrl": "https://signed.example/drop",
                "recordings": [{"signedUrl": "https://signed.example/recording"}],
                "voicemail": {"mimetype": "audio/mp4", "transcription": {"id": id, "status": "completed"}}
            }
        });
    }
    body
}

fn slice<T>(rows: Vec<T>, state: &FakeState, query: &PageQuery) -> (Vec<T>, bool, Option<String>) {
    let requested = query.page_size.unwrap_or(state.page_size as u32) as usize;
    let size = requested.min(state.page_size).max(1);
    let start = query
        .pagination_token
        .as_deref()
        .and_then(|token| token.parse::<usize>().ok())
        .unwrap_or(0);
    let len = rows.len();
    let end = (start + size).min(len);
    let page = rows
        .into_iter()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect();
    let has_more = end < len;
    (page, has_more, has_more.then(|| end.to_string()))
}

fn instant(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

fn at_or_after(value: &str, start: &str) -> bool {
    match (instant(value), instant(start)) {
        (Some(value), Some(start)) => value >= start,
        _ => true,
    }
}

fn after(value: &str, start: &str) -> bool {
    match (instant(value), instant(start)) {
        (Some(value), Some(start)) => value > start,
        _ => true,
    }
}
