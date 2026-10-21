//! Chromium proof of the actual executable, including startup asset wiring.
use std::{
    process::{Command, Stdio},
    time::Duration,
};

#[test]
fn desktop_mobile_motion_reduced_motion_and_static_fallback() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let child = Command::new(env!("CARGO_BIN_EXE_public-site"))
        .env("PUBLIC_SITE_LISTEN_ADDR", address.to_string())
        .env("APP_ENV", "test")
        .env_remove("PUBLIC_SITE_INTERNAL_PROBE_ADDR")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    struct Server(std::process::Child);
    impl Drop for Server {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut server = Server(child);
    for attempt in 0..100 {
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "public-site exited before serving"
        );
        if std::net::TcpStream::connect(address).is_ok() {
            break;
        }
        assert!(attempt < 99, "public-site did not start");
        std::thread::sleep(Duration::from_millis(100));
    }
    let result = Command::new("node")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/browser.mjs"))
        .arg(format!("http://{address}"))
        .status()
        .expect("Node and Chromium are required for browser proof");
    assert!(result.success(), "Chromium public-site proof failed");
}
