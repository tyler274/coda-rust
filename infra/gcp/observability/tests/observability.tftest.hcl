# Plan-only proofs with mock providers: nothing here reaches GCP, the
# cluster, Doppler, or Datadog.
mock_provider "google" {
  mock_data "google_container_cluster" {
    defaults = {
      endpoint = "10.0.0.2"
      master_auth = [{
        cluster_ca_certificate = "Y2E="
        client_certificate     = ""
        client_key             = ""
        client_certificate_config = [{
          issue_client_certificate = false
        }]
      }]
    }
  }
  mock_data "google_compute_network" {
    defaults = {
      id        = "projects/test-rust-stg-only/global/networks/rust-private"
      self_link = "projects/test-rust-stg-only/global/networks/rust-private"
    }
  }
  mock_resource "google_service_account" {
    defaults = {
      email = "coda-langfuse@test-rust-stg-only.iam.gserviceaccount.com"
      name  = "projects/test-rust-stg-only/serviceAccounts/coda-langfuse@test-rust-stg-only.iam.gserviceaccount.com"
    }
  }
}
mock_provider "kubernetes" {}
mock_provider "helm" {}
mock_provider "datadog" {
  mock_resource "datadog_integration_gcp_sts" {
    defaults = {
      delegate_account_email = "ddgci-test@datadog-gci-sts-test.iam.gserviceaccount.com"
    }
  }
}

override_resource {
  target = google_sql_database_instance.langfuse[0]
  values = { connection_name = "test-rust-stg-only:us-east1:coda-langfuse" }
}

# Test-only identifiers shaped like the rust-environments outputs.
variables {
  environment      = "staging"
  project_id       = "test-rust-stg-only"
  cluster_project  = "test-rust-stg-only"
  cluster_location = "us-east1"
  cluster_name     = "rust-observability"
  network_name     = "rust-private"

  otel_gateway_ip            = "10.82.0.10"
  otel_gateway_source_ranges = ["10.80.0.0/24"]
  otel_gateway_audience      = "coda-otel-gateway-test"
  otel_gateway_allowed_callers = [
    "physician-runtime@test-rust-stg-only.iam.gserviceaccount.com",
    "duet-runtime@test-rust-stg-only.iam.gserviceaccount.com",
    "physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com",
  ]
}

run "langfuse_has_no_public_ingress" {
  command = plan

  assert {
    condition     = local.langfuse_values.langfuse.ingress.enabled == false
    error_message = "The Langfuse chart's Ingress must stay off; there is no public edge."
  }

  assert {
    condition     = local.langfuse_values.langfuse.web.service.type == "ClusterIP"
    error_message = "Langfuse web must be a ClusterIP service."
  }

  assert {
    condition     = length(helm_release.langfuse_iap) == 0 && length(google_compute_subnetwork.proxy_only) == 0
    error_message = "No gateway may exist while langfuse_iap_enabled stays false."
  }

  assert {
    condition     = length(google_iap_web_region_backend_service_iam_member.langfuse) == 0
    error_message = "A disabled Langfuse gateway grants no IAP access."
  }

  assert {
    condition     = local.langfuse_values.langfuse.features.signUpDisabled && !local.langfuse_values.langfuse.features.telemetryEnabled
    error_message = "Langfuse must not allow open sign-up or phone home."
  }

  assert {
    condition     = google_storage_bucket.langfuse[0].public_access_prevention == "enforced"
    error_message = "The Langfuse blob bucket must enforce public access prevention."
  }

  assert {
    condition     = !google_sql_database_instance.langfuse[0].settings[0].ip_configuration[0].ipv4_enabled
    error_message = "The Langfuse database must have no public IP."
  }

  assert {
    condition     = kubernetes_service_v1.collector.spec[0].type == "ClusterIP"
    error_message = "The collector stays cluster-private."
  }

  assert {
    condition     = google_service_account_iam_member.langfuse_workload_identity[0].member == "serviceAccount:test-rust-stg-only.svc.id.goog[observability/langfuse]"
    error_message = "Langfuse must bind through the observability cluster project's Workload Identity pool."
  }
}

# Phase one of the staging rollout: the gateway and the Datadog Agent run
# before the Langfuse IAM grants and the HMAC key exist.
run "the_gateway_runs_without_langfuse" {
  command = plan
  variables { langfuse_enabled = false }

  assert {
    condition = (
      length(helm_release.langfuse) + length(google_sql_database_instance.langfuse) + length(google_sql_database.langfuse) +
      length(google_sql_user.langfuse) + length(google_storage_bucket.langfuse) + length(google_storage_bucket_iam_member.langfuse_blobs) +
      length(google_service_account.langfuse) + length(google_service_account.langfuse_blobs) +
      length(google_service_account_iam_member.langfuse_workload_identity) + length(google_project_iam_member.langfuse_cloud_sql) == 0
    )
    error_message = "With Langfuse off, none of its stores, identities or grants may be planned."
  }

  assert {
    condition     = length(setintersection(toset(keys(local.doppler_secrets)), toset(["langfuse-secrets", "langfuse-init", "otel-collector-langfuse"]))) == 0
    error_message = "With Langfuse off, no Langfuse key may be required in Doppler."
  }

  assert {
    condition = (
      join(" ", kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].args) == "--config=/etc/otelcol/collector.yaml --config=/etc/otelcol/collector.gke.yaml --config=/etc/otelcol/collector.gke-no-langfuse.yaml" &&
      kubernetes_config_map_v1.collector.data["collector.gke-no-langfuse.yaml"] == file("${path.module}/../../../dev/observability/collector.gke-no-langfuse.yaml") &&
      length(kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env_from) == 0 &&
      length([for e in kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env : e if startswith(e.name, "LANGFUSE_")]) == 0
    )
    error_message = "Without Langfuse the gateway runs the no-Langfuse overlay last and reads no Langfuse endpoint or keys."
  }

  # The overlay only swaps the generation exporter; the Datadog pipelines and
  # every privacy and caller step stay as the gateway overlay states them.
  assert {
    condition = (
      yamldecode(local.collector_no_langfuse_config).service.pipelines["traces/langfuse"].exporters == ["nop"] &&
      keys(yamldecode(local.collector_no_langfuse_config).service.pipelines) == ["traces/langfuse"] &&
      !can(yamldecode(local.collector_no_langfuse_config).service.pipelines["traces/langfuse"].processors) &&
      !can(yamldecode(local.collector_no_langfuse_config).processors)
    )
    error_message = "The no-Langfuse overlay may change only the generation pipeline's exporter."
  }

  assert {
    condition     = kubernetes_service_v1.otel_gateway.spec[0].load_balancer_ip == "10.82.0.10" && helm_release.datadog_agent.namespace == "observability"
    error_message = "The gateway and the Datadog Agent must still deploy without Langfuse."
  }
}

run "reject_an_iap_gateway_without_langfuse" {
  command = plan
  variables {
    langfuse_enabled         = false
    langfuse_iap_enabled     = true
    langfuse_hostname        = "langfuse.internal.example.test"
    langfuse_iap_client_id   = "000000000000-test.apps.googleusercontent.com"
    langfuse_iap_certificate = "langfuse-internal"
  }
  expect_failures = [var.langfuse_iap_enabled]
}

run "the_iap_gateway_is_internal_only" {
  command = plan

  variables {
    langfuse_iap_enabled         = true
    langfuse_hostname            = "langfuse.internal.example.test"
    langfuse_iap_client_id       = "000000000000-test.apps.googleusercontent.com"
    langfuse_iap_certificate     = "langfuse-internal"
    langfuse_iap_members         = ["group:coda-eng@example.test"]
    langfuse_iap_backend_service = "test-gke-created-langfuse-backend"
  }

  assert {
    condition     = local.langfuse_gateway_class == "gke-l7-rilb"
    error_message = "The Langfuse gateway must be the regional internal class, never an external one."
  }

  assert {
    condition     = one([for m in local.langfuse_iap_manifests : m.spec.default.iap.enabled if m.kind == "GCPBackendPolicy"])
    error_message = "IAP must guard the Langfuse backend when the gateway is on."
  }

  assert {
    condition     = length(helm_release.langfuse_iap) == 1 && contains(keys(local.doppler_secrets), "langfuse-iap-oauth")
    error_message = "The gateway and its Doppler-synced OAuth secret must exist together."
  }

  assert {
    condition = alltrue([
      for grant in google_iap_web_region_backend_service_iam_member.langfuse :
      grant.project == var.project_id && grant.region == var.region && grant.web_region_backend_service == "test-gke-created-langfuse-backend" && grant.role == "roles/iap.httpsResourceAccessor" && grant.member == "group:coda-eng@example.test"
    ]) && length(google_iap_web_region_backend_service_iam_member.langfuse) == 1
    error_message = "Langfuse users must receive access only to the verified regional backend."
  }
}

run "bootstrap_gateway_without_access_grants" {
  command = plan
  variables {
    langfuse_iap_enabled     = true
    langfuse_hostname        = "langfuse.internal.example.test"
    langfuse_iap_client_id   = "000000000000-test.apps.googleusercontent.com"
    langfuse_iap_certificate = "langfuse-internal"
  }
  assert {
    condition     = length(helm_release.langfuse_iap) == 1 && length(google_iap_web_region_backend_service_iam_member.langfuse) == 0
    error_message = "GKE must be able to create the backend before its identity is used for a scoped grant."
  }
}

run "reject_members_without_backend_identity" {
  command = plan
  variables {
    langfuse_iap_enabled     = true
    langfuse_hostname        = "langfuse.internal.example.test"
    langfuse_iap_client_id   = "000000000000-test.apps.googleusercontent.com"
    langfuse_iap_certificate = "langfuse-internal"
    langfuse_iap_members     = ["group:coda-eng@example.test"]
  }
  expect_failures = [var.langfuse_iap_members]
}

run "reject_blank_backend_identity" {
  command = plan
  variables { langfuse_iap_backend_service = "   " }
  expect_failures = [var.langfuse_iap_backend_service]
}

run "reject_an_iap_gateway_without_its_inputs" {
  command = plan

  variables {
    langfuse_iap_enabled = true
  }

  expect_failures = [var.langfuse_iap_enabled]
}

run "datadog_pipeline_deletes_content_keys" {
  command = plan

  assert {
    condition     = local.collector_parsed.service.pipelines["traces/datadog"].processors == ["memory_limiter", "transform/strip_content", "batch"]
    error_message = "Every Datadog-bound span must pass the content-stripping processor."
  }

  # Each content key the services may emit must match a delete pattern, on
  # span attributes and on span event attributes.
  assert {
    condition = alltrue(flatten([
      for context in ["span", "spanevent"] : [
        for key in [
          "gen_ai.prompt",
          "gen_ai.prompt.0.content",
          "gen_ai.completion",
          "gen_ai.completion.0.content",
          "gen_ai.input.messages",
          "gen_ai.output.messages",
          "langfuse.observation.input",
          "langfuse.observation.output",
          "langfuse.trace.input",
          "langfuse.trace.output",
        ] : !contains(local.collector_trace_allowed_keys[context], key)
      ]
    ]))
    error_message = "A gen_ai or Langfuse content key survives the Datadog pipeline."
  }

  # The patterns must not strip the attributes Datadog and ENG-74 rely on.
  assert {
    condition = alltrue([
      for key in ["gen_ai.request.model", "gen_ai.usage.input_tokens", "gen_ai.system", "coda.run_id"] :
      contains(local.collector_trace_allowed_keys["span"], key)
    ])
    error_message = "The content filter must keep model, usage, and run id attributes."
  }

  assert {
    condition     = local.collector_parsed.service.pipelines["traces/langfuse"].processors == ["memory_limiter", "filter/langfuse_generations", "batch"]
    error_message = "Langfuse must receive only generation spans."
  }

  assert {
    condition     = kubernetes_config_map_v1.collector.data["collector.yaml"] == file("${path.module}/../../../dev/observability/collector.yaml")
    error_message = "The deployed collector must run the shared dev/observability/collector.yaml unchanged."
  }

  # Logs go to Datadog only, through the same content deletion.
  assert {
    condition     = local.collector_parsed.service.pipelines["logs/datadog"].processors == ["memory_limiter", "transform/strip_log_content", "batch"] && local.collector_parsed.service.pipelines["logs/datadog"].exporters == ["otlp/datadog"]
    error_message = "Every Datadog-bound log record must pass the content-stripping processor."
  }

  assert {
    condition = alltrue(flatten([
      for context in ["log", "resource"] : [
        for key in ["gen_ai.prompt", "gen_ai.completion.0.content", "gen_ai.input.messages", "gen_ai.output.messages", "langfuse.observation.input", "langfuse.trace.output", "error.message", "exception.message", "unknown.content"] :
        !contains(local.collector_log_allowed_keys[context], key)
      ]
    ]))
    error_message = "A gen_ai or Langfuse content key survives the Datadog log pipeline."
  }

  assert {
    condition = alltrue([
      for name, pipeline in local.collector_parsed.service.pipelines :
      !contains(pipeline.exporters, "otlphttp/langfuse") if startswith(name, "logs")
    ])
    error_message = "Logs must never reach Langfuse."
  }
}

run "the_gateway_overlay_keeps_the_privacy_steps" {
  command = plan

  assert {
    condition = (
      kubernetes_config_map_v1.collector.data["collector.gke.yaml"] == file("${path.module}/../../../dev/observability/collector.gke.yaml") &&
      join(" ", kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].args) == "--config=/etc/otelcol/collector.yaml --config=/etc/otelcol/collector.gke.yaml" &&
      !contains(keys(kubernetes_config_map_v1.collector.data), "collector.gke-no-langfuse.yaml")
    )
    error_message = "The gateway must mount and run the shared config followed by the unchanged gateway overlay."
  }

  # Lists replace on merge: the overlay's restated processors must still
  # strip content before Datadog and filter generations before Langfuse.
  assert {
    condition = (
      contains(local.collector_gateway_parsed.service.pipelines["traces/datadog"].processors, "transform/strip_content") &&
      contains(local.collector_gateway_parsed.service.pipelines["logs/datadog"].processors, "transform/strip_log_content") &&
      contains(local.collector_gateway_parsed.service.pipelines["traces/langfuse"].processors, "filter/langfuse_generations") &&
      alltrue([for p in values(local.collector_gateway_parsed.service.pipelines) : p.processors[length(p.processors) - 1] == "batch"])
    )
    error_message = "The gateway overlay dropped a content-stripping or generation-filter step."
  }

  assert {
    condition = (
      local.collector_gateway_parsed.receivers.otlp.protocols.http.auth.authenticator == "oidc" &&
      local.collector_gateway_parsed.receivers.otlp.protocols.grpc.auth.authenticator == "oidc" &&
      local.collector_gateway_parsed.extensions.oidc.issuer_url == "https://accounts.google.com"
    )
    error_message = "Every gateway receiver must require a Google-issued ID token."
  }

  assert {
    condition = (
      one([for e in kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env : e.value if e.name == "OTEL_GATEWAY_AUDIENCE"]) == "coda-otel-gateway-test" &&
      one([for e in kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env : e.value if e.name == "OTEL_GATEWAY_ALLOWED_CALLERS"]) == "duet-runtime@test-rust-stg-only.iam.gserviceaccount.com,physician-runtime@test-rust-stg-only.iam.gserviceaccount.com,physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com"
    )
    error_message = "The gateway must verify the shared audience and admit exactly the three runtime identities, comma-joined without spaces."
  }
}

run "the_gateway_is_internal_and_source_restricted" {
  command = plan

  assert {
    condition = (
      kubernetes_service_v1.otel_gateway.spec[0].type == "LoadBalancer" &&
      kubernetes_service_v1.otel_gateway.metadata[0].annotations["networking.gke.io/load-balancer-type"] == "Internal"
    )
    error_message = "The gateway must be a GKE internal passthrough load balancer, never an external one."
  }

  assert {
    condition = (
      kubernetes_service_v1.otel_gateway.spec[0].load_balancer_ip == "10.82.0.10" &&
      kubernetes_service_v1.otel_gateway.spec[0].load_balancer_source_ranges == toset(["10.80.0.0/24"])
    )
    error_message = "The gateway must use the reserved address and admit only the Cloud Run egress range."
  }

  assert {
    condition = (
      length(kubernetes_service_v1.otel_gateway.spec[0].port) == 1 &&
      kubernetes_service_v1.otel_gateway.spec[0].port[0].port == 4318 &&
      kubernetes_service_v1.otel_gateway.spec[0].port[0].target_port == "4318"
    )
    error_message = "The gateway exposes OTLP/HTTP 4318 only."
  }

  assert {
    condition     = kubernetes_service_v1.collector.spec[0].type == "ClusterIP"
    error_message = "The in-cluster collector Service stays ClusterIP."
  }
}

run "reject_an_open_gateway_range" {
  command = plan
  variables { otel_gateway_source_ranges = ["0.0.0.0/0"] }
  expect_failures = [var.otel_gateway_source_ranges]
}

run "reject_public_gateway_address" {
  command = plan
  variables { otel_gateway_ip = "8.8.8.8" }
  expect_failures = [var.otel_gateway_ip]
}

run "reject_public_gateway_source" {
  command = plan
  variables { otel_gateway_source_ranges = ["8.8.8.0/24"] }
  expect_failures = [var.otel_gateway_source_ranges]
}

run "reject_a_non_service_account_caller" {
  command = plan
  variables { otel_gateway_allowed_callers = ["user:someone@example.test"] }
  expect_failures = [var.otel_gateway_allowed_callers]
}

run "reject_production" {
  command = plan
  variables { environment = "production" }
  expect_failures = [var.environment]
}

run "content_defaults_to_none" {
  command = plan

  assert {
    condition     = var.telemetry_content == "none" && local.service_telemetry_env.CODA_TELEMETRY_CONTENT == "none"
    error_message = "Services must default to sending no prompt or completion text."
  }

  assert {
    condition = (
      local.service_telemetry_env.OTEL_EXPORTER_OTLP_ENDPOINT == "http://localhost:4318" &&
      local.service_telemetry_env.DD_ENV == "staging" &&
      local.service_telemetry_env.CODA_LOG_FORMAT == "json" &&
      !contains(keys(local.service_telemetry_env), "DD_VERSION")
    )
    error_message = "Products export to their localhost sidecar, tag env:staging, log JSON, and leave DD_VERSION to the image."
  }

  assert {
    condition     = local.otel_gateway_otlp_http == "http://10.82.0.10:4318"
    error_message = "Sidecars must target the gateway's reserved internal address."
  }
}

run "gcp_integration_is_off_by_default" {
  command = plan

  assert {
    condition = (
      length(datadog_integration_gcp_sts.staging) == 0 &&
      length(google_service_account.datadog_gcp) == 0 &&
      length(google_project_iam_member.datadog_gcp) == 0 &&
      length(google_service_account_iam_member.datadog_gcp_delegate) == 0
    )
    error_message = "The Datadog GCP integration is billable and external; it must default off."
  }
}

run "gcp_integration_is_project_scoped_and_keyless" {
  command = plan
  variables { datadog_gcp_integration_enabled = true }

  assert {
    condition = (
      length(datadog_integration_gcp_sts.staging) == 1 &&
      google_service_account.datadog_gcp["integration"].project == "test-rust-stg-only" &&
      alltrue([for grant in google_project_iam_member.datadog_gcp : grant.project == "test-rust-stg-only"]) &&
      toset([for grant in google_project_iam_member.datadog_gcp : grant.role]) == toset(["roles/browser", "roles/cloudasset.viewer", "roles/compute.viewer", "roles/monitoring.viewer"])
    )
    error_message = "The integration identity must live in the cluster project with read-only roles only."
  }

  assert {
    condition = (
      google_service_account_iam_member.datadog_gcp_delegate["integration"].role == "roles/iam.serviceAccountTokenCreator" &&
      google_service_account_iam_member.datadog_gcp_delegate["integration"].member == "serviceAccount:ddgci-test@datadog-gci-sts-test.iam.gserviceaccount.com"
    )
    error_message = "Only Datadog's STS delegate may mint tokens, for this one service account."
  }

  assert {
    condition = (
      !datadog_integration_gcp_sts.staging["integration"].resource_collection_enabled &&
      !datadog_integration_gcp_sts.staging["integration"].is_org_folder_resource_collection_enabled &&
      !datadog_integration_gcp_sts.staging["integration"].is_cspm_enabled &&
      !datadog_integration_gcp_sts.staging["integration"].is_security_command_center_enabled &&
      datadog_integration_gcp_sts.staging["integration"].account_tags == toset(["env:staging"])
    )
    error_message = "The integration must collect metrics for this project only, tagged env:staging."
  }

  assert {
    condition = (
      one([for c in datadog_integration_gcp_sts.staging["integration"].monitored_resource_configs : c.filters if c.type == "cloud_run_revision"]) == toset(["service_name:duet", "service_name:physician", "service_name:physician-worker"]) &&
      one([for c in datadog_integration_gcp_sts.staging["integration"].monitored_resource_configs : c.filters if c.type == "gce_instance"]) == toset(["goog-k8s-cluster-name:rust-observability"]) &&
      alltrue([for c in datadog_integration_gcp_sts.staging["integration"].metric_namespace_configs : c.disabled])
    )
    error_message = "Only the three staging services' revisions and the observability cluster's nodes may be imported, and namespace configs may only disable."
  }
}

run "reject_disabling_cloud_run_metrics" {
  command = plan
  variables { datadog_gcp_disabled_metric_namespaces = ["run"] }
  expect_failures = [var.datadog_gcp_disabled_metric_namespaces]
}

run "reject_an_unknown_content_policy" {
  command = plan

  variables {
    telemetry_content = "prompts"
  }

  expect_failures = [var.telemetry_content]
}

run "secrets_come_only_from_doppler" {
  command = plan

  assert {
    condition     = can(regex("^[1-9][0-9]*$", var.doppler_token_secret_version)) && kubernetes_secret_v1.doppler_token.data_wo_revision == 1
    error_message = "The in-cluster Doppler token must come from a pinned Secret Manager version."
  }

  assert {
    condition     = local.doppler_config == "stg" && alltrue([for m in yamldecode(helm_release.doppler_secrets.values[0]).manifests : m.spec.project == "coda-rust" && m.spec.config == "stg"])
    error_message = "Staging must sync from Doppler coda-rust/stg."
  }

  assert {
    condition     = local.doppler_secrets["datadog-secret"]["DD_API_KEY"] == "api-key"
    error_message = "The DatadogAgent's API key must come from the Doppler-synced secret."
  }

  # Chart 1.7.3 renders doppler-operator-system itself; a second creator
  # fails the install.
  assert {
    condition     = !helm_release.doppler_operator.create_namespace && helm_release.doppler_operator.namespace != local.doppler_namespace
    error_message = "The Doppler operator chart owns its namespace; Helm must not create it."
  }

  # The mocked service account yields the IAM user
  # coda-langfuse@test-rust-stg-only.iam, whose `@` must be encoded.
  assert {
    condition     = one([for e in local.langfuse_values.langfuse.additionalEnv : e.value if e.name == "DATABASE_URL"]) == "postgresql://coda-langfuse%40test-rust-stg-only.iam@127.0.0.1:5432/langfuse"
    error_message = "Langfuse must reach Cloud SQL through the IAM proxy sidecar as its encoded IAM user, with no password in its URL."
  }
}

run "monitors_notify_the_alert_handle" {
  command = plan

  variables {
    datadog_monitors_enabled = true
    alert_notify             = "@slack-confirmed-test"
  }

  assert {
    condition = alltrue([
      for m in concat(values(datadog_monitor.site_5xx_rate), datadog_monitor.chat_generation_errors, values(datadog_monitor.site_p95_latency), values(datadog_monitor.async_failures)) :
      strcontains(m.message, "@slack-confirmed-test") && strcontains(m.query, "env:staging")
    ])
    error_message = "Every monitor must notify the confirmed handle and scope its query to staging."
  }
  assert {
    condition = (
      toset(keys(datadog_monitor.site_5xx_rate)) == toset(["physician-sample", "physician-worker", "duet-api"]) &&
      toset(keys(datadog_monitor.site_p95_latency)) == toset(keys(datadog_monitor.site_5xx_rate)) &&
      toset(keys(datadog_monitor.async_failures)) == toset(["physician-sample", "physician-worker", "duet-api"]) &&
      alltrue([for service, monitor in datadog_monitor.async_failures :
        monitor.type == "log alert" &&
        strcontains(monitor.query, "service:${service} env:staging status:error") &&
        contains(monitor.tags, "service:${service}")
      ]) &&
      strcontains(datadog_monitor.async_failures["physician-worker"].query, "status:error") &&
      !strcontains(datadog_monitor.async_failures["duet-api"].query, "@event:") &&
      datadog_monitor.chat_generation_errors[0].type == "trace-analytics alert" &&
      strcontains(datadog_monitor.chat_generation_errors[0].query, "service:physician-worker") &&
      strcontains(datadog_monitor.chat_generation_errors[0].query, "@coda.job:physician_*") &&
      strcontains(datadog_monitor.chat_generation_errors[0].query, "@gen_ai.system:*") &&
      contains(datadog_monitor.chat_generation_errors[0].tags, "service:physician-worker") &&
      !strcontains(datadog_monitor.chat_generation_errors[0].query, "operation_name:")
    )
    error_message = "All services need HTTP coverage and asynchronous products need failure events independent of HTTP status."
  }
}

run "monitors_default_off" {
  command = plan
  assert {
    condition     = length(datadog_monitor.site_5xx_rate) + length(datadog_monitor.chat_generation_errors) + length(datadog_monitor.site_p95_latency) + length(datadog_monitor.async_failures) == 0
    error_message = "Collection installation must not create monitors before the separately authorized drill."
  }
}

run "reject_unconfirmed_alert_route" {
  command = plan
  variables { datadog_monitors_enabled = true }
  expect_failures = [var.datadog_monitors_enabled]
}

run "runtime_never_gets_management_credentials" {
  command = plan
  assert {
    condition     = !contains(keys(local.doppler_secrets["datadog-secret"]), "DD_APP_KEY") && !contains(keys(local.datadog_agent.spec.global.credentials), "appSecret")
    error_message = "Agent ingestion must not receive the operator's Datadog management key."
  }
}
