locals {
  name           = "${var.name}-${var.environment}"
  doppler_config = var.environment == "staging" ? "stg" : "prd"
  runtime_env = {
    PUBLIC_SITE_LISTEN_ADDR = "0.0.0.0:8080"
    PHYSICIAN_APP_URL       = var.physician_app_url
    APP_ENV                 = var.environment
    RELEASE_SHA             = var.release_sha
  }
}

resource "google_service_account" "site" {
  account_id   = local.name
  display_name = "Isolated public website (${var.environment})"
}

# No database, physician runtime, model, storage, or project-wide IAM grants.
resource "google_secret_manager_secret_iam_member" "doppler" {
  project   = var.project_id
  secret_id = var.doppler_token_secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.site.email}"
}

resource "google_cloud_run_v2_service" "site" {
  name                 = local.name
  location             = var.region
  deletion_protection  = true
  ingress              = "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"
  default_uri_disabled = true

  template {
    service_account                  = google_service_account.site.email
    max_instance_request_concurrency = var.concurrency
    timeout                          = "30s"

    scaling {
      min_instance_count = var.min_instances
      max_instance_count = var.max_instances
    }

    containers {
      image   = var.image
      command = ["/doppler"]
      args = [
        "run", "--no-fallback", "--no-check-version", "--config-dir=/tmp/doppler", "--project", "coda-rust", "--config", local.doppler_config,
        "--preserve-env=PUBLIC_SITE_LISTEN_ADDR,PHYSICIAN_APP_URL,APP_ENV,RELEASE_SHA",
        "--", "/public-site"
      ]
      ports {
        container_port = 8080
      }
      resources {
        limits = {
          cpu    = "1"
          memory = "512Mi"
        }
        cpu_idle          = true
        startup_cpu_boost = true
      }
      dynamic "env" {
        for_each = local.runtime_env
        content {
          name  = env.key
          value = env.value
        }
      }
      env {
        name = "DOPPLER_TOKEN"
        value_source {
          secret_key_ref {
            secret  = var.doppler_token_secret_id
            version = var.doppler_token_secret_version
          }
        }
      }
      # TCP startup checks avoid unauthenticated HTTP exemptions for staging.
      startup_probe {
        tcp_socket {
          port = 8080
        }
        initial_delay_seconds = 0
        period_seconds        = 2
        timeout_seconds       = 1
        failure_threshold     = 30
      }
    }
  }

  lifecycle {
    precondition {
      condition     = var.min_instances <= var.max_instances
      error_message = "min_instances cannot exceed max_instances."
    }
  }
  depends_on = [google_secret_manager_secret_iam_member.doppler]
}

# The external ALB does not authenticate to Cloud Run. Network ingress and the
# disabled run.app URL constrain this grant; staging also verifies Access at origin.
resource "google_cloud_run_v2_service_iam_member" "load_balancer" {
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.site.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

resource "google_compute_region_network_endpoint_group" "site" {
  name                  = local.name
  region                = var.region
  network_endpoint_type = "SERVERLESS"
  cloud_run {
    service = google_cloud_run_v2_service.site.name
  }
}

resource "google_compute_backend_service" "site" {
  name                  = local.name
  protocol              = "HTTP"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  # A staging CDN hit would bypass origin-side Cloudflare Access verification.
  # Never offer an override enabling CDN for staging, including for static assets.
  enable_cdn = var.environment == "production"

  backend {
    group = google_compute_region_network_endpoint_group.site.id
  }
  dynamic "cdn_policy" {
    for_each = var.environment == "production" ? [1] : []
    content {
      cache_mode       = "USE_ORIGIN_HEADERS"
      negative_caching = false
      # No FORCE_CACHE_ALL, cookie bypass assumptions, or authentication caching.
      cache_key_policy {
        include_host         = true
        include_protocol     = true
        include_query_string = true
      }
    }
  }
  # Serverless NEGs support neither backend health checks nor timeout_sec.
}

resource "google_compute_url_map" "site" {
  name            = local.name
  default_service = google_compute_backend_service.site.id
}

resource "google_compute_ssl_policy" "site" {
  name            = local.name
  profile         = "MODERN"
  min_tls_version = "TLS_1_2"
}

resource "google_compute_target_https_proxy" "site" {
  name             = local.name
  url_map          = google_compute_url_map.site.id
  ssl_certificates = var.certificate_ids
  ssl_policy       = google_compute_ssl_policy.site.id
}

resource "google_compute_global_address" "site" {
  name = local.name
}

resource "google_compute_global_forwarding_rule" "https" {
  name                  = local.name
  ip_address            = google_compute_global_address.site.id
  target                = google_compute_target_https_proxy.site.id
  port_range            = "443"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  network_tier          = "PREMIUM"
}
