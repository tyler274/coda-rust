# Synthetic patient bank

Fictional adult patients of one primary-care practice in western Massachusetts,
each a longitudinal chart in FHIR R4 NDJSON in the Coda Agents Roger fixture's
format. Every chart ends at the bank's today, 2026-08-27T12:00 Eastern. Nothing
here is real patient data, and nothing is physician reviewed.

## Pipeline

```text
patients/seeds.json            who each patient is
  -> data/author-workflow.js   one author agent per unit (a patient, or a family):
                               story.md, ledger parts, `check` until ACCEPTED, `preview`
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
`{repo, units}` as args; each agent follows `data/authoring-guide.md`. The
patients live apart from the tool in the workspace's `patients/`. Drafts, the bank
and the schema live under the ignored `target/patient-bank/`. `PATIENT_BANK_DATA`,
`PATIENT_BANK_PATIENTS` and `PATIENT_BANK_ROOT` move the three.

## What the code owns

- **The ledger format.** Typed structs that reject unknown fields, misspelled
  actions and malformed times, so a bad draft fails with its path.
- **Time.** `timeline.rs` gives every record its own instant, and a chart as of a
  moment holds exactly the records before it. Drug and problem status come from
  each concept's whole history: a restarted drug stops at its own stop, and a
  resolved problem that returns is a recurrence.
- **Today.** Everything is dated before the bank time. A draw not back yet is
  `pending` (no values, `registered`), and a booked visit is an empty event on the
  practice calendar. What happens after today is decided later, when the patients
  are run.
- **Codes.** `data/catalog.json` holds verified ICD-10-CM, LOINC, RxNorm, SNOMED,
  CVX and CPT codes. `check` verifies any extra condition code or drug text an
  author proposes and stores the answer in the ledger, so build and validate need
  no network.
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
- **The renderer.** Encounters, Conditions, MedicationRequests, Observations
  (lab panels with `hasMember`, BP with components), DiagnosticReports,
  Procedures, AllergyIntolerances, DocumentReferences, Immunizations and
  ServiceRequests, with UTC timestamps and opaque ids. Unstated panel members get
  per-patient normal values; eGFR (CKD-EPI 2021), CBC indices, the differential's
  remainder and other derived values are calculated.

## Reused

- **Format:** the Roger fixture in coda-agents `docs/local-product-fixtures`: its
  families, MRN system, note header, code systems and CPT displays.
- **Lab labels and ranges:** coda-gym's `lab_reference_ranges.xlsx`
  (`data/gym-lab-ranges.json`).
- **Validation:** the official HL7 R4 4.0.1 JSON schema.
