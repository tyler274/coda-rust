# Physician chat-turn queue. The physician site accepts a turn into Postgres
# and publishes only {scope, input_id}; no clinical text goes on these topics.
# Pub/Sub pushes each message to the internal physician-worker with an OIDC
# token minted for the dedicated physician-pubsub-push identity, the only
# principal allowed to invoke the worker. Everything here is keyed
# "physician" through physician_scale_apps; Duet has no topic, subscription or
# Pub/Sub grant.

# The Pub/Sub service agent (service-PROJECT_NUMBER@gcp-sa-pubsub...) signs
# push OIDC tokens and forwards dead letters.
resource "google_project_service_identity" "pubsub" {
  for_each   = local.physician_scale_apps
  provider   = google-beta
  project    = google_project.environment.project_id
  service    = "pubsub.googleapis.com"
  depends_on = [google_project_service.api]
}

resource "google_pubsub_topic" "chat_turns" {
  for_each   = local.physician_scale_apps
  project    = google_project.environment.project_id
  name       = "physician-chat-turns"
  labels     = { environment = var.environment, app = "physician", purpose = "chat-turns" }
  depends_on = [google_project_service.api]
}

resource "google_pubsub_topic" "chat_turns_dead_letter" {
  for_each   = local.physician_scale_apps
  project    = google_project.environment.project_id
  name       = "physician-chat-turns-dead-letter"
  labels     = { environment = var.environment, app = "physician", purpose = "chat-turns-dead-letter" }
  depends_on = [google_project_service.api]
}

# Messages published to a topic without a subscription are dropped, so the
# dead-letter topic keeps a pull subscription for operators to inspect and
# replay. Nothing consumes it automatically.
resource "google_pubsub_subscription" "chat_turns_dead_letter" {
  for_each                   = local.physician_scale_apps
  project                    = google_project.environment.project_id
  name                       = "physician-chat-turns-dead-letter"
  topic                      = google_pubsub_topic.chat_turns_dead_letter[each.key].id
  message_retention_duration = "604800s"
  ack_deadline_seconds       = 60
  labels                     = { environment = var.environment, app = "physician", purpose = "chat-turns-dead-letter" }
  # Idle subscriptions expire after 31 days by default; staging can be idle.
  expiration_policy { ttl = "" }
}

resource "google_service_account" "physician_pubsub_push" {
  for_each     = local.physician_scale_apps
  project      = google_project.environment.project_id
  account_id   = "physician-pubsub-push"
  display_name = "physician Pub/Sub push to physician-worker (${var.environment})"
  description  = "Identity in the OIDC token Pub/Sub attaches to chat-turn pushes. Holds only run.invoker on physician-worker."
  depends_on   = [google_project_service.api]
}

# Pub/Sub mints the push OIDC token through its service agent, which needs
# iam.serviceAccounts.getOpenIdToken on the push identity. Granted on this one
# account only, not project-wide:
# https://cloud.google.com/pubsub/docs/authenticate-push-subscriptions
resource "google_service_account_iam_member" "pubsub_push_token" {
  for_each           = local.physician_scale_apps
  service_account_id = google_service_account.physician_pubsub_push[each.key].name
  role               = "roles/iam.serviceAccountTokenCreator"
  member             = "serviceAccount:${google_project_service_identity.pubsub[each.key].email}"
}

# Only the physician site's runtime identity publishes turns.
resource "google_pubsub_topic_iam_binding" "chat_turns_publisher" {
  for_each = local.physician_scale_apps
  project  = google_project.environment.project_id
  topic    = google_pubsub_topic.chat_turns[each.key].name
  role     = "roles/pubsub.publisher"
  members  = ["serviceAccount:${google_service_account.app["physician-runtime"].email}"]
}

# Dead-lettering: the service agent publishes to the dead-letter topic and
# acknowledges the forwarded message on the source subscription.
resource "google_pubsub_topic_iam_binding" "chat_turns_dead_letter_publisher" {
  for_each = local.physician_scale_apps
  project  = google_project.environment.project_id
  topic    = google_pubsub_topic.chat_turns_dead_letter[each.key].name
  role     = "roles/pubsub.publisher"
  members  = ["serviceAccount:${google_project_service_identity.pubsub[each.key].email}"]
}

resource "google_pubsub_subscription_iam_binding" "chat_turns_dead_letter_subscriber" {
  for_each     = local.physician_scale_apps
  project      = google_project.environment.project_id
  subscription = google_pubsub_subscription.chat_turns[each.key].name
  role         = "roles/pubsub.subscriber"
  members      = ["serviceAccount:${google_project_service_identity.pubsub[each.key].email}"]
}

resource "google_pubsub_subscription" "chat_turns" {
  for_each = local.physician_scale_apps
  project  = google_project.environment.project_id
  name     = "physician-chat-turns-worker"
  topic    = google_pubsub_topic.chat_turns[each.key].id
  # For push, the ack deadline is the request deadline: the worker has up to
  # 600 s (its Cloud Run timeout) to run a turn and return 2xx. Non-2xx or a
  # timeout is a nack and is redelivered with backoff.
  ack_deadline_seconds = 600
  labels               = { environment = var.environment, app = "physician", purpose = "chat-turns" }
  push_config {
    # Pub/Sub counts as internal for Cloud Run ingress when it is in the same
    # project and calls the default run.app URL, so the worker can stay
    # INGRESS_TRAFFIC_INTERNAL_ONLY:
    # https://cloud.google.com/run/docs/securing/ingress#settings
    push_endpoint = "${google_cloud_run_v2_service.physician_worker[each.key].uri}/pubsub/chat-turn"
    # The wrapped (default) format carries messageId and deliveryAttempt,
    # which the worker logs and uses for telemetry.
    oidc_token {
      service_account_email = google_service_account.physician_pubsub_push[each.key].email
      audience              = local.physician_worker_audience
    }
  }
  retry_policy {
    minimum_backoff = "10s"
    maximum_backoff = "600s"
  }
  dead_letter_policy {
    dead_letter_topic     = google_pubsub_topic.chat_turns_dead_letter[each.key].id
    max_delivery_attempts = 5
  }
  expiration_policy { ttl = "" }
  depends_on = [
    google_pubsub_topic_iam_binding.chat_turns_dead_letter_publisher,
    google_service_account_iam_member.pubsub_push_token,
    google_cloud_run_v2_service_iam_binding.physician_worker_invoker,
  ]
}
