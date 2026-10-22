//! Explicit bounded recovery-job entrypoint. No scheduler, migration, HTTP
//! activation, object listing, or restore/coverage declaration is performed.
use duet_api::privacy_archive::GcsReceiptArchive;
use front_door::PostgresFrontDoorStore;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    match run().await {
        Ok(report) => {
            println!(
                "{}",
                serde_json::to_string(&report).expect("aggregate report")
            );
            if report.failed > 0 || report.budget_exhausted || report.pending > 0 {
                std::process::ExitCode::FAILURE
            } else {
                std::process::ExitCode::SUCCESS
            }
        }
        Err(()) => {
            eprintln!("privacy_journal_delivery_unavailable");
            std::process::ExitCode::FAILURE
        }
    }
}
async fn run() -> Result<front_door::journal::DeliveryReport, ()> {
    let bucket = std::env::var("DUET_PRIVACY_JOURNAL_BUCKET").map_err(|_| ())?;
    let url = std::env::var("DUET_PRIVACY_DELIVERY_DATABASE_URL").map_err(|_| ())?;
    let archive = GcsReceiptArchive::new(bucket).map_err(|_| ())?;
    let store = PostgresFrontDoorStore::connect_without_migrations(&url)
        .await
        .map_err(|_| ())?;
    store.check_schema().await.map_err(|_| ())?;
    store
        .recover_luna_restrictions(&archive, chrono::Utc::now().timestamp())
        .await
        .map_err(|_| ())
}
