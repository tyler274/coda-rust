# Direct IAP is supported in the pinned stable google 7.24.0 provider:
# https://registry.terraform.io/providers/hashicorp/google/7.24.0/docs/resources/cloud_run_v2_service#iap_enabled
# https://cloud.google.com/run/docs/securing/identity-aware-proxy-cloud-run
# IAP intercepts ALL ingress paths, including run.app. No LB/IAP bypass path,
# no allUsers invoker, no invoker_iam_disabled. Internal organization OAuth only.
resource "google_project_service_identity" "iap" {
  provider   = google-beta
  project    = google_project.environment.project_id
  service    = "iap.googleapis.com"
  depends_on = [google_project_service.api]
}

resource "google_cloud_run_v2_service" "app" {
  for_each             = local.workloads
  project              = google_project.environment.project_id
  name                 = each.key
  location             = var.region
  deletion_protection  = true
  ingress              = "INGRESS_TRAFFIC_ALL"
  iap_enabled          = true
  invoker_iam_disabled = false
  template {
    service_account                  = google_service_account.app["${each.key}-runtime"].email
    execution_environment            = "EXECUTION_ENVIRONMENT_GEN2"
    timeout                          = "300s"
    max_instance_request_concurrency = 8
    scaling {
      min_instance_count = 0
      max_instance_count = var.max_instances
    }
    vpc_access {
      egress = "PRIVATE_RANGES_ONLY"
      network_interfaces {
        network    = google_compute_network.private.name
        subnetwork = google_compute_subnetwork.run.name
      }
    }
    containers {
      image = try(var.images[each.key].runtime, "")
      ports { container_port = 8080 }
      resources {
        limits            = { cpu = "1", memory = "1Gi" }
        cpu_idle          = true
        startup_cpu_boost = false
      }
      env {
        name  = "DOPPLER_PROJECT"
        value = "coda-rust"
      }
      env {
        name  = "DOPPLER_CONFIG"
        value = try(var.doppler_configs["${each.key}-runtime"], "")
      }
      env {
        name = "DOPPLER_TOKEN"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.doppler["${each.key}-runtime"].secret_id
            version = try(var.doppler_token_versions[each.key].runtime, "")
          }
        }
      }
      dynamic "env" {
        for_each = try(var.runtime_environment[each.key], {})
        content {
          name  = env.key
          value = env.value
        }
      }
      dynamic "env" {
        for_each = each.key == "duet" ? google_storage_bucket.duet_privacy_journal : {}
        content {
          name  = "DUET_PRIVACY_JOURNAL_BUCKET"
          value = env.value.name
        }
      }
      startup_probe {
        initial_delay_seconds = 0
        period_seconds        = 5
        timeout_seconds       = 2
        failure_threshold     = 24
        http_get { path = "/readyz" }
      }
    }
  }
  lifecycle {
    # Release owns ONLY image digest, revision name, revision traffic/tag routing
    # and CLI client metadata. Infrastructure still owns auth, env, network,
    # identity, secret version pins, resource caps and probes. Never ignore template.
    ignore_changes = [template[0].containers[0].image, template[0].revision, traffic, client, client_version]
  }
  depends_on = [google_secret_manager_secret_iam_binding.bootstrap_reader, google_sql_user.app, google_project_service_identity.iap, google_storage_bucket_iam_member.duet_privacy_journal]
}

# Duet's independent privacy receipt journal. Hosted Duet refuses to serve
# without it (crates/duet-api/src/main.rs). Receipts are create-only
# (ifGenerationMatch=0), so the runtime gets create + read, never delete or
# overwrite. Physician has no journal and gets no binding.
resource "google_storage_bucket" "duet_privacy_journal" {
  for_each                    = contains(local.workloads, "duet") ? toset(["duet"]) : toset([])
  project                     = google_project.environment.project_id
  name                        = "${local.project_id}-duet-privacy-journal"
  location                    = var.region
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false
  versioning { enabled = true }
  lifecycle { prevent_destroy = true }
}

resource "google_storage_bucket_iam_member" "duet_privacy_journal" {
  for_each = { for role in ["roles/storage.objectCreator", "roles/storage.objectViewer"] : role => role if contains(local.workloads, "duet") }
  bucket   = google_storage_bucket.duet_privacy_journal["duet"].name
  role     = each.value
  member   = "serviceAccount:${google_service_account.app["duet-runtime"].email}"
}

resource "google_cloud_run_v2_service_iam_binding" "iap_invoker" {
  for_each = local.workloads
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.app[each.key].name
  role     = "roles/run.invoker"
  members  = ["serviceAccount:${google_project_service_identity.iap.email}"]
}

resource "google_iap_web_cloud_run_service_iam_binding" "team" {
  for_each               = local.workloads
  project                = google_project.environment.project_id
  location               = var.region
  cloud_run_service_name = google_cloud_run_v2_service.app[each.key].name
  role                   = "roles/iap.httpsResourceAccessor"
  members                = ["group:${var.iap_group}"]
}

resource "google_cloud_run_v2_job" "migration" {
  for_each            = local.workloads
  project             = google_project.environment.project_id
  name                = "${each.key}-migrate"
  location            = var.region
  deletion_protection = true
  template {
    task_count  = 1
    parallelism = 1
    template {
      service_account = google_service_account.app["${each.key}-migration"].email
      timeout         = "600s"
      max_retries     = 0
      vpc_access {
        egress = "PRIVATE_RANGES_ONLY"
        network_interfaces {
          network    = google_compute_network.private.name
          subnetwork = google_compute_subnetwork.run.name
        }
      }
      containers {
        image = try(var.images[each.key].migration, "")
        resources { limits = { cpu = "1", memory = "1Gi" } }
        env {
          name  = "DOPPLER_PROJECT"
          value = "coda-rust"
        }
        env {
          name  = "DOPPLER_CONFIG"
          value = try(var.doppler_configs["${each.key}-migration"], "")
        }
        env {
          name = "DOPPLER_TOKEN"
          value_source {
            secret_key_ref {
              secret  = google_secret_manager_secret.doppler["${each.key}-migration"].secret_id
              version = try(var.doppler_token_versions[each.key].migration, "")
            }
          }
        }
      }
    }
  }
  lifecycle {
    ignore_changes = [template[0].template[0].containers[0].image, client, client_version]
  }
  depends_on = [google_secret_manager_secret_iam_binding.bootstrap_reader, google_sql_user.app]
}
