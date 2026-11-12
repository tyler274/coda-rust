use std::fs;
use std::num::NonZeroUsize;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use chrono::Utc;
use tempfile::TempDir;

use super::*;
use crate::adapter::CommandAdapter;
use crate::file_store::FileStore;
use crate::policy::Policy;
use crate::roster::{Member, Role};
use crate::store::SwarmId;

/// Behavior by member name. Every turn appends `<member> <prompt>` to
/// `$1/turns`; a hanging turn leaves a background child whose pid it writes
/// to `$1/<member>.child`.
const STUB: &str = r#"#!/bin/sh
dir="$1"
prompt=$(cat)
echo "$CODA_SWARM_MEMBER $prompt" >> "$dir/turns"
echo "$CODA_SWARM_AGENT $CODA_SWARM_DIR $API_KEY $CODA_SWARM_MCP_COMMAND" > "$dir/$CODA_SWARM_MEMBER.board"
case "$CODA_SWARM_MEMBER" in
  *crash*) exit 3 ;;
  *hang*)
    sleep 30 &
    echo $! > "$dir/$CODA_SWARM_MEMBER.child"
    wait ;;
  *stubborn*)
    trap 'echo term >> "$dir/stubborn.term"; sleep 30' TERM
    sleep 30 &
    echo $! > "$dir/$CODA_SWARM_MEMBER.child"
    wait; wait ;;
  *quota*) echo retry_after_secs=600; exit 75 ;;
  *) exit 0 ;;
esac
"#;

fn name(text: &str) -> MemberName {
    MemberName::parse(text).expect("valid name")
}

/// Records every call; `remove` fails while `fail_removals` is positive.
#[derive(Default)]
struct FakeWorkspaces {
    root: PathBuf,
    calls: Mutex<Vec<String>>,
    fail_removals: Mutex<usize>,
}

impl Workspaces for Arc<FakeWorkspaces> {
    fn create(&self, slug: &str, _mode: WorktreeMode) -> Result<PathBuf, WorkspaceError> {
        lock(&self.calls).push(format!("new {slug}"));
        let path = self.root.join(slug);
        fs::create_dir_all(&path).expect("worktree dir");
        Ok(path)
    }

    fn remove(&self, slug: &str) -> Result<(), WorkspaceError> {
        lock(&self.calls).push(format!("rm {slug}"));
        let mut fail = lock(&self.fail_removals);
        if *fail > 0 {
            *fail -= 1;
            return Err(WorkspaceError::Failed {
                command: format!("rm {slug}"),
                detail: "worktree is busy".into(),
            });
        }
        Ok(())
    }
}

struct FakeKeys;

impl KeySource for FakeKeys {
    fn get(&self, _item: &str) -> Result<Secret, KeyError> {
        Ok(Secret::new("test-key"))
    }
}

struct Harness {
    dir: TempDir,
    workspaces: Arc<FakeWorkspaces>,
    supervisor: Supervisor<FileStore>,
}

fn harness(configure: impl FnOnce(&mut CommandAdapter)) -> Harness {
    let dir = TempDir::new().expect("tempdir");
    let stub = dir.path().join("stub.sh");
    fs::write(&stub, STUB).expect("stub");
    fs::set_permissions(&stub, fs::Permissions::from_mode(0o755)).expect("chmod");
    let store = FileStore::create(
        dir.path().join("swarm"),
        SwarmId("test".into()),
        Utc::now(),
        Policy::default(),
    )
    .expect("store");
    store
        .update_roster(|roster| {
            roster.add_member(name("lead"), Member::new(Role::Coordinator, None))
        })
        .expect("coordinator");
    let mut adapter = CommandAdapter::new(&stub);
    adapter.args = vec![dir.path().display().to_string()];
    adapter.limits.poll = Duration::from_millis(10);
    adapter.limits.kill_grace = Duration::from_millis(300);
    configure(&mut adapter);
    let workspaces = Arc::new(FakeWorkspaces {
        root: dir.path().join("worktrees"),
        ..FakeWorkspaces::default()
    });
    let mut config = SupervisorConfig::new(dir.path(), dir.path().join("swarm"), "sw");
    config.poll = Duration::from_millis(20);
    config.keys = vec![KeyEnv {
        var: "API_KEY".into(),
        item: "api".into(),
    }];
    let supervisor = Supervisor::new(
        store,
        name("lead"),
        Box::new(adapter),
        Box::new(Arc::clone(&workspaces)),
        Box::new(FakeKeys),
        config,
    );
    Harness {
        dir,
        workspaces,
        supervisor,
    }
}

impl Harness {
    /// Adds the group to the roster, as the MCP server does, then starts it.
    fn spawn(&self, base: &str, workspace: Workspace, count: usize) -> Vec<MemberName> {
        let names =
            MemberName::group(base, NonZeroUsize::new(count).expect("nonzero")).expect("names");
        self.supervisor
            .store()
            .update_roster(|roster| {
                names.iter().try_for_each(|member| {
                    let mut entry = Member::new(Role::Worker, Some(name("lead")));
                    entry.model = Some("stub-model".into());
                    roster.add_member(member.clone(), entry)
                })
            })
            .expect("roster");
        for member in &names {
            self.supervisor
                .start(&SpawnSpec {
                    name: member.clone(),
                    group: base.into(),
                    brief: "do the thing".into(),
                    model: Some("stub-model".into()),
                    workspace,
                    channels: Vec::new(),
                    fork: None,
                })
                .expect("started");
        }
        names
    }

    fn turns(&self) -> Vec<String> {
        fs::read_to_string(self.dir.path().join("turns"))
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect()
    }

    fn state(&self, who: &str) -> MemberState {
        self.supervisor
            .store()
            .roster()
            .expect("roster")
            .member(&name(who))
            .expect("member")
            .state
    }

    fn outcome(&self, who: &str) -> Option<TurnOutcome> {
        self.supervisor.status(&name(who))?.last_outcome
    }

    fn calls(&self) -> Vec<String> {
        lock(&self.workspaces.calls).clone()
    }
}

fn eventually(what: &str, mut done: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !done() {
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        thread::sleep(Duration::from_millis(20));
    }
}

fn is_waiting(state: MemberState) -> bool {
    matches!(state, MemberState::Waiting { .. })
}

fn process_gone(pid_file: &Path) -> bool {
    let pid: i32 = fs::read_to_string(pid_file)
        .expect("pid file")
        .trim()
        .parse()
        .expect("pid");
    nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid), None).is_err()
}

#[test]
fn spawn_names_a_group_and_runs_each_member_once_in_its_worktree() {
    let h = harness(|_| {});
    let names = h.spawn("scout", Workspace::Branch, 3);
    assert_eq!(names, [name("scout-1"), name("scout-2"), name("scout-3")]);
    for who in ["scout-1", "scout-2", "scout-3"] {
        eventually("member to park", || is_waiting(h.state(who)));
        assert_eq!(h.outcome(who), Some(TurnOutcome::Completed));
    }
    let mut turns = h.turns();
    turns.sort();
    assert_eq!(
        turns,
        [
            "scout-1 do the thing",
            "scout-2 do the thing",
            "scout-3 do the thing"
        ]
    );
    let models = h.supervisor.member_models().expect("models");
    assert_eq!(models[&name("scout-2")].as_deref(), Some("stub-model"));
    assert_eq!(
        h.calls(),
        ["new sw-scout-1", "new sw-scout-2", "new sw-scout-3"]
    );

    let single = h.spawn("solo", Workspace::Inherit, 1);
    assert_eq!(single, [name("solo")]);
}

#[test]
fn each_turn_reaches_the_board_as_its_own_member() {
    let h = harness(|_| {});
    h.spawn("scout", Workspace::Inherit, 1);
    eventually("member to park", || is_waiting(h.state("scout")));
    let swarm = h.dir.path().join("swarm");
    let board = fs::read_to_string(h.dir.path().join("scout.board")).expect("board env");
    let mcp = serde_json::to_string(&[
        "coda-swarm",
        "mcp",
        "--dir",
        &swarm.display().to_string(),
        "--agent",
        "scout",
    ])
    .expect("json");
    assert_eq!(
        board.trim(),
        format!("scout {} test-key {mcp}", swarm.display())
    );
}

#[test]
fn mixed_outcomes_complete_crash_and_time_out() {
    let h = harness(|adapter| adapter.limits.turn_timeout = Some(Duration::from_millis(500)));
    h.spawn("done", Workspace::Inherit, 1);
    h.spawn("crash", Workspace::Inherit, 1);
    h.spawn("hang", Workspace::Inherit, 1);
    for who in ["done", "crash", "hang"] {
        eventually("member to park", || is_waiting(h.state(who)));
    }
    assert_eq!(h.outcome("done"), Some(TurnOutcome::Completed));
    assert_eq!(
        h.outcome("crash"),
        Some(TurnOutcome::Failed {
            reason: "exited with status 3".into()
        })
    );
    assert!(matches!(
        h.outcome("hang"),
        Some(TurnOutcome::Failed { reason }) if reason.contains("timed out")
    ));
    // The timeout took the whole process group, background child included.
    eventually("hang child to exit", || {
        process_gone(&h.dir.path().join("hang.child"))
    });
}

#[test]
fn kill_during_child_cleanup_escalates_to_sigkill() {
    let h = harness(|_| {});
    h.spawn("stubborn", Workspace::Share, 1);
    let child = h.dir.path().join("stubborn.child");
    eventually("turn to start", || child.exists());
    let started = Instant::now();
    let report = h.supervisor.kill(&name("stubborn")).expect("killed");
    assert_eq!(report.killed, [name("stubborn")]);
    assert!(started.elapsed() < Duration::from_secs(5));
    // The child was still in its SIGTERM handler when SIGKILL arrived.
    assert!(h.dir.path().join("stubborn.term").exists());
    assert!(process_gone(&child));
    assert_eq!(h.state("stubborn"), MemberState::Killed);
    assert_eq!(h.calls(), ["new sw-stubborn", "rm sw-stubborn"]);
    let again = h.supervisor.kill(&name("stubborn")).expect("kill again");
    assert!(again.killed.is_empty());
}

#[test]
fn failed_worktree_removal_stays_owned_and_is_retried() {
    let h = harness(|_| {});
    h.spawn("a", Workspace::Branch, 1);
    h.spawn("b", Workspace::Inherit, 1);
    *lock(&h.workspaces.fail_removals) = 1;
    let first = h.supervisor.kill(&name("a")).expect("kill a");
    assert_eq!(first.killed, [name("a")]);
    assert_eq!(first.pending_cleanup, ["sw-a"]);
    assert_eq!(first.cleanup_errors.len(), 1);
    assert_eq!(h.state("a"), MemberState::Killed);

    let second = h.supervisor.kill(&name("b")).expect("kill b");
    assert!(second.pending_cleanup.is_empty());
    assert_eq!(h.calls(), ["new sw-a", "rm sw-a", "rm sw-a"]);
}

#[test]
fn shutdown_retries_pending_cleanup() {
    let h = harness(|_| {});
    h.spawn("a", Workspace::Branch, 1);
    *lock(&h.workspaces.fail_removals) = 1;
    assert_eq!(
        h.supervisor.kill(&name("a")).expect("kill").pending_cleanup,
        ["sw-a"]
    );
    let report = h.supervisor.shutdown().expect("shutdown");
    assert!(report.pending_cleanup.is_empty());
    assert_eq!(h.calls(), ["new sw-a", "rm sw-a", "rm sw-a"]);
}

#[test]
fn shared_worktree_is_removed_after_its_last_member() {
    let h = harness(|_| {});
    h.spawn("pair", Workspace::Share, 2);
    h.supervisor.kill(&name("pair-1")).expect("kill one");
    assert_eq!(h.calls(), ["new sw-pair"]);
    h.supervisor.kill(&name("pair-2")).expect("kill other");
    assert_eq!(h.calls(), ["new sw-pair", "rm sw-pair"]);
}

#[test]
fn waiting_member_wakes_for_a_mention() {
    let h = harness(|_| {});
    h.spawn("scout", Workspace::Inherit, 1);
    eventually("member to park", || is_waiting(h.state("scout")));
    h.supervisor
        .store()
        .append(Draft::post(name("lead"), "status, please"), Utc::now())
        .expect("plain post");
    h.supervisor
        .store()
        .append(Draft::post(name("lead"), "@scout report"), Utc::now())
        .expect("mention");
    // The wake prompt is two lines: the header, then the post text.
    eventually("second turn", || h.turns().len() == 3);
    assert_eq!(h.turns()[1], "scout New post #2 from lead in #main:");
    assert_eq!(h.turns()[2], "@scout report");
    eventually("member to park again", || is_waiting(h.state("scout")));
    let MemberState::Waiting { wait_since, .. } = h.state("scout") else {
        unreachable!("checked above");
    };
    assert_eq!(wait_since, Seq(2));
}

#[test]
fn quota_exhaustion_pauses_and_posts_a_notice() {
    let h = harness(|_| {});
    h.spawn("quota", Workspace::Inherit, 1);
    eventually("pause", || {
        h.supervisor
            .status(&name("quota"))
            .is_some_and(|status| status.paused_until.is_some())
    });
    assert_eq!(
        h.outcome("quota"),
        Some(TurnOutcome::QuotaExhausted {
            retry_after: Duration::from_secs(600)
        })
    );
    eventually("notice", || {
        h.supervisor
            .store()
            .read_since(Seq::ZERO, 10)
            .expect("board")
            .iter()
            .any(|m| m.agent == name("quota") && m.text.contains("quota exhausted"))
    });
    assert_eq!(h.state("quota"), MemberState::Stopped);
    h.supervisor
        .kill(&name("quota"))
        .expect("kill while paused");
    assert_eq!(h.state("quota"), MemberState::Killed);
}
