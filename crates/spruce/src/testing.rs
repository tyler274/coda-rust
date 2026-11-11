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
    /// Spruce transcription status. Defaults to `completed` when text is present.
    pub transcription_status: Option<String>,
    /// Summary text, when the transcription resource includes one.
    pub transcription_summary: Option<String>,
}

struct StoredTranscription {
    status: String,
    text: String,
    summary: Option<String>,
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
    direction: &'static str,
    deleted: bool,
    /// The item's content is not a shape the reader can decode.
    unreadable: bool,
    /// File names attached to the item, served from `files`.
    files: Vec<String>,
}

/// A saved contact the stand-in serves from `GET /contacts/{id}`.
#[derive(Debug, Clone)]
pub struct FakeContact {
    /// Contact id.
    pub id: String,
    /// Display name.
    pub display_name: String,
    /// Phone numbers, E.164.
    pub phone_numbers: Vec<String>,
    /// Date of birth, `YYYY-MM-DD`.
    pub date_of_birth: Option<String>,
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
    transcriptions: HashMap<String, StoredTranscription>,
    contacts: HashMap<String, FakeContact>,
    /// Associated contact ids per conversation.
    associations: HashMap<String, Vec<String>>,
    /// File bytes by name.
    files: HashMap<String, Vec<u8>>,
    /// Bumped by [`FakeSpruce::expire_links`]; a link signed under an older
    /// generation answers 403.
    link_generation: u64,
    /// Every path the stand-in was asked for, in order.
    requests: Vec<String>,
    /// `http://host:port`, for signing file links.
    origin: String,
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
            contacts: HashMap::new(),
            associations: HashMap::new(),
            files: HashMap::new(),
            link_generation: 0,
            requests: Vec::new(),
            origin: String::new(),
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
            .route("/v1/contacts/{id}", get(contact))
            .route("/files/{generation}/{name}", get(file))
            .layer(axum::middleware::from_fn_with_state(
                Arc::clone(&state),
                record_request,
            ))
            .with_state(Arc::clone(&state));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("spruce stand-in port");
        let address = listener.local_addr().expect("spruce stand-in address");
        state.lock().expect("spruce stand-in lock").origin = format!("http://{address}");
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
            state.transcriptions.insert(
                id.clone(),
                StoredTranscription {
                    status: delivery
                        .transcription_status
                        .clone()
                        .unwrap_or_else(|| "completed".into()),
                    text,
                    summary: delivery.transcription_summary.clone(),
                },
            );
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
            direction: "inbound",
            deleted: false,
            unreadable: false,
            files: Vec::new(),
        };
        if let Some(existing) = state.items.iter_mut().find(|item| item.id == stored.id) {
            *existing = stored;
        } else {
            state.items.push(stored);
        }
    }

    /// Marks an item as sent by the practice (`outbound`).
    pub fn mark_outbound(&self, id: &str) {
        self.with_item(id, |item| item.direction = "outbound");
    }

    /// Spruce flags the item deleted and bumps `modifiedAt`. The item stays listed.
    pub fn flag_deleted(&self, id: &str, modified_at: &str) {
        self.with_item(id, |item| {
            item.deleted = true;
            item.modified_at = modified_at.to_owned();
        });
    }

    /// The item's content stops decoding; its id and app link still do.
    pub fn make_unreadable(&self, id: &str) {
        self.with_item(id, |item| item.unreadable = true);
    }

    /// Attaches a file to an item. Its link is signed under the current
    /// generation and stops working after [`Self::expire_links`].
    pub fn attach(&self, id: &str, name: &str, bytes: &[u8]) {
        let mut state = self.state.lock().expect("spruce stand-in lock");
        state.files.insert(name.to_owned(), bytes.to_vec());
        let item = state
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .expect("item to attach to");
        item.files.push(name.to_owned());
    }

    /// Every link handed out so far answers 403 from now on.
    pub fn expire_links(&self) {
        self.state
            .lock()
            .expect("spruce stand-in lock")
            .link_generation += 1;
    }

    /// Serves a contact and associates it with a conversation.
    pub fn add_contact(&self, conversation: &str, contact: FakeContact) {
        let mut state = self.state.lock().expect("spruce stand-in lock");
        state
            .associations
            .entry(conversation.to_owned())
            .or_default()
            .push(contact.id.clone());
        state.contacts.insert(contact.id.clone(), contact);
    }

    /// Every request path so far, oldest first.
    pub fn requests(&self) -> Vec<String> {
        self.state
            .lock()
            .expect("spruce stand-in lock")
            .requests
            .clone()
    }

    /// Reads answer normally again after [`Self::fail`], [`Self::forbid`] or
    /// [`Self::reject_unauthorized`].
    pub fn recover(&self) {
        let mut state = self.state.lock().expect("spruce stand-in lock");
        state.failing.clear();
        state.forbidden = false;
        state.unauthorized = false;
    }

    fn with_item(&self, id: &str, change: impl FnOnce(&mut StoredItem)) {
        let mut state = self.state.lock().expect("spruce stand-in lock");
        let item = state
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .expect("item in the stand-in");
        change(item);
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
        .map(|conversation| conversation_json(&state, conversation))
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
    Json(json!({"conversation": conversation_json(&state, conversation)})).into_response()
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
    let conversation_items: Vec<Value> = page.iter().map(|item| item_json(&state, item)).collect();
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
    Json(json!({"conversationItem": item_json(&state, item)})).into_response()
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
    let Some(transcription) = state.transcriptions.get(&id) else {
        return (StatusCode::NOT_FOUND, Json(json!({}))).into_response();
    };
    Json(json!({
        "transcription": {
            "id": id,
            "status": transcription.status,
            "text": transcription.text,
            "summarization": transcription.summary.as_ref().map(|text| json!({"text": text})),
        }
    }))
    .into_response()
}

async fn contact(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    if let Some(response) = deny(&state, &headers) {
        return response;
    }
    let state = state.lock().expect("spruce stand-in lock");
    let Some(contact) = state.contacts.get(&id) else {
        return (StatusCode::NOT_FOUND, Json(json!({}))).into_response();
    };
    let phones: Vec<Value> = contact
        .phone_numbers
        .iter()
        .map(|number| json!({"value": number, "label": "mobile"}))
        .collect();
    Json(json!({
        "id": contact.id,
        "object": "contact",
        "category": "patient",
        "displayName": contact.display_name,
        "dateOfBirth": contact.date_of_birth,
        "phoneNumbers": phones,
        "emailAddresses": [],
        "integrationLinks": [],
    }))
    .into_response()
}

/// A signed file link. It needs no bearer credential, like Spruce's.
async fn file(
    State(state): State<Shared>,
    Path((generation, name)): Path<(u64, String)>,
) -> Response {
    let state = state.lock().expect("spruce stand-in lock");
    if generation != state.link_generation {
        return (StatusCode::FORBIDDEN, "expired").into_response();
    }
    match state.files.get(&name) {
        Some(bytes) => bytes.clone().into_response(),
        None => (StatusCode::NOT_FOUND, "missing").into_response(),
    }
}

async fn record_request(
    State(state): State<Shared>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    state
        .lock()
        .expect("spruce stand-in lock")
        .requests
        .push(request.uri().path().to_owned());
    next.run(request).await
}

fn conversation_json(state: &FakeState, conversation: &StoredConversation) -> Value {
    let associated = state
        .associations
        .get(&conversation.id)
        .cloned()
        .unwrap_or_default();
    json!({
        "id": conversation.id,
        "type": "secure",
        "lastMessageAt": conversation.last_message_at,
        "externalParticipants": [{"displayName": "Synthetic", "contact": associated.first(), "endpoint": {"channel": "phone", "rawValue": "+12065550100"}}],
        "internalEndpoint": {"channel": "phone", "rawValue": "+12065551000"},
        "associatedContactIds": associated,
    })
}

fn item_json(state: &FakeState, item: &StoredItem) -> Value {
    let app_url = format!(
        "https://app.sprucehealth.com/org/{ORGANIZATION_ID}/thread/{}/message/{}",
        item.conversation, item.id
    );
    if item.unreadable {
        return json!({
            "id": item.id,
            "conversationId": item.conversation,
            "direction": item.direction,
            "createdAt": item.created_at,
            "modifiedAt": item.modified_at,
            "appURL": app_url,
            "text": {"unexpected": true},
        });
    }
    let attachments: Vec<Value> = item
        .files
        .iter()
        .map(|name| {
            json!({
                "type": "document",
                "title": name,
                "data": {
                    "mimetype": "application/pdf",
                    "name": name,
                    "signedUrl": {
                        "url": format!("{}/files/{}/{name}", state.origin, state.link_generation),
                        "expiresAt": "2026-12-31T00:00:00Z",
                    },
                },
            })
        })
        .collect();
    let mut body = json!({
        "id": item.id,
        "conversationId": item.conversation,
        "direction": item.direction,
        "isInternalNote": false,
        "author": {"displayName": "Synthetic"},
        "createdAt": item.created_at,
        "modifiedAt": item.modified_at,
        "appURL": app_url,
        "text": item.text,
        "attachments": attachments,
        "deleted": item.deleted,
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
