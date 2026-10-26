# Patients

The synthetic patient set of the primary-care practice, kept apart from the tool in
`crates/patient-bank`. Nothing here is real patient data.

Each patient is made from one adult patient of
[synthetic_hospital](https://github.com/sparkcpark/synthetic_hospital)
(`benchmark_v1.3.db`, MIT licence, fully synthetic), placed in the practice. Every
unit has a folder, `units/unit-<id>/`, holding everything about it:

- `seed.json`: who the patient is in the practice (fictional name, town and contact
  details; age, sex, race and insurance from the source; and in `chart_texture`,
  whether the patient's outside records arrive complete, with gaps or sparse).
- `source.md`: synthetic_hospital patient `<id>`, exported once, with each
  encounter's practice date (the source span moved to end before the practice's
  today, 2026-08-27, and off the 15th of the month).
- `build-req.md`: what the chart may hold, every clinical fact traced to the source,
  a listed resolution or addition, or the primary-care layer. A unit whose source
  cannot be a living primary-care patient says `UNFIT` and has no chart.
- `story.md` and `part-*.json`: what the author wrote.
- `ledger.json`: the accepted chart, written only by `check`, with every number the
  chart shows and every out-of-catalog code it verified.

All 1,020 adult sources are here with their seeds; a unit gains the rest as its chart
is written and accepted. Units `unit-2013`, `unit-2188` and `unit-2797` were the
pilots; they keep the MRNs they were authored with.
