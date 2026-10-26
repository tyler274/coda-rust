# Build requirements: unit-1820 (Caitlin Maguire, MRN00001820)

Source: synthetic_hospital patient 1820, in `source.md` beside this file. This file and
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

Born 1991-03-27, female, White, not Hispanic or Latino, English-speaking, Medicaid,
lives in Westhampton. Administrative coordinator. Former smoker, quit in 2014 (R2).
No alcohol. Intravenous drug use stopped in 2014 and has stayed in sustained remission;
no medication for it. No known drug allergies. No surgical history before 2024 (R6).
Family history: mother with type 2 diabetes; paternal grandmother with breast cancer.
Single (marital status only; no partner appears in the chart).

Joined the practice 2019-09-10 (A1, a new-patient visit). She brought:

- **Chronic hepatitis C infection**, found on routine screening in 2015 at a prior
  practice and never treated (S: "diagnosed 5 years ago" at the 2020 prenatal visit).
- **History of intravenous drug use disorder in sustained remission** (stopped 2014,
  S: "discontinued 6 years ago" in 2020).
- No medications.

## Timeline (dates fixed; S unless marked)

Her four pregnancies are dated from the gestational ages the source gives:
pregnancy 1 LMP 2019-12-07, pregnancy 2 LMP 2021-11-21, pregnancy 3 LMP 2023-11-25,
pregnancy 4 LMP 2025-10-28. All prenatal and delivery care is with an outside
obstetrics group and its regional hospital labor and delivery unit.

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | new-patient visit (A1): history as above; exam normal; baseline labs (CMP, CBC, HIV screen negative); liver enzymes mildly raised (A2); **hepatology referral placed** |
| 2019-11-19 | outside | hepatology consult letter (A2): chronic hepatitis C, elastography 5.8 kPa (no significant fibrosis); plans sofosbuvir-velpatasvir for 12 weeks once Medicaid approves, with a pregnancy test before the start |
| 2020-01-08 | practice | telephone: positive home pregnancy test (P); hepatitis C treatment put on hold, hepatology told; **prenatal vitamin 1 tablet daily started** (A3); obstetrics referral placed; **pregnancy recorded** |
| 2020-02-18 | outside | obstetrics prenatal visit at 10 weeks (moved, R1); record received by the practice: uneventful so far, taking the prenatal vitamin; hepatitis C known 5 years, no known hepatitis A or B history, unsure of immunizations; denies jaundice, pruritus, abdominal pain; vitals normal; exam unremarkable. First-trimester labs: HCV RNA quantitative 1,500,000 IU/mL; hepatitis B surface antigen negative; hepatitis B core antibody total negative; hepatitis B surface antibody negative; hepatitis A IgG negative. **Susceptibility to hepatitis B virus infection diagnosed** |
| 2020-07-18 | outside | labor and delivery admission at 32 weeks: clear vaginal fluid for 2 hours; denies contractions, bleeding, recent intercourse; uncomplicated pregnancy with prenatal care; BP 120/75, HR 80, RR 16, T 37.0 °C (98.6 °F); speculum shows pooling of clear fluid in the posterior fornix, nitrazine positive, ferning seen; fetal tracing reassuring (baseline 140, moderate variability, no decelerations, no contractions); Hgb 12.5 g/dL, WBC 9,000/mm³, platelets 250,000/mm³; CRP normal. **PPROM diagnosed**; admitted for observation. Course (A4): betamethasone, latency antibiotics, induction at 34 weeks on 2020-08-01 with vaginal delivery, discharged 2020-08-03 |
| 2020-08-05 | outside | discharge summary received |
| 2020-08-18 | practice | post-discharge visit (P): recovering well; breastfeeding; hepatology follow-up after breastfeeding; **pregnancy resolved**, PPROM resolved |
| 2021-03-16 | outside | hepatology follow-up letter (A2): still breastfeeding, she prefers to wait to treat; plan to treat when she is ready and not pregnant or breastfeeding |
| 2021-12-23 | practice | portal message: positive home pregnancy test; obstetrics care; **pregnancy recorded** |
| 2022-07-24 | outside | admission at 35 weeks (labor and delivery, then ICU), age 31 (R3): 2 weeks of severe itching of palms and soles, worst at night; denies rash, jaundice, nausea, vomiting, bowel change; no skin lesions or rash; fundus appropriate for dates; normal fetal heart tones. Total bile acids 45 µmol/L, ALT 70 U/L, AST 68 U/L, alkaline phosphatase 120 U/L, total bilirubin 1.2 mg/dL. Early next morning (A5): glucose 58 mg/dL, uric acid 7.9 mg/dL, WBC 14,800/mm³, Hgb 12.4 g/dL, PT 16.2 s with INR 1.5; right upper quadrant ultrasound shows increased hepatic echogenicity, no biliary dilation. **Acute fatty liver of pregnancy diagnosed**; moved to the ICU; induction and vaginal delivery 2022-07-25; discharged 2022-07-30 |
| 2022-08-01 | outside | discharge summary received |
| 2022-08-09 | practice | post-discharge visit (P): itching gone; CMP, CBC, PT/INR back to her baseline (A2 levels, INR normal, glucose normal); **pregnancy resolved**; AFLP resolved |
| 2023-12-27 | practice | portal message: positive home pregnancy test; **pregnancy recorded** |
| 2024-07-20 | outside | admission at 34 weeks (labor and delivery, ICU): sudden sharp constant abdominal pain 8/10 with vaginal bleeding; denies trauma or intercourse; prenatal course complicated by severe polyhydramnios (**polyhydramnios recorded**, R5); BP 140/90, HR 110; uterus tender with frequent contractions; repetitive late decelerations; Hgb 10.2 g/dL (baseline 12.5), platelets 210,000/mm³, PT 12 s, aPTT 30 s, fibrinogen 400 mg/dL. **Placental abruption diagnosed**. Course (A6): emergency cesarean delivery the same day; discharged 2024-07-24 |
| 2024-07-26 | outside | discharge summary received |
| 2024-08-06 | practice | post-discharge visit (P): incision healing; Hgb 10.6 g/dL (A6); **pregnancy, polyhydramnios and abruption resolved** |
| 2024-10-08 | practice | nurse visit and draw: Hgb 12.2 g/dL (A6); influenza vaccine |
| 2025-11-29 | practice | portal message: positive home pregnancy test; **pregnancy recorded** |
| 2026-07-28 | outside | labor and delivery admission at 39 weeks: 1 hour of sudden severe abdominal pain and vaginal bleeding, baby moving less; history of prior abruption; pregnancy uncomplicated until now with prenatal care; BP 120/75, HR 88, T 37.0 °C (98.6 °F); uterus tender and hypertonic; ultrasound with no sign of abruption; fetal monitoring with recurrent late decelerations, minimal variability, no accelerations over 10 minutes; oxygen by face mask, IV fluids, left lateral position, and the tracing does not improve. **Fetal distress (Category III tracing) diagnosed**. Course (A7): emergency repeat cesarean the same day; discharged 2026-08-01 |
| 2026-08-04 | outside | discharge summary received |
| 2026-08-11 | practice | post-discharge visit (P): incision healing; CBC and CMP; **pregnancy and fetal distress resolved**; she is ready to treat hepatitis C now; **hepatology referral placed** (open today) |
| 2026-08-18 | practice | telephone: hepatology asks for a pre-visit HCV RNA; ordered (A2) |
| 2026-08-21 | practice | lab draw: HCV RNA quantitative, **pending** today; hepatology visit set for 2026-10-06 (P) |
| 2026-09-22 | practice | booked follow-up visit |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source puts the 10-week visit on 2020-01-31 and the PPROM at 32 weeks on
  2020-07-18, which is 24 weeks later, not 22. Resolution: the PPROM date stays and the
  10-week visit moves to 2020-02-18 (10 weeks 3 days; LMP 2019-12-07).
- **R2.** "Quit 6 years ago" is copied unchanged into every encounter from 2020 to 2026,
  and the profile says the same. Resolution: she quit smoking in 2014, the same year she
  stopped drug use. Outside obstetric notes may still copy "quit 6 years ago", as outside
  social histories do; the practice says 2014.
- **R3.** The 2022 note calls her 30; by her birth date she is 31 in July 2022. The chart
  uses 31.
- **R4.** The source's problem lists carry hepatitis B susceptibility, PPROM, AFLP and
  abruption forward as active through 2026 and stamp them with source dates. Resolution:
  hepatitis B susceptibility starts on 2020-02-18 and stays active. PPROM, AFLP,
  abruption and each pregnancy are recorded at their admission and resolved after
  delivery; later notes mention them as history. The 2026 HPI "prior abruption" refers to
  2024.
- **R5.** The 2024 note mentions severe polyhydramnios in that pregnancy's prenatal course
  but no list carries it. Resolution: polyhydramnios is recorded with the 2024 admission and
  resolved after delivery. The practice first learns of it from the discharge summary.
- **R6.** The profile lists no surgical history, but the abruption with repetitive late
  decelerations (2024) and the Category III tracing that does not improve (2026) end in
  cesarean deliveries (A6, A7). Resolution: no surgery before 2024; two cesareans after.
- **R7.** The 2022 presentation (palm and sole itching, bile acids 45) is labeled acute fatty
  liver of pregnancy by the source and carried as AFLP in 2024 and 2026. Resolution: the
  chart keeps AFLP as the 2022 diagnosis; the presenting itching and bile acids stay as the
  admission's first findings, and A5 gives the findings that support AFLP. No other
  liver-of-pregnancy diagnosis is named.
- **R8.** The 2026 ultrasound shows no abruption while the picture is that of one.
  Resolution: the 2026 discharge diagnosis is the Category III tracing with a clinically
  suspected recurrent abruption not seen on ultrasound; no new abruption is coded.
- **Left as the source has it:** she is not immune to hepatitis A or B and nobody gives a
  hepatitis A or B vaccine; no note comments on it. Her blood pressure of 140/90 at the
  2024 admission is not worked up and carries no diagnosis. No naloxone, no contraception
  discussion in practice notes, no iron after the 2024 delivery.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2019-09-10 (she has 7 years with the practice; the first source
  encounter is an outside prenatal visit). Prior-practice records arrive after it.
- **A2.** Hepatitis C ongoing care (the source carries chronic hepatitis C to 2026, never
  treated): hepatology consult 2019-11-19 with elastography 5.8 kPa (no significant
  fibrosis) and a plan for sofosbuvir-velpatasvir 12 weeks; the January 2020 pregnancy puts
  it on hold; hepatology follow-up 2021-03-16 while breastfeeding, she defers; at the 2021,
  2023 and 2025 physicals she still prefers to wait; in August 2026 she is ready and the PCP
  re-refers. Treatment is never started. When she is not pregnant, her liver enzymes run
  mildly raised: ALT about 40 to 60 U/L, AST about 35 to 50 U/L; bilirubin, albumin,
  platelets and INR normal. Hepatology letters do not mention vaccines.
- **A3.** Prenatal vitamin 1 tablet daily (the source's only home drug) starts at the
  2020-01-08 call and continues through today.
- **A4.** PPROM management: betamethasone 12 mg IM, two doses 24 hours apart; latency
  antibiotics for 7 days (IV ampicillin and erythromycin, then oral amoxicillin and
  erythromycin); induction at 34 weeks 0 days (2020-08-01), vaginal delivery, liveborn
  infant to the special care nursery; discharged 2020-08-03 on the prenatal vitamin.
- **A5.** Supporting findings for the source's AFLP: glucose 58 mg/dL, uric acid 7.9 mg/dL,
  WBC 14,800/mm³ (Hgb 12.4 g/dL on the same draw), PT 16.2 s (INR 1.5), right upper quadrant ultrasound with increased
  hepatic echogenicity and no biliary dilation. ICU care, dextrose infusion, induction and
  vaginal delivery 2022-07-25 at 35 weeks, liveborn infant to the nursery; enzymes and INR
  improving at discharge 2022-07-30; itching resolves after delivery.
- **A6.** Abruption course: emergency low transverse cesarean 2024-07-20 at 34 weeks,
  liveborn infant to the NICU; discharged 2024-07-24 on the prenatal vitamin with ibuprofen
  600 mg every 6 hours as needed and acetaminophen 1,000 mg every 6 hours as needed (short
  course), no opioid at discharge given her history. Hgb 10.6 g/dL on 2024-08-06, 12.2 g/dL
  on 2024-10-08.
- **A7.** Category III course: emergency repeat low transverse cesarean 2026-07-28 at 39
  weeks, liveborn infant to the NICU for observation; discharged 2026-08-01 on the prenatal
  vitamin with the same ibuprofen and acetaminophen short course, no opioid at discharge.
- **A8.** Pregnancy is recorded as a condition (pregnant state) from each positive test to
  each delivery.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice (received after A1) may mention routine screening done
  there with normal results: a normal Pap in 2018, and the 2015 hepatitis C finding.
- Height 165 cm. Weight 63 kg in 2019; about 70 to 73 kg at each post-delivery visit,
  settling back to about 64 to 67 kg between pregnancies. Blood pressure normal at the
  practice throughout.
- Yearly physicals when not pregnant (2021, 2023, 2025) with CMP and CBC; post-discharge
  visits after each delivery; HIV screening once (2019, negative); a Pap with HPV cotest in
  2023, normal; PHQ-2 or PHQ-9 at visits, low scores, including after each delivery.
- Obstetrics postpartum visit letters about 6 weeks after the 2020, 2022 and 2024 deliveries:
  routine recovery, normal exam, low depression screen.
- Influenza vaccine each fall; COVID vaccines (primary series spring 2021, then fall boosters
  2022 to 2025); Tdap in each pregnancy, given by obstetrics and reported in the discharge
  summaries.
- Prenatal vitamin refills, pharmacy traffic, portal messages about results and scheduling,
  records requests, a missed or rescheduled visit, reminders.
- Booked visit after 2026-08-27: follow-up on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
