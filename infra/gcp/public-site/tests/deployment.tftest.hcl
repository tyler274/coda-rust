mock_provider "google" {}

variables {
  project_id                   = "coda-health-staging"
  project_number               = "134570912572"
  image                        = "us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  release_sha                  = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  physician_app_url            = "https://physician.example.test"
  doppler_token_secret_id      = "public-site-stg-doppler-token"
  doppler_token_secret_version = "1"
}

run "staging_origin_is_protected" {
  command = plan

  assert {
    condition     = google_cloud_run_v2_service.site.ingress == "INGRESS_TRAFFIC_ALL" && !google_cloud_run_v2_service.site.default_uri_disabled && google_cloud_run_v2_service.site.iap_enabled && !google_cloud_run_v2_service.site.invoker_iam_disabled
    error_message = "Staging must expose run.app only behind native IAP with invoker IAM checks enabled."
  }
  assert {
    condition     = length(google_cloud_run_v2_service_iam_member.load_balancer) == 0 && length(google_cloud_run_v2_service_iam_binding.iap_invoker) == 1 && google_cloud_run_v2_service_iam_binding.iap_invoker[0].role == "roles/run.invoker" && google_cloud_run_v2_service_iam_binding.iap_invoker[0].members == toset(["serviceAccount:service-134570912572@gcp-sa-iap.iam.gserviceaccount.com"])
    error_message = "Only the existing project's IAP service agent may invoke staging; never allUsers or CI."
  }
  assert {
    condition     = google_cloud_run_v2_service_iam_binding.iap_invoker[0].project == var.project_id && google_cloud_run_v2_service_iam_binding.iap_invoker[0].location == "us-east1" && google_cloud_run_v2_service_iam_binding.iap_invoker[0].name == google_cloud_run_v2_service.site.name
    error_message = "IAP invocation must be scoped to this Cloud Run service, not the project."
  }
  assert {
    condition     = length(google_iap_web_cloud_run_service_iam_binding.team) == 1 && google_iap_web_cloud_run_service_iam_binding.team[0].role == "roles/iap.httpsResourceAccessor" && google_iap_web_cloud_run_service_iam_binding.team[0].members == toset(["group:gcp-engineering@codahealth.ai"])
    error_message = "The authoritative staging accessor binding must contain only the engineering Google Group."
  }
  assert {
    condition     = google_iap_web_cloud_run_service_iam_binding.team[0].project == var.project_id && google_iap_web_cloud_run_service_iam_binding.team[0].location == "us-east1" && google_iap_web_cloud_run_service_iam_binding.team[0].cloud_run_service_name == google_cloud_run_v2_service.site.name
    error_message = "IAP access must be scoped to this Cloud Run service, not the project or another service."
  }
  assert {
    condition     = length(google_compute_backend_service.site) == 0 && length(google_compute_region_network_endpoint_group.site) == 0 && length(google_compute_url_map.site) == 0 && length(google_compute_ssl_policy.site) == 0 && length(google_compute_target_https_proxy.site) == 0 && length(google_compute_global_address.site) == 0 && length(google_compute_global_forwarding_rule.https) == 0
    error_message = "Staging must not provision any ALB, NEG, CDN, IP or TLS frontend."
  }
  assert {
    condition     = output.load_balancer_ip == null && !output.cdn_enabled && length(var.certificate_ids) == 0
    error_message = "Staging must need no external certificate, reserved address or CDN."
  }
  assert {
    condition     = local.runtime_env.APP_ENV == "staging" && local.runtime_env.PUBLIC_SITE_LISTEN_ADDR == "0.0.0.0:8080" && local.runtime_env.PUBLIC_SITE_IAP_AUDIENCE == "/projects/134570912572/locations/us-east1/services/public-site-staging" && output.iap_audience == local.runtime_env.PUBLIC_SITE_IAP_AUDIENCE
    error_message = "Staging must activate assertion verification with the exact direct Cloud Run audience."
  }
  assert {
    condition     = one([for e in google_cloud_run_v2_service.site.template[0].containers[0].env : e.value if e.name == "PUBLIC_SITE_IAP_AUDIENCE"]) == output.iap_audience
    error_message = "The expected IAP assertion audience must reach the deployed container."
  }
  assert {
    condition     = contains(google_cloud_run_v2_service.site.template[0].containers[0].args, "stg") && contains(google_cloud_run_v2_service.site.template[0].containers[0].args, "--preserve-env=PUBLIC_SITE_LISTEN_ADDR,PHYSICIAN_APP_URL,APP_ENV,RELEASE_SHA,PUBLIC_SITE_IAP_AUDIENCE")
    error_message = "Doppler must preserve the deployment-controlled IAP audience and environment."
  }
  assert {
    condition     = google_cloud_run_v2_service.site.template[0].scaling[0].min_instance_count == 0 && google_cloud_run_v2_service.site.template[0].scaling[0].max_instance_count == 10 && google_cloud_run_v2_service.site.template[0].max_instance_request_concurrency == 1000
    error_message = "Scale-to-zero, bounded scale-out, and concurrency must retain the documented defaults."
  }
  assert {
    condition     = google_cloud_run_v2_service.site.template[0].containers[0].startup_probe[0].tcp_socket[0].port == 8080 && length(google_cloud_run_v2_service.site.template[0].containers[0].startup_probe[0].http_get) == 0
    error_message = "Startup must retain the TCP 8080 probe, not an unauthenticated HTTP bypass."
  }
  assert {
    condition     = one([for e in google_cloud_run_v2_service.site.template[0].containers[0].env : e.value_source[0].secret_key_ref[0].version if e.name == "DOPPLER_TOKEN"]) == "1"
    error_message = "Only the pinned Doppler service token is injected from Secret Manager."
  }
}

# This is an in-memory mocked apply, never a live GCP deployment.
run "staging_url_is_protected_service_uri" {
  command = apply
  override_resource {
    target = google_cloud_run_v2_service.site
    values = { uri = "https://public-site-staging.example.run.app" }
  }
  assert {
    condition     = output.staging_url == "https://public-site-staging.example.run.app"
    error_message = "The preview URL must be the native-IAP-protected Cloud Run URI."
  }
}

run "production_respects_origin_cache_headers" {
  command = plan
  variables {
    environment     = "production"
    certificate_ids = ["projects/public-site-test/global/sslCertificates/test"]
  }
  assert {
    condition     = google_cloud_run_v2_service.site.ingress == "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER" && google_cloud_run_v2_service.site.default_uri_disabled && !google_cloud_run_v2_service.site.iap_enabled && !google_cloud_run_v2_service.site.invoker_iam_disabled
    error_message = "Production must retain ALB-only ingress and its disabled run.app endpoint without IAP."
  }
  assert {
    condition     = length(google_cloud_run_v2_service_iam_binding.iap_invoker) == 0 && length(google_iap_web_cloud_run_service_iam_binding.team) == 0 && google_cloud_run_v2_service_iam_member.load_balancer[0].role == "roles/run.invoker" && google_cloud_run_v2_service_iam_member.load_balancer[0].member == "allUsers"
    error_message = "Production must retain its public ALB invoker grant and have no staging IAP bindings."
  }
  assert {
    condition     = google_compute_backend_service.site[0].enable_cdn && google_compute_backend_service.site[0].cdn_policy[0].cache_mode == "USE_ORIGIN_HEADERS" && !google_compute_backend_service.site[0].cdn_policy[0].negative_caching && output.cdn_enabled
    error_message = "Production CDN must respect origin cache headers without negative caching."
  }
  assert {
    condition     = google_compute_backend_service.site[0].load_balancing_scheme == "EXTERNAL_MANAGED" && google_compute_region_network_endpoint_group.site[0].network_endpoint_type == "SERVERLESS" && google_compute_region_network_endpoint_group.site[0].cloud_run[0].service == google_cloud_run_v2_service.site.name
    error_message = "Production Cloud Run requires a serverless NEG behind the global external ALB."
  }
  assert {
    condition     = length(google_compute_url_map.site) == 1 && length(google_compute_global_address.site) == 1 && google_compute_global_forwarding_rule.https[0].port_range == "443" && google_compute_global_forwarding_rule.https[0].load_balancing_scheme == "EXTERNAL_MANAGED" && google_compute_global_forwarding_rule.https[0].network_tier == "PREMIUM" && google_compute_ssl_policy.site[0].min_tls_version == "TLS_1_2" && google_compute_ssl_policy.site[0].profile == "MODERN" && google_compute_target_https_proxy.site[0].ssl_certificates == tolist(var.certificate_ids)
    error_message = "Production must retain its reserved IP and HTTPS/TLS frontend."
  }
  assert {
    condition     = local.runtime_env.APP_ENV == "production" && !contains(keys(local.runtime_env), "PUBLIC_SITE_IAP_AUDIENCE") && contains(google_cloud_run_v2_service.site.template[0].containers[0].args, "prd") && contains(google_cloud_run_v2_service.site.template[0].containers[0].args, "--preserve-env=PUBLIC_SITE_LISTEN_ADDR,PHYSICIAN_APP_URL,APP_ENV,RELEASE_SHA") && output.iap_audience == null && output.staging_url == null
    error_message = "Production must retain its runtime and Doppler configuration without a staging IAP audience or preview URL."
  }
}

run "audience_tracks_service_inputs" {
  command = plan
  variables {
    project_id     = "another-existing-project"
    project_number = "123456789012"
    region         = "us-west1"
    name           = "preview"
    iap_group      = "preview-team@example.test"
    image          = "us-west1-docker.pkg.dev/another-existing-project/public-site/public-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  assert {
    condition     = output.iap_audience == "/projects/123456789012/locations/us-west1/services/preview-staging" && google_cloud_run_v2_service_iam_binding.iap_invoker[0].members == toset(["serviceAccount:service-123456789012@gcp-sa-iap.iam.gserviceaccount.com"]) && google_iap_web_cloud_run_service_iam_binding.team[0].members == toset(["group:preview-team@example.test"])
    error_message = "Audience, service agent and group must follow the existing-project inputs, not hardcoded resource identities."
  }
}

run "reject_production_without_certificate" {
  command = plan
  variables {
    environment = "production"
  }
  expect_failures = [google_compute_target_https_proxy.site]
}

run "reject_project_id_as_project_number" {
  command = plan
  variables {
    project_number = "coda-health-staging"
  }
  expect_failures = [var.project_number]
}

run "reject_zero_project_number" {
  command = plan
  variables {
    project_number = "0"
  }
  expect_failures = [var.project_number]
}

run "reject_domain_access" {
  command = plan
  variables {
    iap_group = "codahealth.ai"
  }
  expect_failures = [var.iap_group]
}

run "reject_non_group_principal" {
  command = plan
  variables {
    iap_group = "serviceAccount:ci@coda-health-staging.iam.gserviceaccount.com"
  }
  expect_failures = [var.iap_group]
}

run "reject_mutable_image" {
  command = plan
  variables {
    image = "example.test/site:latest"
  }
  expect_failures = [var.image]
}

run "staging_image_comes_from_private_repository" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site:b18760ef98a82e68aef29ac7a20bc48796794bf9@sha256:a179ab017b9b47179b0f9e71e7237242b9d454d8e27d98b2eb671bbbfd263140"
  }
  assert {
    condition     = startswith(google_cloud_run_v2_service.site.template[0].containers[0].image, "us-east1-docker.pkg.dev/coda-health-staging/public-site/") && endswith(google_cloud_run_v2_service.site.template[0].containers[0].image, "@sha256:a179ab017b9b47179b0f9e71e7237242b9d454d8e27d98b2eb671bbbfd263140")
    error_message = "Staging must deploy the digest-pinned image from its private public-site repository."
  }
}

run "reject_shared_coda_agents_repository" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev/coda-health-staging/coda-agents/public-site@sha256:a179ab017b9b47179b0f9e71e7237242b9d454d8e27d98b2eb671bbbfd263140"
  }
  expect_failures = [var.image]
}

run "reject_public_docker_hub_image" {
  command = plan
  variables {
    image = "docker.io/library/nginx@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  expect_failures = [var.image]
}

run "reject_public_gcr_image" {
  command = plan
  variables {
    image = "gcr.io/coda-health-staging/public-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  expect_failures = [var.image]
}

run "reject_base_image_cache_repository" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev/coda-health-staging/public-site-dockerhub/public-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  expect_failures = [var.image]
}

run "reject_other_project_repository" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev/coda-health-prod/public-site/public-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  expect_failures = [var.image]
}

run "reject_lookalike_registry_host" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev.example.test/coda-health-staging/public-site/public-site@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  expect_failures = [var.image]
}

run "reject_private_repository_tag_without_digest" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site:b18760ef98a82e68aef29ac7a20bc48796794bf9"
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
