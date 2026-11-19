//! Patient memory: the markdown files Coda keeps for one patient in one
//! practice, and the rules every write holds. The feature spec is
//! `verification/physician/memory/README.md`.
//!
//! Every function reads or writes only one practice and patient, named by a
//! [`PatientKey`] that an authorized [`Scope`] makes, or by that scope
//! itself. Patient rounds, which no physician starts, have their own maker
//! of a [`PatientKey`]. A run adds notes through [`Store::append_note`];
//! [`Store::edit_memory`] is the one write of the summary, the direction,
//! the person page and the subject pages, each of which is its markdown
//! text. A patient rounds run that finishes saves a rounds mark, and
//! [`Store::last_rounds`] reads the newest. The memory tables are insert and
//! select only, so a saved note never changes and every earlier version
//! stays readable. A write applies to the file as it stands: writes for one
//! patient take the patient's memory lock, so they apply one after the other
//! and none loses another's edits.
//!
//! The `page` module holds the stored text, the edits and the generated log,
//! with no I/O. The [`tools`] module holds the tools a chat run and a reply
//! revision are offered: three reads, the note and the edit. Every request
//! of a chat or a reply opens with the memory the conversation started
//! with, which [`Store::memory_at_start`] builds. The [`edit`] module holds
//! the edit tool, which chat, reply revisions and patient rounds
//! ([`super::rounds`]) all run.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use chrono::{DateTime, Utc};
use harness::engine::Item;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
    sea_query::{Alias, Expr, Func, Query},
};

pub mod edit;
mod page;
pub mod tools;
pub use page::{
    Day, Edit, NoteId, NoteNumber, OneLine, START_NOTES_CHARACTERS, START_SUMMARY_CHARACTERS, Words,
};

use super::entities::{memory_note, memory_rounds, memory_version};
use super::{ConversationStart, Scope, Store, conversion_error, error, now};
use page::{StartOf, Text};

/// The memory a conversation started with, from [`Store::memory_at_start`]:
/// the developer message every request and fresh window of the conversation
/// opens with, or nothing when the patient had no memory then.
#[derive(Clone, Debug, PartialEq)]
pub struct StartMemory(pub(super) Option<Item>);

impl StartMemory {
    /// `items` with the memory message first.
    pub fn first(self, items: impl IntoIterator<Item = Item>) -> Vec<Item> {
        self.0.into_iter().chain(items).collect()
    }
}

/// What a patient rounds run starts with, from [`Store::rounds_start`].
pub(super) struct RoundsStart {
    /// The memory message, built as a conversation's is.
    pub(super) memory: StartMemory,
    /// The newest rounds mark saved before the run started; `None` when no
    /// run had finished.
    pub(super) last: Option<RoundsMark>,
    /// Each log day with notes the memory message chose from, and how many,
    /// oldest day first.
    pub(super) new_notes: Vec<(Day, usize)>,
}

/// One patient in one practice. An authorized [`Scope`] makes one, and so
/// does [`PatientKey::for_rounds`] for patient rounds, so no memory function
/// takes a loose practice or workspace.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PatientKey {
    practice: String,
    workspace: String,
}

impl PatientKey {
    /// The patient of a rounds run. No physician starts rounds, so no
    /// signed-in scope can make this key. A blank practice or workspace is
    /// an error.
    ///
    /// Infra seam: production must confirm the practice has this patient
    /// before making it.
    pub fn for_rounds(practice: &str, workspace: &str) -> Result<Self, DbErr> {
        if practice.trim().is_empty() || workspace.trim().is_empty() {
            return Err(error("empty rounds patient"));
        }
        Ok(Self {
            practice: practice.to_owned(),
            workspace: workspace.to_owned(),
        })
    }

    /// The patient of a practice-owned background job. No physician owns a
    /// practice job, so no signed-in `Scope` can make this key.
    pub(crate) fn for_practice_job(scope: &super::jobs::PracticeScope) -> Result<Self, DbErr> {
        if scope.job_id.trim().is_empty() {
            return Err(error("empty practice job id"));
        }
        Self::for_rounds(&scope.practice, &scope.workspace)
    }

    /// The patient's workspace, whose chart a run reads.
    pub(crate) fn workspace(&self) -> &str {
        &self.workspace
    }
}

impl TryFrom<&Scope> for PatientKey {
    type Error = DbErr;
    /// The patient of a scope the server authorized. A blank scope field is
    /// an error.
    fn try_from(scope: &Scope) -> Result<Self, DbErr> {
        scope.validate()?;
        Ok(Self {
            practice: scope.practice.clone(),
            workspace: scope.workspace.clone(),
        })
    }
}

/// A subject page's name in its path: lowercase letters and digits, with one
/// hyphen between each run of them.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageId(String);

impl PageId {
    /// The page id `text` names: lowercased, each run of characters that are
    /// not letters or digits one hyphen, and no hyphen at either end. `None`
    /// when no letter or digit is left.
    pub fn parse(text: &str) -> Option<Self> {
        let mut id = String::new();
        let mut gap = false;
        for c in text.chars().flat_map(char::to_lowercase) {
            if !c.is_alphanumeric() {
                gap = true;
                continue;
            }
            if gap && !id.is_empty() {
                id.push('-');
            }
            gap = false;
            id.push(c);
        }
        (!id.is_empty()).then_some(Self(id))
    }
}

/// One memory file. The order of the variants, then of page ids and days,
/// is the order files are listed and searched in.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryPath {
    /// `summary.md`.
    Summary,
    /// `direction.md`.
    Direction,
    /// `person.md`.
    Person,
    /// `pages/<id>.md`.
    Page(PageId),
    /// `log/<YYYY-MM-DD>.md`.
    Log(Day),
}

impl MemoryPath {
    /// The file `text` names. The one parser of a path, for the tool boundary
    /// and for stored rows. A page's id is taken in its normal form, with or
    /// without `.md`, so `pages/CKD Stage 3.md` names `pages/ckd-stage-3.md`;
    /// a stored path is already in that form.
    pub fn parse(text: &str) -> Result<Self, MemoryRefusal> {
        let unknown = || MemoryRefusal::UnknownPath {
            path: text.to_owned(),
        };
        let path = match text {
            "summary.md" => Self::Summary,
            "direction.md" => Self::Direction,
            "person.md" => Self::Person,
            _ => {
                if let Some(id) = text.strip_prefix("pages/") {
                    let id = id.strip_suffix(".md").unwrap_or(id);
                    Self::Page(PageId::parse(id).ok_or_else(unknown)?)
                } else if let Some(day) = text
                    .strip_prefix("log/")
                    .and_then(|day| day.strip_suffix(".md"))
                {
                    Self::Log(Day::parse(day).ok_or_else(unknown)?)
                } else {
                    return Err(unknown());
                }
            }
        };
        Ok(path)
    }
}

impl fmt::Display for MemoryPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Summary => f.write_str("summary.md"),
            Self::Direction => f.write_str("direction.md"),
            Self::Person => f.write_str("person.md"),
            Self::Page(id) => write!(f, "pages/{}.md", id.0),
            Self::Log(day) => write!(f, "log/{day}.md"),
        }
    }
}

/// A memory file that has versions, which edits change: every memory file
/// but a log day. The order of the variants, then of page ids, is the order
/// files are listed and searched in.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionedFile {
    /// `summary.md`.
    Summary,
    /// `direction.md`.
    Direction,
    /// `person.md`.
    Person,
    /// `pages/<id>.md`.
    Page(PageId),
}

impl From<VersionedFile> for MemoryPath {
    fn from(file: VersionedFile) -> Self {
        match file {
            VersionedFile::Summary => Self::Summary,
            VersionedFile::Direction => Self::Direction,
            VersionedFile::Person => Self::Person,
            VersionedFile::Page(id) => Self::Page(id),
        }
    }
}

impl TryFrom<MemoryPath> for VersionedFile {
    /// The day of a log path: a log day is the one path that is not a
    /// versioned file.
    type Error = Day;

    /// The versioned file at `path`, or the day when `path` is a log day.
    fn try_from(path: MemoryPath) -> Result<Self, Day> {
        match path {
            MemoryPath::Summary => Ok(Self::Summary),
            MemoryPath::Direction => Ok(Self::Direction),
            MemoryPath::Person => Ok(Self::Person),
            MemoryPath::Page(id) => Ok(Self::Page(id)),
            MemoryPath::Log(day) => Err(day),
        }
    }
}

/// A memory call the model can correct. Nothing was saved.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MemoryRefusal {
    /// The path names no memory file.
    #[error("no memory file is named {path:?}")]
    UnknownPath {
        /// The path as given.
        path: String,
    },
    /// A note with no text.
    #[error("a note needs text")]
    EmptyNote,
    /// Text that must have words, such as a note's quote or an edit's
    /// text, has none.
    #[error("the text has no words")]
    NoWords,
    /// The file does not exist.
    #[error("there is no file {0}")]
    NoFile(MemoryPath),
    /// The file has no such version.
    #[error("{path} has no version {version}; it is at version {current}")]
    NoVersion {
        /// The file.
        path: MemoryPath,
        /// The version asked for.
        version: u64,
        /// The file's current version.
        current: u64,
    },
    /// A version of a log day was asked for; a log day has none.
    #[error("a log day has no versions")]
    LogNotVersioned,
    /// A search with no words.
    #[error("a search needs at least one word")]
    EmptyQuery,
}

/// Why one edit of a write was left out. The edit changed nothing, and the
/// write's other edits still apply.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EditRefusal {
    /// The edit's own fields did not parse.
    #[error(transparent)]
    Input(#[from] EditInputError),
    /// A replace whose `old_text` is nowhere in the file exactly as given.
    #[error("old_text is not in the file exactly as given")]
    NotFound,
    /// A replace whose `old_text` is in the file in more than one place.
    #[error(
        "old_text is in the file more than once, on {}; give more of the text so it names one place",
        on_lines(.lines)
    )]
    ManyPlaces {
        /// The line, from 1, each place starts on, each line once.
        lines: Vec<usize>,
    },
    /// An edit that would leave its file exactly as it is, such as a
    /// replace with the named text itself.
    #[error("nothing to change: the file already reads this")]
    NoChange,
    /// A create of a file that exists.
    #[error("the file exists; change it with replace or append")]
    Exists,
    /// A replace or a delete of a file that does not exist.
    #[error("there is no file {0}; create makes it")]
    NoFile(MemoryPath),
}

/// One edit of a call whose own fields do not parse. The edit is left out,
/// and the call's other edits still apply.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EditInputError {
    /// The edit did not match the offered fields. `serde_json::Error` is
    /// neither `Clone` nor comparable, and a `from_value` error is only its
    /// message, so the message is kept.
    #[error(
        "Give the edit as an object that holds one of create, replace, append or delete, with the fields each takes ({0})."
    )]
    Shape(String),
    /// Text that must have words has none.
    #[error("Give {field} as one or more words ({source}).")]
    Words {
        /// The field as the tool names it.
        field: &'static str,
        /// The refusal.
        #[source]
        source: MemoryRefusal,
    },
}

/// Line numbers as a reason gives them: `line 4`, `lines 3, 4` or
/// `lines 2, 3, 4`.
fn on_lines(lines: &[usize]) -> String {
    let word = if lines.len() == 1 { "line" } else { "lines" };
    let numbers: Vec<String> = lines.iter().map(ToString::to_string).collect();
    format!("{word} {}", numbers.join(", "))
}

/// One edit a write left out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotApplied {
    /// The edit's place in its list, from 0.
    pub edit: usize,
    /// Why it was left out.
    pub reason: EditRefusal,
}

/// A memory call that did not complete: a refusal the model can act on, or
/// the store's own failure.
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    /// The call was refused; nothing was saved.
    #[error(transparent)]
    Refused(#[from] MemoryRefusal),
    /// Mutation cited or originated from a withdrawn wrong-patient source or fenced run.
    #[error("{0}")]
    WithdrawnFence(#[from] super::follow_ups::WithdrawalFenceViolation),
    /// The database refused or failed the operation.
    #[error(transparent)]
    Database(#[from] DbErr),
}

impl From<super::follow_ups::WithdrawalFenceCheckError> for MemoryError {
    fn from(err: super::follow_ups::WithdrawalFenceCheckError) -> Self {
        match err {
            super::follow_ups::WithdrawalFenceCheckError::Violation(v) => Self::WithdrawnFence(v),
            super::follow_ups::WithdrawalFenceCheckError::Database(db) => Self::Database(db),
        }
    }
}

/// What a note may carry from the model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoteDraft {
    /// The note.
    pub text: String,
    /// Words the model quotes from the physician's message, as it gives
    /// them.
    pub quote: Option<OneLine>,
}

impl NoteDraft {
    /// Refuses a draft with no text.
    fn check(&self) -> Result<(), MemoryRefusal> {
        if self.text.trim().is_empty() {
            return Err(MemoryRefusal::EmptyNote);
        }
        Ok(())
    }
}

/// The run's own part of a note's origin, never the model's. The practice,
/// patient, physician and chat come from the run's authorized scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoteOrigin {
    /// The run's work.
    pub work: String,
    /// The tool call that adds the note.
    pub call: String,
    /// The id of the physician message the note comes from: the newest
    /// message the run's work answers.
    pub message: String,
}

/// A saved note.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    /// The note's identity, its row's key.
    pub id: NoteId,
    /// Its place among the patient's notes, from 1.
    pub number: NoteNumber,
    /// The store's clock when it was saved.
    pub time: DateTime<Utc>,
    /// The physician whose run added it.
    ///
    /// Infra seam: this is the physician's stored id; memory has no display
    /// name for them.
    pub physician: String,
    /// The chat or reply work the run belongs to.
    pub chat: String,
    /// The id of the physician message the note comes from.
    pub message: String,
    /// The note.
    pub text: String,
    /// The physician's words, as the model quoted them.
    pub physician_words: Option<OneLine>,
}

/// A rounds mark: a patient rounds run that finished, as memory records it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoundsMark {
    /// The time the run was given as its start, in whole seconds.
    pub started: DateTime<Utc>,
    /// The store's clock when the run finished and the mark was saved.
    pub saved: DateTime<Utc>,
}

/// A write's outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Written {
    /// The version saved, or the current one when no edit applied; `None`
    /// when the file has no version.
    pub version: Option<u64>,
    /// The size of the file's text as it now stands, in characters.
    pub characters: usize,
    /// Each edit left out, in edit order.
    pub not_applied: Vec<NotApplied>,
}

/// A file as the listing shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Listed {
    /// Its current version.
    pub version: u64,
    /// The size of its text, in characters.
    pub characters: usize,
}

/// The patient's files that exist, and the log.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Listing {
    /// The summary.
    pub summary: Option<Listed>,
    /// The direction.
    pub direction: Option<Listed>,
    /// The person page.
    pub person: Option<Listed>,
    /// Each subject page, by id.
    pub pages: BTreeMap<PageId, Listed>,
    /// Each day that has notes or a rounds mark, oldest first.
    pub log_days: Vec<Day>,
}

/// A file as read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileRead {
    /// A version of the summary, the direction or a page.
    Version {
        /// The version read.
        version: u64,
        /// The file as markdown.
        markdown: String,
    },
    /// A log day, which has no versions.
    Log {
        /// The day's notes as markdown.
        markdown: String,
    },
}

/// One line search found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    /// The file.
    pub path: MemoryPath,
    /// The line's number in the file, from 1.
    pub line: usize,
    /// The line.
    pub text: String,
}

/// One physician-facing patient memory document (`summary.md` or `direction.md`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicianMemoryDocument {
    /// The current version number.
    pub version: u64,
    /// The UTC day this version was saved.
    pub saved: Day,
    /// The document's markdown text.
    pub markdown: String,
}

/// The physician-facing memory view (`Memory.Code.20`): only the summary and
/// the direction. Subject pages, `person.md`, and the dated log are Coda's
/// working files and are structurally excluded from this type.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PhysicianMemory {
    /// Current `summary.md`, when present.
    pub summary: Option<PhysicianMemoryDocument>,
    /// Current `direction.md`, when present.
    pub direction: Option<PhysicianMemoryDocument>,
}

/// Which physician-facing memory document a physician edited (`Memory.Code.22`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhysicianMemoryTarget {
    /// `summary.md`.
    Summary,
    /// `direction.md`.
    Direction,
}

impl PhysicianMemoryTarget {
    /// The memory file path this target refers to.
    pub const fn path(self) -> MemoryPath {
        match self {
            Self::Summary => MemoryPath::Summary,
            Self::Direction => MemoryPath::Direction,
        }
    }
}

/// One saved version of a versioned file.
struct Version {
    number: u64,
    /// The UTC day the store's clock saved it.
    saved: Day,
    text: Text,
}

fn version_number(stored: i64) -> Result<u64, DbErr> {
    u64::try_from(stored).map_err(conversion_error("stored memory version", "u64"))
}

/// The one reader of a version row: its number, the day it was saved, and
/// its text.
fn stored_version(row: memory_version::Model) -> Result<Version, DbErr> {
    let saved = DateTime::from_timestamp(row.saved_at, 0)
        .ok_or_else(|| error("stored memory version time out of range"))?;
    Ok(Version {
        number: version_number(row.version)?,
        saved: Day::of(saved),
        text: Text::of(&row.content),
    })
}

/// A version row with the file it is a version of.
fn stored_file(row: memory_version::Model) -> Result<(VersionedFile, Version), DbErr> {
    let path = MemoryPath::parse(&row.path)
        .map_err(conversion_error("stored memory path", "MemoryPath"))?;
    let file = VersionedFile::try_from(path)
        .map_err(|_| error("a stored memory version names a log day"))?;
    Ok((file, stored_version(row)?))
}

fn stored_note(row: memory_note::Model) -> Result<Note, DbErr> {
    Ok(Note {
        id: NoteId::from(row.note),
        number: NoteNumber::new(
            u64::try_from(row.position).map_err(conversion_error("stored note position", "u64"))?,
        ),
        time: DateTime::from_timestamp(row.saved_at, 0)
            .ok_or_else(|| error("stored note time out of range"))?,
        physician: row.physician,
        chat: row.chat,
        message: row.message,
        text: row.text,
        physician_words: row
            .physician_words
            .as_deref()
            .map(OneLine::parse)
            .transpose()
            .map_err(conversion_error("stored physician words", "OneLine"))?,
    })
}

fn stored_mark(row: memory_rounds::Model) -> Result<RoundsMark, DbErr> {
    let time = |seconds| {
        DateTime::from_timestamp(seconds, 0)
            .ok_or_else(|| error("stored rounds mark time out of range"))
    };
    Ok(RoundsMark {
        started: time(row.started_at)?,
        saved: time(row.saved_at)?,
    })
}

fn patient_marks(key: &PatientKey) -> sea_orm::Select<memory_rounds::Entity> {
    use memory_rounds::Column as R;
    memory_rounds::Entity::find()
        .filter(R::Practice.eq(key.practice.as_str()))
        .filter(R::Workspace.eq(key.workspace.as_str()))
}

/// The newest of the marks `marks` selects: the last saved, and of marks
/// saved in one second, the last started.
async fn newest_mark(
    db: &impl ConnectionTrait,
    marks: sea_orm::Select<memory_rounds::Entity>,
) -> Result<Option<RoundsMark>, DbErr> {
    marks
        .order_by_desc(memory_rounds::Column::SavedAt)
        .order_by_desc(memory_rounds::Column::StartedAt)
        .one(db)
        .await?
        .map(stored_mark)
        .transpose()
}

fn patient_versions(key: &PatientKey) -> sea_orm::Select<memory_version::Entity> {
    use memory_version::Column as V;
    memory_version::Entity::find()
        .filter(V::Practice.eq(key.practice.as_str()))
        .filter(V::Workspace.eq(key.workspace.as_str()))
}

fn patient_notes(key: &PatientKey) -> sea_orm::Select<memory_note::Entity> {
    use memory_note::Column as N;
    memory_note::Entity::find()
        .filter(N::Practice.eq(key.practice.as_str()))
        .filter(N::Workspace.eq(key.workspace.as_str()))
}

fn file_versions(key: &PatientKey, path: &MemoryPath) -> sea_orm::Select<memory_version::Entity> {
    patient_versions(key).filter(memory_version::Column::Path.eq(path.to_string()))
}

/// The file's newest row.
async fn latest_row(
    db: &impl ConnectionTrait,
    key: &PatientKey,
    path: &MemoryPath,
) -> Result<Option<memory_version::Model>, DbErr> {
    file_versions(key, path)
        .order_by_desc(memory_version::Column::Version)
        .one(db)
        .await
}

/// The file's newest version saved before the database-clock second
/// `start`, when the file existed then.
async fn existing_before(
    db: &impl ConnectionTrait,
    key: &PatientKey,
    path: &MemoryPath,
    start: i64,
) -> Result<Option<Version>, DbErr> {
    let newest = file_versions(key, path)
        .filter(memory_version::Column::SavedAt.lt(start))
        .order_by_desc(memory_version::Column::Version)
        .one(db)
        .await?
        .map(stored_version)
        .transpose()?;
    Ok(newest.filter(|version| version.text.exists()))
}

/// The version `number` of the file, when it has one.
async fn earlier(
    db: &impl ConnectionTrait,
    key: &PatientKey,
    path: &MemoryPath,
    number: u64,
) -> Result<Option<Version>, DbErr> {
    let Ok(number) = i64::try_from(number) else {
        return Ok(None);
    };
    memory_version::Entity::find_by_id((
        key.practice.clone(),
        key.workspace.clone(),
        path.to_string(),
        number,
    ))
    .one(db)
    .await?
    .map(stored_version)
    .transpose()
}

/// The newest version of every file of the patient's that exists, in file
/// order. A file whose newest version has no text is not listed or
/// searched, and its versions still read by number.
async fn current_files(
    db: &impl ConnectionTrait,
    key: &PatientKey,
) -> Result<Vec<(VersionedFile, Version)>, DbErr> {
    use memory_version::Column as V;
    let mut files = patient_versions(key)
        .distinct_on([V::Path])
        .order_by_asc(V::Path)
        .order_by_desc(V::Version)
        .all(db)
        .await?
        .into_iter()
        .map(stored_file)
        .collect::<Result<Vec<_>, _>>()?;
    files.retain(|(_, version)| version.text.exists());
    files.sort_by(|(a, _), (b, _)| a.cmp(b));
    Ok(files)
}

/// One log day: its notes in note order and the rounds marks whose start
/// falls on it, in start order.
#[derive(Default)]
struct LogDay {
    notes: Vec<Note>,
    marks: Vec<RoundsMark>,
}

impl LogDay {
    /// The day's rendered lines.
    fn lines(&self, day: Day) -> Vec<String> {
        page::log_lines(day, &self.notes, &self.marks)
    }
}

/// The patient's log days. A note's day is the UTC date of its time, and a
/// rounds mark's the UTC date of its start.
async fn log_days(
    db: &impl ConnectionTrait,
    key: &PatientKey,
) -> Result<BTreeMap<Day, LogDay>, DbErr> {
    let mut days: BTreeMap<Day, LogDay> = days_of(db, key, patient_notes(key))
        .await?
        .into_iter()
        .map(|(day, notes)| {
            let log = LogDay {
                notes,
                marks: Vec::new(),
            };
            (day, log)
        })
        .collect();
    let marks = patient_marks(key)
        .order_by_asc(memory_rounds::Column::StartedAt)
        .all(db)
        .await?;
    for row in marks {
        let mark = stored_mark(row)?;
        days.entry(Day::of(mark.started))
            .or_default()
            .marks
            .push(mark);
    }
    Ok(days)
}

async fn withdrawn_note_ids(
    db: &impl ConnectionTrait,
    key: &PatientKey,
) -> Result<BTreeSet<String>, DbErr> {
    use super::follow_ups::memory_withdrawal_entity as mw;
    let rows = mw::Entity::find()
        .filter(mw::Column::Practice.eq(key.practice.as_str()))
        .filter(mw::Column::Workspace.eq(key.workspace.as_str()))
        .select_only()
        .column(mw::Column::MemoryId)
        .into_tuple::<String>()
        .all(db)
        .await?;
    Ok(rows.into_iter().collect())
}

/// The non-withdrawn, non-withheld notes `notes` selects, grouped by log day as [`log_days`] groups them.
async fn days_of(
    db: &impl ConnectionTrait,
    key: &PatientKey,
    notes: sea_orm::Select<memory_note::Entity>,
) -> Result<BTreeMap<Day, Vec<Note>>, DbErr> {
    let withdrawn = withdrawn_note_ids(db, key).await?;
    let withheld =
        super::link_correction::load_workspace_withheld_state_in_tx(db, &key.workspace).await?;
    let rows = notes
        .order_by_asc(memory_note::Column::Position)
        .all(db)
        .await?;
    let mut days: BTreeMap<Day, Vec<Note>> = BTreeMap::new();
    for row in rows {
        if withdrawn.contains(&row.note) {
            continue;
        }
        if !withheld.is_empty() {
            let note_work = row.note.split_once(':').map(|(work, _)| work);
            if withheld.message_ids.contains(&row.message)
                || withheld.candidate_ids.contains(&row.message)
                || withheld.work_ids.contains(&row.chat)
                || withheld.work_ids.contains(&row.note)
                || note_work.is_some_and(|w| withheld.work_ids.contains(w))
            {
                continue;
            }
        }
        let note = stored_note(row)?;
        days.entry(Day::of(note.time)).or_default().push(note);
    }
    Ok(days)
}

async fn saved_note(
    db: &impl ConnectionTrait,
    key: &PatientKey,
    id: &NoteId,
) -> Result<Option<Note>, DbErr> {
    memory_note::Entity::find_by_id((
        key.practice.clone(),
        key.workspace.clone(),
        id.as_str().to_owned(),
    ))
    .one(db)
    .await?
    .map(stored_note)
    .transpose()
}

/// The one way a version is saved: the version after `current`, at the
/// store's clock. The caller holds the patient's memory lock, so no other
/// write can have saved that version.
async fn save_version(
    tx: &DatabaseTransaction,
    key: &PatientKey,
    path: &MemoryPath,
    current: Option<u64>,
    text: &Text,
) -> Result<u64, DbErr> {
    let version = current.map_or(1, |current| current + 1);
    let saved_at = now(tx).await?;
    memory_version::Entity::insert(memory_version::ActiveModel {
        practice: Set(key.practice.clone()),
        workspace: Set(key.workspace.clone()),
        path: Set(path.to_string()),
        version: Set(i64::try_from(version).map_err(conversion_error("memory version", "i64"))?),
        content: Set(text.as_str().to_owned()),
        saved_at: Set(saved_at),
    })
    .exec_without_returning(tx)
    .await?;
    Ok(version)
}

/// What memory holds as of a start, from [`read_at_start`].
struct AtStart {
    summary: Option<Version>,
    direction: Option<Version>,
    /// The newest rounds mark saved before the start.
    last: Option<RoundsMark>,
    /// The notes saved before the start and since `last` started, by log
    /// day.
    days: BTreeMap<Day, Vec<Note>>,
}

impl AtStart {
    /// The memory message for `of`, rendered by `page::at_start`.
    fn message(&self, of: StartOf) -> StartMemory {
        StartMemory(
            page::at_start(
                of,
                self.summary.as_ref(),
                self.direction.as_ref(),
                self.last.as_ref(),
                &self.days,
            )
            .map(|text| Item::Message {
                role: "developer".into(),
                text,
            }),
        )
    }
}

/// What memory holds as of the database-clock second `start`: the newest
/// summary and direction versions saved before it, when those files existed
/// then, the newest rounds mark
/// saved before it, and the notes saved before it, at or after that mark's
/// start when there is one. Clock seconds are whole, so a row saved in the
/// start's own second is left out: the conversation's own first run may
/// have saved it.
async fn read_at_start(
    db: &impl ConnectionTrait,
    key: &PatientKey,
    start: i64,
) -> Result<AtStart, DbErr> {
    use memory_note::Column as N;
    let summary = existing_before(db, key, &MemoryPath::Summary, start).await?;
    let direction = existing_before(db, key, &MemoryPath::Direction, start).await?;
    let last = newest_mark(
        db,
        patient_marks(key).filter(memory_rounds::Column::SavedAt.lt(start)),
    )
    .await?;
    let mut notes = patient_notes(key).filter(N::SavedAt.lt(start));
    if let Some(mark) = &last {
        notes = notes.filter(N::SavedAt.gte(mark.started.timestamp()));
    }
    let days = days_of(db, key, notes).await?;
    Ok(AtStart {
        summary,
        direction,
        last,
        days,
    })
}

/// The two-key advisory lock space for one patient's memory, apart from the
/// single-key lock that serializes migration.
const MEMORY_LOCK: i32 = 804_082;

/// Holds the patient's memory lock until `tx` ends, so notes take their
/// places one at a time, a repeated call finds the first call's note, and
/// each file write applies to the version the last write left. SeaQuery has
/// no advisory-lock expression, so the call is a typed function call with
/// the patient as a bound value.
async fn lock_memory(tx: &DatabaseTransaction, key: &PatientKey) -> Result<(), DbErr> {
    let patient = serde_json::json!([key.practice, key.workspace]).to_string();
    let lock = Query::select()
        .expr(
            Func::cust(Alias::new("pg_advisory_xact_lock"))
                .arg(Expr::val(MEMORY_LOCK))
                .arg(Func::cust(Alias::new("hashtext")).arg(Expr::val(patient))),
        )
        .to_owned();
    tx.execute(&lock).await?;
    Ok(())
}

/// The search rule: a line matches when it holds every term of the query,
/// ignoring case.
fn line_matches(line: &str, terms: &[String]) -> bool {
    let line = line.to_lowercase();
    terms.iter().all(|term| line.contains(term.as_str()))
}

impl Store {
    /// Adds a note to the log of the scope's patient. The practice, patient,
    /// physician and chat come from the authorized scope, the work, call and
    /// message from the run, and the time from the store's clock; the model
    /// supplies only the draft. The draft's quote is kept as the note's
    /// physician words, as given. Repeating the same work and call with the
    /// same text returns the saved note and adds nothing; the same work and
    /// call with other text is an error.
    pub async fn append_note(
        &self,
        scope: &Scope,
        origin: &NoteOrigin,
        draft: &NoteDraft,
    ) -> Result<Note, MemoryError> {
        let key = PatientKey::try_from(scope)?;
        self.append_note_for_key(&key, &scope.physician, &scope.chat, origin, draft)
            .await
    }

    /// Adds a note attributed to a practice-owned job (`PRACTICE_OWNER`,
    /// `job_id`). Physician-authored calls go through [`Self::append_note`].
    pub(crate) async fn append_practice_note(
        &self,
        scope: &super::jobs::PracticeScope,
        origin: &NoteOrigin,
        draft: &NoteDraft,
    ) -> Result<Note, MemoryError> {
        let key = PatientKey::for_practice_job(scope)?;
        self.append_note_for_key(
            &key,
            super::jobs::PRACTICE_OWNER,
            &scope.job_id,
            origin,
            draft,
        )
        .await
    }

    async fn append_note_for_key(
        &self,
        key: &PatientKey,
        physician: &str,
        chat: &str,
        origin: &NoteOrigin,
        draft: &NoteDraft,
    ) -> Result<Note, MemoryError> {
        let blank = |field: &String| field.trim().is_empty();
        if blank(&origin.work) || blank(&origin.call) || blank(&origin.message) {
            return Err(error("empty note origin").into());
        }
        draft.check()?;
        let id = NoteId::of_call(&origin.work, &origin.call);
        let tx = self.db.begin().await?;
        lock_memory(&tx, key).await?;
        super::follow_ups::check_withdrawal_fence_in_tx(
            &tx,
            &key.practice,
            &key.workspace,
            &[],
            Some(&origin.message),
            Some(&origin.work),
        )
        .await?;
        if let Some(saved) = saved_note(&tx, key, &id).await? {
            if saved.text != draft.text {
                return Err(error("memory note identity collision").into());
            }
            tx.commit().await?;
            return Ok(saved);
        }
        let last = patient_notes(key)
            .select_only()
            .column(memory_note::Column::Position)
            .order_by_desc(memory_note::Column::Position)
            .into_tuple::<i64>()
            .one(&tx)
            .await?;
        let saved_at = now(&tx).await?;
        let row = memory_note::ActiveModel {
            practice: Set(key.practice.clone()),
            workspace: Set(key.workspace.clone()),
            note: Set(id.as_str().to_owned()),
            position: Set(last.unwrap_or(0) + 1),
            saved_at: Set(saved_at),
            physician: Set(physician.to_owned()),
            chat: Set(chat.to_owned()),
            message: Set(origin.message.clone()),
            text: Set(draft.text.clone()),
            physician_words: Set(draft.quote.clone().map(String::from)),
        }
        .insert(&tx)
        .await?;
        let note = stored_note(row)?;
        tx.commit().await?;
        Ok(note)
    }

    /// The note a run's `work` added for the patient with the tool call
    /// `call`, when it is saved. It reads only.
    pub async fn saved_note(
        &self,
        key: &PatientKey,
        work: &str,
        call: &str,
    ) -> Result<Option<Note>, DbErr> {
        saved_note(&self.db, key, &NoteId::of_call(work, call)).await
    }

    /// Edits the summary, the direction, the person page or a subject page:
    /// applies `edits` in order to the text of the file's current version,
    /// each edit as given or already left out at the parse, and saves the
    /// next version when any edit applied. An edit the file cannot take, or
    /// that would change nothing, is left out. The outcome names each edit
    /// left out with its reason, and gives the size of the file's text.
    pub async fn edit_memory(
        &self,
        key: &PatientKey,
        file: &VersionedFile,
        edits: &[Result<Edit, EditRefusal>],
    ) -> Result<Written, DbErr> {
        let path = MemoryPath::from(file.clone());
        let tx = self.db.begin().await?;
        lock_memory(&tx, key).await?;
        let current = latest_row(&tx, key, &path)
            .await?
            .map(stored_version)
            .transpose()?;
        let number = current.as_ref().map(|v| v.number);
        let text = current.map_or_else(Text::default, |v| v.text);
        let applied = page::apply(&path, &text, edits);
        let (version, now) = match applied.text {
            Some(edited) => (
                Some(save_version(&tx, key, &path, number, &edited).await?),
                edited,
            ),
            None => (number, text),
        };
        tx.commit().await?;
        Ok(Written {
            version,
            characters: now.characters(),
            not_applied: applied.not_applied,
        })
    }

    /// The patient's files that exist, each at its current version with its
    /// size, and each log day.
    pub async fn list_memory(&self, key: &PatientKey) -> Result<Listing, DbErr> {
        let tx = self.snapshot().await?;
        let files = current_files(&tx, key).await?;
        let days = log_days(&tx, key).await?;
        tx.commit().await?;
        let mut listing = Listing {
            log_days: days.into_keys().collect(),
            ..Listing::default()
        };
        for (file, version) in files {
            let listed = Listed {
                version: version.number,
                characters: version.text.characters(),
            };
            match file {
                VersionedFile::Summary => listing.summary = Some(listed),
                VersionedFile::Direction => listing.direction = Some(listed),
                VersionedFile::Person => listing.person = Some(listed),
                VersionedFile::Page(id) => {
                    listing.pages.insert(id, listed);
                }
            }
        }
        Ok(listing)
    }

    /// Reads a file as markdown: the text of its current version, or of the
    /// version named. With no version named, a file that does not exist,
    /// never made or deleted, is refused; a deleted file's versions still
    /// read by number, the delete's own as no text. A log day reads its
    /// notes and rounds marks in time order and has no versions.
    pub async fn read_memory(
        &self,
        key: &PatientKey,
        path: &MemoryPath,
        version: Option<u64>,
    ) -> Result<FileRead, MemoryError> {
        let tx = self.snapshot().await?;
        if let MemoryPath::Log(day) = path {
            if version.is_some() {
                return Err(MemoryRefusal::LogNotVersioned.into());
            }
            let Some(log) = log_days(&tx, key).await?.remove(day) else {
                return Err(MemoryRefusal::NoFile(path.clone()).into());
            };
            tx.commit().await?;
            return Ok(FileRead::Log {
                markdown: page::markdown(&log.lines(*day)),
            });
        }
        let Some(current) = latest_row(&tx, key, path)
            .await?
            .map(stored_version)
            .transpose()?
        else {
            return Err(MemoryRefusal::NoFile(path.clone()).into());
        };
        let chosen = match version {
            Some(asked) if asked != current.number => {
                let Some(earlier) = earlier(&tx, key, path, asked).await? else {
                    return Err(MemoryRefusal::NoVersion {
                        path: path.clone(),
                        version: asked,
                        current: current.number,
                    }
                    .into());
                };
                earlier
            }
            Some(_) => current,
            None if !current.text.exists() => {
                return Err(MemoryRefusal::NoFile(path.clone()).into());
            }
            None => current,
        };
        tx.commit().await?;
        Ok(FileRead::Version {
            version: chosen.number,
            markdown: chosen.text.as_str().to_owned(),
        })
    }

    /// Reads the physician-facing memory view (`Memory.Code.20`): only
    /// `summary.md` and `direction.md`. Pages, `person.md`, and log files are
    /// Coda's working files and are never returned here.
    pub async fn read_physician_memory(&self, key: &PatientKey) -> Result<PhysicianMemory, DbErr> {
        let tx = self.snapshot().await?;
        let summary = latest_row(&tx, key, &MemoryPath::Summary)
            .await?
            .map(stored_version)
            .transpose()?
            .filter(|v| v.text.exists())
            .map(|v| PhysicianMemoryDocument {
                version: v.number,
                saved: v.saved,
                markdown: v.text.as_str().to_owned(),
            });
        let direction = latest_row(&tx, key, &MemoryPath::Direction)
            .await?
            .map(stored_version)
            .transpose()?
            .filter(|v| v.text.exists())
            .map(|v| PhysicianMemoryDocument {
                version: v.number,
                saved: v.saved,
                markdown: v.text.as_str().to_owned(),
            });
        tx.commit().await?;
        Ok(PhysicianMemory { summary, direction })
    }

    /// Saves a physician's own edit of `summary.md` or `direction.md` as a log
    /// note in their exact words (`Memory.Code.22`). Never overwrites the
    /// versioned file directly; patient rounds fold the note in.
    pub async fn save_physician_memory_edit(
        &self,
        scope: &Scope,
        target: PhysicianMemoryTarget,
        words: &str,
        command_id: &str,
    ) -> Result<Note, MemoryError> {
        let command_id = command_id.trim();
        if command_id.is_empty() {
            return Err(MemoryRefusal::NoWords.into());
        }
        let quote = OneLine::parse(words)?;
        let trimmed = words.trim();
        let target_path = target.path();
        let edit_scope = Scope {
            practice: scope.practice.clone(),
            physician: scope.physician.clone(),
            workspace: scope.workspace.clone(),
            chat: format!("physician-memory-edit:{target_path}"),
        };
        let origin = NoteOrigin {
            work: format!("physician-memory-edit:{command_id}"),
            call: target_path.to_string(),
            message: format!("physician-memory-edit:{command_id}"),
        };
        let draft = NoteDraft {
            text: format!("Physician edit to {target_path}:\n{trimmed}"),
            quote: Some(quote),
        };
        self.append_note(&edit_scope, &origin, &draft).await
    }

    /// The memory a conversation started with, as one developer message:
    /// what [`read_at_start`] reads as of `start` in one snapshot, rendered
    /// by `page::at_start`. The memory tables are insert and select only, so
    /// every request of the conversation reads the same text, and a rounds
    /// mark saved after the start never changes it.
    pub async fn memory_at_start(
        &self,
        key: &PatientKey,
        start: ConversationStart,
    ) -> Result<StartMemory, DbErr> {
        let ConversationStart(start) = start;
        let tx = self.snapshot().await?;
        let read = read_at_start(&tx, key, start).await?;
        tx.commit().await?;
        Ok(read.message(StartOf::Conversation))
    }

    /// What a patient rounds run starts with, as of `start`, read in one
    /// snapshot: the memory message as a conversation's is built, and the
    /// last rounds and the notes since, from the same read.
    pub(super) async fn rounds_start(
        &self,
        key: &PatientKey,
        start: DateTime<Utc>,
    ) -> Result<RoundsStart, DbErr> {
        let tx = self.snapshot().await?;
        let read = read_at_start(&tx, key, start.timestamp()).await?;
        tx.commit().await?;
        Ok(RoundsStart {
            memory: read.message(StartOf::RoundsRun),
            last: read.last,
            new_notes: read
                .days
                .iter()
                .map(|(day, notes)| (*day, notes.len()))
                .collect(),
        })
    }

    /// Saves the rounds mark of a patient rounds run that finished: the run,
    /// the time it was given as its start, and the store's clock now.
    pub(super) async fn save_rounds_mark(
        &self,
        key: &PatientKey,
        run: &str,
        started: DateTime<Utc>,
    ) -> Result<RoundsMark, DbErr> {
        let tx = self.db.begin().await?;
        let saved_at = now(&tx).await?;
        let row = memory_rounds::ActiveModel {
            practice: Set(key.practice.clone()),
            workspace: Set(key.workspace.clone()),
            run: Set(run.to_owned()),
            started_at: Set(started.timestamp()),
            saved_at: Set(saved_at),
        }
        .insert(&tx)
        .await?;
        tx.commit().await?;
        stored_mark(row)
    }

    /// When patient rounds last finished for this patient: the newest rounds
    /// mark, or `None` when no run has finished.
    pub async fn last_rounds(&self, key: &PatientKey) -> Result<Option<RoundsMark>, DbErr> {
        newest_mark(&self.db, patient_marks(key)).await
    }

    /// Searches the current text of every file that exists and every log
    /// day, in file order. Each hit names its file, its line number in the
    /// file and the line.
    pub async fn search_memory(
        &self,
        key: &PatientKey,
        query: &str,
    ) -> Result<Vec<Hit>, MemoryError> {
        let terms: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
        if terms.is_empty() {
            return Err(MemoryRefusal::EmptyQuery.into());
        }
        let tx = self.snapshot().await?;
        let files = current_files(&tx, key).await?;
        let days = log_days(&tx, key).await?;
        tx.commit().await?;
        let rendered = files
            .into_iter()
            .map(|(file, version)| {
                let lines = version.text.as_str().lines().map(str::to_owned).collect();
                (MemoryPath::from(file), lines)
            })
            .chain(
                days.into_iter()
                    .map(|(day, log)| (MemoryPath::Log(day), log.lines(day))),
            );
        let mut hits = Vec::new();
        for (path, lines) in rendered {
            for (index, line) in lines.into_iter().enumerate() {
                if line_matches(&line, &terms) {
                    hits.push(Hit {
                        path: path.clone(),
                        line: index + 1,
                        text: line,
                    });
                }
            }
        }
        Ok(hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_parses_from_its_own_form_and_a_page_id_from_any_spelling() {
        for text in [
            "summary.md",
            "direction.md",
            "person.md",
            "pages/chronic-kidney-disease-3.md",
            "pages/m\u{e9}ni\u{e8}re.md",
            "log/2026-11-13.md",
        ] {
            assert_eq!(MemoryPath::parse(text).unwrap().to_string(), text);
        }
        // A page id is lowercased, each run of other characters is one
        // hyphen, and none is left at either end; the .md may be left off.
        for (text, page) in [
            ("pages/CKD Stage 3.md", "pages/ckd-stage-3.md"),
            ("pages/CKD.md", "pages/ckd.md"),
            ("pages/ckd_1", "pages/ckd-1.md"),
            (
                "pages/ -- Type 2: diabetes -- .md",
                "pages/type-2-diabetes.md",
            ),
            ("pages/a/b.md", "pages/a-b.md"),
            (
                "pages/M\u{c9}NI\u{c8}RE's disease",
                "pages/m\u{e9}ni\u{e8}re-s-disease.md",
            ),
        ] {
            assert_eq!(MemoryPath::parse(text).unwrap().to_string(), page, "{text}");
        }
        for text in [
            "summary",
            "Summary.md",
            "pages/.md",
            "pages/ - .md",
            "pages/",
            "log/2026-02-30.md",
            "log/2026-11.md",
            "log/2026-11-13",
            "notes.md",
            "../summary.md",
        ] {
            assert_eq!(
                MemoryPath::parse(text),
                Err(MemoryRefusal::UnknownPath { path: text.into() }),
                "{text}"
            );
        }
    }
}
