# Synthetic patient bank

The tool that writes, checks and builds the synthetic patient bank. Where the
patients come from and what a chart holds is in
[Synthetic patients](../../book/src/patient-bank.md); this file is how the tool
works.

## Pipeline

```text
import --db benchmark_v1.3.db  adds each adult's source.md and a seed drawn from
                               it (name from data/name-pool.json, town, practice
                               dates, last seen, booked); never overwrites
patients/units/<unit>/
  seed.json                    who the patient is in the practice
  source.md                    the synthetic_hospital patient the unit is made from
  -> data/author-workflow.js   one agent per unit: build-req.md (what the chart may
                               hold, traced to the source), then story.md and ledger
                               parts, `check` until ACCEPTED, `preview` and a reread
  -> ledger.json               the clinical source of truth, written only by `check`
  -> build                     bank/patients/<MRN>/<Family>.ndjson, bank/calendar/
                               Appointment.ndjson (visits from today on), manifest.json
  -> validate                  HL7 schema, ids, references, codes, times, and what the
                               charts repeat from one another; validation.json
```

```bash
cargo run -q -p patient-bank -- import --db <synthetic_hospital benchmark_v1.3.db>
cargo run -q -p patient-bank -- brief|check|preview <unit>
cargo run -q -p patient-bank -- build
cargo run -q -p patient-bank -- validate [--schema <HL7 fhir.schema.json R4 4.0.1>]
cargo run -q -p patient-bank -- catalog
```

Authoring runs `data/author-workflow.js` through Claude Code's Workflow tool with
`{repo, units}` as args; each agent follows `data/authoring-guide.md`, which sends
it to `data/build-req-guide.md` first. The patients live apart from the tool in the
workspace's `patients/units/`, one folder per unit. Previews (chart.txt, taken.md),
the terminology cache, the bank and the schema live under the ignored
`target/patient-bank/`. `PATIENT_BANK_DATA`, `PATIENT_BANK_PATIENTS` and
`PATIENT_BANK_ROOT` move the three.

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
  CVX and CPT codes. A concept is added by writing its label and code there and
  running `catalog`, which reads every entry back from its official service. `check` verifies any extra condition code, drug text, or lab or
  study LOINC an author proposes and stores the answer in the ledger, so build and
  validate need no network.
- **Repeats.** Authors write alone, so they fall back on the same defaults.
  `repeats.rs` finds the names and phrases a few charts share (the practice's own
  templates are shared by many and do not count) and lab values far more common
  than the values beside them. `validate` reports them for the bank, and `brief`
  writes each author a `taken.md` from every other chart.
- **The check.** `check` holds chronology and codes only: the patient matches
  the seed, every label is the catalog's, nothing is after today but pending
  results and booked visits, the last held practice visit is the seed's
  `last_seen` and nothing after it changes the practice's lists, a visit is booked
  only when the seed says so, nothing stops, resolves or closes before it started,
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
