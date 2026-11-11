# Build, CI, and release

This page links the owners of each release path. Each owner keeps its own procedure.

## Continuous integration

- [`.github/workflows/ci.yml`](/.github/workflows/ci.yml) owns pull-request and main checks plus the existing staging deploys. Every run checks engineering skills and documentation. [CI selection](/scripts/ci_scope.py) adds the full Rust, database, infrastructure and image suite for product/build or mixed changes, and the runtime clinical `skills/` count as product; unavailable comparisons also select full. A change under `crates/workspace-tools/sandbox/` selects only the `sandbox` job, which runs `nix flake check ./crates/workspace-tools/sandbox`; a root `Cargo.toml` or `Cargo.lock` change selects it beside the full suite, and `CI result` requires it whenever it is selected. Browser tests are ignored by the ordinary test harness; their explicit targets run on `workflow_dispatch`. A green routine CI run does not establish browser qualification. The workflow and its comments own the exact step list.
- `main` takes pull requests through GitHub's merge queue, set in the repository's `main` ruleset. A pull request passes `CI result` on its own branch and need not be level with `main`. Merging it, or auto-merge, adds it to the queue, which builds a candidate from `main`, the entries ahead and this pull request, and runs the workflow on that candidate as a `merge_group` event. The pull request merges when the candidate's `CI result` passes, so other merges do not send it back for another branch update and run. A candidate is scoped by what it adds to the commit it was built on, runs no deployment, and saves no build cache.
- The other jobs are `foundations` (offline cloud foundation contracts and mocked release adapters), `images` (the locked Nix images, built and run but not pushed), `postgres` (fresh migrations and the Postgres proofs), `observability` (the collector config and its OpenTofu roots), and `cargo-deny` (which applies [`deny.toml`](/deny.toml)).
- `deploy-public-site-staging` applies the public site's reviewed staging plan after full validation on pushes to `main` only when the repository variable `PUBLIC_SITE_STAGING_DEPLOY` is `enabled` ([public site](public-site.md)).
- [`.github/workflows/promote-public-site.yml`](/.github/workflows/promote-public-site.yml) is a manual workflow that promotes the image staging tested for one `main` commit to public-site production, with the `production` environment's approval ([promotion](public-site.md#promotion)).
- [`.github/workflows/doppler-watch.yml`](/.github/workflows/doppler-watch.yml) is a separate daily workflow that watches the pinned Doppler CLI for releases and advisories.
- [PStack skill updates](/.github/workflows/pstack-watch.yml) uses the built-in `GITHUB_TOKEN` to open or refresh a PR for the <a href="/verification/README%2Emd#pstack">vendored verification skills</a>. Enable **Allow GitHub Actions to create and approve pull requests** in the repository's Actions settings for publication. A maintainer with write access approves the CI runs on the resulting PR.
- `deploy-staging` runs after full validation on pushes to `main` only when the repository variable `RUST_STAGING_DEPLOY` is `enabled`. It downloads the successful image proof's archives from the same run, verifies their commit and checksums, and publishes those exact images without rebuilding. It runs both migration jobs and moves the `coda-rust-staging` services to Ready revisions on the new digests. `physician-worker` is updated once the scaling plan has created that service; [direct staging rollout](gcp.md#direct-staging-rollout) describes the three-day artifact retention, retry behavior and how to turn it off.
- The staging release command in [`workspace-tools`](/crates/workspace-tools/src/staging_release.rs) owns rollout receipts and final-state reconciliation. It updates only each service's product container image and refuses a rollout that moves a sidecar or any other infrastructure-owned field ([direct staging rollout](gcp.md#direct-staging-rollout)). A partial rollout or unknown traffic is not success. [Operating limits and failure investigation](gcp.md#operating-limits-and-failure-investigation) distinguishes declared resource limits from verified spending controls; [privacy recovery](privacy-recovery.md#isolated-restore-qualification) describes the additional Duet restore gate. Mocked release tests and successful image checks do not qualify a hosted database restore.
- Skills/docs-only main pushes skip both deployments. A successful `CI result` confirms the selected checks passed, not that a deployment occurred.
- [`rust-toolchain.toml`](/rust-toolchain.toml) is the one Rust pin. The Nix packages and dev shells read it through [`nix/rust-toolchain.nix`](/nix/rust-toolchain.nix), the `ci` job installs it with `rustup`, and `cargo-deny` runs on it. A toolchain bump is its own PR, and `ci_scope.py` treats it as a dependency change. The Rust cache key includes the version, so it only changes in that PR.
- A weekly `schedule` run on `main` takes the full path without browser qualification or deployments. Restoring the `rust-v2-*` and `postgres-*-shard*` entries keeps them from being evicted after seven idle days, so the first PR after a quiet week starts warm. The private Nix cache has no idle expiry, so only `main` pushes publish to it. `workflow_dispatch` also runs the full path, plus browser qualification.
- Postgres shard caches save one entry per `main` or scheduled commit, so concurrent merges don't race for a key. Restores fall back from the same dependency set to the same shard. A PR saves only when `main` had nothing for its dependency set.
- Rust build caches are best effort. A restore, save, or cache-size failure can make the next build slower; it does not invalidate passing checks.
- CI installs `wasm-bindgen`, mdBook, and lychee from pinned, checksum-verified release archives, not with `cargo install`. The comments next to each install step give the reason for its pin.
- CI makes no anonymous GitHub downloads that it can authenticate, because GitHub throttles anonymous traffic first. Release archives ([`release-archive`](/.github/actions/release-archive/action.yml), with curl, since the `act` image has no `gh`) and the Doppler CLI come through the GitHub API with the job token; `github.com` download URLs ignore the token. OpenTofu fetches providers from those URLs and cannot send a token, so [`tofu-providers`](/.github/actions/tofu-providers/action.yml) caches them per job, keyed on the committed lock files.
- CI runs on Blacksmith runners, which boot GitHub's Ubuntu 24.04 image and serve `actions/cache` from Blacksmith's own cache. The `ci` job uses `blacksmith-4vcpu-ubuntu-2404` and `cargo-deny` uses `blacksmith-2vcpu-ubuntu-2404`. `images` uses `blacksmith-8vcpu-ubuntu-2404` when `ci_scope.py` reports a change to `Cargo.lock`, a `Cargo.toml`, the flake or `nix/`, since those rebuild the Rust dependency derivations; other runs use 2 vCPU. [`.actrc`](/.actrc) maps these labels to the `act` image, and [`.github/actionlint.yaml`](/.github/actionlint.yaml) declares them. To roll back, set `runs-on` to `ubuntu-latest`.
- The repository variable `CI_JOB_RUNNER` moves the `ci` job to another runner label; deleting it returns the job to Blacksmith.[^cyrene]
- Run the workflow locally with `act` from `nix develop`; it needs the host's Docker daemon, which the shell does not provide. [`.actrc`](/.actrc) selects the runner image and the `ci` job. Check the workflow with `actionlint .github/workflows/ci.yml`. Where there is no Docker, `coda-sandbox ci run [<repo-path-or-commit>] --job <job>` from [`crates/workspace-tools/sandbox/`](/crates/workspace-tools/sandbox/) runs the job of a committed revision the same way on a self-hosted CI spool, with no secrets, and exits with its status.[^cyrene]
- [`.githooks/pre-commit`](/.githooks/pre-commit) runs the same rustfmt and clippy steps before a commit.

[^cyrene]: The `cyrene-coda` label is a disposable Firecracker microVM on Cyrene, the developer workstation. It runs the job in the `act` image (`catthehacker/ubuntu`) rather than GitHub's, so a step that passes under `act` passes there, and it has no Blacksmith cache, so `actions/cache` restores miss. On Cyrene, agents work in the `coda-agent` container: Cursor connects to it with Remote-SSH, the main checkout is `/agents/coda/src/coda-rust` (snapshotted hourly), and git, `gh` and `doppler` get a token limited to this repository (and a read-only `dev` Doppler token) from a token broker on the host, which reads them from the developer's keyring, so no token sits in a file in the container. The container cannot reach the host, the LAN or Tailscale and has no Docker; `coda-sandbox ci run` queues its jobs on the same Firecracker runner. `coda-sandbox worktree new <slug>` makes each worktree under `.worktrees/<slug>` with a `target/` that is a reflink clone of a seed of third-party build output on the build root its config names (`/agents/coda/build` on Cyrene), so a worktree costs its source until a build diverges; `coda-sandbox worktree --help` lists the commands. Where `coda-sandbox` is not installed yet, run it from the checkout as `nix run ./crates/workspace-tools/sandbox -- worktree new <slug>`. The container, runner and their limits are described in [`crates/workspace-tools/sandbox/README.md`](/crates/workspace-tools/sandbox), whose NixOS module declares them.

## Private Nix binary cache

[`infra/gcp/nix-cache`](/infra/gcp/nix-cache) owns an isolated private GCS bucket
and bucket-scoped reader/writer identities. Select its project and remote-state
backend explicitly; apply only an operator-approved saved plan. It does not own
product deployment permissions. Nix uses GCS's S3 interoperability endpoint, so
its HMAC credentials are distinct from the WIF credentials used by deploy jobs.

Keep HMAC and signing secrets in Doppler, outside OpenTofu state. Deliver the
reader credentials to CI separately from the writer credentials. The reader HMAC
pair is a repository secret, because every Nix job reads the cache and most run
outside any environment; it grants only object reads on this bucket. The GitHub
`nix-cache-write` environment must restrict deployment branches to `main` before
installing any writer or signing secrets. Never put those secrets at repository
scope: a workflow condition alone cannot protect secrets from edited PR code.
Only the `nix-cache-publish` job enters `nix-cache-write`. It runs on a `main`
push after `images` passes, rebuilds the allowlisted roots from the cache, and
uploads them. `images` itself holds no cache-writer secrets on any event.
Changing cache trust or rotating its signing key requires operator review.
Publishers share the `nix-cache-main-writer` concurrency group because create-only
object permissions cannot safely handle simultaneous uploads of the same object.
An active publisher is not cancelled; GitHub may replace an older pending one
with a newer one, whose closure supersedes it. The image proofs, `CI result` and
deployments stay outside that group and do not wait for publication, so a burst
of merges cannot cancel them. Any additional publisher must use the same
serialization policy rather than weakening the bucket permissions.

The cache retains compiled dependency artifacts as well as application outputs;
final image closures alone do not retain the intermediate Rust compilation work.
Native server, WebAssembly and patient-bank builds keep separate dependency
artifacts in [`nix/packages.nix`](/nix/packages.nix). Cache reads preserve Nix's
signature verification and the upstream substituter. A cache miss or outage must
still run the build and all image proofs. Disabling `NIX_CACHE_ENABLED` restores
the uncached workflow without changing the proof commands.

The cache contains private build inputs and outputs. Do not make the bucket
public or copy credentials into the Nix store. Object-age deletion is not a safe
cache garbage collector: deleting a NAR while retaining its metadata or references
can break substitution. Retention changes must account for the reference graph.

### Local cache warming with act

[The local workflow](/.github/workflows/nix-cache-local.yml) builds the same
[allowlisted roots](/scripts/nix-cache/nix_cache_roots.py) as the hosted writer: three
dependency artifacts, six runtime, migration and job images, and the four dev
shells CI enters (`agent`, `postgres-ci`, `release`, `observability`). It only warms the
binary cache; it does not load or push images, deploy services, or replace the
hosted image proofs. Hosted publication remains a protected `main`-push operation.
Local publication is an explicit operator-authorized exception for a reviewed,
full commit SHA. Serialize it with every other writer: do not run it while a
hosted or another local publication is active. `act` does not participate in
GitHub's writer concurrency group.

Use a clean, standalone checkout containing the reviewed commit, not a linked
worktree: `act` copies files into the container, and a worktree's `.git` file points
outside that copy. A local `git clone --no-hardlinks /path/to/repository
/path/to/cache-warm-checkout` provides a standalone checkout without a GitHub token;
check out the reviewed SHA there. Commit the reviewed changes before running; do
not bypass cleanliness checks. The workflow verifies `HEAD` and tracked/untracked
changes before installation and credentials, and the publisher rechecks the
revision. Never pass unreviewed local code signing credentials.

From that checkout, enter `nix develop .#agent`. First check workflow selection and
syntax without credentials or a build:

```bash
EXPECTED_SHA="$(git rev-parse HEAD)"
act workflow_dispatch -W .github/workflows/nix-cache-local.yml -j nix-cache-local \
  --var ACT=true --input expected_sha="$EXPECTED_SHA" \
  --env-file /dev/null --secret-file /dev/null --var-file /dev/null --input-file /dev/null \
  --container-daemon-socket - --list
# Repeat the command with --dryrun instead of --list to validate step planning.
```

The explicit `-W` and `-j nix-cache-local` are required: [`.actrc`](/.actrc)
defaults to `-j ci`. Both the manual event and the local `ACT` checks are required;
a hosted manual dispatch cannot publish. The installer uses
`cachix/install-nix-action@v31` with `--no-daemon`: its single-user branch clears
`build-users-group` and exports the user profile, without systemd. The container
uses its own disposable `/nix`, with sandboxing disabled inside Docker. Do not add
host `/nix`, systemd, home, credential or Docker-socket mounts, `--bind`,
`--privileged`, or `--reuse`. The Docker daemon is needed by `act` on the host, not
inside the job. A cold build can need substantial time and disk space.

For an authorized write, first record the current `NIX_CACHE_ENABLED` repository
variable, set it to `false` with operator approval, and wait for active hosted
writers to finish. This pauses private reads as well as writes. Restore the exact
previous value after the local run, including failures. Do not assume a local lock
coordinates with GitHub's concurrency group.

The [launcher](/scripts/nix-cache/warm_nix_cache.py) fetches only seven named settings from
Doppler: bucket, public key, writer HMAC pair, signing key, and reader HMAC pair.
It passes names rather than values on `act` arguments; no unrelated Doppler
secrets are forwarded. The writer HMAC remains bucket-scoped and create-only.

```bash
# Inside nix develop .#agent, with a clean reviewed standalone checkout
# and hosted publishers paused/drained under operator authorization:
python scripts/nix-cache/warm_nix_cache.py \
  --expected-sha "$(git rev-parse HEAD)" \
  --doppler-config stg_nix_cache --acknowledge-exclusive-writer
```

Do not use broad Doppler export/forwarding, `.secrets` files, literal secret
arguments, shell tracing, `--verbose`, `--insecure-secrets`, or environment dumps.
Only publication receives writer/signing credentials; the subsequent fresh-reader
step receives only reader credentials and the public key. Publication first sends
a leaf-derivation smoke closure, then the full closure twice to prove safe reuse.
The reader copies the server-dependency closure into an empty disposable store
from the private cache alone, without builds, and verifies contents and trust.
All temporary credential files and the reader store are removed on normal failure
or completion; `act --rm` also removes the job container on failure. Abrupt host
termination can require removing that specific stopped act container/volume; do
not prune unrelated Docker resources.

The helper reports stages, elapsed heartbeats, exit codes, and fixed error categories
without printing raw SDK output. A heartbeat means the process is alive, not that
an object was uploaded. It seeds repeated NAR content before the parallel copy to
avoid create-only HEAD/PUT races within one publisher. This does not make multiple
publishers safe. Unknown errors remain withheld; reproduce with synthetic local
transport credentials rather than enabling verbose real-secret logs. Failures
are fatal locally. No cache credentials are needed for list, dry-run, checkout
verification, installation, or building from the upstream cache.

## CI timing reports

[`scripts/ci_timings.py`](/scripts/ci_timings.py) collects GitHub job and step
timings through `gh api` and renders inspectable graphs with raw data. The
`CI result` job retains a report artifact and links it from the step summary.
Reporting is best effort and does not replace the required result gate. Its live
snapshot excludes unfinished jobs; deployments occur after validation and must
not be counted as PR validation latency. Historical comparisons separate full
successful validation from lightweight, failed, cancelled and manual runs.

## Secrets: Doppler

The rule is in <a href="/AGENTS%2Emd#dev-shell"><code>AGENTS.md</code></a>.

- Doppler project `coda-rust` has the parent configs `dev`, `stg` and `prd`. Locally, log in once with `doppler login` and prefix a command with `doppler run --`; [`doppler.yaml`](/doppler.yaml) selects `dev`, and the shell loads it when a login exists and continues when it does not. Creating a config is an operator step, and this repository stores no token and no secret value.
- Physician and Duet services read per-product, per-role child configs, never a parent, because a parent-scoped token would carry the other product's secrets. [GCP deployment](gcp.md) lists those configs and how each image starts under `doppler run`.
- The public marketing site reads no secrets and runs without Doppler. CI installs the pinned CLI only to verify it and fetches no secrets through it; GitHub jobs reach GCP through Workload Identity Federation.
- `cargo run -p workspace-tools -- install-doppler OUTPUT_PATH` ([`doppler.rs`](/crates/workspace-tools/src/doppler.rs)) installs the pinned, checksum-verified CLI for images and runners.

## Retained physician infrastructure references

| Root | Owns | Owner document |
| --- | --- | --- |
| [`infra/gcp/physician-runtime`](/infra/gcp/physician-runtime) | The private GKE Autopilot cluster, Cloud SQL with IAM authentication, and the site and migration identities for `physician-sample` and `physician-auth` | <a href="/infra/gcp/physician-runtime/README%2Emd">README</a>, including its `tofu test` and `validate` commands |
| [`infra/gcp/foundation`](/infra/gcp/foundation) | Shared project foundation | [`main.tf`](/infra/gcp/foundation/main.tf) |
| [`infra/gcp/staging`](/infra/gcp/staging) | The hosted synthetic integration platform on Cloud Run | <a href="/infra/gcp/staging/README%2Emd">README</a> |
| [`infra/gcp/clinical-evidence`](/infra/gcp/clinical-evidence) | The production Clinical Evidence runbook | <a href="/infra/gcp/clinical-evidence/README%2Emd">README</a> |

This workspace does not run `init` against a backend and does not apply these roots. Apply authority stays with the product repositories and the operator of the target project. [GCP deployment](gcp.md) says what runs Duet production.

## Workspace-owned roots

| Root | Owns | Owner document |
| --- | --- | --- |
| [`infra/gcp/public-site`](/infra/gcp/public-site) | Public-site Cloud Run: staging behind IAP, production behind the load balancer without sign-in. This workspace owns both states and applies each only from a reviewed saved plan; production applies need Dev's explicit authorization. | [Public site](public-site.md#production-state-and-deployment) |
| [`infra/gcp/rust-environments`](/infra/gcp/rust-environments) | The isolated Rust Cloud Run environments for the physician and Duet services. It needs an administrator bootstrap before its first apply. | [GCP deployment](gcp.md#rust-staging-bootstrap-and-deployment-gate) |
| [`infra/gcp/shared-mail`](/infra/gcp/shared-mail) | Production mail only in `coda-health-prod`, with separate state and operator-approved saved-plan applies. | <a href="/infra/gcp/shared-mail/README%2Emd">Production shared mail</a> |
| [`infra/gcp/observability`](/infra/gcp/observability) | The shared collector deployment | [Observability](observability.md) |

## iPhone and TestFlight

- The Rust bindings for iPhone are in [`platform/ios`](/platform/ios) (see the [crate map](crate-map.md#platform-bindings)).

## Android and Google Play

- `nix develop .#android` ([`nix/android-shell.nix`](/nix/android-shell.nix)) provides the Android SDK, NDK, emulator image, JDK and a Rust toolchain with the Android standard library. Run the commands below inside it.
- `cargo run -p android -- build` ([`crates/android`](/crates/android/src/main.rs)) builds the debug APK for the `mobile` mode shell. `cargo run -p android -- avd` creates the `duet` AVD once, as CI does.
- `cargo run -p duet-stub` serves the host stand-in that the emulator calls. `cargo run -p android -- window` opens the emulator, and `cargo run -p android -- test coda.duet.ModeShellTest` builds the APK and runs UI Automator (see [`crates/android`](/crates/android/src/main.rs)).
- CI's `Android emulator` job runs `cargo run -p android -- test --headless coda.duet.ModeShellTest` on a headless emulator. For now it runs only when CI is dispatched by hand: Actions, CI, Run workflow, choosing the branch (or `gh workflow run ci.yml --ref <branch>`). Setting `MOBILE_ON_CHANGE` in [`scripts/ci_scope.py`](/scripts/ci_scope.py) makes pull requests and main pushes run it whenever a change touches the APK's build.
- A debug APK is not a Play upload.
