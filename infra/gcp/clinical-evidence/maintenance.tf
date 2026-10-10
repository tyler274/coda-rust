resource "google_service_account" "clinical_source_acquisition" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  project      = var.project_id
  account_id   = "coda-clinical-sources"
  display_name = "Coda clinical source acquisition"

  depends_on = [google_project_service.workload]
}

resource "google_service_account" "clinical_source_scheduler" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  project      = var.project_id
  account_id   = "coda-clinical-scheduler"
  display_name = "Coda clinical source scheduler"

  depends_on = [google_project_service.workload]
}

# Operational snapshots are write-once: both identities create and read them but cannot replace
# or delete them. Only the acquisition state object is updatable, by generation match.
locals {
  clinical_source_operations_writers = var.clinical_source_acquisition_enabled ? {
    sources    = "serviceAccount:${google_service_account.clinical_source_acquisition[0].email}"
    maintainer = "serviceAccount:${google_service_account.clinical_corpus_maintainer[0].email}"
  } : {}
}

resource "google_storage_bucket_iam_member" "clinical_source_operations_create" {
  for_each = local.clinical_source_operations_writers

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectCreator"
  member = each.value

  condition {
    title       = "production-clinical-source-operations-create"
    description = "Create operational source snapshots; replacing or deleting them is not granted."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/source-acquisition/')"
  }
}

resource "google_storage_bucket_iam_member" "clinical_source_operations_read" {
  for_each = local.clinical_source_operations_writers

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = each.value

  condition {
    title       = "production-clinical-source-operations-read"
    description = "Read operational source snapshots and state."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/source-acquisition/')"
  }
}

resource "google_storage_bucket_iam_member" "clinical_source_operations_state" {
  for_each = local.clinical_source_operations_writers

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectUser"
  member = each.value

  condition {
    title       = "production-clinical-source-operations-state"
    description = "Update only the acquisition state object, by generation match."
    expression  = "resource.name == 'projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/source-acquisition/state-v1.json'"
  }
}

resource "google_storage_bucket_iam_member" "clinical_corpus_maintainer_payload_create" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectCreator"
  member = "serviceAccount:${google_service_account.clinical_corpus_maintainer[0].email}"

  condition {
    title       = "production-clinical-maintainer-payload-create-only"
    description = "Allow create-only canonical payload publication."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/sha256/')"
  }
}

resource "google_storage_bucket_iam_member" "clinical_corpus_maintainer_payload_read" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.clinical_corpus_maintainer[0].email}"

  condition {
    title       = "production-clinical-maintainer-payload-read-only"
    description = "Allow exact canonical payload collision readback."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/sha256/')"
  }
}

resource "google_cloud_run_v2_job" "clinical_source_admission" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  project             = var.project_id
  name                = "coda-clinical-source-admission"
  location            = var.region
  deletion_protection = true

  template {
    parallelism = 1
    task_count  = 1

    template {
      service_account = google_service_account.clinical_corpus_maintainer[0].email
      timeout         = "3600s"
      max_retries     = 0

      containers {
        name    = "source-admission"
        image   = var.source_maintenance_image
        command = ["node"]
        args    = ["dist/src/server/clinicalCorpus/admit/hostedClinicalSourceAdmission.js"]

        resources {
          limits = {
            cpu    = "2"
            memory = "8Gi"
          }
        }

        env {
          name  = "CODA_CLINICAL_CORPUS_BUCKET"
          value = var.clinical_corpus_bucket_name
        }
        env {
          name  = "CODA_CLOUD_SQL_INSTANCE"
          value = google_sql_database_instance.clinical_evidence.connection_name
        }
        env {
          name  = "CODA_CLOUD_SQL_IAM_USER"
          value = trimsuffix(google_service_account.clinical_corpus_maintainer[0].email, ".gserviceaccount.com")
        }
        env {
          name = "CODA_CLINICAL_SOURCE_ADMISSION_CONFIG"
          value = jsonencode(merge({
            schema_version         = 1
            state_object           = "operations/source-acquisition/state-v1.json"
            expected_database_name = local.clinical_database_name
            terms_retrieved_at     = var.clinical_source_terms_retrieved_at
            decision_actor         = "coda-health/production-clinical-source-maintainer"
            python_executable      = "python3"
            adapter_root           = "/app/dist/src/adapters/clinicalLibrary/sources"
            }, var.clinical_bibliographic_contact_email == null ? {} : {
            bibliographic_status = {
              contact_email    = var.clinical_bibliographic_contact_email
              max_pmids        = 20000
              deadline_seconds = 1200
              # NLM's 2026 baseline ends at n1334; every later daily update file is applied
              # in order from the saved cursor, up to max_update_files per run.
              update_files_start_after = "pubmed26n1334.xml.gz"
              max_update_files         = 40
            }
          }))
        }
      }

      vpc_access {
        egress = "PRIVATE_RANGES_ONLY"

        network_interfaces {
          network    = google_compute_network.clinical_evidence.name
          subnetwork = google_compute_subnetwork.operations.name
        }
      }
    }
  }

  depends_on = [
    google_project_service.workload,
    google_project_iam_member.clinical_corpus_maintainer_cloud_sql,
    google_sql_user.clinical_corpus_maintainer,
    google_storage_bucket_iam_member.clinical_source_operations_create,
    google_storage_bucket_iam_member.clinical_source_operations_read,
    google_storage_bucket_iam_member.clinical_source_operations_state,
    google_storage_bucket_iam_member.clinical_corpus_maintainer_payload_create,
    google_storage_bucket_iam_member.clinical_corpus_maintainer_payload_read,
  ]
}

resource "google_cloud_run_v2_job" "clinical_source_finalization" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project             = var.project_id
  name                = "coda-clinical-source-finalization"
  location            = var.region
  deletion_protection = true

  template {
    parallelism = 1
    task_count  = 1

    template {
      service_account = google_service_account.clinical_source_acquisition[0].email
      timeout         = "300s"
      max_retries     = 0

      containers {
        name    = "source-finalization"
        image   = var.source_maintenance_image
        command = ["node"]
        args    = ["dist/src/server/clinicalCorpus/operations/finalizeHostedClinicalSourceRefresh.js"]

        resources {
          limits = {
            cpu    = "1"
            memory = "512Mi"
          }
        }

        env {
          name  = "CODA_CLINICAL_CORPUS_BUCKET"
          value = var.clinical_corpus_bucket_name
        }
      }
    }
  }

  depends_on = [
    google_project_service.workload,
    google_storage_bucket_iam_member.clinical_source_operations_create,
    google_storage_bucket_iam_member.clinical_source_operations_read,
    google_storage_bucket_iam_member.clinical_source_operations_state,
  ]
}

resource "google_cloud_run_v2_job" "clinical_payload_reconciliation" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  project             = var.project_id
  name                = "coda-clinical-payload-reconciliation"
  location            = var.region
  deletion_protection = true

  template {
    parallelism = 1
    task_count  = 1

    template {
      service_account = google_service_account.clinical_corpus_maintainer[0].email
      timeout         = "3600s"
      max_retries     = 0

      containers {
        image   = var.source_maintenance_image
        command = ["node"]
        args    = ["dist/src/server/clinicalCorpus/operations/hostedClinicalPayloadReconciliationStart.js"]

        resources {
          limits = {
            cpu    = "1"
            memory = "1Gi"
          }
        }

        env {
          name  = "CODA_CLINICAL_CORPUS_BUCKET"
          value = var.clinical_corpus_bucket_name
        }
        env {
          name  = "CODA_CLOUD_SQL_INSTANCE"
          value = google_sql_database_instance.clinical_evidence.connection_name
        }
        env {
          name  = "CODA_CLOUD_SQL_IAM_USER"
          value = trimsuffix(google_service_account.clinical_corpus_maintainer[0].email, ".gserviceaccount.com")
        }
        env {
          name = "CODA_CLINICAL_PAYLOAD_RECONCILIATION_CONFIG"
          value = jsonencode({
            schema_version         = 1
            expected_database_name = local.clinical_database_name
            page_size              = 256
            metadata_concurrency   = 32
          })
        }
      }

      vpc_access {
        egress = "PRIVATE_RANGES_ONLY"

        network_interfaces {
          network    = google_compute_network.clinical_evidence.name
          subnetwork = google_compute_subnetwork.operations.name
        }
      }
    }
  }

  depends_on = [
    google_project_service.workload,
    google_project_iam_member.clinical_corpus_maintainer_cloud_sql,
    google_sql_user.clinical_corpus_maintainer,
    google_storage_bucket_iam_member.clinical_corpus_maintainer_payload_read,
  ]
}

resource "google_cloud_scheduler_job" "clinical_source_acquisition" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project          = var.project_id
  region           = var.region
  name             = "coda-clinical-source-acquisition-daily"
  description      = "Daily change-driven clinical corpus refresh through exact activation and source finalization"
  schedule         = var.clinical_source_acquisition_schedule
  time_zone        = "America/New_York"
  attempt_deadline = "1800s"
  paused           = !var.clinical_source_scheduler_enabled

  retry_config {
    retry_count = 0
  }

  http_target {
    http_method = "POST"
    uri         = "https://workflowexecutions.googleapis.com/v1/${google_workflows_workflow.clinical_refresh[0].id}/executions"

    headers = {
      "Content-Type" = "application/json"
    }
    body = base64encode(jsonencode({ argument = "{}" }))

    oauth_token {
      service_account_email = google_service_account.clinical_source_scheduler[0].email
      scope                 = "https://www.googleapis.com/auth/cloud-platform"
    }
  }

  depends_on = [
    google_project_iam_member.clinical_refresh_scheduler,
    google_project_service.workload,
  ]
}

resource "google_logging_metric" "clinical_source_acquisition_success" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project = var.project_id
  name    = "coda/clinical_corpus_refresh_success"
  filter  = <<-EOT
    resource.type="workflows.googleapis.com/Workflow"
    resource.labels.workflow_id="coda-clinical-corpus-refresh"
    jsonPayload.owner="clinical-corpus-refresh"
    jsonPayload.execution_succeeded=true
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }
}

resource "google_logging_metric" "clinical_source_acquisition_failure" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project = var.project_id
  name    = "coda/clinical_corpus_refresh_failure"
  filter  = <<-EOT
    resource.type="workflows.googleapis.com/Workflow"
    resource.labels.workflow_id="coda-clinical-corpus-refresh"
    jsonPayload.owner="clinical-corpus-refresh"
    jsonPayload.execution_succeeded=false
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }
}

resource "google_logging_metric" "clinical_source_incomplete" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project = var.project_id
  name    = "coda/clinical_corpus_refresh_source_incomplete"
  filter  = <<-EOT
    resource.type="workflows.googleapis.com/Workflow"
    resource.labels.workflow_id="coda-clinical-corpus-refresh"
    jsonPayload.owner="clinical-corpus-refresh"
    jsonPayload.execution_succeeded=true
    jsonPayload.source_checks_complete=false
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }
}

resource "google_logging_metric" "clinical_optional_visual_unavailable" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project = var.project_id
  name    = "coda/clinical_corpus_refresh_optional_visual_unavailable"
  filter  = <<-EOT
    resource.type="workflows.googleapis.com/Workflow"
    resource.labels.workflow_id="coda-clinical-corpus-refresh"
    jsonPayload.owner="clinical-corpus-refresh"
    jsonPayload.execution_succeeded=true
    jsonPayload.source_observations.pmc.details.changed_article_media.unavailable_static_images>0
  EOT

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }
}

resource "google_monitoring_notification_channel" "clinical_operations_email" {
  count = var.clinical_source_acquisition_enabled && var.clinical_operations_notification_email != null ? 1 : 0

  project      = var.project_id
  display_name = "Coda Clinical Evidence operations email"
  type         = "email"

  labels = { email_address = var.clinical_operations_notification_email }
}

resource "google_monitoring_alert_policy" "clinical_source_acquisition_failure" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project               = var.project_id
  display_name          = "Coda clinical corpus refresh failed"
  combiner              = "OR"
  notification_channels = local.clinical_operations_notification_channels
  severity              = "ERROR"

  documentation {
    mime_type = "text/markdown"
    content   = "A clinical corpus workflow failed. Inspect the execution and its child jobs, including cleanup. Parent and child failures are grouped to at most one email per hour. An incident closing after log inactivity does not prove recovery; require a later successful execution and cleanup readback."
  }

  alert_strategy {
    notification_prompts = ["OPENED"]
    auto_close           = "3600s"
    notification_rate_limit {
      period = "3600s"
    }
  }

  conditions {
    display_name = "Refresh workflow emitted a failure"
    condition_matched_log {
      filter = google_logging_metric.clinical_source_acquisition_failure[0].filter
    }
  }
}

resource "google_monitoring_alert_policy" "clinical_source_incomplete" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project               = var.project_id
  display_name          = "Coda clinical corpus source errors"
  combiner              = "OR"
  notification_channels = local.clinical_operations_notification_channels
  severity              = "WARNING"

  documentation {
    mime_type = "text/markdown"
    content   = "A source's night needs a person: the job that ran it logged a clinical-source-night line with needs_person above zero, whichever route ran it (the nightly refresh or a backfill), and the line names the source and the night's attempted_at. Each thing is reported once: a set-aside item that crossed its own threshold (a rejected item on its third failed attempt, work tried every night on its third failed night whatever the cause, an integrity or bug failure at once; then that source's set-aside records), or a whole source whose nights without progress were reported (our own failure on its first night, an outside one on its second; then its failure kind and code, and the words in its diagnostics_object). Backlog, items waiting on their own retry, and bounded progress with work left do not email. Repeats are grouped to at most one email per day. Incident closure after log inactivity does not establish recovery."
  }

  alert_strategy {
    notification_prompts = ["OPENED"]
    auto_close           = "86400s"
    notification_rate_limit {
      period = "86400s"
    }
  }

  conditions {
    display_name = "Refresh source checks need attention"
    condition_matched_log {
      # Email only what needs a person. Backlog, records waiting on their own
      # retry, and bounded progress with work left are the pipeline working;
      # each source counts its own nights so one bad night does not page. The
      # line is written by the job that saves the night, on Batch or Cloud Run,
      # whichever route ran it, so no resource type is named.
      filter = <<-EOT
        jsonPayload.owner="clinical-source-night"
        jsonPayload.needs_person>0
      EOT
    }
  }
}

resource "google_monitoring_alert_policy" "clinical_source_acquisition_stale" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project               = var.project_id
  display_name          = "Coda clinical corpus refresh missing"
  enabled               = var.clinical_source_scheduler_enabled
  combiner              = "OR"
  notification_channels = local.clinical_operations_notification_channels
  severity              = "ERROR"

  alert_strategy {
    notification_prompts = ["OPENED"]
  }

  conditions {
    display_name = "No successful daily refresh for 25 hours"
    condition_prometheus_query_language {
      # Native absence conditions cap at 23h30m. PromQL preserves a one-hour
      # grace period beyond the daily schedule within the supported 25h window.
      query               = "absent_over_time({\"__name__\"=\"logging_googleapis_com:user_${replace(google_logging_metric.clinical_source_acquisition_success[0].name, "/", "_")}\"}[25h])"
      duration            = "0s"
      evaluation_interval = "300s"
    }
  }
}
