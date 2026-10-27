# unit-1687 build requirements

Every clinical fact in this chart traces to one of: **S** the source as carried
here, **R** a numbered resolution, **A** a numbered addition, or **P** the
primary-care layer below. Nothing else. Texture (voice, scheduling, refills, portal
traffic, admin, missed visits) is free and carries no clinical fact. Re-voice
everything: no source sentence, clinician name, problem-list dump or quiz text.

## Who she is (S)

- Samantha Jenkins, MRN00001687, female, born 1996-01-28, Westhampton MA 01027,
  +1-413-555-0159, samantha.jenkins@example.test. White, not Hispanic or Latino,
  English, Private PPO. Married (no spouse named, no other household detail).
- Elementary school teacher. Never smoker. No alcohol.
- Allergies: no known drug allergies.
- Surgical history: none before 2026-04-05 (R5 adds the cesarean).
- Family history: mother had a cesarean delivery for failure to progress; mother
  has asthma; father has seasonal allergies.
- **Joins the practice 2023-08-24** (new-patient preventive visit), age 27,
  bringing: **Mild intermittent asthma** (since childhood), on **albuterol 90 mcg
  inhaler, 2 puffs every 4-6 hours as needed for wheezing**, used rarely.
  Obstetric history at joining: G0, never pregnant.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2023-08-24 (Thu) | Practice, establish care | New-patient preventive visit. Asthma mild intermittent, albuterol PRN continued (refill). Labs (P). |
| 2023-08-31 | Outside records received | Prior primary care records: asthma history, albuterol, cervical cytology 2022 normal (P). |
| 2023-10-30 | Practice, portal | Positive home pregnancy test, LMP 2023-09-19. **Supervision of normal pregnancy** starts. OB referral placed (A1). |
| 2023-11-20 | Outside OB letter | New-OB visit letter: intrauterine pregnancy, dating consistent with LMP, EDD 2024-06-25, G1P0. Prenatal labs unremarkable (S). Albuterol continued. OB referral done. |
| 2024-06-19 (Wed) | Outside, labor and delivery (Enc 0) | Admitted at 39 weeks (39w1d) with regular contractions every 3-4 minutes and slow cervical change; epidural; **oxytocin augmentation** for slow progress; **Active phase arrest** diagnosed (R1). Vital signs stable, no maternal fever or tachycardia. Fetal tracing reassuring at first. Two hours into oxytocin: 6 contractions in 10 minutes, cervix 5 cm, membranes intact; tracing now with repetitive late decelerations, baseline 140 bpm, moderate variability, no accelerations. Outcome per R2: spontaneous vaginal delivery. Pregnancy and active phase arrest resolve. |
| 2024-06-24 | Discharge summary received | As above. |
| 2024-08-22 (Thu) | Practice, annual physical | Nine weeks postpartum, PHQ-9 normal, asthma quiet. |
| 2024-10-28 | Practice, portal | Second pregnancy, LMP 2024-09-13, EDD 2025-06-20. **Supervision of normal pregnancy** starts again; care with the same outside OB. |
| 2024-11-25 | Outside OB letter | New-OB visit letter, G2P1, dating consistent; prenatal labs unremarkable (S, carried per R3). |
| 2025-06-15 (Sun) | Outside, labor and delivery (Enc 1) | Age 29. Admitted at term (39w2d) with contractions every 3 minutes, cervix 4 cm; uncomplicated pregnancy, prenatal labs unremarkable; epidural; on oxytocin augmentation for 6 hours. **Active phase arrest** diagnosed again. Vital signs stable, afebrile. Cervix still 4 cm, 80% effaced, fetal head at -2 station; strong contractions every 2-3 minutes. Continuous fetal monitoring reassuring with no decelerations; intrauterine pressure catheter average 220 Montevideo units over 10 minutes. Outcome per R2: vaginal delivery. Pregnancy and active phase arrest resolve. |
| 2025-06-18 | Discharge summary received | As above. |
| 2025-07-24 (Thu) | Practice, annual physical | Six weeks postpartum. Cervical cytology (P). PHQ-9 normal. |
| 2025-08-25 | Practice, portal | Third pregnancy. LMP 2025-07-12 by history but dated by first-trimester ultrasound (R4); EDD 2026-04-18. **Supervision of normal pregnancy** starts again. |
| 2025-09-22 | Outside OB letter | New-OB visit letter, G3P2, ultrasound dating, short interval since last delivery noted (R4); prenatal labs unremarkable (S). |
| 2026-04-05 (Sun) | Outside, labor and delivery (Enc 2) | Age 30 (R6). Admitted in active labor at 38w1d, progressed to complete dilation; epidural. Pushing effectively for 3 hours; fetal head at +1 station with no change in station over the last hour despite adequate contractions and maternal effort (R3); sagittal suture right occiput transverse; cervix fully dilated and effaced. Fetal tracing reassuring. Vital signs stable. Denies pain apart from fatigue. Prior labors 2024-06-19 and 2025-06-15 with active phase arrest. Diagnosis: arrest of descent, occiput transverse. **Cesarean delivery** (R5). Pregnancy resolves; **History of cesarean delivery** starts. Short ibuprofen course at discharge (A2). Discharged home 2026-04-08. |
| 2026-04-09 | Discharge summary received | As above. |
| 2026-05-21 (Thu) | Practice, office visit | Post-discharge follow-up, about 6.5 weeks after cesarean. Incision healed per patient and OB. PHQ-9 normal. CBC (P). |

## Resolutions (R)

1. **Active phase arrest at Enc 0.** The source's problem lists date active phase
   arrest to the first delivery, while that encounter describes slow cervical
   change needing oxytocin. The chart records it as the outside OB's discharge
   diagnosis at that delivery (labor arrested in the active phase, corrected with
   augmentation), resolved at discharge, and diagnosed again at Enc 1. ICD-10 O62.1.
2. **Unfinished labors at Enc 0 and Enc 1.** The source stops mid-labor both times
   and its profile lists no surgery, and Enc 2 is a labor with no prior uterine
   scar mentioned. So both end in vaginal deliveries. Enc 0: oxytocin stopped for
   tachysystole with late decelerations, position change and fluid bolus, tracing
   recovered, oxytocin resumed at a lower rate, amniotomy, progressed to complete,
   spontaneous vaginal delivery of a liveborn infant. Enc 1: at 4 cm she was not
   yet in active labor by current criteria, so the team continued oxytocin with
   amniotomy rather than operate; she then progressed and delivered vaginally, a
   liveborn infant. The discharge summary still lists "active phase arrest" as its
   working diagnosis, as the source does. No infant name, sex, weight or Apgar is
   carried; infants stay in their own records. No quiz text.
3. **Prenatal labs and Enc 2 timing.** "Prenatal labs unremarkable" is carried for
   all three pregnancies. Enc 2 says 3 hours of pushing with no change in station,
   then "no change for the past hour": the chart reads it as descent to +1 early in
   the second stage, then no further descent for the last hour of a 3-hour push.
   "Fetal tracing reassuring" at Enc 0 admission then late decelerations two hours
   later are sequential, both kept.
4. **Short interval to the third pregnancy.** The source dates force conception
   about 6 weeks after the second delivery. Dating is by first-trimester ultrasound
   (LMP 2025-07-12 by history fits it); the OB letter notes the short interval as
   a fact of the dates, with no diagnosis attached.
5. **Enc 2 outcome.** Arrest of descent at +1 station in occiput transverse after
   3 hours of pushing ends in cesarean delivery under the existing epidural, liveborn
   infant, uncomplicated recovery, discharged postoperative day 3 (2026-04-08).
   Diagnosis ICD-10 O64.0 (obstructed labor due to incomplete rotation of fetal
   head). Surgical history from then on: cesarean delivery 2026. History of
   cesarean delivery ICD-10 Z98.891.
6. **Ages.** Source ages 28 (Enc 0) and 29 (Enc 1) match the seed birth date; Enc 2
   "29-year-old" becomes 30. Source years 2020 and 2021 for the prior deliveries
   become 2024-06-19 and 2025-06-15.
7. **Multiparity / term pregnancy.** Enc 0 is her first delivery (G1P0); Enc 1 G2P1;
   Enc 2 G3P2 (multiparous). All three at term (39w1d, 39w2d, 38w1d). Recorded in
   the OB documents, not as separate conditions.
8. **Oxytocin, epidural, inpatient drugs** appear only in the outside documents'
   text, not as practice medications.

**Left as the source has it:** no asthma controller, no spirometry, no asthma
action plan beyond albuterol as needed. Nobody in the practice comments on the
pregnancy spacing, contraception or the labor history.

## Additions (A)

1. **OB referral** at the first positive pregnancy test (2023-10-30), done with the
   new-OB letter (2023-11-20). Later pregnancies stay with the same outside OB.
2. **Ibuprofen 600 mg tablet, 1 tablet every 6 hours as needed for incisional
   pain**, short course, prescribed by the hospital at discharge after cesarean
   (2026-04-08), finished by the 2026-05-21 visit.

## Primary-care layer (P)

- Visits: establish care 2023-08-24 (preventive, new patient); annual physicals
  2024-08-22 and 2025-07-24; post-cesarean follow-up 2026-05-21. A booked annual
  physical 2026-09-17. One cancelled or missed visit allowed.
- Body: height 165 cm. Weight 63.5 kg at joining, 69.0 kg at 2024-08-22, 68.0 kg at
  2025-07-24, 71.0 kg at 2026-05-21. Blood pressure normal throughout.
- Labs, all normal: establish care CBC, CMP, lipid panel, HIV-1/2 Ag/Ab, hepatitis
  C antibody; CBC 2024-08-22; CBC 2026-05-21.
- Cervical screening: prior practice cytology 2022 normal (in outside records);
  cervical cytology 2025-07-24, negative for intraepithelial lesion or malignancy.
- Immunizations: influenza each fall (2023-10-05, 2024-10-03, 2025-10-02) and
  COVID-19 updated formula with the 2024 and 2025 flu shots at the practice; Tdap
  in each pregnancy given by the OB (2024-04-03, 2025-03-26, 2026-01-21), reported
  in OB correspondence.
- Depression screens (PHQ-2/PHQ-9) normal at each practice visit.
- Albuterol refills by portal or pharmacy request about yearly; patient reports use
  a few times a year, no night symptoms, no oral steroids or ED visits.
- Records requests, OB correspondence, discharge summaries filed, portal messages
  about results and scheduling.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001687 on 2026-05-21. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
