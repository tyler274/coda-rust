//! Proves the installed mode shell on a booted Android emulator.
//!
//! Client mode must show Duet, Today, You, and Shared, with Duet selected.
//! Physician mode must show Today and Find patient, and must not keep the
//! client tabs. A missing device, an empty WebView, or the wrong screen fails.

use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

#[path = "common/devtools.rs"]
mod devtools;

use devtools::{connect, targets};
use serde_json::Value;

const PACKAGE: &str = "com.example.Mobile";
const ACTIVITY: &str = "dev.dioxus.main.MainActivity";

#[test]
fn the_android_shell_shows_both_modes() {
    let serial = booted_emulator();
    install(&serial);
    adb(&serial, &["shell", "pm", "clear", PACKAGE]);
    let status = adb(
        &serial,
        &[
            "shell",
            "am",
            "start",
            "-W",
            "-n",
            &format!("{PACKAGE}/{ACTIVITY}"),
        ],
    );
    assert!(
        status.status.success(),
        "the mode shell did not start:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );

    let chooser = wait_for(&serial, "Physician", "Client");
    assert!(
        chooser.text.contains("Coda"),
        "the chooser did not show Coda:\n{}",
        chooser.text
    );
    tap_label(&serial, &chooser, "Client");

    let client = wait_for(&serial, "Duet", "Shared");
    let duet = client.tabs.iter().position(|tab| tab.label == "Duet");
    let today = client.tabs.iter().position(|tab| tab.label == "Today");
    let you = client.tabs.iter().position(|tab| tab.label == "You");
    let shared = client.tabs.iter().position(|tab| tab.label == "Shared");
    assert!(
        matches!((duet, today, you, shared), (Some(d), Some(t), Some(y), Some(s)) if d < t && t < y && y < s),
        "client mode did not show Duet, Today, You, Shared in that order:\n{}",
        client.text
    );
    assert!(
        client
            .tabs
            .iter()
            .any(|tab| tab.label == "Duet" && tab.current == "page"),
        "Duet was not the selected tab on open:\n{:?}",
        client.tabs
    );
    assert!(
        !client.text.contains("Find patient"),
        "client mode showed the physician shell:\n{}",
        client.text
    );

    tap_label(&serial, &client, "Switch mode");
    let chooser = wait_for(&serial, "Physician", "Client");
    tap_label(&serial, &chooser, "Physician");
    let physician = wait_for(&serial, "Today", "Find patient");
    assert!(
        physician.tabs.iter().all(|tab| tab.label != "Shared"),
        "physician mode kept the client tabs:\n{}",
        physician.text
    );
    assert!(
        !physician.text.contains("Shared"),
        "physician mode kept the client screen:\n{}",
        physician.text
    );
}

struct Screen {
    text: String,
    tabs: Vec<Control>,
    controls: Vec<Control>,
    screen_x: f64,
    screen_y: f64,
    dpr: f64,
}

#[derive(Clone, Debug)]
struct Control {
    label: String,
    current: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn wait_for(serial: &str, first: &str, second: &str) -> Screen {
    let started = Instant::now();
    let mut last = String::new();
    let mut error = String::new();
    while started.elapsed() < Duration::from_secs(30) {
        match read_screen(serial) {
            Ok(screen) => {
                last = screen.text.clone();
                if screen.text.contains(first) && screen.text.contains(second) {
                    return screen;
                }
            }
            Err(reason) => error = reason,
        }
        thread::sleep(Duration::from_millis(400));
    }
    panic!("the screen never showed {first} and {second}. Last text:\n{last}\nLast error:\n{error}");
}

fn read_screen(serial: &str) -> Result<Screen, String> {
    let output = adb(serial, &["shell", "pidof", PACKAGE]);
    if !output.status.success() {
        return Err(format!(
            "pidof failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let pid = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\r')
        .to_owned();
    if pid.is_empty() {
        return Err("the mode shell process was not running".to_owned());
    }
    let forwarded = adb(
        serial,
        &[
            "forward",
            "tcp:0",
            &format!("localabstract:webview_devtools_remote_{pid}"),
        ],
    );
    if !forwarded.status.success() {
        return Err(format!(
            "DevTools forward failed: {}",
            String::from_utf8_lossy(&forwarded.stderr)
        ));
    }
    let port = String::from_utf8_lossy(&forwarded.stdout)
        .trim()
        .parse()
        .map_err(|_| {
            format!(
                "adb did not assign a DevTools port:\n{}",
                String::from_utf8_lossy(&forwarded.stdout)
            )
        })?;
    let target = targets(port)?
        .into_iter()
        .next()
        .ok_or_else(|| "DevTools listed no page".to_owned())?;
    let mut page = connect(port, &target.socket_path)?;
    let raw = page
        .evaluate(
            r#"JSON.stringify({
                dpr: window.devicePixelRatio || 1,
                text: document.body ? document.body.innerText : "",
                controls: [...document.querySelectorAll("button")].map((button) => {
                    const rect = button.getBoundingClientRect();
                    return {
                        t: button.innerText,
                        current: button.getAttribute("aria-current") || "",
                        x: rect.x, y: rect.y, w: rect.width, h: rect.height
                    };
                })
            })"#,
        )
        .as_str()
        .unwrap_or("")
        .to_owned();
    if raw.is_empty() {
        return Err("the WebView document was empty".to_owned());
    }
    let value: Value = serde_json::from_str(&raw).unwrap_or_else(|error| {
        panic!("the page description was not JSON: {error}\n{raw}");
    });
    let controls = value
        .get("controls")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(control)
        .collect::<Vec<_>>();
    let text = value
        .get("text")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    if text.trim().is_empty() {
        return Err("the WebView document was empty".to_owned());
    }
    Ok(Screen {
        tabs: controls
            .iter()
            .filter(|control| matches!(control.label.as_str(), "Duet" | "Today" | "You" | "Shared"))
            .cloned()
            .collect(),
        text,
        controls,
        screen_x: target.screen_x,
        screen_y: target.screen_y,
        dpr: value.get("dpr").and_then(Value::as_f64).unwrap_or(1.0),
    })
}

fn control(value: &Value) -> Option<Control> {
    Some(Control {
        label: value.get("t")?.as_str()?.trim().to_owned(),
        current: value
            .get("current")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
        x: value.get("x")?.as_f64()?,
        y: value.get("y")?.as_f64()?,
        width: value.get("w")?.as_f64()?,
        height: value.get("h")?.as_f64()?,
    })
}

fn tap_label(serial: &str, screen: &Screen, label: &str) {
    let control = screen
        .controls
        .iter()
        .find(|control| control.label == label)
        .unwrap_or_else(|| panic!("{label} was not a button on the screen:\n{}", screen.text));
    assert!(
        control.width > 0.0 && control.height > 0.0,
        "{label} was in the document but had no visible bounds"
    );
    let x = screen.screen_x + (control.x + control.width / 2.0) * screen.dpr;
    let y = screen.screen_y + (control.y + control.height / 2.0) * screen.dpr;
    let status = adb(
        serial,
        &[
            "shell",
            "input",
            "tap",
            &x.round().to_string(),
            &y.round().to_string(),
        ],
    );
    assert!(status.status.success(), "tap on {label} failed");
}

fn booted_emulator() -> String {
    if let Some(serial) = device_serial() {
        return serial;
    }
    let emulator = find_on_path("emulator").unwrap_or_else(|| {
        panic!("no Android emulator is booted, and the emulator tool is not on PATH")
    });
    let mut child = Command::new(emulator);
    child.args([
        "-avd",
        "duet",
        "-no-snapshot-save",
        "-no-boot-anim",
        "-no-audio",
        "-no-window",
    ]);
    let child = child.spawn().expect("failed to boot the duet emulator");
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(180) {
        if let Some(serial) = device_serial() {
            let boot = adb_text(&serial, &["shell", "getprop", "sys.boot_completed"]);
            if boot.trim() == "1" {
                return serial;
            }
        }
        thread::sleep(Duration::from_secs(2));
    }
    let _ = child.id();
    panic!("the duet emulator did not finish booting");
}

fn device_serial() -> Option<String> {
    let output = Command::new(adb_bin()).args(["devices"]).output().ok()?;
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let serial = parts.next()?;
            (parts.next()? == "device").then(|| serial.to_owned())
        })
        .find(|serial| serial.starts_with("emulator-"))
}

fn install(serial: &str) {
    let apk = PathBuf::from(env_apk());
    assert!(
        apk.is_file(),
        "the mode-shell APK is missing at {}",
        apk.display()
    );
    let status = adb(serial, &["install", "-r", apk.to_str().unwrap()]);
    assert!(
        status.status.success(),
        "install failed:\n{}",
        String::from_utf8_lossy(&status.stdout)
    );
}

fn env_apk() -> String {
    if let Ok(path) = std::env::var("CODA_ANDROID_APK") {
        return path;
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    root.join("target/dx/mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk")
        .to_string_lossy()
        .into_owned()
}

fn adb(serial: &str, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(adb_bin());
    command.args(["-s", serial]);
    command.args(args);
    command.output().expect("adb")
}

fn adb_text(serial: &str, args: &[&str]) -> String {
    let output = adb(serial, args);
    assert!(
        output.status.success(),
        "adb {} failed:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\r')
        .to_owned()
}

fn adb_bin() -> PathBuf {
    find_on_path("adb").expect("adb is required to validate the Android shell")
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}
