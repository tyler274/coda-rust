# Build requirements: unit-1782 (Tiana Toussaint, MRN00001782)

Source: synthetic_hospital patient 1782, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished assessment text.

## Who she is (S)

Tiana Toussaint, born 1990-04-23, female, Black, not Hispanic or Latino, English,
commercial PPO, Chesterfield 01012. Administrative assistant. Never smoker, no alcohol.
No known drug allergies. Surgical history: laparoscopic cholecystectomy 2019 and a
cesarean delivery in February 2023 (source years 2015 and 2019, re-dated). Family
history: mother had preeclampsia needing early delivery; father has essential
hypertension; maternal grandmother had a stroke.

Joined the practice on 2023-04-11 (A1), about two months after the 2023 cesarean,
sent back to primary care by her obstetrician for chronic hypertension. She brought:
chronic hypertension, class I obesity (height 163 cm, weight about 88 kg, BMI about 33),
labetalol 200 mg twice daily (started by obstetrics during the 2023 pregnancy) and a
prenatal vitamin with DHA daily, which she keeps taking.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-04-11 | practice | new patient visit (A1): history as above; BP mildly above goal on labetalol, no change; baseline labs normal |
| 2023-08-14 | practice (portal) | positive home pregnancy test, LMP 2023-07-11 (A2); labetalol and prenatal vitamin continued; prenatal care with an outside obstetrics group |
| 2023-09 | outside | obstetric intake letter: estimated due date 2024-04-16, chronic hypertension on labetalol 200 mg twice daily (A2) |
| 2024-01-22 | outside | ED (obstetrics) at 27 weeks 6 days: sudden painless vaginal bleeding for one hour; uneventful pregnancy until then, normal fetal movement, no trauma, fever, abdominal pain, contractions, leakage of fluid or recent intercourse; anxious, hemodynamically stable; gravid abdomen soft and nontender; fetal heart baseline 140/min, moderate variability, no decelerations; speculum exam with blood in the vaginal vault; **digital cervical examination deferred as contraindicated**; ultrasound ordered and obstetrics consulted; **admitted antepartum** (R4, A3) |
| 2024-02-18 | outside | during the antepartum stay, **preeclampsia superimposed on chronic hypertension** diagnosed (S: "1 week ago" from 2024-02-25) |
| 2024-02-25 | outside | antepartum stay at 32 weeks 5 days, on magnesium sulfate for seizure prophylaxis: short of breath and palpitations; BP 150/100, HR 88, RR 22; lethargic, deep tendon reflexes absent, shallow respirations; arterial gas pH 7.35, PaCO2 45, PaO2 95; Na 138, K 4.2, Cl 100, HCO3 24, Ca 8.9, Mg 5.2 mg/dL (elevated). **Hypermagnesemia (magnesium toxicity)** (R2): infusion stopped, IV calcium gluconate given, reflexes and alertness return; magnesium 2.4 the next morning (A4) |
| 2024-03-06 | outside | **repeat cesarean at 34 weeks 1 day**, liveborn infant (A5); discharged 2024-03-10 on labetalol 200 mg twice daily and the prenatal vitamin |
| 2024-03-19 | practice | post-discharge visit (P): pregnancy, preeclampsia, the cervical-exam contraindication and hypermagnesemia resolved on the list; history of preeclampsia recorded; labetalol continued |
| 2024-04-24 | outside | obstetric postpartum letter (A5): incision healed, BP acceptable on labetalol, back to primary care for blood pressure |
| 2025-06-16 | practice (portal) | positive home pregnancy test, LMP 2025-05-16 (A6); labetalol and prenatal vitamin continued; obstetrics again |
| 2025-07-22 | outside | obstetric intake letter: estimated due date 2026-02-20, chronic hypertension, prior preeclampsia, two prior cesareans (A6) |
| 2025-09-05 | practice | **last seen**: chronic hypertension in pregnancy at 16 weeks; BP at goal on labetalol 200 mg twice daily; baseline CBC, CMP and urine albumin/creatinine normal (P) |
| 2026-01-09 | outside | admitted to obstetrics, ICU level care, at 34 weeks 0 days, third pregnancy (R5): new right upper quadrant pain and persistent nausea; recent fatigue and poor appetite; BP 148/96 on arrival; mild scleral icterus, RUQ tenderness, reflexes brisk and symmetric; urine protein 2+; platelets 74,000/µL, AST 290, ALT 235, total bilirubin 3.1, LDH 680, haptoglobin undetectable; fetal heart tracing normal. **Severe preeclampsia** (the letter may call this the HELLP form, R6). **Cesarean the same day** with postpartum magnesium (A7); discharged 2026-01-13 on labetalol 200 mg twice daily; platelets 146, AST 52, ALT 66, total bilirubin 0.9 on 2026-01-12 (A7) |
| 2026-01-16 | outside | ED (obstetrics) one week postpartum (R3): BP 150/100, HR 88; postpartum preeclampsia, magnesium sulfate started (A8); on the infusion: difficulty breathing and generalized weakness, lethargic, patellar reflexes absent, breath sounds decreased both sides without wheezes or crackles, RR 12; arterial gas pH 7.35, PaCO2 50, PaO2 70; Na 138, K 4.2, Cl 100, HCO3 24, Ca 8.9, Mg 5.2 mg/dL (R7). Infusion stopped, IV calcium gluconate; magnesium 2.6 the next morning; discharged 2026-01-18 on labetalol 200 mg twice daily (A8) |
| 2026-02-24 | outside | obstetric postpartum letter (A9): BP 132/86 on labetalol, incision healing well, platelets 231, AST 24, ALT 21; resume primary care for blood pressure |
| 2026-09-14 | practice | booked: postpartum blood pressure follow-up |

Every 2026 record reaches the practice after `last_seen` and is filed as received; the
practice problem and medication lists stay as they were on 2025-09-05 (pregnancy
still listed, labetalol and prenatal vitamin).

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Surgical years follow the re-dated timeline: cholecystectomy 2019, first
  cesarean February 2023. Conception for the 2024 pregnancy follows about five months
  later, as the source's own spacing has it.
- **R2.** The source lists "Hypomagnesemia" yet its only magnesium is 5.2 and marked
  elevated, in a woman on magnesium sulfate with absent reflexes and shallow breathing.
  Resolution: the chart records hypermagnesemia (magnesium toxicity), never
  hypomagnesemia, resolved at the post-discharge visit.
- **R3.** Encounter 3 is "one week postpartum" but the source places it a month after
  the delivery admission. Resolution: it is dated 2026-01-16, one week after the
  2026-01-09 cesarean.
- **R4.** Encounter 0's assessment stops mid-sentence. Resolution: the ED record ends
  with the ultrasound, the obstetric consult and antepartum admission (the "prolonged
  antepartum admission" of encounter 1). No ultrasound result and no placental
  diagnosis reach the chart; the discharge summary says only that digital cervical
  exams were avoided for the rest of the pregnancy.
- **R5.** Encounter 2 calls the 2026 pregnancy her second; with the 2023 cesarean and
  the 2024 delivery it is her third. The chart says third. Its "returns after 2020
  admissions" is the 2024 admission. Its "denies history of hypertension" is not
  carried: she knows she has chronic hypertension on labetalol.
- **R6.** The source labels the 2026 illness severe preeclampsia; its platelets,
  transaminases, LDH and haptoglobin are the HELLP form of it. The problem is severe
  preeclampsia; the discharge summary may name HELLP as its form. No other diagnosis.
- **R7.** Encounter 3 reports magnesium as 5.2 mEq/L, encounter 1 as 5.2 mg/dL.
  Resolution: 5.2 mg/dL both times.
- **R8.** Encounter 0 says she takes nothing but prenatal vitamins while every list has
  labetalol. Resolution: she takes both throughout.
- **R9.** The source's profile lists severe preeclampsia, hypomagnesemia and the
  cervical-exam contraindication from the start. Each starts at its own encounter.
- **Left as the source has it:** no low-dose aspirin is started in either pregnancy
  despite chronic hypertension and prior preeclampsia; no one comments. Labetalol stays
  200 mg twice daily through both pregnancies and both postpartum readmissions; no
  outpatient blood pressure drug is added. Neither infant's course is described.

## Additions (A): physician to confirm

- **A1.** 2023-04-11 new-patient visit (the 2026 encounter has her "returning" with
  known chronic hypertension and the seed has her with the practice about four years).
- **A2.** The 2023 pregnancy becoming known (portal message, obstetric intake letter):
  the source's 2024 pregnancy has to start somewhere.
- **A3.** Antepartum admission from the 2024-01-22 ED visit through delivery, to
  carry encounter 1's "prolonged antepartum admission".
- **A4.** Treatment of magnesium toxicity (stop the infusion, IV calcium gluconate) and
  a repeat magnesium of 2.4 the next morning, as any obstetric team would.
- **A5.** Repeat cesarean on 2024-03-06 at 34 weeks 1 day ending the 2024 pregnancy,
  discharge 2024-03-10, and the 2024-04-24 postpartum letter.
- **A6.** The 2025 pregnancy becoming known (portal message, intake letter).
- **A7.** Cesarean on 2026-01-09 for severe preeclampsia at 34 weeks, 24 hours of
  postpartum magnesium, and recovery labs on 2026-01-12 (platelets 146, AST 52, ALT 66,
  total bilirubin 0.9).
- **A8.** Encounter 3 framed as a postpartum preeclampsia readmission on which
  magnesium was started; its toxicity treated as in A4; magnesium 2.6 next morning.
- **A9.** 2026-02-24 obstetric postpartum letter (platelets 231, AST 24, ALT 21,
  BP 132/86), the usual next step after the source ends.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior primary care office and obstetrics may mention routine
  screening done there (a normal Pap in 2022), with normal results.
- Blood pressure visits every 4 to 6 months when not pregnant, with home readings
  mostly 126 to 136 over 80 to 88 on labetalol; office BP about 130-138/84-88; weight
  86 to 91 kg outside pregnancy (class I obesity throughout); BMP or CMP, CBC, lipids,
  A1c and urine albumin/creatinine at intervals, all normal.
- Prevention for a woman in her 30s: cervical screening (Pap with HPV, January 2025,
  normal), HIV and hepatitis C screening once (negative), influenza and updated COVID
  vaccines in the fall when she is with the practice, depression screens (low scores,
  including after the 2024 delivery).
- Refills, pharmacy traffic, portal messages about results, refills and scheduling,
  a missed visit, records requests.
- After 2025-09-05: only messages, labetalol refills and the outside records above,
  filed as received.
- Booked visit after 2026-08-27: 2026-09-14, postpartum blood pressure follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001782 on 2025-09-05. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
