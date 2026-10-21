// Link the real Swift bridge into every iOS binary using duet-client (including mobile).
use std::{env, path::PathBuf, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=../../platform/ios/SecureRetentionBridge.swift");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("ios") {
        return;
    }
    let target = env::var("TARGET").unwrap();
    let simulator = target.contains("sim") || target.starts_with("x86_64");
    let sdk = if simulator {
        "iphonesimulator"
    } else {
        "iphoneos"
    };
    let arch = if target.starts_with("aarch64") {
        "arm64"
    } else {
        "x86_64"
    };
    let swift_target = format!(
        "{arch}-apple-ios15.0{}",
        if simulator { "-simulator" } else { "" }
    );
    let output = Command::new("xcrun")
        .args(["--sdk", sdk, "--show-sdk-path"])
        .output()
        .expect("iOS builds require Xcode xcrun");
    assert!(output.status.success(), "Xcode iOS SDK is required");
    let sdk_path = String::from_utf8(output.stdout).unwrap();
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let archive = out.join("libduet_secure.a");
    let status = Command::new("xcrun")
        .args([
            "--sdk",
            sdk,
            "swiftc",
            "-swift-version",
            "5",
            "-parse-as-library",
            "-emit-library",
            "-static",
            "-module-name",
            "DuetSecureBridge",
            "-target",
            &swift_target,
            "-sdk",
            sdk_path.trim(),
            "../../platform/ios/SecureRetentionBridge.swift",
            "-o",
        ])
        .arg(&archive)
        .status()
        .expect("Swift compiler is required");
    assert!(
        status.success(),
        "Duet secure bridge compilation failed; no plaintext fallback"
    );
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=duet_secure");
    for framework in [
        "Foundation",
        "Security",
        "AuthenticationServices",
        "CryptoKit",
        "UIKit",
    ] {
        println!("cargo:rustc-link-lib=framework={framework}");
    }
    // Swift runtime libraries supplied by the Xcode toolchain and OS.
    let tool = Command::new("xcrun")
        .args(["--find", "swiftc"])
        .output()
        .unwrap();
    let compiler = PathBuf::from(String::from_utf8(tool.stdout).unwrap().trim());
    let lib = compiler
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("lib/swift")
        .join(sdk);
    println!("cargo:rustc-link-search=native={}", lib.display());
    for runtime in [
        "swiftCore",
        "swiftFoundation",
        "swiftDispatch",
        "swift_Concurrency",
        "swift_StringProcessing",
        "swift_RegexParser",
    ] {
        println!("cargo:rustc-link-lib=dylib={runtime}");
    }
    println!(
        "cargo:rustc-link-search=native={}/usr/lib/swift",
        sdk_path.trim()
    );
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
}
