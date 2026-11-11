//! The practice's messaging channel (Spruce in the MVP): its connection, the
//! single reader that saves its messages, and what Today reads about it.
//!
//! A practice has at most one connection row, and the row is the lease. A
//! reader claims it, and every page it saves is committed under that row's
//! lock after checking it still holds the lease, so a stale reader's commit
//! fails. One transaction saves a page's message versions, files and owed
//! reads, then stores the cursor: a crash before commit replays the page, and
//! a replay inserts nothing twice.
//!
//! Versions are append-only. An edit or a Spruce deletion is a new version
//! row; an item missing from a later read is not a deletion. Disconnect
//! deletes every message of a conversation that was never linked to a patient
//! and keeps the rest. Coda never sends in the channel.
//!
//! The credential is sealed with AES-256-GCM under `PHYSICIAN_CHANNEL_KEY` and
//! bound to the practice and organization it was set up for.
//!
//! Each row records the [`GENERATION`] of the release that last wrote it. An
//! older release refuses a row a newer one touched, so after a rollback that
//! practice stops filing and its status reports why until an operator acts.

use std::collections::{HashMap, HashSet};

use aes_gcm::{
    Aes256Gcm, KeyInit,
    aead::{Aead, Payload},
};
use rand::{TryRng, rngs::SysRng};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, Order, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait,
    sea_query::{NullOrdering, OnConflict, Query},
};
use serde::{Deserialize, Serialize};

use super::entities::{channel_connection, channel_conversation, channel_file, channel_message};
use super::{Store, now};

/// One bounded intake pass over a practice's channel.
pub mod intake;

/// Largest attachment Coda saves. A larger file is recorded as too large.
pub const MAX_FILE_BYTES: usize = 25 * 1024 * 1024;

/// The channel schema generation this release writes. Raise it when a change
/// would let an older release misread rows this one writes.
pub const GENERATION: i64 = 1;

/// One practice's channel connection.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Connection {
    /// Coda reads the channel.
    Reading {
        /// The Spruce organization bound to this practice.
        organization: String,
        /// The practice member who set the connection up.
        bound_by: String,
        /// Whether messages from before the connection may be read.
        backlog: Backlog,
    },
    /// The grant was revoked or blocked. The cursor stays where it was, and a
    /// reconnect with the same organization resumes from it.
    ReconnectRequired {
        /// The organization still bound to this practice.
        organization: String,
        /// The practice member who set the connection up.
        bound_by: String,
        /// The backlog state to resume with.
        backlog: Backlog,
        /// Why reading stopped.
        reason: AccessLoss,
    },
    /// Not connected. Nothing never linked to a patient is kept.
    Disconnected,
}

/// Messages from before the connection. Coda reads them only after a
/// practice member accepts the offer.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "backlog", rename_all = "snake_case")]
pub enum Backlog {
    /// Offered and not accepted. History stays unread.
    Offered,
    /// Accepted. Conversations are being listed from `through`.
    Accepted {
        /// Spruce's page token for the next conversation page, if any.
        through: Option<String>,
    },
    /// Every conversation has been queued for a full read.
    Read,
}

impl Backlog {
    fn allows_history(&self) -> bool {
        !matches!(self, Self::Offered)
    }
}

/// Why the channel asks for a reconnect.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccessLoss {
    /// Spruce refused the credential.
    Revoked,
    /// An administrator blocked the grant.
    Blocked,
}

/// Why Coda can't read the channel right now.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "failure", rename_all = "snake_case")]
pub enum ReadFailure {
    /// Spruce did not answer. Reading resumes from the same place.
    Unavailable {
        /// Database-clock seconds of the first failed pass in this run.
        since: i64,
    },
    /// The grant is gone. Today asks for a reconnect.
    AccessLost {
        /// Revoked or blocked.
        reason: AccessLoss,
        /// Database-clock seconds it was noticed.
        since: i64,
    },
}

/// What Today reads about the channel. A failure never erases the last good read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadStatus {
    /// The connection.
    pub connection: Connection,
    /// Database-clock seconds of the last page Coda saved.
    pub last_read_at: Option<i64>,
    /// Why the channel can't be read, while it can't.
    pub failure: Option<ReadFailure>,
    /// Conversations still owed a full read, and why.
    pub unread: Vec<UnreadConversation>,
}

/// A conversation Coda could not finish reading.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnreadConversation {
    /// Spruce conversation id.
    pub conversation: String,
    /// Why, and since when.
    pub owed: Owed,
}

/// Why a conversation is owed a full read.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OwedCause {
    /// Something in it had no shape Coda can record.
    Undecodable,
    /// The read failed for now, a file link failed, or a transcript is not ready.
    TemporarilyUnavailable,
    /// The accepted backlog queued it.
    Backlog,
    /// A routine full reread, which finds edits and deletion flags the change
    /// reader can't see. Not a failure, so Today does not list it as unread.
    Recheck,
}

/// An owed full read.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Owed {
    /// Why it is owed.
    pub cause: OwedCause,
    /// Database-clock seconds it was first owed.
    pub since: i64,
}

/// An owed read as stored: where a read spread over several passes resumes,
/// and whether a page already read in this sweep left something unsaved.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct StoredOwed {
    cause: OwedCause,
    since: i64,
    #[serde(default)]
    resume: Option<String>,
    #[serde(default)]
    dirty: bool,
}

/// An owed conversation for the reader: why, and where its read resumes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwedRead {
    /// Spruce conversation id.
    pub conversation: String,
    /// Why it is owed.
    pub cause: OwedCause,
    /// Spruce item page token to continue from; `None` starts from the newest.
    pub resume: Option<String>,
    /// A page read earlier in this sweep left something unsaved, so reaching
    /// the end starts the sweep over instead of settling.
    pub dirty: bool,
}

/// Where an attempted owed read stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attempt {
    /// More pages remain; continue from this token. `dirty` is whether any
    /// page of the sweep so far left something unsaved.
    Continue {
        /// Spruce item page token.
        from: String,
        /// Something in this sweep is still unsaved.
        dirty: bool,
    },
    /// Start the next attempt from the newest page.
    Restart,
}

/// Who a conversation is with, as Spruce lists it.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversationDetails {
    /// People outside the practice.
    pub participants: Vec<Participant>,
    /// Saved contacts on or associated with the conversation.
    pub contacts: Vec<ContactRecord>,
}

/// Someone outside the practice on a conversation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Participant {
    /// Name Spruce showed.
    pub display_name: Option<String>,
    /// Saved-contact id, when they are a saved contact.
    pub contact: Option<String>,
    /// Spruce's channel string.
    pub channel: Option<String>,
    /// The address they used.
    pub address: Option<String>,
}

/// A saved Spruce contact.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactRecord {
    /// Spruce contact id.
    pub id: String,
    /// Spruce's category string.
    pub category: Option<String>,
    /// Name Spruce shows.
    pub display_name: Option<String>,
    /// Given name.
    pub given_name: Option<String>,
    /// Family name.
    pub family_name: Option<String>,
    /// Date of birth as Spruce sent it.
    pub date_of_birth: Option<String>,
    /// Phone numbers.
    pub phone_numbers: Vec<String>,
    /// Email addresses.
    pub email_addresses: Vec<String>,
    /// `(type, external id)` links to other systems, such as an EHR.
    pub integration_links: Vec<(String, String)>,
}

/// Who a message is from.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    /// Spruce's `inbound`: someone outside the practice wrote it.
    Patient,
    /// Spruce's `outbound`: the practice sent it in Spruce.
    Practice,
    /// A direction Spruce added later, kept as sent.
    Other(String),
}

/// A message's content.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "body", rename_all = "snake_case")]
pub enum Body {
    /// Written text.
    Text {
        /// The text.
        text: String,
    },
    /// A voicemail. The recording is never fetched.
    Voicemail {
        /// Any text on the item.
        text: String,
        /// Spruce's transcript status (`completed`, `pending`, `not_found`, `unavailable`, …).
        status: String,
        /// Transcript text, when Spruce has one.
        transcript: Option<String>,
        /// Spruce's summary, when it sent one.
        summary: Option<String>,
    },
    /// A call or other Spruce event, without recordings or signed links.
    Event {
        /// Spruce's event type.
        kind: String,
        /// Any text on the item.
        text: String,
        /// The event's lasting fields.
        data: serde_json::Value,
    },
    /// Coda could not read it. It stays addressable by id with its Spruce link.
    Unreadable,
}

/// Something attached to a message.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "attachment", rename_all = "snake_case")]
pub enum AttachmentRecord {
    /// Bytes Coda saved, readable after Spruce's link expires.
    File {
        /// Lower-case hex SHA-256 of the bytes.
        sha256: String,
        /// File name Spruce sent.
        name: Option<String>,
        /// MIME type Spruce sent.
        mimetype: Option<String>,
        /// Size in bytes.
        size: usize,
    },
    /// Larger than [`MAX_FILE_BYTES`]; not saved.
    TooLarge {
        /// File name Spruce sent.
        name: Option<String>,
        /// MIME type Spruce sent.
        mimetype: Option<String>,
    },
    /// An attachment without a file, such as a card.
    Card {
        /// Spruce's attachment type.
        kind: String,
        /// Title Spruce sent.
        title: Option<String>,
    },
}

/// One saved version of a channel message. Append-only.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageVersion {
    /// Spruce item id.
    pub item: String,
    /// Spruce conversation id.
    pub conversation: String,
    /// `spruce::payload::Item::version`, or `unreadable:` and `modifiedAt`.
    pub version: String,
    /// Who it is from.
    pub direction: Direction,
    /// Author name Spruce showed.
    pub author: Option<String>,
    /// Spruce's internal-note flag.
    pub internal_note: bool,
    /// When Spruce created it.
    pub created_at: Option<String>,
    /// When Spruce last changed it.
    pub modified_at: Option<String>,
    /// Link to it in Spruce's app.
    pub app_url: Option<String>,
    /// Its content.
    pub body: Body,
    /// Its attachments.
    pub attachments: Vec<AttachmentRecord>,
    /// Spruce flagged it deleted. Earlier versions stay.
    pub deleted: bool,
}

/// File bytes to save with a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SavedFile {
    /// The conversation the file arrived in.
    pub conversation: String,
    /// Lower-case hex SHA-256 of `bytes`.
    pub sha256: String,
    /// The bytes.
    pub bytes: Vec<u8>,
}

/// Everything one page saves, in one transaction.
#[derive(Clone, Debug, Default)]
pub struct PageWrite {
    /// Versions to keep. One already kept is skipped.
    pub versions: Vec<MessageVersion>,
    /// Files those versions name.
    pub files: Vec<SavedFile>,
    /// Conversations now owed a full read.
    pub owed: Vec<(String, OwedCause)>,
    /// Conversations whose owed read finished.
    pub settled: Vec<String>,
    /// Conversations whose owed read was attempted and did not settle, and
    /// where the next attempt starts. Each pass begins with the
    /// conversations attempted longest ago.
    pub attempted: Vec<(String, Attempt)>,
    /// Conversation details read with this page.
    pub details: Vec<(String, ConversationDetails)>,
    /// The cursor to store. Present only for a change page, which also
    /// records the read time and clears a channel failure.
    pub cursor: Option<String>,
    /// The backlog state after this page, when it moved.
    pub backlog: Option<Backlog>,
}

/// A held lease on one practice's channel. Commits fail once it is lost.
#[derive(Clone, Debug)]
pub struct Lease {
    /// Practice.
    pub practice: String,
    token: String,
    seconds: i64,
    /// The connection when the lease was claimed.
    pub connection: Connection,
    /// The stored cursor when the lease was claimed.
    pub cursor: Option<String>,
    credential: Option<Vec<u8>>,
}

/// A store operation on the channel that did not happen.
#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    /// Postgres failed.
    #[error("channel store failed")]
    Database(#[from] DbErr),
    /// Another reader holds the lease now, or the connection changed under it.
    #[error("channel lease is no longer held")]
    LeaseLost,
    /// Another practice already reads this organization.
    #[error("the Spruce organization is connected to another practice")]
    OrganizationTaken,
    /// The practice is connected to a different organization. Disconnect first.
    #[error("the practice is connected to a different Spruce organization")]
    DifferentOrganization,
    /// The operation needs a connection in another state.
    #[error("the channel is not in a state that allows this")]
    WrongState,
    /// A newer release wrote this practice's channel. This release leaves it
    /// alone until an operator acts.
    #[error("channel generation {found} was written by a newer release than {GENERATION}")]
    NewerRelease {
        /// The generation on the row.
        found: i64,
    },
    /// A row the operation had just written was not there.
    #[error("stored channel {0} is missing")]
    Missing(&'static str),
    /// A stored value could not be read back, or a value could not be stored.
    #[error("stored channel {what} is unreadable")]
    Corrupt {
        /// Which value.
        what: &'static str,
        /// The JSON error.
        #[source]
        source: serde_json::Error,
    },
    /// The credential could not be sealed or opened.
    #[error("channel credential could not be {action}")]
    Seal {
        /// `sealed` or `opened`.
        action: &'static str,
        /// Why.
        #[source]
        source: SealError,
    },
}

/// Why a credential could not be sealed or opened.
#[derive(Debug, thiserror::Error)]
pub enum SealError {
    /// The system random source failed.
    #[error("no system randomness")]
    Random(#[source] rand::rngs::SysError),
    /// Encryption failed, or the ciphertext, practice or organization does
    /// not match the key.
    #[error("authenticated encryption failed")]
    Cipher(#[source] aes_gcm::Error),
    /// The stored value is too short to hold a nonce and tag.
    #[error("sealed credential is truncated")]
    Truncated,
    /// The opened credential is not text.
    #[error("opened credential is not UTF-8")]
    Text(#[source] std::string::FromUtf8Error),
}

/// The key that seals channel credentials at rest.
#[derive(Clone)]
pub struct ChannelKey([u8; 32]);

impl std::fmt::Debug for ChannelKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ChannelKey(..)")
    }
}

impl ChannelKey {
    /// A key from 64 hex digits, the form `PHYSICIAN_CHANNEL_KEY` holds.
    pub fn from_hex(text: &str) -> Option<Self> {
        let text = text.trim();
        if text.len() != 64 {
            return None;
        }
        let mut key = [0; 32];
        for (index, byte) in key.iter_mut().enumerate() {
            *byte = u8::from_str_radix(text.get(index * 2..index * 2 + 2)?, 16).ok()?;
        }
        Some(Self(key))
    }

    /// A key from raw bytes.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    fn aad(practice: &str, organization: &str) -> Vec<u8> {
        [practice.as_bytes(), &[0], organization.as_bytes()].concat()
    }

    fn seal(
        &self,
        practice: &str,
        organization: &str,
        credential: &str,
    ) -> Result<Vec<u8>, ChannelError> {
        let sealing = |source| ChannelError::Seal {
            action: "sealed",
            source,
        };
        let mut nonce = [0; 12];
        SysRng
            .try_fill_bytes(&mut nonce)
            .map_err(|error| sealing(SealError::Random(error)))?;
        let aad = Self::aad(practice, organization);
        let sealed = Aes256Gcm::new(&self.0.into())
            .encrypt(
                (&nonce).into(),
                Payload {
                    msg: credential.as_bytes(),
                    aad: &aad,
                },
            )
            .map_err(|error| sealing(SealError::Cipher(error)))?;
        Ok([nonce.to_vec(), sealed].concat())
    }

    fn open(
        &self,
        practice: &str,
        organization: &str,
        sealed: &[u8],
    ) -> Result<String, ChannelError> {
        let opening = |source| ChannelError::Seal {
            action: "opened",
            source,
        };
        let Some((nonce, ciphertext)) = sealed.split_first_chunk::<12>() else {
            return Err(opening(SealError::Truncated));
        };
        if ciphertext.len() < 16 {
            return Err(opening(SealError::Truncated));
        }
        let aad = Self::aad(practice, organization);
        let plain = Aes256Gcm::new(&self.0.into())
            .decrypt(
                nonce.into(),
                Payload {
                    msg: ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|error| opening(SealError::Cipher(error)))?;
        String::from_utf8(plain).map_err(|error| opening(SealError::Text(error)))
    }
}

fn to_json<T: Serialize>(value: &T, what: &'static str) -> Result<serde_json::Value, ChannelError> {
    serde_json::to_value(value).map_err(|source| ChannelError::Corrupt { what, source })
}

fn from_json<T: for<'de> Deserialize<'de>>(
    value: serde_json::Value,
    what: &'static str,
) -> Result<T, ChannelError> {
    serde_json::from_value(value).map_err(|source| ChannelError::Corrupt { what, source })
}

/// Refuses a row a newer release wrote.
fn compatible(row: &channel_connection::Model) -> Result<(), ChannelError> {
    if row.generation > GENERATION {
        return Err(ChannelError::NewerRelease {
            found: row.generation,
        });
    }
    Ok(())
}

fn connection_of(row: &channel_connection::Model) -> Result<Connection, ChannelError> {
    from_json(row.state.clone(), "connection")
}

/// Lower-case hex SHA-256 of `bytes`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

impl Store {
    /// Creates the practice's row as disconnected if it has none, then locks it.
    async fn locked_channel(
        &self,
        practice: &str,
    ) -> Result<(DatabaseTransaction, channel_connection::Model, i64), ChannelError> {
        if practice.trim().is_empty() {
            return Err(ChannelError::WrongState);
        }
        let tx = self.db.begin().await?;
        channel_connection::Entity::insert(channel_connection::ActiveModel {
            practice: Set(practice.to_owned()),
            state: Set(to_json(&Connection::Disconnected, "connection")?),
            organization: Set(None),
            credential: Set(None),
            cursor: Set(None),
            lease_holder: Set(None),
            lease_token: Set(None),
            lease_expires: Set(0),
            generation: Set(GENERATION),
            last_read_at: Set(None),
            failure: Set(None),
        })
        .on_conflict(
            OnConflict::column(channel_connection::Column::Practice)
                .do_nothing()
                .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        let row = channel_connection::Entity::find_by_id(practice.to_owned())
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or(ChannelError::Missing("connection"))?;
        compatible(&row)?;
        let now = now(&tx).await?;
        Ok((tx, row, now))
    }

    /// Locks the row and checks `lease` still holds it. Every claim and
    /// every credential change writes a fresh token, so only the latest claim
    /// on the current credential passes.
    async fn fenced(
        &self,
        lease: &Lease,
    ) -> Result<(DatabaseTransaction, channel_connection::Model, i64), ChannelError> {
        let (tx, row, now) = self.locked_channel(&lease.practice).await?;
        if row.lease_token.as_deref() != Some(lease.token.as_str()) || row.lease_expires <= now {
            return Err(ChannelError::LeaseLost);
        }
        Ok((tx, row, now))
    }

    /// The practice's connection. `Disconnected` when it never connected.
    pub async fn channel_connection(&self, practice: &str) -> Result<Connection, ChannelError> {
        match channel_connection::Entity::find_by_id(practice.to_owned())
            .one(&self.db)
            .await?
        {
            Some(row) => connection_of(&row),
            None => Ok(Connection::Disconnected),
        }
    }

    /// The stored cursor, exactly as the last committed page left it.
    pub async fn channel_cursor(&self, practice: &str) -> Result<Option<String>, ChannelError> {
        Ok(channel_connection::Entity::find_by_id(practice.to_owned())
            .one(&self.db)
            .await?
            .and_then(|row| row.cursor))
    }

    /// Binds a checked organization to the practice.
    ///
    /// A disconnected practice starts reading new messages and is offered the
    /// backlog. A practice that must reconnect resumes from its cursor when
    /// the organization is the same. A connected practice with the same
    /// organization replaces its credential. Another practice's organization
    /// is refused.
    pub(super) async fn bind_channel(
        &self,
        key: &ChannelKey,
        practice: &str,
        organization: &str,
        bound_by: &str,
        credential: &str,
    ) -> Result<Connection, ChannelError> {
        if organization.trim().is_empty() || bound_by.trim().is_empty() {
            return Err(ChannelError::WrongState);
        }
        let (tx, row, _) = self.locked_channel(practice).await?;
        let taken = channel_connection::Entity::find()
            .filter(channel_connection::Column::Organization.eq(organization))
            .filter(channel_connection::Column::Practice.ne(practice))
            .one(&tx)
            .await?;
        if taken.is_some() {
            return Err(ChannelError::OrganizationTaken);
        }
        let sealed = key.seal(practice, organization, credential)?;
        let reading = |backlog| Connection::Reading {
            organization: organization.to_owned(),
            bound_by: bound_by.to_owned(),
            backlog,
        };
        let mut active: channel_connection::ActiveModel = row.clone().into();
        let next = match connection_of(&row)? {
            Connection::Disconnected => {
                active.cursor = Set(None);
                active.last_read_at = Set(None);
                reading(Backlog::Offered)
            }
            Connection::ReconnectRequired {
                organization: bound,
                backlog,
                ..
            }
            | Connection::Reading {
                organization: bound,
                backlog,
                ..
            } if bound == organization => reading(backlog),
            _ => return Err(ChannelError::DifferentOrganization),
        };
        active.state = Set(to_json(&next, "connection")?);
        active.organization = Set(Some(organization.to_owned()));
        active.credential = Set(Some(sealed));
        active.failure = Set(None);
        // A reader still holding the old credential must not commit after
        // the new one is in place, or it could undo this reconnect.
        active.lease_holder = Set(None);
        active.lease_token = Set(None);
        active.lease_expires = Set(0);
        active.generation = Set(GENERATION);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(next)
    }

    /// Accepts the backlog offer: earlier messages are read from the next pass.
    pub async fn accept_channel_backlog(&self, practice: &str) -> Result<(), ChannelError> {
        let (tx, row, _) = self.locked_channel(practice).await?;
        let Connection::Reading {
            organization,
            bound_by,
            backlog: Backlog::Offered,
        } = connection_of(&row)?
        else {
            return Err(ChannelError::WrongState);
        };
        let mut active: channel_connection::ActiveModel = row.into();
        active.state = Set(to_json(
            &Connection::Reading {
                organization,
                bound_by,
                backlog: Backlog::Accepted { through: None },
            },
            "connection",
        )?);
        active.generation = Set(GENERATION);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Disconnects. Every message, file and conversation never linked to a
    /// patient is deleted; linked evidence stays under the record's policy.
    /// The credential, cursor and lease are cleared, so a running reader's
    /// next commit fails.
    pub async fn disconnect_channel(&self, practice: &str) -> Result<(), ChannelError> {
        let (tx, row, _) = self.locked_channel(practice).await?;
        let linked = || {
            Query::select()
                .column(channel_conversation::Column::Conversation)
                .from(channel_conversation::Entity)
                .and_where(channel_conversation::Column::Practice.eq(practice))
                .and_where(channel_conversation::Column::Link.is_not_null())
                .to_owned()
        };
        channel_message::Entity::delete_many()
            .filter(channel_message::Column::Practice.eq(practice))
            .filter(channel_message::Column::Conversation.not_in_subquery(linked()))
            .exec(&tx)
            .await?;
        channel_file::Entity::delete_many()
            .filter(channel_file::Column::Practice.eq(practice))
            .filter(channel_file::Column::Conversation.not_in_subquery(linked()))
            .exec(&tx)
            .await?;
        channel_conversation::Entity::delete_many()
            .filter(channel_conversation::Column::Practice.eq(practice))
            .filter(channel_conversation::Column::Link.is_null())
            .exec(&tx)
            .await?;
        let mut active: channel_connection::ActiveModel = row.into();
        active.state = Set(to_json(&Connection::Disconnected, "connection")?);
        active.organization = Set(None);
        active.credential = Set(None);
        active.cursor = Set(None);
        active.lease_holder = Set(None);
        active.lease_token = Set(None);
        active.lease_expires = Set(0);
        active.failure = Set(None);
        active.generation = Set(GENERATION);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Claims the practice's single reader lease. `None` while another live
    /// reader holds it.
    pub async fn claim_channel(
        &self,
        practice: &str,
        holder: &str,
        seconds: i64,
    ) -> Result<Option<Lease>, ChannelError> {
        if !(1..=900).contains(&seconds) || holder.trim().is_empty() {
            return Err(ChannelError::WrongState);
        }
        let (tx, row, now) = self.locked_channel(practice).await?;
        if row.lease_token.is_some() && row.lease_expires > now {
            tx.commit().await?;
            return Ok(None);
        }
        let token = uuid::Uuid::new_v4().to_string();
        let lease = Lease {
            practice: practice.to_owned(),
            token: token.clone(),
            seconds,
            connection: connection_of(&row)?,
            cursor: row.cursor.clone(),
            credential: row.credential.clone(),
        };
        let mut active: channel_connection::ActiveModel = row.into();
        active.lease_holder = Set(Some(holder.to_owned()));
        active.lease_token = Set(Some(token));
        active.lease_expires = Set(now + seconds);
        active.generation = Set(GENERATION);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(Some(lease))
    }

    /// Gives the lease back so the next wake-up need not wait for it to run out.
    pub async fn release_channel(&self, lease: &Lease) -> Result<(), ChannelError> {
        let (tx, row, _) = match self.fenced(lease).await {
            Ok(locked) => locked,
            Err(ChannelError::LeaseLost) => return Ok(()),
            Err(error) => return Err(error),
        };
        let mut active: channel_connection::ActiveModel = row.into();
        active.lease_holder = Set(None);
        active.lease_token = Set(None);
        active.lease_expires = Set(0);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Opens the credential the lease was claimed with.
    pub fn channel_credential(
        &self,
        key: &ChannelKey,
        lease: &Lease,
    ) -> Result<String, ChannelError> {
        let Connection::Reading { organization, .. } = &lease.connection else {
            return Err(ChannelError::WrongState);
        };
        let sealed = lease
            .credential
            .as_deref()
            .ok_or(ChannelError::Missing("credential"))?;
        key.open(&lease.practice, organization, sealed)
    }

    /// Saves one page under the lease, all or nothing, and extends the lease.
    /// Returns how many new versions were kept.
    pub async fn apply_channel_page(
        &self,
        lease: &Lease,
        page: PageWrite,
    ) -> Result<usize, ChannelError> {
        let (tx, row, now) = self.fenced(lease).await?;
        let practice = lease.practice.as_str();
        let mut connection = connection_of(&row)?;
        if !matches!(connection, Connection::Reading { .. }) {
            return Err(ChannelError::LeaseLost);
        }

        let items: Vec<String> = page.versions.iter().map(|v| v.item.clone()).collect();
        let mut existing: HashSet<(String, String)> = HashSet::new();
        let mut ordinals: HashMap<String, i64> = HashMap::new();
        if !items.is_empty() {
            for stored in channel_message::Entity::find()
                .filter(channel_message::Column::Practice.eq(practice))
                .filter(channel_message::Column::Item.is_in(items))
                .all(&tx)
                .await?
            {
                let ordinal = ordinals.entry(stored.item.clone()).or_insert(0);
                *ordinal = (*ordinal).max(stored.ordinal);
                existing.insert((stored.item, stored.version));
            }
        }
        let mut rows = Vec::new();
        let mut conversations: HashSet<String> = HashSet::new();
        for version in &page.versions {
            conversations.insert(version.conversation.clone());
            if !existing.insert((version.item.clone(), version.version.clone())) {
                continue;
            }
            let ordinal = ordinals.entry(version.item.clone()).or_insert(0);
            *ordinal += 1;
            rows.push(channel_message::ActiveModel {
                practice: Set(practice.to_owned()),
                item: Set(version.item.clone()),
                version: Set(version.version.clone()),
                conversation: Set(version.conversation.clone()),
                ordinal: Set(*ordinal),
                recorded_at: Set(now),
                record: Set(to_json(version, "message")?),
            });
        }
        let kept = rows.len();
        for batch in rows.chunks(100) {
            channel_message::Entity::insert_many(batch.to_vec())
                .on_conflict(
                    OnConflict::columns([
                        channel_message::Column::Practice,
                        channel_message::Column::Item,
                        channel_message::Column::Version,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec_without_returning(&tx)
                .await?;
        }
        for file in &page.files {
            channel_file::Entity::insert(channel_file::ActiveModel {
                practice: Set(practice.to_owned()),
                conversation: Set(file.conversation.clone()),
                sha256: Set(file.sha256.clone()),
                bytes: Set(file.bytes.clone()),
                saved_at: Set(now),
            })
            .on_conflict(
                OnConflict::columns([
                    channel_file::Column::Practice,
                    channel_file::Column::Conversation,
                    channel_file::Column::Sha256,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec_without_returning(&tx)
            .await?;
            conversations.insert(file.conversation.clone());
        }

        let mut touched: HashMap<String, ConversationChange> = HashMap::new();
        for conversation in conversations {
            touched.entry(conversation).or_default();
        }
        for conversation in &page.settled {
            touched.entry(conversation.clone()).or_default().settle = true;
        }
        if !page.attempted.is_empty() {
            let last: Option<channel_conversation::Model> = channel_conversation::Entity::find()
                .filter(channel_conversation::Column::Practice.eq(practice))
                .filter(channel_conversation::Column::OwedAttempt.is_not_null())
                .order_by_desc(channel_conversation::Column::OwedAttempt)
                .one(&tx)
                .await?;
            let mut turn = last.and_then(|row| row.owed_attempt).unwrap_or(0);
            for (conversation, attempt) in &page.attempted {
                turn += 1;
                touched.entry(conversation.clone()).or_default().attempt =
                    Some((attempt.clone(), turn));
            }
        }
        for (conversation, cause) in &page.owed {
            touched.entry(conversation.clone()).or_default().owe = Some(*cause);
        }
        for (conversation, details) in &page.details {
            touched.entry(conversation.clone()).or_default().details = Some(details.clone());
        }
        for (conversation, change) in touched {
            self.change_conversation(&tx, practice, &conversation, change, now)
                .await?;
        }

        let mut active: channel_connection::ActiveModel = row.into();
        if let Some(backlog) = page.backlog
            && let Connection::Reading {
                backlog: current, ..
            } = &mut connection
        {
            *current = backlog;
            active.state = Set(to_json(&connection, "connection")?);
        }
        if let Some(cursor) = page.cursor {
            active.cursor = Set(Some(cursor));
            active.last_read_at = Set(Some(now));
            active.failure = Set(None);
        }
        active.lease_expires = Set(now + lease.seconds);
        active.generation = Set(GENERATION);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(kept)
    }

    async fn change_conversation(
        &self,
        tx: &DatabaseTransaction,
        practice: &str,
        conversation: &str,
        change: ConversationChange,
        now: i64,
    ) -> Result<(), ChannelError> {
        channel_conversation::Entity::insert(channel_conversation::ActiveModel {
            practice: Set(practice.to_owned()),
            conversation: Set(conversation.to_owned()),
            details: Set(None),
            owed: Set(None),
            owed_attempt: Set(None),
            checked_at: Set(now),
            link: Set(None),
        })
        .on_conflict(
            OnConflict::columns([
                channel_conversation::Column::Practice,
                channel_conversation::Column::Conversation,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec_without_returning(tx)
        .await?;
        let row = channel_conversation::Entity::find_by_id((
            practice.to_owned(),
            conversation.to_owned(),
        ))
        .lock_exclusive()
        .one(tx)
        .await?
        .ok_or(ChannelError::Missing("conversation"))?;
        let mut owed: Option<StoredOwed> = row
            .owed
            .clone()
            .map(|value| from_json(value, "owed read"))
            .transpose()?;
        let mut active: channel_conversation::ActiveModel = row.into();
        if change.settle {
            owed = None;
            active.owed_attempt = Set(None);
            active.checked_at = Set(now);
        }
        if let Some((attempt, turn)) = change.attempt {
            active.owed_attempt = Set(Some(turn));
            if let Some(current) = owed.as_mut() {
                (current.resume, current.dirty) = match attempt {
                    Attempt::Continue { from, dirty } => (Some(from), dirty),
                    Attempt::Restart => (None, false),
                };
            }
        }
        if let Some(cause) = change.owe {
            owed = Some(match owed {
                // A failure outranks a routine recheck, and the earliest time stays.
                Some(current) => StoredOwed {
                    cause: if cause == OwedCause::Recheck {
                        current.cause
                    } else {
                        cause
                    },
                    // Only a sweep under way has earlier pages to spoil.
                    dirty: current.dirty
                        || (cause != OwedCause::Recheck && current.resume.is_some()),
                    ..current
                },
                None => StoredOwed {
                    cause,
                    since: now,
                    resume: None,
                    dirty: false,
                },
            });
        }
        active.owed = Set(owed.map(|owed| to_json(&owed, "owed read")).transpose()?);
        if let Some(details) = change.details {
            active.details = Set(Some(to_json(&details, "conversation")?));
        }
        active.update(tx).await?;
        Ok(())
    }

    /// Records that Spruce did not answer. The cursor and last read time stay.
    pub async fn channel_unavailable(&self, lease: &Lease) -> Result<(), ChannelError> {
        let (tx, row, now) = self.fenced(lease).await?;
        let since = match row
            .failure
            .clone()
            .map(|value| from_json::<ReadFailure>(value, "failure"))
            .transpose()?
        {
            Some(ReadFailure::Unavailable { since }) => since,
            _ => now,
        };
        let mut active: channel_connection::ActiveModel = row.into();
        active.failure = Set(Some(to_json(
            &ReadFailure::Unavailable { since },
            "failure",
        )?));
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }

    /// The grant is gone: the connection asks for a reconnect, the cursor stays.
    pub async fn channel_access_lost(
        &self,
        lease: &Lease,
        reason: AccessLoss,
    ) -> Result<(), ChannelError> {
        let (tx, row, now) = self.fenced(lease).await?;
        let Connection::Reading {
            organization,
            bound_by,
            backlog,
        } = connection_of(&row)?
        else {
            return Err(ChannelError::LeaseLost);
        };
        let mut active: channel_connection::ActiveModel = row.into();
        active.state = Set(to_json(
            &Connection::ReconnectRequired {
                organization,
                bound_by,
                backlog,
                reason,
            },
            "connection",
        )?);
        active.failure = Set(Some(to_json(
            &ReadFailure::AccessLost { reason, since: now },
            "failure",
        )?));
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }

    /// What Today reads about the practice's channel.
    pub async fn channel_status(&self, practice: &str) -> Result<ReadStatus, ChannelError> {
        let row = channel_connection::Entity::find_by_id(practice.to_owned())
            .one(&self.db)
            .await?;
        let mut unread: Vec<UnreadConversation> = self
            .owed_reads(practice, 1_000)
            .await?
            .into_iter()
            .filter(|(read, _)| read.cause != OwedCause::Recheck)
            .map(|(read, since)| UnreadConversation {
                conversation: read.conversation,
                owed: Owed {
                    cause: read.cause,
                    since,
                },
            })
            .collect();
        unread.sort_by_key(|entry| entry.owed.since);
        let Some(row) = row else {
            return Ok(ReadStatus {
                connection: Connection::Disconnected,
                last_read_at: None,
                failure: None,
                unread,
            });
        };
        compatible(&row)?;
        Ok(ReadStatus {
            connection: connection_of(&row)?,
            last_read_at: row.last_read_at,
            failure: row
                .failure
                .map(|value| from_json(value, "failure"))
                .transpose()?,
            unread,
        })
    }

    /// Up to `limit` owed reads, never-attempted first and then the ones
    /// attempted longest ago, so a conversation that keeps failing does not
    /// hold back the rest. Each comes with when it was first owed.
    pub async fn owed_reads(
        &self,
        practice: &str,
        limit: u64,
    ) -> Result<Vec<(OwedRead, i64)>, ChannelError> {
        let rows = channel_conversation::Entity::find()
            .filter(channel_conversation::Column::Practice.eq(practice))
            .filter(channel_conversation::Column::Owed.is_not_null())
            .order_by_with_nulls(
                channel_conversation::Column::OwedAttempt,
                Order::Asc,
                NullOrdering::First,
            )
            .order_by_asc(channel_conversation::Column::Conversation)
            .limit(limit)
            .all(&self.db)
            .await?;
        let mut reads = Vec::new();
        for row in rows {
            let Some(owed) = row.owed else { continue };
            let owed: StoredOwed = from_json(owed, "owed read")?;
            reads.push((
                OwedRead {
                    conversation: row.conversation,
                    cause: owed.cause,
                    resume: owed.resume,
                    dirty: owed.dirty,
                },
                owed.since,
            ));
        }
        Ok(reads)
    }

    /// Queues a routine reread of up to `limit` conversations last checked
    /// over `every` seconds ago, least recently checked first, so every
    /// conversation of any age comes round. The change reader only sees new
    /// items, and Spruce does not say whether an edit or a deletion flag moves
    /// a conversation's activity, so this reread is how those arrive.
    pub async fn queue_rechecks(
        &self,
        lease: &Lease,
        every: i64,
        limit: u64,
    ) -> Result<usize, ChannelError> {
        let (tx, row, now) = self.fenced(lease).await?;
        let due = channel_conversation::Entity::find()
            .filter(channel_conversation::Column::Practice.eq(&lease.practice))
            .filter(channel_conversation::Column::Owed.is_null())
            .filter(channel_conversation::Column::CheckedAt.lte(now - every))
            .order_by_asc(channel_conversation::Column::CheckedAt)
            .limit(limit)
            .all(&tx)
            .await?;
        let queued = due.len();
        for conversation in due {
            self.change_conversation(
                &tx,
                &lease.practice,
                &conversation.conversation,
                ConversationChange {
                    owe: Some(OwedCause::Recheck),
                    ..ConversationChange::default()
                },
                now,
            )
            .await?;
        }
        let mut active: channel_connection::ActiveModel = row.into();
        active.lease_expires = Set(now + lease.seconds);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(queued)
    }

    /// Every saved version of one item, oldest first. Readable by id whether
    /// or not Coda could read its content.
    pub async fn channel_message(
        &self,
        practice: &str,
        item: &str,
    ) -> Result<Vec<MessageVersion>, ChannelError> {
        let rows = channel_message::Entity::find()
            .filter(channel_message::Column::Practice.eq(practice))
            .filter(channel_message::Column::Item.eq(item))
            .order_by_asc(channel_message::Column::Ordinal)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(|row| from_json(row.record, "message"))
            .collect()
    }

    /// The latest version of each item in a conversation, oldest item first.
    pub async fn channel_messages(
        &self,
        practice: &str,
        conversation: &str,
    ) -> Result<Vec<MessageVersion>, ChannelError> {
        let rows = channel_message::Entity::find()
            .filter(channel_message::Column::Practice.eq(practice))
            .filter(channel_message::Column::Conversation.eq(conversation))
            .order_by_asc(channel_message::Column::Item)
            .order_by_desc(channel_message::Column::Ordinal)
            .all(&self.db)
            .await?;
        let mut latest: Vec<MessageVersion> = Vec::new();
        let mut last_item: Option<String> = None;
        for row in rows {
            if last_item.as_deref() == Some(row.item.as_str()) {
                continue;
            }
            last_item = Some(row.item.clone());
            latest.push(from_json(row.record, "message")?);
        }
        latest.sort_by(|left, right| left.created_at.cmp(&right.created_at));
        Ok(latest)
    }

    /// `(item, version)` pairs already saved, among `items`.
    pub async fn channel_versions(
        &self,
        practice: &str,
        items: &[String],
    ) -> Result<HashSet<(String, String)>, ChannelError> {
        if items.is_empty() {
            return Ok(HashSet::new());
        }
        Ok(channel_message::Entity::find()
            .filter(channel_message::Column::Practice.eq(practice))
            .filter(channel_message::Column::Item.is_in(items.iter().cloned()))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| (row.item, row.version))
            .collect())
    }

    /// Saved file bytes, by the conversation they arrived in and their hash.
    pub async fn channel_file(
        &self,
        practice: &str,
        conversation: &str,
        sha256: &str,
    ) -> Result<Option<Vec<u8>>, ChannelError> {
        Ok(channel_file::Entity::find_by_id((
            practice.to_owned(),
            conversation.to_owned(),
            sha256.to_owned(),
        ))
        .one(&self.db)
        .await?
        .map(|row| row.bytes))
    }

    /// Who a conversation is with, once read.
    pub async fn channel_conversation(
        &self,
        practice: &str,
        conversation: &str,
    ) -> Result<Option<ConversationDetails>, ChannelError> {
        channel_conversation::Entity::find_by_id((practice.to_owned(), conversation.to_owned()))
            .one(&self.db)
            .await?
            .and_then(|row| row.details)
            .map(|value| from_json(value, "conversation"))
            .transpose()
    }

    /// Conversation ids among `conversations` whose details are saved.
    pub async fn channel_known_conversations(
        &self,
        practice: &str,
        conversations: &[String],
    ) -> Result<HashSet<String>, ChannelError> {
        if conversations.is_empty() {
            return Ok(HashSet::new());
        }
        Ok(channel_conversation::Entity::find()
            .filter(channel_conversation::Column::Practice.eq(practice))
            .filter(channel_conversation::Column::Conversation.is_in(conversations.iter().cloned()))
            .filter(channel_conversation::Column::Details.is_not_null())
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| row.conversation)
            .collect())
    }

    /// Connected practices, for the wake-up to visit. A pass on one that is
    /// not reading returns at once; one this release can't read fails its
    /// pass, so the wake-up reports it instead of skipping it.
    pub async fn reading_channels(&self) -> Result<Vec<String>, ChannelError> {
        Ok(channel_connection::Entity::find()
            .filter(channel_connection::Column::Organization.is_not_null())
            .order_by_asc(channel_connection::Column::Practice)
            .all(&self.db)
            .await?
            .into_iter()
            .filter(|row| {
                row.generation > GENERATION
                    || !matches!(
                        connection_of(row),
                        Ok(Connection::ReconnectRequired { .. } | Connection::Disconnected)
                    )
            })
            .map(|row| row.practice)
            .collect())
    }

    /// Stands in for message matching's link, which owns this column.
    #[cfg(test)]
    pub(super) async fn link_channel_conversation(
        &self,
        practice: &str,
        conversation: &str,
        patient: &str,
    ) -> Result<(), ChannelError> {
        let tx = self.db.begin().await?;
        self.change_conversation(
            &tx,
            practice,
            conversation,
            ConversationChange::default(),
            0,
        )
        .await?;
        channel_conversation::Entity::update_many()
            .col_expr(
                channel_conversation::Column::Link,
                sea_orm::sea_query::Expr::val(patient),
            )
            .filter(channel_conversation::Column::Practice.eq(practice))
            .filter(channel_conversation::Column::Conversation.eq(conversation))
            .exec(&tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}

#[derive(Default)]
struct ConversationChange {
    settle: bool,
    /// Where the next attempt starts, and this attempt's turn.
    attempt: Option<(Attempt, i64)>,
    owe: Option<OwedCause>,
    details: Option<ConversationDetails>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sealed_credential_opens_only_for_its_practice_and_organization() {
        let key = ChannelKey::from_bytes([7; 32]);
        let sealed = key.seal("practice", "entity_org", "secret").unwrap();
        assert!(!sealed.windows(6).any(|window| window == b"secret"));
        assert_eq!(
            key.open("practice", "entity_org", &sealed).unwrap(),
            "secret"
        );
        assert!(key.open("other", "entity_org", &sealed).is_err());
        assert!(key.open("practice", "entity_other", &sealed).is_err());
        assert!(
            ChannelKey::from_bytes([8; 32])
                .open("practice", "entity_org", &sealed)
                .is_err()
        );
    }

    #[test]
    fn a_key_reads_from_64_hex_digits() {
        assert!(ChannelKey::from_hex(&"ab".repeat(32)).is_some());
        assert!(ChannelKey::from_hex("abc").is_none());
        assert!(ChannelKey::from_hex(&"zz".repeat(32)).is_none());
        assert_eq!(
            format!("{:?}", ChannelKey::from_bytes([1; 32])),
            "ChannelKey(..)"
        );
    }
}
