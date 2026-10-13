# The physician site runs as one private deployment behind one ClusterIP
# service. Nothing here faces the public internet: the site serves the
# recorded synthetic practice, so an ingress is added only with an exact
# browser origin and a separately approved edge.
resource "kubernetes_namespace" "physician" {
  metadata {
    name = "physician"
    labels = {
      application = "physician-runtime"
    }
  }
}

resource "kubernetes_service_account" "physician_site" {
  count = var.physician_site_enabled ? 1 : 0

  metadata {
    name      = "physician-site"
    namespace = kubernetes_namespace.physician.metadata[0].name
    annotations = {
      "iam.gke.io/gcp-service-account" = local.site_account
    }
  }
}

resource "kubernetes_service_account" "database_migrator" {
  count = var.migration_enabled ? 1 : 0

  metadata {
    name      = "physician-migrator"
    namespace = kubernetes_namespace.physician.metadata[0].name
    annotations = {
      "iam.gke.io/gcp-service-account" = local.migration_account
    }
  }
}

resource "kubernetes_secret" "physician_database" {
  count = var.physician_site_enabled ? 1 : 0

  metadata {
    name      = "physician-database"
    namespace = kubernetes_namespace.physician.metadata[0].name
  }

  data = {
    # IAM database authentication; no password exists anywhere.
    PHYSICIAN_DATABASE_URL     = local.site_connection
    PHYSICIAN_AUTH             = "required"
    PHYSICIAN_SIGN_IN_DELIVERY = var.sign_in_delivery
  }
}

resource "kubernetes_deployment" "physician_site" {
  count = var.physician_site_enabled ? 1 : 0

  metadata {
    name      = "physician-site"
    namespace = kubernetes_namespace.physician.metadata[0].name
    labels = {
      application = "physician-runtime"
      component   = "site"
    }
  }

  spec {
    replicas = local.site_replicas

    selector {
      match_labels = {
        application = "physician-runtime"
        component   = "site"
      }
    }

    template {
      metadata {
        labels = {
          application = "physician-runtime"
          component   = "site"
        }
      }

      spec {
        service_account_name = kubernetes_service_account.physician_site[0].metadata[0].name

        container {
          name  = "physician-site"
          image = var.physician_site_image

          port {
            container_port = local.site_port
          }

          env_from {
            secret_ref {
              name = kubernetes_secret.physician_database[0].metadata[0].name
            }
          }

          readiness_probe {
            http_get {
              path = "/health"
              port = local.site_port
            }

            initial_delay_seconds = 5
            period_seconds        = 10
          }

          liveness_probe {
            http_get {
              path = "/health"
              port = local.site_port
            }

            initial_delay_seconds = 20
            period_seconds        = 20
          }

          resources {
            requests = {
              cpu    = "250m"
              memory = "512Mi"
            }
            limits = {
              cpu    = "1"
              memory = "1Gi"
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "physician_site" {
  count = var.physician_site_enabled ? 1 : 0

  metadata {
    name      = "physician-site"
    namespace = kubernetes_namespace.physician.metadata[0].name
  }

  spec {
    selector = {
      application = "physician-runtime"
      component   = "site"
    }

    port {
      name        = "http"
      port        = 80
      target_port = local.site_port
    }

    type = "ClusterIP"
  }
}

# The migration job runs to completion ahead of a new site image: it applies
# the SeaORM migration chain and exits. It holds its own IAM database user.
resource "kubernetes_job" "physician_migration" {
  count = var.migration_enabled ? 1 : 0

  metadata {
    name      = "physician-migration"
    namespace = kubernetes_namespace.physician.metadata[0].name
  }

  spec {
    backoff_limit = 0

    template {
      metadata {
        labels = {
          application = "physician-runtime"
          component   = "migration"
        }
      }

      spec {
        service_account_name = kubernetes_service_account.database_migrator[0].metadata[0].name
        restart_policy       = "Never"

        container {
          name    = "physician-migration"
          image   = var.physician_migration_image
          command = ["/usr/local/bin/physician-auth-migrate"]

          env {
            name  = "PHYSICIAN_DATABASE_URL"
            value = "postgres://${trimsuffix(local.migration_account, ".gserviceaccount.com")}@/${local.database_name}?host=/cloudsql/${google_sql_database_instance.physician.connection_name}"
          }
        }
      }
    }
  }
}
