# Nonsecret staging inputs. The gateway address and caller identities are
# not repeated here: they come from infra/gcp/rust-environments outputs as a
# second -var-file (see README.md, "Inputs from rust-environments").
environment      = "staging"
project_id       = "coda-rust-staging"
cluster_project  = "coda-rust-staging"
cluster_location = "us-east1"
cluster_name     = "rust-observability"
network_name     = "rust-private"
region           = "us-east1"

# Datadog and Grafana behind the gateway, each with its own exporter queue.
# Datadog stays the operational notifier; Grafana evaluates in shadow mode,
# including its native Cloud Monitoring PromQL rules. Datadog monitors, the
# Datadog GCP integration, Grafana test notifications and availability
# monitors keep their off defaults until each is separately approved.
telemetry_backend                = "both"
grafana_cloud_monitoring_enabled = true

# Langfuse (README.md, "Secrets"). Before the saved plan is applied, the
# operator seeds the 11 LANGFUSE_* keys in Doppler coda-rust/stg, and the apply
# identity needs resourcemanager.projects.setIamPolicy for the Cloud SQL grants.
# The S3 HMAC pair is minted with `gcloud storage hmac create` for
# coda-langfuse-blobs, which this root creates.
langfuse_enabled = true

# Langfuse Enterprise features for a testing trial (README.md, "Enterprise
# trial"). Doppler coda-rust/stg holds LANGFUSE_EE_LICENSE_KEY as a
# langfuse_ee_ placeholder, which ee/LICENSE allows for testing only.
langfuse_ee_trial = true

# The Datadog Agent scrapes the gateway's own exporter metrics, and the
# Langfuse exporter and workload monitors page #engineering-observability.
# Their gate is separate from datadog_monitors_enabled.
collector_self_metrics_enabled = true
langfuse_monitors_enabled      = true

# The physician chat generation failure monitor pages
# #engineering-observability on its own gate, without the other service
# monitors. Its threshold stays the shared default (Grafana's rules.yaml
# holds the same number), and `observability-drill langfuse chat-failure`
# sends one more marked rehearsal turn than it.
chat_failure_monitor_enabled = true

# Services attach prompt and completion text to generation spans. Only the
# Langfuse pipeline keeps it; the ops pipelines strip it. Must match
# infra/gcp/rust-environments' telemetry_content.
telemetry_content = "full"

# Slack #engineering-observability through Datadog's Coda_Health Slack account.
# Monitor creation has a separate datadog_monitors_enabled authorization gate.
alert_notify = "@slack-Coda_Health-engineering-observability"

# Grafana and Langfuse under https://staging.codahealth.ai/grafana/ and
# /langfuse/ (README.md, "Staging proxy rollout"). observability_web is
# rendered from rust-environments' observability_relay output.
staging_proxy = {
  origin       = "https://staging.codahealth.ai"
  iap_audience = "/projects/134570912572/locations/us-east1/services/public-site-staging"
}
# The NEXT_PUBLIC_BASE_PATH=/langfuse build of v3.225.11 (langfuse-web-image.yml).
langfuse_web_image = "us-east1-docker.pkg.dev/coda-rust-staging/apps/langfuse-web@sha256:41e4b17935ccfff07d55b7ee87b908548736e1544d0f304ac785f2fddc95e884"
# The coda-langfuse-sso build of 9fd184d (main), published by the staging release.
langfuse_sso_image = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-langfuse-sso@sha256:fb26c1817eef061b1f463905ff6f0f6021e226a6904285dc7512cd6710ec63fe"
