mock_provider "google" {}

variables {
  project_id                   = "public-site-test"
  image                        = "us-east1-docker.pkg.dev/public-site-test/site/web@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  release_sha                  = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  physician_app_url            = "https://physician.example.test"
  certificate_ids              = ["projects/public-site-test/global/sslCertificates/test"]
  doppler_token_secret_id      = "public-site-stg-doppler-token"
  doppler_token_secret_version = "1"
}

run "staging_origin_is_protected" {
  command = plan

  assert {
    condition     = google_cloud_run_v2_service.site.ingress == "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER" && google_cloud_run_v2_service.site.default_uri_disabled
    error_message = "Staging must not expose a bypass through run.app or unrestricted ingress."
  }
  assert {
    condition     = !google_compute_backend_service.site.enable_cdn && length(google_compute_backend_service.site.cdn_policy) == 0
    error_message = "No staging response may be served from Cloud CDN ahead of the Access gate."
  }
  assert {
    condition     = local.runtime_env.APP_ENV == "staging" && local.runtime_env.PUBLIC_SITE_LISTEN_ADDR == "0.0.0.0:8080"
    error_message = "Staging must activate the application Access gate and container binding."
  }
  assert {
    condition     = contains(google_cloud_run_v2_service.site.template[0].containers[0].args, "stg") && contains(google_cloud_run_v2_service.site.template[0].containers[0].args, "--preserve-env=PUBLIC_SITE_LISTEN_ADDR,PHYSICIAN_APP_URL,APP_ENV,RELEASE_SHA")
    error_message = "Doppler must use staging and must not override the environment that controls Access."
  }
  assert {
    condition     = google_cloud_run_v2_service.site.template[0].scaling[0].min_instance_count == 0 && google_cloud_run_v2_service.site.template[0].scaling[0].max_instance_count == 10 && google_cloud_run_v2_service.site.template[0].max_instance_request_concurrency == 1000
    error_message = "Scale-to-zero, bounded scale-out, and concurrency must retain the documented defaults."
  }
  assert {
    condition     = google_compute_backend_service.site.load_balancing_scheme == "EXTERNAL_MANAGED" && google_compute_region_network_endpoint_group.site.network_endpoint_type == "SERVERLESS"
    error_message = "Cloud Run requires a serverless NEG behind the global external ALB."
  }
  assert {
    condition     = one([for e in google_cloud_run_v2_service.site.template[0].containers[0].env : e.value_source[0].secret_key_ref[0].version if e.name == "DOPPLER_TOKEN"]) == "1"
    error_message = "Only the pinned Doppler service token is injected from Secret Manager."
  }
}

run "production_respects_origin_cache_headers" {
  command = plan
  variables {
    environment = "production"
  }
  assert {
    condition     = google_compute_backend_service.site.enable_cdn && google_compute_backend_service.site.cdn_policy[0].cache_mode == "USE_ORIGIN_HEADERS" && !google_compute_backend_service.site.cdn_policy[0].negative_caching
    error_message = "Production CDN must respect origin cache headers without negative caching."
  }
  assert {
    condition     = local.runtime_env.APP_ENV == "production" && contains(google_cloud_run_v2_service.site.template[0].containers[0].args, "prd")
    error_message = "Production must select the production runtime and Doppler configuration."
  }
}

run "reject_mutable_image" {
  command = plan
  variables {
    image = "example.test/site:latest"
  }
  expect_failures = [var.image]
}

run "reject_unknown_environment" {
  command = plan
  variables {
    environment = "stagng"
  }
  expect_failures = [var.environment]
}

run "reject_excessive_concurrency" {
  command = plan
  variables {
    concurrency = 1001
  }
  expect_failures = [var.concurrency]
}

run "reject_inverted_scaling" {
  command = plan
  variables {
    min_instances = 11
    max_instances = 10
  }
  expect_failures = [google_cloud_run_v2_service.site]
}

run "reject_unpinned_secret" {
  command = plan
  variables {
    doppler_token_secret_version = "latest"
  }
  expect_failures = [var.doppler_token_secret_version]
}
