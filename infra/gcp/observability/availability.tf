# Datadog remains the only primary service alert route. The probe computes
# exact consecutive-result hysteresis; a time-window average of raw failures
# would not implement three failures / two successes.
variable "availability_firing_metric" {
  type        = string
  default     = null
  nullable    = true
  description = "Observed Datadog name for custom.googleapis.com/coda/availability/firing. Null keeps service monitors absent until GCP ingestion and labels are qualified."
  validation {
    condition     = var.availability_firing_metric == null ? true : can(regex("^[A-Za-z][A-Za-z0-9_.]+$", var.availability_firing_metric))
    error_message = "Use a qualified Datadog metric name, not a query."
  }
}
variable "availability_monitors_enabled" {
  type    = bool
  default = false
  validation {
    condition     = !var.availability_monitors_enabled || (var.datadog_monitors_enabled && var.datadog_gcp_integration_enabled && var.availability_firing_metric != null)
    error_message = "Availability alerts require enabled primary monitors, GCP ingestion and a qualified firing metric."
  }
}
variable "availability_evaluation_delay_seconds" {
  type        = number
  default     = 120
  description = "GCP-to-Datadog ingestion allowance; measure before enabling. Does not change probe streak counts."
  validation {
    condition     = var.availability_evaluation_delay_seconds >= 0 && var.availability_evaluation_delay_seconds <= 900 && floor(var.availability_evaluation_delay_seconds) == var.availability_evaluation_delay_seconds
    error_message = "Use an integer ingestion allowance between 0 and 900 seconds."
  }
}
resource "datadog_monitor" "availability" {
  for_each = local.datadog_backend && var.availability_monitors_enabled ? toset(["physician", "duet", "physician-worker"]) : toset([])
  name     = "[${local.dd_env}] ${each.key} authenticated readiness"
  type     = "query alert"
  query    = "avg(last_1m):max:${var.availability_firing_metric}{project_id:${var.cluster_project},target:${each.key}}.fill(null) >= 1"
  monitor_thresholds {
    critical          = 1
    critical_recovery = 0.5
  }
  evaluation_delay    = var.availability_evaluation_delay_seconds
  require_full_window = false
  notify_no_data      = false
  # Do not auto-recover on silence. The independent Monitoring absence policy
  # owns execution loss; a missing run is neither a success nor a failure.
  timeout_h    = 0
  include_tags = true
  message      = <<-EOT
    {{#is_alert}}${each.key} in ${local.dd_env} has failed three consecutive minute-slot authenticated /readyz probes.{{/is_alert}}
    {{#is_recovery}}${each.key} recovered after two consecutive successful probes.{{/is_recovery}}
    Inspect the fixed outcome label to distinguish target failure, timeout/refusal and probe authentication failure. A redirect is not readiness. Ingestion allowance is ${var.availability_evaluation_delay_seconds}s plus evaluator delay; qualify observed timing before enabling.
    Check the availability-probe Cloud Run job and service readiness, not clinical endpoints. Missing execution is owned by the independent Cloud Monitoring heartbeat watchdog.
    Runbook: https://github.com/tyler274/coda-rust/blob/main/book/src/observability.md
    ${var.alert_notify}
  EOT
  tags         = ["service:${each.key}", "env:${local.dd_env}", "team:coda", "signal:authenticated-readiness"]
}
