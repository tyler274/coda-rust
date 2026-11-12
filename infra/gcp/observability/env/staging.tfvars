# Nonsecret staging inputs. The gateway address and caller identities are
# not repeated here: they come from infra/gcp/rust-environments outputs as a
# second -var-file (see README.md, "Inputs from rust-environments").
environment      = "staging"
project_id       = "coda-rust-staging"
cluster_project  = "coda-rust-staging"
cluster_location = "us-east1"
cluster_name     = "rust-observability"
region           = "us-east1"

# Datadog and Grafana behind the gateway, each with its own exporter queue.
# Datadog stays the operational notifier; Grafana evaluates in shadow mode,
# including its native Cloud Monitoring PromQL rules. Datadog monitors, the
# Datadog GCP integration, Grafana test notifications and availability
# monitors keep their off defaults until each is separately approved.
telemetry_backend                = "both"
grafana_cloud_monitoring_enabled = true

# The Langfuse HIPAA cloud project (README.md, "Langfuse"). Before the saved
# plan is applied, the operator sets the project's LANGFUSE_PUBLIC_KEY and
# LANGFUSE_SECRET_KEY in Doppler coda-rust/stg.
langfuse_host = "https://hipaa.cloud.langfuse.com"

# The Datadog Agent scrapes the gateway's own exporter metrics, and the
# Langfuse exporter monitors page #engineering-observability. Their gate is
# separate from datadog_monitors_enabled.
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

# Grafana under https://staging.codahealth.ai/grafana/ (README.md, "Staging
# proxy"). observability_web is rendered from rust-environments'
# observability_relay output.
staging_proxy = {
  origin       = "https://staging.codahealth.ai"
  iap_audience = "/projects/134570912572/locations/us-east1/services/public-site-staging"
}
