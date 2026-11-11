variable "observability_workload_targets" {
  description = "Qualified stable, continuously expected GKE workloads in this namespace. Empty until actual series and intended minimum availability are verified; omit intentionally scaled-to-zero workloads. otel-collector uses its existing monitor pair instead."
  type = map(object({
    kind          = string
    minimum_ready = number
  }))
  default = {}
  validation {
    condition = alltrue([for name, target in var.observability_workload_targets :
      can(regex("^[a-z][a-z0-9-]{0,62}$", name)) && name != "otel-collector" &&
      contains(["deployment", "statefulset"], target.kind) && target.minimum_ready >= 1 && floor(target.minimum_ready) == target.minimum_ready
    ])
    error_message = "Targets must be stable deployment/statefulset names other than otel-collector with a positive integral minimum."
  }
}

variable "observability_pvc_scopes" {
  description = "Exact qualified PVC identity tag(s), excluding env/cluster/namespace supplied here. No PVC is presumed to exist. Verify kubelet used/capacity bytes, matching identity tags and sampling before configuring."
  type        = map(string)
  default     = {}
  validation {
    condition = alltrue([for name, scope in var.observability_pvc_scopes :
      can(regex("^[a-z][a-z0-9-]{0,62}$", name)) && can(regex("^[a-z_]+:[a-z0-9][a-z0-9_.-]*$", scope)) &&
      !can(regex("patient|run_id|chat_id|conversation|env:|namespace:|cluster", scope))
    ])
    error_message = "Supply one verified exact nonclinical PVC identity tag per target, not environment/cluster/namespace or wildcard scopes."
  }
}

variable "observability_workload_monitors_enabled" {
  description = "Separate gate after scoped workload/PVC signals and routing have been qualified; empty target maps generate no monitors. Uses the existing Agent Kubernetes State Core and kubelet checks, not a new scraper."
  type        = bool
  default     = false
  validation {
    condition     = !var.observability_workload_monitors_enabled || (var.environment == "staging" && var.datadog_monitors_enabled)
    error_message = "Workload monitors require staging and existing Datadog monitor authorization."
  }
}

locals {
  workload_scope = "env:${local.dd_env},kube_cluster_name:${var.cluster_name},kube_namespace:${local.namespace}"
  # Operator-qualified targets under their own gate.
  monitored_workloads = var.observability_workload_monitors_enabled ? var.observability_workload_targets : {}
  workload_signals = {
    for name, target in local.monitored_workloads : name => {
      metric  = target.kind == "deployment" ? "kubernetes_state.deployment.replicas_available" : "kubernetes_state.statefulset.replicas_ready"
      scope   = "${local.workload_scope},${target.kind == "deployment" ? "kube_deployment" : "kube_stateful_set"}:${name}"
      minimum = target.minimum_ready
    }
  }
  workload_rules = { for rule in flatten([for name, target in local.workload_signals : [
    for absence in [false, true] : { key = "${name}/${absence ? "absence" : "availability"}", name = name, target = target, absence = absence }
  ]]) : rule.key => rule }
  pvc_rules = { for rule in flatten([for name, scope in var.observability_pvc_scopes : [
    for mode in ["pressure", "used_absence", "capacity_absence"] : { key = "${name}/${mode}", name = name, scope = "${local.workload_scope},${scope}", mode = mode }
  ]]) : rule.key => rule if var.observability_workload_monitors_enabled }
}

resource "datadog_monitor" "workload_health" {
  for_each = local.workload_rules
  name     = "[${local.dd_env}] ${each.key}"
  type     = "query alert"
  query    = "max(last_5m):max:${each.value.target.metric}{${each.value.target.scope}}.fill(null) < ${each.value.absence ? 0 : each.value.target.minimum}"
  message  = <<-EOT
    Environment: ${local.dd_env}; service: ${each.value.name}; owner: Coda engineering.
    {{#is_alert}}Symptom: fewer than ${each.value.target.minimum} ready/available replicas for 5 minutes.{{/is_alert}}
    {{#is_no_data}}Symptom: continuously expected workload telemetry absent for 10 minutes. This is not proof of workload failure.{{/is_no_data}}
    Inspect [GKE workloads](https://console.cloud.google.com/kubernetes/workload/overview?project=${var.cluster_project}), [metrics](https://app.${var.datadog_site}/metric/explorer) and the [runbook](${local.dependency_runbook}). Check rollout, scheduling and dependency readiness. Remove this target from configuration when deliberately retiring/scaling it to zero; never retain per-node groups.
    {{#is_recovery}}Recovery: measured availability returned or telemetry resumed for the absence monitor. Verify the workload's actual function separately.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = each.value.absence ? 0 : each.value.target.minimum }
  notify_no_data       = each.value.absence
  no_data_timeframe    = each.value.absence ? 10 : null
  require_full_window  = !each.value.absence
  timeout_h            = 0
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = each.value.absence ? ["no data"] : ["alert"]
  tags                 = ["env:${local.dd_env}", "service:${each.value.name}", "team:coda"]
}

resource "datadog_monitor" "pvc_storage" {
  for_each = local.pvc_rules
  name     = "[${local.dd_env}] PVC ${each.key}"
  type     = "query alert"
  query    = each.value.mode == "pressure" ? "min(last_15m):max:kubernetes.kubelet.volume.stats.used_bytes{${each.value.scope}}.fill(null) / max:kubernetes.kubelet.volume.stats.capacity_bytes{${each.value.scope}}.fill(null) > 0.85" : "max(last_5m):max:kubernetes.kubelet.volume.stats.${each.value.mode == "used_absence" ? "used" : "capacity"}_bytes{${each.value.scope}}.fill(null) < 0"
  message  = <<-EOT
    Environment: ${local.dd_env}; service: observability-storage; PVC: ${each.value.name}; owner: Coda engineering.
    {{#is_alert}}Symptom: volume used/capacity ratio above 85% for 15 minutes.{{/is_alert}}
    {{#is_no_data}}Symptom: ${each.value.mode} telemetry absent for 10 minutes. Missing volume telemetry is not zero utilization.{{/is_no_data}}
    Inspect [GKE storage](https://console.cloud.google.com/kubernetes/persistentvolume?project=${var.cluster_project}), [metrics](https://app.${var.datadog_site}/metric/explorer) and the [runbook](${local.dependency_runbook}). Check retention and the PVC/CSI volume stats before an approved expansion; do not delete data to silence an alert. Remove retired claims from the qualified target map.
    {{#is_recovery}}Recovery: measured storage pressure cleared or the missing metric resumed. Confirm both used and capacity series before declaring storage healthy.{{/is_recovery}}
    ${var.alert_notify}
  EOT
  monitor_thresholds { critical = each.value.mode == "pressure" ? 0.85 : 0 }
  notify_no_data       = each.value.mode != "pressure"
  no_data_timeframe    = each.value.mode == "pressure" ? null : 10
  require_full_window  = each.value.mode == "pressure"
  timeout_h            = 0
  renotify_interval    = 60
  renotify_occurrences = 2
  renotify_statuses    = each.value.mode == "pressure" ? ["alert"] : ["no data"]
  tags                 = ["env:${local.dd_env}", "service:observability-storage", "team:coda"]
}
