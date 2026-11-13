# Coda's Chrome extension

The extension (`Coda`) reads a practice's EHR chart in the background under the physician's own sign-in, as the [physician spec](../intent/physician.md#where-codas-information-comes-from) describes. Its connector pairs with Chords, takes named read jobs without URLs, runs them through a registered EHR reader, keeps sign-in tokens strictly in Chrome's session memory, and delivers each response once with its SHA-256 receipt. Chrome asks for an EHR's hosts through `optional_host_permissions` only when that EHR is connected.

Hint is the first registered reader, and its chart records file into Coda's buckets through the table in [`crates/local-product/src/hint.rs`](../crates/local-product/src/hint.rs). The extension never takes the physician's tab or focus, downloads nothing to disk, never sees a password or MFA code, and keeps running across worker restarts and sign-out/sign-in catch-up for as long as the physician's EHR session can renew.

The extension's popup is built in Dioxus from [`crates/ui`](../crates/ui), like the physician web app, while its background connector and source readers run as plain JavaScript in Chrome's service worker.

## Layout

- `chrome/` is the unpacked extension to load in Chrome:
  - `connector.js` and `worker.js` own Chords pairing, job admission, rate-limit backoff, per-read fault isolation, durable outbox delivery, and memory-only credential storage (`chrome.storage.session`), with no EHR, host, patient, or chart-part names.
  - `readers/hint.js` and `readers/hint-page.js` hold Hint's hosts, Frontegg session renewal, complete named chart reads (roster, patient, health sections, paged notes and full note text, tasks, appointments, messages, and patient files), response shape validators, and billing and prescription-price redaction.
  - `readers/index.js` lists the readers registered in the extension.
  - `options.html` starts [`crates/extension-popup`](../crates/extension-popup), which pairs with Chords, requests optional host permissions on Connect, shows where the connection stands in one line with its one action, and offers `Go to Hint`, `Copy diagnostics`, `Disconnect`, and `Unpair extension` in its menu.
- `sandbox/bridge.py` is the local Chords-compatible pairing, job-lease, receipt, and MCP server (`127.0.0.1:51383` with `--serve`, or MCP over stdio without `--serve`).
- `sandbox/run_agent.py` drives a Codex agent over those MCP tools.
- `sandbox/test_connector.mjs` runs the unchanged connector and Hint reader, plus a second synthetic EHR reader, in Node with simulated Chrome APIs against real local HTTP and a real bridge and MCP process.

## Building the popup

The popup's WebAssembly is generated into `chrome/popup/`, which git ignores. Build it before loading the extension:

```sh
cargo build -p extension-popup --target wasm32-unknown-unknown --features web
wasm-bindgen --target web --no-typescript --out-dir extension/chrome/popup \
  target/wasm32-unknown-unknown/debug/extension-popup.wasm
```

## Local tests

```sh
cargo test -p extension-popup
cargo test -p local-product hint
node extension/sandbox/test_connector.mjs
```

`cargo test -p extension-popup --test popup -- --ignored` builds the popup, loads the extension in Chromium, and checks that its manifest is named `Coda`, that reader host permissions are not pre-granted before connecting, that its page refuses `eval`, that storage holds no tokens, and that its menu opens, takes its keys, and closes with no page errors.

## A live sandbox read

This needs access to a Hint sandbox account.

1. Build the popup, then in Chrome open `chrome://extensions`, turn on Developer mode, and load `extension/chrome` unpacked. The manifest's public key fixes the extension ID (`sandbox/extension-identity.json`).
2. Start the local bridge: `python3 extension/sandbox/bridge.py --serve`.
3. Open the extension from Chrome's toolbar, enter `http://127.0.0.1:51383` as the Chords address, and choose **Pair extension**.
4. Sign into Hint in one tab on your practice, open the popup, and choose **Connect Hint**. Approve Chrome's host-access prompt for Hint; you can then close the Hint tab and use Chrome normally. **Disconnect**, in the popup's menu, stops reading and clears the in-memory session.
5. Request roster or charts over MCP: `python3 extension/sandbox/run_agent.py roster`, `chart <patient_id>`, or `batch`.
