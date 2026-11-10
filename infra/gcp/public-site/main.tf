locals {
  name         = "${var.name}-${var.environment}"
  staging      = var.environment == "staging"
  iap_audience = "/projects/${var.project_number}/locations/${var.region}/services/${local.name}"
  # API enablement and creation of this Google-managed identity belong to the
  # existing project's operator, not this service-only root.
  iap_service_agent = "service-${var.project_number}@gcp-sa-iap.iam.gserviceaccount.com"
  # A physician upstream is the one input that makes the site offer sign-in.
  physician = var.physician_app_url != null
  runtime_env = merge(
    {
      PUBLIC_SITE_LISTEN_ADDR = "0.0.0.0:8080"
      APP_ENV                 = var.environment
      RELEASE_SHA             = var.release_sha
    },
    local.staging ? { PUBLIC_SITE_IAP_AUDIENCE = local.iap_audience } : {},
    local.physician ? {
      PHYSICIAN_APP_URL               = var.physician_app_url
      PHYSICIAN_PROXY_SERVICE_ACCOUNT = google_service_account.site.email
    } : {},
    # The relay mounts sign with the same runtime identity as /physician.
    var.observability_relay_url == null ? {} : {
      OBSERVABILITY_RELAY_URL         = var.observability_relay_url
      PHYSICIAN_PROXY_SERVICE_ACCOUNT = google_service_account.site.email
      LANGFUSE_SIGN_IN                = var.langfuse_sso ? "sso" : "password"
    },
  )
}

resource "google_service_account" "site" {
  account_id   = local.name
  display_name = "Isolated public website (${var.environment})"
}

resource "google_cloud_run_v2_service" "site" {
  name                 = local.name
  location             = var.region
  deletion_protection  = true
  ingress              = local.staging ? "INGRESS_TRAFFIC_ALL" : "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"
  default_uri_disabled = !local.staging
  iap_enabled          = local.staging
  # Production's external ALB does not authenticate to Cloud Run, and the
  # organization's domain restriction forbids an allUsers invoker grant. With
  # the invoker check off, ALB-only ingress and the disabled run.app URL are the
  # boundary. Staging keeps the check: only the IAP service agent may invoke.
  invoker_iam_disabled = !local.staging

  template {
    # Bound long-lived /physician SSE requests; clients reconnect after this deadline.
    service_account                  = google_service_account.site.email
    max_instance_request_concurrency = var.concurrency
    timeout                          = "300s"

    scaling {
      min_instance_count = var.min_instances
      max_instance_count = var.max_instances
    }

    containers {
      image = var.image
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
}

# Authoritative, service-scoped bindings: staging has no public invoker and no
# implicit deployer/CI website access. Separate smoke access needs authorization.
resource "google_cloud_run_v2_service_iam_binding" "iap_invoker" {
  count    = local.staging ? 1 : 0
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.site.name
  role     = "roles/run.invoker"
  members  = ["serviceAccount:${local.iap_service_agent}"]
}

resource "google_iap_web_cloud_run_service_iam_binding" "team" {
  count                  = local.staging ? 1 : 0
  project                = var.project_id
  location               = var.region
  cloud_run_service_name = google_cloud_run_v2_service.site.name
  role                   = "roles/iap.httpsResourceAccessor"
  members                = ["group:${var.iap_group}"]
}

# Keep direct Cloud Run IAP as the access owner for the custom staging hostname.
# The mapping provisions TLS, not a public invoker, proxy, CDN, or load balancer.
resource "google_cloud_run_domain_mapping" "staging" {
  count    = local.staging && var.staging_domain != null ? 1 : 0
  name     = var.staging_domain
  location = var.region
  metadata {
    namespace = var.project_id
  }
  spec {
    route_name = google_cloud_run_v2_service.site.name
  }
  depends_on = [
    google_cloud_run_v2_service_iam_binding.iap_invoker,
    google_iap_web_cloud_run_service_iam_binding.team,
  ]
}

resource "google_compute_region_network_endpoint_group" "site" {
  count                 = local.staging ? 0 : 1
  name                  = local.name
  region                = var.region
  network_endpoint_type = "SERVERLESS"
  cloud_run {
    service = google_cloud_run_v2_service.site.name
  }
}

resource "google_compute_backend_service" "site" {
  count                 = local.staging ? 0 : 1
  name                  = local.name
  protocol              = "HTTP"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  enable_cdn            = true

  backend {
    group = google_compute_region_network_endpoint_group.site[0].id
  }
  cdn_policy {
    cache_mode       = "USE_ORIGIN_HEADERS"
    negative_caching = false
    # No FORCE_CACHE_ALL, cookie bypass assumptions, or authentication caching.
    cache_key_policy {
      include_host         = true
      include_protocol     = true
      include_query_string = true
    }
  }
  # Serverless NEGs support neither backend health checks nor timeout_sec.
}

resource "google_compute_url_map" "site" {
  count           = local.staging ? 0 : 1
  name            = local.name
  default_service = google_compute_backend_service.site[0].id
}

resource "google_compute_ssl_policy" "site" {
  count           = local.staging ? 0 : 1
  name            = local.name
  profile         = "MODERN"
  min_tls_version = "TLS_1_2"
}

# One Google-managed certificate for every production hostname, authorized by
# DNS records rather than by serving traffic, so it is active before DNS moves.
resource "google_certificate_manager_dns_authorization" "site" {
  for_each = local.staging ? toset([]) : toset(var.domains)
  name     = "${local.name}-${replace(each.key, ".", "-")}"
  domain   = each.key
}

resource "google_certificate_manager_certificate" "site" {
  count = local.staging ? 0 : 1
  name  = local.name
  managed {
    domains            = var.domains
    dns_authorizations = [for domain in var.domains : google_certificate_manager_dns_authorization.site[domain].id]
  }
  lifecycle {
    precondition {
      condition     = length(var.domains) > 0
      error_message = "Production requires its hostnames in domains."
    }
  }
}

resource "google_certificate_manager_certificate_map" "site" {
  count = local.staging ? 0 : 1
  name  = local.name
}

resource "google_certificate_manager_certificate_map_entry" "site" {
  for_each     = local.staging ? toset([]) : toset(var.domains)
  name         = "${local.name}-${replace(each.key, ".", "-")}"
  map          = google_certificate_manager_certificate_map.site[0].name
  certificates = [google_certificate_manager_certificate.site[0].id]
  hostname     = each.key
}

resource "google_compute_target_https_proxy" "site" {
  count           = local.staging ? 0 : 1
  name            = local.name
  url_map         = google_compute_url_map.site[0].id
  certificate_map = "//certificatemanager.googleapis.com/${google_certificate_manager_certificate_map.site[0].id}"
  ssl_policy      = google_compute_ssl_policy.site[0].id
}

resource "google_compute_global_address" "site" {
  count = local.staging ? 0 : 1
  name  = local.name
}

resource "google_compute_global_forwarding_rule" "https" {
  count                 = local.staging ? 0 : 1
  name                  = local.name
  ip_address            = google_compute_global_address.site[0].id
  target                = google_compute_target_https_proxy.site[0].id
  port_range            = "443"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  network_tier          = "PREMIUM"
}

# Plain HTTP on the same address answers with a 308 to HTTPS, keeping host, path
# and query, as production does today. It never reaches Cloud Run.
resource "google_compute_url_map" "http_redirect" {
  count = local.staging ? 0 : 1
  name  = "${local.name}-http"
  default_url_redirect {
    https_redirect         = true
    redirect_response_code = "PERMANENT_REDIRECT"
    strip_query            = false
  }
}

resource "google_compute_target_http_proxy" "http_redirect" {
  count   = local.staging ? 0 : 1
  name    = "${local.name}-http"
  url_map = google_compute_url_map.http_redirect[0].id
}

resource "google_compute_global_forwarding_rule" "http" {
  count                 = local.staging ? 0 : 1
  name                  = "${local.name}-http"
  ip_address            = google_compute_global_address.site[0].id
  target                = google_compute_target_http_proxy.http_redirect[0].id
  port_range            = "80"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  network_tier          = "PREMIUM"
}
