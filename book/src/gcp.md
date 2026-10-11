# GCP deployment

Duet production composition stays in [`infra/gcp/README.md`](../../submodules/coda-app/infra/gcp/README.md). The physician OpenTofu roots in this workspace are [`foundation`](../../infra/gcp/foundation), [`staging`](../../infra/gcp/staging/README.md), and [`clinical-evidence`](../../infra/gcp/clinical-evidence/README.md). This workspace does not apply them.

## Release procedure

- [Environments](../../submodules/coda-app/infra/gcp/README.md#environments)
- [Branch rehearsal](../../submodules/coda-app/infra/gcp/README.md#branch-rehearsal)
- [Staging window](../../submodules/coda-app/infra/gcp/README.md#staging-window)
- [Release runner](../../submodules/coda-app/infra/gcp/README.md#release-runner)
- [One-time staging provider admission](../../submodules/coda-app/infra/gcp/README.md#one-time-staging-provider-admission)
- [Compatibility-checked rollback](../../submodules/coda-app/infra/gcp/README.md#compatibility-checked-rollback)
- [Staging identity setup and deletion proof](../../submodules/coda-app/infra/gcp/README.md#staging-identity-setup-and-deletion-proof)

## Operations

- [Monitoring](../../submodules/coda-app/infra/gcp/README.md#monitoring)
- [Required services](../../submodules/coda-app/infra/gcp/README.md#required-services)
- [Current launch guardrails](../../submodules/coda-app/infra/gcp/README.md#current-launch-guardrails)
- [Doctor-page cutover](../../submodules/coda-app/infra/gcp/README.md#doctor-page-cutover)
- [Worker queue capacity](../../submodules/coda-app/infra/gcp/README.md#worker-queue-capacity)
- [Continuation preparation queue](../../submodules/coda-app/infra/gcp/README.md#continuation-preparation-queue)
- [Scheduled conversation queue](../../submodules/coda-app/infra/gcp/README.md#scheduled-conversation-queue)
- [Follow-up and review wake job](../../submodules/coda-app/infra/gcp/README.md#follow-up-and-review-wake-job)
- [Acquisition worker operations](../../submodules/coda-app/infra/gcp/README.md#acquisition-worker-operations)
- [Secrets and runtime settings](../../submodules/coda-app/infra/gcp/README.md#secrets-and-runtime-settings)
- [Minimum supported iPhone build](../../submodules/coda-app/infra/gcp/README.md#minimum-supported-iphone-build)
- [Sign out everywhere readback](../../submodules/coda-app/infra/gcp/README.md#sign-out-everywhere-readback)
- [Model cost controls](../../submodules/coda-app/infra/gcp/README.md#model-cost-controls)
- [Order](../../submodules/coda-app/infra/gcp/README.md#order)
- [Removed runtime decommission](../../submodules/coda-app/infra/gcp/README.md#removed-runtime-decommission)
- [Access and security operations](../../submodules/coda-app/infra/gcp/README.md#access-and-security-operations)
- [Cloud SQL restore drill](../../submodules/coda-app/infra/gcp/README.md#cloud-sql-restore-drill)
- [Privacy reconciliation before recovery serves traffic](../../submodules/coda-app/infra/gcp/README.md#privacy-reconciliation-before-recovery-serves-traffic)

## Dated release and security evidence

Source: [`Dated release and security evidence`](../../submodules/coda-app/infra/gcp/README.md#dated-release-and-security-evidence).

- [October 5, 2026 spend records and reviewed doctor page release](../../submodules/coda-app/infra/gcp/README.md#october-5-2026-spend-records-and-reviewed-doctor-page-release)
- [October 5, 2026 structural cleanup release](../../submodules/coda-app/infra/gcp/README.md#october-5-2026-structural-cleanup-release)
- [October 3, 2026 Wave 1 and Genome removal release](../../submodules/coda-app/infra/gcp/README.md#october-3-2026-wave-1-and-genome-removal-release)
- [October 2, 2026 patient isolation and graceful runs release](../../submodules/coda-app/infra/gcp/README.md#october-2-2026-patient-isolation-and-graceful-runs-release)
- [October 1, 2026 security and build 133 release](../../submodules/coda-app/infra/gcp/README.md#october-1-2026-security-and-build-133-release)
- [October 1, 2026 Doctors & hospitals release](../../submodules/coda-app/infra/gcp/README.md#october-1-2026-doctors-hospitals-release)
- [September 30, 2026 minimum build 131 release](../../submodules/coda-app/infra/gcp/README.md#september-30-2026-minimum-build-131-release)
- [September 30, 2026 companion release](../../submodules/coda-app/infra/gcp/README.md#september-30-2026-companion-release)
- [September 27, 2026 first-session recovery release](../../submodules/coda-app/infra/gcp/README.md#september-27-2026-first-session-recovery-release)
- [Earlier releases](../../submodules/coda-app/infra/gcp/README.md#earlier-releases)
