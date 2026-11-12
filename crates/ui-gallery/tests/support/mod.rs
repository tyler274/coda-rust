//! The gallery as its documented command serves it, shared by the browser tests.

use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use crate::devtools::free_port;

/// Each start rewrites the one built bundle, so a test holds this while it serves it.
static BUNDLE: Mutex<()> = Mutex::new(());

/// The documented command, serving on a port of its own.
pub struct Gallery {
    server: Child,
    pub url: String,
    _bundle: MutexGuard<'static, ()>,
}

impl Gallery {
    pub fn start() -> Self {
        let bundle = BUNDLE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let port = free_port().expect("a free local port for the gallery");
        let mut command = Command::new(workspace.join("scripts/ui-gallery.sh"));
        // Cargo's own variables for this test would change the inner build's fingerprints.
        for (key, _) in std::env::vars_os() {
            let key = key.to_string_lossy().into_owned();
            let cargo_home = matches!(
                key.as_str(),
                "CARGO_HOME" | "CARGO_TARGET_DIR" | "CARGO_NET_OFFLINE"
            );
            if key == "LD_LIBRARY_PATH" || (key.starts_with("CARGO_") && !cargo_home) {
                command.env_remove(key);
            }
        }
        let mut server = command
            .env("UI_GALLERY_LISTEN_ADDR", format!("127.0.0.1:{port}"))
            .spawn()
            .expect("scripts/ui-gallery.sh");
        // Builds the gallery first, so this allows for a cold WebAssembly build.
        let deadline = Instant::now() + Duration::from_secs(1200);
        while TcpStream::connect(("127.0.0.1", port)).is_err() {
            if let Some(status) = server.try_wait().expect("the gallery command") {
                panic!("scripts/ui-gallery.sh exited before serving: {status}");
            }
            if Instant::now() > deadline {
                let _ = server.kill();
                let _ = server.wait();
                panic!("scripts/ui-gallery.sh never served port {port}");
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Self {
            server,
            url: format!("http://127.0.0.1:{port}/"),
            _bundle: bundle,
        }
    }
}

impl Drop for Gallery {
    fn drop(&mut self) {
        let _ = self.server.kill();
        let _ = self.server.wait();
    }
}
