#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

out_dir="${OUT_DIR:-$root/_site}"
base_path="${BASE_PATH:-/}"
origin="${ORIGIN:-https://coda.dabney.moe}"
book_url="/book/"
if [ "$base_path" != "/" ]; then
  book_url="${base_path%/}/book/"
fi

sqljs_tag="v1.16.0"
sqljs_sha256="5ac3671ec7d03a0fb73b92031f576bccd4c591a1a454fca01b125610fc691bd6"
sqljs_zip="${SQLJS_ARCHIVE:-/tmp/sqljs-wasm-${sqljs_tag}.zip}"

if [ ! -f "$sqljs_zip" ] || ! echo "${sqljs_sha256}  ${sqljs_zip}" | sha256sum -c - >/dev/null 2>&1; then
  rm -f "$sqljs_zip"
  curl -fsSL --retry 3 -o "$sqljs_zip" \
    "https://github.com/sql-js/sql.js/releases/download/${sqljs_tag}/sqljs-wasm.zip"
  echo "${sqljs_sha256}  ${sqljs_zip}" | sha256sum -c -
fi

rm -rf "$out_dir"
mkdir -p "$out_dir/physician/assets"

cargo run --locked -p public-site --bin public-site -- \
  --export-dir "$out_dir" \
  --base-path "$base_path" \
  --origin "$origin" \
  --physician-demo

MDBOOK_OUTPUT__HTML__SITE_URL="$book_url" \
  mdbook build book -d "$out_dir/book"

cargo run --locked -p physician-demo-db --bin seed_demo_db -- \
  --units "$root/patients/units" \
  --out "$out_dir/physician/demo.sqlite" \
  --limit 60

python3 - "$sqljs_zip" "$out_dir/physician" <<'PY'
import pathlib, sys, zipfile

archive = pathlib.Path(sys.argv[1])
dest = pathlib.Path(sys.argv[2])
with zipfile.ZipFile(archive) as zf:
    for wanted in ("sql-wasm.js", "sql-wasm.wasm"):
        matches = [n for n in zf.namelist() if n.endswith("/" + wanted) or n == wanted]
        if not matches:
            raise SystemExit(f"missing {wanted} in {archive}")
        (dest / wanted).write_bytes(zf.read(matches[0]))
PY

PHYSICIAN_DEMO_DB="./demo.sqlite" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p physician-app --no-default-features --features web

wasm-bindgen "${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/physician-app.wasm" \
  --target web \
  --out-dir "$out_dir/physician" \
  --out-name physician-app

cp crates/physician-app/index.html "$out_dir/physician/index.html"
cp crates/physician-app/assets/favicon.svg "$out_dir/physician/favicon.svg"
cp crates/physician-app/assets/coda-health-logo-transparent.png \
  "$out_dir/physician/assets/coda-health-logo-transparent.png"
