//! Event-backed working notes for authorized Chords conversations and revision jobs.
//! Callers supply only their scoped records. ToolStarted retains the exact text;
//! the acknowledged ToolReturned receipt makes the note readable.
use super::{Event, Invocation, ToolOutcome, engine, error};
use serde_json::Value;

const SAVE_WORKING_NOTE: &str = "save_working_note";
const READ_WORKING_NOTE: &str = "read_working_note";

pub(super) fn is_tool(name: &str) -> bool {
    matches!(name, SAVE_WORKING_NOTE | READ_WORKING_NOTE)
}

fn working_note_reference(work: &str, call_id: &str) -> String {
    format!("{work}:{call_id}")
}

fn saved_note(
    records: &[(&str, &Event)],
    reference: &str,
) -> Result<Option<String>, engine::Cause> {
    let confirmed = records.iter().any(|(work, event)| {
        matches!(&event.kind, engine::EventKind::ToolReturned {
            call_id,
            result: engine::results::ToolResult::Original(output),
        } if working_note_reference(work, call_id) == reference
            && serde_json::from_str::<engine::WorkingNoteReceipt>(output)
                .is_ok_and(|receipt| receipt.reference == reference))
    });
    if !confirmed {
        return Ok(None);
    }
    let mut found = None;
    for (work, event) in records {
        if let engine::EventKind::ToolStarted(call) = &event.kind
            && call.name == SAVE_WORKING_NOTE
            && working_note_reference(work, &call.id) == reference
        {
            let arguments: engine::SaveWorkingNote = serde_json::from_str(&call.arguments)?;
            if found
                .as_deref()
                .is_some_and(|previous| previous != arguments.text)
            {
                return Err(error("working note identity collision").into());
            }
            found = Some(arguments.text);
        }
    }
    Ok(found)
}

/// List only acknowledged notes from already-authorized conversation records,
/// in the order the records saved them.
pub fn references<'a>(
    records: impl IntoIterator<Item = (&'a str, &'a Event)>,
) -> Result<Vec<String>, engine::Cause> {
    let records: Vec<_> = records.into_iter().collect();
    let mut references = Vec::new();
    for (work, event) in &records {
        if let engine::EventKind::ToolStarted(call) = &event.kind
            && call.name == SAVE_WORKING_NOTE
        {
            let reference = working_note_reference(work, &call.id);
            if !references.contains(&reference) && saved_note(&records, &reference)?.is_some() {
                references.push(reference);
            }
        }
    }
    Ok(references)
}

/// Execute a note capability against records the product has already authorized.
/// Unknown tool names return None so the owning job can dispatch its other tools.
pub fn execute<'a>(
    work: &str,
    call: &Invocation<Value>,
    records: impl IntoIterator<Item = (&'a str, &'a Event)>,
) -> Result<Option<ToolOutcome>, engine::Cause> {
    if !is_tool(&call.name) {
        return Ok(None);
    }
    let records: Vec<_> = records.into_iter().collect();
    if call.name == SAVE_WORKING_NOTE {
        let save: engine::SaveWorkingNote = match serde_json::from_value(call.arguments.clone()) {
            Ok(save) => save,
            Err(reason) => return Ok(Some(ToolOutcome::refused(reason))),
        };
        let text = save.text.as_str();
        let reference = working_note_reference(work, &call.id);
        for (prior_work, event) in &records {
            if let engine::EventKind::ToolStarted(saved) = &event.kind
                && saved.name == SAVE_WORKING_NOTE
                && working_note_reference(prior_work, &saved.id) == reference
            {
                let retained: engine::SaveWorkingNote = serde_json::from_str(&saved.arguments)?;
                if retained.text != text {
                    return Err("working note identity collision".into());
                }
            }
        }
        return Ok(Some(ToolOutcome::Returned(serde_json::to_string(
            &engine::WorkingNoteReceipt { reference },
        )?)));
    }
    if call.name == READ_WORKING_NOTE {
        let read: engine::ReadWorkingNote = match serde_json::from_value(call.arguments.clone()) {
            Ok(read) => read,
            Err(reason) => return Ok(Some(ToolOutcome::refused(reason))),
        };
        let reference = read.reference.as_str();
        let Some(text) = saved_note(&records, reference)? else {
            return Ok(Some(ToolOutcome::refused(
                "working note is not available in this conversation",
            )));
        };
        return Ok(Some(ToolOutcome::Returned(
            serde_json::json!({"reference":reference,"text":text}).to_string(),
        )));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::{EventKind, results::ToolResult};
    use serde_json::json;

    #[test]
    fn refused_saves_do_not_poison_note_discovery_or_reads() {
        let mut events = Vec::new();
        for (index, arguments) in [
            json!({"text":"", "fresh_window":true}),
            json!({"text":"draft", "fresh_window":"true"}),
            json!({"text":"draft", "fresh_window":false, "extra":true}),
            json!({"text":"Saved progress", "fresh_window":false}),
        ]
        .into_iter()
        .enumerate()
        {
            let call = Invocation {
                id: index.to_string(),
                name: SAVE_WORKING_NOTE.into(),
                arguments,
            };
            let outcome = execute("work", &call, events.iter().map(|event| ("work", event)))
                .unwrap()
                .unwrap();
            let result = match outcome {
                ToolOutcome::Returned(output) => ToolResult::Original(output),
                ToolOutcome::Refused(refusal) => ToolResult::Refused(refusal),
                _ => panic!("note tools return a result or refusal"),
            };
            assert_eq!(matches!(result, ToolResult::Original(_)), index == 3);
            for kind in [
                EventKind::ToolStarted(Invocation {
                    id: call.id.clone(),
                    name: call.name,
                    arguments: call.arguments.to_string(),
                }),
                EventKind::ToolReturned {
                    call_id: call.id,
                    result,
                },
            ] {
                events.push(Event {
                    attempt_id: "attempt".into(),
                    sequence: events.len() as u64 + 1,
                    ordinal: 1,
                    kind,
                });
            }
        }
        assert_eq!(
            references(events.iter().map(|event| ("work", event)))
                .expect("refused saves must not break note discovery"),
            ["work:3"]
        );
        for index in 0..4 {
            let call = Invocation {
                id: format!("read-{index}"),
                name: READ_WORKING_NOTE.into(),
                arguments: json!({"reference":format!("work:{index}")}),
            };
            let outcome = execute(
                "next-work",
                &call,
                events.iter().map(|event| ("work", event)),
            )
            .unwrap()
            .unwrap();
            match outcome {
                ToolOutcome::Returned(output) if index == 3 => assert_eq!(
                    serde_json::from_str::<Value>(&output).unwrap()["text"],
                    "Saved progress"
                ),
                ToolOutcome::Refused(_) if index < 3 => {}
                _ => panic!("only an acknowledged valid save is readable"),
            }
        }
    }
}
