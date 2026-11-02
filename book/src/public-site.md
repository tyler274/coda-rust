# Public site

`crates/public-site` is the Rust public marketing site. It serves `codahealth.ai`
and `www.codahealth.ai` from `coda-health-prod`, and `staging.codahealth.ai` behind
IAP from `coda-health-staging`. It is separate from the physician runtime and Duet:
no database, physician accounts, model execution, or clinical types are introduced
by the site. Sign-in exists only where `PHYSICIAN_APP_URL` is configured: that one
setting mounts the same-origin `/physician/` proxy and renders the nav's Sign in
link. Staging sets it; production doesn't, so production offers no sign-in and
`/physician/` is an ordinary 404.
`submodules/kytos` is an optional read-only developer reference, never a build,
test, CI, or runtime dependency.

## Application contract

The application design is Dioxus route components prerendered at startup, served
by a stateless axum binary. The sitemap lists only current documents; archived
versions stay reachable from their policy pages, and `/scheduling/dev` is noindex.
`/physicians` and `/api/physician-waitlist` are 404.

Startup builds the page/asset tables and their identity, gzip and Brotli
representations once; requests do not render Dioxus or query a database. The server
serves content-addressed immutable assets and HTML with bounded public/shared-cache
lifetimes, encoding-specific ETags, conditional requests and bodyless HEAD responses.
Every response carries `Strict-Transport-Security: max-age=63072000`.
Motion is `/homepage/site.js`, a reviewed snapshot served byte-for-byte on `/` and
`/duet`. It also paints the landscapes. Parity therefore holds by construction: `tests/parity.rs`
fails if the shipped script, `site.css` or `chrome.css` differ from the reviewed
snapshots, and the browser proof fails if the hero pointer jumps instead of
gliding. Site-owned additions are layout or fallback only, never motion:
`stage.css` reserves and scales each product frame before the script runs (it then
sets the same values inline), `noscript.css` reveals content when scripts are
disabled, and `chrome.js` floats the nav after 8px of scroll. Reduced motion is
decided by `site.js` at load.

The one deliberate exception is the Duet story (`src/pages/phones.rs`). It replaces
the original Duet phones on `/` and `/duet` with one story across three phones. It
reuses `site.css` phone hardware and the stage scaling and landscapes `site.js`
applies to every frame, but its classes (`duet-story`, `ds-*`) are its own, so the
original Duet loop never binds to it. Its styles are `css/duet.css`, every selector
scoped to `.duet-story`; its motion is `homepage/duet.js`, which plays only while
the stage is on screen. The markup is the story's last frame, which reduced-motion
and no-script visitors see. The region is marked `data-owner="site"`: the parity
suite compares the rest of the page with the original's served output and checks
that it had the original phones (`duet-phones.html`) where the story stands.

The same rule covers everything else the original computes in a build or runtime
step: the port consumes the output instead of re-implementing the step, because a
re-implementation drifts without any test noticing when the tests re-derive their
expectations the same way. Pages belong to one of the original's two systems
(`Route::shell`). `/` and `/duet` use the homepage system: `chrome.css`, `site.css`
and the motion script (plus the site-owned Duet story). Every other page uses the
utility system: the original layout's shell markup and its compiled Tailwind
stylesheet, served verbatim as `css/utility.css` (it already contains
`chrome.css`). The `/scheduling/dev` social image is the PNG the original's
`ImageResponse` rendered. Head metadata and icons follow the original's
`lib/metadata.ts` and root layout. URLs follow Next's default rule: a trailing slash
is removed with a 308 that keeps the query.

Staging applies its origin IAP gate to pages, assets, redirects, health routes
and errors. Responses force `private, no-store, max-age=0`, CDN-specific no-store
headers, `Vary: *` and `X-Robots-Tag: noindex, nofollow`. IAP authenticates the
user in front of Cloud Run. The origin still verifies the signed
`x-goog-iap-jwt-assertion` header on every request in
[`access.rs`](/crates/public-site/src/access.rs), against the exact audience
`/projects/NUMBER/locations/REGION/services/NAME`. That is the direct Cloud Run
signed-header audience, not an OAuth client ID or URL. It does not trust the
signature-free `x-goog-authenticated-user-*` headers, and it does not let IAP's
presence stand in for verification. When a key refresh fails, cached keys are not
used, so the request is denied.

Missing or invalid assertions return 403. A staging process with a missing or
malformed audience refuses to start. Cloudflare Access variables, headers and
service tokens authorize nothing. An optional separate loopback-only probe listener
serves no site content. Local tests prove this application boundary with a public
test-only key. They do not prove a deployed IAP configuration; member, nonmember
and direct-origin checks are deployment checks.

Runtime configuration:

- `PUBLIC_SITE_LISTEN_ADDR`: local default `127.0.0.1:8791`; the image and Cloud
  Run root explicitly set `0.0.0.0:8080`.
- `PHYSICIAN_APP_URL`: optional fixed upstream origin for `/physician/`, not the
  browser navigation link. When set, the site mounts `/physician/` and the nav offers
  Sign in; when unset, neither exists. There is no default: a local run against the
  sample physician server sets `http://127.0.0.1:8790`. Deployed values must use
  HTTPS; credentials, non-root paths, queries and fragments are rejected.
- `PHYSICIAN_PROXY_SERVICE_ACCOUNT`: service identity used for keyless upstream
  IAP authentication. Required in staging whenever `PHYSICIAN_APP_URL` is set.
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
  from the project number, region and service name, and it is not a secret.
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

Use the shell's Chromium (or set `CHROMIUM` to the approved browser executable).
The default startup uses port 8791; choose another `PUBLIC_SITE_LISTEN_ADDR` if a
preview already owns it rather than stopping that preview.

## Physician entry point

Production configures no physician upstream. Configuring one there requires
qualified production physician authentication first; production must not expose
unconfigured proof-token delivery.

The proxy must preserve physician bearer authorization, query strings and streamed
responses without forwarding browser cookies, caller-supplied service credentials
or IAP identity assertions to the upstream. Upstream IAP authentication is a
separate service-account credential in `Proxy-Authorization`, not a replacement
for the physician `Authorization` header. The public origin's verified IAP gate
remains mandatory for all staging proxy requests. Missing service credentials or
failed refreshes fail closed; neither IAP boundary may be disabled to make the
proxy work.

The credential flow is keyless: IAM Credentials `signJwt`, using the public-site
runtime identity, a short-lived JWT scoped to the configured physician origin, and
a self-scoped signing permission. The custom role `publicSitePhysicianProxySigner`
(`iam.serviceAccounts.signJwt` only) is bound runtime-to-itself; the
[staging admin bootstrap](#staging-admin-bootstrap)'s `--physician-proxy` flag
creates it. This permission is identity-signing authority, **not** an IAM
restriction to one audience. The physician infrastructure owner grants that runtime
identity IAP access to **only** the physician service through its authoritative
`physician_proxy_invokers` binding; public-site infrastructure does not compete
with that binding. No service-account private key or token belongs in source, URLs,
browser state or logs.

`/physician/` is not a browser security boundary: marketing scripts on the same origin
can access the physician session stored in local storage. The entire origin
must therefore be treated as trusted physician-session code. Proxy responses
must never enter public or CDN caches. Cloud Run streams are finite; clients
reconnect rather than relying on a longer request timeout.

## Original-source snapshots

Ordinary builds, tests, and CI do not fetch or access the old private repository,
so they need no private credential or application secret, and fork and Dependabot
changes run the same parity tests. The site itself reads no secrets.

`crates/public-site/tests/fixtures/original/` contains narrowly selected
original-source snapshots from `Coda-Health/kytos`, at the reviewed commit named in
`provenance.json`. That file records the source path, original SHA-256, snapshot
SHA-256, and extraction policy for each input. HTML, page/legal TypeScript, archive
JSON, notices, CSS, and the scheduling social image definition are verbatim source
files, as is `public/homepage/site.js`, the motion script the site ships unchanged,
and `lib/site.ts`, which the legal modules import. No Next application, package
dependencies, private credentials, or rendered Rust output are imported.

`crates/public-site/tests/fixtures/served/` records what the kytos deployment served
at that same commit: every page, the not-found page, the sitemap, `robots.txt`, the
compiled utility stylesheet, the rendered social image, and the status, `Location`
and `Strict-Transport-Security` of each probe URL in `responses.json`. `provenance.json`
records each file's URL and SHA-256 under `served`. These are the outputs of Next,
Tailwind and `ImageResponse`, which the port must not re-implement.

These are test oracles, separate from generated application `data/`, CSS, and
assets. The normal parity suite verifies snapshot integrity; compares every
document with the page served for it (head metadata and icons, visible text of main
and footer, and which stylesheet it links); checks every probe URL's status and
redirect; and compares CSS, the motion script and the social image byte-for-byte
with what the site serves. Missing or changed fixtures fail; there is no submodule
fallback or parity skip. Hashes detect accidental fixture edits, not malicious
co-edits to the provenance manifest: source updates require manual review.

### Deliberate reference updates (optional developer operation)

The `submodules/kytos` gitlink is pinned and read-only. Only developers who
choose to inspect or update the original source need repository access:

```bash
nix develop --command git submodule update --init -- submodules/kytos
nix develop --command node crates/public-site/tests/import-snapshots.mjs --import
nix develop --command node crates/public-site/tests/import-snapshots.mjs --check
```

The importer reads Git blobs at the explicit reviewed commit, checks that the
optional checkout is clean and pinned, and writes only allowlisted fixture paths.
The served fixtures are a frozen record and can't be regenerated.
Builds and tests never run `--import`. To adopt a new publication, review the
original changes first, explicitly update the importer's commit and optional
gitlink, then import and review the fixture/provenance diff. Do not regenerate
expected fixtures from Rust output or copy generated `data/` back into them to make
tests pass. Legal publication changes require their normal review.
Application generation (`data/import-reference.mjs`) reads these frozen snapshots
and is a separate, explicit operation. It imports the original TypeScript modules
as modules (Node strips their types), so legal copy is exactly what Next
evaluated, and copies the verbatim CSS, motion script and social image into
`assets/`. Normal tests only use `--check`, so they also work in source exports
without Git metadata or initialized submodules.

## Image

`crates/public-site/Dockerfile` builds a static `x86_64-unknown-linux-musl` binary
and copies it into distroless static Debian 12 as UID/GID 65532. Builder/runtime
images are digest-pinned, Cargo uses `--locked`, and no mutable OS package
installation occurs during the build. Only workspace manifests/crates are copied,
not reference submodules, local credentials, or Terraform state. The binary is
checked for a dynamic ELF interpreter during the build.
Pinned inputs improve repeatability; bit-for-bit image reproducibility is not
claimed without a repeated-build comparison.

The server embeds `assets/`, including the original `/homepage/site.js`, so
`docker build` alone produces the complete site; there is no separate motion build.

The site reads no secrets, so it has no Doppler config, token or Secret Manager
access. Cloud Run runs the image's own `/public-site` entrypoint, exactly as the
local smoke tests do, with the deployment-controlled environment values above set
directly. The dedicated service account has no database, physician or secret access.

### Private registries only

No image is pulled from or pushed to a public registry. The three staging
repositories are private Artifact Registry Docker repositories in
`coda-health-staging`, `us-east1`, with Google-managed encryption. Production has
one, `public-site` in `coda-health-prod`, which holds digests copied from staging's.
The admin bootstraps create them and refuse to continue if any grants `allUsers` or
`allAuthenticatedUsers`.

| Repository | Mode | Holds | Readers / writers |
| --- | --- | --- | --- |
| `public-site` | standard, immutable tags | site release images | CI deployer writes; runtime SA `public-site-staging@` reads |
| `public-site-dockerhub` | remote, preset upstream `docker-hub` | pull-through cache of `library/rust` | CI deployer and `gcp-engineering@` read |
| `public-site-base` | standard, immutable tags | digest copy of `gcr.io/distroless/static-debian12:nonroot` | CI deployer and `gcp-engineering@` read |

The Dockerfile names each base image by registry path (a build `ARG`) plus a
fixed digest, and the defaults are the private repositories. An override changes
only where a base image is fetched from. The digest stays fixed, so a different
registry must serve identical content or the build fails.

Distroless is a one-time copy by digest into a standard immutable repository
rather than a remote repository, so builds never contact `gcr.io` and the copy
cannot later change.

The bootstrap grants the runtime SA reader on `public-site` alone, so that
identity's image access is explicit and limited to that repository. OpenTofu
rejects any `image` outside `REGION-docker.pkg.dev/PROJECT_ID/public-site/` that
is not digest-pinned (see [Checks](#checks)).

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

Where `*.pkg.dev` does not resolve, a developer can build from the same digests at
their public origin. This is for local use only and must never publish a release:

```bash
docker build --platform=linux/amd64 \
  --build-arg RUST_REPO=docker.io/library/rust \
  --build-arg RUNTIME_REPO=gcr.io/distroless/static-debian12 \
  -f crates/public-site/Dockerfile -t public-site:local .
```

## Infrastructure: staging IAP, production load balancer

`infra/gcp/public-site` is an isolated OpenTofu root. It owns public-site staging
and production state through a partial GCS backend, one file per environment (see
[Staging state and deployment](#staging-state-and-deployment) and
[Production state and deployment](#production-state-and-deployment)).
Both environments get a separate Cloud Run service and a dedicated service account.
The root creates no DNS records, project, API enablement, service identity, Google
Group, database, or physician runtime dependency. Production's one certificate is the
Certificate Manager certificate described below. Its scaling defaults are bounded
configuration, not demonstrated load capacity. Startup uses a TCP probe on port 8080
rather than an unauthenticated HTTP health route.

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
- `PUBLIC_SITE_IAP_AUDIENCE` comes from `project_number`, `region` and the service
  name, and it is set directly on the container. The output
  `iap_audience` exposes the same value.
- `staging_domain` optionally maps a verified hostname to this same service.
  `env/staging.tfvars` sets it to `staging.codahealth.ai`. Cloud Run manages TLS;
  the Cloudflare `staging` record is a DNS-only CNAME to `ghs.googlehosted.com`, as
  `staging_domain_records` returns. Native IAP remains the access owner on both
  hostnames.

**Production** (`environment = "production"`) serves the same image behind a load
balancer. Ingress is `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER` and the `run.app` URI
is disabled. It uses a serverless NEG, a global external managed HTTPS load
balancer, an HTTP frontend on the same address that answers every request with a
308 to HTTPS (host, path and query kept), a MODERN / TLS 1.2 policy, Cloud CDN, and
the Cloud Run invoker check turned off (`invoker_iam_disabled`), because the ALB
does not authenticate and the organization's domain restriction forbids an
`allUsers` grant; ALB-only ingress and the disabled `run.app` URL are the boundary.
TLS comes from one Google-managed Certificate Manager certificate for `domains`
(apex and www), attached through a certificate map with an entry per hostname. It
is authorized by the DNS records the `certificate_dns_records` output lists, not by
serving traffic, and those records also renew it. Production without `domains` is
rejected. IAP is off. No Cloud Run custom-domain mapping is created for production.

### Cloud CDN and staging safety

IAP and Cloud CDN cannot be enabled together on one backend. That is one reason
staging uses direct Cloud Run IAP with no backend service.

- **Production:** Cloud CDN is enabled with `USE_ORIGIN_HEADERS`, no forced
  caching and no negative caching. The server emits the cache headers described
  above; the infrastructure does not manufacture application cacheability.
- **Staging:** there is no load balancer or Cloud CDN, and no variable can add
  them. Origin responses are `no-store`, so no upstream layer may cache protected
  content. There is no Cloudflare dependency.

## Checks

Site checks, from the workspace root:

```bash
nix develop --command cargo test --locked -p public-site
nix develop --command cargo test --locked -p public-site --test site -- --ignored --nocapture
nix develop --command node crates/public-site/tests/import-snapshots.mjs --check
nix develop --command mdbook build book
```

Routine CI does not launch browsers. The `site` browser test is ignored by default;
run the explicit command above or manually dispatch the CI workflow for browser
qualification.

Infrastructure checks initialize without the backend (`-backend=false`), so they
need no ADC credentials and make no live GCP API calls. Do not initialize the other
GCP roots from this workspace.

```bash
nix develop --command tofu -chdir=infra/gcp/public-site init -backend=false -input=false
nix develop --command tofu -chdir=infra/gcp/public-site fmt -check -recursive
nix develop --command tofu -chdir=infra/gcp/public-site validate
nix develop --command tofu -chdir=infra/gcp/public-site test
```

The generated `.terraform.lock.hcl` belongs in version control. These checks
validate resource contracts, not cloud permissions, API enablement, IAP brand/OAuth
configuration, group membership, regional quotas, autoscaling behavior, application
health, or a real deployment.

The image smoke builds and runs the image locally:

```bash
nix develop --command docker build --platform=linux/amd64 \
  -f crates/public-site/Dockerfile -t public-site:deployment-check .
nix develop --command python infra/gcp/public-site/smoke.py public-site:deployment-check
```

`smoke.py` runs the image in disposable, locked-down containers three ways: as
production does, with a physician upstream, and in staging mode. It has no signed
positive path: signed allow, key rotation and fetch failure are proven only by Rust
tests with a public test key. It does not replace browser animation/parity tests, a
deep secret scan of embedded source assets, a load test, cloud ingress verification,
or a deployment.

### Local load check

`crates/public-site/load_check.py` starts only disposable loopback listeners. It
neither targets arbitrary URLs nor starts model/data services.

```bash
nix develop --command env -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER \
  -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
  cargo build --locked --release -p public-site --bin public-site
nix develop --command python crates/public-site/load_check.py
```

Output goes under `target/public-site-load/` (`--output` chooses another directory).
The results are closed-loop, fixed-concurrency numbers from the machine that runs
them, with no TLS, CDN, browser or Brotli. They don't establish Cloud Run capacity,
behavior above the request-permit limit, or a single-digit-millisecond p99.

## Staging state and deployment

This root owns public-site staging state. The physician roots under `infra/gcp`
keep their product-repository apply authority; this ownership covers only
`infra/gcp/public-site` with `environment = "staging"`.

- **State.** `backend "gcs" {}` is partial. `backend/staging.gcs.tfbackend`
  selects a dedicated staging bucket. Staging state never lives in a
  `coda-health-prod` bucket, so the CI deployer needs no production-project IAM.
- **Inputs.** `env/staging.tfvars` holds the non-secret staging inputs: the last
  operator-reviewed image digest in the private `public-site` repository, release
  SHA, and `physician_app_url`, the physician preview service in `coda-rust-staging`
  behind its own IAP. Setting it mounts `/physician/`, renders Sign in and gives the
  runtime its proxy signing identity.
- **No Doppler.** The running site and its deploy read no Doppler config and no
  secret; these files and the CI inputs are the only deploy values.

### Staging admin bootstrap

`infra/gcp/public-site/scripts/staging-admin-bootstrap.sh` is run by an
administrator of `coda-health-staging`, not by the deployer. It prepares only
what the root deliberately does not own: APIs, the state bucket, the
[private repositories](#private-registries-only) and their base images, the keyless
CI deployer with resource-scoped roles, and GitHub Workload Identity Federation
limited to this repository's `main` branch and `staging` environment. Each create
step checks first, so it is safe to rerun. It prints every command and never
touches `coda-health-prod`. Review it with `--print` first; that mode makes no
gcloud calls.

```bash
# From the workspace root, inside the dev shell, as the staging admin:
nix develop
gcloud auth login                     # the admin's account
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --print
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh   # needs docker (buildx)
# OPTIONAL, only if DEPLOYER_USER should run a first apply (expires in 24 h):
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --temporary-deployer-grant
# After a first apply, rerun once to add the service-scoped CI bindings:
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh
```

`--physician-proxy` also creates the [proxy signing role](#physician-entry-point) and
binds it runtime-to-itself. Expired temporary grants grant nothing; remove them
afterwards with `remove-iam-policy-binding ... --all`.

CI gets no `iam.serviceAccountAdmin` (it never creates or edits the runtime
account: a first apply is a human apply), no project-wide `run.admin`,
`iap.admin` or `actAs`, and no `run.invoker` or `iap.httpsResourceAccessor`, so
it has no website access. The service-scoped grants need the service and runtime
account to exist; the script skips them until then and adds them on a rerun.
Tofu's authoritative bindings cover only `roles/run.invoker` and
`roles/iap.httpsResourceAccessor`, so they do not remove these grants.

A Google Workspace or Cloud Identity admin can confirm the IAP group exists and
whether `DEPLOYER_USER` is in it (read-only):

```bash
bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --check-group
```

### CI deploy on push to main

The `deploy-public-site-staging` job in
[`.github/workflows/ci.yml`](/.github/workflows/ci.yml) redeploys staging from
pushes to `main` selected for full validation, after `CI result` passes.
Engineering-skills/docs-only pushes skip deployment. It runs only when the repository variable
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

CI and deployment identities have no website-access grant, so the job checks only
that anonymous requests are sent to sign-in; an authenticated render is a manual
check by a member of `gcp-engineering@codahealth.ai`.

`env/staging.tfvars` keeps the last operator-reviewed digest; a local plan run
from it after a CI deploy shows the image going back. Pass the current digest
with `-var image=...` (read it from `gcloud run services describe`) when
planning by hand. A plan that changes anything beyond the service, such as IAM or
the domain mapping, stops at the CI guard and needs an operator apply from a
reviewed saved plan.

### Operator apply

Run from the workspace root as the staging admin, or as `DEPLOYER_USER` while a
temporary grant is valid:

```bash
nix develop
export GOOGLE_OAUTH_ACCESS_TOKEN="$(gcloud auth print-access-token --project=coda-health-staging)"
tofu -chdir=infra/gcp/public-site init -input=false -reconfigure \
  -backend-config=backend/staging.gcs.tfbackend
tofu -chdir=infra/gcp/public-site plan -input=false \
  -var-file=env/staging.tfvars -var image=CURRENT_DIGEST_REF -out=staging.tfplan
# Review the saved plan, then apply only that plan; never apply production here.
tofu -chdir=infra/gcp/public-site apply staging.tfplan
tofu -chdir=infra/gcp/public-site output staging_url
```

After a first apply, rerun the bootstrap so CI gets its service-scoped roles. A new
staging environment is live only when a member of the engineering group is allowed,
a nonmember denied, an unauthenticated direct `run.app` request redirected to
sign-in, and no protected content served from a cache, on both
`staging.codahealth.ai` and the `run.app` URL.

## Production state and deployment

Production serves the image staging tested, without sign-in, at `codahealth.ai`
and `www.codahealth.ai` from `coda-health-prod`. Operator applies and DNS changes
need Dev's explicit authorization.

- **State.** `backend/production.gcs.tfbackend` selects a dedicated state bucket,
  as staging has its own. The deployer has no access to the shared production state
  bucket: Cloud Storage cannot narrow object listing to a prefix with an IAM
  condition.
- **Inputs.** `env/production.tfvars` holds the non-secret production inputs: the
  project and number, and `domains`. It sets no `physician_app_url`, so production
  has no Sign in link and no `/physician/` mount. `image` and `release_sha` are
  supplied per release: the digest staging tested and its commit.
- **DNS.** In the `codahealth.ai` Cloudflare zone, `codahealth.ai` and
  `www.codahealth.ai` are DNS-only A records to the root's `load_balancer_ip`. The
  `_acme-challenge` CNAMEs from `certificate_dns_records` stay in place: Certificate
  Manager renews the certificate through them. The zone's other records belong to
  other owners.

### Production admin bootstrap

`scripts/production-admin-bootstrap.sh` prepares what the root does not own: APIs,
the state bucket, the private image repository, the keyless
`public-site-prod-deployer` account, and a Workload Identity Federation provider
accepting only `Coda-Health/coda-rust` on `refs/heads/main` in the `production`
environment. An administrator of `coda-health-prod` runs it with Dev's authorization;
one step needs an administrator of `coda-health-staging`. `--print` shows every
command without calling gcloud. It is safe to rerun, and a first apply is followed by
a rerun for the bindings that need the service and runtime account to exist.

The deployer gets no compute, certificate or IAM admin role: promotion cannot change
the load balancer, TLS or identities, only clear the CDN. The GitHub `production`
environment admits only `main` and has no required reviewer.

### Promotion

[`promote-public-site.yml`](/.github/workflows/promote-public-site.yml) is manual
(`workflow_dispatch` on `main`, input `release_sha`) and runs in the `production`
environment. Starting a run is the approval: anyone with write access to the
repository can promote a commit staging already built and tested. It:

1. requires a full SHA that is on `main` and a successful "Deploy public site to
   staging" job for that push, so the image was deployed and verified on staging,
   not only built. Skills/docs-only pushes skip that job, so promote the latest
   commit staging actually deployed;
2. finds the image the staging deploy tagged with that commit (tags are immutable)
   and copies that digest into the production repository unchanged;
3. plans the production root with `env/production.tfvars`, that image and that SHA,
   and fails if the plan changes anything but an in-place update of the Cloud Run
   service. Load balancer, certificate, IAM and identity changes are operator applies;
4. applies the saved plan and checks the Ready revision serves the digest with all
   traffic;
5. clears the CDN and waits for it to finish;
6. pins `codahealth.ai` and `www.codahealth.ai` to the load balancer IP with
   `curl --resolve` and checks each: `/api/health` reports production and the SHA,
   the homepage sends production's HSTS and offers no Sign in, `/physician/today` is
   404, and plain HTTP answers 308 to the same HTTPS URL.

The CDN is cleared because its cached HTML names hashed CSS and JS the new image no
longer serves, which would load unstyled pages. Clearing the CDN does not clear
browsers, so a returning visitor may see one unstyled load before the next load is
current.

### Recovery

Recovery is a promotion of an earlier staging-tested commit: run the promotion
workflow with that commit's SHA.

### Rebuilding production

Each step needs Dev's authorization at the time it runs.

1. Run the [production admin bootstrap](#production-admin-bootstrap).
2. Copy the staging-tested digest into the production repository as the workflow
   does, then, as an administrator:

   ```bash
   tofu -chdir=infra/gcp/public-site init -backend-config=backend/production.gcs.tfbackend
   tofu -chdir=infra/gcp/public-site plan -var-file=env/production.tfvars \
     -var image=IMAGE@DIGEST -var release_sha=SHA -out=production.tfplan
   tofu -chdir=infra/gcp/public-site apply production.tfplan   # after review
   ```

   Rerun the bootstrap for the bindings that need the service and runtime account.
3. Add the `certificate_dns_records` CNAMEs in Cloudflare, DNS only, and wait until
   `gcloud --project=coda-health-prod certificate-manager certificates describe
   public-site-production` reports the certificate `ACTIVE`.
4. Run the promotion workflow for the same SHA. Its plan is a no-op and its readback
   checks both hostnames through the load balancer before DNS points there. Also
   look at the site in a browser pinned to the load balancer IP.
5. Point the apex and `www` A records at `load_balancer_ip`, DNS only, and check
   public DNS answers and the live responses.
