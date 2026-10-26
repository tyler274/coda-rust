# Synthetic patients

The patient bank is a set of fictional adult patients of one primary-care
practice, each a longitudinal chart in FHIR R4. Coda runs against them where it
needs realistic patients. Nothing in it is real patient data, and no physician
has reviewed it.

## Where the patients come from

- **The people:** [synthetic_hospital](https://github.com/sparkcpark/synthetic_hospital)
  (`benchmark_v1.3.db`, MIT, fully synthetic). Each of its 1,020 adult patients is
  the source of one patient in the practice. Because they are in the bank, Coda is
  never evaluated on synthetic_hospital's own benchmark.
- **The method:** coda-gym's. A chart is written from a requirements file that says
  what it may hold, so every clinical fact comes from the source or a listed
  addition, and nothing is invented.
- **Lab labels and reference ranges:** coda-gym's `lab_reference_ranges.xlsx`.
- **The format:** the Coda Agents Roger fixture in
  `submodules/coda-agents/docs/local-product-fixtures`: its resource families, MRN
  system, note header and code systems.

## What a chart is

The practice is a membership internal medicine practice in western
Massachusetts, and its today is 2026-08-27 12:00 Eastern. Each source becomes the
primary-care chart the same person would have there:

- What the source says stays: its diagnoses, medications, results and events,
  rare ones included. Its internal and family medicine visits become the
  practice's own; specialist, emergency and hospital care arrives as outside
  records, and how complete those records are varies by patient.
- The practice adds what primary care does over the years: regular visits,
  preventive care, medication management, labs over time, notes in its own voice
  and visits booked after today.
- A source diagnosis with no treatment gets its standard care. After the source
  ends, the patient follows the ordinary course, with no new diagnoses.
- Dates move so the history ends before today.
- A source who cannot be a living primary-care patient has no chart.

Vitals, lab values and calculated results are made by code from the patient's
state, never typed by an author.

## Where it lives

| What | Where |
| --- | --- |
| The tool: authoring, checks, FHIR build and validation | [`crates/patient-bank`](/crates/patient-bank/src/lib.rs); its <a href="/crates/patient-bank/README%2Emd">README</a> has the commands and what the code owns |
| The patients, one folder each: the seed, the source, the requirements, the author's drafts and the accepted chart | `patients/units/<unit>/` |
| The FHIR bank, previews and caches, all derived | `target/patient-bank/`, ignored and rebuilt |

Charts are written in batches; a unit folder without `ledger.json` has no chart
yet.
