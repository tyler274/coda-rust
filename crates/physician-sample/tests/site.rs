//! Loads the physician page in Chromium and fails when the document is empty.
//! Dioxus mounts into `#main`. A missing element leaves the visible page blank.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[tokio::test]
async fn the_recorded_practice_is_visible() {
    let site = build_site();
    // SAFETY: this test sets the static site path before starting the server.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
        env::remove_var("PHYSICIAN_UPSTREAM");
        env::remove_var("CODA_LOCAL_MODEL_EXECUTION");
        env::remove_var("CODA_ALLOW_MODEL_CALLS");
        env::remove_var("CODA_LOCAL_SIMULATION");
    }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { physician_sample::serve(listener).await });
    wait_until_ready(address).await;

    let chrome = chromium();
    let dom = dump_dom(&chrome, &format!("http://{address}/")).await;
    assert!(
        dom.contains("Today")
            && dom.contains("Coda hasn’t read Gmail yet.")
            && dom.contains("Coda hasn’t read Spruce yet.")
            && dom.contains("3:30 PM")
            && dom.contains("Mary Louis")
            && dom.contains("m3 11 9-8 9 8")
            && dom.contains("viewBox=\"0 0 24 24\"")
            && dom.contains("stroke-width=\"1.8\"")
            && dom.contains("box-sizing: border-box")
            && !dom.contains("Nothing needs you right now.")
            && !dom.contains("Connections"),
        "Today did not match the React shell:\n{dom}"
    );
    let workspace = dump_dom(
        &chrome,
        &format!("http://{address}/?patient=pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e"),
    )
    .await;
    assert!(
        workspace.contains("Patient summary")
            && workspace.contains("Age 54 years")
            && workspace.contains("MRN 00000005")
            && workspace.contains("Gender Female")
            && workspace.contains("[1]")
            && workspace.contains("Source 2: Discharge Summary. Preview source.")
            && workspace.contains("Upcoming visits")
            && workspace.contains("3:30 PM")
            && workspace.contains("Follow-up visit")
            && workspace.contains("Preparation failed")
            && workspace.contains("Summary written")
            && workspace.contains("Today, 8:00 AM")
            && workspace.contains("Records as of")
            && workspace.contains("Today, 12:57 PM")
            && !workspace.contains("[[patient_source")
            && !workspace.contains("Write a message")
            && !workspace.contains("Visit preparation failed. No new briefing was completed."),
        "the workspace did not match the React patient page:\n{workspace}"
    );
    let logo = tokio::task::spawn_blocking(move || read_logo(address))
        .await
        .expect("logo read");
    let head = String::from_utf8_lossy(&logo[..logo.len().min(80)]);
    assert!(
        head.contains("200") && logo.windows(8).any(|window| window == b"\x89PNG\r\n\x1a\n"),
        "the reference logo was not a PNG:\n{head}"
    );
}

#[tokio::test]
async fn the_local_simulation_panel_scenarios_and_investor_metric_render_and_interact() {
    let site = build_site();
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
    }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, physician_sample::simulation_router())
            .await
            .unwrap();
    });
    wait_until_ready(address).await;

    let chrome = chromium();
    let today_dom = dump_dom(&chrome, &format!("http://{address}/")).await;
    assert!(
        today_dom.contains("Simulated practice walk — Connected care walk")
            && today_dom.contains("Items the physician touched per 100 patients per week")
            && today_dom.contains("20.0")
            && today_dom.contains("1 physician touches across 5 patients")
            && today_dom.contains("Critical lab arrival (ED reply)")
            && today_dom.contains("Lab correction supersedes ED reply")
            && today_dom.contains("Undecided medication guard")
            && today_dom.contains("Jake Smith (MRN 00000001)")
            && today_dom.contains("Tanvir Chowdhury (MRN 00000003)")
            && today_dom.contains("Dan Cohen (MRN 00000002)")
            && today_dom.contains("Teresa Hayes (MRN 00000004)")
            && today_dom.contains("Mary Louis (MRN 00000005)")
            && !today_dom.contains("Urgent potassium 6.3 mmol/L"),
        "Simulated Today did not render panel, scenarios, and investor metric:\n{today_dom}"
    );

    let jake_url = format!("http://{address}/?patient=pw-82c44e9b-58c0-4af1-bd21-771bf77e101d");
    let jake_dom = dump_dom(&chrome, &jake_url).await;
    assert!(
        jake_dom.contains("Jake Smith")
            && jake_dom.contains("Patient replies")
            && jake_dom.contains("Superseded by corrected labs")
            && jake_dom.contains("prior K+ 6.3 mmol/L entered in error")
            && jake_dom.contains("Active plan")
            && jake_dom.contains("Record home blood-pressure log for 1 week")
            && jake_dom.contains("Undecided medication recommendations")
            && jake_dom.contains("Losartan 50 mg daily — Held (undecided)")
            && jake_dom.contains("Approve into plan")
            && !jake_dom.contains("data-plan-item=\"plan-rec-jake-losartan-50\""),
        "Jake workspace did not guard undecided Losartan or show superseded ED reply:\n{jake_dom}"
    );

    // Switch to `lab-arrives-ed-reply` (pre-correction) and verify the urgent ED reply is active on Today.
    let http = reqwest::Client::new();
    let resp = http
        .post(format!("http://{address}/commands"))
        .json(&serde_json::json!({
            "type": "simulation.run",
            "payload": { "scenario": "lab-arrives-ed-reply" }
        }))
        .send()
        .await
        .expect("run lab-arrives-ed-reply");
    assert!(resp.status().is_success());

    let pre_correction_today = dump_dom(&chrome, &format!("http://{address}/")).await;
    assert!(
        pre_correction_today.contains("Urgent potassium 6.3 mmol/L — advise ED evaluation")
            && pre_correction_today.contains("Critical lab arrival (ED reply)"),
        "Pre-correction scenario did not surface urgent ED reply on Today:\n{pre_correction_today}"
    );

    // Admit corrected labs and approve undecided Losartan, then verify the UI reflects both transitions and updates the investor metric.
    let resp = http
        .post(format!("http://{address}/commands"))
        .json(&serde_json::json!({
            "type": "simulation.admit_corrected_labs",
            "payload": { "patient_workspace_id": "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" }
        }))
        .send()
        .await
        .expect("admit corrected labs");
    assert!(resp.status().is_success());

    let resp = http
        .post(format!("http://{address}/commands"))
        .json(&serde_json::json!({
            "type": "plan.decide_recommendation",
            "payload": {
                "patient_workspace_id": "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d",
                "recommendation_id": "rec-jake-losartan-50"
            }
        }))
        .send()
        .await
        .expect("decide recommendation");
    assert!(resp.status().is_success());

    let jake_after_dom = dump_dom(&chrome, &jake_url).await;
    assert!(
        jake_after_dom.contains("Superseded by corrected labs")
            && jake_after_dom.contains("data-plan-item=\"plan-rec-jake-losartan-50\"")
            && jake_after_dom.contains("No undecided medication recommendations.")
            && jake_after_dom.contains("40.0"),
        "Jake workspace after lab correction + recommendation approval did not update:\n{jake_after_dom}"
    );
}

async fn dump_dom(chrome: &std::path::Path, url: &str) -> String {
    let child = Command::new(chrome)
        .env_remove("LD_LIBRARY_PATH")
        .args([
            "--headless=new",
            "--disable-gpu",
            "--no-sandbox",
            "--virtual-time-budget=15000",
            "--timeout=20000",
            "--dump-dom",
            url,
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("chromium");
    let pid = child.id();
    let wait = tokio::task::spawn_blocking(move || child.wait_with_output());
    let output = match tokio::time::timeout(Duration::from_secs(40), wait).await {
        Ok(Ok(output)) => output.expect("chromium"),
        _ => {
            let _ = Command::new("kill").arg(pid.to_string()).status();
            panic!("chromium did not finish");
        }
    };
    let dom = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "chromium exited {:?}\n{dom}\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    dom.into_owned()
}

fn read_logo(address: std::net::SocketAddr) -> Vec<u8> {
    use std::io::{Read, Write};
    let mut stream = std::net::TcpStream::connect(address).expect("logo connection");
    stream
        .write_all(
            b"GET /assets/coda-health-logo-transparent.png HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        )
        .unwrap();
    let mut body = Vec::new();
    stream.read_to_end(&mut body).unwrap();
    body
}

fn build_site() -> PathBuf {
    static SITE: OnceLock<PathBuf> = OnceLock::new();
    SITE.get_or_init(|| {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let target = env::temp_dir().join("coda-physician-wasm");
        let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
        let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
        cargo_without_parent_package(&mut cargo);
        let status = cargo
            .current_dir(&workspace)
            .env("CARGO_TARGET_DIR", &target)
            .env_remove("PHYSICIAN_API_URL")
            .args([
                "build",
                "-p",
                "physician-app",
                "--target",
                "wasm32-unknown-unknown",
                "--features",
                "web",
            ])
            .status()
            .expect("cargo build");
        assert!(status.success(), "physician-app wasm build failed");
        let site = workspace.join("target/physician-web");
        fs::create_dir_all(&site).unwrap();
        let status = Command::new(wasm_bindgen())
            .args([
                "--target",
                "web",
                "--out-dir",
                site.to_str().unwrap(),
                wasm.to_str().unwrap(),
            ])
            .status()
            .expect("wasm-bindgen");
        assert!(status.success(), "wasm-bindgen failed");
        site
    })
    .clone()
}

fn cargo_without_parent_package(command: &mut Command) {
    command.env_clear();
    for key in [
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
    ] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
    if let Some(paths) = env::var_os("PATH") {
        let filtered = env::join_paths(
            env::split_paths(&paths).filter(|p| !p.to_string_lossy().contains("swiftly")),
        )
        .unwrap_or(paths);
        command.env("PATH", filtered);
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(path) = find_on_path("wasm-bindgen") {
        return path;
    }
    let local = env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen 0.2.129 is required to build the physician page");
}

fn chromium() -> PathBuf {
    if let Some(path) = env::var_os("CHROMIUM") {
        return PathBuf::from(path);
    }
    for name in [
        "chromium",
        "chromium-browser",
        "google-chrome",
        "google-chrome-stable",
    ] {
        if let Some(path) = find_on_path(name) {
            return path;
        }
    }
    panic!("chromium is required to validate the physician page");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

async fn wait_until_ready(address: std::net::SocketAddr) {
    let started = Instant::now();
    loop {
        if started.elapsed() > Duration::from_secs(10) {
            panic!("physician site did not accept connections on {address}");
        }
        if std::net::TcpStream::connect(address).is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
