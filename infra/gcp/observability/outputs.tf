output "namespace" {
  value = kubernetes_namespace.observability.metadata[0].name
}

output "collector_otlp_http_endpoint" {
  description = "In-cluster OTLP/HTTP address (ClusterIP). Not reachable from Cloud Run; sidecars use otel_gateway."
  value       = local.collector_otlp_http
}

output "collector_otlp_grpc_endpoint" {
  value = local.collector_otlp_grpc
}

output "otel_gateway" {
  description = "Where Cloud Run sidecars send OTLP/HTTP and what their Google ID tokens must carry. Echoes the rust-environments inputs so both roots can be checked against each other."
  value = {
    endpoint        = local.otel_gateway_otlp_http
    audience        = var.otel_gateway_audience
    source_ranges   = var.otel_gateway_source_ranges
    allowed_callers = sort(distinct(var.otel_gateway_allowed_callers))
  }
}

output "service_telemetry_env" {
  description = "Environment the Cloud Run product containers set when observability is on (infra/gcp/rust-environments binds the same values). DD_VERSION is baked into each image."
  value       = local.service_telemetry_env
}

output "datadog_gcp_integration" {
  description = "Read-only identity Datadog impersonates for the cluster project's Cloud Run and GKE metrics (null while datadog_gcp_integration_enabled is off)."
  value = local.datadog_gcp_enabled ? {
    service_account = google_service_account.datadog_gcp["integration"].email
    project         = var.cluster_project
  } : null
}

output "langfuse_otlp_endpoint" {
  description = "Where the gateway sends generation spans: the Langfuse project's OTLP endpoint. Null while langfuse_host is unset."
  value       = local.langfuse_otlp_endpoint
}

output "telemetry_backend" {
  description = "The operations backend this root deploys behind the gateway: datadog or grafana."
  value       = var.telemetry_backend
}

# Null under grafana, which has no Datadog monitors.
output "datadog_monitor_ids" {
  value = local.datadog_backend ? {
    service_5xx_rate       = { for service, monitor in datadog_monitor.site_5xx_rate : service => monitor.id }
    chat_generation_errors = [for monitor in datadog_monitor.chat_generation_errors : monitor.id]
    service_p95_latency    = { for service, monitor in datadog_monitor.site_p95_latency : service => monitor.id }
    async_failures         = { for service, monitor in datadog_monitor.async_failures : service => monitor.id }
    container_memory       = [for monitor in datadog_monitor.observability_container_memory : monitor.id]
    container_oom          = [for monitor in datadog_monitor.observability_container_oom : monitor.id]
    qualification          = [for monitor in datadog_monitor.qualification : monitor.id]
  } : null
}

# The Grafana outputs are null under datadog.
output "grafana" {
  description = "In-cluster Grafana stack (telemetry_backend = grafana or both). People reach Grafana at <staging_proxy origin>/grafana/ when staging_proxy is set, otherwise with kubectl port-forward svc/grafana 3000."
  value = local.grafana_backend ? {
    url                   = local.grafana_url
    tempo_otlp_grpc       = local.tempo_otlp_grpc
    loki_otlp_http        = local.loki_otlp_http
    prometheus_url        = local.prometheus_url
    cloud_monitoring_gsa  = one(google_service_account.grafana_monitoring[*].email)
    traces_retention_days = var.grafana_traces_retention_days
    logs_retention_days   = var.grafana_logs_retention_days
    metrics_retention     = local.prometheus_retention
  } : null
}

# Null until staging_proxy is set.
output "staging_proxy" {
  description = "Where people reach Grafana through public-site, and the internal load balancer address the relay forwards to."
  value = var.staging_proxy == null ? null : {
    grafana_url = local.grafana_backend ? local.grafana_public_url : null
    internal    = { for name, lb in local.observability_web : name => "http://${lb.ip}:3000/${name}/" }
  }
}
