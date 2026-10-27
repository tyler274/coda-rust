# Build requirements: unit-1696 (Maya Pereira, MRN00001696)

Source: synthetic_hospital patient 1696, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style phrasing.

## Who she is (S)

Maya Pereira, born 1996-05-22, female, White, not Hispanic or Latino, English,
Greenfield MA 01301, Blue Cross Blue Shield PPO. High school mathematics teacher.
Never smoker. Social alcohol before her pregnancies, none during them. No known drug
allergies. No surgical history until the cesarean in A6 (July 2026, after the last
visit, so the practice's lists never show it). Family history: father with type 1
diabetes, mother with autoimmune hypothyroidism, a sister with celiac disease.

Joined the practice 2021-06-08 (A1), age 25, bringing autoimmune hypothyroidism on
levothyroxine 75 mcg daily (R1). No other problems or drugs on joining. Type 1
diabetes is not known on joining: it is diagnosed in May 2022 (R2, A2).

Obstetric course (R3): three pregnancies, all with outside obstetrics.
G1: LMP 2023-11-16, delivered 2024-08-23. G2: LMP 2024-11-29, delivered
2025-08-30 at 39 weeks. G3: LMP 2025-10-27, delivered 2026-07-24 at 38 weeks.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-06-08 | practice | new-patient visit (A1): history as above; levothyroxine 75 mcg daily continued; height 165 cm, weight 63 kg; HIV and hepatitis C screening negative (P) |
| 2022-05-10 | practice | three weeks of thirst, frequent urination and about 4 kg unintended weight loss (A2); random glucose 287 mg/dL on a BMP with normal bicarbonate; A1c 10.4%; urine glucose positive, ketones negative. New diabetes, likely autoimmune given age, build and father's history; urgent endocrinology referral (A2) |
| 2022-05-13 | outside | endocrinology consult (letter received 2022-05-16) (A2): GAD65 antibody positive at 48 U/mL (ref <5); C-peptide 0.4 ng/mL with simultaneous glucose 241 mg/dL; tissue transglutaminase IgA negative (celiac screen, given the sister). **Type 1 diabetes mellitus diagnosed. Insulin glargine 16 units at bedtime and insulin lispro with meals at 1 unit per 10 g carbohydrate started** |
| 2022-08-22 | outside | endocrinology follow-up letter: fasting readings still above target; **glargine increased to 22 units at bedtime** (A3); lispro 1:10 unchanged |
| 2023-09-12 | practice | diabetes and thyroid follow-up; planning a pregnancy; **prenatal vitamin with iron 1 tablet daily started** (A4) |
| 2023-12-19 | practice | first pregnancy confirmed (urine pregnancy test positive); **pregnancy with pre-existing type 1 diabetes recorded**; outside obstetrics booked |
| 2024-08-23 | outside | **admission in labor (encounter 0)**, age 28: regular contractions every 5 minutes for 6 hours, membranes ruptured 2 hours before arrival; pregnancy uncomplicated, prenatal labs unremarkable; BP 120/78, HR 88, temperature 37.2 °C (99.0 °F); fetal heart tracing reassuring; sterile speculum exam clear fluid; cervix 6 cm, 90% effaced, vertex at -1 station; 4 hours later 7 cm and fully effaced. **Normal spontaneous vaginal delivery** (R4) of a healthy liveborn infant (A5). Home medications insulin glargine 22 units at bedtime, lispro 1:10 with meals, levothyroxine 75 mcg daily, prenatal vitamin with iron daily. Discharge summary received 2024-08-27 |
| 2024-09-10 | practice | postpartum visit; pregnancy closed on the problem list; prenatal vitamin continued (she is breastfeeding: texture) |
| 2025-01-07 | practice | second pregnancy confirmed (urine pregnancy test positive); **pregnancy with pre-existing type 1 diabetes recorded again** |
| 2025-05-06 | practice | mid-pregnancy follow-up: frequent high and low glucose readings, managed with endocrinology; insulin doses as listed (R5) |
| 2025-08-30 | outside | **admission in active labor at 39 weeks (encounter 1)**, age 29, G2P1, one year after her first vaginal delivery: glucose hard to control this pregnancy with frequent hyperglycemia and hypoglycemia; adherent to insulin and prenatal care; otherwise unremarkable prenatal course; moderate distress with contractions, vital signs stable, fetal tracing reassuring, cervix 6 cm and 90% effaced. **Normal spontaneous vaginal delivery** 2 hours after admission of a macrosomic boy, 4,500 g, Apgar 8 and 9 at 1 and 5 minutes; jittery and irritable at 30 minutes. Infant's results: glucose 25 mg/dL, hemoglobin 17 g/dL, calcium 9.2 mg/dL, magnesium 2.1 mg/dL, CBC otherwise unremarkable. **Neonatal hypoglycemia** in the infant, handed to the newborn team (R6). Same home medications. Discharge summary received 2025-09-03 |
| 2025-09-16 | practice | postpartum visit; pregnancy closed |
| 2025-12-16 | practice | third pregnancy confirmed (urine pregnancy test positive); **pregnancy with pre-existing type 1 diabetes recorded again** |
| 2026-06-02 | practice | **last visit** (last_seen): 31 weeks pregnant; diabetes and thyroid check; TSH, free T4, A1c drawn and resulted 2026-06-03; plan unchanged |
| 2026-07-24 | outside | **admission in labor at 38 weeks (encounter 2, re-dated R3)**, age 30, G3P2, after vaginal deliveries in August 2024 and August 2025, the second with hard glucose control and an infant with neonatal hypoglycemia. Membranes ruptured spontaneously soon after admission with a large volume of clear fluid; 15 minutes later a sudden sense of something between her legs, intense anxiety, decreased fetal movement; fetal heart rate fell from normal to 70 bpm for over a minute despite repositioning and oxygen; cervix 7 cm, a pulsating firm cord palpable beside the presenting part, head high and poorly applied. **Umbilical cord prolapse.** **Emergency cesarean delivery** (A6). Same home medications. Discharge summary received 2026-07-29 and filed; the practice's lists are not updated |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source gives no onset for hypothyroidism. Resolution: it came with her
  from her prior practice (prior records: diagnosed 2018, thyroid peroxidase
  antibodies positive, no value carried), on levothyroxine 75 mcg daily throughout.
- **R2.** The source lists type 1 diabetes on every list but dates it "18 months
  prior to" the first pregnancy. On the re-dated timeline the first conception is
  late 2023, so diabetes is diagnosed in May 2022, during her time with the practice.
  It is not on any list before 2022-05-13, and notes before then never mention it.
- **R3.** The source places the third delivery six months after the second, which
  cannot be a term pregnancy. Resolution: encounter 2 moves from 2026-02-20 to
  2026-07-24 (38 weeks, LMP 2025-10-27). Ages follow the birth date: 28 at the first
  delivery, 29 at the second, 30 at the third (the source's "29" is not carried). The
  source's "January 2020 and January 2021" become August 2024 and August 2025.
- **R4.** Encounter 0's note stops at 7 cm. Its primary diagnosis is normal
  spontaneous vaginal delivery, and later encounters confirm it; the chart records
  that delivery the same day.
- **R5.** The source keeps glargine 22 units, lispro 1:10 and levothyroxine 75 mcg
  unchanged at all three admissions, despite pregnancy and "difficult glucose
  control". Resolution: the listed doses are what every list carries; any day-to-day
  pregnancy adjustment happens at endocrinology and is not itemized.
- **R6.** Neonatal hypoglycemia and the labs in encounter 1 (glucose 25, hemoglobin
  17, calcium 9.2, magnesium 2.1, CBC) belong to the newborn. They appear only as text
  in the mother's discharge summary and later histories, never as her own results
  or problems. Umbilical cord prolapse and the deliveries likewise live in the
  outside records and obstetric history.
- **R7.** Encounter 2 says her previous pregnancy and delivery were uncomplicated,
  contradicting encounter 1 (hard glucose control, neonatal hypoglycemia). Not
  carried: the second pregnancy is recorded as encounter 1 has it. Encounter 0's
  "uncomplicated pregnancy" stays for the first pregnancy.
- **R8.** The source copies the deliveries and the neonatal hypoglycemia onto
  problem lists. Resolution: each appears only after it happened.
- **Left as the source has it:** levothyroxine stays at 75 mcg through all three
  pregnancies and no one raises it; no note comments on it. Three pregnancies within
  three years go without any contraception discussion in the chart.

## Additions (A): physician to confirm

- **A1.** 2021-06-08 new-patient visit (years with the practice; the source brings
  her with established problems).
- **A2.** The May 2022 presentation and diagnosis (R2): symptoms, glucose 287, A1c
  10.4, urine glucose positive and ketones negative, normal bicarbonate; the
  endocrinology consult with GAD65 48 U/mL, C-peptide 0.4 ng/mL at glucose 241, and
  a negative tTG-IgA. Insulin starts here because the source names it.
- **A3.** Glargine starts at 16 units and reaches the source's 22 units in August
  2022 (a usual starting dose for her weight, then titration).
- **A4.** The prenatal vitamin starts at the September 2023 preconception visit,
  since the source lists it at every admission.
- **A5.** The first infant is healthy and liveborn; her postpartum stay is
  uncomplicated and she goes home on postpartum day 2.
- **A6.** Encounter 2 stops at the prolapse. Its usual next step is immediate
  cesarean: emergency low transverse cesarean under general anesthesia; liveborn
  girl, Apgar 6 and 8 at 1 and 5 minutes, taken to the NICU for observation; the
  mother's recovery uncomplicated, home on postoperative day 3 on her home
  medications.

## Primary-care layer (P)

- Visits: new-patient visit, an annual physical in 2022 and 2023, diabetes and
  thyroid visits two to three times a year, pregnancy confirmation and mid-pregnancy
  check-ins, postpartum visits. Endocrinology runs insulin; outside obstetrics runs
  prenatal care; the practice follows thyroid and the whole.
- Labs those conditions call for: TSH (normal throughout), A1c every few months
  after diagnosis (about 7 on insulin, a little higher in the second pregnancy,
  about 6.8 to 7.0 otherwise), lipids yearly (normal), urine microalbumin/creatinine
  yearly (normal), CMP and CBC (normal).
- Outside records as they arrive (records are complete): endocrinology letters
  every few months; yearly diabetic eye exams with no retinopathy; obstetric intake
  summaries with unremarkable prenatal labs; the three discharge summaries.
- Prevention: HIV and hepatitis C screening once (negative); cervical cytology
  normal in 2020 at her prior practice and in March 2023 here; HPV series completed
  in adolescence (prior records); COVID primary series in spring 2021 and boosters
  each fall; influenza each fall; Tdap in each pregnancy at obstetrics; PCV20 and a
  three-dose adult hepatitis B series in 2022 after the diabetes diagnosis; PHQ-9
  screens low, including after each delivery.
- Refills, prior authorization for insulin pens, portal messages about results and
  scheduling, one missed visit.
- Nothing booked after today (seed `booked` false).
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
