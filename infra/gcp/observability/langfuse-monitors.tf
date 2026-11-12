# Langfuse delivery monitors: the collector's otlphttp/langfuse exporter
# failing or queueing. Langfuse is a content sink outside the cluster, never
# on the product path, so these page engineering without implying a product
# outage.
variable "langfuse_monitors_enabled" {
  description = "Separate gate for the Langfuse exporter monitors. Needs langfuse_host, the gateway self-metric scrape and a confirmed alert handle; it does not enable the per-service monitors behind datadog_monitors_enabled."
  type        = bool
  default     = false
  validation {
    condition = !var.langfuse_monitors_enabled || (
      var.environment == "staging" && var.langfuse_host != null && var.collector_self_metrics_enabled &&
      var.alert_notify != "" && contains(["datadog", "both"], var.telemetry_backend)
    )
    error_message = "Langfuse monitors require staging, langfuse_host, collector self metrics, a confirmed alert handle and a Datadog backend."
  }
}

locals {
  langfuse_exporter       = "otlphttp/langfuse"
  langfuse_export_scope   = "${local.collector_export_scope},exporter:${local.langfuse_exporter}"
  langfuse_export_metrics = var.langfuse_monitors_enabled ? toset(["send_failed_spans", "enqueue_failed_spans"]) : toset([])
  langfuse_status_page    = "https://status.langfuse.com"
}

resource "datadog_monitor" "langfuse_export_failures" {
  for_each = local.langfuse_export_metrics
  name     = "[${local.dd_env}] Langfuse export ${each.key}"
  type     = "query alert"
  query    = "sum(last_5m):sum:coda.collector.exporter.${each.key}.count{${local.langfuse_export_scope}} by {exporter}.as_count() > 0"
  message  = <<-EOT
    Environment: ${local.dd_env}; service: otel-collector; destination: Langfuse (${coalesce(var.langfuse_host, "unset")}); owner: Coda engineering.
    {{#is_alert}}Symptom: the collector's ${local.langfuse_exporter} exporter reported ${each.key} in 5 minutes. Generations are missing from Langfuse; chats and the operations backends are unaffected.{{/is_alert}}
    Inspect [collector metrics](https://app.${var.datadog_site}/metric/explorer), the collector logs for the HTTP status Langfuse returned, the [Langfuse status page](${local.langfuse_status_page}) and the [runbook](${local.dependency_runbook}). A 401 means the Doppler key pair no longer matches the project. Lost spans are not replayed.
    {{#is_recovery}}Recovery: the exporter reported no ${each.key} for 5 minutes. Confirm a new generation reaches Langfuse before declaring delivery restored.{{/is_recovery}}
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
  tags                 = ["env:${local.dd_env}", "service:otel-collector", "destination:langfuse", "team:coda"]
}

resource "datadog_monitor" "langfuse_export_queue" {
  count = var.langfuse_monitors_enabled ? 1 : 0
  name  = "[${local.dd_env}] Langfuse export queue pressure"
  type  = "query alert"
  # Per collector replica, like collector_queue_pressure, so one replica's
  # empty queue cannot hide another's full one.
  query   = "min(last_5m):max:coda.collector.exporter.queue_size{${local.langfuse_export_scope}} by {collector_replica,exporter,data_type} / max:coda.collector.exporter.queue_capacity{${local.langfuse_export_scope}} by {collector_replica,exporter,data_type} > 0.8"
  message = <<-EOT
    Environment: ${local.dd_env}; service: otel-collector; destination: Langfuse (${coalesce(var.langfuse_host, "unset")}); owner: Coda engineering.
    {{#is_alert}}Symptom: the ${local.langfuse_exporter} {{data_type.name}} queue stayed above 80% of capacity for 5 minutes; new spans will be dropped once it is full.{{/is_alert}}
    Check [metrics](https://app.${var.datadog_site}/metric/explorer), the collector logs, the [Langfuse status page](${local.langfuse_status_page}) and the [runbook](${local.dependency_runbook}). Restore Langfuse ingestion before considering capacity; the queue stays bounded and never blocks chats.
    {{#is_recovery}}Recovery: the queue drained or this replica stopped reporting. Confirm new generations reach Langfuse.{{/is_recovery}}
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
  tags                 = ["env:${local.dd_env}", "service:otel-collector", "destination:langfuse", "team:coda"]
}
