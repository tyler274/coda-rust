# Synthetic patient bank

About 1,000 fictional adult patients of one longitudinal primary-care practice in
western Massachusetts, each a dense, textured chart in strict FHIR R4 NDJSON in
the Coda Agents Roger fixture's format. They are just patients: scenarios and
journeys tied to the product are added later, separately. Nothing here is real
patient data, and nothing is physician reviewed.

## Pipeline

```text
data/seeds.json      who each patient is: identity, demographics, complexity, targets,
      |              anchors, episodes, texture, life context
stories              draws a concrete clinical story from data/stories.json into every
      |              seed without an accepted ledger; authored seeds never change
data/author-workflow.js
      |              one agent per unit (a patient, or a family authored together):
      |              story.md first, then the ledger in parts, `check` until ACCEPTED,
      |              `preview` and reread; batches are then reviewed by hand
      v
ledgers/<unit>.json  the clinical source of truth, written only by `check`, with the
      |              unit's verified extra codes (`terms`)
build                renders every ledger as FHIR R4 as of 2026-08-27T12:00 Eastern;
      |              panels/panel-{100,300,500,1000}.json (nested, families kept whole);
      |              manifest.json
      v
validate             bulk checks and validation.json
materialize          one panel as a multi-patient NDJSON directory, optionally as of an
                     earlier moment
```

```bash
cargo run -q -p patient-bank -- stories
cargo run -q -p patient-bank -- brief|check|preview <unit>
cargo run -q -p patient-bank -- build [--roger submodules/coda-agents/docs/local-product-fixtures/internal_resources]
cargo run -q -p patient-bank -- validate [--schema <HL7 fhir.schema.json R4 4.0.1>]
cargo run -q -p patient-bank -- materialize --panel 100 --out target/patient-bank/out/panel-100 [--as-of 2024-03-01T09:00]
cargo run -q -p patient-bank -- catalog [--offline]
```

Authoring runs `data/author-workflow.js` through Claude Code's Workflow tool with
`{repo, units}` as args. Inputs live in `data/`; drafts, ledgers, the bank and the
schema live under the ignored `target/patient-bank/`. `PATIENT_BANK_DATA` and
`PATIENT_BANK_ROOT` move the two, and `build` and `materialize` refuse an output
directory that overlaps their inputs. The build reads every input first and swaps
the finished directory into place.

### What the code owns

- **The ledger format.** Part files and ledgers parse into typed structs that
  reject unknown fields, misspelled actions and times not written
  `YYYY-MM-DDTHH:MM`, so a malformed draft fails with its path instead of losing
  content.
- **Time.** `timeline.rs` is the only owner of time. Every record has its own
  instant (an order's date, a vital set's time, a result's release), and a chart as
  of a moment holds exactly the records before it. Medication and problem status
  come from each concept's whole history: a restarted drug stops again at its own
  stop, and a resolved problem that returns is a `recurrence`. An earlier snapshot is
  the same ledger rendered as of that moment, so later records are absent, later
  stops and resolutions have not happened, and results not yet released are
  `registered`.
- **Verified extra codes.** `check` verifies the ICD-10-CM codes and drug texts an
  author proposed and stores the answers in the ledger, so build, validate and
  materialize need no network. A drug text is coded only when its strength is
  equivalent to the RxNorm drug's (`3 mg/0.5 mL` is 6 mg/mL, never a 3 mg/mL pen);
  text with no strength stays text-only. Each unit's new lookups go to its own
  `drafts/<unit>/lookups.json`; only `catalog` writes the shared cache.
- **A varied practice.** `stories` gives each unwritten patient common conditions
  at primary-care rates, minor problems and smoking history at their own rates, a
  distinctive long-tail condition shared by at most five patients, for some a
  hospital-grade event (shared by at most three), concrete episodes (which injury,
  what the colonoscopy found) and a life context. Labels that exclude one another
  share a group in `data/stories.json`, and every label is a verified catalog
  condition. Those seeds carry `coded_anchors`: `check` requires every anchor on
  the problem list and allows at most two other problems still active at the bank
  time. `validate` reports how varied the active problem lists are across the
  panel (`problem_list` in validation.json).
- **Roles, not names.** `check` rejects a note or title that names a clinician or
  staff member ("Dr. Okafor", "M. Lindqvist, MD", "-- B. Kowal").

## Density

Targets were measured from real reference charts: coda-gym's 34 charts span 10-15
years with 25-280 notes (median about 108) and 100-900 observations (median about
320); the Roger fixture averages 64 notes and 456 observations per patient.

| tier | years with the practice | notes | observations |
|---|---|---|---|
| light | 5-8 | 40-80 | 200-500 |
| moderate | 8-15 | 80-150 | 500-1,200 |
| complex | 12-20 | 150-300 | 1,000-2,500 |

Young adults join the adult practice no earlier than 18; their targets scale with
the years they can have. `check` rejects a chart below 80% of either target.

## What is reused and what is new

Reused:
- **Format bar:** the Roger fixture in coda-agents `docs/local-product-fixtures`. Its
  eight FHIR families, MRN identifier system, note header, code systems and
  simulation time are followed. Its CPT codes and displays are the only CPT
  displays used.
- **Authoring contract:** coda-gym's `eval-chart` chart-build flow. The patient's truth
  is authored first and then translated to strict FHIR R4, with UTC timestamps
  derived from Eastern wall-clock time, opaque IDs, BP as a panel with components,
  lab panels as a DiagnosticReport plus a panel Observation with `hasMember`, and
  non-lab reports with `presentedForm`. Lab labels and ranges come from coda-gym's
  `lab_reference_ranges.xlsx` (`data/gym-lab-ranges.json`); vital codes come from its
  `vital_fields.md`.
- **Realism ideas:** the synthetic_hospital project, for clinical voices, between-visit
  documentation and chart imperfections. None of its data is used.
- **Official HL7 R4 4.0.1 JSON schema:** used for validation.

New:
- The verified terminology catalog (`src/catalog_seed.rs`, `catalog`, `data/catalog.json`).
  Every ICD-10-CM, LOINC, RxNorm, SNOMED and HCPCS code is read back from NLM
  Clinical Tables, RxNav or tx.fhir.org, and lookups are cached in
  `data/terminology-cache.json`. CPT has no public service: codes outside the fixture are
  marked `recalled` and carry no display.
- The seeds, authoring guide, authoring commands, renderer, builder, validator and
  materializer.

Not reused: the earlier Synthetic Hospital import package. The later 74,799-resource
corpus was quarantined for future-fact leakage, and its charts did not meet the
realism bar.

## Assumptions

- Adults only, in one practice. The practice's "today" is 2026-08-27, and every
  record is strictly before 2026-08-27T12:00 Eastern.
- Western Massachusetts towns, 413-555-01xx-style phones and `@example.test` emails.
  Other area codes appear for people who moved in.
- Clinicians and staff appear by role only. No real care organization, lab, pharmacy, store or
  EHR or portal product is named. Drug, vaccine and assay product names stay, as in
  real charts.
- Unstated panel members are filled with per-patient normal baselines that vary from
  draw to draw, and the renderer receipts list them. Hematocrit, RBC, MCH and MCHC are
  derived from hemoglobin and MCV. The unstated differential is filled so it totals
  100%. eGFR (CKD-EPI 2021), anion gap, cholesterol totals and transferrin saturation
  are derived.
- Turnaround is about two days for cultures and three to eight days for pathology and
  cytology unless the ledger sets `resulted` (on a draw or one result) or `issued`.
- Drugs that are not in the catalog are coded only when RxNav resolves them with a
  matching ingredient and strength. Otherwise they stay text-only and are counted.
- No scenarios, answer keys or evaluation splits. The panels are plain nested
  membership lists.
- The bank includes the five Roger fixture patients (MRN00000001-5) verbatim only when
  `build --roger` is passed. With them, 995 authored patients make 1,000.

## Lineage

Every patient's manifest entry records the seed, the author model (Claude Opus 5.5
as a Claude Code workflow subagent), the authoring guide, ledger and file hashes,
and "not physician reviewed; generated, not clinically validated".
