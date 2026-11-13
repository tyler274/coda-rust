use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::mention::Mention;
use crate::message::{Body, Control, Message, Seq};
use crate::names::{ChannelName, MemberName};
use crate::policy::{MAX_TOPIC_LEN, RESERVED_CHANNEL_NAMES};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Coordinator,
    Worker,
    Narrator,
    Monitor,
}

impl Role {
    /// Roles that hear every `@#channel` mention, subscribed or not.
    fn hears_all_channels(self) -> bool {
        !matches!(self, Self::Worker)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WakeMode {
    /// Wake only for posts that name the member.
    Mentions,
    /// Wake for any post the member can see.
    Anything,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase")]
pub enum MemberState {
    /// Parked after `wait_since` until a post rings it.
    Waiting {
        wake_mode: WakeMode,
        wait_since: Seq,
    },
    Working,
    Stopped,
    Killed,
}

impl MemberState {
    pub fn is_live(self) -> bool {
        !matches!(self, Self::Killed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    pub role: Role,
    #[serde(flatten)]
    pub state: MemberState,
    pub model: Option<String>,
    /// Subscribed channels besides `main`; the first active one is home.
    /// Replayed from the board's control rows, never stored here.
    #[serde(skip)]
    channels: Vec<ChannelName>,
    pub added_by: Option<MemberName>,
    #[serde(default)]
    pub last_read_seq: Seq,
    /// The head at which a stop-time intervention last fired for this
    /// member; see [`Member::take_park_block`].
    #[serde(default)]
    pub last_park_blocked_seq: Seq,
    /// The head the last turn-start `[board]` nudge covered.
    #[serde(default)]
    pub last_nudged_seq: Seq,
}

impl Member {
    pub fn new(role: Role, added_by: Option<MemberName>) -> Self {
        Self {
            role,
            state: MemberState::Working,
            model: None,
            channels: Vec::new(),
            added_by,
            last_read_seq: Seq::ZERO,
            last_park_blocked_seq: Seq::ZERO,
            last_nudged_seq: Seq::ZERO,
        }
    }

    pub fn channels(&self) -> &[ChannelName] {
        &self.channels
    }

    /// Claims the one stop-time intervention (blocking a stop, or ringing
    /// itself on stop) allowed at `head`. Returns false when one already
    /// fired at this head, so the member may stop.
    pub fn take_park_block(&mut self, head: Seq) -> bool {
        if head <= self.last_park_blocked_seq {
            return false;
        }
        self.last_park_blocked_seq = head;
        true
    }

    /// Moves the read cursor forward; it never moves back.
    pub fn mark_read(&mut self, seq: Seq) {
        self.last_read_seq = self.last_read_seq.max(seq);
    }
}

/// A channel topic: short text with no `@` and no newline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Topic(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TopicError {
    #[error("topic has {len} characters; the limit is {max}")]
    TooLong { len: usize, max: usize },
    #[error("topic must not contain '@' or a newline")]
    BadChar,
}

impl Topic {
    pub fn parse(text: &str) -> Result<Self, TopicError> {
        let len = text.chars().count();
        if len > MAX_TOPIC_LEN {
            return Err(TopicError::TooLong {
                len,
                max: MAX_TOPIC_LEN,
            });
        }
        if text.contains(['@', '\n', '\r']) {
            return Err(TopicError::BadChar);
        }
        Ok(Self(text.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Topic {
    type Error = TopicError;
    fn try_from(text: String) -> Result<Self, TopicError> {
        Self::parse(&text)
    }
}

impl From<Topic> for String {
    fn from(topic: Topic) -> Self {
        topic.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    pub topic: Topic,
    pub created_by: MemberName,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ChannelError {
    #[error("channel {0} does not exist")]
    Unknown(ChannelName),
    #[error("channel {0} is archived")]
    Archived(ChannelName),
    #[error("channel {0} already exists")]
    Exists(ChannelName),
    #[error("channel name {0} is reserved")]
    Reserved(ChannelName),
    #[error("only the channel's creator or the coordinator may archive {0}")]
    NotAllowed(ChannelName),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RosterError {
    #[error("member {0} already exists")]
    Exists(MemberName),
    #[error("member {0} does not exist")]
    Unknown(MemberName),
    #[error("{0} has been killed")]
    Killed(MemberName),
    #[error(transparent)]
    Channel(#[from] ChannelError),
}

/// The swarm title and its members, stored in `members.json`, plus channel
/// state replayed from the board's control rows. `main` always exists and
/// is never recorded.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roster {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub members: BTreeMap<MemberName, Member>,
    #[serde(skip)]
    channels: BTreeMap<ChannelName, Channel>,
}

impl Roster {
    pub fn member(&self, name: &MemberName) -> Result<&Member, RosterError> {
        self.members
            .get(name)
            .ok_or_else(|| RosterError::Unknown(name.clone()))
    }

    pub fn member_mut(&mut self, name: &MemberName) -> Result<&mut Member, RosterError> {
        self.members
            .get_mut(name)
            .ok_or_else(|| RosterError::Unknown(name.clone()))
    }

    pub fn add_member(&mut self, name: MemberName, member: Member) -> Result<(), RosterError> {
        if self.members.contains_key(&name) {
            return Err(RosterError::Exists(name));
        }
        self.members.insert(name, member);
        Ok(())
    }

    pub fn remove_member(&mut self, name: &MemberName) -> Result<Member, RosterError> {
        self.members
            .remove(name)
            .ok_or_else(|| RosterError::Unknown(name.clone()))
    }

    /// Channels besides `main`, archived ones included.
    pub fn channels(&self) -> &BTreeMap<ChannelName, Channel> {
        &self.channels
    }

    /// Accepts `main` or an existing, unarchived channel.
    pub fn active_channel(&self, name: &ChannelName) -> Result<(), ChannelError> {
        if name.is_main() {
            return Ok(());
        }
        match self.channels.get(name) {
            None => Err(ChannelError::Unknown(name.clone())),
            Some(channel) if channel.archived => Err(ChannelError::Archived(name.clone())),
            Some(_) => Ok(()),
        }
    }

    fn is_active(&self, name: &ChannelName) -> bool {
        self.active_channel(name).is_ok()
    }

    /// Where a member's post goes when it names no channel.
    pub fn home_channel(&self, member: &Member) -> ChannelName {
        member
            .channels
            .iter()
            .find(|channel| self.is_active(channel))
            .cloned()
            .unwrap_or_else(ChannelName::main)
    }

    /// Checks `control` by `author` against the current state. The store
    /// records the row only when this passes.
    pub(crate) fn check(&self, author: &MemberName, control: &Control) -> Result<(), RosterError> {
        if !self.member(author)?.state.is_live() {
            return Err(RosterError::Killed(author.clone()));
        }
        let known =
            |members: &[MemberName]| members.iter().try_for_each(|m| self.member(m).map(drop));
        match control {
            Control::Create { channel, .. } => {
                if RESERVED_CHANNEL_NAMES.contains(&channel.as_str()) {
                    return Err(ChannelError::Reserved(channel.clone()).into());
                }
                if self.channels.contains_key(channel) {
                    return Err(ChannelError::Exists(channel.clone()).into());
                }
            }
            Control::Archive { channel } => {
                self.active_channel(channel)?;
                let created_by = &self
                    .channels
                    .get(channel)
                    .ok_or_else(|| ChannelError::Reserved(channel.clone()))?
                    .created_by;
                if created_by != author && self.member(author)?.role != Role::Coordinator {
                    return Err(ChannelError::NotAllowed(channel.clone()).into());
                }
            }
            Control::Subscribe { channel, members } => {
                self.active_channel(channel)?;
                known(members)?;
            }
            Control::Unsubscribe { members, .. } => known(members)?,
        }
        Ok(())
    }

    /// Applies one control row. Rows were checked when recorded, so this
    /// never fails; a member no longer on the roster is skipped.
    pub(crate) fn apply(&mut self, author: &MemberName, control: &Control) {
        match control {
            Control::Create { channel, topic } => {
                self.channels
                    .entry(channel.clone())
                    .or_insert_with(|| Channel {
                        topic: topic.clone(),
                        created_by: author.clone(),
                        archived: false,
                    });
            }
            Control::Archive { channel } => {
                if let Some(entry) = self.channels.get_mut(channel) {
                    entry.archived = true;
                }
            }
            Control::Subscribe { channel, members } if !channel.is_main() => {
                for member in members {
                    let Some(entry) = self.members.get_mut(member) else {
                        continue;
                    };
                    if !entry.channels.contains(channel) {
                        entry.channels.push(channel.clone());
                    }
                }
            }
            Control::Subscribe { .. } => {}
            Control::Unsubscribe { channel, members } => {
                for member in members {
                    let Some(entry) = self.members.get_mut(member) else {
                        continue;
                    };
                    entry.channels.retain(|c| c != channel);
                }
            }
        }
    }

    /// Rebuilds channel state from the board's control rows, in seq order.
    pub(crate) fn replay(&mut self, board: &[Message]) {
        for message in board {
            if let Body::Control(control) = &message.body {
                self.apply(&message.agent, control);
            }
        }
    }

    /// Whether `mentions` call on this member: by name, group, `@all`, or an
    /// `@#channel` it hears. Unknown or archived channels call on no one.
    pub fn is_mentioned(&self, name: &MemberName, member: &Member, mentions: &[Mention]) -> bool {
        mentions.iter().any(|mention| match mention {
            Mention::Channel(token) => ChannelName::parse(token).is_ok_and(|channel| {
                self.is_active(&channel)
                    && (channel.is_main()
                        || member.role.hears_all_channels()
                        || member.channels.contains(&channel))
            }),
            other => other.names(name),
        })
    }

    /// Whether the message is in this member's view: `main`, a subscribed
    /// channel, or a post that mentions it.
    pub fn sees(
        &self,
        name: &MemberName,
        member: &Member,
        message: &Message,
        mentions: &[Mention],
    ) -> bool {
        message.channel.is_main()
            || member.channels.contains(&message.channel)
            || self.is_mentioned(name, member, mentions)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;
    use crate::message::MessageKind;
    use crate::policy::rings;

    fn name(text: &str) -> MemberName {
        MemberName::parse(text).expect("valid")
    }

    fn channel(text: &str) -> ChannelName {
        ChannelName::parse(text).expect("valid")
    }

    fn control(roster: &mut Roster, author: &str, control: Control) -> Result<(), RosterError> {
        roster.check(&name(author), &control)?;
        roster.apply(&name(author), &control);
        Ok(())
    }

    fn create(channel_name: &str) -> Control {
        Control::Create {
            channel: channel(channel_name),
            topic: Topic::parse("deploys").expect("valid"),
        }
    }

    fn archive(channel_name: &str) -> Control {
        Control::Archive {
            channel: channel(channel_name),
        }
    }

    fn roster() -> Roster {
        let mut roster = Roster::default();
        roster
            .add_member(name("lead"), Member::new(Role::Coordinator, None))
            .expect("added");
        for worker in ["scout-1", "scout-2"] {
            roster
                .add_member(name(worker), Member::new(Role::Worker, Some(name("lead"))))
                .expect("added");
        }
        control(&mut roster, "scout-1", create("ops")).expect("created");
        let subscribe = Control::Subscribe {
            channel: channel("ops"),
            members: vec![name("scout-1")],
        };
        control(&mut roster, "scout-1", subscribe).expect("subscribed");
        roster
    }

    fn message(author: &str, text: &str, channel: ChannelName, seq: u64) -> Message {
        Message {
            seq: Seq(seq),
            agent: name(author),
            text: text.into(),
            time: DateTime::from_timestamp(1_800_000_000, 0).expect("valid"),
            body: Body::Post(MessageKind::Post),
            request_id: None,
            channel,
        }
    }

    fn park(roster: &mut Roster, who: &str, wake_mode: WakeMode, since: u64) {
        roster.member_mut(&name(who)).expect("member").state = MemberState::Waiting {
            wake_mode,
            wait_since: Seq(since),
        };
    }

    fn rung(roster: &Roster, who: &str, message: &Message) -> bool {
        let member = roster.member(&name(who)).expect("member");
        rings(roster, &name(who), member, message)
    }

    #[test]
    fn main_always_exists_and_reserved_names_cannot_be_created() {
        let mut roster = roster();
        assert_eq!(roster.active_channel(&ChannelName::main()), Ok(()));
        for reserved in ["main", "all", "here"] {
            assert_eq!(
                control(&mut roster, "lead", create(reserved)),
                Err(ChannelError::Reserved(channel(reserved)).into())
            );
        }
    }

    #[test]
    fn topics_reject_mentions_and_newlines() {
        assert_eq!(Topic::parse("ping @lead"), Err(TopicError::BadChar));
        assert_eq!(Topic::parse("a\nb"), Err(TopicError::BadChar));
    }

    #[test]
    fn home_channel_skips_archived_channels() {
        let mut roster = roster();
        let scout = roster.member(&name("scout-1")).expect("member").clone();
        assert_eq!(roster.home_channel(&scout), channel("ops"));
        control(&mut roster, "lead", archive("ops")).expect("coordinator archives");
        assert_eq!(roster.home_channel(&scout), ChannelName::main());
    }

    #[test]
    fn only_creator_or_coordinator_archives() {
        let mut roster = roster();
        assert_eq!(
            control(&mut roster, "scout-2", archive("ops")),
            Err(ChannelError::NotAllowed(channel("ops")).into())
        );
        assert_eq!(control(&mut roster, "scout-1", archive("ops")), Ok(()));
        assert_eq!(
            control(&mut roster, "lead", archive("ops")),
            Err(ChannelError::Archived(channel("ops")).into())
        );
    }

    #[test]
    fn channel_mentions_reach_subscribers_and_the_coordinator() {
        let roster = roster();
        let mentions = [Mention::Channel("ops".into())];
        let hears = |who: &str| {
            let member = roster.member(&name(who)).expect("member");
            roster.is_mentioned(&name(who), member, &mentions)
        };
        assert!(hears("scout-1"));
        assert!(hears("lead"));
        assert!(!hears("scout-2"));
        let unknown = [Mention::Channel("nope".into())];
        let lead = roster.member(&name("lead")).expect("member");
        assert!(!roster.is_mentioned(&name("lead"), lead, &unknown));
    }

    #[test]
    fn mentions_only_members_wake_when_named() {
        let mut roster = roster();
        park(&mut roster, "scout-2", WakeMode::Mentions, 4);
        let plain = message("lead", "status?", ChannelName::main(), 5);
        assert!(!rung(&roster, "scout-2", &plain));
        let group = message("lead", "@scout go", ChannelName::main(), 5);
        assert!(rung(&roster, "scout-2", &group));
        let stale = message("lead", "@scout go", ChannelName::main(), 4);
        assert!(!rung(&roster, "scout-2", &stale));
    }

    #[test]
    fn anything_members_wake_for_visible_posts_but_not_their_own() {
        let mut roster = roster();
        park(&mut roster, "scout-2", WakeMode::Anything, 0);
        assert!(rung(
            &roster,
            "scout-2",
            &message("lead", "hi", ChannelName::main(), 1)
        ));
        assert!(!rung(
            &roster,
            "scout-2",
            &message("scout-1", "deploying", channel("ops"), 2)
        ));
        assert!(!rung(
            &roster,
            "scout-2",
            &message("scout-2", "@all hi", ChannelName::main(), 3)
        ));
    }

    #[test]
    fn briefs_never_ring() {
        let mut roster = roster();
        park(&mut roster, "scout-2", WakeMode::Mentions, 0);
        let mut brief = message("lead", "@scout-2 go", ChannelName::main(), 1);
        brief.body = Body::Post(MessageKind::Brief);
        assert!(!rung(&roster, "scout-2", &brief));
    }

    #[test]
    fn control_rows_never_ring() {
        let mut roster = roster();
        park(&mut roster, "scout-2", WakeMode::Anything, 0);
        let mut row = message("lead", "@scout-2 @all", ChannelName::main(), 1);
        row.body = Body::Control(archive("ops"));
        assert!(!rung(&roster, "scout-2", &row));
        row.body = Body::Post(MessageKind::Post);
        assert!(rung(&roster, "scout-2", &row));
    }

    #[test]
    fn replay_rebuilds_channels_and_subscriptions_in_seq_order() {
        let mut fresh = roster();
        let rows = [
            ("lead", create("ops")),
            (
                "lead",
                Control::Subscribe {
                    channel: channel("ops"),
                    members: vec![name("scout-1"), name("scout-2")],
                },
            ),
            (
                "lead",
                Control::Unsubscribe {
                    channel: channel("ops"),
                    members: vec![name("scout-1")],
                },
            ),
            ("lead", archive("ops")),
        ];
        let board: Vec<Message> = rows
            .into_iter()
            .enumerate()
            .map(|(index, (author, row))| {
                let mut entry = message(author, "", ChannelName::main(), index as u64 + 1);
                entry.body = Body::Control(row);
                entry
            })
            .collect();
        fresh.channels.clear();
        for member in fresh.members.values_mut() {
            member.channels.clear();
        }
        fresh.replay(&board);
        let ops = &fresh.channels()[&channel("ops")];
        assert!(ops.archived);
        assert_eq!(ops.created_by, name("lead"));
        assert!(
            fresh
                .member(&name("scout-1"))
                .expect("member")
                .channels()
                .is_empty()
        );
        let scout = fresh.member(&name("scout-2")).expect("member");
        assert_eq!(scout.channels(), [channel("ops")]);
        assert_eq!(fresh.home_channel(scout), ChannelName::main());
        assert_eq!(
            fresh.active_channel(&channel("ops")),
            Err(ChannelError::Archived(channel("ops")))
        );
    }

    #[test]
    fn killed_members_cannot_change_channels() {
        let mut roster = roster();
        roster.member_mut(&name("scout-2")).expect("member").state = MemberState::Killed;
        assert_eq!(
            control(&mut roster, "scout-2", create("dev")),
            Err(RosterError::Killed(name("scout-2")))
        );
    }

    #[test]
    fn park_block_fires_at_most_once_per_head() {
        let mut member = Member::new(Role::Worker, None);
        assert!(!member.take_park_block(Seq::ZERO));
        assert!(member.take_park_block(Seq(4)));
        assert!(!member.take_park_block(Seq(4)));
        assert!(!member.take_park_block(Seq(3)));
        assert!(member.take_park_block(Seq(5)));
        assert_eq!(member.last_park_blocked_seq, Seq(5));
    }

    #[test]
    fn working_members_are_never_rung() {
        let roster = roster();
        let post = message("lead", "@all hi", ChannelName::main(), 1);
        assert!(!rung(&roster, "scout-1", &post));
    }

    #[test]
    fn read_cursor_only_moves_forward() {
        let mut member = Member::new(Role::Worker, None);
        member.mark_read(Seq(5));
        member.mark_read(Seq(3));
        assert_eq!(member.last_read_seq, Seq(5));
    }

    #[test]
    fn member_state_serializes_flat() {
        let mut member = Member::new(Role::Worker, Some(name("lead")));
        member.state = MemberState::Waiting {
            wake_mode: WakeMode::Mentions,
            wait_since: Seq(7),
        };
        let json = serde_json::to_value(&member).expect("encodes");
        assert_eq!(json["state"], "waiting");
        assert_eq!(json["wake_mode"], "mentions");
        assert_eq!(json["wait_since"], 7);
        assert!(json.get("channels").is_none(), "channels live on the board");
        assert_eq!(
            serde_json::from_value::<Member>(json).expect("decodes"),
            member
        );
    }
}
