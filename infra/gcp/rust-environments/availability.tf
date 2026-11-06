# Disabled until the image, authenticated reachability, metric ingestion and
# notification routes have been qualified and a saved plan is approved.
variable "availability_probe_image" {
  type        = string
  default     = null
  nullable    = true
  description = "Digest-pinned availability-probe image; null creates no probe resources."
  validation {
    condition     = var.availability_probe_image == null ? true : var.environment == "stg" && can(regex("@sha256:[a-f0-9]{64}$", var.availability_probe_image)) && var.enable_physician_scale
    error_message = "Availability probes require staging, physician scale and a digest-pinned image."
  }
}
variable "availability_probe_schedule_enabled" {
  type    = bool
  default = false
}
variable "availability_watchdog_enabled" {
  type    = bool
  default = false
  validation {
    condition     = !var.availability_watchdog_enabled || (var.availability_probe_image != null && length(var.availability_watchdog_channels) > 0)
    error_message = "Watchdog activation requires a probe and explicitly qualified notification channels."
  }
}
variable "availability_watchdog_channels" {
  type        = list(string)
  default     = []
  description = "Existing Cloud Monitoring notification channel names; no Slack credentials in state."
}
locals {
  availability = var.availability_probe_image == null ? toset([]) : toset(["probe"])
}
resource "google_project_service" "availability_scheduler" {
  for_each           = local.availability
  project            = google_project.environment.project_id
  service            = "cloudscheduler.googleapis.com"
  disable_on_destroy = false
}
resource "google_service_account" "availability" {
  for_each   = local.availability
  project    = google_project.environment.project_id
  account_id = "availability-probe"
}
resource "google_service_account" "availability_scheduler" {
  for_each   = local.availability
  project    = google_project.environment.project_id
  account_id = "availability-scheduler"
}
# Only signJwt, not arbitrary account impersonation or private keys.
resource "google_project_iam_custom_role" "availability_signer" {
  for_each    = local.availability
  project     = google_project.environment.project_id
  role_id     = "availabilitySigner"
  title       = "Availability self JWT signing"
  permissions = ["iam.serviceAccounts.signJwt"]
}
resource "google_service_account_iam_member" "availability_signer" {
  for_each           = local.availability
  service_account_id = google_service_account.availability[each.key].name
  role               = google_project_iam_custom_role.availability_signer[each.key].name
  member             = "serviceAccount:${google_service_account.availability[each.key].email}"
}
resource "google_project_iam_member" "availability_metrics" {
  for_each = local.availability
  project  = google_project.environment.project_id
  role     = "roles/monitoring.metricWriter"
  member   = "serviceAccount:${google_service_account.availability[each.key].email}"
}
resource "google_storage_bucket" "availability" {
  for_each                    = local.availability
  project                     = google_project.environment.project_id
  name                        = "${google_project.environment.project_id}-availability-state"
  location                    = var.region
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false
}
resource "google_storage_bucket_iam_member" "availability" {
  for_each = local.availability
  bucket   = google_storage_bucket.availability[each.key].name
  role     = "roles/storage.objectUser"
  member   = "serviceAccount:${google_service_account.availability[each.key].email}"
  condition {
    title      = "probe-state-only"
    expression = "resource.name == 'projects/_/buckets/${google_storage_bucket.availability[each.key].name}/objects/state.json'"
  }
}
resource "google_cloud_run_v2_job" "availability" {
  for_each            = local.availability
  project             = google_project.environment.project_id
  name                = "availability-probe"
  location            = var.region
  deletion_protection = true
  template {
    task_count  = 1
    parallelism = 1
    template {
      service_account = google_service_account.availability[each.key].email
      timeout         = "120s"
      max_retries     = 0
      # ALL_TRAFFIC is required for run.app worker requests to count as internal.
      # Private Google Access on rust-run reaches Google APIs/run.app without
      # depending on GKE or its collector/NAT. No external non-Google targets.
      vpc_access {
        egress = "ALL_TRAFFIC"
        network_interfaces {
          network    = google_compute_network.private.name
          subnetwork = google_compute_subnetwork.run.name
        }
      }
      containers {
        image = var.availability_probe_image
        resources { limits = { cpu = "1", memory = "512Mi" } }
        dynamic "env" {
          for_each = {
            AVAILABILITY_PROBE_PROJECT         = google_project.environment.project_id
            AVAILABILITY_PROBE_SERVICE_ACCOUNT = google_service_account.availability[each.key].email
            AVAILABILITY_PROBE_STATE_BUCKET    = google_storage_bucket.availability[each.key].name
            AVAILABILITY_PROBE_PHYSICIAN_URL   = google_cloud_run_v2_service.app["physician"].uri
            AVAILABILITY_PROBE_DUET_URL        = google_cloud_run_v2_service.app["duet"].uri
            AVAILABILITY_PROBE_WORKER_URL      = google_cloud_run_v2_service.physician_worker["physician"].uri
            AVAILABILITY_PROBE_TIMEOUT_SECONDS = "30"
          }
          content {
            name  = env.key
            value = env.value
          }
        }
      }
    }
  }
  depends_on = [google_service_account_iam_member.availability_signer, google_project_iam_member.availability_metrics, google_storage_bucket_iam_member.availability]
}
resource "google_service_account" "availability_dispatch" {
  for_each   = local.availability
  project    = google_project.environment.project_id
  account_id = "availability-dispatch"
}
resource "google_cloud_run_v2_job_iam_member" "availability_dispatch" {
  for_each = local.availability
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_job.availability[each.key].name
  role     = "roles/run.jobsExecutorWithOverrides"
  member   = "serviceAccount:${google_service_account.availability_dispatch[each.key].email}"
}
# Only the scheduler identity can invoke this adapter. It receives the documented
# retry-stable X-CloudScheduler-ScheduleTime and passes a job env override.
resource "google_cloud_run_v2_service" "availability_dispatch" {
  for_each             = local.availability
  project              = google_project.environment.project_id
  location             = var.region
  name                 = "availability-dispatch"
  deletion_protection  = true
  ingress              = "INGRESS_TRAFFIC_ALL"
  invoker_iam_disabled = false
  template {
    service_account = google_service_account.availability_dispatch[each.key].email
    timeout         = "30s"
    scaling { max_instance_count = 2 }
    containers {
      image = var.availability_probe_image
      ports { container_port = 8080 }
      resources { limits = { cpu = "1", memory = "256Mi" } }
      env {
        name  = "AVAILABILITY_MODE"
        value = "dispatch"
      }
      env {
        name  = "AVAILABILITY_JOB_RUN_URL"
        value = "https://run.googleapis.com/v2/${google_cloud_run_v2_job.availability[each.key].id}:run"
      }
      env {
        name  = "AVAILABILITY_SCHEDULER_JOB"
        value = "projects/${google_project.environment.project_id}/locations/${var.region}/jobs/availability-probe"
      }
    }
  }
  depends_on = [google_cloud_run_v2_job_iam_member.availability_dispatch]
}
resource "google_cloud_run_v2_service_iam_binding" "availability_dispatch" {
  for_each = local.availability
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.availability_dispatch[each.key].name
  role     = "roles/run.invoker"
  members  = ["serviceAccount:${google_service_account.availability_scheduler[each.key].email}"]
}
# Read-back resolves an acknowledged-write/lost-response ambiguity. No logs,
# traces, clinical records, or project-wide Monitoring viewer grant.
resource "google_project_iam_custom_role" "availability_readback" {
  for_each    = local.availability
  project     = google_project.environment.project_id
  role_id     = "availabilityReadback"
  title       = "Availability metric acknowledgement"
  permissions = ["monitoring.timeSeries.list"]
}
resource "google_project_iam_member" "availability_readback" {
  for_each = local.availability
  project  = google_project.environment.project_id
  role     = google_project_iam_custom_role.availability_readback[each.key].name
  member   = "serviceAccount:${google_service_account.availability[each.key].email}"
}
resource "google_cloud_scheduler_job" "availability" {
  for_each         = local.availability
  project          = google_project.environment.project_id
  region           = var.region
  name             = "availability-probe"
  schedule         = "* * * * *"
  time_zone        = "Etc/UTC"
  paused           = !var.availability_probe_schedule_enabled
  attempt_deadline = "30s"
  retry_config { retry_count = 0 }
  http_target {
    http_method = "POST"
    uri         = "${google_cloud_run_v2_service.availability_dispatch[each.key].uri}/dispatch"
    body        = base64encode("{}")
    headers     = { "Content-Type" = "application/json" }
    oidc_token {
      service_account_email = google_service_account.availability_scheduler[each.key].email
      audience              = google_cloud_run_v2_service.availability_dispatch[each.key].uri
    }
  }
  depends_on = [google_project_service.availability_scheduler, google_cloud_run_v2_service_iam_binding.availability_dispatch]
}
resource "google_monitoring_alert_policy" "availability_watchdog" {
  for_each              = local.availability
  project               = google_project.environment.project_id
  display_name          = "[staging] availability probe heartbeat missing"
  enabled               = var.availability_watchdog_enabled
  combiner              = "OR"
  notification_channels = var.availability_watchdog_channels
  conditions {
    display_name = "No directly published probe heartbeat for five minutes"
    condition_absent {
      filter   = "resource.type = \"global\" AND metric.type = \"custom.googleapis.com/coda/availability/heartbeat\""
      duration = "300s"
      trigger { count = 1 }
    }
  }
  alert_strategy {
    notification_prompts = ["OPENED", "CLOSED"]
  }
  documentation {
    mime_type = "text/markdown"
    content   = "The staging probe heartbeat is absent for five minutes (plus Cloud Monitoring evaluation delay). Check Cloud Scheduler, Cloud Run job executions, state storage and Monitoring publication. This independent watchdog does not assert service downtime. Restore probe execution and direct publication; recovery closes the incident. See book/src/observability.md."
  }
}
