# Build, CI, and release

This page links the owners of each release path. Each owner keeps its own procedure. The rule in <a href="/AGENTS%2Emd#dev-shell"><code>AGENTS.md</code></a> still applies: this repository does not claim a production cutover, a TestFlight release, or a physician production deploy.

## Continuous integration

- [`.github/workflows/ci.yml`](/.github/workflows/ci.yml) owns pull-request and main checks plus the existing staging deploys. Routine CI runs formatting, Clippy, `cargo test --workspace` and documentation checks. Browser tests are ignored by the ordinary test harness; their explicit targets run on `workflow_dispatch`. A green routine CI run does not establish browser qualification. The workflow and its comments own the exact step list.
- The other jobs are `foundations` (offline cloud foundation contracts and mocked release adapters), `images` (the locked Nix images, built and run but not pushed), `postgres` (fresh migrations and the Postgres proofs), `observability` (the collector config and its OpenTofu roots), and `cargo-deny` (which applies [`deny.toml`](/deny.toml)).
- `deploy-public-site-staging` applies the public site's reviewed staging plan on pushes to `main` only when the repository variable `PUBLIC_SITE_STAGING_DEPLOY` is `enabled` ([public site](public-site.md)).
- [`.github/workflows/promote-public-site.yml`](/.github/workflows/promote-public-site.yml) is a manual workflow that promotes the image staging tested for one `main` commit to public-site production, with the `production` environment's approval ([promotion](public-site.md#promotion)).
- [`.github/workflows/doppler-watch.yml`](/.github/workflows/doppler-watch.yml) is a separate daily workflow that watches the pinned Doppler CLI for releases and advisories.
- `deploy-staging` runs on pushes to `main` only when the repository variable `RUST_STAGING_DEPLOY` is `enabled`. It downloads the successful image proof's archives from the same run, verifies their commit and checksums, and publishes those exact images without rebuilding. It runs both migration jobs and moves the `coda-rust-staging` services to Ready revisions on the new digests. `physician-worker` is updated once the scaling plan has created that service; [direct staging rollout](gcp.md#direct-staging-rollout) describes the three-day artifact retention, retry behavior and how to turn it off.
- Rust build caches are best effort. A restore, save, or cache-size failure can make the next build slower; it does not invalidate passing checks.
- CI installs `wasm-bindgen`, mdBook, and lychee from pinned, checksum-verified release archives, not with `cargo install`. The comments next to each install step give the reason for its pin.
- Run the workflow locally with `act` from `nix develop`. [`.actrc`](/.actrc) selects the runner image and the `ci` job. Check the workflow with `actionlint .github/workflows/ci.yml`.
- [`.githooks/pre-commit`](/.githooks/pre-commit) runs the same rustfmt and clippy steps before a commit.

## Secrets: Doppler

<a href="/AGENTS%2Emd#dev-shell"><code>AGENTS.md</code>: Dev shell</a> owns this policy. In short: Doppler project `coda-rust` holds every secret value in configs `dev`, `stg`, and `prd`. GCP Secret Manager holds only a read-only `DOPPLER_TOKEN` for one config. The process starts under `doppler run --`. `cargo run -p workspace-tools -- install-doppler OUTPUT_PATH` ([`doppler.rs`](/crates/workspace-tools/src/doppler.rs)) installs the pinned CLI for images and runners, and [`doppler.yaml`](/doppler.yaml) selects `dev` locally. CI does not call Doppler.

## Retained physician infrastructure references

| Root | Owns | Owner document |
| --- | --- | --- |
| [`infra/gcp/physician-runtime`](/infra/gcp/physician-runtime) | The private GKE Autopilot cluster, Cloud SQL with IAM authentication, and the site and migration identities for `physician-sample` and `physician-auth` | <a href="/infra/gcp/physician-runtime/README%2Emd">README</a>, including its `tofu test` and `validate` commands |
| [`infra/gcp/foundation`](/infra/gcp/foundation) | Shared project foundation | [`main.tf`](/infra/gcp/foundation/main.tf) |
| [`infra/gcp/staging`](/infra/gcp/staging) | The hosted synthetic integration platform on Cloud Run | <a href="/infra/gcp/staging/README%2Emd">README</a> |
| [`infra/gcp/clinical-evidence`](/infra/gcp/clinical-evidence) | The production Clinical Evidence runbook | <a href="/infra/gcp/clinical-evidence/README%2Emd">README</a> |

This workspace does not run `init` against a backend and does not apply these roots. Apply authority stays with the product repositories and the operator of the target project. [GCP deployment](gcp.md) links Duet's production composition.

## Workspace-owned roots

| Root | Owns | Owner document |
| --- | --- | --- |
| [`infra/gcp/public-site`](/infra/gcp/public-site) | Public-site Cloud Run: staging behind IAP, production behind the load balancer without sign-in. This workspace owns both states and applies each only from a reviewed saved plan; production applies need Dev's explicit authorization. | [Public site](public-site.md#production-state-and-deployment) |
| [`infra/gcp/rust-environments`](/infra/gcp/rust-environments) | The isolated Rust Cloud Run environments for the physician and Duet services. It needs an administrator bootstrap before its first apply. | [GCP deployment](gcp.md#rust-staging-bootstrap-and-deployment-gate) |
| [`infra/gcp/shared-mail`](/infra/gcp/shared-mail) | Production mail only in `coda-health-prod`, with separate state and operator-approved saved-plan applies. | <a href="/infra/gcp/shared-mail/README%2Emd">Production shared mail</a> |
| [`infra/gcp/observability`](/infra/gcp/observability) | The shared collector deployment | [Observability](observability.md) |

## Duet backend on GCP

The original Python Duet service's production composition, staging windows, release runner, and rollback are retained in <a href="/submodules/coda-app/infra/gcp/README%2Emd">coda-app <code>infra/gcp/README.md</code></a>. Its GitHub workflows are in <a href="/submodules/coda-app/%2Egithub/workflows">coda-app <code>.github/workflows</code></a>. [GCP deployment](gcp.md) lists each section.

## iPhone and TestFlight

- The original [TestFlight runbook](https://github.com/Coda-Health/coda-app/blob/5a1dc47b8feacb8db3fcc71522982f76822c0b01/docs/testflight-release.md), [iOS verification](https://github.com/Coda-Health/coda-app/blob/5a1dc47b8feacb8db3fcc71522982f76822c0b01/docs/ios-verification.md), and [release packets](https://github.com/Coda-Health/coda-app/tree/5a1dc47b8feacb8db3fcc71522982f76822c0b01/docs) preserve historical distribution and approval evidence. They do not qualify the Rust app. A Rust build is not a TestFlight upload.
- The Rust bindings for iPhone are in [`platform/ios`](/platform/ios) (see the [crate map](crate-map.md#platform-bindings)).

## Android and Google Play

- `cargo run -p android -- build` ([`crates/android`](/crates/android/src/main.rs)) builds the debug APK for the `mobile` mode shell. Its module comment explains the toolchain it needs.
- `cargo run -p duet-stub` serves the host stand-in that the emulator calls. `cargo run -p android -- window` opens the emulator, and `cargo run -p android -- test coda.duet.ModeShellTest` runs UI Automator (see [`crates/android`](/crates/android/src/main.rs)). CI does not run the emulator.
- This repository has no signed release build and no Google Play procedure yet. Do not treat a debug APK as a Play upload.
