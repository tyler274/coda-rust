# Imported GCP names are catalogued by Datadog, but resource tags must be
# verified on actual series before an operator supplies a scope and enables it.
# No fallback scope: a project-wide query could hide a failed dependency.
variable "dependency_metric_scopes" {
  description = "Exact verified Datadog series scopes, including env and resource identity, by dependency signal key. Empty until scoped series and units have been inspected. This is not an ingestion switch."
  type        = map(string)
  default     = {}
  validation {
    condition = alltrue([
      for key, scope in var.dependency_metric_scopes :
      contains(["physician_sql_up", "duet_sql_up", "physician_sql_storage", "duet_sql_storage", "redis_pressure", "redis_uptime", "worker_oldest_due", "worker_due", "worker_failed"], key) &&
      can(regex("(^|,)env:staging(,|$)", scope)) &&
      length(split(",", scope)) >= 3 &&
      !can(regex("[{}*]|patient|run_id|chat_id|conversation", scope))
    ])
    error_message = "Scopes require a known signal key, env:staging and verified project/resource tags; wildcards, expressions and clinical identifiers are forbidden."
  }
}

variable "dependency_monitors_enabled" {
  description = "Separate qualification gate for dependency monitors; requires the existing monitor gate too. Supply only scopes whose series/query and routing have been verified."
  type        = bool
  default     = false
  validation {
    condition     = !var.dependency_monitors_enabled || (var.environment == "staging" && var.datadog_monitors_enabled)
    error_message = "Dependency monitors require staging and the existing Datadog monitor authorization gate."
  }
}

variable "collector_availability_monitor_enabled" {
  description = "Enable the independently scoped collector deployment availability and telemetry absence pair after routing qualification. Does not measure exporter success or application readiness."
  type        = bool
  default     = false
  validation {
    condition     = !var.collector_availability_monitor_enabled || (var.environment == "staging" && var.datadog_monitors_enabled)
    error_message = "Collector monitors require staging and the existing Datadog monitor authorization gate."
  }
}

locals {
  dependency_runbook = "https://github.com/Coda-Health/coda-rust/blob/main/book/src/observability.md"
  # The worker signals are gauges each physician-worker instance writes every
  # minute from the Postgres rows that own the work (physician-worker
  # signals.rs). Work is due once its retry delay passes or its lease runs
  # out; a worker with room starts due work within one 10 s poll, so due work
  # still waiting after 300 s throughout five minutes means no worker is
  # taking it. Every instance reads the same rows, so queries take the max.
  # Age is continuous: a running worker writes it even when nothing is due,
  # so its absence means no worker is reporting. Failed work waits for the
  # physician's Try again, so the failed count stays up until then.
  dependency_signals = {
    physician_sql_up      = { service = "physician-sample", metric = "gcp.cloudsql.database.up", aggregation = "max", window = "5m", operator = "<", threshold = 1, continuous = true, symptom = "Cloud SQL reports unavailable" }
    duet_sql_up           = { service = "duet-api", metric = "gcp.cloudsql.database.up", aggregation = "max", window = "5m", operator = "<", threshold = 1, continuous = true, symptom = "Cloud SQL reports unavailable" }
    physician_sql_storage = { service = "physician-sample", metric = "gcp.cloudsql.database.disk.utilization", aggregation = "min", window = "15m", operator = ">", threshold = 0.85, continuous = true, symptom = "Cloud SQL disk fraction exceeds 0.85" }
    duet_sql_storage      = { service = "duet-api", metric = "gcp.cloudsql.database.disk.utilization", aggregation = "min", window = "15m", operator = ">", threshold = 0.85, continuous = true, symptom = "Cloud SQL disk fraction exceeds 0.85" }
    redis_pressure        = { service = "physician-sample", metric = "gcp.redis.stats.memory.usage_ratio", aggregation = "min", window = "15m", operator = ">", threshold = 0.85, continuous = true, symptom = "Redis memory fraction exceeds 0.85" }
    # Uptime is an availability heartbeat, not a reachability test. Only its
    # absence monitor is generated: a normal Redis restart is not an outage.
    redis_uptime      = { service = "physician-sample", metric = "gcp.redis.server.uptime", aggregation = "max", window = "5m", operator = "<", threshold = 0, continuous = true, symptom = "Redis uptime telemetry is unavailable" }
    worker_oldest_due = { service = "physician-worker", metric = "gcp.custom.coda.physician.queue.oldest_due_seconds", aggregation = "min", window = "5m", operator = ">", threshold = 300, continuous = true, symptom = "due physician work has waited more than 300 seconds" }
    worker_due        = { service = "physician-worker", metric = "gcp.custom.coda.physician.queue.due", aggregation = "min", window = "30m", operator = ">", threshold = 0, continuous = false, symptom = "due physician work persists" }
    worker_failed     = { service = "physician-worker", metric = "gcp.custom.coda.physician.queue.failed", aggregation = "min", window = "5m", operator = ">", threshold = 0, continuous = false, symptom = "physician work ended failed and waits for Try again" }
  }
  active_dependency_signals = {
    for key, signal in local.dependency_signals : key => signal
    if local.datadog_backend && var.dependency_monitors_enabled && contains(keys(var.dependency_metric_scopes), key)
  }
  collector_scope = "env:${local.dd_env},kube_cluster_name:${var.cluster_name},kube_namespace:${local.namespace},kube_deployment:otel-collector"
}

resource "datadog_monitor" "dependency_symptom" {
  for_each = { for key, signal in local.active_dependency_signals : key => signal if key != "redis_uptime" }
  name     = "[${local.dd_env}] ${each.key}: ${each.value.symptom}"
  type     = "query alert"
  query    = "${each.value.aggregation}(last_${each.value.window}):max:${each.value.metric}{${var.dependency_metric_scopes[each.key]}}.fill(null) ${each.value.operator} ${each.value.threshold}"
  message  = <<-EOT
    Environment: ${local.dd_env}; service: ${each.value.service}; owner: Coda engineering.
    {{#is_alert}}Symptom: ${each.value.symptom} throughout ${each.value.window}.{{/is_alert}}
    Investigate [GCP resources](https://console.cloud.google.com/home/dashboard?project=${var.cluster_project}) and [metrics](https://app.${var.datadog_site}/metric/explorer).
    Follow the [observability runbook](${local.dependency_runbook}). For queue alerts inspect canonical work before replaying; do not acknowledge or replay automatically. Redis uptime does not establish network reachability.
    {{#is_recovery}}Resolution: the alert condition cleared, or reporting stopped and the sparse queue alert auto-resolved. Check current metrics and canonical work before treating this as measured recovery; resolution is not proof of successful processing.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = each.value.threshold }
  # Sparse queues deliberately have no absence monitor: idle/empty is normal.
  notify_no_data       = false
  require_full_window  = true
  evaluation_delay     = 300
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = ["alert"]
  timeout_h            = each.value.continuous ? 0 : 1
  tags                 = ["env:${local.dd_env}", "service:${each.value.service}", "team:coda", "signal:${each.key}"]
}

resource "datadog_monitor" "dependency_absence" {
  for_each = { for key, signal in local.active_dependency_signals : key => signal if signal.continuous }
  name     = "[${local.dd_env}] ${each.key}: telemetry absent"
  type     = "query alert"
  # All these gauges are nonnegative. This threshold never fires for a real
  # sample, including SQL up=0; only this separate monitor owns NO DATA.
  query   = "max(last_5m):max:${each.value.metric}{${var.dependency_metric_scopes[each.key]}}.fill(null) < 0"
  message = <<-EOT
    Environment: ${local.dd_env}; service: ${each.value.service}; owner: Coda engineering.
    {{#is_no_data}}Symptom: continuously expected ${each.key} telemetry absent for 20 minutes. This is monitoring loss, not a proven dependency outage.{{/is_no_data}}
    Check [GCP monitoring](https://console.cloud.google.com/monitoring?project=${var.cluster_project}), [Datadog metrics](https://app.${var.datadog_site}/metric/explorer) and the [runbook](${local.dependency_runbook}). Check integration permissions, resource scope and ingestion delay before changing a dependency.
    {{#is_recovery}}Recovery: telemetry resumed. Verify actual health separately.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = 0 }
  notify_no_data       = true
  no_data_timeframe    = 20
  require_full_window  = false
  evaluation_delay     = 300
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = ["no data"]
  tags                 = ["env:${local.dd_env}", "service:${each.value.service}", "team:coda", "signal:${each.key}"]
}

resource "datadog_monitor" "collector_availability" {
  for_each = local.datadog_backend && var.collector_availability_monitor_enabled ? toset(["unavailable", "telemetry_absent"]) : toset([])
  name     = "[${local.dd_env}] otel-collector ${each.key}"
  type     = "query alert"
  query    = "max(last_5m):min:kubernetes_state.deployment.replicas_available{${local.collector_scope}}.fill(null) < ${each.key == "unavailable" ? 1 : 0}"
  message  = <<-EOT
    Environment: ${local.dd_env}; service: otel-collector; owner: Coda engineering.
    {{#is_alert}}Symptom: no available gateway replica for 5 minutes. Application requests are not blocked by the exporter.{{/is_alert}}
    {{#is_no_data}}Symptom: collector deployment telemetry absent for 10 minutes. Check the Datadog Agent and cluster; this is not proof that the collector is down.{{/is_no_data}}
    Inspect [GKE workloads](https://console.cloud.google.com/kubernetes/workload/overview?project=${var.cluster_project}), [Datadog metrics](https://app.${var.datadog_site}/metric/explorer), and the [runbook](${local.dependency_runbook}). Check gateway endpoints and exporter queues; do not resize nodes from host memory alarms.
    {{#is_recovery}}Recovery: deployment availability telemetry resumed or an available replica returned. Verify export delivery separately.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = each.key == "unavailable" ? 1 : 0 }
  notify_no_data       = each.key == "telemetry_absent"
  no_data_timeframe    = each.key == "telemetry_absent" ? 10 : null
  require_full_window  = each.key == "unavailable"
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = each.key == "unavailable" ? ["alert"] : ["no data"]
  timeout_h            = 0
  tags                 = ["env:${local.dd_env}", "service:otel-collector", "team:coda"]
}
