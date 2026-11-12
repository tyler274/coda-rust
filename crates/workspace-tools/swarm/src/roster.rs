use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::mention::Mention;
use crate::message::{Message, Seq};
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
    #[serde(default)]
    pub channels: Vec<ChannelName>,
    pub added_by: Option<MemberName>,
    #[serde(default)]
    pub last_read_seq: Seq,
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
        }
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
    #[error(transparent)]
    Channel(#[from] ChannelError),
}

/// The contents of `members.json`: the swarm title, its members and the
/// channels besides `main`, which always exists and is never stored.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roster {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub members: BTreeMap<MemberName, Member>,
    #[serde(default)]
    pub channels: BTreeMap<ChannelName, Channel>,
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

    /// Adds a member. Each of its channels must exist and be active.
    pub fn add_member(&mut self, name: MemberName, member: Member) -> Result<(), RosterError> {
        if self.members.contains_key(&name) {
            return Err(RosterError::Exists(name));
        }
        for channel in &member.channels {
            self.active_channel(channel)?;
        }
        self.members.insert(name, member);
        Ok(())
    }

    pub fn remove_member(&mut self, name: &MemberName) -> Result<Member, RosterError> {
        self.members
            .remove(name)
            .ok_or_else(|| RosterError::Unknown(name.clone()))
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

    pub fn create_channel(
        &mut self,
        name: ChannelName,
        topic: Topic,
        created_by: MemberName,
    ) -> Result<(), RosterError> {
        if RESERVED_CHANNEL_NAMES.contains(&name.as_str()) {
            return Err(ChannelError::Reserved(name).into());
        }
        if self.channels.contains_key(&name) {
            return Err(ChannelError::Exists(name).into());
        }
        self.member(&created_by)?;
        self.channels.insert(
            name,
            Channel {
                topic,
                created_by,
                archived: false,
            },
        );
        Ok(())
    }

    pub fn archive_channel(
        &mut self,
        name: &ChannelName,
        by: &MemberName,
    ) -> Result<(), RosterError> {
        self.active_channel(name)?;
        let is_coordinator = self.member(by)?.role == Role::Coordinator;
        let channel = self
            .channels
            .get_mut(name)
            .ok_or_else(|| ChannelError::Reserved(name.clone()))?;
        if channel.created_by != *by && !is_coordinator {
            return Err(ChannelError::NotAllowed(name.clone()).into());
        }
        channel.archived = true;
        Ok(())
    }

    pub fn subscribe(
        &mut self,
        member: &MemberName,
        channel: &ChannelName,
    ) -> Result<(), RosterError> {
        self.active_channel(channel)?;
        if channel.is_main() {
            return Ok(());
        }
        let entry = self.member_mut(member)?;
        if !entry.channels.contains(channel) {
            entry.channels.push(channel.clone());
        }
        Ok(())
    }

    pub fn unsubscribe(
        &mut self,
        member: &MemberName,
        channel: &ChannelName,
    ) -> Result<(), RosterError> {
        self.member_mut(member)?.channels.retain(|c| c != channel);
        Ok(())
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
        roster
            .create_channel(
                channel("ops"),
                Topic::parse("deploys").expect("valid"),
                name("scout-1"),
            )
            .expect("created");
        roster
            .subscribe(&name("scout-1"), &channel("ops"))
            .expect("subscribed");
        roster
    }

    fn message(author: &str, text: &str, channel: ChannelName, seq: u64) -> Message {
        Message {
            seq: Seq(seq),
            agent: name(author),
            text: text.into(),
            time: DateTime::from_timestamp(1_800_000_000, 0).expect("valid"),
            kind: MessageKind::Post,
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
        let topic = Topic::parse("x").expect("valid");
        for reserved in ["main", "all", "here"] {
            assert_eq!(
                roster.create_channel(channel(reserved), topic.clone(), name("lead")),
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
        roster
            .archive_channel(&channel("ops"), &name("lead"))
            .expect("coordinator archives");
        assert_eq!(roster.home_channel(&scout), ChannelName::main());
    }

    #[test]
    fn only_creator_or_coordinator_archives() {
        let mut roster = roster();
        assert_eq!(
            roster.archive_channel(&channel("ops"), &name("scout-2")),
            Err(ChannelError::NotAllowed(channel("ops")).into())
        );
        assert_eq!(
            roster.archive_channel(&channel("ops"), &name("scout-1")),
            Ok(())
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
        brief.kind = MessageKind::Brief;
        assert!(!rung(&roster, "scout-2", &brief));
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
        assert_eq!(
            serde_json::from_value::<Member>(json).expect("decodes"),
            member
        );
    }
}
