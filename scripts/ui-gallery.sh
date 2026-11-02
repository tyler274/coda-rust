#!/usr/bin/env bash
# Build the UI gallery and serve it from the dev shell. Each section opens at its
# own address, such as http://127.0.0.1:8080/#charts. UI_GALLERY_LISTEN_ADDR
# chooses the address; in a container, 0.0.0.0 on a published port reaches the Mac.
set -euo pipefail

cd "$(dirname "$0")/.."
addr=${UI_GALLERY_LISTEN_ADDR:-127.0.0.1:8080}
target=${CARGO_TARGET_DIR:-target}
site=$target/ui-gallery-web

cargo build -p ui-gallery --target wasm32-unknown-unknown --features web
wasm-bindgen --target web --out-dir "$site" "$target/wasm32-unknown-unknown/debug/ui-gallery.wasm"
cp crates/ui-gallery/web/index.html "$site/"

echo "UI gallery: http://$addr/"
exec python3 -m http.server --bind "${addr%:*}" --directory "$site" "${addr##*:}"
