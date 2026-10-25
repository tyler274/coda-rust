# Production Clinical Evidence runbook

This is the Rust workspace's reference copy of the product-owned stack. Source
paths and operational commands below refer to `submodules/coda-agents`; apply
authority remains in that product repository. Do not initialize its backend or
apply this root from the Rust workspace.

This Terraform stack owns the one production Clinical Evidence subsystem in `coda-health-prod`. The
[architecture](../../../submodules/coda-agents/docs/architecture.md#hosted-environments-and-promotion) owns its product
boundary and the [runtime protocol](../../../submodules/coda-agents/docs/runtime-protocol.md) owns the search/read contract.
[Readiness](../../../submodules/coda-agents/docs/readiness.md#clinical-corpus-operations) owns scope decisions, current state
and open work. This runbook owns how the pipeline works operationally and how to operate it.

The stack provisions:

- one private PostgreSQL corpus database (Cloud SQL, parked when unused);
- one content-addressed Artifact Registry repository and a one-day build-source bucket;
- one daily refresh workflow over bounded acquisition, admission, retrieval-build, activation and
  finalization jobs, plus a clinical-schema migration job;
- one authenticated NVIDIA L4 Cloud Run search service, minimum `0` and maximum `1` instance; and
- the narrow identities, private network and bucket permissions those resources require.

It does not own patient data, the Coda application runtime, provider calls or clinical judgment.
The private corpus bucket is the immutable source-artifact boundary; Terraform never copies or
rewrites corpus objects. Local and hosted Coda call the service only through the existing
`RemoteClinicalEvidenceGateway`; no database or object-store credential, retrieval asset or fallback
path enters the product composition. Real patient-derived queries remain prohibited from
subscription-CLI runs.

## Pipeline and state owners

```
acquire (Batch, CPU) → admit (Batch, CPU) → build (Batch, GPU) → activate → finalize
   GCS operations/        PostgreSQL corpus          Lance table     activation    GCS baseline
   state + candidates     rights + membership        under builds/   binding       advances
```

| State | Owner | Notes |
| --- | --- | --- |
| Source observations, immutable source material, checkpoints, retry history | GCS `operations/source-acquisition/` | Generation-conditional state object; create-only immutable objects |
| Admitted evidence, rights, source membership, exact reads | PostgreSQL `clinical_library` | Live rights and currentness views apply to every search result and read |
| Search projection | Native Lance table under `builds/lance/` | Rebuildable; never the evidence owner |
| Which snapshot serves | `retrieval_build_activations` | One active binding; immutable publication under `builds/sha256/` |

Where each stage lives. Jobs and their orchestration code are in `src/server/clinicalCorpus/`, one
folder per stage (`acquire/`, `admit/`, `build/`, `serve/`) plus `operations/` for the shared
acquisition state, maintenance lock, finalization and repair. Concrete adapters are in
`src/adapters/clinicalLibrary/`: `sources/` (source readers, payload stores and their Python),
`postgres/` (the `clinical_library` owners) and `retrieval/` (projection, Lance build, packaging and the
search worker, with their Python). Each Python group stays in one folder because its modules import
their siblings and hash them for implementation identity. Contracts stay in `src/clinicalLibrary/`.

| Stage | Entry point | Main collaborators |
| --- | --- | --- |
| Acquire | `acquire/hostedClinicalSourceAcquisition.ts` | `hostedReleaseSourceAcquisition.ts` (DailyMed, MMWR, USPSTF), `hostedPmcSourceAcquisition.ts` → `pmcAcquisitionWorker.ts` → `sources/pmc_article_dataset_acquirer.py` and its `pmc_*`/`nlm_*` modules; `pmcAcquisitionManifest.ts`, `pmcManifestTree.ts`, `pmcHistoricalBatch.ts`, `operations/clinicalSourceAcquisitionCheckpoint.ts` |
| Admit | `admit/hostedClinicalSourceAdmission.ts` | `localClinicalSourcePreparation.ts`, `pmcSourcePreparation.ts`, `pmcAdmissionArchiveCache.ts`, `postgres/postgresClinicalLibraryAdmission.ts`, `postgresClinicalDocumentAdmission.ts`, `postgresClinicalInventory*.ts`, `postgresClinicalRights.ts` |
| Build | `build/hostedClinicalRetrievalBuild.ts` | `prepareClinicalRetrievalCandidate.ts`, `googleCloudClinicalBuildStore.ts`, `postgres/postgresClinicalProjectionExport.ts`, `retrieval/build_clinical_retrieval.py`, `clinical_lance_delta.py`, `clinical_lance_updates.py` |
| Activate | `build/activateHostedClinicalRetrieval.ts` | `postgres/postgresClinicalRetrievalActivation.ts` |
| Finalize | `operations/finalizeHostedClinicalSourceRefresh.ts` | `clinicalRepresentationRepair.ts`, `pmcRepresentationRepair.ts` |
| Serve | `serve/hostedClinicalEvidenceStart.ts` | `hostedClinicalEvidenceServiceComposition.ts`, `hostedClinicalRetrievalRuntime.ts`, `retrieval/local_retrieval_worker.py` |
| Orchestrate | `clinical-*-workflow.yaml.tftpl` | `refresh-workflow.tf`, `maintenance.tf`, `*-batch.tf`, `clinical-workflow-windows.yaml.tftpl` |

Schema lives in `migrations/clinical-library/`; boundary tests are `test/clinical*Boundary.test.ts`
and `test/pmc*Boundary.test.ts`, with native checks in `test/clinicalLanceBoundary.py` (run by `pnpm test`);
Terraform's tests in `tests/` run with `pnpm test:infra`.

Three states stay distinct: the **finalized baseline**, the **moving acquisition head** and at most
one **frozen pending candidate**. Acquisition continues while a candidate is admitted, built and
published. Finalization promotes only the exact activated candidate; the next candidate derives from
accumulated qualified changes. New source units become searchable only after activation.

**`USER_SETTLED`: publish a prepared snapshot with live rights/currentness enforcement.** Preparation
owns eligibility, fingerprints and target counts. Activation compares the refresh receipt with the
immutable publication, checks the exact expected predecessor and changes the binding atomically; it
never repeats the corpus export. New admissions do not block publishing an earlier prepared snapshot:
live SQL excludes revoked or outdated units from search and reads, and later additions enter the next
build. Duplicate activation is idempotent.

## Daily refresh

Cloud Scheduler (`clinical_source_acquisition_schedule`, default 22:30 America/New_York, after the day's PMC inventory and inside NCBI's off-peak window) starts one
`coda-clinical-corpus-refresh` execution with zero delivery retries:

1. CPU-only acquisition observes every enabled source and stages at most one candidate. With no
   pending candidate and bibliographic checking disabled, it exits without starting SQL or GPU.
2. For a changed or previously pending candidate: start parked SQL, admit the exact candidate, build
   and publish an immutable retrieval snapshot, activate the run-scoped receipt, finalize the source
   baseline, and park SQL on both success and failure paths.
3. A metadata-only run starts SQL for the bounded PubMed status check, skips build/activation, and
   parks SQL.

`PROJECT_SETTLED`: the workflow owns SQL start/park on its success and caught-failure paths when SQL
is configured parked (`sql_activation_policy = "NEVER"`). With `ALWAYS`, the same workflow skips SQL
power changes. A fresh SQL corpus alone is never treated as a fresh active search service.

Daily acquisition defers publishing a candidate that was already pending when its run began, so the
next daily run cannot silently spend on another build; explicit recovery owns failed or bulk work.
A shortage of GPU capacity is waited out inside the publication itself (Build, below), so only a
publication that failed for another reason, or found no machine for a whole day, needs that recovery.

**Healthy looks like:** a terminal `activated`, `unchanged` or metadata-only outcome; per-source check
timestamps advancing; SQL parked afterward. `activated_with_status_failure` accompanies a successful
publication when the PubMed status check failed (below). A completed workflow alone is not
proof that every publisher was checked; inspect per-source observations.

**Alerts** are native log policies: execution failures (at most one email per hour), whatever needs a
person (at most one per 24 hours), and a missing successful daily terminal after 25 hours. What needs a
person is each source's own count for its night, `needs_person`, which the job that ran the night logs
(owner `clinical-source-night`) whether the nightly refresh or a backfill ran it; each thing is reported once: a set-aside item that crossed its own
threshold (a rejected item on its third failed attempt, work tried every night on its third failed night
whatever the cause, our own integrity or bug failure at once), or a
whole source that moved nothing (our own failure on its first night, an outside one on its second). A
night moved nothing when it failed outright or had due work and finished none of it; reading a listing is
not progress by itself, and a listing page that fails is due work left undone. Each source states its own
`work_due`, `work_done` and `progressed`, counted by one item ledger (`clinical_source_ledger.py` and its
TypeScript twin `src/clinicalLibrary/sourceLedger.ts`, both held to `clinical_source_ledger_cases.json`;
PMC counts its batches through the same ledger). A night that published while
items stay set aside is `partial`, not complete, and does not advance `last_success_at`. Every piece of
work that can fail has a set-aside record in the shared shape: articles, releases, labels and topics, MMWR
index pages (`index_failures`, beside the inventory's last membership) and PubMed's update files and status
checks (`set_aside` on its observation). Work retried every night has no retry time. A record leaves
only when its work is done or the latest observed listing no longer shows it (a page read tonight, or an
unread page's last membership); one waiting on its retry time is
carried untouched unless its new content is already in hand. A failure of any kind sets aside only the
item it touches; an ordered stream (DailyMed releases, PubMed update files) keeps what came before and
stops there. Every outside request has a limit per try and a total per call (2 minutes, 8 for file
downloads): each try is cut to the time left, and no retry wait starts that would end past the total. The
waits, thresholds and limits live in `src/adapters/clinicalLibrary/sources/clinical_source_rule.json`. A failure's
own words never enter logs: they are kept in the object its observation's `diagnostics_object` names.
Backlog, records waiting on their own retry, bounded progress with work left,
optional image unavailability and deferred overlapping work do not notify. Policies notify on
opening only; incident auto-closure is not recovery evidence. Pausing the scheduler disables only the
missing-success policy. Delivery requires `clinical_operations_notification_channels` or
`clinical_operations_notification_email`; a visible policy alone does not prove delivery.

## Sources

Every source adapter feeds the same admission owner; none becomes a second corpus, search, citation
or activation owner. Record-local failures retain the last-known-good observation with retry history
and never discard unrelated qualified changes. Source-scope, checksum, aggregate-budget and
byte-integrity failures fail the source check rather than being mislabeled as optional unavailability.

**DailyMed.** Daily releases with verified weekly catch-up; fails closed on missing coverage or a
catalog older than seven days. An absent weekday archive cannot prove zero changes. Each archive is
qualified package by package; malformed packages enter the retry receipt while siblings advance.
Retries reference the original archive object instead of uploading a copy.

**PMC.** A maintained [tag policy](pmc-collection-policy.json) selects the clinical collection through
NLM's PubMed publication-type index, PMC's official dated PMID→PMCID export (at most 72 hours old,
checksum- and format-verified) and bounded live NLM links for PubMed IDs missing from that export.
Existing export mappings stay authoritative when live links differ (`USER_SETTLED`, 2026-09-13).
Selection is intersected with the complete, checksum-verified publisher inventory before metadata,
JATS and image acquisition, and is re-enumerated on every refresh so tag and policy changes apply
without an inventory change. An empty, incomplete or failed collection query retains prior state.
Admission accepts only articles with one unambiguous CC0, CC BY 3.0 or CC BY 4.0 license and excludes
preprints and author manuscripts. PDF, OCR and supplements are out of scope.

- Current-version selection uses the official PMC ID Converter's explicit current flag in batches of
  at most 200 IDs and 300 seconds; the highest numeric version is not authoritative. A temporary API
  failure saves qualified artifacts as unresolved work under their retry history. A provider that did
  not answer retries in minutes (5, doubling) and a record-specific failure in hours (1, doubling),
  capped at a day (`clinical_source_rule.json`); the daily run waits inside its article window for the
  next retry deadline rather than ending early.
- Each PMC observation reports `advanced_this_run` (records that left pending or unresolved; attempts
  that end unresolved, and sibling versions rewritten in the status they already had, do not count). A night with due work
  that advanced nothing counts toward `nights_without_progress`, kept the same way for every source. PMC is acquired after
  DailyMed, MMWR and USPSTF, so its window never delays theirs.
- Removed records leave current search while historical evidence and reappearance tracking remain.
  `absent` means absent from the current scoped inventory, not a publisher retraction.
- Images are optional to text: an article above 16 images keeps its qualified text, acquires no
  partial image set and records the unavailable count. Publisher metadata is bounded at 8 MiB.
- NLM requests treat transport, decoding, response shape and page completeness as one attempt with
  three attempts total; explicit query rejections, conflicting identities and resource-boundary
  violations stop immediately. Terminal failures keep bounded request/response evidence.
- Preview the tag set's PubMed count without enumerating IDs or acquiring content (this is not the
  searchable article count):
  `pnpm clinical:acquire-pmc-inventory collection --count-only --policy-file infra/gcp/clinical-evidence/pmc-collection-policy.json`

**MMWR.** Rechecks every retained index and article daily, derives current-year index URLs from the
configured families and tolerates an unpublished new annual index. Response bodies are bounded at
1 GiB per acquisition within the 2,000-article limit. A failed existing article keeps its
last-known-good body; a failed new article holds that snapshot out of the candidate until retry.

**USPSTF.** AHRQ's Prevention TaskForce API, approved for Coda on 2026-09-09 and acquired only when
`clinical_uspstf_enabled` is set. Each night makes one `lastupdated` request and downloads the dataset
only when that time changed. Only a dataset newer than the accepted one publishes: an older one, or
different bytes under the accepted update time, fails the check and the accepted inventory keeps
serving; a dataset that has not advanced carries unchanged and is fetched again the next night. A
set-aside topic is tried again once its retry time comes, against the accepted dataset if the publisher
has not moved. Most nights report `downloaded: false`. The AHRQ organization key lives in the Terraform-owned secret
`coda-clinical-uspstf-api-key`, readable only by the acquisition identity, and reaches the job as a
Batch secret variable. It is never in tfvars, state, job configuration, receipts or logs.

Enabling takes two reviewed applies, because acquisition reads the key when its job starts:

1. Apply with `clinical_uspstf_enabled = false`. This creates the secret, with no version, and its grant.
2. Add the key once from the AHRQ approval email, through standard input so it never lands in a file
   or shell history: `gcloud secrets versions add coda-clinical-uspstf-api-key --project=coda-health-prod --data-file=-`,
   paste the key, then press Ctrl-D. The acquirer trims surrounding whitespace.
3. Apply with `clinical_uspstf_enabled = true` and a `source_maintenance_image` that contains the USPSTF
   adapter; an older image rejects the `uspstf` configuration and fails every source's acquisition.
   The first night admits every topic as one initial load.

To rotate, add a new version: the next job reads `latest`. Disable the old version after one healthy
night. Setting the variable back to `false` stops acquisition and keeps the secret (deletion
protection). Once production search and exact read are proven, Dev tells AHRQ the integration is
complete, which is their condition.

**PubMed status.** A separate check of known PMIDs for publication types and correction/retraction
relationships; it admits no abstract or full text. Each run first applies NLM's daily update files
after the saved `update_cursor`, in number order (up to `max_update_files`), then checks at most 20,000
of the oldest due PMIDs, never-checked first. Remaining files or due records make the check `partial`:
successful bounded progress, not a failure. A missing file number stops the files before it while the
rest of the night runs. Only a night that moved nothing is `failed`, and `nights_without_progress`
counts repeats, as for every source. Inspect `update_files`, confirmed, missing and
remaining counts before diagnosing an API failure. Do not drop the status contract or raise request
fan-out to turn the aggregate green.

## Admission

Admission runs the canonical container on one Batch VM for every publication (`e2-standard-4`,
16 GiB, 100 GB scratch, 16 GiB verified compressed-archive cache,
`clinical_bulk_admission_timeout_seconds`). The Cloud Run admission job is the container's canonical
definition and runs only rights enforcement.
Inventory staging and its ordered checkpoint commit in one source-locked transaction; completed
checkpoints are the recovery boundary. New versions, identities, rights and units use one set-based
insert after bounded history reads. No constraints or durability are disabled. Progress logs separate
archive preparation, XML decoding, media, SQL transaction time and scratch.

Other admission-owner operations, all through the workflow or existing job identities:

- **Rights observations** (`operation: rights`) record explicit source-native observations without
  taking the publication lock. Absence from an inventory never implies withdrawal. An explicit
  observation governs its version until a newer explicit observation replaces it; admitting a later
  publisher revision does not lift it.
- **Representation repair** is explicit: an unchanged publisher revision reuses its admitted
  representation, and reader changes require repair rather than silent re-acquisition.
- **Payload reconciliation** (`coda-clinical-payload-reconciliation`) repairs only the storage
  coordinates of already-copied content-addressed payloads after verifying name, SHA-256, size, media
  type and generation. It cannot change identity, metadata, units, rights, membership or builds.

## Retrieval build and search index

The builder (`coda-clinical-retrieval-builder`, CUDA, one L4) holds the corpus-maintenance advisory
lock, exports the rights-filtered projection as narrow unit IDs plus bounded payload batches, and skips
the build when the input set and retrieval recipe match. Within the recipe-owned Lance table,
unchanged units bypass inference and changed passages reuse vectors only when their exact embedding
input and recipe match. Delta mapping queries are bounded to 8,192 IDs and payload groups to 8 MiB;
native writes append at most 16,384 passages per transaction. Changed indexes are consolidated to one
segment without default compaction or version pruning; referenced native versions are retained.

- **Build** runs for every publication, daily or explicit, on one Batch VM (48 GiB task) in the
  SQL/bucket region with 200 GB scratch. Batch takes one machine per job, so the build takes
  `clinical_retrieval_build_machines` in turn (an L4 `g2-standard-12`, then a T4 `n1-standard-16`;
  never A100s), each as its own job, round after round within one elapsed deadline,
  `clinical_gpu_capacity_wait_seconds` (24 hours by default). Every Batch job must start running within
  `clinical_batch_scheduling_wait_seconds` (two hours by default) or the time left, whichever is less; one
  that doesn't is deleted and logged as `machine_unavailable`, and the next attempt is made. Only SQL runs
  while it waits. When no machine starts before the deadline, the publication fails, cleans up and
  alerts that the pending candidate and active corpus are kept: publish the candidate again once capacity
  returns. A build that starts and fails is never retried automatically. `clinical_bulk_build_timeout_seconds`, plus fifteen minutes, counts from when the
  build starts running. Batch owns GPU device and driver mounts. The shared runner submits each job once
  and settles every attempt's exact job before SQL cleanup. The backfill batches' builds spent about an hour before
  building and half an hour rebuilding indexes (batch 7, 43.8 million passages, 2026-09-30), past the
  30-minute Cloud Run task that daily publication used; a daily run's own minimum is unmeasured. A
  day with no candidate skips building, and an unchanged input returns the existing build before
  rebuilding indexes. The Cloud Run builder job is the container's canonical definition and runs
  only recipe conversion.
- **Capacity guard:** `clinical_retrieval_max_passages` refuses an oversized snapshot before inference,
  and the builder also holds a refresh to its trained layout's allowed size. The guard is at most 46
  million, the backfill's projected final size, set once rather than per batch (Dev, 2026-09-28). Neither qualifies encoder throughput, concurrent search latency or clinical quality.
- **Index layout:** IVF_PQ (96 sub-vectors) in one of two qualified layouts, owned by
  `clinical_lance_updates.py`, with search settings owned by the worker. The small layout trains at
  most 16 partitions, probes all of them with refinement factor 32, and holds 15 million passages. The
  full-size layout trains 4,096 partitions (sample rate 256, 50 iterations), probes 256 with
  refinement factor 10, and is qualified at 42 million and allowed to 46 million. Ordinary refreshes keep the trained index; only an
  explicit exact-target replacement repartitions retained vectors, choosing the layout by size.
- **Search recipes** are file-hashed. A recipe change requires an explicit qualified replacement
  (`operation: convert` with `replace_build_id`, or `publish` with `replace_build_id` in batch mode);
  ordinary refresh refuses recipe mismatch and never silently re-embeds. A replacement whose search
  recipe is unchanged extends its target by the ordinary delta and retrains the layout; only a changed
  recipe re-reads every unit from a full projection.
- **Publication** schema v2 binds the exact table URI/version and runtime configuration. The
  model/worker archive lives under `builds/runtime-assets/sha256/`, uploaded only when absent. The
  builder has read-only corpus access and object-update authority only under `builds/lance/`.
- `pnpm clinical:prepare-retrieval-candidate` is the equivalent local development and recovery
  composition; it does not upload or activate anything.

Native mechanical checks: `test/clinicalLanceBoundary.py`, run by `pnpm test` (deterministic Torch
fixtures, no downloaded weights). Real MedCPT quality and CUDA capacity are separate qualifications.

## Historical backfill

Backfill follows the archive-first decision in readiness: reuse the completed raw archive, select
PMC's explicit current version, and publish a few large batches.

**`USER_SETTLED`: qualify the complete lifecycle before the next larger run.** Review acquisition,
admission, preparation, embedding/index construction, activation, finalization, daily overlap and
cleanup together. A successful stage or local suite does not qualify the rest. After a failure,
preserve completed work, identify the owning stage and related failure paths, and verify one coherent
correction before another paid attempt. Never use successive cloud failures to discover the next
stage's capacity limit.

Before proposing a run, count distinct articles in its pinned queue and distinguish matching archived
full text, previously excluded observations and unresolved or changed inputs. Reconcile already
selected sibling versions before calling an article new. A record count, an archive hit, a successful
stage and a newly searchable article are different measurements; lead with newly searchable articles
per hour and total cost through activation.

1. **Pin and acquire** with `operation: backfill` (offered when `clinical_pmc_raw_archive_receipt` is
   set). Its Batch job pins the next cohort of `clinical_backfill_articles` (at most 100,000) before its
   first checkpoint: in record-tree order, articles the archive settled for the current inventory
   observation (acquired XML, or an exclusion with the metadata it was decided on) and whose other known
   versions are unselected historical work or settled exclusions. The ordinary qualifier re-decides an
   archived exclusion from that metadata without NLM lookups; a cohort that only excludes needs no
   publication. The
   cohort is one immutable operations object `{ inventory, articles: [{ pmcid, versions }] }` (at most
   200,000 version hints), saved in the PMC observation; no unrelated refresh work fills it. A later
   backfill run resumes an unfinished cohort, and a finished cohort that is not yet frozen blocks the
   next pin, so each candidate holds one cohort. Article work stops after
   `clinical_backfill_time_budget_seconds` (8 hours by default); the task allows 12 hours. A pin with no
   eligible article does no article work. Pinning is the only way a cohort starts: the acquisition
   config accepts no hand-supplied article list. A cohort run works from the saved inventory: it never repeats the
   NLM collection search or the bucket listing, which belong to daily acquisition, so an NLM search
   outage does not stop backfill. While a cohort has `pending` or `unobserved` articles, daily
   acquisition defers PMC; DailyMed, MMWR and USPSTF continue.
2. **Pipeline.** A backfill run may pin and acquire the next cohort while the previous candidate
   publishes; it freezes on the first backfill run after that candidate finalizes.
3. **Freeze.** **`USER_SETTLED`: an article failure must not block successful articles.** Once every
   article has an outcome, freeze the selected articles even if some remain `unresolved`; keep
   `complete: false` and the unresolved count visible. Never overwrite a pending candidate, reset the
   input mid-attempt, hide failures as exclusions or retry indefinitely.
4. **Publish** with `operation: publish` and the frozen candidate's
   `acquisition_id` (required, so the run stays recoverable), sizing the bulk admission and build
   deadlines and `clinical_retrieval_max_passages` against the frozen candidate. Check SQL storage
   (`sql_disk_autoresize_limit_gb`) as well; longer deadlines do not qualify passage count, memory,
   scratch, SQL storage or spend.
5. **Recover** a failed publication with `resume_refresh_id` and `acquisition_id`,
   plus the original `replace_build_id` only when the original publication was a replacement. The
   workflow verifies the original execution is terminal, that its request named exactly this candidate
   (a publication that named none cannot be recovered; publish again), that the candidate is still the
   pending one and that the replacement target matches, then skips admission and build. The activator
   binds the saved build only against its exact predecessor: the one it recorded at preparation, or the
   replacement target. Activation and finalization can be retried without rebuilding. Never overwrite a
   receipt or rebuild to manufacture metadata.

Separating CPU preparation from GPU execution is `OPEN` pending measured benefit and a concrete design
within the existing owners.

## Search service

The Cloud Run service (`gpu_region`, one L4, `clinical_evidence_worker_count` workers, 2 GiB index and
256 MiB metadata cache per worker) reads the active database binding at startup, materializes the
exact model/worker assets and opens the pinned Lance version. Routine source refreshes need no image
build or revision rollout: workers swap generations without closing in-flight work, readiness fails
closed while the loaded build differs from the active binding, and queued requests keep their own
version. Worker failures report only stage, exception class, duration and native version.

Serving placement is changing (readiness: *Serving placement*); until then:

- A serving-image rotation creates and probes a new revision that materializes the active binding, so
  verify Cloud SQL is `RUNNABLE` before applying a serving digest and keep the proof in the same
  maintenance window.
- If an activation-order failure leaves the desired image on a failed revision, activate the exact
  build first, then create one same-image recovery revision with an explicit suffix.
- `proof_window_active` keeps one instance resident for a bounded proof; `clinical_evidence_service_paused`
  disables requests while retaining the service.

**Serving replicas** (`serving-replicas.tf`; test windows only before launch). `clinical_serving_replica_count = 2`
runs two L4 servers in separate `us-east1` zones behind an internal load balancer. Each copies the active build's
table version onto its own disk, checking every file, and is ready only once it serves it. The Cloud Run service then
drops its GPU and only authenticates and relays to the load balancer's fixed address. Setting the count back to 0
removes the replicas and the load balancer and restores the GPU service. Replicas need a build whose worker
reads a local copy (#242).

A test window is one `verify_serving` execution, so Google stops the servers even if the operator disappears:
1. With scheduling paused, submit `verify_serving` with a `window_seconds` that covers the run, and wait for SQL.
   A replica reads the active build from SQL on its first readiness check, so SQL comes first.
2. Apply the saved plan at 2 replicas. Measure, then send the callback.
3. The close drains the group and parks SQL. If the window ends without a callback, the same close runs at its
   deadline. Either way, the load balancer and the count of 2 stay in Terraform until the plan at 0 is applied. That
   plan restores the GPU service, whose new revision fails its startup probe against parked SQL and stays at manual
   zero (*Search service*, above).

## Maintenance operations

`USER_SETTLED`: the workflow owns maintenance deadlines and cleanup. Maintenance is admitted only
while daily scheduling is disabled, desired SQL activation is `NEVER`, and the migration job and
service are provisioned. Publication and maintenance share one conditional execution window;
acquisition and rights reconciliation may run alongside it.

| Operation | What it does |
| --- | --- |
| `backfill` | Pin (or resume) one historical PMC cohort and acquire it with DailyMed, MMWR and USPSTF; the candidate waits for an explicit `publish` |
| `publish` | Admit, build, activate and finalize the pending candidate, named by `acquisition_id` (recovery inputs above) |
| `migrate` | Start SQL, run the clinical migrator, park SQL. Schema changes are additive, so this is allowed while a source candidate is pending; it holds the publication window, so it never overlaps a publication |
| `convert` | Start SQL, run the clinical migrator, build with an exact replacement target, activate, park SQL (6,300-second work budget) |
| `rights` | Record explicit source rights observations |
| `verify_serving` | Start SQL and wait `window_seconds` (1,800 by default, at most 14,400) for an authenticated callback with `{"outcome":"verified"}`; the URL appears in the `awaiting_caller_verification` log entry and requires `workflows.callbacks.send`. On success or failure, the workflow starts the fixed `coda-clinical-serving-close` job at most once, then cleanup parks SQL; a close that is not confirmed holds the publication window for recovery, and retrying SQL cleanup never starts another close. The job drains the replica group to zero, then sets the service to manual zero. Only that job's identity can change either; the workflow can only run the job, with no overrides |

For any authorized maintenance:

1. Bind the exact project, database, images, artifact, operation and budget. Check for active owners;
   preserve the source baseline and last immutable runtime package.
2. Submit the operation to `coda-clinical-corpus-refresh` and keep its execution ID and revision.
   Begin caller work only after the callback window is announced; on proof failure send a failed
   outcome to start cleanup.
3. Submit long Cloud Build/Cloud Run operations with `--async` and save the ID; reconcile ambiguous
   submissions before retrying.
4. Do not apply a temporary `sql_activation_policy = "ALWAYS"` plan without examining its dependencies:
   it changes workflow SQL management and can remove the SQL-control grant.
5. Verify exact activation before asking a new revision to load it. An image rollback does not undo
   corpus admission.
6. Measure cold readiness and warm queries separately; the startup probe has a 240-second window.
7. Finish by verifying the exact SQL stop operation is `DONE` without error and activation is `NEVER`
   (`RUNNABLE` in the API can mean owner-stopped; gcloud shows `STOPPED`).

**Execution windows.** The state object's `workflow_windows` bound one Batch acquisition and one
publisher. SQL users register under a short exclusive power-transition window (at most 32), so the
last user parks SQL without racing a new start. An interrupted owner is retained for exact recovery,
never replaced because a lease timer expired. Direct manual job invocation does not acquire windows.

**Interrupted-owner recovery.** Inspect each exact native execution, settle its Batch/Cloud Run jobs and
any SQL control operation, then release only its matching acquisition, publication, SQL-user or
transition fields with the current GCS generation precondition. Never clear another live owner or
overwrite the whole state. Recheck parked SQL and remaining users before resuming scheduling.
A `serving_close_unresolved` failure is recovered the same way. Its close start failed or went
unanswered, so a `coda-clinical-serving-close` run may still appear. Wait until every run of that job
is terminal, confirm serving is at manual zero, then release the publication window. The workflow
never starts a second close run on its own.

**Pending-candidate retirement** is only for a reviewed, unrecoverable candidate — not a retry,
timeout, SQL rollback or substitute for fixing the owning source contract.

1. Disable the scheduler and prove no workflow or pipeline job is active.
2. Read the GCS state generation, admitted baseline and exact pending acquisition ID; preserve the
   failing receipts.
3. Inside the maintenance window, inspect `clinical_library.sync_runs` joined to `sources` for every
   candidate source and the active build binding. Retirement undoes none of them.
4. Run `dist/src/server/clinicalCorpus/operations/retireHostedClinicalSourceCandidate.js` under the source-finalization job
   identity with an execution-time override supplying `CODA_CLINICAL_ACQUISITION_ID`,
   `CODA_CLINICAL_CANDIDATE_RETIREMENT_DECIDED_AT`, `CODA_CLINICAL_DECISION_ACTOR` and
   `CODA_CLINICAL_CANDIDATE_RETIREMENT_REASON`. Do not change the job definition.
5. Require a terminal `retired` result, then read back: `pending` is null, `baseline` is unchanged and
   `last_candidate_retirement` names the exact acquisition and receipt.
6. Park SQL and restore scheduling; release sources reacquire from the retained baseline. For a
   pinned PMC backfill, daily acquisition holds its unpublished work. The next `backfill` freezes
   the saved PMC predecessor boundary before resuming any successor cohort. The successor's moving
   head and checkpoints remain intact; after the recovered
   predecessor publishes and finalizes, the next `backfill` resumes or freezes the successor alone.
   Older observations without a predecessor binding use the exact retirement receipt; if it cannot
   establish that boundary, backfill stops for operator recovery rather than combining cohorts.

`pnpm clinical:retire-hosted-source-candidate` runs the same entry point for bounded local proof.

## Deploying code and schema

Rotate the maintainer/activator and GPU builder/serving image digests only for application or
dependency changes. A rotation is one rollout, approved as a whole: build the image, bind its digest, and read
back the next artifact it produces. `infra/gcp/runtime/cloudbuild.runtime.yaml` builds code and dependency images and
never downloads retrieval bytes; image smoke checks run Python entry points with `--help`. Pass
`--gcs-source-staging-dir=gs://<cloud_build_source_bucket>/source` and
`--service-account=<cloud_build_service_account>` to `gcloud builds submit`; builds run as that
identity, which can only read staged source, push to the clinical repository and write logs.

Every apply uses a saved plan kept outside the repository, never `-auto-approve`. Terraform state,
plans, credentials, database exports, retrieval archives, model output and proof receipts stay outside
Git. Migrations and conversion run in the workflow; migrate only the clinical database with the
clinical migrator. A release binds the new migration image, then runs the `migrate` operation. Every
migration stays backward compatible with the code that is already running. Roll out builder and
activator together when their contract changes. New images alone do not authorize a corpus
conversion.

**Initial provisioning** (one time). Cloud SQL cannot be created stopped: apply a reviewed plan with
`sql_activation_policy = "ALWAYS"`, create the database and IAM users, and leave the service disabled.
Then build and push the images, bind their digests in a second plan, run the migration job, import
the approved content-addressed dump with `--user=coda-clinical-migrator@coda-health-prod.iam`, re-run
migrations, activate one exact receipt, enable the service with explicit invokers, prove anonymous
denial, readiness, search and exact read through the local remote adapter, and return SQL to `NEVER`.
Enable the schedule only after one controlled and one unchanged end-to-end execution succeed, with
`source_maintenance_image`, `clinical_source_acquisition_bootstrap`, `clinical_mmwr_index_urls` and
`clinical_source_acquisition_enabled` reviewed together.

## Identities

Each job runs as its own service account; trace every call under that identity, including failure
cleanup. The workflow identity reads build receipts, manages windows, runs and settles jobs and
starts/parks SQL. The activator reads the receipt and immutable publication descriptor, read-only,
without index-archive or payload access. Source finalization reads and replaces the acquisition
checkpoint. The builder has read-only corpus access and update authority only under `builds/lance/`.
A new read ships with its resource-scoped grant. Mocked responses and plans cannot prove workload
authorization; account for IAM propagation, and establish permissions from the call path before any
paid run.

## Cost bounds and cleanup

Mechanical bounds are the one-instance, scale-to-zero service; parked SQL; jobs with task deadlines that
rerun at most twice, only when Batch lost the VM (exit 50003 rebooted, 50006 recreated); and SQL storage auto-growth capped by `sql_disk_autoresize_limit_gb` (default 50 GB; production
runs at 250 GB for the backfill, and the variable accepts at most 500 GB). The disk grows only as data
arrives and never shrinks. Budgets alert but do not cap spend. Storage,
images, network transfer and SQL disk bill independently of job timeouts. The build-source bucket
deletes objects after one day with soft delete disabled. Artifact Registry deletes untagged versions
after seven days while keeping the two most recent per package.

Cleanup reconciles native execution history, cancels only jobs carrying the execution ID, and requires
terminal readback before parking SQL. Registered foreign SQL users keep their window and the last user
parks. Failed cleanup retains its execution references. Control-plane failures, revoked permissions or
explicit cancellation can still prevent cleanup; this is not an absolute billing cap.

**Completion evidence** — keep these separate:

- **Code/schema:** exact source commit, image digests, migration names and checksums.
- **Index:** terminal execution, immutable publication, artifact identity, canonical and passage
  counts, fingerprints and exact activation.
- **Caller:** authenticated `/ready` binds the expected build; remote-gateway searches and ordered exact
  reads preserve IDs and content hashes. Selected probes do not prove clinical recall.
- **Freshness:** DailyMed, PMC, MMWR, USPSTF and NLM observations individually, with checked-through dates,
  partial failures and backlog; candidate finalization and activation agree.
- **Cleanup:** SQL `STOPPED`/`NEVER`, serving minimum zero, no task-owned jobs left, no one-time inputs
  in job configuration, schedule and alerts restored, and the final full plan empty.

## Configuration reference

| Variable | Default | Purpose |
| --- | --- | --- |
| `region` / `gpu_region` | `us-east1` / `us-east4` | Database, bucket and jobs / L4 Cloud Run service |
| `sql_tier` | `db-custom-1-3840` | Corpus database machine |
| `sql_activation_policy` | `NEVER` | Parked outside windows; `ALWAYS` only for initialization or a named window |
| `sql_disk_autoresize_limit_gb` | 50 | Hard SQL storage ceiling (20–500; production 250) |
| `clinical_retrieval_max_passages` | 1,000,000 | Pre-inference passage guard (≤ 46 million) |
| `clinical_retrieval_build_batch_size` | 64 | Document-embedding batch size (1–256) |
| `clinical_evidence_worker_count` | 2 | Resident search workers per instance |
| `clinical_source_acquisition_schedule` | `30 22 * * *` | Daily refresh start (America/New_York) |
| `clinical_source_acquisition_time_budget_seconds` | 7200 | Daily PMC article-work window after the inventory checkpoint (≤ 20 hours) |
| `clinical_bulk_admission_timeout_seconds` | 7,200 | Bulk admission deadline (2–12 hours) |
| `clinical_bulk_build_timeout_seconds` | 14,400 | Bulk GPU build deadline (4–18 hours) |
| `clinical_source_acquisition_enabled` / `clinical_source_scheduler_enabled` | `false` | Provision acquisition / run the daily schedule after end-to-end proof |
| `clinical_bibliographic_contact_email` | `null` | Enables the PubMed status refresh (NCBI contact) |
| `clinical_uspstf_enabled` | `false` | Acquires USPSTF with the key from its secret; set only after the secret has a version |
| `proof_window_active` | `false` | Keeps one search instance resident for a bounded proof |
| `clinical_serving_replica_count` | 0 | Serving replicas (0 or 2); 2 only in a named test window before launch |
