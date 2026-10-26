# Patients

The synthetic patient set of the primary-care practice, kept apart from the tool in
`crates/patient-bank`. Nothing here is real patient data.

Each patient is made from one adult patient of
[synthetic_hospital](https://github.com/sparkcpark/synthetic_hospital)
(`benchmark_v1.3.db`, MIT licence, fully synthetic), placed in the practice:

- `seeds.json`: who each of the 1,020 patients is in the practice (fictional names,
  towns and contact details; age, sex, race and insurance from the source; and in
  `chart_texture`, whether the patient's outside records arrive complete, with gaps
  or sparse).
- `units/unit-<id>/source.md`: synthetic_hospital patient `<id>`, exported once, with
  each encounter's practice date (the source span moved to end before the practice's
  today, 2026-08-27, and off the 15th of the month).
- `units/unit-<id>/build-req.md`: what the chart may hold, every clinical fact traced
  to the source, a listed resolution or addition, or the primary-care layer. A unit
  whose source cannot be a living primary-care patient says `UNFIT` and has no chart.
- `units/unit-<id>/story.md` and `part-*.json`: what the author wrote;
  `lookups.json`: the code verifications `check` made for it.
- `ledgers/<unit>.json`: the accepted charts, written only by `check`, with every
  number the chart shows.

Units arrive as their charts are accepted. Units `unit-2013`, `unit-2188` and
`unit-2797` were the pilots; they keep the MRNs they were authored with.
