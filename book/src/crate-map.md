# Crate map

Every crate in [`crates/`](/crates) is listed here, with the file where it starts and the chapter that covers its behavior. The module docs at the top of each entry file are the maintained description. When this table and a crate disagree, the crate is right and this table needs fixing. [`boundaries`](/crates/boundaries/src/lib.rs) keeps Duet and the physician client from importing each other's clinical crates. The Shared table below is the list of crates both products may use. One crate sits a level deeper: `coda-sandbox`, the agent sandbox tool, lives in [`crates/workspace-tools/sandbox/`](/crates/workspace-tools/sandbox/flake.nix) and is listed under [Tooling and guards](#tooling-and-guards).

## Shared

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `ui` | [`src/lib.rs`](/crates/ui/src/lib.rs) | Coda's design, as a catalog of AI-native component families: tokens, icons, and each component's look, motion, gestures, states and accessibility, with no product rules | — |
| `extension-popup` | [`src/lib.rs`](/crates/extension-popup/src/lib.rs) | The Chrome extension's popup, built from `ui`: where the connection stands, in one line, and its one action. The extension's worker owns the connection | [Chrome extension](#chrome-extension) |
| `coda-http` | [`src/lib.rs`](/crates/http/src/lib.rs) | Request and stream mechanics with no schema knowledge | [Backend architecture](backend-architecture.md) |
| `gmail` | [`src/lib.rs`](/crates/gmail/src/lib.rs) | Plain-text or HTML messages through the Gmail API or the Workspace SMTP relay, plus the intake primitives: push-wake routing ([`src/push.rs`](/crates/gmail/src/push.rs)) and the `history.list` cursor reader ([`src/history.rs`](/crates/gmail/src/history.rs)). Products own the copy, the links, the durable cursor and the effects | [GCP](gcp.md) |
| `spruce` | [`src/lib.rs`](/crates/spruce/src/lib.rs) | Read-only Spruce REST reader and change cursor. Products own the cursor store, the effects, and patient linking | — |
| `pubsub` | [`src/lib.rs`](/crates/pubsub/src/lib.rs) | Pub/Sub push-envelope decoding, the loopback-emulator check and the official publisher; the `test-util` feature is the disposable emulator for tests (dev-dependencies only, checked by `boundaries`) | [GCP](gcp.md) |
| `encoder` | [`src/lib.rs`](/crates/encoder/src/lib.rs), [`src/main.rs`](/crates/encoder/src/main.rs) | Offline BGE encoder binary that speaks both the Duet and the physician encoder protocols; [`tests/compare.rs`](/crates/encoder/tests/compare.rs) checks it against the existing Python workers. No crate depends on it | [Physician site](physician-site.md) |
| `harness` | [`src/lib.rs`](/crates/harness/src/lib.rs) | One product-neutral bounded provider/tool loop and ordered turn in `engine`. Chords chat and reply revisions supply a frozen recipe and implement `engine::Product`. The harness builds each request, emits ordered events and exposes the shared `skill_view` tool for each job’s visible package. Products keep authority, effects and persistence. The harness reconstructs recovery from their admitted records and messages. | <a href="/intent/harness%2Emd">Harness spec</a> |
| `agent-skills` | [`src/lib.rs`](/crates/agent-skills/src/lib.rs) | Validates and reads the [physician](/skills) skill package | [Physician site](physician-site.md) |
| `clinical-library` | [`src/lib.rs`](/crates/clinical-library/src/lib.rs), [`src/bin/clinical-library-migrate.rs`](/crates/clinical-library/src/bin/clinical-library-migrate.rs), [`src/bin/clinical-library-pubmed-status.rs`](/crates/clinical-library/src/bin/clinical-library-pubmed-status.rs) | The clinical library's records and the nightly's failure rules: article states, night outcomes counted from each night's ledger, the stop rule, the slot lease and activation, with their Postgres schema; the night runner every source goes through, and the alerts the records raise; and its sources, starting with PubMed's retraction and correction status read from NCBI's E-utilities. Depends on no other local crate | <a href="/verification/clinical-library/nightly/README%2Emd">Clinical library nightly</a> |
| `telemetry` | [`src/lib.rs`](/crates/telemetry/src/lib.rs) | Shared tracing, logging, and OpenTelemetry setup for the Rust services; the `testing` feature starts the disposable Docker Langfuse and collector for ignored tests | [Observability](observability.md) |

## Duet (patient app)

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `duet-app` | [`src/lib.rs`](/crates/duet-app/src/lib.rs), [`src/roots.rs`](/crates/duet-app/src/roots.rs) | Standalone account entry, tab roots and connection controls | [UI and design](ui-surface-map.md) |
| `duet-api` | [`src/lib.rs`](/crates/duet-api/src/lib.rs), [`src/main.rs`](/crates/duet-api/src/main.rs) | Axum owner for identity, evidence, the doctor workspace, health context, privacy and the existing connection adapter | [Backend architecture](backend-architecture.md), [Patient front door](front-door.md) |
| `duet-client` | [`src/lib.rs`](/crates/duet-client/src/lib.rs) | Duet caller over the generated client, adding the app-build headers | [Backend architecture](backend-architecture.md) |
| `duet-wire` (package `duet_wire`) | [`src/lib.rs`](/crates/duet-wire/src/lib.rs), <a href="/crates/duet-wire/README%2Emd">generated API index</a> | Models, operations and reference docs generated from the pinned coda-app OpenAPI plus Duet-owned additions. Regenerate with `cargo run -p workspace-tools -- generate-duet-wire`; do not edit outputs by hand. | [API generation](backend-architecture.md#api-and-clients) |
| `duet-health` | [`src/lib.rs`](/crates/duet-health/src/lib.rs), [`src/acquisition.rs`](/crates/duet-health/src/acquisition.rs), [`src/outbox.rs`](/crates/duet-health/src/outbox.rs) | On-device health samples from HealthKit and Health Connect; durable acquisition records, the native source adapter contract and the transactional upload outbox (account fencing and the batch transport live in `duet-client::health_upload`) | [Duet backend](backend-architecture.md) |
| `duet-notify` | [`src/lib.rs`](/crates/duet-notify/src/lib.rs) | Alert permission and one local insight alert | [Duet backend](backend-architecture.md) |
| `duet-page` | [`src/lib.rs`](/crates/duet-page/src/lib.rs) | The US Letter doctor-page layout, where overflow blocks sharing | [UI and design](ui-surface-map.md) |
| `duet-stub` | [`src/lib.rs`](/crates/duet-stub/src/lib.rs), [`src/main.rs`](/crates/duet-stub/src/main.rs) | Local Duet stand-in for the emulator and tests (`cargo run -p duet-stub`) | [Release](release.md#android-and-google-play) |

## Physician product

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `physician-app` | [`src/lib.rs`](/crates/physician-app/src/lib.rs), [`src/main.rs`](/crates/physician-app/src/main.rs) | The physician Dioxus shell: sidebar, Today, patient page, and reader | [Physician site](physician-site.md) |
| `physician-wire` | [`src/lib.rs`](/crates/physician-wire/src/lib.rs) | The physician read types, following coda-agents' <a href="/submodules/coda-agents/src/server/apiTypes%2Ets"><code>apiTypes.ts</code></a> | [Physician site](physician-site.md) |
| `physician-client` | [`src/lib.rs`](/crates/physician-client/src/lib.rs) | The caller for the Node `CodaServer` and the sample server | [Physician site](physician-site.md) |
| `physician-sample` | [`src/lib.rs`](/crates/physician-sample/src/lib.rs), [`src/main.rs`](/crates/physician-sample/src/main.rs) | Serves the page and the synthetic practice, plus auth and live chat routes; the `physician-migrate` binary runs the physician schema migrations | [Physician site](physician-site.md) |
| `physician-auth` | [`src/lib.rs`](/crates/physician-auth/src/lib.rs), [`src/bin/physician-auth-migrate.rs`](/crates/physician-auth/src/bin/physician-auth-migrate.rs) | Physician identity, membership, panels, and sessions | [Physician site](physician-site.md#physician-sign-in-and-practice-access) |
| `local-product` | [`src/lib.rs`](/crates/local-product/src/lib.rs) | Local subscription or API model execution; the physician runtime store, and hosted chat's instructions and `skill_view` tool (`physician_chat`) | [Physician site](physician-site.md) |
| `physician-bus` | [`src/lib.rs`](/crates/physician-bus/src/lib.rs) | Telling open physician pages to reread changed state, through a local bus or Redis fan-out. Lossy invalidation, not a durable work queue | [Physician site](physician-site.md) |
| `patient-work` | [`src/lib.rs`](/crates/patient-work/src/lib.rs) | The patient-work dispatcher's grouping window | [Physician site](physician-site.md) |

## Between the products

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `front-door` | [`src/lib.rs`](/crates/front-door/src/lib.rs), [`src/bin/front-door-migrate.rs`](/crates/front-door/src/bin/front-door-migrate.rs) | Invitation, roster and consent adapter; standalone account authority stays in Duet | [Patient front door](front-door.md) |
| `mobile` | [`src/lib.rs`](/crates/mobile/src/lib.rs), [`src/main.rs`](/crates/mobile/src/main.rs) | The one Android and iOS shell, with an explicit physician or client mode. It is the only crate that mounts both apps. | [Release](release.md#android-and-google-play) |

## Public website

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `public-site` | [`src/lib.rs`](/crates/public-site/src/lib.rs) | The startup-rendered public marketing site, isolated from the physician runtime | [Public site](public-site.md) |
| `observability-relay` | [`src/lib.rs`](/crates/observability-relay/src/lib.rs) | The staging hop from public-site's `/grafana/` mount to Grafana's private load balancer | [Public site](public-site.md#observability-mounts) |
## Tooling and guards

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `patient-bank` | [`src/lib.rs`](/crates/patient-bank/src/lib.rs), [`src/main.rs`](/crates/patient-bank/src/main.rs) | The synthetic patient bank: chart authoring checks, the verified code catalog, the FHIR build and its validation | [Synthetic patients](patient-bank.md) |
| `ui-gallery` | [`src/main.rs`](/crates/ui-gallery/src/main.rs), [`scripts/ui-gallery.sh`](/scripts/ui-gallery.sh) | Every `ui` family rendered with its states, each at its own address, the command that builds and serves it, and the browser test that opens every family. Nothing depends on it | — |
| `boundaries` | [`src/lib.rs`](/crates/boundaries/src/lib.rs) | Tests that keep product dependencies apart, retain the reference shipping clients, and hold both apps' raw controls, hand-built cards and library overrides to a [checked-in table](/crates/boundaries/src/shared_ui.rs) | This page |
| `workspace-tools` | [`src/main.rs`](/crates/workspace-tools/src/main.rs) | Workspace command entrypoints, including `generate-duet-wire`, `install-doppler`, `apply-migrations` and the `observability-drill` routes, among them the staging Langfuse drills; supporting checks and generator assets remain in [`scripts/`](/scripts) | [Build, CI, and release](release.md) |
| `coda-sandbox` | [`src/main.rs`](/crates/workspace-tools/sandbox/src/main.rs), [`src/lib.rs`](/crates/workspace-tools/sandbox/src/lib.rs) | The agent sandbox tool: its config, `doctor` and `status`, `worktree` (seeded reflink worktrees), `ci run` (the CI spool client and the spool format the controller shares), `ci-controller` (the Firecracker CI controller the `coda-ci` units run), `carry`, and `broker` (the token broker that serves the container's tokens from the host keyring) with its container clients `credential` and `token`. A root workspace member nested under `workspace-tools`, with its own flake ([README](/crates/workspace-tools/sandbox)) that builds only this crate from the root lockfile and ships the NixOS module `services.coda-sandbox`, which declares the container and CI it checks, so a host deploys it without the product flake. No crate depends on it | — |
| `android` | [`src/main.rs`](/crates/android/src/main.rs) | Builds the APK, opens the Android emulator (a window, or headless in CI) and runs one UI Automator class (`cargo run -p android -- test [--headless] coda.duet.ModeShellTest`) | [Release](release.md#android-and-google-play) |

## Platform bindings

[`platform/`](/platform) holds the native code that the Rust crates call. It is not a Cargo crate.

| File | Bound to |
| --- | --- |
| [`ios/HealthKitBridge.swift`](/platform/ios/HealthKitBridge.swift) | `duet-health` |
| [`ios/NotificationBridge.swift`](/platform/ios/NotificationBridge.swift) | `duet-notify` |
| [`android/HealthConnectBridge.kt`](/platform/android/HealthConnectBridge.kt) | `duet-health` |
| [`android/NotificationBridge.kt`](/platform/android/NotificationBridge.kt) | `duet-notify` |
| [`android/HardwareKeyDispatch.kt`](/platform/android/HardwareKeyDispatch.kt) | `mobile` WebView key input, copied in by `cargo run -p android -- build` ([`build.rs`](/crates/android/src/build.rs)) |
| [`android/ModeShellTest.kt`](/platform/android/ModeShellTest.kt) | The UI Automator class that `android` runs |

## Chrome extension

[`extension/`](/extension) holds Coda's Chrome extension, which reads the practice's EHR under the physician's own sign-in, and its local sandbox bridge and tests. Its popup is the `extension-popup` crate; the rest is not a Cargo crate. Its <a href="/extension/README%2Emd">README</a> covers loading it and running a sandbox read.
