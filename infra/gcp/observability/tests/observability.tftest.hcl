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
      id        = "projects/coda-health-staging/global/networks/coda-physician-runtime"
      self_link = "projects/coda-health-staging/global/networks/coda-physician-runtime"
    }
  }
  mock_resource "google_service_account" {
    defaults = {
      email = "coda-langfuse@coda-health-staging.iam.gserviceaccount.com"
      name  = "projects/coda-health-staging/serviceAccounts/coda-langfuse@coda-health-staging.iam.gserviceaccount.com"
    }
  }
}
mock_provider "kubernetes" {}
mock_provider "helm" {}
mock_provider "datadog" {}

override_resource {
  target = google_sql_database_instance.langfuse
  values = { connection_name = "coda-health-staging:us-east1:coda-langfuse" }
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
    condition     = google_storage_bucket.langfuse.public_access_prevention == "enforced"
    error_message = "The Langfuse blob bucket must enforce public access prevention."
  }

  assert {
    condition     = !google_sql_database_instance.langfuse.settings[0].ip_configuration[0].ipv4_enabled
    error_message = "The Langfuse database must have no public IP."
  }

  assert {
    condition     = kubernetes_service_v1.collector.spec[0].type == "ClusterIP"
    error_message = "The collector stays cluster-private."
  }
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
          ] : anytrue([
            for pattern in local.collector_strip_patterns[context] : can(regex(pattern, key))
        ])
      ]
    ]))
    error_message = "A gen_ai or Langfuse content key survives the Datadog pipeline."
  }

  # The patterns must not strip the attributes Datadog and ENG-74 rely on.
  assert {
    condition = alltrue(flatten([
      for key in ["gen_ai.request.model", "gen_ai.usage.input_tokens", "gen_ai.system", "coda.run_id", "langfuse.session.id", "langfuse.observation.type"] : [
        for pattern in local.collector_strip_patterns["span"] : !can(regex(pattern, key))
      ]
    ]))
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
}

run "content_defaults_to_none" {
  command = plan

  assert {
    condition     = var.telemetry_content == "none" && local.service_telemetry_env.CODA_TELEMETRY_CONTENT == "none"
    error_message = "Services must default to sending no prompt or completion text."
  }

  assert {
    condition     = local.service_telemetry_env.OTEL_EXPORTER_OTLP_ENDPOINT == "http://otel-collector.observability.svc.cluster.local:4318"
    error_message = "Services must export to the in-cluster collector."
  }
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

  # The mocked service account yields the IAM user
  # coda-langfuse@coda-health-staging.iam, whose `@` must be encoded.
  assert {
    condition     = one([for e in local.langfuse_values.langfuse.additionalEnv : e.value if e.name == "DATABASE_URL"]) == "postgresql://coda-langfuse%40coda-health-staging.iam@127.0.0.1:5432/langfuse"
    error_message = "Langfuse must reach Cloud SQL through the IAM proxy sidecar as its encoded IAM user, with no password in its URL."
  }
}

run "monitors_notify_the_alert_handle" {
  command = plan

  variables {
    alert_notify = "@pagerduty-coda-test"
  }

  assert {
    condition = alltrue([
      for m in [datadog_monitor.site_5xx_rate, datadog_monitor.chat_generation_errors, datadog_monitor.site_p95_latency] :
      strcontains(m.message, "@pagerduty-coda-test")
    ])
    error_message = "Every monitor must notify var.alert_notify."
  }
}
