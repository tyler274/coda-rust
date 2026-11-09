//! Conversation changes from a stored cursor.
//!
//! The first pass only finds the latest activity. Nothing from that pass is a
//! change: it was already there when Coda connected. Later passes list
//! conversations from that boundary, pulled back by an overlap so an item
//! Spruce had not shown yet is read again. A repeated item keeps its version.
//!
//! One conversation is the unit of failure. A conversation that fails or that
//! cannot be decoded is named in [`ChangePage::unread`] and the pass continues.
//! A credential refusal stops the pass instead, and the cursor does not move.

use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

use crate::api::{Client, Error, ListQuery, Order};
use crate::payload::{
    Conversation, ConversationEntry, Envelope, Item, ItemEntry, MAX_PAGE_SIZE, TranscriptionState,
};

/// How far a later pass reaches back past the boundary.
///
/// Spruce's activity list is eventually consistent. Two minutes covers a
/// message that was created just before the boundary and listed just after it.
pub const DEFAULT_OVERLAP: Duration = Duration::from_secs(120);

/// A stored cursor this module did not write, or can no longer read.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CursorError {
    /// The text is not a cursor this module encoded.
    #[error("spruce synchronization cursor is invalid")]
    Invalid,
}

/// Where the next pass continues.
///
/// Opaque apart from [`encode`](Self::encode) and [`decode`](Self::decode).
/// Store `encode` only after the page's effects are durable. Until then, pass
/// the previous cursor and the page is read again.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    /// Latest activity when the connection was established. Older items are history.
    floor: Option<String>,
    /// Latest `lastMessageAt` a finished scan saw.
    boundary: Option<String>,
    /// Page token of the scan in progress.
    page: Option<String>,
    /// Latest activity the scan in progress has seen.
    seen: Option<String>,
}

impl Cursor {
    /// The stored form.
    pub fn encode(&self) -> String {
        serde_json::to_string(self).expect("cursor json")
    }

    /// Restores a stored cursor. Anything this module did not write is refused.
    pub fn decode(stored: &str) -> Result<Self, CursorError> {
        let cursor: Self = serde_json::from_str(stored).map_err(|_| CursorError::Invalid)?;
        if !instant_ok(&cursor.floor) || !instant_ok(&cursor.boundary) || !instant_ok(&cursor.seen)
        {
            return Err(CursorError::Invalid);
        }
        if cursor.page.as_deref().is_some_and(str::is_empty) {
            return Err(CursorError::Invalid);
        }
        Ok(cursor)
    }

    /// The first pass has finished, so [`Self::is_history`] can answer.
    pub fn is_established(&self) -> bool {
        self.floor.is_some() && self.boundary.is_some()
    }

    /// The item was already there when the connection was established, so
    /// it is history and not a change. `false` until the first pass has set
    /// the floor, and for a time that does not parse.
    pub fn is_history(&self, created_at: Option<&str>) -> bool {
        self.floor
            .as_deref()
            .is_some_and(|floor| at_or_before(created_at, floor))
    }
}

/// Why a conversation on this page still needs a full read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnreadCause {
    /// The conversation or one of its items had no shape this crate could record.
    Undecodable,
    /// The read failed for now. A later pass can try the conversation again.
    TemporarilyUnavailable,
}

/// A conversation the pass could not finish.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unread {
    /// Spruce conversation id.
    pub conversation_id: String,
    /// Why it is still owed a read.
    pub cause: UnreadCause,
}

/// An item the pass observed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Observed {
    /// A decoded item, including its transcript when one was fetched.
    Message(Box<Item>),
    /// The envelope was readable and the content was not.
    Unreadable(Envelope),
}

/// One page of changes and the cursor to store once they are applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangePage {
    /// Items created after the checkpoint, newest first within each conversation.
    pub changes: Vec<Observed>,
    /// Conversations this page could not finish.
    pub unread: Vec<Unread>,
    /// Another page follows from `cursor`.
    pub has_more: bool,
    /// Store this only after `changes` and `unread` are durable.
    pub cursor: Cursor,
}

/// A whole-conversation read, for the re-read [`crate::reconcile`] compares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationRead {
    /// One page of the conversation, newest first.
    Messages {
        /// Items and unreadable envelopes on this page.
        items: Vec<Observed>,
        /// Present when Spruce has another page.
        next_page: Option<String>,
        /// Some item on this page had no id, or no shape that could be recorded.
        undecodable: bool,
    },
    /// Spruce answered 404. What was already stored stays stored.
    Missing,
    /// Spruce's body was not a conversation this crate can read.
    Undecodable,
    /// The credential was refused.
    ReconnectRequired,
    /// Spruce did not answer, or refused this conversation for now.
    Unavailable,
}

/// Result of one change pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// A page of changes. The cursor moves only here.
    Page(ChangePage),
    /// The credential was refused. Keep the cursor and reconnect.
    ReconnectRequired,
    /// Spruce did not answer the list. Keep the cursor and retry.
    Unavailable,
}

/// Reads one Spruce organization's changes.
#[derive(Debug, Clone)]
pub struct Reader {
    client: Client,
    overlap: Duration,
    page_size: u32,
}

impl Reader {
    /// A reader with a two-minute overlap and Spruce's maximum page size.
    pub fn new(client: Client) -> Self {
        Self {
            client,
            overlap: DEFAULT_OVERLAP,
            page_size: MAX_PAGE_SIZE as u32,
        }
    }

    /// How far each later pass reaches back past the stored boundary.
    pub fn with_overlap(mut self, overlap: Duration) -> Self {
        self.overlap = overlap;
        self
    }

    /// Entries requested per Spruce page, from 1 to [`MAX_PAGE_SIZE`].
    pub fn with_page_size(mut self, page_size: u32) -> Result<Self, Error> {
        if !(1..=MAX_PAGE_SIZE as u32).contains(&page_size) {
            return Err(Error::InvalidRequest);
        }
        self.page_size = page_size;
        Ok(self)
    }

    /// The next page of changes.
    ///
    /// `cursor` is `None` until the first pass has established a boundary.
    /// [`Error::Contract`] and [`Error::InvalidRequest`] are faults in Spruce's
    /// list response or in this call. A conversation-sized failure is an
    /// [`Outcome::Page`] with that conversation in `unread`.
    pub async fn changes(&self, cursor: Option<&Cursor>) -> Result<Outcome, Error> {
        let cursor = cursor.cloned().unwrap_or(Cursor {
            floor: None,
            boundary: None,
            page: None,
            seen: None,
        });
        let mut org = OrgCheck::Unknown;
        if cursor.boundary.is_none() || cursor.floor.is_none() {
            return self.establish(cursor, &mut org).await;
        }
        self.scan(cursor, &mut org).await
    }

    /// One page of a conversation, newest first, including items from before the boundary.
    ///
    /// This is the read [`crate::reconcile`] compares. It does not move the change cursor.
    pub async fn read_conversation(
        &self,
        id: &str,
        page_token: Option<&str>,
    ) -> Result<ConversationRead, Error> {
        let mut org = OrgCheck::Unknown;
        let conversation = match self.client.get_conversation(id).await {
            Ok(conversation) => conversation,
            Err(error) => return self.read_failure(error, &mut org).await,
        };
        let query = match page_token {
            Some(token) => ListQuery {
                page_size: self.page_size,
                start_from: None,
                pagination_token: Some(token),
            },
            None => ListQuery::first(self.page_size),
        };
        let page = match self
            .client
            .list_items(&conversation.id, Order::NewestFirst, query)
            .await
        {
            Ok(page) => page,
            Err(error) => return self.read_failure(error, &mut org).await,
        };
        let mut entries = page.entries;
        if let Err(error) = self.attach_transcriptions(&mut entries, &mut org).await {
            return self.read_failure(error, &mut org).await;
        }
        let observed = observe(&entries);
        Ok(ConversationRead::Messages {
            items: observed.items,
            next_page: page.has_more.then_some(page.pagination_token).flatten(),
            undecodable: observed.undecodable,
        })
    }

    async fn establish(&self, cursor: Cursor, org: &mut OrgCheck) -> Result<Outcome, Error> {
        let query = match cursor.page.as_deref() {
            Some(token) => ListQuery {
                page_size: self.page_size,
                start_from: None,
                pagination_token: Some(token),
            },
            None => ListQuery::first(self.page_size),
        };
        let page = match self.client.list_conversations(query).await {
            Ok(page) => page,
            Err(error) => return self.list_failure(error, org).await,
        };
        let seen = latest(
            cursor.seen.as_deref(),
            page.entries.iter().filter_map(|entry| match entry {
                ConversationEntry::Conversation(conversation) => {
                    conversation.last_message_at.as_deref()
                }
                ConversationEntry::Malformed { .. } => None,
            }),
        );
        let done = !page.has_more;
        let next = if done {
            let boundary = seen.unwrap_or_else(epoch);
            Cursor {
                floor: Some(boundary.clone()),
                boundary: Some(boundary),
                page: None,
                seen: None,
            }
        } else {
            Cursor {
                floor: None,
                boundary: None,
                page: page.pagination_token,
                seen,
            }
        };
        Ok(Outcome::Page(ChangePage {
            changes: Vec::new(),
            unread: Vec::new(),
            has_more: !done,
            cursor: next,
        }))
    }

    async fn scan(&self, cursor: Cursor, org: &mut OrgCheck) -> Result<Outcome, Error> {
        let floor = cursor.floor.clone().ok_or(Error::InvalidRequest)?;
        let boundary = cursor.boundary.clone().ok_or(Error::InvalidRequest)?;
        let since = checkpoint(&floor, &boundary, self.overlap);
        let query = match cursor.page.as_deref() {
            Some(token) => ListQuery {
                page_size: self.page_size,
                start_from: None,
                pagination_token: Some(token),
            },
            None => ListQuery {
                page_size: self.page_size,
                start_from: Some(&since),
                pagination_token: None,
            },
        };
        let page = match self.client.list_conversations(query).await {
            Ok(page) => page,
            Err(error) => return self.list_failure(error, org).await,
        };
        let mut changes = Vec::new();
        let mut unread = Vec::new();
        let mut seen = cursor.seen.clone();
        for entry in &page.entries {
            match entry {
                ConversationEntry::Malformed { id: None } => {}
                ConversationEntry::Malformed { id: Some(id) } => unread.push(Unread {
                    conversation_id: id.clone(),
                    cause: UnreadCause::Undecodable,
                }),
                ConversationEntry::Conversation(conversation) => {
                    seen = latest(seen.as_deref(), conversation.last_message_at.as_deref());
                    if let Err(error) = self
                        .read_conversation_since(
                            conversation,
                            &since,
                            org,
                            &mut changes,
                            &mut unread,
                        )
                        .await
                    {
                        return self.list_failure(error, org).await;
                    }
                }
            }
        }
        let done = !page.has_more;
        let next = if done {
            Cursor {
                floor: Some(floor),
                boundary: Some(
                    latest(Some(boundary.as_str()), seen.as_deref()).unwrap_or(boundary),
                ),
                page: None,
                seen: None,
            }
        } else {
            Cursor {
                floor: Some(floor),
                boundary: Some(boundary),
                page: page.pagination_token,
                seen,
            }
        };
        Ok(Outcome::Page(ChangePage {
            changes,
            unread,
            has_more: !done,
            cursor: next,
        }))
    }

    async fn read_conversation_since(
        &self,
        conversation: &Conversation,
        since: &str,
        org: &mut OrgCheck,
        changes: &mut Vec<Observed>,
        unread: &mut Vec<Unread>,
    ) -> Result<(), Error> {
        let mut token: Option<String> = None;
        let mut undecodable = false;
        loop {
            let query = match token.as_deref() {
                Some(page) => ListQuery {
                    page_size: self.page_size,
                    start_from: None,
                    pagination_token: Some(page),
                },
                None => ListQuery {
                    page_size: self.page_size,
                    start_from: Some(since),
                    pagination_token: None,
                },
            };
            let page = match self
                .client
                .list_items(&conversation.id, Order::NewestFirst, query)
                .await
            {
                Ok(page) => page,
                Err(error) => {
                    return self
                        .conversation_error(
                            conversation.id.clone(),
                            error,
                            org,
                            changes,
                            unread,
                            undecodable,
                        )
                        .await;
                }
            };
            let mut entries = page.entries;
            if let Err(error) = self.attach_transcriptions(&mut entries, org).await {
                return self
                    .conversation_error(
                        conversation.id.clone(),
                        error,
                        org,
                        changes,
                        unread,
                        undecodable,
                    )
                    .await;
            }
            let reached_history = entries
                .iter()
                .any(|entry| at_or_before(created_at(entry), since));
            entries.retain(|entry| !at_or_before(created_at(entry), since));
            let observed = observe(&entries);
            undecodable |= observed.undecodable;
            changes.extend(observed.items);
            let more = page.has_more && !reached_history;
            token = more.then_some(page.pagination_token).flatten();
            if token.is_none() {
                break;
            }
        }
        if undecodable {
            unread.push(Unread {
                conversation_id: conversation.id.clone(),
                cause: UnreadCause::Undecodable,
            });
        }
        Ok(())
    }

    async fn conversation_error(
        &self,
        conversation_id: String,
        error: Error,
        org: &mut OrgCheck,
        _changes: &mut Vec<Observed>,
        unread: &mut Vec<Unread>,
        undecodable: bool,
    ) -> Result<(), Error> {
        if self.refused(&error, org).await? {
            return Err(error);
        }
        if matches!(error, Error::NotFound) {
            if undecodable {
                unread.push(Unread {
                    conversation_id,
                    cause: UnreadCause::Undecodable,
                });
            }
            return Ok(());
        }
        let cause = if undecodable || matches!(error, Error::Contract) {
            UnreadCause::Undecodable
        } else {
            UnreadCause::TemporarilyUnavailable
        };
        unread.push(Unread {
            conversation_id,
            cause,
        });
        Ok(())
    }

    async fn list_failure(&self, error: Error, org: &mut OrgCheck) -> Result<Outcome, Error> {
        if self.refused(&error, org).await? {
            return Ok(Outcome::ReconnectRequired);
        }
        if matches!(error, Error::Temporary { .. } | Error::Unavailable(_)) {
            return Ok(Outcome::Unavailable);
        }
        Err(error)
    }

    async fn read_failure(
        &self,
        error: Error,
        org: &mut OrgCheck,
    ) -> Result<ConversationRead, Error> {
        if self.refused(&error, org).await? {
            return Ok(ConversationRead::ReconnectRequired);
        }
        Ok(match error {
            Error::NotFound => ConversationRead::Missing,
            Error::Contract => ConversationRead::Undecodable,
            Error::Temporary { .. } | Error::Unavailable(_) | Error::Rejected { .. } => {
                ConversationRead::Unavailable
            }
            other => return Err(other),
        })
    }

    async fn attach_transcriptions(
        &self,
        entries: &mut [ItemEntry],
        org: &mut OrgCheck,
    ) -> Result<(), Error> {
        let mut cache = HashMap::<String, TranscriptionState>::new();
        for entry in entries {
            let ItemEntry::Item(item) = entry else {
                continue;
            };
            let Some(id) = item
                .event
                .as_ref()
                .and_then(|event| event.voicemail.as_ref())
                .and_then(|voicemail| voicemail.transcription_id.clone())
            else {
                continue;
            };
            let state = if let Some(state) = cache.get(&id) {
                state.clone()
            } else {
                let state = self.transcription_state(&id, org).await?;
                cache.insert(id, state.clone());
                state
            };
            item.transcription = Some(state);
        }
        Ok(())
    }

    async fn transcription_state(
        &self,
        id: &str,
        org: &mut OrgCheck,
    ) -> Result<TranscriptionState, Error> {
        match self.client.get_transcription(id).await {
            Ok(transcription) => {
                let state = if transcription.status == "completed" {
                    TranscriptionState::Ready {
                        id: id.to_owned(),
                        text: transcription.text,
                        summary: transcription.summary,
                    }
                } else {
                    TranscriptionState::Unfinished {
                        id: id.to_owned(),
                        status: transcription.status,
                        text: transcription.text,
                        summary: transcription.summary,
                    }
                };
                Ok(state)
            }
            Err(error) if self.refused(&error, org).await? => Err(error),
            Err(Error::NotFound) => Ok(TranscriptionState::NotFound { id: id.to_owned() }),
            Err(
                Error::Contract
                | Error::Temporary { .. }
                | Error::Unavailable(_)
                | Error::Rejected { .. },
            ) => Ok(TranscriptionState::Unavailable { id: id.to_owned() }),
            Err(error) => Err(error),
        }
    }

    /// A 401 is the credential. A 403 is the credential only when the organization read refuses it too.
    async fn refused(&self, error: &Error, org: &mut OrgCheck) -> Result<bool, Error> {
        match error {
            Error::AuthorizationRejected => Ok(true),
            Error::Rejected { status: 403 } => self.organization_refused(org).await,
            _ => Ok(false),
        }
    }

    async fn organization_refused(&self, org: &mut OrgCheck) -> Result<bool, Error> {
        if let OrgCheck::Unknown = *org {
            *org = match self.client.organization().await {
                Err(Error::AuthorizationRejected | Error::Rejected { status: 403 }) => {
                    OrgCheck::Refused
                }
                Err(Error::Temporary { .. } | Error::Unavailable(_)) => OrgCheck::Unreachable,
                Err(error) => return Err(error),
                Ok(_) => OrgCheck::Accepted,
            };
        }
        Ok(matches!(org, OrgCheck::Refused))
    }
}

#[derive(Debug, Clone, Copy)]
enum OrgCheck {
    Unknown,
    Refused,
    Accepted,
    Unreachable,
}

struct ObservedPage {
    items: Vec<Observed>,
    undecodable: bool,
}

fn observe(entries: &[ItemEntry]) -> ObservedPage {
    let mut items = Vec::new();
    let mut undecodable = false;
    for entry in entries {
        match entry {
            ItemEntry::Item(item) => items.push(Observed::Message(item.clone())),
            ItemEntry::Unreadable(envelope) => items.push(Observed::Unreadable(envelope.clone())),
            ItemEntry::Malformed => undecodable = true,
        }
    }
    ObservedPage { items, undecodable }
}

fn created_at(entry: &ItemEntry) -> Option<&str> {
    match entry {
        ItemEntry::Item(item) => item.envelope.created_at.as_deref(),
        ItemEntry::Unreadable(envelope) => envelope.created_at.as_deref(),
        ItemEntry::Malformed => None,
    }
}

/// An instant that does not parse is not known to be at or before the checkpoint, so it is kept.
fn at_or_before(value: Option<&str>, limit: &str) -> bool {
    match (value.and_then(parse_instant), parse_instant(limit)) {
        (Some(value), Some(limit)) => value <= limit,
        _ => false,
    }
}

fn checkpoint(floor: &str, boundary: &str, overlap: Duration) -> String {
    let floor_at = parse_instant(floor).unwrap_or_else(epoch_at);
    let boundary_at = parse_instant(boundary).unwrap_or(floor_at);
    let pulled = chrono::Duration::from_std(overlap)
        .ok()
        .and_then(|overlap| boundary_at.checked_sub_signed(overlap))
        .unwrap_or_else(epoch_at);
    if pulled < floor_at {
        if parse_instant(floor).is_some() {
            return floor.to_owned();
        }
        return format_instant(floor_at);
    }
    format_instant(pulled)
}

fn latest<'a>(seed: Option<&str>, values: impl IntoIterator<Item = &'a str>) -> Option<String> {
    let mut found = seed.and_then(|value| parse_instant(value).map(|at| (at, value.to_owned())));
    for value in values {
        if let Some(at) = parse_instant(value)
            && found.as_ref().is_none_or(|(best, _)| at > *best)
        {
            // Keep Spruce's own timestamp. Reformatting to whole seconds would
            // move the history floor earlier than the winning activity.
            found = Some((at, value.to_owned()));
        }
    }
    found.map(|(_, value)| value)
}

fn instant_ok(value: &Option<String>) -> bool {
    match value {
        None => true,
        Some(value) => value.len() <= 64 && parse_instant(value).is_some(),
    }
}

fn parse_instant(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

fn format_instant(value: DateTime<Utc>) -> String {
    let precision = if value.timestamp_subsec_nanos() == 0 {
        SecondsFormat::Secs
    } else {
        SecondsFormat::Millis
    };
    value.to_rfc3339_opts(precision, true)
}

fn epoch() -> String {
    format_instant(epoch_at())
}

fn epoch_at() -> DateTime<Utc> {
    DateTime::UNIX_EPOCH
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cursor_this_module_did_not_write_is_refused() {
        assert_eq!(Cursor::decode("nope"), Err(CursorError::Invalid));
        assert_eq!(Cursor::decode(r#"{"floor":1}"#), Err(CursorError::Invalid));
        assert_eq!(
            Cursor::decode(r#"{"floor":"yesterday","boundary":null,"page":null,"seen":null}"#),
            Err(CursorError::Invalid)
        );
        assert_eq!(
            Cursor::decode(r#"{"floor":null,"boundary":null,"page":"","seen":null}"#),
            Err(CursorError::Invalid)
        );
    }

    #[test]
    fn a_cursor_round_trips() {
        let cursor = Cursor {
            floor: Some("2026-10-27T18:12:50Z".into()),
            boundary: Some("2026-10-27T18:12:50Z".into()),
            page: None,
            seen: None,
        };
        assert_eq!(Cursor::decode(&cursor.encode()).unwrap(), cursor);
    }

    #[test]
    fn history_is_at_or_before_the_floor() {
        let unset = Cursor {
            floor: None,
            boundary: None,
            page: None,
            seen: None,
        };
        assert!(!unset.is_history(Some("2026-10-27T18:00:00Z")));
        let cursor = Cursor {
            floor: Some("2026-10-27T18:00:00Z".into()),
            ..unset
        };
        assert!(cursor.is_history(Some("2026-10-27T18:00:00Z")));
        assert!(cursor.is_history(Some("2026-10-27T17:59:59Z")));
        assert!(!cursor.is_history(Some("2026-10-27T18:00:01Z")));
        assert!(!cursor.is_history(None));
    }

    #[test]
    fn the_checkpoint_never_moves_before_the_floor() {
        let floor = "2026-10-27T18:00:00Z";
        let since = checkpoint(floor, floor, Duration::from_secs(120));
        assert_eq!(since, floor);
        let later = "2026-10-27T18:10:00Z";
        assert_eq!(
            checkpoint(floor, later, Duration::from_secs(60)),
            "2026-10-27T18:09:00Z"
        );
    }
}
