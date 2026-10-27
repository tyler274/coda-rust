# Build requirements: unit-2205 (Francis Sawyer, MRN00002205)

Source: synthetic_hospital patient 2205, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its "demonstrated in the image" lines or its quiz
text about the cell type behind the disorder.

## Who he is (S)

Francis Sawyer, born 1954-01-19, male, White, not Hispanic or Latino, English,
Medicare, lives in Chesterfield, MA 01012. Retired construction worker; spends most
of his time at home. Never smoker. Heavy daily alcohol for decades, about 6 to 8
drinks a day. No known drug allergies. No surgical history. Family history: father
type 2 diabetes, mother hypertension, paternal grandfather MI at 72.

Joined the practice on 2020-08-24 (A1, a new patient visit) from a prior primary-care
office. He brought type 2 diabetes, hypertension, hyperlipidemia and chronic alcohol
use disorder, on metformin 1000 mg twice daily, lisinopril 10 mg daily and
atorvastatin 40 mg daily (all recorded as already taking). Height 180 cm, weight
about 93 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-08-24 | practice | new patient visit (A1); history as above; BP at goal; adherent; alcohol counseling, declines medication or a counseling referral for drinking (A2); intake labs (P) |
| 2021-06-21 | practice | routine follow-up; no new symptoms; adherent to his three drugs; asks what more he can do to lower his cardiovascular risk. BP 132/78; exam unremarkable. A1c 6.8%, fasting glucose 138, TC 190, LDL 90, HDL 55, TG 150. Physician reviews risk: BP and LDL acceptable, cutting down alcohol is the largest step open to him; **no drug change** (R3) |
| 2021-09-07 | practice (telephone) | covering clinician renews lisinopril 10 mg for a new pharmacy with the sig "take as directed" (R2) |
| 2021-09-13 | practice | routine follow-up about 3 months later; says he takes his drugs, but is confused by the blood-pressure prescription he got from a different clinician last week, labeled only "as directed", and cannot recall how he was told to take it. No chest pain, palpitations or dizziness. BP 158/92, other vitals normal; exam unremarkable. A1c 7.2%, creatinine 1.1, TC 190, LDL 100, HDL 40, TG 250. **Medication noncompliance diagnosed.** Plan: same drug and dose, lisinopril 10 mg once daily with a written sig, pill organizer, nurse BP recheck (R2) |
| 2021-10-12 | practice | nurse BP recheck: back at goal on lisinopril 10 mg taken daily (R2) |
| 2022-11-28 | practice | diabetes follow-up; mentions slowly worsening stiffness of the right hand; referred to orthopedics (A3) |
| 2022-12-23 | outside | orthopedics consult: several years of progressive difficulty with the right hand, stiffness and trouble straightening some fingers, harder to grip and do daily tasks; no trauma, redness, acute pain, numbness or tingling; hand feels tight; bent fingers do not spring back; stiffness constant. Exam: palpable cords in the palm; sensation and pulses intact. **Dupuytren contracture, right hand, diagnosed.** Plan: observation, home tabletop check, hand surgery if it progresses (A4). Letter reaches the practice late, 2023-02-13, after a records request |
| 2024-11-18 | practice | diabetes follow-up; reports the left hand now doing the same; referred to hand surgery (A3) |
| 2024-12-11 | outside | hand surgery consult: progressive left-hand difficulty like the right-hand problem of December 2022; several years of stiffness and inability to fully straighten several fingers, now interfering with daily activities; no trauma, infection or sensory change. Sensation and pulses intact; palmar cords on the left. **Dupuytren contracture, now both hands.** Heavy alcohol use noted as his main risk factor. Treatment options reviewed; he defers (A5). No letter is sent; the practice hears of it from the patient (portal, January 2025, and the 2025-04-14 visit) and receives the note 2025-05-20 after a records request |
| 2025-04-14 | practice | last practice visit: diabetes and BP follow-up; Dupuytren now recorded as both hands from his account; records request for the hand surgery note; labs drawn, resulted 2025-04-15 |
| 2025-12-22 | outside | hand surgery follow-up: progressive right-hand symptoms, first evaluated in 2022 with the left involved in 2024; stiffness and inability to straighten some fingers now interfere with daily activities; no recent trauma, redness or acute pain. Palmar cords on the right; sensation and pulses intact. Plan: percutaneous needle aponeurotomy of the right hand (A6) |
| 2026-01-26 | outside | hand surgery office procedure: needle aponeurotomy of the right palmar cord under local anesthesia; improved finger extension on the table; night extension splint; no complications (A6). This note and the 2025-12-22 note arrive together on 2026-03-09 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source stamps type 2 diabetes "diagnosed 2020-01-15" (the first
  encounter) while that same encounter already has him on metformin and calls it a
  routine follow-up. Resolution: diabetes predates the practice and comes with him on
  2020-08-24; the stamp is not carried. Likewise the profile lists Dupuytren
  contracture and medication noncompliance with his chronic conditions; each starts
  at the encounter that documents it (2021-09-13 and 2022-12-23).
- **R2.** The source says he got "a new prescription for his blood pressure" from a
  different provider the week before 2021-09-13, yet no new blood-pressure drug ever
  appears: lisinopril 10 mg daily is his only one before and after. Resolution: the
  new prescription is a lisinopril 10 mg renewal by a covering clinician on
  2021-09-07, sent to a new pharmacy with the sig "take as directed"; the different
  tablet and vague label confused him and his dosing became irregular, which is the
  noncompliance and the 158/92. The 2021-09-13 visit rewrites the order with a clear
  once-daily sig. The recheck on 2021-10-12 is at goal (about 134/82), so no drug is
  added, and lisinopril stays 10 mg throughout as the source's later lists say. The
  chart never explains how the vague sig came about.
- **R3.** The source gives no answer to his question about further cardiovascular
  risk reduction. Resolution: the physician's answer is counseling (alcohol, diet,
  activity) with no new drug.
- **R4.** The September 2021 lipids (HDL 40, TG 250) and A1c 7.2 differ from June's
  (HDL 55, TG 150, A1c 6.8) three months apart on unchanged drugs. Resolution: carried
  as written; the September note ties them loosely to irregular dosing and drinking.
  Later draws run A1c about 7.0 to 7.3, LDL about 90 to 100, HDL about 42 to 50,
  TG about 170 to 230.
- **R5.** The source's years (right hand "2021", left hand "2023") follow the
  re-dated timeline: 2022 and 2024.
- **R6.** The source's "lesion demonstrated in the image" lines become written
  examination findings (palmar cords, inability to fully straighten fingers); its
  closing quiz text about the cell type is dropped.
- **Left as the source has it:** atorvastatin stays 40 mg with LDL 90 to 100 in a
  diabetic who asked about lowering his risk; no aspirin, no statin change, no
  second diabetes drug at A1c around 7.2. No note comments on it. The September 2021
  lipid shift is not repeated or chased.

## Additions (A): physician to confirm

- **A1.** 2020-08-24 new-patient visit (the first source encounter is a routine
  follow-up, so he was already a patient).
- **A2.** Standard ongoing care for chronic alcohol use disorder, chosen
  conservatively: brief counseling at visits and an offer of naltrexone or a
  counseling referral, which he declines each time; AUDIT-C 11 to 12. No drug is
  started. Liver tests stay normal.
- **A3.** The practice visits that send him to orthopedics (2022-11-28) and hand
  surgery (2024-11-18).
- **A4.** The orthopedics plan (observation, tabletop check, hand surgery if
  progression), which the source leaves unwritten.
- **A5.** At hand surgery in December 2024: needle aponeurotomy, collagenase injection
  and fasciectomy are discussed; he chooses to wait.
- **A6.** Next step after the source ends: needle aponeurotomy of the right hand on
  2026-01-26 with a night extension splint; extension improved, no complications.
  The left hand is still untreated.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice: colonoscopy in 2017 with no polyps (repeat in 10
  years), PCV13 in 2019, diabetic eye exams without retinopathy.
- Diabetes and BP visits every 3 to 9 months, with vitals, A1c, metabolic panel,
  lipids yearly, urine albumin/creatinine yearly (normal), CBC, foot exams (normal
  sensation and pulses). Medicare annual wellness visits with PHQ-2 (negative) and
  AUDIT-C.
- Yearly optometry letters, no retinopathy; some years none arrives.
- Influenza vaccine each fall, PPSV23 in 2020, COVID vaccines, Shingrix series,
  RSV vaccine in 2024 (learned of from a pharmacy record). Hepatitis C and HIV
  screening once, negative.
- Refills, pharmacy traffic, portal messages, records requests, a missed visit,
  overdue-visit reminders after April 2025.
- Records gaps per the seed: orthopedics letter late, hand surgery letters only after
  requests, an outside vaccine learned of from the pharmacy; the problem list lags the
  left hand until April 2025.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002205 on 2025-04-14. Practice contacts after that day do
not happen except messages, refills and the results of that day's draw; outside care
after it still does, and its records are filed as received without changing the
practice's lists. No visit is booked after today.
