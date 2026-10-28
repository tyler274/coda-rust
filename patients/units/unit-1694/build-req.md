# Build requirements: unit-1694 (Zoe Tremblay, MRN00001694)

Source: synthetic_hospital patient 1694, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz framing ("the obstetrician is considering...").

## Who she is (S)

Zoe Tremblay, born 1997-04-22, female, White, not Hispanic or Latino, English,
employer-based private PPO. Elementary school teacher. Never smoker. Alcohol: social
use before conception, none during pregnancy. No known drug allergies. No surgical
history. Family history: mother had preeclampsia during a pregnancy; a maternal cousin
has systemic lupus erythematosus.

Obstetric history (S, R2): one prior pregnancy, before she joined, with gestational
hypertension, delivered at full term without complication (one child, born 2021).
Her blood pressure was normal after that delivery.

Joined the practice on 2022-08-23 (A2, a new-patient visit). She brought:
myasthenia gravis (diagnosed before joining, followed by an outside neurology group,
well controlled) on **pyridostigmine 60 mg by mouth three times daily**, and a history
of gestational hypertension (first pregnancy).

Height 165 cm. Weight about 61 kg before pregnancy (BMI about 22).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-23 | practice | new-patient visit (A2): history as above; MG stable on pyridostigmine; BP normal; CBC, CMP, lipids, TSH; HIV and hepatitis C screening once (P), all normal |
| 2022-11-17 | outside | neurology follow-up letter (A1): MG stable, normal strength, no ptosis or diplopia; continue pyridostigmine 60 mg three times daily |
| 2023-08-22 | practice | annual physical (P) |
| 2023-11-14 | outside | neurology letter (A1), stable, unchanged |
| 2024-08-27 | practice | annual physical (P); cervical cytology, normal |
| 2024-11-19 | outside | neurology letter (A1), stable, unchanged |
| 2025-08-19 | practice | annual physical (P): planning a second pregnancy; **prenatal vitamin 1 tablet daily started** (R4) |
| 2025-10-09 | practice | pregnancy visit (A8): positive home test, LMP 2025-09-04, urine pregnancy test positive; **pregnancy recorded**; referred to obstetrics |
| 2025-10-28 | outside | neurology pregnancy counseling letter (A1): continue pyridostigmine through pregnancy; magnesium sulfate and certain other drugs can worsen MG and should be avoided or used only with neurology input |
| 2025-11-25 | outside | obstetrics prenatal visit, 11 weeks 5 days (S enc 0): pregnancy uncomplicated so far, taking prenatal vitamins, no complaints; worried about flu season and asks how to protect herself and the baby; vitals normal; exam unremarkable. **Influenza vaccine given**; Tdap counseled for 27 to 36 weeks (R3). Letter received by the practice |
| 2026-03-19 | outside | obstetrics prenatal visit, 28 weeks (R3, A7): **Tdap given** (the source's tetanus prophylaxis); BP normal. Record received |
| 2026-04-30 | outside | ED, 34 weeks 0 days (S enc 1, R1): 1 day of severe headache and visual disturbance (flashing lights); BP 158/102; 2+ lower-extremity edema; brisk deep tendon reflexes with clonus; urine protein 2+; creatinine 1.2; AST 70; ALT 80; platelets 180,000; hemoglobin 12; LDH normal (A4). **Severe preeclampsia without HELLP features**; sent to labor and delivery |
| 2026-04-30 to 2026-05-03 | outside | labor and delivery admission (A3): magnesium sulfate withheld because of MG (R6); levetiracetam 500 mg twice daily for seizure prophylaxis through 24 hours after delivery; IV hydralazine for severe-range pressures; oxytocin induction; **vaginal delivery 2026-05-01** at 34 weeks 1 day, liveborn infant. Pyridostigmine continued. Discharge labs: creatinine 1.0, AST 45, ALT 52, platelets 168,000. Discharged on no antihypertensive with a home cuff. ED record and discharge summary received 2026-05-04 |
| 2026-05-05 | practice | telehealth post-discharge visit (P), **last seen**: postpartum day 4; home BP 126 to 138 / 80 to 88; headache and visual symptoms gone; no MG weakness. **Severe preeclampsia without HELLP features added to the problem list; pregnancy resolved** |
| 2026-06-04 | outside | ED, 34 days postpartum (S enc 2, R1): headache, visual disturbance and right upper quadrant pain; BP 160/110; 2+ lower-extremity edema; brisk deep tendon reflexes; urine protein 3+; creatinine 0.9; platelets 110,000; AST 70; ALT 65; LDH normal (A4). Postpartum severe preeclampsia without HELLP features; obstetrics consulted and admits her (R6) |
| 2026-06-04 to 2026-06-07 | outside | postpartum admission (A5): magnesium sulfate withheld after neurology input (R6); levetiracetam 500 mg twice daily for 24 hours; IV hydralazine; **nifedipine ER 30 mg daily started** by obstetrics; discharge labs platelets 142,000, AST 44, ALT 41. ED record and discharge summary received 2026-06-09; practice lists not changed (after last seen) |
| 2026-07-09 | outside | obstetrics postpartum visit (A6): BP 116/74 on nifedipine; **nifedipine stopped** by obstetrics; counseled that preeclampsia raises later cardiovascular risk and to have yearly blood pressure checks with primary care. Letter received 2026-07-14, filed |
| 2026-09-17 | practice | booked: annual physical and postpartum follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's comorbidity "pregnancy (34 weeks gestation)" sits with both ED
  visits, five weeks apart, both with severe preeclampsia. A severe preeclampsia at 34
  weeks is delivered, so the 34 weeks belongs to the 2026-04-30 visit (LMP 2025-09-04,
  due date 2026-06-11). The 2026-06-04 visit is 34 days after a 2026-05-01 delivery: a
  delayed postpartum preeclampsia with severe features (ICD O14.15), still without
  HELLP features. The source's prenatal visit then falls at 11 weeks 5 days.
- **R2.** The source calls her first delivery uncomplicated and also says she had
  gestational hypertension in that pregnancy. Both hold: gestational hypertension in
  the first pregnancy, term delivery without complication.
- **R3.** The source stamps "tetanus prophylaxis" on the prenatal vaccination visit.
  Tdap in pregnancy belongs at 27 to 36 weeks, so at the 11-week visit the
  obstetrician gives the influenza vaccine (her flu-season question) and counsels
  Tdap, which is given at 28 weeks (2026-03-19). Tetanus prophylaxis is carried as
  that Tdap immunization, not as a problem-list diagnosis.
- **R4.** The prenatal visit lists only pyridostigmine but says she takes her prenatal
  vitamins; later lists add the vitamin. The practice starts it on 2025-08-19 when
  she is planning pregnancy; the obstetrics letter's list omits it, as outside lists do.
- **R5.** The source's first ED note says she was "seen January 15" with gestational
  hypertension. That is the re-dated prenatal visit, and it is a history of
  gestational hypertension; her pressures were normal in this pregnancy until 34 weeks.
- **R6.** The source ends with magnesium sulfate "being considered". Magnesium can
  precipitate a myasthenic crisis, so at both admissions it is withheld after
  neurology input and levetiracetam is used for seizure prophylaxis; severe-range
  pressures get IV hydralazine. The first ED note's treatment is not stated in the
  source; the chart carries it in the admission record (A3).
- **R7.** Creatinine 1.2 at 34 weeks and 0.9 five weeks postpartum are both carried;
  the first improves after delivery. Hemoglobin 12 is carried only at the first ED
  visit; the second ED visit's CBC states platelets as given.
- **Left as the source has it:** no low-dose aspirin is started in this pregnancy
  despite prior gestational hypertension and a mother with preeclampsia, and no note
  remarks on it. Her MG antibody status, thymus imaging and any thymectomy question are
  never documented. The source's surgical history is "none" throughout.

## Additions (A): physician to confirm

- **A1.** Outside neurology letters (2022-11-17, 2023-11-14, 2024-11-19): MG stable on
  pyridostigmine 60 mg three times daily, normal strength, no ptosis, diplopia,
  dysphagia or dyspnea. Pregnancy counseling letter 2025-10-28 as in the timeline.
  Standard ongoing care for a treated MG, run by neurology.
- **A2.** The 2022-08-23 new-patient visit (the source's first encounter is an
  obstetric visit of an established adult; she joined about four years before today).
- **A3.** The labor and delivery admission after the first ED visit: its management
  and discharge labs as in the timeline. A severe preeclampsia at 34 weeks must be
  delivered somewhere.
- **A4.** LDH normal at both ED visits, the evidence for "without HELLP features"
  together with platelets above 100,000.
- **A5.** The postpartum admission after the second ED visit, as in the timeline;
  nifedipine for postpartum severe-range hypertension.
- **A6.** The obstetrics postpartum visit that stops nifedipine.
- **A7.** The 28-week obstetrics visit that gives Tdap (R3).
- **A8.** The 2025-10-09 practice visit that confirms pregnancy and refers to obstetrics.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from her former primary care office may mention routine screening done
  there with normal results (a normal cervical cytology in 2021 during her first
  pregnancy).
- Yearly physicals with vitals and depression screens (PHQ-2 or PHQ-9, negative);
  basic labs as above; cervical cytology every three years (2024).
- Influenza vaccine each fall (2022, 2023, 2024 at the practice; 2025 at obstetrics);
  updated COVID vaccine in fall 2022 and 2023.
- Pyridostigmine and prenatal vitamin refills, pharmacy traffic, portal messages about
  results, scheduling and home blood pressure logs, a rescheduled physical.
- The 2026-05-05 telehealth visit is the last practice visit. After it: portal
  messages, refills and outside records filed as received, with no change to the
  practice's problem or medication lists.
- Booked visit after 2026-08-27: 2026-09-17 annual physical and postpartum follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001694 on 2026-05-05. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
