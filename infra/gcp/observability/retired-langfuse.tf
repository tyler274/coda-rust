# Destroy the retired self-hosted Langfuse stores and identities in staging.
# Cloud SQL deletion protection must be disabled before destruction; the
# bucket must be emptied before deletion. Review and authorize those data
# deletions along with the saved destruction plan (README.md).
removed {
  from = google_sql_database_instance.langfuse

  lifecycle {
    destroy = true
  }
}

removed {
  from = google_sql_database.langfuse

  lifecycle {
    destroy = true
  }
}

removed {
  from = google_sql_user.langfuse

  lifecycle {
    destroy = true
  }
}

removed {
  from = google_storage_bucket.langfuse

  lifecycle {
    destroy = true
  }
}

# Retire both identities with the stores they served.
removed {
  from = google_service_account.langfuse

  lifecycle {
    destroy = true
  }
}

removed {
  from = google_service_account.langfuse_blobs

  lifecycle {
    destroy = true
  }
}
