# Crate map

Every crate in [`crates/`](/crates) is listed here, with the file where it starts and the chapter that covers its behavior. The module docs at the top of each entry file are the maintained description. When this table and a crate disagree, the crate is right and this table needs fixing. [`boundaries`](/crates/boundaries/src/lib.rs) keeps Duet and the physician client from importing each other's clinical crates. `ui` and `http` are the only crates the two products share.

## Shared

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `ui` | [`src/lib.rs`](/crates/ui/src/lib.rs) | Shared tokens and the tab bar for both Dioxus apps | [UI surface map](ui-surface-map.md) |
| `http` | [`src/lib.rs`](/crates/http/src/lib.rs) | Request and stream mechanics with no schema knowledge | [Backend architecture](backend-architecture.md) |
| `encoder` | [`src/lib.rs`](/crates/encoder/src/lib.rs), [`src/main.rs`](/crates/encoder/src/main.rs) | Offline BGE encoder used by Duet discovery and physician retrieval | [Physician site](physician-site.md) |

## Duet (patient app)

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `duet-app` | [`src/lib.rs`](/crates/duet-app/src/lib.rs), [`src/roots.rs`](/crates/duet-app/src/roots.rs) | The four tab roots and the join and consent screens | [North Star](north-star.md), [UI surface map](ui-surface-map.md) |
| `duet-api` | [`src/lib.rs`](/crates/duet-api/src/lib.rs), [`src/main.rs`](/crates/duet-api/src/main.rs) | Axum owner for moments, account, and session routes, with the front door embedded for join, consent, and premium | [Backend architecture](backend-architecture.md), [Patient front door](front-door.md) |
| `duet-client` | [`src/lib.rs`](/crates/duet-client/src/lib.rs) | Duet caller over the generated client, adding the app-build headers | [Backend architecture](backend-architecture.md) |
| `duet-wire` (package `duet_wire`) | [`src/lib.rs`](/crates/duet-wire/src/lib.rs) | Models and operations generated from <a href="/submodules/coda-app/docs/openapi%2Ejson">coda-app's <code>openapi.json</code></a> by [`scripts/generate-duet-wire.sh`](/scripts/generate-duet-wire.sh). Regenerate it; do not edit it by hand. | [Backend architecture](backend-architecture.md) |
| `duet-health` | [`src/lib.rs`](/crates/duet-health/src/lib.rs) | On-device health samples from HealthKit and Health Connect | [Behaviour contract](behaviour-contract.md) |
| `duet-notify` | [`src/lib.rs`](/crates/duet-notify/src/lib.rs) | Alert permission and one local insight alert | [Behaviour contract](behaviour-contract.md) |
| `duet-page` | [`src/lib.rs`](/crates/duet-page/src/lib.rs) | The US Letter doctor-page layout, where overflow blocks sharing | [North Star](north-star.md), [Insight visual contract](visual-contract.md) |
| `duet-widget` | [`src/lib.rs`](/crates/duet-widget/src/lib.rs) | The snapshot that the native home-screen widgets read | [North Star](north-star.md) |
| `duet-stub` | [`src/lib.rs`](/crates/duet-stub/src/lib.rs), [`src/main.rs`](/crates/duet-stub/src/main.rs) | Local Duet stand-in for the emulator and tests (`cargo run -p duet-stub`) | [Release](release.md#android-and-google-play) |

## Physician product

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `physician-app` | [`src/lib.rs`](/crates/physician-app/src/lib.rs), [`src/main.rs`](/crates/physician-app/src/main.rs) | The physician Dioxus shell: sidebar, Today, patient page, and reader | [Physician site](physician-site.md) |
| `physician-wire` | [`src/lib.rs`](/crates/physician-wire/src/lib.rs) | The physician read types, following coda-agents' <a href="/submodules/coda-agents/src/server/apiTypes%2Ets"><code>apiTypes.ts</code></a> | [Physician site](physician-site.md) |
| `physician-client` | [`src/lib.rs`](/crates/physician-client/src/lib.rs) | The caller for the Node `CodaServer` and the sample server | [Physician site](physician-site.md) |
| `physician-sample` | [`src/lib.rs`](/crates/physician-sample/src/lib.rs), [`src/main.rs`](/crates/physician-sample/src/main.rs) | Serves the page and the recorded synthetic practice, plus auth and live chat routes | [Physician site](physician-site.md) |
| `physician-auth` | [`src/lib.rs`](/crates/physician-auth/src/lib.rs), [`src/bin/physician-auth-migrate.rs`](/crates/physician-auth/src/bin/physician-auth-migrate.rs) | Physician identity, membership, panels, and sessions | [Physician site](physician-site.md#physician-sign-in-and-practice-access) |
| `local-product` | [`src/lib.rs`](/crates/local-product/src/lib.rs) | Model execution in `live`, `record`, `replay`, and `api` modes | [Physician site](physician-site.md) |
| `patient-work` | [`src/lib.rs`](/crates/patient-work/src/lib.rs) | The patient-work dispatcher's grouping window | [Physician site](physician-site.md) |
| `harness` | [`src/lib.rs`](/crates/harness/src/lib.rs) | Loads `skills/` once and admits `skill_view` | [Physician site](physician-site.md) |
| `agent-skills` | [`src/lib.rs`](/crates/agent-skills/src/lib.rs) | The skill catalog reader for [`skills/`](/skills) | [Physician site](physician-site.md) |

## Between the products

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `front-door` | [`src/lib.rs`](/crates/front-door/src/lib.rs), [`src/bin/front-door-migrate.rs`](/crates/front-door/src/bin/front-door-migrate.rs) | Patient accounts, practice invitations, the roster link, consent, and premium | [Patient front door](front-door.md) |
| `mobile` | [`src/lib.rs`](/crates/mobile/src/lib.rs), [`src/main.rs`](/crates/mobile/src/main.rs) | The one Android and iOS shell, with an explicit physician or client mode. It is the only crate that mounts both apps. | [Release](release.md#android-and-google-play) |

## Tooling and guards

| Crate | Entry | Owns | Chapter |
| --- | --- | --- | --- |
| `boundaries` | [`src/lib.rs`](/crates/boundaries/src/lib.rs) | Tests that keep the product crates apart and require the book to cite every Duet document heading | This page |
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
| [`android/HardwareKeyDispatch.kt`](/platform/android/HardwareKeyDispatch.kt) | `mobile` WebView key input, copied in by [`scripts/android-debug.sh`](/scripts/android-debug.sh) |
| [`android/ModeShellTest.kt`](/platform/android/ModeShellTest.kt) | The UI Automator class that `android` runs |

## Not in Rust yet

Each of these chapters says which part is still owned by Python, Swift, or TypeScript: [Privacy recovery](privacy-recovery.md), [Security posture](security-posture.md), [Clinical catalog](clinical-catalog-migration.md), [Coda Gym](coda-gym.md), and [Computer use](computer-use.md).
