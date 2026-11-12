use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::message::{Draft, Message, Seq};
use crate::policy::Policy;
use crate::roster::Roster;
use crate::store::{
    Admitted, AppendError, Backend, Store, StoreError, SwarmId, SwarmMeta, UpdateError, admit,
};

const META: &str = "swarm.json";
const BOARD: &str = "board.jsonl";
const MEMBERS: &str = "members.json";
const LOCK: &str = ".lock";

/// A swarm directory holding `swarm.json`, an append-only `board.jsonl` and
/// `members.json`. Every operation holds an advisory lock on `.lock`, so
/// handles in different processes see one ordered board.
#[derive(Debug, Clone)]
pub struct FileStore {
    dir: PathBuf,
    policy: Policy,
}

enum Access {
    Read,
    Write,
}

impl FileStore {
    /// Opens the swarm in `dir`, creating its anchor when none exists yet.
    /// An existing swarm is opened as it is.
    pub fn create(
        dir: impl Into<PathBuf>,
        swarm_id: SwarmId,
        now: DateTime<Utc>,
        policy: Policy,
    ) -> Result<Self, StoreError> {
        let store = Self {
            dir: dir.into(),
            policy,
        };
        fs::create_dir_all(&store.dir).map_err(|source| StoreError::Io {
            action: "create swarm directory",
            path: store.dir.clone(),
            source,
        })?;
        let _lock = store.lock(Access::Write)?;
        if !store.path(META).exists() {
            let meta = SwarmMeta {
                version: SwarmMeta::VERSION,
                backend: Backend::Local,
                swarm_id,
                created_at: now,
            };
            store.write_json(META, "swarm meta", &meta)?;
        }
        store.read_meta()?;
        Ok(store)
    }

    /// Opens an existing swarm; fails if `dir` holds no valid anchor.
    pub fn open(dir: impl Into<PathBuf>, policy: Policy) -> Result<Self, StoreError> {
        let store = Self {
            dir: dir.into(),
            policy,
        };
        let _lock = store.lock(Access::Read)?;
        store.read_meta()?;
        Ok(store)
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    fn io(&self, action: &'static str, name: &str) -> impl FnOnce(io::Error) -> StoreError {
        let path = self.path(name);
        move |source| StoreError::Io {
            action,
            path,
            source,
        }
    }

    /// The lock is held until the returned file is dropped.
    fn lock(&self, access: Access) -> Result<File, StoreError> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(self.path(LOCK))
            .map_err(self.io("open lock file", LOCK))?;
        match access {
            Access::Read => file.lock_shared(),
            Access::Write => file.lock(),
        }
        .map_err(self.io("lock", LOCK))?;
        Ok(file)
    }

    fn read_meta(&self) -> Result<SwarmMeta, StoreError> {
        let meta: SwarmMeta = self
            .read_json(META)?
            .ok_or_else(|| self.io("read", META)(io::ErrorKind::NotFound.into()))?;
        if meta.version != SwarmMeta::VERSION {
            return Err(StoreError::Version {
                path: self.path(META),
                found: meta.version,
            });
        }
        Ok(meta)
    }

    fn read_json<T: DeserializeOwned>(&self, name: &str) -> Result<Option<T>, StoreError> {
        let bytes = match fs::read(self.path(name)) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(self.io("read", name)(error)),
        };
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|source| StoreError::Invalid {
                path: self.path(name),
                source,
            })
    }

    /// Replaces `name` by renaming a fully written temp file over it, so a
    /// reader never sees a partial file.
    fn write_json<T: Serialize>(
        &self,
        name: &str,
        what: &'static str,
        value: &T,
    ) -> Result<(), StoreError> {
        let mut bytes = serde_json::to_vec_pretty(value)
            .map_err(|source| StoreError::Encode { what, source })?;
        bytes.push(b'\n');
        let mut temp =
            tempfile::NamedTempFile::new_in(&self.dir).map_err(self.io("create temp for", name))?;
        temp.write_all(&bytes)
            .and_then(|()| temp.as_file().sync_all())
            .map_err(self.io("write temp for", name))?;
        temp.persist(self.path(name))
            .map_err(|error| self.io("replace", name)(error.error))?;
        Ok(())
    }

    fn read_roster(&self) -> Result<Roster, StoreError> {
        Ok(self.read_json(MEMBERS)?.unwrap_or_default())
    }

    fn read_board(&self) -> Result<Vec<Message>, StoreError> {
        let file = match File::open(self.path(BOARD)) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(self.io("open", BOARD)(error)),
        };
        let mut board = Vec::new();
        for (index, line) in BufReader::new(file).lines().enumerate() {
            let line = line.map_err(self.io("read", BOARD))?;
            if line.trim().is_empty() {
                continue;
            }
            let message = serde_json::from_str(&line).map_err(|source| StoreError::Corrupt {
                path: self.path(BOARD),
                line: index + 1,
                source,
            })?;
            board.push(message);
        }
        Ok(board)
    }

    fn append_line(&self, message: &Message) -> Result<(), StoreError> {
        let mut line = serde_json::to_vec(message).map_err(|source| StoreError::Encode {
            what: "board message",
            source,
        })?;
        line.push(b'\n');
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path(BOARD))
            .map_err(self.io("open", BOARD))?;
        file.write_all(&line)
            .and_then(|()| file.sync_data())
            .map_err(self.io("append to", BOARD))
    }
}

impl Store for FileStore {
    fn meta(&self) -> Result<SwarmMeta, StoreError> {
        let _lock = self.lock(Access::Read)?;
        self.read_meta()
    }

    fn head(&self) -> Result<Seq, StoreError> {
        let _lock = self.lock(Access::Read)?;
        Ok(self.read_board()?.last().map_or(Seq::ZERO, |m| m.seq))
    }

    fn append(&self, draft: Draft, now: DateTime<Utc>) -> Result<Message, AppendError> {
        let _lock = self.lock(Access::Write)?;
        let board = self.read_board()?;
        let roster = self.read_roster()?;
        match admit(&self.policy, &roster, &board, draft, now)? {
            Admitted::Duplicate(message) => Ok(message),
            Admitted::New(message) => {
                self.append_line(&message)?;
                Ok(message)
            }
        }
    }

    fn read_since(&self, since: Seq, limit: usize) -> Result<Vec<Message>, StoreError> {
        let _lock = self.lock(Access::Read)?;
        Ok(self
            .read_board()?
            .into_iter()
            .filter(|m| m.seq > since)
            .take(limit)
            .collect())
    }

    fn roster(&self) -> Result<Roster, StoreError> {
        let _lock = self.lock(Access::Read)?;
        self.read_roster()
    }

    fn update_roster<T, E>(
        &self,
        change: impl FnOnce(&mut Roster) -> Result<T, E>,
    ) -> Result<T, UpdateError<E>> {
        let _lock = self.lock(Access::Write)?;
        let mut roster = self.read_roster()?;
        let value = change(&mut roster).map_err(UpdateError::Rejected)?;
        self.write_json(MEMBERS, "roster", &roster)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;
    use std::process::Command;
    use std::time::Duration;

    use super::*;
    use crate::message::{MessageKind, RequestId};
    use crate::names::{ChannelName, MemberName};
    use crate::policy::RateLimit;
    use crate::roster::{ChannelError, Member, Role, RosterError, Topic};
    use crate::store::Rejection;

    const CHILD_DIR: &str = "CODA_SWARM_TEST_CHILD_DIR";
    const CHILD_AUTHOR: &str = "CODA_SWARM_TEST_CHILD_AUTHOR";
    const POSTS_EACH: usize = 40;

    fn name(text: &str) -> MemberName {
        MemberName::parse(text).expect("valid")
    }

    fn at(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_800_000_000 + secs, 0).expect("valid")
    }

    fn unlimited() -> Policy {
        Policy {
            rate: RateLimit {
                max_posts: NonZeroU32::MAX,
                window: Duration::from_secs(1),
            },
            ..Policy::default()
        }
    }

    fn swarm(dir: &Path, policy: Policy, members: &[&str]) -> FileStore {
        let store =
            FileStore::create(dir, SwarmId("conv-1".into()), at(0), policy).expect("created");
        store
            .update_roster(|roster| {
                members.iter().try_for_each(|who| {
                    roster.add_member(name(who), Member::new(Role::Worker, None))
                })
            })
            .expect("members added");
        store
    }

    fn assert_dense(board: &[Message], expected: usize) {
        let seqs: Vec<u64> = board.iter().map(|m| m.seq.0).collect();
        let want: Vec<u64> = (1..=expected as u64).collect();
        assert_eq!(seqs, want, "seqs are unique, ordered and gapless");
    }

    #[test]
    fn create_is_idempotent_and_writes_the_anchor() {
        let dir = tempfile::tempdir().expect("tempdir");
        FileStore::create(dir.path(), SwarmId("a".into()), at(0), Policy::default())
            .expect("created");
        let again = FileStore::create(dir.path(), SwarmId("b".into()), at(9), Policy::default())
            .expect("reopened");
        let meta = again.meta().expect("meta");
        assert_eq!(meta.swarm_id, SwarmId("a".into()));
        assert_eq!(meta.created_at, at(0));
        let raw: serde_json::Value =
            serde_json::from_slice(&fs::read(dir.path().join(META)).expect("read")).expect("json");
        assert_eq!(raw["version"], 1);
        assert_eq!(raw["backend"], "local");
    }

    #[test]
    fn open_requires_an_existing_swarm() {
        let dir = tempfile::tempdir().expect("tempdir");
        assert!(matches!(
            FileStore::open(dir.path(), Policy::default()),
            Err(StoreError::Io { .. })
        ));
    }

    #[test]
    fn append_assigns_seqs_and_read_since_pages_forward() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = swarm(dir.path(), unlimited(), &["lead"]);
        for (index, text) in ["one", "two", "three"].into_iter().enumerate() {
            let message = store
                .append(Draft::post(name("lead"), text), at(index as i64))
                .expect("appended");
            assert_eq!(message.seq, Seq(index as u64 + 1));
        }
        assert_eq!(store.head().expect("head"), Seq(3));
        let page: Vec<String> = store
            .read_since(Seq(1), 1)
            .expect("read")
            .into_iter()
            .map(|m| m.text)
            .collect();
        assert_eq!(page, ["two"]);
        let reopened = FileStore::open(dir.path(), unlimited()).expect("opened");
        assert_dense(&reopened.read_since(Seq::ZERO, 10).expect("read"), 3);
    }

    #[test]
    fn append_rejects_strangers_oversize_and_unknown_channels() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = swarm(dir.path(), Policy::default(), &["lead"]);
        let reject = |draft: Draft| match store.append(draft, at(0)) {
            Err(AppendError::Rejected(rejection)) => rejection,
            other => panic!("expected rejection, got {other:?}"),
        };
        assert_eq!(
            reject(Draft::post(name("ghost"), "hi")),
            Rejection::NotMember(name("ghost"))
        );
        assert_eq!(reject(Draft::post(name("lead"), "  ")), Rejection::Empty);
        let big = "x".repeat(Policy::default().max_post_bytes + 1);
        assert!(matches!(
            reject(Draft::post(name("lead"), big)),
            Rejection::TooLarge { .. }
        ));
        let ops = ChannelName::parse("ops").expect("valid");
        let mut draft = Draft::post(name("lead"), "hi");
        draft.channel = Some(ops.clone());
        assert_eq!(reject(draft), ChannelError::Unknown(ops).into());
        assert_eq!(store.head().expect("head"), Seq::ZERO);
    }

    #[test]
    fn append_defaults_to_home_channel_and_refuses_archived() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = swarm(dir.path(), Policy::default(), &["lead"]);
        let ops = ChannelName::parse("ops").expect("valid");
        store
            .update_roster(|roster| {
                roster.create_channel(
                    ops.clone(),
                    Topic::parse("ops").expect("valid"),
                    name("lead"),
                )?;
                roster.subscribe(&name("lead"), &ops)
            })
            .expect("channel created");
        let posted = store
            .append(Draft::post(name("lead"), "hi"), at(0))
            .expect("appended");
        assert_eq!(posted.channel, ops);
        store
            .update_roster(|roster| roster.archive_channel(&ops, &name("lead")))
            .expect("archived");
        let mut draft = Draft::post(name("lead"), "late");
        draft.channel = Some(ops.clone());
        assert!(matches!(
            store.append(draft, at(1)),
            Err(AppendError::Rejected(Rejection::Channel(
                ChannelError::Archived(_)
            )))
        ));
        let fallback = store
            .append(Draft::post(name("lead"), "hi"), at(2))
            .expect("appended");
        assert!(fallback.channel.is_main());
    }

    #[test]
    fn rate_limit_applies_per_author_and_skips_briefs() {
        let dir = tempfile::tempdir().expect("tempdir");
        let policy = Policy {
            rate: RateLimit {
                max_posts: NonZeroU32::new(1).expect("nonzero"),
                window: Duration::from_secs(10),
            },
            ..Policy::default()
        };
        let store = swarm(dir.path(), policy, &["lead", "scout"]);
        store
            .append(Draft::post(name("lead"), "a"), at(0))
            .expect("first post");
        assert!(matches!(
            store.append(Draft::post(name("lead"), "b"), at(3)),
            Err(AppendError::Rejected(Rejection::RateLimited { retry_after }))
                if retry_after == Duration::from_secs(7)
        ));
        let mut brief = Draft::post(name("lead"), "@scout do it");
        brief.kind = MessageKind::Brief;
        store.append(brief, at(3)).expect("briefs are not limited");
        store
            .append(Draft::post(name("scout"), "ok"), at(3))
            .expect("other authors are not limited");
        store
            .append(Draft::post(name("lead"), "c"), at(10))
            .expect("window passed");
    }

    #[test]
    fn request_id_makes_retries_land_once() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = swarm(dir.path(), unlimited(), &["lead"]);
        let mut draft = Draft::post(name("lead"), "once");
        draft.request_id = Some(RequestId("r-1".into()));
        let first = store.append(draft.clone(), at(0)).expect("appended");
        let retry = store.append(draft, at(1)).expect("deduplicated");
        assert_eq!(first, retry);
        assert_eq!(store.head().expect("head"), Seq(1));
    }

    #[test]
    fn failed_roster_update_saves_nothing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = swarm(dir.path(), unlimited(), &["lead"]);
        let result = store.update_roster(|roster| {
            roster.title = "renamed".into();
            roster.add_member(name("lead"), Member::new(Role::Worker, None))
        });
        assert!(matches!(
            result,
            Err(UpdateError::Rejected(RosterError::Exists(_)))
        ));
        assert_eq!(store.roster().expect("roster").title, "");
        store
            .update_roster(|roster| roster.remove_member(&name("lead")))
            .expect("removed");
        assert!(store.roster().expect("roster").members.is_empty());
    }

    #[test]
    fn corrupt_board_line_fails_rather_than_reading_empty() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = swarm(dir.path(), unlimited(), &["lead"]);
        fs::write(dir.path().join(BOARD), "{not json\n").expect("write");
        assert!(matches!(
            store.read_since(Seq::ZERO, 10),
            Err(StoreError::Corrupt { line: 1, .. })
        ));
    }

    #[test]
    fn concurrent_threads_keep_seqs_unique_and_ordered() {
        let dir = tempfile::tempdir().expect("tempdir");
        let authors = ["a", "b", "c", "d"];
        swarm(dir.path(), unlimited(), &authors);
        std::thread::scope(|scope| {
            for author in authors {
                let path = dir.path();
                scope.spawn(move || {
                    let store = FileStore::open(path, unlimited()).expect("opened");
                    for index in 0..POSTS_EACH {
                        store
                            .append(Draft::post(name(author), index.to_string()), at(0))
                            .expect("appended");
                    }
                });
            }
        });
        let store = FileStore::open(dir.path(), unlimited()).expect("opened");
        assert_dense(
            &store.read_since(Seq::ZERO, usize::MAX).expect("read"),
            authors.len() * POSTS_EACH,
        );
    }

    #[test]
    fn concurrent_processes_keep_seqs_unique_and_ordered() {
        let dir = tempfile::tempdir().expect("tempdir");
        let authors = ["p", "q"];
        swarm(dir.path(), unlimited(), &authors);
        let exe = std::env::current_exe().expect("test binary");
        let children: Vec<_> = authors
            .iter()
            .map(|author| {
                Command::new(&exe)
                    .args(["--exact", "file_store::tests::child_appender", "--ignored"])
                    .env(CHILD_DIR, dir.path())
                    .env(CHILD_AUTHOR, author)
                    .spawn()
                    .expect("child started")
            })
            .collect();
        for mut child in children {
            assert!(child.wait().expect("child exited").success());
        }
        let store = FileStore::open(dir.path(), unlimited()).expect("opened");
        let board = store.read_since(Seq::ZERO, usize::MAX).expect("read");
        assert_dense(&board, authors.len() * POSTS_EACH);
        for author in authors {
            let theirs: Vec<String> = board
                .iter()
                .filter(|m| m.agent == name(author))
                .map(|m| m.text.clone())
                .collect();
            let want: Vec<String> = (0..POSTS_EACH).map(|i| i.to_string()).collect();
            assert_eq!(theirs, want, "each process's posts keep their order");
        }
    }

    #[test]
    #[ignore = "run as a child by concurrent_processes_keep_seqs_unique_and_ordered"]
    fn child_appender() {
        let (Ok(dir), Ok(author)) = (std::env::var(CHILD_DIR), std::env::var(CHILD_AUTHOR)) else {
            return;
        };
        let store = FileStore::open(dir, unlimited()).expect("opened");
        for index in 0..POSTS_EACH {
            store
                .append(Draft::post(name(&author), index.to_string()), at(0))
                .expect("appended");
        }
    }
}
