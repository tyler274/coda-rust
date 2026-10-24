# Physician runtime (GKE Autopilot)

This is a retained GKE deployment reference. The current isolated Rust staging
path is [Cloud Run](../../../book/src/gcp.md#rust-staging-bootstrap-and-deployment-gate).
This workspace does not apply this GKE root. Its configuration describes
 one private GKE Autopilot cluster,
one private Cloud SQL Postgres 16 instance with IAM database authentication, one
Artifact Registry repository, the keyless site and migration identities, and
the private network binding them. It describes a deployment target for
[`physician-sample`](/crates/physician-sample) (the site) and
[`physician-auth`](/crates/physician-auth) (its store and migration binary).

The default configuration parks the database (`sql_activation_policy = "NEVER"`)
and deploys no workload: `physician_site_image`, `physician_browser_origin`, and
`physician_migration_image` are null, so the root plans only network, cluster,
database, and identity scaffolding. A release binds digest-pinned images and an
exact HTTPS browser origin, sets `physician_site_enabled = true` (and
`migration_enabled = true` when the schema changes), and returns the database
policy to `NEVER` when the window ends.

## Boundary

- The cluster is private: private nodes, a private endpoint allowlist is not
  opened here, and the site service is `ClusterIP`. Public exposure requires a
  separately reviewed ingress and the exact browser origin as its allowlist.
- The site and the migration job hold separate Google service accounts. Both
  authenticate to Cloud SQL as themselves (`roles/cloudsql.instanceUser`); no
  database password exists. The migration account alone owns schema changes.
- `PHYSICIAN_AUTH=required` rides the site's environment, so the bearer guard
  and workspace-membership checks are configured; prove actual hosted access
  separately from local guard tests.
- Sign-in tokens are single-use with a 24-hour TTL; sessions are bearer
  tokens with a 30-day TTL and deactivation fencing. Delivery in this
  deployment is `proof` (the token returns in the sign-in response); a real
  email delivery is a separate authorized step.
- The site sends OTLP spans to the in-cluster collector that
  [`infra/gcp/observability`](/infra/gcp/observability) runs
  (`otel_collector_endpoint`, which must be a `*.svc.cluster.local` address),
  with `DD_ENV = var.environment`, `DD_VERSION` taken from the image digest,
  and `CODA_TELEMETRY_CONTENT = var.telemetry_content` (default `none`, so no
  prompt or completion text leaves the pod).

## Testing

```sh
tofu test -chdir=infra/gcp/physician-runtime
tofu fmt -check -recursive infra/gcp/physician-runtime
tofu -chdir=infra/gcp/physician-runtime init -backend=false
tofu -chdir=infra/gcp/physician-runtime validate
```

This workspace does not apply the root. Apply authority stays with the
operator who owns the target project.
