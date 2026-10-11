//! Opens the duet Android emulator in a window and runs one UI Automator class.
//!
//! ```text
//! cargo run -p android -- window
//! cargo run -p android -- test coda.duet.ModeShellTest
//! cargo run -p android -- test coda.duet.ModeShellTest#clientThenPhysician
//! ```
//!
//! A headless emulator is not a window. `window` replaces one. This does not
//! create or delete an AVD, and there is no no-argument test suite.

use std::env;
use std::fs::{self, File};
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{Duration, Instant};

const LOCK_EX: i32 = 2;
const LOCK_NB: i32 = 4;
const LOCK_UN: i32 = 8;
const SIGTERM: i32 = 15;
const SIGKILL: i32 = 9;

unsafe extern "C" {
    fn flock(fd: i32, operation: i32) -> i32;
    fn setsid() -> i32;
    fn kill(pid: i32, signal: i32) -> i32;
}

struct Failure {
    message: String,
    code: u8,
}

impl Failure {
    fn new(code: u8, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }
}

fn main() -> ExitCode {
    match run(env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("{}", failure.message);
            ExitCode::from(failure.code)
        }
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Failure> {
    let args: Vec<String> = args.collect();
    let command = parse(&args)?;
    let _lock = Lock::acquire()?;
    match command {
        Request::Window => open_window(),
        Request::Test(class_name) => {
            open_window()?;
            run_instrumentation(&class_name)
        }
    }
}

enum Request {
    Window,
    Test(String),
}

fn parse(args: &[String]) -> Result<Request, Failure> {
    match args {
        [command] if command == "window" => Ok(Request::Window),
        [command, class_name] if command == "test" && valid_selector(class_name) => {
            Ok(Request::Test(class_name.clone()))
        }
        _ => Err(Failure::new(
            2,
            "\
android opens the duet emulator window and runs one UI Automator class.

There is intentionally no no-argument suite.

  cargo run -p android -- window
  cargo run -p android -- test coda.duet.ModeShellTest
  cargo run -p android -- test coda.duet.ModeShellTest#clientThenPhysician",
        )),
    }
}

fn valid_selector(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("coda.duet.") else {
        return false;
    };
    let mut parts = rest.split('#');
    let class_name = parts.next().unwrap_or("");
    let method = parts.next();
    parts.next().is_none()
        && ident(class_name, true)
        && method.is_none_or(|method| ident(method, false))
}

fn ident(value: &str, allow_dot: bool) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || (allow_dot && character == '.')
        })
}

struct Lock {
    file: File,
    owner: PathBuf,
}

impl Lock {
    fn acquire() -> Result<Self, Failure> {
        let path = PathBuf::from("/tmp/duet-android-emulator.lock");
        let file = File::create(&path).map_err(|error| {
            Failure::new(1, format!("could not open the emulator lock: {error}"))
        })?;
        let status = unsafe { flock(file.as_raw_fd(), LOCK_EX | LOCK_NB) };
        if status != 0 {
            let owner = fs::read_to_string("/tmp/duet-android-emulator.lock.owner")
                .unwrap_or_else(|_| "unknown".to_owned());
            return Err(Failure::new(
                1,
                format!(
                    "The duet emulator is already owned by Android verification ({}).\nWait for that command to finish. Do not boot a second duet window.",
                    owner.trim()
                ),
            ));
        }
        let owner = PathBuf::from("/tmp/duet-android-emulator.lock.owner");
        fs::write(&owner, format!("PID {} :: android\n", std::process::id())).map_err(|error| {
            Failure::new(1, format!("could not record the emulator owner: {error}"))
        })?;
        Ok(Self { file, owner })
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        unsafe { flock(self.file.as_raw_fd(), LOCK_UN) };
        let _ = fs::remove_file(&self.owner);
    }
}

fn open_window() -> Result<(), Failure> {
    if env::var_os("DISPLAY").is_none_or(|value| value.is_empty()) {
        return Err(Failure::new(
            1,
            "The emulator window needs DISPLAY. Refusing to start headless.",
        ));
    }
    let emulator = tool("emulator")?;
    let adb = tool("adb")?;
    let avd = env::var("DUET_ANDROID_AVD").unwrap_or_else(|_| "duet".to_owned());
    let avds = output(&emulator, &["-list-avds"])?;
    if !avds.lines().any(|line| line.trim() == avd) {
        return Err(Failure::new(
            1,
            format!("AVD '{avd}' does not exist. This command does not create one."),
        ));
    }

    let mut pid = None;
    if let Some(qemu) = qemu_for_avd(&avd) {
        if is_headless(&qemu.args) {
            println!("Replacing the headless {avd} emulator with a window.");
            stop_headless(&adb, qemu.pid);
        } else {
            println!("Waiting for the existing {avd} emulator window.");
            pid = Some(qemu.pid);
        }
    }

    let log_path = PathBuf::from("/tmp/duet-android-emulator/window.log");
    if pid.is_none() {
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                Failure::new(
                    1,
                    format!("could not create the emulator log directory: {error}"),
                )
            })?;
        }
        let log = File::create(&log_path).map_err(|error| {
            Failure::new(1, format!("could not open the emulator log: {error}"))
        })?;
        let log_err = log.try_clone().map_err(|error| {
            Failure::new(1, format!("could not open the emulator log: {error}"))
        })?;
        let mut command = Command::new(&emulator);
        command
            .args([
                "-avd",
                &avd,
                "-gpu",
                "swiftshader_indirect",
                "-no-snapshot-load",
                "-no-snapshot-save",
                "-no-boot-anim",
                "-no-audio",
            ])
            .env(
                "QT_QPA_PLATFORM",
                env::var("QT_QPA_PLATFORM").unwrap_or_else(|_| "xcb".to_owned()),
            )
            .stdin(Stdio::null())
            .stdout(Stdio::from(log))
            .stderr(Stdio::from(log_err));
        // A snapshot from -no-window does not restore a window. This launch cold-boots.
        // swiftshader draws that window when the AVD has no host GPU.
        unsafe {
            command.pre_exec(|| {
                if setsid() == -1 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let child = command
            .spawn()
            .map_err(|error| Failure::new(1, format!("could not start the emulator: {error}")))?;
        pid = Some(child.id());
    }
    let mut pid = pid.unwrap_or(0);

    let deadline = Instant::now() + Duration::from_secs(300);
    while Instant::now() < deadline {
        if boot_completed(&adb) {
            if qemu_for_avd(&avd).is_some_and(|qemu| is_headless(&qemu.args)) {
                return Err(Failure::new(
                    1,
                    format!(
                        "The emulator booted without a window. See {}",
                        log_path.display()
                    ),
                ));
            }
            println!("The {avd} emulator window is ready.");
            return Ok(());
        }
        if !process_alive(pid) {
            if let Some(qemu) = qemu_for_avd(&avd) {
                pid = qemu.pid;
            } else {
                return Err(exited(&log_path));
            }
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    Err(exited(&log_path))
}

fn exited(log_path: &Path) -> Failure {
    let mut message = format!(
        "The emulator window did not finish booting. See {}",
        log_path.display()
    );
    if let Ok(log) = fs::read_to_string(log_path) {
        let tail = log.lines().rev().take(40).collect::<Vec<_>>();
        let tail = tail.into_iter().rev().collect::<Vec<_>>().join("\n");
        if !tail.is_empty() {
            message.push('\n');
            message.push_str(&tail);
        }
    }
    Failure::new(1, message)
}

fn stop_headless(adb: &Path, pid: u32) {
    if let Some(serial) = emulator_serial(&output(adb, &["devices"]).unwrap_or_default(), false) {
        let _ = Command::new(adb)
            .args(["-s", &serial, "emu", "kill"])
            .status();
    }
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline && process_alive(pid) {
        std::thread::sleep(Duration::from_secs(1));
    }
    if process_alive(pid) {
        unsafe { kill(pid as i32, SIGTERM) };
        std::thread::sleep(Duration::from_secs(1));
    }
    if process_alive(pid) {
        unsafe { kill(pid as i32, SIGKILL) };
    }
}

struct Qemu {
    pid: u32,
    args: String,
}

fn qemu_for_avd(avd: &str) -> Option<Qemu> {
    let needle = format!("-avd {avd}");
    let entries = fs::read_dir("/proc").ok()?;
    for entry in entries.flatten() {
        let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() else {
            continue;
        };
        let Ok(bytes) = fs::read(format!("/proc/{pid}/cmdline")) else {
            continue;
        };
        let args: String = bytes
            .iter()
            .map(|byte| if *byte == 0 { ' ' } else { *byte as char })
            .collect();
        if args.contains("qemu-system")
            && args.split_whitespace().any(|arg| arg == "-avd")
            && args.contains(&needle)
        {
            return Some(Qemu { pid, args });
        }
    }
    None
}

fn is_headless(args: &str) -> bool {
    args.split_whitespace()
        .any(|arg| arg == "-no-window" || arg == "-no-qt")
        || (args.contains("qemu-system-") && args.contains("-headless"))
}

fn boot_completed(adb: &Path) -> bool {
    let Ok(devices) = output(adb, &["devices"]) else {
        return false;
    };
    let Some(serial) = emulator_serial(&devices, true) else {
        return false;
    };
    output(
        adb,
        &["-s", &serial, "shell", "getprop", "sys.boot_completed"],
    )
    .is_ok_and(|value| value.trim() == "1")
}

fn emulator_serial(devices: &str, require_device: bool) -> Option<String> {
    devices.lines().find_map(|line| {
        let mut parts = line.split_whitespace();
        let serial = parts.next()?;
        let state = parts.next().unwrap_or("");
        if serial.starts_with("emulator-") && (!require_device || state == "device") {
            Some(serial.to_owned())
        } else {
            None
        }
    })
}

fn process_alive(pid: u32) -> bool {
    pid != 0 && Path::new(&format!("/proc/{pid}")).exists()
}

fn run_instrumentation(class_name: &str) -> Result<(), Failure> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut gradle = find_gradlew(&root.join("target/dx/mobile"));
    if gradle.is_none() {
        println!("Generating the Android project with scripts/android-debug.sh");
        let status = Command::new(root.join("scripts/android-debug.sh"))
            .current_dir(&root)
            .status()
            .map_err(|error| Failure::new(1, format!("could not run android-debug.sh: {error}")))?;
        if !status.success() {
            return Err(Failure::new(
                1,
                "scripts/android-debug.sh failed".to_owned(),
            ));
        }
        gradle = find_gradlew(&root.join("target/dx/mobile"));
    }
    let Some(gradle) = gradle else {
        return Err(Failure::new(
            1,
            "scripts/android-debug.sh did not generate an Android project".to_owned(),
        ));
    };
    println!("Running {class_name} on the duet emulator window");
    let mut command = Command::new(&gradle);
    command
        .current_dir(gradle.parent().unwrap_or(Path::new(".")))
        .arg(":app:connectedDebugAndroidTest")
        .arg(format!(
            "-Pandroid.testInstrumentationRunnerArguments.class={class_name}"
        ));
    if let Some(home) = java_home() {
        command.env("JAVA_HOME", &home);
        let mut path = home.join("bin").into_os_string();
        path.push(":");
        if let Some(existing) = env::var_os("PATH") {
            path.push(existing);
        }
        command.env("PATH", path);
    }
    let status = command
        .status()
        .map_err(|error| Failure::new(1, format!("could not run gradlew: {error}")))?;
    if status.success() {
        Ok(())
    } else {
        Err(Failure::new(
            1,
            format!("{class_name} failed on the emulator"),
        ))
    }
}

fn find_gradlew(dir: &Path) -> Option<PathBuf> {
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().is_some_and(|name| name == "gradlew") {
                return Some(path);
            }
            if path.is_dir() && path.file_name().is_some_and(|name| name != "build") {
                stack.push(path);
            }
        }
    }
    None
}

fn java_home() -> Option<PathBuf> {
    if tool("java").is_ok() {
        return None;
    }
    let Ok(entries) = fs::read_dir("/nix/store") else {
        return None;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.contains("-android-studio-unwrapped-") {
            let home = entry.path().join("jbr");
            if home.join("bin/java").is_file() {
                return Some(home);
            }
        }
    }
    None
}

fn tool(name: &str) -> Result<PathBuf, Failure> {
    let mut dirs = Vec::new();
    if let Ok(user) = env::var("USER") {
        dirs.push(PathBuf::from(format!("/etc/profiles/per-user/{user}/bin")));
    }
    if let Some(path) = env::var_os("PATH") {
        dirs.extend(env::split_paths(&path));
    }
    for dir in dirs {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(Failure::new(1, format!("{name} is not on PATH.")))
}

fn output(program: &Path, args: &[&str]) -> Result<String, Failure> {
    let output = Command::new(program).args(args).output().map_err(|error| {
        Failure::new(1, format!("could not run {}: {error}", program.display()))
    })?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(Failure::new(
            1,
            format!(
                "{} failed\n{}",
                program.display(),
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_headless_emulator_is_not_a_window() {
        assert!(is_headless(
            "qemu-system-x86_64-headless -avd duet -no-snapshot-save -no-boot-anim -no-audio -no-window"
        ));
        assert!(is_headless("qemu-system-x86_64 -avd duet -no-qt"));
        assert!(!is_headless(
            "qemu-system-x86_64 -avd duet -gpu swiftshader_indirect -no-audio"
        ));
    }

    #[test]
    fn the_test_command_names_one_duet_class() {
        assert!(valid_selector("coda.duet.ModeShellTest"));
        assert!(valid_selector(
            "coda.duet.ModeShellTest#clientThenPhysician"
        ));
        assert!(!valid_selector("ModeShellTest"));
        assert!(!valid_selector("coda.duet."));
        assert!(!valid_selector("coda.duet.ModeShellTest#"));
        assert!(!valid_selector("coda.duet.ModeShellTest#one#two"));
    }

    #[test]
    fn an_emulator_serial_comes_from_adb() {
        let devices = "List of devices attached\nemulator-5554          device product:sdk\n";
        assert_eq!(
            emulator_serial(devices, true).as_deref(),
            Some("emulator-5554")
        );
        assert_eq!(
            emulator_serial("emulator-5554          offline\n", true),
            None
        );
        assert_eq!(
            emulator_serial("emulator-5554          offline\n", false).as_deref(),
            Some("emulator-5554")
        );
    }

    #[test]
    fn window_and_test_are_the_only_commands() {
        assert!(parse(&[]).is_err());
        assert!(parse(&["test".to_owned()]).is_err());
        assert!(matches!(parse(&["window".to_owned()]), Ok(Request::Window)));
        assert!(matches!(
            parse(&["test".to_owned(), "coda.duet.ModeShellTest".to_owned()]),
            Ok(Request::Test(_))
        ));
    }
}
