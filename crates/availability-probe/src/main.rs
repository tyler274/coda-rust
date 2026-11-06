//! Cloud Run Job entrypoint: one probe pass, then exit.
//!
//! Target failures are data, not job failures: the job exits 0 once it has
//! published. It exits 1 only when it cannot configure itself or publish, and
//! the missing heartbeat is what alerts on that.

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let config = match availability_probe::Config::from_env(|key| std::env::var(key).ok()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({ "severity": "ERROR", "message": "availability probe misconfigured", "error": error })
            );
            return std::process::ExitCode::FAILURE;
        }
    };
    match availability_probe::run(config).await {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({ "severity": "ERROR", "message": "availability probe could not publish", "error": error })
            );
            std::process::ExitCode::FAILURE
        }
    }
}
