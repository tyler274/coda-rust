# Physician-only Memorystore Redis that carries workspace_changed and
# workspace_settled between physician-sample instances and from the
# physician-worker to them. Sign-in and panel notices stay on the Postgres
# NOTIFY triggers. Duet has no key in physician_scale_apps, so it gets no cache,
# no network grant and no Redis settings.
#
# BASIC tier is a single node without replication: a restart drops pub/sub
# subscribers, and the physician bus reports `live = false` so streams resync.
# Nothing durable lives here, which is also why deletion protection is off:
# turning enable_physician_scale off is the intended way to stop the billing,
# and the reviewed saved plan is the guard against an accidental destroy.
# Recreating the instance changes its host, AUTH string and CA, so the operator
# re-runs the Doppler step in book/src/gcp.md afterwards.
resource "google_redis_instance" "physician" {
  for_each       = local.physician_scale_apps
  project        = google_project.environment.project_id
  name           = "rust-physician-cache"
  display_name   = "Physician workspace event fan-out (${var.environment})"
  region         = var.region
  tier           = "BASIC"
  memory_size_gb = 1
  redis_version  = "REDIS_7_2"
  # Private services access on the existing rust-sql-private peering; the /16
  # range has room for Cloud SQL and Memorystore. Direct VPC egress
  # (PRIVATE_RANGES_ONLY) from Cloud Run reaches it like the SQL private IP.
  connect_mode       = "PRIVATE_SERVICE_ACCESS"
  authorized_network = google_compute_network.private.id
  reserved_ip_range  = google_compute_global_address.sql.name
  # AUTH plus TLS with server authentication (port 6378). Clients pin the
  # instance CA from the physician_redis_ca_pem output; the AUTH string is
  # only ever read by the operator into Doppler.
  auth_enabled            = true
  transit_encryption_mode = "SERVER_AUTHENTICATION"
  deletion_protection     = false
  labels                  = { environment = var.environment, app = "physician", purpose = "workspace-events" }
  maintenance_policy {
    weekly_maintenance_window {
      day = "SUNDAY"
      start_time {
        hours   = 5
        minutes = 0
      }
    }
  }
  depends_on = [google_project_service.api, google_service_networking_connection.sql]
}
