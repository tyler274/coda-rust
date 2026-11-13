use std::num::NonZeroUsize;
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use super::spawner::{SpawnError, SpawnSpec, Spawner, Workspace};
use super::tools::{
    DEFAULT_LIMIT, DEFAULT_VIEW_LIMIT, DEFAULT_WAIT_SECS, MAX_LIMIT, MAX_WAIT_SECS, Tool,
};
use crate::adapter::Catalog;
use crate::mention::Mention;
use crate::message::{Control, Draft, Message, MessageKind, Seq};
use crate::names::{ChannelName, MemberName, NameError};
use crate::policy::rings;
use crate::roster::{Member, MemberState, Role, Roster, RosterError, Topic, TopicError, WakeMode};
use crate::store::{AppendError, Store, StoreError, UpdateError};

const POLL: Duration = Duration::from_millis(200);

/// Why a tool call failed. Each becomes an MCP tool error whose text is the
/// error and its causes.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("invalid arguments for {tool}")]
    Args {
        tool: &'static str,
        #[source]
        source: serde_json::Error,
    },
    #[error(
        "{caller} may not {action} {target}; only the coordinator or the member that added it may"
    )]
    NotAllowed {
        caller: MemberName,
        action: &'static str,
        target: MemberName,
    },
    #[error("only the coordinator may {0}")]
    CoordinatorOnly(&'static str),
    #[error("{0} is the coordinator and cannot be killed")]
    KillCoordinator(MemberName),
    #[error("no live member is named {0} or belongs to group {0}")]
    NoTarget(String),
    #[error("{0} has been killed")]
    Killed(MemberName),
    #[error("n is {n} but {tasks} tasks were given; pass one or make them agree")]
    GroupSize { n: usize, tasks: usize },
    #[error("no model given and {0} has none to default to")]
    NoModel(MemberName),
    #[error("model {0} is not available; `swarm` action `models` lists them")]
    UnknownModel(String),
    #[error("channel {action} needs `{param}`")]
    Missing {
        action: &'static str,
        param: &'static str,
    },
    #[error("join needs a shared remote board, which this build does not support yet")]
    JoinUnsupported,
    #[error(
        "{started} of {total} members started before a spawn failed; the rest are marked killed"
    )]
    PartialAdd {
        started: usize,
        total: usize,
        #[source]
        source: SpawnError,
    },
    #[error(transparent)]
    Name(#[from] NameError),
    #[error(transparent)]
    Topic(#[from] TopicError),
    #[error(transparent)]
    Roster(#[from] RosterError),
    #[error(transparent)]
    Append(#[from] AppendError),
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Spawn(#[from] SpawnError),
}

impl<E: Into<ToolError>> From<UpdateError<E>> for ToolError {
    fn from(error: UpdateError<E>) -> Self {
        match error {
            UpdateError::Rejected(error) => error.into(),
            UpdateError::Store(error) => error.into(),
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
enum SwarmAction {
    Status {},
    Models {},
    Add(AddArgs),
    Model {
        name: Option<MemberName>,
        model: Option<String>,
    },
    Kill {
        name: String,
    },
    Title {
        title: String,
    },
    Channel(ChannelOp),
    Join {},
}

#[derive(Deserialize)]
struct AddArgs {
    name: String,
    brief: String,
    model: Option<String>,
    #[serde(default)]
    workspace: Workspace,
    #[serde(default)]
    channels: Vec<ChannelName>,
    n: Option<NonZeroUsize>,
    #[serde(default)]
    tasks: Vec<String>,
    fork: Option<MemberName>,
    #[serde(default)]
    role: AddRole,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum AddRole {
    #[default]
    Worker,
    Narrator,
    Monitor,
}

impl From<AddRole> for Role {
    fn from(role: AddRole) -> Self {
        match role {
            AddRole::Worker => Self::Worker,
            AddRole::Narrator => Self::Narrator,
            AddRole::Monitor => Self::Monitor,
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
enum ChannelOp {
    Create {
        channel: Option<ChannelName>,
        topic: Option<Topic>,
    },
    Archive {
        channel: Option<ChannelName>,
    },
    Subscribe {
        channel: Option<ChannelName>,
        name: Option<MemberName>,
    },
    Unsubscribe {
        channel: Option<ChannelName>,
        name: Option<MemberName>,
    },
    List {},
}

#[derive(Deserialize)]
struct PostArgs {
    text: String,
    channel: Option<ChannelName>,
}

#[derive(Deserialize)]
struct ReadArgs {
    since: Option<Seq>,
    limit: Option<usize>,
}

#[derive(Deserialize)]
struct WaitArgs {
    since: Seq,
    #[serde(default = "yes")]
    mentions_only: bool,
    timeout: Option<u64>,
}

#[derive(Deserialize)]
struct ViewArgs {
    limit: Option<usize>,
}

fn yes() -> bool {
    true
}

fn parse<T: DeserializeOwned>(tool: Tool, args: Value) -> Result<T, ToolError> {
    serde_json::from_value(args).map_err(|source| ToolError::Args {
        tool: tool.name(),
        source,
    })
}

fn required<T>(
    value: Option<T>,
    action: &'static str,
    param: &'static str,
) -> Result<T, ToolError> {
    value.ok_or(ToolError::Missing { action, param })
}

/// The caller as a live roster member.
fn live<'r>(roster: &'r Roster, caller: &MemberName) -> Result<&'r Member, ToolError> {
    let member = roster.member(caller)?;
    if !member.state.is_live() {
        return Err(ToolError::Killed(caller.clone()));
    }
    Ok(member)
}

/// The coordinator manages everyone; a worker manages only the helpers it
/// added.
fn authorize(
    roster: &Roster,
    caller: &MemberName,
    action: &'static str,
    target: &MemberName,
) -> Result<(), ToolError> {
    let manager = live(roster, caller)?;
    let member = roster.member(target)?;
    if manager.role == Role::Coordinator || member.added_by.as_ref() == Some(caller) {
        return Ok(());
    }
    Err(ToolError::NotAllowed {
        caller: caller.clone(),
        action,
        target: target.clone(),
    })
}

fn last_post<'b>(board: &'b [Message], name: &MemberName) -> Option<&'b Message> {
    board.iter().rev().find(|m| m.agent == *name)
}

fn member_row(name: &MemberName, member: &Member, last: Option<&Message>) -> Value {
    let mut row = serde_json::to_value(member).unwrap_or_default();
    row["name"] = json!(name);
    row["last_post_seq"] = json!(last.map(|m| m.seq));
    row["last_post_time"] = json!(last.map(|m| m.time));
    row
}

/// One member's board connection: every tool call acts as `caller`.
pub struct Server<S, P> {
    store: S,
    spawner: P,
    caller: MemberName,
}

impl<S: Store, P: Spawner> Server<S, P> {
    pub fn new(store: S, spawner: P, caller: MemberName) -> Self {
        Self {
            store,
            spawner,
            caller,
        }
    }

    pub fn call(&self, tool: Tool, args: Value) -> Result<Reply, ToolError> {
        match tool {
            Tool::Swarm => self.swarm(parse(tool, args)?).map(Reply::from),
            Tool::PostMessage => self.post(parse(tool, args)?).map(Reply::from),
            Tool::ReadBoard => self.read(parse(tool, args)?).map(Reply::from),
            Tool::SwarmWait => self.wait(parse(tool, args)?).map(Reply::from),
            Tool::BoardView => self.view(parse(tool, args)?),
        }
    }

    fn swarm(&self, action: SwarmAction) -> Result<Value, ToolError> {
        match action {
            SwarmAction::Status {} => self.status(),
            SwarmAction::Models {} => Ok(match self.spawner.probe_models() {
                Catalog::Any => json!({ "any": true, "models": [] }),
                Catalog::Listed(models) => json!({ "any": false, "models": models }),
                Catalog::Unregistered => json!({
                    "any": true,
                    "models": [],
                    "note": "no model registry is configured, so added members are roster entries \
                             only; start the coordinator's `coda-swarm mcp` with --models <path> to run them",
                }),
            }),
            SwarmAction::Add(args) => self.add(args),
            SwarmAction::Model { name, model } => self.set_model(name, model),
            SwarmAction::Kill { name } => self.kill(&name),
            SwarmAction::Title { title } => {
                self.store.update_roster(|roster| {
                    if live(roster, &self.caller)?.role != Role::Coordinator {
                        return Err(ToolError::CoordinatorOnly("set the title"));
                    }
                    roster.title.clone_from(&title);
                    Ok(())
                })?;
                Ok(json!({ "title": title }))
            }
            SwarmAction::Channel(op) => self.channel(op),
            SwarmAction::Join {} => Err(ToolError::JoinUnsupported),
        }
    }

    fn status(&self) -> Result<Value, ToolError> {
        let roster = self.store.roster()?;
        let board = self.store.read_since(Seq::ZERO, usize::MAX)?;
        let members: Vec<Value> = roster
            .members
            .iter()
            .map(|(name, member)| member_row(name, member, last_post(&board, name)))
            .collect();
        Ok(json!({
            "swarm_id": self.store.meta()?.swarm_id,
            "title": roster.title,
            "head": board.last().map_or(Seq::ZERO, |m| m.seq),
            "you": self.caller,
            "members": members,
        }))
    }

    fn add(&self, args: AddArgs) -> Result<Value, ToolError> {
        let count = match (args.n, args.tasks.len()) {
            (None, 0) => NonZeroUsize::MIN,
            (Some(n), 0) => n,
            (n, tasks) => match NonZeroUsize::new(tasks) {
                Some(k) if n.is_none_or(|n| n == k) => k,
                _ => {
                    return Err(ToolError::GroupSize {
                        n: n.map_or(0, NonZeroUsize::get),
                        tasks,
                    });
                }
            },
        };
        let names = MemberName::group(&args.name, count)?;
        let head = self.store.head()?;
        let current = self.store.roster()?;
        for channel in &args.channels {
            current.active_channel(channel).map_err(RosterError::from)?;
        }
        let catalog = self.spawner.models();
        let role = Role::from(args.role);
        let specs = self.store.update_roster(|roster| {
            if role != Role::Worker && live(roster, &self.caller)?.role != Role::Coordinator {
                return Err(ToolError::CoordinatorOnly("add a narrator or monitor"));
            }
            live(roster, &self.caller)?;
            let model = args
                .model
                .clone()
                .or_else(|| live(roster, &self.caller).ok()?.model.clone());
            if let Some(model) = &model
                && !catalog.admits(model)
            {
                return Err(ToolError::UnknownModel(model.clone()));
            }
            let mut specs = Vec::new();
            for (index, name) in names.iter().enumerate() {
                let mut member = Member::new(role, Some(self.caller.clone()));
                member.state = MemberState::Waiting {
                    wake_mode: WakeMode::Mentions,
                    wait_since: head,
                };
                member.model.clone_from(&model);
                roster.add_member(name.clone(), member)?;
                let brief = match args.tasks.get(index) {
                    Some(task) => format!("{}\n\nYour task: {task}", args.brief),
                    None => args.brief.clone(),
                };
                specs.push(SpawnSpec {
                    name: name.clone(),
                    group: args.name.clone(),
                    brief,
                    model: model.clone(),
                    workspace: args.workspace,
                    channels: args.channels.clone(),
                    fork: args.fork.clone(),
                });
            }
            Ok(specs)
        })?;
        for channel in &args.channels {
            let control = Control::Subscribe {
                channel: channel.clone(),
                members: names.clone(),
            };
            self.store
                .record(self.caller.clone(), control, Utc::now())?;
        }
        let roster = self.store.roster()?;

        let mut added = Vec::new();
        for (index, spec) in specs.iter().enumerate() {
            let home = roster.home_channel(roster.member(&spec.name)?);
            let posted = self.store.append(
                Draft {
                    author: self.caller.clone(),
                    text: format!("@{} {}", spec.name, spec.brief),
                    channel: Some(home.clone()),
                    kind: MessageKind::Brief,
                    request_id: None,
                },
                Utc::now(),
            );
            let started = posted
                .map_err(ToolError::from)
                .and_then(|brief| Ok((brief, self.spawner.spawn(spec)?)));
            match started {
                Ok((brief, ())) => added.push(json!({
                    "name": spec.name,
                    "model": spec.model,
                    "channel": home,
                    "brief_seq": brief.seq,
                })),
                Err(error) => {
                    let unstarted: Vec<MemberName> =
                        specs[index..].iter().map(|s| s.name.clone()).collect();
                    self.store.update_roster(|roster| {
                        for name in &unstarted {
                            roster.member_mut(name)?.state = MemberState::Killed;
                        }
                        Ok::<_, RosterError>(())
                    })?;
                    return Err(match error {
                        ToolError::Spawn(source) => ToolError::PartialAdd {
                            started: index,
                            total: specs.len(),
                            source,
                        },
                        other => other,
                    });
                }
            }
        }
        Ok(json!({ "added": added }))
    }

    fn set_model(
        &self,
        name: Option<MemberName>,
        model: Option<String>,
    ) -> Result<Value, ToolError> {
        let target = name.unwrap_or_else(|| self.caller.clone());
        let roster = self.store.roster()?;
        let caller = live(&roster, &self.caller)?;
        if target != self.caller {
            authorize(&roster, &self.caller, "change the model of", &target)?;
        }
        if !roster.member(&target)?.state.is_live() {
            return Err(ToolError::Killed(target));
        }
        let model = model
            .or_else(|| caller.model.clone())
            .ok_or_else(|| ToolError::NoModel(self.caller.clone()))?;
        if !self.spawner.models().admits(&model) {
            return Err(ToolError::UnknownModel(model));
        }
        self.spawner.set_model(&target, &model)?;
        self.store.update_roster(|roster| {
            roster.member_mut(&target)?.model = Some(model.clone());
            Ok::<_, RosterError>(())
        })?;
        Ok(json!({ "name": target, "model": model }))
    }

    fn kill(&self, name: &str) -> Result<Value, ToolError> {
        let roster = self.store.roster()?;
        let group = Mention::Name(name.to_owned());
        let targets: Vec<MemberName> = roster
            .members
            .iter()
            .filter(|(member, entry)| group.names(member) && entry.state.is_live())
            .map(|(member, _)| member.clone())
            .collect();
        if targets.is_empty() {
            return Err(ToolError::NoTarget(name.to_owned()));
        }
        for target in &targets {
            if roster.member(target)?.role == Role::Coordinator {
                return Err(ToolError::KillCoordinator(target.clone()));
            }
            authorize(&roster, &self.caller, "kill", target)?;
        }
        let mut killed = Vec::new();
        let outcome = targets.iter().try_for_each(|target| {
            self.spawner.kill(target)?;
            killed.push(target.clone());
            Ok::<_, SpawnError>(())
        });
        self.store.update_roster(|roster| {
            for target in &killed {
                roster.member_mut(target)?.state = MemberState::Killed;
            }
            Ok::<_, RosterError>(())
        })?;
        outcome?;
        Ok(json!({ "killed": killed }))
    }

    fn channel(&self, op: ChannelOp) -> Result<Value, ToolError> {
        match op {
            ChannelOp::List {} => {
                let roster = self.store.roster()?;
                let channels: Vec<Value> = roster
                    .channels()
                    .iter()
                    .map(|(name, channel)| {
                        let mut row = json!(channel);
                        row["name"] = json!(name);
                        row
                    })
                    .collect();
                Ok(json!({ "channels": channels }))
            }
            ChannelOp::Create { channel, topic } => {
                let channel = required(channel, "create", "channel")?;
                let topic = topic.map_or_else(|| Topic::parse(""), Ok)?;
                let row = self.record(Control::Create {
                    channel: channel.clone(),
                    topic,
                })?;
                Ok(json!({ "created": channel, "seq": row.seq }))
            }
            ChannelOp::Archive { channel } => {
                let channel = required(channel, "archive", "channel")?;
                let row = self.record(Control::Archive {
                    channel: channel.clone(),
                })?;
                Ok(json!({ "archived": channel, "seq": row.seq }))
            }
            ChannelOp::Subscribe { channel, name } => {
                let channel = required(channel, "subscribe", "channel")?;
                let member = self.subscriber(name)?;
                let row = self.record(Control::Subscribe {
                    channel: channel.clone(),
                    members: vec![member.clone()],
                })?;
                Ok(json!({ "subscribed": member, "channel": channel, "seq": row.seq }))
            }
            ChannelOp::Unsubscribe { channel, name } => {
                let channel = required(channel, "unsubscribe", "channel")?;
                let member = self.subscriber(name)?;
                let row = self.record(Control::Unsubscribe {
                    channel: channel.clone(),
                    members: vec![member.clone()],
                })?;
                Ok(json!({ "unsubscribed": member, "channel": channel, "seq": row.seq }))
            }
        }
    }

    fn record(&self, control: Control) -> Result<Message, ToolError> {
        Ok(self
            .store
            .record(self.caller.clone(), control, Utc::now())?)
    }

    /// The member whose subscriptions change: the caller, or one it may
    /// manage.
    fn subscriber(&self, name: Option<MemberName>) -> Result<MemberName, ToolError> {
        let member = name.unwrap_or_else(|| self.caller.clone());
        if member != self.caller {
            authorize(
                &self.store.roster()?,
                &self.caller,
                "change the channels of",
                &member,
            )?;
        }
        Ok(member)
    }

    fn post(&self, args: PostArgs) -> Result<Value, ToolError> {
        let mut draft = Draft::post(self.caller.clone(), args.text);
        draft.channel = args.channel;
        if self.store.roster()?.member(&self.caller)?.role == Role::Narrator {
            draft.kind = MessageKind::Summary;
        }
        let message = self.store.append(draft, Utc::now())?;
        Ok(json!({ "seq": message.seq, "channel": message.channel }))
    }

    /// Messages after `since` that the caller can see, oldest first.
    fn visible_since(&self, roster: &Roster, since: Seq) -> Result<Vec<Message>, ToolError> {
        let member = roster.member(&self.caller)?;
        Ok(self
            .store
            .read_since(since, usize::MAX)?
            .into_iter()
            .filter(|m| roster.sees(&self.caller, member, m, &m.mentions()))
            .collect())
    }

    fn mark_read(&self, through: Option<Seq>) -> Result<Seq, ToolError> {
        Ok(self.store.update_roster(|roster| {
            let member = roster.member_mut(&self.caller)?;
            if let Some(seq) = through {
                member.mark_read(seq);
            }
            Ok::<_, RosterError>(member.last_read_seq)
        })?)
    }

    fn read(&self, args: ReadArgs) -> Result<Value, ToolError> {
        let limit = args.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
        let roster = self.store.roster()?;
        let mut visible = self.visible_since(&roster, args.since.unwrap_or(Seq::ZERO))?;
        if args.since.is_some() {
            visible.truncate(limit);
        } else {
            visible.drain(..visible.len().saturating_sub(limit));
        }
        let last_read_seq = self.mark_read(visible.last().map(|m| m.seq))?;
        Ok(json!({
            "messages": visible,
            "head": self.store.head()?,
            "last_read_seq": last_read_seq,
        }))
    }

    fn wait(&self, args: WaitArgs) -> Result<Value, ToolError> {
        let wake_mode = if args.mentions_only {
            WakeMode::Mentions
        } else {
            WakeMode::Anything
        };
        let timeout = Duration::from_secs(
            args.timeout
                .unwrap_or(DEFAULT_WAIT_SECS)
                .clamp(1, MAX_WAIT_SECS),
        );
        let (wait_since, supervised) = self.store.update_roster(|roster| {
            let member = roster.member_mut(&self.caller)?;
            if !member.state.is_live() {
                return Err(ToolError::Killed(self.caller.clone()));
            }
            let wait_since = args.since.max(member.last_read_seq);
            member.state = MemberState::Waiting {
                wake_mode,
                wait_since,
            };
            Ok((wait_since, member.role != Role::Coordinator))
        })?;
        let deadline = Instant::now() + timeout;
        loop {
            let roster = self.store.roster()?;
            let member = live(&roster, &self.caller)?;
            let board = self.store.read_since(wait_since, usize::MAX)?;
            let woken = board
                .iter()
                .any(|m| rings(&roster, &self.caller, member, m));
            if supervised && !woken {
                return Ok(json!({
                    "parked": true,
                    "wait_since": wait_since,
                    "instruction": "You are parked. End your turn now without calling another \
                                    tool; a post that wakes you starts your next turn with the \
                                    new messages.",
                }));
            }
            if woken || Instant::now() >= deadline {
                self.store.update_roster(|roster| {
                    let member = roster.member_mut(&self.caller)?;
                    if member.state.is_live() {
                        member.state = MemberState::Working;
                    }
                    Ok::<_, RosterError>(())
                })?;
                if !woken {
                    return Ok(json!({ "woken": false, "head": self.store.head()? }));
                }
                let visible = self.visible_since(&roster, args.since)?;
                let last_read_seq = self.mark_read(visible.last().map(|m| m.seq))?;
                return Ok(json!({
                    "woken": true,
                    "messages": visible,
                    "last_read_seq": last_read_seq,
                }));
            }
            thread::sleep(POLL);
        }
    }

    fn view(&self, args: ViewArgs) -> Result<Reply, ToolError> {
        let limit = args.limit.unwrap_or(DEFAULT_VIEW_LIMIT).clamp(1, MAX_LIMIT);
        let roster = self.store.roster()?;
        let board = self.store.read_since(Seq::ZERO, usize::MAX)?;
        let swarm_id = self.store.meta()?.swarm_id;
        let head = board.last().map_or(Seq::ZERO, |m| m.seq);
        let members: Vec<Value> = roster
            .members
            .iter()
            .map(|(name, member)| member_row(name, member, last_post(&board, name)))
            .collect();
        let channels: Vec<Value> = std::iter::once(json!({ "name": ChannelName::main() }))
            .chain(roster.channels().iter().map(|(name, channel)| {
                let mut row = json!(channel);
                row["name"] = json!(name);
                row
            }))
            .collect();
        let recent = &board[board.len().saturating_sub(limit)..];
        let summary = view_summary(&roster, &swarm_id.0, head, recent);
        Ok(Reply {
            structured: json!({
                "swarm_id": swarm_id,
                "title": roster.title,
                "head": head,
                "members": members,
                "channels": channels,
                "messages": recent,
            }),
            text: Some(summary),
        })
    }
}

/// A tool's result: the structured value, and the text the agent reads when
/// it should differ from that value's JSON.
pub struct Reply {
    pub structured: Value,
    pub text: Option<String>,
}

impl From<Value> for Reply {
    fn from(structured: Value) -> Self {
        Self {
            structured,
            text: None,
        }
    }
}

fn view_summary(roster: &Roster, swarm_id: &str, head: Seq, recent: &[Message]) -> String {
    use std::fmt::Write as _;
    let mut text = String::new();
    let title = if roster.title.is_empty() {
        "Swarm"
    } else {
        &roster.title
    };
    let _ = writeln!(text, "{title} ({swarm_id}), head {head}");
    let _ = writeln!(text, "Members:");
    for (name, member) in &roster.members {
        let row = json!(member);
        let model = member.model.as_deref().unwrap_or("no model");
        let _ = writeln!(
            text,
            "- {name} ({}): {}, {model}",
            row["role"].as_str().unwrap_or_default(),
            row["state"].as_str().unwrap_or_default(),
        );
    }
    let channels: Vec<String> = std::iter::once(format!("#{}", ChannelName::main()))
        .chain(roster.channels().iter().map(|(name, channel)| {
            let archived = if channel.archived { ", archived" } else { "" };
            match channel.topic.as_str() {
                "" => format!("#{name}{archived}"),
                topic => format!("#{name} ({topic}{archived})"),
            }
        }))
        .collect();
    let _ = writeln!(text, "Channels: {}", channels.join(", "));
    let _ = writeln!(text, "Recent messages ({}):", recent.len());
    for message in recent {
        let _ = writeln!(
            text,
            "[{}] #{} {}: {}",
            message.seq, message.channel, message.agent, message.text
        );
    }
    text
}
