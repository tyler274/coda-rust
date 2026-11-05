# Secrets flow: Doppler holds every value. Secret Manager holds only
# DOPPLER_TOKEN, a read-only service token scoped to the coda-rust stg or prd
# config. Tofu reads that one value ephemerally and writes it into the
# operator's token Secret as a write-only attribute, so it never lands in
# state or plan files. The Doppler Kubernetes Operator then syncs the named
# keys into the Kubernetes Secrets the workloads mount.
locals {
  doppler_namespace    = "doppler-operator-system"
  doppler_token_secret = "doppler-token-observability"

  # Managed Secret name => Doppler keys it carries, renamed where the
  # consumer expects a different key.
  doppler_secrets = merge(
    {
      # Runtime ingestion gets only the API key. Management credentials
      # stay with the operator running the Datadog provider.
      "datadog-secret" = {
        "DD_API_KEY" = "api-key"
      }
    },
    # Without Langfuse, none of its keys need to exist in Doppler.
    var.langfuse_enabled ? {
      # Referenced by key from the Langfuse chart values (secretKeyRef and
      # existingSecret); names stay as Doppler holds them. The HMAC pair is
      # minted by the operator for the Langfuse service account and stored
      # in Doppler, so it never passes through tofu state.
      "langfuse-secrets" = {
        "LANGFUSE_SALT"                 = "LANGFUSE_SALT"
        "LANGFUSE_NEXTAUTH_SECRET"      = "LANGFUSE_NEXTAUTH_SECRET"
        "LANGFUSE_ENCRYPTION_KEY"       = "LANGFUSE_ENCRYPTION_KEY"
        "LANGFUSE_CLICKHOUSE_PASSWORD"  = "LANGFUSE_CLICKHOUSE_PASSWORD"
        "LANGFUSE_REDIS_PASSWORD"       = "LANGFUSE_REDIS_PASSWORD"
        "LANGFUSE_S3_ACCESS_KEY_ID"     = "LANGFUSE_S3_ACCESS_KEY_ID"
        "LANGFUSE_S3_SECRET_ACCESS_KEY" = "LANGFUSE_S3_SECRET_ACCESS_KEY"
      }
      # Injected as environment (envFrom) into Langfuse web: headless init of
      # the org, the agents project, its ingestion keys, and the first admin.
      "langfuse-init" = {
        "LANGFUSE_INIT_PROJECT_PUBLIC_KEY" = "LANGFUSE_INIT_PROJECT_PUBLIC_KEY"
        "LANGFUSE_INIT_PROJECT_SECRET_KEY" = "LANGFUSE_INIT_PROJECT_SECRET_KEY"
        "LANGFUSE_INIT_USER_EMAIL"         = "LANGFUSE_INIT_USER_EMAIL"
        "LANGFUSE_INIT_USER_PASSWORD"      = "LANGFUSE_INIT_USER_PASSWORD"
      }
      # The collector authenticates to Langfuse with the same project keys.
      "otel-collector-langfuse" = {
        "LANGFUSE_INIT_PROJECT_PUBLIC_KEY" = "LANGFUSE_PUBLIC_KEY"
        "LANGFUSE_INIT_PROJECT_SECRET_KEY" = "LANGFUSE_SECRET_KEY"
      }
    } : {},
    var.langfuse_iap_enabled ? {
      # GCPBackendPolicy reads the IAP OAuth client secret from key `key`.
      "langfuse-iap-oauth" = {
        "LANGFUSE_IAP_CLIENT_SECRET" = "key"
      }
    } : {},
  )
}

ephemeral "google_secret_manager_secret_version" "doppler_token" {
  project = var.project_id
  secret  = var.doppler_token_secret_id
  version = var.doppler_token_secret_version
}

# The chart renders its own doppler-operator-system Namespace and hard-codes
# it on every resource, so Helm must not create it too. The release record
# lives in the observability namespace instead.
resource "helm_release" "doppler_operator" {
  name       = "doppler-kubernetes-operator"
  repository = "https://helm.doppler.com"
  chart      = "doppler-kubernetes-operator"
  version    = "1.7.3"
  namespace  = kubernetes_namespace.observability.metadata[0].name
  atomic     = true
}

resource "kubernetes_secret_v1" "doppler_token" {
  metadata {
    name      = local.doppler_token_secret
    namespace = local.doppler_namespace
  }

  data_wo = {
    serviceToken = ephemeral.google_secret_manager_secret_version.doppler_token.secret_data
  }
  # A new Secret Manager version is the only way to rotate the token here.
  data_wo_revision = tonumber(var.doppler_token_secret_version)

  depends_on = [helm_release.doppler_operator]
}

resource "helm_release" "doppler_secrets" {
  name      = "observability-doppler-secrets"
  chart     = local.manifests_chart
  namespace = local.doppler_namespace
  atomic    = true

  values = [yamlencode({
    manifests = [for name, key_map in local.doppler_secrets : {
      apiVersion = "secrets.doppler.com/v1alpha1"
      kind       = "DopplerSecret"
      metadata = {
        name      = name
        namespace = local.doppler_namespace
      }
      spec = {
        tokenSecret = { name = local.doppler_token_secret }
        project     = "coda-rust"
        config      = local.doppler_config
        secrets     = keys(key_map)
        processors  = { for doppler_key, as_name in key_map : doppler_key => { asName = as_name } }
        managedSecret = {
          name      = name
          namespace = kubernetes_namespace.observability.metadata[0].name
          type      = "Opaque"
        }
        resyncSeconds = 300
      }
    }]
  })]

  depends_on = [
    helm_release.doppler_operator,
    kubernetes_secret_v1.doppler_token,
  ]
}
