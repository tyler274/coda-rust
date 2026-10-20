#!/bin/sh
# Build the mode-shell debug APK. The Nix Android SDK is read-only and only has
# build-tools 37, while dx asks Gradle for 34. Rustc must be rustup's stable:
# the Nix rustc has no Android standard library. DUET_API_URL and DUET_BEARER
# are baked in at compile time; an unset URL uses http://10.0.2.2:8000.
# The host stand-in is `cargo run -p duet-stub`. It listens on 127.0.0.1:8000
# and keeps accepted messages in its state file.
# The emulator window is `cargo run -p android -- window`. UI Automator is
# `cargo run -p android -- test coda.duet.ModeShellTest`. This build does not
# start the emulator.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root"

export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"
if [ -d "/etc/profiles/per-user/${USER}/bin" ]; then
  PATH="/etc/profiles/per-user/${USER}/bin:${PATH}"
fi
export PATH

if ! command -v java >/dev/null 2>&1; then
  jbr=$(find /nix/store -maxdepth 2 -type d -name jbr -path '*android-studio*' | head -n 1)
  if [ -z "$jbr" ]; then
    echo "java is not on PATH and the Android Studio JBR was not found" >&2
    exit 1
  fi
  export JAVA_HOME="$jbr"
  export PATH="${JAVA_HOME}/bin:${PATH}"
fi

dx=${DX:-$(command -v dx || true)}
if [ -z "$dx" ]; then
  echo "dx, the Dioxus CLI, is not on PATH" >&2
  exit 1
fi

log=$(mktemp)
set +e
"$dx" build --android --package mobile --features mobile >"$log" 2>&1
status=$?
set -e
cat "$log"
if [ "$status" -ne 0 ] && ! grep -q "build-tools" "$log"; then
  exit "$status"
fi

gradle_files=$(find target/dx/mobile -name build.gradle.kts ! -path '*/build/*')
if [ -z "$gradle_files" ]; then
  echo "dx did not generate an Android Gradle project" >&2
  exit 1
fi
printf '%s\n' "$gradle_files" | while IFS= read -r path; do
  if grep -q 'buildToolsVersion' "$path"; then
    sed -i 's/buildToolsVersion *= *"[^"]*"/buildToolsVersion = "37.0.0"/' "$path"
  elif grep -q 'compileSdk =' "$path"; then
    sed -i 's/compileSdk = \([0-9.]*\)/compileSdk = \1\n    buildToolsVersion = "37.0.0"/' "$path"
  fi
done
find target/dx/mobile -path '*/src/main/AndroidManifest.xml' | while IFS= read -r path; do
  if ! grep -q usesCleartextTraffic "$path"; then
    sed -i 's/<application/<application android:usesCleartextTraffic="true"/' "$path"
  fi
  if ! grep -q 'windowSoftInputMode' "$path"; then
    sed -i 's/<activity /<activity android:windowSoftInputMode="adjustResize" /' "$path"
  fi
done

# dx regenerates the activity. A hardware key never reaches a focused field;
# the soft keyboard and paste commit through the input connection instead.
keys="$root/platform/android/HardwareKeyDispatch.kt"
find target/dx/mobile -name WryActivity.kt | while IFS= read -r activity; do
  cp "$keys" "$(dirname "$activity")/HardwareKeyDispatch.kt"
  if ! grep -q 'duet-hardware-keys' "$activity"; then
    awk '
      /fun getAppClass/ && !done {
        print "    // duet-hardware-keys"
        print "    override fun dispatchKeyEvent(event: KeyEvent): Boolean {"
        print "        if (event.keyCode != KeyEvent.KEYCODE_BACK && ::mWebView.isInitialized && duetInsertHardwareKey(mWebView, event)) {"
        print "            return true"
        print "        }"
        print "        return super.dispatchKeyEvent(event)"
        print "    }"
        print ""
        done = 1
      }
      { print }
    ' "$activity" > "$activity.tmp" && mv "$activity.tmp" "$activity"
  fi
done

# Health Connect is copied into the generated app because dx regenerates it.
bridge="$root/platform/android/HealthConnectBridge.kt"
java_dir=$(find target/dx/mobile -path '*/src/main/AndroidManifest.xml' | head -n 1 | xargs dirname)
if [ -z "$java_dir" ] || [ ! -f "$bridge" ]; then
  echo "Health Connect bridge or generated manifest is missing" >&2
  exit 1
fi
notify="$root/platform/android/NotificationBridge.kt"
if [ ! -f "$notify" ]; then
  echo "the Android notification bridge is missing" >&2
  exit 1
fi
mkdir -p "$java_dir/java/coda/duet"
cp "$bridge" "$java_dir/java/coda/duet/HealthConnectBridge.kt"
cp "$notify" "$java_dir/java/coda/duet/NotificationBridge.kt"
app_gradle=$(find target/dx/mobile -path '*/app/build.gradle.kts' ! -path '*/build/*' | while IFS= read -r path; do
  if grep -q 'androidx.webkit' "$path"; then
    printf '%s\n' "$path"
  fi
done | head -n 1)
if [ -z "$app_gradle" ]; then
  echo "the app Gradle file was not generated" >&2
  exit 1
fi
if ! grep -q 'connect-client' "$app_gradle"; then
  sed -i 's/dependencies {/dependencies {\n    implementation("androidx.health.connect:connect-client:1.1.0-alpha11")\n    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.10.2")/' "$app_gradle"
fi
if ! grep -q 'androidx.core:core' "$app_gradle"; then
  sed -i 's/dependencies {/dependencies {\n    implementation("androidx.core:core-ktx:1.15.0")/' "$app_gradle"
fi
# UI Automator is copied in for the same reason: dx regenerates this project.
test_src="$root/platform/android/ModeShellTest.kt"
test_dir="$java_dir/../androidTest/java/coda/duet"
if [ ! -f "$test_src" ]; then
  echo "the mode-shell UI Automator test is missing" >&2
  exit 1
fi
mkdir -p "$test_dir"
cp "$test_src" "$test_dir/ModeShellTest.kt"
if ! grep -q 'uiautomator:uiautomator' "$app_gradle"; then
  sed -i 's/dependencies {/dependencies {\n    androidTestImplementation("androidx.test.ext:junit:1.2.1")\n    androidTestImplementation("androidx.test:runner:1.6.2")\n    androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")\n    androidTestUtil("androidx.test:orchestrator:1.5.1")/' "$app_gradle"
fi
if ! grep -q 'androidx.test:orchestrator' "$app_gradle"; then
  sed -i 's/androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")/androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")\n    androidTestUtil("androidx.test:orchestrator:1.5.1")/' "$app_gradle"
fi
if ! grep -q 'testInstrumentationRunner' "$app_gradle"; then
  sed -i 's/versionName = "0.1.0"/versionName = "0.1.0"\n        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"\n        testInstrumentationRunnerArguments["clearPackageData"] = "true"/' "$app_gradle"
fi
if ! grep -q 'clearPackageData' "$app_gradle"; then
  sed -i 's/testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"/testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"\n        testInstrumentationRunnerArguments["clearPackageData"] = "true"/' "$app_gradle"
fi
if ! grep -q 'ANDROIDX_TEST_ORCHESTRATOR' "$app_gradle"; then
  sed -i 's/sourceSets {/testOptions {\n        execution = "ANDROIDX_TEST_ORCHESTRATOR"\n    }\n    sourceSets {/' "$app_gradle"
fi
# connect-client 1.1.0-alpha11 compiles against API 35 and requires minSdk 26.
sed -i 's/compileSdk = 34/compileSdk = 35/' "$app_gradle"
sed -i 's/minSdk = 24/minSdk = 26/' "$app_gradle"
manifest="$java_dir/AndroidManifest.xml"
if ! grep -q 'android.permission.health.READ_STEPS' "$manifest"; then
  sed -i '/android.permission.INTERNET/a\
    <uses-permission android:name="android.permission.health.READ_STEPS" />\
    <uses-permission android:name="android.permission.health.READ_ACTIVE_CALORIES_BURNED" />\
    <uses-permission android:name="android.permission.health.READ_EXERCISE" />\
    <uses-permission android:name="android.permission.health.READ_DISTANCE" />\
    <uses-permission android:name="android.permission.health.READ_NUTRITION" />\
    <uses-permission android:name="android.permission.health.READ_HYDRATION" />\
    <uses-permission android:name="android.permission.health.READ_RESTING_HEART_RATE" />\
    <uses-permission android:name="android.permission.health.READ_WEIGHT" />\
    <uses-permission android:name="android.permission.health.READ_SLEEP" />\
    <uses-permission android:name="android.permission.health.READ_BLOOD_PRESSURE" />\
    <uses-permission android:name="android.permission.health.READ_BODY_FAT" />\
    <uses-permission android:name="android.permission.health.READ_LEAN_BODY_MASS" />\
    <uses-permission android:name="android.permission.health.READ_HEIGHT" />\
    <uses-permission android:name="android.permission.health.READ_BLOOD_GLUCOSE" />\
    <uses-permission android:name="android.permission.health.READ_BODY_TEMPERATURE" />\
    <uses-permission android:name="android.permission.health.READ_BASAL_BODY_TEMPERATURE" />\
    <uses-permission android:name="android.permission.health.READ_OXYGEN_SATURATION" />\
    <uses-permission android:name="android.permission.health.READ_VO2_MAX" />\
    <queries>\
        <package android:name="com.google.android.apps.healthdata" />\
    </queries>' "$manifest"
fi
if ! grep -q 'android.permission.POST_NOTIFICATIONS' "$manifest"; then
  sed -i '/android.permission.INTERNET/a\
    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />\
    <uses-permission android:name="android.permission.SCHEDULE_EXACT_ALARM" />' "$manifest"
fi
if ! grep -q 'coda.duet.InsightAlertReceiver' "$manifest"; then
  sed -i 's|</application>|    <receiver android:name="coda.duet.InsightAlertReceiver" android:exported="false" />\
    </application>|' "$manifest"
fi
if ! grep -q 'VIEW_PERMISSION_USAGE' "$manifest"; then
  sed -i 's|</activity>|    <intent-filter>\
                <action android:name="androidx.health.ACTION_SHOW_PERMISSIONS_RATIONALE" />\
            </intent-filter>\
        </activity>\
        <activity-alias android:name="ViewPermissionUsageActivity" android:exported="true" android:targetActivity="dev.dioxus.main.MainActivity" android:permission="android.permission.START_VIEW_PERMISSION_USAGE">\
            <intent-filter>\
                <action android:name="android.intent.action.VIEW_PERMISSION_USAGE" />\
                <category android:name="android.intent.category.HEALTH_PERMISSIONS" />\
            </intent-filter>\
        </activity-alias>|' "$manifest"
fi

gradle=$(find target/dx/mobile -name gradlew | head -n 1)
if [ -z "$gradle" ]; then
  echo "gradlew was not generated" >&2
  exit 1
fi
(cd "$(dirname "$gradle")" && ./gradlew assembleDebug)
apk=$(find target/dx/mobile -name 'app-debug.apk' | head -n 1)
echo "$apk"
