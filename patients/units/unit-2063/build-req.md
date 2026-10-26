# Build requirements: unit-2063 (Steven Brooks, MRN00002063)

Source: synthetic_hospital patient 2063, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its quiz text (trial sample sizes, p-values, power,
type I and type II errors).

## Who he is (S)

Steven Brooks, born 1963-08-19, male, White, not Hispanic or Latino, English, lives in
Leeds, MA. Medicare (due to disability) with a supplemental private plan. Retired law
enforcement officer. Former smoker, quit about 20 years ago, 10 pack-years. Lifelong
abstainer from alcohol. No known drug allergies. No prior surgeries. Family history:
father with type 2 diabetes and a myocardial infarction at 62; mother with hypertension
and diabetic nephropathy; one sibling with gestational diabetes.

Joined the practice on 2020-08-18 (A1, a new-patient visit). He brought:

- type 2 diabetes mellitus, diagnosed in 2009 (R2), never well controlled;
- epilepsy, on phenytoin for years before joining, no seizures since (well controlled);
- metformin 1000 mg twice daily, insulin glargine 24 units at bedtime, insulin lispro by
  sliding scale with meals, phenytoin 300 mg daily (R1);
- acetaminophen as needed, which he had tried, with ibuprofen, for foot pain with little
  relief.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-08-18 | practice | new-patient visit (A1): history as above; about 4 months of burning, tingling feet worse at night; A1c 8.0 (A1); **referred to endocrinology** |
| 2020-10-23 | outside | endocrinology consult: 6 months of burning, tingling feet, worse at night, sharp electric pain sometimes running up into the lower legs; no injury or medication change; acetaminophen and ibuprofen gave little relief. Decreased light touch and pinprick in a stocking-glove distribution; ankle reflexes diminished both sides. A1c 7.8. **Painful symmetric diabetic peripheral polyneuropathy** and **diabetic motor neuropathy** (R3). No drug started (left as the source has it). Letter reaches the practice late, 2020-11-19 |
| 2021-03-09 | practice | about 4 months of difficulty starting urination and a weak stream (A2); A1c 9.4 (R4); **referred to urology** |
| 2021-04-26 | outside | urology consult, about 6 months after the endocrinology visit: 6 months of hesitancy, weak stream, incomplete emptying and post-void dribbling; bladder palpable and percussible above the pubic symphysis; post-void residual 300 mL. **Neurogenic bladder** (R5); **clean intermittent self-catheterization taught** (A3). No letter sent; the patient tells the practice on 2021-06-08, the note arrives 2021-08-03 after a records request |
| 2021-06-08 | practice | diabetes follow-up; reports urology visit and self-catheterization; A1c 9.1 (R4) |
| 2021-10-19 | practice | about 4 weeks of loose stools (A4); stool culture sent, negative; **referred to gastroenterology for colonoscopy** |
| 2021-11-29 | outside | colonoscopy (A4): no structural abnormality or inflammation; report received next day |
| 2021-12-27 | practice | 3 months of severe diarrhea with bloating after meals and lightheadedness after meals; no travel, no antibiotics, no fever, no blood, no significant weight loss. Decreased pinprick both feet; bowel sounds mildly decreased; heart and abdomen otherwise normal. A1c 8.9 (drawn 2021-12-20); stool culture negative; colonoscopy a month before normal. **Diabetic autonomic neuropathy, gastrointestinal and cardiovascular** diagnosed. Plan: small frequent meals, fluids, rise slowly after eating; no drug (left as the source has it) |
| 2022-06-14 | practice | UTI (A5): dysuria and frequency; urine culture E. coli; **cephalexin 500 mg twice daily for 7 days** |
| 2023-01-10 | practice | UTI (A5): same symptoms; urine culture E. coli; **cephalexin 500 mg twice daily for 7 days**; **recurrent urinary tract infections** added to the problem list |
| 2023-02-24 | outside | endocrinology: enrollment visit for a randomized trial of a new oral glucose-lowering agent (R6). No letter ever arrives; the patient reports it on 2023-04-11 |
| 2024-03-19 | practice | UTI (A5): urine culture E. coli; **nitrofurantoin 100 mg twice daily for 7 days** |
| 2024-11-05 | practice | routine diabetes follow-up at 61: since spring 2024 a lifestyle program of a 45-minute walk at 6 am daily, home cooking from whole foods and regular glucose logging; says it is routine now and he gets back on track after a missed day; proud of his A1c; asks about a long-term diabetes support group. Mentions the endocrinology visits of 2020 and 2023. A1c 7.0 (A6). **Referred to a diabetes self-management education and support group** (P) |
| 2025-10-21 | practice | UTI (A5): dysuria; urine culture E. coli, sensitive to trimethoprim-sulfamethoxazole and cephalexin; **trimethoprim-sulfamethoxazole 800/160 mg twice daily for 7 days** (A7) |
| 2025-10-28 | outside | ED, age 62: 2 days of unsteady walking, double vision and nausea; no head injury, no alcohol, no diet change; on TMP-SMX for one week for a UTI. Vital signs normal. Horizontal nystagmus, wide-based gait. Phenytoin 32 mcg/mL (therapeutic 10 to 20); sodium 140, potassium 4.2, creatinine 1.0, BUN 14; liver tests normal. **Acute phenytoin toxicity** (interaction). **TMP-SMX stopped**, **cephalexin 500 mg four times daily for 5 days** to finish the course, phenytoin held, level to be rechecked by the PCP (A8). Discharged home. The patient messages the practice next day; the practice learns of the cephalexin from a pharmacy fill on 2025-10-30; the ED record arrives 2025-11-18 |
| 2025-10-31 | practice | phenytoin 17 mcg/mL (A8); by phone, **phenytoin resumed at 300 mg daily** |
| 2025-11-07 | practice | follow-up (A8): steady gait, no double vision or nausea, no nystagmus; phenytoin 14 mcg/mL; **phenytoin toxicity resolved**; TMP-SMX to be avoided with phenytoin |
| 2026-05-01 | practice | last visit: diabetes follow-up; A1c 7.2; labs that day; next visit booked |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 prose says diabetes is managed with "metformin and dietary
  modifications" while its list carries both insulins, and encounter 0 and 2 lists drop
  metformin. Resolution: he takes metformin 1000 mg twice daily, glargine 24 units at
  bedtime, lispro sliding scale and phenytoin 300 mg daily from joining to today, without
  dose change. The "metformin and diet" phrase is not carried.
- **R2.** Diabetes duration: "15 years" (2020 profile), "diagnosed 12 years ago" (2021),
  "15-year" (2024), and "diagnosed six months ago" (2024). Resolution: diagnosed in 2009,
  which fits the 2021 and 2024 statements. The "six months ago" line is dropped.
- **R3.** The source stamps "diabetic motor neuropathy" at encounter 0 without naming a
  motor finding. Resolution: the endocrinologist reads the diminished ankle reflexes as
  the motor component of a sensorimotor polyneuropathy and records both the painful
  sensory polyneuropathy and diabetic motor neuropathy. No weakness or wasting is
  described.
- **R4.** A1c 7.8 at the October 2020 consult, then "consistently above 9%" by the
  urology visit, then 8.9 in December 2021. Resolution: A1c 8.0 (August 2020), 7.8
  (October 2020), 9.4 (March 2021), 9.1 (June 2021), 8.9 (December 2021), 8.3 to 8.7
  through 2022 and 2023, about 7.9 in spring 2024, 7.0 in November 2024, 7.0 to 7.4 after.
  The profile's "recent A1c above 9%" refers to the 2021 values.
- **R5.** Encounter 1 calls the neurogenic bladder "known" and the visit a return, but the
  stamp puts the diagnosis at that visit, and the profile lists it from the start.
  Resolution: the urology visit is his first, on practice referral; neurogenic bladder is
  diagnosed there. Recurrent UTIs, also on the profile from the start, begin after it
  (A5). Diabetic autonomic neuropathy, also listed from the start, is diagnosed on
  2021-12-27 only.
- **R6.** Encounter 3 is quiz text about a trial's statistics. Resolution: the chart
  keeps only that he enrolled in a randomized trial of a new oral glucose-lowering agent
  at endocrinology. No study statistics, no study drug on the practice list (blinded,
  supplied by the research team), no result. His listed diabetes drugs do not change.
- **Left as the source has it:** no drug is ever started for the painful neuropathy
  (acetaminophen only) or for the diabetic diarrhea; no statin despite diabetes at his
  age; no ACE inhibitor or ARB; no further urology follow-up after 2021; phenytoin stays
  his antiseizure drug after the toxicity. No note comments on these.

## Additions (A): physician to confirm

- **A1.** 2020-08-18 new-patient visit (6 years with the practice; the endocrinology
  consult needs a referral). A1c 8.0 that day.
- **A2.** 2021-03-09 practice visit that sends him to urology, A1c 9.4.
- **A3.** Clean intermittent self-catheterization, the standard step for a neurogenic
  bladder with a residual of 300 mL; it explains the later recurrent UTIs. Catheter
  supplies through a medical supply company.
- **A4.** The October 2021 visit, stool culture (negative) and the November 2021
  colonoscopy (normal) that the December 2021 visit reports.
- **A5.** Four UTI episodes (2022-06, 2023-01, 2024-03, 2025-10), each with a urinalysis
  positive for leukocyte esterase and a culture growing E. coli, to carry the source's
  recurrent UTIs and the TMP-SMX course the ED names. No fever, no flank pain, no
  admission in any episode.
- **A6.** The November 2024 A1c of 7.0, the "significant improvement" the source states.
- **A7.** The October 2025 visit that starts the TMP-SMX the ED reports.
- **A8.** After the ED: phenytoin held, cephalexin to finish the UTI course, level 17 on
  2025-10-31 with phenytoin resumed at 300 mg daily, and a follow-up on 2025-11-07 with a
  level of 14 and the toxicity resolved.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (a screening colonoscopy about 2014, normal; Tdap 2016).
- Diabetes visits every 3 to 6 months with vitals, A1c, basic or comprehensive metabolic
  panel, lipids and urine albumin/creatinine yearly (normal), CBC yearly, foot exams
  showing the neuropathy above; phenytoin levels about yearly, therapeutic (12 to 16).
  Height 180 cm; weight about 97 kg at joining, drifting to about 91 kg after the 2024
  lifestyle program. Blood pressure normal throughout.
- Yearly diabetic eye exam letters from an outside optometrist: no retinopathy (some years
  never arrive).
- Prevention: Medicare annual wellness visits; influenza vaccine each fall; COVID vaccines;
  PCV20 once; Shingrix series; RSV vaccine once (2024); hepatitis C and HIV screening once,
  negative; PHQ-2 negative.
- Refills, pharmacy and prior-authorization traffic (insulin pens, catheter supplies,
  glucose test strips), portal messages about results and scheduling, a missed visit.
- Records: gaps, as in the timeline (late endocrinology letter, urology note only after a
  request, no trial letter, late ED record, an eye letter that never comes).
- Booked visit after 2026-08-27: a routine diabetes follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002063 on 2026-05-01. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
