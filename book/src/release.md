# Build, CI, and release

This page links the owners of each release path. Each owner keeps its own procedure. The rule in <a href="/AGENTS%2Emd#dev-shell"><code>AGENTS.md</code></a> still applies: this repository does not claim a production cutover, a TestFlight release, or a physician production deploy.

## Continuous integration

- [`.github/workflows/ci.yml`](/.github/workflows/ci.yml) is the only workflow. The `ci` job runs these steps in order: `cargo fmt --check`, clippy, `cargo test --workspace`, the browser proofs (`physician-sample` auth, site, and React, then the `mobile` mode shells), the Doppler installer syntax check, then `mdbook build book`, the [book link check](setup.md#reading-and-checking-this-book), and `cargo doc`. The `cargo-deny` job applies [`deny.toml`](/deny.toml).
- CI installs `wasm-bindgen`, mdBook, and lychee from pinned, checksum-verified release archives, not with `cargo install`. The comments next to each install step give the reason for its pin.
- Run the workflow locally with `act` from `nix develop`. [`.actrc`](/.actrc) selects the runner image and the `ci` job. Check the workflow with `actionlint .github/workflows/ci.yml`.
- [`.githooks/pre-commit`](/.githooks/pre-commit) runs the same rustfmt and clippy steps before a commit.

## Secrets: Doppler

<a href="/AGENTS%2Emd#dev-shell"><code>AGENTS.md</code>: Dev shell</a> owns this policy. In short: Doppler project `coda-rust` holds every secret value in configs `dev`, `stg`, and `prd`. GCP Secret Manager holds only a read-only `DOPPLER_TOKEN` for one config. The process starts under `doppler run --`. [`scripts/gcp/install_doppler.sh`](/scripts/gcp/install_doppler.sh) installs the pinned CLI for images and runners, and [`doppler.yaml`](/doppler.yaml) selects `dev` locally. CI does not call Doppler.

## Physician hosting

| Root | Owns | Owner document |
| --- | --- | --- |
| [`infra/gcp/physician-runtime`](/infra/gcp/physician-runtime) | The private GKE Autopilot cluster, Cloud SQL with IAM authentication, and the site and migration identities for `physician-sample` and `physician-auth` | <a href="/infra/gcp/physician-runtime/README%2Emd">README</a>, including its `tofu test` and `validate` commands |
| [`infra/gcp/foundation`](/infra/gcp/foundation) | Shared project foundation | [`main.tf`](/infra/gcp/foundation/main.tf) |
| [`infra/gcp/staging`](/infra/gcp/staging) | The hosted synthetic integration platform on Cloud Run | <a href="/infra/gcp/staging/README%2Emd">README</a> |
| [`infra/gcp/clinical-evidence`](/infra/gcp/clinical-evidence) | The production Clinical Evidence runbook | <a href="/infra/gcp/clinical-evidence/README%2Emd">README</a> |

This workspace does not run `init` against a backend and does not apply these roots. Apply authority stays with the product repositories and the operator of the target project. [GCP deployment](gcp.md) links Duet's production composition.

## Duet backend on GCP

Duet's production composition, staging windows, release runner, and rollback belong to <a href="/submodules/coda-app/infra/gcp/README%2Emd">coda-app <code>infra/gcp/README.md</code></a>. Its GitHub workflows are in <a href="/submodules/coda-app/%2Egithub/workflows">coda-app <code>.github/workflows</code></a>. [GCP deployment](gcp.md) lists each section.

## iPhone and TestFlight

- [TestFlight](testflight-release.md) links coda-app's runbook, which owns distribution identity, build history, and the current procedure. A Rust build is not a TestFlight upload.
- [iOS verification](ios-verification.md) links the proof matrix that a release must meet.
- [Consumer-health release](consumer-health-release-approval.md) and [Epic MyChart](epic-mychart-release-approval.md) link the approval packets.
- The Rust bindings for iPhone are in [`platform/ios`](/platform/ios) (see the [crate map](crate-map.md#platform-bindings)).

## Android and Google Play

- [`scripts/android-debug.sh`](/scripts/android-debug.sh) builds the debug APK for the `mobile` mode shell. Its header comment explains the toolchain it needs.
- `cargo run -p duet-stub` serves the host stand-in that the emulator calls. `cargo run -p android -- window` opens the emulator, and `cargo run -p android -- test coda.duet.ModeShellTest` runs UI Automator (see [`crates/android`](/crates/android/src/main.rs)). CI does not run the emulator.
- This repository has no signed release build and no Google Play procedure yet. Do not treat a debug APK as a Play upload.
