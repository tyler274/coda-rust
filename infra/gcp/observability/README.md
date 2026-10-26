# Observability (Datadog + self-hosted Langfuse)

This root adds the observability stack to the existing physician runtime
Autopilot cluster (`infra/gcp/physician-runtime`). It reads that cluster and
its VPC through data sources and never manages them. The data flow, content
policy, and runbook are in [book/src/observability.md](/book/src/observability.md).

| Part | What it is |
| --- | --- |
| `collector.tf` | Two-replica OpenTelemetry Collector (`otel/opentelemetry-collector-contrib:0.155.0`) behind a ClusterIP service, running [`dev/observability/collector.yaml`](/dev/observability/collector.yaml) unchanged from a ConfigMap |
| `datadog.tf` | Datadog Operator Helm chart `2.30.0`, one `DatadogAgent` (`gke-autopilot` provider, APM, logs, OTLP on 4317/4318), and three `datadog_monitor`s: site 5xx rate, chat generation errors, site p95 latency, all notifying `var.alert_notify` |
| `langfuse.tf` | `langfuse/langfuse` Helm chart `1.5.41` (Langfuse `3.225.11`), Cloud SQL Postgres 16 with IAM login through a Cloud SQL Proxy sidecar, a GCS blob bucket through S3 interop, in-cluster ClickHouse and Valkey on PVCs, ClusterIP only, and an optional IAP-gated internal Gateway |
| `doppler.tf` | Doppler Kubernetes Operator `1.7.3` and one `DopplerSecret` per consuming Kubernetes Secret |
| `charts/manifests` | A local chart that renders custom resources (DatadogAgent, DopplerSecret, Gateway) so they apply after their operators' CRDs exist |

The default plan deploys the full stack with `CODA_TELEMETRY_CONTENT=none`,
no gateway, and a 90-day Langfuse retention (`var.langfuse_retention_days`,
used as the blob lifecycle and recorded for the project's data-retention
setting).

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
- **Content never reaches Datadog.** The collector's Datadog pipeline deletes
  `gen_ai.prompt*`, `gen_ai.completion*`, `gen_ai.{input,output}.messages`, and
  `langfuse.{observation,trace}.{input,output}` from span and span event
  attributes, whatever the service's content setting. The Agent also excludes
  container logs from this namespace, because Langfuse can log payloads on errors.
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
  | `DD_API_KEY`, `DD_APP_KEY` | `datadog-secret` (`api-key`, `app-key`) |
  | `LANGFUSE_SALT`, `LANGFUSE_NEXTAUTH_SECRET`, `LANGFUSE_ENCRYPTION_KEY`, `LANGFUSE_CLICKHOUSE_PASSWORD`, `LANGFUSE_REDIS_PASSWORD`, `LANGFUSE_S3_ACCESS_KEY_ID`, `LANGFUSE_S3_SECRET_ACCESS_KEY` | `langfuse-secrets` |
  | `LANGFUSE_INIT_PROJECT_PUBLIC_KEY`, `LANGFUSE_INIT_PROJECT_SECRET_KEY`, `LANGFUSE_INIT_USER_EMAIL`, `LANGFUSE_INIT_USER_PASSWORD` | `langfuse-init` (Langfuse web env) |
  | the same project key pair | `otel-collector-langfuse` (`LANGFUSE_PUBLIC_KEY`, `LANGFUSE_SECRET_KEY`) |
  | `LANGFUSE_IAP_CLIENT_SECRET` (gateway only) | `langfuse-iap-oauth` (`key`) |

  The S3-interop HMAC key belongs to `coda-langfuse-blobs` (output
  `langfuse_hmac_service_account`). The operator mints it with
  `gcloud storage hmac create` and stores both halves in Doppler.
  `google_storage_hmac_key` is not used because it would write the secret into
  state. The Datadog provider reads `DD_API_KEY`/`DD_APP_KEY` from the
  environment: `doppler run --project coda-rust --config prd -- tofu plan`.
- **Identity.** Langfuse runs as `coda-langfuse` through Workload Identity
  and logs in to Cloud SQL as that IAM user, so no database password exists. The
  collector holds no Google identity.

## Wiring services

`output.service_telemetry_env` is what an instrumented service sets.
`infra/gcp/physician-runtime` binds the same three values on the physician
site:

```text
OTEL_EXPORTER_OTLP_ENDPOINT = http://otel-collector.observability.svc.cluster.local:4318
DD_ENV                      = staging | production
CODA_TELEMETRY_CONTENT      = none
```

Cloud Run services get a collector sidecar in a follow-up.

## Known gaps before an apply

- The Langfuse chart `1.5.41` pins its own ClickHouse and Valkey images from
  `bitnamilegacy/*`. Chart `2.x` (Langfuse v4) replaces them with an operator
  and SeaweedFS and is a separate upgrade.
- The project's Langfuse data-retention setting (Enterprise in some versions)
  is applied in the UI. The blob lifecycle enforces `langfuse_retention_days`
  on GCS.

## Testing

```sh
tofu -chdir=infra/gcp/observability init -backend=false
tofu -chdir=infra/gcp/observability validate
tofu -chdir=infra/gcp/observability test
tofu fmt -check -recursive infra/gcp/observability
```

The tests plan against mock providers. They assert that no public Langfuse
ingress exists, that the IAP gateway is internal-only and fails closed without
its inputs, that the Datadog pipeline deletes every content key while keeping
model, usage, and run ids, that the content default is `none`, that secrets sync only from
Doppler, and that every monitor notifies `alert_notify`.

This workspace never initializes the backend and never applies this root.
Apply authority stays with the operator who owns the target project.
