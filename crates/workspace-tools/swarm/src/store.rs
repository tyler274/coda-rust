use std::path::PathBuf;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::message::{Body, Control, Draft, Message, MessageKind, Seq};
use crate::names::MemberName;
use crate::policy::Policy;
use crate::roster::{ChannelError, Roster, RosterError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SwarmId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Local,
}

/// The contents of `swarm.json`, the anchor that says a swarm exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwarmMeta {
    pub version: u32,
    pub backend: Backend,
    pub swarm_id: SwarmId,
    pub created_at: DateTime<Utc>,
}

impl SwarmMeta {
    pub const VERSION: u32 = 1;
}

/// Why a post was refused. The board is unchanged.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Rejection {
    #[error("{0} is not a member of this swarm")]
    NotMember(MemberName),
    #[error("{0} has been killed and can no longer post")]
    NotLive(MemberName),
    #[error("post is empty")]
    Empty,
    #[error(
        "post is {bytes} bytes; the limit is {max}. Write the details to a file and post its path"
    )]
    TooLarge { bytes: usize, max: usize },
    #[error(
        "posting faster than {per_minute} posts a minute; retry in {:.1}s",
        retry_after.as_secs_f64()
    )]
    RateLimited {
        retry_after: Duration,
        per_minute: u32,
    },
    #[error(transparent)]
    Channel(#[from] ChannelError),
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("{action} {}", path.display())]
    Io {
        action: &'static str,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{} line {line} is not a valid record", path.display())]
    Corrupt {
        path: PathBuf,
        line: usize,
        #[source]
        source: serde_json::Error,
    },
    #[error("{} is not valid", path.display())]
    Invalid {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("swarm at {} has version {found}; this build reads {}", path.display(), SwarmMeta::VERSION)]
    Version { path: PathBuf, found: u32 },
    #[error("encode {what}")]
    Encode {
        what: &'static str,
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum AppendError {
    #[error(transparent)]
    Rejected(#[from] Rejection),
    #[error(transparent)]
    Store(#[from] StoreError),
}

/// A roster update either failed its own rule (`E`) or could not be stored.
#[derive(Debug, thiserror::Error)]
pub enum UpdateError<E> {
    #[error(transparent)]
    Rejected(E),
    #[error(transparent)]
    Store(#[from] StoreError),
}

/// Swarm storage. Each method is atomic with respect to every other handle
/// on the same swarm, in this process or another.
pub trait Store {
    fn meta(&self) -> Result<SwarmMeta, StoreError>;

    /// The highest seq on the board, or `Seq::ZERO` when it is empty.
    fn head(&self) -> Result<Seq, StoreError>;

    /// Admits `draft` under the policy and appends it as the next seq. A
    /// draft whose request id is already on the board returns that message.
    fn append(&self, draft: Draft, now: DateTime<Utc>) -> Result<Message, AppendError>;

    /// The oldest `limit` messages after `since`, in seq order.
    fn read_since(&self, since: Seq, limit: usize) -> Result<Vec<Message>, StoreError>;

    /// The members, with channel state replayed from the board.
    fn roster(&self) -> Result<Roster, StoreError>;

    /// Checks a channel change by `author` and appends it to the board as a
    /// control row, the only record of channel state.
    fn record(
        &self,
        author: MemberName,
        control: Control,
        now: DateTime<Utc>,
    ) -> Result<Message, UpdateError<RosterError>>;

    /// Runs `change` on the current roster and saves the members only if it
    /// succeeds. Channel changes made here are not saved; use `record`.
    fn update_roster<T, E>(
        &self,
        change: impl FnOnce(&mut Roster) -> Result<T, E>,
    ) -> Result<T, UpdateError<E>>;
}

/// The rule that admits a post, shared by every backend. `board` is the
/// board so far, in seq order.
pub(crate) fn admit(
    policy: &Policy,
    roster: &Roster,
    board: &[Message],
    draft: Draft,
    now: DateTime<Utc>,
) -> Result<Admitted, Rejection> {
    if let Some(id) = &draft.request_id
        && let Some(existing) = board.iter().find(|m| m.request_id.as_ref() == Some(id))
    {
        return Ok(Admitted::Duplicate(existing.clone()));
    }
    let author = roster
        .members
        .get(&draft.author)
        .ok_or_else(|| Rejection::NotMember(draft.author.clone()))?;
    if !author.state.is_live() {
        return Err(Rejection::NotLive(draft.author));
    }
    if draft.text.trim().is_empty() {
        return Err(Rejection::Empty);
    }
    let bytes = draft.text.len();
    if bytes > policy.max_post_bytes {
        return Err(Rejection::TooLarge {
            bytes,
            max: policy.max_post_bytes,
        });
    }
    let channel = match draft.channel {
        Some(channel) => {
            roster.active_channel(&channel)?;
            channel
        }
        None => roster.home_channel(author),
    };
    let body = Body::Post(draft.kind);
    if draft.kind == MessageKind::Post {
        let earlier = board.iter().filter(|m| m.body == body).map(|m| m.time);
        policy
            .rate
            .check(earlier, now)
            .map_err(|retry_after| Rejection::RateLimited {
                retry_after,
                per_minute: policy.rate.per_minute(),
            })?;
    }
    let head = board.last().map_or(Seq::ZERO, |m| m.seq);
    Ok(Admitted::New(Message {
        seq: head.next(),
        agent: draft.author,
        text: draft.text,
        time: now,
        body,
        request_id: draft.request_id,
        channel,
    }))
}

/// The rule that admits a channel change, shared by every backend. `roster`
/// has already replayed `board`.
pub(crate) fn admit_control(
    roster: &Roster,
    board: &[Message],
    author: MemberName,
    control: Control,
    now: DateTime<Utc>,
) -> Result<Message, RosterError> {
    roster.check(&author, &control)?;
    let channel = match &control {
        Control::Create { channel, .. }
        | Control::Archive { channel }
        | Control::Subscribe { channel, .. }
        | Control::Unsubscribe { channel, .. } => channel.clone(),
    };
    Ok(Message {
        seq: board.last().map_or(Seq::ZERO, |m| m.seq).next(),
        agent: author,
        text: control.describe(),
        time: now,
        body: Body::Control(control),
        request_id: None,
        channel,
    })
}

pub(crate) enum Admitted {
    New(Message),
    Duplicate(Message),
}
