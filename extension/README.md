# Coda's Chrome extension

The extension reads a practice's EHR chart in the background under the physician's own sign-in, as the [physician spec](../intent/physician.md#where-codas-information-comes-from) describes. Hint is its first EHR. It reads the structured chart data Hint's web app uses; it never takes the physician's tab or focus, downloads nothing, never sees a password or MFA code, and keeps Hint's session inside Chrome. Its access lasts as long as the physician's Hint sign-in.

This version is a sandbox prototype. It reads only the Coda Health Sandbox Practice and its four synthetic patients, twelve chart sections per patient, and a local bridge stands in for Coda's backend.

The extension's screens are built in Dioxus from [`crates/ui`](../crates/ui), like the physician web app, so they look and behave like the rest of Coda as the extension grows. Its background engine stays plain JavaScript.

## Layout

- `chrome/` is the extension to load in Chrome. `worker.js` owns Hint's session: it renews access, reads chart sections, and hands each result to the bridge until the bridge acknowledges it. When renewal fails it stops and says so; it never opens a tab of its own. `bootstrap.js` reads the Hint page on Connect, to confirm the sandbox practice and identity, and whenever the popup asks whether Hint is open, signed in and on the practice Coda reads; it copies no credential and nothing it finds is kept. `options.html` is the popup: it starts [`crates/extension-popup`](../crates/extension-popup), which says where the connection stands in one line with its one action, and before connecting the steps still to take, and keeps Go to Hint, Copy diagnostics and Disconnect in its menu.
- `sandbox/bridge.py` is the local bridge. With `--serve` it accepts the extension's results on `127.0.0.1:51383` and keeps progress in a checkpoint; without it, it serves the MCP tools `connection_status`, `request_roster`, `request_chart` and `read_result` over stdio from the same checkpoint.
- `sandbox/run_agent.py` runs a Codex agent whose only tools are those MCP tools.
- `sandbox/test_connector.mjs` runs the unchanged worker in Node with simulated Chrome APIs, against real local HTTP and a real bridge and MCP process. It is not proof in real Chrome.

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
node extension/sandbox/test_connector.mjs
```

The first checks the popup's words for each connection state. The second prints a JSON report ending in `"passed": true` and exits 0.

`cargo test -p extension-popup --test popup -- --ignored` builds the popup, loads the extension in Chromium, and checks that its page refuses `eval`, renders, and that its menu opens, takes its keys and closes, with nothing in the console. CI runs the worker test on every full run and this one with the other browser tests, on a manual run.

## A live sandbox read

This needs access to the Hint sandbox.

1. Build the popup, then in Chrome open `chrome://extensions`, turn on Developer mode, and load `extension/chrome` unpacked. The manifest's public key fixes the extension ID the bridge expects (`sandbox/extension-identity.json`).
2. Start the bridge: `python3 extension/sandbox/bridge.py --serve`.
3. Sign into Hint and select Coda Health Sandbox Practice.
4. Open the extension from Chrome's toolbar and choose **Connect Hint**. You can then close the Hint tab and use Chrome normally. A run reads one section every 30 seconds and ends after four hours or when the connection needs attention; **Disconnect**, in the popup's menu, stops it and clears the session.
5. Ask an agent for charts: `python3 extension/sandbox/run_agent.py roster`, `chart` or `batch`. This needs the Codex CLI. Each run writes its result to `extension/sandbox/agent-*/result.md`.
