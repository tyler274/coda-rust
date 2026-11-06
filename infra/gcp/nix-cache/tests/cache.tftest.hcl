mock_provider "google" {}

variables {
  project_id  = "example-nix-cache"
  region      = "us-central1"
  bucket_name = "example-private-nix-cache"
}

override_resource {
  target = google_service_account.reader
  values = { email = "nix-cache-reader@example-nix-cache.iam.gserviceaccount.com" }
}

override_resource {
  target = google_service_account.writer
  values = { email = "nix-cache-writer@example-nix-cache.iam.gserviceaccount.com" }
}

override_resource {
  target = google_project_iam_custom_role.publisher
  values = { name = "projects/example-nix-cache/roles/nixCachePublisher" }
}

run "private_bucket_and_scoped_identities" {
  command = plan

  assert {
    condition     = google_storage_bucket.cache.project == var.project_id && google_storage_bucket.cache.name == var.bucket_name && lower(google_storage_bucket.cache.location) == var.region
    error_message = "The bucket must use the explicitly selected project, name and region."
  }
  assert {
    condition     = google_storage_bucket.cache.uniform_bucket_level_access && google_storage_bucket.cache.public_access_prevention == "enforced"
    error_message = "The cache must forbid public access and object ACL bypasses."
  }
  assert {
    condition     = !google_storage_bucket.cache.force_destroy && length(google_storage_bucket.cache.lifecycle_rule) == 0
    error_message = "The root must neither purge a nonempty cache on destroy nor expire objects independently of NAR references."
  }
  assert {
    condition     = google_service_account.reader.project == var.project_id && google_service_account.writer.project == var.project_id && google_service_account.reader.account_id != google_service_account.writer.account_id
    error_message = "Read and publish access must use separate cache-project service accounts."
  }
  assert {
    condition     = google_storage_bucket_iam_member.reader.bucket == var.bucket_name && google_storage_bucket_iam_member.reader.role == "roles/storage.objectViewer" && google_storage_bucket_iam_member.reader.member == "serviceAccount:${google_service_account.reader.email}"
    error_message = "Read access must be limited to the reader identity and cache bucket."
  }
  assert {
    condition     = google_project_iam_custom_role.publisher.project == var.project_id && toset(google_project_iam_custom_role.publisher.permissions) == toset(["storage.objects.get", "storage.objects.list", "storage.objects.create"])
    error_message = "Publishing may read/list/create only, never delete, overwrite, administer IAM or access product APIs."
  }
  assert {
    condition     = google_storage_bucket_iam_member.writer.bucket == var.bucket_name && google_storage_bucket_iam_member.writer.role == google_project_iam_custom_role.publisher.name && google_storage_bucket_iam_member.writer.member == "serviceAccount:${google_service_account.writer.email}" && google_storage_bucket_iam_member.writer.member != google_storage_bucket_iam_member.reader.member
    error_message = "The custom publisher role must be granted only to the writer on the cache bucket."
  }
  assert {
    condition     = output.cache_url == "s3://${var.bucket_name}?endpoint=storage.googleapis.com&region=auto&multipart-upload=false" && output.bucket_name == var.bucket_name && output.reader_service_account == google_service_account.reader.email && output.writer_service_account == google_service_account.writer.email
    error_message = "Outputs must expose only integration coordinates, with distinct reader/publisher identities and the GCS S3 endpoint."
  }
}

run "explicit_location_is_respected" {
  command = plan
  variables {
    project_id  = "another-cache-project"
    region      = "europe-west1"
    bucket_name = "another-private-cache"
  }
  assert {
    condition     = google_storage_bucket.cache.project == var.project_id && lower(google_storage_bucket.cache.location) == var.region && google_storage_bucket.cache.name == var.bucket_name && google_service_account.reader.project == var.project_id && google_service_account.writer.project == var.project_id && google_project_iam_custom_role.publisher.project == var.project_id
    error_message = "The root must not pin any live project or region."
  }
}

run "reject_invalid_project" {
  command = plan
  variables { project_id = "" }
  expect_failures = [var.project_id]
}

run "reject_multiregion" {
  command = plan
  variables { region = "US" }
  expect_failures = [var.region]
}

run "reject_zonal_location" {
  command = plan
  variables { region = "us-central1-a" }
  expect_failures = [var.region]
}

run "reject_invalid_bucket" {
  command = plan
  variables { bucket_name = "https://not-a-bucket" }
  expect_failures = [var.bucket_name]
}
