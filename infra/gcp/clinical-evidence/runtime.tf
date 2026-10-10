locals {
  # What the clinical evidence server needs wherever it runs: on Cloud Run's GPU, or on a serving replica.
  clinical_evidence_server_environment = [
    { name = "CODA_CLOUD_SQL_INSTANCE", value = google_sql_database_instance.clinical_evidence.connection_name },
    { name = "CODA_CLOUD_SQL_IAM_USER", value = trimsuffix(google_service_account.clinical_retrieval.email, ".gserviceaccount.com") },
    { name = "CODA_CLINICAL_DATABASE", value = google_sql_database.clinical.name },
    { name = "CODA_CLINICAL_CORPUS_BUCKET", value = var.clinical_corpus_bucket_name },
  ]
}

resource "google_cloud_run_v2_service" "clinical_evidence" {
  count = var.gpu_clinical_retrieval_image == null || !var.clinical_evidence_service_enabled ? 0 : 1

  project             = var.project_id
  name                = "coda-clinical-evidence"
  location            = var.gpu_region
  deletion_protection = true
  ingress             = "INGRESS_TRAFFIC_ALL"

  # Cloud Run records the client that last created a revision. That client
  # metadata is not desired service configuration. The generated revision name
  # remains provider-managed so a code or environment change cannot accidentally
  # reuse a prior immutable revision name.
  lifecycle {
    ignore_changes = [
      client,
      client_version,
    ]
  }

  # Omitting this optional/computed block retains the remote scaling mode.
  # Explicitly resume automatic scaling after a parked maintenance window.
  scaling {
    scaling_mode          = var.clinical_evidence_service_paused ? "MANUAL" : "AUTOMATIC"
    manual_instance_count = var.clinical_evidence_service_paused ? 0 : null
  }

  # With serving replicas, this service is only their authenticated entry point: no GPU, no index, and it relays
  # each request to the replicas' internal load balancer (serving-replicas.tf).
  template {
    service_account                  = google_service_account.clinical_retrieval.email
    timeout                          = "60s"
    max_instance_request_concurrency = local.clinical_serving_replicas_enabled ? 80 : 8
    gpu_zonal_redundancy_disabled    = local.clinical_serving_replicas_enabled ? null : true

    scaling {
      min_instance_count = var.proof_window_active || local.clinical_serving_replicas_enabled ? 1 : 0
      max_instance_count = local.clinical_serving_replicas_enabled ? 4 : 1
    }

    containers {
      image   = var.gpu_clinical_retrieval_image
      command = local.clinical_serving_replicas_enabled ? ["node"] : null
      args    = local.clinical_serving_replicas_enabled ? ["dist/src/server/clinicalCorpus/serve/hostedClinicalEvidenceEntryStart.js"] : null

      resources {
        limits = local.clinical_serving_replicas_enabled ? {
          cpu    = "1"
          memory = "1Gi"
          } : {
          cpu              = "8"
          memory           = "32Gi"
          "nvidia.com/gpu" = "1"
        }
        cpu_idle          = false
        startup_cpu_boost = true
      }

      dynamic "env" {
        for_each = local.clinical_serving_replicas_enabled ? [1] : []
        content {
          name  = "CODA_CLINICAL_SERVING_URL"
          value = "http://${google_compute_address.clinical_serving.address}:8080"
        }
      }

      dynamic "env" {
        for_each = local.clinical_serving_replicas_enabled ? [] : concat(local.clinical_evidence_server_environment, [
          { name = "CODA_CLINICAL_RETRIEVAL_CACHE_ROOT", value = "/tmp/coda-clinical-retrieval" },
          { name = "CODA_CLINICAL_RETRIEVAL_WORKERS", value = tostring(var.clinical_evidence_worker_count) },
        ])
        content {
          name  = env.value.name
          value = env.value.value
        }
      }

      startup_probe {
        # Cloud Run limits the startup probe window to 240 seconds.
        # https://cloud.google.com/run/docs/configuring/healthchecks
        initial_delay_seconds = 0
        timeout_seconds       = 5
        period_seconds        = 10
        failure_threshold     = 24

        http_get {
          # The entry starts at once; whether a replica is ready is the load balancer's answer, not its own.
          path = local.clinical_serving_replicas_enabled ? "/health" : "/ready"
          port = 8080
        }
      }
    }

    dynamic "node_selector" {
      for_each = local.clinical_serving_replicas_enabled ? [] : [1]
      content {
        accelerator = "nvidia-l4"
      }
    }

    vpc_access {
      egress = "PRIVATE_RANGES_ONLY"

      network_interfaces {
        network    = google_compute_network.clinical_evidence.name
        subnetwork = google_compute_subnetwork.retrieval.name
      }
    }
  }

  depends_on = [
    google_project_service.workload,
    google_project_iam_member.clinical_retrieval_cloud_sql,
    google_sql_user.clinical_retrieval,
    google_storage_bucket_iam_member.clinical_retrieval_build_read,
  ]
}

resource "google_cloud_run_v2_service_iam_member" "invoker" {
  for_each = var.clinical_evidence_service_enabled ? var.clinical_evidence_invokers : toset([])

  project  = var.project_id
  location = var.gpu_region
  name     = google_cloud_run_v2_service.clinical_evidence[0].name
  role     = "roles/run.invoker"
  member   = each.value
}

resource "google_cloud_run_v2_job" "database_migrator" {
  count = var.migration_image == null ? 0 : 1

  project             = var.project_id
  name                = "coda-clinical-database-migrator"
  location            = var.region
  deletion_protection = true

  template {
    template {
      service_account = google_service_account.database_migrator.email
      timeout         = "900s"
      max_retries     = 0

      containers {
        image   = var.migration_image
        command = ["node"]
        args    = ["dist/src/server/clinicalCorpus/operations/migrateHostedClinicalDatabase.js"]

        resources {
          limits = {
            cpu    = "1"
            memory = "1Gi"
          }
        }

        env {
          name  = "CODA_CLOUD_SQL_INSTANCE"
          value = google_sql_database_instance.clinical_evidence.connection_name
        }
        env {
          name  = "CODA_CLOUD_SQL_MIGRATION_USER"
          value = trimsuffix(google_service_account.database_migrator.email, ".gserviceaccount.com")
        }
        env {
          name  = "CODA_CLOUD_SQL_CLINICAL_RETRIEVAL_USER"
          value = trimsuffix(google_service_account.clinical_retrieval.email, ".gserviceaccount.com")
        }
        env {
          name  = "CODA_CLOUD_SQL_CLINICAL_ACTIVATOR_USER"
          value = trimsuffix(google_service_account.clinical_activator.email, ".gserviceaccount.com")
        }
        dynamic "env" {
          for_each = var.clinical_source_acquisition_enabled ? [1] : []
          content {
            name  = "CODA_CLOUD_SQL_CLINICAL_MAINTAINER_USER"
            value = trimsuffix(google_service_account.clinical_corpus_maintainer[0].email, ".gserviceaccount.com")
          }
        }
        dynamic "env" {
          for_each = var.clinical_retrieval_build_enabled ? [1] : []
          content {
            name  = "CODA_CLOUD_SQL_CLINICAL_BUILDER_USER"
            value = trimsuffix(google_service_account.clinical_retrieval_builder[0].email, ".gserviceaccount.com")
          }
        }
        env {
          name  = "CODA_CLINICAL_DATABASE"
          value = google_sql_database.clinical.name
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
    google_project_iam_member.migrator_cloud_sql,
    google_sql_user.database_migrator,
    google_sql_user.clinical_retrieval,
    google_sql_user.clinical_activator,
    google_sql_user.clinical_corpus_maintainer,
    google_sql_user.clinical_retrieval_builder,
  ]
}

resource "google_cloud_run_v2_job" "clinical_retrieval_activator" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project             = var.project_id
  name                = "coda-clinical-retrieval-activator"
  location            = var.region
  deletion_protection = true

  template {
    template {
      service_account = google_service_account.clinical_activator.email
      timeout         = "${local.clinical_activation_timeout_seconds}s"
      max_retries     = 0

      containers {
        name    = "retrieval-activator"
        image   = var.source_maintenance_image
        command = ["node"]
        args    = ["dist/src/server/clinicalCorpus/build/activateHostedClinicalRetrieval.js"]

        resources {
          limits = {
            cpu    = "2"
            memory = "4Gi"
          }
        }

        env {
          name  = "CODA_CLOUD_SQL_INSTANCE"
          value = google_sql_database_instance.clinical_evidence.connection_name
        }
        env {
          name  = "CODA_CLOUD_SQL_IAM_USER"
          value = trimsuffix(google_service_account.clinical_activator.email, ".gserviceaccount.com")
        }
        env {
          name  = "CODA_CLINICAL_DATABASE"
          value = google_sql_database.clinical.name
        }
        env {
          name  = "CODA_CLINICAL_CORPUS_BUCKET"
          value = var.clinical_corpus_bucket_name
        }
        env {
          name  = "CODA_CLINICAL_RETRIEVAL_ACTOR"
          value = "production-clinical-retrieval-activation-job"
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
    google_project_iam_member.clinical_activator_cloud_sql,
    google_sql_user.clinical_activator,
    google_storage_bucket_iam_member.clinical_activator_refresh_read,
  ]
}

# Closing serving is one fixed job. Cloud Run requires actAs on the serving identity even for a
# scaling-only update, so that authority lives with an identity nothing but this job runs as, never with
# the workflow or the serving identity itself. The workflow may only run the job, without overrides.
resource "google_service_account" "clinical_serving_closer" {
  count = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0

  project      = var.project_id
  account_id   = "coda-clinical-serving-closer"
  display_name = "Coda clinical serving closer"

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_custom_role" "clinical_refresh_service_control" {
  count       = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0
  project     = var.project_id
  role_id     = "CodaClinicalRefreshServingControl"
  title       = "Coda Clinical Serving Close"
  description = "Inspect the clinical serving service and set it to zero instances."
  permissions = ["run.services.get", "run.services.update"]
}

resource "google_cloud_run_v2_service_iam_member" "clinical_serving_closer_control" {
  count    = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0
  project  = var.project_id
  location = var.gpu_region
  name     = google_cloud_run_v2_service.clinical_evidence[0].name
  role     = google_project_iam_custom_role.clinical_refresh_service_control[0].name
  member   = "serviceAccount:${google_service_account.clinical_serving_closer[0].email}"
}

# Draining the replica group is the other half of closing serving. Compute grants on instance groups only at
# project level, and IAM conditions can't name an instance group (resource.name isn't available for them), so this
# reaches every instance group in the project: today the serving group and Batch's short-lived ones. Only the fixed
# close job runs as this identity, and the workflow can't override what it runs (Dev, 2026-10-04).
resource "google_project_iam_custom_role" "clinical_serving_replica_close" {
  count       = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0
  project     = var.project_id
  role_id     = "CodaClinicalServingReplicaClose"
  title       = "Coda Clinical Serving Replica Close"
  description = "Inspect instance groups and resize them; held only by the fixed serving close job."
  permissions = ["compute.instanceGroupManagers.get", "compute.instanceGroupManagers.update"]
}

resource "google_project_iam_member" "clinical_serving_closer_replicas" {
  count   = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0
  project = var.project_id
  role    = google_project_iam_custom_role.clinical_serving_replica_close[0].id
  member  = "serviceAccount:${google_service_account.clinical_serving_closer[0].email}"
}

resource "google_service_account_iam_member" "clinical_serving_closer_identity" {
  count              = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0
  service_account_id = google_service_account.clinical_retrieval.name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.clinical_serving_closer[0].email}"
}

resource "google_cloud_run_v2_job" "clinical_serving_close" {
  count = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0

  project             = var.project_id
  name                = "coda-clinical-serving-close"
  location            = var.region
  deletion_protection = true

  template {
    template {
      service_account = google_service_account.clinical_serving_closer[0].email
      timeout         = "600s"
      max_retries     = 0

      containers {
        name    = "serving-close"
        image   = var.source_maintenance_image
        command = ["node"]
        args    = ["dist/src/server/clinicalCorpus/serve/closeHostedClinicalServing.js"]

        resources {
          limits = {
            cpu    = "1"
            memory = "512Mi"
          }
        }

        env {
          name  = "CODA_CLINICAL_SERVING_SERVICE"
          value = google_cloud_run_v2_service.clinical_evidence[0].id
        }

        env {
          name  = "CODA_CLINICAL_SERVING_REPLICAS"
          value = local.clinical_serving_group
        }
      }
    }
  }

  depends_on = [
    google_project_service.workload,
    google_cloud_run_v2_service_iam_member.clinical_serving_closer_control,
    google_project_iam_member.clinical_serving_closer_replicas,
    google_service_account_iam_member.clinical_serving_closer_identity,
  ]
}

resource "google_cloud_run_v2_job_iam_member" "clinical_refresh_serving_close" {
  count    = local.clinical_refresh_enabled && var.clinical_evidence_service_enabled ? 1 : 0
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_job.clinical_serving_close[0].name
  role     = "roles/run.jobsExecutor"
  member   = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}
