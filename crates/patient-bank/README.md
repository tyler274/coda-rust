# Synthetic patient bank

Fictional adult patients of one primary-care practice in western Massachusetts,
each a longitudinal chart in FHIR R4 NDJSON in the Coda Agents Roger fixture's
format. Each patient is made from one adult patient of
[synthetic_hospital](https://github.com/sparkcpark/synthetic_hospital) (MIT, fully
synthetic), placed in the practice. Every chart ends at the bank's today, 2026-08-27T12:00 Eastern. Nothing
here is real patient data, and nothing is physician reviewed.

## Pipeline

```text
patients/units/<unit>/source.md   the synthetic_hospital patient the unit is made from
patients/seeds.json            who each patient is in the practice
  -> data/author-workflow.js   one agent per unit: build-req.md (what the chart may
                               hold, traced to the source), then story.md and ledger
                               parts, `check` until ACCEPTED, `preview` and a reread
  -> patients/ledgers/<unit>.json   the clinical source of truth, written only by `check`
  -> build                     bank/patients/<MRN>/<Family>.ndjson, bank/calendar/
                               Appointment.ndjson (visits from today on), manifest.json
  -> validate                  HL7 schema, ids, references, codes, times, and what the
                               charts repeat from one another; validation.json
```

```bash
cargo run -q -p patient-bank -- brief|check|preview <unit>
cargo run -q -p patient-bank -- build
cargo run -q -p patient-bank -- validate [--schema <HL7 fhir.schema.json R4 4.0.1>]
cargo run -q -p patient-bank -- catalog [--offline]
```

Authoring runs `data/author-workflow.js` through Claude Code's Workflow tool with
`{repo, units}` as args; each agent follows `data/authoring-guide.md`, which sends
it to `data/build-req-guide.md` first. The patients live apart from the tool in the
workspace's `patients/`. Previews (chart.txt, taken.md), the bank and the schema live
under the ignored `target/patient-bank/`. `PATIENT_BANK_DATA`,
`PATIENT_BANK_PATIENTS` and `PATIENT_BANK_ROOT` move the three.

## What the code owns

- **The ledger format.** Typed structs that reject unknown fields, misspelled
  actions and malformed times, so a bad draft fails with its path.
- **Time.** `timeline.rs` gives every record its own instant, and a chart as of a
  moment holds exactly the records before it. Drug and problem status come from
  each concept's whole history: a restarted drug stops at its own stop, and a
  problem diagnosed again after it resolved is a new episode with its own row.
- **Today.** Everything is dated before the bank time. A draw not back yet is
  `pending` (no values, `registered`), and a booked visit is an empty event on the
  practice calendar. What happens after today is decided later, when the patients
  are run.
- **Codes.** `data/catalog.json` holds verified ICD-10-CM, LOINC, RxNorm, SNOMED,
  CVX and CPT codes. `check` verifies any extra condition code, drug text, or lab or
  study LOINC an author proposes and stores the answer in the ledger, so build and
  validate need no network.
- **Repeats.** Authors write alone, so they fall back on the same defaults.
  `repeats.rs` finds the names and phrases a few charts share (the practice's own
  templates are shared by many and do not count) and lab values far more common
  than the values beside them. `validate` reports them for the bank, and `brief`
  writes each author a `taken.md` from every other chart.
- **The check.** `check` holds chronology and codes only: the patient matches
  the seed, every label is the catalog's, nothing is after today but pending
  results and booked visits, nothing stops, resolves or closes before it started,
  results come after their draw, and lab arithmetic is what an analyzer reports.
  It does not judge how a chart reads; the author's reread of `chart.txt` does.
- **Numbers.** `measure` makes every number the chart shows, at `check`, and
  writes it into the ledger. A contact's vitals and each draw's results read the
  patient's level (set in `state`, or the patient's own normal) plus the scatter a
  cuff or an analyzer gives. Unstated panel members are measured too, the open
  differential shares what is left of 100%, and red-cell indices, eGFR (CKD-EPI
  2021), anion gap, cholesterol totals, transferrin saturation and prothrombin
  time are calculated from the draw. A level set on a calculated test moves the
  inputs it comes from.
- **The renderer.** Encounters, Conditions, MedicationRequests, Observations
  (lab panels with `hasMember`, BP with components), DiagnosticReports,
  Procedures, AllergyIntolerances, DocumentReferences, Immunizations and
  ServiceRequests, with UTC timestamps and opaque ids, from the ledger as it is.

## Reused

- **Format:** the Roger fixture in coda-agents `docs/local-product-fixtures`: its
  families, MRN system, note header, code systems and CPT displays.
- **Lab labels and ranges:** coda-gym's `lab_reference_ranges.xlsx`
  (`data/gym-lab-ranges.json`).
- **Validation:** the official HL7 R4 4.0.1 JSON schema.
