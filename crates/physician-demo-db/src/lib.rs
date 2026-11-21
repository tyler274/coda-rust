//! Browser-based SQLite store and build-time fixture seeder for the static Chords demo.
#![deny(missing_docs)]

pub mod driver;
pub mod follow_ups;
pub mod reviews_memory;
pub(crate) mod schema;
#[cfg(not(target_arch = "wasm32"))]
pub mod seed;
pub mod sources_admin;
pub mod spruce;
pub mod store;
pub mod updates_briefings;

pub use driver::{DbError, SqliteDriver};
pub use store::{DemoSourceRecord, DemoStore, DraftSaveOutcome};

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use physician_wire::{ExchangeSteps, RecordRead, Step, StepState};
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn seeds_queries_mutates_and_resets() {
        let ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("coda-demo-db-{}-{ns}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (seed_path, active_path) = (dir.join("seed.sqlite"), dir.join("active.sqlite"));
        let seed_store = DemoStore::new(SqliteDriver::open_file(&seed_path).unwrap());
        let units = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../patients/units");
        assert_eq!(
            seed::seed_from_units_dir(&seed_store, &units, 6).unwrap(),
            6
        );
        drop(seed_store);
        std::fs::copy(&seed_path, &active_path).unwrap();
        let driver = SqliteDriver::open_file(&active_path).unwrap();
        driver.bind_reset_files(seed_path, active_path);
        let store = DemoStore::new(driver);

        assert!(!store.today().await.unwrap().rows.is_empty());
        let search = store.search("Kowalczyk").await.unwrap();
        assert_eq!(search.patients[0].patient_workspace_id, "unit-2013");

        let ws = store.workspace("unit-2013").await.unwrap();
        assert_eq!(ws.display_name, "Gary Kowalczyk");
        let rec_id = ws.undecided_recommendations[0].recommendation_id.clone();
        let before_plans = ws.active_plan_items.len();

        let msg_cmd = serde_json::json!({
            "type": "message.submit", "idempotency_key": "k-1",
            "payload": { "patient_workspace_id": "unit-2013", "chat_id": "chat-unit-2013-1", "text": "What is his atorvastatin dose and LDL?" }
        });
        assert_eq!(store.command(&msg_cmd).await.unwrap()["ok"], true);
        let snap = store
            .chat_snapshot("unit-2013", "chat-unit-2013-1")
            .await
            .unwrap();
        assert_eq!(snap.snapshot.transcript.len(), 2);
        let ExchangeSteps::Read(steps) = &snap.snapshot.transcript.last().unwrap().steps else {
            panic!("expected read steps");
        };
        assert!(matches!(steps[0], Step::SearchedChart { .. }));
        let Step::ReadRecords {
            state: StepState::Returned { result },
            ..
        } = &steps[1]
        else {
            panic!("expected records");
        };
        let RecordRead::Whole { version, .. } = &result[0] else {
            panic!("expected whole");
        };
        assert!(
            !store
                .source_record("unit-2013", &version.candidate_id)
                .await
                .unwrap()
                .body
                .is_empty()
        );

        let decide_cmd = serde_json::json!({
            "type": "plan.decide_recommendation",
            "payload": { "patient_workspace_id": "unit-2013", "recommendation_id": rec_id }
        });
        store.command(&decide_cmd).await.unwrap();
        let ws_after = store.workspace("unit-2013").await.unwrap();
        assert!(ws_after.undecided_recommendations.is_empty());
        assert_eq!(ws_after.active_plan_items.len(), before_plans + 1);

        assert!(matches!(
            store
                .save_draft("unit-2013", "chat-unit-2013-1", 0, "Check A1c")
                .await
                .unwrap(),
            DraftSaveOutcome::Saved(_)
        ));
        assert!(matches!(
            store
                .save_draft("unit-2013", "chat-unit-2013-1", 0, "Stale edit")
                .await
                .unwrap(),
            DraftSaveOutcome::Conflict(_)
        ));

        store.reset().await.unwrap();
        let ws_reset = store.workspace("unit-2013").await.unwrap();
        assert_eq!(ws_reset.active_plan_items.len(), before_plans);
        assert!(!ws_reset.undecided_recommendations.is_empty());
        let _ = std::fs::remove_dir_all(dir);
    }
}
