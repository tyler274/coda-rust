# Fixed-interval Cloud Scheduler wake for physician-worker.
#
# Cloud Scheduler sends an OIDC-signed POST to /schedule/wake under the
# dedicated physician-scheduler service account, which holds no role other than
# Cloud Run Invoker on physician-worker. The worker verifies the OIDC audience
# and exact service account email in-process before running any wake work.
locals {
  physician_schedule          = local.physician_scale && var.physician_schedule_enabled ? toset(["physician"]) : toset([])
  physician_schedule_audience = "https://physician-worker-${google_project.environment.number}.${var.region}.run.app"
  physician_schedule_env = length(local.physician_schedule) > 0 ? {
    PHYSICIAN_WORKER_AUDIENCE           = local.physician_schedule_audience
    PHYSICIAN_SCHEDULER_SERVICE_ACCOUNT = google_service_account.physician_scheduler["physician"].email
  } : {}
}

resource "google_service_account" "physician_scheduler" {
  for_each     = local.physician_schedule
  project      = google_project.environment.project_id
  account_id   = "physician-scheduler"
  display_name = "Physician schedule wake invoker (${var.environment})"
  depends_on   = [google_project_service.api]
}

resource "google_cloud_scheduler_job" "physician_schedule" {
  for_each         = local.physician_schedule
  project          = google_project.environment.project_id
  region           = var.region
  name             = "physician-schedule"
  description      = "Wakes physician-worker at a fixed interval to check due patients and poll Spruce."
  schedule         = trimspace(var.physician_schedule_cron)
  time_zone        = trimspace(var.physician_schedule_time_zone)
  paused           = var.physician_schedule_paused
  attempt_deadline = "180s"

  retry_config {
    retry_count          = 3
    min_backoff_duration = "10s"
    max_backoff_duration = "120s"
    max_doublings        = 4
    max_retry_duration   = "240s"
  }

  http_target {
    http_method = "POST"
    uri         = "${google_cloud_run_v2_service.physician_worker[each.key].uri}/schedule/wake"
    body        = base64encode("{}")
    headers     = { "Content-Type" = "application/json" }
    oidc_token {
      service_account_email = google_service_account.physician_scheduler[each.key].email
      audience              = local.physician_schedule_audience
    }
  }

  depends_on = [google_project_service.api, google_cloud_run_v2_service_iam_binding.physician_worker_invoker]
}

resource "google_logging_metric" "physician_schedule_wake_success" {
  for_each = local.physician_schedule
  project  = google_project.environment.project_id
  name     = "coda/physician_schedule_wake_success"
  filter   = <<-EOT
    resource.type="cloud_run_revision"
    resource.labels.service_name="physician-worker"
    (jsonPayload.event="physician.worker.schedule_wake" OR textPayload:"physician.worker.schedule_wake")
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }

  depends_on = [google_project_service.api]
}

resource "google_logging_metric" "physician_schedule_wake_failure" {
  for_each = local.physician_schedule
  project  = google_project.environment.project_id
  name     = "coda/physician_schedule_wake_failure"
  filter   = <<-EOT
    (resource.type="cloud_run_revision" AND resource.labels.service_name="physician-worker" AND (jsonPayload.event="physician.worker.schedule_wake_failed" OR textPayload:"physician.worker.schedule_wake_failed"))
    OR
    (resource.type="cloud_scheduler_job" AND resource.labels.job_id="physician-schedule" AND severity>=ERROR)
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }

  depends_on = [google_project_service.api]
}

resource "google_monitoring_alert_policy" "physician_schedule_failure" {
  for_each              = local.physician_schedule
  project               = google_project.environment.project_id
  display_name          = "[${var.environment}] physician schedule wake failed"
  enabled               = true
  combiner              = "OR"
  severity              = "ERROR"
  notification_channels = var.availability_watchdog_channels

  conditions {
    display_name = "Physician schedule wake failed in worker or Cloud Scheduler"
    condition_matched_log {
      filter = google_logging_metric.physician_schedule_wake_failure[each.key].filter
    }
  }

  alert_strategy {
    notification_prompts = ["OPENED"]
    auto_close           = "3600s"
    notification_rate_limit {
      period = "300s"
    }
  }

  documentation {
    mime_type = "text/markdown"
    content   = "A scheduled wake to physician-worker failed in the worker handler or in Cloud Scheduler delivery. Inspect Cloud Scheduler job physician-schedule and physician-worker logs for event physician.worker.schedule_wake_failed. Confirm a subsequent physician.worker.schedule_wake log line before considering the schedule healthy."
  }

  depends_on = [google_logging_metric.physician_schedule_wake_failure]
}
