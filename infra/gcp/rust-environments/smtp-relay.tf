# Workspace authenticates this exact public IP. No Gmail user, OAuth grant,
# SMTP password or inbound listener is created. Only physician runtime sends
# public traffic through NAT; SQL continues using the existing private route.
variable "enable_physician_smtp_relay" {
  type        = bool
  default     = false
  description = "Reserve static outbound IPv4 and route physician runtime through Cloud NAT for Workspace SMTP relay. Separately review/apply the saved plan and allow this IP in Workspace before enabling mail delivery."
}

resource "google_compute_address" "physician_mail" {
  count        = var.enable_physician_smtp_relay ? 1 : 0
  project      = google_project.environment.project_id
  name         = "physician-mail-egress"
  region       = var.region
  address_type = "EXTERNAL"
}

resource "google_compute_router" "physician_mail" {
  count   = var.enable_physician_smtp_relay ? 1 : 0
  project = google_project.environment.project_id
  name    = "physician-mail-egress"
  region  = var.region
  network = google_compute_network.private.id
}

resource "google_compute_router_nat" "physician_mail" {
  count                              = var.enable_physician_smtp_relay ? 1 : 0
  project                            = google_project.environment.project_id
  name                               = "physician-mail-egress"
  region                             = var.region
  router                             = google_compute_router.physician_mail[0].name
  nat_ip_allocate_option             = "MANUAL_ONLY"
  nat_ips                            = [google_compute_address.physician_mail[0].self_link]
  source_subnetwork_ip_ranges_to_nat = "LIST_OF_SUBNETWORKS"
  subnetwork {
    name                    = google_compute_subnetwork.run.id
    source_ip_ranges_to_nat = ["ALL_IP_RANGES"]
  }
}

output "physician_mail_egress_ip" {
  description = "Exact /32 to allow in Workspace SMTP relay. No mailbox or SMTP authentication. Require TLS and domain-owned sender addresses."
  value       = try(google_compute_address.physician_mail[0].address, null)
}
