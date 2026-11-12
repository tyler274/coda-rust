# Direct IAP is supported in the pinned stable google 7.24.0 provider:
# https://registry.terraform.io/providers/hashicorp/google/7.24.0/docs/resources/cloud_run_v2_service#iap_enabled
# https://cloud.google.com/run/docs/securing/identity-aware-proxy-cloud-run
# IAP intercepts ALL ingress paths, including run.app. No LB/IAP bypass path,
# no allUsers invoker, no invoker_iam_disabled. Internal organization OAuth only.
resource "google_project_service_identity" "iap" {
  provider   = google-beta
  project    = google_project.environment.project_id
  service    = "iap.googleapis.com"
  depends_on = [google_project_service.api]
}

resource "google_cloud_run_v2_service" "app" {
  for_each             = local.workloads
  project              = google_project.environment.project_id
  name                 = each.key
  location             = var.region
  deletion_protection  = true
  ingress              = "INGRESS_TRAFFIC_ALL"
  iap_enabled          = true
  invoker_iam_disabled = false
  template {
    service_account                  = google_service_account.app["${each.key}-runtime"].email
    execution_environment            = "EXECUTION_ENVIRONMENT_GEN2"
    timeout                          = "300s"
    max_instance_request_concurrency = 8
    scaling {
      # With physician scaling on, the site owns the recovery sweep that
      # republishes accepted turns whose Pub/Sub publish failed. It runs only
      # while an instance is up, so one physician instance stays up; at zero
      # an accepted turn would wait for unrelated traffic to start the site.
      # The chat topic exists only while physician scaling is on.
      min_instance_count = each.key == "physician" && length(google_pubsub_topic.chat_turns) > 0 ? 1 : 0
      max_instance_count = var.max_instances
    }
    vpc_access {
      egress = each.key == "physician" && var.enable_physician_smtp_relay ? "ALL_TRAFFIC" : "PRIVATE_RANGES_ONLY"
      network_interfaces {
        network    = google_compute_network.private.name
        subnetwork = google_compute_subnetwork.run.name
      }
    }
    containers {
      # Named only beside the otel-collector sidecar (index 1); unset keeps
      # the single-container template unchanged. Release updates this
      # container's image by that name (workspace-tools staging-release).
      name  = local.observability ? each.key : null
      image = try(var.images[each.key].runtime, "")
      ports { container_port = 8080 }
      resources {
        limits = { cpu = "1", memory = "1Gi" }
        # Physician keeps CPU allocated for the instance lifetime (instance-based
        # billing): its Postgres authority listener and the Redis bus subscriber
        # run between requests and stall under request-only CPU. With physician
        # scaling on, one instance stays up for the recovery sweep (see
        # scaling above). Duet keeps request-based CPU with or without the
        # sidecar: it has no work between requests, and telemetry is not a
        # reason to bill it for idle time (book/src/gcp.md, "Telemetry
        # sidecars and CPU allocation").
        cpu_idle          = each.key != "physician"
        startup_cpu_boost = false
      }
      env {
        name  = "DOPPLER_PROJECT"
        value = "coda-rust"
      }
      env {
        name  = "DOPPLER_CONFIG"
        value = try(var.doppler_configs["${each.key}-runtime"], "")
      }
      env {
        name = "DOPPLER_TOKEN"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.doppler["${each.key}-runtime"].secret_id
            version = try(var.doppler_token_versions[each.key].runtime, "")
          }
        }
      }
      dynamic "env" {
        for_each = try(var.runtime_environment[each.key], {})
        content {
          name  = env.key
          value = env.value
        }
      }
      dynamic "env" {
        for_each = each.key == "duet" ? google_storage_bucket.duet_privacy_journal : {}
        content {
          name  = "DUET_PRIVACY_JOURNAL_BUCKET"
          value = env.value.name
        }
      }
      # Physician only, and only behind enable_physician_scale: the site
      # publishes accepted chat turns here instead of running them inline.
      # PHYSICIAN_REDIS_URL / PHYSICIAN_REDIS_CA_PEM come from Doppler.
      dynamic "env" {
        for_each = each.key == "physician" ? google_pubsub_topic.chat_turns : {}
        content {
          name  = "PHYSICIAN_CHAT_TOPIC"
          value = env.value.id
        }
      }
      dynamic "env" {
        for_each = each.key == "physician" ? local.physician_simulation_env : {}
        content {
          name  = env.key
          value = env.value
        }
      }
      dynamic "env" {
        for_each = each.key == "physician" ? local.physician_rehearsal_env : {}
        content {
          name  = env.key
          value = env.value
        }
      }
      dynamic "env" {
        for_each = each.key == "physician" ? local.physician_clinical_library_env : {}
        content {
          name  = env.key
          value = env.value
        }
      }
      env {
        name  = "DD_ENV"
        value = local.dd_env
      }
      dynamic "env" {
        for_each = local.observability_product_env
        content {
          name  = env.key
          value = env.value
        }
      }
      startup_probe {
        initial_delay_seconds = 0
        period_seconds        = 5
        timeout_seconds       = 2
        failure_threshold     = 24
        http_get { path = "/readyz" }
      }
    }
    dynamic "containers" {
      for_each = local.otel_sidecar
      content {
        name  = containers.value.name
        image = containers.value.image
        args  = containers.value.args
        resources {
          limits = { cpu = "1", memory = "256Mi" }
          # Same CPU allocation as the product container: Cloud Run bills and
          # throttles the instance as a whole.
          cpu_idle          = each.key != "physician"
          startup_cpu_boost = false
        }
        env {
          name  = "OTEL_SIDECAR_CONFIG"
          value = containers.value.config
        }
        env {
          name  = "OTEL_GATEWAY_ENDPOINT"
          value = local.otel_gateway_endpoint
        }
        env {
          name  = "OTEL_GATEWAY_AUDIENCE"
          value = local.otel_gateway_audience
        }
        startup_probe {
          initial_delay_seconds = 0
          period_seconds        = 2
          timeout_seconds       = 1
          failure_threshold     = 30
          http_get {
            path = "/"
            port = 13133
          }
        }
      }
    }
  }
  lifecycle {
    # Release owns ONLY image digest, revision name, revision traffic/tag routing
    # and CLI client metadata. Infrastructure still owns auth, env, network,
    # identity, secret version pins, resource caps and probes. Never ignore template.
    ignore_changes = [template[0].containers[0].image, template[0].revision, traffic, client, client_version]
  }
  depends_on = [google_secret_manager_secret_iam_binding.bootstrap_reader, google_sql_user.app, google_project_service_identity.iap, google_storage_bucket_iam_member.duet_privacy_journal, google_compute_router_nat.physician_mail, google_pubsub_topic_iam_binding.chat_turns_publisher]
}

# Duet's independent privacy receipt journal. Hosted Duet refuses to serve
# without it (crates/duet-api/src/main.rs). Receipts are create-only
# (ifGenerationMatch=0), so the runtime gets create + read, never delete or
# overwrite. Physician has no journal and gets no binding.
resource "google_storage_bucket" "duet_privacy_journal" {
  for_each                    = contains(local.workloads, "duet") ? toset(["duet"]) : toset([])
  project                     = google_project.environment.project_id
  name                        = "${local.project_id}-duet-privacy-journal"
  location                    = var.region
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false
  versioning { enabled = true }
  lifecycle { prevent_destroy = true }
}

resource "google_storage_bucket_iam_member" "duet_privacy_journal" {
  for_each = { for role in ["roles/storage.objectCreator", "roles/storage.objectViewer"] : role => role if contains(local.workloads, "duet") }
  bucket   = google_storage_bucket.duet_privacy_journal["duet"].name
  role     = each.value
  member   = "serviceAccount:${google_service_account.app["duet-runtime"].email}"
}

resource "google_cloud_run_v2_service_iam_binding" "iap_invoker" {
  for_each = local.workloads
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.app[each.key].name
  role     = "roles/run.invoker"
  members  = ["serviceAccount:${google_project_service_identity.iap.email}"]
}

resource "google_iap_web_cloud_run_service_iam_binding" "team" {
  for_each               = local.workloads
  project                = google_project.environment.project_id
  location               = var.region
  cloud_run_service_name = google_cloud_run_v2_service.app[each.key].name
  role                   = "roles/iap.httpsResourceAccessor"
  # This authoritative binding is the sole owner of proxy access. Never add a
  # competing IAP member resource in the public-site root or a project grant.
  members = setunion(
    toset(["group:${var.iap_group}"]),
    each.key == "physician" ? var.physician_proxy_invokers : toset([]),
    toset([for identity in google_service_account.availability : "serviceAccount:${identity.email}"]),
    # Physician only (rehearsal.tf); the site still accepts it for nothing
    # but POST /auth/rehearsal, which mints only a rehearsal session.
    each.key == "physician" ? toset([for identity in google_service_account.physician_rehearsal : "serviceAccount:${identity.email}"]) : toset([]),
  )
}

resource "google_cloud_run_v2_job" "migration" {
  for_each            = local.workloads
  project             = google_project.environment.project_id
  name                = "${each.key}-migrate"
  location            = var.region
  deletion_protection = true
  template {
    task_count  = 1
    parallelism = 1
    template {
      service_account = google_service_account.app["${each.key}-migration"].email
      timeout         = "600s"
      max_retries     = 0
      vpc_access {
        egress = "PRIVATE_RANGES_ONLY"
        network_interfaces {
          network    = google_compute_network.private.name
          subnetwork = google_compute_subnetwork.run.name
        }
      }
      containers {
        image = try(var.images[each.key].migration, "")
        resources { limits = { cpu = "1", memory = "1Gi" } }
        env {
          name  = "DOPPLER_PROJECT"
          value = "coda-rust"
        }
        env {
          name  = "DOPPLER_CONFIG"
          value = try(var.doppler_configs["${each.key}-migration"], "")
        }
        env {
          name = "DOPPLER_TOKEN"
          value_source {
            secret_key_ref {
              secret  = google_secret_manager_secret.doppler["${each.key}-migration"].secret_id
              version = try(var.doppler_token_versions[each.key].migration, "")
            }
          }
        }
        dynamic "env" {
          for_each = try(var.migration_environment[each.key], {})
          content {
            name  = env.key
            value = env.value
          }
        }
        dynamic "env" {
          for_each = each.key == "physician" ? local.physician_simulation_env : {}
          content {
            name  = env.key
            value = env.value
          }
        }
      }
    }
  }
  lifecycle {
    ignore_changes = [template[0].template[0].containers[0].image, client, client_version]
  }
  depends_on = [google_secret_manager_secret_iam_binding.bootstrap_reader, google_sql_user.app]
}

# --- Physician chat-turn worker (enable_physician_scale) --------------------
#
# Receives Pub/Sub pushes for physician-chat-turns, claims the accepted turn in
# Postgres, runs the model and publishes workspace_changed on Redis. It is not
# a browser surface: no IAP, INTERNAL ingress (Pub/Sub in the same project is
# internal when it calls the default run.app URL,
# https://cloud.google.com/run/docs/securing/ingress#settings), IAM invoker
# checks on, and the push identity is the only invoker.
locals {
  # Cloud Run's deterministic run.app URL. Known before the service exists, so
  # the worker can be told its own OIDC audience without a self-reference:
  # https://cloud.google.com/run/docs/triggering/https-request#deterministic
  physician_worker_audience = "https://physician-worker-${google_project.environment.number}.${var.region}.run.app"
}

# Own identity so the worker's grants are visible and revocable on their own.
# It reuses the physician-runtime Doppler config and token slot rather than a
# separate one: the worker needs exactly that config's secrets (database URL,
# model key, Redis URL), so a second token on the same config would add a
# rotation step without narrowing authority. The slot's reader binding below
# names both identities; Duet's slots are untouched.
resource "google_service_account" "physician_worker" {
  for_each     = local.physician_scale_apps
  project      = google_project.environment.project_id
  account_id   = "physician-worker-runtime"
  display_name = "physician worker runtime (${var.environment})"
  description  = "physician-worker Cloud Run identity: physician SQL, physician-runtime Doppler token. No Pub/Sub publish, no Duet access."
  depends_on   = [google_project_service.api]
}

resource "google_project_iam_member" "physician_worker_sql" {
  for_each = { for role in ["roles/cloudsql.client", "roles/cloudsql.instanceUser"] : role => role if local.physician_scale }
  project  = google_project.environment.project_id
  role     = each.value
  member   = "serviceAccount:${google_service_account.physician_worker["physician"].email}"
  condition {
    title      = "only-physician-instance"
    expression = "resource.name == 'projects/${local.project_id}/instances/${local.sql_instance_names["physician"]}'"
  }
}

# Cloud Scheduler HTTP targets originate outside this VPC (like availability.tf),
# so enabling the schedule opens network ingress while Cloud Run Invoker IAM and
# in-process route-specific OIDC verification gate every request.
resource "google_cloud_run_v2_service" "physician_worker" {
  for_each             = local.physician_scale_apps
  project              = google_project.environment.project_id
  name                 = "physician-worker"
  location             = var.region
  description          = "Physician worker; invoked only by authenticated Pub/Sub push, Cloud Scheduler wake and availability probes."
  ingress              = length(local.physician_schedule) > 0 ? "INGRESS_TRAFFIC_ALL" : "INGRESS_TRAFFIC_INTERNAL_ONLY"
  iap_enabled          = false
  invoker_iam_disabled = false
  custom_audiences     = [local.physician_worker_audience]
  # Stateless (turns live in Postgres, the queue in Pub/Sub). Turning the gate
  # off must remove it in one reviewed plan.
  deletion_protection = false
  template {
    service_account       = google_service_account.physician_worker[each.key].email
    execution_environment = "EXECUTION_ENVIRONMENT_GEN2"
    # Matches the subscription's 600 s ack deadline: a turn runs inside the push.
    timeout = "600s"
    # Turns wait on the model, so a few share an instance; the cap bounds model
    # concurrency and keeps the shared physician-runtime DB role within its
    # connection limit together with the site (2 x 4 + 2 x 4 pooled).
    max_instance_request_concurrency = 4
    scaling {
      min_instance_count = 0
      max_instance_count = var.max_instances
    }
    vpc_access {
      egress = "PRIVATE_RANGES_ONLY"
      network_interfaces {
        network    = google_compute_network.private.name
        subnetwork = google_compute_subnetwork.run.name
      }
    }
    containers {
      name  = local.observability ? "physician-worker" : null
      image = try(coalesce(var.images[each.key].worker), "")
      ports { container_port = 8080 }
      resources {
        limits = { cpu = "1", memory = "1Gi" }
        # Request-based CPU is enough: every unit of work is a push request held
        # open until the turn commits, and the worker keeps no listener or
        # subscriber between requests (it only PUBLISHes to Redis). The
        # sidecar does not change that; see the app services above.
        cpu_idle          = true
        startup_cpu_boost = false
      }
      env {
        name  = "DOPPLER_PROJECT"
        value = "coda-rust"
      }
      env {
        name  = "DOPPLER_CONFIG"
        value = try(var.doppler_configs["physician-runtime"], "")
      }
      env {
        name = "DOPPLER_TOKEN"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.doppler["physician-runtime"].secret_id
            version = try(var.doppler_token_versions["physician"].runtime, "")
          }
        }
      }
      dynamic "env" {
        for_each = try(var.runtime_environment["physician"], {})
        content {
          name  = env.key
          value = env.value
        }
      }
      env {
        name  = "PHYSICIAN_WORKER_AUDIENCE"
        value = local.physician_worker_audience
      }
      env {
        name  = "PHYSICIAN_PUSH_SERVICE_ACCOUNT"
        value = google_service_account.physician_pubsub_push[each.key].email
      }
      dynamic "env" {
        for_each = local.physician_schedule_env
        content {
          name  = env.key
          value = env.value
        }
      }
      dynamic "env" {
        for_each = local.physician_clinical_library_env
        content {
          name  = env.key
          value = env.value
        }
      }
      dynamic "env" {
        for_each = local.physician_simulation_env
        content {
          name  = env.key
          value = env.value
        }
      }
      env {
        name  = "DD_ENV"
        value = local.dd_env
      }
      dynamic "env" {
        for_each = local.observability_product_env
        content {
          name  = env.key
          value = env.value
        }
      }
      startup_probe {
        initial_delay_seconds = 0
        period_seconds        = 5
        timeout_seconds       = 2
        failure_threshold     = 24
        http_get { path = "/readyz" }
      }
    }
    # The same otel-collector sidecar as the app services; see above.
    dynamic "containers" {
      for_each = local.otel_sidecar
      content {
        name  = containers.value.name
        image = containers.value.image
        args  = containers.value.args
        resources {
          limits            = { cpu = "1", memory = "256Mi" }
          cpu_idle          = true
          startup_cpu_boost = false
        }
        env {
          name  = "OTEL_SIDECAR_CONFIG"
          value = containers.value.config
        }
        env {
          name  = "OTEL_GATEWAY_ENDPOINT"
          value = local.otel_gateway_endpoint
        }
        env {
          name  = "OTEL_GATEWAY_AUDIENCE"
          value = local.otel_gateway_audience
        }
        startup_probe {
          initial_delay_seconds = 0
          period_seconds        = 2
          timeout_seconds       = 1
          failure_threshold     = 30
          http_get {
            path = "/"
            port = 13133
          }
        }
      }
    }
  }
  lifecycle {
    # Same release-owned fields as the app services; see above.
    ignore_changes = [template[0].containers[0].image, template[0].revision, traffic, client, client_version]
  }
  depends_on = [google_secret_manager_secret_iam_binding.bootstrap_reader, google_sql_user.app, google_project_iam_member.physician_worker_sql]
}

# Authoritative for run.invoker: only the push, probe and scheduler identities,
# never allUsers, the site runtime or the IAP agent.
resource "google_cloud_run_v2_service_iam_binding" "physician_worker_invoker" {
  for_each = local.physician_scale_apps
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.physician_worker[each.key].name
  role     = "roles/run.invoker"
  members = concat(
    ["serviceAccount:${google_service_account.physician_pubsub_push[each.key].email}"],
    [for identity in google_service_account.availability : "serviceAccount:${identity.email}"],
    [for identity in google_service_account.physician_scheduler : "serviceAccount:${identity.email}"],
  )
}

# Release updates the worker image like the app services (deploy-staging).
resource "google_cloud_run_v2_service_iam_member" "physician_worker_release" {
  for_each = local.physician_scale_apps
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.physician_worker[each.key].name
  role     = google_project_iam_custom_role.release_service.name
  member   = "serviceAccount:${google_service_account.deployer.email}"
}

resource "google_service_account_iam_member" "physician_worker_release_act_as" {
  for_each           = local.physician_scale_apps
  service_account_id = google_service_account.physician_worker[each.key].name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.deployer.email}"
}
