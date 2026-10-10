output "staging_project_id" {
  value = google_project.staging.project_id
}

output "staging_project_number" {
  value = google_project.staging.number
}

output "terraform_state_bucket" {
  value = google_storage_bucket.terraform_state.name
}

output "clinical_corpus_bucket" {
  value = google_storage_bucket.clinical_corpus.name
}
