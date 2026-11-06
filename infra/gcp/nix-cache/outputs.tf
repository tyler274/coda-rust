output "bucket_name" {
  description = "Private cache bucket; it is not the remote state bucket."
  value       = google_storage_bucket.cache.name
}

output "reader_service_account" {
  description = "Identity for read-only HMAC credentials generated outside OpenTofu."
  value       = google_service_account.reader.email
}

output "writer_service_account" {
  description = "Identity for publisher HMAC credentials generated outside OpenTofu."
  value       = google_service_account.writer.email
}

output "cache_url" {
  description = "Nix S3 cache URL; use separate AWS credential profiles. Single-part uploads preserve the minimal publisher permissions."
  value       = "s3://${google_storage_bucket.cache.name}?endpoint=storage.googleapis.com&region=auto&multipart-upload=false"
}
