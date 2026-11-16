//! The local rounds command against a disposable Postgres and the local
//! Responses stub. Memory records when rounds finished: each run's record
//! holds the newest rounds mark before and after the run, as the store reads
//! it, and the command keeps no last-ran file of its own.

use std::path::Path;
use std::process::Command;

use chrono::{DateTime, Utc};
use local_product::runtime::Store;
use local_product::runtime::memory::{PatientKey, RoundsMark};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

/// Runs the rounds command once from `home` and returns its run record.
fn run_rounds(home: &Path, practice: &str, workspace: &str, database: &str, stub: &str) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_physician-rounds"))
        .args([practice, workspace])
        .current_dir(home)
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .env("HOME", home)
        .envs(support::models::environment())
        .env("PHYSICIAN_DATABASE_URL", database)
        .env("PHYSICIAN_RESPONSES_STUB_PORT", stub)
        .output()
        .expect("run the rounds command");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(
        output.status.success(),
        "{stdout}{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let record = stdout
        .lines()
        .find_map(|line| line.strip_prefix("run record: "))
        .unwrap_or_else(|| panic!("no run record: {stdout}"));
    serde_json::from_str(&std::fs::read_to_string(home.join(record)).unwrap()).unwrap()
}

/// A mark as the run record holds it.
fn recorded(mark: RoundsMark) -> Value {
    json!({
        "started_at": mark.started.to_rfc3339(),
        "saved_at": mark.saved.to_rfc3339(),
    })
}

/// The record's start in whole seconds, as memory keeps a run's start.
fn started(record: &Value) -> String {
    let time = DateTime::parse_from_rfc3339(record["started_at"].as_str().unwrap())
        .unwrap()
        .with_timezone(&Utc);
    DateTime::from_timestamp(time.timestamp(), 0)
        .unwrap()
        .to_rfc3339()
}

#[tokio::test]
async fn each_run_record_holds_the_newest_rounds_mark_before_and_after_and_no_last_ran_file_is_written()
 {
    let Some(database) = support::Database::create("rounds_command").await else {
        return;
    };
    let home = std::env::temp_dir().join(format!(
        "physician-rounds-command-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).unwrap();
    let stub = support::ResponsesStub::start_with_reply(&home, 0, 0, "[]", "Nothing new.").await;
    let (practice, workspace) = (
        uuid::Uuid::new_v4().to_string(),
        uuid::Uuid::new_v4().to_string(),
    );
    let store = Store::connect(&database.url).await.unwrap();
    let patient = PatientKey::for_rounds(&practice, &workspace).unwrap();

    let first = run_rounds(&home, &practice, &workspace, &database.url, &stub.port);
    let mark = store.last_rounds(&patient).await.unwrap().unwrap();
    assert_eq!(
        first["ending"],
        json!({"finished": {"account": "Nothing new."}})
    );
    assert_eq!(first["last_rounds_before"], Value::Null);
    assert_eq!(first["last_rounds_after"], recorded(mark));
    assert_eq!(
        first["last_rounds_after"]["started_at"],
        json!(started(&first))
    );

    // The record is named by its run's start second.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let second = run_rounds(&home, &practice, &workspace, &database.url, &stub.port);
    let newest = store.last_rounds(&patient).await.unwrap().unwrap();
    assert_eq!(second["last_rounds_before"], recorded(mark));
    assert_eq!(second["last_rounds_after"], recorded(newest));
    assert_eq!(
        second["last_rounds_after"]["started_at"],
        json!(started(&second))
    );

    drop(stub);
    std::fs::remove_dir_all(&home).unwrap();
}
