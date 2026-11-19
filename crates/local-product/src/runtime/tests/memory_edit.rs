//! Postgres proof that the one edit tool changes a memory file's text
//! through the real store: each of the summary, the direction, the person
//! page and a subject page takes the same edits and gives the same result
//! and reads back as written; an edit left out names its reason and the
//! others save; the line as the model read it, its bullet and all, names
//! its place, and a replace names its text exactly or not at all; a path
//! that names no versioned file is refused. A chat run runs the tool for
//! its own patient.
use harness::engine::ToolOutcome;
use sea_orm::DbErr;
use serde_json::{Value, json};

use super::memory_tests::{key, note, patient, store};
use super::{Scope, Store};
use crate::runtime::memory::edit::EditTool;
use crate::runtime::memory::tools::MemoryTool;
use crate::runtime::memory::{MemoryPath, NoteOrigin};

/// The tool's name, as every job is offered it.
const EDIT: &str = "edit_patient_memory";

async fn call(store: &Store, scope: &Scope, arguments: Value) -> ToolOutcome {
    EditTool::named(EDIT)
        .expect("the edit tool")
        .execute(store, &key(scope), &arguments)
        .await
        .unwrap()
}

/// The result of one call that edits the file at `path`.
async fn edit(store: &Store, scope: &Scope, path: &str, edits: Value) -> Value {
    returned(call(store, scope, json!({"path": path, "edits": edits})).await)
}

fn returned(outcome: ToolOutcome) -> Value {
    match outcome {
        ToolOutcome::Returned(text) => serde_json::from_str(&text).unwrap(),
        other => panic!("expected a result, got {other:?}"),
    }
}

fn refused(outcome: ToolOutcome) -> String {
    match outcome {
        ToolOutcome::Refused(refusal) => refusal.reason().to_owned(),
        other => panic!("expected a refusal, got {other:?}"),
    }
}

/// The read tool's outcome for the file at `path`, at its current version.
async fn read_outcome(store: &Store, scope: &Scope, path: &str) -> ToolOutcome {
    MemoryTool::named("read_patient_memory")
        .expect("the read tool")
        .execute(
            store,
            scope,
            &json!({"path": path, "version": null}),
            || -> Result<NoteOrigin, DbErr> { panic!("a read adds no note") },
        )
        .await
        .unwrap()
}

/// The file as the read tool gives it to the model.
async fn read(store: &Store, scope: &Scope, path: &str) -> Value {
    returned(read_outcome(store, scope, path).await)
}

/// The listing as the list tool gives it to the model.
async fn list(store: &Store, scope: &Scope) -> Value {
    let outcome = MemoryTool::named("list_patient_memory")
        .expect("the list tool")
        .execute(store, scope, &json!({}), || -> Result<NoteOrigin, DbErr> {
            panic!("a list adds no note")
        })
        .await
        .unwrap();
    returned(outcome)
}

fn create(text: &str) -> Value {
    json!({"create": {"text": text}})
}

fn append(text: &str) -> Value {
    json!({"append": {"text": text}})
}

fn replace(old_text: &str, text: &str) -> Value {
    json!({"replace": {"old_text": old_text, "text": text}})
}

fn size(text: &str) -> usize {
    text.chars().count()
}

/// Each kind of file takes the same edits and gives the same result: the
/// path, the version saved, the size of its text and each edit left out.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn the_one_tool_edits_each_kind_of_file_and_gives_one_result() {
    let store = store().await;
    let scope = patient();
    let first = "# Now\n\n- Has CKD [N1]\n- Lives alone\n";
    let second = "# Now\n\n- Has CKD stage 3 [N1]\n- Lives alone\n- Recheck potassium";
    for path in ["summary.md", "direction.md", "person.md", "pages/ckd.md"] {
        assert_eq!(
            edit(&store, &scope, path, json!([create(first)])).await,
            json!({"path": path, "version": 1, "characters": size(first), "not_applied": []}),
            "{path}"
        );
        assert_eq!(
            edit(
                &store,
                &scope,
                path,
                json!([replace("CKD", "CKD stage 3"), append("- Recheck potassium")])
            )
            .await,
            json!({"path": path, "version": 2, "characters": size(second), "not_applied": []}),
            "{path}"
        );
        assert_eq!(
            read(&store, &scope, path).await,
            json!({"path": path, "version": 2, "markdown": second}),
            "{path}"
        );
    }
    // The list gives each file's version and size, and a page by its path.
    let listed = json!({"version": 2, "characters": size(second)});
    let at = |path: &str| {
        let mut file = listed.clone();
        file["path"] = json!(path);
        file
    };
    assert_eq!(
        list(&store, &scope).await,
        json!({
            "summary": at("summary.md"),
            "direction": at("direction.md"),
            "person": at("person.md"),
            "pages": [at("pages/ckd.md")],
            "log": [],
        })
    );
}

/// One mistake costs the model only that edit: text not in the file, a
/// create of a file that exists, a blank text and a malformed edit are each
/// left out with the reason, and the other edits are saved in one version.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_write_saves_the_edits_that_apply_and_names_each_edit_left_out() {
    let store = store().await;
    let scope = patient();
    let written = edit(
        &store,
        &scope,
        "pages/ckd.md",
        json!([
            create("## Kidneys\n\n- Creatinine 1.8"),
            replace("Creatinine 1.7", "Creatinine 1.9"),
            create("## Kidneys again"),
            append(" \n"),
            {"append": {"text": "- eGFR 38", "sources": ["N1"]}},
            append("- eGFR 38 [chart]")
        ]),
    )
    .await;
    assert_eq!(written["version"], 1, "{written}");
    let reasons: Vec<(u64, &str)> = written["not_applied"]
        .as_array()
        .unwrap()
        .iter()
        .map(|left_out| {
            (
                left_out["edit"].as_u64().unwrap(),
                left_out["reason"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        reasons[..3],
        [
            (1, "old_text is not in the file exactly as given"),
            (2, "the file exists; change it with replace or append"),
            (
                3,
                "Give append.text as one or more words (the text has no words)."
            ),
        ]
    );
    assert_eq!(reasons[3].0, 4);
    assert!(
        reasons[3].1.starts_with(
            "Give the edit as an object that holds one of create, replace, append or delete, with the fields each takes ("
        ) && reasons[3].1.contains("unknown field `sources`"),
        "{}",
        reasons[3].1
    );
    assert_eq!(reasons.len(), 4);
    let markdown = "## Kidneys\n\n- Creatinine 1.8\n- eGFR 38 [chart]";
    assert_eq!(written["characters"], size(markdown));
    assert_eq!(
        read(&store, &scope, "pages/ckd.md").await["markdown"],
        markdown
    );
}

/// A write where no edit applies saves no version: the result gives the
/// current version, or null for a file that has none, and why each edit
/// was left out. An empty list applies nothing either.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_write_where_no_edit_applies_saves_no_version() {
    let store = store().await;
    let scope = patient();
    assert_eq!(
        edit(
            &store,
            &scope,
            "person.md",
            json!([replace("Lives alone", "Lives with her son"), {"delete": {}}])
        )
        .await,
        json!({"path": "person.md", "version": null, "characters": 0, "not_applied": [
            {"edit": 0, "reason": "there is no file person.md; create makes it"},
            {"edit": 1, "reason": "there is no file person.md; create makes it"},
        ]})
    );
    assert_eq!(store.list_memory(&key(&scope)).await.unwrap().person, None);
    let text = "- Has a daughter\n";
    edit(&store, &scope, "person.md", json!([create(text)])).await;
    assert_eq!(
        edit(
            &store,
            &scope,
            "person.md",
            json!([replace("daughter", "daughter")])
        )
        .await,
        json!({"path": "person.md", "version": 1, "characters": size(text), "not_applied": [
            {"edit": 0, "reason": "nothing to change: the file already reads this"},
        ]})
    );
    assert_eq!(
        edit(&store, &scope, "person.md", json!([])).await["version"],
        1
    );
    assert_eq!(read(&store, &scope, "person.md").await["version"], 1);
}

/// A delete takes the file out of the list the model reads, its size is
/// none, and a read of it is refused; an append brings it back.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_delete_takes_the_file_out_of_the_list_and_an_append_brings_it_back() {
    let store = store().await;
    let scope = patient();
    // A file is stored as written: `- Flare` is 7 characters.
    assert_eq!(
        edit(&store, &scope, "pages/gout.md", json!([create("- Flare")])).await,
        json!({"path": "pages/gout.md", "version": 1, "characters": 7, "not_applied": []})
    );
    assert_eq!(
        edit(&store, &scope, "pages/gout.md", json!([{"delete": {}}])).await,
        json!({"path": "pages/gout.md", "version": 2, "characters": 0, "not_applied": []})
    );
    assert_eq!(list(&store, &scope).await["pages"], json!([]));
    assert_eq!(
        refused(read_outcome(&store, &scope, "pages/gout.md").await),
        "there is no file pages/gout.md"
    );
    assert_eq!(
        edit(
            &store,
            &scope,
            "pages/gout.md",
            json!([append("- Uric acid 9.1")])
        )
        .await,
        json!({"path": "pages/gout.md", "version": 3, "characters": size("- Uric acid 9.1"), "not_applied": []})
    );
    assert_eq!(
        list(&store, &scope).await["pages"],
        json!([{"path": "pages/gout.md", "version": 3, "characters": size("- Uric acid 9.1")}])
    );
}

/// A path that names a log day or no memory file has no file to edit: the
/// call is refused whole, and nothing is saved.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_path_to_a_log_day_or_no_memory_file_is_refused() {
    let store = store().await;
    let scope = patient();
    let saved = note(&store, &scope, "seen", None).await;
    let day = MemoryPath::Log(crate::runtime::memory::Day::of(saved.time)).to_string();
    let before = store.list_memory(&key(&scope)).await.unwrap();
    for path in [
        day.as_str(),
        "notes.md",
        "pages/ - .md",
        "log/2026-02-30.md",
    ] {
        assert_eq!(
            refused(
                call(
                    &store,
                    &scope,
                    json!({"path": path, "edits": [create("Has CKD")]}),
                )
                .await
            ),
            format!(
                "Give path as summary.md, direction.md, person.md or pages/<id>.md; {path} cannot be edited."
            )
        );
    }
    assert_eq!(store.list_memory(&key(&scope)).await.unwrap(), before);
}

/// A page path given with spaces and capitals saves under its normal form,
/// and the result and every read name that path.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_spaced_capitalised_page_id_saves_and_reads_back_under_its_normal_path() {
    let store = store().await;
    let scope = patient();
    assert_eq!(
        edit(
            &store,
            &scope,
            "pages/CKD Stage 3.md",
            json!([create("- Stage 3b")])
        )
        .await["path"],
        "pages/ckd-stage-3.md"
    );
    for spelling in [
        "pages/ckd-stage-3.md",
        "pages/CKD stage 3",
        "pages/ckd_stage_3.md",
    ] {
        let page = read(&store, &scope, spelling).await;
        assert_eq!(
            (&page["path"], &page["version"]),
            (&json!("pages/ckd-stage-3.md"), &json!(1)),
            "{spelling}"
        );
    }
    assert_eq!(
        list(&store, &scope).await["pages"][0]["path"],
        "pages/ckd-stage-3.md"
    );
    // A second write by another spelling continues the same page.
    assert_eq!(
        edit(
            &store,
            &scope,
            "pages/Ckd  Stage 3",
            json!([replace("3b", "3a")])
        )
        .await["version"],
        2
    );
}

/// A model copies the line as it reads it, with its leading `- `, and the
/// replace applies. A tag two lines share names neither, and the reason
/// gives both line numbers.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn the_line_as_read_with_its_bullet_names_its_place_and_a_shared_tag_names_both_lines() {
    let store = store().await;
    let scope = patient();
    let first = "- 2026-11-14: \u{2026} because of bilateral Achilles pain; she completed treatment with ofloxacin drops alone. [N1]";
    let page = format!("# Achilles pain\n\n{first}\n- Rounds asked to recheck. [N1]\n");
    edit(
        &store,
        &scope,
        "pages/achilles-pain.md",
        json!([create(&page)]),
    )
    .await;
    let as_read = read(&store, &scope, "pages/achilles-pain.md").await["markdown"]
        .as_str()
        .unwrap()
        .lines()
        .nth(2)
        .unwrap()
        .to_owned();
    assert_eq!(as_read, first);
    let changed = edit(
        &store,
        &scope,
        "pages/achilles-pain.md",
        json!([
            replace(
                &as_read,
                "- 2026-11-14: completed ofloxacin drops alone. [N1]"
            ),
            replace("[N1]", "[N2]")
        ]),
    )
    .await;
    let now = "# Achilles pain\n\n- 2026-11-14: completed ofloxacin drops alone. [N1]\n- Rounds asked to recheck. [N1]\n";
    assert_eq!(
        changed,
        json!({
            "path": "pages/achilles-pain.md",
            "version": 2,
            "characters": size(now),
            "not_applied": [{
                "edit": 1,
                "reason": "old_text is in the file more than once, on lines 3, 4; give more of the text so it names one place"
            }],
        })
    );
    assert_eq!(
        read(&store, &scope, "pages/achilles-pain.md").await["markdown"],
        now
    );
}

/// A replace whose old_text differs from the file only by a curly quote or
/// a dash names no text: it is left out with its reason, the call's other
/// edit saves, and the file reads back unchanged outside that edit.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_replace_that_differs_by_a_quote_or_a_dash_is_left_out_and_the_other_edit_saves() {
    let store = store().await;
    let scope = patient();
    let text = "- Daughter\u{2019}s number is on file \u{2014} call her Friday\n- Lives alone\n";
    edit(&store, &scope, "person.md", json!([create(text)])).await;
    let changed = edit(
        &store,
        &scope,
        "person.md",
        json!([
            replace("Daughter's number", "Son's number"),
            replace("on file - call", "on file, call"),
            replace("Lives alone", "Lives with her son")
        ]),
    )
    .await;
    let reason = "old_text is not in the file exactly as given";
    assert_eq!(
        changed["not_applied"],
        json!([{"edit": 0, "reason": reason}, {"edit": 1, "reason": reason}])
    );
    assert_eq!(changed["version"], 2);
    assert_eq!(
        read(&store, &scope, "person.md").await["markdown"],
        "- Daughter\u{2019}s number is on file \u{2014} call her Friday\n- Lives with her son\n"
    );
}

/// A chat run's model edits its patient's summary: the edit saves for the
/// chat's own patient and no other, and the call shows the physician no
/// step.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_chat_run_edits_its_own_patients_memory() {
    use harness::engine::{Invocation, Product};
    let store = store().await;
    let scope = patient();
    let other = patient();
    super::accept(&store, &scope, &super::delivery_input("input"), 0)
        .await
        .unwrap();
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = super::PhysicianHost {
        store: store.clone(),
        claim,
        authority: super::OpenAuthority,
        shown_steps: super::ShownSteps::new(()),
    };
    let arguments = json!({"path": "summary.md", "edits": [
        create("- Stopped lisinopril for the cough."),
        append("- Recheck potassium in May.")
    ]});
    let call = Invocation {
        id: "edit".into(),
        name: EDIT.into(),
        arguments: arguments.to_string(),
    };
    let text = "- Stopped lisinopril for the cough.\n- Recheck potassium in May.";
    assert_eq!(
        returned(host.execute(&call, arguments).await.unwrap()),
        json!({"path": "summary.md", "version": 1, "characters": size(text), "not_applied": []})
    );
    assert_eq!(read(&store, &scope, "summary.md").await["markdown"], text);
    assert_eq!(list(&store, &other).await["summary"], Value::Null);
    let shown =
        crate::physician_chat::step(&call, crate::physician_chat::CallEnding::Running).unwrap();
    assert_eq!(
        shown,
        Some(physician_wire::Step::EditedMemory {
            state: physician_wire::StepState::Running,
        })
    );
}
