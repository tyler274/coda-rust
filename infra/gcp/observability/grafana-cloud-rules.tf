locals {
  grafana_cloud_selectors = {
    physician_sql_up      = "cloudsql_googleapis_com:database_up{project_id=\"${var.cluster_project}\",database_id=\"${var.cluster_project}:rust-physician\"}"
    duet_sql_up           = "cloudsql_googleapis_com:database_up{project_id=\"${var.cluster_project}\",database_id=\"${var.cluster_project}:rust-duet\"}"
    physician_sql_storage = "cloudsql_googleapis_com:database_disk_utilization{project_id=\"${var.cluster_project}\",database_id=\"${var.cluster_project}:rust-physician\"}"
    duet_sql_storage      = "cloudsql_googleapis_com:database_disk_utilization{project_id=\"${var.cluster_project}\",database_id=\"${var.cluster_project}:rust-duet\"}"
    redis_pressure        = "redis_googleapis_com:stats_memory_usage_ratio{project_id=\"${var.cluster_project}\",instance_id=\"projects/${var.cluster_project}/locations/${var.cluster_location}/instances/rust-physician-cache\",role=\"primary\"}"
    redis_uptime          = "redis_googleapis_com:server_uptime{project_id=\"${var.cluster_project}\",instance_id=\"projects/${var.cluster_project}/locations/${var.cluster_location}/instances/rust-physician-cache\",role=\"primary\"}"
    worker_oldest_due     = "custom_googleapis_com:coda_physician_queue_oldest_due_seconds{${local.grafana_probe_labels}}"
    worker_due            = "custom_googleapis_com:coda_physician_queue_due{${local.grafana_probe_labels}}"
    worker_failed         = "custom_googleapis_com:coda_physician_queue_failed{${local.grafana_probe_labels}}"
  }
  # Custom metrics can sit on many resource types, so Cloud Monitoring PromQL
  # rejects them without a monitored_resource matcher. availability-probe and
  # the physician worker's queue gauges both write on the global resource.
  grafana_probe_labels = "monitored_resource=\"global\",project_id=\"${var.cluster_project}\""
  grafana_cloud_conditions = merge(
    { for target in ["physician", "duet", "physician-worker"] : "readiness-${target}" => {
      expr     = "max(custom_googleapis_com:coda_availability_firing{${local.grafana_probe_labels},target=\"${target}\"})"
      duration = "0s"
      summary  = "${target} failed three scheduled probes; clears after two successes. Missing execution is not recovery."
      no_data  = "KeepLast"
    } },
    { for target in ["physician", "duet", "physician-worker"] : "freshness-${target}" => {
      expr     = "(time() - max(custom_googleapis_com:coda_availability_observed_at{${local.grafana_probe_labels},target=\"${target}\"}) > bool 300) or vector(1)"
      duration = "0s"
      summary  = "${target} observation is absent or older than five minutes."
      no_data  = "NoData"
    } },
    { for key, signal in local.dependency_signals : "dependency-${key}" => {
      expr     = "max(${local.grafana_cloud_selectors[key]}) ${signal.operator} bool ${signal.threshold}"
      duration = signal.window
      summary  = signal.symptom
      no_data  = signal.continuous ? "KeepLast" : "OK"
    } if key != "redis_uptime" },
    { for key, signal in local.dependency_signals : "absence-${key}" => {
      expr     = "max(absent_over_time(${local.grafana_cloud_selectors[key]}[20m])) or vector(0)"
      duration = "0s"
      summary  = "${key} continuously expected telemetry has been absent for 20 minutes."
      no_data  = "NoData"
    } if signal.continuous }
  )
  grafana_cloud_rule_uids = [for key in keys(local.grafana_cloud_conditions) : "gcp-${key}"]
  # Provisioning keeps a rule whose UID leaves the file, so the Pub/Sub
  # subscription rules the queue gauges replaced are deleted by name.
  grafana_cloud_retired_rule_uids = ["gcp-dependency-worker_age", "gcp-absence-worker_age", "gcp-dependency-worker_backlog", "gcp-dependency-worker_dead_letters"]
  grafana_cloud_rules = yamlencode(local.grafana_monitoring_enable ? {
    apiVersion = 1
    groups = [{
      orgId    = 1
      name     = "coda-cloud-shadow"
      folder   = "Coda"
      interval = "1m"
      rules = [for key, signal in local.grafana_cloud_conditions : {
        uid          = "gcp-${key}"
        title        = "Cloud monitoring ${key}"
        condition    = "C"
        for          = signal.duration
        noDataState  = signal.no_data
        execErrState = "Error"
        labels       = { team = "coda", mode = "shadow", environment = var.environment, signal = key }
        annotations  = { summary = signal.summary, runbook_url = local.dependency_runbook }
        data = [
          {
            refId             = "A"
            datasourceUid     = "cloud-monitoring"
            relativeTimeRange = { from = 600, to = 0 }
            model = {
              refId     = "A"
              queryType = "promQL"
              # stackdriver 12.6.4 otherwise migrates away promQLQuery before dispatch.
              # https://github.com/grafana/grafana-cloudmonitoring-datasource/blob/v12.6.4/pkg/cloudmonitoring/cloudmonitoring.go#L307-L334
              timeSeriesList = {}
              promQLQuery    = { projectName = var.cluster_project, expr = signal.expr, step = "60s" }
            }
          },
          {
            refId             = "B"
            datasourceUid     = "__expr__"
            relativeTimeRange = { from = 0, to = 0 }
            model             = { refId = "B", type = "reduce", expression = "A", reducer = "last" }
          },
          {
            refId             = "C"
            datasourceUid     = "__expr__"
            relativeTimeRange = { from = 0, to = 0 }
            model             = { refId = "C", type = "threshold", expression = "B", conditions = [{ evaluator = { type = "gt", params = [0] } }] }
          }
        ]
      }]
    }]
    deleteRules = [for uid in local.grafana_cloud_retired_rule_uids : { orgId = 1, uid = uid }]
    } : {
    apiVersion  = 1
    groups      = []
    deleteRules = [for uid in concat(local.grafana_cloud_rule_uids, local.grafana_cloud_retired_rule_uids) : { orgId = 1, uid = uid }]
  })
}
