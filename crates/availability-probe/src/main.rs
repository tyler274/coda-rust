//! Same image serves the small authenticated dispatcher or runs one job.
#[tokio::main]
async fn main() -> std::process::ExitCode {
    let result = if std::env::var("AVAILABILITY_MODE").as_deref() == Ok("dispatch") {
        availability_probe::dispatch::serve().await
    } else {
        run_job().await
    };
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"severity": "ERROR", "message": error.to_string()})
            );
            std::process::ExitCode::FAILURE
        }
    }
}
async fn run_job() -> Result<(), availability_probe::Error> {
    use availability_probe::{Config, Error, delivery};
    let config = Config::from_env(|key| std::env::var(key).ok())?;
    let slot = std::env::var("AVAILABILITY_PROBE_SLOT")
        .map_err(|e| Error::caused("explicit scheduler slot required", e))?
        .parse::<u64>()
        .map_err(|e| Error::caused("invalid scheduler slot", e))?;
    let execution = std::env::var("CLOUD_RUN_EXECUTION")
        .map_err(|e| Error::caused("Cloud Run execution identity required", e))?;
    delivery::run_slot(config, slot, execution).await
}
