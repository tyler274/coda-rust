# Patients

The synthetic patient set: one folder per patient, `units/unit-<id>/`, where
`<id>` is the synthetic_hospital patient it is made from. Where the patients come
from and what a chart holds is in
[Synthetic patients](../book/src/patient-bank.md); the tool that writes and checks
them is `crates/patient-bank`. Nothing here is real patient data.

- `seed.json`: who the patient is in the practice.
- `source.md`: the synthetic_hospital patient, with each encounter's practice date.
- `build-req.md`: what the chart may hold, traced to the source. A unit whose
  source cannot be a living primary-care patient says `UNFIT` and has no chart.
- `story.md` and `part-*.json`: what the author wrote.
- `ledger.json`: the accepted chart, written only by `check`.

A folder without `ledger.json` has no chart yet. Units `unit-2013`, `unit-2188`
and `unit-2797` were the pilots and keep the MRNs they were authored with.
