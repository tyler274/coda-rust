# Cloud Run and GKE platform metrics (request counts, latencies, instance and
# container CPU/memory) through the Datadog GCP integration, scoped to the
# cluster project only. Datadog's STS delegate impersonates a dedicated,
# read-only service account in that project: no service-account key exists,
# so none reaches state, Doppler or Datadog. Off by default
# (datadog_gcp_integration_enabled) because it is billable and changes an
# external account.
#
# Request/error/latency for the three services come from APM trace metrics;
# this integration adds the platform view Cloud Run itself reports (cold
# starts, instance counts, container CPU and memory, 5xx answered before the
# product container).
variable "datadog_gcp_dependency_collection" {
  description = "Default-off per-namespace collection opt-ins. Every entry requires exact resource filters verified against Datadog's integration before apply. Only SQL and Redis may be opted in; no IAM roles or global-location collection are added. Imports all metrics in the selected namespace for matching resources, so review series cost before activation."
  type        = map(object({ verified_resource_filters = set(string) }))
  default     = {}
  validation {
    condition = alltrue([for namespace, config in var.datadog_gcp_dependency_collection :
      contains(["cloudsql", "redis"], namespace) && length(config.verified_resource_filters) > 0 &&
      alltrue([for filter in config.verified_resource_filters : can(regex("^[a-z_]+:[A-Za-z0-9_:/.-]+$", filter)) && !can(regex("patient|run_id|chat_id|^(env|environment|project|project_id|region|zone|location|app|service|purpose):", filter))])
    ])
    error_message = "Dependency collection requires known namespace keys and nonempty exact verified resource identity filters; broad environment/project/location/service tags, wildcards and boolean scopes are forbidden."
  }
  validation {
    condition     = length(var.datadog_gcp_dependency_collection) == 0 || (var.environment == "staging" && var.datadog_gcp_integration_enabled)
    error_message = "Dependency collection requires the staging GCP integration gate."
  }
}

variable "datadog_gcp_global_location_enabled" {
  description = "Default-off import of metrics with no GCP location, which is where custom gauges on the global resource land: the physician worker's queue gauges and the availability probe's. Every such metric in the project is imported, so review series cost before activation."
  type        = bool
  default     = false
  validation {
    condition     = !var.datadog_gcp_global_location_enabled || (var.environment == "staging" && var.datadog_gcp_integration_enabled)
    error_message = "Global-location collection requires the staging GCP integration gate."
  }
}

locals {
  dependency_resource_types = { cloudsql = "cloudsql_database", redis = "redis_instance" }
  # The variable's validation already refuses it under grafana; the backend
  # gate here keeps the resources tied to Datadog regardless.
  datadog_gcp_enabled = local.datadog_backend && var.datadog_gcp_integration_enabled
  datadog_gcp         = local.datadog_gcp_enabled ? toset(["integration"]) : toset([])

  # Read-only roles Datadog documents for the integration. browser lets it
  # list the project it lives in; no write, secret or data-plane access.
  datadog_gcp_roles = toset([
    "roles/browser",
    "roles/cloudasset.viewer",
    "roles/compute.viewer",
    "roles/monitoring.viewer",
  ])

  # The integration API has no namespace allowlist: every namespace in the
  # project is collected unless its id is disabled. The project itself only
  # holds the Rust staging stack, and the monitored-resource filters below
  # narrow Cloud Run and GCE to the selected services and this cluster.
  datadog_gcp_metric_namespace_configs = concat([
    for namespace in sort(tolist(setsubtract(var.datadog_gcp_disabled_metric_namespaces, toset(keys(var.datadog_gcp_dependency_collection))))) : {
      id       = namespace
      disabled = true
      filters  = []
    }
    # Namespace filters select metric names, not resource tags. Resource
    # identity restrictions belong only in monitored_resource_configs.
    ], [for namespace in keys(var.datadog_gcp_dependency_collection) : {
      id       = namespace
      disabled = false
      filters  = []
  }])
}

resource "google_service_account" "datadog_gcp" {
  for_each     = local.datadog_gcp
  project      = var.cluster_project
  account_id   = "datadog-gcp-integration"
  display_name = "Datadog GCP integration (${var.environment}, read-only)"
  description  = "Impersonated by Datadog's STS delegate to read Cloud Run and GKE metrics in this project only. No key."
}

resource "google_project_iam_member" "datadog_gcp" {
  for_each = local.datadog_gcp_enabled ? local.datadog_gcp_roles : toset([])
  project  = var.cluster_project
  role     = each.value
  member   = "serviceAccount:${google_service_account.datadog_gcp["integration"].email}"
}

resource "datadog_integration_gcp_sts" "staging" {
  for_each     = local.datadog_gcp
  client_email = google_service_account.datadog_gcp[each.key].email

  account_tags = ["env:${local.dd_env}"]
  automute     = false

  # Project-scoped: no org/folder scans, no resource-change or security
  # collection, nothing outside the cluster project.
  resource_collection_enabled               = false
  is_org_folder_resource_collection_enabled = false
  is_resource_change_collection_enabled     = false
  is_cspm_enabled                           = false
  is_security_command_center_enabled        = false
  is_global_location_enabled                = var.datadog_gcp_global_location_enabled

  # Only the selected Cloud Run services' revisions and this cluster's nodes
  # are imported; other Cloud Run services and VMs in the project are not.
  monitored_resource_configs = concat([
    { type = "cloud_run_revision", filters = [for service in sort(tolist(var.datadog_gcp_cloud_run_services)) : "service_name:${service}"] },
    { type = "gce_instance", filters = ["goog-k8s-cluster-name:${var.cluster_name}"] },
    ], [for namespace, config in var.datadog_gcp_dependency_collection : {
      type    = local.dependency_resource_types[namespace]
      filters = sort(tolist(config.verified_resource_filters))
  }])

  metric_namespace_configs = local.datadog_gcp_metric_namespace_configs

  depends_on = [google_project_iam_member.datadog_gcp]
}

# Datadog's delegate may mint tokens for this one service account only.
resource "google_service_account_iam_member" "datadog_gcp_delegate" {
  for_each           = local.datadog_gcp
  service_account_id = google_service_account.datadog_gcp[each.key].name
  role               = "roles/iam.serviceAccountTokenCreator"
  member             = "serviceAccount:${datadog_integration_gcp_sts.staging[each.key].delegate_account_email}"
}
