//! Spruce wire values, decoded one item at a time.
//!
//! Spruce adds enum values without notice, so conversation type, endpoint
//! channel and event type stay strings. A page never fails because one item
//! failed: that item is [`ItemEntry::Unreadable`] when its id was readable and
//! [`ItemEntry::Malformed`] when nothing can address it.
//!
//! Call recordings and voicemail audio are dropped, and so is every signed URL
//! except an attachment's, which is kept only as a [`FileLink`]. A link cannot
//! be serialized and is not part of [`Item::version`], because Spruce signs it
//! again on every read. Voicemail stays as a transcription id; the text is
//! filled in by the change reader.

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// Largest page Spruce documents, and the most entries one decoded page holds.
pub const MAX_PAGE_SIZE: usize = 200;

/// An id Spruce gives us and that we can put back into a path.
///
/// Non-empty, at most 1024 bytes, and free of whitespace and of `/`, `?` and
/// `#`. Every id this module returns passes this, so the client can address it.
pub fn valid_id(value: &str) -> bool {
    let length = value.len();
    (1..=1024).contains(&length)
        && !value
            .chars()
            .any(|character| character.is_whitespace() || matches!(character, '/' | '?' | '#'))
}

/// The organization a credential belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    /// Spruce's organization id.
    pub id: String,
    /// Display name, when Spruce sent one.
    pub name: Option<String>,
}

/// One of the organization's own phone numbers, addresses or Spruce links.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    /// Spruce's channel string (`phone`, `fax`, `email`, `secure`, or a later one).
    pub channel: Option<String>,
    /// The stable address: E.164 digits, an email address, or a Spruce link.
    pub raw_value: Option<String>,
    /// Spruce's display form of `raw_value`.
    pub display_value: Option<String>,
}

/// Someone outside the practice on a conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    /// Name Spruce showed, when it sent one.
    pub display_name: Option<String>,
    /// Saved-contact id. Absent when this participant is not a saved contact.
    pub contact: Option<String>,
    /// The endpoint they used.
    pub endpoint: Option<Endpoint>,
}

/// A conversation as the list and the single-conversation read return it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversation {
    /// Spruce conversation id.
    pub id: String,
    /// Spruce's conversation type string.
    pub kind: Option<String>,
    /// When the conversation last had a message, as Spruce sent it.
    pub last_message_at: Option<String>,
    /// Participants outside the practice.
    pub external_participants: Vec<Participant>,
    /// The practice endpoint the conversation sits on.
    pub internal_endpoint: Option<Endpoint>,
    /// Contacts the practice associated with the conversation, which may not
    /// be participants (a parent writing for a child).
    pub associated_contact_ids: Vec<String>,
}

/// A saved contact from `GET /contacts/{id}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    /// Spruce contact id (`entity_…`).
    pub id: String,
    /// Spruce's category string (`patient`, `clinic`, or a later one).
    pub category: Option<String>,
    /// Name Spruce shows.
    pub display_name: Option<String>,
    /// Given name, when the practice entered one.
    pub given_name: Option<String>,
    /// Family name, when the practice entered one.
    pub family_name: Option<String>,
    /// Date of birth as Spruce sent it (`YYYY-MM-DD`).
    pub date_of_birth: Option<String>,
    /// Phone numbers, E.164 as Spruce sent them.
    pub phone_numbers: Vec<String>,
    /// Email addresses.
    pub email_addresses: Vec<String>,
    /// Links to records in other systems, such as an EHR.
    pub integration_links: Vec<IntegrationLink>,
}

/// A contact's link to a record in another system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationLink {
    /// Spruce's link type string (`custom`, or an EHR name).
    pub kind: String,
    /// The other system's record id.
    pub external_id: String,
}

/// One conversation in a list page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationEntry {
    /// The conversation decoded.
    Conversation(Conversation),
    /// It could not be decoded. `id` is set only when that much was readable.
    Malformed {
        /// Spruce's id, when the rest of the conversation was not readable.
        id: Option<String>,
    },
}

/// Fields every item carries before its content, enough to record it and open it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    /// Spruce item id.
    pub id: String,
    /// Conversation the item belongs to, when Spruce sent it.
    pub conversation_id: Option<String>,
    /// `inbound` or `outbound`, or a later Spruce value.
    pub direction: Option<String>,
    /// Spruce's internal-note flag, when it was a boolean.
    pub is_internal_note: Option<bool>,
    /// Author display name.
    pub author_name: Option<String>,
    /// Author entity id. Spruce sets it for teammates and for a saved contact
    /// writing over secure messaging, and omits it for SMS, voice, email and fax.
    pub author_id: Option<String>,
    /// When Spruce created the item.
    pub created_at: Option<String>,
    /// When Spruce last changed the item. Part of [`Item::version`].
    pub modified_at: Option<String>,
    /// Link to the item in Spruce's app.
    pub app_url: Option<String>,
}

/// A file or card attached to an item. Bytes are fetched only through
/// [`crate::api::Client::download`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    /// Spruce's attachment type string.
    pub kind: String,
    /// Title Spruce sent.
    pub title: Option<String>,
    /// MIME type of the descriptor, when the attachment has one.
    pub mimetype: Option<String>,
    /// File name Spruce sent.
    pub name: Option<String>,
    /// Where this read's bytes are, when the attachment is a file.
    pub link: Option<FileLink>,
}

/// A signed download link from one read.
///
/// It expires, so fetch the bytes while the read is current. It has no
/// serialized form and its `Debug` hides the URL, so it cannot be stored or
/// logged by accident.
#[derive(Clone, PartialEq, Eq)]
pub struct FileLink {
    url: String,
    expires_at: Option<String>,
}

impl FileLink {
    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    /// When Spruce said the link expires.
    pub fn expires_at(&self) -> Option<&str> {
        self.expires_at.as_deref()
    }
}

impl std::fmt::Debug for FileLink {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FileLink")
            .field("expires_at", &self.expires_at)
            .finish_non_exhaustive()
    }
}

/// A button on an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
    /// Spruce button id.
    pub id: String,
    /// Button label.
    pub text: String,
    /// Spruce's button state string.
    pub state: String,
}

/// Audio on a voicemail. The recording URL is not kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Voicemail {
    /// MIME type Spruce sent.
    pub mimetype: Option<String>,
    /// Transcription job id, fetched separately.
    pub transcription_id: Option<String>,
}

/// A call or other Spruce event.
///
/// `data` is Spruce's event fields with `signedUrl` removed at every depth,
/// and with `recordings` and `voicemail` removed. Voicemail is [`Voicemail`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// Spruce's event type string.
    pub kind: String,
    /// Event fields that stay meaningful across reads.
    pub data: Value,
    /// Voicemail audio, when this event carried one.
    pub voicemail: Option<Voicemail>,
}

/// What a transcription fetch became. The id is the one the item named.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranscriptionState {
    /// Spruce finished the transcript. `pending`, `failed`, and `noSpeech` are not this.
    Ready {
        /// Transcription id.
        id: String,
        /// Transcript text.
        text: String,
        /// Summary text, when Spruce sent one.
        summary: Option<String>,
    },
    /// Spruce returned a transcription that is not a finished body.
    ///
    /// `status` is Spruce's string (`pending`, `failed`, `noSpeech`, or anything else
    /// that is not `completed`). The product can read the conversation again while
    /// it is still `pending`.
    Unfinished {
        /// Transcription id.
        id: String,
        /// Spruce's status string.
        status: String,
        /// Transcript text so far. Empty when Spruce has not written one.
        text: String,
        /// Summary text, when Spruce sent one.
        summary: Option<String>,
    },
    /// Spruce has no transcript. Transcriptions from before November 2025 answer 404.
    NotFound {
        /// Transcription id.
        id: String,
    },
    /// The transcript could not be read this pass. The item itself was readable.
    Unavailable {
        /// Transcription id.
        id: String,
    },
}

/// One conversation item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    /// Identity and timestamps.
    pub envelope: Envelope,
    /// Body text. Empty when Spruce sent none.
    pub text: String,
    /// Attachments, without their bytes.
    pub attachments: Vec<Attachment>,
    /// Buttons.
    pub buttons: Vec<Button>,
    /// Call or other event.
    pub event: Option<Event>,
    /// Spruce marked the item deleted (`deleted` or `isDeleted`).
    ///
    /// Absence of the flag is not a deletion. An item that disappears from a
    /// later read is missing, and only this flag says Spruce deleted it.
    pub deleted: bool,
    /// Transcript, once the reader has fetched it. `None` before that fetch.
    pub transcription: Option<TranscriptionState>,
}

impl Item {
    /// Spruce item id.
    pub fn id(&self) -> &str {
        &self.envelope.id
    }

    /// Identity of this item's content for de-duplication.
    ///
    /// `modifiedAt` plus a digest of the text, attachments, buttons, event
    /// fields and transcript. A signed URL is not part of it, so a repeated
    /// read of an unchanged item is the same version. A changed body or a
    /// changed `modifiedAt` is a new version.
    pub fn version(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.text.as_bytes());
        hasher.update([0xff]);
        hasher.update([u8::from(self.deleted)]);
        hasher.update(self.envelope.direction.as_deref().unwrap_or(""));
        hasher.update([0xff]);
        for attachment in &self.attachments {
            hasher.update(attachment.kind.as_bytes());
            hasher.update([0]);
            hasher.update(attachment.title.as_deref().unwrap_or(""));
            hasher.update([0]);
            hasher.update(attachment.name.as_deref().unwrap_or(""));
            hasher.update([0]);
            hasher.update(attachment.mimetype.as_deref().unwrap_or(""));
            hasher.update([0xff]);
        }
        for button in &self.buttons {
            hasher.update(button.id.as_bytes());
            hasher.update([0]);
            hasher.update(button.text.as_bytes());
            hasher.update([0]);
            hasher.update(button.state.as_bytes());
            hasher.update([0xff]);
        }
        if let Some(event) = &self.event {
            hasher.update(event.kind.as_bytes());
            hasher.update(canonical_json(&event.data).as_bytes());
            if let Some(voicemail) = &event.voicemail {
                hasher.update(voicemail.transcription_id.as_deref().unwrap_or(""));
            }
        }
        match &self.transcription {
            Some(TranscriptionState::Ready { text, summary, .. }) => {
                hasher.update([1]);
                hasher.update(b"completed");
                hasher.update(text.as_bytes());
                hasher.update([0]);
                hasher.update(summary.as_deref().unwrap_or(""));
            }
            Some(TranscriptionState::Unfinished {
                status,
                text,
                summary,
                ..
            }) => {
                hasher.update([4]);
                hasher.update(status.as_bytes());
                hasher.update([0]);
                hasher.update(text.as_bytes());
                hasher.update([0]);
                hasher.update(summary.as_deref().unwrap_or(""));
            }
            Some(TranscriptionState::NotFound { .. }) => hasher.update([2]),
            Some(TranscriptionState::Unavailable { .. }) => hasher.update([3]),
            None => hasher.update([0]),
        }
        let digest = hex(&hasher.finalize());
        format!(
            "{}:{digest}",
            self.envelope.modified_at.as_deref().unwrap_or("")
        )
    }
}

/// A transcription's text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transcription {
    /// Transcription id.
    pub id: String,
    /// Spruce's status string.
    pub status: String,
    /// Transcript text. Empty when Spruce has not finished it.
    pub text: String,
    /// Summary text, when Spruce sent a summary.
    pub summary: Option<String>,
}

/// One item in a conversation's item page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemEntry {
    /// The item decoded.
    Item(Box<Item>),
    /// The id and envelope decoded; the content did not.
    Unreadable(Envelope),
    /// No id, so nothing can address it.
    Malformed,
}

/// One page of a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    /// Entries on this page, in Spruce's order.
    pub entries: Vec<T>,
    /// Spruce has another page.
    pub has_more: bool,
    /// Token for the next page. Present when `has_more` is set.
    pub pagination_token: Option<String>,
}

/// Organization from `GET /organization`.
pub fn organization(value: &Value) -> Option<Organization> {
    let row = record(record(value)?.get("organization")?)?;
    Some(Organization {
        id: identity(row.get("id")?)?,
        name: optional_text(row.get("name")),
    })
}

/// Endpoints from `GET /internalendpoints`. One bad entry is skipped.
pub fn internal_endpoints(value: &Value) -> Option<Vec<Endpoint>> {
    let rows = array(record(value)?.get("internalEndpoints")?, 1_000)?;
    let mut endpoints = Vec::new();
    for entry in rows {
        if let Some(endpoint) = record(entry)
            .and_then(|row| row.get("endpoint"))
            .and_then(endpoint)
        {
            endpoints.push(endpoint);
        }
    }
    Some(endpoints)
}

/// Conversations from `GET /conversations`.
pub fn conversation_page(value: &Value) -> Option<Page<ConversationEntry>> {
    let row = record(value)?;
    let (has_more, pagination_token) = paging(row)?;
    let entries = array(row.get("conversations")?, MAX_PAGE_SIZE)?
        .iter()
        .map(conversation_entry)
        .collect();
    Some(Page {
        entries,
        has_more,
        pagination_token,
    })
}

/// One conversation from `GET /conversations/{id}`.
pub fn conversation_response(value: &Value) -> Option<Conversation> {
    conversation(record(value)?.get("conversation")?)
}

/// Items from `GET /conversations/{id}/items`.
pub fn item_page(value: &Value) -> Option<Page<ItemEntry>> {
    let row = record(value)?;
    let (has_more, pagination_token) = paging(row)?;
    let entries = array(row.get("conversationItems")?, MAX_PAGE_SIZE)?
        .iter()
        .map(item_entry)
        .collect();
    Some(Page {
        entries,
        has_more,
        pagination_token,
    })
}

/// One item from `GET /conversationItems/{id}`.
pub fn item_response(value: &Value) -> Option<ItemEntry> {
    Some(item_entry(record(value)?.get("conversationItem")?))
}

/// A transcript from `GET /transcriptions/{id}`.
pub fn transcription(value: &Value) -> Option<Transcription> {
    let row = record(record(value)?.get("transcription")?)?;
    let summary = row
        .get("summarization")
        .filter(|value| !value.is_null())
        .and_then(record)
        .and_then(|summary| short(summary.get("text")?));
    Some(Transcription {
        id: identity(row.get("id")?)?,
        status: short(row.get("status")?)?,
        text: body(row.get("text").unwrap_or(&Value::String(String::new())))?,
        summary,
    })
}

fn conversation_entry(value: &Value) -> ConversationEntry {
    if let Some(conversation) = conversation(value) {
        return ConversationEntry::Conversation(conversation);
    }
    ConversationEntry::Malformed {
        id: record(value)
            .and_then(|row| row.get("id"))
            .and_then(identity),
    }
}

fn conversation(value: &Value) -> Option<Conversation> {
    let row = record(value)?;
    let participants = match row.get("externalParticipants") {
        None | Some(Value::Null) => Vec::new(),
        Some(value) => array(value, 1_000)?
            .iter()
            .filter_map(participant)
            .collect(),
    };
    Some(Conversation {
        id: identity(row.get("id")?)?,
        kind: optional_text(row.get("type")),
        last_message_at: optional_text(row.get("lastMessageAt")),
        external_participants: participants,
        internal_endpoint: row
            .get("internalEndpoint")
            .filter(|value| !value.is_null())
            .and_then(endpoint),
        associated_contact_ids: ids(row.get("associatedContactIds")),
    })
}

/// A contact from `GET /contacts/{id}`. Spruce answers with the bare contact.
pub fn contact(value: &Value) -> Option<Contact> {
    let row = record(value)?;
    let values = |key: &str| -> Vec<String> {
        row.get(key)
            .and_then(|value| array(value, 1_000))
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| match entry {
                        Value::String(_) => optional_text(Some(entry)),
                        _ => optional_text(record(entry)?.get("value")),
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let integration_links = row
        .get("integrationLinks")
        .and_then(|value| array(value, 1_000))
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let link = record(entry)?;
                    Some(IntegrationLink {
                        kind: short(link.get("type")?)?,
                        external_id: short(link.get("externalId")?)?,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    Some(Contact {
        id: identity(row.get("id")?)?,
        category: optional_text(row.get("category")),
        display_name: optional_text(row.get("displayName")),
        given_name: optional_text(row.get("givenName")),
        family_name: optional_text(row.get("familyName")),
        date_of_birth: optional_text(row.get("dateOfBirth")),
        phone_numbers: values("phoneNumbers"),
        email_addresses: values("emailAddresses"),
        integration_links,
    })
}

/// Ids in a list of strings. An entry that is not an id is skipped.
fn ids(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(|value| array(value, 1_000))
        .map(|entries| entries.iter().filter_map(identity).collect())
        .unwrap_or_default()
}

fn participant(value: &Value) -> Option<Participant> {
    let row = record(value)?;
    Some(Participant {
        display_name: optional_text(row.get("displayName")),
        contact: optional_text(row.get("contact")),
        endpoint: row
            .get("endpoint")
            .filter(|value| !value.is_null())
            .and_then(endpoint),
    })
}

fn endpoint(value: &Value) -> Option<Endpoint> {
    let row = record(value)?;
    Some(Endpoint {
        channel: optional_text(row.get("channel")),
        raw_value: raw_endpoint(row.get("rawValue")),
        display_value: optional_text(row.get("displayValue")),
    })
}

fn item_entry(value: &Value) -> ItemEntry {
    let Some(envelope) = envelope(value) else {
        return ItemEntry::Malformed;
    };
    match item_content(value, envelope.clone()) {
        Some(item) => ItemEntry::Item(Box::new(item)),
        None => ItemEntry::Unreadable(envelope),
    }
}

fn envelope(value: &Value) -> Option<Envelope> {
    let row = record(value)?;
    let author = row
        .get("author")
        .filter(|value| !value.is_null())
        .and_then(record);
    Some(Envelope {
        id: identity(row.get("id")?)?,
        conversation_id: optional_text(row.get("conversationId")),
        direction: optional_text(row.get("direction")),
        is_internal_note: row.get("isInternalNote").and_then(Value::as_bool),
        author_name: author.and_then(|author| optional_text(author.get("displayName"))),
        author_id: author.and_then(|author| author.get("id").and_then(identity)),
        created_at: optional_text(row.get("createdAt")),
        modified_at: optional_text(row.get("modifiedAt")),
        app_url: optional_text(row.get("appURL")),
    })
}

fn item_content(value: &Value, envelope: Envelope) -> Option<Item> {
    let row = record(value)?;
    let attachments = match row.get("attachments") {
        None | Some(Value::Null) => Vec::new(),
        Some(value) => array(value, 256)?
            .iter()
            .map(attachment)
            .collect::<Option<_>>()?,
    };
    let buttons = match row.get("buttons") {
        None | Some(Value::Null) => Vec::new(),
        Some(value) => array(value, 64)?
            .iter()
            .map(button)
            .collect::<Option<_>>()?,
    };
    Some(Item {
        envelope,
        text: match row.get("text") {
            None | Some(Value::Null) => String::new(),
            Some(value) => body(value)?,
        },
        attachments,
        buttons,
        event: match row.get("event") {
            None | Some(Value::Null) => None,
            Some(value) => Some(event(value)?),
        },
        deleted: deleted_flag(row),
        transcription: None,
    })
}

fn deleted_flag(row: &Map<String, Value>) -> bool {
    row.get("deleted")
        .or_else(|| row.get("isDeleted"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn attachment(value: &Value) -> Option<Attachment> {
    let row = record(value)?;
    let data = row
        .get("data")
        .filter(|value| !value.is_null())
        .and_then(record);
    Some(Attachment {
        kind: short(row.get("type")?)?,
        title: optional_text(row.get("title")),
        mimetype: data
            .as_ref()
            .and_then(|data| optional_text(data.get("mimetype"))),
        name: data
            .as_ref()
            .and_then(|data| optional_text(data.get("name"))),
        link: data.and_then(|data| file_link(data.get("signedUrl")?)),
    })
}

/// Spruce documents `signedUrl` as `{url, expiresAt}`. A bare string is read as the URL.
fn file_link(value: &Value) -> Option<FileLink> {
    let (url, expires_at) = match value {
        Value::String(url) => (url.clone(), None),
        Value::Object(row) => (
            row.get("url")?.as_str()?.to_owned(),
            optional_text(row.get("expiresAt")),
        ),
        _ => return None,
    };
    (url.starts_with("https://") || url.starts_with("http://"))
        .then_some(FileLink { url, expires_at })
}

fn button(value: &Value) -> Option<Button> {
    let row = record(value)?;
    Some(Button {
        id: short(row.get("id")?)?,
        text: body(row.get("text")?)?,
        state: short(row.get("state")?)?,
    })
}

fn event(value: &Value) -> Option<Event> {
    let row = record(value)?;
    let data = match row.get("data") {
        None | Some(Value::Null) => Map::new(),
        Some(value) => record(value)?.clone(),
    };
    let voicemail = match data.get("voicemail") {
        None | Some(Value::Null) => None,
        Some(value) => Some(audio(value)?),
    };
    let mut fields = data;
    fields.remove("recordings");
    fields.remove("voicemail");
    Some(Event {
        kind: short(row.get("type")?)?,
        data: without_signed_urls(Value::Object(fields)),
        voicemail,
    })
}

fn audio(value: &Value) -> Option<Voicemail> {
    let row = record(value)?;
    let job = row
        .get("transcription")
        .filter(|value| !value.is_null())
        .and_then(record);
    Some(Voicemail {
        mimetype: optional_text(row.get("mimetype")),
        transcription_id: match job {
            None => None,
            Some(job) => Some(identity(job.get("id")?)?),
        },
    })
}

fn without_signed_urls(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(without_signed_urls).collect()),
        Value::Object(map) => {
            let mut kept = Map::new();
            for (key, entry) in map {
                if key == "signedUrl" {
                    continue;
                }
                kept.insert(key, without_signed_urls(entry));
            }
            Value::Object(kept)
        }
        other => other,
    }
}

fn paging(row: &Map<String, Value>) -> Option<(bool, Option<String>)> {
    let has_more = row.get("hasMore")?.as_bool()?;
    let token = optional_text(row.get("paginationToken"));
    if has_more && token.is_none() {
        return None;
    }
    Some((has_more, token))
}

fn record(value: &Value) -> Option<&Map<String, Value>> {
    value.as_object()
}

fn array(value: &Value, maximum: usize) -> Option<&Vec<Value>> {
    let array = value.as_array()?;
    (array.len() <= maximum).then_some(array)
}

fn body(value: &Value) -> Option<String> {
    let text = value.as_str()?;
    (text.len() <= 1_000_000).then(|| text.to_owned())
}

fn short(value: &Value) -> Option<String> {
    let text = body(value)?;
    (!text.is_empty() && text.len() <= 2_048).then_some(text)
}

fn optional_text(value: Option<&Value>) -> Option<String> {
    match value {
        None | Some(Value::Null) => None,
        Some(Value::String(text)) if text.is_empty() => None,
        Some(value) => short(value),
    }
}

fn identity(value: &Value) -> Option<String> {
    let text = short(value)?;
    valid_id(&text).then_some(text)
}

/// Spruce's phone example sends `rawValue` as a JSON number. Keep its digits.
fn raw_endpoint(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::Number(number)) => {
            let digits = number.as_i64()?;
            (digits >= 0).then(|| digits.to_string())
        }
        Some(other) => optional_text(Some(other)),
        None => None,
    }
}

fn canonical_json(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let fields: Vec<String> = keys
                .into_iter()
                .map(|key| format!("{key}:{}", canonical_json(&map[key])))
                .collect();
            format!("{{{}}}", fields.join(","))
        }
        Value::Array(items) => {
            let fields: Vec<String> = items.iter().map(canonical_json).collect();
            format!("[{}]", fields.join(","))
        }
        other => other.to_string(),
    }
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> Value {
        json!({
            "hasMore": false,
            "conversationItems": [
                {
                    "id": "ti_ok",
                    "conversationId": "t_1",
                    "direction": "inbound",
                    "isInternalNote": false,
                    "text": "hello",
                    "createdAt": "2026-10-27T18:12:50Z",
                    "modifiedAt": "2026-10-27T18:12:50Z",
                    "attachments": [{"type": "image", "title": "scan", "data": {"mimetype": "image/jpeg", "name": "a.jpg", "signedUrl": "https://signed.example/a"}}],
                    "event": {
                        "type": "voicemail",
                        "data": {
                            "signedUrl": "https://signed.example/event",
                            "recordings": [{"signedUrl": "https://signed.example/rec"}],
                            "duration": 3,
                            "voicemail": {"mimetype": "audio/mp4", "transcription": {"id": "tr_1", "status": "completed"}}
                        }
                    }
                },
                {"id": "ti_bad", "text": 12},
                {"text": "no id"}
            ]
        })
    }

    #[test]
    fn one_bad_item_does_not_fail_the_page() {
        let page = item_page(&sample()).unwrap();
        assert!(!page.has_more);
        assert!(matches!(page.entries[0], ItemEntry::Item(_)));
        assert!(matches!(page.entries[1], ItemEntry::Unreadable(_)));
        assert!(matches!(page.entries[2], ItemEntry::Malformed));
    }

    #[test]
    fn signed_urls_and_recordings_are_not_kept() {
        let page = item_page(&sample()).unwrap();
        let ItemEntry::Item(item) = &page.entries[0] else {
            panic!("expected the readable item");
        };
        let event = item.event.as_ref().unwrap();
        let data = event.data.to_string();
        assert!(!data.contains("signed"));
        assert!(!data.contains("recordings"));
        assert!(data.contains("duration"));
        assert_eq!(
            event
                .voicemail
                .as_ref()
                .unwrap()
                .transcription_id
                .as_deref(),
            Some("tr_1")
        );
        assert!(item.attachments[0].mimetype.as_deref() == Some("image/jpeg"));
    }

    #[test]
    fn version_is_stable_until_the_content_or_modified_at_changes() {
        let page = item_page(&sample()).unwrap();
        let ItemEntry::Item(item) = &page.entries[0] else {
            panic!("expected the readable item");
        };
        let again = item_page(&sample()).unwrap();
        let ItemEntry::Item(repeat) = &again.entries[0] else {
            panic!("expected the readable item");
        };
        assert_eq!(item.version(), repeat.version());

        let mut edited = item.clone();
        edited.text = "changed".into();
        assert_ne!(item.version(), edited.version());

        let mut touched = item.clone();
        touched.envelope.modified_at = Some("2026-10-27T19:00:00Z".into());
        assert_ne!(item.version(), touched.version());
    }

    #[test]
    fn a_page_that_claims_more_without_a_token_is_unreadable() {
        let body = json!({"hasMore": true, "conversationItems": []});
        assert!(item_page(&body).is_none());
    }

    #[test]
    fn a_numeric_phone_raw_value_keeps_its_digits() {
        let body = json!({"internalEndpoints": [{"endpoint": {"channel": "phone", "rawValue": 12065550100_i64}}]});
        let endpoints = internal_endpoints(&body).unwrap();
        assert_eq!(endpoints[0].raw_value.as_deref(), Some("12065550100"));
    }

    #[test]
    fn deletion_is_only_the_flag_spruce_sends() {
        let mut body = sample();
        body["conversationItems"][0]["deleted"] = json!(true);
        let page = item_page(&body).unwrap();
        let ItemEntry::Item(item) = &page.entries[0] else {
            panic!("expected the readable item");
        };
        assert!(item.deleted);
    }
}
