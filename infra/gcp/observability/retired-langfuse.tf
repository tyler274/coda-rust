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
