# Build requirements: unit-2219 (Francis Yeager, MRN00002219)

Source: synthetic_hospital patient 2219, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its wording.

## Who he is (S)

Francis Yeager, born 1956-07-23, male, White, not Hispanic or Latino, English,
Medicare. Retired postal worker. Former smoker, 25 pack-years, quit in 2016 (R2).
Alcohol: occasional, 1 to 2 drinks a month. Allergy: sulfa drugs (rash), recorded as
sulfonamide antibiotics. Surgical history: appendectomy at 22 (about 1978), right knee
arthroscopy at 54 (about 2010). Family history: father MI at 70 and hypertension;
mother type 2 diabetes and chronic heart failure, died at 81; brother hypertension and
coronary artery disease. Marital status recorded as single; no household members are
named in the chart.

Joined the practice on 2021-06-10 (A1), a new-patient visit after a prior primary-care
office. Brought: essential hypertension, type 2 diabetes mellitus (without
complications), obesity, hyperlipidemia. Drugs on joining: metformin 1000 mg twice
daily, lisinopril 20 mg daily, hydrochlorothiazide 25 mg daily. No statin. Heart
failure is not known at joining (R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-06-10 | practice | new-patient visit (A1): history as above; BP above goal on lisinopril and HCTZ, no change; BMI about 34 (P) |
| 2024-12-05 | outside | ED visit (S, encounter 0): about 2 months of worsening exertional breathlessness and swelling of both legs; no chest pain, fever or cough. BP 145/90, HR 88, RR 20. S3 gallop, jugular venous distension, diminished breath sounds at both bases without crackles, 2+ pitting edema to the mid-shins. Chest x-ray: small bilateral pleural effusions. Thoracentesis in the ED: pleural fluid protein 1.5 g/dL with serum protein 6.8 g/dL, pleural fluid LDH 110 U/L with serum LDH 200 U/L, pleural fluid pH 7.45 (transudate). ED impression **acute decompensated heart failure**; home drugs unchanged, discharged home with PCP follow-up (R7). Record received 2024-12-09 |
| 2024-12-10 | practice | post-ED visit (A2): breathing better than at the ED, still some ankle swelling; trace to 1+ ankle edema, lungs clear, no JVD seen; BP 140/86. BMP. **Transthoracic echo ordered** |
| 2024-12-19 | outside | echo (A3): LVEF 35% with global hypokinesis, LV mildly dilated, no significant valve disease, no pericardial effusion. **Heart failure with reduced ejection fraction recorded** (R1) |
| 2025-01-14 | practice | heart-failure follow-up: no edema, lungs clear; ADHF episode **resolved** on the list; HFrEF stays; drugs unchanged (R7) |
| 2025-09-04 | outside | ED visit (S, encounter 1): about 2 weeks of worsening exertional breathlessness and orthopnea; notices his waistline has grown though his eating and weight have not changed (R5). BP 150/90, HR 102, RR 22, SpO2 92% on room air. JVD, crackles in both lungs, pitting edema of the legs, S3. BNP 900 pg/mL, creatinine 1.2 mg/dL, potassium 4.5, sodium 140, fasting glucose 160 mg/dL (R4). Chest x-ray: cardiomegaly and pulmonary vascular congestion. ECG: left ventricular hypertrophy. Impression ADHF; home drugs unchanged, discharged home (R7). The practice first hears of it from the patient on 2025-09-06; the ED record arrives late, on 2025-10-21 |
| 2025-09-10 | practice | post-ED visit from the patient's account (A4): breathing near his usual; 1+ ankle edema, faint basilar crackles at the right base, SpO2 95%; **ADHF recorded again** from his account; drugs unchanged |
| 2025-11-20 | practice | heart-failure follow-up: no edema, lungs clear; ADHF **resolved** on the list |
| 2026-02-25 | practice | last practice visit (last seen): routine follow-up for diabetes, blood pressure and heart failure; no edema; drugs unchanged; return in 6 months |
| 2026-05-30 | outside | ED visit (S, encounter 2, a Saturday), after the patient's two earlier episodes: 2 days of worsening breathlessness, orthopnea and swelling of both legs; no fever, cough or sick contacts. BP 160/90, HR 102, RR 22, SpO2 88% on room air. JVP raised, rales halfway up both lung fields, S3 gallop, 2+ pitting edema to the mid-shins. Sodium 138, potassium 4.5, creatinine 1.2, BNP 900 pg/mL, WBC 7.5, hemoglobin 14, platelets 250. Chest x-ray: bilateral pulmonary infiltrates (R6). Impression ADHF. Record received 2026-06-16 and filed; the practice's lists are not changed (after last seen) |
| 2026-09-10 | practice | booked visit after today: routine six-month follow-up |

Ages: 68 at the first ED visit, 69 at the second and third (R3).

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists chronic HFrEF from the start, and the first ED visit puts it on
  the problem list, yet the first ED visit is a first presentation (two months of new
  breathlessness and edema) and his drug list has no heart-failure therapy. Resolution:
  heart failure is not known at joining; the first ED visit is its first presentation;
  the reduced ejection fraction is established by the echo of 2024-12-19 (A3), and HFrEF
  is recorded then. Later outside records call it chronic HFrEF, as the source does.
- **R2.** Each encounter copies "quit 8 years ago". Resolution: he quit in 2016, so the
  chart says "quit 2016" (about 8 years before the first ED visit) and never repeats a
  stale count.
- **R3.** The second encounter calls him 68; by the seed's birth date he is 69 on
  2025-09-04. The chart uses 69.
- **R4.** An ED glucose labeled "fasting" is carried as the ED reported it: fasting
  glucose 160 mg/dL. No note interprets it as a diagnostic fasting value.
- **R5.** A growing waistline without a change in diet or weight is carried as his
  report only. No abdominal finding, ascites, imaging or new diagnosis is added.
- **R6.** The third ED chest x-ray reads "bilateral pulmonary infiltrates". It is carried
  in the radiology wording; with no fever, no cough and a normal WBC, the ED impression
  is ADHF only. No pneumonia diagnosis, no antibiotic.
- **R7.** The source gives no disposition and no drug change at any ED visit and keeps the
  same three home drugs throughout. Resolution: each ED record ends with discharge home,
  home drugs unchanged, PCP follow-up advised. The source's later "Acute decompensated
  heart failure (diagnosed ...)" stamp is not carried; each episode is recorded when the
  practice learns of it and resolved at a later follow-up.
- **R8.** A thoracentesis in the ED for small effusions is unusual; it is carried as the
  source has it, with the fluid analysis read as transudative.
- **Interobserver:** the first ED heard no crackles; the second heard crackles in both
  lungs. Both stay as written.
- **Left as the source has it:** no loop diuretic, beta blocker, mineralocorticoid
  antagonist or SGLT2 inhibitor is ever started for HFrEF, and no cardiology referral is
  made; hydrochlorothiazide continues despite the sulfa allergy; no statin is started for
  hyperlipidemia in a diabetic. Nobody remarks on any of these. The physician who
  inherits the chart can find them.

## Additions (A): physician to confirm

- **A1.** 2021-06-10 new-patient visit (five years with the practice).
- **A2.** 2024-12-10 post-ED visit, with the findings in the timeline, which orders the echo.
- **A3.** Transthoracic echo 2024-12-19: LVEF 35%, global hypokinesis, mildly dilated LV,
  no significant valve disease, no pericardial effusion. It supports the source's HFrEF.
- **A4.** 2025-09-10 post-ED visit from the patient's account, with the findings in the
  timeline; the ED record arrives later.
- **A5.** Annual low-dose CT lung cancer screening (25 pack-years, quit 2016, age 64 to 69):
  2021-07-20, 2022-07-12, 2023-08-08, 2024-07-16, 2025-07-15, each Lung-RADS 1, no nodules.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior primary-care office (received 2021-06-29): screening colonoscopy
  2017 with no polyps, repeat in 10 years; his three home drugs.
- Diabetes, blood pressure and (from December 2024) heart-failure visits about every 3 to
  6 months, with vitals, A1c, BMP or CMP, lipids yearly, urine albumin/creatinine yearly
  (normal), foot exams (normal sensation and pulses). A1c runs about 7.3 to 7.9 on
  metformin; LDL about 130 to 140 untreated; office BP about 136 to 146 over 84 to 90;
  creatinine about 1.0 to 1.2 with eGFR above 60; weight about 108 to 112 kg at 178 cm
  (BMI 34 to 35).
- Yearly diabetic eye exams at an outside optometrist: no retinopathy. The 2025 letter
  never arrives (the patient reports the exam).
- Abdominal aortic ultrasound once (man 65 to 75 who smoked): normal aortic caliber.
- Annual wellness visits with PHQ-2 screens (negative). Hepatitis C and HIV screening once
  (2021, nonreactive).
- Vaccines: high-dose influenza each fall; COVID-19 primary series spring 2021 at a
  pharmacy, then boosters and updated doses each fall; PCV20 once (2022); Shingrix two
  doses (2022); Tdap (2023); RSV once (2024).
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled visit and a no-show, reminders.
- After 2026-02-25: refills, messages and outside records only.
- Booked visit after 2026-08-27: routine six-month follow-up on 2026-09-10.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002219 on 2026-02-25. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
