# All identifiers below are test-only, not approved provisioning inputs.
mock_provider "google" {
  mock_resource "google_compute_network" {
    defaults = { id = "projects/test-rust-stg-only/global/networks/rust-private" }
  }
  mock_resource "google_project" {
    defaults = { number = "123456789012" }
  }
  mock_resource "google_service_account" {
    defaults = {
      name  = "projects/test-rust-stg-only/serviceAccounts/test-account@test-rust-stg-only.iam.gserviceaccount.com"
      email = "test-account@test-rust-stg-only.iam.gserviceaccount.com"
    }
  }
}
mock_provider "google-beta" {}

variables {
  environment = "stg"
  projects = {
    dev = { project_id = "test-rust-dev-only", state_bucket_name = "test-rust-dev-state-only" }
    stg = { project_id = "test-rust-stg-only", state_bucket_name = "test-rust-stg-state-only" }
    prd = { project_id = "test-rust-prd-only", state_bucket_name = "test-rust-prd-state-only" }
  }
  organization_id      = "123456789"
  billing_account      = "000000-000000-000000"
  iap_group            = "test-team@example.invalid"
  state_admin_members  = ["group:test-infra@example.invalid"]
  github_repository_id = "100000001"
  github_owner_id      = "100000002"
}

run "foundations_only" {
  command = plan
  assert {
    condition     = length(google_cloud_run_v2_service.app) == 0 && length(google_cloud_run_v2_job.migration) == 0 && length(google_sql_database_instance.app) == 0
    error_message = "All billable workloads and databases must default off."
  }
  assert {
    condition     = google_project.environment.project_id == "test-rust-stg-only" && google_storage_bucket.state.name == "test-rust-stg-state-only"
    error_message = "Staging must use its own project and state bucket."
  }
  assert {
    condition     = length(google_service_account.app) == 4 && length(google_secret_manager_secret.doppler) == 4
    error_message = "Each product/role must have a distinct identity and bootstrap secret."
  }
  assert {
    condition     = strcontains(local.oidc_condition, "assertion.environment == 'staging'") && strcontains(local.oidc_condition, "assertion.repository_id == '100000001'") && strcontains(local.oidc_condition, "assertion.repository_owner_id == '100000002'") && strcontains(local.oidc_condition, "assertion.ref == 'refs/heads/main'") && strcontains(local.oidc_condition, "assertion.event_name == 'push'")
    error_message = "Staging OIDC must check immutable owner/repo IDs, main ref and push environment."
  }
  assert {
    condition     = length(google_service_account_iam_member.release_act_as) == 0
    error_message = "Inactive workloads must not grant deployer actAs."
  }
  assert {
    condition     = length(google_project_iam_member.sql_login) == 4 && length(google_project_iam_member.sql_connect) == 4
    error_message = "Administrator bootstrap must install database IAM before database activation."
  }
  assert {
    condition = alltrue([for key, grant in google_project_iam_member.sql_connect :
      grant.condition[0].expression == (startswith(key, "physician-") ? "resource.name == 'projects/test-rust-stg-only/instances/rust-physician'" : "resource.name == 'projects/test-rust-stg-only/instances/rust-duet'")
    ])
    error_message = "Bootstrap database grants must remain scoped to the principal's product instance."
  }
}

run "single_environment_bootstrap" {
  command = plan
  variables {
    projects = {
      stg = { project_id = "test-rust-stg-only", state_bucket_name = "test-rust-stg-state-only" }
    }
  }
  assert {
    condition     = google_project.environment.project_id == "test-rust-stg-only" && length(google_sql_database_instance.app) == 0 && length(google_cloud_run_v2_service.app) == 0
    error_message = "Staging bootstrap must not require other project IDs or activate databases/services."
  }
}

run "reject_duplicate_projects" {
  command = plan
  variables {
    projects = {
      dev = { project_id = "test-rust-same-only", state_bucket_name = "test-rust-dev-state-only" }
      stg = { project_id = "test-rust-same-only", state_bucket_name = "test-rust-stg-state-only" }
      prd = { project_id = "test-rust-prd-only", state_bucket_name = "test-rust-prd-state-only" }
    }
  }
  expect_failures = [var.projects]
}

run "reject_prod_activation" {
  command = plan
  variables {
    environment      = "prd"
    enable_databases = true
  }
  expect_failures = [google_project.environment]
}

run "production_identity" {
  command = plan
  variables { environment = "prd" }
  assert {
    condition     = google_project.environment.project_id == "test-rust-prd-only" && google_storage_bucket.state.name == "test-rust-prd-state-only" && local.github_subject == "repo:Coda-Health@100000002/coda-rust@100000001:environment:production" && strcontains(local.oidc_condition, "assertion.event_name == 'workflow_dispatch'") && !strcontains(local.oidc_condition, "'staging'")
    error_message = "Production must never admit staging identity or automatic push releases."
  }
}

run "private_product_databases" {
  command = plan
  variables { enable_databases = true }
  assert {
    condition     = length(google_sql_database_instance.app) == 2 && length(google_sql_user.app) == 4
    error_message = "Products require distinct SQL instances and runtime/migration principals."
  }
  assert {
    condition     = alltrue([for db in google_sql_database_instance.app : !db.settings[0].ip_configuration[0].ipv4_enabled && db.settings[0].backup_configuration[0].enabled && db.settings[0].backup_configuration[0].point_in_time_recovery_enabled && db.settings[0].disk_autoresize_limit == 50])
    error_message = "SQL must be private, backed up, PITR-enabled and capped."
  }
}

run "reject_unapproved_runtime" {
  command = plan
  variables {
    enable_workloads = true
  }
  expect_failures = [google_project.environment]
}

run "active_staging_contract" {
  command = plan
  variables {
    enable_databases                       = true
    enable_workloads                       = true
    runtime_and_database_contract_approved = true
    images = {
      physician = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-migrate@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
      }
      duet = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/duet@sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/duet-migrate@sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
      }
    }
    doppler_configs = {
      physician-runtime   = "stg_physician_runtime"
      physician-migration = "stg_physician_migration"
      duet-runtime        = "stg_duet_runtime"
      duet-migration      = "stg_duet_migration"
    }
    doppler_token_versions = {
      physician = { runtime = "1", migration = "2" }
      duet      = { runtime = "3", migration = "4" }
    }
    runtime_environment = {
      physician = { TEST_CONTRACT_ONLY = "true" }
      duet      = { TEST_CONTRACT_ONLY = "true" }
    }
    migration_environment = {
      physician = { PHYSICIAN_STAGING_ACCOUNTS = "doc@example.invalid|Doc|admin" }
    }
  }
  assert {
    condition     = contains([for env in google_cloud_run_v2_job.migration["physician"].template[0].template[0].containers[0].env : "${env.name}=${coalesce(env.value, "<secret>")}"], "PHYSICIAN_STAGING_ACCOUNTS=doc@example.invalid|Doc|admin") && !contains([for env in google_cloud_run_v2_job.migration["duet"].template[0].template[0].containers[0].env : env.name], "PHYSICIAN_STAGING_ACCOUNTS")
    error_message = "Migration settings reach only their own app's job."
  }
  assert {
    condition     = alltrue([for service in google_cloud_run_v2_service.app : service.iap_enabled && !service.invoker_iam_disabled && service.template[0].scaling[0].min_instance_count == 0 && service.template[0].scaling[0].max_instance_count == 2 && service.template[0].vpc_access[0].egress == "PRIVATE_RANGES_ONLY"])
    error_message = "Every service requires direct IAP, IAM checks, scale caps and private VPC access."
  }
  assert {
    condition     = alltrue([for binding in google_iap_web_cloud_run_service_iam_binding.team : binding.members == toset(["group:test-team@example.invalid"])])
    error_message = "Only the approved group can pass IAP."
  }
  assert {
    condition     = alltrue([for job in google_cloud_run_v2_job.migration : job.template[0].parallelism == 1 && job.template[0].task_count == 1 && job.template[0].template[0].max_retries == 0])
    error_message = "Migration jobs must be single-task without automatic retries."
  }
  assert {
    condition     = !contains(google_project_iam_custom_role.release_service.permissions, "run.services.setIamPolicy") && !contains(google_project_iam_custom_role.release_service.permissions, "run.services.create") && !contains(google_project_iam_custom_role.release_job.permissions, "run.jobs.runWithOverrides")
    error_message = "Release roles must not create resources, change IAM or override migration execution."
  }
}

run "reject_migration_bootstrap_override" {
  command = plan
  variables {
    enable_databases                       = true
    enable_workloads                       = true
    runtime_and_database_contract_approved = true
    images = {
      physician = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-migrate@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
      }
      duet = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/duet@sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/duet-migrate@sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
      }
    }
    doppler_configs = {
      physician-runtime   = "stg_physician_runtime"
      physician-migration = "stg_physician_migration"
      duet-runtime        = "stg_duet_runtime"
      duet-migration      = "stg_duet_migration"
    }
    doppler_token_versions = {
      physician = { runtime = "1", migration = "2" }
      duet      = { runtime = "3", migration = "4" }
    }
    runtime_environment = {
      physician = { TEST_CONTRACT_ONLY = "true" }
      duet      = { TEST_CONTRACT_ONLY = "true" }
    }
    migration_environment = {
      physician = { DOPPLER_CONFIG = "stg_physician_runtime" }
    }
  }
  expect_failures = [google_project.environment]
}
