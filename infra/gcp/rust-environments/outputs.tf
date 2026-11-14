output "backend_configuration" {
  description = "Operator-only backend config AFTER authorized bootstrap; migrate local state separately per environment. Do not give deployer bucket access."
  value = {
    bucket = google_storage_bucket.state.name
    prefix = "rust-environments/${var.environment}"
  }
}

output "release_contract" {
  description = "Deployment interface. Only main push may enter staging; dev/prod require workflow_dispatch plus protected environment approval. Updates must preserve IAP/env/identity/network/caps; IAM cannot field-scope run.services.update."
  value = {
    project_id                          = google_project.environment.project_id
    region                              = var.region
    github_environment                  = local.github_environment
    oidc_provider                       = google_iam_workload_identity_pool_provider.github.name
    deployer_service_account            = google_service_account.deployer.email
    artifact_repository                 = "${var.region}-docker.pkg.dev/${local.project_id}/apps"
    workloads_enabled                   = var.enable_workloads
    physician_smtp_relay_egress_enabled = var.enable_physician_smtp_relay
    services                            = { for app, service in google_cloud_run_v2_service.app : app => service.name }
    migration_jobs                      = { for app, job in google_cloud_run_v2_job.migration : app => job.name }
    urls                                = { for app, service in google_cloud_run_v2_service.app : app => service.uri }
    release_managed_fields              = ["service image digest", "service revision name", "service traffic and tags", "job image digest", "client metadata"]
    infrastructure_owned_fields         = ["IAP", "IAM", "service accounts", "network", "env and secret pins", "caps", "probes"]
    readiness_path                      = "/readyz"
    smoke_test_identity                 = "No deployer IAP access granted. Operator must separately authorize a dedicated in-org test identity and OAuth audience for authenticated smoke tests."
  }
}

output "database_contract" {
  description = "No passwords. Operator installs SQL grants; owners implement private TLS + IAM token refresh (a mounted Cloud SQL socket alone does not enable automatic IAM authentication). Supply app-specific accepted DSN/config through scoped Doppler config."
  value = { for app, instance in google_sql_database_instance.app : app => {
    database                 = google_sql_database.app[app].name
    private_ip               = instance.private_ip_address
    connection_name          = instance.connection_name
    runtime_principal        = google_sql_user.app["${app}-runtime"].name
    migration_principal      = google_sql_user.app["${app}-migration"].name
    required_pool_max        = 4
    max_database_connections = 100
    migration_schema         = "public"
    operator_grant_script    = "bootstrap-grants.sql"
  } }
}

output "doppler_bootstrap_secrets" {
  description = "Operator supplies four read-only least-scope Doppler bootstrap token versions OUTSIDE Terraform. No application keys belong in Secret Manager."
  value       = { for key, secret in google_secret_manager_secret.doppler : key => secret.id }
}

output "activation_blockers" {
  value = [
    "Operator verifies project IDs, organization/folder/billing association authority, separate state access, region and in-organization IAP group. Billing administrators manage budget alerts outside this engineering state.",
    "Operator creates protected GitHub dev/staging/production environments; production requires independent reviewers. No CI infrastructure credentials.",
    "Operator provisions foundations, publishes immutable images, and separately enables databases (two continuously billed instances).",
    "Operator bootstraps per-app SQL grants; runtime owners prove IAM token refresh/private TLS, durable auth/chat, no auto-migrate, bounded pool <=4 and enforced model usage limits/provider quotas.",
    "Operator creates least-scope per-app/per-role Doppler tokens and pins numeric Secret Manager versions; no secret values in plans. `doppler configs tokens create --plain` ends with a newline that breaks the runtime's Authorization header: pipe through `tr -d '\\r\\n'` before `gcloud secrets versions add --data-file=-` (public-site staging bootstrap, PR #42).",
    "Runtime env names and migration/image interfaces require owner agreement; /readyz and port 8080 must work before activation.",
    "Only then may an authorized operator enable workloads; prod additionally requires production_activation_approved. Hosted acceptance remains a later phase.",
    "Budget alerts are not hard caps; logs retained 30 days, backups/PITR seven days, state history 90 days/ten versions, rollback images retained until release keep-set policy exists.",
  ]
}

output "physician_scale" {
  description = "Nonsecret physician scaling endpoints (null while enable_physician_scale is off or ineffective). The operator builds PHYSICIAN_REDIS_URL=rediss://:AUTH@host:port for Doppler stg_physician_runtime from this and physician_redis_auth_string."
  value = local.physician_scale ? {
    redis_host            = google_redis_instance.physician["physician"].host
    redis_port            = google_redis_instance.physician["physician"].port
    worker_service        = google_cloud_run_v2_service.physician_worker["physician"].name
    worker_url            = google_cloud_run_v2_service.physician_worker["physician"].uri
    worker_identity       = google_service_account.physician_worker["physician"].email
    worker_doppler_config = try(var.doppler_configs["physician-runtime"], "")
  } : null
}

output "physician_redis_ca_pem" {
  description = "Memorystore server CA bundle (public certificates, not secret) for PHYSICIAN_REDIS_CA_PEM. Read with `tofu output -raw physician_redis_ca_pem`."
  value       = local.physician_scale ? join("", [for ca in google_redis_instance.physician["physician"].server_ca_certs : ca.cert]) : null
}

output "physician_redis_auth_string" {
  description = "Memorystore AUTH string. Sensitive: read only with `tofu output -raw physician_redis_auth_string` straight into Doppler; never paste it into a plan review, log or commit."
  value       = local.physician_scale ? google_redis_instance.physician["physician"].auth_string : null
  sensitive   = true
}

output "physician_clinical_library" {
  description = "Read-only clinical-evidence service wiring for physician and physician-worker, with the exact staging runtime principals to admit in infra/gcp/clinical-evidence's clinical_evidence_invokers."
  value = var.physician_clinical_library_url != null && var.enable_workloads ? {
    url      = var.physician_clinical_library_url
    audience = coalesce(var.physician_clinical_library_audience, var.physician_clinical_library_url)
    caller_principals = compact([
      "serviceAccount:${google_service_account.app["physician-runtime"].email}",
      local.physician_scale ? "serviceAccount:${google_service_account.physician_worker["physician"].email}" : "",
    ])
  } : null
}
