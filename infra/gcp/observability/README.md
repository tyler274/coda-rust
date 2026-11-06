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
| `langfuse.tf` | `langfuse/langfuse` Helm chart `1.5.41` (Langfuse `3.225.11`), Cloud SQL Postgres 16 with IAM login through a Cloud SQL Proxy sidecar, a GCS blob bucket through S3 interop, in-cluster ClickHouse and Valkey on PVCs, ClusterIP only, and an optional IAP-gated internal Gateway |
| `doppler.tf` | Doppler Kubernetes Operator `1.7.3` and one `DopplerSecret` per consuming Kubernetes Secret |
| `charts/manifests` | A local chart that renders custom resources (DatadogAgent, DopplerSecret, Gateway) so they apply after their operators' CRDs exist |

The default plan deploys the full stack with `telemetry_backend = "datadog"`,
`CODA_TELEMETRY_CONTENT=none`, no gateway, and a 90-day Langfuse retention
(`var.langfuse_retention_days`, used as the blob lifecycle and recorded for
the project's data-retention setting).

`langfuse_enabled = false` deploys only the collector gateway, the selected
backend and the Doppler operator. Nothing from `langfuse.tf` is planned, no
Langfuse key is synced from Doppler, and the gateway merges
[`collector.gke-no-langfuse.yaml`](/dev/observability/collector.gke-no-langfuse.yaml)
before the backend overlay, sending generation spans to `nop`. Staging starts
there because two Langfuse prerequisites need the project owner: the
project-level Cloud SQL grants (`resourcemanager.projects.setIamPolicy`) and
the HMAC key (`storage.hmacKeys.create`). Turning it on later adds Langfuse
without changing the operations path.

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

Cloud Monitoring is a supported, default-off datasource through Workload
Identity (`grafana_cloud_monitoring_enabled`), not through Tempo span metrics.
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

```sh
root=infra/gcp/observability
tofu -chdir=infra/gcp/rust-environments output -json otel_gateway \
  | jq '{otel_gateway_ip: .ip, otel_gateway_audience: .audience,
         otel_gateway_source_ranges: .source_ranges,
         otel_gateway_allowed_callers: .allowed_callers}' > /tmp/otel-gateway.tfvars.json
tofu -chdir=$root init -backend-config=backend/staging.gcs.tfbackend
doppler run --project coda-rust --config stg -- \
  tofu -chdir=$root plan -var-file=env/staging.tfvars \
  -var-file=/tmp/otel-gateway.tfvars.json -out=observability.tfplan
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

## Boundary

- **No public edge.** The Langfuse chart's Ingress is off and every service is
  `ClusterIP`, including Tempo, Loki, Prometheus and Grafana. People reach
  the UIs with
  `kubectl -n observability port-forward svc/langfuse-web 3000:3000` or
  `kubectl -n observability port-forward svc/grafana 3000`. Setting
  `langfuse_iap_enabled = true` (with hostname, OAuth client id and certificate)
  adds a `gke-l7-rilb` internal Gateway with IAP on the backend. It is reachable
  only from inside the VPC. First create the gateway with no
  `langfuse_iap_members`. After GKE reconciles it, the operator verifies the
  regional backend serving the `langfuse-web` HTTPRoute and supplies its actual
  name as `langfuse_iap_backend_service`, together with the intended members.
  The IAM grant targets only that backend; its name is not the Kubernetes
  Service name and must not be guessed. A recreated backend needs a new verified
  name before this root grants it access.
  If an older version of this root was applied, the reviewed migration plan must
  remove its project-wide `google_iap_web_iam_member.langfuse` grants and add the
  scoped grants. Do not move the old state entry to the new resource type or
  leave the broader grant unmanaged. Verify effective access to Langfuse and
  refusal on an unrelated IAP application after the operator applies the plan.
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
  manage integrations.
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
  | `LANGFUSE_IAP_CLIENT_SECRET` (gateway only) | `langfuse-iap-oauth` (`key`) |

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
CODA_TELEMETRY_CONTENT      = none
CODA_TELEMETRY_BACKEND      = datadog                 # or grafana: telemetry_backend
```

`DD_VERSION` is baked into each image at build time and is never set here.
`DD_ENV` names the environment under both backends. `output.telemetry_backend`
and `output.grafana` (the in-cluster endpoints and retention, null under
`datadog`) let a reviewer compare the choice across roots.
`output.otel_gateway` echoes the gateway endpoint, audience, ranges and
callers, so a reviewer can compare both roots' plans.

The project's Langfuse data-retention setting (Enterprise in some versions) is
applied in the UI. The blob lifecycle enforces `langfuse_retention_days` on GCS.

## Testing

```sh
tofu -chdir=infra/gcp/observability init -backend=false
tofu -chdir=infra/gcp/observability validate
tofu -chdir=infra/gcp/observability test
tofu fmt -check -recursive infra/gcp/observability
```

The tests plan against mock providers. They assert that no public Langfuse
ingress exists, that the IAP gateway is internal-only and fails closed without
its inputs, that the ops trace and log pipelines delete every content key
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
