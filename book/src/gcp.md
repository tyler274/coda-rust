# GCP deployment

Duet production composition stays in <a href="/submodules/coda-app/infra/gcp/README%2Emd"><code>infra/gcp/README.md</code></a>. The physician OpenTofu roots in this workspace are [`foundation`](/infra/gcp/foundation), <a href="/infra/gcp/staging/README%2Emd"><code>staging</code></a>, <a href="/infra/gcp/clinical-evidence/README%2Emd"><code>clinical-evidence</code></a>, and <a href="/infra/gcp/physician-runtime/README%2Emd"><code>physician-runtime</code></a>. The last one is the GKE Autopilot and Cloud SQL target for `physician-sample` and `physician-auth`. This workspace does not apply those reference roots. Public-site staging and the isolated Rust staging root are workspace-owned and require separately authorized saved-plan applies. [Build, CI, and release](release.md#physician-hosting) says what each root owns.

## Public marketing site preparation

The separate [`infra/gcp/public-site`](/infra/gcp/public-site) root prepares the
stateless Rust marketing server on Cloud Run. Staging runs in the existing
`coda-health-staging` project with direct Cloud Run IAP and no load balancer or
CDN. Access is a service-scoped `roles/iap.httpsResourceAccessor` binding for
`group:gcp-engineering@codahealth.ai`, and the IAP service agent is the only
invoker. The origin also verifies the signed IAP assertion. Production stays behind
a global external HTTPS load balancer with Cloud CDN. There is no Cloudflare
dependency. Unlike the physician roots, this root owns its **staging** state
(GCS prefix `public-site/staging` in the existing state bucket) and may be planned
and applied for staging from this workspace. It is never planned or applied for
production here. Staging is deployed at `staging.codahealth.ai` behind IAP (see
[deployment status](public-site.md#staging-deployment-status-2026-10-21)). The
root does not share physician/Duet databases or authorize a product cutover. See
[Public site](public-site.md) for the architecture, IAP and cache controls, runtime
configuration, image build/smoke procedure, verification evidence and the
[staging state and deployment](public-site.md#staging-state-and-deployment)
procedure.

[PR #36](https://github.com/Coda-Health/coda-rust/pull/36) merged the implementation;
[ENG-79](https://linear.app/coda-health/issue/ENG-79) still owns hosting migration
completion. Dev's staging review and production approval, hostname/IAP/analytics
decisions, verified cutover/recovery procedures and the agreed recovery window
precede removal of website-only Vercel resources and credentials. Neither this
chapter nor passing local/CI validation authorizes deployment, DNS changes or
Vercel retirement.

## Rust container images

`nix/packages.nix` builds the Rust Cloud Run images from `Cargo.lock` and `flake.lock`. Each image tag is the git revision, and the build sets `created` to the epoch, so a revision always produces the same archive.

| Flake output | Image | Role | Entrypoint binary |
| --- | --- | --- | --- |
| `physician-sample-image` | `coda-physician-sample` | runtime | `physician-sample` serving `PHYSICIAN_SITE_DIR` (Dioxus WASM bundle) |
| `duet-api-image` | `coda-duet-api` | runtime | `duet-api` serving `DUET_SITE_DIR` (Dioxus WASM bundle) |
| `physician-migration-image` | `coda-physician-migrate` | migration | `physician-migrate`: physician-auth, front-door, then runtime chains |
| `duet-migration-image` | `coda-duet-migrate` | migration | `duet-migrate`: front-door chain, then the Duet coverage check |
| `duet-privacy-delivery-image` | `coda-duet-privacy-delivery` | job | `duet-privacy-delivery`, a bounded Luna receipt job |

- The runtime servers never migrate. They bind `0.0.0.0:$PORT`, where `PORT` defaults to 8080. `/health` reports liveness. Hosted startup accepts only the explicit `stg` / `staging-preview` profile with its required stores and identity configuration; other hosted profiles refuse to start. `/readyz` checks the configured schemas. This preview profile and a successful schema check do not establish hosted sign-in, live-model acceptance or production readiness. The startup owners are [`physician-sample/src/hosting.rs`](/crates/physician-sample/src/hosting.rs) and [`duet-api/src/main.rs`](/crates/duet-api/src/main.rs).
- Each image contains the binary and the libraries it links, Doppler, CA roots, tzdata and a passwd entry for uid `65532`. The image has no shell or toolchain. It runs as `65532:65532` with working directory `/tmp`.
- Every entrypoint is `doppler run --no-fallback --preserve-env=PORT -- <binary>`. The Doppler CLI is the static release pinned in [`crates/workspace-tools/src/doppler.rs`](/crates/workspace-tools/src/doppler.rs). Nix reads the version and SHA-256 from that file, so the runners and the images run the same verified binary. Cloud Run mounts `DOPPLER_TOKEN` from Secret Manager. No secret is in a layer or in `Env`, and `--no-fallback` keeps secrets off the filesystem.
- `scripts/check-images.sh` (run with `nix develop .#release`) builds every image and inspects each archive. It loads them into Docker and checks rejection of missing credentials or an unconfigured hosted profile. Against disposable PostgreSQL it runs both migration images twice, then starts both servers to check `/health` 200, schema readiness 200, the WASM assets and a clean SIGTERM exit. The local Duet smoke uses a file journal and placeholder Identity Platform configuration; it does not exercise Cloud Run's required independent GCS journal or real sign-in. The `images` CI job runs this script. Nothing is pushed.

## Rust staging bootstrap and deployment gate

The isolated project is `coda-rust-staging` in `us-east1`. It is separate from
`coda-health-staging`, which hosts the public marketing site. The root owns its
GCS state under `rust-environments/stg`; the reviewed nonsecret inputs are
[`env/staging.tfvars`](/infra/gcp/rust-environments/env/staging.tfvars) and
[`backend/staging.gcs.tfbackend`](/infra/gcp/rust-environments/backend/staging.gcs.tfbackend).

From an authorized engineering account:

```bash
root=infra/gcp/rust-environments
nix develop .#release --command tofu -chdir="$root" init -input=false \
  -backend-config=backend/staging.gcs.tfbackend
nix develop .#release --command tofu -chdir="$root" plan -input=false \
  -var-file=env/staging.tfvars -out=staging.tfplan
```

Review the saved plan before any separately authorized apply. Engineering-group
access owns routine infrastructure administration and state access. Project IAM,
custom-role changes and billing administration remain with Dev. The administrator
bootstrap installs conditional database IAM grants before databases exist, so
later database activation does not itself need another project-IAM grant.

Engineering-group access includes Identity Platform administration for Duet's
hosted sign-in setup. Tyler's domain-mapping access concern remains unverified;
this root does not create domain mappings. The foundation plan was verified under
Dev's account; an infrastructure plan and deployment from Tyler's account remain
unverified.

The foundation comprises the private network and SQL peering, five service
accounts, GitHub WIF, immutable application registry, four empty Doppler bootstrap
secret slots with separate readers, CI grants and 30-day logs. Dev owns the
separate `coda-rust-stg` $10 monthly budget alert in Cloud Billing, using the
existing billing recipients; it is not a spending cap. Budgets are deliberately
outside this state so an engineering plan does not need billing-account access.
The committed inputs keep databases and workloads disabled.

The `deploy-staging` job in [`.github/workflows/ci.yml`](/.github/workflows/ci.yml)
runs on pushes to `main` only when the repository variable
`RUST_STAGING_DEPLOY` is `enabled`. Keep it unset until runtime acceptance and the
release integration below are complete, saved-plan applies have succeeded, and
the GitHub `staging` environment holds
`GCP_WORKLOAD_IDENTITY_PROVIDER`, `GCP_DEPLOYER_SERVICE_ACCOUNT`,
`GCP_PROJECT_ID`, `GCP_REGION` and the qualified `RELEASE_ENVIRONMENT`. Only after
that work and deployment approval, enable it:

```bash
gh variable set RUST_STAGING_DEPLOY --body enabled
```

Unsetting the variable stops staging deploys without editing the workflow.

The first deployment remains gated. Bootstrap and green CI do not resolve these
current owners' outstanding work:

- **Hosted identity:** physician [`auth.rs`](/crates/physician-sample/src/auth.rs)
  still returns proof tokens from sign-in. Duet web requires a registered Apple
  Services ID/return URL, its SDK, Identity Platform configuration and the public
  build-time settings consumed by [`browser_identity.rs`](/crates/duet-app/src/browser_identity.rs)
  and [`account.rs`](/crates/duet-app/src/account.rs); the current Nix image builder
  does not supply those settings.
- **Runtime and data setup:** activate the two separate SQL instances, install
  the SQL grants, run the explicit migration images, configure the independent
  Duet GCS privacy journal, and populate four least-scope Doppler configs/tokens.
  Verify private TLS, account isolation, retained conversations, readiness and
  the runtime's actual model/pool limits before accepting the activation gate.
- **Release wiring:** the current workflow does not produce the immutable
  build/runtime/compatibility manifests required by
  [`release_commands.py`](/scripts/gcp/release_commands.py), nor configure its
  deployer/verifier/smoke identities. Its region/token environment and `ci` job
  lookup also need correction. The runner requires previous-release evidence,
  so the first-release bootstrap must be explicitly implemented and verified;
  do not fabricate a previous release or weaken the evidence checks.

These are deployment blockers, not evidence that ENG-167's hosted acceptance is
complete. No Rust application URL exists until the workloads are deployed.

## Release procedure

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#environments">Environments</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#branch-rehearsal">Branch rehearsal</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#staging-window">Staging window</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#release-runner">Release runner</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#one-time-staging-provider-admission">One-time staging provider admission</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#compatibility-checked-rollback">Compatibility-checked rollback</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#staging-identity-setup-and-deletion-proof">Staging identity setup and deletion proof</a>

## Operations

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#monitoring">Monitoring</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#required-services">Required services</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#current-launch-guardrails">Current launch guardrails</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#doctor-page-cutover">Doctor-page cutover</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#worker-queue-capacity">Worker queue capacity</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#continuation-preparation-queue">Continuation preparation queue</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#scheduled-conversation-queue">Scheduled conversation queue</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#follow-up-and-review-wake-job">Follow-up and review wake job</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#acquisition-worker-operations">Acquisition worker operations</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#secrets-and-runtime-settings">Secrets and runtime settings</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#minimum-supported-iphone-build">Minimum supported iPhone build</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#sign-out-everywhere-readback">Sign out everywhere readback</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#model-cost-controls">Model cost controls</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#order">Order</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#removed-runtime-decommission">Removed runtime decommission</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#access-and-security-operations">Access and security operations</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#cloud-sql-restore-drill">Cloud SQL restore drill</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#privacy-reconciliation-before-recovery-serves-traffic">Privacy reconciliation before recovery serves traffic</a>

## Dated release and security evidence

Source: <a href="/submodules/coda-app/infra/gcp/README%2Emd#dated-release-and-security-evidence"><code>Dated release and security evidence</code></a>.

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-5-2026-spend-records-and-reviewed-doctor-page-release">October 5, 2026 spend records and reviewed doctor page release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-5-2026-structural-cleanup-release">October 5, 2026 structural cleanup release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-3-2026-wave-1-and-genome-removal-release">October 3, 2026 Wave 1 and Genome removal release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-2-2026-patient-isolation-and-graceful-runs-release">October 2, 2026 patient isolation and graceful runs release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-1-2026-security-and-build-133-release">October 1, 2026 security and build 133 release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-1-2026-doctors--hospitals-release">October 1, 2026 Doctors &amp; hospitals release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-30-2026-minimum-build-131-release">September 30, 2026 minimum build 131 release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-30-2026-companion-release">September 30, 2026 companion release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-27-2026-first-session-recovery-release">September 27, 2026 first-session recovery release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#earlier-releases">Earlier releases</a>
