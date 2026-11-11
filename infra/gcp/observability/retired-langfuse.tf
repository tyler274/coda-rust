# The self-hosted Langfuse stores hold staging traces from before Langfuse
# moved to its HIPAA cloud. Cloud SQL carries deletion protection and the
# bucket refuses to delete with objects in it, so this root forgets them
# instead of destroying them. Deleting their data is a separate, explicitly
# authorized operation (README.md, "Retired self-hosted stores").
removed {
  from = google_sql_database_instance.langfuse

  lifecycle {
    destroy = false
  }
}

removed {
  from = google_sql_database.langfuse

  lifecycle {
    destroy = false
  }
}

removed {
  from = google_sql_user.langfuse

  lifecycle {
    destroy = false
  }
}

removed {
  from = google_storage_bucket.langfuse

  lifecycle {
    destroy = false
  }
}

# The workload and blob accounts outlive the stores with them: the bucket's
# HMAC key belongs to `coda-langfuse-blobs`, and rolling back to the
# self-hosted Langfuse needs both.
removed {
  from = google_service_account.langfuse

  lifecycle {
    destroy = false
  }
}

removed {
  from = google_service_account.langfuse_blobs

  lifecycle {
    destroy = false
  }
}
