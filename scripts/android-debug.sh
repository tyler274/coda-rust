#!/bin/sh
# Build the Duet debug APK. The Nix Android SDK is read-only and only has
# build-tools 37, while dx asks Gradle for 34. Rustc must be rustup's stable:
# the Nix rustc has no Android standard library. DUET_API_URL and DUET_BEARER
# are baked in at compile time; an unset URL uses http://10.0.2.2:8000.
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
"$dx" build --android --package duet-app --features mobile >"$log" 2>&1
status=$?
set -e
cat "$log"
if [ "$status" -ne 0 ] && ! grep -q "build-tools" "$log"; then
  exit "$status"
fi

gradle_files=$(find target/dx/duet-app -name build.gradle.kts ! -path '*/build/*')
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
find target/dx/duet-app -path '*/src/main/AndroidManifest.xml' | while IFS= read -r path; do
  if ! grep -q usesCleartextTraffic "$path"; then
    sed -i 's/<application/<application android:usesCleartextTraffic="true"/' "$path"
  fi
done

gradle=$(find target/dx/duet-app -name gradlew | head -n 1)
if [ -z "$gradle" ]; then
  echo "gradlew was not generated" >&2
  exit 1
fi
(cd "$(dirname "$gradle")" && ./gradlew assembleDebug)
apk=$(find target/dx/duet-app -name 'app-debug.apk' | head -n 1)
echo "$apk"
