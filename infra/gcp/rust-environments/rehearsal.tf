# The physician rehearsal identity: one service account that drives synthetic
# chats through physician's IAP (workspace-tools observability-drill langfuse).
# IAP admits it like any team member; physician's POST /auth/rehearsal then
# re-verifies IAP's assertion, requires this exact principal and mints a
# session for the rehearsal physician only. Staging only.
variable "physician_rehearsal_enabled" {
  type        = bool
  default     = false
  description = "Create the physician rehearsal service account, admit it through physician's IAP and enable POST /auth/rehearsal. Staging with workloads and physician_simulation_practices only."
  validation {
    condition     = !var.physician_rehearsal_enabled || (var.environment == "stg" && var.enable_workloads && length(var.physician_simulation_practices) > 0)
    error_message = "The rehearsal sign-in is staging-only and needs workloads and a rehearsal practice in physician_simulation_practices."
  }
}

locals {
  physician_rehearsal = var.physician_rehearsal_enabled ? toset(["physician"]) : toset([])
  # The site's own direct Cloud Run IAP audience and the one principal it
  # accepts; empty unless rehearsal is enabled.
  physician_rehearsal_env = var.physician_rehearsal_enabled ? {
    PHYSICIAN_IAP_AUDIENCE        = "/projects/${google_project.environment.number}/locations/${var.region}/services/physician"
    PHYSICIAN_REHEARSAL_PRINCIPAL = google_service_account.physician_rehearsal["physician"].email
  } : {}
}

resource "google_service_account" "physician_rehearsal" {
  for_each     = local.physician_rehearsal
  project      = google_project.environment.project_id
  account_id   = "physician-rehearsal"
  display_name = "Physician rehearsal (synthetic chats through IAP)"
}

# Only signJwt, not access tokens, keys or any project role. The drill signs
# an IAP JWT as this account; the engineering group may do the same from an
# operator workstation, and nothing else may.
resource "google_project_iam_custom_role" "physician_rehearsal_signer" {
  for_each    = local.physician_rehearsal
  project     = google_project.environment.project_id
  role_id     = "physicianRehearsalSigner"
  title       = "Physician rehearsal JWT signing"
  permissions = ["iam.serviceAccounts.signJwt"]
}

resource "google_service_account_iam_binding" "physician_rehearsal_signer" {
  for_each           = local.physician_rehearsal
  service_account_id = google_service_account.physician_rehearsal[each.key].name
  role               = google_project_iam_custom_role.physician_rehearsal_signer[each.key].name
  members = [
    "serviceAccount:${google_service_account.physician_rehearsal[each.key].email}",
    "group:${var.iap_group}",
  ]
}
