//! The pure half of patient memory: a versioned file's stored text, how
//! edits change it, how a log day reads as markdown, and the memory text a
//! conversation starts with. Nothing here touches the database.
//!
//! The summary, the direction, the person page and each subject page are
//! their markdown text, as the model wrote it. Code reads none of it: no
//! front matter, heading, bullet or link has a meaning here. A log day is
//! generated from its notes and rounds marks on every read.
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Range;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{EditRefusal, MemoryPath, MemoryRefusal, NotApplied, Note, RoundsMark, Version};

/// The digits of `part` as a number, when `part` is exactly `len` ASCII digits.
fn number(part: &str, len: usize) -> Option<u32> {
    if part.len() != len || !part.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    part.parse().ok()
}

/// One calendar day, written `YYYY-MM-DD`. A log file is one day.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(NaiveDate);

impl Day {
    /// The day `text` names, when it is a real `YYYY-MM-DD` date.
    pub(super) fn parse(text: &str) -> Option<Self> {
        let [year, month, day] = text.split('-').collect::<Vec<_>>()[..] else {
            return None;
        };
        let year = i32::try_from(number(year, 4)?).ok()?;
        NaiveDate::from_ymd_opt(year, number(month, 2)?, number(day, 2)?).map(Self)
    }

    /// The UTC day of `time`: a note's log day.
    pub(crate) fn of(time: DateTime<Utc>) -> Self {
        Self(time.date_naive())
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

/// Model words that render on one line, as a note's quote does. Each run of
/// spaces and line breaks is one space, with none at either end, and there
/// is at least one word.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OneLine(String);

impl OneLine {
    /// The one-line form of `text`, or [`MemoryRefusal::NoWords`] when it
    /// has no words.
    pub fn parse(text: &str) -> Result<Self, MemoryRefusal> {
        let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if line.is_empty() {
            return Err(MemoryRefusal::NoWords);
        }
        Ok(Self(line))
    }

    /// The text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<OneLine> for String {
    fn from(line: OneLine) -> Self {
        line.0
    }
}

impl fmt::Display for OneLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Text an edit gives that has at least one word, kept exactly as given.
/// The tool parses each edit's text into it once, so the store's edit never
/// sees a blank `create` or `append` text or a blank `old_text`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Words(String);

impl Words {
    /// `text` as given, or [`MemoryRefusal::NoWords`] when it has no words.
    pub fn parse(text: &str) -> Result<Self, MemoryRefusal> {
        if text.split_whitespace().next().is_none() {
            return Err(MemoryRefusal::NoWords);
        }
        Ok(Self(text.to_owned()))
    }

    /// The text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The markdown text one version of a file holds. Text with no words is
/// the empty text; any other text is kept exactly as written.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Text(String);

impl Text {
    /// The one constructor of stored text: `text` exactly as given, and text
    /// with no words made the empty text.
    pub(super) fn of(text: &str) -> Self {
        if text.split_whitespace().next().is_none() {
            return Self::default();
        }
        Self(text.to_owned())
    }

    /// Whether the file this text is the newest version of exists: it does
    /// when the text is not empty, so when it has words. A file with no
    /// version is the empty text. The one rule: list, search, the start
    /// memory, every read and every edit ask it.
    pub(super) fn exists(&self) -> bool {
        !self.0.is_empty()
    }

    /// The text.
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }

    /// The text's size in characters, as a result reports it.
    pub(super) fn characters(&self) -> usize {
        self.0.chars().count()
    }
}

/// Each place `part` is in `text`, places that overlap included, as the
/// bytes it covers.
fn places(text: &str, part: &str) -> Vec<Range<usize>> {
    text.char_indices()
        .filter(|(at, _)| text[*at..].starts_with(part))
        .map(|(at, _)| at..at + part.len())
        .collect()
}

/// Where `old_text` is in `text`, as the stored bytes it covers: the one
/// place it is exactly as given. No place, or more than one, names none.
/// The one owner of where a replace's text is.
fn place(text: &str, old_text: &Words) -> Result<Range<usize>, EditRefusal> {
    let found = places(text, old_text.as_str());
    match &found[..] {
        [part] => Ok(part.clone()),
        [] => Err(EditRefusal::NotFound),
        many => {
            let mut lines: Vec<usize> = many
                .iter()
                .map(|part| text[..part.start].matches('\n').count() + 1)
                .collect();
            lines.dedup();
            Err(EditRefusal::ManyPlaces { lines })
        }
    }
}

/// One change to a versioned file's text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Edit {
    /// Makes a file that does not exist, with `text`.
    Create {
        /// The file's text.
        text: Words,
    },
    /// Changes the one place `old_text` is in the file to `text`; the rest
    /// of the file stays.
    Replace {
        /// Text from the file, in one place.
        old_text: Words,
        /// What takes its place; empty takes the named text out.
        text: String,
    },
    /// Adds `text` at the end of the file, on a new line, or makes the file
    /// with it when there is none.
    Append {
        /// The text to add.
        text: Words,
    },
    /// Takes the file out of the list. The version it saves has no text.
    Delete,
}

impl Edit {
    /// The file's text after this edit, or why it cannot apply to `file`,
    /// the text of the file at `path`. Only an append to a file with no
    /// final line break adds a character: that line break.
    fn apply_to(&self, path: &MemoryPath, file: &Text) -> Result<Text, EditRefusal> {
        match (self, file.exists()) {
            (Self::Create { .. }, true) => Err(EditRefusal::Exists),
            (Self::Create { text } | Self::Append { text }, false) => Ok(Text::of(text.as_str())),
            (Self::Append { text }, true) => {
                let file = file.as_str();
                let joint = if file.ends_with('\n') { "" } else { "\n" };
                Ok(Text::of(&format!("{file}{joint}{}", text.as_str())))
            }
            (Self::Replace { .. } | Self::Delete, false) => Err(EditRefusal::NoFile(path.clone())),
            (Self::Replace { old_text, text }, true) => {
                let at = place(file.as_str(), old_text)?;
                let mut edited = file.as_str().to_owned();
                edited.replace_range(at, text);
                Ok(Text::of(&edited))
            }
            (Self::Delete, true) => Ok(Text::default()),
        }
    }
}

/// What a list of edits made of a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Applied {
    /// The file's text after the edits that applied; `None` when none did.
    pub(super) text: Option<Text>,
    /// Each edit left out, in edit order.
    pub(super) not_applied: Vec<NotApplied>,
}

/// The one edit engine. Each edit of `edits`, as given or already left out
/// at the parse, applies in order to the text the earlier ones left of
/// `file`, the text of the file at `path`, or is left out with its reason.
/// An edit that would leave the text exactly as it is is left out as one
/// that changes nothing.
pub(super) fn apply(
    path: &MemoryPath,
    file: &Text,
    edits: &[Result<Edit, EditRefusal>],
) -> Applied {
    let mut edited = file.clone();
    let mut not_applied = Vec::new();
    for (edit, given) in edits.iter().enumerate() {
        let outcome = given.clone().and_then(|given| {
            let next = given.apply_to(path, &edited)?;
            if next == edited {
                return Err(EditRefusal::NoChange);
            }
            Ok(next)
        });
        match outcome {
            Ok(next) => edited = next,
            Err(reason) => not_applied.push(NotApplied { edit, reason }),
        }
    }
    Applied {
        text: (not_applied.len() < edits.len()).then_some(edited),
        not_applied,
    }
}

/// A note's identity: the run's work and the tool call that added it. It is
/// the stored row's key, so a repeated call finds its note; the model never
/// sees it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NoteId(String);

impl NoteId {
    /// The note a run's tool call adds.
    pub(super) fn of_call(work: &str, call: &str) -> Self {
        Self(format!("{work}:{call}"))
    }

    /// The id as stored.
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for NoteId {
    /// A note's id as its row stores it.
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl fmt::Display for NoteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A note's number: its place among the patient's notes, from 1. It reads
/// as `N4`, as its log heading gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteNumber(u64);

impl NoteNumber {
    /// The note at `position` among the patient's notes.
    pub(super) fn new(position: u64) -> Self {
        Self(position)
    }
}

impl fmt::Display for NoteNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}", self.0)
    }
}

/// Rendered lines joined as one markdown file. A log day reads through this.
pub(super) fn markdown(lines: &[String]) -> String {
    let mut out = String::new();
    for line in lines {
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// A JSON string, so the physician's words stay one value on one line.
fn quoted(words: &str) -> String {
    Value::String(words.to_owned()).to_string()
}

/// One log day's lines: each note in order with its time, number and
/// physician words, then its text; and each rounds mark, in start order,
/// as one heading before the first note saved at or after its start.
pub(super) fn log_lines(day: Day, notes: &[Note], marks: &[RoundsMark]) -> Vec<String> {
    let mut lines = vec![format!("# Log {day}")];
    let mut marks = marks.iter().peekable();
    for note in notes {
        while let Some(mark) = marks.next_if(|mark| mark.started <= note.time) {
            lines.push(String::new());
            lines.push(mark_line(mark));
        }
        lines.push(String::new());
        lines.extend(note_lines(note));
    }
    for mark in marks {
        lines.push(String::new());
        lines.push(mark_line(mark));
    }
    lines
}

/// A rounds mark's one line in a log day, at the time its run started.
fn mark_line(mark: &RoundsMark) -> String {
    format!(
        "## {} UTC, patient rounds finished a run that started here",
        mark.started.format("%H:%M:%S")
    )
}

/// One note's lines in a log day.
fn note_lines(note: &Note) -> Vec<String> {
    let mut lines = vec![
        format!(
            "## {} UTC, note {}, physician {}",
            note.time.format("%H:%M:%S"),
            note.number,
            note.physician
        ),
        String::new(),
    ];
    if let Some(words) = &note.physician_words {
        lines.push(format!("Quoted: {}", quoted(words.as_str())));
        lines.push(String::new());
    }
    lines.extend(note.text.lines().map(str::to_owned));
    lines
}

/// The most of the summary a conversation starts with, in characters. A
/// first value for local real-model tests.
pub const START_SUMMARY_CHARACTERS: usize = 16_000;

/// The most of the log a conversation starts with, in characters of whole
/// notes. A first value for local real-model tests.
pub const START_NOTES_CHARACTERS: usize = 8_000;

/// What a memory message is read for. Only its first sentence differs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StartOf {
    /// A conversation: a chat, or a reply and its revisions.
    Conversation,
}

/// The patient memory a conversation or a rounds run starts with, each file
/// under its path, version and saved day: the summary up to
/// [`START_SUMMARY_CHARACTERS`], cut at a line end; the whole direction; and
/// the newest whole notes of `days` up to [`START_NOTES_CHARACTERS`], newest
/// first, under a heading that says whether they are the notes saved since
/// the last finished rounds run started (`last`) or every note. What does not
/// fit is named by its path. `None` when there is no summary, no direction
/// and no note.
pub(super) fn at_start(
    of: StartOf,
    summary: Option<&Version>,
    direction: Option<&Version>,
    last: Option<&RoundsMark>,
    days: &BTreeMap<Day, Vec<Note>>,
) -> Option<String> {
    if summary.is_none() && direction.is_none() && days.is_empty() {
        return None;
    }
    let started = match of {
        StartOf::Conversation => "This patient's memory as this conversation started.",
    };
    let mut parts = vec![format!(
        "{started} list_patient_memory lists every file, and read_patient_memory reads one."
    )];
    if let Some(summary) = summary {
        parts.push(summary_at_start(summary));
    }
    if let Some(direction) = direction {
        parts.push(format!(
            "{}\n{}",
            start_heading(&MemoryPath::Direction, direction),
            direction.text.as_str()
        ));
    }
    if !days.is_empty() {
        parts.push(notes_at_start(last, days));
    }
    let parts: Vec<&str> = parts
        .iter()
        .map(|part| part.trim_end_matches('\n'))
        .collect();
    Some(parts.join("\n\n"))
}

/// The line a file starts under in the start memory: its path, its version
/// and the day that version was saved.
fn start_heading(path: &MemoryPath, version: &Version) -> String {
    format!(
        "{path}, version {}, saved {}:",
        version.number, version.saved
    )
}

/// The summary under its heading: whole when it fits, otherwise its first
/// lines that fit and a line that says it continues.
fn summary_at_start(summary: &Version) -> String {
    let heading = start_heading(&MemoryPath::Summary, summary);
    let text = summary.text.as_str();
    if summary.text.characters() <= START_SUMMARY_CHARACTERS {
        return format!("{heading}\n{text}");
    }
    let mut kept = String::new();
    let mut size = 0;
    for line in text.split_inclusive('\n') {
        size += line.chars().count();
        if size > START_SUMMARY_CHARACTERS {
            break;
        }
        kept.push_str(line);
    }
    format!(
        "{heading}\n{kept}{} continues; read_patient_memory reads it whole.",
        MemoryPath::Summary
    )
}

/// The newest whole notes that fit, newest first, under their log days, and
/// a line that counts the rest and names their days. The heading says they
/// are the notes saved since the last finished rounds run started when a run
/// has finished, `last`.
fn notes_at_start(last: Option<&RoundsMark>, days: &BTreeMap<Day, Vec<Note>>) -> String {
    let newest = days
        .iter()
        .rev()
        .flat_map(|(day, notes)| notes.iter().rev().map(move |note| (*day, note)));
    let mut shown: Vec<(Day, Vec<Note>)> = Vec::new();
    // Once a note does not fit, it and every older note are left out.
    let mut more = 0;
    let mut more_days: Vec<Day> = Vec::new();
    let mut size = 0;
    for (day, note) in newest {
        if more == 0 {
            size += markdown(&note_lines(note)).chars().count();
        }
        if more == 0 && size <= START_NOTES_CHARACTERS {
            match shown.last_mut() {
                Some((shown_day, notes)) if *shown_day == day => notes.push(note.clone()),
                Some(_) | None => shown.push((day, vec![note.clone()])),
            }
            continue;
        }
        more += 1;
        if more_days.last() != Some(&day) {
            more_days.push(day);
        }
    }
    let heading = match last {
        Some(_) => "Notes saved since the last finished patient rounds started, newest first:",
        None => "Notes saved for this patient, newest first:",
    };
    let mut lines = vec![String::from(heading)];
    for (day, notes) in &shown {
        lines.push(String::new());
        lines.extend(log_lines(*day, notes, &[]));
    }
    if more > 0 {
        // Oldest day first, as the list gives the log.
        let paths: Vec<String> = more_days
            .iter()
            .rev()
            .map(|day| MemoryPath::Log(*day).to_string())
            .collect();
        let count = if more == 1 {
            String::from("1 more note is")
        } else {
            format!("{more} more notes are")
        };
        lines.push(String::new());
        lines.push(format!("{count} in {}.", paths.join(", ")));
    }
    markdown(&lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::memory::EditInputError;

    fn words(text: &str) -> Words {
        Words::parse(text).unwrap()
    }

    fn create(text: &str) -> Edit {
        Edit::Create { text: words(text) }
    }

    fn append(text: &str) -> Edit {
        Edit::Append { text: words(text) }
    }

    fn replace(old_text: &str, text: &str) -> Edit {
        Edit::Replace {
            old_text: words(old_text),
            text: text.into(),
        }
    }

    fn left_out(edit: usize, reason: EditRefusal) -> NotApplied {
        NotApplied { edit, reason }
    }

    fn path() -> MemoryPath {
        MemoryPath::Summary
    }

    /// What `edits`, each as given, made of the file whose text is `file`.
    fn applied(file: &str, edits: &[Edit]) -> Applied {
        let given: Vec<_> = edits.iter().cloned().map(Ok).collect();
        apply(&path(), &Text::of(file), &given)
    }

    /// The text `edits` leave of `file`, when at least one applied.
    fn edited(file: &str, edits: &[Edit]) -> String {
        let applied = applied(file, edits);
        assert_eq!(applied.not_applied, [], "every edit applies");
        applied.text.expect("an edit applied").as_str().to_owned()
    }

    /// Why each edit of `edits` was left out of `file`, as the model reads it.
    fn reasons(file: &str, edits: &[Edit]) -> Vec<(usize, String)> {
        applied(file, edits)
            .not_applied
            .into_iter()
            .map(|left_out| (left_out.edit, left_out.reason.to_string()))
            .collect()
    }

    #[test]
    fn stored_text_is_kept_exactly_as_written() {
        for (given, stored) in [
            ("Has CKD", "Has CKD"),
            ("Has CKD\n", "Has CKD\n"),
            ("Has CKD\n\n\r\n", "Has CKD\n\n\r\n"),
            ("  Has CKD", "  Has CKD"),
            (
                "---\nname: CKD\n---\n\n- a  b \n",
                "---\nname: CKD\n---\n\n- a  b \n",
            ),
            ("", ""),
            ("\n\n", ""),
            ("  \n", ""),
            (" \n\t\r\n", ""),
        ] {
            assert_eq!(Text::of(given).as_str(), stored, "{given:?}");
        }
        // A text with no words is the empty text and no file; any word
        // makes one.
        assert_eq!(Text::of(" \n\t"), Text::default());
        assert!(!Text::default().exists());
        assert!(!Text::of(" \n\t").exists());
        assert!(Text::of("-").exists());
    }

    #[test]
    fn each_edit_applies_to_a_file_that_exists_or_one_that_does_not() {
        let file = "# CKD\n\n- Stage 3b [N1]\n";
        // Create makes a file that does not exist, and is left out of one
        // that does.
        assert_eq!(edited("", &[create("# CKD")]), "# CKD");
        assert_eq!(
            reasons(file, &[create("# Other")]),
            [(
                0,
                "the file exists; change it with replace or append".into()
            )]
        );
        // Append adds on a new line, or makes the file.
        assert_eq!(
            edited(file, &[append("- eGFR 38 [chart]")]),
            "# CKD\n\n- Stage 3b [N1]\n- eGFR 38 [chart]"
        );
        assert_eq!(edited("", &[append("- eGFR 38")]), "- eGFR 38");
        // Replace changes the one place and keeps the rest.
        assert_eq!(
            edited(file, &[replace("3b", "4")]),
            "# CKD\n\n- Stage 4 [N1]\n"
        );
        // Delete leaves no text; the file no longer exists.
        let deleted = applied(file, &[Edit::Delete]).text.unwrap();
        assert_eq!(deleted, Text::default());
        assert!(!deleted.exists());
        // Replace and delete of no file are left out, and create makes it.
        for edit in [replace("Stage", "Phase"), Edit::Delete] {
            assert_eq!(
                reasons("", std::slice::from_ref(&edit)),
                [(0, "there is no file summary.md; create makes it".into())],
                "{edit:?}"
            );
        }
        assert_eq!(applied("", &[]).text, None);
    }

    #[test]
    fn a_middle_edit_left_out_leaves_the_others_applied_in_order() {
        let input = EditRefusal::Input(EditInputError::Shape("unknown variant `move`".into()));
        let given = [
            Ok(append("- eGFR 38")),
            Ok(replace("eGFR 41", "eGFR 44")),
            Err(input.clone()),
            // Each edit sees the text the earlier ones left.
            Ok(replace("eGFR 38", "eGFR 39")),
            Ok(Edit::Delete),
            Ok(create("# New")),
            Ok(create("# Again")),
        ];
        let applied = apply(&path(), &Text::of("# CKD"), &given);
        assert_eq!(
            applied.not_applied,
            [
                left_out(1, EditRefusal::NotFound),
                left_out(2, input),
                left_out(6, EditRefusal::Exists),
            ]
        );
        assert_eq!(applied.text, Some(Text::of("# New")));
        // When none applies, no text is made.
        let none = apply(&path(), &Text::of("# CKD"), &[Ok(create("# CKD"))]);
        assert_eq!(none.text, None);
    }

    #[test]
    fn each_reason_reads_plainly_and_names_no_text() {
        let file = "- eGFR 38 in May\n- eGFR 41 in June\n- Creatinine 1000\n";
        assert_eq!(
            reasons(
                file,
                &[
                    replace("potassium", "K"),
                    replace("eGFR", "GFR"),
                    // Places that overlap count too, and two on one line name
                    // that line once.
                    replace("00", "0"),
                    replace("eGFR 38", "eGFR 38"),
                ]
            ),
            [
                (0, "old_text is not in the file exactly as given".into()),
                (
                    1,
                    "old_text is in the file more than once, on lines 1, 2; give more of the text so it names one place".into()
                ),
                (
                    2,
                    "old_text is in the file more than once, on line 3; give more of the text so it names one place".into()
                ),
                (3, "nothing to change: the file already reads this".into()),
            ]
        );
        assert_eq!(
            reasons(file, &[replace("in", "at")])[0].1,
            "old_text is in the file more than once, on lines 1, 2, 3; give more of the text so it names one place"
        );
    }

    #[test]
    fn a_replace_may_span_line_breaks_and_an_empty_text_takes_the_named_text_out() {
        let file = "# Plan\n\n- Recheck potassium\n- Stop lisinopril\n\n## Done\n";
        assert_eq!(
            edited(
                file,
                &[replace(
                    "potassium\n- Stop lisinopril\n",
                    "potassium in May\n"
                )]
            ),
            "# Plan\n\n- Recheck potassium in May\n\n## Done\n"
        );
        assert_eq!(
            edited(file, &[replace("- Stop lisinopril\n", "")]),
            "# Plan\n\n- Recheck potassium\n\n## Done\n"
        );
        // Taking out every word leaves no file, with no characters.
        let gone = applied("  Only words \n", &[replace("Only words", "")])
            .text
            .unwrap();
        assert!(!gone.exists());
        assert_eq!(gone.characters(), 0);
    }

    #[test]
    fn a_replace_names_its_text_exactly_or_is_left_out() {
        // "in 1 " is not in "in 10": no space is dropped to find it.
        let file = "Recheck in 10 days.";
        assert_eq!(
            reasons(file, &[replace("in 1 ", "in 2 ")]),
            [(0, "old_text is not in the file exactly as given".into())]
        );
        assert_eq!(applied(file, &[replace("in 1 ", "in 2 ")]).text, None);
    }

    #[test]
    fn a_replace_removes_only_the_named_text_and_an_append_starts_on_a_new_line() {
        assert_eq!(
            [
                // The blank line before the named text stays.
                edited("Header\n\nTail\n", &[replace("Tail\n", "")]),
                // A file with no final line break takes one before the
                // appended text.
                edited("- a", &[append("- b")]),
                // A file that ends with a line break takes no blank line.
                edited("- a\n", &[append("- b")]),
            ],
            ["Header\n\n", "- a\n- b", "- a\n- b"]
        );
    }
}
