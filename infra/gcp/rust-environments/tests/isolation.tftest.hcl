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

# This service-agent email is a plan-time input to the invoker assertion.
override_resource {
  target = google_project_service_identity.iap
  values = {
    email = "service-123456789012@gcp-sa-iap.iam.gserviceaccount.com"
  }
}

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
  database_passwords = {
    physician-runtime   = "synthetic-physician-runtime-password"
    physician-migration = "synthetic-physician-migration-password"
    duet-runtime        = "synthetic-duet-runtime-password"
    duet-migration      = "synthetic-duet-migration-password"
  }
}

run "foundations_need_no_database_passwords" {
  command = plan
  variables { database_passwords = {} }
  assert {
    condition     = length(google_sql_user.app) == 0
    error_message = "Foundations must not require database credentials or create SQL users."
  }
}

run "reject_missing_database_passwords" {
  command = plan
  variables {
    enable_databases   = true
    database_passwords = {}
  }
  expect_failures = [var.database_passwords]
}

run "reject_partial_database_passwords" {
  command = plan
  variables {
    enable_databases   = true
    database_passwords = { physician-runtime = "synthetic-test-password" }
  }
  expect_failures = [var.database_passwords]
}

run "reject_blank_database_password" {
  command = plan
  variables {
    enable_databases = true
    database_passwords = {
      physician-runtime   = "synthetic-physician-runtime-password"
      physician-migration = "synthetic-physician-migration-password"
      duet-runtime        = "   "
      duet-migration      = "synthetic-duet-migration-password"
    }
  }
  expect_failures = [var.database_passwords]
}

run "reject_known_bootstrap_password" {
  command = plan
  variables {
    enable_databases = true
    database_passwords = {
      physician-runtime   = "synthetic-physician-runtime-password"
      physician-migration = "bootstrap-replaced-out-of-band"
      duet-runtime        = "synthetic-duet-runtime-password"
      duet-migration      = "synthetic-duet-migration-password"
    }
  }
  expect_failures = [var.database_passwords]
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

run "reject_public_proxy_invoker" {
  command = plan
  variables { physician_proxy_invokers = ["allUsers"] }
  expect_failures = [var.physician_proxy_invokers]
}

run "reject_group_proxy_invoker" {
  command = plan
  variables { physician_proxy_invokers = ["group:test-team@example.invalid"] }
  expect_failures = [var.physician_proxy_invokers]
}

run "production_identity" {
  command = plan
  assert {
    condition     = length(var.physician_proxy_invokers) == 0
    error_message = "Production must not inherit staging's cross-project proxy grant."
  }
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
  assert {
    condition     = alltrue([for db in google_sql_database_instance.app : db.settings[0].backup_configuration[0].transaction_log_retention_days == 7 && db.settings[0].backup_configuration[0].backup_retention_settings[0].retained_backups == 7 && db.deletion_protection])
    error_message = "Recovery needs seven retained backups, seven days of PITR logs and protected source instances."
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
    physician_proxy_invokers               = ["serviceAccount:public-site-staging@test-public-staging.iam.gserviceaccount.com"]
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
    condition     = google_iap_web_cloud_run_service_iam_binding.team["physician"].members == toset(["group:test-team@example.invalid", "serviceAccount:public-site-staging@test-public-staging.iam.gserviceaccount.com"]) && google_iap_web_cloud_run_service_iam_binding.team["duet"].members == toset(["group:test-team@example.invalid"])
    error_message = "Only physician may include the explicit proxy allowlist; Duet remains group-only."
  }
  assert {
    condition     = google_iap_web_cloud_run_service_iam_binding.team["physician"].cloud_run_service_name == "physician" && google_iap_web_cloud_run_service_iam_binding.team["physician"].project == "test-rust-stg-only" && google_iap_web_cloud_run_service_iam_binding.team["physician"].role == "roles/iap.httpsResourceAccessor" && alltrue([for binding in google_cloud_run_v2_service_iam_binding.iap_invoker : !contains(binding.members, "serviceAccount:public-site-staging@test-public-staging.iam.gserviceaccount.com")])
    error_message = "Proxy access is physician-service IAP access, never direct Cloud Run invocation or a project grant."
  }
  assert {
    condition     = alltrue([for job in google_cloud_run_v2_job.migration : job.template[0].parallelism == 1 && job.template[0].task_count == 1 && job.template[0].template[0].max_retries == 0])
    error_message = "Migration jobs must be single-task without automatic retries."
  }
  assert {
    condition     = alltrue([for service in google_cloud_run_v2_service.app : service.template[0].timeout == "300s" && service.template[0].max_instance_request_concurrency == 8 && service.template[0].containers[0].resources[0].limits == tomap({ cpu = "1", memory = "1Gi" })])
    error_message = "Runtime request concurrency, duration and per-instance resources must remain bounded."
  }
  assert {
    condition     = alltrue([for job in google_cloud_run_v2_job.migration : job.template[0].template[0].timeout == "600s" && job.template[0].template[0].containers[0].resources[0].limits == tomap({ cpu = "1", memory = "1Gi" })])
    error_message = "Migration execution time and per-task resources must remain bounded."
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

run "physician_relay_routing" {
  command = plan
  variables {
    enable_physician_smtp_relay            = true
    physician_proxy_invokers               = ["serviceAccount:public-site-staging@test-public-staging.iam.gserviceaccount.com"]
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
    condition     = google_cloud_run_v2_service.app["physician"].template[0].vpc_access[0].egress == "ALL_TRAFFIC" && google_cloud_run_v2_service.app["duet"].template[0].vpc_access[0].egress == "PRIVATE_RANGES_ONLY" && alltrue([for job in google_cloud_run_v2_job.migration : job.template[0].template[0].vpc_access[0].egress == "PRIVATE_RANGES_ONLY"])
    error_message = "Only physician runtime uses static public egress; Duet and migration routes remain private-only."
  }
  assert {
    condition     = length(google_compute_address.physician_mail) == 1 && google_compute_router_nat.physician_mail[0].nat_ip_allocate_option == "MANUAL_ONLY" && google_compute_router_nat.physician_mail[0].source_subnetwork_ip_ranges_to_nat == "LIST_OF_SUBNETWORKS"
    error_message = "SMTP relay requires one explicit public IP, limited to the runtime subnet."
  }
}

run "physician_scale_gate_off" {
  # Active workloads without the scaling gate: no Redis, Pub/Sub or worker.
  # The physician site still keeps CPU between requests for the Postgres
  # authority listener. Duet stays request-based.
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
  }
  assert {
    condition     = length(google_redis_instance.physician) == 0 && length(google_pubsub_topic.chat_turns) == 0 && length(google_pubsub_topic.chat_turns_dead_letter) == 0 && length(google_pubsub_subscription.chat_turns) == 0 && length(google_pubsub_subscription.chat_turns_dead_letter) == 0
    error_message = "Without enable_physician_scale there is no Redis, topic or subscription."
  }
  assert {
    condition     = length(google_cloud_run_v2_service.physician_worker) == 0 && length(google_service_account.physician_worker) == 0 && length(google_service_account.physician_pubsub_push) == 0 && length(google_project_iam_member.physician_worker_sql) == 0 && length(google_project_service_identity.pubsub) == 0
    error_message = "Without enable_physician_scale there is no worker, push identity or worker grant."
  }
  assert {
    condition     = !contains([for env in google_cloud_run_v2_service.app["physician"].template[0].containers[0].env : env.name], "PHYSICIAN_CHAT_TOPIC") && length(google_secret_manager_secret_iam_binding.bootstrap_reader["physician-runtime"].members) == 1
    error_message = "Without the gate the site runs turns inline and only the site reads its Doppler slot."
  }
  assert {
    condition     = output.physician_scale == null && output.physician_redis_ca_pem == null
    error_message = "Scaling outputs are null while the gate is off."
  }
  assert {
    condition     = !google_cloud_run_v2_service.app["physician"].template[0].containers[0].resources[0].cpu_idle && google_cloud_run_v2_service.app["duet"].template[0].containers[0].resources[0].cpu_idle
    error_message = "The physician site keeps CPU for its Postgres listener with the scaling gate off. Duet stays request-based."
  }
  assert {
    condition     = alltrue([for service in google_cloud_run_v2_service.app : service.template[0].scaling[0].min_instance_count == 0])
    error_message = "Without the gate there is no recovery sweep to keep alive, so every site scales to zero."
  }
}

run "physician_scale_ineffective_without_workloads" {
  command = plan
  variables { enable_physician_scale = true }
  assert {
    condition     = length(google_redis_instance.physician) == 0 && length(google_pubsub_topic.chat_turns) == 0 && length(google_cloud_run_v2_service.physician_worker) == 0
    error_message = "The scaling gate takes effect only with enable_workloads."
  }
}

run "physician_scale_requires_worker_image" {
  command = plan
  variables {
    enable_databases                       = true
    enable_workloads                       = true
    enable_physician_scale                 = true
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
  }
  expect_failures = [google_project.environment]
}

run "reject_runtime_override_of_scale_settings" {
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
      # Duet must never be pointed at the physician bus.
      duet = { PHYSICIAN_REDIS_URL = "rediss://example.invalid:6378" }
    }
  }
  expect_failures = [google_project.environment]
}

run "physician_scale_active" {
  command = plan
  # Distinct identities so membership assertions cannot pass by coincidence
  # (the provider mock gives every service account the same email).
  override_resource {
    target = google_service_account.physician_pubsub_push
    values = {
      name  = "projects/test-rust-stg-only/serviceAccounts/physician-pubsub-push@test-rust-stg-only.iam.gserviceaccount.com"
      email = "physician-pubsub-push@test-rust-stg-only.iam.gserviceaccount.com"
    }
  }
  override_resource {
    target = google_service_account.physician_worker
    values = {
      name  = "projects/test-rust-stg-only/serviceAccounts/physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com"
      email = "physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com"
    }
  }
  override_resource {
    target = google_project_service_identity.pubsub
    values = { email = "service-123456789012@gcp-sa-pubsub.iam.gserviceaccount.com" }
  }
  override_resource {
    target = google_project_service_identity.iap
    values = { email = "service-123456789012@gcp-sa-iap.iam.gserviceaccount.com" }
  }
  variables {
    enable_databases                       = true
    enable_workloads                       = true
    enable_physician_scale                 = true
    runtime_and_database_contract_approved = true
    images = {
      physician = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-migrate@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        worker    = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-worker@sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
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
  }

  # Memorystore: private, authenticated, TLS, on the existing peering.
  assert {
    condition = keys(google_redis_instance.physician) == ["physician"] && alltrue([for r in google_redis_instance.physician :
      r.name == "rust-physician-cache" && r.tier == "BASIC" && r.memory_size_gb == 1 && r.redis_version == "REDIS_7_2" &&
      r.connect_mode == "PRIVATE_SERVICE_ACCESS" && r.auth_enabled && r.transit_encryption_mode == "SERVER_AUTHENTICATION" &&
      r.authorized_network == "projects/test-rust-stg-only/global/networks/rust-private" && r.reserved_ip_range == "rust-sql-private" && r.region == "us-east1"
    ])
    error_message = "The physician cache is one Basic 1 GB Redis 7.2 with AUTH and TLS on the rust-sql-private peering."
  }

  # Queue shape.
  assert {
    condition = google_pubsub_topic.chat_turns["physician"].name == "physician-chat-turns" && google_pubsub_topic.chat_turns_dead_letter["physician"].name == "physician-chat-turns-dead-letter" && alltrue([for s in google_pubsub_subscription.chat_turns :
      s.ack_deadline_seconds == 600 && s.retry_policy[0].minimum_backoff == "10s" && s.retry_policy[0].maximum_backoff == "600s" && s.dead_letter_policy[0].max_delivery_attempts == 5 &&
      s.push_config[0].oidc_token[0].service_account_email == "physician-pubsub-push@test-rust-stg-only.iam.gserviceaccount.com" &&
      s.push_config[0].oidc_token[0].audience == "https://physician-worker-123456789012.us-east1.run.app"
    ])
    error_message = "Chat turns push to the worker with the push identity's OIDC token, 600 s deadline, 10-600 s backoff and a DLQ after 5 attempts."
  }

  # Pub/Sub IAM: runtime publishes only to chat turns; the service agent
  # dead-letters and mints push tokens for the push identity only.
  assert {
    condition     = google_pubsub_topic_iam_binding.chat_turns_publisher["physician"].members == toset(["serviceAccount:test-account@test-rust-stg-only.iam.gserviceaccount.com"]) && google_pubsub_topic_iam_binding.chat_turns_publisher["physician"].role == "roles/pubsub.publisher"
    error_message = "Only the physician runtime publishes chat turns."
  }
  assert {
    condition = (
      google_pubsub_topic_iam_binding.chat_turns_dead_letter_publisher["physician"].members == toset(["serviceAccount:service-123456789012@gcp-sa-pubsub.iam.gserviceaccount.com"]) &&
      google_pubsub_subscription_iam_binding.chat_turns_dead_letter_subscriber["physician"].members == toset(["serviceAccount:service-123456789012@gcp-sa-pubsub.iam.gserviceaccount.com"]) &&
      google_pubsub_subscription_iam_binding.chat_turns_dead_letter_subscriber["physician"].role == "roles/pubsub.subscriber" &&
      google_service_account_iam_member.pubsub_push_token["physician"].member == "serviceAccount:service-123456789012@gcp-sa-pubsub.iam.gserviceaccount.com" &&
      google_service_account_iam_member.pubsub_push_token["physician"].role == "roles/iam.serviceAccountTokenCreator" &&
      google_service_account_iam_member.pubsub_push_token["physician"].service_account_id == "projects/test-rust-stg-only/serviceAccounts/physician-pubsub-push@test-rust-stg-only.iam.gserviceaccount.com"
    )
    error_message = "The Pub/Sub service agent may dead-letter and mint tokens only for the push identity."
  }

  # Worker: internal ingress, no IAP, IAM checks on, push identity only.
  assert {
    condition = keys(google_cloud_run_v2_service.physician_worker) == ["physician"] && alltrue([for w in google_cloud_run_v2_service.physician_worker :
      w.ingress == "INGRESS_TRAFFIC_INTERNAL_ONLY" && !w.iap_enabled && !w.invoker_iam_disabled &&
      w.template[0].service_account == "physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com" &&
      w.template[0].vpc_access[0].egress == "PRIVATE_RANGES_ONLY" && w.template[0].scaling[0].min_instance_count == 0 && w.template[0].scaling[0].max_instance_count == 2 &&
      w.template[0].timeout == "600s" && w.template[0].max_instance_request_concurrency == 4 &&
      w.template[0].containers[0].resources[0].limits == tomap({ cpu = "1", memory = "1Gi" }) &&
      w.template[0].containers[0].resources[0].cpu_idle &&
      w.template[0].containers[0].image == var.images["physician"].worker
    ])
    error_message = "The worker is internal-only, IAM-checked, private-egress, scale-capped and runs the recorded worker digest."
  }
  assert {
    condition     = google_cloud_run_v2_service_iam_binding.physician_worker_invoker["physician"].role == "roles/run.invoker" && google_cloud_run_v2_service_iam_binding.physician_worker_invoker["physician"].members == toset(["serviceAccount:physician-pubsub-push@test-rust-stg-only.iam.gserviceaccount.com"])
    error_message = "The push identity is the only invoker of the worker."
  }
  assert {
    condition = (
      contains([for env in google_cloud_run_v2_service.physician_worker["physician"].template[0].containers[0].env : "${env.name}=${coalesce(env.value, "<secret>")}"], "PHYSICIAN_WORKER_AUDIENCE=https://physician-worker-123456789012.us-east1.run.app") &&
      contains([for env in google_cloud_run_v2_service.physician_worker["physician"].template[0].containers[0].env : "${env.name}=${coalesce(env.value, "<secret>")}"], "PHYSICIAN_PUSH_SERVICE_ACCOUNT=physician-pubsub-push@test-rust-stg-only.iam.gserviceaccount.com") &&
      contains([for env in google_cloud_run_v2_service.physician_worker["physician"].template[0].containers[0].env : "${env.name}=${coalesce(env.value, "<secret>")}"], "DOPPLER_CONFIG=stg_physician_runtime") &&
      google_cloud_run_v2_service.physician_worker["physician"].custom_audiences == tolist(["https://physician-worker-123456789012.us-east1.run.app"])
    )
    error_message = "The worker verifies its own audience and the push identity, and runs on the physician runtime Doppler config."
  }
  assert {
    condition = (
      google_secret_manager_secret_iam_binding.bootstrap_reader["physician-runtime"].members == toset(["serviceAccount:test-account@test-rust-stg-only.iam.gserviceaccount.com", "serviceAccount:physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com"]) &&
      alltrue([for key in ["physician-migration", "duet-runtime", "duet-migration"] : !contains(google_secret_manager_secret_iam_binding.bootstrap_reader[key].members, "serviceAccount:physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com")]) &&
      alltrue([for grant in google_project_iam_member.physician_worker_sql : grant.condition[0].expression == "resource.name == 'projects/test-rust-stg-only/instances/rust-physician'"])
    )
    error_message = "The worker reads only the physician-runtime Doppler slot and reaches only the physician SQL instance."
  }

  # Site: physician keeps CPU between requests and learns the topic; Duet is unchanged.
  assert {
    condition     = !google_cloud_run_v2_service.app["physician"].template[0].containers[0].resources[0].cpu_idle && google_cloud_run_v2_service.app["duet"].template[0].containers[0].resources[0].cpu_idle
    error_message = "Only the physician site keeps CPU allocated between requests."
  }
  assert {
    condition     = google_cloud_run_v2_service.app["physician"].template[0].scaling[0].min_instance_count == 1 && google_cloud_run_v2_service.app["physician"].template[0].scaling[0].max_instance_count == 2 && google_cloud_run_v2_service.app["duet"].template[0].scaling[0].min_instance_count == 0
    error_message = "One physician site instance stays up to run the recovery sweep; Duet still scales to zero."
  }
  assert {
    condition     = contains([for env in google_cloud_run_v2_service.app["physician"].template[0].containers[0].env : env.name], "PHYSICIAN_CHAT_TOPIC")
    error_message = "The physician site publishes chat turns to the topic."
  }
  assert {
    condition     = length([for env in google_cloud_run_v2_service.app["duet"].template[0].containers[0].env : env.name if startswith(env.name, "PHYSICIAN_") || strcontains(lower(env.name), "redis") || strcontains(lower(env.name), "pubsub")]) == 0
    error_message = "Duet gets no Redis, Pub/Sub or physician settings."
  }
  assert {
    # Every scaling resource is keyed "physician"; none exists for Duet. (The
    # mock gives both runtime identities one email, so isolation is proven by
    # the keys and by the publisher binding's single member above.)
    condition = alltrue([for keyset in [
      keys(google_redis_instance.physician), keys(google_pubsub_topic.chat_turns), keys(google_pubsub_topic.chat_turns_dead_letter),
      keys(google_pubsub_subscription.chat_turns), keys(google_pubsub_subscription.chat_turns_dead_letter),
      keys(google_pubsub_topic_iam_binding.chat_turns_publisher), keys(google_cloud_run_v2_service.physician_worker),
      keys(google_cloud_run_v2_service_iam_binding.physician_worker_invoker), keys(google_service_account.physician_worker),
    ] : keyset == ["physician"]]) && length(google_pubsub_topic_iam_binding.chat_turns_publisher["physician"].members) == 1
    error_message = "Redis, Pub/Sub and the worker exist only for physician; Duet holds no grant on them."
  }
  assert {
    condition     = alltrue([for b in google_cloud_run_v2_service_iam_binding.iap_invoker : b.members == toset(["serviceAccount:service-123456789012@gcp-sa-iap.iam.gserviceaccount.com"])])
    error_message = "The IAP agent stays the only invoker of the sites."
  }
  assert {
    condition     = alltrue([for service in google_cloud_run_v2_service.app : service.iap_enabled && !service.invoker_iam_disabled && service.ingress == "INGRESS_TRAFFIC_ALL"])
    error_message = "The sites keep direct IAP."
  }
}

# --- Observability (observability-cluster.tf and the workloads.tf sidecars) --

run "observability_default_off" {
  # Active staging with physician scaling, both observability gates at their
  # defaults: no cluster, no sidecar, one unnamed container per service.
  command = plan
  variables {
    enable_databases                       = true
    enable_workloads                       = true
    enable_physician_scale                 = true
    runtime_and_database_contract_approved = true
    images = {
      physician = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-migrate@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        worker    = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-worker@sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
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
  }
  assert {
    condition = (
      length(google_container_cluster.observability) == 0 && length(google_compute_subnetwork.observability) == 0 &&
      length(google_compute_address.otel_gateway) == 0 && length(google_compute_router_nat.observability) == 0 &&
      !contains(keys(google_project_service.api), "container.googleapis.com")
    )
    error_message = "With the gates off there is no observability cluster, subnet, gateway address, NAT or GKE API."
  }
  assert {
    condition = alltrue([for containers in concat(
      [for service in google_cloud_run_v2_service.app : service.template[0].containers],
      [for worker in google_cloud_run_v2_service.physician_worker : worker.template[0].containers],
    ) : length(containers) == 1 && containers[0].name == null])
    error_message = "With observability off, physician, duet and physician-worker each run one unnamed container, as before."
  }
  assert {
    condition = alltrue([for containers in concat(
      [for service in google_cloud_run_v2_service.app : service.template[0].containers],
      [for worker in google_cloud_run_v2_service.physician_worker : worker.template[0].containers],
      ) : (
      contains([for env in containers[0].env : "${env.name}=${coalesce(env.value, "<secret>")}"], "DD_ENV=staging") &&
      length([for env in containers[0].env : env.name if contains(["DD_VERSION", "OTEL_EXPORTER_OTLP_ENDPOINT", "CODA_LOG_FORMAT", "CODA_TELEMETRY_CONTENT", "CODA_TELEMETRY_BACKEND"], env.name)]) == 0
    )])
    error_message = "DD_ENV=staging is always set; DD_VERSION never, and the telemetry settings only while observability is on."
  }
  assert {
    condition     = output.observability_cluster == null && output.otel_gateway == null && output.otel_gateway_endpoint == null
    error_message = "Observability outputs are null while the cluster gate is off."
  }
}

run "reject_observability_without_cluster" {
  command = plan
  variables { observability_enabled = true }
  expect_failures = [google_project.environment]
}

run "reject_runtime_override_of_telemetry_env" {
  command = plan
  variables {
    runtime_environment = { physician = { DD_VERSION = "pretend" } }
  }
  expect_failures = [google_project.environment]
}

run "reject_runtime_override_of_telemetry_backend" {
  command = plan
  variables {
    runtime_environment = { duet = { CODA_TELEMETRY_BACKEND = "grafana" } }
  }
  expect_failures = [google_project.environment]
}

run "reject_unknown_telemetry_backend" {
  command = plan
  variables { telemetry_backend = "unknown" }
  expect_failures = [var.telemetry_backend]
}

run "reject_production_observability_cluster" {
  command = plan
  variables {
    environment                   = "prd"
    observability_cluster_enabled = true
  }
  expect_failures = [var.observability_cluster_enabled]
}

run "reject_unpinned_sidecar_image" {
  command = plan
  variables { otel_sidecar_image = "otel/opentelemetry-collector-contrib:latest" }
  expect_failures = [var.otel_sidecar_image]
}

run "observability_cluster_only" {
  # The cluster can come up (and the gateway be deployed into it) before any
  # service sends telemetry.
  command = plan
  variables { observability_cluster_enabled = true }
  assert {
    condition = (
      length(google_container_cluster.observability) == 1 && length(google_compute_address.otel_gateway) == 1 &&
      contains(keys(google_project_service.api), "container.googleapis.com") &&
      length(google_cloud_run_v2_service.app) == 0
    )
    error_message = "The cluster gate alone creates the cluster, its gateway address and the GKE API, and no service."
  }
  assert {
    condition = (
      google_container_cluster.observability[0].private_cluster_config[0].enable_private_endpoint &&
      length(google_container_cluster.observability[0].master_authorized_networks_config) == 1 &&
      length(google_container_cluster.observability[0].master_authorized_networks_config[0].cidr_blocks) == 0
    )
    error_message = "GKE requires authorized networks with a private endpoint; none may be listed."
  }
  assert {
    condition     = output.otel_gateway.audience == "coda-otel-gateway-stg" && jsonencode(output.otel_gateway.source_ranges) == jsonencode(["10.80.0.0/24"])
    error_message = "The gateway audience defaults per environment and admits only the Cloud Run egress subnet."
  }
}

run "observability_active" {
  command = plan
  override_resource {
    target = google_compute_address.otel_gateway
    values = { address = "10.82.0.10" }
  }
  override_resource {
    target = google_service_account.physician_worker
    values = {
      name  = "projects/test-rust-stg-only/serviceAccounts/physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com"
      email = "physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com"
    }
  }
  variables {
    telemetry_backend                      = "both"
    availability_probe_image               = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/availability-probe@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    availability_watchdog_enabled          = true
    availability_watchdog_channels         = ["projects/test-rust-stg-only/notificationChannels/synthetic"]
    observability_cluster_enabled          = true
    observability_enabled                  = true
    enable_databases                       = true
    enable_workloads                       = true
    enable_physician_scale                 = true
    runtime_and_database_contract_approved = true
    images = {
      physician = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-migrate@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        worker    = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-worker@sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
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
  }

  assert {
    condition = (
      google_cloud_scheduler_job.availability["probe"].paused &&
      google_cloud_scheduler_job.availability["probe"].schedule == "* * * * *" &&
      length(google_cloud_scheduler_job.availability["probe"].retry_config) == 0 &&
      google_cloud_run_v2_job.availability["probe"].template[0].template[0].vpc_access[0].egress == "ALL_TRAFFIC" &&
      google_cloud_run_v2_job.availability["probe"].template[0].template[0].max_retries == 0
    )
    error_message = "Provisioning must leave scheduling paused; probes use the VPC and cannot multiply retries."
  }
  assert {
    condition = (
      google_project_iam_custom_role.availability_signer["probe"].permissions == toset(["iam.serviceAccounts.signJwt"]) &&
      google_project_iam_custom_role.availability_readback["probe"].permissions == toset(["monitoring.timeSeries.list"]) &&
      google_cloud_run_v2_service_iam_binding.availability_dispatch["probe"].members == toset(["serviceAccount:${google_service_account.availability_scheduler["probe"].email}"]) &&
      !google_cloud_run_v2_service.availability_dispatch["probe"].invoker_iam_disabled &&
      google_cloud_run_v2_service.availability_dispatch["probe"].template[0].containers[0].resources[0].cpu_idle &&
      google_storage_bucket.availability["probe"].public_access_prevention == "enforced" &&
      endswith(google_storage_bucket_iam_member.availability["probe"].condition[0].expression, "/objects/state.json'")
    )
    error_message = "Probe signing/readback and dispatch grants must remain minimal, authenticated and object-scoped."
  }
  assert {
    condition = (
      google_monitoring_alert_policy.availability_watchdog["probe"].enabled &&
      google_monitoring_metric_descriptor.availability_heartbeat["probe"].type == "custom.googleapis.com/coda/availability/heartbeat" &&
      google_monitoring_metric_descriptor.availability_heartbeat["probe"].metric_kind == "GAUGE" &&
      google_monitoring_metric_descriptor.availability_heartbeat["probe"].value_type == "INT64" &&
      google_monitoring_alert_policy.availability_watchdog["probe"].conditions[0].condition_absent[0].duration == "300s" &&
      toset(google_monitoring_alert_policy.availability_watchdog["probe"].alert_strategy[0].notification_prompts) == toset(["OPENED", "CLOSED"]) &&
      toset(google_monitoring_alert_policy.availability_watchdog["probe"].notification_channels) == toset(var.availability_watchdog_channels)
    )
    error_message = "Independent direct-heartbeat absence must preserve five-minute detection and recovery on the explicit channel."
  }

  # Cluster: private Autopilot on rust-private with the agreed ranges.
  assert {
    condition = (
      google_compute_subnetwork.observability[0].name == "rust-observability" &&
      google_compute_subnetwork.observability[0].ip_cidr_range == "10.82.0.0/22" &&
      google_compute_subnetwork.observability[0].network == "projects/test-rust-stg-only/global/networks/rust-private" &&
      { for range in google_compute_subnetwork.observability[0].secondary_ip_range : range.range_name => range.ip_cidr_range } == { "observability-pods" = "10.84.0.0/14", "observability-services" = "10.88.0.0/20" }
    )
    error_message = "rust-observability is 10.82.0.0/22 on rust-private with pods 10.84.0.0/14 and services 10.88.0.0/20."
  }
  assert {
    condition = alltrue([for cluster in google_container_cluster.observability :
      cluster.enable_autopilot && cluster.deletion_protection && cluster.network == "projects/test-rust-stg-only/global/networks/rust-private" &&
      cluster.private_cluster_config[0].enable_private_nodes && cluster.private_cluster_config[0].enable_private_endpoint &&
      cluster.private_cluster_config[0].master_ipv4_cidr_block == "172.16.80.0/28" &&
      cluster.control_plane_endpoints_config[0].dns_endpoint_config[0].allow_external_traffic &&
      cluster.ip_allocation_policy[0].cluster_secondary_range_name == "observability-pods" &&
      cluster.ip_allocation_policy[0].services_secondary_range_name == "observability-services" &&
      cluster.workload_identity_config[0].workload_pool == "test-rust-stg-only.svc.id.goog" &&
      cluster.release_channel[0].channel == "REGULAR"
    ]) && length(google_container_cluster.observability) == 1
    error_message = "The cluster is private Autopilot (private nodes and IP endpoint, DNS endpoint for operators) with Workload Identity, REGULAR channel and deletion protection."
  }
  assert {
    # Every range on rust-private, read from the resources themselves. Two
    # aligned blocks overlap exactly when one contains the other's base.
    condition = alltrue([for ranges in [[
      google_compute_subnetwork.run.ip_cidr_range,
      "${google_compute_global_address.sql.address}/${google_compute_global_address.sql.prefix_length}",
      google_compute_subnetwork.observability[0].ip_cidr_range,
      google_compute_subnetwork.observability[0].secondary_ip_range[0].ip_cidr_range,
      google_compute_subnetwork.observability[0].secondary_ip_range[1].ip_cidr_range,
      google_container_cluster.observability[0].private_cluster_config[0].master_ipv4_cidr_block,
      ]] : length(distinct(ranges)) == 6 && alltrue([for pair in setproduct(ranges, ranges) : pair[0] == pair[1] || (
        cidrhost(pair[0], 0) != cidrhost("${cidrhost(pair[1], 0)}/${split("/", pair[0])[1]}", 0) &&
        cidrhost(pair[1], 0) != cidrhost("${cidrhost(pair[0], 0)}/${split("/", pair[1])[1]}", 0)
    )])])
    error_message = "No rust-private range (run, SQL peering, observability nodes, pods, services, control plane) may overlap another."
  }
  assert {
    condition = (
      google_compute_address.otel_gateway[0].address_type == "INTERNAL" && google_compute_address.otel_gateway[0].name == "otel-gateway" &&
      [for subnet in google_compute_router_nat.observability[0].subnetwork : subnet.source_ip_ranges_to_nat] == [toset(["ALL_IP_RANGES"])] &&
      google_compute_router_nat.observability[0].source_subnetwork_ip_ranges_to_nat == "LIST_OF_SUBNETWORKS"
    )
    error_message = "The gateway has an internal address in the observability subnet, which alone gets the observability NAT."
  }
  assert {
    condition = (
      output.observability_cluster.name == "rust-observability" && output.observability_cluster.location == "us-east1" &&
      output.observability_cluster.project == "test-rust-stg-only" && output.observability_cluster.network == "rust-private" &&
      output.otel_gateway.ip == "10.82.0.10" && output.otel_gateway.endpoint == "http://10.82.0.10:4318" &&
      output.otel_gateway_endpoint == "http://10.82.0.10:4318" && output.otel_gateway.audience == "coda-otel-gateway-stg" &&
      jsonencode(output.otel_gateway.source_ranges) == jsonencode(["10.80.0.0/24"]) &&
      contains(output.otel_gateway.allowed_callers, "physician-worker-runtime@test-rust-stg-only.iam.gserviceaccount.com")
    )
    error_message = "The observability root gets the cluster, the gateway endpoint, audience, Cloud Run source range and caller identities."
  }

  # Sidecars: product at index 0 (named, ingress, no depends_on), collector at
  # index 1 on every observed service.
  assert {
    condition = alltrue([for name, containers in merge(
      { for app, service in google_cloud_run_v2_service.app : app => service.template[0].containers },
      { for _, worker in google_cloud_run_v2_service.physician_worker : "physician-worker" => worker.template[0].containers },
      ) : (
      length(containers) == 2 &&
      containers[0].name == name && containers[0].ports[0].container_port == 8080 && length(coalesce(containers[0].depends_on, [])) == 0 &&
      containers[0].startup_probe[0].http_get[0].path == "/readyz" &&
      containers[1].name == "otel-collector" && containers[1].image == "otel/opentelemetry-collector-contrib:0.155.0" &&
      containers[1].args == tolist(["--config=env:OTEL_SIDECAR_CONFIG"]) && length(containers[1].ports) == 0 &&
      length(coalesce(containers[1].depends_on, [])) == 0 &&
      containers[1].resources[0].limits == tomap({ cpu = "1", memory = "256Mi" }) &&
      containers[1].startup_probe[0].http_get[0].path == "/" && containers[1].startup_probe[0].http_get[0].port == 13133
    )]) && length(google_cloud_run_v2_service.physician_worker) == 1
    error_message = "physician, duet and physician-worker run the named product container at index 0 (port 8080, no depends_on) and the pinned otel-collector at index 1 with its health probe."
  }
  # CPU allocation does not follow the sidecar: only physician, whose
  # listener, bus subscriber and recovery sweep run between requests, keeps
  # instance-based CPU and its one minimum instance. Duet and the worker stay
  # request-billed and scale to zero.
  assert {
    condition = alltrue([for name, service in merge(
      { for app, service in google_cloud_run_v2_service.app : app => service },
      { for _, worker in google_cloud_run_v2_service.physician_worker : "physician-worker" => worker },
      ) : (
      alltrue([for container in service.template[0].containers : container.resources[0].cpu_idle == (name != "physician")]) &&
      service.template[0].scaling[0].min_instance_count == (name == "physician" ? 1 : 0)
    )])
    error_message = "With observability on, only physician keeps CPU allocated (both containers) and one minimum instance; duet and physician-worker keep request-based CPU and scale to zero."
  }
  assert {
    condition = alltrue([for containers in concat(
      [for service in google_cloud_run_v2_service.app : service.template[0].containers],
      [for worker in google_cloud_run_v2_service.physician_worker : worker.template[0].containers],
      ) : (
      { for env in containers[1].env : env.name => env.value } == {
        OTEL_SIDECAR_CONFIG   = file("${path.module}/../../../dev/observability/sidecar.yaml")
        OTEL_GATEWAY_ENDPOINT = "http://10.82.0.10:4318"
        OTEL_GATEWAY_AUDIENCE = "coda-otel-gateway-stg"
      }
    )])
    error_message = "Each sidecar gets the shared sidecar config, the gateway endpoint and the audience, and nothing else."
  }
  assert {
    condition = alltrue([for containers in concat(
      [for service in google_cloud_run_v2_service.app : service.template[0].containers],
      [for worker in google_cloud_run_v2_service.physician_worker : worker.template[0].containers],
      ) : (
      length(setsubtract(
        ["DD_ENV=staging", "OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318", "CODA_LOG_FORMAT=json", "CODA_TELEMETRY_CONTENT=none", "CODA_TELEMETRY_BACKEND=both"],
        [for env in containers[0].env : "${env.name}=${coalesce(env.value, "<secret>")}"],
      )) == 0 &&
      !contains([for env in containers[0].env : env.name], "DD_VERSION")
    )])
    error_message = "Every product container exports to its sidecar on localhost with DD_ENV=staging, JSON logs, no content and concurrent backends; DD_VERSION stays baked into the image."
  }
  assert {
    condition     = google_cloud_run_v2_service.app["physician"].template[0].containers[0].image == var.images["physician"].runtime && google_cloud_run_v2_service.physician_worker["physician"].template[0].containers[0].image == var.images["physician"].worker
    error_message = "Index 0 stays the release-owned product image (ignore_changes covers containers[0].image)."
  }
}

run "observability_grafana_backend" {
  # The same active staging shape with the Grafana backend: product
  # containers are told the backend so their log lines carry no dd.* keys.
  command = plan
  override_resource {
    target = google_compute_address.otel_gateway
    values = { address = "10.82.0.10" }
  }
  variables {
    observability_cluster_enabled          = true
    observability_enabled                  = true
    telemetry_backend                      = "grafana"
    enable_databases                       = true
    enable_workloads                       = true
    enable_physician_scale                 = true
    runtime_and_database_contract_approved = true
    images = {
      physician = {
        runtime   = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        migration = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-migrate@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        worker    = "us-east1-docker.pkg.dev/test-rust-stg-only/apps/physician-worker@sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
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
  }
  assert {
    condition = alltrue([for containers in concat(
      [for service in google_cloud_run_v2_service.app : service.template[0].containers],
      [for worker in google_cloud_run_v2_service.physician_worker : worker.template[0].containers],
      ) : (
      [for env in containers[0].env : env.value if env.name == "CODA_TELEMETRY_BACKEND"] == ["grafana"] &&
      contains([for env in containers[0].env : "${env.name}=${coalesce(env.value, "<secret>")}"], "DD_ENV=staging")
    )]) && length(google_cloud_run_v2_service.physician_worker) == 1
    error_message = "With the grafana backend every product container gets CODA_TELEMETRY_BACKEND=grafana once, and still names its environment."
  }
}
