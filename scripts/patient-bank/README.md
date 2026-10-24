# Synthetic patient bank

About 1,000 fictional adult patients of one longitudinal primary-care practice in
western Massachusetts, each a dense, textured chart in strict FHIR R4 NDJSON in
the Coda Agents Roger fixture's format. They are just patients: scenarios and
journeys tied to the product are added later, separately. Nothing here is real
patient data, and nothing is physician reviewed.

## Pipeline

```
design_panel.py   seeds.json: identity, demographics, complexity, targets, anchors, episodes, texture
      |
author-workflow.js  one agent per unit (a patient, or a family authored together):
      |               story.md first, then the ledger in parts, unit.py check until ACCEPTED,
      |               unit.py preview and reread; then an independent physician-eye review
      |               and, for major findings, a fix pass
      v
ledgers/<unit>.json   the clinical source of truth (written only by unit.py check)
      |
build_bank.py     render.py encodes every ledger as FHIR R4 as of 2026-08-27T12:00 Eastern;
      |           panels/panel-{100,300,500,1000}.json (nested, families kept whole); manifest.json
      v
validate.py       bulk checks and validation.json
materialize.py    one panel as a multi-patient NDJSON directory, optionally as of an earlier instant
```

The renderer is the only owner of FHIR encoding and of time: an earlier snapshot is
the same ledger rendered with an earlier `as_of`, so later records are absent,
later stops and resolutions have not happened, and results not yet released are
`registered`.

```
python3 -I scripts/patient-bank/design_panel.py --count 995 --first-mrn 6 --out target/patient-bank/seeds.json
# author: run author-workflow.js through Claude Code's Workflow tool with a list of unit ids as args
python3 -I scripts/patient-bank/build_bank.py --seeds target/patient-bank/seeds.json \
    --ledgers target/patient-bank/ledgers --out target/patient-bank/bank \
    [--roger submodules/coda-agents/docs/local-product-fixtures/internal_resources]
python3 -I scripts/patient-bank/validate.py --bank target/patient-bank/bank \
    --seeds target/patient-bank/seeds.json --schema <HL7 fhir.schema.json R4 4.0.1>
python3 -I scripts/patient-bank/materialize.py --bank target/patient-bank/bank \
    --seeds target/patient-bank/seeds.json --panel 100 --out target/patient-bank/out/panel-100 [--as-of 2024-03-01T09:00]
```

Outputs live under the ignored `target/patient-bank/`.

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
the years they can have. `unit.py check` rejects a chart below 80% of either target.

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
  `lab_reference_ranges.xlsx` (`gym-lab-ranges.json`); vital codes come from its
  `vital_fields.md`.
- **Realism ideas:** the synthetic_hospital project, for clinical voices, between-visit
  documentation and chart imperfections. None of its data is used.
- **Official HL7 R4 4.0.1 JSON schema:** used for validation.

New:
- The verified terminology catalog (`catalog_seed.py`, `build_catalog.py`, `catalog.json`).
  Every ICD-10-CM, LOINC, RxNorm, SNOMED and HCPCS code is read back from NLM
  Clinical Tables, RxNav or tx.fhir.org, and lookups are cached in
  `terminology-cache.json`. CPT has no public service: codes outside the fixture are
  marked `recalled` and carry no display.
- The seed design, authoring guide, agent tool, renderer, builder, validator and
  materializer.

Not reused: the earlier Synthetic Hospital import package. The later 74,799-resource
corpus was quarantined for future-fact leakage, and its charts did not meet the
realism bar.

## Assumptions

- Adults only, in one practice. The practice's "today" is 2026-08-27, and every
  record is strictly before 2026-08-27T12:00 Eastern.
- Western Massachusetts towns, 413-555-01xx-style phones and `@example.test` emails.
  Other area codes appear for people who moved in.
- Clinicians appear by role only. No real care organization, lab, pharmacy, store or
  EHR or portal product is named. Drug, vaccine and assay product names stay, as in
  real charts.
- Unstated panel members are filled with per-patient normal baselines that vary from
  draw to draw, and the renderer receipts list them. Hematocrit, RBC, MCH and MCHC are
  derived from hemoglobin and MCV. The unstated differential is filled so it totals
  100%. eGFR (CKD-EPI 2021), anion gap, cholesterol totals and transferrin saturation
  are derived.
- Turnaround is about two days for cultures and three to eight days for pathology and
  cytology unless the ledger sets `resulted` or `issued`.
- Drugs that are not in the catalog are coded only when RxNav resolves them with a
  matching ingredient and strength. Otherwise they stay text-only and are counted.
- No scenarios, answer keys or evaluation splits. The panels are plain nested
  membership lists.
- The bank includes the five Roger fixture patients (MRN00000001-5) verbatim only when
  `build_bank.py --roger` is passed. With them, 995 authored patients make 1,000.

## Lineage

Every patient's manifest entry records the seed, the author model (Claude Opus 5.5
as a Claude Code workflow subagent), the authoring guide, ledger and file hashes,
and "not physician reviewed; generated, not clinically validated".
