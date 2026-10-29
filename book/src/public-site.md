# Public site

`crates/public-site` is the Rust public marketing site. It is separate from the
physician runtime and Duet: no database, physician accounts, model execution, or
clinical types are introduced by the site. The same-origin physician entry point
is `/physician/`; `PHYSICIAN_APP_URL` identifies the separately operated upstream, not
an external navigation destination. The site is a transport-only reverse proxy:
it strips `/physician` and forwards requests, while physician authentication, clinical
authorization, effects and persistence remain owned by the physician service.
Sign-in exists only where `PHYSICIAN_APP_URL` is configured: that one setting mounts
`/physician/` and renders the nav's Sign in link. Without it the site offers no
sign-in and `/physician/` is an ordinary 404.
`submodules/kytos` is an optional read-only developer reference, never a build,
test, CI, or runtime dependency.

### Same-origin physician proxy qualification

The `/physician/` proxy is deployed on staging. On October 24, 2026, an approved
email to Dev's mailbox completed the actual public-origin sign-in journey:
request, mailbox delivery, email-button click, automatic Today, reload, sign-out,
and used-link rejection. Both IAP boundaries remained enabled. This establishes
the exercised browser and service-account path; it does not establish every
user's IAP access or complete P1/D1 acceptance.

The local physician browser tests run through the actual public-site proxy to a
separate loopback physician server. They cover prefixed assets/API calls, the
mailed HTML button, sign-in, session restoration, sign-out/token revocation,
account revocation and finite-stream reconnection. The explicit `?api=` override
check uses a separate recorded fixture. Local tests and live staging evidence
establish different parts of the flow.
The same relative entry point is intended for both `staging.codahealth.ai` and
`codahealth.ai`; this does not authorize production deployment or DNS changes.
Production configures no physician upstream, so it shows no Sign in and has no
`/physician/` mount. Configuring one there requires qualified production physician
authentication first; production must not expose unconfigured proof-token
delivery. Launching the marketing site itself requires an explicit hosting
approval, not physician qualification.

The proxy must preserve physician bearer authorization, query strings and streamed
responses without forwarding browser cookies, caller-supplied service credentials
or IAP identity assertions to the upstream. Upstream IAP authentication is a
separate service-account credential in `Proxy-Authorization`, not a replacement
for the physician `Authorization` header. The public origin's verified IAP gate
remains mandatory for all staging proxy requests. Missing service credentials or
failed refreshes must fail closed; neither IAP boundary may be disabled to make
the proxy work.

The intended keyless credential flow is IAM Credentials `signJwt`, using the
public-site runtime identity, a short-lived JWT scoped to the configured physician
origin, and a self-scoped signing permission. The physician service's own
infrastructure owner must grant that runtime identity IAP access to **only** the
physician service. Public-site infrastructure must not compete with that owner's
authoritative IAM binding. No service-account private key or token belongs in
source, URLs, browser state or logs.

Before a staging release, an authorized operator must enable/verify the IAM
Credentials API and provision the `publicSitePhysicianProxySigner` custom role
(`iam.serviceAccounts.signJwt` only), bound runtime-to-itself on
`public-site-staging@coda-health-staging.iam.gserviceaccount.com`. This permission
is identity-signing authority, **not** an IAM restriction to one audience. Its
approval must account for the runtime identity's existing privileges. The
optional `--physician-proxy` administrator bootstrap flag supplies that grant,
but the full bootstrap also repeats other provisioning steps; it must not be run
as an unreviewed shortcut. Separately, the physician infrastructure operator must
apply the staging `physician_proxy_invokers` allowlist through the existing
authoritative physician IAP binding, retaining the engineering group and leaving
Duet unchanged. These grants have not been applied by this change. Release must
wait for them; application unit tests cannot prove deployed IAM access.

`/physician/` is not a browser security boundary: marketing scripts on the same origin
can access the physician session stored in session storage. The entire origin
must therefore be treated as trusted physician-session code. Proxy responses
must never enter public or CDN caches, including in production. Browser proof
must cover prefix-relative assets and API requests, sign-in, session restoration,
sign-out and streamed events through the proxy, not just direct-backend access.
Cloud Run streams remain finite; client reconnection and revocation must be
qualified rather than treating a longer request timeout as an indefinite stream.

[PR #36](https://github.com/Coda-Health/coda-rust/pull/36) merged this implementation
into `main` as `ee8ae88c9675bd008ff91e45dc799c88f13d5f24`. This is implementation
and deployment preparation, **not a production release**. Production remains on
Vercel. The staging root and hostname cutover are applied, and authenticated
browser access checks passed, as described in
[Staging deployment status](#staging-deployment-status-2026-10-21). Production
resources and DNS are unchanged. Production still requires visual qualification,
release-workflow integration, hosting approval and recovery/retirement work. The old public-staging
Vercel project and Cloudflare Access application have been retired.

Staging access uses Google Identity-Aware Proxy (IAP) directly on Cloud Run, in
the existing `coda-health-staging` project (number `134570912572`). The earlier
Cloudflare Access design was removed: there is no Cloudflare application, service
token, staging load balancer or staging CDN. This reuses the direct Cloud Run IAP
pattern of the [Rust staging environments](gcp.md#rust-staging-bootstrap-and-deployment-gate).

## Application contract

The application design is Dioxus route components prerendered at startup, served
by a stateless axum binary. The route inventory includes `/`, `/duet`, current and
archived Duet privacy/terms, `/duet/consumer-health-data`, `/privacy`, `/terms`,
`/clinical-transparency`, and `/scheduling/dev`. The sitemap lists only the eight
current documents, as production's does; archived versions stay reachable from
their policy pages, and `/scheduling/dev` is noindex. Retired `/physicians` and
`/api/physician-waitlist` remain 404. Pages, asset parity, compression, conditional
requests, IAP assertion verification, and motion integration have separate crate tests;
passing infrastructure tests does not establish any of those behaviors.

Startup builds the page/asset tables and their identity, gzip and Brotli
representations once; requests do not render Dioxus or query a database. The server
serves content-addressed immutable assets and HTML with bounded public/shared-cache
lifetimes, encoding-specific ETags, conditional requests and bodyless HEAD responses.
Motion is the original kytos `public/homepage/site.js`, served byte-for-byte as
`/homepage/site.js` on `/` and `/duet`, exactly as the Next site loads it. It also
paints the landscapes. Parity therefore holds by construction: `tests/parity.rs`
fails if the shipped script, `site.css` or `chrome.css` differ from the reviewed
snapshots. Site-owned additions are layout or fallback only, never motion:
`stage.css` reserves and scales each product frame before the script runs (it then
sets the same values inline), `noscript.css` reveals content when scripts are
disabled, and `chrome.js` floats the nav after 8px of scroll, as kytos' SiteNav
does. Reduced motion is decided by the original script at load, as on production.

The one deliberate exception is the Duet story (`src/pages/phones.rs`), which runs
ahead of production and of the app. It replaces the original Duet phones on `/` and
`/duet` with one story across three phones, drawn from the October 21 Duet mockups
in which Duet leads and Maya says yes: Today shows what Duet
noticed about her home blood pressure; Duet offers a morning Check-in and she accepts
it; on Monday the Check-in returns by itself and Duet prepares an update to her page
for her doctor; she reviews it, saves it and previews it, and the story ends on the
page her clinician receives (the same constants render the preview and that page).
Each screen is laid out at the mockups' native width and scaled into the phone. It reuses
`site.css` phone hardware and the stage scaling and landscapes `site.js` applies to
every frame, but its classes (`duet-story`, `ds-*`) are its own, so the original Duet
loop never binds to it. Its styles are `css/duet.css`, every selector scoped to
`.duet-story`; its motion is `homepage/duet.js`, which plays only while the stage is
on screen and, on narrow stages, brings the phone doing the work to the front. The
markup is the story's last frame, which reduced-motion and no-script visitors see.
The region is marked `data-owner="site"`: the parity suite compares the rest of the
page with production and checks that production serves the original phones
(`duet-phones.html`) where the story stands.

An earlier `web-sys` WASM island re-implemented this choreography on a global
50 ms timer. Frame sampling against production showed the drift: a stylesheet rule
overrode stage children's transitions (the hero pointer and popups jumped), the
memory chart advanced at 20 fps, loops visibly rewound, and loop lengths differed.
It was replaced by the original script; the browser proof now fails if the hero
pointer jumps instead of gliding.

The same rule covers everything else the original computes in a build or runtime
step: the port consumes the output instead of re-implementing the step. Pages
belong to one of the original's two systems (`Route::shell`). `/` and `/duet` use
the homepage system: `chrome.css`, `site.css` and the motion script (plus the
site-owned Duet story). Every other
page uses the utility system: the original layout's shell markup and
production's compiled Tailwind stylesheet, served verbatim as `css/utility.css`
(it already contains `chrome.css`). The `/scheduling/dev` social image is the PNG
production's `ImageResponse` rendered. Head metadata and icons follow the
original's `lib/metadata.ts` and root layout. URLs follow Next's default rule: a
trailing slash is removed with a 308 that keeps the query. Earlier, hand-written
equivalents (a Tailwind subset, a Chromium re-render of the social image, and a
regex evaluation of the legal TypeScript that deleted the word "export" from
three policy sentences) drifted from production without any test noticing,
because the tests re-derived their expectations the same way.

Staging applies its origin IAP gate to pages, assets, redirects, health routes
and errors. Responses force `private, no-store, max-age=0`, CDN-specific no-store
headers, `Vary: *` and `X-Robots-Tag: noindex, nofollow`. IAP authenticates the
user in front of Cloud Run. The origin still verifies the signed
`x-goog-iap-jwt-assertion` header on every request. It does not trust the
signature-free `x-goog-authenticated-user-*` headers, and it does not let IAP's
presence stand in for verification:

- Exactly one assertion header, at most 16 KiB, ES256 only, with a key ID.
- Signature checked against Google's fixed JWK set
  (`https://www.gstatic.com/iap/verify/public_key-jwk`). The key must be P-256
  and must not be marked for another algorithm, use or operation. A key ID that
  matches more than one key is rejected.
- Issuer `https://cloud.google.com/iap` and the exact audience
  `/projects/NUMBER/locations/REGION/services/NAME`. That is the direct Cloud Run
  signed-header audience, not an OAuth client ID or URL.
- `exp`, `iat` and an optional numeric `nbf` checked with one clock reading and
  30 seconds of skew. The lifetime may not exceed 660 seconds.
- Key retrieval is HTTPS-only with no redirects, 2 s to connect, 3 s in total,
  a 256 KiB body limit and 1–32 keys. One fetch runs at a time and the cache lasts
  5 minutes. An unknown key ID triggers at most one refresh every 30 seconds.
  When a refresh fails, cached keys are not used, so the request is denied.

Missing or invalid assertions return 403. A staging process with a missing or
malformed audience refuses to start. The old Cloudflare variables, headers and
service-token bypass were removed and do not authorize anything. An optional
separate loopback-only probe listener serves no site content. Local tests prove
this application boundary with a public test-only key. They do not prove a
deployed IAP configuration; member, nonmember and direct-origin checks remain
deployment qualification steps.

Runtime configuration:

- `PUBLIC_SITE_LISTEN_ADDR`: local default `127.0.0.1:8791`; the image and Cloud
  Run root explicitly set `0.0.0.0:8080`.
- `PHYSICIAN_APP_URL`: optional fixed upstream origin for `/physician/`, not the
  browser navigation link. When set, the site mounts `/physician/` and the nav offers
  Sign in; when unset, neither exists. There is no default: a local run against the
  sample physician server sets `http://127.0.0.1:8790`. Deployed values must use
  HTTPS; credentials, non-root paths, queries and fragments are rejected.
- `PHYSICIAN_PROXY_SERVICE_ACCOUNT`: service identity used for keyless upstream
  IAP authentication. Required in staging whenever `PHYSICIAN_APP_URL` is set; its
  signing permission and physician IAP access must be provisioned before release.
  No browser credential supplies this value.
- `PUBLIC_SITE_ORIGIN`: optional exact browser origin for the physician proxy's
  unsafe-method Origin validation. Defaults to `https://staging.codahealth.ai` in staging,
  `https://codahealth.ai` in production, and the local listener origin otherwise.
  Unspecified local bind addresses (`0.0.0.0` or `::`) map to the corresponding
  loopback browser host. Alternate hosts or mapped container ports must set this
  origin explicitly so unsafe-method Origin checks match the browser.
- `APP_ENV`: deployment root permits only `staging` or `production`; staging
  selects the origin IAP gate. Do not use a misspelled environment to disable it.
- `RELEASE_SHA`: full Git revision, supplied at runtime for the health response.
- `PUBLIC_SITE_IAP_AUDIENCE`: required for staging. The deployment root sets it
  from the project number, region and service name, and it is not a secret. For
  staging it is
  `/projects/134570912572/locations/us-east1/services/public-site-staging`.
  Production and local runs do not read it.
- `STAGING_HEALTH_TOKEN`: when configured, `/api/health` requires it in addition
  to a valid IAP assertion; it is not an IAP bypass.
- `PUBLIC_SITE_MAX_CONCURRENCY`: default 1024, accepted range 1–65536; excess
  concurrent requests receive 503 with `Retry-After: 1`.
- `PUBLIC_SITE_REQUEST_TIMEOUT_MS`: static-site response deadline; default 5000,
  accepted range 100–30000. `/physician/` requests have one 60-second deadline
  through authentication, upstream startup and response headers. The proxy
  does not impose a second deadline, and streaming response bodies are not
  capped by this header deadline.
- `PUBLIC_SITE_INTERNAL_PROBE_ADDR`: optional, loopback-only probe address.

Local startup (from the workspace root):

```bash
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
- All 35 public-site tests passed, including four Cloudflare Access tests (since
  replaced by the IAP tests below), original-source
  parity and Chromium desktop/mobile, reduced-motion, delayed-initialization,
  motion-preference transitions and blocked-WASM coverage. Three further browser
  suite repetitions passed after the repair.
- The rebuilt motion island totaled **51,446 Brotli bytes / 60,000-byte budget**.
- The real disposable PostgreSQL proof passed **18 checks**: one fresh isolated
  and shared migration-chain/idempotence test, seven physician-auth tests, eight
  front-door tests and two physician-sample front-door tests. Upstream main fixes
  resolved the earlier pre-integration database failures; there is no outstanding
  database blocker from those runs. See [database-client versions and proof](backend-architecture.md#proof).
- The rebuilt production/staging image passed nonroot/read-only HTTP and shutdown
  smoke checks, as detailed below. It was not published or deployed.
- cargo-deny passed under inherited main policy. ZIP 8.6.0 is **not remediated**:
  the unchanged exceptions for RUSTSEC-2026-0367/0368/0369 remain in
  [`deny.toml`](/deny.toml).

Reproduce the site checks from the workspace root, inside the dev shell:

```bash
nix develop --command cargo test --locked -p public-site
nix develop --command cargo test --locked -p public-site --test site -- --ignored --nocapture
nix develop --command node crates/public-site/tests/import-snapshots.mjs --check
nix develop --command mdbook build book
```

Routine CI does not launch browsers. The `site` browser test is ignored by default; run the explicit command above or manually dispatch the CI workflow for browser qualification.

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

`crates/public-site/tests/fixtures/original/` contains 18 narrowly selected
original-source snapshots from `Coda-Health/kytos` commit
`5012bb749e29104e00ccb208a14a810f93fa420f`. `provenance.json` records the source
path, original SHA-256, snapshot SHA-256, and extraction policy for each input.
HTML, page/legal TypeScript, archive JSON, notices, CSS, and the scheduling social
image definition are verbatim source files, as is `public/homepage/site.js`, the
motion script the site ships unchanged, and `lib/site.ts`, which the legal modules
import. No Next application, package dependencies, private credentials, or
rendered Rust output are imported.

`crates/public-site/tests/fixtures/served/` records what production served at
that same commit (its `/api/health` reports the release SHA, and the importer
refuses any other): every page, the not-found page, the sitemap, the compiled utility
stylesheet, the rendered social image, and the status and `Location` of each
probe URL in `responses.json`. `provenance.json` records each file's URL and
SHA-256 under `served`. These are the outputs of Next, Tailwind and
`ImageResponse`, which the port must not re-implement.

These are test oracles, separate from generated application `data/`, CSS, and
assets. The normal parity suite verifies snapshot integrity; compares every
document with the page production served for it (head metadata and icons,
visible text of main and footer, and which stylesheet it links); checks every
probe URL's status and redirect; and compares CSS, the motion script and the
social image byte-for-byte with what the site serves. Missing or changed fixtures fail; there is no submodule
fallback or parity skip. Hashes detect accidental fixture edits, not malicious
co-edits to the provenance manifest: source updates require manual review.

### Deliberate reference updates (optional developer operation)

The `submodules/kytos` gitlink remains pinned and read-only. Only developers who
choose to inspect or update the historical source need repository access:

```bash
nix develop --command git submodule update --init -- submodules/kytos
nix develop --command node crates/public-site/tests/import-snapshots.mjs --import
nix develop --command node crates/public-site/tests/import-snapshots.mjs --import-served
nix develop --command node crates/public-site/tests/import-snapshots.mjs --check
```

The importer reads Git blobs at the explicit reviewed commit, checks that the
optional checkout is clean and pinned, and writes only allowlisted fixture paths.
`--import-served` reads `https://codahealth.ai` only while it reports that commit,
so it works until production moves off kytos; after that the frozen snapshot is
the record.
It is never invoked with `--import` by builds or tests. To adopt a new publication,
review the original changes first, explicitly update the importer's commit and
optional gitlink, then import and review the fixture/provenance diff. Do not
regenerate expected fixtures from Rust output or copy generated `data/` back into
them to make tests pass. Legal publication changes require their normal review.
Application generation (`data/import-reference.mjs`) reads these frozen snapshots
and is a separate, explicit operation. It imports the original TypeScript modules
as modules (Node strips their types), so legal copy is exactly what Next
evaluated, and copies the verbatim CSS, motion script and social image into
`assets/`. Normal tests
only use `--check`, so they also work in source exports without Git metadata or
initialized submodules.

## Image

`crates/public-site/Dockerfile` builds a static `x86_64-unknown-linux-musl` binary
and copies it into distroless static Debian 12 as UID/GID 65532. Builder/runtime
images are digest-pinned, Cargo uses `--locked`, and no mutable OS package
installation occurs during the build. Only workspace manifests/crates are copied,
not reference submodules, local credentials, or Terraform state. The Doppler CLI
is pinned to the same version and checksum as `cargo run -p workspace-tools -- install-doppler`.
Both executables are checked for a dynamic ELF interpreter during the build.
Pinned inputs improve repeatability; bit-for-bit image reproducibility is not
claimed without a repeated-build comparison.

### Private registries only

No image is pulled from or pushed to a public registry. All three repositories
are private Artifact Registry Docker repositories in `coda-health-staging`,
`us-east1`, with Google-managed encryption. The
[admin bootstrap](#staging-admin-bootstrap) creates them and refuses to continue
if any grants `allUsers` or `allAuthenticatedUsers`.

| Repository | Mode | Holds | Readers / writers |
| --- | --- | --- | --- |
| `public-site` | standard, immutable tags | site release images | CI deployer writes; runtime SA `public-site-staging@` reads |
| `public-site-dockerhub` | remote, preset upstream `docker-hub` | pull-through cache of `library/rust` | CI deployer and `gcp-engineering@` read |
| `public-site-base` | standard, immutable tags | digest copy of `gcr.io/distroless/static-debian12:nonroot` | CI deployer and `gcp-engineering@` read |

The Dockerfile names each base image by registry path (a build `ARG`) plus a
fixed digest. The defaults are the private repositories:

```dockerfile
ARG RUST_REPO=us-east1-docker.pkg.dev/coda-health-staging/public-site-dockerhub/library/rust
ARG RUNTIME_REPO=us-east1-docker.pkg.dev/coda-health-staging/public-site-base/distroless/static-debian12
FROM ${RUST_REPO}@sha256:606fd313a0f49743ee2a7bd49a0914bab7deedb12791f3a846a34a4711db7ed2 AS build
FROM ${RUNTIME_REPO}@sha256:afa5c872c891853ca7fcf1f12c3edb23f7eeef36189728842dd51042ff57f7ab
```

An override changes only where a base image is fetched from. The digest stays
fixed, so a different registry must serve identical content or the build fails.

**Why distroless is copied, not proxied.** Artifact Registry remote repositories
accept the preset `docker-hub` upstream and any custom HTTPS Docker upstream, so
`https://gcr.io` would also work as a remote. A standard immutable repository
plus a one-time copy by digest is used instead. Builds then never contact
`gcr.io`, and the copy cannot later change. The bootstrap copies it with
`docker buildx imagetools create --prefer-index=false`. With one source, this
writes the manifest index bytes unchanged, so the digest stays
`sha256:afa5c872…`. This was checked locally with `--dry-run`: the output
manifest matched the upstream index. `crane copy` is an equivalent alternative.
`gcloud artifacts docker images copy` does not exist in the pinned gcloud.

**Cloud Run pulls.** For an image in the same project, Cloud Run pulls through
its service agent `service-134570912572@serverless-robot-prod.iam.gserviceaccount.com`.
That agent's project-level `roles/run.serviceAgent` includes
`artifactregistry.repositories.downloadArtifacts`. Cloud Run's docs require an
explicit reader grant for this agent only for cross-project images. The
bootstrap grants it reader on `public-site` only if that default role was
removed. It also grants the runtime SA reader on `public-site` alone, so that
identity's image access is explicit and limited to that repository. OpenTofu
rejects any `image` outside `REGION-docker.pkg.dev/PROJECT_ID/public-site/` that
is not digest-pinned (see Validation).

**Local builds.** Engineers in `gcp-engineering@codahealth.ai` read the two
base-image repositories. Authenticate Docker once through gcloud's credential
helper; no key file or password is involved:

```bash
nix develop
gcloud auth login
gcloud auth configure-docker us-east1-docker.pkg.dev
docker build --platform=linux/amd64 \
  -f crates/public-site/Dockerfile -t public-site:local .
docker run --rm -p 127.0.0.1:8791:8080 public-site:local
```

The repositories do not exist until the bootstrap runs. Until then, or where
`*.pkg.dev` does not resolve, a developer can build from the same digests at
their public origin. This is for local use only and must never publish a
release:

```bash
docker build --platform=linux/amd64 \
  --build-arg RUST_REPO=docker.io/library/rust \
  --build-arg RUNTIME_REPO=gcr.io/distroless/static-debian12 \
  -f crates/public-site/Dockerfile -t public-site:local .
```

**Verification limit (2026-10-20).** This workstation does not resolve
`us-east1-docker.pkg.dev`, and the private repositories do not exist yet, so the
default private-registry build was not run here. `docker buildx build --check`
parsed the Dockerfile without warnings, using the public-origin overrides.
With the default ARGs it fails only at the DNS lookup. With the overrides,
`--network=host --platform=linux/amd64` produced image ID
`sha256:c1cc488bac07c783646c26d20a11642808979f80530f2012a09a998a00647ea9`. That
is byte-identical to the image built from `b18760e`, which was pushed as
`…@sha256:a179ab01…`. Only the registry path changed. The first build after the
bootstrap must use the private defaults.

The image defaults to `/public-site`, so local smoke tests need no Doppler account.
Cloud Run overrides its command to `/doppler run ... -- /public-site`. Secrets
remain in Doppler project `coda-rust`, config `stg` or `prd`. Secret Manager holds
only a read-only, environment-scoped `DOPPLER_TOKEN`, with a numeric version
pinned by the deployment root. The dedicated service account has access only to
that named secret, not database or physician resources. No secret value is read
by OpenTofu or placed in its variables/state. Doppler fallback files are disabled;
the four deployment-controlled environment values above are explicitly preserved
so Doppler cannot accidentally change the staging gate or listening address.

**Motion in the image:** the server embeds `assets/`, including the original
`/homepage/site.js`, so `docker build` alone produces the complete site; there is no
separate motion build. The local image HTTP smoke below checks that the homepage
references the content-addressed `chrome.js`, `site.js` and `duet.js` and that the served
`site.js` has the reviewed original SHA-256. It does not execute browser animation
or replace the browser proof. No host tooling or flake changes are introduced by the deployment artifacts.

## Prepared Cloud Run: staging IAP, production load balancer

`infra/gcp/public-site` is an isolated OpenTofu root. It owns public-site
**staging** state through a partial GCS backend (see
[Staging state and deployment](#staging-state-and-deployment)); production has
no backend configuration here and is never initialized from this workspace.
Both environments get a separate Cloud Run service and a dedicated service account.
The root creates no DNS records, standalone certificates, project, API enablement, service
identity, Google Group, database, or physician runtime dependency. Defaults are
min 0 / max 10 instances, concurrency 1000, one vCPU, and 512 MiB; these are bounded
configuration defaults, not demonstrated load capacity. Startup uses a TCP probe
on port 8080 rather than an unauthenticated HTTP health route.

**Staging** (`environment = "staging"`, the default) uses direct Cloud Run IAP and
no load balancer:

- `iap_enabled = true`, ingress `INGRESS_TRAFFIC_ALL`, the `run.app` URL
  enabled (output `staging_url`) and `invoker_iam_disabled = false`.
- The only `roles/run.invoker` member is the project's IAP service agent,
  `service-PROJECT_NUMBER@gcp-sa-iap.iam.gserviceaccount.com`. It is set through
  an authoritative binding on this service only. There is no `allUsers` grant,
  and CI or deployer identities get no implicit website access.
- The only `roles/iap.httpsResourceAccessor` member is
  `group:gcp-engineering@codahealth.ai` (variable `iap_group`). This is also an
  authoritative binding on this service only, not a project or domain grant.
  `iap_group` validation rejects bare domains and IAM principal prefixes.
- `PUBLIC_SITE_IAP_AUDIENCE` comes from `project_number`, `region` and the service
  name, and it is passed through Doppler with `--preserve-env`. The output
  `iap_audience` exposes the same value.
- `staging_domain` optionally maps a verified hostname to this same service.
  `env/staging.tfvars` sets it to `staging.codahealth.ai`. Cloud Run manages TLS;
  the operator configures the CNAME returned by `staging_domain_records`.
  Native IAP remains the access owner on both hostnames.
  This uses Google's [preview domain mapping](https://docs.cloud.google.com/run/docs/mapping-custom-domains#run)
  for staging only; it does not change the production topology.
- There is no serverless NEG, backend service, Cloud CDN, URL map, TLS policy,
  HTTPS proxy, reserved IP or forwarding rule. No standalone certificate is needed.

**Production** keeps its previous topology unchanged. Ingress is
`INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER` and the `run.app` URI is disabled. It
uses a serverless NEG, a global external managed HTTPS load balancer, a MODERN /
TLS 1.2 policy, the existing-certificate attachment (required, enforced by a
precondition), Cloud CDN, and the `allUsers` invoker grant the ALB needs. IAP is
off. `moved` blocks map the former unindexed load-balancer resources to `[0]`, so
existing production state keeps its addresses. Existing staging load-balancer
resources, if any, would be planned for destruction, not recreated. No Cloud Run
custom-domain mapping is created for production.

### Cloud CDN support and staging safety

Google explicitly documents Cloud CDN support for Cloud Run/serverless NEGs on a
**global external Application Load Balancer** in
[Enable Cloud CDN](https://cloud.google.com/load-balancing/docs/https/setup-global-ext-https-serverless#enable_cloud_cdn).
The [serverless NEG limitations](https://cloud.google.com/load-balancing/docs/negs/serverless-neg-concepts#limitations)
exclude backend health checks and configurable backend timeouts, not this CDN
combination. Those unsupported fields are intentionally omitted. IAP and Cloud
CDN cannot be enabled together on one backend. That is one reason staging uses
direct Cloud Run IAP with no backend service. Documentation was checked on
2026-10-20 rather than assuming the proposed topology was supported.

- **Production:** Cloud CDN is enabled with `USE_ORIGIN_HEADERS`, no forced
  caching and no negative caching. The server emits the cache headers described
  above; the infrastructure does not manufacture application cacheability.
- **Staging:** there is no load balancer or Cloud CDN, and no variable can add
  them. Origin responses are `no-store`, so no upstream layer may cache protected
  content. There is no Cloudflare dependency.

Do not declare staging live until post-deployment checks pass. An engineering
group member must be allowed, a nonmember denied, an unauthenticated direct
`run.app` request rejected, and no protected content served from a cache.

## Validation

Validation and the mocked tests initialize without the backend
(`-backend=false`), so they need no ADC credentials and make no live GCP API
calls. Do not initialize the other GCP roots from this workspace. Staging plan
and apply are covered in
[Staging state and deployment](#staging-state-and-deployment).

```bash
nix develop --command tofu -chdir=infra/gcp/public-site init -backend=false -input=false
nix develop --command tofu -chdir=infra/gcp/public-site fmt -check -recursive
nix develop --command tofu -chdir=infra/gcp/public-site validate
nix develop --command tofu -chdir=infra/gcp/public-site test
```

Original preparation checks on 2026-10-20 (before IAP): `tofu validate`, the then seven mocked test runs,
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
It runs the image as production does, with no physician upstream (no Sign in,
`/physician` 404); with an upstream (Sign in present, upstream origin absent from
the HTML); and in staging mode (IAP denials).
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

The IAP change was rebuilt and smoked again on 2026-10-20 as local image
`sha256:c1cc488bac07c783646c26d20a11642808979f80530f2012a09a998a00647ea9`
(24,870,060 bytes). This is also local only and was not pushed. The staging
container now receives the fixed `PUBLIC_SITE_IAP_AUDIENCE`. It returns 403 with
`no-store` and `noindex` for no credentials, for an unsigned
`x-goog-iap-jwt-assertion`, and for the unsigned `x-goog-authenticated-user-*`
headers. Legacy Cloudflare client-ID/secret and JWT headers, and the health token
by itself, are also denied on `/`, `/api/health` and `/readyz`. SIGTERM exits were
0 in 0.21/0.22 seconds. The smoke has no signed positive path. Signed allow, key
rotation and fetch failure are proven only by Rust tests with a public test key.

This HTTP proof does not replace browser animation/parity tests, a deep secret
scan of embedded source assets, a load test, cloud ingress verification, or an
actual deployment. No model calls, secret fetches, or live GCP requests occur in
the smoke script.

The provider is pinned to `hashicorp/google` 7.43.0, and its generated
`.terraform.lock.hcl` belongs in version control. The IAP and private-registry
changes left both unchanged. Twenty-two mocked test runs pass. They cover:

- Staging direct IAP, the `run.app` URL, invoker IAM checks, the IAP service agent
  as the only invoker, the engineering group as the only accessor, and both
  bindings scoped to this service.
- No staging ALB, NEG, CDN, IP, TLS frontend or certificate, plus the audience
  reaching the container through Doppler `--preserve-env`.
- The TCP startup probe, an in-memory mocked apply of the `staging_url` output,
  and the audience, service agent and group following the inputs.
- Production keeping ALB-only ingress, a disabled `run.app` URL, no IAP, the
  public ALB invoker, origin-header CDN and its HTTPS/TLS frontend.
- Rejection of production without a certificate, a project ID or `0` as the
  project number, a domain or IAM-prefixed `iap_group`, a mutable image, an
  unknown environment, excessive concurrency, inverted scaling and an unpinned
  secret.
- The image coming only from the private repository: a digest-pinned
  `us-east1-docker.pkg.dev/coda-health-staging/public-site/...` reference (also
  with a tag) reaches the service. These are rejected: the shared `coda-agents`
  repository, the `public-site-dockerhub` cache, Docker Hub, `gcr.io`, another
  project's `public-site`, a look-alike `pkg.dev` host, and a tag without a
  digest.

These checks validate resource contracts, not cloud permissions, API enablement,
IAP brand/OAuth configuration, group membership, regional quotas, autoscaling
behavior, application health, or a real deployment.

## Staging state and deployment

This root owns public-site staging state. The physician roots under `infra/gcp`
keep their product-repository apply authority; this ownership covers only
`infra/gcp/public-site` with `environment = "staging"`.

- **State.** `backend "gcs" {}` is partial. `backend/staging.gcs.tfbackend`
  selects a dedicated staging bucket, `coda-health-staging-tofu-state-us-east1`
  in `coda-health-staging`, with prefix `public-site/staging`. It is versioned,
  uses uniform bucket-level access, enforces public access prevention, lives in
  `us-east1` and keeps the default soft-delete policy. The
  [admin bootstrap](#staging-admin-bootstrap) creates it; it does not exist until
  that runs. Staging state never lives in a `coda-health-prod` bucket, so the CI
  deployer needs no production-project IAM. Production has no backend file.
- **Inputs.** `env/staging.tfvars` holds the non-secret staging inputs: the
  published image digest in the private `public-site` repository, release SHA,
  secret ID and pinned version, and `physician_app_url`, the physician preview
  service in `coda-rust-staging` behind its own IAP. Setting it mounts `/physician/`,
  renders Sign in and gives the runtime its proxy signing identity; leaving it
  unset, as production does, offers no sign-in.
  That image reference resolves only after the bootstrap copies it there.
- **Doppler.** Config `coda-rust/stg` holds the same non-secret deploy values
  under `PUBLIC_SITE_DEPLOY_*` (project, number, region, service, image, release
  SHA, IAP audience, state bucket and prefix). Secret Manager holds only the
  `public-site-stg-doppler-token` secret (a read-only `stg` service token).

The first, abandoned initialization wrote an **empty** state object under
`gs://coda-health-prod-terraform-state-us-east1/public-site/staging/`. Nothing was
applied, so nothing needs migrating. That prefix is unused; a bucket owner may
delete it. This workspace does not delete anything in the production bucket.

### Staging admin bootstrap

`infra/gcp/public-site/scripts/staging-admin-bootstrap.sh` is run once by an
administrator of `coda-health-staging`, not by the deployer. It prepares only
what the root deliberately does not own. Each create step checks first, so it is
safe to rerun. It prints every command, never prints or stores a secret value,
and never touches `coda-health-prod`. Review it with `--print` first; that mode
makes no gcloud or Doppler calls.

```bash
# From the workspace root, inside the dev shell, as the staging admin:
nix develop
gcloud auth login                     # the admin's account
doppler login                         # only needed for the token step
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --print
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh   # needs docker (buildx)
# OPTIONAL, only if Tyler should run the first apply himself (expires in 24 h):
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --temporary-deployer-grant
# After the first apply, rerun once to add the service-scoped CI bindings:
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh
```

In order, it:

1. **APIs.** Enables `serviceusage`, `cloudresourcemanager`, `iam`,
   `iamcredentials`, `sts`, `run`, `iap`, `artifactregistry`, `storage` and
   `secretmanager` (`gcloud services enable` is idempotent).
2. **State bucket.** Creates `gs://coda-health-staging-tofu-state-us-east1`
   (`--location=us-east1 --uniform-bucket-level-access --public-access-prevention`)
   and asserts `--versioning`. Soft delete stays at the default.
3. **Private Artifact Registry repositories** (see
   [Private registries only](#private-registries-only)). Creates `public-site`
   (standard, `--immutable-tags`), `public-site-dockerhub`
   (`--mode=remote-repository --remote-docker-repo=docker-hub`) and
   `public-site-base` (standard, `--immutable-tags`). It reasserts immutable
   tags. On `public-site` it sets a cleanup policy: delete untagged versions
   after 7 days, always keep the newest 10 (`SITE_KEEP_COUNT`). Tagged images
   cannot be deleted in an immutable-tag repository. Then it **refuses to
   continue** if any of the three grants `allUsers` or `allAuthenticatedUsers`,
   or if any of their policies cannot be read. It checks `coda-agents` too and
   fails if that repository is public. An unreadable `coda-agents` policy is
   only a warning, because this script does not own that repository. Finally it
   grants `gcp-engineering@codahealth.ai` reader on the two base-image
   repositories. It grants the Cloud Run service agent reader on `public-site`
   only if the agent lacks `roles/run.serviceAgent`.
4. **Doppler token secret.** Creates `public-site-stg-doppler-token` with
   `--replication-policy=user-managed --locations=us-east1`. The root only
   references the secret by ID and numeric version, so either replication works;
   user-managed keeps it in the service's region. If no enabled version exists it
   runs
   `doppler configs tokens create public-site-staging-runtime --project coda-rust --config stg --access read --plain | tr -d '\r\n' | gcloud secrets versions add public-site-stg-doppler-token --data-file=-`.
   Line endings from Doppler's CLI are removed before storing the token; a
   trailing newline makes the runtime Authorization header invalid. The token
   stays in the pipe and is never echoed, assigned to a variable or written to
   disk. The project and config are fixed in the script (the dev shell exports
   `DOPPLER_CONFIG=dev`). Pin the resulting numeric version in
   `env/staging.tfvars`; deployed staging uses version `2`.
   `--rotate-doppler-token` adds another version; then update that pin.
5. **CI deployer.** Creates `public-site-ci-deployer@coda-health-staging.iam.gserviceaccount.com`
   with no keys. Its roles follow what this root manages (see the table below).
   If a `roles/artifactregistry.writer` grant for it exists on `coda-agents`, it
   is removed. After the first apply the runtime SA gets reader on `public-site`.
6. **GitHub Workload Identity Federation.** Creates pool `github-actions` and
   OIDC provider `github-oidc` (issuer `https://token.actions.githubusercontent.com`).
   The provider's attribute condition admits only repository
   `Coda-Health/coda-rust` (name, repository ID `1412706552` and owner ID
   `306538119`), ref `refs/heads/main` and GitHub environment `staging`. The CI
   service account grants `roles/iam.workloadIdentityUser` only to
   `principal://iam.googleapis.com/projects/134570912572/locations/global/workloadIdentityPools/github-actions/subject/repo:Coda-Health@306538119/coda-rust@1412706552:environment:staging`.
   The repository uses GitHub's immutable subject claim, so `sub` carries the
   owner and repository IDs; a `repo:Coda-Health/coda-rust:...` binding never
   matches and the token exchange fails with `iam.serviceAccounts.getAccessToken`
   denied. A rerun reasserts the condition.
7. **Base images, by digest.** Runs `gcloud auth configure-docker us-east1-docker.pkg.dev`.
   Copies `gcr.io/distroless/static-debian12@sha256:afa5c872…` into
   `public-site-base/distroless/static-debian12` (tag `nonroot-afa5c872c891`) and
   checks that the digest exists there. Resolves
   `public-site-dockerhub/library/rust@sha256:606fd313…` through the cache.
8. **Site image migration.** Copies
   `coda-agents/public-site@sha256:a179ab01…` (tag
   `b18760ef98a82e68aef29ac7a20bc48796794bf9`) into `public-site/public-site` by
   digest. Checks it with `gcloud artifacts docker images describe`, then repeats
   the public-access check on every repository. A copy that already exists is
   skipped, so reruns are safe. The copy needs read on `coda-agents` (the
   admin's) and writer on `public-site`.
9. **Optional temporary grant** (`--temporary-deployer-grant`). Gives
   `tyler@codahealth.ai` the first-apply roles, each with an IAM condition
   `request.time < timestamp(NOW + 24h)`: project-level
   `roles/iam.serviceAccountCreator`, `roles/iam.serviceAccountUser`,
   `roles/run.admin` and `roles/iap.admin`, plus `roles/secretmanager.admin` on
   the one secret, `roles/storage.objectAdmin` on the state bucket and
   `roles/artifactregistry.reader` on `public-site`. The project-level roles are
   project-level because the service account and the service do not exist before
   the first apply. Expired bindings grant nothing; remove them afterwards with
   `remove-iam-policy-binding ... --all`. Skip this block if the admin runs the
   first apply.

CI deployer roles, evaluated against the resources this root manages (runtime
service account, its accessor on the secret, the Cloud Run service, its invoker
policy and its IAP accessor policy):

| Role | Scope | Why |
| --- | --- | --- |
| `roles/storage.objectAdmin` | the staging state bucket only | Read, write and lock state objects |
| `roles/artifactregistry.writer` | repository `public-site` only | Publish digest-pinned site images (no grant on `coda-agents`) |
| `roles/artifactregistry.reader` | repositories `public-site-dockerhub` and `public-site-base` only | Pull the two base images for the build |
| `roles/secretmanager.admin` | secret `public-site-stg-doppler-token` only | Manage the runtime accessor binding (`setIamPolicy`); no narrower predefined role has it |
| `roles/run.viewer` | project | Read-only; the provider polls location-scoped Cloud Run operations |
| `roles/run.admin` | service `public-site-staging` only (after first apply) | Update the service and its invoker policy |
| `roles/iap.admin` | IAP resource of `public-site-staging` only (after first apply) | Manage the authoritative IAP accessor binding |
| `roles/iam.serviceAccountUser` | runtime SA `public-site-staging@` only (after first apply) | Deploy revisions that run as it |

CI gets no `iam.serviceAccountAdmin` (it never creates or edits the runtime
account: the first apply is a human apply), no project-wide `run.admin`,
`iap.admin` or `actAs`, and no `run.invoker` or `iap.httpsResourceAccessor`, so
it has no website access. The service-scoped rows need the service and runtime
account to exist; the script skips them until then and adds them on a rerun.
Tofu's authoritative bindings cover only `roles/run.invoker` and
`roles/iap.httpsResourceAccessor`, so they do not remove these grants. The
`deploy-public-site-staging` job (below) declares `environment: staging`, runs on
`main` and uses `google-github-actions/auth` with
`workload_identity_provider: projects/134570912572/locations/global/workloadIdentityPools/github-actions/providers/github-oidc`
and `service_account: public-site-ci-deployer@coda-health-staging.iam.gserviceaccount.com`.

### CI deploy on push to main

The `deploy-public-site-staging` job in
[`.github/workflows/ci.yml`](/.github/workflows/ci.yml) redeploys staging from every
push to `main`, after `ci` (unit, service, storage and HTTP checks) and
`cargo-deny` pass. It runs only when the repository variable
`PUBLIC_SITE_STAGING_DEPLOY` is `enabled`; unset it to stop deploys without
editing the workflow. The GitHub `staging` environment allows only the `main`
branch to deploy, which matches the provider's attribute condition. The job:

1. Builds `crates/public-site/Dockerfile` with the private base-image defaults
   and runs `smoke.py` on the result.
2. Pushes the image to `public-site/public-site:<sha>`. Tags are immutable, so a
   rerun of the same commit reuses the digest it already published.
3. Runs `tofu plan` with `env/staging.tfvars` plus `-var image=<digest>` and
   `-var release_sha=<sha>`, and fails unless the saved plan is empty or an
   in-place update of `google_cloud_run_v2_service.site` alone. Then it applies
   that saved plan.
4. Requires the latest revision to be Ready, serve 100% of traffic and report
   the new digest. It also requires anonymous requests to the run.app homepage
   and `/health` and to `staging.codahealth.ai` to redirect to Google sign-in.

`env/staging.tfvars` keeps the last operator-reviewed digest; a local plan run
from it after a CI deploy shows the image going back. Pass the current digest
with `-var image=...` (read it from `gcloud run services describe`) when
planning by hand.

Before the job was enabled, a rehearsal impersonated the CI account under a
3-hour conditional `roles/iam.serviceAccountTokenCreator` grant to Tyler. As
that identity it initialized the backend, refreshed the full state (including
the domain mapping) with no changes, read the image digest, and held
`uploadArtifacts` on `public-site`, `run.services.update` on the service and
`actAs` on the runtime account. The grant was then removed; the account's policy
again holds only the GitHub `workloadIdentityUser` binding.

**Group check (Workspace admin).** The IAP accessor is
`group:gcp-engineering@codahealth.ai`. A Google Workspace or Cloud Identity admin
can confirm the group exists and whether Tyler is in it (read-only):

```bash
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --check-group
# runs:
gcloud identity groups describe gcp-engineering@codahealth.ai --project=coda-health-staging
gcloud identity groups memberships check-transitive-membership \
  --project=coda-health-staging --group-email=gcp-engineering@codahealth.ai \
  --member-email=tyler@codahealth.ai
```

### First apply (after the bootstrap)

The new bucket's state is empty, and so was the abandoned prefix in the
production bucket, so `-migrate-state` is not needed. `-reconfigure` makes the
local `.terraform` directory forget the old backend. Run from the workspace root
as the admin, or as Tyler while his temporary grant is valid:

```bash
nix develop
export GOOGLE_OAUTH_ACCESS_TOKEN="$(gcloud auth print-access-token --project=coda-health-staging)"
tofu -chdir=infra/gcp/public-site init -input=false -reconfigure \
  -backend-config=backend/staging.gcs.tfbackend
tofu -chdir=infra/gcp/public-site plan -input=false \
  -var-file=env/staging.tfvars -out=staging.tfplan
# Read the plan: a new deployment has 6 resources, including the domain mapping.
# Apply only that saved plan; never apply production.
tofu -chdir=infra/gcp/public-site apply staging.tfplan
tofu -chdir=infra/gcp/public-site output staging_url
```

Then have the admin rerun the bootstrap once so CI gets its service-scoped roles,
and run the qualification checks below.

**Post-bootstrap: Doppler image.** `coda-rust/stg` still holds
`PUBLIC_SITE_DEPLOY_IMAGE` at the `coda-agents` digest. Change it only after
step 8 has copied the image and
`gcloud artifacts docker images describe us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site@sha256:a179ab017b9b47179b0f9e71e7237242b9d454d8e27d98b2eb671bbbfd263140`
succeeds:

```bash
doppler secrets set \
  PUBLIC_SITE_DEPLOY_IMAGE=us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site@sha256:a179ab017b9b47179b0f9e71e7237242b9d454d8e27d98b2eb671bbbfd263140 \
  --project coda-rust --config stg
```

### Staging deployment status (2026-10-21)

The admin bootstrap and first deployment are applied in `coda-health-staging`.
Cloud Run reports revision `public-site-staging-00003-5mw` Ready at
[staging.codahealth.ai](https://staging.codahealth.ai), with the
[protected Cloud Run URL](https://public-site-staging-5yiq4mt4lq-ue.a.run.app)
available directly.

- The dedicated versioned state bucket, three private image repositories,
  read-only `coda-rust/stg` token, CI service account, and restricted GitHub WIF
  provider exist. The CI account has the documented service-scoped post-apply
  grants. The CI deploy job is described in
  [CI deploy on push to main](#ci-deploy-on-push-to-main).
- The site image and distroless base were copied with identical digests. The
  Docker Hub cache resolves the pinned Rust base. Repository IAM readback found
  no `allUsers` or `allAuthenticatedUsers` grants, including on `coda-agents`.
  Doppler's image setting now uses `public-site/public-site`.
- The first revision failed because `doppler --plain` appended a newline to the
  token. Version `2` contains the same token without that line ending; it was
  authenticated against Doppler before deployment. The bootstrap now strips
  line endings, and `env/staging.tfvars` pins version `2`.
- Cloud Run's invoker binding contains only the IAP service agent. IAP website
  access is limited to `gcp-engineering@codahealth.ai`; Cloud Identity confirmed
  Dev and Tyler are members. An authenticated Chrome session as Dev rendered
  the homepage and Clinical Transparency page. The motion JS/WASM and image/CSS
  assets loaded successfully. Observed page and asset responses were HTTP 200,
  `Cache-Control: private, no-store, max-age=0` and
  `X-Robots-Tag: noindex, nofollow`; assets were not served from disk cache or a
  service worker. A signed-in personal Google account was denied by IAP.
  Anonymous homepage, health and hashed-asset requests redirect to Google sign-in.
  Google's signed-assertion test with an invalid signature returned HTTP 403
  from the application (`Google IAP assertion required`), with no-store/noindex.
  These checks do not claim Tyler has tested his own browser.
- The first apply used Dev's existing admin account. No temporary project-wide
  grant was made to Tyler.
- On October 21, a reviewed saved plan added only the staging domain mapping:
  1 added, 0 changed, 0 destroyed. Cloudflare's staging CNAME changed from
  `cname.vercel-dns.com` (proxied) to `ghs.googlehosted.com` (DNS only), as returned
  by Cloud Run. DNS readback confirmed the target. Root and `www` production
  records were unchanged, and no production resources were planned or applied.
  The old public-staging Vercel project and its Cloudflare Access application
  were then deleted at Dev's request. Readback confirmed both absent. Shared
  Cloudflare policies and service tokens were preserved because the separate
  `app-staging.codahealth.ai` application still uses them. Production remained
  on the same Vercel deployment.
- Google reported the custom hostname certificate provisioned at 20:29 UTC.
  After edge propagation, normal HTTPS, engineering-member page access,
  signed-in nonmember denial, anonymous page/health/asset redirects and
  no-store/noindex page and asset responses also passed on `staging.codahealth.ai`.
  The post-apply staging plan had no changes.
- Later on October 21, Tyler redeployed from main `3609430` (after the WASM island
  was replaced by the original motion script). His account refreshed the full
  state, including the domain mapping. The preflight plan showed no changes. A
  local build passed `smoke.py`, which no longer looks for the removed WASM
  assets in its staging leg. The image was pushed by digest as
  `public-site@sha256:5a8693ef4801ca73a661cb7c018f743495c86ba4b681ac6e4a34135709133e29`.
  The reviewed saved plan was 0 added, 1 changed, 0 destroyed: an in-place update
  of `google_cloud_run_v2_service.site`. After apply, the replan showed no changes.
  Revision `public-site-staging-00003-5mw` is Ready, serves 100% of traffic, and
  reports that digest. Anonymous requests to the homepage and `/health` on
  `staging.codahealth.ai`, and to the run.app URL, still redirect to Google
  sign-in. Doppler `coda-rust/stg` `PUBLIC_SITE_DEPLOY_IMAGE` and
  `PUBLIC_SITE_DEPLOY_RELEASE_SHA` match the new digest and SHA. No authenticated
  browser render of this revision has been recorded yet: the automation browser
  was signed in with a non-member account, and IAP correctly denied it.

### Qualification after the October 21 deployment

The October 21 access checks passed on both staging hostnames. That deployment
still had the WASM animation stutter and a Sign in link back to the marketing
site. The source now uses the original motion script described in
[Application contract](#application-contract). The later October 24
[same-origin physician sign-in qualification](#same-origin-physician-proxy-qualification)
supersedes the earlier sign-in limitation. Physician/Duet application staging
has a separate [deployment owner](gcp.md#rust-staging-bootstrap-and-deployment-gate).

CI and deployment identities have no website-access grant, so the CI deploy job
checks only that anonymous requests are sent to sign-in; an authenticated render
is still a manual check. The legacy kytos Deploy Staging and Staging Smoke
workflows are disabled, and their staging-only repository variables and secrets
are removed. Their Cloudflare headers and Next.js source comparisons cannot
qualify this deployment; do not retarget them to the IAP hostname.

The [legacy production promotion script](https://github.com/Coda-Health/kytos/blob/main/scripts/promote-production.mjs)
and production workflow are unchanged. They still require successful legacy
staging checks for the exact kytos `origin/main` SHA, so new commits cannot pass
those retired gates. Historical greens and this Rust site's access checks do
not qualify a new Next.js release. The next production release needs an
explicitly agreed qualification path before those gates change.

There is no DNS-only rollback to Vercel after deleting its public-staging
project. Keep the `staging` CNAME on `ghs.googlehosted.com` with proxying disabled;
the protected Cloud Run URL is also available for diagnosis. Do not change the
root, `www`, app or work records. Production cutover remains a separate decision
and is never planned or applied from this workspace.

## Local release load check (2026-10-20)

The approved `oha` package is included in the Nix shell (resolved version 1.16.0).
`crates/public-site/load_check.py` starts only disposable loopback listeners, with
a clean child environment containing the tool PATH, `APP_ENV`, and the local
listen address. It neither targets arbitrary URLs nor starts model/data services.
Production cache behavior is measured locally; no production traffic, cloud
resource, staging policy, or deployed service is changed.

Reproduce from this worktree root:

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
Those runs predate IAP. Staging now refuses to start without
`PUBLIC_SITE_IAP_AUDIENCE`, so the harness passes the fixed staging audience to
its staging child. The load runs were not repeated for the IAP change. The local
deny-all behavior is covered by the crate tests and the image smoke instead.
Both child servers exited cleanly with code 0 after SIGTERM in each run, and all
oha processes exited. Nix-shell activation, Python compilation, the final harness,
`cargo fmt --all --check`, and `git diff --check` passed.
