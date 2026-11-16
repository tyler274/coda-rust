//! Postgres proof that patient memory keeps each note as the store stamped
//! it, keeps each file as its text with every version, saves one version for
//! a call where an edit applied and none otherwise, applies two writes at
//! once one after the other, takes a deleted file out of the list and the
//! search and brings it back with a create, finds words in every file that
//! exists, and never crosses a practice or a patient.
use super::{Scope, Store};
use crate::runtime::memory::{
    Edit, EditRefusal, FileRead, Hit, Listed, Listing, MemoryError, MemoryPath, MemoryRefusal,
    NotApplied, Note, NoteDraft, NoteId, NoteOrigin, OneLine, PageId, PatientKey, VersionedFile,
    Words, Written,
};

pub(super) async fn store() -> Store {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store
}

/// A patient of its own: memory and the chart are both per patient, so each
/// test gets a fresh practice and workspace.
pub(super) fn patient() -> Scope {
    Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "dr-okafor".into(),
        workspace: uuid::Uuid::new_v4().to_string(),
        chat: "chat".into(),
    }
}

pub(super) fn key(scope: &Scope) -> PatientKey {
    PatientKey::try_from(scope).unwrap()
}

fn origin(call: &str) -> NoteOrigin {
    NoteOrigin {
        work: "work".into(),
        call: call.into(),
        message: "message-1".into(),
    }
}

fn draft(text: &str, quote: Option<&str>) -> NoteDraft {
    NoteDraft {
        text: text.into(),
        quote: quote.map(one),
    }
}

pub(super) async fn note(store: &Store, scope: &Scope, call: &str, quote: Option<&str>) -> Note {
    store
        .append_note(scope, &origin(call), &draft(&format!("Note {call}"), quote))
        .await
        .unwrap()
}

fn one(text: &str) -> OneLine {
    OneLine::parse(text).unwrap()
}

/// An edit as given, none left out at the parse.
pub(super) type Given = Result<Edit, EditRefusal>;

fn words(text: &str) -> Words {
    Words::parse(text).unwrap()
}

/// A create of the file with `text`, as given.
pub(super) fn create(text: &str) -> Given {
    Ok(Edit::Create { text: words(text) })
}

/// An append of `text`, as given.
fn append(text: &str) -> Given {
    Ok(Edit::Append { text: words(text) })
}

/// A replace of the one place `old_text` is, as given.
fn replace(old_text: &str, text: &str) -> Given {
    Ok(Edit::Replace {
        old_text: words(old_text),
        text: text.into(),
    })
}

fn left_out(edit: usize, reason: EditRefusal) -> NotApplied {
    NotApplied { edit, reason }
}

fn refusal<T: std::fmt::Debug>(result: Result<T, MemoryError>) -> MemoryRefusal {
    match result {
        Err(MemoryError::Refused(refusal)) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    }
}

fn markdown(read: FileRead) -> String {
    match read {
        FileRead::Version { markdown, .. } | FileRead::Log { markdown } => markdown,
    }
}

async fn read(store: &Store, key: &PatientKey, path: &MemoryPath) -> String {
    markdown(store.read_memory(key, path, None).await.unwrap())
}

/// The UTC date of the note's stamp, as its log file names it.
fn day(saved: &Note) -> String {
    saved.time.format("%Y-%m-%d").to_string()
}

fn log(saved: &Note) -> MemoryPath {
    MemoryPath::parse(&format!("log/{}.md", day(saved))).unwrap()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_note_keeps_its_stamp_across_a_new_connection_and_a_repeat_adds_nothing() {
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    let saved = store
        .append_note(&scope, &origin("call-1"), &draft("Asked about cough", None))
        .await
        .unwrap();
    // The work and call are the note's key; its number is its place.
    assert_eq!(saved.id, NoteId::from("work:call-1".to_owned()));
    assert_eq!(saved.number.to_string(), "N1");
    // The physician and chat are the scope's; the message is the run's.
    assert_eq!(
        (
            saved.physician.as_str(),
            saved.chat.as_str(),
            saved.message.as_str()
        ),
        ("dr-okafor", "chat", "message-1")
    );
    assert_eq!(saved.physician_words, None);

    let fresh = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let heading = format!(
        "## {} UTC, note N1, physician dr-okafor",
        saved.time.format("%H:%M:%S")
    );
    assert_eq!(
        read(&fresh, &key, &log(&saved)).await,
        format!("# Log {}\n\n{heading}\n\nAsked about cough\n", day(&saved))
    );
    let days: Vec<String> = fresh
        .list_memory(&key)
        .await
        .unwrap()
        .log_days
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(days, [day(&saved)]);

    // A retried call returns the first call's note, stamp and all.
    let repeated = fresh
        .append_note(&scope, &origin("call-1"), &draft("Asked about cough", None))
        .await
        .unwrap();
    assert_eq!(repeated, saved);
    // The same call with other text is not a repeat, and saves nothing.
    let collision = fresh
        .append_note(&scope, &origin("call-1"), &draft("Asked about fever", None))
        .await;
    assert!(
        matches!(collision, Err(MemoryError::Database(_))),
        "{collision:?}"
    );
    assert_eq!(
        read(&fresh, &key, &log(&saved))
            .await
            .matches(&heading)
            .count(),
        1
    );
}

/// A note keeps the quote the model gave, in its one-line form, whether or
/// not the physician's message holds those words; nothing compares them.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_note_keeps_the_quote_it_was_given() {
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    // The message the note comes from said "stop the lisinopril"; the
    // quotes need not be its words.
    for (call, quote, words) in [
        ("verbatim", " stop the\nlisinopril ", "stop the lisinopril"),
        ("reworded", "stop lisinopril", "stop lisinopril"),
        (
            "elsewhere",
            "she\u{2019}s off the metformin",
            "she\u{2019}s off the metformin",
        ),
    ] {
        let saved = note(&store, &scope, call, Some(quote)).await;
        assert_eq!(saved.message, "message-1", "{call}");
        assert_eq!(
            saved.physician_words.as_ref().map(OneLine::as_str),
            Some(words),
            "{call}"
        );
        let day = read(&store, &key, &log(&saved)).await;
        assert!(
            day.contains(&format!(
                "Quoted: {}\n\nNote {call}\n",
                serde_json::Value::String(words.into())
            )),
            "{day}"
        );
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_write_applies_to_the_current_file_two_at_once_both_land_and_earlier_versions_stay_readable()
 {
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    let (first, second) = ("- Creatinine 1.8 [chart]", "- Creatinine 1.9 [chart]");
    let saved = store
        .edit_memory(
            &key,
            &VersionedFile::Person,
            &[create(&format!("## Kidneys\n\n{first}"))],
        )
        .await
        .unwrap();
    assert_eq!(saved.version, Some(1));
    let saved = store
        .edit_memory(&key, &VersionedFile::Person, &[replace(first, second)])
        .await
        .unwrap();
    assert_eq!(saved.version, Some(2));
    assert_eq!(
        store
            .read_memory(&key, &MemoryPath::Person, None)
            .await
            .unwrap(),
        FileRead::Version {
            version: 2,
            markdown: format!("## Kidneys\n\n{second}"),
        }
    );
    assert_eq!(
        markdown_at(&store, &key, &MemoryPath::Person, 1).await,
        format!("## Kidneys\n\n{first}")
    );
    assert_eq!(
        refusal(store.read_memory(&key, &MemoryPath::Person, Some(3)).await),
        MemoryRefusal::NoVersion {
            path: MemoryPath::Person,
            version: 3,
            current: 2
        }
    );

    // Two writers for the patient at the same moment, each with its own
    // edit list, both save, one after the other, each on the text the other
    // left. The pair runs several times, so later pairs start on
    // connections the pool already holds.
    let lines = |writer: &str, round: u64| -> Vec<Given> {
        [
            format!("- From {writer} {round}"),
            format!("- Also from {writer} {round}"),
        ]
        .iter()
        .map(|text| append(text))
        .collect()
    };
    let rounds = 5;
    for round in 0..rounds {
        let (from_a, from_b) = (lines("a", round), lines("b", round));
        let (a, b) = tokio::join!(
            store.edit_memory(&key, &VersionedFile::Person, &from_a),
            store.edit_memory(&key, &VersionedFile::Person, &from_b),
        );
        let (a, b) = (a.unwrap(), b.unwrap());
        let mut saved = [a.version, b.version];
        saved.sort();
        assert_eq!(saved, [Some(3 + 2 * round), Some(4 + 2 * round)]);
        assert_eq!((a.not_applied, b.not_applied), (vec![], vec![]));
    }
    let FileRead::Version { version, markdown } = store
        .read_memory(&key, &MemoryPath::Person, None)
        .await
        .unwrap()
    else {
        panic!("a page reads as a version");
    };
    assert_eq!(version, 2 + 2 * rounds);
    assert!(
        markdown.starts_with(&format!("## Kidneys\n\n{second}\n")),
        "{markdown}"
    );
    for round in 0..rounds {
        for writer in ["a", "b"] {
            // Each writer's two lines stay together, in its order.
            assert!(
                markdown.contains(&format!(
                    "\n- From {writer} {round}\n- Also from {writer} {round}"
                )),
                "{writer} {round}: {markdown}"
            );
        }
    }
}

async fn markdown_at(store: &Store, key: &PatientKey, path: &MemoryPath, version: u64) -> String {
    match store.read_memory(key, path, Some(version)).await.unwrap() {
        FileRead::Version {
            version: read,
            markdown,
        } => {
            assert_eq!(read, version);
            markdown
        }
        FileRead::Log { .. } => panic!("a versioned file reads as a version"),
    }
}

/// Each call that applies an edit saves one version, however many edits
/// applied, and gives the size of the text it saved in characters, not
/// bytes. A call where no edit applies saves nothing and gives the current
/// version and size with each reason.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_call_saves_one_version_when_an_edit_applied_and_none_when_none_did() {
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    let cough = "- No ACE inhibitor \u{2014} it caused a cough.";
    let text = format!("- Patient with CKD stage 3.\n{cough}\n");
    let written = |version: u64, text: &str, not_applied: Vec<NotApplied>| Written {
        version: Some(version),
        characters: text.chars().count(),
        not_applied,
    };
    assert_eq!(
        store
            .edit_memory(&key, &VersionedFile::Summary, &[create(&text)])
            .await
            .unwrap(),
        written(1, &text, vec![])
    );
    assert_ne!(text.chars().count(), text.len());
    // Two edits that apply save one version.
    let second = "- Patient with CKD stage 3b, see [[ckd]].\n";
    assert_eq!(
        store
            .edit_memory(
                &key,
                &VersionedFile::Summary,
                &[
                    replace("stage 3.", "stage 3b, see [[ckd]]."),
                    replace(&format!("{cough}\n"), ""),
                ],
            )
            .await
            .unwrap(),
        written(2, second, vec![])
    );
    // An edit left out costs only itself.
    let third = format!("{second}- Lives alone.");
    assert_eq!(
        store
            .edit_memory(
                &key,
                &VersionedFile::Summary,
                &[append("- Lives alone."), replace("stage 3.", "stage 4.")],
            )
            .await
            .unwrap(),
        written(3, &third, vec![left_out(1, EditRefusal::NotFound)])
    );
    // No edit applies: no version is saved.
    assert_eq!(
        store
            .edit_memory(
                &key,
                &VersionedFile::Summary,
                &[replace("stage 3.", "stage 4."), create("- Other")],
            )
            .await
            .unwrap(),
        written(
            3,
            &third,
            vec![
                left_out(0, EditRefusal::NotFound),
                left_out(1, EditRefusal::Exists)
            ]
        )
    );
    assert_eq!(
        store
            .read_memory(&key, &MemoryPath::Summary, None)
            .await
            .unwrap(),
        FileRead::Version {
            version: 3,
            markdown: third,
        }
    );
    // A file with no version stays without one when nothing applies.
    let fresh = self::key(&patient());
    assert_eq!(
        store
            .edit_memory(&fresh, &VersionedFile::Summary, &[replace("Lives", "Has")])
            .await
            .unwrap(),
        Written {
            version: None,
            characters: 0,
            not_applied: vec![left_out(0, EditRefusal::NoFile(MemoryPath::Summary))],
        }
    );
    assert_eq!(store.list_memory(&fresh).await.unwrap(), Listing::default());
}

/// A delete takes the file out of the list and the search, and its versions
/// still read. A second create of a file that exists is left out; a create
/// after the delete brings it back at the next version.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_deleted_file_leaves_the_list_and_search_keeps_its_versions_and_comes_back_with_a_create()
{
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    let gout = VersionedFile::Page(PageId::parse("gout").unwrap());
    let path = MemoryPath::from(gout.clone());
    let page = "---\nname: Gout\n---\n\n## Flares\n\n- Flare in the left toe\n";
    let created = store
        .edit_memory(&key, &gout, &[create(page)])
        .await
        .unwrap();
    assert_eq!(created.version, Some(1));
    // The same create again is left out.
    let repeated = store
        .edit_memory(&key, &gout, &[create(page)])
        .await
        .unwrap();
    assert_eq!(
        repeated,
        Written {
            version: Some(1),
            characters: page.chars().count(),
            not_applied: vec![left_out(0, EditRefusal::Exists)],
        }
    );
    let listed = |version, text: &str| Listed {
        version,
        characters: text.chars().count(),
    };
    assert_eq!(
        store.list_memory(&key).await.unwrap().pages,
        [(PageId::parse("gout").unwrap(), listed(1, page))].into()
    );
    let deleted = store
        .edit_memory(&key, &gout, &[Ok(Edit::Delete)])
        .await
        .unwrap();
    assert_eq!(
        (deleted.version, deleted.characters, deleted.not_applied),
        (Some(2), 0, vec![])
    );
    assert_eq!(store.list_memory(&key).await.unwrap(), Listing::default());
    assert!(store.search_memory(&key, "gout").await.unwrap().is_empty());
    // A read of the file is refused as an edit is: there is no file. Its
    // versions still read by number, the delete's own as no text.
    assert_eq!(
        refusal(store.read_memory(&key, &path, None).await),
        MemoryRefusal::NoFile(path.clone())
    );
    assert_eq!(markdown_at(&store, &key, &path, 2).await, "");
    assert_eq!(markdown_at(&store, &key, &path, 1).await, page);
    // A deleted file takes no replace and no second delete.
    let gone = store
        .edit_memory(&key, &gout, &[replace("Gout", "Gouty"), Ok(Edit::Delete)])
        .await
        .unwrap();
    assert_eq!(gone.version, Some(2));
    assert_eq!(
        gone.not_applied
            .iter()
            .map(|left_out| left_out.reason.to_string())
            .collect::<Vec<_>>(),
        [
            "there is no file pages/gout.md; create makes it",
            "there is no file pages/gout.md; create makes it"
        ]
    );
    // A create brings it back at the next version.
    let back = store
        .edit_memory(&key, &gout, &[create("- Uric acid 9.1")])
        .await
        .unwrap();
    assert_eq!(back.version, Some(3));
    assert_eq!(
        store.list_memory(&key).await.unwrap().pages,
        [(PageId::parse("gout").unwrap(), listed(3, "- Uric acid 9.1"))].into()
    );
    let hits = store.search_memory(&key, "uric").await.unwrap();
    assert_eq!(
        hits,
        [Hit {
            path: path.clone(),
            line: 1,
            text: "- Uric acid 9.1".into(),
        }]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn search_matches_every_word_of_a_line_in_each_file_that_exists_and_the_log() {
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    // A note with text over two lines comes first, so every later line's
    // number counts its lines.
    store
        .append_note(
            &scope,
            &origin("family"),
            &draft(
                "Spoke with her daughter.\nShe keeps the potassium log.",
                None,
            ),
        )
        .await
        .unwrap();
    let saved = store
        .append_note(
            &scope,
            &origin("recheck"),
            &draft("Physician asked to recheck creatinine", None),
        )
        .await
        .unwrap();
    let summary = "Patient with CKD stage 3.\nSee [[ckd]].\n";
    store
        .edit_memory(&key, &VersionedFile::Summary, &[create(summary)])
        .await
        .unwrap();
    let ckd = PageId::parse("ckd").unwrap();
    let page = "---\nname: Chronic kidney disease\nother_names: [renal insufficiency]\n---\n\n- Creatinine rose to 1.9 after losartan\n";
    store
        .edit_memory(&key, &VersionedFile::Page(ckd.clone()), &[create(page)])
        .await
        .unwrap();

    let listing = store.list_memory(&key).await.unwrap();
    let listed = |text: &str| Listed {
        version: 1,
        characters: text.chars().count(),
    };
    assert_eq!(
        (listing.summary, listing.direction, listing.person),
        (Some(listed(summary)), None, None)
    );
    assert_eq!(listing.pages, [(ckd.clone(), listed(page))].into());
    assert_eq!(listing.log_days.len(), 1);

    let page_path = MemoryPath::Page(ckd);
    let paths = |hits: &[Hit]| hits.iter().map(|hit| hit.path.clone()).collect::<Vec<_>>();
    let creatinine = store.search_memory(&key, "CREATININE").await.unwrap();
    assert_eq!(paths(&creatinine), [page_path.clone(), log(&saved)]);
    // Each hit names the line where the file has it.
    for hit in &creatinine {
        let file = read(&store, &key, &hit.path).await;
        assert_eq!(
            file.lines().nth(hit.line - 1),
            Some(hit.text.as_str()),
            "{hit:?}"
        );
    }
    let potassium = store.search_memory(&key, "potassium").await.unwrap();
    let [hit] = &potassium[..] else {
        panic!("one line holds potassium: {potassium:?}");
    };
    let file = read(&store, &key, &hit.path).await;
    assert_eq!(
        file.lines().nth(hit.line - 1),
        Some("She keeps the potassium log.")
    );
    let both = store.search_memory(&key, "losartan  rose").await.unwrap();
    assert_eq!(paths(&both), std::slice::from_ref(&page_path));
    assert!(
        store
            .search_memory(&key, "losartan creatinine 2.0")
            .await
            .unwrap()
            .is_empty()
    );
    // Front matter the model wrote is lines of the file like any other.
    let other_name = store
        .search_memory(&key, "Renal INSUFFICIENCY")
        .await
        .unwrap();
    assert_eq!(
        other_name,
        [Hit {
            path: page_path.clone(),
            line: 3,
            text: "other_names: [renal insufficiency]".into(),
        }]
    );
    assert_eq!(
        paths(&store.search_memory(&key, "ckd").await.unwrap()),
        [MemoryPath::Summary, MemoryPath::Summary]
    );
    assert_eq!(
        refusal(store.search_memory(&key, " \n ").await),
        MemoryRefusal::EmptyQuery
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn another_practice_or_patient_reads_and_writes_nothing_of_this_patient() {
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    let saved = note(&store, &scope, "ours", Some("stop the lisinopril")).await;
    store
        .edit_memory(
            &key,
            &VersionedFile::Summary,
            &[create("Stop the lisinopril.")],
        )
        .await
        .unwrap();
    store
        .edit_memory(
            &key,
            &VersionedFile::Direction,
            &[create("- Stop the lisinopril. [N1]")],
        )
        .await
        .unwrap();
    let ours = store.list_memory(&key).await.unwrap();

    let other_practice = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        ..scope.clone()
    };
    let other_patient = Scope {
        workspace: uuid::Uuid::new_v4().to_string(),
        ..scope.clone()
    };
    for stranger in [&other_practice, &other_patient] {
        let theirs = self::key(stranger);
        assert_eq!(
            store.list_memory(&theirs).await.unwrap(),
            Listing::default()
        );
        for path in [MemoryPath::Summary, MemoryPath::Direction, log(&saved)] {
            assert_eq!(
                refusal(store.read_memory(&theirs, &path, None).await),
                MemoryRefusal::NoFile(path)
            );
        }
        assert!(
            store
                .search_memory(&theirs, "lisinopril")
                .await
                .unwrap()
                .is_empty()
        );
        // A note in its own scope is its own note, even with the same call.
        let own = note(&store, stranger, "ours", None).await;
        assert_eq!(own.id, saved.id);
        assert_eq!(own.physician_words, None);
        // Its own summary is its own file: a replace finds none, and a
        // create makes the first version.
        let edited = store
            .edit_memory(
                &theirs,
                &VersionedFile::Summary,
                &[
                    replace("Stop the lisinopril.", "Someone else"),
                    create("Someone else"),
                ],
            )
            .await
            .unwrap();
        assert_eq!(edited.version, Some(1));
        assert_eq!(
            edited.not_applied,
            [left_out(0, EditRefusal::NoFile(MemoryPath::Summary))]
        );
    }
    assert_eq!(store.list_memory(&key).await.unwrap(), ours);
    assert_eq!(
        read(&store, &key, &MemoryPath::Summary).await,
        "Stop the lisinopril."
    );
    assert_eq!(
        read(&store, &key, &log(&saved))
            .await
            .matches("## ")
            .count(),
        1
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn notes_added_at_the_same_time_each_take_a_place_and_one_call_saves_one_note() {
    let store = store().await;
    let scope = patient();
    let key = key(&scope);
    let add = |call: &'static str| {
        let (store, scope) = (store.clone(), scope.clone());
        async move {
            store
                .append_note(&scope, &origin(call), &draft(&format!("Note {call}"), None))
                .await
        }
    };
    let (first, second) = tokio::join!(add("first"), add("second"));
    // Positions are unique for the patient, so both saving means each took
    // its own place.
    let (first, second) = (first.unwrap(), second.unwrap());
    let (same, again) = tokio::join!(add("same"), add("same"));
    let (same, again) = (same.unwrap(), again.unwrap());
    assert_eq!(same, again);

    let mut days = vec![log(&first), log(&second), log(&same)];
    days.sort();
    days.dedup();
    let mut logged = String::new();
    for day in &days {
        logged.push_str(&read(&store, &key, day).await);
    }
    // Three notes, each with its own number.
    for number in ["N1", "N2", "N3"] {
        assert_eq!(
            logged
                .matches(&format!("note {number}, physician dr-okafor\n"))
                .count(),
            1,
            "{logged}"
        );
    }
    assert!(!logged.contains("note N4"), "{logged}");
}
