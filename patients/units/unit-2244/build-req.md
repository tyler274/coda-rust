# Build requirements: unit-2244 (Dennis Rousseau, MRN00002244)

Source: synthetic_hospital patient 2244, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz voice ("you note").

## Who he is (S)

Dennis Rousseau, born 1957-03-02, male, White, not Hispanic or Latino, English,
Medicare, lives in Westfield MA 01085, married (his wife and a daughter appear in the
source). Retired accountant. Former smoker, 35 pack-years, quit in 2017 (R2).
Alcohol occasional, 1 to 2 drinks a month. No known drug allergies. Surgical history:
cataract surgery in 2021 (R3). Family history: father ischemic stroke at 74, mother
hypertension, brother coronary artery disease.

Joined the practice on 2022-09-12 (A1), a new patient visit, bringing essential
hypertension on lisinopril 20 mg daily and hyperlipidemia on atorvastatin 40 mg daily.
No prior stroke or TIA.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-12 | practice | new patient visit (A1): history as above; establishes care; baseline labs and prevention (P) |
| 2025-05-25 (Sun) | outside ED | brought in after his wife noticed him dragging his left foot while walking; no pain; says he cannot lift the leg well. Alert and oriented, cranial nerves intact, strength 5/5 both arms and right leg, **3/5 left leg with slightly increased tone**, no sensory loss, arm coordination and reflexes normal. Non-contrast CT head: small hypodensity in the right frontal lobe, no hemorrhage (R1). **Acute ischemic stroke diagnosed.** Home meds lisinopril and atorvastatin. Admitted (A2) |
| 2025-05-25 to 2025-05-27 | outside hospital | admission (A2): left-leg weakness improving; discharged home with his wife on lisinopril 20 mg and atorvastatin 40 mg unchanged; outpatient physical therapy and neurology follow-up in about 6 months |
| 2025-06-02 | practice | post-discharge visit (P); outpatient PT referral (A3) |
| 2025-11-17 | outside neurology | 6-month post-stroke follow-up, with his daughter: over the past week a sudden change in behavior and thinking: forgetful, easily distracted, trouble planning his day, unlike his usual organized self; he reports mild left-sided weakness. Alert but apathetic; fluent speech, no aphasia; mild left-sided drift; cranial nerves intact; sensation normal. Non-contrast CT head: small well-circumscribed hypodensity in the right frontal lobe (the known infarct, R1). **Vascular dementia diagnosed.** Meds lisinopril and atorvastatin unchanged. Letter received 2025-11-20 |
| 2025-12-08 | practice | dementia follow-up with wife and daughter (A4): MoCA 22/30; vitamin B12 486 pg/mL and TSH 2.1 mIU/L; driving stops (A4) |
| 2026-06-01 | outside ED | sudden dizziness, nausea and unsteadiness on his feet; the room spins and does not settle lying still or fixing his gaze; mild headache that began with it; no head trauma, no loss of consciousness. BP 160/90, HR 78, afebrile (temperature 36.8 C). Horizontal nystagmus not suppressed by visual fixation, slight slurring of speech, dysmetria on right finger-to-nose; no facial droop, no arm drift (R4). Home meds lisinopril 20 mg daily, atorvastatin 40 mg daily. CT head: no hemorrhage, old right frontal infarct (A5). Admitted (A5) |
| 2026-06-01 to 2026-06-03 | outside hospital | admission (A5): MRI brain shows a small acute infarct in the right cerebellar hemisphere and the old right frontal infarct; **second acute ischemic stroke (right cerebellar)** (R4); vertigo and nausea settle; **aspirin 81 mg daily started**; lisinopril 20 mg and atorvastatin 40 mg unchanged; home with his wife |
| 2026-06-09 | practice | post-discharge visit (P): BP 136/82 (A5); steadier; on aspirin |
| 2026-07-20 | outside neurology | follow-up (A6): dizziness resolved; no nystagmus; finger-to-nose normal; gait steady; continue aspirin, atorvastatin, lisinopril; return in 6 months. Letter received 2026-07-23 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0's CT "confirms the diagnosis" is an is-shown image, and an early
  non-contrast CT does not confirm infarction by itself. Resolution: the ED CT reads a
  small right frontal hypodensity with no hemorrhage, taken with the left-leg weakness as
  an acute ischemic stroke. The November 2025 CT's right frontal hypodensity is the same
  infarct, now well-circumscribed. No new infarct is read in November 2025.
- **R2.** "Quit 8 years ago" repeats unchanged across encounters a year apart.
  Resolution: he quit in 2017; 35 pack-years throughout.
- **R3.** "Cataract surgery (2021)" appears on the list at the first encounter, which by
  a re-dated reading would precede it. Resolution: the surgery was in 2021 (calendar),
  before he joined, at an outside eye practice; eye not stated. It is on every list.
- **R4.** Encounter 2 gives slight slurring of speech and also "no speech deficits
  suggestive of a classic stroke". Resolution: mild dysarthria is present; there is no
  aphasia. The encounter gives no diagnosis; with right-sided dysmetria and nystagmus not
  suppressed by fixation the ED treats it as a central cause, and MRI (A5) shows a small
  right cerebellar infarct: the source's acute ischemic stroke, recurring. The mild left
  drift of November 2025 and "no arm drift" in June 2026 are two examiners months apart;
  both stay as written.
- **R5.** The source profile and every encounter list only hypertension and
  hyperlipidemia as chronic conditions with lisinopril and atorvastatin. Resolution: the
  practice list holds those two from joining; the 2025 stroke from 2025-05-25; vascular
  dementia from 2025-11-17; the cerebellar stroke from 2026-06-01. Lisinopril 20 mg and
  atorvastatin 40 mg run unchanged throughout.
- **Left as the source has it:** no antiplatelet after the first stroke (the 2025
  discharge, the practice visits and the November 2025 neurology visit all leave him on
  lisinopril and atorvastatin only); atorvastatin stays 40 mg after both strokes; no MRI,
  vascular imaging, echocardiogram or rhythm monitoring is described after the first
  stroke; no drug is started for vascular dementia. Nobody remarks on any of it.

## Additions (A): physician to confirm

- **A1.** New patient visit 2022-09-12. Seed: about 4 years with the practice.
- **A2.** The 2025 ED visit is admitted (2025-05-25 to 2025-05-27); the discharge summary
  carries the CT, improving left-leg weakness, unchanged medications, outpatient PT and
  neurology follow-up. Inpatient labs (CBC, basic metabolic panel, lipid panel, A1c)
  ordinary and within the levels in P.
- **A3.** Outpatient physical therapy for the left leg, June to August 2025: evaluation
  2025-06-09 (left hip flexion and ankle dorsiflexion weak, gait with left foot drag,
  no device needed indoors); discharged 2025-08-12 with gait improved, home exercise
  program, no device.
- **A4.** Standard ongoing care for vascular dementia, conservative: a MoCA of 22/30 at
  the practice on 2025-12-08; vitamin B12 486 pg/mL and TSH 2.1 mIU/L that day (normal);
  he stops driving after the first stroke on PCP advice (2025-06-02) and does not resume, which neurology confirms in November 2025; his wife drives; his daughter helps
  with bills; no dementia drug.
- **A5.** The 2026 ED visit: CT head without hemorrhage, showing the old right frontal
  infarct; admitted 2026-06-01 to 2026-06-03; MRI brain finding as in the timeline;
  inpatient labs ordinary; aspirin 81 mg daily started at discharge. Practice BP
  136/82 on 2026-06-09.
- **A6.** Neurology follow-up 2026-07-20 with the findings in the timeline.
- **A7.** Lung cancer screening (35 pack-years, quit 2017): low-dose CT chest yearly,
  2022-10-18, 2023-10-10, 2024-10-22, 2025-10-21, each Lung-RADS 1. One-time abdominal
  aortic ultrasound for a male former smoker, 2022-10-18: aorta normal caliber, 2.1 cm,
  no aneurysm.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Levels: height 178 cm, weight about 84 kg, steady. Office BP on lisinopril about
  126 to 138 / 76 to 86. LDL on atorvastatin 40 about 78 to 92; A1c 5.4 (2022-09), 5.5 (2023-09), 5.6 (2024-09), 5.4 (2025-05 inpatient), 5.5 (2025-09), 5.6 (2026-06 inpatient); kidney
  function, electrolytes, liver tests and blood counts normal.
- Visits for hypertension and hyperlipidemia about every 6 months with a basic metabolic
  panel, and a Medicare annual wellness visit each fall with a lipid panel and metabolic
  panel; post-discharge visits after each admission; dementia follow-up visits with his
  wife.
- Records from his prior practice: colonoscopy in 2018, normal, next due 2028; COVID
  primary series in 2021; cataract surgery 2021.
- Prevention: hepatitis C screen once (nonreactive); influenza each fall; PCV20 once;
  Shingrix series 2023; Tdap 2023; COVID updated vaccines in the fall; depression
  screens (negative) at wellness visits.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  missed wellness visit in 2024 rescheduled, the daughter's calls and messages.
- Booked visit after 2026-08-27: annual wellness visit 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
