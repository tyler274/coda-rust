resource "google_storage_bucket" "cache" {
  project                     = var.project_id
  name                        = var.bucket_name
  location                    = var.region
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false

  # NARs and narinfo must remain consistent: do not expire either by object age.
  # Garbage collection needs a separate, reference-aware operator procedure.
  labels = { purpose = "nix-binary-cache" }
}

resource "google_service_account" "reader" {
  project      = var.project_id
  account_id   = "nix-cache-reader"
  display_name = "Nix binary cache reader"
}

resource "google_service_account" "writer" {
  project      = var.project_id
  account_id   = "nix-cache-writer"
  display_name = "Nix binary cache publisher"
}

resource "google_storage_bucket_iam_member" "reader" {
  bucket = google_storage_bucket.cache.name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.reader.email}"
}

# Defined in the project, but granted only on this bucket. No deletion means no
# overwrite of existing objects in GCS. Nix checks for existing cache paths and
# writes new NAR/narinfo objects; racing uploads or key rotation that rewrites
# existing narinfo can fail closed. Do not grant delete to make those succeed.
# Use single-part uploads (explicit in cache_url); multipart upload requires
# additional storage.multipartUploads permissions that this role does not grant.
resource "google_project_iam_custom_role" "publisher" {
  project     = var.project_id
  role_id     = "nixCachePublisher"
  title       = "Nix binary cache immutable publisher"
  description = "Read and create cache objects without deleting or replacing existing objects."
  permissions = [
    "storage.objects.get",
    "storage.objects.list",
    "storage.objects.create",
  ]
}

resource "google_storage_bucket_iam_member" "writer" {
  bucket = google_storage_bucket.cache.name
  role   = google_project_iam_custom_role.publisher.name
  member = "serviceAccount:${google_service_account.writer.email}"
}

# Generate reader/writer HMAC credentials and the Nix signing key outside
# OpenTofu, and store them in Doppler, not state. HMAC is required by Nix's S3
# transport to GCS; WIF does not restrict possession/use of these credentials.
# Release the writer HMAC and signing key only to the main-only protected GitHub
# publishing environment. Readers receive only the separate read HMAC and the
# public Nix verification key. This root does not enforce GitHub environment
# policy, generate secrets, or grant either identity access to product resources.
