# Self-hosted Langfuse v3: the agent-run workbench. It holds a one-way copy of
# generation spans (Postgres and git stay the truth), so retention is short
# and expiry loses nothing. Postgres is Cloud SQL with IAM login; blobs are a
# GCS bucket through the S3 interoperability API; ClickHouse and Valkey run
# in-cluster with PVCs. The web service is ClusterIP only.
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

  # Every value the chart sees. Secrets are referenced by Secret name and key;
  # the Doppler operator fills those Secrets (doppler.tf).
  langfuse_values = !var.langfuse_enabled ? null : {
    langfuse = {
      image = { tag = local.langfuse_image_tag }

      salt          = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_SALT" } }
      encryptionKey = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_ENCRYPTION_KEY" } }
      nextauth = {
        # The UI is reached through kubectl port-forward until the IAP
        # gateway is on; then it is the gateway hostname.
        url    = var.langfuse_iap_enabled ? "https://${var.langfuse_hostname}" : "http://localhost:3000"
        secret = { secretKeyRef = { name = "langfuse-secrets", key = "LANGFUSE_NEXTAUTH_SECRET" } }
      }

      features = {
        telemetryEnabled = false
        signUpDisabled   = true
      }
      logging = { format = "json" }

      serviceAccount = {
        create      = true
        name        = local.langfuse_ksa
        annotations = { "iam.gke.io/gcp-service-account" = local.langfuse_gsa }
      }

      # No public edge, ever: the chart's Ingress stays off. Human access is
      # port-forward or the IAP-gated internal Gateway (iap.tf below).
      ingress = { enabled = false }

      web = {
        service = { type = "ClusterIP", port = 3000 }
        pod = {
          additionalEnv = [
            { name = "LANGFUSE_INIT_ORG_ID", value = "coda" },
            { name = "LANGFUSE_INIT_ORG_NAME", value = "Coda" },
            { name = "LANGFUSE_INIT_PROJECT_ID", value = "coda-agents-${var.environment}" },
            { name = "LANGFUSE_INIT_PROJECT_NAME", value = "coda-agents" },
            { name = "LANGFUSE_INIT_USER_NAME", value = "Coda Admin" },
          ]
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

# Optional human access: an internal (regional, VPC-only) Gateway with IAP in
# front of langfuse-web. Off by default; there is never an external Gateway.
locals {
  langfuse_gateway_class = "gke-l7-rilb"

  # Rendered only by helm_release.langfuse_iap, which exists only when
  # langfuse_iap_enabled is true.
  langfuse_iap_manifests = [
    {
      apiVersion = "gateway.networking.k8s.io/v1"
      kind       = "Gateway"
      metadata   = { name = "langfuse-internal", namespace = local.namespace }
      spec = {
        gatewayClassName = local.langfuse_gateway_class
        listeners = [{
          name     = "https"
          protocol = "HTTPS"
          port     = 443
          hostname = var.langfuse_hostname
          tls = {
            mode    = "Terminate"
            options = { "networking.gke.io/pre-shared-certs" = var.langfuse_iap_certificate }
          }
        }]
      }
    },
    {
      apiVersion = "gateway.networking.k8s.io/v1"
      kind       = "HTTPRoute"
      metadata   = { name = "langfuse-web", namespace = local.namespace }
      spec = {
        parentRefs = [{ name = "langfuse-internal" }]
        hostnames  = [var.langfuse_hostname]
        rules      = [{ backendRefs = [{ name = local.langfuse_web_service, port = 3000 }] }]
      }
    },
    {
      apiVersion = "networking.gke.io/v1"
      kind       = "GCPBackendPolicy"
      metadata   = { name = "langfuse-web-iap", namespace = local.namespace }
      spec = {
        default = {
          iap = {
            enabled            = true
            clientID           = var.langfuse_iap_client_id
            oauth2ClientSecret = { name = "langfuse-iap-oauth" }
          }
        }
        targetRef = { group = "", kind = "Service", name = local.langfuse_web_service }
      }
    },
  ]
}

resource "google_compute_subnetwork" "proxy_only" {
  count = var.langfuse_iap_enabled ? 1 : 0

  project       = var.project_id
  name          = "coda-observability-proxy-only-${var.region}"
  region        = var.region
  network       = data.google_compute_network.runtime.id
  ip_cidr_range = var.proxy_only_subnet_cidr
  purpose       = "REGIONAL_MANAGED_PROXY"
  role          = "ACTIVE"
}

resource "google_iap_web_region_backend_service_iam_member" "langfuse" {
  # GKE owns the backend identity. Bootstrap the gateway without a grant,
  # then bind only the backend the operator verified for its Langfuse route.
  for_each = var.langfuse_iap_enabled && var.langfuse_iap_backend_service != null ? toset(var.langfuse_iap_members) : toset([])

  project                    = var.project_id
  region                     = var.region
  web_region_backend_service = var.langfuse_iap_backend_service
  role                       = "roles/iap.httpsResourceAccessor"
  member                     = each.value
}

resource "helm_release" "langfuse_iap" {
  count = var.langfuse_iap_enabled ? 1 : 0

  name      = "langfuse-iap"
  chart     = local.manifests_chart
  namespace = kubernetes_namespace.observability.metadata[0].name
  atomic    = true

  values = [yamlencode({ manifests = local.langfuse_iap_manifests })]

  depends_on = [
    helm_release.langfuse,
    helm_release.doppler_secrets,
    google_compute_subnetwork.proxy_only,
  ]
}
