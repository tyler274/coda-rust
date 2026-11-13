# Offline OpenTofu verification of the Cloud SQL backup policy, RPO guardrails,
# and Cloud Monitoring backup/WAL-archiving alerts across stg and prd.
mock_provider "google" {
  mock_resource "google_compute_network" {
    defaults = { id = "projects/test-rust-stg-only/global/networks/rust-private" }
  }
  mock_resource "google_project" {
    defaults = { number = "123456789012" }
  }
  mock_resource "google_service_account" {
    defaults = {
      name  = "projects/test-rust-stg-only/serviceAccounts/test-account@test-rust-stg-only.iam.gserviceaccount.com"
      email = "test-account@test-rust-stg-only.iam.gserviceaccount.com"
    }
  }
}
mock_provider "google-beta" {}

variables {
  environment = "stg"
  projects = {
    dev = { project_id = "test-rust-dev-only", state_bucket_name = "test-rust-dev-state-only" }
    stg = { project_id = "test-rust-stg-only", state_bucket_name = "test-rust-stg-state-only" }
    prd = { project_id = "test-rust-prd-only", state_bucket_name = "test-rust-prd-state-only" }
  }
  organization_id      = "123456789"
  billing_account      = "000000-000000-000000"
  region               = "us-east1"
  iap_group            = "test-team@codahealth.test"
  state_admin_members  = ["group:test-infra@codahealth.test"]
  github_repository_id = "100000001"
  github_owner_id      = "100000002"
  database_passwords = {
    physician-runtime   = "synthetic-physician-runtime-pw"
    physician-migration = "synthetic-physician-migration-pw"
    duet-runtime        = "synthetic-duet-runtime-pw"
    duet-migration      = "synthetic-duet-migration-pw"
  }
}

run "foundations_create_no_backup_metrics_or_alerts" {
  command = plan
  variables {
    database_passwords = {}
  }
  assert {
    condition = (
      length(google_logging_metric.sql_backup_succeeded) == 0 &&
      length(google_logging_metric.sql_backup_failed) == 0 &&
      length(google_logging_metric.physician_sql_wal_archive_failed) == 0 &&
      length(google_monitoring_alert_policy.sql_backup_stale) == 0 &&
      length(google_monitoring_alert_policy.sql_backup_failed) == 0 &&
      length(google_monitoring_alert_policy.physician_sql_wal_archive_failed) == 0 &&
      length(google_monitoring_alert_policy.physician_sql_disk_pressure) == 0 &&
      output.database_backup_policy == null
    )
    error_message = "With databases disabled, no SQL backup log metric or alert policy may exist."
  }
}

run "staging_backup_policy_and_alerts" {
  command = plan
  variables {
    enable_databases               = true
    sql_tier                       = "db-f1-micro"
    availability_watchdog_channels = ["projects/test-rust-stg-only/notificationChannels/101"]
  }

  # AC2: Both staging product databases enforce US multi-region daily backups + 7-day PITR.
  assert {
    condition = alltrue([
      for app in ["physician", "duet"] :
      google_sql_database_instance.app[app].deletion_protection &&
      google_sql_database_instance.app[app].settings[0].deletion_protection_enabled &&
      google_sql_database_instance.app[app].settings[0].availability_type == "ZONAL" &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].enabled &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].location == "us" &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].start_time == "03:00" &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].point_in_time_recovery_enabled &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].transaction_log_retention_days == 7 &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].backup_retention_settings[0].retained_backups == 7 &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].backup_retention_settings[0].retention_unit == "COUNT"
    ])
    error_message = "Physician and Duet backups must explicitly pin the US multi-region location, daily 03:00 window, 7 retained backups, PITR and 7-day log retention."
  }

  # AC2: Committed env/staging.tfvars enables databases in us-east1 with active alert routing
  # and never disables backup alerts or overrides US multi-region backup residency.
  assert {
    condition = (
      can(regex("(?m)^environment\\s*=\\s*\"stg\"$", file("${path.module}/env/staging.tfvars"))) &&
      can(regex("(?m)^region\\s*=\\s*\"us-east1\"$", file("${path.module}/env/staging.tfvars"))) &&
      can(regex("(?m)^enable_databases\\s*=\\s*true$", file("${path.module}/env/staging.tfvars"))) &&
      can(regex("(?m)^availability_watchdog_channels\\s*=\\s*\\[\"projects/coda-rust-staging/notificationChannels/[0-9]+\"\\]$", file("${path.module}/env/staging.tfvars"))) &&
      !can(regex("(?m)^database_backup_alerts_enabled\\s*=\\s*false$", file("${path.module}/env/staging.tfvars"))) &&
      !can(regex("(?m)^sql_backup_location\\s*=", file("${path.module}/env/staging.tfvars")))
    )
    error_message = "Committed env/staging.tfvars must keep databases enabled in us-east1 with active notification channels and US multi-region backup defaults."
  }

  # AC4 (part 1): Per-product daily backup success/failure metrics and 26-hour staleness alerts.
  assert {
    condition = (
      keys(google_logging_metric.sql_backup_succeeded) == ["duet", "physician"] &&
      keys(google_monitoring_alert_policy.sql_backup_stale) == ["duet", "physician"] &&
      strcontains(google_logging_metric.sql_backup_succeeded["physician"].filter, "resource.labels.database_id=\"test-rust-stg-only:rust-physician\"") &&
      strcontains(google_logging_metric.sql_backup_succeeded["duet"].filter, "resource.labels.database_id=\"test-rust-stg-only:rust-duet\"") &&
      !strcontains(google_logging_metric.sql_backup_succeeded["physician"].filter, "rust-duet") &&
      alltrue([
        for app in ["physician", "duet"] :
        strcontains(google_logging_metric.sql_backup_succeeded[app].filter, "protoPayload.methodName=\"cloudsql.instances.automatedBackup\"") &&
        strcontains(google_logging_metric.sql_backup_succeeded[app].filter, "protoPayload.metadata.windowStatus=\"STATUS_SUCCEEDED\"") &&
        google_monitoring_alert_policy.sql_backup_stale[app].enabled &&
        google_monitoring_alert_policy.sql_backup_stale[app].severity == "ERROR" &&
        toset(google_monitoring_alert_policy.sql_backup_stale[app].notification_channels) == toset(["projects/test-rust-stg-only/notificationChannels/101"]) &&
        google_monitoring_alert_policy.sql_backup_stale[app].conditions[0].condition_prometheus_query_language[0].query == "absent_over_time({\"__name__\"=\"logging_googleapis_com:user_coda_sql_backup_succeeded_${app}\"}[24h])" &&
        google_monitoring_alert_policy.sql_backup_stale[app].conditions[0].condition_prometheus_query_language[0].duration == "7200s" &&
        google_monitoring_alert_policy.sql_backup_stale[app].conditions[0].condition_prometheus_query_language[0].evaluation_interval == "300s"
      ])
    )
    error_message = "Each database must track its own STATUS_SUCCEEDED backup audit events and fire a 26-hour (24h absence + 2h hold) staleness alert without cross-product masking."
  }

  assert {
    condition = alltrue([
      for app in ["physician", "duet"] :
      google_monitoring_alert_policy.sql_backup_failed[app].enabled &&
      google_monitoring_alert_policy.sql_backup_failed[app].conditions[0].condition_matched_log[0].filter == google_logging_metric.sql_backup_failed[app].filter &&
      strcontains(google_logging_metric.sql_backup_failed[app].filter, "STATUS_FAILED") &&
      strcontains(google_logging_metric.sql_backup_failed[app].filter, "STATUS_SKIPPED") &&
      toset(google_monitoring_alert_policy.sql_backup_failed[app].notification_channels) == toset(["projects/test-rust-stg-only/notificationChannels/101"])
    ])
    error_message = "Failed or skipped automated backups must notify immediately on the configured channel."
  }

  # AC4 (part 2): Physician-only WAL archiver failure and disk backpressure RPO alerts.
  assert {
    condition = (
      keys(google_logging_metric.physician_sql_wal_archive_failed) == ["physician"] &&
      keys(google_monitoring_alert_policy.physician_sql_wal_archive_failed) == ["physician"] &&
      keys(google_monitoring_alert_policy.physician_sql_disk_pressure) == ["physician"] &&
      strcontains(google_logging_metric.physician_sql_wal_archive_failed["physician"].filter, "resource.labels.database_id=\"test-rust-stg-only:rust-physician\"") &&
      strcontains(google_logging_metric.physician_sql_wal_archive_failed["physician"].filter, "log_id(\"cloudsql.googleapis.com/postgres.log\")") &&
      strcontains(google_logging_metric.physician_sql_wal_archive_failed["physician"].filter, "archive command failed") &&
      google_monitoring_alert_policy.physician_sql_wal_archive_failed["physician"].enabled &&
      google_monitoring_alert_policy.physician_sql_wal_archive_failed["physician"].severity == "CRITICAL" &&
      google_monitoring_alert_policy.physician_sql_wal_archive_failed["physician"].alert_strategy[0].notification_rate_limit[0].period == "900s" &&
      google_monitoring_alert_policy.physician_sql_disk_pressure["physician"].enabled &&
      strcontains(google_monitoring_alert_policy.physician_sql_disk_pressure["physician"].conditions[0].condition_threshold[0].filter, "cloudsql.googleapis.com/database/disk/utilization") &&
      strcontains(google_monitoring_alert_policy.physician_sql_disk_pressure["physician"].conditions[0].condition_threshold[0].filter, "test-rust-stg-only:rust-physician") &&
      google_monitoring_alert_policy.physician_sql_disk_pressure["physician"].conditions[0].condition_threshold[0].threshold_value == 0.85 &&
      google_monitoring_alert_policy.physician_sql_disk_pressure["physician"].conditions[0].condition_threshold[0].duration == "600s" &&
      output.database_backup_policy.physician_rpo_target_minutes == 5 &&
      output.database_backup_policy.physician_rpo_limit_minutes == 15 &&
      output.database_backup_policy.backup_stale_threshold_hours == 26
    )
    error_message = "Physician RPO alerts must cover rust-physician postgres.log WAL archive failures and disk backpressure, with zero Duet cross-wiring."
  }
}

run "dedicated_backup_alert_channels_override_watchdog" {
  command = plan
  variables {
    enable_databases               = true
    availability_watchdog_channels = ["projects/test-rust-stg-only/notificationChannels/watchdog"]
    database_backup_alert_channels = ["projects/test-rust-stg-only/notificationChannels/db-ops"]
  }
  assert {
    condition = (
      toset(google_monitoring_alert_policy.sql_backup_stale["physician"].notification_channels) == toset(["projects/test-rust-stg-only/notificationChannels/db-ops"]) &&
      toset(google_monitoring_alert_policy.physician_sql_wal_archive_failed["physician"].notification_channels) == toset(["projects/test-rust-stg-only/notificationChannels/db-ops"])
    )
    error_message = "Dedicated database_backup_alert_channels must override availability_watchdog_channels for SQL backup and RPO alerts."
  }
}

run "production_backup_policy_and_alerts" {
  command = plan
  variables {
    environment                    = "prd"
    enable_databases               = true
    production_activation_approved = true
    database_backup_alert_channels = ["projects/test-rust-prd-only/notificationChannels/prd-ops"]
  }
  assert {
    condition = alltrue([
      for app in ["physician", "duet"] :
      google_sql_database_instance.app[app].project == "test-rust-prd-only" &&
      google_sql_database_instance.app[app].deletion_protection &&
      google_sql_database_instance.app[app].settings[0].deletion_protection_enabled &&
      google_sql_database_instance.app[app].settings[0].availability_type == "REGIONAL" &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].enabled &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].location == "us" &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].start_time == "03:00" &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].point_in_time_recovery_enabled &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].transaction_log_retention_days == 7 &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].backup_retention_settings[0].retained_backups == 7 &&
      google_sql_database_instance.app[app].settings[0].backup_configuration[0].backup_retention_settings[0].retention_unit == "COUNT"
    ])
    error_message = "Production databases must enforce REGIONAL HA plus the identical US multi-region daily backup and 7-day PITR policy."
  }
  assert {
    condition = (
      google_monitoring_alert_policy.sql_backup_stale["physician"].display_name == "[prd] physician database daily backup older than 26h" &&
      google_monitoring_alert_policy.sql_backup_stale["physician"].enabled &&
      google_monitoring_alert_policy.physician_sql_wal_archive_failed["physician"].display_name == "[prd] physician database WAL archiving failed (15m RPO at risk)" &&
      google_monitoring_alert_policy.physician_sql_wal_archive_failed["physician"].enabled &&
      google_monitoring_alert_policy.physician_sql_disk_pressure["physician"].enabled &&
      toset(google_monitoring_alert_policy.sql_backup_stale["physician"].notification_channels) == toset(["projects/test-rust-prd-only/notificationChannels/prd-ops"]) &&
      strcontains(google_logging_metric.physician_sql_wal_archive_failed["physician"].filter, "resource.labels.database_id=\"test-rust-prd-only:rust-physician\"")
    )
    error_message = "Production databases must deploy active [prd] backup staleness, failure, and physician WAL-archiving alerts."
  }
}

run "reject_non_us_backup_location" {
  command = plan
  variables {
    sql_backup_location = "eu"
  }
  expect_failures = [var.sql_backup_location]
}

run "reject_single_region_backup_location" {
  command = plan
  variables {
    sql_backup_location = "us-east1"
  }
  expect_failures = [var.sql_backup_location]
}

run "reject_non_us_database_region" {
  command = plan
  variables {
    region           = "europe-west1"
    enable_databases = true
  }
  expect_failures = [google_sql_database_instance.app]
}

run "reject_invalid_backup_alert_channel" {
  command = plan
  variables {
    database_backup_alert_channels = ["https://hooks.slack.com/services/forbidden-secret-in-state"]
  }
  expect_failures = [var.database_backup_alert_channels]
}

run "reject_production_databases_without_backup_alert_channels" {
  command = plan
  variables {
    environment                    = "prd"
    enable_databases               = true
    production_activation_approved = true
    availability_watchdog_channels = []
    database_backup_alert_channels = []
  }
  expect_failures = [google_sql_database_instance.app]
}

run "reject_production_databases_with_backup_alerts_disabled" {
  command = plan
  variables {
    environment                    = "prd"
    enable_databases               = true
    production_activation_approved = true
    database_backup_alerts_enabled = false
    database_backup_alert_channels = ["projects/test-rust-prd-only/notificationChannels/prd-ops"]
  }
  expect_failures = [google_sql_database_instance.app]
}
