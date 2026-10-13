# The mock network id must satisfy the SQL provider's self-link validation,
# so the mock returns a real-shaped project-global network self-link.
mock_provider "google" {
  mock_resource "google_compute_network" {
    defaults = {
      id        = "projects/coda-health-staging/global/networks/coda-physician-runtime"
      self_link = "projects/coda-health-staging/global/networks/coda-physician-runtime"
    }
  }
}
mock_provider "google-beta" {}
mock_provider "kubernetes" {}

# The mock returns no Cloud SQL connection name; the socket URLs in the
# secret and the migration job assert against the real shape.
override_resource {
  target = google_sql_database_instance.physician
  values = { connection_name = "coda-health-staging:us-east1:coda-physician-runtime" }
}

run "the_site_stays_absent_until_its_image_and_origin_are_bound" {
  command = plan

  assert {
    condition     = length(kubernetes_deployment.physician_site) == 0 && length(kubernetes_service.physician_site) == 0
    error_message = "No site workload may exist while physician_site_enabled stays false."
  }
}

run "reject_a_site_without_an_exact_browser_origin" {
  command = plan

  variables {
    physician_site_enabled   = true
    physician_site_image     = "us-east1-docker.pkg.dev/coda-health-staging/physician-site/physician-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    physician_browser_origin = "https://physician.example.com/extra"
  }

  expect_failures = [var.physician_browser_origin]
}

run "the_enabled_site_serves_two_private_replicas_on_the_iam_database_url" {
  command = plan

  variables {
    sql_activation_policy    = "ALWAYS"
    physician_site_enabled   = true
    physician_site_image     = "us-east1-docker.pkg.dev/coda-health-staging/physician-site/physician-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    physician_browser_origin = "https://physician.example.com"
  }

  # Replicas, port, and connection shape are config-owned values asserted
  # through their locals: the mock provider regenerates computed secret
  # and replica fields, so those would assert the mock, not the config.
  assert {
    condition     = local.site_replicas == 2
    error_message = "Two replicas must serve so one pod's replacement never unseats the practice."
  }

  assert {
    condition     = kubernetes_deployment.physician_site[0].spec[0].template[0].spec[0].container[0].image == var.physician_site_image
    error_message = "The site must run exactly the digest-pinned image."
  }

  assert {
    condition     = strcontains(local.site_connection, "host=/cloudsql/") && strcontains(local.site_connection, local.database_name)
    error_message = "The database URL must reach Cloud SQL through its Unix socket mount, never a public address."
  }

  assert {
    condition     = kubernetes_secret.physician_database[0].metadata[0].name != "" && kubernetes_deployment.physician_site[0].spec[0].template[0].spec[0].container[0].env_from[0].secret_ref[0].name == kubernetes_secret.physician_database[0].metadata[0].name
    error_message = "The site must take its database and auth settings from the mounted secret."
  }

  assert {
    condition     = kubernetes_service.physician_site[0].spec[0].type == "ClusterIP"
    error_message = "The site stays cluster-private until an approved edge exists."
  }
}

run "the_migration_job_requires_a_digest_pinned_image" {
  command = plan

  variables {
    migration_enabled = true
  }

  expect_failures = [var.migration_enabled]
}

run "the_database_parks_outside_a_proof_window" {
  command = plan

  assert {
    condition     = google_sql_database_instance.physician.settings[0].activation_policy == "NEVER"
    error_message = "The database must stay parked until an approved window turns it on."
  }
}
