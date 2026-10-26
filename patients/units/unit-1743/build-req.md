# Build requirements: unit-1743 (Ava Paquette, MRN00001743)

Source: synthetic_hospital patient 1743, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who she is (S)

Ava Paquette, born 1995-09-22, female, White, not Hispanic or Latino, English,
Chesterfield MA 01012, private commercial insurance, administrative assistant. Never
smoker. Alcohol: none during pregnancy, social use before conception. No known drug
allergies. No surgical history. Family history: mother had preeclampsia in pregnancy;
father has chronic hypertension.

Obstetric history: gestational hypertension in her first pregnancy, resolved after
delivery (year not stated; the chart never gives one). The source pregnancies below
are her second and third (R1).

Joins the practice 2024-10-29 (A1), about seven weeks after delivering her second
child (below). Brings: history of gestational hypertension (first pregnancy), the
severe preeclampsia of September 2024, prenatal vitamin with folic acid 1 tablet
daily and ferrous sulfate 325 mg daily (both started by her obstetrician).

## Records (seed: gaps)

- September 2024 hospital stay: the patient describes it at her first visit; the
  practice requests records and the hospital's ED record and discharge summary arrive
  2024-12-10, about six weeks later.
- October 2025 obstetric office visit: the obstetric practice never sends a note. The
  practice learns of the gum lesion only from the patient's portal message the next
  day, in her words (no measurements beyond what she relays).
- December 2025 hospital stay: the practice learns of it from the patient's phone call
  2025-12-08; the ED record and discharge summary arrive 2026-01-08 after a request.
- The practice medication list lags: ferrous sulfate stays on it after she stops it
  (March 2026) until she mentions it at the June 2026 physical.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-07 (Sat) | outside, regional hospital ED, admitted to labor and delivery | Second pregnancy, **32 weeks' gestation**. Two days of persistent headache and visual disturbance, intermittent epigastric pain. BP 158/102 on two readings 4 hours apart. 2+ pitting edema of the legs, brisk reflexes, mild right upper quadrant tenderness. Hemoglobin 11.2 g/dL, platelets 110,000/uL, creatinine 1.1 mg/dL, AST and ALT elevated (no values given), urine protein/creatinine ratio 0.4, 24-hour urine protein 350 mg. **Severe preeclampsia without HELLP features diagnosed.** Home medications prenatal vitamin and ferrous sulfate 325 mg daily. Inpatient care and delivery as A2 |
| 2024-09-09 | outside | **Vaginal delivery at 32 weeks 2 days** after induction (A2); preeclampsia resolves after delivery; discharged 2024-09-12 with no antihypertensive (A2) |
| 2024-10-29 | practice | new patient visit (A1); history as above; BP normal; baseline labs (P); records request |
| 2024-12-10 | outside records received | September 2024 ED record and discharge summary, as above |
| 2025-03-18 | practice | annual physical (P); BP normal; lipids, A1c, CMP, CBC normal |
| 2025-06-03 | practice (portal) | patient reports a positive home pregnancy test (third pregnancy; last menstrual period about 2025-04-28), prenatal care with her obstetric practice (A3) |
| 2025-10-13 (Mon) | outside, obstetric office | third pregnancy at 24 weeks (R2), uncomplicated so far. A rapidly growing, painless, red gum nodule that bleeds easily with brushing; exam: 1 cm pedunculated erythematous smooth-surfaced nodule on the upper gingiva, no pus or infection, rest of oral and general exam normal; vital signs normal. **Pyogenic granuloma (pregnancy epulis) diagnosed.** Age 30 (R3). No note reaches the practice |
| 2025-10-14 | practice (portal) | patient describes the nodule and the obstetrician's diagnosis; physician advises a soft brush and a dental visit; pyogenic granuloma recorded |
| 2025-12-02 (Tue) | outside, regional hospital ED, admitted to labor and delivery | third pregnancy, 31 weeks 1 day (R2). Sudden severe constant abdominal pain and vaginal bleeding; prenatal course unremarkable until this. Reports amphetamine use several times during this pregnancy, no other substances. BP 160/110, HR 102, temperature 37.2 C (99.0 F). Tender uterus, small amount of vaginal bleeding, fetal heart tones 140/min. Hemoglobin 10.2 g/dL, platelets 120,000/mm3, creatinine 1.2 mg/dL, AST 70 U/L, ALT 65 U/L, urinalysis protein 3+. **Placental abruption (R4) and severe preeclampsia without HELLP features in this pregnancy (R5) diagnosed; stimulant (amphetamine) use recorded** (R6). Delivery and course as A4 |
| 2025-12-02 | outside | **Vaginal delivery at 31 weeks 1 day** the same evening (A4); discharged 2025-12-05 with no antihypertensive (A4) |
| 2025-12-08 | practice (phone) | patient calls to report the delivery; postpartum BP visit booked |
| 2025-12-11 | practice | postpartum BP check (A5): BP normal without medication; gum nodule smaller; CBC hemoglobin 11.4 g/dL with normal platelets, CMP normal (A5); iron continued |
| 2026-01-08 | outside records received | December 2025 ED record and discharge summary, as above |
| 2026-01-13 | practice | six-week postpartum follow-up (A5): gum nodule gone, **pyogenic granuloma resolved** (A6); amphetamine use from the hospital record discussed: she confirms use several times during the pregnancy and none since delivery; counseling and resources offered (P); PHQ-9 low |
| 2026-03 | (no contact) | she stops ferrous sulfate on her own (A7) |
| 2026-06-09 | practice | annual physical (P): BP normal; lipids, A1c, CMP, CBC normal; **ferrous sulfate removed from the list** (A7); prenatal vitamin continues |
| 2026-10-20 | practice, booked | nurse visit: influenza vaccine and blood pressure check |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls the gestational hypertension her "first pregnancy" and then
  shows two more pregnancies (September 2024 at 32 weeks; a new, uncomplicated
  pregnancy in October 2025 that the source calls her pregnancy after the
  preeclampsia one). Resolution: three pregnancies: the first (gestational
  hypertension, before she joined, year unstated), the second (2024, severe
  preeclampsia) and the third (2025). The October 2025 note's "severe preeclampsia in
  her prior pregnancy" is the 2024 one.
- **R2.** The source gives "Pregnancy at 32 weeks gestation" without placing it, and no
  gestational age for 2025. Resolution: 32 weeks belongs to September 2024. The third
  pregnancy is dated from a last menstrual period of about 2025-04-28: 24 weeks on
  2025-10-13 and 31 weeks 1 day on 2025-12-02, in keeping with a mid-pregnancy
  pregnancy epulis and a viable fetus with heart tones of 140.
- **R3.** The October 2025 note calls her 29; born 1995-09-22 she is 30 on 2025-10-13.
  The chart uses 30 (and 28 in September 2024).
- **R4.** The December 2025 encounter stops at the findings (a vignette with no stated
  diagnosis). Resolution: the hospital record names placental abruption, the
  diagnosis the presentation sets up (sudden constant pain, bleeding, tender uterus,
  severe-range pressure, stimulant use). Physician to confirm.
- **R5.** The December 2025 note says "with gestational hypertension and preeclampsia"
  while also calling the prenatal course unremarkable until that day. Resolution: the
  pregnancy was normotensive until 2025-12-02; that day's BP 160/110 with protein 3+
  and AST 70 / ALT 65, platelets 120,000 is severe preeclampsia without HELLP
  features in the third pregnancy, diagnosed at that visit. The October 2025
  "active" severe preeclampsia is the 2024 history.
- **R6.** Amphetamine use appears only in December 2025; the earlier social histories
  say no drugs. Resolution: first recorded 2025-12-02 as stimulant (amphetamine) use,
  ICD-10 F15.90; earlier notes do not mention it. The source never says whether it was
  prescribed; the chart does not say either, and no amphetamine prescription exists.
- **R7.** The source's comorbidity "Uric acid crystal nephropathy" has no supporting
  finding in any encounter (no uric acid, urine crystals or kidney imaging), and the
  creatinines of 1.1 and 1.2 go with preeclampsia. Resolution: not carried. No uric
  acid result and no kidney diagnosis appear.
- **R8.** The September 2024 AST and ALT have no values in the source. Resolution: the
  hospital record says "elevated" with no numbers, and the chart carries no number.
- **R9.** The source lists severe preeclampsia and the pregnancy tumor as active at later
  visits. Resolution: each pregnancy's preeclampsia resolves after its delivery; the
  practice lists "history of preeclampsia". The pyogenic granuloma resolves after the
  December 2025 delivery (A6).
- **Left as the source has it:** no low-dose aspirin in the third pregnancy despite the
  prior severe preeclampsia; no one in the chart raises it. No dental visit result
  is ever documented.

## Additions (A): physician to confirm

- **A1.** New patient visit 2024-10-29, about seven weeks postpartum, establishing
  primary care after the preeclampsia (the source's first encounter is an ED visit and
  she needs a primary-care home).
- **A2.** Standard care at the September 2024 admission, in the hospital record:
  magnesium sulfate for seizure prophylaxis, IV labetalol for severe-range pressure,
  betamethasone two doses; induction after the steroid course and vaginal delivery
  2024-09-09 at 32 weeks 2 days, liveborn infant to the NICU; pressure back below
  140/90 by postpartum day 2; discharged 2024-09-12 on prenatal vitamin and ferrous
  sulfate only, BP check with obstetrics.
- **A3.** The June 2025 portal message announcing the third pregnancy (the source's
  October 2025 pregnancy must begin somewhere).
- **A4.** Standard care at the December 2025 admission, in the hospital record:
  magnesium sulfate, IV labetalol, first betamethasone dose; labor followed and
  vaginal delivery the evening of 2025-12-02 at 31 weeks 1 day, abruption confirmed
  at delivery (retroplacental clot), liveborn infant to the NICU; amphetamine use
  noted and social work consulted; pressure below 140/90 by postpartum day 2;
  discharged 2025-12-05 with no antihypertensive.
- **A5.** The two postpartum primary-care visits (2025-12-11 and 2026-01-13), with BP
  normal without medication at both. The 2025-12-11 CBC shows hemoglobin 11.4 g/dL
  (expected after the abruption and delivery; she is already on iron, which continues)
  with normal platelets; the CMP is normal (creatinine and liver tests recovered).
- **A6.** The pregnancy epulis regresses after delivery: smaller on 2025-12-11, gone on
  2026-01-13 (gum exam normal). Recorded resolved 2026-01-13.
- **A7.** Ferrous sulfate stopped by the patient in March 2026, once she was no longer
  pregnant; the June 2026 CBC is normal.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice or her obstetrician may mention routine screening done
  there, with normal results (cervical screening up to date with obstetrics).
- Height about 165 cm; weight about 74 kg in October 2024, about 71 to 72 kg by spring
  2025, about 79 kg two weeks after the December 2025 delivery, about 72 kg by June
  2026. Blood pressure normal at every practice visit.
- Because of preeclampsia twice and a father with hypertension: yearly BP, lipids and
  A1c with normal results, and counseling about later cardiovascular risk.
- Baseline at joining: CBC, CMP, lipids, A1c, TSH, one-time HIV and hepatitis C
  screens, all normal. Postpartum CBC and CMP in December 2025 as A5.
- Depression screening (PHQ-9) at joining, after each delivery and yearly, low scores.
- Influenza vaccine at the practice in fall 2024 and fall 2025; COVID vaccine fall
  2024. Tdap in the third pregnancy is given by obstetrics (patient report only).
- Portal messages about results and scheduling, a return-to-work form after the 2025
  delivery, a missed nurse BP check in January 2025, reminders.
- Booked visit after 2026-08-27: a nurse visit for influenza vaccine and BP check.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
