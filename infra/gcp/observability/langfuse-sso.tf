# langfuse-sso (crates/langfuse-sso): the OIDC issuer that signs engineers
# into Langfuse as the Google account staging's IAP already verified.
#
# Browsers reach only its authorize step, at <origin>/langfuse-sso/authorize
# through public-site and the relay, where it reads the forwarded IAP
# assertion. Langfuse web discovers it and exchanges codes over this
# ClusterIP Service, because its server cannot pass IAP. One replica, since
# authorization codes live in its memory for a minute.
#
# Null langfuse_sso_image leaves the issuer out and Langfuse on its password
# form alone.
locals {
  langfuse_sso          = var.langfuse_sso_image != null
  langfuse_sso_name     = "langfuse-sso"
  langfuse_sso_labels   = { application = "observability", component = local.langfuse_sso_name }
  langfuse_sso_issuer   = "http://${local.langfuse_sso_name}.${local.namespace}.svc.cluster.local:3000/langfuse-sso"
  langfuse_sso_client   = "langfuse"
  langfuse_sso_callback = local.langfuse_sso ? "${var.staging_proxy.origin}/langfuse/api/auth/callback/custom" : null

  langfuse_sso_env = !local.langfuse_sso ? {} : {
    PORT                       = "3000"
    LANGFUSE_SSO_ISSUER        = local.langfuse_sso_issuer
    LANGFUSE_SSO_AUTHORIZE_URL = "${var.staging_proxy.origin}/langfuse-sso/authorize"
    LANGFUSE_SSO_CLIENT_ID     = local.langfuse_sso_client
    LANGFUSE_SSO_REDIRECT_URI  = local.langfuse_sso_callback
    LANGFUSE_SSO_IAP_AUDIENCE  = var.staging_proxy.iap_audience
  }
}

resource "kubernetes_deployment_v1" "langfuse_sso" {
  for_each = local.langfuse_sso ? toset([local.langfuse_sso_name]) : toset([])

  metadata {
    name      = each.key
    namespace = kubernetes_namespace.observability.metadata[0].name
    labels    = local.langfuse_sso_labels
  }

  spec {
    replicas = 1

    # Never two replicas, even during a rollout: a code minted by one would
    # be unknown to the other.
    strategy {
      type = "Recreate"
    }

    selector {
      match_labels = local.langfuse_sso_labels
    }

    template {
      metadata {
        labels = local.langfuse_sso_labels
      }

      spec {
        automount_service_account_token = false

        security_context {
          run_as_non_root = true
          run_as_user     = 65532
          run_as_group    = 65532

          seccomp_profile {
            type = "RuntimeDefault"
          }
        }

        toleration {
          key      = "kubernetes.io/arch"
          operator = "Equal"
          value    = "amd64"
          effect   = "NoSchedule"
        }

        container {
          name  = each.key
          image = var.langfuse_sso_image

          dynamic "env" {
            for_each = local.langfuse_sso_env
            content {
              name  = env.key
              value = env.value
            }
          }

          # LANGFUSE_SSO_CLIENT_SECRET and LANGFUSE_SSO_SIGNING_KEY, synced
          # from Doppler (doppler.tf).
          env_from {
            secret_ref {
              name = local.langfuse_sso_name
            }
          }

          port {
            name           = "http"
            container_port = 3000
          }

          readiness_probe {
            http_get {
              path = "/langfuse-sso/.well-known/openid-configuration"
              port = 3000
            }
            period_seconds = 10
          }

          security_context {
            allow_privilege_escalation = false
            privileged                 = false
            read_only_root_filesystem  = true
            run_as_non_root            = true

            capabilities {
              add  = []
              drop = ["ALL"]
            }
          }

          # Autopilot bills requests and sets limits to match.
          resources {
            requests = { cpu = "250m", memory = "256Mi", ephemeral-storage = "1Gi" }
            limits   = { cpu = "250m", memory = "256Mi", ephemeral-storage = "1Gi" }
          }
        }
      }
    }
  }

  lifecycle {
    ignore_changes = [
      metadata[0].annotations["autopilot.gke.io/resource-adjustment"],
      metadata[0].annotations["autopilot.gke.io/warden-version"],
    ]
  }

  depends_on = [helm_release.doppler_secrets]
}

resource "kubernetes_service_v1" "langfuse_sso" {
  for_each = kubernetes_deployment_v1.langfuse_sso

  metadata {
    name      = each.key
    namespace = kubernetes_namespace.observability.metadata[0].name
  }

  spec {
    type     = "ClusterIP"
    selector = local.langfuse_sso_labels

    port {
      name        = "http"
      port        = 3000
      target_port = 3000
    }
  }

  lifecycle {
    ignore_changes = [metadata[0].annotations["cloud.google.com/neg"]]
  }
}
