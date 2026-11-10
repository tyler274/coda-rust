variable "collector_self_metrics_enabled" {
  description = "Opt-in direct Datadog Agent scrape of gateway self metrics. Adds no application metrics pipeline and no public service port. Billable custom metrics: seven explicitly mapped families with exporter/data_type labels only."
  type        = bool
  default     = false
  validation {
    condition     = !var.collector_self_metrics_enabled || (var.environment == "staging" && contains(["datadog", "both"], var.telemetry_backend))
    error_message = "Collector self metrics are a staging Datadog collection path."
  }
}

variable "collector_export_monitors_enabled" {
  description = "Separate operator gate after the mapped counter/gauge series, tags, queries and routing are qualified. Collection alone does not enable these monitors."
  type        = bool
  default     = false
  validation {
    condition     = !var.collector_export_monitors_enabled || (var.collector_self_metrics_enabled && var.datadog_monitors_enabled)
    error_message = "Export monitors require self-metric collection and the existing monitor gate."
  }
}

locals {
  # Names and types from Collector v0.155.0 exporterhelper/metadata.yaml,
  # corroborated by a synthetic scrape of the pinned image. OpenMetrics maps
  # counters to <name>.count; uptime is deliberately overridden to a gauge.
  collector_metric_mapping = {
    otelcol_exporter_send_failed_spans          = { name = "exporter.send_failed_spans", type = "counter" }
    otelcol_exporter_send_failed_log_records    = { name = "exporter.send_failed_log_records", type = "counter" }
    otelcol_exporter_enqueue_failed_spans       = { name = "exporter.enqueue_failed_spans", type = "counter" }
    otelcol_exporter_enqueue_failed_log_records = { name = "exporter.enqueue_failed_log_records", type = "counter" }
    otelcol_exporter_queue_size                 = { name = "exporter.queue_size", type = "gauge" }
    otelcol_exporter_queue_capacity             = { name = "exporter.queue_capacity", type = "gauge" }
    otelcol_process_uptime                      = { name = "uptime", type = "gauge" }
  }
  collector_metrics_annotations = var.collector_self_metrics_enabled ? {
    "ad.datadoghq.com/otel-collector.checks" = jsonencode({
      openmetrics = {
        init_config = {}
        instances = [{
          openmetrics_endpoint    = "http://%%host%%:8888/metrics"
          namespace               = "coda.collector"
          metrics                 = [for source, mapping in local.collector_metric_mapping : { (source) = mapping }]
          include_labels          = ["exporter", "data_type"]
          tag_by_endpoint         = false
          min_collection_interval = 15
          tags                    = ["env:${local.dd_env}", "service:otel-collector", "kube_cluster_name:${var.cluster_name}", "kube_namespace:${local.namespace}", "collector_replica:%%host%%"]
        }]
      }
    })
  } : {}
  collector_export_scope = "env:${local.dd_env},service:otel-collector,kube_cluster_name:${var.cluster_name},kube_namespace:${local.namespace}"
  collector_failure_metrics = toset([
    "send_failed_spans", "send_failed_log_records", "enqueue_failed_spans", "enqueue_failed_log_records",
  ])
}

resource "datadog_monitor" "collector_export_failures" {
  for_each = var.collector_export_monitors_enabled ? local.collector_failure_metrics : toset([])
  name     = "[${local.dd_env}] otel-collector ${each.key}"
  type     = "query alert"
  query    = "sum(last_5m):sum:coda.collector.exporter.${each.key}.count{${local.collector_export_scope}} by {exporter}.as_count() > 0"
  message  = <<-EOT
    Environment: ${local.dd_env}; service: otel-collector; owner: Coda engineering.
    {{#is_alert}}Symptom: exporter {{exporter.name}} reported ${each.key} in 5 minutes. Send failures include retry attempts; enqueue failures mean telemetry could not enter the bounded queue.{{/is_alert}}
    Inspect [collector metrics](https://app.${var.datadog_site}/metric/explorer) and [GKE workloads](https://console.cloud.google.com/kubernetes/workload/overview?project=${var.cluster_project}); follow the [runbook](${local.dependency_runbook}). Check destination connectivity and queue pressure without blocking product requests or replaying clinical work.
    {{#is_recovery}}Resolution: no failures remain in the evaluation window or reporting ceased. Verify the separate scrape heartbeat and delivery before declaring recovery.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = 0 }
  notify_no_data       = false
  require_full_window  = false
  timeout_h            = 1
  new_group_delay      = 60
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = ["alert"]
  tags                 = ["env:${local.dd_env}", "service:otel-collector", "team:coda"]
}

resource "datadog_monitor" "collector_queue_pressure" {
  count = var.collector_export_monitors_enabled ? 1 : 0
  name  = "[${local.dd_env}] otel-collector export queue pressure"
  type  = "query alert"
  # Per-pod arithmetic avoids hiding one saturated replica with another's
  # empty queue. Only this transient pressure rule groups by pod; it expires
  # retired groups and is NOT a pod absence monitor.
  query   = "min(last_5m):max:coda.collector.exporter.queue_size{${local.collector_export_scope}} by {collector_replica,exporter,data_type} / max:coda.collector.exporter.queue_capacity{${local.collector_export_scope}} by {collector_replica,exporter,data_type} > 0.8"
  message = <<-EOT
    Environment: ${local.dd_env}; service: otel-collector; owner: Coda engineering.
    {{#is_alert}}Symptom: exporter {{exporter.name}} {{data_type.name}} queue above 80% capacity for 5 minutes.{{/is_alert}}
    Check [metrics](https://app.${var.datadog_site}/metric/explorer), [GKE workloads](https://console.cloud.google.com/kubernetes/workload/overview?project=${var.cluster_project}) and the [runbook](${local.dependency_runbook}). Restore destination connectivity before considering capacity; exporter queues must stay bounded and nonblocking.
    {{#is_recovery}}Resolution: pressure cleared or this pod stopped reporting. Verify the stable collector availability/heartbeat signals and end-to-end export before declaring delivery recovered.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = 0.8 }
  notify_no_data       = false
  require_full_window  = true
  timeout_h            = 1
  new_group_delay      = 300
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = ["alert"]
  tags                 = ["env:${local.dd_env}", "service:otel-collector", "team:coda"]
}

resource "datadog_monitor" "collector_scrape_absence" {
  count   = var.collector_export_monitors_enabled ? 1 : 0
  name    = "[${local.dd_env}] otel-collector self metrics absent"
  type    = "query alert"
  query   = "max(last_5m):max:coda.collector.uptime{${local.collector_export_scope}}.fill(null) < 0"
  message = <<-EOT
    Environment: ${local.dd_env}; service: otel-collector; owner: Coda engineering.
    {{#is_no_data}}Symptom: no gateway self-metric scrape for 10 minutes. Export failure counters are sparse; their absence is not an outage.{{/is_no_data}}
    Inspect [metrics](https://app.${var.datadog_site}/metric/explorer), [GKE workloads](https://console.cloud.google.com/kubernetes/workload/overview?project=${var.cluster_project}) and the [runbook](${local.dependency_runbook}). Check Agent autodiscovery, port 8888 access and the collector telemetry reader. This signal shares Datadog's failure domain, not an independent watchdog.
    {{#is_recovery}}Recovery: a gateway scrape resumed. Verify each replica and export delivery separately.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = 0 }
  notify_no_data       = true
  no_data_timeframe    = 10
  require_full_window  = false
  timeout_h            = 0
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = ["no data"]
  tags                 = ["env:${local.dd_env}", "service:otel-collector", "team:coda"]
}
