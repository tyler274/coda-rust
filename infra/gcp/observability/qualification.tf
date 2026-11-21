resource "datadog_monitor" "qualification" {
  count = local.datadog_backend && var.datadog_qualification_monitor_enabled ? 1 : 0
  name  = "[${local.dd_env}] synthetic qualification readiness"
  type  = "query alert"
  query = "avg(last_1m):max:coda.qualification.synthetic_readiness{env:staging}.fill(null) >= 1"
  monitor_thresholds {
    critical          = 1
    critical_recovery = 0.5
  }
  require_full_window  = false
  notify_no_data       = false
  timeout_h            = 0
  include_tags         = true
  renotify_interval    = 15
  renotify_occurrences = 2
  renotify_statuses    = ["alert"]
  message              = <<-EOT
    {{#is_alert}}The synthetic readiness series for ${local.dd_env} is at or above 1. This series is the qualification gauge, not a product service. The gauge stands for three consecutive failures the same way the availability probe does.{{/is_alert}}
    {{#is_recovery}}The synthetic readiness series recovered. A gauge at or below the recovery threshold stands for two consecutive successes.{{/is_recovery}}
    Runbook: https://github.com/tyler274/coda-rust/blob/main/book/src/observability.md
    ${var.alert_notify}
  EOT
  tags                 = ["qualification:synthetic", "env:${local.dd_env}", "team:coda"]
}
