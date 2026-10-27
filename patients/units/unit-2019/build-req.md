# Build requirements: unit-2019 (Patrick Michaud, MRN00002019)

Source: synthetic_hospital patient 2019, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the trial risk figures).

## Who he is (S)

Born 1969-02-21, male, White, not Hispanic or Latino, English, commercial PPO, lives
in the seed's town. Senior financial analyst. Never smoker. Alcohol socially, 1 to 2
drinks a week. No known drug allergies. Surgical history: a diagnostic percutaneous
left kidney biopsy. Family history: father MI at 58; mother hypertension and type 2
diabetes; older brother ischemic stroke at 52.

Joined the practice on 2023-09-12 (A1, a new-patient visit). He brought:
essential hypertension (about 8 years, onset about 2015), hyperlipidemia (onset about
2018), and nephrotic syndrome secondary to membranous nephropathy, diagnosed in early
2023 by an outside nephrology group on the left kidney biopsy (R3), followed there.
Drugs on joining: lisinopril 20 mg daily, atorvastatin 40 mg daily (both historical,
prescribed before). Aspirin is not yet taken (R1). Height 180 cm, BMI about 27
(weight about 87.5 kg) throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new patient visit (A1): history as above; BP about 131/83 on lisinopril; exam normal apart from what A3 allows; labs CMP, CBC, urinalysis, A1c, hepatitis C and HIV screening (A3 levels) |
| 2023-10-03 | outside | prior primary-care records arrive: the problems and drugs above, a normal screening colonoscopy in 2021, Tdap in 2019, and a copy of the 2023 kidney biopsy pathology report: membranous nephropathy (R3). The nephrology group's own consult notes are requested twice and never arrive (records gap) |
| 2024-04-23 | practice | annual physical (A2): no complaints; asks about aspirin given his father and brother; after a discussion of a small benefit and a real bleeding risk, **aspirin 81 mg daily is started** for primary prevention (R1). PSA screening discussed and drawn, normal |
| 2024-05-29 | practice lab | fasting draw: TC 190, HDL 45, LDL 100, TG 225; CMP (A3 levels) |
| 2024-06-06 | practice | follow-up about 6 weeks after starting aspirin: no complaints, taking it daily; BP 132/84, HR 78, BMI 27; lipids above reviewed; no drug change (Left as the source has it) |
| 2024-08-20 | outside | letter from an outside clinical research center: enrolled in a randomized, blinded trial of a new antihypertensive added to his usual treatment (R4) |
| 2024-10-16 | practice | antihypertensive follow-up: hypertension, hyperlipidemia, membranous nephropathy; in the trial; adherent to drugs and a low-sodium diet; no chest pain, shortness of breath or dizziness; BP 132/84; exam unremarkable. No drug change. BMP and urinalysis (A3 levels) |
| 2025-01-07 | practice | last seen. Routine follow-up of blood pressure and the kidney (P): no new symptoms; BP about 131/83; CMP and urinalysis (A3 levels); return in 4 months, which he does not book |
| 2025-10-14 | outside | ED (S), age 56: two days of left flank pain and gross hematuria, increasing swelling of the left leg and scrotum. BP 145/90, pulse 88, afebrile. Left leg markedly larger than the right with pitting edema to mid-thigh; scrotal edema. Urinalysis 3+ protein, RBCs too numerous to count; albumin 2.1; creatinine 1.2; D-dimer elevated; TC 300, LDL 190, HDL 30 (TG 400, R6). The ED history lists "prior MI and stroke" and calls the nephrotic syndrome a recent diagnosis (R2, R3). CT abdomen and pelvis with contrast: **left renal vein thrombosis** (A4) |
| 2025-10-14 to 2025-10-17 | outside | admission (A4): heparin infusion, then **apixaban 10 mg twice daily for 7 days, then 5 mg twice daily**; **aspirin stopped** (primary prevention only, now anticoagulated); nephrology saw him inpatient and will follow him as an outpatient for the nephrotic syndrome. Lisinopril and atorvastatin continue unchanged |
| 2025-10-21 | practice (portal) | he messages that he was in the hospital and is on a blood thinner; records requested |
| 2025-11-18 | outside | ED record, CT report and discharge summary received by fax after a second request; filed as received, practice lists not changed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists aspirin from the start, yet encounter 0 is a follow-up "after
  starting new medication for primary cardiovascular prevention". Resolution: the new
  drug is aspirin 81 mg, started at the 2024-04-23 physical, six weeks before encounter 0.
  The trial figures in encounter 0 (absolute and relative risk reductions) are quiz
  text and are not carried; the 2024-04-23 note records a plain discussion of small
  benefit and bleeding risk.
- **R2.** The source stamps acute myocardial infarction "diagnosed" at encounter 0 and
  stroke "diagnosed" at encounter 1, and the ED history mentions "prior MI and stroke".
  Encounter 0 is a primary-prevention visit with no complaints and encounter 1 records no
  chest pain, no dizziness and an unremarkable exam; MI and stroke are the outcomes the
  vignettes' trials measure. Resolution: he has never had an MI or a stroke. Neither is
  on any practice list or in any practice note. The ED physician's history in the
  2025-10-14 record keeps the source's phrase "prior MI and stroke" as an unexplained
  error in an outside record; the discharge summary lists his history correctly
  (hypertension, hyperlipidemia, membranous nephropathy). The practice receives these
  records after `last_seen` and does not correct anything.
- **R3.** The ED calls the nephrotic syndrome a "recent diagnosis", but it is on the list
  at encounter 0 (2024-06-06). Resolution: diagnosed by biopsy in early 2023 (onset
  2023-02), before he joined; the ED phrase "recent diagnosis" may stay in the ED record
  as the ED wrote it. The biopsy is the source's "percutaneous left kidney biopsy".
- **R4.** Encounter 1 says he takes part in a randomized trial of a new antihypertensive.
  Resolution: he enrolled between encounter 0 and encounter 1 (letter 2024-08-20). The
  study drug is blinded and run by the research center: it is named only in notes as a
  blinded study medication and is not a coded drug on the practice list. The trial's
  stroke figures (RR 0.85) are quiz text and are not carried.
- **R5.** The source repeats BP 132/84 at both outpatient visits. Carried as written on
  those two dates; other practice readings run about 128 to 136 over 80 to 86.
- **R6.** The ED lipid panel gives TC 300, LDL 190 and HDL 30 but no triglycerides; the
  only value that makes the three agree (calculated LDL) is TG 400, carried. The worse
  lipids and albumin than in 2024 go with the relapsed nephrotic syndrome; no one
  explains them.
- **Left as the source has it:** LDL 100 and TG 225 on atorvastatin 40 mg in 2024 bring no
  change; no prophylactic anticoagulation, diuretic or immunosuppression appears in the
  practice's care of the nephrotic syndrome (nephrology runs it and its notes never
  arrive); the ED lipids of 300/190 bring no statin change in the hospital record. No
  note comments on any of these. A left leg venous duplex is not part of the ED record.

## Additions (A): physician to confirm

- **A1.** The 2023-09-12 new-patient visit (encounter 0 is a follow-up, so he was already
  a patient; three years with the practice).
- **A2.** The 2024-04-23 annual physical where aspirin starts (it must start somewhere).
- **A3.** Practice lab levels for a treated membranous nephropathy in partial remission:
  albumin about 3.2 g/dL, total protein about 5.7 g/dL, creatinine about 1.0 mg/dL,
  urinalysis protein 2+ with negative blood on each practice urinalysis. Exam: trace
  ankle edema at most, otherwise normal. A1c 5.4% and PSA 0.7 ng/mL (both normal); hepatitis C and HIV nonreactive.
- **A4.** The usual next steps of the ED's renal vein thrombosis: the CT abdomen and pelvis
  with contrast showing a left renal vein thrombus (kidneys otherwise unremarkable, no
  hydronephrosis, IVC patent), total protein about 4.6 g/dL in the ED, the 3-day admission, apixaban as above and aspirin
  stopped, inpatient nephrology consult with outpatient follow-up. The nephrology
  follow-up letter never reaches the practice.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (colonoscopy 2021, Tdap 2019).
- Visits for blood pressure and the kidney every few months while he comes, with vitals,
  BMP or CMP and urinalysis; a yearly physical with PHQ-2.
- Prevention: influenza and updated COVID vaccines in fall 2023 and fall 2024; Shingrix
  series (2024-01 and 2024-04); PCV20 once (nephrotic syndrome); hepatitis C and HIV
  screening once; PSA after shared decision.
- Refills of lisinopril and atorvastatin by the practice before and after `last_seen`,
  pharmacy traffic, portal messages about results and scheduling, a reminder he lets
  slide in 2025, records requests.
- Booked visit after 2026-08-27: a physical and post-hospital follow-up he books in June
  2026.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records (chart texture: gaps)

The prior primary-care packet arrives; the nephrology consult notes never do. The trial
letter arrives. After `last_seen` the practice hears of the October 2025 hospitalization
first from the patient's message; the ED record, CT report and discharge summary arrive a
month later after a second request and are filed without updating the problem or
medication lists, which still show aspirin and no renal vein thrombosis or apixaban.

## Last seen

The practice last saw MRN00002019 on 2025-01-07. Practice contacts after that day are
only messages, refills and records; outside care after it still happens, and its records
are filed as received without changing the practice's lists. A visit is booked after
today.
