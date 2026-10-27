# Build requirements: unit-1676 (Willa Gonzalez, MRN00001676)

Source: synthetic_hospital patient 1676, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a hole;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its source years.

## Who she is (S)

Willa Gonzalez, born 1996-01-22, female, Hispanic or Latino (race recorded as other),
English-speaking, private insurance, lives in Springfield. Elementary school teacher.
Never smoker. Occasional social alcohol before pregnancy, none while pregnant. No known
drug allergies. No surgeries. Family history: a maternal second cousin with spina
bifida; her mother had one miscarriage. Married (A1).

Obstetric history on joining: one pregnancy, one uncomplicated full-term vaginal
delivery in 2021 (R3). Personal history of gallstones, no surgery (R2). Otherwise
healthy, no regular medications.

Joined the practice 2023-09-12 (new patient visit, A2), age 27, after her prior
primary-care physician left practice (texture). Height 163 cm, weight about 64 kg.

Obstetric care in both later pregnancies is with an outside obstetrics group at a
regional hospital; the practice receives their notes.

## Timeline (dates fixed; S unless marked)

Pregnancy 2 dating: 32 weeks 0 days on 2025-05-04 (LMP 2024-09-22, due 2025-06-29).
Pregnancy 3 dating: 28 weeks 0 days on 2026-07-21 (LMP 2026-01-06, 20 weeks on 2026-05-26).

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new patient visit (A2): history as above; normal exam; baseline labs (P) |
| 2024-03-11 | practice | portal: she and her husband are planning another pregnancy; **prenatal vitamin with folic acid (800 mcg folic acid), 1 tablet daily, started** (A3) |
| 2024-04-16 | practice | annual physical (P); cervical cytology (P) |
| 2024-10-24 | practice | nurse visit after a positive home test: urine pregnancy test positive, LMP 2024-09-22; **pregnancy recorded**; referral to obstetrics (P); influenza vaccine (P) |
| 2025-05-04 | outside | hospital labor and delivery triage (record received 2025-05-06): 32 weeks; regular painful contractions every 5 minutes for 2 hours, increasing; prior pregnancy uncomplicated with a healthy term baby; this pregnancy uneventful so far with regular prenatal care; no vaginal bleeding or fluid leakage. BP 120/78, HR 88, temperature 37.1 °C (98.8 °F). Fundal height 30 cm; uterus firm with palpable contractions. Sterile speculum exam (no pooling) and then cervical exam: 2 cm dilated, 50% effaced (R4). Fetal heart tones regular at 150. Fetal fibronectin positive. Transvaginal cervical length 25 mm. **Threatened preterm labor diagnosed.** Home medications on the record: prenatal vitamin with folic acid 800 mcg daily, **ferrous sulfate 325 mg daily** (started by obstetrics earlier in the pregnancy; recorded at the practice as an outside prescription, R5). Contractions spaced out and stopped with observation; discharged home with preterm labor precautions and obstetrics follow-up (A4) |
| 2025-06-02 | outside | obstetrics clinic visit at 36 weeks 1 day (R1; record received 2025-06-04): returns after the threatened preterm labor; no vaginal discharge, fever or urinary symptoms; vital signs normal; fetal heart rate regular without decelerations. Rectovaginal Group B Streptococcus screening culture positive. **GBS colonization in pregnancy diagnosed**, intrapartum antibiotic prophylaxis planned |
| 2025-06-22 | outside | admission for spontaneous labor at 39 weeks 0 days (A5): intravenous penicillin G in labor for GBS; uncomplicated vaginal delivery of a healthy term infant; discharged 2025-06-24; summary received 2025-06-26. **Threatened preterm labor and GBS colonization resolved; pregnancy ended** |
| 2025-08-04 | outside | obstetrics postpartum visit letter (P): recovered, normal exam, mood screen normal |
| 2025-10-14 | practice | annual physical (P) |
| 2026-02-03 | practice | portal message: positive home pregnancy test, LMP 2026-01-06; **pregnancy recorded**; referral to obstetrics (P). Prenatal vitamin and ferrous sulfate continue |
| 2026-05-26 | outside | maternal-fetal medicine ultrasound at 20 weeks (letter received 2026-05-28): **anencephaly** confirmed. Counseled that the condition is lethal; she chooses to continue the pregnancy with obstetrics and maternal-fetal medicine follow-up (R6) |
| 2026-07-21 | outside | hospital labor and delivery (records received 2026-07-24): 28 weeks; 4 hours of spontaneous rupture of membranes with regular contractions every 5 minutes; history of threatened preterm labor and GBS colonization in her previous pregnancy; two previous uncomplicated vaginal deliveries; anencephaly confirmed at 20 weeks. Tearful; understands the fetus's poor prognosis; worried about the risks to her own health; asks about management options and what each means for her. Vital signs stable (normal). Cervix 3 cm dilated, 80% effaced. Fetal heart tones not detected. **Preterm premature rupture of membranes with active preterm labor diagnosed.** Bedside ultrasound confirms no fetal cardiac activity (A6). Options reviewed; labor allowed to continue; vaginal delivery of a stillborn infant the same day without maternal complication; discharged 2026-07-23 (A6) |
| 2026-07-24 | practice | telephone check-in by the physician after the records arrive (P) |
| 2026-08-04 | practice | post-delivery visit (P): physically recovering; grieving; depression screen in the normal range |
| 2026-08-18 | outside | obstetrics postpartum visit letter (A7): bleeding settled, normal exam; because of the neural tube defect, folic acid 4 mg daily is advised starting at least one month before any future conception, with a preconception visit at maternal-fetal medicine when she is ready. No prescription started now |
| 2026-09-29 | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The practice dates put the 36-week GBS visit 15 days after the 32-week triage
  visit, which cannot be. Resolution: the 32-week visit stays on 2025-05-04 and the
  36-week visit moves to 2025-06-02 (36 weeks 1 day), a Monday.
- **R2.** The source lists "personal history of cholelithiasis" as a comorbidity while
  its notes call her history unremarkable and list no surgery. Resolution: gallstones
  were seen on an ultrasound at her prior practice in 2022 after a single episode of
  upper abdominal pain that settled on its own; no surgery, no episodes since. Recorded
  as history at joining; never active, never worked up again.
- **R3.** The first delivery has no date. Resolution: 2021, before she joined, term,
  vaginal, uncomplicated. "Multiparity" is carried as obstetric history text
  (G1P1 at joining, G2P2 after 2025, G3 after 2026), not as a problem-list entry.
- **R4.** The source reads cervical dilation off a speculum exam. Resolution: a sterile
  speculum exam with the fibronectin swab, then a cervical exam giving 2 cm and 50%.
  The fundal height "consistent with 30 weeks" is carried as 30 cm.
- **R5.** Prenatal vitamin and ferrous sulfate appear on every source list. Resolution:
  the prenatal vitamin starts 2024-03-11 (A3) and continues throughout; ferrous sulfate
  is an obstetrics prescription first seen on the 2025-05-04 record and continues
  through today. No anemia is diagnosed; blood counts stay normal.
- **R6.** The source has her at 28 weeks with a lethal anomaly but says nothing of the
  20-week counseling. Resolution: the maternal-fetal medicine letter records the
  confirmed diagnosis, the lethal prognosis and her choice to continue the pregnancy.
  No note debates that choice.
- Source notes call the 2025 pregnancy "uncomplicated" after the threatened preterm
  labor; the chart reads that as the delivery being uncomplicated and leaves the
  obstetrics wording as an outside voice.
- **Left as the source has it:** the 2025 triage record names no tocolytic, antenatal
  corticosteroid or GBS prophylaxis, and the chart adds none; no note comments on it.
  She was taking a prenatal vitamin with 800 mcg folic acid when the anencephaly
  pregnancy began; nobody remarks on that.

## Additions (A): physician to confirm

- **A1.** Married (the seed gives no marital status; the source's planned pregnancies
  imply a partner). Her husband appears only as a name-less mention.
- **A2.** 2023-09-12 new patient visit (the seed gives three years with the practice).
- **A3.** Prenatal vitamin start (the source names it; it must start somewhere).
- **A4.** The 2025-05-04 triage outcome: contractions stopped with observation and she
  went home (she is still pregnant at 36 weeks).
- **A5.** The 2025 term vaginal delivery with intrapartum penicillin G (the 2026 note
  counts two previous uncomplicated vaginal deliveries; GBS colonization "requiring
  intrapartum antibiotic prophylaxis" means it was given).
- **A6.** The 2026-07-21 course after the source stops: ultrasound confirming no
  cardiac activity, vaginal delivery of a stillborn infant the same day, discharge
  2026-07-23. Recorded as intrauterine fetal death in the anencephalic pregnancy
  (the specific form of "fetal heart tones not detected").
- **A7.** The 2026-08-18 postpartum letter with the standard folic acid 4 mg advice
  after a neural tube defect pregnancy (advice only; no new prescription).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there (cervical
  cytology 2021 normal), with normal results.
- A healthy young woman's care: a new patient visit, yearly physicals with vitals,
  blood counts and metabolic panels, a lipid panel once, HIV and hepatitis C
  screening once (negative), cervical cytology in 2024 (normal), depression screens
  (PHQ-2/PHQ-9) in the normal range, alcohol screening.
- Influenza vaccine each fall (including in pregnancy), COVID vaccines; Tdap in the
  2025 pregnancy given by obstetrics (third trimester, named on an outside record).
- Weight about 64 kg before pregnancies, about 66 to 67 kg after; blood pressure normal.
- The obstetrics referrals for each pregnancy, records requests and receipt notes,
  portal messages about results and scheduling, a refill of the prenatal vitamin or
  ferrous sulfate, one cancelled visit.
- Booked visit after 2026-08-27: a routine follow-up on 2026-09-29.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001676 on 2025-10-14. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
