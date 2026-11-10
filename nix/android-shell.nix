# The Android emulator shell for `cargo run -p android` and the CI
# android-emulator job. It is separate so the default shell carries neither
# the SDK licence nor its download.
{ pkgs, rustToolchain }:
let
  composition = pkgs.androidenv.composeAndroidPackages {
    platformVersions = [ "35" ];
    buildToolsVersions = [ "37.0.0" ];
    includeNDK = true;
    includeEmulator = true;
    includeSystemImages = true;
    systemImageTypes = [ "google_apis" ];
    abiVersions = [ "x86_64" ];
  };
  sdk = "${composition.androidsdk}/libexec/android-sdk";
in
pkgs.mkShell {
  packages = [
    composition.androidsdk
    (rustToolchain.override { targets = [ "x86_64-linux-android" ]; })
    pkgs.dioxus-cli
    pkgs.jdk17_headless
    pkgs.python3
  ];
  env = {
    ANDROID_HOME = sdk;
    ANDROID_SDK_ROOT = sdk;
    ANDROID_NDK_HOME = "${sdk}/ndk-bundle";
  };
}
