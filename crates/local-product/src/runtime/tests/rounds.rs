//! Postgres proof of patient rounds with a scripted model: a finished run
//! saves one rounds mark at its start, which its log day shows in time order;
//! a run opens with the patient's memory, then when rounds last finished,
//! today and how many notes each log day holds since; a conversation starts
//! with only the notes saved since the last finished run started, and one
//! that started before a run finished keeps its message;
//! a run that reads the log and the chart, creates a page whose line points
//! to a note in its words and replaces a part of it, creates a direction and
//! appends to the summary ends finished
//! and memory reads back what it wrote; a run a limit stops ends not
//! finished, keeps what it saved and saves no mark, and so does a run whose
//! provider fails, whose notes the next run sees as new; and a run with nothing
//! new changes no file and adds no note.
use chrono::{DateTime, Duration, Utc};
use harness::engine::results::ToolResult;
use harness::engine::{Event, EventKind, Item, LimitBackstop};
use serde_json::json;

use super::memory_tests::{key, note, patient, store};
use super::{ConversationStart, Scope, Script, Store, calls, says, usage};
use crate::physician_sources::adapter::file_fhir_record;
use crate::physician_sources::{SourcePatient, SourceRevision};
use crate::runtime::memory::{FileRead, MemoryPath, PatientKey, VersionedFile};
use crate::runtime::rounds::{self, Ending, RoundsRun};

/// The source the test's records come from.
const SOURCE: &str = "fhir-r4";

/// A creatinine result as a FHIR source sends it. A new value is a new
/// revision of the same record.
fn lab(id: &str, value: &str) -> String {
    format!(
        r#"{{"resourceType":"Observation","id":"{id}","status":"final","code":{{"text":"Creatinine"}},"valueQuantity":{{"value":{value},"unit":"mg/dL"}},"effectiveDateTime":"2026-08-01"}}"#
    )
}

/// Files each record as a FHIR source sends it and stores them in one
/// complete pull of that source, as a chart pull does. Returns the records
/// as filed, in order.
async fn pull<const N: usize>(
    store: &Store,
    scope: &Scope,
    raw: [String; N],
) -> [SourceRevision; N] {
    let records = raw.map(|raw| file_fhir_record(&scope.workspace, SOURCE, &raw, None).unwrap());
    let pull = super::sources::SourcePull {
        source_id: SOURCE.into(),
        source_label: SOURCE.into(),
        patient: SourcePatient {
            source_patient_id: "patient-1".into(),
            display_name: "Mary Louis".into(),
            birth_date: None,
            sex_or_gender: None,
            medical_record_numbers: vec![],
        },
        pulled_at: Some(1_760_000_000),
    };
    store
        .record_chart_with_receipt(&scope.workspace, &records, &pull)
        .await
        .unwrap();
    records
}

async fn run(
    store: &Store,
    model: &mut Script,
    patient: &PatientKey,
    now: DateTime<Utc>,
) -> RoundsRun {
    rounds::run(
        store,
        model,
        patient,
        now,
        &super::test_skills(),
        &super::test_binding(),
    )
    .await
}

/// What each call of the run returned, by its call id.
fn results(run: &RoundsRun) -> Vec<(&str, &ToolResult)> {
    run.events
        .iter()
        .filter_map(|event: &Event| match &event.kind {
            EventKind::ToolReturned { call_id, result } => Some((call_id.as_str(), result)),
            _ => None,
        })
        .collect()
}

/// The current markdown of a versioned file.
async fn markdown(store: &Store, patient: &PatientKey, path: &str) -> String {
    match store
        .read_memory(patient, &MemoryPath::parse(path).unwrap(), None)
        .await
        .unwrap()
    {
        FileRead::Version { markdown, .. } | FileRead::Log { markdown } => markdown,
    }
}

fn day(time: DateTime<Utc>) -> String {
    time.format("%Y-%m-%d").to_string()
}

/// The time as a log heading gives it.
fn clock(time: DateTime<Utc>) -> String {
    time.format("%H:%M:%S").to_string()
}

/// `time` in whole seconds, as memory keeps a run's start.
fn whole_second(time: DateTime<Utc>) -> DateTime<Utc> {
    DateTime::from_timestamp(time.timestamp(), 0).unwrap()
}

/// A rounds mark's line in a log day, for a run that started at `started`.
fn mark_line(started: DateTime<Utc>) -> String {
    format!(
        "## {} UTC, patient rounds finished a run that started here",
        clock(started)
    )
}

/// Waits into a later clock second, since memory keeps whole seconds.
async fn next_second() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_finished_run_saves_one_rounds_mark_that_its_log_day_shows_in_time_order() {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    let before = note(&store, &scope, "before", None).await;
    assert_eq!(store.last_rounds(&patient).await.unwrap(), None);
    next_second().await;
    let now = Utc::now();
    let mut model = Script::new([says("Nothing new.")]);
    let done = run(&store, &mut model, &patient, now).await;
    assert!(done.finished(), "{:?}", done.ending);
    let mark = store
        .last_rounds(&patient)
        .await
        .unwrap()
        .expect("a finished run saves a rounds mark");
    assert_eq!(mark.started, whole_second(now));
    assert!(mark.saved >= mark.started, "{mark:?}");
    next_second().await;
    let after = note(&store, &scope, "after", None).await;

    // The mark sits between the note saved before the run started and the
    // note saved after it.
    let log = format!("log/{}.md", day(before.time));
    assert_eq!(
        markdown(&store, &patient, &log).await,
        format!(
            "# Log {}\n\
             \n\
             ## {} UTC, note N1, physician dr-okafor\n\
             \n\
             Note before\n\
             \n\
             {}\n\
             \n\
             ## {} UTC, note N2, physician dr-okafor\n\
             \n\
             Note after\n",
            day(before.time),
            clock(before.time),
            mark_line(now),
            clock(after.time),
        )
    );

    // A run given an earlier day as its start shows on that day, which then
    // lists and reads with the mark alone. The newest mark is the one saved
    // last.
    let earlier = now - Duration::days(3);
    let mut model = Script::new([says("Nothing new.")]);
    let done = run(&store, &mut model, &patient, earlier).await;
    assert!(done.finished(), "{:?}", done.ending);
    let newest = store.last_rounds(&patient).await.unwrap().unwrap();
    assert_eq!(newest.started, whole_second(earlier));
    let earlier_log = format!("log/{}.md", day(earlier));
    let listed: Vec<String> = store
        .list_memory(&patient)
        .await
        .unwrap()
        .log_days
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(listed, [day(earlier), day(now)]);
    assert_eq!(
        markdown(&store, &patient, &earlier_log).await,
        format!("# Log {}\n\n{}\n", day(earlier), mark_line(earlier))
    );
    // Each run saved one mark.
    assert_eq!(
        markdown(&store, &patient, &log)
            .await
            .matches("patient rounds finished")
            .count(),
        1
    );
}

/// What a finished rounds run that starts at `now` opened with.
async fn opening(store: &Store, patient: &PatientKey, now: DateTime<Utc>) -> Vec<Item> {
    let mut model = Script::new([says("Nothing new.")]);
    let done = run(store, &mut model, patient, now).await;
    assert!(done.finished(), "{:?}", done.ending);
    let first = &model.requests[0];
    assert_eq!(
        first.instructions,
        rounds::recipe(&super::test_binding(), &super::test_skills()).instructions
    );
    first.items.clone()
}

/// The text of the memory message a conversation that started at `start`
/// opens with.
async fn conversation_memory(store: &Store, patient: &PatientKey, start: i64) -> Option<String> {
    let items = store
        .memory_at_start(patient, ConversationStart(start))
        .await
        .unwrap()
        .first([]);
    match items.as_slice() {
        [] => None,
        [Item::Message { role, text }] if role == "developer" => Some(text.clone()),
        other => panic!("memory opens a conversation as one developer message: {other:?}"),
    }
}

/// A conversation that starts now, in whole seconds.
fn started_now() -> i64 {
    Utc::now().timestamp()
}

fn user(text: String) -> Item {
    Item::Message {
        role: "user".into(),
        text,
    }
}

/// The text of a developer message.
fn developer_text(item: &Item) -> &str {
    match item {
        Item::Message { role, text } if role == "developer" => text,
        other => panic!("a developer message: {other:?}"),
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_run_opens_with_its_memory_then_when_rounds_last_finished_today_and_the_notes_since() {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    store
        .edit_memory(
            &patient,
            &VersionedFile::Summary,
            &[super::memory_tests::create("- Patient with CKD.")],
        )
        .await
        .unwrap();
    next_second().await;

    // Rounds have never run, and the patient has no note. The memory
    // message is the one a conversation starting then gets, named for the
    // run.
    let first = Utc::now();
    let items = opening(&store, &patient, first).await;
    let chat = conversation_memory(&store, &patient, first.timestamp())
        .await
        .unwrap();
    assert_eq!(
        items,
        [
            Item::Message {
                role: "developer".into(),
                text: chat.replacen(
                    "This patient's memory as this conversation started.",
                    "This patient's memory as this run started.",
                    1
                ),
            },
            user(format!(
                "Patient rounds have not run for this patient.\n\
                 Today is {}.\n\
                 No notes are saved for this patient.",
                day(first)
            )),
        ]
    );
    assert!(
        developer_text(&items[0]).starts_with(
            "This patient's memory as this run started. list_patient_memory lists every file, and read_patient_memory reads one.\n\n"
        ),
        "{items:?}"
    );
    let finished = store.last_rounds(&patient).await.unwrap().unwrap();

    // Two notes after that run started: the next run counts them by day,
    // and its memory shows them.
    let a = note(&store, &scope, "a", None).await;
    note(&store, &scope, "b", None).await;
    next_second().await;
    let second = Utc::now();
    let items = opening(&store, &patient, second).await;
    assert_eq!(
        items[1],
        user(format!(
            "Patient rounds last finished on {}.\n\
             Today is {}.\n\
             Notes saved since that run started:\n\
             - log/{}.md: 2 notes",
            day(finished.saved),
            day(second),
            day(a.time)
        ))
    );
    let memory = developer_text(&items[0]);
    for held in ["Note a", "Note b"] {
        assert!(memory.contains(held), "{held}: {memory}");
    }
    assert_eq!(
        Some(memory.replacen("as this run started.", "as this conversation started.", 1)),
        conversation_memory(&store, &patient, second.timestamp()).await
    );

    // Nothing since the second run.
    next_second().await;
    let third = Utc::now();
    let finished = store.last_rounds(&patient).await.unwrap().unwrap();
    let items = opening(&store, &patient, third).await;
    assert_eq!(
        items[1],
        user(format!(
            "Patient rounds last finished on {}.\n\
             Today is {}.\n\
             No notes were saved since that run started.",
            day(finished.saved),
            day(third)
        ))
    );
    assert!(!developer_text(&items[0]).contains("Note a"), "{items:?}");

    // A patient rounds have never run for lists every log day.
    let other = super::memory_tests::patient();
    let only = note(&store, &other, "only", None).await;
    next_second().await;
    let now = Utc::now();
    let items = opening(&store, &key(&other), now).await;
    assert_eq!(
        items[1],
        user(format!(
            "Patient rounds have not run for this patient.\n\
             Today is {}.\n\
             Notes saved for this patient:\n\
             - log/{}.md: 1 note",
            day(now),
            day(only.time)
        ))
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_conversation_after_a_finished_run_starts_with_only_the_notes_saved_since_it_started() {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    store
        .edit_memory(
            &patient,
            &VersionedFile::Summary,
            &[super::memory_tests::create("- Patient with CKD.")],
        )
        .await
        .unwrap();
    let old = note(&store, &scope, "old", None).await;
    next_second().await;
    opening(&store, &patient, Utc::now()).await;
    next_second().await;

    // The run saw the old note, and no note is newer: the message has the
    // summary and no notes part.
    assert_eq!(
        conversation_memory(&store, &patient, started_now()).await,
        Some(format!(
            "This patient's memory as this conversation started. list_patient_memory lists every file, and read_patient_memory reads one.\n\
             \n\
             summary.md, version 1, saved {}:\n\
             - Patient with CKD.",
            day(old.time)
        ))
    );

    // A note saved after the run is shown, and the old one still is not.
    note(&store, &scope, "new", None).await;
    next_second().await;
    let text = conversation_memory(&store, &patient, started_now())
        .await
        .unwrap();
    assert!(
        text.contains("Notes saved since the last finished patient rounds started, newest first:"),
        "{text}"
    );
    assert!(text.contains("Note new"), "{text}");
    assert!(!text.contains("Note old"), "{text}");

    // A patient with only notes rounds already saw gets no message.
    let other = super::memory_tests::patient();
    note(&store, &other, "seen", None).await;
    next_second().await;
    opening(&store, &key(&other), Utc::now()).await;
    next_second().await;
    assert_eq!(
        conversation_memory(&store, &key(&other), started_now()).await,
        None
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_conversation_that_started_before_a_run_finished_keeps_its_message() {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    note(&store, &scope, "old", None).await;
    next_second().await;
    let start = started_now();
    let first_request = conversation_memory(&store, &patient, start).await.unwrap();
    assert!(first_request.contains("Note old"), "{first_request}");
    next_second().await;
    opening(&store, &patient, Utc::now()).await;
    assert!(store.last_rounds(&patient).await.unwrap().is_some());

    // The mark was saved after the conversation started, so a later request
    // sends the same text.
    assert_eq!(
        conversation_memory(&store, &patient, start).await,
        Some(first_request)
    );
    // A conversation that starts after the run gets no notes.
    next_second().await;
    assert_eq!(
        conversation_memory(&store, &patient, started_now()).await,
        None
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_note_saved_while_a_run_is_under_way_is_new_to_the_next_conversation_and_the_next_rounds()
{
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    let old = note(&store, &scope, "old", None).await;
    next_second().await;
    let started = Utc::now();
    // Saved after the run's start and before it finishes.
    let during = note(&store, &scope, "during", None).await;
    let items = opening(&store, &patient, started).await;
    assert_eq!(
        items[1],
        user(format!(
            "Patient rounds have not run for this patient.\n\
             Today is {}.\n\
             Notes saved for this patient:\n\
             - log/{}.md: 1 note",
            day(started),
            day(old.time)
        ))
    );
    let memory = developer_text(&items[0]);
    assert!(memory.contains("Note old"), "{memory}");
    assert!(!memory.contains("Note during"), "{memory}");
    let finished = store.last_rounds(&patient).await.unwrap().unwrap();
    next_second().await;

    let text = conversation_memory(&store, &patient, started_now())
        .await
        .unwrap();
    assert!(text.contains("Note during"), "{text}");
    assert!(!text.contains("Note old"), "{text}");

    let next = Utc::now();
    let items = opening(&store, &patient, next).await;
    assert_eq!(
        items[1],
        user(format!(
            "Patient rounds last finished on {}.\n\
             Today is {}.\n\
             Notes saved since that run started:\n\
             - log/{}.md: 1 note",
            day(finished.saved),
            day(next),
            day(during.time)
        ))
    );
    let memory = developer_text(&items[0]);
    assert!(memory.contains("Note during"), "{memory}");
    assert!(!memory.contains("Note old"), "{memory}");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_run_that_folds_notes_into_a_page_the_direction_and_the_summary_ends_finished() {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    let [creatinine] = pull(&store, &scope, [lab("cr-1", "1.9")]).await;
    let quoted = note(&store, &scope, "quoted", Some("stop the lisinopril")).await;
    let plain = note(&store, &scope, "plain", None).await;
    let log = format!("log/{}.md", day(quoted.time));
    let line = "Lisinopril stopped while creatinine rose [N1]";
    let account = "Added the kidney page, the lisinopril direction and the summary.";
    let mut model = Script::new([
        calls([
            ("list", "list_patient_memory", json!({})),
            (
                "log",
                "read_patient_memory",
                json!({"path": log, "version": null}),
            ),
        ]),
        calls([(
            "chart",
            crate::physician_sources::READ_TOOL,
            json!({
                "references": [{
                    "candidate_id": creatinine.candidate_id,
                    "revision": creatinine.revision,
                    "cursor": null
                }],
                "max_result_bytes": 4096
            }),
        )]),
        calls([(
            "page",
            "edit_patient_memory",
            json!({"path": "pages/ckd.md", "edits": [
                {"create": {"text": format!("---\nname: Chronic kidney disease\n---\n\n- {line}\n")}},
                // A part of the line, named after it is made.
                {"replace": {"old_text": "creatinine rose", "text": "creatinine rose to 1.9"}}
            ]}),
        )]),
        calls([(
            "direction",
            "edit_patient_memory",
            json!({"path": "direction.md", "edits": [
                {"create": {"text": "- Stop the lisinopril. [N1]"}}
            ]}),
        )]),
        calls([(
            "summary",
            "edit_patient_memory",
            json!({"path": "summary.md", "edits": [
                {"append": {"text": "- Lisinopril stopped; see [[ckd]]."}}
            ]}),
        )]),
        says(account),
    ]);
    let now = quoted.time + Duration::days(1);
    let done = run(&store, &mut model, &patient, now).await;

    let Ending::Finished { account: given } = &done.ending else {
        panic!("{:?}", done.ending);
    };
    assert_eq!(given, account);
    let returned = results(&done);
    let mut names: Vec<&str> = returned.iter().map(|(id, _)| *id).collect();
    // The first round's two reads run at once, so either may be saved first;
    // each later round has one call, so their order is the script's.
    names[..2].sort_unstable();
    assert_eq!(
        names,
        ["list", "log", "chart", "page", "direction", "summary"]
    );
    for (id, result) in &returned {
        assert!(
            matches!(result, ToolResult::Original(_)),
            "{id}: {result:?}"
        );
    }
    let original = |call: &str| {
        let Some((_, ToolResult::Original(text))) = returned.iter().find(|(id, _)| *id == call)
        else {
            panic!("{call} returned no original result")
        };
        text.clone()
    };
    let log_read = original("log");
    assert!(log_read.contains("note N2"), "{log_read}");
    assert_eq!(plain.number.to_string(), "N2");
    let chart_read = original("chart");
    assert!(chart_read.contains("Creatinine"), "{chart_read}");
    let usage = done.usage();
    assert_eq!(usage.calls, 6);
    assert_eq!(usage.known_tokens, 6 * 15);
    assert_eq!(usage.unknown_usage, 0);

    let page = markdown(&store, &patient, "pages/ckd.md").await;
    assert_eq!(
        page,
        "---\nname: Chronic kidney disease\n---\n\n- Lisinopril stopped while creatinine rose to 1.9 [N1]\n"
    );
    let direction = markdown(&store, &patient, "direction.md").await;
    assert_eq!(direction, "- Stop the lisinopril. [N1]");
    let summary = markdown(&store, &patient, "summary.md").await;
    assert_eq!(summary, "- Lisinopril stopped; see [[ckd]].");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_run_a_limit_stops_ends_not_finished_and_keeps_what_it_saved() {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    let seen = note(&store, &scope, "seen", None).await;
    let mut write = calls([(
        "page",
        "edit_patient_memory",
        json!({"path": "person.md", "edits": [
            {"append": {"text": "- Lives with her daughter [N1]"}}
        ]}),
    )]);
    // The first request already spends the run's whole token allowance, so
    // the next request is the last one the run may make.
    write.usage = usage(2_000_000);
    let mut model = Script::new([write, says("I wrote the person page and stopped.")]);
    let now = seen.time + Duration::days(1);
    let done = run(&store, &mut model, &patient, now).await;

    assert!(!done.finished());
    let Ending::Limit { backstops, text } = &done.ending else {
        panic!("{:?}", done.ending);
    };
    assert_eq!(backstops, &[LimitBackstop::Tokens]);
    assert_eq!(text, "I wrote the person page and stopped.");
    assert!(
        markdown(&store, &patient, "person.md")
            .await
            .contains("Lives with her daughter"),
        "the page the run saved before the limit stays saved"
    );
    assert_eq!(
        store.last_rounds(&patient).await.unwrap(),
        None,
        "a run a limit stops saves no rounds mark"
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_run_whose_provider_fails_after_an_edit_keeps_the_edit_and_the_next_run_sees_the_same_notes()
 {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    let seen = note(&store, &scope, "seen", None).await;
    // The model edits the person page, and the provider fails on the next
    // request: the script has no turn left.
    let mut model = Script::new([calls([(
        "page",
        "edit_patient_memory",
        json!({"path": "person.md", "edits": [
            {"append": {"text": "- Lives with her daughter [N1]"}}
        ]}),
    )])]);
    let now = seen.time + Duration::days(1);
    let done = run(&store, &mut model, &patient, now).await;

    assert!(!done.finished());
    assert!(
        matches!(done.ending, Ending::Failed(_)),
        "{:?}",
        done.ending
    );
    assert_eq!(
        markdown(&store, &patient, "person.md").await,
        "- Lives with her daughter [N1]",
        "the page the run saved before the failure stays saved"
    );
    assert_eq!(
        store.last_rounds(&patient).await.unwrap(),
        None,
        "a run that fails saves no rounds mark"
    );

    // The next run opens with the same note as new.
    let next = now + Duration::seconds(1);
    let items = opening(&store, &patient, next).await;
    assert_eq!(
        items[1],
        user(format!(
            "Patient rounds have not run for this patient.\n\
             Today is {}.\n\
             Notes saved for this patient:\n\
             - log/{}.md: 1 note",
            day(next),
            day(seen.time)
        ))
    );
    let memory = developer_text(&items[0]);
    assert!(memory.contains("Note seen"), "{memory}");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_run_with_nothing_new_changes_no_file_but_logs_its_mark_and_cannot_add_a_note() {
    let store = store().await;
    let scope = patient();
    let patient = key(&scope);
    let seen = note(&store, &scope, "seen", None).await;
    store
        .edit_memory(
            &patient,
            &VersionedFile::Summary,
            &[super::memory_tests::create("- Patient with CKD.")],
        )
        .await
        .unwrap();
    let log = format!("log/{}.md", day(seen.time));
    let listed = store.list_memory(&patient).await.unwrap();
    let logged = markdown(&store, &patient, &log).await;
    let mut model = Script::new([
        calls([
            ("list", "list_patient_memory", json!({})),
            (
                "note",
                "add_patient_memory_note",
                json!({"text": "Rounds ran", "quote": null}),
            ),
        ]),
        says("Nothing new since the last rounds; no file changed."),
    ]);
    let now = seen.time + Duration::seconds(1);
    let done = run(&store, &mut model, &patient, now).await;

    assert!(done.finished(), "{:?}", done.ending);
    let returned = results(&done);
    assert!(matches!(returned[0], ("list", ToolResult::Original(_))));
    let ("note", ToolResult::Refused(refusal)) = returned[1] else {
        panic!("{returned:?}");
    };
    assert!(
        refusal.reason().contains("add_patient_memory_note"),
        "{refusal:?}"
    );
    // The files stand as they were; the log day gains only the run's mark.
    assert_eq!(store.list_memory(&patient).await.unwrap(), listed);
    assert_eq!(
        markdown(&store, &patient, &log).await,
        format!("{logged}\n{}\n", mark_line(now))
    );
}
