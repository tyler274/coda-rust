//! Serves the Duet stand-in on the host loopback the emulator already uses.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use duet_stub::{Store, router};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

async fn run() -> Result<(), String> {
    let mut listen = "127.0.0.1:8000".to_owned();
    let mut state = default_state_path()?;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--listen" => {
                listen = args
                    .next()
                    .ok_or_else(|| "--listen needs an address".to_owned())?;
            }
            "--state" => {
                state = PathBuf::from(
                    args.next()
                        .ok_or_else(|| "--state needs a path".to_owned())?,
                );
            }
            other => {
                return Err(format!(
                    "unknown argument {other}\n\n  cargo run -p duet-stub\n  cargo run -p duet-stub -- --listen 127.0.0.1:8000 --state PATH"
                ));
            }
        }
    }
    let store = Store::open(&state)?;
    let app = router(Arc::new(Mutex::new(store)));
    let listener = tokio::net::TcpListener::bind(&listen)
        .await
        .map_err(|error| format!("listen on {listen}: {error}"))?;
    println!("Duet stub listening on {listen}");
    println!("Writes stay beside {}", state.display());
    axum::serve(listener, app)
        .await
        .map_err(|error| error.to_string())
}

fn default_state_path() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("DUET_STUB_STATE") {
        return Ok(PathBuf::from(path));
    }
    let base = env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os("HOME").map(|home| PathBuf::from(home).join(".local").join("state"))
        })
        .ok_or_else(|| "HOME is unset; pass --state".to_owned())?;
    Ok(base.join("duet-stub").join("messages.json"))
}
