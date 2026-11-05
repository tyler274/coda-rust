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

  # Cloud Run sidecars reach the collector only through the internal gateway
  # (collector.tf); the ClusterIP names above serve in-cluster callers.
  otel_gateway_otlp_http = "http://${var.otel_gateway_ip}:4318"

  # What every instrumented Cloud Run product container reads
  # (crates/telemetry). The product exports to its own sidecar on localhost;
  # the sidecar forwards to otel_gateway_otlp_http with a Google ID token.
  # infra/gcp/rust-environments sets these on the product containers;
  # DD_VERSION is baked into each image, never set here.
  service_telemetry_env = {
    OTEL_EXPORTER_OTLP_ENDPOINT = "http://localhost:4318"
    DD_ENV                      = local.dd_env
    CODA_LOG_FORMAT             = "json"
    CODA_TELEMETRY_CONTENT      = var.telemetry_content
  }
}

data "google_project" "runtime" {
  project_id = var.project_id
}

# Read only: infra/gcp/rust-environments owns rust-private and its Private
# Service Access range, which Langfuse's Cloud SQL attaches to.
data "google_compute_network" "runtime" {
  project = var.cluster_project
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
