//! End-to-end Chromium browser proof over the unified public demo stack:
//! `/coda/physician/` WASM + `demo.sqlite` -> `POST /auth/demo-session` ->
//! `2x physician-sample` + `physician-worker` + Postgres + Valkey + multi-round
//! `ResponsesStub` -> Valkey restart -> `429 demo_turn_budget_exhausted` ->
//! `"Use local chart preview"` browser-SQLite fallback (`DemoStore`) citations.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::Body;
use axum::http::{StatusCode, Uri, header};
use axum::response::Response;
use serde_json::json;

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[path = "scale/mod.rs"]
mod support;

use devtools::Browser;

static BUILT_DEMO_SITE: OnceLock<PathBuf> = OnceLock::new();

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn unified_public_demo_live_harness_valkey_budget_and_sqlite_fallback_in_chromium() {
    let Some(database) = support::Database::create("demo_live_browser").await else {
        return;
    };

    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .expect("auth store");
    let runtime = local_product::runtime::Store::connect(&database.url)
        .await
        .expect("runtime store");
    physician_sample::demo_session::seed_demo_practice(&auth, &runtime, unix_now())
        .await
        .expect("seed public demo practice");
    drop(auth);

    let quote = "Potassium 5.2 mmol/L";
    let content = "Potassium 5.2 mmol/L on synthetic BMP (reference 3.5-5.1 mmol/L).".to_owned();
    let revision = local_product::sha256_hex(content.as_bytes());
    for workspace in physician_sample::demo_session::demo_workspaces() {
        runtime
            .record_source(&local_product::physician_sources::SourceRevision {
                workspace,
                source_id: physician_sample::synthetic_chart::SOURCE_ID.to_owned(),
                candidate_id: "Observation/demo-bmp-1".to_owned(),
                revision: revision.clone(),
                record_type: "Observation".to_owned(),
                dates: vec![],
                sha256: revision.clone(),
                content: content.clone(),
                readable_parts: vec![],
                provenance: json!({
                    "native_record": "Observation/demo-bmp-1",
                    "title": "Basic Metabolic Panel",
                    "recorded_at": "2026-09-18T10:00:00Z",
                    "record_status": "final"
                }),
                envelope: None,
            })
            .await
            .expect("record synthetic demo observation");
    }

    let site_root = build_demo_subpath_site();
    let std_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind static site");
    std_listener
        .set_nonblocking(true)
        .expect("nonblocking static listener");
    let static_addr = std_listener.local_addr().expect("static addr");
    let (static_stop_tx, static_stop_rx) = tokio::sync::oneshot::channel::<()>();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("static server runtime");
        rt.block_on(async move {
            let static_listener =
                tokio::net::TcpListener::from_std(std_listener).expect("tokio static listener");
            let serve_root = site_root.clone();
            let app = Router::new().fallback(move |uri: Uri| {
                let root = serve_root.clone();
                async move { serve_static(&root, uri.path()) }
            });
            let _ = axum::serve(static_listener, app)
                .with_graceful_shutdown(async move {
                    let _ = static_stop_rx.await;
                })
                .await;
        });
    });

    let home = Home(env::temp_dir().join(format!(
        "physician-demo-live-browser-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    )));
    fs::create_dir_all(&home.0).expect("home");
    let mut valkey = support::Valkey::start().await;
    let redis = valkey.url();

    let rounds = json!([
        [
            {
                "name": "search_patient_information",
                "arguments": {
                    "query": "Potassium",
                    "record_type": "Observation",
                    "after": null
                }
            }
        ],
        [
            {
                "name": "read_patient_sources",
                "arguments": {
                    "references": [
                        {
                            "candidate_id": "Observation/demo-bmp-1",
                            "revision": revision,
                            "cursor": null
                        }
                    ],
                    "max_result_bytes": 4096
                }
            }
        ],
        [
            {
                "name": "add_patient_memory_note",
                "arguments": {
                    "text": "Latest synthetic BMP shows potassium 5.2 mmol/L.",
                    "quote": quote
                }
            }
        ]
    ])
    .to_string();

    let finish_reply = json!({
        "answer": "Recent synthetic metabolic panel demonstrates serum potassium 5.2 mmol/L [S1].",
        "quotes": [{ "label": "S1", "quote": quote }],
        "descriptions": [],
        "basis": "Verified synthetic BMP lab observation [S1].",
        "cites_none": false
    })
    .to_string();

    let stub =
        support::ResponsesStub::start_with_reply(&home.0, 0, 0, &rounds, &finish_reply).await;
    let worker_port = support::free_port();
    let site_a_port = support::free_port();
    let site_b_port = support::free_port();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let site_a_listen = format!("127.0.0.1:{site_a_port}");
    let site_b_listen = format!("127.0.0.1:{site_b_port}");

    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
            ("PHYSICIAN_PUBLIC_DEMO", "1"),
        ],
        &home.0,
    );
    support::wait_ready(&mut worker, worker_port).await;

    let site_common = [
        ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
        ("PHYSICIAN_REDIS_URL", redis.as_str()),
        support::DURABLE,
        ("PHYSICIAN_AUTH", "required"),
        ("PHYSICIAN_PUBLIC_DEMO", "1"),
        ("CODA_DEMO_SESSION_MAX_TURNS", "1"),
    ];
    let mut site_a = support::Process::spawn(
        "site-a",
        support::site_bin(),
        &[],
        &[
            site_common[0],
            site_common[1],
            site_common[2],
            site_common[3],
            site_common[4],
            site_common[5],
            ("PHYSICIAN_LISTEN_ADDR", site_a_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site_a, site_a_port).await;

    let mut site_b = support::Process::spawn(
        "site-b",
        support::site_bin(),
        &[],
        &[
            site_common[0],
            site_common[1],
            site_common[2],
            site_common[3],
            site_common[4],
            site_common[5],
            ("PHYSICIAN_LISTEN_ADDR", site_b_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site_b, site_b_port).await;

    let deadline = Instant::now() + Duration::from_secs(15);
    while valkey.subscribers() < 3 {
        assert!(
            Instant::now() < deadline,
            "expected >= 3 Valkey subscribers, saw {}\n{}\n{}\n{}",
            valkey.subscribers(),
            site_a.tail(),
            site_b.tail(),
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let url =
        format!("http://{static_addr}/coda/physician/?api=http%3A%2F%2F127.0.0.1%3A{site_a_port}");
    let mut browser = Browser::launch();
    browser.resize((1440, 960));
    browser.watch_page_errors();
    browser.page().call_session("Page.enable", json!({}));
    browser.page().call_session("Runtime.enable", json!({}));
    browser.page().navigate(&url);

    // 1. Live-first bootstrap switches to Live harness and hides Sign out / Invite colleague.
    let live_boot = wait_eval(
        &mut browser,
        "document.querySelector('.demo-quick-bar')?.innerText.includes('Live harness') && document.body.innerText.includes('Dr. Hannah Alvarez')",
        "Live harness bootstrap on /coda/physician/?api=...",
        Duration::from_secs(30),
    )
    .await;
    assert!(
        !live_boot.contains("Sign out") && !live_boot.contains("Invite colleague"),
        "live demo must hide Sign out and Invite colleague:\n{live_boot}"
    );

    // 2. Open Mary Louis via Find patient, submit turn 1, and wait for Valkey SSE updates.
    click_button_named(&mut browser, "Find patient");
    wait_eval(
        &mut browser,
        "Boolean(document.querySelector('dialog input'))",
        "Find patient dialog input",
        Duration::from_secs(10),
    )
    .await;
    set_input_value(&mut browser, "dialog input", "Mary");
    wait_eval(
        &mut browser,
        "[...document.querySelectorAll('dialog button')].some(b => b.innerText.includes('Mary Louis'))",
        "Mary Louis result in Find patient dialog",
        Duration::from_secs(10),
    )
    .await;
    click_dialog_button(&mut browser, "Mary Louis");
    wait_eval(
        &mut browser,
        "document.body.innerText.includes('Mary Louis') && document.body.innerText.includes('New conversation')",
        "Mary Louis workspace ready for New conversation",
        Duration::from_secs(15),
    )
    .await;
    click_button_named(&mut browser, "New conversation");
    wait_eval(
        &mut browser,
        "Boolean(document.querySelector('textarea.composer-input'))",
        "Mary Louis workspace with composer",
        Duration::from_secs(15),
    )
    .await;

    set_textarea_value(
        &mut browser,
        "textarea.composer-input",
        "What is the latest potassium and please note it?",
    );
    click_button_exact(&mut browser, "Send");

    wait_eval(
        &mut browser,
        "document.body.innerText.includes('Recent synthetic metabolic panel demonstrates serum potassium 5.2 mmol/L') && document.body.innerText.includes('Basis and limitations') && document.querySelectorAll('.answer-words button.citation-chip').length > 0",
        "finished live harness answer with citation chip and Basis",
        Duration::from_secs(30),
    )
    .await;

    // 3. Expand WorkTrace and verify physician-facing step descriptions.
    browser.click(".work-trace-line");
    let steps_dom = wait_eval(
        &mut browser,
        "document.querySelector('.work-trace')?.getAttribute('data-state') === 'open' && document.body.innerText.includes('Saved note to patient memory')",
        "expanded WorkTrace with SavedMemoryNote step",
        Duration::from_secs(10),
    )
    .await;
    assert!(
        steps_dom.contains("Saved note to patient memory")
            && (steps_dom.contains("Searched") || steps_dom.contains("Read")),
        "missing expected step descriptions:\n{steps_dom}"
    );

    // 4. Click inline [1] citation chip -> SourcePane opens with verbatim <mark> highlight.
    browser.click(".answer-words button.citation-chip");
    let pane_text = wait_eval(
        &mut browser,
        "Boolean(document.querySelector('aside.source-reading-pane mark')) && (document.querySelector('aside.source-reading-pane mark')?.textContent || '').includes('Potassium 5.2 mmol/L')",
        "SourcePane open with highlighted Potassium 5.2 mmol/L passage",
        Duration::from_secs(15),
    )
    .await;
    assert!(
        !pane_text.contains("Cited passage could not be matched"),
        "unexpected passage-not-found notice in live SourcePane:\n{pane_text}"
    );
    assert_no_load_errors(&mut browser, "live citation SourcePane");

    // 5. Restart Valkey and verify the live page stays healthy with no load error banners.
    valkey.restart().await;
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert_no_load_errors(&mut browser, "after Valkey restart");

    // 6. Close SourcePane, submit second turn -> hits CODA_DEMO_SESSION_MAX_TURNS=1 (429).
    browser.click("aside.source-reading-pane button.source-pane-close");
    wait_eval(
        &mut browser,
        "!document.querySelector('aside.source-reading-pane')",
        "closed live SourcePane",
        Duration::from_secs(5),
    )
    .await;

    let pre_429_errors = browser.take_page_errors();
    assert!(
        pre_429_errors.is_empty(),
        "unexpected JS runtime errors before 429 budget test: {pre_429_errors:#?}"
    );

    set_textarea_value(
        &mut browser,
        "textarea.composer-input",
        "Follow-up turn hitting live session turn cap.",
    );
    click_button_exact(&mut browser, "Send");

    wait_eval(
        &mut browser,
        "document.body.innerText.includes('Live turn limit reached') && document.body.innerText.includes('This demo session has reached its turn limit.') && [...document.querySelectorAll('button')].some(b => b.innerText.includes('Use local chart preview'))",
        "429 budget exhaustion notice and Use local chart preview fallback",
        Duration::from_secs(15),
    )
    .await;
    let budget_errors = browser.take_page_errors();
    assert!(
        budget_errors
            .iter()
            .all(|msg| msg.contains("429 (Too Many Requests)") && msg.contains("/commands")),
        "unexpected non-429 JS runtime errors during budget exhaustion: {budget_errors:#?}"
    );

    // 7. Switch to browser-SQLite Local chart preview -> clean remount at Today -> Gary Kowalczyk citation.
    click_button_named(&mut browser, "Use local chart preview");
    wait_eval(
        &mut browser,
        "document.querySelector('.demo-quick-bar')?.innerText.includes('Local chart preview') && document.body.innerText.includes('Gary Kowalczyk')",
        "clean remount at Today under Local chart preview",
        Duration::from_secs(20),
    )
    .await;

    browser.click("[data-today-item=\"today-unit-2013\"] .today-row-open");
    wait_eval(
        &mut browser,
        "document.body.innerText.includes('High-intensity statin review') && Boolean(document.querySelector('textarea.composer-input'))",
        "Gary Kowalczyk workspace in Local chart preview",
        Duration::from_secs(15),
    )
    .await;

    set_textarea_value(
        &mut browser,
        "textarea.composer-input",
        "What is his latest LDL and atorvastatin dose?",
    );
    click_button_exact(&mut browser, "Send");

    wait_eval(
        &mut browser,
        "document.body.innerText.includes('From SQLite chart search across') && document.querySelectorAll('.answer-words button.citation-chip').length > 0 && document.body.innerText.includes('Basis and limitations')",
        "SQLite chat answer with inline citation chip and Basis and limitations",
        Duration::from_secs(20),
    )
    .await;

    browser.click(".answer-words button.citation-chip");
    let local_pane = wait_eval(
        &mut browser,
        "Boolean(document.querySelector('aside.source-reading-pane mark')) && (document.querySelector('aside.source-reading-pane')?.innerText || '').includes('Technical details')",
        "local SQLite SourcePane with highlighted passage and Technical details",
        Duration::from_secs(15),
    )
    .await;
    assert!(
        !local_pane.contains("Cited passage could not be matched"),
        "unexpected passage-not-found notice in SQLite SourcePane:\n{local_pane}"
    );
    assert_no_load_errors(&mut browser, "SQLite fallback SourcePane");

    let page_errors = browser.take_page_errors();
    assert!(
        page_errors.is_empty(),
        "unexpected JS runtime errors in Chromium: {page_errors:#?}"
    );

    let _ = static_stop_tx.send(());
}

async fn wait_eval(
    browser: &mut Browser,
    condition: &str,
    description: &str,
    timeout: Duration,
) -> String {
    let wrapped = format!("Boolean({condition})");
    let started = Instant::now();
    loop {
        if browser.page().evaluate(&wrapped) == json!(true) {
            return browser
                .page()
                .evaluate("document.body?.innerText || ''")
                .as_str()
                .unwrap_or_default()
                .to_owned();
        }
        if started.elapsed() >= timeout {
            let body = browser
                .page()
                .evaluate("document.body?.innerText || ''")
                .as_str()
                .unwrap_or_default()
                .to_owned();
            let errors = browser.take_page_errors();
            panic!(
                "timed out waiting for {description} after {timeout:?}\nerrors: {errors:#?}\nbody:\n{body}"
            );
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn assert_no_load_errors(browser: &mut Browser, step: &str) {
    let body = browser
        .page()
        .evaluate("document.body?.innerText || ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    for bad in [
        "HTTP client requested on demo transport",
        "Coda couldn’t load",
        "Coda couldn't load",
        "couldn’t load",
        "couldn't load",
    ] {
        assert!(!body.contains(bad), "{step}: found {bad:?} in:\n{body}");
    }
}

fn click_button_named(browser: &mut Browser, needle: &str) {
    let expr = format!(
        "(() => {{ const b = [...document.querySelectorAll('button')].find(el => el.innerText.includes({})); if (!b) return false; b.click(); return true; }})()",
        json!(needle)
    );
    assert_eq!(
        browser.page().evaluate(&expr),
        json!(true),
        "missing button containing {needle:?}"
    );
}

fn click_button_exact(browser: &mut Browser, label: &str) {
    let expr = format!(
        "(() => {{ const b = [...document.querySelectorAll('button')].find(el => el.innerText.trim() === {}); if (!b) return false; b.click(); return true; }})()",
        json!(label)
    );
    assert_eq!(
        browser.page().evaluate(&expr),
        json!(true),
        "missing button with exact label {label:?}"
    );
}

fn click_dialog_button(browser: &mut Browser, needle: &str) {
    let expr = format!(
        "(() => {{ const b = [...document.querySelectorAll('dialog button')].find(el => el.innerText.includes({})); if (!b) return false; b.click(); return true; }})()",
        json!(needle)
    );
    assert_eq!(
        browser.page().evaluate(&expr),
        json!(true),
        "missing dialog button containing {needle:?}"
    );
}

fn set_input_value(browser: &mut Browser, selector: &str, value: &str) {
    let expr = format!(
        "(() => {{ const el = document.querySelector({}); if (!el) return false; Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set.call(el, {}); el.dispatchEvent(new Event('input', {{bubbles: true}})); return true; }})()",
        json!(selector),
        json!(value)
    );
    assert_eq!(
        browser.page().evaluate(&expr),
        json!(true),
        "missing input {selector:?}"
    );
}

fn set_textarea_value(browser: &mut Browser, selector: &str, value: &str) {
    let expr = format!(
        "(() => {{ const el = document.querySelector({}); if (!el) return false; Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value').set.call(el, {}); el.dispatchEvent(new Event('input', {{bubbles: true}})); return true; }})()",
        json!(selector),
        json!(value)
    );
    assert_eq!(
        browser.page().evaluate(&expr),
        json!(true),
        "missing textarea {selector:?}"
    );
}

fn build_demo_subpath_site() -> PathBuf {
    BUILT_DEMO_SITE
        .get_or_init(|| {
            let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            let target = workspace.join("target/coda-physician-demo-wasm");
            let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
            let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            cargo_without_parent_package(&mut cargo);
            let wasm_status = cargo
                .current_dir(&workspace)
                .env("CARGO_TARGET_DIR", &target)
                .env("PHYSICIAN_DEMO_DB", "./demo.sqlite")
                .env_remove("PHYSICIAN_API_URL")
                .args([
                    "build",
                    "--locked",
                    "-p",
                    "physician-app",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--no-default-features",
                    "--features",
                    "web",
                ])
                .status()
                .expect("build physician-app wasm");
            assert!(wasm_status.success(), "physician-app wasm build failed");

            let out_root = workspace.join("target/coda-demo-browser-site");
            let mount = out_root.join("coda/physician");
            let _ = fs::remove_dir_all(&out_root);
            fs::create_dir_all(mount.join("assets")).expect("create mount assets dir");

            let bindgen_status = Command::new(wasm_bindgen())
                .args([
                    "--target",
                    "web",
                    "--out-dir",
                    mount.to_str().unwrap(),
                    "--out-name",
                    "physician-app",
                    wasm.to_str().unwrap(),
                ])
                .status()
                .expect("run wasm-bindgen");
            assert!(bindgen_status.success(), "wasm-bindgen failed");

            fs::copy(
                workspace.join("crates/physician-app/index.html"),
                mount.join("index.html"),
            )
            .expect("copy index.html");
            fs::copy(
                workspace.join("crates/physician-app/assets/favicon.svg"),
                mount.join("favicon.svg"),
            )
            .expect("copy favicon.svg");
            fs::copy(
                workspace.join("crates/physician-app/assets/coda-health-logo-transparent.png"),
                mount.join("assets/coda-health-logo-transparent.png"),
            )
            .expect("copy logo");

            let mut seed_cmd = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            cargo_without_parent_package(&mut seed_cmd);
            if let Ok(existing_target) = env::var("CARGO_TARGET_DIR") {
                seed_cmd.env("CARGO_TARGET_DIR", existing_target);
            }
            let seed_status = seed_cmd
                .current_dir(&workspace)
                .args([
                    "run",
                    "--locked",
                    "-p",
                    "physician-demo-db",
                    "--bin",
                    "seed_demo_db",
                    "--",
                    "--units",
                ])
                .arg(workspace.join("patients/units"))
                .arg("--out")
                .arg(mount.join("demo.sqlite"))
                .args(["--limit", "60"])
                .status()
                .expect("run seed_demo_db");
            assert!(seed_status.success(), "seed_demo_db failed");

            let sqljs_zip = env::var("SQLJS_ARCHIVE")
                .unwrap_or_else(|_| "/tmp/sqljs-wasm-v1.16.0.zip".to_owned());
            let extract_status = Command::new("python3")
                .args([
                    "-c",
                    r#"import pathlib, sys, zipfile
zf = zipfile.ZipFile(sys.argv[1])
dest = pathlib.Path(sys.argv[2])
for want in ("sql-wasm.js", "sql-wasm.wasm"):
    match = next((n for n in zf.namelist() if n.endswith("/" + want) or n == want), None)
    assert match, f"missing {want} in {sys.argv[1]}"
    (dest / want).write_bytes(zf.read(match))
"#,
                    &sqljs_zip,
                    mount.to_str().unwrap(),
                ])
                .status()
                .expect("extract sql-wasm");
            assert!(extract_status.success(), "extract sql-wasm failed");

            out_root
        })
        .clone()
}

fn serve_static(root: &Path, uri_path: &str) -> Response {
    let trimmed = uri_path.trim_start_matches('/');
    if trimmed.contains("..") || trimmed.contains('\\') {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }
    let clean = trimmed.trim_end_matches('/');
    let direct = root.join(clean);
    let candidate = if direct.is_file() {
        Some(direct)
    } else if root.join(clean).join("index.html").is_file() {
        Some(root.join(clean).join("index.html"))
    } else {
        None
    };
    let Some(file_path) = candidate else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap();
    };
    let bytes = fs::read(&file_path).unwrap_or_default();
    let content_type = match file_path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        _ => "application/octet-stream",
    };
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(bytes))
        .unwrap()
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
        "PATH",
    ] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(paths) = env::var_os("PATH")
        && let Some(found) = env::split_paths(&paths)
            .map(|dir| dir.join("wasm-bindgen"))
            .find(|p| p.is_file())
    {
        return found;
    }
    panic!("wasm-bindgen is required to build the demo physician page");
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}
