# Production mail only. The existing project, APIs and state bucket have their
# own owners. This root owns no product data, product service or project IAM.
locals {
  project = "coda-health-prod"
  region  = "us-east1"
}

variable "image" {
  type        = string
  default     = null
  description = "Digest of the reviewed mail-relay image copied to this root's registry. Null prepares the registry and egress before the first image can be published."
  validation {
    condition     = var.image == null ? true : can(regex("^us-east1-docker\\.pkg\\.dev/coda-health-prod/shared-mail/mail-relay@sha256:[0-9a-f]{64}$", var.image))
    error_message = "Use a digest-pinned mail-relay image in the production shared-mail registry."
  }
}

variable "invokers" {
  type        = set(string)
  default     = ["serviceAccount:workspace-web@coda-gym.iam.gserviceaccount.com"]
  description = "Explicit production product identities allowed to submit email."
  validation {
    condition     = length(var.invokers) > 0 && alltrue([for member in var.invokers : can(regex("^serviceAccount:[^@]+@[^@]+\\.iam\\.gserviceaccount\\.com$", member))])
    error_message = "Name service-account callers; public or user-wide invocation is not allowed."
  }
}

resource "google_artifact_registry_repository" "mail" {
  project       = local.project
  location      = local.region
  repository_id = "shared-mail"
  format        = "DOCKER"
  docker_config { immutable_tags = true }
}

resource "google_compute_network" "mail" {
  project                 = local.project
  name                    = "shared-mail"
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "mail" {
  project                  = local.project
  name                     = "shared-mail"
  region                   = local.region
  network                  = google_compute_network.mail.id
  ip_cidr_range            = "10.35.0.0/26"
  private_ip_google_access = true
}

resource "google_compute_address" "mail" {
  project      = local.project
  name         = "shared-mail-egress"
  region       = local.region
  address_type = "EXTERNAL"
}

resource "google_compute_router" "mail" {
  project = local.project
  name    = "shared-mail-egress"
  region  = local.region
  network = google_compute_network.mail.id
}

resource "google_compute_router_nat" "mail" {
  project                            = local.project
  name                               = "shared-mail-egress"
  region                             = local.region
  router                             = google_compute_router.mail.name
  nat_ip_allocate_option             = "MANUAL_ONLY"
  nat_ips                            = [google_compute_address.mail.self_link]
  source_subnetwork_ip_ranges_to_nat = "LIST_OF_SUBNETWORKS"
  subnetwork {
    name                    = google_compute_subnetwork.mail.id
    source_ip_ranges_to_nat = ["ALL_IP_RANGES"]
  }
}

resource "google_service_account" "mail" {
  count        = var.image == null ? 0 : 1
  project      = local.project
  account_id   = "shared-mail"
  display_name = "Production SMTP mail relay (no application data access)"
}

resource "google_cloud_run_v2_service" "mail" {
  count                = var.image == null ? 0 : 1
  project              = local.project
  name                 = "mail-relay"
  location             = local.region
  deletion_protection  = true
  ingress              = "INGRESS_TRAFFIC_ALL"
  invoker_iam_disabled = false
  template {
    service_account                  = google_service_account.mail[0].email
    execution_environment            = "EXECUTION_ENVIRONMENT_GEN2"
    timeout                          = "60s"
    max_instance_request_concurrency = 8
    scaling {
      min_instance_count = 1 # Keep the sign-in mail path warm between requests.
      max_instance_count = 1
    }
    vpc_access {
      egress = "ALL_TRAFFIC"
      network_interfaces {
        network    = google_compute_network.mail.name
        subnetwork = google_compute_subnetwork.mail.name
      }
    }
    containers {
      image = var.image
      ports { container_port = 8080 }
      resources {
        limits   = { cpu = "1", memory = "512Mi" } # Cloud Run Gen2 minimum.
        cpu_idle = true
      }
      env {
        name  = "MAIL_FROM"
        value = "signin@codahealth.ai"
      }
      env {
        name  = "MAIL_LISTEN_ADDR"
        value = "0.0.0.0:8080"
      }
      startup_probe {
        http_get { path = "/readyz" }
      }
    }
  }
  depends_on = [google_compute_router_nat.mail]
}

resource "google_cloud_run_v2_service_iam_binding" "mail" {
  count    = var.image == null ? 0 : 1
  project  = local.project
  location = local.region
  name     = google_cloud_run_v2_service.mail[0].name
  role     = "roles/run.invoker"
  members  = var.invokers
}

output "smtp_egress_ip" {
  description = "Allow this exact /32 in Workspace SMTP relay before sending."
  value       = google_compute_address.mail.address
}

output "mail_relay_url" {
  description = "Production mail URL and ID-token audience; absent until an image is selected."
  value       = try(google_cloud_run_v2_service.mail[0].uri, null)
}
