# Build requirements: unit-1891 (Abigail Lemieux, MRN00001891)

Source: synthetic_hospital patient 1891, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its quiz text (the hidden "a medication" and the
mechanism sentence in encounter 2).

## Who she is (S)

Abigail Lemieux, born 1999-10-08, female, White, not Hispanic or Latino, English,
lives in Easthampton MA 01027, university student health insurance, graduate student.
Never smoker; alcohol rare (less than one drink a month). An avid runner (S, encounter 0).
Allergies: no known drug allergies. Surgical history: wisdom teeth extraction at age 18
(2018). Family history: maternal aunt with Hashimoto thyroiditis; paternal grandfather
with type 2 diabetes.

Joined the practice in September 2022 (first practice visit 2022-09-13, a new-patient
visit, A1), age 22. She brought type 1 diabetes, diagnosed in 2019 (R1) and followed by an
outside endocrinology group, on a basal-bolus regimen since then:

- insulin glargine 20 units subcutaneously at bedtime
- insulin aspart 8 units subcutaneously before meals
- glucagon emergency kit 1 mg intramuscularly as needed for severe hypoglycemia

Endocrinology prescribes all three; the practice lists them as outside medications. Doses
never change in the chart.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new-patient visit (A1): history as above, establishes primary care; insulin regimen as above; no hypoglycemia; physical exam normal; height and weight (P) |
| 2024-09-10 | outside | endocrinology routine follow-up: basal-bolus for the past 5 years with good control, recent A1c 6.8%, no hypoglycemic episodes; doses unchanged (glargine 20 units at bedtime, aspart 8 units before meals, glucagon kit); she is planning to train for a marathon with daily sessions over 60 minutes and asks how to handle insulin and food during training; vitals normal, exam unremarkable. Letter plan (R6): exercise counseling, no dose change. **Letter reaches the practice about two weeks later** |
| 2025-05-09 | outside | regional ED (age 25, R2): 3 days of left ear pain worse with pulling on the earlobe, left ear fullness and decreased hearing, no drainage, no systemic symptoms; swam in a lake 5 days earlier. Left canal red and swollen, tragal tenderness, tympanic membrane not visible for swelling. Blood glucose 180 mg/dL. **Left malignant otitis externa diagnosed** (R4); **ciprofloxacin 750 mg twice daily and ofloxacin otic drops started** (A2); ENT follow-up advised. **The practice learns of it from the patient's portal message the next day; the ED record arrives late, about 2025-05-27** |
| 2025-05-12 | practice | telehealth visit (A3): ear pain about the same, taking both drugs; urgent ENT referral placed |
| 2025-05-14 | outside | ENT consult (A4): canal debrided, culture taken, wick placed; CT temporal bones; ESR 41 mm/h. Early necrotizing (malignant) otitis externa, left; continue ciprofloxacin 750 mg twice daily for 6 weeks in all, ofloxacin drops for 14 days. Culture (resulted 2025-05-17): Pseudomonas aeruginosa, susceptible to ciprofloxacin. **Letter arrives about 2025-05-30** |
| 2025-06-20 | outside | ENT follow-up (A5): pain gone, canal healed, tympanic membrane intact and normal, hearing subjectively back to normal; ESR 12 mm/h. **Ciprofloxacin stopped.** Malignant otitis externa resolved. **Letter arrives late, about 2025-08-04; the practice problem list carries the otitis as active until the 2025 physical, where it is resolved** |
| 2026-05-17 | outside | regional ED, a Sunday: found unconscious by her roommate after studying late the night before and skipping breakfast. EMS found her diaphoretic and unresponsive with a fingerstick glucose of 38 mg/dL and gave glucagon 1 mg intramuscularly; she woke (R5). On ED arrival awake and answering questions, glucose 96 mg/dL. **Severe hypoglycemia (type 1 diabetes with hypoglycemia, with loss of consciousness) diagnosed**; observed, ate, discharged home the same day with her roommate (A6). Insulin doses unchanged; follow up with primary care and endocrinology. **The patient calls the practice the next morning; the ED record arrives about 2026-05-19** |
| 2026-05-20 | practice | post-ED visit (A7): feels well; reviews the episode; no dose change at the practice; counseling; message to endocrinology. Hypoglycemia resolved on the problem list |
| 2026-06 | outside | endocrinology follow-up (P): reviewed the episode, no dose change. **No letter ever reaches the practice; the patient reports it by portal** |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source stamps type 1 diabetes "diagnosed 2020-01-15" (its first encounter)
  while that same encounter says she has been on basal-bolus insulin for 5 years.
  Resolution: diagnosed in 2019 (age 19), basal-bolus from diagnosis; onset 2019. The
  stamp is dropped. Nothing is said about how she presented at diagnosis.
- **R2.** Encounter 1 calls her 24; on 2025-05-09 she is 25 by the seed's birth date. The
  chart says 25. The source's "last seen in Endocrinology 8 months prior" (September 2024
  to May 2025) and "seen in this ED one year ago" (May 2025 to May 2026) already fit the
  re-dated timeline and are kept.
- **R3.** The source profile lists all three insulin products from the start and never
  changes them. The chart keeps them unchanged throughout; no dose change at any contact.
- **R4.** The source names malignant otitis externa from a 3-day presentation with no
  drainage or systemic symptoms, which on its face reads like ordinary acute otitis
  externa. The diagnosis is the source's and stays: the ED makes it on the canal findings
  in a patient with type 1 diabetes, and the ENT workup (A4) supports early disease.
  "Otitis externa" alone is not used as a separate diagnosis.
- **R5.** Encounter 2's exam ("diaphoretic, unresponsive") and the 38 mg/dL glucose are
  the prehospital findings; the injected "medication" is glucagon 1 mg IM given by EMS,
  as the source's own mechanism sentence implies. The source's note breaks off mid-sentence
  at ED arrival: the chart completes it with her awake and the 96 mg/dL reading. The
  mechanism sentence is quiz text and is not carried; the ED note simply says glucagon
  corrected the low.
- **R6.** Encounter 0 has no plan. Resolution: the endocrinology letter gives general
  exercise counseling (check glucose before, during and after long runs, carry fast-acting
  carbohydrate, consider a smaller pre-run aspart dose by her own judgment) with no change to
  the prescribed doses.
- **Left as the source has it:** no continuous glucose monitor or pump is raised at any
  point, including after the severe low, and the insulin doses are never adjusted. No one
  comments on either.

## Additions (A): physician to confirm

- **A1.** 2022-09-13 new-patient visit: the source's first encounter is a routine
  endocrinology follow-up, so she had care before; the seed gives about 4 years with the
  practice.
- **A2.** ED treatment for the source's malignant otitis externa (the source gives none):
  ciprofloxacin 750 mg by mouth twice daily and ofloxacin 0.3% otic solution 10 drops in the
  left ear twice daily.
- **A3.** 2025-05-12 practice telehealth visit that places the ENT referral the ED advised.
- **A4.** ENT consult 2025-05-14, the standard workup for the source's diagnosis: canal
  debridement with culture (Pseudomonas aeruginosa, susceptible to ciprofloxacin) and a
  wick; ESR 41 mm/h; CT temporal bones without contrast: soft-tissue thickening and
  narrowing of the left external auditory canal without bony erosion, middle ear and
  mastoid clear, right side normal. Six weeks of ciprofloxacin in all; drops for 14 days.
- **A5.** ENT follow-up 2025-06-20: healed, ESR 12 mm/h, ciprofloxacin stopped, resolved.
- **A6.** The ED disposition on 2026-05-17 (observed, fed, discharged home the same day),
  which the source leaves unfinished.
- **A7.** 2026-05-20 post-ED visit: no dose change; counseled not to skip meals on her
  bedtime glargine, to check glucose before bed and on waking during exam weeks, to keep
  the glucagon kit where her roommate knows it and show her how to use it, and not to drive
  until endocrinology has reviewed the episode.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from her prior primary care (a university student health service) received in
  2022: type 1 diabetes since 2019, wisdom teeth extraction 2018, a normal Pap in 2021,
  childhood and college vaccines complete (HPV series, MenACWY, hepatitis B series, MMR,
  varicella), COVID primary series 2021.
- Yearly preventive visits, with vitals, foot exam and PHQ-2 or PHQ-9 (negative). Height
  165 cm; weight about 57 kg throughout (BMI about 21); blood pressure and other vitals
  normal.
- Type 1 diabetes labs at the practice: CMP, lipid panel, TSH and urine
  albumin/creatinine yearly, CBC at establishing care and in 2025, all normal (LDL about 90); A1c at establishing
  care. A1c runs 6.6 to 7.2 in every value outside the 6.8 above (practice draws and the
  endocrinology letters that arrive). Tissue transglutaminase IgA once (negative).
  Hepatitis C and HIV screening once (nonreactive).
- Endocrinology follow-up about every 6 months throughout; only some letters reach the
  practice (one in 2023 besides the 2024 source letter), each with an A1c in range and no
  dose change.
- Yearly diabetic eye exam letters from an optometrist: no retinopathy (some years only by
  the patient's report).
- Cervical screening: Pap in fall 2024, three years after the 2021 Pap, normal.
- Vaccines: Tdap at establishing care (last at about age 11), PCV20 once for diabetes,
  influenza each fall, updated COVID each fall.
- Refills of insulin, pen needles, test strips and the glucagon kit routed to
  endocrinology or sent by the practice as a bridge, pharmacy fill notices, portal
  messages about results, refills and scheduling, a missed or cancelled visit, reminders.
- Booked visit after 2026-08-27: her yearly physical.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
