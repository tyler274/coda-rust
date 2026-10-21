locals {
  namespace      = "observability"
  doppler_config = var.environment == "staging" ? "stg" : "prd"
  dd_env         = var.environment

  workload_services = toset([
    "container.googleapis.com",
    "iam.googleapis.com",
    "iap.googleapis.com",
    "secretmanager.googleapis.com",
    "servicenetworking.googleapis.com",
    "sqladmin.googleapis.com",
    "storage.googleapis.com",
  ])

  manifests_chart = "${path.module}/charts/manifests"

  # In-cluster DNS names. Services find the collector here; the collector
  # finds the Agent's node-local service and the Langfuse web service.
  collector_service       = "otel-collector"
  collector_host          = "${local.collector_service}.${local.namespace}.svc.cluster.local"
  collector_otlp_http     = "http://${local.collector_host}:4318"
  collector_otlp_grpc     = "${local.collector_host}:4317"
  datadog_agent_name      = "datadog"
  datadog_agent_otlp_grpc = "${local.datadog_agent_name}-agent.${local.namespace}.svc.cluster.local:4317"
  langfuse_release        = "langfuse"
  langfuse_web_service    = "${local.langfuse_release}-web"
  langfuse_web_url        = "http://${local.langfuse_web_service}.${local.namespace}.svc.cluster.local:3000"
  langfuse_otlp_endpoint  = "${local.langfuse_web_url}/api/public/otel"

  # What every instrumented service reads (crates/telemetry). Rendered as an
  # output so physician-runtime and later Cloud Run roots bind the same values.
  service_telemetry_env = {
    OTEL_EXPORTER_OTLP_ENDPOINT = local.collector_otlp_http
    DD_ENV                      = local.dd_env
    CODA_TELEMETRY_CONTENT      = var.telemetry_content
  }
}

data "google_project" "runtime" {
  project_id = var.project_id
}

data "google_compute_network" "runtime" {
  project = var.project_id
  name    = var.network_name
}

resource "google_project_service" "workload" {
  for_each = local.workload_services

  project = var.project_id
  service = each.value

  disable_on_destroy = false
}

# Langfuse, its stores, the collector, and the Datadog Agent share one
# namespace. Nothing in it is exposed outside the cluster by default.
resource "kubernetes_namespace" "observability" {
  metadata {
    name = local.namespace
    labels = {
      application = "observability"
    }
  }
}
