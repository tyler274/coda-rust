# Self-hosted Langfuse v3: the agent-run workbench. It holds a one-way copy of
# generation spans (Postgres and git stay the truth), so retention is short
# and expiry loses nothing. Postgres is Cloud SQL with IAM login; blobs are a
# GCS bucket through the S3 interoperability API; ClickHouse and Valkey run
# in-cluster with PVCs. The web service is ClusterIP; people reach it only
# through public-site's /langfuse/ mount (web.tf) or port-forward.
#
# Everything here exists only with langfuse_enabled. Off, the gateway still
# runs and sends generation spans nowhere (collector.tf); the project IAM
# grants and the HMAC key the operator mints are not needed yet.
locals {
  langfuse_count         = var.langfuse_enabled ? 1 : 0
  langfuse_chart_version = "1.5.41"
  langfuse_image_tag     = "3.225.11"
  langfuse_database      = "langfuse"
  langfuse_ksa           = local.langfuse_release
  langfuse_gsa           = one(google_service_account.langfuse[*].email)
  langfuse_db_user       = var.langfuse_enabled ? trimsuffix(local.langfuse_gsa, ".gserviceaccount.com") : null
  cloud_sql_proxy_image  = "gcr.io/cloud-sql-connectors/cloud-sql-proxy:2.20.0"

  # Under staging_proxy, Langfuse web serves everything below /langfuse
  # (main.tf langfuse_base_path). Next.js fixes the base path at build time,
  # so web runs the digest-pinned base-path build of the same upstream tag
  # (.github/workflows/langfuse-web-image.yml). The chart renders
  # repository:tag, and repo:tag@sha256:... resolves by digest. The worker
  # serves no pages and keeps the stock image.
  langfuse_web_image = var.langfuse_web_image == null || local.langfuse_base_path == "" ? {} : {
    repository = split("@", var.langfuse_web_image)[0]
    tag        = "${local.langfuse_image_tag}@${split("@", var.langfuse_web_image)[1]}"
  }

  # Sign-in through langfuse-sso (langfuse-sso.tf), Langfuse's generic OIDC
  # provider `custom`. A first sign-in joins the coda organization with the
  # NONE role and no project, so nobody sees traces until an admin grants a
  # role. LANGFUSE_ALLOWED_ORGANIZATION_CREATORS names the admin; Langfuse
  # enforces it only with an EE license (langfuse_ee_trial). Without one an
  # engineer may create an empty organization of their own, which holds none
  # of coda's traces. Account
  # linking lets the admin's existing user sign in the same way. Passwords are
  # off: sign-up is on so SSO can create users, and a password route would let
  # anyone behind IAP register or claim another engineer's email. NextAuth
  # checks PKCE, state and nonce and verifies the ES256 ID token against the
  # issuer's JWKS.
  langfuse_sso_web_env = [for env in [
    { name = "AUTH_CUSTOM_NAME", value = "Google (staging IAP)" },
    { name = "AUTH_CUSTOM_ISSUER", value = local.langfuse_sso_issuer },
    { name = "AUTH_CUSTOM_CLIENT_ID", value = local.langfuse_sso_client },
    { name = "AUTH_CUSTOM_CLIENT_SECRET", valueFrom = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_SSO_CLIENT_SECRET" } } },
    { name = "AUTH_CUSTOM_SCOPE", value = "openid email profile" },
    { name = "AUTH_CUSTOM_CHECKS", value = "pkce,state,nonce" },
    { name = "AUTH_CUSTOM_CLIENT_AUTH_METHOD", value = "client_secret_basic" },
    { name = "AUTH_CUSTOM_ID_TOKEN_SIGNED_RESPONSE_ALG", value = "ES256" },
    { name = "AUTH_CUSTOM_ALLOW_ACCOUNT_LINKING", value = "true" },
    { name = "LANGFUSE_DEFAULT_ORG_ID", value = "coda" },
    { name = "LANGFUSE_DEFAULT_ORG_ROLE", value = "NONE" },
    { name = "LANGFUSE_ALLOWED_ORGANIZATION_CREATORS", valueFrom = { secretKeyRef = { name = "langfuse-init", key = "LANGFUSE_INIT_USER_EMAIL" } } },
    { name = "AUTH_DISABLE_USERNAME_PASSWORD", value = "true" },
  ] : env if local.langfuse_sso]

  # Every value the chart sees. Secrets are referenced by Secret name and key;
  # the Doppler operator fills those Secrets (doppler.tf).
  langfuse_values = !var.langfuse_enabled ? null : {
    langfuse = {
      image = { tag = local.langfuse_image_tag }

      salt          = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_SALT" } }
      encryptionKey = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_ENCRYPTION_KEY" } }
      nextauth = {
        # The UI is public-site's /langfuse/ mount under staging_proxy, and
        # kubectl port-forward otherwise.
        url    = var.staging_proxy == null ? "http://localhost:3000" : "${var.staging_proxy.origin}/langfuse/api/auth"
        secret = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_NEXTAUTH_SECRET" } }
      }

      # With langfuse-sso, an engineer's first sign-in creates their user,
      # so sign-up must be on. Staging's IAP gates who reaches the issuer,
      # and AUTH_DISABLE_USERNAME_PASSWORD refuses password sign-up and
      # sign-in on every route (langfuse_sso_web_env).
      features = {
        telemetryEnabled = false
        signUpDisabled   = !local.langfuse_sso
      }

      # Enterprise features (langfuse_ee_trial). Every EE check in 3.225.11
      # reads only the key's langfuse_ee_ prefix, on web and worker alike.
      # With a key set, Langfuse's usage report ignores TELEMETRY_ENABLED,
      # but its PostHog client exists only when NEXT_PUBLIC_POSTHOG_KEY is
      # set or telemetry is on, and neither ever is here (tested), so the
      # report goes nowhere. Re-check ServerPosthog and telemetry() on every
      # image bump. Empty name and key render no env var.
      licenseKey = {
        secretKeyRef = {
          name = var.langfuse_ee_trial ? "langfuse-secrets" : ""
          key  = var.langfuse_ee_trial ? "LANGFUSE_EE_LICENSE_KEY" : ""
        }
      }
      logging = { format = "json" }

      serviceAccount = {
        create      = true
        name        = local.langfuse_ksa
        annotations = { "iam.gke.io/gcp-service-account" = local.langfuse_gsa }
      }

      # No public edge, ever: the chart's Ingress stays off. Human access is
      # port-forward or public-site's /langfuse/ mount through the relay and
      # the internal load balancer (web.tf).
      ingress = { enabled = false }

      web = {
        image          = local.langfuse_web_image
        service        = { type = "ClusterIP", port = 3000 }
        livenessProbe  = { path = "${local.langfuse_base_path}/api/public/health" }
        readinessProbe = { path = "${local.langfuse_base_path}/api/public/ready" }
        pod = {
          additionalEnv = concat([
            { name = "LANGFUSE_INIT_ORG_ID", value = "coda" },
            { name = "LANGFUSE_INIT_ORG_NAME", value = "Coda" },
            { name = "LANGFUSE_INIT_PROJECT_ID", value = "coda-agents-${var.environment}" },
            { name = "LANGFUSE_INIT_PROJECT_NAME", value = "coda-agents" },
            { name = "LANGFUSE_INIT_USER_NAME", value = "Coda Admin" },
          ], local.langfuse_sso_web_env)
          # Init keys and the first admin's login, synced from Doppler.
          additionalEnvFrom = [{ secretRef = { name = "langfuse-init" } }]
        }
      }

      additionalEnv = [
        # IAM database login: the proxy sidecar authenticates as the pod's
        # Google service account, so the URL carries no password. The IAM
        # user name contains `@`, which must be percent-encoded in a URL.
        { name = "DATABASE_URL", value = "postgresql://${urlencode(local.langfuse_db_user)}@127.0.0.1:5432/${local.langfuse_database}" },
        { name = "LANGFUSE_S3_BATCH_EXPORT_ENABLED", value = "false" },
        # Recorded for operators and the ENG-74 improver: the project's data
        # retention setting matches the bucket lifecycle below.
        { name = "CODA_LANGFUSE_RETENTION_DAYS", value = tostring(var.langfuse_retention_days) },
      ]

      extraContainers = [{
        name  = "cloud-sql-proxy"
        image = local.cloud_sql_proxy_image
        args = [
          "--private-ip",
          "--auto-iam-authn",
          "--structured-logs",
          "--port=5432",
          one(google_sql_database_instance.langfuse[*].connection_name),
        ]
        securityContext = { runAsNonRoot = true }
        resources = {
          requests = { cpu = "100m", memory = "128Mi" }
        }
      }]
    }

    postgresql = {
      deploy = false
    }

    clickhouse = {
      deploy          = true
      clusterEnabled  = false
      replicaCount    = 1
      shards          = 1
      resourcesPreset = "large"
      persistence     = { size = "${var.clickhouse_disk_gb}Gi" }
      zookeeper       = { enabled = false }
      auth = {
        username          = "default"
        existingSecret    = "langfuse-secrets"
        existingSecretKey = "LANGFUSE_CLICKHOUSE_PASSWORD"
      }
    }

    redis = {
      deploy = true
      auth = {
        existingSecret            = "langfuse-secrets"
        existingSecretPasswordKey = "LANGFUSE_REDIS_PASSWORD"
      }
      primary = {
        extraFlags  = ["--maxmemory-policy noeviction"]
        persistence = { size = "8Gi" }
      }
    }

    s3 = {
      deploy          = false
      storageProvider = "s3"
      bucket          = one(google_storage_bucket.langfuse[*].name)
      region          = "auto"
      endpoint        = "https://storage.googleapis.com"
      forcePathStyle  = false
      accessKeyId     = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_S3_ACCESS_KEY_ID" } }
      secretAccessKey = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_S3_SECRET_ACCESS_KEY" } }
      eventUpload     = { prefix = "events/" }
      batchExport     = { enabled = false }
      mediaUpload     = { prefix = "media/" }
    }
  }
}

resource "google_service_account" "langfuse" {
  count = local.langfuse_count

  project      = var.project_id
  account_id   = "coda-langfuse"
  display_name = "Coda Langfuse (web and worker)"

  depends_on = [google_project_service.workload]
}

# The S3-interop HMAC key belongs to this account. The operator mints it with
# `gcloud storage hmac create` and stores the pair in Doppler, so the secret
# half never enters tofu state (google_storage_hmac_key would write it there).
resource "google_service_account" "langfuse_blobs" {
  count = local.langfuse_count

  project      = var.project_id
  account_id   = "coda-langfuse-blobs"
  display_name = "Coda Langfuse blob storage (S3 interop HMAC)"

  depends_on = [google_project_service.workload]
}

resource "google_service_account_iam_member" "langfuse_workload_identity" {
  count = local.langfuse_count

  service_account_id = google_service_account.langfuse[0].name
  role               = "roles/iam.workloadIdentityUser"
  # The cluster's Workload Identity pool belongs to the cluster project.
  member = "serviceAccount:${var.cluster_project}.svc.id.goog[${local.namespace}/${local.langfuse_ksa}]"
}

resource "google_project_iam_member" "langfuse_cloud_sql" {
  for_each = var.langfuse_enabled ? toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ]) : toset([])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${local.langfuse_gsa}"
}

resource "google_sql_database_instance" "langfuse" {
  count = local.langfuse_count

  project             = var.project_id
  name                = "coda-langfuse"
  region              = var.region
  database_version    = "POSTGRES_16"
  deletion_protection = true

  settings {
    tier                        = var.langfuse_sql_tier
    edition                     = "ENTERPRISE"
    availability_type           = "ZONAL"
    deletion_protection_enabled = true
    disk_type                   = "PD_SSD"
    disk_size                   = 20
    disk_autoresize             = true
    disk_autoresize_limit       = var.langfuse_sql_disk_autoresize_limit_gb

    ip_configuration {
      ipv4_enabled                                  = false
      private_network                               = data.google_compute_network.runtime.id
      enable_private_path_for_google_cloud_services = true
    }

    database_flags {
      name  = "cloudsql.iam_authentication"
      value = "on"
    }

    backup_configuration {
      enabled = true
    }

    maintenance_window {
      day          = 7
      hour         = 7
      update_track = "stable"
    }

    user_labels = {
      application         = "langfuse"
      environment         = var.environment
      data_classification = "agent-traces"
    }
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_project_service.workload]
}

resource "google_sql_database" "langfuse" {
  count = local.langfuse_count

  project  = var.project_id
  name     = local.langfuse_database
  instance = google_sql_database_instance.langfuse[0].name
}

# Langfuse runs its own Prisma migrations, so its IAM user owns the schema.
resource "google_sql_user" "langfuse" {
  count = local.langfuse_count

  project        = var.project_id
  name           = local.langfuse_db_user
  instance       = google_sql_database_instance.langfuse[0].name
  type           = "CLOUD_IAM_SERVICE_ACCOUNT"
  database_roles = ["cloudsqlsuperuser"]
}

resource "google_storage_bucket" "langfuse" {
  count = local.langfuse_count

  project                     = var.project_id
  name                        = "${var.project_id}-langfuse-blobs"
  location                    = var.region
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false

  lifecycle_rule {
    condition {
      age = var.langfuse_retention_days
    }
    action {
      type = "Delete"
    }
  }

  labels = {
    application         = "langfuse"
    environment         = var.environment
    data_classification = "agent-traces"
  }

  depends_on = [google_project_service.workload]
}

resource "google_storage_bucket_iam_member" "langfuse_blobs" {
  count = local.langfuse_count

  bucket = google_storage_bucket.langfuse[0].name
  role   = "roles/storage.objectUser"
  member = "serviceAccount:${google_service_account.langfuse_blobs[0].email}"
}

resource "helm_release" "langfuse" {
  count = local.langfuse_count

  name       = local.langfuse_release
  repository = "https://langfuse.github.io/langfuse-k8s"
  chart      = "langfuse"
  version    = local.langfuse_chart_version
  namespace  = kubernetes_namespace.observability.metadata[0].name
  atomic     = true
  timeout    = 900

  values = [yamlencode(local.langfuse_values)]

  depends_on = [
    helm_release.doppler_secrets,
    google_sql_database.langfuse,
    google_sql_user.langfuse,
    google_service_account_iam_member.langfuse_workload_identity,
    google_storage_bucket_iam_member.langfuse_blobs,
  ]
}
