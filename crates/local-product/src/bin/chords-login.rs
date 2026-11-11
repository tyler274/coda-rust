//! Authorize this local developer's ChatGPT plan for Chords.
#[tokio::main]
async fn main() -> std::process::ExitCode {
    match local_product::chatgpt::sign_in().await {
        Ok(()) => {
            println!(
                "Local Chords is authorized. Set CODA_LOCAL_MODEL_EXECUTION=subscription to use it."
            );
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
