//! Opens the duet Android emulator in a window, builds the debug APK, and runs
//! one UI Automator class.
//!
//! ```text
//! cargo run -p android -- build
//! cargo run -p android -- avd
//! cargo run -p android -- window
//! cargo run -p android -- test coda.duet.ModeShellTest
//! cargo run -p android -- test coda.duet.ModeShellTest#clientThenPhysician
//! cargo run -p android -- test --headless coda.duet.ModeShellTest
//! ```
//!
//! Run these in `nix develop .#android`. A headless emulator is not a window.
//! `window` and `test` replace one. `test --headless` is the CI form: it needs
//! no DISPLAY, boots its own headless emulator when none is running, and stops
//! the one it started. It refuses to take over a window someone is using.
//! Every adb and Gradle step targets the duet AVD's own serial, so another
//! emulator is never tested or stopped. `avd` creates the duet AVD when it is
//! missing, as CI does; nothing here replaces or deletes one. There is no
//! no-argument test suite.
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

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

mod build;

unsafe extern "C" {
    fn flock(fd: i32, operation: i32) -> i32;
    fn setsid() -> i32;
    fn kill(pid: i32, signal: i32) -> i32;
}

#[derive(Debug)]
struct Failure {
    message: String,
    code: u8,
    source: Option<io::Error>,
}

impl Failure {
    fn new(code: u8, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code,
            source: None,
        }
    }

    /// What this command was doing when the operating system refused it.
    fn io(doing: impl Into<String>) -> impl FnOnce(io::Error) -> Self {
        let doing = doing.into();
        move |error| Self {
            message: doing,
            code: 1,
            source: Some(error),
        }
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            Some(source) => write!(f, "{}: {source}", self.message),
            None => f.write_str(&self.message),
        }
    }
}

fn main() -> ExitCode {
    match run(env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("{failure}");
            ExitCode::from(failure.code)
        }
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Failure> {
    let args: Vec<String> = args.collect();
    let command = parse(&args)?;
    match command {
        Request::Build => build::build(&build::workspace_root()),
        Request::Avd => create_avd(),
        Request::Window => {
            let _lock = Lock::acquire()?;
            open_emulator(Display::Window).map(drop)
        }
        Request::Test(display, class_name) => {
            let _lock = Lock::acquire()?;
            // A capture from an earlier run would describe a different failure.
            clear_host_failure()?;
            let emulator = open_emulator(display)?;
            let serial = emulator.serial().ok_or_else(|| {
                Failure::new(1, format!("the {} emulator left adb", emulator.avd))
            })?;
            output(
                &emulator.adb,
                &["-s", &serial, "shell", "rm", "-f", DEVICE_SCREEN],
            )?;
            let result = run_instrumentation(&class_name, &serial);
            if result.is_err() {
                for problem in capture_failure(&emulator.adb, &serial) {
                    eprintln!("{problem}");
                }
            }
            result
        }
    }
}

/// Where a failed test leaves the device's logcat and failing screen.
const FAILURE_DIR: &str = "/tmp/duet-android-emulator/failure";
/// `ModeShellTest.FAILURE_SCREEN`: the test saves the screen there when it
/// fails, before the runner closes the app.
const DEVICE_SCREEN: &str = "/data/local/tmp/duet-failure.png";

fn clear_host_failure() -> Result<(), Failure> {
    match fs::remove_dir_all(FAILURE_DIR) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(Failure::io(format!("could not clear {FAILURE_DIR}"))(error)),
    }
}

/// Saves logcat and the test's failing screen before the emulator stops.
/// Returns what could not be saved; the test failure stays the error.
fn capture_failure(adb: &Path, serial: &str) -> Vec<Failure> {
    let dir = Path::new(FAILURE_DIR);
    if let Err(error) = fs::create_dir_all(dir) {
        return vec![Failure::io(format!("could not create {FAILURE_DIR}"))(
            error,
        )];
    }
    let mut problems = Vec::new();
    let logcat = dir.join("logcat.txt");
    match output(adb, &["-s", serial, "logcat", "-d", "-v", "threadtime"]) {
        Ok(text) => match fs::write(&logcat, text) {
            Ok(()) => eprintln!("Saved {}", logcat.display()),
            Err(error) => {
                problems.push(Failure::io(format!("could not write {}", logcat.display()))(error));
            }
        },
        Err(failure) => problems.push(failure),
    }
    let screen = dir.join("screen.png");
    let screen_arg = screen.to_string_lossy();
    match output(adb, &["-s", serial, "pull", DEVICE_SCREEN, &screen_arg]) {
        Ok(_) => eprintln!("Saved {}", screen.display()),
        Err(failure) => problems.push(failure),
    }
    problems
}

enum Request {
    Build,
    Avd,
    Window,
    Test(Display, String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Display {
    Window,
    Headless,
}

fn parse(args: &[String]) -> Result<Request, Failure> {
    match args {
        [command] if command == "build" => Ok(Request::Build),
        [command] if command == "avd" => Ok(Request::Avd),
        [command] if command == "window" => Ok(Request::Window),
        [command, class_name] if command == "test" && valid_selector(class_name) => {
            Ok(Request::Test(Display::Window, class_name.clone()))
        }
        [command, flag, class_name]
            if command == "test" && flag == "--headless" && valid_selector(class_name) =>
        {
            Ok(Request::Test(Display::Headless, class_name.clone()))
        }
        _ => Err(Failure::new(
            2,
            "\
android builds the debug APK, opens the duet emulator window, and runs one UI Automator class.

There is intentionally no no-argument suite.

  cargo run -p android -- build
  cargo run -p android -- avd
  cargo run -p android -- window
  cargo run -p android -- test coda.duet.ModeShellTest
  cargo run -p android -- test coda.duet.ModeShellTest#clientThenPhysician
  cargo run -p android -- test --headless coda.duet.ModeShellTest",
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
        let file = File::create(&path).map_err(Failure::io("could not open the emulator lock"))?;
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
        fs::write(&owner, format!("PID {} :: android\n", std::process::id()))
            .map_err(Failure::io("could not record the emulator owner"))?;
        Ok(Self { file, owner })
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        unsafe { flock(self.file.as_raw_fd(), LOCK_UN) };
        let _ = fs::remove_file(&self.owner);
    }
}

/// The duet emulator this run uses. On drop it stops the emulator only when
/// this run booted it headless; a window, or one already running, stays.
struct Booted {
    adb: PathBuf,
    avd: String,
    pid: u32,
    stop: bool,
}

impl Booted {
    /// The adb serial of this AVD's emulator, never another device's.
    fn serial(&self) -> Option<String> {
        serial_for_avd(&self.adb, &self.avd)
    }
}

impl Drop for Booted {
    fn drop(&mut self) {
        if self.stop {
            stop_emulator(&self.adb, &self.avd, self.pid);
        }
    }
}

/// The one system image `nix develop .#android` provides.
const SYSTEM_IMAGE: &str = "system-images;android-35;google_apis;x86_64";

/// Creates the duet AVD when it is missing. An existing AVD is kept as is.
fn create_avd() -> Result<(), Failure> {
    let emulator = tool("emulator")?;
    let avd = avd_name();
    if output(&emulator, &["-list-avds"])?
        .lines()
        .any(|line| line.trim() == avd)
    {
        println!("AVD '{avd}' already exists.");
        return Ok(());
    }
    let mut child = Command::new(tool("avdmanager")?)
        .args(["create", "avd", "--name", &avd, "--device", "pixel_6"])
        .args(["--package", SYSTEM_IMAGE])
        .stdin(Stdio::piped())
        .spawn()
        .map_err(Failure::io("could not run avdmanager"))?;
    // avdmanager asks whether to make a custom hardware profile.
    if let Some(mut stdin) = child.stdin.take() {
        let _ = io::Write::write_all(&mut stdin, b"no\n");
    }
    let status = child
        .wait()
        .map_err(Failure::io("avdmanager did not finish"))?;
    if status.success() {
        println!("Created AVD '{avd}' from {SYSTEM_IMAGE}.");
        Ok(())
    } else {
        Err(Failure::new(1, "avdmanager create avd failed"))
    }
}

fn avd_name() -> String {
    env::var("DUET_ANDROID_AVD").unwrap_or_else(|_| "duet".to_owned())
}

fn open_emulator(display: Display) -> Result<Booted, Failure> {
    if display == Display::Window && env::var_os("DISPLAY").is_none_or(|value| value.is_empty()) {
        return Err(Failure::new(
            1,
            "The emulator window needs DISPLAY. Refusing to start headless; use test --headless.",
        ));
    }
    let emulator = tool("emulator")?;
    let adb = tool("adb")?;
    let avd = avd_name();
    let avds = output(&emulator, &["-list-avds"])?;
    if !avds.lines().any(|line| line.trim() == avd) {
        return Err(Failure::new(
            1,
            format!("AVD '{avd}' does not exist. Create it with: cargo run -p android -- avd"),
        ));
    }

    let mut pid = None;
    if let Some(qemu) = qemu_for_avd(&avd) {
        match (display, is_headless(&qemu.args)) {
            (Display::Window, true) => {
                println!("Replacing the headless {avd} emulator with a window.");
                stop_emulator(&adb, &avd, qemu.pid);
            }
            (Display::Headless, false) => {
                return Err(Failure::new(
                    1,
                    format!(
                        "The {avd} emulator window is open. Close it, or run test without --headless."
                    ),
                ));
            }
            _ => {
                println!("Using the running {avd} emulator.");
                pid = Some(qemu.pid);
            }
        }
    }

    let log_path = PathBuf::from(match display {
        Display::Window => "/tmp/duet-android-emulator/window.log",
        Display::Headless => "/tmp/duet-android-emulator/headless.log",
    });
    let mut stop = false;
    if pid.is_none() {
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent)
                .map_err(Failure::io("could not create the emulator log directory"))?;
        }
        let log =
            File::create(&log_path).map_err(Failure::io("could not open the emulator log"))?;
        let log_err = log
            .try_clone()
            .map_err(Failure::io("could not open the emulator log"))?;
        let mut command = Command::new(&emulator);
        command.args([
            "-avd",
            &avd,
            "-gpu",
            "swiftshader_indirect",
            "-no-snapshot-load",
            "-no-snapshot-save",
            "-no-boot-anim",
            "-no-audio",
        ]);
        match display {
            Display::Window => {
                command.env(
                    "QT_QPA_PLATFORM",
                    env::var("QT_QPA_PLATFORM").unwrap_or_else(|_| "xcb".to_owned()),
                );
            }
            Display::Headless => {
                command.arg("-no-window");
            }
        }
        command
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
            .map_err(Failure::io("could not start the emulator"))?;
        pid = Some(child.id());
        stop = display == Display::Headless;
    }
    let mut booted = Booted {
        adb,
        avd,
        pid: pid.unwrap_or(0),
        stop,
    };
    let avd = booted.avd.clone();

    let deadline = Instant::now() + Duration::from_secs(300);
    while Instant::now() < deadline {
        if booted
            .serial()
            .is_some_and(|serial| boot_completed(&booted.adb, &serial))
        {
            if display == Display::Window
                && qemu_for_avd(&avd).is_some_and(|qemu| is_headless(&qemu.args))
            {
                return Err(Failure::new(
                    1,
                    format!(
                        "The emulator booted without a window. See {}",
                        log_path.display()
                    ),
                ));
            }
            println!("The {avd} emulator is ready.");
            return Ok(booted);
        }
        if !process_alive(booted.pid) {
            if let Some(qemu) = qemu_for_avd(&avd) {
                booted.pid = qemu.pid;
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
        "The emulator did not finish booting. See {}",
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

fn stop_emulator(adb: &Path, avd: &str, pid: u32) {
    if let Some(serial) = serial_for_avd(adb, avd) {
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

/// Whether a qemu command line runs exactly this AVD: `-avd duet` is not
/// `-avd duet-probe`.
fn runs_avd(args: &str, avd: &str) -> bool {
    let mut words = args.split_whitespace();
    args.contains("qemu-system")
        && words
            .by_ref()
            .skip_while(|word| *word != "-avd")
            .nth(1)
            .is_some_and(|name| name == avd)
}

fn qemu_for_avd(avd: &str) -> Option<Qemu> {
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
        if runs_avd(&args, avd) {
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

fn boot_completed(adb: &Path, serial: &str) -> bool {
    output(
        adb,
        &["-s", serial, "shell", "getprop", "sys.boot_completed"],
    )
    .is_ok_and(|value| value.trim() == "1")
}

/// Asks each attached emulator for its AVD name, so another AVD's emulator
/// is never booted-checked, tested, or killed in this one's place.
fn serial_for_avd(adb: &Path, avd: &str) -> Option<String> {
    emulator_serials(&output(adb, &["devices"]).ok()?)
        .into_iter()
        .find(|serial| {
            output(adb, &["-s", serial, "emu", "avd", "name"])
                .is_ok_and(|reply| reply.lines().next().map(str::trim) == Some(avd))
        })
}

fn emulator_serials(devices: &str) -> Vec<String> {
    devices
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .filter(|serial| serial.starts_with("emulator-"))
        .map(str::to_owned)
        .collect()
}

fn process_alive(pid: u32) -> bool {
    pid != 0 && Path::new(&format!("/proc/{pid}")).exists()
}

fn run_instrumentation(class_name: &str, serial: &str) -> Result<(), Failure> {
    // Gradle does not rebuild the Rust library, so build first: testing a
    // stale or half-generated project would test something else.
    let root = build::workspace_root();
    build::build(&root)?;
    let Some(gradle) = find_gradlew(&root.join("target/dx/mobile")) else {
        return Err(Failure::new(
            1,
            "the Android build did not generate an Android project".to_owned(),
        ));
    };
    println!("Running {class_name} on {serial}");
    // connectedDebugAndroidTest otherwise installs on every attached device.
    let status = Command::new(&gradle)
        .env("ANDROID_SERIAL", serial)
        .current_dir(gradle.parent().unwrap_or(Path::new(".")))
        .arg(":app:connectedDebugAndroidTest")
        .arg(format!(
            "-Pandroid.testInstrumentationRunnerArguments.class={class_name}"
        ))
        .status()
        .map_err(Failure::io("could not run gradlew"))?;
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

fn tool(name: &str) -> Result<PathBuf, Failure> {
    env::var_os("PATH")
        .iter()
        .flat_map(env::split_paths)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| {
            Failure::new(
                1,
                format!("{name} is not on PATH. Run this in nix develop .#android."),
            )
        })
}

fn output(program: &Path, args: &[&str]) -> Result<String, Failure> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(Failure::io(format!("could not run {}", program.display())))?;
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
        assert!(runs_avd("qemu-system-x86_64 -avd duet -no-window", "duet"));
        assert!(!runs_avd(
            "qemu-system-x86_64 -avd duet-probe -no-window",
            "duet"
        ));
        assert!(!runs_avd(
            "qemu-system-x86_64 -avd duet -no-window",
            "duet-probe"
        ));
        assert!(!runs_avd("bash -c emulator -avd duet", "duet"));
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
    fn every_emulator_serial_comes_from_adb() {
        let devices = "List of devices attached\nemulator-5554\tdevice product:sdk\n\
                       R5CT12345\tdevice\nemulator-5556\toffline\n";
        assert_eq!(
            emulator_serials(devices),
            ["emulator-5554", "emulator-5556"]
        );
        assert!(emulator_serials("List of devices attached\n\n").is_empty());
    }

    #[test]
    fn build_window_and_test_are_the_commands() {
        assert!(parse(&[]).is_err());
        assert!(parse(&["test".to_owned()]).is_err());
        assert!(matches!(parse(&["build".to_owned()]), Ok(Request::Build)));
        assert!(matches!(parse(&["avd".to_owned()]), Ok(Request::Avd)));
        assert!(matches!(parse(&["window".to_owned()]), Ok(Request::Window)));
        assert!(matches!(
            parse(&["test".to_owned(), "coda.duet.ModeShellTest".to_owned()]),
            Ok(Request::Test(Display::Window, _))
        ));
    }

    #[test]
    fn only_test_runs_headless() {
        let args = |list: &[&str]| list.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        assert!(matches!(
            parse(&args(&["test", "--headless", "coda.duet.ModeShellTest"])),
            Ok(Request::Test(Display::Headless, _))
        ));
        assert!(parse(&args(&["window", "--headless"])).is_err());
        assert!(parse(&args(&["test", "--headless"])).is_err());
        assert!(parse(&args(&["test", "--headless", "ModeShellTest"])).is_err());
        assert!(parse(&args(&["test", "--window", "coda.duet.ModeShellTest"])).is_err());
    }
}
