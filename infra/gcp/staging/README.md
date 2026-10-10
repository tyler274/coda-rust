# Hosted synthetic integration platform

This stack owns the isolated non-PHI hosted analogue of the local Coda application. It is an
application-only environment: one private Cloud SQL instance and runtime database, one private
runtime-evidence bucket, one Artifact Registry repository, the keyless application and database
migration identities, and the private network used by Direct VPC egress.

Clinical Evidence is not duplicated here. The canonical corpus bucket, corpus database, retrieval
projections, activation jobs, retrieval service, and their identities live only in
`coda-health-prod`. When the hosted synthetic application is enabled, its
`clinical_evidence_service_url` points to that authenticated production service. The production
Clinical Evidence stack owns the explicit Cloud Run Invoker grant for the staging application
identity; this stack never grants itself access across that boundary.

The default configuration leaves `application_image`, `application_browser_origin`, `migration_image`,
and `clinical_evidence_service_url` null, so it creates no Cloud Run workload. A release may bind a
digest-pinned application image, exact HTTPS browser origin, and production Clinical Evidence URL to
enable the private hosted application. A separately digest-pinned migration image creates the runtime-only
`coda-database-migrator` job, which applies the existing runtime and evidence migrations and grants
only the application database identity.

Cloud SQL is parked with `sql_activation_policy = "NEVER"` outside a named proof window. A proof
window changes it to `ALWAYS` and may set `proof_window_active = true` only when the application
service is enabled. That keeps one application instance resident for bounded end-to-end proof.
Return the instance to `NEVER` and Cloud Run minimum scale to zero afterward. No scheduler or
background-delivery system is implied by this control.

The application service account authenticates only to the runtime database and runtime-evidence
bucket. The database migration account is separate, keyless, and limited to the application
database migration job. Models and clients receive neither database credentials nor direct corpus
access. Clinical searches always cross the typed authenticated evidence gateway.

Cloud Run uses Direct VPC egress with `PRIVATE_RANGES_ONLY`: PostgreSQL private-IP traffic enters
the staging VPC, while Google control-plane and provider API traffic retain Cloud Run's managed
egress. Do not change this to `ALL_TRAFFIC` without adding and reviewing the corresponding Google
API DNS and NAT boundary.

Artifact Registry retains digest-pinned application and migration images. Its cleanup policy
deletes only untagged image versions older than seven days. The staging project's Cloud Build
identity may read its build-source bucket, write this repository, and write build logs; it has no
access to the production clinical-corpus bucket.

All plans are saved outside the repository. Never commit state, plan files, variable files,
credentials, patient data, synchronized corpus data, retrieval assets, provider artifacts, or
command output.

Google Service Usage and Cloud Resource Manager must be enabled once when a new project is
bootstrapped. Enable only `serviceusage.googleapis.com` and
`cloudresourcemanager.googleapis.com` under the named corporate administrator, then let this stack
manage its declared workload APIs with `disable_on_destroy = false`.
