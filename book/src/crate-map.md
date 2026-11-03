# Crate map

Every crate in [`crates/`](/crates) is listed here, with the file where it starts and the chapter that covers its behavior. The module docs at the top of each entry file are the maintained description. When this table and a crate disagree, the crate is right and this table needs fixing. [`boundaries`](/crates/boundaries/src/lib.rs) keeps Duet and the physician client from importing each other's clinical crates. The Shared table below is the list of crates both products may use.

## Shared

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `ui` | [`src/lib.rs`](/crates/ui/src/lib.rs) | Coda's design, as a catalog of AI-native component families: tokens, icons, and each component's look, motion, gestures, states and accessibility, with no product rules | — |
| `extension-popup` | [`src/lib.rs`](/crates/extension-popup/src/lib.rs) | The Chrome extension's popup, built from `ui`: where the connection stands, in one line, and its one action. The extension's worker owns the connection | [Chrome extension](#chrome-extension) |
| `http` | [`src/lib.rs`](/crates/http/src/lib.rs) | Request and stream mechanics with no schema knowledge | [Backend architecture](backend-architecture.md) |
| `gmail` | [`src/lib.rs`](/crates/gmail/src/lib.rs) | Plain-text or HTML messages through the Gmail API or the Workspace SMTP relay, plus the intake primitives: push-wake routing ([`src/push.rs`](/crates/gmail/src/push.rs)) and the `history.list` cursor reader ([`src/history.rs`](/crates/gmail/src/history.rs)). Products own the copy, the links, the durable cursor and the effects | [GCP](gcp.md) |
| `spruce` | [`src/lib.rs`](/crates/spruce/src/lib.rs) | Read-only Spruce REST reader and change cursor. Products own the cursor store, the effects, and patient linking | — |
| `pubsub` | [`src/lib.rs`](/crates/pubsub/src/lib.rs) | Pub/Sub push-envelope decoding, the loopback-emulator check and the official publisher; the `test-util` feature is the disposable emulator for tests (dev-dependencies only, checked by `boundaries`) | [GCP](gcp.md) |
| `encoder` | [`src/lib.rs`](/crates/encoder/src/lib.rs), [`src/main.rs`](/crates/encoder/src/main.rs) | Offline BGE encoder binary that speaks both the Duet and the physician encoder protocols; [`tests/compare.rs`](/crates/encoder/tests/compare.rs) checks it against the existing Python workers. No crate depends on it | [Physician site](physician-site.md) |
| `harness` | [`src/lib.rs`](/crates/harness/src/lib.rs) | One product-neutral bounded provider/tool loop in `engine`; `execution` forwards through compatibility adapters and preserves its saved request, response and record formats. Also reads `skills/` and admits `skill_view`. Hosted physician chat in `physician-worker` loads the catalog through `local-product`; nothing else does. Products keep authority, effects, persistence, and recovery. | <a href="/intent/harness%2Emd">Harness spec</a> |
| `agent-skills` | [`src/lib.rs`](/crates/agent-skills/src/lib.rs) | The skill catalog reader for [`skills/`](/skills), used by `harness` | [Physician site](physician-site.md) |
| `telemetry` | [`src/lib.rs`](/crates/telemetry/src/lib.rs) | Shared tracing, logging, and OpenTelemetry setup for the Rust services | [Observability](observability.md) |

## Duet (patient app)

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `duet-app` | [`src/lib.rs`](/crates/duet-app/src/lib.rs), [`src/roots.rs`](/crates/duet-app/src/roots.rs) | Standalone account entry, tab roots and connection controls | [UI and design](ui-surface-map.md) |
| `duet-api` | [`src/lib.rs`](/crates/duet-api/src/lib.rs), [`src/main.rs`](/crates/duet-api/src/main.rs) | Axum owner for identity, evidence, conversation, saved work, privacy and the existing connection adapter | [Backend architecture](backend-architecture.md), [Patient front door](front-door.md) |
| `duet-client` | [`src/lib.rs`](/crates/duet-client/src/lib.rs) | Duet caller over the generated client, adding the app-build headers | [Backend architecture](backend-architecture.md) |
| `duet-wire` (package `duet_wire`) | [`src/lib.rs`](/crates/duet-wire/src/lib.rs), <a href="/crates/duet-wire/README%2Emd">generated API index</a> | Models, operations and reference docs generated from the pinned coda-app OpenAPI plus Duet-owned additions. Regenerate with `cargo run -p workspace-tools -- generate-duet-wire`; do not edit outputs by hand. | [API generation](backend-architecture.md#api-and-clients) |
| `duet-health` | [`src/lib.rs`](/crates/duet-health/src/lib.rs), [`src/acquisition.rs`](/crates/duet-health/src/acquisition.rs), [`src/outbox.rs`](/crates/duet-health/src/outbox.rs) | On-device health samples from HealthKit and Health Connect; durable acquisition records, the native source adapter contract and the transactional upload outbox (account fencing and the batch transport live in `duet-client::health_upload`) | [Duet backend](backend-architecture.md) |
| `duet-notify` | [`src/lib.rs`](/crates/duet-notify/src/lib.rs) | Alert permission and one local insight alert | [Duet backend](backend-architecture.md) |
| `duet-page` | [`src/lib.rs`](/crates/duet-page/src/lib.rs) | The US Letter doctor-page layout, where overflow blocks sharing | [UI and design](ui-surface-map.md) |
| `duet-widget` | [`src/lib.rs`](/crates/duet-widget/src/lib.rs) | The snapshot that the native home-screen widgets read | [UI and design](ui-surface-map.md) |
| `duet-stub` | [`src/lib.rs`](/crates/duet-stub/src/lib.rs), [`src/main.rs`](/crates/duet-stub/src/main.rs) | Local Duet stand-in for the emulator and tests (`cargo run -p duet-stub`) | [Release](release.md#android-and-google-play) |

## Physician product

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `physician-app` | [`src/lib.rs`](/crates/physician-app/src/lib.rs), [`src/main.rs`](/crates/physician-app/src/main.rs) | The physician Dioxus shell: sidebar, Today, patient page, and reader | [Physician site](physician-site.md) |
| `physician-wire` | [`src/lib.rs`](/crates/physician-wire/src/lib.rs) | The physician read types, following coda-agents' <a href="/submodules/coda-agents/src/server/apiTypes%2Ets"><code>apiTypes.ts</code></a> | [Physician site](physician-site.md) |
| `physician-client` | [`src/lib.rs`](/crates/physician-client/src/lib.rs) | The caller for the Node `CodaServer` and the sample server | [Physician site](physician-site.md) |
| `physician-sample` | [`src/lib.rs`](/crates/physician-sample/src/lib.rs), [`src/main.rs`](/crates/physician-sample/src/main.rs) | Serves the page and the synthetic practice, plus auth and live chat routes; the `physician-migrate` binary runs the physician schema migrations | [Physician site](physician-site.md) |
| `physician-auth` | [`src/lib.rs`](/crates/physician-auth/src/lib.rs), [`src/bin/physician-auth-migrate.rs`](/crates/physician-auth/src/bin/physician-auth-migrate.rs) | Physician identity, membership, panels, and sessions | [Physician site](physician-site.md#physician-sign-in-and-practice-access) |
| `local-product` | [`src/lib.rs`](/crates/local-product/src/lib.rs) | Model execution in `live`, `record`, `replay`, and `api` modes; the physician runtime store, and hosted chat's instructions and `skill_view` tool (`physician_chat`) | [Physician site](physician-site.md) |
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

## Tooling and guards

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `patient-bank` | [`src/lib.rs`](/crates/patient-bank/src/lib.rs), [`src/main.rs`](/crates/patient-bank/src/main.rs) | The synthetic patient bank: chart authoring checks, the verified code catalog, the FHIR build and its validation | [Synthetic patients](patient-bank.md) |
| `ui-gallery` | [`src/main.rs`](/crates/ui-gallery/src/main.rs), [`scripts/ui-gallery.sh`](/scripts/ui-gallery.sh) | Every `ui` family rendered with its states, each at its own address, the command that builds and serves it, and the browser test that opens every family. Nothing depends on it | — |
| `boundaries` | [`src/lib.rs`](/crates/boundaries/src/lib.rs) | Tests that keep product dependencies apart, retain the reference shipping clients, and hold both apps' raw controls, hand-built cards and library overrides to a [checked-in table](/crates/boundaries/src/shared_ui.rs) | This page |
| `workspace-tools` | [`src/main.rs`](/crates/workspace-tools/src/main.rs) | Workspace command entrypoints, including `generate-duet-wire`, `install-doppler`, and `apply-migrations`; supporting checks and generator assets remain in [`scripts/`](/scripts) | [Build, CI, and release](release.md) |
| `android` | [`src/main.rs`](/crates/android/src/main.rs) | Opens the Android emulator window and runs one UI Automator class (`cargo run -p android -- test coda.duet.ModeShellTest`) | [Release](release.md#android-and-google-play) |

## Platform bindings

[`platform/`](/platform) holds the native code that the Rust crates call. It is not a Cargo crate.

| File | Bound to |
| --- | --- |
| [`ios/HealthKitBridge.swift`](/platform/ios/HealthKitBridge.swift) | `duet-health` |
| [`ios/NotificationBridge.swift`](/platform/ios/NotificationBridge.swift) | `duet-notify` |
| [`ios/StatWidget.swift`](/platform/ios/StatWidget.swift) | `duet-widget` snapshot |
| [`android/HealthConnectBridge.kt`](/platform/android/HealthConnectBridge.kt) | `duet-health` |
| [`android/NotificationBridge.kt`](/platform/android/NotificationBridge.kt) | `duet-notify` |
| [`android/StatWidget.kt`](/platform/android/StatWidget.kt) | `duet-widget` snapshot |
| [`android/HardwareKeyDispatch.kt`](/platform/android/HardwareKeyDispatch.kt) | `mobile` WebView key input, copied in by `cargo run -p android -- build` ([`build.rs`](/crates/android/src/build.rs)) |
| [`android/ModeShellTest.kt`](/platform/android/ModeShellTest.kt) | The UI Automator class that `android` runs |

## Chrome extension

[`extension/`](/extension) holds Coda's Chrome extension, which reads the practice's EHR under the physician's own sign-in, and its local sandbox bridge and tests. Its popup is the `extension-popup` crate; the rest is not a Cargo crate. Its <a href="/extension/README%2Emd">README</a> covers loading it and running a sandbox read.
