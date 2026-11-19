//! The patient memory tools a chat run and a reply revision are offered: list,
//! read and search the patient's memory, add a note to its log, and edit a
//! file with the [`EditTool`]. Arguments parse once into a `MemoryRead`, a
//! note's draft or the edit tool's own request; past the parse no code reads
//! a wire field. The patient is always the run's, never an argument.
//! [`MemoryTool::execute`] runs a call from its arguments to its outcome for
//! the run's authorized scope, [`saved_outcome`] answers for a note call
//! after a restart, and `guidance!` is what both jobs tell the
//! model about memory. The reads alone are a [`ReadTool`]: they need only the
//! patient's key, so patient rounds, which no physician starts, are offered
//! them and the edit tool, and not the note.
use harness::engine::{Invocation, Tool, ToolOutcome};
use sea_orm::DbErr;
use serde::Deserialize;
use serde_json::{Value, json};

use super::edit::EditTool;
use super::{
    Day, FileRead, Hit, Listed, Listing, MemoryError, MemoryPath, MemoryRefusal, Note, NoteDraft,
    NoteOrigin, OneLine, PatientKey,
};
use crate::runtime::{Scope, Store};

/// The paragraph that tells the model about patient memory. Chat and reply
/// revision instructions both include it. It is a macro so that `concat!`
/// can place it inside their constant instructions.
macro_rules! guidance {
    () => {
        "Patient memory is what Coda keeps about this patient between conversations: a summary, the physician's \
direction, a page for each subject, and a dated log of notes. It holds what was worked out, said or decided, \
so later work starts from it. Add a note when you learn something that later work on this patient should know \
and could not look up in the chart. For example: the physician asks you to remember something, tells you \
something the chart does not hold, corrects you or the record, gives a direction or makes a decision, or you \
work out something that took several records to put together. When a note comes from the physician, quote their \
words. When the summary, the direction or a page is wrong or is missing something that matters now, you can \
change it with edit_patient_memory; patient rounds tidy memory later. Do not log the work you did or the answer \
you gave, and do not copy what a chart record already says. \
Memory can be behind the chart. Current medications, doses and values always come from the chart. \
When the physician and the chart disagree, follow the physician, and state both in memory, each with its date."
    };
}
pub(crate) use guidance;

/// What physician-visible step a memory tool call shows, when it shows one.
///
/// A saved note or a memory edit shows the physician one step; a memory read
/// shows none (`Memory.Code.21`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryStepKind {
    /// A note saved to the patient's memory log (`add_patient_memory_note`).
    SavedNote,
    /// An edit to one of the patient's memory files (`edit_patient_memory`).
    EditedMemory,
}

/// One memory tool.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryTool {
    /// Reads memory.
    Read(ReadTool),
    /// Adds a note to the log.
    AddNote,
    /// Edits a file: the [`EditTool`].
    Edit,
}

/// A memory tool that only reads. It needs only the patient's key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadTool {
    /// Lists the patient's files.
    List,
    /// Reads one file.
    Read,
    /// Searches every file.
    Search,
}

impl ReadTool {
    const ALL: [Self; 3] = [Self::List, Self::Read, Self::Search];

    /// The read tool called `name`, if it is one.
    pub fn named(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tool| tool.name() == name)
    }

    fn name(self) -> &'static str {
        match self {
            Self::List => "list_patient_memory",
            Self::Read => "read_patient_memory",
            Self::Search => "search_patient_memory",
        }
    }

    /// Runs one read of the patient's memory, from its arguments to its
    /// outcome. Arguments the model can correct and reads the memory
    /// refuses come back refused; a database failure stays an error.
    pub async fn execute(
        self,
        store: &Store,
        key: &PatientKey,
        arguments: &Value,
    ) -> Result<ToolOutcome, DbErr> {
        match MemoryRead::parse(self, arguments) {
            Ok(read) => read.run(store, key).await,
            Err(reason) => Ok(ToolOutcome::refused(reason)),
        }
    }

    fn definition(self) -> Tool {
        let (description, properties) = match self {
            Self::List => (
                "List this patient's memory files: the summary, the direction, the person page and each subject page, each with its version and size, and each day of the log.",
                json!({}),
            ),
            Self::Read => (
                "Read one memory file as markdown, by a path the list gives. Give a version to read an earlier version of the file; a log day has no versions.",
                json!({"path":{"type":"string"},"version":{"type":["integer","null"]}}),
            ),
            Self::Search => (
                "Search this patient's memory. A line matches when it holds every word of the query. Each result names its file and line.",
                json!({"query":{"type":"string"}}),
            ),
        };
        Tool {
            name: self.name().into(),
            description: description.into(),
            parameters: object(properties),
        }
    }
}

impl MemoryTool {
    const ALL: [Self; 5] = [
        Self::Read(ReadTool::List),
        Self::Read(ReadTool::Read),
        Self::Read(ReadTool::Search),
        Self::AddNote,
        Self::Edit,
    ];

    /// The memory tool called `name`, if it is one.
    pub fn named(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tool| tool.name() == name)
    }

    fn name(self) -> &'static str {
        match self {
            Self::Read(read) => read.name(),
            Self::AddNote => "add_patient_memory_note",
            Self::Edit => EditTool::NAME,
        }
    }

    /// Whether the tool only reads memory.
    pub fn reads(self) -> bool {
        match self {
            Self::Read(_) => true,
            Self::AddNote | Self::Edit => false,
        }
    }

    /// The single owner of which memory tools show a step to the physician:
    /// saving a note and editing memory each show one step; memory reads show
    /// none (`Memory.Code.21`).
    pub const fn physician_step(self) -> Option<MemoryStepKind> {
        match self {
            Self::Read(_) => None,
            Self::AddNote => Some(MemoryStepKind::SavedNote),
            Self::Edit => Some(MemoryStepKind::EditedMemory),
        }
    }

    /// Validates `arguments` for a step-showing memory write tool. Calls with
    /// malformed arguments that are refused at the boundary did nothing and
    /// show no step.
    pub fn parse_step_request(self, arguments: &Value) -> Option<MemoryStepKind> {
        let kind = self.physician_step()?;
        match self {
            Self::Read(_) => None,
            Self::AddNote => note_draft(arguments).ok().map(|_| kind),
            Self::Edit => EditTool::accepts_arguments(arguments).then_some(kind),
        }
    }

    /// Runs one call of this tool for the scope's patient, from its arguments
    /// to its outcome. The host supplies the authorized scope and `origin`,
    /// which builds the run's note origin and runs only for a note. Arguments
    /// the model can correct and calls the memory refuses come back refused;
    /// a database failure stays an error.
    pub async fn execute(
        self,
        store: &Store,
        scope: &Scope,
        arguments: &Value,
        origin: impl FnOnce() -> Result<NoteOrigin, DbErr>,
    ) -> Result<ToolOutcome, DbErr> {
        self.execute_target(store, NoteTarget::Physician(scope), arguments, origin)
            .await
    }

    /// Runs one call of this tool for a practice-owned background job.
    pub(crate) async fn execute_for_practice(
        self,
        store: &Store,
        scope: &super::super::jobs::PracticeScope,
        arguments: &Value,
        origin: impl FnOnce() -> Result<NoteOrigin, DbErr>,
    ) -> Result<ToolOutcome, DbErr> {
        self.execute_target(store, NoteTarget::Practice(scope), arguments, origin)
            .await
    }

    async fn execute_target(
        self,
        store: &Store,
        target: NoteTarget<'_>,
        arguments: &Value,
        origin: impl FnOnce() -> Result<NoteOrigin, DbErr>,
    ) -> Result<ToolOutcome, DbErr> {
        match self {
            Self::Read(read) => read.execute(store, &target.key()?, arguments).await,
            Self::Edit => EditTool.execute(store, &target.key()?, arguments).await,
            Self::AddNote => match note_draft(arguments) {
                Ok(draft) => add_note(store, target, &origin()?, &draft).await,
                Err(reason) => Ok(ToolOutcome::refused(reason)),
            },
        }
    }

    fn definition(self) -> Tool {
        let (description, properties) = match self {
            Self::Read(read) => return read.definition(),
            Self::Edit => return EditTool.definition(),
            Self::AddNote => (
                "Add a dated note to this patient's memory log. It is saved at once and never changed. In quote, copy words exactly from a physician message this work answers when the note records what they said or directed. The result gives the note's number.",
                json!({
                    "text":{"type":"string"},
                    "quote":{"type":["string","null"]}
                }),
            ),
        };
        Tool {
            name: self.name().into(),
            description: description.into(),
            parameters: object(properties),
        }
    }
}

/// The memory tools, in the order they are offered.
pub fn tools() -> [Tool; 5] {
    MemoryTool::ALL.map(MemoryTool::definition)
}

/// The memory reads alone, in the order they are offered.
pub fn read_tools() -> [Tool; 3] {
    ReadTool::ALL.map(ReadTool::definition)
}

/// An object schema in which every property is required and no other is
/// allowed; null stands for an absent optional, as in the chart tools.
pub(super) fn object(properties: Value) -> Value {
    let required: Vec<&String> = properties
        .as_object()
        .into_iter()
        .flatten()
        .map(|(name, _)| name)
        .collect();
    json!({
        "type":"object","properties":properties,
        "required":required,"additionalProperties":false
    })
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ListWire {}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadWire {
    path: String,
    version: Option<u64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SearchWire {
    query: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NoteWire {
    text: String,
    quote: Option<String>,
}

/// A memory tool call the model can correct. Nothing was read or saved.
#[derive(Debug, thiserror::Error)]
enum MemoryInputError {
    /// The list was given arguments.
    #[error("Supply no arguments ({0}).")]
    List(#[source] serde_json::Error),
    /// A read did not name a path and a version or null.
    #[error("Supply path as a string from the list and version as an integer or null ({0}).")]
    Read(#[source] serde_json::Error),
    /// A search did not give its query.
    #[error("Supply query as a string ({0}).")]
    Search(#[source] serde_json::Error),
    /// A note did not match the offered fields.
    #[error("Supply text as a string and quote as a string or null ({0}).")]
    Note(#[source] serde_json::Error),
    /// Well-formed arguments the memory refuses, such as a path that names
    /// no memory file or a note with no text.
    #[error(transparent)]
    Refused(#[from] MemoryRefusal),
}

/// A memory read: it touches only the patient's key.
#[derive(Clone, Debug, PartialEq, Eq)]
enum MemoryRead {
    /// The patient's files.
    List,
    /// One file, at its current version or the version named.
    File {
        /// The file.
        path: MemoryPath,
        /// The earlier version asked for, if any.
        version: Option<u64>,
    },
    /// Every line that holds each word of the query.
    Search {
        /// The query as given.
        query: String,
    },
}

/// A note's draft from the call's arguments: the one place a note's wire
/// fields are read.
fn note_draft(arguments: &Value) -> Result<NoteDraft, MemoryInputError> {
    let wire: NoteWire =
        serde_json::from_value(arguments.clone()).map_err(MemoryInputError::Note)?;
    let draft = NoteDraft {
        text: wire.text,
        // A quote with no words is no quote.
        quote: wire
            .quote
            .as_deref()
            .and_then(|quote| OneLine::parse(quote).ok()),
    };
    draft.check()?;
    Ok(draft)
}

impl MemoryRead {
    /// The read's typed request. The one place a read tool's wire fields are
    /// read.
    fn parse(tool: ReadTool, arguments: &Value) -> Result<Self, MemoryInputError> {
        let arguments = arguments.clone();
        Ok(match tool {
            ReadTool::List => {
                let ListWire {} =
                    serde_json::from_value(arguments).map_err(MemoryInputError::List)?;
                Self::List
            }
            ReadTool::Read => {
                let wire: ReadWire =
                    serde_json::from_value(arguments).map_err(MemoryInputError::Read)?;
                Self::File {
                    path: MemoryPath::parse(&wire.path)?,
                    version: wire.version,
                }
            }
            ReadTool::Search => {
                let wire: SearchWire =
                    serde_json::from_value(arguments).map_err(MemoryInputError::Search)?;
                Self::Search { query: wire.query }
            }
        })
    }

    /// Runs the read for the patient. A refusal is the model's to correct; a
    /// database failure stays an error.
    async fn run(&self, store: &Store, key: &PatientKey) -> Result<ToolOutcome, DbErr> {
        let result = match self {
            Self::List => Ok(listing(store.list_memory(key).await?)),
            Self::File { path, version } => store
                .read_memory(key, path, *version)
                .await
                .map(|read| file(path, read)),
            Self::Search { query } => store
                .search_memory(key, query)
                .await
                .map(|hits| results(&hits)),
        };
        outcome(result)
    }
}

/// The saved outcome of a memory call that started and has no recorded
/// result, which the harness asks for after a restart instead of running
/// the call again. When `call` adds a note and the note of `work` and the
/// call is saved for the scope's patient, it is the result the tool returned
/// when it saved the note; otherwise `None`. It reads only. An edit has no
/// receipt, so its outcome stays unknown and the model reads the file.
pub async fn saved_outcome(
    store: &Store,
    scope: &Scope,
    work: &str,
    call: &Invocation,
) -> Result<Option<ToolOutcome>, DbErr> {
    if MemoryTool::named(&call.name) != Some(MemoryTool::AddNote) {
        return Ok(None);
    }
    let key = PatientKey::try_from(scope)?;
    saved_outcome_for_key(store, &key, work, call).await
}

/// Shared key-based lookup for [`saved_outcome`] and practice-owned jobs.
pub(crate) async fn saved_outcome_for_key(
    store: &Store,
    key: &PatientKey,
    work: &str,
    call: &Invocation,
) -> Result<Option<ToolOutcome>, DbErr> {
    if MemoryTool::named(&call.name) != Some(MemoryTool::AddNote) {
        return Ok(None);
    }
    match store.saved_note(key, work, &call.id).await? {
        Some(saved) => {
            if let Err(err) = crate::runtime::follow_ups::check_withdrawal_fence_in_tx(
                &store.db,
                &key.practice,
                &key.workspace,
                &[],
                Some(&saved.message),
                Some(work),
            )
            .await
            {
                return match err {
                    crate::runtime::follow_ups::WithdrawalFenceCheckError::Violation(v) => {
                        Ok(Some(ToolOutcome::refused(format!("withdrawn_fence: {v}"))))
                    }
                    crate::runtime::follow_ups::WithdrawalFenceCheckError::Database(db) => Err(db),
                };
            }
            outcome(Ok(note(&saved))).map(Some)
        }
        None => Ok(None),
    }
}

#[derive(Clone, Copy)]
enum NoteTarget<'a> {
    Physician(&'a Scope),
    Practice(&'a super::super::jobs::PracticeScope),
}

impl NoteTarget<'_> {
    fn key(self) -> Result<PatientKey, DbErr> {
        match self {
            Self::Physician(scope) => PatientKey::try_from(scope),
            Self::Practice(scope) => PatientKey::for_practice_job(scope),
        }
    }
}

/// Adds the note to the log of the target's patient, with the run's origin.
async fn add_note(
    store: &Store,
    target: NoteTarget<'_>,
    origin: &NoteOrigin,
    draft: &NoteDraft,
) -> Result<ToolOutcome, DbErr> {
    let saved = match target {
        NoteTarget::Physician(scope) => store.append_note(scope, origin, draft).await,
        NoteTarget::Practice(scope) => store.append_practice_note(scope, origin, draft).await,
    };
    outcome(saved.map(|saved| note(&saved)))
}

pub(super) fn outcome(result: Result<Value, MemoryError>) -> Result<ToolOutcome, DbErr> {
    match result {
        Ok(value) => Ok(ToolOutcome::Returned(value.to_string())),
        Err(MemoryError::Refused(refusal)) => Ok(ToolOutcome::refused(refusal)),
        Err(MemoryError::WithdrawnFence(violation)) => Ok(ToolOutcome::refused(format!(
            "withdrawn_fence: {violation}"
        ))),
        Err(MemoryError::Database(cause)) => Err(cause),
    }
}

fn listing(listing: Listing) -> Value {
    let file = |path: MemoryPath, listed: Listed| json!({"path": path.to_string(), "version": listed.version, "characters": listed.characters});
    let pages: Vec<Value> = listing
        .pages
        .into_iter()
        .map(|(id, listed)| file(MemoryPath::Page(id), listed))
        .collect();
    let log: Vec<String> = listing
        .log_days
        .into_iter()
        .map(|day| MemoryPath::Log(day).to_string())
        .collect();
    json!({
        "summary": listing.summary.map(|listed| file(MemoryPath::Summary, listed)),
        "direction": listing.direction.map(|listed| file(MemoryPath::Direction, listed)),
        "person": listing.person.map(|listed| file(MemoryPath::Person, listed)),
        "pages": pages,
        "log": log,
    })
}

fn file(path: &MemoryPath, read: FileRead) -> Value {
    let (version, markdown) = match read {
        FileRead::Version { version, markdown } => (Some(version), markdown),
        FileRead::Log { markdown } => (None, markdown),
    };
    json!({"path": path.to_string(), "version": version, "markdown": markdown})
}

fn results(hits: &[Hit]) -> Value {
    let results: Vec<Value> = hits
        .iter()
        .map(|hit| json!({"path": hit.path.to_string(), "line": hit.line, "text": hit.text}))
        .collect();
    json!({"results": results})
}

/// A note's result: its number and its log day.
fn note(saved: &Note) -> Value {
    json!({
        "note": saved.number.to_string(),
        "path": MemoryPath::Log(Day::of(saved.time)).to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Why the call's arguments are refused at the parse, if they are.
    fn parse(name: &str, arguments: Value) -> Result<(), String> {
        let parsed = match MemoryTool::named(name).expect("a memory tool") {
            MemoryTool::Read(read) => MemoryRead::parse(read, &arguments).map(drop),
            MemoryTool::AddNote => note_draft(&arguments).map(drop),
            MemoryTool::Edit => panic!("the edit tool parses its own request"),
        };
        parsed.map_err(|error| error.to_string())
    }

    #[test]
    fn chat_and_reply_revisions_are_offered_five_memory_tools_in_order() {
        let names: Vec<String> = tools().into_iter().map(|tool| tool.name).collect();
        assert_eq!(
            names,
            [
                "list_patient_memory",
                "read_patient_memory",
                "search_patient_memory",
                "add_patient_memory_note",
                "edit_patient_memory",
            ]
        );
        // Only the three reads leave memory as it is.
        let reads: Vec<bool> = names
            .iter()
            .map(|name| MemoryTool::named(name).expect("an offered tool").reads())
            .collect();
        assert_eq!(reads, [true, true, true, false, false]);
        // Chat runs the same edit tool patient rounds are offered.
        assert_eq!(tools()[4].function(), super::super::edit::tool().function());
    }

    #[test]
    fn malformed_arguments_are_refused_with_the_fields_to_supply() {
        for (name, arguments, fields) in [
            (
                "list_patient_memory",
                json!({"path": "summary.md"}),
                "Supply no arguments",
            ),
            (
                "read_patient_memory",
                json!({"path": ["summary.md"], "version": null}),
                "Supply path",
            ),
            (
                "read_patient_memory",
                json!({"path": "summary.md", "version": -1}),
                "Supply path",
            ),
            (
                "search_patient_memory",
                json!({"words": "cough"}),
                "Supply query",
            ),
            (
                "add_patient_memory_note",
                json!({"text": ["Asked about cough"], "quote": null}),
                "Supply text",
            ),
            // A note has no other field: an unknown one refuses the call.
            (
                "add_patient_memory_note",
                json!({"text": "Asked about cough", "quote": null, "sources": ["N1"]}),
                "Supply text",
            ),
        ] {
            let refused = parse(name, arguments).unwrap_err();
            assert!(refused.starts_with(fields), "{name}: {refused}");
        }
    }

    #[test]
    fn a_path_the_store_does_not_own_and_a_blank_note_are_refused_at_the_parse() {
        for path in ["notes.md", "pages/--.md", "log/2026-02-30.md"] {
            assert_eq!(
                parse(
                    "read_patient_memory",
                    json!({"path": path, "version": null})
                ),
                Err(format!("no memory file is named {path:?}"))
            );
        }
        assert_eq!(
            parse(
                "add_patient_memory_note",
                json!({"text": " \n", "quote": null})
            ),
            Err("a note needs text".to_owned())
        );
    }
}
