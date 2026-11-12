# Plan-only proofs with mock providers: nothing here reaches GCP, the
# cluster, Doppler, Datadog or Langfuse.
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
  mock_data "google_project" {
    defaults = {
      number = "123456789012"
    }
  }
  mock_resource "google_service_account" {
    defaults = {
      email = "coda-observability@test-rust-stg-only.iam.gserviceaccount.com"
      name  = "projects/test-rust-stg-only/serviceAccounts/coda-observability@test-rust-stg-only.iam.gserviceaccount.com"
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

# Test-only identifiers shaped like the rust-environments outputs.
variables {
  environment      = "staging"
  project_id       = "test-rust-stg-only"
  cluster_project  = "test-rust-stg-only"
  cluster_location = "us-east1"
  cluster_name     = "rust-observability"
  langfuse_host    = "https://hipaa.cloud.langfuse.com"

  otel_gateway_ip            = "10.82.0.10"
  otel_gateway_source_ranges = ["10.80.0.0/24"]
  otel_gateway_audience      = "coda-otel-gateway-test"
  otel_gateway_allowed_callers = [
    "physician-runtime@test-rust-stg-only.iam.gserviceaccount.com",
    "duet-runtime@test-rust-stg-only.iam.gserviceaccount.com",
    "physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com",
  ]
}

run "native_cloud_rules_use_real_source_and_scoped_inventory" {
  command = plan
  variables {
    telemetry_backend                = "both"
    grafana_cloud_monitoring_enabled = true
  }
  assert {
    condition     = contains(keys(kubernetes_config_map_v1.grafana_stack["grafana"].data), "cloud-rules.yaml")
    error_message = "Cloud Monitoring must provision rules backed by its native datasource, not an unimplemented compatibility producer."
  }
  assert {
    condition = alltrue([for rule in yamldecode(local.grafana_cloud_rules).groups[0].rules :
      rule.data[0].datasourceUid == "cloud-monitoring" &&
      rule.data[0].model.queryType == "promQL" &&
      rule.data[0].model.promQLQuery.projectName == var.cluster_project &&
      rule.labels.mode == "shadow" && length(rule.uid) <= 40
    ])
    error_message = "Every native rule must query the project through the authenticated cloud datasource and remain shadow-only."
  }
  assert {
    condition = (
      local.grafana_cloud_conditions["readiness-physician"].no_data == "KeepLast" &&
      local.grafana_cloud_conditions["dependency-worker_due"].no_data == "OK" &&
      !contains(keys(local.grafana_cloud_conditions), "absence-worker_due") && !contains(keys(local.grafana_cloud_conditions), "absence-worker_failed") && contains(keys(local.grafana_cloud_conditions), "absence-worker_oldest_due") &&
      strcontains(local.grafana_cloud_conditions["freshness-duet"].expr, "or vector(1)") &&
      alltrue([for key, c in local.grafana_cloud_conditions : strcontains(c.expr, "monitored_resource=\"global\"") if startswith(key, "readiness-") || startswith(key, "freshness-")]) &&
      strcontains(local.grafana_cloud_conditions["dependency-worker_oldest_due"].expr, "custom_googleapis_com:coda_physician_queue_oldest_due_seconds{monitored_resource=\"global\"") && strcontains(local.grafana_cloud_conditions["dependency-worker_oldest_due"].expr, "> bool 300") &&
      local.grafana_cloud_conditions["dependency-worker_due"].duration == "30m"
    )
    error_message = "Native rules must select the probe's global resource, preserve readiness on silence, distinguish missing observations, and never page for idle sparse queues."
  }
  assert {
    condition = (
      toset([for rule in yamldecode(local.grafana_cloud_rules).deleteRules : rule.uid]) == toset(["gcp-dependency-worker_age", "gcp-absence-worker_age", "gcp-dependency-worker_backlog", "gcp-dependency-worker_dead_letters"]) &&
      length(setintersection(toset([for rule in yamldecode(local.grafana_cloud_rules).deleteRules : rule.uid]), toset(local.grafana_cloud_rule_uids))) == 0
    )
    error_message = "The retired Pub/Sub subscription rules must be deleted from Grafana state without deleting a live rule."
  }
}

run "native_cloud_rules_are_removed_when_disabled" {
  command = plan
  variables {
    telemetry_backend = "both"
  }
  assert {
    condition = (
      length(yamldecode(local.grafana_cloud_rules).groups) == 0 &&
      length(yamldecode(local.grafana_cloud_rules).deleteRules) == length(local.grafana_cloud_conditions) + length(local.grafana_cloud_retired_rule_uids)
    )
    error_message = "Disabling the native source must delete its rules from persisted Grafana state."
  }
}

run "the_collector_stays_cluster_private" {
  command = plan

  assert {
    condition     = kubernetes_service_v1.collector.spec[0].type == "ClusterIP"
    error_message = "The collector stays cluster-private."
  }

  assert {
    condition     = length(kubernetes_service_v1.observability_web) == 0 && output.staging_proxy == null
    error_message = "Without staging_proxy no load balancer exposes Grafana."
  }
}

# The collector sends generation spans to the cloud project's OTLP endpoint
# with Basic auth from the project key pair, which Doppler syncs under its
# own names.
run "langfuse_is_the_cloud_project" {
  command = plan

  assert {
    condition = (
      output.langfuse_otlp_endpoint == "https://hipaa.cloud.langfuse.com/api/public/otel" &&
      one([for e in kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env : e.value if e.name == "LANGFUSE_OTLP_ENDPOINT"]) == "https://hipaa.cloud.langfuse.com/api/public/otel"
    )
    error_message = "The collector must post generation spans to <langfuse_host>/api/public/otel."
  }

  assert {
    condition     = local.doppler_secrets["otel-collector-langfuse"] == { LANGFUSE_PUBLIC_KEY = "LANGFUSE_PUBLIC_KEY", LANGFUSE_SECRET_KEY = "LANGFUSE_SECRET_KEY" }
    error_message = "The collector's Langfuse keys must be the project key pair from Doppler, under their own names."
  }

  assert {
    condition     = length(setintersection(toset(keys(local.doppler_secrets)), toset(["langfuse-secrets", "langfuse-init", "langfuse-sso"]))) == 0
    error_message = "No self-hosted Langfuse secret may be synced."
  }
}

run "reject_a_langfuse_host_that_is_not_an_https_origin" {
  command = plan
  variables { langfuse_host = "http://hipaa.cloud.langfuse.com" }
  expect_failures = [var.langfuse_host]
}

run "reject_a_langfuse_host_with_a_path" {
  command = plan
  variables { langfuse_host = "https://hipaa.cloud.langfuse.com/api/public/otel" }
  expect_failures = [var.langfuse_host]
}

# Without a Langfuse project the gateway and the Datadog Agent still run.
run "the_gateway_runs_without_langfuse" {
  command = plan
  variables { langfuse_host = null }

  assert {
    condition     = !contains(keys(local.doppler_secrets), "otel-collector-langfuse") && output.langfuse_otlp_endpoint == null
    error_message = "With Langfuse off, no Langfuse key may be required in Doppler."
  }

  assert {
    condition = (
      join(" ", kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].args) == "--config=/etc/otelcol/collector.yaml --config=/etc/otelcol/collector.gke.yaml --config=/etc/otelcol/collector.gke-no-langfuse.yaml --config=/etc/otelcol/collector.datadog.yaml" &&
      kubernetes_config_map_v1.collector.data["collector.gke-no-langfuse.yaml"] == file("${path.module}/../../../dev/observability/collector.gke-no-langfuse.yaml") &&
      length(kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env_from) == 0 &&
      length([for e in kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env : e if startswith(e.name, "LANGFUSE_")]) == 0
    )
    error_message = "Without Langfuse the gateway runs the no-Langfuse overlay before the backend overlay and reads no Langfuse endpoint or keys."
  }

  # The overlay only swaps the generation exporter; the ops pipelines and
  # every privacy and caller step stay as the gateway overlay states them.
  assert {
    condition = (
      yamldecode(local.collector_no_langfuse_config).service.pipelines["traces/langfuse"].exporters == ["nop"] &&
      keys(yamldecode(local.collector_no_langfuse_config).service.pipelines) == ["traces/langfuse"] &&
      !can(yamldecode(local.collector_no_langfuse_config).service.pipelines["traces/langfuse"].processors) &&
      !can(yamldecode(local.collector_no_langfuse_config).processors) &&
      !can(yamldecode(local.collector_no_langfuse_config).exporters)
    )
    error_message = "The no-Langfuse overlay may change only the generation pipeline's exporter."
  }

  assert {
    condition     = kubernetes_service_v1.otel_gateway.spec[0].load_balancer_ip == "10.82.0.10" && helm_release.datadog_agent[0].namespace == "observability"
    error_message = "The gateway and the Datadog Agent must still deploy without Langfuse."
  }
}

run "ops_pipeline_deletes_content_keys" {
  command = plan

  assert {
    condition     = local.collector_parsed.service.pipelines["traces/ops"].processors == ["memory_limiter", "transform/strip_content", "batch"]
    error_message = "Every span bound for the operations backend must pass the content-stripping processor."
  }

  # Without a backend overlay the ops pipelines drop data rather than send it.
  assert {
    condition     = local.collector_parsed.service.pipelines["traces/ops"].exporters == ["nop"] && local.collector_parsed.service.pipelines["logs/ops"].exporters == ["nop"]
    error_message = "The shared policy's ops pipelines must default to nop."
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
    error_message = "A gen_ai or Langfuse content key survives the ops pipeline."
  }

  # The patterns must not strip the attributes either backend and ENG-74 rely on.
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

  # Logs go to the operations backend only, through the same content deletion.
  assert {
    condition     = local.collector_parsed.service.pipelines["logs/ops"].processors == ["memory_limiter", "transform/strip_log_content", "batch"]
    error_message = "Every log record bound for the operations backend must pass the content-stripping processor."
  }

  # The Datadog overlay sets only the ops exporters, so the processor lists
  # (and the content stripping) survive the merge.
  assert {
    condition = (
      yamldecode(local.collector_backend_config).service.pipelines["traces/ops"].exporters == ["otlp/datadog"] &&
      yamldecode(local.collector_backend_config).service.pipelines["logs/ops"].exporters == ["otlp/datadog"] &&
      toset(keys(yamldecode(local.collector_backend_config).service.pipelines)) == toset(["traces/ops", "logs/ops"]) &&
      alltrue([for p in values(yamldecode(local.collector_backend_config).service.pipelines) : !can(p.processors) && !can(p.receivers)]) &&
      !can(yamldecode(local.collector_backend_config).processors) &&
      !can(yamldecode(local.collector_backend_config).service.extensions)
    )
    error_message = "The Datadog overlay may only set the ops pipelines' exporters to otlp/datadog."
  }

  assert {
    condition = alltrue(flatten([
      for context in ["log", "resource"] : [
        for key in ["gen_ai.prompt", "gen_ai.completion.0.content", "gen_ai.input.messages", "gen_ai.output.messages", "langfuse.observation.input", "langfuse.trace.output", "error.message", "exception.message", "unknown.content"] :
        !contains(local.collector_log_allowed_keys[context], key)
      ]
    ]))
    error_message = "A gen_ai or Langfuse content key survives the ops log pipeline."
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
      join(" ", kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].args) == "--config=/etc/otelcol/collector.yaml --config=/etc/otelcol/collector.gke.yaml --config=/etc/otelcol/collector.datadog.yaml" &&
      kubernetes_config_map_v1.collector.data["collector.datadog.yaml"] == file("${path.module}/../../../dev/observability/collector.datadog.yaml") &&
      toset(keys(kubernetes_config_map_v1.collector.data)) == toset(["collector.yaml", "collector.gke.yaml", "collector.datadog.yaml"])
    )
    error_message = "The gateway must mount and run the shared config, the unchanged gateway overlay, then the Datadog backend overlay last."
  }

  # Lists replace on merge: the overlay's restated processors must still
  # strip content before the operations backend and filter generations
  # before Langfuse.
  assert {
    condition = (
      contains(local.collector_gateway_parsed.service.pipelines["traces/ops"].processors, "transform/strip_content") &&
      contains(local.collector_gateway_parsed.service.pipelines["logs/ops"].processors, "transform/strip_log_content") &&
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
      local.service_telemetry_env.CODA_TELEMETRY_BACKEND == "datadog" &&
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

run "reject_content_without_langfuse" {
  command = plan

  variables {
    telemetry_content = "full"
    langfuse_host     = null
  }

  expect_failures = [var.telemetry_content]
}

run "content_with_langfuse_reaches_services" {
  command = plan

  variables {
    telemetry_content = "full"
  }

  assert {
    condition     = local.service_telemetry_env.CODA_TELEMETRY_CONTENT == "full"
    error_message = "With Langfuse on, services get the full content policy."
  }
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
  # Autopilot memory: per-container against its own limit and OOM kills,
  # grouped by container name so node and pod churn leaves no stale group.
  assert {
    condition = alltrue([for m in concat(datadog_monitor.observability_container_memory, datadog_monitor.observability_container_oom) : (
      strcontains(m.query, "kube_cluster_name:rust-observability,kube_namespace:observability,env:staging") &&
      strcontains(m.query, "by {kube_container_name}") &&
      !strcontains(m.query, "system.mem") && !strcontains(m.query, "by {host}") &&
      strcontains(m.message, "@slack-confirmed-test") &&
      !m.notify_no_data && m.new_group_delay == 300 && m.timeout_h == 1
    )]) && length(datadog_monitor.observability_container_memory) == 1 && length(datadog_monitor.observability_container_oom) == 1
    error_message = "Cluster memory monitors watch observability containers by name (never host memory), skip new groups for 5 minutes and resolve groups that stop reporting."
  }
  assert {
    condition = (
      # min over the window: above the ratio for all 10 minutes, not one sample.
      startswith(datadog_monitor.observability_container_memory[0].query, "min(last_10m):max:kubernetes.memory.working_set") &&
      strcontains(datadog_monitor.observability_container_memory[0].query, "/ max:kubernetes_state.container.memory_limit") &&
      endswith(datadog_monitor.observability_container_memory[0].query, "> 0.9") &&
      strcontains(datadog_monitor.observability_container_oom[0].query, "kubernetes.containers.last_state.terminated") &&
      strcontains(datadog_monitor.observability_container_oom[0].query, "reason:oomkilled")
    )
    error_message = "Memory alerts compare the working set with the container's limit at 90% for the whole window and count OOM-killed terminations."
  }
}

run "reject_container_memory_ratio_out_of_range" {
  command = plan
  variables { container_memory_limit_ratio = 1.2 }
  expect_failures = [var.container_memory_limit_ratio]
}

run "monitors_default_off" {
  command = plan
  assert {
    condition     = length(datadog_monitor.site_5xx_rate) + length(datadog_monitor.chat_generation_errors) + length(datadog_monitor.site_p95_latency) + length(datadog_monitor.async_failures) + length(datadog_monitor.observability_container_memory) + length(datadog_monitor.observability_container_oom) + length(datadog_monitor.qualification) == 0
    error_message = "Collection installation must not create monitors before the separately authorized drill."
  }
}

run "qualification_monitor_stays_off_when_service_monitors_are_on" {
  command = plan
  variables {
    datadog_monitors_enabled = true
    alert_notify             = "@slack-confirmed-test"
  }
  assert {
    condition     = length(datadog_monitor.qualification) == 0
    error_message = "Service monitor authorization must not enable the synthetic qualification monitor."
  }
}

run "qualification_monitor_uses_readiness_thresholds" {
  command = plan
  variables {
    datadog_qualification_monitor_enabled = true
    alert_notify                          = "@slack-confirmed-test"
  }
  assert {
    condition = (
      length(datadog_monitor.qualification) == 1 &&
      datadog_monitor.qualification[0].query == "avg(last_1m):max:coda.qualification.synthetic_readiness{env:staging}.fill(null) >= 1" &&
      datadog_monitor.qualification[0].monitor_thresholds[0].critical == "1" &&
      datadog_monitor.qualification[0].monitor_thresholds[0].critical_recovery == "0.5" &&
      contains(datadog_monitor.qualification[0].tags, "qualification:synthetic") &&
      strcontains(datadog_monitor.qualification[0].message, var.alert_notify) &&
      datadog_monitor.qualification[0].renotify_interval == 15 &&
      datadog_monitor.qualification[0].renotify_occurrences == 2 &&
      datadog_monitor.qualification[0].renotify_statuses == toset(["alert"]) &&
      !datadog_monitor.qualification[0].notify_no_data &&
      datadog_monitor.qualification[0].timeout_h == 0 &&
      !datadog_monitor.qualification[0].require_full_window
    )
    error_message = "The qualification monitor must use the readiness gauge, the synthetic tag, and the confirmed handle."
  }
}

run "reject_qualification_monitor_without_alert_handle" {
  command = plan
  variables { datadog_qualification_monitor_enabled = true }
  expect_failures = [var.datadog_qualification_monitor_enabled]
}

run "reject_qualification_monitor_under_grafana" {
  command = plan
  variables {
    telemetry_backend                     = "grafana"
    datadog_qualification_monitor_enabled = true
    alert_notify                          = "@slack-confirmed-test"
  }
  expect_failures = [var.datadog_qualification_monitor_enabled]
}

run "chat_failure_monitor_has_its_own_gate" {
  command = plan
  variables {
    chat_failure_monitor_enabled = true
    alert_notify                 = "@slack-confirmed-test"
  }
  assert {
    condition = (
      length(datadog_monitor.chat_generation_errors) == 1 &&
      strcontains(datadog_monitor.chat_generation_errors[0].query, "service:physician-worker env:staging status:error") &&
      endswith(datadog_monitor.chat_generation_errors[0].query, ".last(\"10m\") > 5") &&
      datadog_monitor.chat_generation_errors[0].monitor_thresholds[0].critical == "5" &&
      strcontains(datadog_monitor.chat_generation_errors[0].message, "@slack-confirmed-test")
    )
    error_message = "The chat failure gate must create the chat generation monitor at its threshold, notifying the confirmed handle."
  }
  assert {
    condition     = length(datadog_monitor.site_5xx_rate) + length(datadog_monitor.site_p95_latency) + length(datadog_monitor.async_failures) + length(datadog_monitor.observability_container_memory) + length(datadog_monitor.observability_container_oom) + length(datadog_monitor.qualification) == 0
    error_message = "The chat failure gate must not enable any other Datadog monitor."
  }
}

run "reject_chat_failure_monitor_without_alert_handle" {
  command = plan
  variables { chat_failure_monitor_enabled = true }
  expect_failures = [var.chat_failure_monitor_enabled]
}

run "reject_chat_failure_monitor_under_grafana" {
  command = plan
  variables {
    telemetry_backend            = "grafana"
    chat_failure_monitor_enabled = true
    alert_notify                 = "@slack-confirmed-test"
  }
  expect_failures = [var.chat_failure_monitor_enabled]
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

# --- Backend choice (telemetry_backend) --------------------------------------

run "datadog_is_the_default_backend" {
  command = plan

  assert {
    condition     = var.telemetry_backend == "datadog" && output.telemetry_backend == "datadog"
    error_message = "Staging's tfvars set no backend, so the default must stay datadog."
  }

  assert {
    condition = (
      length(kubernetes_stateful_set_v1.grafana_stack) + length(kubernetes_service_v1.grafana_stack) +
      length(kubernetes_config_map_v1.grafana_stack) + length(kubernetes_service_account_v1.grafana) +
      length(google_service_account.grafana_monitoring) + length(google_project_iam_member.grafana_monitoring) +
      length(google_service_account_iam_member.grafana_workload_identity) == 0 &&
      output.grafana == null
    )
    error_message = "Under datadog no Grafana stack resource, identity or output may be planned."
  }

  assert {
    condition     = length(helm_release.datadog_operator) == 1 && length(helm_release.datadog_agent) == 1
    error_message = "Under datadog the Operator and the Agent deploy as before."
  }

  # The only collector change is the backend overlay mounted and run last,
  # with the Agent endpoint and no Grafana endpoint.
  assert {
    condition = (
      { for e in kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env : e.name => e.value if !startswith(e.name, "LANGFUSE_") } == {
        DATADOG_OTLP_ENDPOINT        = "datadog-agent.observability.svc.cluster.local:4317"
        OTEL_GATEWAY_AUDIENCE        = "coda-otel-gateway-test"
        OTEL_GATEWAY_ALLOWED_CALLERS = "duet-runtime@test-rust-stg-only.iam.gserviceaccount.com,physician-runtime@test-rust-stg-only.iam.gserviceaccount.com,physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com"
      }
    )
    error_message = "Under datadog the collector reads the Agent endpoint and the gateway settings, and no Tempo or Loki endpoint."
  }

  assert {
    condition     = contains(keys(local.doppler_secrets), "datadog-secret") && !contains(keys(local.doppler_secrets), "grafana-secrets")
    error_message = "Under datadog Doppler syncs the Agent key and no Grafana key."
  }

  assert {
    condition     = output.datadog_monitor_ids != null
    error_message = "Under datadog the monitor ids output keeps its shape."
  }
}

run "grafana_backend_replaces_datadog" {
  command = plan
  variables { telemetry_backend = "grafana" }

  assert {
    condition = (
      length(helm_release.datadog_operator) + length(helm_release.datadog_agent) +
      length(datadog_monitor.site_5xx_rate) + length(datadog_monitor.chat_generation_errors) +
      length(datadog_monitor.site_p95_latency) + length(datadog_monitor.async_failures) +
      length(datadog_monitor.observability_container_memory) + length(datadog_monitor.observability_container_oom) +
      length(datadog_integration_gcp_sts.staging) + length(google_service_account.datadog_gcp) +
      length(google_project_iam_member.datadog_gcp) + length(google_service_account_iam_member.datadog_gcp_delegate) == 0
    )
    error_message = "Under grafana no Datadog release, monitor or GCP integration may be planned."
  }

  assert {
    condition     = output.datadog_monitor_ids == null && output.datadog_gcp_integration == null && output.telemetry_backend == "grafana"
    error_message = "Under grafana the Datadog outputs are null."
  }

  assert {
    condition = (
      !contains(keys(local.doppler_secrets), "datadog-secret") &&
      local.doppler_secrets["grafana-secrets"] == {
        GRAFANA_ADMIN_PASSWORD = "GF_SECURITY_ADMIN_PASSWORD"
      } &&
      contains([for m in yamldecode(helm_release.doppler_secrets.values[0]).manifests : m.metadata.name], "grafana-secrets") &&
      !contains([for m in yamldecode(helm_release.doppler_secrets.values[0]).manifests : m.metadata.name], "datadog-secret")
    )
    error_message = "Under grafana Doppler syncs only the Grafana admin password and Slack webhook, never DD_API_KEY."
  }

  assert {
    condition = (
      join(" ", kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].args) == "--config=/etc/otelcol/collector.yaml --config=/etc/otelcol/collector.gke.yaml --config=/etc/otelcol/collector.grafana.yaml" &&
      kubernetes_config_map_v1.collector.data["collector.grafana.yaml"] == file("${path.module}/../../../dev/observability/collector.grafana.yaml") &&
      !contains(keys(kubernetes_config_map_v1.collector.data), "collector.datadog.yaml")
    )
    error_message = "Under grafana the gateway runs the unchanged Grafana overlay last and does not mount the Datadog overlay."
  }

  assert {
    condition = (
      { for e in kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].env : e.name => e.value if contains(["DATADOG_OTLP_ENDPOINT", "TEMPO_OTLP_ENDPOINT", "LOKI_OTLP_ENDPOINT"], e.name) } == {
        TEMPO_OTLP_ENDPOINT = "tempo.observability.svc.cluster.local:4317"
        LOKI_OTLP_ENDPOINT  = "http://loki.observability.svc.cluster.local:3100/otlp"
      }
    )
    error_message = "Under grafana the collector reads the in-cluster Tempo and Loki endpoints and no Datadog endpoint."
  }

  # The Grafana overlay sets only the ops exporters; the content-stripping
  # processor lists from earlier files survive the merge.
  assert {
    condition = (
      yamldecode(local.collector_backend_config).service.pipelines["traces/ops"].exporters == ["otlp/tempo"] &&
      yamldecode(local.collector_backend_config).service.pipelines["logs/ops"].exporters == ["otlphttp/loki"] &&
      toset(keys(yamldecode(local.collector_backend_config).service.pipelines)) == toset(["traces/ops", "logs/ops"]) &&
      alltrue([for p in values(yamldecode(local.collector_backend_config).service.pipelines) : !can(p.processors) && !can(p.receivers)]) &&
      !can(yamldecode(local.collector_backend_config).processors) &&
      !can(yamldecode(local.collector_backend_config).service.extensions)
    )
    error_message = "The Grafana overlay may only set the ops pipelines' exporters to Tempo and Loki."
  }

  assert {
    condition     = local.service_telemetry_env.CODA_TELEMETRY_BACKEND == "grafana" && output.service_telemetry_env.CODA_TELEMETRY_BACKEND == "grafana"
    error_message = "Services must be told the backend so their log lines drop the dd.* keys."
  }
}

run "grafana_stack_is_cluster_private_and_bounded" {
  command = plan
  variables { telemetry_backend = "grafana" }

  assert {
    condition = (
      toset(keys(kubernetes_stateful_set_v1.grafana_stack)) == toset(["tempo", "loki", "prometheus", "grafana"]) &&
      alltrue([for s in kubernetes_stateful_set_v1.grafana_stack : s.spec[0].replicas == "1" && s.metadata[0].namespace == "observability"])
    )
    error_message = "Tempo, Loki, Prometheus and Grafana each run as one StatefulSet replica in the observability namespace."
  }

  assert {
    condition = alltrue([for name, s in kubernetes_stateful_set_v1.grafana_stack :
      length(s.spec[0].volume_claim_template) == 1 &&
      s.spec[0].volume_claim_template[0].spec[0].access_modes == toset(["ReadWriteOnce"]) &&
      length(s.spec[0].template[0].spec[0].container[0].resources[0].requests) == 3 &&
      s.spec[0].template[0].spec[0].container[0].resources[0].requests["cpu"] != "" &&
      s.spec[0].template[0].spec[0].container[0].resources[0].requests["memory"] != "" &&
      s.spec[0].template[0].spec[0].container[0].resources[0].requests["ephemeral-storage"] == "1Gi" &&
      s.spec[0].template[0].spec[0].container[0].resources[0].limits["ephemeral-storage"] == "1Gi" &&
      s.spec[0].template[0].spec[0].security_context[0].run_as_non_root &&
      s.spec[0].template[0].spec[0].security_context[0].seccomp_profile[0].type == "RuntimeDefault"
    ])
    error_message = "Every Grafana stack pod has a persistent disk, explicit CPU, memory and ephemeral-storage requests for Autopilot, a RuntimeDefault seccomp profile, and runs as non-root."
  }

  assert {
    condition = (
      alltrue([for s in kubernetes_service_v1.grafana_stack : s.spec[0].type == "ClusterIP"]) &&
      toset(keys(kubernetes_service_v1.grafana_stack)) == toset(["tempo", "loki", "prometheus", "grafana"]) &&
      toset([for p in kubernetes_service_v1.grafana_stack["tempo"].spec[0].port : p.port]) == toset([4317, 3200]) &&
      kubernetes_service_v1.grafana_stack["loki"].spec[0].port[0].port == 3100 &&
      kubernetes_service_v1.grafana_stack["prometheus"].spec[0].port[0].port == 9090 &&
      kubernetes_service_v1.grafana_stack["grafana"].spec[0].port[0].port == 3000
    )
    error_message = "The Grafana stack is reachable only through ClusterIP Services named tempo, loki, prometheus and grafana."
  }

  # Retention defaults: traces 14d, logs 30d, metrics 30d.
  assert {
    condition = (
      one([for e in kubernetes_stateful_set_v1.grafana_stack["tempo"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "TEMPO_BLOCK_RETENTION"]) == "336h" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["loki"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "LOKI_RETENTION_PERIOD"]) == "720h" &&
      contains(kubernetes_stateful_set_v1.grafana_stack["prometheus"].spec[0].template[0].spec[0].container[0].args, "--storage.tsdb.retention.time=30d")
    )
    error_message = "Tempo keeps 14 days, Loki 30 days and Prometheus 30 days by default."
  }

  assert {
    condition = (
      contains(kubernetes_stateful_set_v1.grafana_stack["tempo"].spec[0].template[0].spec[0].container[0].args, "-config.expand-env=true") &&
      contains(kubernetes_stateful_set_v1.grafana_stack["loki"].spec[0].template[0].spec[0].container[0].args, "-config.expand-env=true") &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["tempo"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "PROMETHEUS_REMOTE_WRITE_URL"]) == "http://prometheus.observability.svc.cluster.local:9090/api/v1/write" &&
      contains(kubernetes_stateful_set_v1.grafana_stack["prometheus"].spec[0].template[0].spec[0].container[0].args, "--web.enable-remote-write-receiver")
    )
    error_message = "Tempo and Loki expand their config env, and Tempo remote-writes span metrics to the in-cluster Prometheus."
  }

  assert {
    condition = (
      kubernetes_config_map_v1.grafana_stack["tempo"].data["tempo.yaml"] == file("${path.module}/../../../dev/observability/grafana/tempo.yaml") &&
      kubernetes_config_map_v1.grafana_stack["loki"].data["loki.yaml"] == file("${path.module}/../../../dev/observability/grafana/loki.yaml") &&
      kubernetes_config_map_v1.grafana_stack["prometheus"].data["prometheus.yaml"] == file("${path.module}/../../../dev/observability/grafana/prometheus.yaml") &&
      kubernetes_config_map_v1.grafana_stack["grafana"].data["datasources.yaml"] == file("${path.module}/../../../dev/observability/grafana/provisioning/datasources.yaml") &&
      kubernetes_config_map_v1.grafana_stack["grafana"].data["contact.yaml"] == local.grafana_contact_config
    )
    error_message = "The deployed stack must run the shared dev/observability/grafana files unchanged."
  }

  # The four parity alerts, routed to Slack.
  assert {
    condition = (
      kubernetes_config_map_v1.grafana_stack["grafana"].data["rules.yaml"] == file("${path.module}/../../../dev/observability/grafana/provisioning/alerting/rules.yaml") &&
      toset([for rule in flatten([for group in yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["rules.yaml"]).groups : group.rules]) : rule.uid]) == toset([
        "coda-site-5xx-rate", "coda-site-p95-latency", "coda-chat-generation-errors", "coda-async-failures",
      ]) &&
      yamldecode(local.grafana_contact_config).contactPoints[0].receivers[0].type == "email" &&
      yamldecode(local.grafana_contact_config).policies[0].receiver == "coda-shadow" &&
      yamldecode(local.grafana_contact_config).policies[0].routes[0].mute_time_intervals == ["coda-shadow-always"]
    )
    error_message = "Grafana must provision exactly the four Coda alert rules and the Slack contact point."
  }

  assert {
    condition = (
      alltrue([for rule in flatten([for group in yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["rules.yaml"]).groups : group.rules]) : !contains(keys(rule.labels), "qualification")]) &&
      yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["synthetic-rules.yaml"]).deleteRules[0].uid == "coda-synthetic-notification-test" &&
      length(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["synthetic-rules.yaml"]).groups) == 0 &&
      contains([for item in one([for volume in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].volume : volume if volume.name == "config"]).config_map[0].items : "${item.key}=${item.path}"], "synthetic-rules.yaml=alerting/synthetic-rules.yaml")
    )
    error_message = "The synthetic notification rule stays deleted until the test gate is on, and operational rules stay unlabeled."
  }

  assert {
    condition = (
      [for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env_from : e.secret_ref[0].name] == ["grafana-secrets"] &&
      !contains([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.name], "GF_SECURITY_ADMIN_PASSWORD") &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_AUTH_ANONYMOUS_ENABLED"]) == "false" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_USERS_ALLOW_SIGN_UP"]) == "false"
    )
    error_message = "Grafana reads its admin password and Slack webhook only from the Doppler-synced secret and allows no anonymous access or sign-up."
  }

  assert {
    condition = (
      length(google_service_account.grafana_monitoring) == 0 &&
      yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["cloud-monitoring.yaml"]).deleteDatasources[0].name == "Google Cloud Monitoring" &&
      length(kubernetes_service_account_v1.grafana[0].metadata[0].annotations) == 0
    )
    error_message = "The Cloud Monitoring datasource and its identity are off by default."
  }
}

run "grafana_stays_on_port_forward_by_default" {
  command = plan
  variables { telemetry_backend = "both" }

  assert {
    condition = (
      length(kubernetes_service_v1.observability_web) == 0 &&
      output.staging_proxy == null &&
      !contains(keys(local.doppler_secrets), "grafana-iap-oauth") &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_SERVER_ROOT_URL"]) == "http://localhost:3000" &&
      kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].readiness_probe[0].http_get[0].path == "/api/health" &&
      !contains([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.name], "GF_AUTH_JWT_ENABLED")
    )
    error_message = "Without staging_proxy Grafana keeps its port-forward URL, no JWT sign-in and no load balancer."
  }
}

run "staging_proxy_serves_grafana_under_its_prefix" {
  command = plan
  variables {
    telemetry_backend = "both"
    staging_proxy = {
      origin       = "https://staging.example.test"
      iap_audience = "/projects/134570912572/locations/us-east1/services/public-site-staging"
    }
    observability_web = {
      grafana_ip    = "10.82.0.11"
      source_ranges = ["10.80.1.0/26"]
    }
  }

  assert {
    condition = (
      keys(kubernetes_service_v1.observability_web) == ["grafana"] &&
      kubernetes_service_v1.observability_web["grafana"].spec[0].type == "LoadBalancer" &&
      kubernetes_service_v1.observability_web["grafana"].metadata[0].annotations["networking.gke.io/load-balancer-type"] == "Internal" &&
      kubernetes_service_v1.observability_web["grafana"].spec[0].load_balancer_source_ranges == toset(["10.80.1.0/26"]) &&
      kubernetes_service_v1.observability_web["grafana"].spec[0].port[0].port == 3000 &&
      kubernetes_service_v1.observability_web["grafana"].spec[0].port[0].target_port == "3000" &&
      kubernetes_service_v1.observability_web["grafana"].spec[0].load_balancer_ip == "10.82.0.11" &&
      kubernetes_service_v1.observability_web["grafana"].spec[0].selector == tomap({ application = "observability", component = "grafana" })
    )
    error_message = "Grafana gets one internal passthrough load balancer on its reserved address and port 3000 that admits only the relay subnet and selects only Grafana's pods."
  }

  assert {
    condition = (
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_SERVER_ROOT_URL"]) == "https://staging.example.test/grafana/" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_SERVER_SERVE_FROM_SUB_PATH"]) == "true" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_LIVE_MAX_CONNECTIONS"]) == "0" &&
      kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].readiness_probe[0].http_get[0].path == "/grafana/api/health" &&
      local.grafana_url == "http://grafana.observability.svc.cluster.local:3000/grafana"
    )
    error_message = "Grafana must serve from /grafana/ on the staging origin, with Live off and its probe under the sub-path."
  }

  assert {
    condition = (
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_AUTH_JWT_ENABLED"]) == "true" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_AUTH_JWT_HEADER_NAME"]) == "X-Coda-IAP-Assertion" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_AUTH_JWT_JWK_SET_URL"]) == "https://www.gstatic.com/iap/verify/public_key-jwk" &&
      jsondecode(one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_AUTH_JWT_EXPECT_CLAIMS"])) == { iss = "https://cloud.google.com/iap", aud = "/projects/134570912572/locations/us-east1/services/public-site-staging" } &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_USERS_AUTO_ASSIGN_ORG_ROLE"]) == "Editor" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "GF_AUTH_ANONYMOUS_ENABLED"]) == "false"
    )
    error_message = "Grafana must accept only Google-signed IAP assertions for public-site's audience, read from the header public-site forwards."
  }

  assert {
    condition = output.staging_proxy == {
      grafana_url = "https://staging.example.test/grafana/"
      internal = {
        grafana = "http://10.82.0.11:3000/grafana/"
      }
    }
    error_message = "The staging_proxy output must name Grafana's public URL and its relay upstream."
  }
}

run "staging_proxy_under_datadog_exposes_nothing" {
  command = plan
  variables {
    staging_proxy = {
      origin       = "https://staging.example.test"
      iap_audience = "/projects/134570912572/locations/us-east1/services/public-site-staging"
    }
    observability_web = {
      grafana_ip    = "10.82.0.11"
      source_ranges = ["10.80.1.0/26"]
    }
  }
  assert {
    condition     = length(kubernetes_service_v1.observability_web) == 0 && output.staging_proxy.grafana_url == null && length(output.staging_proxy.internal) == 0
    error_message = "Without the Grafana backend no load balancer exists."
  }
}

run "reject_a_staging_proxy_origin_with_a_path" {
  command = plan
  variables {
    staging_proxy = {
      origin       = "https://staging.example.test/"
      iap_audience = "/projects/134570912572/locations/us-east1/services/public-site-staging"
    }
  }
  expect_failures = [var.staging_proxy]
}

run "reject_a_backend_service_audience" {
  command = plan
  variables {
    staging_proxy = {
      origin       = "https://staging.example.test"
      iap_audience = "/projects/123456789012/global/backendServices/4242"
    }
  }
  expect_failures = [var.staging_proxy]
}

run "reject_load_balancers_open_to_the_cloud_run_subnet" {
  command = plan
  variables {
    staging_proxy = {
      origin       = "https://staging.example.test"
      iap_audience = "/projects/134570912572/locations/us-east1/services/public-site-staging"
    }
    observability_web = {
      grafana_ip    = "10.82.0.11"
      source_ranges = ["10.80.0.0/16"]
    }
  }
  expect_failures = [var.observability_web]
}

run "reject_public_load_balancer_addresses" {
  command = plan
  variables {
    staging_proxy = {
      origin       = "https://staging.example.test"
      iap_audience = "/projects/134570912572/locations/us-east1/services/public-site-staging"
    }
    observability_web = {
      grafana_ip    = "34.1.2.3"
      source_ranges = ["10.80.1.0/26"]
    }
  }
  expect_failures = [var.observability_web]
}

run "reject_load_balancers_without_the_sub_paths" {
  command = plan
  variables {
    observability_web = {
      grafana_ip    = "10.82.0.11"
      source_ranges = ["10.80.1.0/26"]
    }
  }
  expect_failures = [var.observability_web]
}

run "grafana_synthetic_rule_is_labeled_and_operational_rules_stay_shadow" {
  command = plan
  variables {
    telemetry_backend                  = "grafana"
    grafana_test_notifications_enabled = true
  }
  assert {
    condition = (
      kubernetes_config_map_v1.grafana_stack["grafana"].data["rules.yaml"] == file("${path.module}/../../../dev/observability/grafana/provisioning/alerting/rules.yaml") &&
      one(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["synthetic-rules.yaml"]).groups).rules[0].title == "Coda synthetic notification test" &&
      one(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["synthetic-rules.yaml"]).groups).rules[0].labels.qualification == "synthetic-notification-test" &&
      one(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["synthetic-rules.yaml"]).groups).rules[0]["for"] == "1m" &&
      strcontains(one(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["synthetic-rules.yaml"]).groups).rules[0].data[0].model.expr, "{qualification=\"synthetic-notification-test\"} |= \"fail\"")
    )
    error_message = "The gated Grafana rule must count synthetic fail lines without changing the operational rules file."
  }
}

run "grafana_retention_is_configurable" {
  command = plan
  variables {
    telemetry_backend              = "grafana"
    grafana_traces_retention_days  = 7
    grafana_logs_retention_days    = 60
    grafana_metrics_retention_days = 90
  }

  assert {
    condition = (
      one([for e in kubernetes_stateful_set_v1.grafana_stack["tempo"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "TEMPO_BLOCK_RETENTION"]) == "168h" &&
      one([for e in kubernetes_stateful_set_v1.grafana_stack["loki"].spec[0].template[0].spec[0].container[0].env : e.value if e.name == "LOKI_RETENTION_PERIOD"]) == "1440h" &&
      contains(kubernetes_stateful_set_v1.grafana_stack["prometheus"].spec[0].template[0].spec[0].container[0].args, "--storage.tsdb.retention.time=90d")
    )
    error_message = "The retention variables must reach Tempo, Loki and Prometheus."
  }
}

run "grafana_cloud_monitoring_is_read_only_workload_identity" {
  command = plan
  variables {
    telemetry_backend                = "grafana"
    grafana_cloud_monitoring_enabled = true
  }
  override_resource {
    target = google_service_account.grafana_monitoring
    values = {
      email = "grafana-cloud-monitoring@test-rust-stg-only.iam.gserviceaccount.com"
      name  = "projects/test-rust-stg-only/serviceAccounts/grafana-cloud-monitoring@test-rust-stg-only.iam.gserviceaccount.com"
    }
  }

  assert {
    condition = (
      google_service_account.grafana_monitoring[0].project == "test-rust-stg-only" &&
      length(google_project_iam_member.grafana_monitoring) == 1 &&
      google_project_iam_member.grafana_monitoring[0].role == "roles/monitoring.viewer" &&
      google_project_iam_member.grafana_monitoring[0].project == "test-rust-stg-only" &&
      google_project_iam_member.grafana_monitoring[0].member == "serviceAccount:grafana-cloud-monitoring@test-rust-stg-only.iam.gserviceaccount.com"
    )
    error_message = "The datasource identity lives in the cluster project and holds only roles/monitoring.viewer."
  }

  assert {
    condition = (
      google_service_account_iam_member.grafana_workload_identity[0].role == "roles/iam.workloadIdentityUser" &&
      google_service_account_iam_member.grafana_workload_identity[0].member == "serviceAccount:test-rust-stg-only.svc.id.goog[observability/grafana]" &&
      kubernetes_service_account_v1.grafana[0].metadata[0].annotations["iam.gke.io/gcp-service-account"] == "grafana-cloud-monitoring@test-rust-stg-only.iam.gserviceaccount.com" &&
      kubernetes_stateful_set_v1.grafana_stack["grafana"].spec[0].template[0].spec[0].service_account_name == "grafana"
    )
    error_message = "Only the Grafana Kubernetes service account may assume the datasource identity, through Workload Identity."
  }

  assert {
    condition = (
      one(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["cloud-monitoring.yaml"]).datasources).type == "stackdriver" &&
      one(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["cloud-monitoring.yaml"]).datasources).jsonData.authenticationType == "gce" &&
      one(yamldecode(kubernetes_config_map_v1.grafana_stack["grafana"].data["cloud-monitoring.yaml"]).datasources).jsonData.defaultProject == "test-rust-stg-only"
    )
    error_message = "The Cloud Monitoring datasource authenticates through the pod's Workload Identity, never a key."
  }
}

run "concurrent_backends_remain_independent_and_shadow" {
  command = plan
  variables {
    telemetry_backend                  = "both"
    grafana_compatibility_metrics_url  = "http://compatibility.observability.svc.cluster.local:9090"
    grafana_test_notifications_enabled = true
    datadog_gcp_integration_enabled    = true
    grafana_cloud_monitoring_enabled   = true
  }
  assert {
    condition     = length(helm_release.datadog_agent) == 1 && length(kubernetes_stateful_set_v1.grafana_stack) == 4 && length(local.collector_backend_env) == 3
    error_message = "Both must retain every backend and its collector endpoint."
  }
  assert {
    condition     = yamldecode(local.collector_backend_config).service.pipelines["traces/ops"].exporters == ["otlp/datadog", "otlp/tempo"] && yamldecode(local.collector_backend_config).service.pipelines["logs/ops"].exporters == ["otlp/datadog", "otlphttp/loki"] && alltrue([for e in values(yamldecode(local.collector_backend_config).exporters) : e.sending_queue.enabled && e.sending_queue.queue_size == 1000 && !e.sending_queue.block_on_overflow && e.retry_on_failure.max_elapsed_time == "30s"])
    error_message = "Both signals must fan out with independent bounded nonblocking queues and retries."
  }
  assert {
    condition     = yamldecode(local.grafana_contact_config).policies[0].receiver == "coda-shadow" && yamldecode(local.grafana_contact_config).policies[0].routes[0].object_matchers == [["qualification", "=", "synthetic-notification-test"], ["alertname", "=", "Coda synthetic notification test"]] && length(local.grafana_compatibility_config) == 2 && contains(keys(local.doppler_secrets["grafana-secrets"]), "GRAFANA_SLACK_WEBHOOK_URL")
    error_message = "Only explicit synthetic labels may notify; compatibility source is separately configured and the webhook is a secret reference."
  }
}

run "reject_an_unknown_backend" {
  command = plan
  variables { telemetry_backend = "unknown" }
  expect_failures = [var.telemetry_backend]
}

run "reject_datadog_gcp_integration_under_grafana" {
  command = plan
  variables {
    telemetry_backend               = "grafana"
    datadog_gcp_integration_enabled = true
  }
  expect_failures = [var.datadog_gcp_integration_enabled]
}

run "reject_datadog_monitors_under_grafana" {
  command = plan
  variables {
    telemetry_backend        = "grafana"
    datadog_monitors_enabled = true
    alert_notify             = "@slack-confirmed-test"
  }
  expect_failures = [var.datadog_monitors_enabled]
}

run "reject_cloud_monitoring_under_datadog" {
  command = plan
  variables { grafana_cloud_monitoring_enabled = true }
  expect_failures = [var.grafana_cloud_monitoring_enabled]
}

run "reject_grafana_thresholds_that_differ_from_the_rules" {
  command = plan
  variables {
    telemetry_backend       = "grafana"
    site_5xx_rate_threshold = 0.1
  }
  expect_failures = [kubernetes_config_map_v1.grafana_stack]
}

run "dependency_rules_are_opt_in_even_with_existing_monitors" {
  command = plan
  variables {
    datadog_monitors_enabled = true
    alert_notify             = "@slack-confirmed-test"
  }
  assert {
    condition     = length(datadog_monitor.dependency_symptom) == 0 && length(datadog_monitor.dependency_absence) == 0 && length(datadog_monitor.collector_availability) == 0
    error_message = "Existing monitor authorization must not enable unqualified dependency signals."
  }
}

run "worker_queue_scopes_require_global_location_import" {
  command = plan
  variables {
    telemetry_backend               = "both"
    datadog_gcp_integration_enabled = true
    datadog_monitors_enabled        = true
    dependency_monitors_enabled     = true
    alert_notify                    = "@slack-confirmed-test"
    dependency_metric_scopes = {
      worker_oldest_due = "env:staging,project_id:test-rust-stg-only,resource_type:global"
    }
  }
  expect_failures = [var.dependency_metric_scopes]
}

run "dependency_missing_data_and_idle_contract" {
  command = plan
  variables {
    telemetry_backend                      = "both"
    availability_monitors_enabled          = true
    availability_firing_metric             = "synthetic.availability.firing"
    datadog_gcp_integration_enabled        = true
    datadog_gcp_global_location_enabled    = true
    datadog_monitors_enabled               = true
    dependency_monitors_enabled            = true
    collector_availability_monitor_enabled = true
    alert_notify                           = "@slack-confirmed-test"
    dependency_metric_scopes = {
      physician_sql_up  = "env:staging,project_id:test-rust-stg-only,database_id:test-rust-stg-only:rust-physician"
      redis_uptime      = "env:staging,project_id:test-rust-stg-only,instance_id:test-redis"
      worker_oldest_due = "env:staging,project_id:test-rust-stg-only,resource_type:global"
      worker_due        = "env:staging,project_id:test-rust-stg-only,resource_type:global"
      worker_failed     = "env:staging,project_id:test-rust-stg-only,resource_type:global"
    }
  }
  assert {
    condition     = toset(keys(datadog_monitor.dependency_absence)) == toset(["physician_sql_up", "redis_uptime", "worker_oldest_due"]) && alltrue([for monitor in datadog_monitor.dependency_symptom : !monitor.notify_no_data])
    error_message = "Only continuously expected signals get separate absence monitors; empty queues cannot page on absence."
  }
  assert {
    condition     = datadog_monitor.dependency_symptom["worker_oldest_due"].query == "min(last_5m):max:gcp.custom.coda.physician.queue.oldest_due_seconds{env:staging,project_id:test-rust-stg-only,resource_type:global}.fill(null) > 300" && datadog_monitor.dependency_symptom["worker_due"].monitor_thresholds[0].critical == "0" && datadog_monitor.dependency_symptom["worker_failed"].query == "min(last_5m):max:gcp.custom.coda.physician.queue.failed{env:staging,project_id:test-rust-stg-only,resource_type:global}.fill(null) > 0"
    error_message = "Worker signals read the Postgres-derived gauges: age past five minutes of polls, positive due work, any failed work."
  }
  assert {
    condition     = datadog_integration_gcp_sts.staging["integration"].is_global_location_enabled
    error_message = "Worker queue monitors need the global-location import that carries their custom gauges."
  }
  assert {
    condition     = alltrue([for monitor in datadog_monitor.dependency_absence : monitor.notify_no_data && monitor.no_data_timeframe == 20 && !monitor.require_full_window && monitor.renotify_occurrences == 2])
    error_message = "Absence must have an explicit delayed NO DATA policy and bounded reminders."
  }
  assert {
    condition     = datadog_monitor.collector_availability["telemetry_absent"].notify_no_data && !datadog_monitor.collector_availability["unavailable"].notify_no_data && datadog_monitor.collector_availability["unavailable"].query == "max(last_5m):min:kubernetes_state.deployment.replicas_available{env:staging,kube_cluster_name:rust-observability,kube_namespace:observability,kube_deployment:otel-collector}.fill(null) < 1"
    error_message = "Collector availability must use stable deployment identity, never retired node/pod groups, with separate absence."
  }
  assert {
    condition     = datadog_monitor.dependency_symptom["physician_sql_up"].timeout_h == 0 && datadog_monitor.dependency_symptom["worker_due"].timeout_h == 1 && datadog_monitor.dependency_symptom["worker_oldest_due"].timeout_h == 0 && datadog_monitor.collector_availability["unavailable"].timeout_h == 0
    error_message = "Continuous symptoms cannot auto-resolve on missing data; sparse queue resolution is explicitly qualified."
  }
  assert {
    condition     = length(datadog_monitor.site_5xx_rate) == 3 && length(datadog_monitor.site_p95_latency) == 3 && length(datadog_monitor.async_failures) == 3 && length(datadog_monitor.chat_generation_errors) == 1 && length(datadog_monitor.observability_container_memory) == 1 && length(datadog_monitor.observability_container_oom) == 1
    error_message = "Dependency coverage must preserve existing application and memory monitors."
  }
  assert {
    condition = (
      toset(keys(datadog_monitor.availability)) == toset(["physician", "duet", "physician-worker"]) &&
      alltrue([for name, monitor in datadog_monitor.availability :
        !monitor.notify_no_data && monitor.timeout_h == 0 &&
        monitor.query == "avg(last_1m):max:synthetic.availability.firing{project_id:test-rust-stg-only,target:${name}}.fill(null) >= 1" &&
        strcontains(monitor.message, var.alert_notify) && strcontains(monitor.message, "{{#is_recovery}}")
      ]) && yamldecode(local.grafana_contact_config).policies[0].receiver == "coda-shadow"
    )
    error_message = "Concurrent availability/dependency alerts retain fixed target labels, primary routing and no recovery on silence; Grafana stays shadow-only."
  }
}

run "reject_unscoped_dependency_series" {
  command = plan
  variables {
    dependency_metric_scopes = { worker_oldest_due = "env:staging,*" }
  }
  expect_failures = [var.dependency_metric_scopes]
}

run "collector_collection_is_independent_of_monitor_activation" {
  command = plan
  variables {
    collector_self_metrics_enabled = true
  }
  assert {
    condition     = contains(keys(kubernetes_config_map_v1.collector.data), "collector.self-metrics.yaml") && contains(kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container[0].args, "--config=/etc/otelcol/collector.self-metrics.yaml")
    error_message = "Opt-in must mount and activate the pinned self-metric reader."
  }
  assert {
    condition     = length(datadog_monitor.collector_export_failures) == 0 && length(datadog_monitor.collector_queue_pressure) == 0 && length(datadog_monitor.collector_scrape_absence) == 0
    error_message = "Collection is not permission to activate unqualified monitors."
  }
  assert {
    condition     = jsondecode(kubernetes_deployment_v1.collector.spec[0].template[0].metadata[0].annotations["ad.datadoghq.com/otel-collector.checks"]).openmetrics.instances[0].openmetrics_endpoint == "http://%%host%%:8888/metrics" && length(jsondecode(kubernetes_deployment_v1.collector.spec[0].template[0].metadata[0].annotations["ad.datadoghq.com/otel-collector.checks"]).openmetrics.instances[0].metrics) == 7
    error_message = "The Agent must scrape the pod directly with seven explicit mappings, not wildcard application metrics."
  }
  assert {
    condition     = jsondecode(kubernetes_deployment_v1.collector.spec[0].template[0].metadata[0].annotations["ad.datadoghq.com/otel-collector.checks"]).openmetrics.instances[0].include_labels == ["exporter", "data_type"] && alltrue([for port in kubernetes_service_v1.otel_gateway.spec[0].port : port.port != 8888])
    error_message = "Self metrics retain only bounded labels and cannot be exposed through the gateway LoadBalancer."
  }
}

run "collector_export_and_workload_contracts" {
  command = plan
  variables {
    datadog_monitors_enabled                = true
    collector_self_metrics_enabled          = true
    collector_export_monitors_enabled       = true
    observability_workload_monitors_enabled = true
    alert_notify                            = "@slack-confirmed-test"
    observability_workload_targets = {
      datadog-cluster-agent = { kind = "deployment", minimum_ready = 1 }
      test-stateful         = { kind = "statefulset", minimum_ready = 1 }
    }
    observability_pvc_scopes = { test-storage = "persistentvolumeclaim:test-storage" }
  }
  assert {
    condition     = length(datadog_monitor.collector_export_failures) == 4 && alltrue([for monitor in datadog_monitor.collector_export_failures : !monitor.notify_no_data && !monitor.require_full_window]) && datadog_monitor.collector_scrape_absence[0].notify_no_data && datadog_monitor.collector_scrape_absence[0].no_data_timeframe == 10
    error_message = "Failure counters are sparse, with one separate continuous scrape absence monitor."
  }
  assert {
    condition     = datadog_monitor.collector_queue_pressure[0].monitor_thresholds[0].critical == "0.8" && !datadog_monitor.collector_queue_pressure[0].notify_no_data && datadog_monitor.collector_queue_pressure[0].timeout_h == 1
    error_message = "Empty queues cannot alert; retired pod pressure groups must expire without pod absence pages."
  }
  assert {
    condition     = strcontains(datadog_monitor.collector_queue_pressure[0].query, "by {collector_replica,exporter,data_type}") && contains(jsondecode(kubernetes_deployment_v1.collector.spec[0].template[0].metadata[0].annotations["ad.datadoghq.com/otel-collector.checks"]).openmetrics.instances[0].tags, "collector_replica:%%host%%")
    error_message = "Queue arithmetic must pair each scrape endpoint's numerator/denominator without relying on optional Agent pod tag cardinality."
  }
  assert {
    condition     = length(datadog_monitor.workload_health) == 4 && datadog_monitor.workload_health["test-stateful/absence"].notify_no_data && !datadog_monitor.workload_health["test-stateful/availability"].notify_no_data && datadog_monitor.workload_health["datadog-cluster-agent/availability"].timeout_h == 0
    error_message = "Stable workload identities need independent availability and absence, never node-group retention."
  }
  assert {
    condition     = length(datadog_monitor.pvc_storage) == 3 && datadog_monitor.pvc_storage["test-storage/used_absence"].notify_no_data && datadog_monitor.pvc_storage["test-storage/capacity_absence"].notify_no_data && !datadog_monitor.pvc_storage["test-storage/pressure"].notify_no_data && datadog_monitor.pvc_storage["test-storage/pressure"].monitor_thresholds[0].critical == "0.85"
    error_message = "PVC ratio needs both continuous numerator and denominator absence checks, not zero filling."
  }
}

run "gcp_dependency_opt_in_stays_scoped" {
  command = plan
  variables {
    datadog_gcp_integration_enabled   = true
    datadog_gcp_dependency_collection = { cloudsql = { verified_resource_filters = ["database_id:test-rust-stg-only:rust-physician"] } }
  }
  assert {
    condition     = length([for config in datadog_integration_gcp_sts.staging["integration"].metric_namespace_configs : config if config.id == "cloudsql" && !config.disabled && length(config.filters) == 0]) == 1 && length([for config in datadog_integration_gcp_sts.staging["integration"].metric_namespace_configs : config if contains(["redis", "pubsub", "storage", "prometheus"], config.id) && config.disabled]) == 4
    error_message = "Only the opted-in namespace may change; its filters select metric names, not resource tags."
  }
  assert {
    condition     = length([for config in datadog_integration_gcp_sts.staging["integration"].monitored_resource_configs : config if config.type == "cloudsql_database" && config.filters == toset(["database_id:test-rust-stg-only:rust-physician"])]) == 1 && !datadog_integration_gcp_sts.staging["integration"].is_global_location_enabled && length(google_project_iam_member.datadog_gcp) == 4
    error_message = "Resource allowlist must match the opt-in without global collection or new IAM."
  }
}

run "reject_unbounded_gcp_dependency_opt_in" {
  command = plan
  variables {
    datadog_gcp_integration_enabled   = true
    datadog_gcp_dependency_collection = { redis = { verified_resource_filters = [] } }
  }
  expect_failures = [var.datadog_gcp_dependency_collection]
}

run "autopilot_workloads_declare_admission_defaults" {
  command = plan
  variables { telemetry_backend = "both" }

  assert {
    condition = (
      length(kubernetes_stateful_set_v1.grafana_stack) == 4 &&
      alltrue([for container in concat(
        kubernetes_deployment_v1.collector.spec[0].template[0].spec[0].container,
        [for set in kubernetes_stateful_set_v1.grafana_stack : set.spec[0].template[0].spec[0].container[0]],
        ) : (
        container.resources[0].requests["ephemeral-storage"] == "1Gi" &&
        container.resources[0].limits["ephemeral-storage"] == "1Gi" &&
        toset(container.security_context[0].capabilities[0].drop) == toset(["NET_RAW"]) &&
        container.security_context[0].allow_privilege_escalation == false &&
        container.security_context[0].privileged == false &&
        container.security_context[0].read_only_root_filesystem == false &&
        container.security_context[0].run_as_non_root == false
      )])
    )
    error_message = "The collector and each Grafana container must drop NET_RAW and request and limit ephemeral-storage at 1Gi."
  }

  assert {
    condition = alltrue(concat(
      [for spec in [kubernetes_deployment_v1.collector.spec[0].template[0].spec[0]] : (
        spec.security_context[0].seccomp_profile[0].type == "RuntimeDefault" &&
        spec.security_context[0].run_as_non_root == false &&
        length(spec.security_context[0].supplemental_groups) == 0 &&
        one([for item in spec.toleration : item if item.key == "kubernetes.io/arch"]).operator == "Equal" &&
        one([for item in spec.toleration : item if item.key == "kubernetes.io/arch"]).value == "amd64" &&
        one([for item in spec.toleration : item if item.key == "kubernetes.io/arch"]).effect == "NoSchedule"
      )],
      [for set in kubernetes_stateful_set_v1.grafana_stack : (
        set.spec[0].template[0].spec[0].security_context[0].seccomp_profile[0].type == "RuntimeDefault" &&
        set.spec[0].template[0].spec[0].security_context[0].run_as_non_root == true &&
        one([for item in set.spec[0].template[0].spec[0].toleration : item if item.key == "kubernetes.io/arch"]).operator == "Equal" &&
        one([for item in set.spec[0].template[0].spec[0].toleration : item if item.key == "kubernetes.io/arch"]).value == "amd64" &&
        one([for item in set.spec[0].template[0].spec[0].toleration : item if item.key == "kubernetes.io/arch"]).effect == "NoSchedule"
      )],
    ))
    error_message = "Each workload pod must use RuntimeDefault seccomp and tolerate kubernetes.io/arch=amd64:NoSchedule. Grafana pods stay non-root."
  }
}

run "reject_project_wide_dependency_collection" {
  command = plan
  variables {
    datadog_gcp_integration_enabled   = true
    datadog_gcp_dependency_collection = { cloudsql = { verified_resource_filters = ["project_id:test-rust-stg-only"] } }
  }
  expect_failures = [var.datadog_gcp_dependency_collection]
}

run "new_collection_defaults_remain_off" {
  command = plan
  assert {
    condition     = !contains(keys(kubernetes_config_map_v1.collector.data), "collector.self-metrics.yaml") && !contains(keys(kubernetes_deployment_v1.collector.spec[0].template[0].metadata[0].annotations), "ad.datadoghq.com/otel-collector.checks") && length(datadog_monitor.pvc_storage) == 0 && length(datadog_monitor.workload_health) == 0
    error_message = "No self scrape or workload/PVC monitors may appear by default."
  }
}

run "reject_grafana_latency_thresholds_that_differ_from_the_rules" {
  command = plan
  variables {
    telemetry_backend = "grafana"
    service_p95_latency_threshold_seconds = {
      physician-sample = 2
      physician-worker = 60
      duet-api         = 2
    }
  }
  expect_failures = [kubernetes_config_map_v1.grafana_stack]
}

run "langfuse_monitors_watch_only_the_langfuse_exporter" {
  command = plan
  variables {
    telemetry_backend              = "both"
    collector_self_metrics_enabled = true
    langfuse_monitors_enabled      = true
    alert_notify                   = "@slack-confirmed-test"
  }
  assert {
    condition = toset(keys(datadog_monitor.langfuse_export_failures)) == toset(["send_failed_spans", "enqueue_failed_spans"]) && alltrue([
      for monitor in concat(values(datadog_monitor.langfuse_export_failures), datadog_monitor.langfuse_export_queue) :
      strcontains(monitor.query, "exporter:otlphttp/langfuse") && strcontains(monitor.query, "env:staging") &&
      strcontains(monitor.message, "@slack-confirmed-test") && strcontains(monitor.message, "https://hipaa.cloud.langfuse.com") &&
      !monitor.notify_no_data && monitor.timeout_h == 1 &&
      contains(monitor.tags, "destination:langfuse")
    ])
    error_message = "Langfuse exporter monitors filter to otlphttp/langfuse in staging, name the cloud host, notify the confirmed handle and expire retired groups."
  }
  assert {
    condition     = datadog_monitor.langfuse_export_queue[0].monitor_thresholds[0].critical == "0.8" && strcontains(datadog_monitor.langfuse_export_queue[0].query, "by {collector_replica,exporter,data_type}")
    error_message = "Queue pressure pairs each replica's size with its capacity at 80%."
  }
  assert {
    condition     = length([for key in keys(datadog_monitor.workload_health) : key if startswith(key, "langfuse")]) == 0
    error_message = "No in-cluster Langfuse workload is left to watch."
  }
  assert {
    condition     = length(datadog_monitor.site_5xx_rate) + length(datadog_monitor.chat_generation_errors) + length(datadog_monitor.async_failures) + length(datadog_monitor.collector_export_failures) == 0
    error_message = "The Langfuse gate must not enable the per-service or all-exporter monitors."
  }
}

run "langfuse_monitors_default_off" {
  command = plan
  variables {
    telemetry_backend              = "both"
    collector_self_metrics_enabled = true
  }
  assert {
    condition     = length(datadog_monitor.langfuse_export_failures) + length(datadog_monitor.langfuse_export_queue) == 0
    error_message = "Self-metric collection alone creates no Langfuse monitor."
  }
}

run "reject_langfuse_monitors_without_the_scrape" {
  command = plan
  variables {
    langfuse_monitors_enabled = true
    alert_notify              = "@slack-confirmed-test"
  }
  expect_failures = [var.langfuse_monitors_enabled]
}

run "reject_langfuse_monitors_without_langfuse" {
  command = plan
  variables {
    telemetry_backend              = "both"
    langfuse_host                  = null
    collector_self_metrics_enabled = true
    langfuse_monitors_enabled      = true
    alert_notify                   = "@slack-confirmed-test"
  }
  expect_failures = [var.langfuse_monitors_enabled]
}

run "reject_langfuse_monitors_under_grafana" {
  command = plan
  variables {
    telemetry_backend         = "grafana"
    langfuse_monitors_enabled = true
    alert_notify              = "@slack-confirmed-test"
  }
  expect_failures = [var.langfuse_monitors_enabled]
}
