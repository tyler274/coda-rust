variable "project_id" {
  description = "Production project that owns the canonical Clinical Evidence subsystem."
  type        = string
  default     = "coda-health-prod"
}

variable "region" {
  description = "Region for the Clinical Evidence database, images, and operational jobs."
  type        = string
  default     = "us-east1"
}

variable "gpu_region" {
  description = "Nearest Cloud Run region supporting NVIDIA L4 for Clinical Evidence serving."
  type        = string
  default     = "us-east4"
}

variable "clinical_corpus_bucket_name" {
  description = "Canonical private public-literature bucket."
  type        = string
  default     = "coda-health-prod-clinical-corpus-us-east1"
}

variable "network_cidr" {
  description = "Direct VPC egress subnet for Clinical Evidence operational jobs."
  type        = string
  default     = "10.34.0.0/26"
}

variable "gpu_network_cidr" {
  description = "Direct VPC egress subnet for the L4 Clinical Evidence service."
  type        = string
  default     = "10.34.0.64/26"
}

variable "serving_network_cidr" {
  description = "Subnet for the always-on L4 serving replicas and their internal load balancer address."
  type        = string
  default     = "10.34.0.128/26"
}

variable "serving_proxy_network_cidr" {
  description = "Proxy-only subnet Google runs the internal load balancer's proxies in."
  type        = string
  default     = "10.34.1.0/24"
}

variable "sql_tier" {
  description = "Cloud SQL machine tier used during explicit serving or proof windows."
  type        = string
  default     = "db-custom-1-3840"
}

variable "sql_disk_autoresize_limit_gb" {
  description = "Hard Cloud SQL storage ceiling. Increase only after a measured corpus-capacity review."
  type        = number
  default     = 50

  validation {
    condition     = var.sql_disk_autoresize_limit_gb >= 20 && var.sql_disk_autoresize_limit_gb <= 500
    error_message = "sql_disk_autoresize_limit_gb must remain between 20 GB and 500 GB."
  }
}

variable "sql_activation_policy" {
  description = "ALWAYS during initialization or a named serving window; NEVER when parked."
  type        = string
  default     = "NEVER"

  validation {
    condition     = contains(["ALWAYS", "NEVER"], var.sql_activation_policy)
    error_message = "sql_activation_policy must be ALWAYS or NEVER."
  }
}

variable "clinical_retrieval_build_enabled" {
  description = "Creates the explicit scale-to-zero retrieval projection and index build job."
  type        = bool
  default     = false

  validation {
    condition = !var.clinical_retrieval_build_enabled || (
      var.clinical_retrieval_build_image != null &&
      var.clinical_document_model_archive != null &&
      var.clinical_query_model_archive != null &&
      var.clinical_cross_encoder_archive != null
    )
    error_message = "The retrieval build job requires an immutable image and exact article, query, and cross-encoder archives."
  }
}

variable "clinical_retrieval_build_image" {
  description = "Exact CUDA builder image. Compatible embeddings can be reused from the last activated immutable Lance snapshot."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.clinical_retrieval_build_image == null || can(regex(
      "^us-east1-docker\\.pkg\\.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:[0-9a-f]{64}$",
      var.clinical_retrieval_build_image,
    ))
    error_message = "clinical_retrieval_build_image must be an exact production CUDA runtime image digest."
  }
}

variable "clinical_document_model_archive" {
  description = "Exact private MedCPT article-encoder archive used only by the scale-to-zero retrieval builder."
  type = object({
    object         = string
    sha256         = string
    weights_sha256 = string
  })
  default  = null
  nullable = true

  validation {
    condition = var.clinical_document_model_archive == null || (
      can(regex("^builds/sha256/[0-9a-f]{64}/MedCPT-Article-Encoder\\.tar\\.gz$", var.clinical_document_model_archive.object)) &&
      can(regex("^[0-9a-f]{64}$", var.clinical_document_model_archive.sha256)) &&
      can(regex("^[0-9a-f]{64}$", var.clinical_document_model_archive.weights_sha256))
    )
    error_message = "The document model must be one exact content-addressed private build object with exact archive and weights hashes."
  }
}

variable "clinical_query_model_archive" {
  description = "Exact private MedCPT-Query-Encoder archive included in the immutable serving package."
  type = object({
    object = string
    sha256 = string
  })
  default  = null
  nullable = true

  validation {
    condition = var.clinical_query_model_archive == null || (
      can(regex("^builds/sha256/[0-9a-f]{64}/MedCPT-Query-Encoder\\.tar\\.gz$", var.clinical_query_model_archive.object)) &&
      can(regex("^[0-9a-f]{64}$", var.clinical_query_model_archive.sha256))
    )
    error_message = "The MedCPT-Query-Encoder archive must be one exact content-addressed private build object with an exact hash."
  }
}

variable "clinical_cross_encoder_archive" {
  description = "Exact private MedCPT-Cross-Encoder archive included in the immutable serving package."
  type = object({
    object = string
    sha256 = string
  })
  default  = null
  nullable = true

  validation {
    condition = var.clinical_cross_encoder_archive == null || (
      can(regex("^builds/sha256/[0-9a-f]{64}/MedCPT-Cross-Encoder\\.tar\\.gz$", var.clinical_cross_encoder_archive.object)) &&
      can(regex("^[0-9a-f]{64}$", var.clinical_cross_encoder_archive.sha256))
    )
    error_message = "The MedCPT-Cross-Encoder archive must be one exact content-addressed private build object with an exact hash."
  }
}

variable "clinical_retrieval_build_batch_size" {
  description = "Bounded MedCPT document-embedding batch size for the single retrieval build task."
  type        = number
  default     = 64

  validation {
    condition     = var.clinical_retrieval_build_batch_size >= 1 && var.clinical_retrieval_build_batch_size <= 256
    error_message = "clinical_retrieval_build_batch_size must remain between 1 and 256."
  }
}

variable "clinical_retrieval_max_passages" {
  description = "Hard passage-count ceiling checked before embedding inference; up to 46 million, the backfill's projected final size, set once (Dev, 2026-09-28). The vector layout is qualified at 42 million; the builder also holds a refresh to its trained layout's allowed size."
  type        = number
  default     = 1000000

  validation {
    condition     = var.clinical_retrieval_max_passages >= 1 && var.clinical_retrieval_max_passages <= 46000000
    error_message = "clinical_retrieval_max_passages must remain between 1 and 46000000."
  }
}

variable "gpu_clinical_retrieval_image" {
  description = "Immutable CUDA Clinical Evidence image used by the service."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.gpu_clinical_retrieval_image == null || can(regex(
      "^us-east1-docker\\.pkg\\.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:[0-9a-f]{64}$",
      var.gpu_clinical_retrieval_image,
    ))
    error_message = "gpu_clinical_retrieval_image must be an exact production Artifact Registry sha256 digest."
  }
}

variable "migration_image" {
  description = "Immutable application image used only by the clinical database migration job."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.migration_image == null || can(regex(
      "^us-east1-docker\\.pkg\\.dev/coda-health-prod/coda-clinical-evidence/migration@sha256:[0-9a-f]{64}$",
      var.migration_image,
    ))
    error_message = "migration_image must be an exact production Artifact Registry sha256 digest."
  }
}

variable "clinical_evidence_service_enabled" {
  description = "Creates the authenticated L4 Clinical Evidence service after database import and activation."
  type        = bool
  default     = false

  validation {
    condition     = !var.clinical_evidence_service_enabled || var.gpu_clinical_retrieval_image != null
    error_message = "clinical_evidence_service_enabled requires an immutable GPU image digest."
  }
}

variable "clinical_evidence_service_paused" {
  description = "Disables retrieval requests with manual scaling at zero while retaining the service, images, and invoker bindings."
  type        = bool
  default     = false

  validation {
    condition     = !var.clinical_evidence_service_paused || (var.clinical_evidence_service_enabled && !var.proof_window_active)
    error_message = "A paused Clinical Evidence service must remain provisioned and cannot have an active proof window."
  }
}


variable "clinical_evidence_invokers" {
  description = "Explicit application service accounts and named developer users/groups allowed to invoke retrieval."
  type        = set(string)
  default     = []

  validation {
    condition = alltrue([
      for principal in var.clinical_evidence_invokers : can(regex(
        "^(serviceAccount|user|group):[^@[:space:]]+@[^@[:space:]]+$",
        principal,
      ))
    ])
    error_message = "clinical_evidence_invokers accepts only explicit serviceAccount:, user:, or group: principals."
  }
}

variable "clinical_evidence_worker_count" {
  description = "Identical retrieval workers resident in the one bounded GPU service instance."
  type        = number
  default     = 2

  validation {
    condition     = var.clinical_evidence_worker_count >= 1 && var.clinical_evidence_worker_count <= 2
    error_message = "clinical_evidence_worker_count must be 1 or 2 until a larger topology is measured."
  }
}

variable "clinical_serving_replica_count" {
  description = "Always-on L4 serving replicas, each with the active build on its own disk. Zero outside test windows until launch."
  type        = number
  default     = 0

  validation {
    condition     = contains([0, 2], var.clinical_serving_replica_count)
    error_message = "clinical_serving_replica_count must be 0 or 2 until more capacity is measured."
  }

  validation {
    condition     = var.clinical_serving_replica_count == 0 || var.gpu_clinical_retrieval_image != null
    error_message = "Serving replicas require an immutable GPU image digest."
  }
}

variable "clinical_serving_zones" {
  description = "us-east1 zones with L4 capacity; one replica in each."
  type        = list(string)
  default     = ["us-east1-b", "us-east1-c"]

  validation {
    condition     = length(var.clinical_serving_zones) == 2 && alltrue([for zone in var.clinical_serving_zones : startswith(zone, "us-east1-")])
    error_message = "clinical_serving_zones must name two us-east1 zones."
  }
}

variable "clinical_serving_boot_image" {
  description = "Exact boot image for serving replicas: Google's deep learning image with the NVIDIA 580 driver."
  type        = string
  default     = "projects/deeplearning-platform-release/global/images/common-cu129-ubuntu-2204-nvidia-580-v20261001"

  validation {
    condition     = can(regex("^projects/deeplearning-platform-release/global/images/[a-z0-9-]+-v[0-9]{8}$", var.clinical_serving_boot_image))
    error_message = "clinical_serving_boot_image must name one exact deep learning image, never a family."
  }
}

variable "proof_window_active" {
  description = "Keeps one retrieval instance resident only during an explicit bounded proof window."
  type        = bool
  default     = false

  validation {
    condition     = !var.proof_window_active || var.clinical_evidence_service_enabled
    error_message = "proof_window_active requires the Clinical Evidence service."
  }
}

variable "clinical_source_acquisition_enabled" {
  description = "Enables one daily change-driven source acquisition job. It stages one immutable candidate and never opens SQL, builds retrieval assets, or activates a build."
  type        = bool
  default     = false

  validation {
    condition = !var.clinical_source_acquisition_enabled || (
      var.source_maintenance_image != null &&
      var.clinical_source_acquisition_bootstrap != null
    )
    error_message = "clinical_source_acquisition_enabled requires an immutable maintainer image and an exact admitted baseline."
  }
}

variable "source_maintenance_image" {
  description = "Immutable lightweight maintainer image containing the source-specific acquisition adapters."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.source_maintenance_image == null || can(regex(
      "^us-east1-docker\\.pkg\\.dev/coda-health-prod/coda-clinical-evidence/maintainer@sha256:[0-9a-f]{64}$",
      var.source_maintenance_image,
    ))
    error_message = "source_maintenance_image must be an exact production maintainer image sha256 digest."
  }
}

variable "clinical_source_acquisition_bootstrap" {
  description = "Exact already-admitted source baseline used only when the operational state object does not yet exist."
  type = object({
    dailymed_admitted_through = string
    pmc_receipt_object        = string
    mmwr_inventory_object     = string
  })
  default  = null
  nullable = true

  validation {
    condition = var.clinical_source_acquisition_bootstrap == null || (
      can(regex("^[0-9]{4}-[0-9]{2}-[0-9]{2}$", var.clinical_source_acquisition_bootstrap.dailymed_admitted_through)) &&
      can(regex("^operations/source-acquisition/objects/[0-9a-f]{64}/data$", var.clinical_source_acquisition_bootstrap.pmc_receipt_object)) &&
      (
        startswith(var.clinical_source_acquisition_bootstrap.mmwr_inventory_object, "operations/source-acquisition/bootstrap/") ||
        can(regex("^operations/source-acquisition/snapshots/[0-9a-f]{64}/mmwr/inventory\\.json$", var.clinical_source_acquisition_bootstrap.mmwr_inventory_object))
      )
    )
    error_message = "The acquisition bootstrap must name one dated admitted DailyMed cursor, an immutable PMC root, and an owned MMWR bootstrap or immutable snapshot inventory."
  }
}

variable "clinical_mmwr_index_urls" {
  description = "Official annual CDC MMWR index families; acquisition derives the current year and also rechecks all retained historical indexes and articles."
  type        = list(string)
  default = [
    "https://www.cdc.gov/mmwr/index2026.html",
    "https://www.cdc.gov/mmwr/indrr_2026.html",
    "https://www.cdc.gov/mmwr/indss_2026.html",
  ]

  validation {
    condition = length(var.clinical_mmwr_index_urls) > 0 && alltrue([
      for url in var.clinical_mmwr_index_urls : can(regex(
        "^https://www\\.cdc\\.gov/mmwr/(index|indrr_|indss_)[0-9]{4}\\.html$", url,
      ))
    ])
    error_message = "clinical_mmwr_index_urls must contain only official annual CDC MMWR index URLs."
  }
}

variable "clinical_uspstf_enabled" {
  description = "Acquires the USPSTF Prevention TaskForce dataset nightly with the organization key from the Terraform-owned secret. Enable only after the key's first secret version exists (acquisition reads it when its job starts) and with a maintainer image that contains the USPSTF adapter."
  type        = bool
  default     = false

  validation {
    condition     = !var.clinical_uspstf_enabled || var.clinical_source_acquisition_enabled
    error_message = "clinical_uspstf_enabled requires clinical source acquisition."
  }
}

variable "clinical_source_acquisition_time_budget_seconds" {
  description = "Resumable PMC article-work window each daily run, after the complete inventory checkpoint. Throttled records are retried inside it at their own deadlines. Sized from measured nights; native Batch bounds total task duration."
  type        = number
  default     = 7200

  validation {
    condition     = var.clinical_source_acquisition_time_budget_seconds >= 60 && var.clinical_source_acquisition_time_budget_seconds <= 72000 && floor(var.clinical_source_acquisition_time_budget_seconds) == var.clinical_source_acquisition_time_budget_seconds
    error_message = "The acquisition work budget must be an integer between 60 seconds and 20 hours."
  }
}

variable "clinical_pmc_raw_archive_receipt" {
  description = "Completion receipt of the raw PMC text archive. When set, the refresh workflow offers the backfill operation, which pins each next historical cohort from this archive."
  type = object({
    bucket     = string
    name       = string
    generation = string
    byte_count = number
    sha256     = string
  })
  default = null
}

variable "clinical_backfill_articles" {
  description = "Articles in each pinned backfill cohort. Size it against the passage guard: at 2026-09 rates about 58 passages per article."
  type        = number
  default     = 100000

  validation {
    condition     = var.clinical_backfill_articles >= 1 && var.clinical_backfill_articles <= 100000 && floor(var.clinical_backfill_articles) == var.clinical_backfill_articles
    error_message = "A backfill cohort holds between 1 and 100,000 articles."
  }
}

variable "clinical_backfill_time_budget_seconds" {
  description = "PMC article-work time for one backfill run after the inventory checkpoint; an unfinished cohort resumes on the next backfill run. The task allows 12 hours in total."
  type        = number
  default     = 28800

  validation {
    condition     = var.clinical_backfill_time_budget_seconds >= 60 && var.clinical_backfill_time_budget_seconds <= 32400 && floor(var.clinical_backfill_time_budget_seconds) == var.clinical_backfill_time_budget_seconds
    error_message = "The backfill work budget must be an integer between 60 seconds and 9 hours."
  }
}

variable "clinical_bulk_admission_timeout_seconds" {
  description = "Batch admission deadline for every publication. Size against the largest candidate and approved cost."
  type        = number
  default     = 7200

  validation {
    condition     = var.clinical_bulk_admission_timeout_seconds >= 7200 && var.clinical_bulk_admission_timeout_seconds <= 43200 && floor(var.clinical_bulk_admission_timeout_seconds) == var.clinical_bulk_admission_timeout_seconds
    error_message = "Bulk admission must have an integer deadline between two and twelve hours."
  }
}

variable "clinical_bulk_build_timeout_seconds" {
  description = "Native GPU task deadline for every publication's Batch build, counted from when it starts running; its workflow allows fifteen more minutes."
  type        = number
  default     = 14400

  validation {
    condition     = var.clinical_bulk_build_timeout_seconds >= 14400 && var.clinical_bulk_build_timeout_seconds <= 64800 && floor(var.clinical_bulk_build_timeout_seconds) == var.clinical_bulk_build_timeout_seconds
    error_message = "Bulk building must have an integer deadline between four and eighteen hours."
  }
}

variable "clinical_batch_scheduling_wait_seconds" {
  description = "How long every clinical Batch job may wait for a machine before it is deleted and fails, or the build moves to its next machine. Execution deadlines count from when the job starts running."
  type        = number
  default     = 7200

  validation {
    condition     = var.clinical_batch_scheduling_wait_seconds >= 1800 && var.clinical_batch_scheduling_wait_seconds <= 21600 && floor(var.clinical_batch_scheduling_wait_seconds) == var.clinical_batch_scheduling_wait_seconds
    error_message = "Batch scheduling must have an integer wait between thirty minutes and six hours."
  }
}

variable "clinical_gpu_capacity_wait_seconds" {
  description = "How long one publication keeps taking the build machines in turn, each for one scheduling wait, before it fails and alerts. Only parked work and running SQL wait; no GPU bills until a machine starts."
  type        = number
  default     = 86400

  validation {
    condition     = var.clinical_gpu_capacity_wait_seconds >= 14400 && var.clinical_gpu_capacity_wait_seconds <= 172800 && floor(var.clinical_gpu_capacity_wait_seconds) == var.clinical_gpu_capacity_wait_seconds
    error_message = "A publication's GPU capacity wait must be an integer between four and forty-eight hours."
  }
}

variable "clinical_retrieval_build_machines" {
  description = "Machines a publication's GPU build takes in turn, each as its own Batch job, when the previous one gets no capacity within the scheduling wait, until clinical_gpu_capacity_wait_seconds is spent. All run in var.region, next to the corpus."
  type = list(object({
    name         = string
    machine_type = string
    accelerator  = string
  }))
  default = [
    { name = "l4", machine_type = "g2-standard-12", accelerator = "nvidia-l4" },
    { name = "t4", machine_type = "n1-standard-16", accelerator = "nvidia-tesla-t4" },
  ]

  validation {
    condition = (length(var.clinical_retrieval_build_machines) >= 1 && length(var.clinical_retrieval_build_machines) <= 3 &&
      length(distinct([for machine in var.clinical_retrieval_build_machines : machine.name])) == length(var.clinical_retrieval_build_machines) &&
    alltrue([for machine in var.clinical_retrieval_build_machines : can(regex("^[a-z0-9]{1,6}$", machine.name))]))
    error_message = "Build machines need one to three entries with distinct short lowercase names."
  }

  validation {
    condition     = alltrue([for machine in var.clinical_retrieval_build_machines : contains(["nvidia-l4", "nvidia-tesla-t4"], machine.accelerator)])
    error_message = "Builds run on L4 or T4 GPUs only; A100s are excluded (Dev, 2026-10-06)."
  }
}

variable "clinical_source_acquisition_schedule" {
  description = "Daily Cloud Scheduler cadence (America/New_York) for bounded source acquisition. 22:30 follows the day's PMC inventory publication (observed by 18:05) and keeps NCBI requests inside its 21:00-05:00 off-peak window."
  type        = string
  default     = "30 22 * * *"
}

variable "clinical_source_scheduler_enabled" {
  description = "Runs the already-provisioned end-to-end daily refresh workflow. Keep false until one controlled workflow proof completes."
  type        = bool
  default     = false

  validation {
    condition = !var.clinical_source_scheduler_enabled || (
      var.clinical_source_acquisition_enabled &&
      var.clinical_retrieval_build_enabled
    )
    error_message = "The clinical source scheduler requires acquisition, retrieval build, and activation foundations."
  }
}

variable "clinical_source_terms_retrieved_at" {
  description = "Exact timestamp of the reviewed source-rights terms used by hosted source admission."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.clinical_source_terms_retrieved_at == null || can(formatdate(
      "YYYY-MM-DD'T'hh:mm:ssZ", var.clinical_source_terms_retrieved_at,
    ))
    error_message = "clinical_source_terms_retrieved_at must be an RFC 3339 timestamp."
  }

  validation {
    condition     = !var.clinical_source_acquisition_enabled || var.clinical_source_terms_retrieved_at != null
    error_message = "Enabled source admission requires the exact reviewed source-terms timestamp."
  }
}

variable "clinical_operations_notification_channels" {
  description = "Existing Cloud Monitoring notification-channel resource names for source-change and job-failure alerts."
  type        = set(string)
  default     = []
}

variable "clinical_operations_notification_email" {
  description = "Optional exact email destination for a Terraform-owned Clinical Evidence notification channel."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.clinical_operations_notification_email == null || can(regex(
      "^[^@[:space:]]+@[^@[:space:]]+$",
      var.clinical_operations_notification_email,
    ))
    error_message = "clinical_operations_notification_email must be a valid email address."
  }
}
variable "clinical_bibliographic_contact_email" {
  description = "Registered source-operator contact sent to NCBI. Null leaves the new status-refresh path disabled until pre-production review."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition     = var.clinical_bibliographic_contact_email == null ? true : can(regex("^[^@[:space:]]+@[^@[:space:]]+\\.[^@[:space:]]+$", var.clinical_bibliographic_contact_email))
    error_message = "The bibliographic source contact must be an email address or null."
  }
}
