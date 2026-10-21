# Public site

`crates/public-site` is the Rust public marketing site. It is separate from the
physician runtime and Duet: no database, physician accounts, model execution, or
clinical types are introduced by the site. `PHYSICIAN_APP_URL` is a navigation
link, not a backend connection. `submodules/kytos` is an optional read-only
developer reference, never a build, test, CI, or runtime dependency.

[PR #36](https://github.com/Coda-Health/coda-rust/pull/36) merged this implementation
into `main` as `ee8ae88c9675bd008ff91e45dc799c88f13d5f24`. This is implementation
and deployment preparation, **not a production release**. Vercel remains the
existing hosting boundary until an operator approves cutover. The deployment root
has not been applied; no DNS, Cloudflare policy, certificate, secret value, or
production service is changed by validation. [ENG-79](https://linear.app/coda-health/issue/ENG-79)
remains open for the hosting, approval, recovery-window and retirement work.

## Application contract

The application design is Dioxus route components prerendered at startup, served
by a stateless axum binary. The route inventory includes `/`, `/duet`, current and
archived Duet privacy/terms, `/duet/consumer-health-data`, `/privacy`, `/terms`,
`/clinical-transparency`, and `/scheduling/dev`. Retired `/physicians` and
`/api/physician-waitlist` remain 404. Pages, asset parity, compression, conditional
requests, Access verification, and motion integration have separate crate tests;
passing infrastructure tests does not establish any of those behaviors.

Startup builds the page/asset tables and their identity, gzip and Brotli
representations once; requests do not render Dioxus or query a database. The server
serves content-addressed immutable assets and HTML with bounded public/shared-cache
lifetimes, encoding-specific ETags, conditional requests and bodyless HEAD responses.
A deferred `web-sys` WASM island adds motion, not full-page Dioxus hydration. Static
painted art remains visible until the first motion frame is drawn synchronously
and readiness is published. Blocked/failed WASM and reduced motion retain a usable
static page; live motion-preference transitions are covered by browser tests.

Staging applies its origin Access gate to pages, assets, redirects, health routes
and errors. Responses force `private, no-store, max-age=0`, CDN-specific no-store
headers, `Vary: *` and `X-Robots-Tag: noindex, nofollow`. The gate accepts a verified
RS256 Access JWT (issuer, audience, expiry and bounded JWKS refresh) or the explicitly
configured service-token pair. Missing credentials deny access; malformed issuer
configuration aborts startup. An optional separate loopback-only probe listener
serves no site content. Local tests prove this application boundary, not a deployed
Cloudflare/ALB configuration; repeat direct-origin and cache-bypass proofs before
any deployment.

Runtime configuration:

- `PUBLIC_SITE_LISTEN_ADDR`: local default `127.0.0.1:8791`; the image and Cloud
  Run root explicitly set `0.0.0.0:8080`.
- `PHYSICIAN_APP_URL`: local default `http://127.0.0.1:8790`; deployed values must
  use HTTPS and point to the separately operated physician application.
- `APP_ENV`: deployment root permits only `staging` or `production`; staging
  selects the origin Access gate. Do not use a misspelled environment to disable it.
- `RELEASE_SHA`: full Git revision, supplied at runtime for the health response.
- `CLOUDFLARE_ACCESS_TEAM_DOMAIN` and `CLOUDFLARE_ACCESS_AUD`: staging JWT issuer
  and audience. `CF_ACCESS_CLIENT_ID` plus `CF_ACCESS_CLIENT_SECRET` optionally
  authorize the configured service-token path; neither value belongs in source.
- `STAGING_HEALTH_TOKEN`: when configured, `/api/health` requires it in addition
  to staging Access; it is not an Access bypass.
- `PUBLIC_SITE_MAX_CONCURRENCY`: default 1024, accepted range 1–65536; excess
  concurrent requests receive 503 with `Retry-After: 1`.
- `PUBLIC_SITE_REQUEST_TIMEOUT_MS`: default 5000, accepted range 100–30000.
- `PUBLIC_SITE_INTERNAL_PROBE_ADDR`: optional, loopback-only probe address.

Local startup (from the workspace root):

```bash
nix develop --command bash crates/public-site/island/build.sh
nix develop --command cargo run -p public-site --bin public-site
```

## Merged implementation verification

The final repair commits were `153e689` (motion readiness, browser/Access proofs
and database-client dependency alignment) and `59695be` (integration with current
main and the SeaORM 2 migration-proof API). All three jobs in the
[final PR CI run](https://github.com/Coda-Health/coda-rust/actions/runs/39521784174)
passed: main CI, fresh PostgreSQL migrations/tests and cargo-deny. This is evidence
for the reviewed PR head `59695be6d7f1fb14b40de022ffadbd827638a63f`, not a separate
post-merge deployment or capacity proof.

Recorded final-tree local proof:

- Workspace tests and all-targets Clippy passed. Optional encoder model variables
  were unset and the live React practice comparison was explicitly excluded, as
  in CI; neither optional comparison is claimed as passed.
- All 35 public-site tests passed, including four Access tests, original-source
  parity and Chromium desktop/mobile, reduced-motion, delayed-initialization,
  motion-preference transitions and blocked-WASM coverage. Three further browser
  suite repetitions passed after the repair.
- The rebuilt motion island totaled **51,446 Brotli bytes / 60,000-byte budget**.
- The real disposable PostgreSQL proof passed **18 checks**: one fresh isolated
  and shared migration-chain/idempotence test, seven physician-auth tests, eight
  front-door tests and two physician-sample front-door tests. Upstream main fixes
  resolved the earlier pre-integration database failures; there is no outstanding
  database blocker from those runs. See [database-client versions and proof](backend-architecture.md#rust-database-client-versions-and-proof).
- The rebuilt production/staging image passed nonroot/read-only HTTP and shutdown
  smoke checks, as detailed below. It was not published or deployed.
- cargo-deny passed under inherited main policy. ZIP 8.6.0 is **not remediated**:
  the unchanged exceptions for RUSTSEC-2026-0367/0368/0369 remain tracked in
  [ENG-108](https://linear.app/coda-health/issue/ENG-108).

Reproduce the site checks from the workspace root, inside the dev shell:

```bash
nix develop --command bash crates/public-site/island/build.sh
nix develop --command cargo test --locked -p public-site
nix develop --command cargo test --locked -p public-site --test site -- --nocapture
nix develop --command node crates/public-site/tests/import-snapshots.mjs --check
nix develop --command mdbook build book
```

Use the shell's Chromium (or set `CHROMIUM` to the approved browser executable).
The default startup uses port 8791; choose another `PUBLIC_SITE_LISTEN_ADDR` if a
preview already owns it rather than stopping that preview.

## Self-contained CI and original-source snapshots

Ordinary builds, tests, and CI do not fetch or access the old private repository.
The primary checkout explicitly disables submodules and credential persistence.
No GitHub App, private-reference credential, Doppler CI config, or service token is
needed. Fork and Dependabot changes run the same parity tests without application
secrets; the workflow uses `pull_request`, not `pull_request_target`. Existing
runtime/deployment secrets remain managed by Doppler; this removes no deployment
protection and copies no `dev`, `stg`, or `prd` secrets into CI.

`crates/public-site/tests/fixtures/original/` contains 17 narrowly selected
original-source snapshots from `Coda-Health/kytos` commit
`5012bb749e29104e00ccb208a14a810f93fa420f`. `provenance.json` records the source
path, original SHA-256, snapshot SHA-256, and extraction policy for each input.
HTML, page/legal TypeScript, archive JSON, notices, CSS, and the scheduling social
image definition are verbatim source files. Only the deterministic painter is
sliced: the exact bytes from `function rng` through the end of `paint`, excluding
`var artCache` and unrelated browser interaction code. No Next application,
package dependencies, private credentials, or rendered Rust output are imported.

These are test oracles, separate from generated application `data/`, CSS, and
assets. The normal parity suite verifies snapshot integrity, compares homepage
and page copy, evaluates the original legal definitions and archives, compares
CSS, and evaluates the original JavaScript painter against Rust for all eight
palette/seed pairs. Missing or changed fixtures fail; there is no submodule
fallback or parity skip. Hashes detect accidental fixture edits, not malicious
co-edits to the provenance manifest: source updates require manual review.

### Deliberate reference updates (optional developer operation)

The `submodules/kytos` gitlink remains pinned and read-only. Only developers who
choose to inspect or update the historical source need repository access:

```bash
nix develop --command git submodule update --init -- submodules/kytos
nix develop --command node crates/public-site/tests/import-snapshots.mjs --import
nix develop --command node crates/public-site/tests/import-snapshots.mjs --check
```

The importer reads Git blobs at the explicit reviewed commit, checks that the
optional checkout is clean and pinned, and writes only allowlisted fixture paths.
It is never invoked with `--import` by builds or tests. To adopt a new publication,
review the original changes first, explicitly update the importer's commit and
optional gitlink, then import and review the fixture/provenance diff. Do not
regenerate expected fixtures from Rust output or copy generated `data/` back into
them to make tests pass. Legal publication changes require their normal review.
Application generation (`data/import-reference.mjs` and `tests/generate-social.mjs`)
reads these frozen originals and is a separate, explicit operation. Normal tests
only use `--check`, so they also work in source exports without Git metadata or
initialized submodules.

## Image

`crates/public-site/Dockerfile` builds a static `x86_64-unknown-linux-musl` binary
and copies it into distroless static Debian 12 as UID/GID 65532. Builder/runtime
images are digest-pinned, Cargo uses `--locked`, and no mutable OS package
installation occurs during the build. Only workspace manifests/crates are copied,
not reference submodules, local credentials, or Terraform state. The Doppler CLI
is pinned to the same version and checksum as `scripts/gcp/install_doppler.sh`.
Both executables are checked for a dynamic ELF interpreter during the build.
Pinned inputs improve repeatability; bit-for-bit image reproducibility is not
claimed without a repeated-build comparison.

```bash
nix develop --command bash crates/public-site/island/build.sh
nix develop --command docker build --platform=linux/amd64 \
  -f crates/public-site/Dockerfile -t public-site:local .
nix develop --command docker run --rm -p 127.0.0.1:8791:8080 \
  public-site:local
```

The image defaults to `/public-site`, so local smoke tests need no Doppler account.
Cloud Run overrides its command to `/doppler run ... -- /public-site`. Secrets
remain in Doppler project `coda-rust`, config `stg` or `prd`. Secret Manager holds
only a read-only, environment-scoped `DOPPLER_TOKEN`, with a numeric version
pinned by the deployment root. The dedicated service account has access only to
that named secret, not database or physician resources. No secret value is read
by OpenTofu or placed in its variables/state. Doppler fallback files are disabled;
the four deployment-controlled environment values above are explicitly preserved
so Doppler cannot accidentally change the staging gate or listening address.

**Motion integration gate:** the Dockerfile builds the server and requires the
three generated files in `island/dist` before Cargo runs. First execute the motion
owner's `island/build.sh` in the pinned Nix shell; it builds the nested WASM crate,
runs `wasm-bindgen`, and checks size. The image does not independently rebuild the
island. This is a two-stage workflow, not a standalone Docker-only source build.
The integrated server embeds `bootstrap.js`, `public_site_motion.js`, and
`public_site_motion_bg.wasm` under one shared hashed directory preserving sibling
URLs. The local image HTTP smoke below verifies those served URLs and WASM magic
bytes. It does not execute browser animation, establish generated-asset freshness,
or replace the motion budget and visual/browser proofs. Rebuild the island after
motion source changes and repeat the image proof after final integration changes.
No host tooling or flake changes are introduced by the deployment artifacts.

## Prepared Cloud Run and load balancer

`infra/gcp/public-site` is a new, isolated OpenTofu root with **no backend**. It
prepares a separate Cloud Run service, dedicated service account, a serverless
NEG, global external managed HTTPS load balancer, modern TLS policy, and an
existing-certificate attachment. It creates no DNS records, certificate issuance,
project/API enablement, database, or physician runtime dependencies. Defaults are
min 0 / max 10 instances, concurrency 1000, one vCPU, and 512 MiB; these are bounded
configuration defaults, not demonstrated load capacity. Startup uses a TCP probe,
so staging does not require bypassing Access for a probe page.

Cloud Run ingress is `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER` and the default
`run.app` URI is disabled. The external ALB needs the `allUsers` invoker grant
because it does not authenticate to Cloud Run; that grant does **not** replace
network restrictions or staging's application-level Access verification. No Cloud
Run custom-domain mapping is created. Access verification still applies to
internal requests and requests addressed directly to the load balancer IP.

### Cloud CDN support and staging safety

Google explicitly documents Cloud CDN support for Cloud Run/serverless NEGs on a
**global external Application Load Balancer** in
[Enable Cloud CDN](https://cloud.google.com/load-balancing/docs/https/setup-global-ext-https-serverless#enable_cloud_cdn).
The [serverless NEG limitations](https://cloud.google.com/load-balancing/docs/negs/serverless-neg-concepts#limitations)
exclude backend health checks and configurable backend timeouts, not this CDN
combination. Those unsupported fields are intentionally omitted. IAP and Cloud
CDN cannot be enabled together on one backend; this root does not configure IAP.
Documentation was checked on 2026-10-20 rather than assuming the proposed topology
was supported.

- **Production:** Cloud CDN is enabled with `USE_ORIGIN_HEADERS`, no forced
  caching and no negative caching. The server emits the cache headers described
  above; the infrastructure does not manufacture application cacheability.
- **Staging:** Cloud CDN is unconditionally disabled, including for assets. A
  public cache hit would otherwise bypass origin JWT verification. There is no
  variable that can enable staging CDN.
- **Cloudflare:** Access application/policy, hostname routing, and any upstream
  Cloudflare cache-bypass rules are operator prerequisites, not resources in this
  root. Staging must not cache protected responses at *any* upstream layer. A
  Cloudflare login policy alone is insufficient: origin JWT signature, issuer,
  audience and expiration verification must reject missing/forged credentials,
  including requests to the ALB IP. The provider URL must remain unreachable.

Do not deploy staging until negative tests prove both direct-origin rejection and
absence of cached authenticated content for an unauthenticated request.

## Validation, not deployment

Only this new, backend-free root may be initialized for schema/plugin validation.
Do not initialize existing GCP roots, add a backend, run a real plan, or apply as
part of this work. The tests use a mocked Google provider; no ADC credentials or
live GCP API calls are needed.

```bash
nix develop --command tofu -chdir=infra/gcp/public-site init -backend=false -input=false
nix develop --command tofu -chdir=infra/gcp/public-site fmt -check -recursive
nix develop --command tofu -chdir=infra/gcp/public-site validate
nix develop --command tofu -chdir=infra/gcp/public-site test
```

Preparation checks on 2026-10-20: `tofu validate`, all seven mocked test runs,
`tofu fmt -check`, and `mdbook build book` passed. After the manifests/lockfile and
motion embedding were integrated, the pinned image build and local HTTP smoke
also passed. Docker's default build network had a local crates.io DNS failure;
`--network=host` was used for the successful build (not required on working bridge
networks). No host tools were installed and no secret-bearing build arguments or
container environment values were supplied.

```bash
nix develop --command docker build --network=host --platform=linux/amd64 \
  -f crates/public-site/Dockerfile -t public-site:deployment-check .
nix develop --command python infra/gcp/public-site/smoke.py public-site:deployment-check
```

The final-lockfile repair build recorded in
[the PR verification](https://github.com/Coda-Health/coda-rust/pull/36#issuecomment-6304841069)
has local image ID
`sha256:df89fead5baa71d16ee496f4dc2b04d1fbb066bc2d29aaa69e225d45fd7434e2`
(24,870,060 bytes). This is a local Docker image/config digest, **not** a published
registry manifest digest suitable for the deployment variable. The image was not
pushed. It supersedes the earlier 24,865,964-byte image as repair-build evidence;
it is not a claim of a separately rebuilt merge-commit image. Production/staging
SIGTERM exits were 0 in 0.22/0.23 seconds. Build and smoke the exact release tree
again before publishing a release image.

`smoke.py` starts disposable, nonroot, read-only containers with all capabilities
dropped, no-new-privileges, one CPU, 512 MiB memory, and random host-loopback ports.
It checks the image entrypoint/user/listen configuration and restricts image
environment names to `PATH`, `SSL_CERT_FILE`, and `PUBLIC_SITE_LISTEN_ADDR`.
The successful repair-build run demonstrated:

- Homepage HTTP 200, nonempty title/canonical/description, main and footer
  content, and the explicit physician sign-in link.
- Five hashed CSS files returning CSS and immutable caching; bootstrap JS,
  generated JS, and WASM returning the correct content types with sibling paths
  intact and valid WASM magic bytes.
- `/healthz`, `/readyz`, and `/api/health` returning 200 with the expected runtime
  environment/release, unknown paths returning 404, and the container listening
  on reachable port 8080.
- Credential-free staging denying pages, CSS, JS/WASM, health, and unknown paths
  with `no-store` and `noindex`; no authenticated staging positive path is claimed.
- SIGTERM exit code 0 for both environments, with the timings above. Both
  containers were removed after the run.

This HTTP proof does not replace browser animation/parity tests, a deep secret
scan of embedded source assets, a load test, cloud ingress verification, or an
actual deployment. No model calls, secret fetches, or live GCP requests occur in
the smoke script.

The provider is version-pinned and its generated `.terraform.lock.hcl` belongs in
version control. Seven mocked test runs cover staging ingress/default-URI/CDN and
environment safety, production origin-respecting CDN, scaling defaults, immutable
image references, invalid environments/concurrency/scaling, and pinned secret
versions. These checks validate resource contracts, not cloud permissions,
certificate readiness, Access JWT code, regional quotas, autoscaling behavior,
application health, or a real deployment.

Before deployment, an operator must approve state ownership/backend, project/API
and IAM prerequisites, an image digest from the final tested build, certificate
and hostname choices, scoped Doppler token provisioning, Access configuration and
cache bypass, release/load/browser evidence, and a rollback plan. DNS/Vercel
cutover and any production deploy remain separate decisions. No apply command is
provided here deliberately.

## Local release load check (2026-10-20)

The approved `oha` package is included in the Nix shell (resolved version 1.16.0).
`crates/public-site/load_check.py` starts only disposable loopback listeners, with
a clean child environment containing the tool PATH, `APP_ENV`, and the local
listen address. It neither targets arbitrary URLs nor starts model/data services.
Production cache behavior is measured locally; no production traffic, cloud
resource, staging policy, or deployed service is changed.

Reproduce from this worktree root (build the motion island first if absent):

```bash
nix develop --command env -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER \
  -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
  cargo build --locked --release -p public-site --bin public-site
nix develop --command python crates/public-site/load_check.py
```

The script discovers an actual hashed CSS URL in the homepage and runs each
endpoint sequentially with `oha --no-tui --output-format json
--stats-success-breakdown --http-version 1.1 --disable-compression
-H 'Accept-Encoding: gzip' -c 1000 -z 30s -w -t 10s`. Here
`--disable-compression` disables oha's automatic compression negotiation/decoding;
the explicit header still requests the server's precompressed gzip body. Both
wire/body byte accounting and gzip decompression equality were verified. No TLS,
CDN, browser execution, conditional request, or Brotli measurement is included.
`-w` drains in-flight requests after the 30-second issuing window instead of
counting deadline cancellations as transport failures. Keep-alive is enabled.

Server settings were **unchanged defaults**: 1024 concurrent request permits,
5-second request timeout, and default Tokio workers (33 total server threads).
No concurrency override or tuning comparison was used. All responses in both
runs were HTTP 200; **zero 503s, other non-200 statuses, or transport errors**.
This workload therefore did not trigger shedding; it does not prove behavior
above the configured request-permit limit.

First run, started 19:24:15 UTC:

- `/`: 12,984,857 HTTP 200 responses in 30.011799730 s;
  **432,658.391593 requests/s**; p50 **1.697316 ms**, p95 **5.848630 ms**,
  p99 **9.650737 ms**; 136,548,756,212 response-body bytes.
- Hashed `chrome.css`: 13,380,925 HTTP 200 responses in 30.006888126 s;
  **445,928.446289 requests/s**; p50 **1.750734 ms**, p95 **5.621291 ms**,
  p99 **8.780633 ms**; 30,147,224,025 response-body bytes.

Repeat with the final harness, started 19:26:12 UTC:

- `/`: 7,999,956 HTTP 200 responses in 30.025782652 s;
  **266,436.218923 requests/s**; p50 **2.682625 ms**, p95 **9.786101 ms**,
  p99 **16.845047 ms**; 84,127,537,296 response-body bytes.
- Hashed `chrome.css`: 13,513,247 HTTP 200 responses in 30.010392561 s;
  **450,285.579322 requests/s**; p50 **1.601855 ms**, p95 **5.779617 ms**,
  p99 **10.562002 ms**; 30,445,345,491 response-body bytes.

The proposed single-digit-ms p99 is a target, **not reliably met by these runs**.
Do not discard the slower repeat or extrapolate this workstation result to the
prepared one-vCPU Cloud Run configuration. Home maximum latency was 1050.270 ms
(first run) and 1077.798 ms (repeat), including initial connection establishment.
These are closed-loop fixed-concurrency results, not an independently paced
arrival workload or a coordinated-omission-corrected latency guarantee.

Payload verification: homepage identity/gzip bodies were 44,660/10,516 bytes;
CSS identity/gzip bodies were 6,485/2,253 bytes. Total bytes equal HTTP 200 count
times compressed body length, not error-body throughput. Identity and decoded
gzip bodies matched before load; compressed bodies were unchanged after load.
The asset was
`/a/5fba9b54258486c75142c225491dcf4ed49a0d0fa86174459dd200516b375a2d/css/chrome.css`.
HTML returned `public, max-age=300, s-maxage=3600,
stale-while-revalidate=86400`; CSS returned
`public, max-age=31536000, immutable`. Both varied on `Accept-Encoding`.

Machine/resource context: NixOS Linux 7.2.6 x86_64, AMD Ryzen 9 9950X3D
(16 cores/32 logical CPUs), 63,369,700 KiB RAM, full 32-CPU affinity, and
524,288 soft/hard file descriptors. The Cursor cgroup reported unlimited CPU
and memory (`max 100000` / `max`), not an isolated dedicated benchmark machine.
Client (oha defaults to 16 workers) and server shared the host with other work.
One-minute system load was 27.95 before/55.56 after run one and 43.24 before/63.47
after the repeat; available RAM was approximately 8.9/8.4 GiB at those starts.
Server/client CPU seconds were 203.30/180.55 (home) and 172.50/149.77 (CSS) in
run one, then 136.85/125.25 and 167.60/148.04 in the repeat. Server RSS after
home/CSS was 105,076/119,360 KiB, then 116,456/132,560 KiB. These are snapshots,
not sampled peak-memory measurements. Shared-host/client contention prevents
attributing all latency variation to the server.

The release executable was built with Rust 1.98.1, Cargo 1.98.0, default Cargo
release optimization and SHA-256
`5bf4fc5baeab05b28921fe0411dacb2b958d20706aedea43b5e59825244e6580`.
Raw JSON, headers, commands, payload hashes and resource metadata remain locally
under `target/public-site-load/` and `target/public-site-load-repeat/` (not source
artifacts). Use `--output` to retain another run separately.

The harness also started a clean credential-free staging child and verified
403 plus `no-store` and `noindex` for `/`, the hashed CSS, `/api/health`, and a
missing path. Thus the benchmark's local production configuration did not weaken
the staging gate; no authenticated staging or upstream/cloud behavior is claimed.
Both child servers exited cleanly with code 0 after SIGTERM in each run, and all
oha processes exited. Nix-shell activation, Python compilation, the final harness,
`cargo fmt --all --check`, and `git diff --check` passed.
