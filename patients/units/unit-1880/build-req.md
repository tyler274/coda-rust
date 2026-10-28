# Build requirements: unit-1880 (Beth Gauthier, MRN00001880)

Source: synthetic_hospital patient 1880, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its textbook explanation of how cinacalcet works.

## Who she is (S)

Beth Gauthier, born 1992-04-12, female, White, not Hispanic or Latino, English,
Westfield 01085, private PPO. Administrative data specialist. Never smoker; minimal
alcohol (under one drink a week); no recreational drugs. No known drug allergies. No
surgical history before 2026. Family history: maternal aunt with primary
hyperparathyroidism, father with recurrent kidney stones, mother with hypothyroidism.
Single (texture).

Joins the practice on 2023-08-22 (A1), a new-patient preventive visit, coming from a
prior primary-care office. She brings morbid obesity (BMI 43; height 165 cm, weight
about 117 kg) and takes no medications. Hyperparathyroidism is not known on joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 (Tue) | practice | new-patient preventive visit (A1); BMI 43, BP normal; no medications; NKDA. Labs: CMP with **calcium 10.7 mg/dL** (A2), otherwise normal; CBC, lipids, A1c, TSH normal; HIV and hepatitis C screens negative (P). Physician notes the calcium is just above range and plans a recheck. Obesity counseling (diet, activity) |
| 2023-10 | practice | prior-office records arrive (late): Pap 2021 normal, nothing else of note (P) |
| 2023-10-17 (Tue) | practice | nurse visit: influenza and COVID vaccines (P) |
| 2024-02-27 (Tue) | practice | follow-up visit: weight unchanged; no symptoms. **Calcium 10.9**, **intact PTH 96 pg/mL**, **25-hydroxy vitamin D 31 ng/mL**, phosphorus, creatinine normal (A2) |
| 2024-02-29 (Thu) | practice | telephone: calcium high again with a PTH above range; **primary hyperparathyroidism diagnosed** (biochemical, R1); referral to endocrinology placed |
| 2024-04-19 (Fri) | outside | endocrinology consult (encounter 0): asymptomatic; calcium consistently elevated on routine labs, intact PTH above normal; BMI 43. Deemed a poor surgical candidate because of her morbid obesity (R4); **cinacalcet 30 mg twice daily started** (A3); repeat calcium two weeks after starting. Letter reaches the practice late, 2024-06-07, with the two-week result (R3) |
| 2024-04-23 | practice | portal message from patient: endocrinology started cinacalcet; the practice adds it to her list as an outside prescription |
| 2024-05-03 (Fri) | outside | endocrinology lab, two weeks on cinacalcet: **calcium 10.1** (R3, A4), reported in the June letter packet |
| 2024-08 | practice | weight follow-up visit cancelled by patient (texture) |
| 2024-09-17 (Tue) | practice | annual physical: on cinacalcet, no complaints; BMI 43; **calcium 10.2** (A4); Tdap, influenza vaccine; PHQ-9 low (P) |
| 2025-01 | practice | pharmacy notice that the cinacalcet prior authorization has expired; forwarded to endocrinology, which prescribes it. **She stops cinacalcet in January 2025** when the renewal lapses and does not pursue it (R2); the practice does not learn this until August |
| 2025-08-19 (Tue) | practice | office visit (A5): about four months of fatigue, generalized aches, frequent urination, trouble concentrating at work and occasional constipation; no fever, weight loss or recent illness. Says she has been off cinacalcet since January; **cinacalcet removed from the list**. Exam unremarkable. **Calcium 11.2, intact PTH 118 pg/mL, creatinine 0.8**; CBC, TSH, A1c normal. Referred back to endocrinology for a prompt visit |
| 2025-09-26 (Fri) | outside | ordered by endocrinology before the visit (R1): **neck ultrasound: 1.4-cm hypoechoic lesion adjacent to the left thyroid lobe**, thyroid otherwise normal; **Tc-99m sestamibi parathyroid scan: increased uptake in the same region** |
| 2025-10-03 (Fri) | outside | endocrinology follow-up (encounter 1): symptoms as above for about six months (began around April 2025, R5); no medications; exam unremarkable; creatinine normal; intact PTH elevated; imaging as above. **Left parathyroid adenoma** (R1); symptomatic primary hyperparathyroidism with a localized adenoma, so referred to endocrine surgery for a focused parathyroidectomy (A6). Letter and both imaging reports reach the practice 2025-10-21 |
| 2025-10-14 (Tue) | practice | nurse visit: influenza and COVID vaccines (P) |
| 2025-11-14 (Fri) | outside | endocrine surgery consult (A6): focused left parathyroidectomy planned. No letter ever reaches the practice (records gap) |
| 2026-01-09 (Fri) | outside | **focused left parathyroidectomy**, same-day discharge (A6): intraoperative PTH 142 pg/mL before excision, 19 pg/mL at 10 minutes. Pathology: parathyroid adenoma, 1.4 cm, 610 mg |
| 2026-01-13 | practice | portal message from patient: had the surgery Friday, home the same day, recovering without trouble |
| 2026-03-10 (Tue) | practice | post-operative office visit (A7): fatigue, aches and urinary frequency gone, concentrating better; healed low-collar incision. **Calcium 9.3, intact PTH 41 pg/mL**. **Primary hyperparathyroidism and parathyroid adenoma resolved**; parathyroidectomy added to surgical history. Records requested from the surgical group |
| 2026-04-02 | practice | operative note and surgical pathology report arrive (A6) |
| 2026-07-13 (Mon) | practice | annual physical, **last seen**: well; BMI 43; **calcium 9.4** (A7); lipids, A1c, CBC normal; cervical cytology with HPV co-test, normal (P); PHQ-9 low. Nothing booked |
| 2026-07 | practice | portal results note, Pap/HPV result normal |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 calls the adenoma "known" and encounter 1 says it was "identified
  in January 2020", yet the only localizing studies are the neck ultrasound and
  sestamibi scan in encounter 1. Resolution: April 2024 is a biochemical diagnosis of
  primary hyperparathyroidism (made by the practice in February 2024 and confirmed by
  endocrinology); the adenoma is localized by imaging on 2025-09-26, ordered by
  endocrinology ahead of the October 2025 visit, and the parathyroid adenoma is
  diagnosed then. The October letter's "identified in 2024" refers to the
  hyperparathyroidism.
- **R2.** Cinacalcet is started at encounter 0, yet encounter 1 says she takes no
  medications. Resolution: she stops it herself in January 2025 when the prior
  authorization renewal lapses. The practice's list carries it from 2024-04-23 until
  2025-08-19.
- **R3.** "Repeat labs are ordered two weeks after starting" is an unfinished plan.
  Resolution: endocrinology draws a calcium on 2024-05-03 (10.1), reported in the
  late-arriving June letter packet.
- **R4.** "Poor surgical candidate due to other comorbidities": her only comorbidity is
  morbid obesity. The 2024 letter says obesity is the reason (kept as the source's
  judgment, without comment). By October 2025 she is symptomatic and the adenoma is
  localized, and endocrinology refers her for a focused operation.
- **R5.** Encounter 1's six months of symptoms puts their start around April 2025. The
  August 2025 practice visit records about four months.
- The profile lists primary hyperparathyroidism and the adenoma as standing
  conditions; in the chart each starts at the event above. The textbook mechanism text
  of encounter 0 is not carried.
- **Left as the source has it:** no bone density scan, no 24-hour urine calcium, no
  kidney imaging, and no genetic testing or comment on the family history of
  hyperparathyroidism. No anti-obesity drug or bariatric referral; obesity is
  counseled only. Nobody remarks on these.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2023-08-22 (she has years with the practice before the
  source's first encounter, and routine labs there found the calcium).
- **A2.** The routine calcium values that make "consistently elevated" true and the
  workup that sends her to endocrinology: calcium 10.7 (2023-08-22), 10.9, PTH 96,
  25-OH vitamin D 31 (2024-02-27).
- **A3.** Cinacalcet dose: 30 mg by mouth twice daily (the source names no dose).
- **A4.** Calcium on cinacalcet: 10.1 (2024-05-03, endocrinology), 10.2 (2024-09-17).
- **A5.** The August 2025 practice visit that sends her back to endocrinology, with
  calcium 11.2, PTH 118, creatinine 0.8.
- **A6.** The usual next step for symptomatic primary hyperparathyroidism with a
  localized adenoma: endocrine surgery consult 2025-11-14 and focused left
  parathyroidectomy 2026-01-09, intraoperative PTH 142 to 19 pg/mL, pathology
  parathyroid adenoma 1.4 cm, 610 mg.
- **A7.** Post-operative values: calcium 9.3, PTH 41 (2026-03-10); calcium 9.4
  (2026-07-13).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice may mention routine screening done there, with normal
  results (a normal Pap in 2021).
- Weight about 115 to 119 kg at 165 cm throughout (BMI about 42 to 44); blood pressure
  normal. Obesity counseling at visits. Lipids, A1c, TSH, CBC normal.
- Prevention for a woman in her thirties: cervical co-test in 2026, influenza each fall,
  COVID vaccines, Tdap 2024, HIV and hepatitis C screens once, PHQ-2/PHQ-9 screens low.
- Refills, pharmacy and prior-authorization traffic, portal messages about results and
  scheduling, a cancelled visit.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001880 on 2026-07-13. Practice contacts after that day are
only messages and results; no visit is booked.
