//! Citations: how a chat answer names the sources behind it.
//!
//! The read tool gives each version a label (`S3`). The model ends its work
//! with `finish_answer`: the answer with labels beside its claims, optional
//! quotes for those labels, and the Basis. A label in square brackets that a
//! read of this work gave is the citation. A quote found in the text
//! delivered for that version gives the citation its place; a quote that is
//! missing, empty or not found gives none and is never refused. This module
//! checks that finish and turns it into the saved shape: text without labels,
//! and marks that name a version and, when a quote was found, a place. The
//! label in square brackets is the only thing read from the model's text,
//! and none of the model's words are saved in a mark. A description the
//! model hands in for a cited version is saved with the finish as its own
//! words; any other description is dropped and never refuses a finish.
//! Spec: `verification/physician/citations/README.md`.

use harness::engine::results::ResultSource;
use harness::engine::{Cause, Event, EventKind, Item, ProductMessage, Tool, Turn, recipe::Refusal};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::chart::{Label, Labels, ReadReceipt, RecordReceipt};

/// The tool that ends a chat answer.
pub const FINISH_TOOL: &str = "finish_answer";

/// What the read tool's description adds about labels.
const READ_GUIDANCE: &str = "Each record has cite_as, its label for citing. If a record's revision has changed, search again for its current reference.";

/// At most this many finishes of one work are refused for their labels: no
/// label while `cites_none` is false, or a label no read of the work gave.
/// The next one is saved with such labels dropped.
const REFUSALS_BEFORE_DROPPING: usize = 2;

// ------------------------------------------------------------------ saved shape

/// One version of one source, as the source store names it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceVersion {
    /// The source's candidate id.
    pub candidate_id: String,
    /// The revision Coda read.
    pub revision: String,
}

/// Where in a version a citation points.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Place {
    /// Every place the quoted words occur in the delivered text, in part and
    /// byte order.
    Passage {
        /// One or more spans.
        spans: Spans,
    },
}

/// Bytes of one part: part 0 is the record's own content, parts 1 onward are
/// its readable parts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// The part.
    pub part: usize,
    /// First byte.
    pub start: usize,
    /// The byte after the last.
    pub end: usize,
}

/// One or more spans; never none.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Vec<Span>")]
pub struct Spans(Vec<Span>);

impl Spans {
    /// The spans, in part and byte order.
    pub fn spans(&self) -> &[Span] {
        &self.0
    }
}

impl TryFrom<Vec<Span>> for Spans {
    type Error = &'static str;

    fn try_from(spans: Vec<Span>) -> Result<Self, Self::Error> {
        if spans.is_empty() {
            Err("a quoted place has at least one span")
        } else {
            Ok(Self(spans))
        }
    }
}

/// A numbered mark beside a claim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mark {
    /// Byte offset in the text where the mark sits.
    pub at: usize,
    /// The version cited.
    pub source: SourceVersion,
    /// The place in it; `None` cites the version as a whole.
    pub place: Option<Place>,
}

/// Text with its marks. The text holds no labels; each mark sits at a char
/// boundary inside it. Only a resolved finish builds one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitedText {
    text: String,
    marks: Vec<Mark>,
}

impl CitedText {
    /// The text the physician reads.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The marks, in text order.
    pub fn marks(&self) -> &[Mark] {
        &self.marks
    }
}

/// Coda's description of one cited version, in Coda's own words, as the
/// model handed it in. Its words are never read for meaning.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Description {
    /// The version described.
    pub source: SourceVersion,
    /// The model's words.
    pub text: String,
}

/// A finished answer's citations, Basis and descriptions. The answer's text
/// is the saved message's text; its marks sit in that text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finished {
    marks: Vec<Mark>,
    basis: CitedText,
    descriptions: Vec<Description>,
}

impl Finished {
    /// The answer's marks, in text order.
    pub fn marks(&self) -> &[Mark] {
        &self.marks
    }

    /// The Basis and limitations.
    pub fn basis(&self) -> &CitedText {
        &self.basis
    }

    /// The descriptions kept: at most one per version, each of a version a
    /// mark of the answer or the Basis cites.
    pub fn descriptions(&self) -> &[Description] {
        &self.descriptions
    }
}

// ------------------------------------------------------------------ the read

/// The read tool with what it says about labels.
pub fn labelled(mut read: Tool) -> Tool {
    read.description = format!("{} {READ_GUIDANCE}", read.description);
    read
}

// ------------------------------------------------------------------ the finish tool

/// The chat's finish tool.
pub fn finish_tool() -> Tool {
    let quote = json!({
        "type": "object",
        "properties": {
            "label": {"type": "string"},
            "quote": {"type": "string"}
        },
        "required": ["label", "quote"],
        "additionalProperties": false
    });
    let description = json!({
        "type": "object",
        "properties": {
            "label": {"type": "string"},
            "description": {"type": "string"}
        },
        "required": ["label", "description"],
        "additionalProperties": false
    });
    Tool {
        name: FINISH_TOOL.into(),
        description: "Hand in your finished answer. Nothing reaches the physician until it is accepted.\n\
answer: the complete answer. Beside each claim that rests on a source you read, write that source's cite_as label in square brackets, such as [S3]. The label is the citation.\n\
quotes: for each label you wrote in the answer, the words in that source's text that support the claim, copied exactly. The physician then sees that passage marked in the record; without a quote they see the record with nothing marked. Give them in the order the labels are written.\n\
descriptions: leave this empty unless a source you cite came to you with no text or fields of its own that a physician could be shown. For such a source, write two or three lines on what it says, in your own words: values with their units, dates, status, and nothing the source does not say. The physician sees your words marked as prepared by Coda.\n\
cites_none: true only when the answer rests on no source you read.\n\
basis: what the answer rests on (source records, physician input, skill guidance, general medical knowledge) and any limitation that could materially change interpretation, usually under 80 words. It may carry labels too.\n\
An answer and basis with no label are refused unless cites_none is true, and so is a label that is not from your reads. You are told which, nothing is saved, and you call finish_answer again.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "answer": {"type": "string"},
                "quotes": {"type": "array", "items": quote},
                "descriptions": {"type": "array", "items": description},
                "basis": {"type": "string"},
                "cites_none": {"type": "boolean"}
            },
            "required": ["answer", "quotes", "descriptions", "basis", "cites_none"],
            "additionalProperties": false
        }),
    }
}

/// Whether a turn asks to finish. A finish beside other calls also routes to
/// the finish, which refuses it whole without running any call.
pub fn requests_finish(turn: &Turn) -> bool {
    turn.calls().any(|call| call.name == FINISH_TOOL)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FinishCall {
    answer: String,
    quotes: Vec<Quote>,
    descriptions: Vec<Described>,
    basis: String,
    cites_none: bool,
}

/// Words the model copied for one label of the answer.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Quote {
    label: Label,
    quote: String,
}

/// The model's own words on what one label's source says.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Described {
    label: Label,
    description: String,
}

/// A finish turn with its one call's id and arguments, or why it is not one.
/// An answer or Basis is empty when its text without labels is blank. Such a
/// refusal never counts toward the two label refusals.
fn submitted(turn: &Turn) -> Result<(&str, FinishCall), String> {
    let mut calls = turn.calls();
    let call = match (calls.next(), calls.next()) {
        (Some(call), None) if call.name == FINISH_TOOL => call,
        (None, _) => {
            return Err(
                "Nothing reaches the physician until you call finish_answer. Call it with your complete answer. If you cannot answer, say why in answer and set cites_none to true.".into(),
            );
        }
        (Some(_), _) => {
            return Err(
                "Call finish_answer by itself, after the other calls have returned. Nothing was run and nothing was saved.".into(),
            );
        }
    };
    let finish: FinishCall = serde_json::from_str(&call.arguments).map_err(|error| {
        format!("finish_answer arguments do not match its parameters ({error}). Nothing was saved.")
    })?;
    if without_labels(&finish.answer).trim().is_empty() {
        return Err("Give the complete answer in answer. Nothing was saved.".into());
    }
    if without_labels(&finish.basis).trim().is_empty() {
        return Err(
            "Give the basis: what the answer rests on and its material limits. Nothing was saved."
                .into(),
        );
    }
    Ok((&call.id, finish))
}

// ------------------------------------------------------------------ delivered

/// What the model is working from now: the delivered text of each label's
/// version. Which version a label names is [`Labels`]'s to say.
#[derive(Default)]
struct Delivered(Vec<(Label, DeliveredVersion)>);

/// The delivered text of one version.
struct DeliveredVersion {
    /// Delivered stretches: part, first byte, text.
    stretches: Vec<(usize, usize, String)>,
}

impl Delivered {
    fn of_work(records: &[(Event, bool)], messages: &[ProductMessage]) -> Result<Self, Cause> {
        let mut delivered = Self::default();
        for output in harness::engine::results::delivered_originals(records, messages)? {
            // Search pages and other results are not read receipts.
            let Ok(receipt) = serde_json::from_str::<ReadReceipt>(output) else {
                continue;
            };
            for result in receipt.results {
                delivered.add(result);
            }
        }
        for (_, version) in &mut delivered.0 {
            version.merge();
        }
        Ok(delivered)
    }

    /// A receipt's record or passage, under the label it carries. A receipt
    /// without a label is not one the model can cite.
    fn add(&mut self, result: RecordReceipt) {
        let (label, stretches) = match result {
            RecordReceipt::Available {
                cite_as: Some(label),
                content,
                readable_parts,
                ..
            } => {
                let mut stretches = vec![(0, 0, content)];
                stretches.extend(
                    readable_parts
                        .into_iter()
                        .enumerate()
                        .map(|(index, part)| (index + 1, 0, part.text)),
                );
                (label, stretches)
            }
            RecordReceipt::Passage {
                cite_as: Some(label),
                part,
                start_byte,
                content,
                ..
            } => (label, vec![(part, start_byte, content)]),
            RecordReceipt::Available { cite_as: None, .. }
            | RecordReceipt::Passage { cite_as: None, .. }
            | RecordReceipt::Unavailable { .. }
            | RecordReceipt::OverBudget { .. } => return,
        };
        match self.0.iter_mut().find(|(known, _)| known == &label) {
            Some((_, version)) => version.stretches.extend(stretches),
            None => self.0.push((label, DeliveredVersion { stretches })),
        }
    }

    fn get(&self, label: &Label) -> Option<&DeliveredVersion> {
        self.0
            .iter()
            .find(|(known, _)| known == label)
            .map(|(_, version)| version)
    }
}

impl DeliveredVersion {
    /// Join stretches of one part that touch or overlap, so the same bytes
    /// delivered twice count once. Every stretch is a slice of the same text.
    fn merge(&mut self) {
        self.stretches
            .sort_by_key(|(part, start, _)| (*part, *start));
        let mut merged: Vec<(usize, usize, String)> = Vec::new();
        for (part, start, text) in std::mem::take(&mut self.stretches) {
            if let Some((last_part, last_start, last)) = merged.last_mut()
                && *last_part == part
                && start <= *last_start + last.len()
            {
                let end = *last_start + last.len();
                if start + text.len() > end {
                    last.push_str(&text[end - start..]);
                }
                continue;
            }
            merged.push((part, start, text));
        }
        self.stretches = merged;
    }
}

/// Where a quote lies in the delivered text of one version: every place its
/// words occur, in part and byte order, without overlap. Matching compares
/// the [`significant`] chars of both sides; case and letters are exact. A
/// span runs from the first byte of the first matched char to the last byte
/// of the last one, so chars skipped inside a match lie inside its span and
/// chars skipped around it do not. `None` when the quote is empty or its
/// words occur nowhere.
fn locate(version: &DeliveredVersion, quote: &str) -> Option<Place> {
    let wanted: Vec<char> = significant(quote).into_iter().map(|(c, _, _)| c).collect();
    if wanted.is_empty() {
        return None;
    }
    let mut spans = Vec::new();
    for (part, offset, text) in &version.stretches {
        let chars = significant(text);
        let mut from = 0;
        while let Some(at) = chars[from..]
            .windows(wanted.len())
            .position(|window| window.iter().map(|(c, _, _)| *c).eq(wanted.iter().copied()))
            .map(|at| from + at)
        {
            spans.push(Span {
                part: *part,
                start: offset + chars[at].1,
                end: offset + chars[at + wanted.len() - 1].2,
            });
            from = at + wanted.len();
        }
    }
    Spans::try_from(spans)
        .ok()
        .map(|spans| Place::Passage { spans })
}

/// The text's chars that a quote must match: each char after [`folded`],
/// with the byte span it came from. Chars `folded` skips are left out.
fn significant(text: &str) -> Vec<(char, usize, usize)> {
    text.char_indices()
        .filter_map(|(index, c)| Some((folded(c)?, index, index + c.len_utf8())))
        .collect()
}

/// The only differences a quote may have from the source's text: a char with
/// no visible mark is skipped (`None`), and quotation marks and dashes fold
/// to one form. Everything else, case included, must match exactly.
fn folded(c: char) -> Option<char> {
    Some(match c {
        // Whitespace and line breaks: tab, line feed, vertical tab, form
        // feed, carriage return, space, next line, no-break space, ogham
        // space mark, en quad to hair space, line separator, paragraph
        // separator, narrow no-break space, medium mathematical space,
        // ideographic space.
        '\t'
        | '\n'
        | '\u{0B}'
        | '\u{0C}'
        | '\r'
        | ' '
        | '\u{85}'
        | '\u{A0}'
        | '\u{1680}'
        | '\u{2000}'..='\u{200A}'
        | '\u{2028}'
        | '\u{2029}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}'
        // No visible form: soft hyphen, zero-width space, non-joiner and
        // joiner, word joiner, zero-width no-break space, and the
        // left-to-right, right-to-left and Arabic letter marks.
        | '\u{AD}'
        | '\u{200B}'..='\u{200D}'
        | '\u{2060}'
        | '\u{FEFF}'
        | '\u{200E}'
        | '\u{200F}'
        | '\u{61C}' => return None,
        // Left and right single quotation marks.
        '\u{2018}' | '\u{2019}' => '\'',
        // Left and right double quotation marks.
        '\u{201C}' | '\u{201D}' => '"',
        // Hyphen, non-breaking hyphen, en dash, em dash, minus sign.
        '\u{2010}' | '\u{2011}' | '\u{2013}' | '\u{2014}' | '\u{2212}' => '-',
        c => c,
    })
}

// ------------------------------------------------------------------ deciding

/// Which quotes of one label each of its marks in the answer takes, from the
/// number of its marks and the number of its quotes: for each mark, in text
/// order, the indexes of the quotes it takes among the label's quotes.
/// - as many marks as quotes: each mark takes its own quote, in order;
/// - one mark: it takes every quote;
/// - one quote: every mark takes it;
/// - no quote: no mark takes one;
/// - any other count: every mark takes every quote.
fn pairing(marks: usize, quotes: usize) -> Vec<std::ops::Range<usize>> {
    if marks == quotes {
        (0..marks).map(|nth| nth..nth + 1).collect()
    } else {
        vec![0..quotes; marks]
    }
}

/// Which text of the finish a label sits in.
#[derive(Clone, Copy)]
enum Text {
    Answer,
    Basis,
}

impl Text {
    fn name(self) -> &'static str {
        match self {
            Self::Answer => "answer",
            Self::Basis => "basis",
        }
    }
}

/// A label no read of this work gave, as the model is told.
struct UnknownLabel {
    text: Text,
    label: Label,
}

impl std::fmt::Display for UnknownLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} is not a cite_as label from your reads.",
            self.text.name(),
            self.label.0
        )
    }
}

/// What the model is told when neither the answer nor the Basis has a label
/// and `cites_none` is false.
const NO_LABEL: &str = "Neither the answer nor the basis has a label. Cite the sources they rest on, or set cites_none to true if they rest on no source you read.";

/// What the product does with a finish.
pub enum Decision {
    /// Nothing is saved; the model is told why.
    Refuse(Refusal),
    /// Save the answer text with its citations, Basis and descriptions.
    Save {
        /// The answer the physician reads, without labels.
        text: String,
        /// Its marks, Basis and descriptions.
        finished: Finished,
    },
}

/// Decides one finish from the work's records. The same records give the
/// same decision, so a recovered run that asks again gets the same answer.
pub fn decide(
    turn: &Turn,
    records: &[(Event, bool)],
    messages: &[ProductMessage],
) -> Result<Decision, Cause> {
    let finish = match submitted(turn) {
        Ok((_, finish)) => finish,
        Err(reason) => return Ok(Decision::Refuse(Refusal::new(reason))),
    };
    let labels = Labels::of_work(records);
    let delivered = Delivered::of_work(records, messages)?;
    let mut unknown = Vec::new();
    let answer = resolve(
        Text::Answer,
        &finish.answer,
        &finish.quotes,
        &labels,
        &delivered,
        &mut unknown,
    );
    let basis = resolve(
        Text::Basis,
        &finish.basis,
        &[],
        &labels,
        &delivered,
        &mut unknown,
    );
    let labelled = !marks_in(&finish.answer).is_empty() || !marks_in(&finish.basis).is_empty();
    let unlabelled = !labelled && !finish.cites_none;
    if (unlabelled || !unknown.is_empty())
        && refused_for_citations(records) < REFUSALS_BEFORE_DROPPING
    {
        let mut reason = String::from(
            "Nothing was saved. Correct these and call finish_answer again with the whole finish:",
        );
        for line in unlabelled
            .then(|| NO_LABEL.to_owned())
            .into_iter()
            .chain(unknown.iter().map(ToString::to_string))
        {
            reason.push_str("\n- ");
            reason.push_str(&line);
        }
        return Ok(Decision::Refuse(Refusal::new(reason)));
    }
    let cited: Vec<&SourceVersion> = answer
        .marks
        .iter()
        .chain(&basis.marks)
        .map(|mark| &mark.source)
        .collect();
    let descriptions = kept(finish.descriptions, &labels, &cited);
    Ok(Decision::Save {
        text: answer.text,
        finished: Finished {
            marks: answer.marks,
            basis,
            descriptions,
        },
    })
}

/// The descriptions a finish keeps: for each version a mark cites, the first
/// description whose label names that version and whose text is not blank.
/// Every other description is dropped, and none refuses a finish.
fn kept(described: Vec<Described>, labels: &Labels, cited: &[&SourceVersion]) -> Vec<Description> {
    let mut kept: Vec<Description> = Vec::new();
    for Described { label, description } in described {
        if description.trim().is_empty() {
            continue;
        }
        let Some((candidate_id, revision)) = labels.version(&label) else {
            continue;
        };
        let source = SourceVersion {
            candidate_id: candidate_id.to_owned(),
            revision: revision.to_owned(),
        };
        if !cited.contains(&&source) || kept.iter().any(|known| known.source == source) {
            continue;
        }
        kept.push(Description {
            source,
            text: description,
        });
    }
    kept
}

/// How many finishes of this work were refused for their labels, no label
/// while `cites_none` is false or a label no read gave: well-formed finishes
/// whose refusal was sent back to the model. A finish not yet answered, such
/// as the one being decided, does not count.
fn refused_for_citations(records: &[(Event, bool)]) -> usize {
    let mut refused: Vec<&str> = Vec::new();
    for (event, _) in records {
        let EventKind::ProviderStarted { request, .. } = &event.kind else {
            continue;
        };
        for item in &request.added {
            if let Item::Result { call_id, output } = item
                && matches!(output.source, ResultSource::FinishRefused { .. })
                && !refused.contains(&call_id.as_str())
            {
                refused.push(call_id);
            }
        }
    }
    let mut counted: Vec<&str> = Vec::new();
    for (event, _) in records {
        let EventKind::ProviderReturned(turn) = &event.kind else {
            continue;
        };
        if let Ok((id, _)) = submitted(turn)
            && refused.contains(&id)
            && !counted.contains(&id)
        {
            counted.push(id);
        }
    }
    counted.len()
}

/// One label written in a text: the bytes it takes out of the text, and the
/// label. The first label of a bracket takes the bracket with the spaces
/// before it; a later label of the same bracket takes no bytes.
struct Written {
    remove: std::ops::Range<usize>,
    label: Label,
}

/// Every label written in a text, in order. A label is `S<digits>`; a
/// bracket holds one label, `[S3]`, or several separated by a comma, spaces
/// or both, `[S7, S3]`, `[S7,S3]`, `[S7 S3]`, and each is its own mark at the
/// same place. Nothing else in the text is read.
fn marks_in(text: &str) -> Vec<Written> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(open) = text[from..].find('[').map(|index| from + index) {
        let Some((labels, end)) = bracket(bytes, open) else {
            from = open + 1;
            continue;
        };
        let mut start = open;
        while start > 0 && matches!(bytes[start - 1], b' ' | b'\t') {
            start -= 1;
        }
        for (index, label) in labels.into_iter().enumerate() {
            found.push(Written {
                remove: if index == 0 { start..end } else { end..end },
                label: Label(text[label].to_owned()),
            });
        }
        from = end;
    }
    found
}

/// The labels of the bracket that opens at `open`, and the byte after its
/// `]`, when it holds only labels: `S<digits>`, separated by spaces, at most
/// one comma, or both.
fn bracket(bytes: &[u8], open: usize) -> Option<(Vec<std::ops::Range<usize>>, usize)> {
    let spaces = |from: usize| {
        bytes[from..]
            .iter()
            .take_while(|byte| **byte == b' ')
            .count()
    };
    let mut labels = Vec::new();
    let mut at = open + 1;
    loop {
        if bytes.get(at) != Some(&b'S') {
            return None;
        }
        let digits = bytes[at + 1..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if digits == 0 {
            return None;
        }
        labels.push(at..at + 1 + digits);
        at += 1 + digits;
        if bytes.get(at) == Some(&b']') {
            return Some((labels, at + 1));
        }
        let mut next = at + spaces(at);
        if bytes.get(next) == Some(&b',') {
            next += 1 + spaces(next + 1);
        }
        if next == at {
            return None;
        }
        at = next;
    }
}

/// The text without its labels, as a kept answer shows it: no citation marks
/// show beside text that did not finish.
pub fn without_labels(text: &str) -> String {
    let mut kept = String::with_capacity(text.len());
    let mut from = 0;
    for written in marks_in(text) {
        kept.push_str(&text[from..written.remove.start]);
        from = written.remove.end;
    }
    kept.push_str(&text[from..]);
    kept
}

/// Resolves the labels of one text against its quotes. Every label leaves
/// the text. A label a read of this work gave becomes a mark at the place it
/// sat that names the label's version; a label no read gave becomes no mark
/// and is added to `unknown` once for the text. For each label, [`pairing`]
/// decides which of its quotes each of its marks takes, from the counts. A
/// mark's place is the [`joined`] places of the quotes it takes that are
/// found in the delivered text, and none when no such quote is found. A
/// quote whose label is not in the text is ignored.
fn resolve(
    which: Text,
    text: &str,
    quotes: &[Quote],
    labels: &Labels,
    delivered: &Delivered,
    unknown: &mut Vec<UnknownLabel>,
) -> CitedText {
    let written = marks_in(text);
    let mut decided: Vec<Option<(SourceVersion, Option<Place>)>> = vec![None; written.len()];
    let mut done: Vec<&Label> = Vec::new();
    for mark in &written {
        let label = &mark.label;
        if done.contains(&label) {
            continue;
        }
        done.push(label);
        let Some((candidate_id, revision)) = labels.version(label) else {
            unknown.push(UnknownLabel {
                text: which,
                label: label.clone(),
            });
            continue;
        };
        let source = SourceVersion {
            candidate_id: candidate_id.to_owned(),
            revision: revision.to_owned(),
        };
        let at: Vec<usize> = (0..written.len())
            .filter(|index| &written[*index].label == label)
            .collect();
        let places: Vec<Option<Place>> = quotes
            .iter()
            .filter(|quote| &quote.label == label)
            .map(|quote| {
                delivered
                    .get(label)
                    .and_then(|version| locate(version, &quote.quote))
            })
            .collect();
        for (index, taken) in at.iter().zip(pairing(at.len(), places.len())) {
            let found: Vec<&Place> = places[taken].iter().flatten().collect();
            decided[*index] = Some((source.clone(), joined(&found)));
        }
    }
    let mut cited = CitedText {
        text: String::with_capacity(text.len()),
        marks: Vec::new(),
    };
    let mut kept_from = 0;
    for (mark, decided) in written.iter().zip(decided) {
        cited.text.push_str(&text[kept_from..mark.remove.start]);
        kept_from = mark.remove.end;
        if let Some((source, place)) = decided {
            cited.marks.push(Mark {
                at: cited.text.len(),
                source,
                place,
            });
        }
    }
    cited.text.push_str(&text[kept_from..]);
    cited
}

/// One mark's place from the places of the quotes it takes that were found:
/// the spans of every place in part and byte order, where spans that touch
/// or overlap are one. None when there is no such place.
fn joined(places: &[&Place]) -> Option<Place> {
    let mut spans: Vec<Span> = places
        .iter()
        .flat_map(|place| match place {
            Place::Passage { spans } => spans.spans().iter().cloned(),
        })
        .collect();
    spans.sort_by_key(|span| (span.part, span.start));
    let mut merged: Vec<Span> = Vec::new();
    for span in spans {
        if let Some(last) = merged.last_mut()
            && last.part == span.part
            && span.start <= last.end
        {
            last.end = last.end.max(span.end);
            continue;
        }
        merged.push(span);
    }
    let spans = Spans::try_from(merged).ok()?;
    Some(Place::Passage { spans })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delivered(stretches: Vec<(usize, usize, &str)>) -> DeliveredVersion {
        let mut version = DeliveredVersion {
            stretches: stretches
                .into_iter()
                .map(|(part, start, text)| (part, start, text.to_owned()))
                .collect(),
        };
        version.merge();
        version
    }

    /// A quote is found across whitespace differences, and its place is the
    /// span in the source's own bytes, without the whitespace around it.
    #[test]
    fn a_quote_resolves_to_its_bytes_across_whitespace() {
        let text = "Seen today.\n\nPotassium   was\tnormal. Plan: recheck.";
        let version = delivered(vec![(1, 0, text)]);
        let Some(Place::Passage { spans }) = locate(&version, " Potassium was normal. ") else {
            panic!("quote not found")
        };
        let [Span { part, start, end }] = spans.spans() else {
            panic!("{spans:?}")
        };
        assert_eq!(*part, 1);
        assert_eq!(&text[*start..*end], "Potassium   was\tnormal.");
        assert_eq!(locate(&version, "Sodium"), None);
        assert_eq!(locate(&version, "  "), None);
    }

    /// Two passages that touch read as one text, so a quote across their
    /// seam is found; a gap between passages is not delivered text.
    #[test]
    fn passages_that_touch_join_and_a_gap_does_not() {
        let text = "alpha beta gamma delta";
        let joined = delivered(vec![
            (0, 6, &text[6..12]),
            (0, 0, &text[..8]),
            (0, 12, &text[12..]),
        ]);
        assert_eq!(
            locate(&joined, "beta gamma"),
            Some(Place::Passage {
                spans: Spans(vec![Span {
                    part: 0,
                    start: 6,
                    end: 16
                }])
            })
        );
        let gap = delivered(vec![(0, 0, &text[..6]), (0, 11, &text[11..])]);
        assert!(locate(&gap, "beta gamma").is_none());
    }

    /// Only `[S<digits>]` is a label; the spaces before it leave with it.
    #[test]
    fn only_bracketed_labels_are_read() {
        let found: Vec<_> = marks_in("A [S1] b [S12][S3] [sic] [S] [Sx] [S4")
            .into_iter()
            .map(|mark| (mark.remove, mark.label.0))
            .collect();
        assert_eq!(
            found,
            [
                (1..6, "S1".to_owned()),
                (8..14, "S12".to_owned()),
                (14..18, "S3".to_owned()),
            ]
        );
    }

    /// The count rule, one row per case: which quotes each mark of a label
    /// takes.
    #[test]
    fn the_counts_decide_which_quotes_each_mark_takes() {
        // Each mark's range of quotes is written as (first, end).
        let rows = [
            // As many marks as quotes: each takes its own, in order.
            (1, 1, vec![(0, 1)]),
            (3, 3, vec![(0, 1), (1, 2), (2, 3)]),
            // One mark takes every quote.
            (1, 2, vec![(0, 2)]),
            // One quote serves every mark.
            (3, 1, vec![(0, 1), (0, 1), (0, 1)]),
            // No quote: no mark takes one.
            (2, 0, vec![(0, 0), (0, 0)]),
            // Any other count: every mark takes every quote.
            (3, 4, vec![(0, 4), (0, 4), (0, 4)]),
            (3, 2, vec![(0, 2), (0, 2), (0, 2)]),
            // No mark: no quote is taken.
            (0, 1, vec![]),
        ];
        for (marks, quotes, takes) in rows {
            assert_eq!(
                pairing(marks, quotes),
                takes
                    .into_iter()
                    .map(|(first, end)| first..end)
                    .collect::<Vec<_>>(),
                "{marks} marks, {quotes} quotes"
            );
        }
    }

    /// A bracket with several labels separated by a comma, spaces or both is
    /// read as one mark per label, in order, at one place; the bracket and
    /// the spaces before it leave the text once. Other words inside the
    /// brackets are not read.
    #[test]
    fn a_bracket_with_several_labels_is_read_as_one_mark_each() {
        for text in ["A [S7, S3] b", "A [S7,S3] b", "A [S7 S3] b"] {
            let found: Vec<_> = marks_in(text)
                .into_iter()
                .map(|mark| (mark.remove, mark.label.0))
                .collect();
            let end = text.len() - 2;
            assert_eq!(
                found,
                [(1..end, "S7".to_owned()), (end..end, "S3".to_owned())],
                "{text}"
            );
            assert_eq!(without_labels(text), "A b", "{text}");
        }
        for text in [
            "[S7; S3]",
            "[S7 and S3]",
            "[S 7]",
            "[see S7]",
            "[S7,,S3]",
            "[S7, ]",
            "[ S7]",
        ] {
            assert!(marks_in(text).is_empty(), "{text}");
        }
    }

    fn quote(label: &str, quote: &str) -> Quote {
        Quote {
            label: Label(label.into()),
            quote: quote.into(),
        }
    }

    /// The note text, delivered as part 1 of `S1` and of `S2`.
    const NOTE: &str = "Seen today. Potassium was normal. Sodium was low. Plan: recheck.";

    /// The labels of a work whose reads gave `S1` and `S2`: each read asked
    /// for its note and returned it as a passage with its label.
    fn labels() -> Labels {
        let event = |kind| {
            (
                Event {
                    attempt_id: "attempt".into(),
                    sequence: 1,
                    ordinal: 1,
                    kind,
                },
                true,
            )
        };
        let started = |candidate: &str| {
            event(EventKind::ToolStarted(harness::engine::Invocation {
                id: candidate.into(),
                name: super::super::READ_TOOL.into(),
                arguments: json!({
                    "references": [{"candidate_id": candidate, "revision": "r", "cursor": null}],
                    "max_result_bytes": 4096
                })
                .to_string(),
            }))
        };
        let returned = |candidate: &str, label: &str| {
            let receipt = ReadReceipt {
                results: vec![RecordReceipt::Passage {
                    reference: candidate.into(),
                    revision: "r".into(),
                    part: 1,
                    label: "attachment".into(),
                    start_byte: 0,
                    end_byte: NOTE.len(),
                    total_bytes: NOTE.len(),
                    content: NOTE.into(),
                    next: None,
                    cite_as: Some(Label(label.into())),
                }],
            };
            event(EventKind::ToolReturned {
                call_id: candidate.into(),
                result: harness::engine::results::ToolResult::Original(
                    serde_json::to_string(&receipt).unwrap(),
                ),
            })
        };
        Labels::of_work(&[
            started("n1"),
            returned("n1", "S1"),
            started("n2"),
            returned("n2", "S2"),
        ])
    }

    /// The text resolved against these quotes, and what the model is told.
    fn resolved(text: &str, quotes: &[Quote]) -> (CitedText, Vec<String>) {
        let note = Delivered(
            ["S1", "S2"]
                .into_iter()
                .map(|label| (Label(label.into()), delivered(vec![(1, 0, NOTE)])))
                .collect(),
        );
        let mut unknown = Vec::new();
        let cited = resolve(Text::Answer, text, quotes, &labels(), &note, &mut unknown);
        (cited, unknown.iter().map(ToString::to_string).collect())
    }

    fn spans_of(mark: &Mark) -> Vec<(usize, usize)> {
        match &mark.place {
            Some(Place::Passage { spans }) => {
                spans.spans().iter().map(|s| (s.start, s.end)).collect()
            }
            None => Vec::new(),
        }
    }

    fn at(words: &str) -> (usize, usize) {
        let start = NOTE.find(words).unwrap();
        (start, start + words.len())
    }

    /// One mark that takes two quotes saves one mark whose spans are both
    /// passages in byte order, whatever order the quotes came in.
    #[test]
    fn one_mark_with_two_quotes_holds_both_passages_in_order() {
        let (cited, bad) = resolved(
            "Potassium normal, sodium low [S1].",
            &[quote("S1", "Sodium was low."), quote("S1", "Potassium")],
        );
        assert!(bad.is_empty(), "{bad:?}");
        assert_eq!(cited.text(), "Potassium normal, sodium low.");
        let [mark] = cited.marks() else {
            panic!("{:?}", cited.marks())
        };
        assert_eq!(mark.at, 28);
        assert_eq!(spans_of(mark), [at("Potassium"), at("Sodium was low.")]);
    }

    /// Two quotes of one mark whose spans touch or overlap give one span.
    #[test]
    fn touching_spans_of_one_mark_join() {
        let touching = [quote("S1", "al."), quote("S1", "Potassium was norm")];
        let overlapping = [quote("S1", "Potassium was"), quote("S1", "was normal.")];
        for quotes in [touching, overlapping] {
            let (cited, bad) = resolved("Normal [S1].", &quotes);
            assert!(bad.is_empty(), "{bad:?}");
            let [mark] = cited.marks() else {
                panic!("{:?}", cited.marks())
            };
            assert_eq!(spans_of(mark), [at("Potassium was normal.")]);
        }
    }

    /// One mark with two quotes, one not found: the mark keeps the place of
    /// the quote that was found, and nothing is told.
    #[test]
    fn one_quote_not_found_leaves_the_place_of_the_found_one() {
        let (cited, told) = resolved(
            "Normal [S1].",
            &[quote("S1", "Potassium"), quote("S1", "Calcium")],
        );
        assert!(told.is_empty(), "{told:?}");
        assert_eq!(cited.text(), "Normal.");
        let [mark] = cited.marks() else {
            panic!("{:?}", cited.marks())
        };
        assert_eq!(mark.source.candidate_id, "n1");
        assert_eq!(spans_of(mark), [at("Potassium")]);
    }

    /// Several labels in one bracket each get a mark at the same place, and
    /// the saved text is the text without its labels.
    #[test]
    fn a_bracket_with_two_labels_saves_two_marks_at_one_place() {
        let text = "Potassium normal, sodium low [S1, S2].";
        let (cited, bad) = resolved(text, &[quote("S1", "Potassium"), quote("S2", "Sodium")]);
        assert!(bad.is_empty(), "{bad:?}");
        assert_eq!(cited.text(), "Potassium normal, sodium low.");
        let marks: Vec<_> = cited
            .marks()
            .iter()
            .map(|mark| (mark.at, spans_of(mark)))
            .collect();
        assert_eq!(
            marks,
            [(28, vec![at("Potassium")]), (28, vec![at("Sodium")])]
        );
    }
}
