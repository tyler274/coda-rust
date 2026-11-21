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
      contains(keys(local.dependency_signals), key) &&
      can(regex("(^|,)env:${var.environment}(,|$)", scope)) &&
      length(split(",", scope)) >= 3 &&
      !can(regex("[{}*]|patient|run_id|chat_id|conversation", scope))
    ])
    error_message = "Scopes require a known signal key, this environment's env tag and verified project/resource tags; wildcards, expressions and clinical identifiers are forbidden."
  }
  # The worker gauges are custom metrics on the global resource, which the
  # GCP integration imports only with global-location collection on; a scope
  # is supplied once their series has been observed in Datadog.
  # A continuous signal without its own absence monitor relies on its
  # resource's heartbeat to report telemetry loss, so it is scoped only with it.
  validation {
    condition = alltrue([
      for key in keys(var.dependency_metric_scopes) :
      contains(keys(var.dependency_metric_scopes), local.dependency_heartbeats[key]) if contains(keys(local.dependency_heartbeats), key)
    ])
    error_message = "A Cloud SQL or Redis signal requires its resource's heartbeat scope as well: *_sql_up for the instance, redis_uptime for Redis."
  }
  validation {
    condition     = var.datadog_gcp_global_location_enabled || length([for key in keys(var.dependency_metric_scopes) : key if startswith(key, "worker_")]) == 0
    error_message = "Worker queue scopes require datadog_gcp_global_location_enabled, which imports the worker's global custom gauges."
  }
}

variable "dependency_monitors_enabled" {
  description = "Own gate for the dependency monitors whose scopes dependency_metric_scopes supplies. Independent of datadog_monitors_enabled: it does not enable the 5xx, latency, async-failure or memory monitors, and they do not enable it. Supply only scopes whose series, query and routing have been verified."
  type        = bool
  default     = false
  validation {
    condition     = !var.dependency_monitors_enabled || (var.alert_notify != "" && contains(["datadog", "both"], var.telemetry_backend))
    error_message = "Dependency monitors require telemetry_backend datadog or both, and an explicitly confirmed alert handle."
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
  dependency_runbook = "https://github.com/tyler274/coda-rust/blob/main/book/src/observability.md"
  # The worker signals are gauges each physician-worker instance writes every
  # minute from the Postgres rows that own the work (physician-worker
  # signals.rs). Work is due once its retry delay passes or its lease runs
  # out; a worker with room starts due work within one 10 s poll, so due work
  # still waiting after 300 s throughout five minutes means no worker is
  # taking it. Every instance reads the same rows, so queries take the max.
  # Age is continuous: a running worker writes it even when nothing is due,
  # so its absence means no worker is reporting. Failed work waits for the
  # physician's Try again, so the failed count stays up until then.
  #
  # space is how the scope's series combine into one value: Cloud SQL's
  # num_backends is one series per database on the instance, so the
  # instance's connections are their sum. absence marks the one signal per
  # resource that owns a telemetry-absence monitor: every Cloud SQL metric of
  # an instance arrives through the same integration, so only *_sql_up
  # reports its loss, and one integration outage pages once per instance;
  # redis_uptime does the same for the Redis instance. Absence applies only
  # to continuous signals: sparse ones are legitimately silent.
  # The CPU and memory thresholds hold for 15 minutes so a vacuum or burst on
  # the shared-core tier does not page. 80 connections is 80% of the
  # max_connections = 100 flag in infra/gcp/rust-environments/database.tf.
  dependency_signals = {
    physician_sql_up          = { service = "physician-sample", metric = "gcp.cloudsql.database.up", aggregation = "max", space = "max", window = "5m", operator = "<", threshold = 1, continuous = true, absence = true, symptom = "Cloud SQL reports unavailable" }
    duet_sql_up               = { service = "duet-api", metric = "gcp.cloudsql.database.up", aggregation = "max", space = "max", window = "5m", operator = "<", threshold = 1, continuous = true, absence = true, symptom = "Cloud SQL reports unavailable" }
    physician_sql_storage     = { service = "physician-sample", metric = "gcp.cloudsql.database.disk.utilization", aggregation = "min", space = "max", window = "15m", operator = ">", threshold = 0.85, continuous = true, absence = false, symptom = "Cloud SQL disk fraction exceeds 0.85" }
    duet_sql_storage          = { service = "duet-api", metric = "gcp.cloudsql.database.disk.utilization", aggregation = "min", space = "max", window = "15m", operator = ">", threshold = 0.85, continuous = true, absence = false, symptom = "Cloud SQL disk fraction exceeds 0.85" }
    physician_sql_cpu         = { service = "physician-sample", metric = "gcp.cloudsql.database.cpu.utilization", aggregation = "min", space = "max", window = "15m", operator = ">", threshold = 0.9, continuous = true, absence = false, symptom = "Cloud SQL CPU fraction exceeds 0.9" }
    duet_sql_cpu              = { service = "duet-api", metric = "gcp.cloudsql.database.cpu.utilization", aggregation = "min", space = "max", window = "15m", operator = ">", threshold = 0.9, continuous = true, absence = false, symptom = "Cloud SQL CPU fraction exceeds 0.9" }
    physician_sql_memory      = { service = "physician-sample", metric = "gcp.cloudsql.database.memory.utilization", aggregation = "min", space = "max", window = "15m", operator = ">", threshold = 0.9, continuous = true, absence = false, symptom = "Cloud SQL memory fraction exceeds 0.9" }
    duet_sql_memory           = { service = "duet-api", metric = "gcp.cloudsql.database.memory.utilization", aggregation = "min", space = "max", window = "15m", operator = ">", threshold = 0.9, continuous = true, absence = false, symptom = "Cloud SQL memory fraction exceeds 0.9" }
    physician_sql_connections = { service = "physician-sample", metric = "gcp.cloudsql.database.postgresql.num_backends", aggregation = "min", space = "sum", window = "15m", operator = ">", threshold = 80, continuous = true, absence = false, symptom = "Cloud SQL connections exceed 80 of 100" }
    duet_sql_connections      = { service = "duet-api", metric = "gcp.cloudsql.database.postgresql.num_backends", aggregation = "min", space = "sum", window = "15m", operator = ">", threshold = 80, continuous = true, absence = false, symptom = "Cloud SQL connections exceed 80 of 100" }
    redis_pressure            = { service = "physician-sample", metric = "gcp.redis.stats.memory.usage_ratio", aggregation = "min", space = "max", window = "15m", operator = ">", threshold = 0.85, continuous = true, absence = false, symptom = "Redis memory fraction exceeds 0.85" }
    # Uptime is an availability heartbeat, not a reachability test. Only its
    # absence monitor is generated: a normal Redis restart is not an outage.
    redis_uptime      = { service = "physician-sample", metric = "gcp.redis.server.uptime", aggregation = "max", space = "max", window = "5m", operator = "<", threshold = 0, continuous = true, absence = true, symptom = "Redis uptime telemetry is unavailable" }
    worker_oldest_due = { service = "physician-worker", metric = "gcp.custom.coda.physician.queue.oldest_due_seconds", aggregation = "min", space = "max", window = "5m", operator = ">", threshold = 300, continuous = true, absence = true, symptom = "due physician work has waited more than 300 seconds" }
    worker_due        = { service = "physician-worker", metric = "gcp.custom.coda.physician.queue.due", aggregation = "min", space = "max", window = "30m", operator = ">", threshold = 0, continuous = false, absence = false, symptom = "due physician work persists" }
    worker_failed     = { service = "physician-worker", metric = "gcp.custom.coda.physician.queue.failed", aggregation = "min", space = "max", window = "5m", operator = ">", threshold = 0, continuous = false, absence = false, symptom = "physician work ended failed and waits for Try again" }
  }
  # The absence-owning signal each other continuous signal of the same
  # resource relies on.
  dependency_heartbeats = merge(
    { for key, signal in local.dependency_signals : key => "physician_sql_up" if startswith(key, "physician_sql_") && !signal.absence },
    { for key, signal in local.dependency_signals : key => "duet_sql_up" if startswith(key, "duet_sql_") && !signal.absence },
    { redis_pressure = "redis_uptime" },
  )
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
  query    = "${each.value.aggregation}(last_${each.value.window}):${each.value.space}:${each.value.metric}{${var.dependency_metric_scopes[each.key]}}.fill(null) ${each.value.operator} ${each.value.threshold}"
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
  for_each = { for key, signal in local.active_dependency_signals : key => signal if signal.absence }
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
