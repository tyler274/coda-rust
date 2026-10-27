# HTTPS adapter for products outside this VPC. Reuses the existing SMTP transport
# and NAT address; IAM authenticates callers before the adapter receives mail.
variable "mail_relay_image" {
  type        = string
  default     = null
  description = "Reviewed mail-relay image digest. Null leaves the service absent."
  validation {
    condition     = var.mail_relay_image == null ? true : can(regex("@sha256:[0-9a-f]{64}$", var.mail_relay_image))
    error_message = "Pin the relay image by sha256 digest."
  }
}

variable "mail_relay_invokers" {
  type        = set(string)
  default     = []
  description = "Explicit serviceAccount:<email> callers for this environment's mail service."
  validation {
    condition     = alltrue([for member in var.mail_relay_invokers : can(regex("^serviceAccount:[^@]+@[^@]+\\.iam\\.gserviceaccount\\.com$", member))])
    error_message = "Mail callers must be named service accounts."
  }
}

resource "google_service_account" "mail_relay" {
  count        = var.mail_relay_image == null ? 0 : 1
  project      = google_project.environment.project_id
  account_id   = "mail-relay"
  display_name = "SMTP mail relay (no application data access)"
}

resource "google_cloud_run_v2_service" "mail_relay" {
  count                = var.mail_relay_image == null ? 0 : 1
  project              = google_project.environment.project_id
  name                 = "mail-relay"
  location             = var.region
  deletion_protection  = true
  ingress              = "INGRESS_TRAFFIC_ALL"
  invoker_iam_disabled = false
  template {
    service_account                  = google_service_account.mail_relay[0].email
    execution_environment            = "EXECUTION_ENVIRONMENT_GEN2"
    timeout                          = "60s"
    max_instance_request_concurrency = 8
    scaling { max_instance_count = 1 }
    vpc_access {
      egress = "ALL_TRAFFIC"
      network_interfaces {
        network    = google_compute_network.private.name
        subnetwork = google_compute_subnetwork.run.name
      }
    }
    containers {
      image = var.mail_relay_image
      ports { container_port = 8080 }
      resources {
        limits   = { cpu = "1", memory = "256Mi" }
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
  lifecycle {
    precondition {
      condition     = var.enable_physician_smtp_relay && length(var.mail_relay_invokers) > 0
      error_message = "Enable the existing SMTP NAT and name the environment's callers before creating mail-relay."
    }
  }
  depends_on = [google_project_service.api, google_compute_router_nat.physician_mail]
}

resource "google_cloud_run_v2_service_iam_binding" "mail_relay" {
  count    = var.mail_relay_image == null ? 0 : 1
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.mail_relay[0].name
  role     = "roles/run.invoker"
  members  = var.mail_relay_invokers
}

output "mail_relay_url" {
  description = "Audience and base URL for this environment's IAM-protected mail service."
  value       = try(google_cloud_run_v2_service.mail_relay[0].uri, null)
}
