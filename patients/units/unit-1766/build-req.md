# Build requirements: unit-1766 (Yolanda Doyle, MRN00001766)

Source: synthetic_hospital patient 1766, in `source.md` beside this file. This file and
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

Born 1999-06-18, female, Hispanic or Latino (race recorded as other), English-speaking,
commercial PPO, lives in western Massachusetts (seed address), phone and email from the
seed. Administrative assistant. Never smoker. Alcohol: occasional before pregnancy (R1);
none from confirmation of pregnancy on. No known drug allergies. No surgical history.
Family history: mother with type 2 diabetes, father with hypertension. Has a partner
(S: the partner brings her to the ED); marital status partnered.

Joined the practice in November 2022 (A1), age 23, bringing type 2 diabetes treated by
her prior primary care office since 2021 on **metformin 500 mg twice daily**. Nothing
else active. Height 158 cm, weight about 71 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-11-29 | practice | new patient visit (A1): history as above; type 2 diabetes well controlled, A1c 6.6; metformin 500 mg twice daily continued; NKDA; P layer labs and vaccines |
| 2022-11 to 2024-11 | practice | diabetes care and two physicals (P); A1c stays 6.4 to 6.8 (R3) |
| 2024-11-08 | practice | **last practice visit**: annual physical with diabetes review (P). No practice visit after this day |
| 2025-05-12 | message | patient writes that she has a positive home pregnancy test and an obstetrics appointment; asks whether to keep taking metformin. PCP: keep taking it, the obstetrics team will direct diabetes care in pregnancy. Practice lists not changed (records texture) |
| 2025-07-18 | outside | obstetrics prenatal visit at 16 weeks: no complaints, pregnancy uneventful so far; type 2 diabetes well controlled on metformin; **prenatal vitamin 1 tablet daily** on her list (R1); NKDA; no surgeries; family and social history as above; BP 120/78, HR 72, temperature 37.0 C (98.6 F); exam unremarkable; urine dipstick leukocyte esterase 1+, nitrite, protein, glucose and ketones negative; urine culture >100,000 CFU/mL E. coli: **asymptomatic bacteriuria in pregnancy**. The letter reaches the practice late, 2025-09-30, with the culture result |
| 2025-12-16 | outside | ED, about 37 weeks pregnant (R2): 3 days of urinary frequency and suprapubic pressure, now fever, chills, nausea, right flank pain and dysuria; brought by her partner; no vaginal bleeding or discharge; normal fetal movement; no prior urinary infections; no recent sick contacts; attended early prenatal care but missed her last two prenatal visits; history of asymptomatic bacteriuria found at prenatal care in July 2025 (R2); meds metformin 500 mg twice daily and prenatal vitamin daily; NKDA. T 38.8 C (101.8 F), pulse 110, BP 108/70; flushed, mildly diaphoretic; right CVA tenderness; mild suprapubic tenderness without rebound or guarding. Urinalysis positive leukocyte esterase and nitrite with significant pyuria. Renal ultrasound: right hydronephrosis without calculi or structural abnormality. IV fluids and IV antibiotics (A2, R4); impression right pyelonephritis in pregnancy and admitted to the obstetric service (R4) |
| 2025-12-19 | message | patient writes from the hospital: in the ED Tuesday with a kidney infection, admitted, on IV antibiotics, feeling better (P texture carrying R4) |
| 2026-01-27 | outside | ED record received by fax (records texture). No discharge summary ever arrives |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's profile lists the prenatal vitamin and "alcohol: none
  (discontinued upon confirmation of pregnancy)" as standing facts, but her pregnancy
  begins in spring 2025, after the practice last sees her. Resolution: from 2022 to 2024
  the practice records occasional alcohol (a drink or two a month) and no prenatal
  vitamin; the prenatal vitamin and abstinence appear only in the 2025 obstetric and ED
  records and the patient's messages. The practice's medication list never gains the
  prenatal vitamin (records texture: the list lags).
- **R2.** Gestational dating: 16 weeks on 2025-07-18 puts her at about 37 weeks on
  2025-12-16; the ED record says about 37 weeks. The ED history's "January 2020" for the
  bacteriuria follows the re-dated timeline: July 2025. Her age in both records is 26.
- **R3.** The source calls the diabetes well controlled but gives no numbers, and the
  first source encounter names metformin without a dose. Resolution: metformin 500 mg
  twice daily throughout (the ED dose); practice A1c values 6.6 (2022-11), 6.4 (2023-05),
  6.7 (2023-11), 6.5 (2024-05), 6.4 (2024-11). No other diabetes drug.
- **R4.** The ED plan stops at "IV fluids and antibiotics". Resolution: the ED record
  names the antibiotic (A2), gives the impression right pyelonephritis in pregnancy (the
  picture the source describes, the course of its bacteriuria), and ends with admission
  to the obstetric service for IV antibiotics and fetal monitoring. No ED urine culture
  result, blood count or chemistry is carried. The pyelonephritis appears only in the
  outside records and the patient's message; it never enters the practice problem list.
- **Practice timing.** The practice last saw her on 2024-11-08, before both source
  encounters. Both reach the chart only as records and messages; the practice's problem
  list (type 2 diabetes) and medication list (metformin) stay as they were at that visit.
  Neither asymptomatic bacteriuria nor the pregnancy is added to the practice lists.
- **Left as the source has it:** the obstetric record documents no treatment of the
  bacteriuria and no repeat culture; she stays on metformin alone through pregnancy, with
  no insulin and no A1c in the obstetric or ED records; the ED record carries no urine
  culture result. The pregnancy's outcome never reaches the practice. Nobody in the chart
  remarks on any of this.

## Additions (A): physician to confirm

- **A1.** Practice intake call (2022-11-14) and new patient visit 2022-11-29 (the source
  presents an established diabetic on metformin; she must have a primary care home, and
  the seed gives two years with the practice to the last visit). Prior primary care
  records arrive after that visit, on 2023-01-10.
- **A2.** The ED's IV antibiotic is ceftriaxone 1 g IV (the standard choice for
  pyelonephritis in pregnancy with no drug allergy); it appears only in the ED record.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Prior primary care records (received 2023-01-10): type 2 diabetes diagnosed 2021 there
  and metformin started; Pap 2021 normal; Tdap 2020; childhood hepatitis B and adolescent
  HPV series complete; COVID-19 primary series 2021.
- Diabetes visits about every 6 months 2022-2024 with vitals, A1c, metabolic panel,
  lipids yearly (normal, LDL under 100; no statin at her age), urine albumin/creatinine
  yearly (normal), foot exams normal. BP normal throughout (around 112-122/70-78).
- Diabetic eye exam letter from an optometrist in early 2023: no retinopathy. The 2024
  eye exam she reports as normal; its letter never arrives (records texture).
- Prevention: hepatitis C and HIV screening once (negative); Pap at the 2024 physical,
  negative for intraepithelial lesion; PCV20 once (diabetes); influenza each fall
  2022-2024; updated COVID-19 vaccines fall 2023 and 2024; PHQ-2 or PHQ-9 negative.
- Refills (metformin 90-day fills continue through 2025-2026, last approved with a note
  that she is overdue for a diabetes visit), portal messages about results and
  scheduling, recall reminders after the last visit that go unanswered.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001766 on 2024-11-08. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
