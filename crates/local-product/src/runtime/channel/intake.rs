//! One bounded intake pass, and setting the connection up.
//!
//! A pass claims the practice's lease, reads change pages from the stored
//! cursor, saves each page with its files in one transaction, then gives owed
//! conversations a full read and lists the accepted backlog. It stops at its
//! page budget; the next wake-up continues from the stored cursor.
//!
//! Spruce not answering keeps the cursor and the last good read and records
//! the failure. A refused credential asks for a reconnect.

use std::collections::HashSet;

use spruce::api::{Client, Error as SpruceError, ListQuery};
use spruce::changes::{
    ConversationRead, Cursor, CursorError, Observed, Outcome, Reader, UnreadCause,
};
use spruce::payload::{
    Contact, Conversation, ConversationEntry, Envelope, Item, MAX_PAGE_SIZE, TranscriptionState,
};

use super::{
    AccessLoss, AttachmentRecord, Attempt, Backlog, Body, ChannelError, ChannelKey, Connection,
    ContactRecord, ConversationDetails, Direction, ItemRead, Lease, MAX_FILE_BYTES, MessageVersion,
    OwedCause, PageWrite, Participant, SavedFile, sha256_hex,
};
use crate::runtime::Store;

/// Where the Spruce API is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Api {
    /// Spruce's production API.
    Live,
    /// Another base URL, such as a loopback stand-in.
    At(String),
}

impl Api {
    fn client(&self, credential: &str) -> Result<Client, SpruceError> {
        match self {
            Self::Live => Client::new(credential),
            Self::At(base) => Client::with_base(credential, base),
        }
    }
}

/// How one pass runs.
#[derive(Clone, Debug)]
pub struct Intake {
    /// Opens the stored credential.
    pub key: ChannelKey,
    /// Where Spruce is.
    pub api: Api,
    /// This reader's lease holder name.
    pub holder: String,
    /// Lease length, renewed by every saved page.
    pub lease_seconds: i64,
    /// Change pages per pass.
    pub pages: usize,
    /// Owed conversations given a full read per pass.
    pub owed_reads: u64,
    /// Item pages read per owed conversation per pass.
    pub owed_pages: usize,
    /// Seconds before a conversation is due a routine reread, which finds
    /// edits and deletion flags.
    pub recheck_every: i64,
    /// Routine rereads queued per pass.
    pub rechecks: u64,
}

impl Intake {
    /// Defaults sized for a one-minute wake-up.
    pub fn new(key: ChannelKey, api: Api, holder: impl Into<String>) -> Self {
        Self {
            key,
            api,
            holder: holder.into(),
            lease_seconds: 120,
            pages: 20,
            owed_reads: 20,
            owed_pages: 10,
            recheck_every: 15 * 60,
            rechecks: 10,
        }
    }
}

/// What one pass did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PassOutcome {
    /// Another reader holds the lease.
    Busy,
    /// The practice is not reading its channel.
    NotReading,
    /// The pass ran.
    Read {
        /// Change pages saved.
        pages: usize,
        /// New message versions kept.
        kept: usize,
        /// Change pages remain past this pass's budget.
        more: bool,
    },
    /// Spruce did not answer. The cursor stayed.
    Unavailable,
    /// The grant is gone. The connection asks for a reconnect.
    ReconnectRequired,
}

/// A pass or a connect that did not finish.
#[derive(Debug, thiserror::Error)]
pub enum IntakeError {
    /// The store failed.
    #[error(transparent)]
    Store(#[from] ChannelError),
    /// Spruce answered with something this reader can't act on.
    #[error("spruce read failed")]
    Spruce(#[from] SpruceError),
    /// The stored cursor is not one the Spruce reader wrote.
    #[error("stored spruce cursor is unreadable")]
    Cursor(#[from] CursorError),
}

/// Why a connect was refused before anything was stored.
#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    /// Spruce refused the credential.
    #[error("spruce refused the credential")]
    Refused(#[source] SpruceError),
    /// Spruce did not answer. Nothing changed.
    #[error("spruce did not answer")]
    Unavailable(#[source] SpruceError),
    /// The credential can't be sent, or Spruce's answer was unreadable.
    #[error("spruce could not check the credential")]
    Check(#[source] SpruceError),
    /// Spruce refused the conversation list that sets where new messages
    /// start. Nothing changed.
    #[error("spruce refused the conversation list that sets where new messages start")]
    BoundaryRefused,
    /// Spruce did not answer the conversation list that sets where new
    /// messages start. Nothing changed.
    #[error("spruce did not answer the conversation list that sets where new messages start")]
    BoundaryUnavailable,
    /// The store refused the binding.
    #[error(transparent)]
    Store(#[from] ChannelError),
}

/// Checks a credential against Spruce and binds its organization to the practice.
///
/// Before binding, it reads where new messages start, so a message that
/// arrives after Connect returns is new however long the first pass waits.
pub async fn connect(
    store: &Store,
    key: &ChannelKey,
    api: &Api,
    practice: &str,
    member: &str,
    credential: &str,
) -> Result<Connection, ConnectError> {
    let client = api.client(credential).map_err(ConnectError::Check)?;
    let organization = match client.organization().await {
        Ok(organization) => organization,
        Err(error @ (SpruceError::AuthorizationRejected | SpruceError::Rejected { .. })) => {
            return Err(ConnectError::Refused(error));
        }
        Err(error @ (SpruceError::Temporary { .. } | SpruceError::Unavailable(_))) => {
            return Err(ConnectError::Unavailable(error));
        }
        Err(error) => return Err(ConnectError::Check(error)),
    };
    let start = boundary(&Reader::new(client)).await?;
    Ok(store
        .bind_channel(key, practice, &organization.id, member, credential, &start)
        .await?)
}

/// Lists the organization's conversations until the history floor is set.
async fn boundary(reader: &Reader) -> Result<Cursor, ConnectError> {
    let mut cursor: Option<Cursor> = None;
    loop {
        let page = match reader
            .changes(cursor.as_ref())
            .await
            .map_err(ConnectError::Check)?
        {
            Outcome::Page(page) => page,
            Outcome::ReconnectRequired => return Err(ConnectError::BoundaryRefused),
            Outcome::Unavailable => return Err(ConnectError::BoundaryUnavailable),
        };
        if page.cursor.is_established() {
            return Ok(page.cursor);
        }
        cursor = Some(page.cursor);
    }
}

/// Runs one pass for the practice.
pub async fn pass(
    store: &Store,
    intake: &Intake,
    practice: &str,
) -> Result<PassOutcome, IntakeError> {
    let Some(lease) = store
        .claim_channel(practice, &intake.holder, intake.lease_seconds)
        .await?
    else {
        return Ok(PassOutcome::Busy);
    };
    let outcome = run(store, intake, &lease).await;
    // A lost lease needs no release; any other result gives the lease back so
    // the next wake-up does not wait for it to expire.
    store.release_channel(&lease).await?;
    outcome
}

async fn run(store: &Store, intake: &Intake, lease: &Lease) -> Result<PassOutcome, IntakeError> {
    let Connection::Reading { backlog, .. } = &lease.connection else {
        return Ok(PassOutcome::NotReading);
    };
    let credential = store.channel_credential(&intake.key, lease)?;
    let client = intake.api.client(&credential)?;
    let work = Work {
        store,
        lease,
        client: client.clone(),
        reader: Reader::new(client),
        history: backlog.allows_history(),
    };
    let mut cursor = lease.cursor.as_deref().map(Cursor::decode).transpose()?;
    let mut pages = 0;
    let mut kept = 0;
    let mut more = true;
    while more && pages < intake.pages {
        let outcome = match work.reader.changes(cursor.as_ref()).await {
            Ok(outcome) => outcome,
            Err(error) => {
                store.channel_unavailable(lease).await?;
                return Err(error.into());
            }
        };
        let page = match outcome {
            Outcome::Page(page) => page,
            Outcome::ReconnectRequired => return work.access_lost().await,
            Outcome::Unavailable => {
                store.channel_unavailable(lease).await?;
                return Ok(PassOutcome::Unavailable);
            }
        };
        let mut write = PageWrite::default();
        for unread in &page.unread {
            write
                .owed
                .push((unread.conversation_id.clone(), owed_cause(unread.cause)));
        }
        match work.observed(page.changes, &mut write).await {
            Ok(()) => {}
            Err(Stop::Refused) => return work.access_lost().await,
            Err(Stop::Failed(error)) => return Err(error),
        }
        write.cursor = Some(page.cursor.encode());
        kept += store.apply_channel_page(lease, write).await?;
        pages += 1;
        more = page.has_more;
        cursor = Some(page.cursor);
    }
    // Connect sets the floor. A cursor without one, written before Connect
    // did, can't tell history from new messages yet, so owed reads and the
    // backlog wait for the pass that sets it.
    if let Some(cursor) = cursor.as_ref().filter(|cursor| cursor.is_established()) {
        store
            .queue_rechecks(lease, intake.recheck_every, intake.rechecks)
            .await?;
        match work.owed(intake, cursor).await {
            Ok(saved) => kept += saved,
            Err(Stop::Refused) => return work.access_lost().await,
            Err(Stop::Failed(error)) => return Err(error),
        }
        match work.backlog().await {
            Ok(()) => {}
            Err(Stop::Refused) => return work.access_lost().await,
            Err(Stop::Failed(error)) => return Err(error),
        }
    }
    Ok(PassOutcome::Read { pages, kept, more })
}

enum Stop {
    Refused,
    Failed(IntakeError),
}

impl From<ChannelError> for Stop {
    fn from(error: ChannelError) -> Self {
        Self::Failed(error.into())
    }
}

struct Work<'a> {
    store: &'a Store,
    lease: &'a Lease,
    client: Client,
    reader: Reader,
    history: bool,
}

impl Work<'_> {
    async fn access_lost(&self) -> Result<PassOutcome, IntakeError> {
        self.store
            .channel_access_lost(self.lease, AccessLoss::Revoked)
            .await?;
        Ok(PassOutcome::ReconnectRequired)
    }

    /// Turns observed items into reads, downloading files of new versions
    /// only. A conversation whose file failed or whose item was unreadable is
    /// owed a full read.
    async fn observed(&self, observed: Vec<Observed>, write: &mut PageWrite) -> Result<(), Stop> {
        let items: Vec<String> = observed.iter().map(|o| observed_id(o).to_owned()).collect();
        let saved = self
            .store
            .channel_versions(&self.lease.practice, &items)
            .await?;
        let mut conversations = HashSet::new();
        for entry in observed {
            match entry {
                Observed::Message(item) => {
                    let Some(conversation) = item.envelope.conversation_id.clone() else {
                        continue;
                    };
                    conversations.insert(conversation.clone());
                    // A transcript still on its way keeps the conversation
                    // owed whether or not this version is already saved.
                    if transcript_pending(&item) {
                        write
                            .owed
                            .push((conversation.clone(), OwedCause::TemporarilyUnavailable));
                    }
                    let version = item.version();
                    if saved.contains(&(item.id().to_owned(), version.clone())) {
                        write.versions.push(ItemRead::Saved {
                            item: item.id().to_owned(),
                            version,
                            modified_at: item.envelope.modified_at.clone(),
                        });
                        continue;
                    }
                    let attachments = self.files(&item, &conversation, write).await?;
                    let mut kept = message(&item, conversation, version, attachments);
                    // Files still to fetch: the item is kept with what was
                    // read, under a version naming which files are owed, so
                    // a read that fetches some of them is kept too. Its
                    // conversation is owed the read that saves the rest.
                    let owed = kept.owed_files();
                    if !owed.is_empty() {
                        write
                            .owed
                            .push((kept.conversation.clone(), OwedCause::TemporarilyUnavailable));
                        let positions: Vec<String> = owed.iter().map(ToString::to_string).collect();
                        kept.version =
                            format!("files-owed:{}:{}", positions.join(","), kept.version);
                    }
                    write.versions.push(ItemRead::Version(Box::new(kept)));
                }
                Observed::Unreadable(envelope) => {
                    let Some(conversation) = envelope.conversation_id.clone() else {
                        continue;
                    };
                    conversations.insert(conversation.clone());
                    write
                        .owed
                        .push((conversation.clone(), OwedCause::Undecodable));
                    let version = unreadable_version(&envelope);
                    write.versions.push(
                        if saved.contains(&(envelope.id.clone(), version.clone())) {
                            ItemRead::Saved {
                                item: envelope.id.clone(),
                                version,
                                modified_at: envelope.modified_at.clone(),
                            }
                        } else {
                            ItemRead::Version(Box::new(unreadable(
                                &envelope,
                                conversation,
                                version,
                            )))
                        },
                    );
                }
            }
        }
        let wanted: Vec<String> = conversations.into_iter().collect();
        let known = self
            .store
            .channel_known_conversations(&self.lease.practice, &wanted)
            .await?;
        for conversation in wanted.into_iter().filter(|c| !known.contains(c)) {
            match self.details(&conversation).await? {
                Some(details) => write.details.push((conversation, details)),
                None => write
                    .owed
                    .push((conversation, OwedCause::TemporarilyUnavailable)),
            }
        }
        Ok(())
    }

    /// Saves an item's files. A file that could not be fetched now is
    /// [`AttachmentRecord::Owed`].
    async fn files(
        &self,
        item: &Item,
        conversation: &str,
        write: &mut PageWrite,
    ) -> Result<Vec<AttachmentRecord>, Stop> {
        let mut records = Vec::new();
        for attachment in &item.attachments {
            let Some(link) = &attachment.link else {
                records.push(AttachmentRecord::Card {
                    kind: attachment.kind.clone(),
                    title: attachment.title.clone(),
                });
                continue;
            };
            match self.client.download(link, MAX_FILE_BYTES).await {
                Ok(bytes) => {
                    let sha256 = sha256_hex(&bytes);
                    records.push(AttachmentRecord::File {
                        sha256: sha256.clone(),
                        name: attachment.name.clone(),
                        mimetype: attachment.mimetype.clone(),
                        size: bytes.len(),
                    });
                    write.files.push(SavedFile {
                        conversation: conversation.to_owned(),
                        sha256,
                        bytes,
                    });
                }
                Err(SpruceError::FileTooLarge { .. }) => records.push(AttachmentRecord::TooLarge {
                    name: attachment.name.clone(),
                    mimetype: attachment.mimetype.clone(),
                }),
                // A signed link carries no credential, so a refusal here is an
                // expired link, not a revoked grant: the full read re-signs it.
                Err(_) => records.push(AttachmentRecord::Owed {
                    name: attachment.name.clone(),
                    mimetype: attachment.mimetype.clone(),
                }),
            }
        }
        Ok(records)
    }

    /// Who a conversation is with. `None` when Spruce could not say for now.
    async fn details(&self, conversation: &str) -> Result<Option<ConversationDetails>, Stop> {
        let conversation = match self.client.get_conversation(conversation).await {
            Ok(conversation) => conversation,
            Err(SpruceError::AuthorizationRejected) => return Err(Stop::Refused),
            Err(_) => return Ok(None),
        };
        let mut ids: Vec<String> = conversation.associated_contact_ids.clone();
        for participant in &conversation.external_participants {
            if let Some(contact) = &participant.contact
                && !ids.contains(contact)
            {
                ids.push(contact.clone());
            }
        }
        let mut contacts = Vec::new();
        for id in ids {
            match self.client.get_contact(&id).await {
                Ok(contact) => contacts.push(contact_record(contact)),
                Err(SpruceError::AuthorizationRejected) => return Err(Stop::Refused),
                Err(SpruceError::NotFound) => {}
                Err(_) => return Ok(None),
            }
        }
        Ok(Some(ConversationDetails {
            participants: participants(&conversation),
            contacts,
        }))
    }

    /// Full reads of owed conversations, `owed_pages` item pages each per
    /// pass. A read that runs out of pages resumes from the stored page token
    /// on the next pass, so a long conversation is read to its end across
    /// passes. It settles only when a sweep reached the end with nothing left
    /// unsaved; otherwise the next sweep starts again from the newest page.
    /// Before the backlog is accepted, a full read keeps only items after the
    /// connection.
    async fn owed(&self, intake: &Intake, cursor: &Cursor) -> Result<usize, Stop> {
        let owed = self
            .store
            .owed_reads(&self.lease.practice, intake.owed_reads)
            .await?;
        let mut kept = 0;
        for (entry, _) in owed {
            let conversation = entry.conversation;
            let mut write = PageWrite::default();
            let mut token = entry.resume;
            let mut dirty = entry.dirty;
            let mut reached_end = false;
            let mut stopped = false;
            for _ in 0..intake.owed_pages {
                let read = self
                    .reader
                    .read_conversation(&conversation, token.as_deref())
                    .await
                    .map_err(|error| Stop::Failed(error.into()))?;
                let (items, next, undecodable) = match read {
                    ConversationRead::Messages {
                        items,
                        next_page,
                        undecodable,
                    } => (items, next_page, undecodable),
                    ConversationRead::Missing => {
                        reached_end = true;
                        break;
                    }
                    ConversationRead::ReconnectRequired => return Err(Stop::Refused),
                    ConversationRead::Undecodable => {
                        write
                            .owed
                            .push((conversation.clone(), OwedCause::Undecodable));
                        stopped = true;
                        break;
                    }
                    ConversationRead::Unavailable => {
                        write
                            .owed
                            .push((conversation.clone(), OwedCause::TemporarilyUnavailable));
                        stopped = true;
                        break;
                    }
                };
                if undecodable {
                    write
                        .owed
                        .push((conversation.clone(), OwedCause::Undecodable));
                    dirty = true;
                }
                let items: Vec<Observed> = items
                    .into_iter()
                    .filter(|o| self.history || !cursor.is_history(observed_created(o)))
                    .collect();
                let before = write.owed.len();
                self.observed(items, &mut write).await?;
                dirty |= write.owed.len() != before;
                token = next;
                if token.is_none() {
                    reached_end = true;
                    break;
                }
            }
            match (reached_end, stopped, dirty, token) {
                (true, _, false, _) => {
                    write.settled.push(conversation.clone());
                    if let Some(details) = self.details(&conversation).await? {
                        write.details.push((conversation.clone(), details));
                    }
                }
                // Out of pages for this pass: continue where it stopped.
                (false, false, dirty, Some(from)) => write
                    .attempted
                    .push((conversation.clone(), Attempt::Continue { from, dirty })),
                // Reached the end with something unsaved, or the read
                // failed: the next attempt sweeps again from the newest page.
                _ => write
                    .attempted
                    .push((conversation.clone(), Attempt::Restart)),
            }
            kept += self.store.apply_channel_page(self.lease, write).await?;
        }
        Ok(kept)
    }

    /// Lists one page of conversations for an accepted backlog and queues
    /// each for a full read.
    async fn backlog(&self) -> Result<(), Stop> {
        let connection = self.store.channel_connection(&self.lease.practice).await?;
        let Connection::Reading {
            backlog: Backlog::Accepted { through },
            ..
        } = connection
        else {
            return Ok(());
        };
        let query = match through.as_deref() {
            Some(token) => ListQuery {
                page_size: MAX_PAGE_SIZE as u32,
                start_from: None,
                pagination_token: Some(token),
            },
            None => ListQuery::first(MAX_PAGE_SIZE as u32),
        };
        let page = match self.client.list_conversations(query).await {
            Ok(page) => page,
            Err(SpruceError::AuthorizationRejected) => return Err(Stop::Refused),
            Err(SpruceError::Temporary { .. } | SpruceError::Unavailable(_)) => return Ok(()),
            Err(error) => return Err(Stop::Failed(error.into())),
        };
        let mut write = PageWrite::default();
        for entry in &page.entries {
            let id = match entry {
                ConversationEntry::Conversation(conversation) => conversation.id.clone(),
                ConversationEntry::Malformed { id: Some(id) } => id.clone(),
                ConversationEntry::Malformed { id: None } => continue,
            };
            write.owed.push((id, OwedCause::Backlog));
        }
        write.backlog = Some(
            match page.has_more.then_some(page.pagination_token).flatten() {
                Some(next) => Backlog::Accepted {
                    through: Some(next),
                },
                None => Backlog::Read,
            },
        );
        self.store.apply_channel_page(self.lease, write).await?;
        Ok(())
    }
}

fn owed_cause(cause: UnreadCause) -> OwedCause {
    match cause {
        UnreadCause::Undecodable => OwedCause::Undecodable,
        UnreadCause::TemporarilyUnavailable => OwedCause::TemporarilyUnavailable,
    }
}

fn observed_id(observed: &Observed) -> &str {
    match observed {
        Observed::Message(item) => item.id(),
        Observed::Unreadable(envelope) => &envelope.id,
    }
}

fn observed_created(observed: &Observed) -> Option<&str> {
    match observed {
        Observed::Message(item) => item.envelope.created_at.as_deref(),
        Observed::Unreadable(envelope) => envelope.created_at.as_deref(),
    }
}

fn transcript_pending(item: &Item) -> bool {
    matches!(
        &item.transcription,
        Some(TranscriptionState::Unfinished { status, .. }) if status == "pending"
    ) || matches!(
        &item.transcription,
        Some(TranscriptionState::Unavailable { .. })
    )
}

fn direction(envelope: &Envelope) -> Direction {
    match envelope.direction.as_deref() {
        Some("inbound") => Direction::Patient,
        Some("outbound") => Direction::Practice,
        other => Direction::Other(other.unwrap_or_default().to_owned()),
    }
}

fn body(item: &Item) -> Body {
    let Some(event) = &item.event else {
        return Body::Text {
            text: item.text.clone(),
        };
    };
    if event.voicemail.is_none() {
        return Body::Event {
            kind: event.kind.clone(),
            text: item.text.clone(),
            data: event.data.clone(),
        };
    }
    let (status, transcript, summary) = match &item.transcription {
        Some(TranscriptionState::Ready { text, summary, .. }) => {
            ("completed".to_owned(), Some(text.clone()), summary.clone())
        }
        Some(TranscriptionState::Unfinished {
            status,
            text,
            summary,
            ..
        }) => (
            status.clone(),
            (!text.is_empty()).then(|| text.clone()),
            summary.clone(),
        ),
        Some(TranscriptionState::NotFound { .. }) => ("not_found".to_owned(), None, None),
        Some(TranscriptionState::Unavailable { .. }) => ("unavailable".to_owned(), None, None),
        None => ("none".to_owned(), None, None),
    };
    Body::Voicemail {
        text: item.text.clone(),
        status,
        transcript,
        summary,
    }
}

fn message(
    item: &Item,
    conversation: String,
    version: String,
    attachments: Vec<AttachmentRecord>,
) -> MessageVersion {
    let envelope = &item.envelope;
    MessageVersion {
        item: envelope.id.clone(),
        conversation,
        version,
        direction: direction(envelope),
        author: envelope.author_name.clone(),
        internal_note: envelope.is_internal_note.unwrap_or(false),
        created_at: envelope.created_at.clone(),
        modified_at: envelope.modified_at.clone(),
        app_url: envelope.app_url.clone(),
        body: body(item),
        attachments,
        deleted: item.deleted,
    }
}

fn unreadable_version(envelope: &Envelope) -> String {
    format!(
        "unreadable:{}",
        envelope.modified_at.as_deref().unwrap_or("")
    )
}

fn unreadable(envelope: &Envelope, conversation: String, version: String) -> MessageVersion {
    MessageVersion {
        item: envelope.id.clone(),
        conversation,
        version,
        direction: direction(envelope),
        author: envelope.author_name.clone(),
        internal_note: envelope.is_internal_note.unwrap_or(false),
        created_at: envelope.created_at.clone(),
        modified_at: envelope.modified_at.clone(),
        app_url: envelope.app_url.clone(),
        body: Body::Unreadable,
        attachments: Vec::new(),
        deleted: false,
    }
}

fn participants(conversation: &Conversation) -> Vec<Participant> {
    conversation
        .external_participants
        .iter()
        .map(|participant| Participant {
            display_name: participant.display_name.clone(),
            contact: participant.contact.clone(),
            channel: participant
                .endpoint
                .as_ref()
                .and_then(|endpoint| endpoint.channel.clone()),
            address: participant.endpoint.as_ref().and_then(|endpoint| {
                endpoint
                    .raw_value
                    .clone()
                    .or_else(|| endpoint.display_value.clone())
            }),
        })
        .collect()
}

fn contact_record(contact: Contact) -> ContactRecord {
    ContactRecord {
        id: contact.id,
        category: contact.category,
        display_name: contact.display_name,
        given_name: contact.given_name,
        family_name: contact.family_name,
        date_of_birth: contact.date_of_birth,
        phone_numbers: contact.phone_numbers,
        email_addresses: contact.email_addresses,
        integration_links: contact
            .integration_links
            .into_iter()
            .map(|link| (link.kind, link.external_id))
            .collect(),
    }
}

/// Wake-up across every reading practice: one pass each, in turn. One
/// practice's failure does not stop the others.
pub async fn wake(
    store: &Store,
    intake: &Intake,
) -> Result<Vec<(String, Result<PassOutcome, IntakeError>)>, ChannelError> {
    let mut outcomes = Vec::new();
    for practice in store.reading_channels().await? {
        let outcome = pass(store, intake, &practice).await;
        outcomes.push((practice, outcome));
    }
    Ok(outcomes)
}
