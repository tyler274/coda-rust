# Build requirements: unit-1862 (Hailey Roy, MRN00001862)

Source: synthetic_hospital patient 1862, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan text.

## Who she is (S)

Hailey Roy, born 1992-01-24, female, White, not Hispanic or Latino, English, Medicaid,
lives in the seed's town (01060). Customer service representative. Never smoker, no
alcohol. No known drug allergies. Surgical history: tonsillectomy and adenoidectomy in
childhood. Married (the source's husband). Family history: younger brother with type 1
diabetes, mother with autoimmune thyroiditis, maternal grandmother with a preterm delivery.

Joined the practice 2022-09-14 (new-patient visit, A1) from a prior primary-care office.
Brought on joining: type 1 diabetes mellitus (since childhood, onset 2003, A1),
autoimmune hypothyroidism (onset 2017, A1); insulin glargine 24 units subcutaneously at
bedtime, insulin lispro by sliding scale before meals, levothyroxine 88 mcg daily, and a
prenatal vitamin with iron 1 tablet daily that she takes as her daily vitamin (R3).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-14 | practice | new-patient visit (A1): history as above; diabetes not well controlled (P levels); labs |
| 2025-02-26 | outside ED | brought by her husband after confusion and a witnessed generalized tonic-clonic seizure at home; recent gastroenteritis with heavy vomiting and diarrhea; she had been drinking large amounts of water for several days to stay hydrated; poorly controlled type 1 diabetes. Age 33 (R1). T 37.1 C, BP 130/85, HR 88, RR 16. Lethargic but arousable, oriented to person only, diffuse muscle weakness, neuro exam nonfocal. Na 118, K 4.2, Cl 88, HCO3 24, glucose 180 mg/dL, serum osmolality 240, urine osmolality 500, urine Na 40. Urine pregnancy test negative (A2). **Hyponatremia diagnosed; seizure (provoked, from the hyponatremia) recorded.** Admitted (A2) |
| 2025-02-26 to 2025-03-01 | outside hospital | admission (A2): one 100 mL bolus of 3% saline in the ED, then free-water restriction; sodium 123 the next morning, 131 on 2025-02-28, 134 at discharge on 2025-03-01; no further seizure; no antiseizure drug; home insulin and levothyroxine continued unchanged |
| 2025-03-04 | practice | post-hospital visit (P): alert, oriented, back to baseline; BMP with normal sodium; fluid guidance (drink to thirst, salt-containing fluids with any future vomiting illness). **Hyponatremia, seizure and the gastroenteritis resolved** |
| 2025-05-08 | practice | missed period, home test positive; urine pregnancy test positive in office; last menstrual period 2025-02-14 (11 weeks 6 days, due 2025-11-21) (R4). **Pregnancy recorded.** Referral to high-risk obstetrics (P) |
| 2025-08-26 | practice | pregnancy visit at 27 weeks 4 days; **hemoglobin A1c 9.2%** (the source's "recent" A1c, R5); Tdap given (P) |
| 2025-10-17 to 2025-10-19 | outside hospital, obstetrics | active labor at 35 weeks 0 days after limited prenatal care; pregnancy complicated by poor glycemic control; age 33. She is anxious, not in acute distress; fetal heart tones normal; vital signs stable. Delivers a male infant, 2500 g (vaginal delivery, R6). **Preterm labor with preterm delivery** recorded; pregnancy ends. The infant: immediate grunting, nasal flaring and intercostal retractions, one- and five-minute scores 5 and 7, saturation 85% on room air improving to 92% on supplemental oxygen, chest film with diffuse bilateral ground-glass atelectasis; neonatal respiratory distress syndrome, cared for in the newborn unit (R2). Home medications unchanged at discharge |
| 2025-10-28 | practice | postpartum visit (P); **preterm labor problem resolved** |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The ED note calls her 32; on the re-dated timeline she is 33 (born 1992-01-24).
  The obstetric note's "33" stands. The obstetric note's "January seizure" becomes the
  February 2025 seizure.
- **R2.** "Respiratory distress syndrome", the oxygen saturations, the scores, the grunting
  and retractions and the chest radiograph are the newborn son's, not hers. Her chart
  carries them only as the infant's course inside the obstetric discharge summary: no
  respiratory distress diagnosis, no chest x-ray report and no oxygen saturation of hers.
- **R3.** The prenatal vitamin is on her list at the ED visit, before the pregnancy (last
  menstrual period 2025-02-14, conception after the admission). Resolution: she has taken
  it as her everyday vitamin since before joining; no note gives a reason. She stops it on
  her own in January 2026 and the practice list carries it until the April 2026 physical.
- **R4.** The source gives no pregnancy dates. Delivery at 35 weeks on 2025-10-17 puts the
  last menstrual period at 2025-02-14, so she was not pregnant at the ED visit (negative
  test, A2) and the pregnancy is found in May.
- **R5.** "Recent hemoglobin A1c of 9.2%" is the practice draw of 2025-08-26; the obstetric
  summary quotes it.
- **R6.** The source gives no delivery mode; active labor ending in delivery is recorded as
  a vaginal delivery, with no other procedure.
- **R7.** The obstetric note lists hyponatremia as an active problem; it resolved in March
  2025. The obstetric summary may mention it as history only. Its "(diagnosed ...)" stamp
  is dropped.
- **Left as the source has it:** insulin doses never change (glargine 24 units, lispro by
  sliding scale) despite poor control before and during pregnancy; no head imaging, EEG or
  neurology referral after the seizure; no glucagon prescription; no low-dose aspirin in
  pregnancy; prenatal care stays limited (she misses most obstetric visits). No note
  remarks on any of this.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2022-09-14 (4 years with the practice); onset years for the two
  chronic conditions (type 1 diabetes 2003, hypothyroidism 2017).
- **A2.** The ED course needs an end: urine pregnancy test negative in the ED; admission
  2025-02-26 to 2025-03-01 with one 100 mL bolus of 3% saline and fluid restriction; sodium
  123, 131 and 134 as in the timeline.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice (received October 2022) may mention routine screening done
  there with normal results: a 2021 Pap normal, childhood vaccines including hepatitis B,
  Tdap 2019, COVID primary series 2021.
- Diabetes and thyroid visits about every 3 months, some by video; yearly physicals with
  CMP, CBC, lipids, urine microalbumin/creatinine (normal), TSH (normal on levothyroxine 88
  mcg throughout). Hemoglobin A1c runs about 8.0 to 9.3 before pregnancy, 9.2 in August 2025,
  about 8.5 to 9.4 after delivery. Sodium normal on every draw outside the ED and admission.
  Weight about 63 to 64 kg (height 165 cm) outside pregnancy, about 70 kg at 27 weeks.
- Yearly diabetic eye exam letters from an outside optometrist: no retinopathy. Chart gaps
  (seed): the 2024 exam letter never arrives (she reports it normal); the hospital records
  for the February 2025 admission arrive late after a records request; the obstetric intake
  sends no letter; the practice learns of the delivery from her message and the obstetric
  discharge summary arrives in December 2025.
- Prevention: hepatitis C and HIV screening once (2022, nonreactive); Pap with HPV 2024
  normal; PCV20 once (diabetes); influenza each fall; COVID vaccines each fall; Tdap at 27
  weeks of pregnancy; PHQ-2 at physicals, PHQ-9 postpartum, all negative.
- Refills, pharmacy and prior-authorization traffic (insulin pens), portal messages about
  results, refills and scheduling, one missed visit (June 2023).
- Booked visit after 2026-08-27: diabetes follow-up on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001862 on 2026-01-20. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
