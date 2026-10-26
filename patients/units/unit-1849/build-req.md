# Build requirements: unit-1849 (Monica Krause, MRN00001849)

Source: synthetic_hospital patient 1849, in `source.md` beside this file. This file and
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
never carry a source clinician's name, its "Active Problem List" dumps, or its
"(diagnosed 2020-01-15)" stamps.

## Who she is (S)

Born 1994-02-10, female, White, not Hispanic or Latino, English-speaking, lives in
Amherst, private PPO. Administrative assistant (a lot of keyboard work). Never smoker,
no alcohol. No known drug allergies. No surgical history. Family history: mother had
preeclampsia; mother had a precipitous labor. Married (texture).

Obstetric history (S, dated by R4): one prior pregnancy, uncomplicated, ending in a
full-term spontaneous vaginal delivery in 2021, before she joined the practice.

Joined the practice on 2024-07-08 (the source's first encounter is her new-patient
visit). Brought on joining: no active problems; a prenatal vitamin 1 tablet daily
(she is planning a second pregnancy, R1); no other drugs.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-07-08 (Mon) | practice | new-patient visit. Six weeks of numbness and tingling in the right thumb, index and middle fingers, worse at night and sometimes waking her; no trauma, no earlier episodes. Tinel's sign positive over the right carpal tunnel; Phalen's test reproduces the symptoms at 30 seconds; no swelling, redness or warmth at the wrist; strength and reflexes normal, rest of neuro exam normal. **Carpal tunnel syndrome (right) diagnosed** on exam. Night-time neutral wrist splint and keyboard ergonomics (A1). Prenatal vitamin continued (**historical**). Not pregnant (R1). Baseline labs and screening (P) |
| 2024-07-24 | outside records | prior primary-care/OB records received: 2021 term vaginal delivery, uncomplicated; Pap with HPV in 2022 normal (P); Tdap in 2021 pregnancy (P); childhood and COVID vaccines (P) |
| 2024-09-10 | practice (telehealth) | carpal tunnel follow-up (A1): wearing the splint nightly, now rarely woken; daytime tingling occasional with long typing days; continue splint and ergonomics |
| 2025-07-22 | practice | annual physical (P). Carpal tunnel quiet; uses the splint now and then (A1). Planning pregnancy within the year; prenatal vitamin continues |
| 2025-11-20 | practice (portal + nurse visit) | positive home pregnancy test; last menstrual period 2025-10-17; office urine pregnancy test positive; **Pregnancy diagnosed**; referred to an outside obstetrics group (A2) |
| 2025-12-12 | outside (OB) | new obstetric visit at 8 weeks 0 days: dating ultrasound matches the LMP, single intrauterine pregnancy, EDD 2026-07-24 (A2); prenatal labs normal (P); BP normal. Letter received 2025-12-17 |
| 2026-03-06 | outside (OB) | anatomy ultrasound at 20 weeks, normal (P). Report received 2026-03-10 |
| 2026-05-01 | outside (OB) | 28-week visit: 1-hour glucose challenge normal (P); Tdap given (P); BP normal. Letter received 2026-05-05 |
| 2026-06-19 | outside (OB) | 35 weeks 0 days: BP 142/92, repeat 140/90 four hours later; no headache or visual symptoms; urine protein/creatinine ratio 0.12; platelets, creatinine and liver enzymes normal (A3). **Gestational hypertension diagnosed**, mild, without proteinuria; managed with diet modification (S), home BP checks and weekly visits; delivery planned at 37 weeks, induction booked for 2026-07-08 (A3). Letter received 2026-06-23 |
| 2026-06-26 | outside (OB) | 36-week weekly visit (A3): asymptomatic, group B strep swab negative (P), induction date unchanged. Letter received 2026-06-30 |
| 2026-07-05 (Sun) | outside (ED) | 37 weeks 2 days (term). Four hours of regular contractions and lower back pain; no vaginal bleeding or leakage of fluid. Prenatal course notable for mild gestational hypertension without proteinuria on diet modification; history of carpal tunnel syndrome; prior pregnancy uncomplicated, term vaginal delivery. Prenatal vitamin daily. BP 138/88, HR 88, temperature 37.1 °C (98.8 °F). Fundal height consistent with gestational age; uterus irritable with palpable contractions every 5 minutes; fetal heart tones 140 and 142 bpm (two checks, one fetus, R2); speculum exam with closed cervical os, no bleeding, no amniotic fluid. Fetal fibronectin and cervical length not carried (R2). Sent up to labor and delivery |
| 2026-07-05 to 2026-07-07 | outside (hospital) | labor and delivery admission (A4): rapid cervical change after transfer, spontaneous vaginal delivery of a healthy infant about two hours after ED arrival; **Precipitous labor diagnosed** (R3); **Pregnancy resolved**. Postpartum BP highest 136/86, no antihypertensive. Discharged 2026-07-07 with a BP check in one week. ED record and discharge summary received 2026-07-08 |
| 2026-07-13 (Mon) | practice | postpartum and post-hospital visit (A5): BP 128/82; recovering well; depression screen low (P); precipitous labor moved to resolved (delivery complete); gestational hypertension stays on the list until the OB postpartum visit |
| 2026-08-17 | outside (OB) | six-week postpartum visit (A5): BP 118/76; **gestational hypertension resolved**. Letter received 2026-08-20; the practice updates the problem list |
| 2026-08-21 | practice (portal) | PCP note: after gestational hypertension, a yearly BP and lipid check at physicals (P) |
| 2026-09-29 | practice | **booked** annual physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's profile lists gestational hypertension and a prenatal vitamin at
  the 2024 hand visit, and pregnancy as a comorbidity, yet the 2026 ED note says her
  previous pregnancy was uncomplicated and the gestational hypertension is part of the
  current prenatal course. Resolution: she is not pregnant in July 2024; she takes a
  prenatal vitamin because she plans a second pregnancy. Gestational hypertension
  belongs only to the 2025–2026 pregnancy and is diagnosed at 35 weeks (A3). The 2024
  carpal tunnel is a non-pregnancy carpal tunnel in a keyboard worker. The ED note's
  "diagnosed 2020-01-15" stamp maps to the 2024-07-08 visit and is not carried as a stamp.
- **R2.** Fetal fibronectin and transvaginal cervical length are preterm-labor tests,
  not done at term. The positive fibronectin and the 25 mm cervical length are not
  carried; the speculum finding carries the cervical exam. The two fetal heart rates
  (140 and 142) are two checks of one fetus; the pregnancy is a singleton.
- **R3.** The source names precipitous labor but has a closed cervical os after four
  hours of contractions. Resolution: the active phase was very fast. After she reaches
  labor and delivery the cervix changes rapidly and she delivers about two hours after
  arriving in the ED; the hospital records it as precipitous labor. No laceration,
  hemorrhage or neonatal problem is recorded.
- **R4.** The source gives no year for the prior delivery. Resolution: 2021, at an
  outside obstetric practice, before she joined.
- **Left as the source has it:** carpal tunnel is diagnosed on exam alone; no nerve
  conduction study, steroid injection or hand-surgery referral is ever raised. A term
  laboring patient goes to the ED rather than straight to labor and delivery; nobody
  remarks on it. Gestational hypertension is managed with diet modification as the
  source says.

## Additions (A): physician to confirm

- **A1.** Standard conservative care for the source's carpal tunnel syndrome: a neutral
  night wrist splint and keyboard ergonomics from 2024-07-08; a telehealth follow-up on
  2024-09-10 (night waking now rare, occasional daytime tingling on long typing days);
  quiet at the 2025 physical with occasional splint use. Stays on the active list.
- **A2.** The pregnancy the 2026 ED visit needs: LMP 2025-10-17, positive home and office
  urine tests on 2025-11-20, referral to an outside obstetrics group, dating ultrasound at
  8 weeks matching the LMP, EDD 2026-07-24, single intrauterine pregnancy.
- **A3.** Evidence and plan for the source's gestational hypertension, diagnosed at
  35 weeks 0 days (2026-06-19): BP 142/92 and 140/90 four hours apart; no headache or
  visual symptoms; urine protein/creatinine ratio 0.12; platelets, creatinine, AST and
  ALT normal. Diet modification (S), home BP checks, weekly visits, delivery planned at
  37 weeks with an induction booked for 2026-07-08 (she labors first).
- **A4.** The labor and delivery admission that the source's precipitous labor implies:
  2026-07-05 to 2026-07-07, spontaneous vaginal delivery of a healthy infant, postpartum
  BP highest 136/86 with no antihypertensive, discharge on day 2 with a one-week BP check.
- **A5.** Postpartum course of the gestational hypertension: practice visit 2026-07-13
  with BP 128/82; OB six-week postpartum visit 2026-08-17 with BP 118/76, gestational
  hypertension resolved.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (Pap with HPV in 2022 normal, next due 2027).
- Height about 165 cm; weight about 64 kg before pregnancy, ordinary pregnancy gain,
  falling back toward baseline after delivery. BP normal outside the pregnancy levels above.
- Labs at joining: CBC, CMP, lipid panel, TSH, HIV and hepatitis C screening, all normal.
- Prenatal labs at the OB (normal): blood type O positive, antibody screen negative,
  rubella immune, hepatitis B surface antigen negative, HIV negative, syphilis
  nonreactive, hepatitis C negative, urine culture negative, CBC normal; group B strep
  negative at 36 weeks; anatomy ultrasound normal; 1-hour glucose challenge normal.
- Depression screens (PHQ-2/PHQ-9) at joining, the physical and postpartum, all low.
- Influenza vaccine each fall (2024, 2025); a COVID vaccine in fall 2024; Tdap in the
  2026 pregnancy at 28 weeks.
- Portal messages about results and scheduling, records requests, a rescheduled visit.
- Booked visit after 2026-08-27: annual physical on 2026-09-29.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001849 on 2024-09-10. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
