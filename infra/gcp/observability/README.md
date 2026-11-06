# Observability (Datadog + self-hosted Langfuse)

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
| `collector.tf` | Two-replica OpenTelemetry Collector (`otel/opentelemetry-collector-contrib:0.155.0`) running [`dev/observability/collector.yaml`](/dev/observability/collector.yaml) merged with the gateway overlay [`collector.gke.yaml`](/dev/observability/collector.gke.yaml), both unchanged from one ConfigMap. A ClusterIP Service serves in-cluster callers; the `otel-gateway` internal load balancer serves Cloud Run |
| `datadog.tf` | Datadog Operator Helm chart `2.30.0`, one `DatadogAgent` (`gke-autopilot` provider, APM, OTLP on 4317/4318 with OTLP logs on, `env:<environment>`), and the `datadog_monitor`s behind `datadog_monitors_enabled`: 5xx rate and p95 latency per service, physician chat generation errors, and asynchronous failures for physician-worker and duet-api, all notifying `var.alert_notify` (staging: Slack `#engineering-observability`) |
| `datadog_gcp.tf` | Off by default (`datadog_gcp_integration_enabled`): the Datadog GCP integration for `cluster_project` only, through service-account impersonation |
| `langfuse.tf` | `langfuse/langfuse` Helm chart `1.5.41` (Langfuse `3.225.11`), Cloud SQL Postgres 16 with IAM login through a Cloud SQL Proxy sidecar, a GCS blob bucket through S3 interop, in-cluster ClickHouse and Valkey on PVCs, ClusterIP only, and an optional IAP-gated internal Gateway |
| `doppler.tf` | Doppler Kubernetes Operator `1.7.3` and one `DopplerSecret` per consuming Kubernetes Secret |
| `charts/manifests` | A local chart that renders custom resources (DatadogAgent, DopplerSecret, Gateway) so they apply after their operators' CRDs exist |

The default plan deploys the full stack with `CODA_TELEMETRY_CONTENT=none`,
no gateway, and a 90-day Langfuse retention (`var.langfuse_retention_days`,
used as the blob lifecycle and recorded for the project's data-retention
setting).

`langfuse_enabled = false` deploys only the collector gateway, the Datadog
Agent and the Doppler operator. Nothing from `langfuse.tf` is planned, no
Langfuse key is synced from Doppler, and the gateway merges
[`collector.gke-no-langfuse.yaml`](/dev/observability/collector.gke-no-langfuse.yaml)
last, sending generation spans to `nop`. Staging starts there because two
Langfuse prerequisites need the project owner: the project-level Cloud SQL
grants (`resourcemanager.projects.setIamPolicy`) and the HMAC key
(`storage.hmacKeys.create`). Turning it on later adds Langfuse without
changing the Datadog path.

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
  `ClusterIP`. People reach the UI with
  `kubectl -n observability port-forward svc/langfuse-web 3000:3000`. Setting
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
- **Content never reaches Datadog.** The collector's Datadog pipelines delete
  `gen_ai.prompt*`, `gen_ai.completion*`, `gen_ai.{input,output}.messages`, and
  `langfuse.{observation,trace}.{input,output}` from span and span event
  attributes and from log record and resource attributes, whatever the
  service's content setting. Log records never go to Langfuse. The Agent
  tails no container logs (`containerCollectAll = false`) and excludes this
  namespace's container logs, because Langfuse can log payloads on errors; it
  accepts OTLP logs only from the gateway.
- **GCP metrics.** With `datadog_gcp_integration_enabled`, `datadog_gcp.tf`
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
  | `DD_API_KEY` (ingestion only; `DD_APP_KEY` stays with the operator) | `datadog-secret` (`api-key`) |
  | `LANGFUSE_SALT`, `LANGFUSE_NEXTAUTH_SECRET`, `LANGFUSE_ENCRYPTION_KEY`, `LANGFUSE_CLICKHOUSE_PASSWORD`, `LANGFUSE_REDIS_PASSWORD`, `LANGFUSE_S3_ACCESS_KEY_ID`, `LANGFUSE_S3_SECRET_ACCESS_KEY` | `langfuse-secrets` |
  | `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`, `LANGFUSE_INIT_PROJECT_SECRET_KEY`, `LANGFUSE_INIT_USER_EMAIL`, `LANGFUSE_INIT_USER_PASSWORD` | `langfuse-init` (Langfuse web env) |
  | the same project key pair | `otel-collector-langfuse` (`LANGFUSE_PUBLIC_KEY`, `LANGFUSE_SECRET_KEY`) |
  | `LANGFUSE_IAP_CLIENT_SECRET` (gateway only) | `langfuse-iap-oauth` (`key`) |

  The S3-interop HMAC key belongs to `coda-langfuse-blobs` (output
  `langfuse_hmac_service_account`). The operator mints it with
  `gcloud storage hmac create` and stores both halves in Doppler.
  `google_storage_hmac_key` is not used because it would write the secret into
  state. The Datadog provider reads `DD_API_KEY`/`DD_APP_KEY` from the
  environment: `doppler run --project coda-rust --config stg -- tofu plan`.
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
```

`DD_VERSION` is baked into each image at build time and is never set here.
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
its inputs, that the Datadog trace and log pipelines delete every content key
while keeping model, usage, and run ids, that logs never reach Langfuse, that
the gateway mounts both config files and its overlay keeps the privacy
processors, that `otel-gateway` is an internal load balancer on the reserved
address with exactly the given source ranges and only port 4318, that open
ranges, non-service-account callers and `production` are refused, that the
content default is `none`, that the GCP integration is absent by default and
project-scoped and keyless when enabled, that secrets sync only from Doppler,
and that every monitor notifies `alert_notify`.

This workspace never initializes the backend and never applies this root.
An operator applies it from a reviewed saved plan, after `rust-environments`
has created the cluster, with authorization for the Datadog and GCP changes.
