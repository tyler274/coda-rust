# Cloud SQL backup completion/failure monitoring and physician PITR RPO alerts.
#
# Cloud SQL emits no native Cloud Monitoring metric for backup age or PITR
# recovery-point lag:
# - Daily automated backup outcomes are recorded in System Event audit logs
#   (cloudaudit.googleapis.com/system_event, methodName
#   cloudsql.instances.automatedBackup, windowStatus STATUS_SUCCEEDED /
#   STATUS_FAILED / STATUS_SKIPPED). Per-instance log-based metrics feed both
#   immediate failure notifications and a 26-hour absence alert (24h PromQL
#   lookback + 2h pending hold, keeping lookback + evaluation interval inside
#   Cloud Monitoring's 25-hour limit for user log-based metrics).
# - Continuous WAL archiving failures are recorded in PostgreSQL engine logs
#   (cloudsql.googleapis.com/postgres.log: "archive command failed" or
#   "archiving write-ahead log file ... failed"), and sustained WAL backpressure
#   raises local disk utilization on rust-physician. Silent asynchronous backend
#   GCS archive lag without an engine error has no GCP metric; empirical RPO is
#   verified through isolated point-in-time clone rehearsals (book/src/gcp.md).

variable "database_backup_alerts_enabled" {
  type        = bool
  default     = true
  nullable    = false
  description = "Enable Cloud Monitoring alert policies for Cloud SQL daily backup staleness (>26h), backup failure/skip, and physician WAL-archiving/disk-backpressure RPO risk when databases and notification channels are present."
}

variable "database_backup_alert_channels" {
  type        = list(string)
  default     = []
  nullable    = false
  description = "Optional dedicated Cloud Monitoring notification channel resource names for database backup and RPO alerts; empty reuses availability_watchdog_channels."
  validation {
    condition = alltrue([
      for ch in var.database_backup_alert_channels :
      can(regex("^projects/[a-z][a-z0-9-]{4,28}[a-z0-9]/notificationChannels/[A-Za-z0-9_-]+$", ch))
    ])
    error_message = "database_backup_alert_channels must contain Cloud Monitoring channel resource names (projects/<project>/notificationChannels/<id>)."
  }
}

locals {
  physician_databases     = var.enable_databases ? toset(["physician"]) : toset([])
  sql_database_ids        = { for app in local.apps : app => "${local.project_id}:${local.sql_instance_names[app]}" }
  database_alert_channels = length(var.database_backup_alert_channels) > 0 ? var.database_backup_alert_channels : var.availability_watchdog_channels
  database_alerts_active  = var.enable_databases && var.database_backup_alerts_enabled && length(local.database_alert_channels) > 0
  database_alert_apps     = local.database_alerts_active ? local.apps : toset([])
  physician_alert_apps    = local.database_alerts_active ? toset(["physician"]) : toset([])
}

resource "google_logging_metric" "sql_backup_succeeded" {
  for_each = local.databases
  project  = google_project.environment.project_id
  name     = "coda/sql_backup_succeeded_${each.key}"
  filter   = <<-EOT
    resource.type="cloudsql_database"
    resource.labels.database_id="${local.sql_database_ids[each.key]}"
    log_id("cloudaudit.googleapis.com/system_event")
    protoPayload.methodName="cloudsql.instances.automatedBackup"
    protoPayload.metadata.windowStatus="STATUS_SUCCEEDED"
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }

  depends_on = [google_project_service.api]
}

resource "google_logging_metric" "sql_backup_failed" {
  for_each = local.databases
  project  = google_project.environment.project_id
  name     = "coda/sql_backup_failed_${each.key}"
  filter   = <<-EOT
    resource.type="cloudsql_database"
    resource.labels.database_id="${local.sql_database_ids[each.key]}"
    log_id("cloudaudit.googleapis.com/system_event")
    protoPayload.methodName="cloudsql.instances.automatedBackup"
    (protoPayload.metadata.windowStatus="STATUS_FAILED" OR protoPayload.metadata.windowStatus="STATUS_SKIPPED")
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }

  depends_on = [google_project_service.api]
}

resource "google_logging_metric" "physician_sql_wal_archive_failed" {
  for_each = local.physician_databases
  project  = google_project.environment.project_id
  name     = "coda/physician_sql_wal_archive_failed"
  filter   = <<-EOT
    resource.type="cloudsql_database"
    resource.labels.database_id="${local.sql_database_ids[each.key]}"
    log_id("cloudsql.googleapis.com/postgres.log")
    textPayload=~"archive command failed|archiving write-ahead log file .* failed"
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }

  depends_on = [google_project_service.api]
}

resource "google_monitoring_alert_policy" "sql_backup_stale" {
  for_each              = local.database_alert_apps
  project               = google_project.environment.project_id
  display_name          = "[${var.environment}] ${each.key} database daily backup older than 26h"
  enabled               = true
  combiner              = "OR"
  severity              = "ERROR"
  notification_channels = local.database_alert_channels

  conditions {
    display_name = "No successful ${each.key} automated backup for 26 hours"
    condition_prometheus_query_language {
      # Cloud Monitoring caps PromQL lookback + evaluation interval on
      # logging.googleapis.com/user/* metrics at 25h. Pairing a 24h absence
      # range (24h + 5m <= 25h) with a 2h hold fires at 26h since the last
      # STATUS_SUCCEEDED backup while absorbing window jitter without false alarms.
      query               = "absent_over_time({\"__name__\"=\"logging_googleapis_com:user_${replace(google_logging_metric.sql_backup_succeeded[each.key].name, "/", "_")}\"}[24h])"
      duration            = "7200s"
      evaluation_interval = "300s"
    }
  }

  alert_strategy {
    notification_prompts = ["OPENED", "CLOSED"]
  }

  documentation {
    mime_type = "text/markdown"
    content   = "No successful automated daily backup has been recorded for `${local.sql_instance_names[each.key]}` in more than 26 hours. Run `gcloud sql backups list --instance=${local.sql_instance_names[each.key]} --project=${local.project_id}` and inspect `cloudaudit.googleapis.com/system_event` (`cloudsql.instances.automatedBackup`). Confirm a fresh `SUCCESSFUL` backup before closing operational follow-up. See `book/src/gcp.md#database-backup-policy-and-rpo`."
  }

  depends_on = [google_logging_metric.sql_backup_succeeded]
}

resource "google_monitoring_alert_policy" "sql_backup_failed" {
  for_each              = local.database_alert_apps
  project               = google_project.environment.project_id
  display_name          = "[${var.environment}] ${each.key} database daily backup failed or skipped"
  enabled               = true
  combiner              = "OR"
  severity              = "ERROR"
  notification_channels = local.database_alert_channels

  conditions {
    display_name = "Cloud SQL automated backup reported STATUS_FAILED or STATUS_SKIPPED"
    condition_matched_log {
      filter = google_logging_metric.sql_backup_failed[each.key].filter
    }
  }

  alert_strategy {
    notification_prompts = ["OPENED"]
    auto_close           = "86400s"
    notification_rate_limit {
      period = "3600s"
    }
  }

  documentation {
    mime_type = "text/markdown"
    content   = "Cloud SQL reported `STATUS_FAILED` or `STATUS_SKIPPED` for `${local.sql_instance_names[each.key]}` during its daily automated backup window. Inspect `gcloud sql operations list --instance=${local.sql_instance_names[each.key]} --project=${local.project_id}` and `cloudaudit.googleapis.com/system_event`. Incident auto-close after log inactivity is not recovery; verify a later `SUCCESSFUL` backup in `gcloud sql backups list`. See `book/src/gcp.md#database-backup-policy-and-rpo`."
  }

  depends_on = [google_logging_metric.sql_backup_failed]
}

resource "google_monitoring_alert_policy" "physician_sql_wal_archive_failed" {
  for_each              = local.physician_alert_apps
  project               = google_project.environment.project_id
  display_name          = "[${var.environment}] physician database WAL archiving failed (15m RPO at risk)"
  enabled               = true
  combiner              = "OR"
  severity              = "CRITICAL"
  notification_channels = local.database_alert_channels

  conditions {
    display_name = "PostgreSQL WAL archiver reported a failed archive cycle on rust-physician"
    condition_matched_log {
      filter = google_logging_metric.physician_sql_wal_archive_failed[each.key].filter
    }
  }

  alert_strategy {
    notification_prompts = ["OPENED"]
    auto_close           = "3600s"
    notification_rate_limit {
      period = "900s"
    }
  }

  documentation {
    mime_type = "text/markdown"
    content   = "PostgreSQL on `rust-physician` logged a failed WAL archive cycle (`archive command failed` or `archiving write-ahead log file ... failed`), putting the 5-minute target / 15-minute maximum RPO at risk. Inspect `cloudsql.googleapis.com/postgres.log`, disk utilization (`pg_wal` backpressure), and Google Cloud service health immediately. Cloud SQL exposes no native PITR recovery-point lag metric; after remediation, verify point-in-time recoverability only through an isolated `rust-restore-physician-pitr-<run>` clone never attached to serving or workers. See `book/src/gcp.md#database-backup-policy-and-rpo`."
  }

  depends_on = [google_logging_metric.physician_sql_wal_archive_failed]
}

resource "google_monitoring_alert_policy" "physician_sql_disk_pressure" {
  for_each              = local.physician_alert_apps
  project               = google_project.environment.project_id
  display_name          = "[${var.environment}] physician database disk utilization > 85% (WAL backpressure risk)"
  enabled               = true
  combiner              = "OR"
  severity              = "WARNING"
  notification_channels = local.database_alert_channels

  conditions {
    display_name = "rust-physician disk utilization above 0.85 for 10 minutes"
    condition_threshold {
      filter          = "resource.type = \"cloudsql_database\" AND resource.labels.database_id = \"${local.sql_database_ids[each.key]}\" AND metric.type = \"cloudsql.googleapis.com/database/disk/utilization\""
      comparison      = "COMPARISON_GT"
      threshold_value = 0.85
      duration        = "600s"
      aggregations {
        alignment_period   = "60s"
        per_series_aligner = "ALIGN_MEAN"
      }
      trigger {
        count = 1
      }
    }
  }

  alert_strategy {
    notification_prompts = ["OPENED", "CLOSED"]
  }

  documentation {
    mime_type = "text/markdown"
    content   = "Disk utilization on `rust-physician` exceeded 85% for 10 minutes (`disk_autoresize_limit` is ${var.sql_disk_limit_gb} GiB). Stalled WAL archiving retains unarchived segments in `pg_wal` and can exhaust disk headroom, threatening both writes and the 15-minute physician RPO limit. Inspect `cloudsql.googleapis.com/database/bytes_used_by_data_type` and `cloudsql.googleapis.com/postgres.log`. See `book/src/gcp.md#database-backup-policy-and-rpo`."
  }

  depends_on = [google_project_service.api]
}

output "database_backup_policy" {
  description = "Nonsecret Cloud SQL backup policy, physician RPO target/limit, and active backup/WAL alert policies."
  value = var.enable_databases ? {
    location                       = local.sql_backup_policy.location
    start_time_utc                 = local.sql_backup_policy.start_time
    retained_backups               = local.sql_backup_policy.retained_backups
    retention_unit                 = local.sql_backup_policy.retention_unit
    point_in_time_recovery_enabled = local.sql_backup_policy.point_in_time_recovery_enabled
    transaction_log_retention_days = local.sql_backup_policy.transaction_log_retention_days
    physician_rpo_target_minutes   = 5
    physician_rpo_limit_minutes    = 15
    backup_stale_threshold_hours   = 26
    alerts_active                  = local.database_alerts_active
    stale_alert_policies           = { for app, policy in google_monitoring_alert_policy.sql_backup_stale : app => policy.display_name }
    failure_alert_policies         = { for app, policy in google_monitoring_alert_policy.sql_backup_failed : app => policy.display_name }
    physician_wal_archive_alert    = try(google_monitoring_alert_policy.physician_sql_wal_archive_failed["physician"].display_name, null)
    physician_disk_pressure_alert  = try(google_monitoring_alert_policy.physician_sql_disk_pressure["physician"].display_name, null)
    pitr_lag_metric_gap_disclosed  = "Cloud SQL exposes no native Cloud Monitoring metric for PITR recovery-point age or silent asynchronous GCS WAL archive lag; alerts cover automatedBackup audit events, postgres.log WAL archiver failures, and rust-physician disk backpressure while empirical RPO is verified via isolated point-in-time clone rehearsals."
  } : null
}
