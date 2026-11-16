//! The patient memory tool that changes the summary, the direction, the
//! person page and the subject pages, each a file of markdown text. Chat and
//! reply revisions run it through [`MemoryTool`](super::tools::MemoryTool),
//! and patient rounds run it beside the reads with their own patient key.
//! Arguments parse once into an `EditRequest` of the store's own types; past
//! the parse no code reads a wire field. The patient is always the run's,
//! never an argument.
use harness::engine::{Tool, ToolOutcome};
use sea_orm::DbErr;
use serde::Deserialize;
use serde_json::{Value, json};

use super::tools::object;
use super::{
    Edit, EditInputError, EditRefusal, MemoryPath, NotApplied, PatientKey, VersionedFile, Words,
    Written,
};
use crate::runtime::Store;

/// The one tool that edits a memory file by its path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditTool;

impl EditTool {
    pub(super) const NAME: &'static str = "edit_patient_memory";

    /// The edit tool, when `name` is its name.
    pub fn named(name: &str) -> Option<Self> {
        (name == Self::NAME).then_some(Self)
    }

    /// Runs one call of the tool for the patient, from its arguments to its
    /// outcome. Arguments the model can correct come back refused; a
    /// database failure stays an error.
    pub async fn execute(
        self,
        store: &Store,
        key: &PatientKey,
        arguments: &Value,
    ) -> Result<ToolOutcome, DbErr> {
        let EditRequest { file, edits } = match EditRequest::parse(arguments) {
            Ok(request) => request,
            Err(reason) => return Ok(ToolOutcome::refused(reason)),
        };
        let path = MemoryPath::from(file.clone());
        let written = store.edit_memory(key, &file, &edits).await?;
        Ok(ToolOutcome::Returned(result(&path, &written).to_string()))
    }

    pub(super) fn definition(self) -> Tool {
        let text = || object(json!({"text":{"type":"string"}}));
        let edits = [
            object(json!({"create": text()})),
            object(json!({"replace":object(json!({
                "old_text":{"type":"string"},"text":{"type":"string"}
            }))})),
            object(json!({"append": text()})),
            object(json!({"delete": object(json!({}))})),
        ];
        Tool {
            name: Self::NAME.into(),
            description: "Change one of this patient's memory files. path is summary.md, direction.md, person.md or pages/<id>.md; Coda puts a page's id in lowercase with hyphens. A file is markdown text, and edits is a list of changes applied in order. create makes a file that does not exist yet, with the text you give. replace changes old_text to text; old_text is text copied exactly from the file as you read it, in one place only, and the rest of the file stays. append adds text at the end, and makes the file when there is none. delete takes the file out of the list; its earlier versions stay readable. An edit that cannot apply, or that would change nothing, is left out, and the result names it and why; the other edits are saved. The result gives the version saved, the file's size in characters and each edit left out.".into(),
            parameters: object(json!({
                "path":{"type":"string"},
                "edits":{"type":"array","items":{"anyOf":edits}}
            })),
        }
    }
}

/// The edit tool, as patient rounds are offered it.
pub fn tool() -> Tool {
    EditTool.definition()
}

/// A call. Each edit is read on its own, so one malformed edit is left out
/// and the others still apply.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EditsWire {
    path: String,
    edits: Vec<Value>,
}

/// One edit, the same for every file.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum EditWire {
    Create(TextWire),
    Replace(ReplaceWire),
    Append(TextWire),
    Delete(DeleteWire),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TextWire {
    text: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplaceWire {
    old_text: String,
    text: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeleteWire {}

/// An edit tool call the model can correct. Nothing was saved.
#[derive(Debug, thiserror::Error)]
enum EditCallError {
    /// The call did not match the offered fields.
    #[error(
        "Supply path as summary.md, direction.md, person.md or pages/<id>.md, and edits as an array of objects that each hold one of create, replace, append or delete ({0})."
    )]
    Shape(#[source] serde_json::Error),
    /// The path names a log day or no memory file, so no edit has a file to
    /// go to.
    #[error(
        "Give path as summary.md, direction.md, person.md or pages/<id>.md; {0} cannot be edited."
    )]
    NotEditable(String),
}

/// A parsed edit tool call, in the store's own types.
#[derive(Clone, Debug, PartialEq, Eq)]
struct EditRequest {
    /// The file.
    file: VersionedFile,
    /// Each edit in order, parsed or left out with its reason.
    edits: Vec<Result<Edit, EditRefusal>>,
}

impl EditRequest {
    /// The call's typed request. The one place the tool's wire fields are
    /// read.
    fn parse(arguments: &Value) -> Result<Self, EditCallError> {
        let wire: EditsWire =
            serde_json::from_value(arguments.clone()).map_err(EditCallError::Shape)?;
        // A log day, or a path that names no memory file, has no file to
        // edit; the path as given says which path that was.
        let Some(file) = MemoryPath::parse(&wire.path)
            .ok()
            .and_then(|path| VersionedFile::try_from(path).ok())
        else {
            return Err(EditCallError::NotEditable(wire.path));
        };
        let edits = wire.edits.into_iter().map(EditWire::parse).collect();
        Ok(Self { file, edits })
    }
}

/// An edit's field that must have words.
fn words(field: &'static str, text: &str) -> Result<Words, EditInputError> {
    Words::parse(text).map_err(|source| EditInputError::Words { field, source })
}

impl EditWire {
    /// One edit as given, or why it is left out.
    fn parse(edit: Value) -> Result<Edit, EditRefusal> {
        let wire = serde_json::from_value::<Self>(edit)
            .map_err(|error| EditInputError::Shape(error.to_string()))?;
        Ok(wire.edit()?)
    }

    /// The edit in the store's types.
    fn edit(self) -> Result<Edit, EditInputError> {
        Ok(match self {
            Self::Create(create) => Edit::Create {
                text: words("create.text", &create.text)?,
            },
            Self::Replace(replace) => Edit::Replace {
                old_text: words("replace.old_text", &replace.old_text)?,
                text: replace.text,
            },
            Self::Append(append) => Edit::Append {
                text: words("append.text", &append.text)?,
            },
            Self::Delete(DeleteWire {}) => Edit::Delete,
        })
    }
}

/// A call's result for the file at `path`.
fn result(path: &MemoryPath, written: &Written) -> Value {
    json!({
        "path": path.to_string(),
        "version": written.version,
        "characters": written.characters,
        "not_applied": not_applied(&written.not_applied),
    })
}

/// Each edit left out, by its place in the list and the reason.
fn not_applied(left_out: &[NotApplied]) -> Vec<Value> {
    left_out
        .iter()
        .map(|left_out| json!({"edit": left_out.edit, "reason": left_out.reason.to_string()}))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::memory::PageId;

    fn parse(arguments: Value) -> Result<EditRequest, String> {
        EditRequest::parse(&arguments).map_err(|error| error.to_string())
    }

    /// A parsed call's file and each edit in the store's types.
    type Parsed = (VersionedFile, Vec<Result<Edit, EditRefusal>>);

    /// A call that edits the file at `path`, as its file and edits.
    fn edits(path: &str, edits: Value) -> Result<Parsed, String> {
        parse(json!({"path": path, "edits": edits})).map(|request| (request.file, request.edits))
    }

    fn words(text: &str) -> Words {
        Words::parse(text).unwrap()
    }

    /// The reason each edit of a parsed call was left out at the parse, or
    /// `None` for an edit that parsed.
    fn reasons(request: Result<Parsed, String>) -> Vec<Option<String>> {
        match request {
            Ok((_, edits)) => edits
                .iter()
                .map(|edit| edit.as_ref().err().map(ToString::to_string))
                .collect(),
            Err(refused) => panic!("a call: {refused}"),
        }
    }

    #[test]
    fn a_call_parses_to_its_file_and_each_edit_with_its_text_as_given() {
        assert_eq!(
            edits(
                "pages/ckd.md",
                json!([
                    {"create": {"text": "---\nname: CKD\n---\n\n- Stage 3b [N1]\n"}},
                    {"replace": {"old_text": "- Stage 3b [N1]", "text": ""}},
                    {"append": {"text": "  - eGFR 38 [chart]"}},
                    {"delete": {}}
                ])
            ),
            Ok((
                VersionedFile::Page(PageId::parse("ckd").unwrap()),
                vec![
                    Ok(Edit::Create {
                        text: words("---\nname: CKD\n---\n\n- Stage 3b [N1]\n")
                    }),
                    Ok(Edit::Replace {
                        old_text: words("- Stage 3b [N1]"),
                        text: String::new(),
                    }),
                    Ok(Edit::Append {
                        text: words("  - eGFR 38 [chart]")
                    }),
                    Ok(Edit::Delete),
                ],
            ))
        );
        // A page id given in another spelling names the page in its normal
        // form, and every other file takes the same edits.
        for (path, file) in [
            (
                "pages/CKD Stage 3",
                VersionedFile::Page(PageId::parse("ckd-stage-3").unwrap()),
            ),
            ("summary.md", VersionedFile::Summary),
            ("direction.md", VersionedFile::Direction),
            ("person.md", VersionedFile::Person),
        ] {
            assert_eq!(
                edits(path, json!([{"delete": {}}])),
                Ok((file, vec![Ok(Edit::Delete)])),
                "{path}"
            );
        }
    }

    #[test]
    fn a_call_whose_own_fields_are_malformed_is_refused_with_the_fields_to_supply() {
        for arguments in [
            json!({"path": "person.md", "edits": {"delete": {}}}),
            json!({"path": "person.md"}),
            json!({"edits": []}),
            json!({"path": "summary.md", "edits": [], "text": "Stop the lisinopril"}),
        ] {
            let refused = parse(arguments.clone()).unwrap_err();
            assert!(
                refused.starts_with(
                    "Supply path as summary.md, direction.md, person.md or pages/<id>.md, and edits as an array of objects that each hold one of create, replace, append or delete ("
                ),
                "{arguments}: {refused}"
            );
        }
        for path in [
            "log/2026-11-13.md",
            "pages/ - .md",
            "notes.md",
            "Summary.md",
            "log/2026-02-30.md",
        ] {
            assert_eq!(
                edits(path, json!([])),
                Err(format!(
                    "Give path as summary.md, direction.md, person.md or pages/<id>.md; {path} cannot be edited."
                ))
            );
        }
    }

    #[test]
    fn a_malformed_or_blank_edit_is_left_out_with_what_to_give_and_the_others_parse() {
        let good = json!({"append": {"text": "Has CKD"}});
        for bad in [
            json!({"move": {"text": "Has CKD"}}),
            json!({"append": {"text": "a"}, "create": {"text": "Has CKD"}}),
            json!({"append": {"text": "Has CKD", "sources": ["N1"]}}),
            json!({"replace": {"text": "Has CKD", "new_text": "Has CKD 3"}}),
            json!({"delete": {"path": "summary.md"}}),
            json!({"delete": null}),
            json!({"add": {"section": null, "text": "Has CKD"}}),
            json!("delete"),
        ] {
            let parsed = reasons(edits("person.md", json!([good, bad])));
            let [None, Some(reason)] = &parsed[..] else {
                panic!("{bad}: {parsed:?}");
            };
            assert!(
                reason.starts_with(
                    "Give the edit as an object that holds one of create, replace, append or delete, with the fields each takes ("
                ),
                "{bad}: {reason}"
            );
        }
        let no_words = |field: &str| {
            Some(format!(
                "Give {field} as one or more words (the text has no words)."
            ))
        };
        assert_eq!(
            reasons(edits(
                "person.md",
                json!([
                    {"create": {"text": " \n"}},
                    {"append": {"text": ""}},
                    {"replace": {"old_text": "\t", "text": "Has CKD 3"}},
                    // A replace's text may be empty: the named text goes.
                    {"replace": {"old_text": "Stage 3", "text": ""}},
                    good
                ])
            )),
            [
                no_words("create.text"),
                no_words("append.text"),
                no_words("replace.old_text"),
                None,
                None,
            ]
        );
    }
}
