# Observability (Datadog or Grafana, plus self-hosted Langfuse)

This root deploys the observability stack into the private observability
Autopilot cluster that [`infra/gcp/rust-environments`](/infra/gcp/rust-environments)
creates behind `observability_cluster_enabled`. The target is variable-driven
(`cluster_project`, `cluster_location`, `cluster_name`, `network_name`, all
required): this root reads the cluster and the `rust-private` VPC through data
sources and never manages them, their subnets or the gateway address. The
data flow, content policy, and runbook are in
[book/src/observability.md](/book/src/observability.md).

| Part | What it is |
| --- | --- |
| `collector.tf` | Two-replica OpenTelemetry Collector (`otel/opentelemetry-collector-contrib:0.155.0`) running [`dev/observability/collector.yaml`](/dev/observability/collector.yaml) merged with the gateway overlay [`collector.gke.yaml`](/dev/observability/collector.gke.yaml) and, last, the backend overlay `collector.<telemetry_backend>.yaml`, all unchanged from one ConfigMap. Only the selected backend's endpoints are set (`DATADOG_OTLP_ENDPOINT`, or `TEMPO_OTLP_ENDPOINT` and `LOKI_OTLP_ENDPOINT`). A ClusterIP Service serves in-cluster callers; the `otel-gateway` internal load balancer serves Cloud Run |
| `datadog.tf` | With `telemetry_backend = "datadog"` or `"both"`: Datadog Operator Helm chart `2.30.0`, one `DatadogAgent` (`gke-autopilot` provider, APM, OTLP on 4317/4318 with OTLP logs on, `env:<environment>`), and the `datadog_monitor`s behind `datadog_monitors_enabled`: 5xx rate and p95 latency per service, physician chat generation errors, and asynchronous failures for physician-worker and duet-api, plus memory near the limit and OOM kills for this namespace's containers, all notifying `var.alert_notify` (staging: Slack `#engineering-observability`) |
| `datadog_gcp.tf` | Off by default (`datadog_gcp_integration_enabled`, `datadog` only): the Datadog GCP integration for `cluster_project` only, through service-account impersonation |
| `grafana.tf` | With `telemetry_backend = "grafana"` or `"both"`: one-replica StatefulSets with persistent disks for Tempo `3.1.1`, Loki `3.7.9`, Prometheus `v3.16.0` and Grafana `13.3.0`, each behind a ClusterIP Service, reading the shared files under [`dev/observability/grafana`](/dev/observability/grafana) from ConfigMaps: Tempo's metrics-generator, Loki's OTLP ingestion, the datasources (`tempo`, `loki`, `prometheus`) and the four operational alert rules in shadow mode. Retention: `grafana_traces_retention_days` (14), `grafana_logs_retention_days` (30), `grafana_metrics_retention_days` (30). Off by default (`grafana_cloud_monitoring_enabled`): a Cloud Monitoring datasource read through Workload Identity by a `roles/monitoring.viewer` service account |
| `langfuse.tf` | `langfuse/langfuse` Helm chart `1.5.41` (Langfuse `3.225.11`), Cloud SQL Postgres 16 with IAM login through a Cloud SQL Proxy sidecar, a GCS blob bucket through S3 interop, in-cluster ClickHouse and Valkey on PVCs, ClusterIP only. Under `staging_proxy` web runs the digest-pinned `/langfuse` base-path build (`langfuse_web_image`) |
| `langfuse-sso.tf` | Off by default (`langfuse_sso_image`): one replica of the [`langfuse-sso`](/crates/langfuse-sso) OIDC issuer and its ClusterIP Service, which sign people into Langfuse as their IAP-verified Google account (see [Staging proxy](#staging-proxy)) |
| `web.tf` | Off by default (`observability_web`): internal passthrough load balancers for `grafana:3000`, `langfuse-web:3000` and, with the issuer, `langfuse-sso:3000` on the reserved addresses, admitting only the relay subnet. People reach them only through public-site's `/grafana/`, `/langfuse/` and `/langfuse-sso/` mounts (see [Staging proxy](#staging-proxy)) |
| `doppler.tf` | Doppler Kubernetes Operator `1.7.3` and one `DopplerSecret` per consuming Kubernetes Secret |
| `charts/manifests` | A local chart that renders custom resources (DatadogAgent, DopplerSecret) so they apply after their operators' CRDs exist |

The default plan deploys the full stack with `telemetry_backend = "datadog"`,
`CODA_TELEMETRY_CONTENT=none`, no gateway, and a 90-day Langfuse retention
(`var.langfuse_retention_days`, used as the blob lifecycle and recorded for
the project's data-retention setting).

`langfuse_enabled = false` deploys only the collector gateway, the selected
backend and the Doppler operator. Nothing from `langfuse.tf` is planned, no
Langfuse key is synced from Doppler, and the gateway merges
[`collector.gke-no-langfuse.yaml`](/dev/observability/collector.gke-no-langfuse.yaml)
before the backend overlay, sending generation spans to `nop`. Staging sets
`langfuse_enabled = true`. Its apply needs an identity with
`resourcemanager.projects.setIamPolicy` for the project-level Cloud SQL grants,
the `LANGFUSE_*` keys from [Secrets](#boundary) in Doppler, and an HMAC key
(`storage.hmacKeys.create`) for `coda-langfuse-blobs`, which exists only after
this root creates it. Enabling Langfuse does not change the operations path.

## Switching the backend

`telemetry_backend` selects `datadog` (the compatibility default), `grafana`,
or `both`. Concurrent mode retains both resource sets and fans sanitized traces
and logs into independent bounded, nonblocking exporter queues (1000 batches,
5-second requests, 30-second retry budgets). Overflow drops telemetry rather
than blocking the product; queues are not durable across collector restarts.
Langfuse keeps its independent generation pipeline. `rust-environments` has a variable of
the same name that sets `CODA_TELEMETRY_BACKEND` on the product containers,
and both roots must carry the same value. The Datadog Helm releases moved to
indexed addresses (`moved` blocks keep an applied Agent in place), so the
default plan does not recreate them. To switch to `grafana`:

1. Add `GRAFANA_ADMIN_PASSWORD` to the Doppler config this root syncs from
   (`coda-rust/stg`). A Slack webhook is only required for the separately
   approved synthetic notification test route.
2. Set `datadog_monitors_enabled = false` and
   `datadog_gcp_integration_enabled = false`; variable validation refuses
   either with `grafana`. Use `grafana_cloud_monitoring_enabled` for platform
   metrics instead.
3. Set `telemetry_backend = "grafana"` in both roots' tfvars. Under `grafana`
   the Datadog provider skips key validation, so no Datadog keys are needed.
4. Plan both roots, save the plans, and have them reviewed: this root should
   destroy the Datadog Agent, operator, monitors and `datadog-secret`, add the
   Grafana StatefulSets, Services, ConfigMaps and `grafana-secrets`, and roll
   the collector with `collector.grafana.yaml`; `rust-environments` should
   change only `CODA_TELEMETRY_BACKEND` on the product containers. The operator
   applies the saved plans with authorization, this root first so the
   collector exports to Tempo and Loki before log lines drop their `dd.*` keys.
5. Reach Grafana with
   `kubectl -n observability port-forward svc/grafana 3000` (user `admin`,
   the Doppler password), check that the four rules under the `Coda` folder are
   provisioned, and confirm one trace in Tempo, its log in Loki and its span
   metrics in Prometheus.

Switching back is the same procedure in reverse with `telemetry_backend =
"datadog"`. Destroying the StatefulSets leaves their PersistentVolumeClaims
(`data-tempo-0`, `data-loki-0`, `data-prometheus-0`, `data-grafana-0`) in the
namespace; deleting them, and the stored telemetry, is a separate operator
step.

## Concurrent shadow qualification

Use `both` in both roots to keep Datadog ingestion and its independently gated
primary notifications. JSON logs retain W3C identifiers and Datadog aliases.
Grafana operational rules match the always-muted `coda-shadow` child route:
they evaluate without duplicate Slack paging. Grafana requires a receiver, so
this route references a synthetic `.test` email receiver with no SMTP configured. `grafana_test_notifications_enabled`
is default-off and adds only a route requiring both `qualification=synthetic-notification-test`
and `alertname=Coda synthetic notification test`. Its webhook is synced from
Doppler only when enabled. Creating or firing that dedicated test rule and
confirming Slack receipt require separate operator approval. Never relabel a
service alert to test notifications. The external watchdog must remain outside
Grafana and GKE; shadow evaluation is not monitoring-loss protection.

`datadog_qualification_monitor_enabled` defaults off. When it is on, Datadog
evaluates `coda.qualification.synthetic_readiness{env:staging}` with the
readiness thresholds (critical 1, recovery 0.5), tags the monitor
`qualification:synthetic`, and notifies the confirmed handle. It does not
enable the service monitors. `grafana_test_notifications_enabled` also
provisions `alerting/synthetic-rules.yaml`. That rule is titled
`Coda synthetic notification test`, carries only
`qualification=synthetic-notification-test`, and counts Loki lines matching
`{qualification="synthetic-notification-test"} |= "fail"`. With the gate off,
provisioning deletes that rule. Operational rules stay in `rules.yaml` on the
shadow route.

`workspace-tools observability-drill` takes `datadog`, `grafana`, or
`watchdog` and `fail`, `recover`, or `status`. Each run writes observation
JSON for `qualify-alerts.py --observations`. A run stops at 40 minutes and
two reminders, and restores the signal on abort. Datadog reads `DD_API_KEY`
from Doppler project `coda-rust` config `stg` and does not print it. Grafana
pushes through a local Loki port-forward. Watchdog pauses or resumes the
`availability-probe` scheduler and does not add an evaluator.

Cloud Monitoring is a default-off datasource through Workload Identity
(`grafana_cloud_monitoring_enabled`), not through Tempo span metrics. Enabling it
also provisions native PromQL shadow rules from `grafana-cloud-rules.tf` for the
three probe targets, the two product databases, the physician Redis instance,
and worker subscriptions. Readiness uses the probe's durable firing metric,
which owns three-failure and two-success hysteresis. Missing readiness data
retains the previous alert state. Separate rules detect missing or stale probe
observations and 20-minute absence of continuous dependency signals. Sparse
queues have no absence rules. Resource selectors are fixed to the runtime
infrastructure's resource names and the selected cluster project. Dependency
thresholds and sustained durations share the Datadog signal catalog.

Disabling the datasource explicitly deletes its rules and datasource from
persistent Grafana state. Native rules remain on the muted operational route.
Qualify actual ingestion delays and Workload Identity reads before relying on
these rules. Provisioning alone does not prove successful evaluation.

An optional internal Prometheus-compatible source can be supplied with
`grafana_compatibility_metrics_url`; empty means no datasource or rules. Its
adapter contract is one series per expected target per minute, including idle
periods, with only `environment`, `service`, `target`, `dependency` and `symptom`
labels from fixed inventories:

- `coda_compat_readiness_consecutive_failures`: integer counter of consecutive
  failed probes (including authentication failures); reset only after two
  consecutive successes. The shadow rule tests three failures.
- `coda_compat_dependency_unhealthy`: 0 for healthy/idle, 1 for unhealthy or
  overdue work, based on verified source metrics and product retry budgets.
  The rule requires three minutes of continuous failure.
- `coda_compat_last_observed_seconds`: last successful observation timestamp
  per expected target/signal; a five-minute lag fires. Missing series produce
  NoData, not success. Per-target disappearance must be represented by the
  adapter retaining its expected inventory and last-observed value.

These are explicit compatibility inputs, not a claim that any probe or cloud
producer emits them. Qualify the adapter's real metric names, units, labels,
authentication and missing-data behavior before enabling this source. Never use
span-derived traffic metrics as proof of dependency availability.

Before authorizing a saved plan, review the additional single-replica requests:
Tempo 500m/2Gi and 50Gi disk, Loki 500m/1Gi and 50Gi, Prometheus 250m/1Gi and
20Gi, Grafana 250m/512Mi and 5Gi. Total requested capacity is 1.5 CPU, 4.5Gi
memory and 125Gi storage, in addition to Datadog/collector/Langfuse. Autopilot
may adjust requests; price the actual saved-plan allocation, ingestion and
retention, not these requests alone. Validate non-root volume access, RWO
reattachment, retention cleanup and restart persistence with synthetic data.
Retention defaults (14/30/30 days) do not establish that volumes can hold the
expected load. Rollback to `datadog` retains the Grafana PVCs; deleting telemetry
requires separate approval. No deployment or durability claim follows from
local collector tests or mocked infrastructure plans.

## Inputs from rust-environments

Apply `rust-environments` with `observability_cluster_enabled = true` first.
Its outputs become this root's inputs; [`env/staging.tfvars`](env/staging.tfvars)
holds the stable names, and the operator renders the rest from the outputs:

| This root | `rust-environments` output |
| --- | --- |
| `cluster_project`, `cluster_location`, `cluster_name`, `network_name` | `observability_cluster.{project,location,name,network}` |
| `otel_gateway_ip` | `otel_gateway.ip` (reserved `INTERNAL` address in the node subnet) |
| `otel_gateway_audience` | `otel_gateway.audience` |
| `otel_gateway_source_ranges` | `otel_gateway.source_ranges` (the Cloud Run Direct VPC egress subnet `rust-run`) |
| `otel_gateway_allowed_callers` | `otel_gateway.allowed_callers` (the physician, duet and physician-worker runtime service accounts) |
| `observability_web` | `observability_relay.{grafana_ip,langfuse_ip,langfuse_sso_ip,source_ranges}` (the three reserved addresses and the relay subnet `rust-relay`) |

```sh
root=infra/gcp/observability
tofu -chdir=infra/gcp/rust-environments output -json otel_gateway \
  | jq '{otel_gateway_ip: .ip, otel_gateway_audience: .audience,
         otel_gateway_source_ranges: .source_ranges,
         otel_gateway_allowed_callers: .allowed_callers}' > /tmp/otel-gateway.tfvars.json
tofu -chdir=infra/gcp/rust-environments output -json observability_relay \
  | jq '{observability_web: {grafana_ip, langfuse_ip, langfuse_sso_ip, source_ranges}}' > /tmp/observability-web.tfvars.json
tofu -chdir=$root init -backend-config=backend/staging.gcs.tfbackend
doppler run --project coda-rust --config stg -- \
  tofu -chdir=$root plan -var-file=env/staging.tfvars \
  -var-file=/tmp/otel-gateway.tfvars.json \
  -var-file=/tmp/observability-web.tfvars.json -out=observability.tfplan
```

`environment` has no default and accepts only `staging`; production
observability needs its own design and authorization. The kubernetes and helm
providers reach the private control plane through the cluster's DNS endpoint,
which authorizes by the operator's IAM identity, so no bastion or authorized
network is needed.

## Cloud Run gateway

The Cloud Run services cannot reach a ClusterIP Service. Each one runs an
OpenTelemetry Collector sidecar ([`dev/observability/sidecar.yaml`](/dev/observability/sidecar.yaml),
added by `rust-environments`) that forwards OTLP/HTTP to the `otel-gateway`
Service in this root:

- **Transport.** A GKE internal passthrough Network Load Balancer
  (`networking.gke.io/load-balancer-type: Internal`) on the reserved
  `otel_gateway_ip`, port 4318 only. Traffic never leaves the VPC. Google
  encrypts VPC traffic between its hosts in transit; the receiver itself is
  plain HTTP and this root adds no receiver TLS.
- **Network restriction.** `loadBalancerSourceRanges` is
  `otel_gateway_source_ranges`, which GKE turns into the VPC firewall rule for
  the load balancer; only the Cloud Run egress subnet can connect. The
  variable refuses `0.0.0.0/0` and anything wider than `/16`.
- **Authentication.** Every OTLP request must carry a Google-signed ID token
  whose audience is `otel_gateway_audience`. The sidecar's `googleclientauth`
  extension mints it from the service's own runtime identity through the
  metadata server; the gateway's `oidc` extension verifies issuer
  `https://accounts.google.com`, signature, expiry and audience.
- **Authorization.** `OTEL_GATEWAY_ALLOWED_CALLERS` (the comma-joined
  `otel_gateway_allowed_callers`) lists the only token `email` claims whose
  spans and log records the overlay keeps. A valid token from any other
  Google identity is accepted at the receiver, then its data is dropped.
- **Rotation.** Nothing to rotate by hand: Google mints short-lived ID tokens
  and rotates its signing keys, and no shared secret exists in Doppler, Secret
  Manager or state. Changing the audience is a coordinated apply of both roots.
- **Outage.** If the gateway is down, the sidecar retries for a bounded time,
  then drops; the product container never waits on it.

## Staging proxy

People open Grafana at `https://staging.codahealth.ai/grafana/` and Langfuse at
`https://staging.codahealth.ai/langfuse/`. Neither app has a listener of its
own on the internet:

```text
browser -> Cloud Run IAP (public-site-staging, gcp-engineering)
        -> public-site /grafana/, /langfuse/ and /langfuse-sso/ mounts
        -> Cloud Run IAP (observability-relay: the group and public-site's account)
        -> observability-relay, Direct VPC egress on rust-relay
        -> internal LB grafana:3000, langfuse-web:3000 or langfuse-sso:3000
           (this root, web.tf)
```

- **Path.** public-site ([book/src/public-site.md](/book/src/public-site.md))
  sends a JWT signed for its own service account to the relay as
  `Proxy-Authorization`, as it does for `/physician/`.
  `infra/gcp/rust-environments` owns the relay, its subnet and the
  reserved addresses. The load balancers here admit only
  `observability_web.source_ranges`, the relay's `/26`, so the physician and
  Duet services on `rust-run` cannot reach either app. The variable refuses
  ranges wider than `/24`. Neither app rewrites paths: Grafana serves from
  `/grafana` (`GF_SERVER_SERVE_FROM_SUB_PATH`) and Langfuse from `/langfuse`.
- **Grafana sign-in.** public-site verifies the person's IAP assertion and
  drops any copy of `X-Coda-IAP-Assertion` that the client sent. It then
  forwards the verified assertion in that header. Grafana trusts only that
  header. It checks the signature against Google's IAP keys
  (`https://www.gstatic.com/iap/verify/public_key-jwk`), the issuer
  `https://cloud.google.com/iap`, and the audience
  `staging_proxy.iap_audience`, which is public-site's
  (`infra/gcp/public-site` output `iap_audience`). Then it signs the person in
  by `email`, creating the account as an Editor on the first visit. An
  assertion for another IAP application is refused. The Doppler `admin` login
  remains as break-glass. Live is off (`GF_LIVE_MAX_CONNECTIONS = 0`) because
  the proxy carries no WebSockets.
- **Langfuse sign-in.** With `langfuse_sso_image`, opening `/langfuse/` signs
  the person in as the Google account IAP verified, with no form and no
  click. Langfuse cannot read IAP, so `langfuse-sso.tf` runs
  [`crates/langfuse-sso`](/crates/langfuse-sso), a small OIDC issuer, as
  Langfuse's generic OIDC provider (`AUTH_CUSTOM_*`, provider `custom`):

  ```text
  browser GET /langfuse/ without a session
    -> public-site 303 /langfuse/auth/sso-initiate?provider=custom&target=...
    -> Langfuse 302 <origin>/langfuse-sso/authorize (PKCE, state, nonce)
    -> public-site forwards X-Coda-IAP-Assertion -> relay -> langfuse-sso LB
    -> langfuse-sso 302 /langfuse/api/auth/callback/custom?code&state
    -> Langfuse exchanges the code and reads the JWKS in-cluster
       (svc/langfuse-sso), verifies the ES256 ID token, sets its session
  ```

  The issuer takes the identity only from `X-Coda-IAP-Assertion` for
  `staging_proxy.iap_audience`, so a request that skips public-site carries
  nothing it accepts. Its discovery document names the in-cluster Service as
  issuer, token and JWKS endpoint, and `<origin>/langfuse-sso/authorize` as
  the only browser endpoint; public-site serves nothing else under
  `/langfuse-sso` and lets its redirects reach only the Langfuse callback.
  Codes are single-use, expire after a minute and live in the one replica's
  memory. With `langfuse_sso` set in the public-site root
  (`LANGFUSE_SIGN_IN=sso`), public-site mounts the authorize step and sends
  page navigations without a session, and the sign-in page itself, into
  `sso-initiate`. That setting must match `langfuse_sso_image` here: without
  the issuer, Langfuse offers no `custom` provider to send people to.

  A first sign-in creates the user in the `coda` organization with the
  `NONE` role and no project (`LANGFUSE_DEFAULT_ORG_ROLE`), so a new
  engineer sees no traces until an admin grants a role.
  `LANGFUSE_ALLOWED_ORGANIZATION_CREATORS` names the init admin
  (`LANGFUSE_INIT_USER_EMAIL`), but Langfuse enforces it only with an EE
  license; without one an engineer can still create an empty organization of
  their own, which holds none of `coda`'s traces. Sign-up is on only
  with the issuer, because each first sign-in creates a user. Account
  linking (`AUTH_CUSTOM_ALLOW_ACCOUNT_LINKING`) lets the init admin's user
  sign in the same way and keep its role. Passwords are off with the issuer
  (`AUTH_DISABLE_USERNAME_PASSWORD`), on every route including port-forward:
  with sign-up on, a password route would let anyone behind IAP register,
  or claim an engineer's email before their first SSO sign-in and keep
  access after linking. When a session expires while a page is open,
  Langfuse moves to its sign-in page in the browser, without a request
  public-site sees, so its SSO button shows once; a reload signs the person
  in again.

  Without `langfuse_sso_image` Langfuse keeps only its own login, sign-up is
  off and the admin adds people. That is also the break-glass when the
  issuer fails: set `langfuse_sso_image` to null here and `langfuse_sso` to
  false in public-site, apply both, and the init admin signs in by password. Either way, public-site passes through only
  the NextAuth cookies (`__Secure-next-auth.*`, `__Host-next-auth.csrf-token`).
  `nextauth.url` is `<origin>/langfuse/api/auth`, and the liveness and
  readiness probes and the collector's in-cluster OTLP endpoint carry the
  `/langfuse` prefix.
- **Langfuse image.** Next.js fixes the base path when it builds, so the stock
  web image cannot serve `/langfuse`.
  [`.github/workflows/langfuse-web-image.yml`](/.github/workflows/langfuse-web-image.yml)
  builds upstream `v3.225.11` with `NEXT_PUBLIC_BASE_PATH=/langfuse` and
  pushes it to the staging `apps` repository. It runs when a change to the
  workflow lands on `main`, because the staging identity provider admits only
  push tokens from `main`. The run's summary prints the digest, which becomes
  `langfuse_web_image`. The worker keeps the stock image. The observability
  nodes may read that repository (`rust-environments`). When
  `langfuse_image_tag` changes, update the pinned tag and commit in the
  workflow in the same change; merging it builds the new image.

### Staging proxy rollout

Each apply is from a reviewed saved plan with the operator's authorization.

1. Langfuse runs with its Doppler keys and HMAC pair.
2. Merging this workflow runs the Langfuse web image build; record its digest.
3. Apply `rust-environments` with `observability_relay_image`, then release
   the relay image. Its `observability_relay` output feeds `observability_web`
   here and `observability_relay_url` in `infra/gcp/public-site`.
4. Set `staging_proxy = { origin = "https://staging.codahealth.ai",
   iap_audience = <public-site iap_audience> }` and `langfuse_web_image` in
   `env/staging.tfvars`. Plan and apply this root with the
   `observability_web` var-file.
5. Apply `infra/gcp/public-site` with `observability_relay_url`, then release
   public-site.
6. Proof:
   - `/grafana/` signs you in as your Google account.
   - `/langfuse/` shows the Langfuse login, and the admin can sign in
     (until [Langfuse sign-in rollout](#langfuse-sign-in-rollout)).
   - A request that skips public-site's IAP is refused.
   - The relay URL, called directly without public-site's service account, is refused.
   - `/physician/` still works.

### Langfuse sign-in rollout

Each apply is from a reviewed saved plan with the operator's authorization.

1. Seed Doppler `coda-rust/stg` with `LANGFUSE_SSO_CLIENT_SECRET` (at least
   32 random bytes) and `LANGFUSE_SSO_SIGNING_KEY` (a P-256 key as standard
   base64 of its PKCS#8 DER):

   ```sh
   openssl rand -base64 48 | tr -d '\n' \
     | doppler secrets set LANGFUSE_SSO_CLIENT_SECRET --project coda-rust --config stg
   openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 \
     | openssl pkcs8 -topk8 -nocrypt -outform DER \
     | base64 -w0 \
     | doppler secrets set LANGFUSE_SSO_SIGNING_KEY --project coda-rust --config stg
   ```

   `genpkey -outform DER` alone writes SEC1, which the issuer refuses at
   startup.
2. Apply `rust-environments`. Its `observability_relay` output gains
   `langfuse_sso_ip`, and the relay gets `LANGFUSE_SSO_UPSTREAM`. The
   relay treats that setting as optional and answers `/langfuse-sso` with 404
   until it is set, so a staging release can land before or after this
   apply.
3. Set `langfuse_sso_image` in `env/staging.tfvars` to the `coda-langfuse-sso`
   digest the staging release published, render `observability_web` with
   `langfuse_sso_ip`, then plan and apply this root.
4. Set `langfuse_sso = true` in the public-site root's
   `env/staging.tfvars`, then plan and apply it.
5. Proof: an engineer opens `/langfuse/` and lands signed in, and their
   membership in `coda` has role `NONE`; the admin signs in the same way and
   keeps its role; a password sign-in or sign-up is refused;
   `/langfuse-sso/authorize` without public-site's assertion is refused.

Rotating the client secret or signing key is a new Doppler value; the
operator resyncs both Secrets within five minutes, then
`kubectl -n observability rollout restart deploy/langfuse-sso
deploy/langfuse-web` picks them up. Existing sessions stay valid. Null
`langfuse_sso_image` removes the issuer and its load balancer, turns sign-up
off and passwords back on; set public-site's `langfuse_sso` to false first so
it stops sending people to the issuer.

Removing `staging_proxy` and `observability_web` deletes both load balancers
and returns both apps to their root paths for port-forward. The Langfuse base
path changes only together with its image.

## Boundary

- **No public listener; people come through the staging proxy.** The
  Langfuse chart's Ingress is off and every Service is `ClusterIP`, including
  Tempo, Loki, Prometheus and Grafana. The only other listeners are the
  internal load balancers in `web.tf`. They admit only the relay subnet, so
  the way in from outside is public-site's IAP, then the relay's IAP
  ([Staging proxy](#staging-proxy)). Without `observability_web`, people reach
  the UIs with
  `kubectl -n observability port-forward svc/langfuse-web 3000:3000` or
  `kubectl -n observability port-forward svc/grafana 3000`. This root
  creates no external Gateway and no IAP backend of its own.
- **Content never reaches the operations backend.** The collector's
  `traces/ops` and `logs/ops` pipelines delete `gen_ai.prompt*`,
  `gen_ai.completion*`, `gen_ai.{input,output}.messages`, and
  `langfuse.{observation,trace}.{input,output}` from span and span event
  attributes and from log record and resource attributes, whatever the
  service's content setting and whichever backend overlay is merged. Log
  records never go to Langfuse. The Datadog Agent tails no container logs
  (`containerCollectAll = false`) and excludes this namespace's container
  logs, because Langfuse can log payloads on errors; it accepts OTLP logs only
  from the gateway. Under `grafana`, only the gateway writes to Tempo and
  Loki, and nothing collects container logs.
- **Cluster memory alerts.** On Autopilot, Google sizes and replaces the nodes,
  and its balloon pods reserve spare node memory, so host memory such as
  `system.mem.pct_usable` stays low on healthy nodes and is not ours to act on.
  Under `datadog` with `datadog_monitors_enabled`, `datadog.tf` instead watches
  this namespace's containers: `kubernetes.memory.working_set` above
  `container_memory_limit_ratio` (0.9) of `kubernetes_state.container.memory_limit`
  for every sample in 10 minutes, and any container terminated with `OOMKilled`. Both group
  by `kube_container_name`, which survives node and pod replacement, wait
  `new_group_delay` (300 s) before judging a new group, and resolve a group
  after `timeout_h` (1 h) without data. The Datadog default host monitors (such
  as "Memory space is low") are not managed here. On an Autopilot account they
  alert on every node and leave retired hosts in Alert; mute or delete them in
  Datadog with operator approval.
- **Grafana alerts.** `rules.yaml` provisions the same four alerts as the
  Datadog monitors, with fixed thresholds (Grafana provisioning has no
  defaults for unset variables). A precondition on the Grafana ConfigMap
  fails the plan unless they equal `site_5xx_rate_threshold`,
  `chat_generation_error_threshold`, `async_failure_threshold` and
  `service_p95_latency_threshold_seconds`; change the file and the variables
  together. Every rule routes to the `coda-slack` contact point, which posts to
  `GRAFANA_SLACK_WEBHOOK_URL`.
- **GCP metrics.** With `grafana_cloud_monitoring_enabled` (`grafana` only),
  `grafana.tf` creates `grafana-cloud-monitoring@<cluster_project>` with only
  `roles/monitoring.viewer` on that project, binds the `grafana` Kubernetes
  service account to it through Workload Identity, and provisions a Cloud
  Monitoring datasource. No key is created. With
  `datadog_gcp_integration_enabled` (`datadog` only), `datadog_gcp.tf`
  creates `datadog-gcp-integration@<cluster_project>` with
  `roles/browser`, `roles/cloudasset.viewer`, `roles/compute.viewer` and
  `roles/monitoring.viewer` on that project only, grants Datadog's STS
  delegate (`delegate_account_email`) `roles/iam.serviceAccountTokenCreator` on
  that one account, and registers `datadog_integration_gcp_sts`. No key is
  created. Resource, org/folder, change, CSPM and Security Command Center
  collection stay off. Cloud Run import is limited to
  `datadog_gcp_cloud_run_services`, GCE import to this cluster's nodes, and the
  project's other metric namespaces (`datadog_gcp_disabled_metric_namespaces`)
  are disabled by id because the integration has no namespace allowlist.
  Confirm those ids against the integration's namespace list in Datadog before
  enabling. The Datadog provider's app key must belong to a user allowed to
  manage integrations. `datadog_gcp_dependency_collection` is a separate,
  empty-by-default opt-in for SQL, Redis and Pub/Sub namespaces. Each entry
  supplies verified exact resource filters to the resource configuration.
  Namespace metric-name filters stay empty, importing all metrics in each
  opted-in namespace for matching resources; resource tags are not metric-name
  filters. Other exclusions, global-location collection and IAM stay unchanged.
  Inspect resource-filter effectiveness and the projected series volume of all
  namespace metrics before authorizing collection.
- **Dependency and storage monitors.** `dependency-monitors.tf` owns scoped
  SQL, Redis and queue rules; `workload-monitors.tf` owns stable deployment,
  StatefulSet and PVC targets. Their gates and qualified scope maps default
  off/empty. Populate only after inspecting actual Datadog series, units and
  identity tags. Continuously expected gauges have separate absence monitors;
  sparse queue/failure events do not. Do not register idle scale-to-zero
  workloads as continuously required. Remove deliberately retired targets from
  the maps rather than retaining per-node absence groups. PVC collection uses
  the Agent's existing kubelet `/metrics` volume gauges; verify both used and
  capacity bytes for a mounted claim, not catalog metadata alone.
- **Collector self metrics.** `collector_self_metrics_enabled` adds the
  `collector.self-metrics.yaml` reader and direct Agent OpenMetrics autodiscovery
  to the gateway pod. Port 8888 is not a gateway Service port. Seven mapped
  `coda.collector` metric families retain only exporter/data-type labels, with
  an explicit scrape-endpoint identity for per-replica queue arithmetic.
  This custom-metric collection is billable and does not enable monitors.
  `collector_export_monitors_enabled` requires independent series, tag, query
  and routing qualification. Send/enqueue failures are sparse counters;
  uptime supplies the separate continuous scrape-absence signal. A cleared
  failure window or retired pod is not proof that delivery recovered. The
  direct scrape avoids the collector's exporter path but still shares the
  Datadog failure domain; it is not an independent watchdog.
- **Only generation spans reach Langfuse.** A filter keeps spans with
  `langfuse.observation.type` or the `gen_ai.*` attributes the telemetry crate
  sets.
- **Secrets.** Doppler holds every value. Secret Manager holds only the
  read-only, config-scoped `DOPPLER_TOKEN` (`var.doppler_token_secret_id`,
  pinned version). Tofu reads it as an ephemeral value and writes it into
  `doppler-operator-system/doppler-token-observability` through a write-only
  attribute, so it is never stored in state. The operator syncs these Doppler keys (`coda-rust/stg` or `prd`):

  | Doppler key | Kubernetes Secret (namespace `observability`) |
  | --- | --- |
  | `DD_API_KEY` (`datadog` only; ingestion only, `DD_APP_KEY` stays with the operator) | `datadog-secret` (`api-key`) |
  | `GRAFANA_ADMIN_PASSWORD`, `GRAFANA_SLACK_WEBHOOK_URL` (`grafana` only) | `grafana-secrets` (Grafana env: `GF_SECURITY_ADMIN_PASSWORD`, `GRAFANA_SLACK_WEBHOOK_URL`) |
  | `LANGFUSE_SALT`, `LANGFUSE_NEXTAUTH_SECRET`, `LANGFUSE_ENCRYPTION_KEY`, `LANGFUSE_CLICKHOUSE_PASSWORD`, `LANGFUSE_REDIS_PASSWORD`, `LANGFUSE_S3_ACCESS_KEY_ID`, `LANGFUSE_S3_SECRET_ACCESS_KEY` | `langfuse-secrets` |
  | `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`, `LANGFUSE_INIT_PROJECT_SECRET_KEY`, `LANGFUSE_INIT_USER_EMAIL`, `LANGFUSE_INIT_USER_PASSWORD` | `langfuse-init` (Langfuse web env) |
  | the same project key pair | `otel-collector-langfuse` (`LANGFUSE_PUBLIC_KEY`, `LANGFUSE_SECRET_KEY`) |
  | `LANGFUSE_SSO_CLIENT_SECRET`, `LANGFUSE_SSO_SIGNING_KEY` (`langfuse_sso_image` only) | `langfuse-sso` (issuer env); the client secret also goes to `langfuse-secrets` for Langfuse's `AUTH_CUSTOM_CLIENT_SECRET` |
  The S3-interop HMAC key belongs to `coda-langfuse-blobs` (output
  `langfuse_hmac_service_account`). The operator mints it with
  `gcloud storage hmac create` and stores both halves in Doppler.
  `google_storage_hmac_key` is not used because it would write the secret into
  state. Under `datadog`, the Datadog provider reads `DD_API_KEY`/`DD_APP_KEY`
  from the environment: `doppler run --project coda-rust --config stg -- tofu plan`.
- **Identity.** Langfuse runs as `coda-langfuse` through the cluster
  project's Workload Identity pool and logs in to Cloud SQL as that IAM user,
  so no database password exists. The collector holds no Google identity; it
  only verifies callers' tokens.

## Wiring services

`output.service_telemetry_env` is what each instrumented Cloud Run product
container sets when `rust-environments` has `observability_enabled`:

```text
OTEL_EXPORTER_OTLP_ENDPOINT = http://localhost:4318   # its own sidecar
DD_ENV                      = staging
CODA_LOG_FORMAT             = json
CODA_TELEMETRY_CONTENT      = none                    # or full: telemetry_content
CODA_TELEMETRY_BACKEND      = datadog                 # or grafana: telemetry_backend
```

`telemetry_content` is set in both roots and must match. `full` requires
`langfuse_enabled`. Under `full` the prompt and completion text of generation
spans reaches the in-project Langfuse, and the ops pipelines strip it before
Datadog and Grafana.

`DD_VERSION` is baked into each image at build time and is never set here.
`DD_ENV` names the environment under both backends. `output.telemetry_backend`
and `output.grafana` (the in-cluster endpoints and retention, null under
`datadog`) let a reviewer compare the choice across roots.
`output.otel_gateway` echoes the gateway endpoint, audience, ranges and
callers, so a reviewer can compare both roots' plans.

The project's Langfuse data-retention setting (Enterprise in some versions) is
applied in the UI. The blob lifecycle enforces `langfuse_retention_days` on GCS.

## Notification qualification boundary

Use the [offline qualification flow](/dev/observability/README%2Emd#offline-downtime-notification-rehearsal)
to prepare and check evidence before requesting a live drill. It makes no
provider calls and cannot enable monitors or contacts. Read back the actual
Datadog Slack account/channel and the independent Cloud Monitoring contact;
configuration in this root alone does not prove delivery. Supply the
`rust-environments` watchdog identifiers from its verified outputs, not guessed names.
Keep secrets only in Doppler. The qualification fixture carries only public
identifiers and evidence references, never webhook URLs or credentials.

Datadog remains the primary `#engineering-observability` operational route;
Grafana service evaluation is shadow-only in both `grafana` and `both` modes.
Its separate test route requires the exact synthetic qualification label and
alert title; neither alone may bypass the shadow receiver.
A routing-test send and a synthetic evaluator drill each need explicit operator
approval, separate from approval for any infrastructure saved plan/apply.

## Maintenance and rollback ownership

The observability operator owns collector, Datadog, Grafana and scoped ingestion
configuration here. The runtime infrastructure operator owns authenticated probe
identities, scheduler, direct Cloud Monitoring watchdog and Cloud Run sidecars
in `rust-environments`. Product releases own only the named product image;
`workspace-tools staging-release` checks the before/after service shape. The
separately tested `scripts/gcp/release.py`
state machine is not the ordinary CI rollout path. The product's
on-call owner investigates service readiness and queue progress; telemetry is
not a second clinical run ledger.

For maintenance, record the staging environment, affected fixed service or
resource scope, owner, start and automatic expiry, and recovery check in the
operator change record. Mute only the affected primary monitors. Do not silence
the independent watchdog to hide a collector or Grafana outage; planned probe
maintenance needs a separately approved bounded watchdog mute. Grafana service
rules remain shadow-only, not a backup paging route. Restore and verify every
mute at expiry; missing data is not evidence of successful recovery.

Before any apply, retain the reviewed saved plan, current image digests and
rendered configuration, scope/cost review, and rollback plan. Reverting source
alone changes no deployed resource. Revert the responsible inputs, inspect a
new saved plan and obtain authorization before rollback. To withdraw concurrent
export, return both infrastructure roots to `datadog`, preserve the Datadog
route and direct watchdog, and verify the ordinary release shape again. Do not
approve deletion of Grafana PVCs or state buckets as a routine rollback;
inspect retention and export/backup requirements separately. To withdraw a
synthetic notification test, turn its gate off and verify removal of both its
contact and policy from persistent Grafana provisioning after restart.

## Periodic qualification

After routing, IAM, exporter, datasource, provisioning or release-path changes,
and at the team's scheduled notification drill, rerun local contracts first.
Then request separate authorization for a labeled staging routing send and
synthetic evaluator failure/recovery exercise. Record source revision, deployed
digests, actual signal names/labels/units, each rule's evaluation state, exact
Slack message links, UTC timestamps, observed delay, recovery and cleanup.
Provider notification metadata alone is not Slack receipt. Exercise collector
and Grafana loss through dedicated synthetic signals and the direct watchdog;
never stop real clinical services as an implicit part of qualification.

The [operating runbook](../../../book/src/observability.md#qualification-and-incident-ownership)
separates simulated failure, actual component failure and live delivery gates.
Proof and live rollout status belong in the change record, not this document.

## Testing

Run the commands below inside `nix develop .#agent`. These are mock-only,
loopback or network-isolated tests; they do not apply, mutate monitors or send
to Slack. The CI observability job runs the same existing harnesses.

```sh
python -m unittest discover -s scripts -p test_alert_qualification.py -v
python -m unittest discover -s scripts/gcp -p 'test_release*.py' -v
python -m unittest discover -s scripts -p test_staging_images.py -v
cargo test -p availability-probe --offline
python scripts/test_collector_privacy.py
python scripts/test_grafana_shadow.py
# Initialize providers only; do not initialize a deployment backend.
tofu -chdir=infra/gcp/rust-environments init -backend=false -input=false
tofu -chdir=infra/gcp/rust-environments test
```

The Grafana harness requires the pinned Grafana, Prometheus and Python images
listed in the CI job to be present locally. It compresses evaluation intervals
and uses a loopback Slack-shaped sink; neither its timings nor its receipts
qualify production timing or real Slack. The collector harness exercises actual
privacy processors and isolates either unavailable destination. Mock plans
check keyless IAM, fixed metric scopes, no-data contracts and shadow routing,
not real cloud authentication or ingestion.


```sh
tofu -chdir=infra/gcp/observability init -backend=false
tofu -chdir=infra/gcp/observability validate
tofu -chdir=infra/gcp/observability test
tofu fmt -check -recursive infra/gcp/observability
```

The tests plan against mock providers. They assert that no public Langfuse
ingress exists, that under `staging_proxy` Grafana and Langfuse get internal
load balancers that admit only the relay subnet, with their sub-paths, Grafana's
JWT sign-in from `X-Coda-IAP-Assertion` and the pinned base-path Langfuse
image, that the ops trace and log pipelines delete every content key
while keeping model, usage, and run ids, that logs never reach Langfuse, that
the gateway mounts its config files with the backend overlay last and its
overlay keeps the privacy processors, that `datadog` plans no Grafana
resources and leaves the Datadog resources and collector args unchanged, that
`grafana` plans no Datadog resources or key sync and provisions the four
alert rules, that `otel-gateway` is an internal load balancer on the reserved
address with exactly the given source ranges and only port 4318, that open
ranges, non-service-account callers and `production` are refused, that the
content default is `none`, that the GCP integration is absent by default and
project-scoped and keyless when enabled, that secrets sync only from Doppler,
and that every monitor notifies `alert_notify`.

This workspace never initializes the backend and never applies this root.
An operator applies it from a reviewed saved plan, after `rust-environments`
has created the cluster, with authorization for the Datadog, Grafana and GCP
changes.
