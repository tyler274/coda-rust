# GCP deployment

Duet production composition stays in <a href="/submodules/coda-app/infra/gcp/README%2Emd"><code>infra/gcp/README.md</code></a>. The physician OpenTofu roots in this workspace are [`foundation`](/infra/gcp/foundation), <a href="/infra/gcp/staging/README%2Emd"><code>staging</code></a>, and <a href="/infra/gcp/clinical-evidence/README%2Emd"><code>clinical-evidence</code></a>. This workspace does not apply them. The one exception is public-site staging, described below.

## Public marketing site preparation

The separate [`infra/gcp/public-site`](/infra/gcp/public-site) root prepares the
stateless Rust marketing server on Cloud Run. Staging runs in the existing
`coda-health-staging` project with direct Cloud Run IAP and no load balancer or
CDN. Access is a service-scoped `roles/iap.httpsResourceAccessor` binding for
`group:gcp-engineering@codahealth.ai`, and the IAP service agent is the only
invoker. The origin also verifies the signed IAP assertion. Production stays behind
a global external HTTPS load balancer with Cloud CDN. There is no Cloudflare
dependency. Unlike the physician roots, this root owns its **staging** state
(GCS prefix `public-site/staging` in the existing state bucket) and may be planned
and applied for staging from this workspace. It is never planned or applied for
production here. The image is published and a staging plan has been reviewed, but
the apply is blocked on project IAM (see
[deployment status](public-site.md#staging-deployment-status-2026-10-20)). The
root does not share physician/Duet databases or authorize a product cutover. See
[Public site](public-site.md) for the architecture, IAP and cache controls, runtime
configuration, image build/smoke procedure, verification evidence and the
[staging state and deployment](public-site.md#staging-state-and-deployment)
procedure.

[PR #36](https://github.com/Coda-Health/coda-rust/pull/36) merged the implementation;
[ENG-79](https://linear.app/coda-health/issue/ENG-79) still owns hosting migration
completion. Dev's staging review and production approval, hostname/IAP/analytics
decisions, verified cutover/recovery procedures and the agreed recovery window
precede removal of website-only Vercel resources and credentials. Neither this
chapter nor passing local/CI validation authorizes deployment, DNS changes or
Vercel retirement.

## Release procedure

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#environments">Environments</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#branch-rehearsal">Branch rehearsal</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#staging-window">Staging window</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#release-runner">Release runner</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#one-time-staging-provider-admission">One-time staging provider admission</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#compatibility-checked-rollback">Compatibility-checked rollback</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#staging-identity-setup-and-deletion-proof">Staging identity setup and deletion proof</a>

## Operations

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#monitoring">Monitoring</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#required-services">Required services</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#current-launch-guardrails">Current launch guardrails</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#doctor-page-cutover">Doctor-page cutover</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#worker-queue-capacity">Worker queue capacity</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#continuation-preparation-queue">Continuation preparation queue</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#scheduled-conversation-queue">Scheduled conversation queue</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#follow-up-and-review-wake-job">Follow-up and review wake job</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#acquisition-worker-operations">Acquisition worker operations</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#secrets-and-runtime-settings">Secrets and runtime settings</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#minimum-supported-iphone-build">Minimum supported iPhone build</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#sign-out-everywhere-readback">Sign out everywhere readback</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#model-cost-controls">Model cost controls</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#order">Order</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#removed-runtime-decommission">Removed runtime decommission</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#access-and-security-operations">Access and security operations</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#cloud-sql-restore-drill">Cloud SQL restore drill</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#privacy-reconciliation-before-recovery-serves-traffic">Privacy reconciliation before recovery serves traffic</a>

## Dated release and security evidence

Source: <a href="/submodules/coda-app/infra/gcp/README%2Emd#dated-release-and-security-evidence"><code>Dated release and security evidence</code></a>.

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-5-2026-spend-records-and-reviewed-doctor-page-release">October 5, 2026 spend records and reviewed doctor page release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-5-2026-structural-cleanup-release">October 5, 2026 structural cleanup release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-3-2026-wave-1-and-genome-removal-release">October 3, 2026 Wave 1 and Genome removal release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-2-2026-patient-isolation-and-graceful-runs-release">October 2, 2026 patient isolation and graceful runs release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-1-2026-security-and-build-133-release">October 1, 2026 security and build 133 release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-1-2026-doctors--hospitals-release">October 1, 2026 Doctors &amp; hospitals release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-30-2026-minimum-build-131-release">September 30, 2026 minimum build 131 release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-30-2026-companion-release">September 30, 2026 companion release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-27-2026-first-session-recovery-release">September 27, 2026 first-session recovery release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#earlier-releases">Earlier releases</a>
