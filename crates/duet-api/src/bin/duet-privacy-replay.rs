//! Explicit restore gate. Run after restoring SQL and before reopening serving.
//! Event IDs come from the independently retained journal inventory, not restored SQL.
use duet_api::{
    database::AccountDatabase,
    journal_gcs::GcsJournal,
    privacy::{FileJournal, PrivacyStore, ReceiptJournal},
};
use std::{env, sync::Arc};
#[tokio::main]
async fn main() {
    let ids: Vec<uuid::Uuid> = env::args()
        .skip(1)
        .map(|s| s.parse().expect("receipt UUID"))
        .collect();
    let database =
        AccountDatabase::connect(&env::var("DATABASE_URL").expect("runtime DATABASE_URL"))
            .await
            .expect("account database boundary");
    let journal: Arc<dyn ReceiptJournal> =
        if let Ok(bucket) = env::var("DUET_PRIVACY_JOURNAL_BUCKET") {
            Arc::new(GcsJournal::new(bucket).expect("privacy bucket"))
        } else {
            assert!(
                env::var("K_SERVICE").is_err(),
                "hosted replay requires GCS journal"
            );
            Arc::new(
                FileJournal::open(
                    env::var("DUET_PRIVACY_JOURNAL_DIRECTORY")
                        .expect("independent journal directory")
                        .into(),
                )
                .await
                .expect("privacy journal"),
            )
        };
    let privacy = PrivacyStore::new(database, journal);
    if ids.is_empty() {
        privacy
            .reconcile_inventory()
            .await
            .expect("complete independent inventory replay required");
    }
    for id in ids {
        privacy
            .replay(id)
            .await
            .expect("restore replay must complete before serving");
    }
}
