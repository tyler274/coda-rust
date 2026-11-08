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

# Enabling Langfuse requires its Cloud SQL IAM grants and Doppler secrets; the
# gateway drops generation copies while off.
langfuse_enabled = false

# Slack #engineering-observability through Datadog's Coda_Health Slack account.
# Monitor creation has a separate datadog_monitors_enabled authorization gate.
alert_notify = "@slack-Coda_Health-engineering-observability"

# Grafana at https://grafana.staging.codahealth.ai behind IAP (README.md,
# "Grafana edge rollout"). Phase 1 grants nobody; members, backend_service
# and backend_service_id are added in phase 2.
grafana_edge = {
  hostname = "grafana.staging.codahealth.ai"
}
