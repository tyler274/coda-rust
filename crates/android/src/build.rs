//! Build the mode-shell debug APK.
//!
//! `nix develop .#android` supplies the SDK, the NDK, a JDK and a Rust
//! toolchain with the Android standard library. `dx build --android` asks
//! Gradle for build-tools 34; the Nix SDK only has 37, so a build-tools failure is patched and `gradlew assembleDebug`
//! continues. A Gradle download that outlasts the wrapper's 10 s timeout is
//! recovered the same way. Patch steps take the generated tree so tests can run without
//! `dx` or an emulator.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::Failure;

const BUILD_TOOLS: &str = "37.0.0";

const WRAPPER_TIMEOUT: &str = "networkTimeout=120000";

const MAVEN_CENTRAL_MIRROR: &str = "https://maven-central.storage-download.googleapis.com/maven2/";

const HARDWARE_KEYS: &str = "\
    // duet-hardware-keys
    override fun dispatchKeyEvent(event: KeyEvent): Boolean {
        if (event.keyCode != KeyEvent.KEYCODE_BACK && ::mWebView.isInitialized && duetInsertHardwareKey(mWebView, event)) {
            return true
        }
        return super.dispatchKeyEvent(event)
    }
";

const HEALTH_PERMISSIONS: &str = "\
    <uses-permission android:name=\"android.permission.health.READ_STEPS\" />
    <uses-permission android:name=\"android.permission.health.READ_ACTIVE_CALORIES_BURNED\" />
    <uses-permission android:name=\"android.permission.health.READ_EXERCISE\" />
    <uses-permission android:name=\"android.permission.health.READ_DISTANCE\" />
    <uses-permission android:name=\"android.permission.health.READ_NUTRITION\" />
    <uses-permission android:name=\"android.permission.health.READ_HYDRATION\" />
    <uses-permission android:name=\"android.permission.health.READ_RESTING_HEART_RATE\" />
    <uses-permission android:name=\"android.permission.health.READ_WEIGHT\" />
    <uses-permission android:name=\"android.permission.health.READ_SLEEP\" />
    <uses-permission android:name=\"android.permission.health.READ_BLOOD_PRESSURE\" />
    <uses-permission android:name=\"android.permission.health.READ_BODY_FAT\" />
    <uses-permission android:name=\"android.permission.health.READ_LEAN_BODY_MASS\" />
    <uses-permission android:name=\"android.permission.health.READ_HEIGHT\" />
    <uses-permission android:name=\"android.permission.health.READ_BLOOD_GLUCOSE\" />
    <uses-permission android:name=\"android.permission.health.READ_BODY_TEMPERATURE\" />
    <uses-permission android:name=\"android.permission.health.READ_BASAL_BODY_TEMPERATURE\" />
    <uses-permission android:name=\"android.permission.health.READ_OXYGEN_SATURATION\" />
    <uses-permission android:name=\"android.permission.health.READ_VO2_MAX\" />
    <queries>
        <package android:name=\"com.google.android.apps.healthdata\" />
    </queries>
";

const NOTIFICATION_PERMISSIONS: &str = "\
    <uses-permission android:name=\"android.permission.POST_NOTIFICATIONS\" />
";

const PERMISSION_RATIONALE: &str = "\
    <intent-filter>
                <action android:name=\"androidx.health.ACTION_SHOW_PERMISSIONS_RATIONALE\" />
            </intent-filter>
        </activity>
        <activity-alias android:name=\"ViewPermissionUsageActivity\" android:exported=\"true\" android:targetActivity=\"dev.dioxus.main.MainActivity\" android:permission=\"android.permission.START_VIEW_PERMISSION_USAGE\">
            <intent-filter>
                <action android:name=\"android.intent.action.VIEW_PERMISSION_USAGE\" />
                <category android:name=\"android.intent.category.HEALTH_PERMISSIONS\" />
            </intent-filter>
        </activity-alias>";

pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Returns where `dx build --android` places its generated Gradle project, honoring
/// `CARGO_TARGET_DIR` when set and falling back to `<workspace>/target/dx/mobile`.
pub fn dx_generated_dir(workspace: &Path) -> PathBuf {
    dx_generated_dir_with(workspace, env::var_os("CARGO_TARGET_DIR").as_deref())
}

fn dx_generated_dir_with(workspace: &Path, cargo_target_dir: Option<&std::ffi::OsStr>) -> PathBuf {
    let target = cargo_target_dir
        .filter(|dir| !dir.is_empty())
        .map(PathBuf::from)
        .map(|dir| {
            if dir.is_relative() {
                workspace.join(dir)
            } else {
                dir
            }
        })
        .unwrap_or_else(|| workspace.join("target"));
    target.join("dx/mobile")
}

pub fn build(workspace: &Path) -> Result<(), Failure> {
    run_dx(workspace)?;
    let generated = dx_generated_dir(workspace);
    patch_generated_android_project(workspace, &generated)?;
    assemble_debug(&generated)
}

pub fn patch_generated_android_project(workspace: &Path, generated: &Path) -> Result<(), Failure> {
    let gradle_files = find(generated, |path| {
        path.file_name()
            .is_some_and(|name| name == "build.gradle.kts")
    });
    if gradle_files.is_empty() {
        return Err(Failure::new(
            1,
            "dx did not generate an Android Gradle project",
        ));
    }
    for path in &gradle_files {
        rewrite(path, patch_build_tools)?;
        rewrite(path, patch_maven_repositories)?;
    }
    for path in find(generated, |path| {
        path.file_name()
            .is_some_and(|name| name == "gradle-wrapper.properties")
    }) {
        rewrite(&path, patch_wrapper_timeout)?;
    }

    let manifests = find(generated, is_main_manifest);
    for path in &manifests {
        rewrite(path, patch_cleartext_and_input)?;
    }

    let keys = workspace.join("platform/android/HardwareKeyDispatch.kt");
    for activity in find(generated, |path| {
        path.file_name()
            .is_some_and(|name| name == "WryActivity.kt")
    }) {
        let destination = activity
            .parent()
            .unwrap_or(Path::new("."))
            .join("HardwareKeyDispatch.kt");
        copy_file(&keys, &destination)?;
        rewrite(&activity, insert_hardware_keys)?;
    }

    let bridge = workspace.join("platform/android/HealthConnectBridge.kt");
    let Some(manifest) = manifests.first() else {
        return Err(Failure::new(
            1,
            "Health Connect bridge or generated manifest is missing",
        ));
    };
    if !bridge.is_file() {
        return Err(Failure::new(
            1,
            "Health Connect bridge or generated manifest is missing",
        ));
    }
    let notify = workspace.join("platform/android/NotificationBridge.kt");
    if !notify.is_file() {
        return Err(Failure::new(
            1,
            "the Android notification bridge is missing",
        ));
    }
    let java_dir = manifest.parent().unwrap_or(Path::new("."));
    let kotlin_dir = java_dir.join("java/coda/duet");
    copy_file(&bridge, &kotlin_dir.join("HealthConnectBridge.kt"))?;
    copy_file(&notify, &kotlin_dir.join("NotificationBridge.kt"))?;
    let secure = workspace.join("platform/android/SecureRetentionBridge.kt");
    if !secure.is_file() {
        return Err(Failure::new(
            1,
            "the Android secure retention bridge is missing",
        ));
    }
    let doctor_pdf = workspace.join("platform/android/DoctorPdfProvider.kt");
    if !doctor_pdf.is_file() {
        return Err(Failure::new(
            1,
            "the Android doctor PDF provider is missing",
        ));
    }
    copy_file(&secure, &kotlin_dir.join("SecureRetentionBridge.kt"))?;
    copy_file(&doctor_pdf, &kotlin_dir.join("DoctorPdfProvider.kt"))?;

    let Some(app_gradle) = gradle_files.iter().find(|path| {
        is_app_gradle(path)
            && fs::read_to_string(path).is_ok_and(|text| text.contains("androidx.webkit"))
    }) else {
        return Err(Failure::new(1, "the app Gradle file was not generated"));
    };
    rewrite(app_gradle, patch_app_gradle)?;

    let test_src = workspace.join("platform/android/ModeShellTest.kt");
    if !test_src.is_file() {
        return Err(Failure::new(
            1,
            "the mode-shell UI Automator test is missing",
        ));
    }
    let test_dir = java_dir.join("../androidTest/java/coda/duet");
    copy_file(&test_src, &test_dir.join("ModeShellTest.kt"))?;
    wire_android_manifest(workspace, manifest)?;
    rewrite(manifest, patch_health_manifest)?;
    Ok(())
}

fn wire_android_manifest(workspace: &Path, manifest: &Path) -> Result<(), Failure> {
    let script = workspace.join("scripts/wire-duet-android.py");
    let status = Command::new("python")
        .arg(&script)
        .arg(manifest)
        .status()
        .map_err(Failure::io("could not run python"))?;
    if status.success() {
        Ok(())
    } else {
        Err(Failure::new(1, "scripts/wire-duet-android.py failed"))
    }
}

fn run_dx(workspace: &Path) -> Result<(), Failure> {
    let dx = dx_program()?;
    let mut command = Command::new(&dx);
    command.current_dir(workspace).args([
        "build",
        "--android",
        "--package",
        "mobile",
        "--features",
        "mobile",
    ]);
    let output = command
        .output()
        .map_err(Failure::io(format!("could not run {}", dx.display())))?;
    let mut log = String::from_utf8_lossy(&output.stdout).into_owned();
    log.push_str(&String::from_utf8_lossy(&output.stderr));
    print!("{log}");
    if !output.status.success() && !assembly_continues(&log) {
        let code = output.status.code().unwrap_or(1);
        return Err(Failure::new(
            u8::try_from(code).unwrap_or(1),
            format!("dx build failed ({code})"),
        ));
    }
    Ok(())
}

/// dx has generated the Gradle project by the time it assembles, so three of its
/// Gradle failures are recoverable: the missing build-tools 34 (patched to 37),
/// the wrapper's 10 s download timeout (patched to 120 s), and Maven Central
/// blocking/rate-limiting on shared CI IPs (patched to try Google's Maven Central
/// mirror before `mavenCentral()`). `gradlew assembleDebug` then retries with
/// the patched project.
fn assembly_continues(log: &str) -> bool {
    log.contains("build-tools")
        || (log.contains("services.gradle.org/distributions/") && log.contains("timeout"))
        || (log.contains("Could not resolve all artifacts for configuration 'classpath'")
            && log.contains("repo.maven.apache.org/maven2/"))
}

fn dx_program() -> Result<PathBuf, Failure> {
    if let Some(dx) = env::var_os("DX")
        && !dx.is_empty()
    {
        return Ok(PathBuf::from(dx));
    }
    super::tool("dx").map_err(|_| Failure::new(1, "dx, the Dioxus CLI, is not on PATH"))
}

fn assemble_debug(generated: &Path) -> Result<(), Failure> {
    let Some(gradle) = super::find_gradlew(generated) else {
        return Err(Failure::new(1, "gradlew was not generated"));
    };
    let mut command = Command::new(&gradle);
    command
        .current_dir(gradle.parent().unwrap_or(Path::new(".")))
        .arg("assembleDebug");
    let status = command
        .status()
        .map_err(Failure::io("could not run gradlew"))?;
    if !status.success() {
        let code = status.code().unwrap_or(1);
        return Err(Failure::new(
            u8::try_from(code).unwrap_or(1),
            "gradlew assembleDebug failed",
        ));
    }
    if let Some(apk) = find(generated, |path| {
        path.file_name().is_some_and(|name| name == "app-debug.apk")
    })
    .into_iter()
    .next()
    {
        println!("{}", apk.display());
    }
    Ok(())
}

/// The generated wrapper gives up on the Gradle download after 10 s, which a
/// slow CI network exceeds before any test runs.
fn patch_wrapper_timeout(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.starts_with("networkTimeout=") {
                WRAPPER_TIMEOUT
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// Prepends Google's official Maven Central mirror ahead of `mavenCentral()` so
/// shared CI runners do not fail when Cloudflare blocks `repo.maven.apache.org`.
fn patch_maven_repositories(text: &str) -> String {
    if !text.contains("mavenCentral()") || text.contains(MAVEN_CENTRAL_MIRROR) {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len() + 160);
    for line in text.split_inclusive('\n') {
        if line.trim() == "mavenCentral()" {
            let indent_len = line.len() - line.trim_start().len();
            let indent = &line[..indent_len];
            out.push_str(indent);
            out.push_str(&format!(
                "maven {{ url = uri(\"{MAVEN_CENTRAL_MIRROR}\") }}\n"
            ));
        }
        out.push_str(line);
    }
    out
}

fn patch_build_tools(text: &str) -> String {
    if text.contains("buildToolsVersion") {
        replace_build_tools_version(text)
    } else if text.contains("compileSdk =") {
        insert_build_tools(text)
    } else {
        text.to_owned()
    }
}

fn replace_build_tools_version(text: &str) -> String {
    let needle = "buildToolsVersion";
    let mut out = String::new();
    let mut rest = text;
    while let Some(index) = rest.find(needle) {
        out.push_str(&rest[..index]);
        let after = &rest[index + needle.len()..];
        let trimmed = after.trim_start_matches(' ');
        if let Some(after_eq) = trimmed.strip_prefix('=') {
            let after_eq = after_eq.trim_start_matches(' ');
            if let Some(value) = after_eq.strip_prefix('"')
                && let Some(end) = value.find('"')
            {
                out.push_str(&format!("buildToolsVersion = \"{BUILD_TOOLS}\""));
                rest = &value[end + 1..];
                continue;
            }
        }
        out.push_str(needle);
        rest = after;
    }
    out.push_str(rest);
    out
}

fn insert_build_tools(text: &str) -> String {
    let needle = "compileSdk = ";
    let mut out = String::new();
    let mut rest = text;
    while let Some(index) = rest.find(needle) {
        let start = index + needle.len();
        let bytes = rest.as_bytes();
        let mut end = start;
        while end < bytes.len() && (bytes[end].is_ascii_digit() || bytes[end] == b'.') {
            end += 1;
        }
        if end == start {
            out.push_str(&rest[..start]);
            rest = &rest[start..];
            continue;
        }
        out.push_str(&rest[..end]);
        out.push_str(&format!("\n    buildToolsVersion = \"{BUILD_TOOLS}\""));
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

fn patch_cleartext_and_input(text: &str) -> String {
    let mut text = if text.contains("usesCleartextTraffic") {
        text.to_owned()
    } else {
        text.replace(
            "<application",
            "<application android:usesCleartextTraffic=\"true\"",
        )
    };
    if !text.contains("windowSoftInputMode") {
        text = text.replace(
            "<activity ",
            "<activity android:windowSoftInputMode=\"adjustResize\" ",
        );
    }
    text
}

fn insert_hardware_keys(activity: &str) -> String {
    if activity.contains("duet-hardware-keys") {
        return activity.to_owned();
    }
    let mut out = String::new();
    let mut done = false;
    for line in activity.split_inclusive('\n') {
        if !done && line.contains("fun getAppClass") {
            out.push_str(HARDWARE_KEYS);
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out.push('\n');
            done = true;
        }
        out.push_str(line);
    }
    out
}

fn patch_app_gradle(text: &str) -> String {
    let mut text = text.to_owned();
    if !text.contains("connect-client") {
        text = text.replace(
            "dependencies {",
            "dependencies {\n    implementation(\"androidx.health.connect:connect-client:1.1.0-alpha11\")\n    implementation(\"org.jetbrains.kotlinx:kotlinx-coroutines-android:1.10.2\")",
        );
    }
    if !text.contains("androidx.core:core") {
        text = text.replace(
            "dependencies {",
            "dependencies {\n    implementation(\"androidx.core:core-ktx:1.15.0\")",
        );
    }
    if !text.contains("uiautomator:uiautomator") {
        text = text.replace(
            "dependencies {",
            "dependencies {\n    androidTestImplementation(\"androidx.test.ext:junit:1.2.1\")\n    androidTestImplementation(\"androidx.test:runner:1.6.2\")\n    androidTestImplementation(\"androidx.test.uiautomator:uiautomator:2.3.0\")\n    androidTestUtil(\"androidx.test:orchestrator:1.5.1\")",
        );
    }
    if !text.contains("androidx.test:orchestrator") {
        text = text.replace(
            "androidTestImplementation(\"androidx.test.uiautomator:uiautomator:2.3.0\")",
            "androidTestImplementation(\"androidx.test.uiautomator:uiautomator:2.3.0\")\n    androidTestUtil(\"androidx.test:orchestrator:1.5.1\")",
        );
    }
    if !text.contains("testInstrumentationRunner") {
        text = text.replace(
            "versionName = \"0.1.0\"",
            "versionName = \"0.1.0\"\n        testInstrumentationRunner = \"androidx.test.runner.AndroidJUnitRunner\"\n        testInstrumentationRunnerArguments[\"clearPackageData\"] = \"true\"",
        );
    }
    if !text.contains("clearPackageData") {
        text = text.replace(
            "testInstrumentationRunner = \"androidx.test.runner.AndroidJUnitRunner\"",
            "testInstrumentationRunner = \"androidx.test.runner.AndroidJUnitRunner\"\n        testInstrumentationRunnerArguments[\"clearPackageData\"] = \"true\"",
        );
    }
    if !text.contains("ANDROIDX_TEST_ORCHESTRATOR") {
        text = text.replace(
            "sourceSets {",
            "testOptions {\n        execution = \"ANDROIDX_TEST_ORCHESTRATOR\"\n    }\n    sourceSets {",
        );
    }
    text = text.replace("compileSdk = 34", "compileSdk = 35");
    text.replace("minSdk = 24", "minSdk = 26")
}

fn patch_health_manifest(text: &str) -> String {
    let mut text = text.to_owned();
    if !text.contains("android.permission.health.READ_STEPS") {
        text = append_after_lines(&text, "android.permission.INTERNET", HEALTH_PERMISSIONS);
    }
    if !text.contains("android.permission.POST_NOTIFICATIONS") {
        text = append_after_lines(
            &text,
            "android.permission.INTERNET",
            NOTIFICATION_PERMISSIONS,
        );
    }
    if !text.contains("VIEW_PERMISSION_USAGE") {
        text = text.replace("</activity>", PERMISSION_RATIONALE);
    }
    text
}

fn append_after_lines(text: &str, needle: &str, insertion: &str) -> String {
    let mut out = String::new();
    for line in text.split_inclusive('\n') {
        out.push_str(line);
        if line.contains(needle) {
            if !line.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(insertion);
            if !insertion.ends_with('\n') {
                out.push('\n');
            }
        }
    }
    out
}

fn rewrite(path: &Path, patch: impl Fn(&str) -> String) -> Result<(), Failure> {
    let text = fs::read_to_string(path)
        .map_err(Failure::io(format!("could not read {}", path.display())))?;
    fs::write(path, patch(&text))
        .map_err(Failure::io(format!("could not write {}", path.display())))
}

fn copy_file(from: &Path, to: &Path) -> Result<(), Failure> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).map_err(Failure::io(format!(
            "could not create {}",
            parent.display()
        )))?;
    }
    fs::copy(from, to).map_err(Failure::io(format!(
        "could not copy {} to {}",
        from.display(),
        to.display()
    )))?;
    Ok(())
}

fn is_main_manifest(path: &Path) -> bool {
    let mut components = path.components().rev();
    components
        .next()
        .is_some_and(|component| component.as_os_str() == "AndroidManifest.xml")
        && components
            .next()
            .is_some_and(|component| component.as_os_str() == "main")
        && components
            .next()
            .is_some_and(|component| component.as_os_str() == "src")
}

fn is_app_gradle(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|name| name == "build.gradle.kts")
        && path
            .parent()
            .and_then(|parent| parent.file_name())
            .is_some_and(|name| name == "app")
}

fn find(root: &Path, predicate: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "build") {
                    continue;
                }
                stack.push(path);
            } else if predicate(&path) {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "android-patch-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|duration| duration.as_nanos())
                    .unwrap_or(0)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn only_patched_gradle_failures_let_assembly_continue() {
        assert!(assembly_continues(
            "Failed to assemble apk: Exception in thread \"main\" java.io.IOException: Downloading from https://services.gradle.org/distributions/gradle-9.1.0-bin.zip failed: timeout (10000ms)"
        ));
        assert!(assembly_continues(
            "build-tools;34.0.0 Android SDK Build-Tools 34"
        ));
        assert!(assembly_continues(
            "Could not resolve all artifacts for configuration 'classpath'.\n  - https://repo.maven.apache.org/maven2/org/jetbrains/kotlin/kotlin-gradle-plugin/2.0.20/kotlin-gradle-plugin-2.0.20.pom"
        ));
        assert!(!assembly_continues("error[E0425]: cannot find value"));
        assert!(!assembly_continues(
            "Could not resolve all artifacts for configuration 'classpath'.\n  - https://example.invalid/missing.pom"
        ));
    }

    #[test]
    fn maven_mirror_patch_prepends_google_mirror_and_is_idempotent() {
        let input = "\
buildscript {
    repositories {
        google()
        mavenCentral()
    }
}
allprojects {
    repositories {
        google()
        mavenCentral()
    }
}
";
        let patched = patch_maven_repositories(input);
        assert_eq!(patched.matches(MAVEN_CENTRAL_MIRROR).count(), 2);
        let mirror_pos = patched.find(MAVEN_CENTRAL_MIRROR).unwrap();
        let central_pos = patched.find("mavenCentral()").unwrap();
        assert!(mirror_pos < central_pos);
        assert_eq!(patch_maven_repositories(&patched), patched);
    }

    #[test]
    fn dx_generated_dir_honors_cargo_target_dir() {
        let workspace = Path::new("/repo");
        assert_eq!(
            dx_generated_dir_with(workspace, None),
            PathBuf::from("/repo/target/dx/mobile")
        );
        assert_eq!(
            dx_generated_dir_with(workspace, Some(std::ffi::OsStr::new(""))),
            PathBuf::from("/repo/target/dx/mobile")
        );
        assert_eq!(
            dx_generated_dir_with(workspace, Some(std::ffi::OsStr::new("/tmp/custom-target"))),
            PathBuf::from("/tmp/custom-target/dx/mobile")
        );
        assert_eq!(
            dx_generated_dir_with(workspace, Some(std::ffi::OsStr::new("alt-target"))),
            PathBuf::from("/repo/alt-target/dx/mobile")
        );
    }

    #[test]
    fn gradle_and_manifest_patches_land_on_a_fixture_tree() {
        let temp = TempDir::new();
        let workspace = temp.0.join("workspace");
        let generated = workspace.join("target/dx/mobile");
        fs::create_dir_all(workspace.join("platform/android")).unwrap();
        fs::write(
            workspace.join("platform/android/HardwareKeyDispatch.kt"),
            "fun duetInsertHardwareKey() {}\n",
        )
        .unwrap();
        fs::write(
            workspace.join("platform/android/HealthConnectBridge.kt"),
            "class HealthConnectBridge\n",
        )
        .unwrap();
        fs::write(
            workspace.join("platform/android/NotificationBridge.kt"),
            "class NotificationBridge\n",
        )
        .unwrap();
        fs::write(
            workspace.join("platform/android/ModeShellTest.kt"),
            "class ModeShellTest\n",
        )
        .unwrap();
        fs::write(
            workspace.join("platform/android/SecureRetentionBridge.kt"),
            "class SecureRetentionBridge\n",
        )
        .unwrap();
        fs::write(
            workspace.join("platform/android/DoctorPdfProvider.kt"),
            "class DoctorPdfProvider\n",
        )
        .unwrap();
        fs::create_dir_all(workspace.join("scripts")).unwrap();
        fs::write(
            workspace.join("scripts/wire-duet-android.py"),
            "import pathlib, sys\npath = pathlib.Path(sys.argv[1])\npath.write_text(path.read_text() + '<!-- duet-android-wired -->\\n')\n",
        )
        .unwrap();

        let root_gradle = generated.join("build.gradle.kts");
        fs::create_dir_all(root_gradle.parent().unwrap()).unwrap();
        fs::write(
            &root_gradle,
            "buildscript {\n    repositories {\n        google()\n        mavenCentral()\n    }\n}\n",
        )
        .unwrap();

        let library = generated.join("lib/build.gradle.kts");
        fs::create_dir_all(library.parent().unwrap()).unwrap();
        fs::write(&library, "android {\n    compileSdk = 34\n}\n").unwrap();

        let app = generated.join("app/build.gradle.kts");
        fs::create_dir_all(app.parent().unwrap()).unwrap();
        fs::write(
            &app,
            "\
android {
    compileSdk = 34
    defaultConfig {
        minSdk = 24
        versionName = \"0.1.0\"
    }
    sourceSets {
    }
}
dependencies {
    implementation(\"androidx.webkit:webkit:1.12.1\")
}
",
        )
        .unwrap();

        let manifest = generated.join("app/src/main/AndroidManifest.xml");
        fs::create_dir_all(manifest.parent().unwrap()).unwrap();
        fs::write(
            &manifest,
            "\
<manifest>
    <uses-permission android:name=\"android.permission.INTERNET\" />
    <application>
        <activity android:name=\"dev.dioxus.main.MainActivity\">
        </activity>
    </application>
</manifest>
",
        )
        .unwrap();

        let activity = generated.join("app/src/main/kotlin/WryActivity.kt");
        fs::create_dir_all(activity.parent().unwrap()).unwrap();
        fs::write(
            &activity,
            "class WryActivity {\n    fun getAppClass(): String = \"app\"\n}\n",
        )
        .unwrap();

        let wrapper = generated.join("app/gradle/wrapper/gradle-wrapper.properties");
        fs::create_dir_all(wrapper.parent().unwrap()).unwrap();
        fs::write(
            &wrapper,
            "distributionUrl=https\\://services.gradle.org/distributions/gradle-9.1.0-bin.zip\nnetworkTimeout=10000\n",
        )
        .unwrap();

        patch_generated_android_project(&workspace, &generated).unwrap();

        let wrapper = fs::read_to_string(wrapper).unwrap();
        assert!(wrapper.contains("\nnetworkTimeout=120000\n"));
        assert!(!wrapper.contains("networkTimeout=10000"));
        assert!(wrapper.contains("gradle-9.1.0-bin.zip"));

        let root_gradle = fs::read_to_string(root_gradle).unwrap();
        let library = fs::read_to_string(library).unwrap();
        let app = fs::read_to_string(app).unwrap();
        let manifest = fs::read_to_string(manifest).unwrap();
        assert!(root_gradle.contains(MAVEN_CENTRAL_MIRROR));
        assert!(library.contains("buildToolsVersion = \"37.0.0\""));
        assert!(app.contains("buildToolsVersion = \"37.0.0\""));
        assert!(manifest.contains("android:usesCleartextTraffic=\"true\""));
        assert!(manifest.contains("android.permission.health.READ_STEPS"));
        assert!(manifest.contains("android.permission.POST_NOTIFICATIONS"));
        assert!(manifest.contains("windowSoftInputMode=\"adjustResize\""));
        assert!(manifest.contains("<!-- duet-android-wired -->"));
        assert!(
            generated
                .join("app/src/main/java/coda/duet/SecureRetentionBridge.kt")
                .is_file()
        );
        assert!(
            generated
                .join("app/src/main/java/coda/duet/DoctorPdfProvider.kt")
                .is_file()
        );
    }
}
