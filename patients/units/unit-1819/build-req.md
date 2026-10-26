# Build requirements: unit-1819 (Nilda Mendez, MRN00001819)

Source: synthetic_hospital patient 1819, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its clinical-trial quiz stem ("Based on the educational
objective,"), "A peripheral smear is shown", or the riddle phrasing of its plans ("a drug
that induces differentiation ... through interaction with a nuclear receptor",
"ATRA therapy mechanism", "dopaminergic therapy").

## Who she is (S)

Born 1988-02-20, female, Hispanic, Blue Cross PPO, lives in Westhampton, English
speaking, elementary school teacher. Never smoker, no alcohol, no illicit drugs. NKDA.
No surgery. Family history: mother rheumatoid arthritis and menopause at 38 (the
menopause age is first recorded at the January 2024 gynecology visit), maternal aunt
systemic lupus erythematosus, father hypertension, and a reported family history of
hereditary hemorrhagic telangiectasia with the relative not specified (R10).

Joined the practice in 2020 (first visit 2020-09-22, a new-patient visit, age 32; A1). She
brought one problem: **acute promyelocytic leukemia diagnosed in 2018 at age 30, treated
at an outside cancer center, in complete remission** (R1). The transferred records say
only that treatment was completed and remission achieved; the chart names no regimen. Her
one medication then: **vitamin D3 2000 units daily**, over the counter (S: on every list).
Systemic lupus erythematosus and hypergonadotropic hypogonadism are not known at joining:
each starts at its encounter (R2, R3).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-09-22 | practice | new-patient visit (A1); history as above; vitamin D3 on her list; normal exam |
| 2022-07-12 | practice | annual physical; she has started trying to conceive (no partner details); **folic acid 1 mg daily started** (A2) |
| 2023-03-21 | practice | tired and aching in her joints for roughly five months, with a rash on her face after time outdoors in the sun; referred to rheumatology (P) |
| 2023-04-11 | outside | rheumatology consult letter (age 35): half a year of tiredness and aching joints, plus a rash across the face that comes up now and then after time in the sun; malar rash; mild synovitis of wrists and knees. Hb 10.5 g/dL, WBC 3,500/µL, platelets 100,000/µL, ANA positive 1:640, anti-dsDNA positive, C3 and C4 low. **Systemic lupus erythematosus diagnosed** (R2), with leukopenia, anemia and **thrombocytopenia** attributed to it; **hydroxychloroquine 200 mg daily started** (R4); sun protection; rheumatology follow-up about every 3 months; baseline retinal exam (P). No trial enrollment (R5) |
| 2023-12-05 | practice | about 17 months trying to conceive without success; regular cycles; referred to gynecology (P) |
| 2024-01-10 | outside | gynecology consult letter: no pregnancy after a year and a half of trying, though her periods come like clockwork every 28 days; history of SLE and APL in remission; mother's menopause at 38; unremarkable exam. TSH normal, prolactin normal, estradiol low-normal, anti-Müllerian hormone low, early-follicular FSH 14.8 mIU/mL (A3). **Diminished ovarian reserve; hypergonadotropic hypogonadism added to the problem list** (R3). Plan: findings and options discussed; no treatment started (R6) |
| 2025-10-10 | practice | same-day visit (A4): for about a month a tiredness that will not lift and headaches that come and go; now and then she bruises more easily than usual and her gums bleed a little; she blames a stressful stretch and bad nights; mild pallor, scattered petechiae on the lower legs; vitals normal. Stat CBC: **Hb 8.4 g/dL, platelets 34,000/mm³, WBC 2,600/mm³**; critical result phoned that evening and she is directed to the hospital |
| 2025-10-11 | outside | admission to hematology/oncology at a regional hospital (a Saturday; age 37); the admission record quotes the counts above and the same exam; vitals normal. Peripheral smear reviewed (R7). Bone marrow biopsy with t(15;17): **relapsed acute promyelocytic leukemia** (R1). **Tretinoin (ATRA) 45 mg/m² per day in two divided doses started** (A5) |
| 2025-11-07 | outside | discharge summary received (A5); practice post-discharge visit within 2 weeks (P) |
| 2025-12-10 | outside | hematology letter: marrow in complete remission; ATRA consolidation continues (A5) |
| 2026-03-18 | outside | hematology letter: consolidation finished, PML-RARA negative by PCR, second complete remission; **ATRA stopped**; PCR surveillance every 3 months (A5) |
| 2026-04-16 | outside | rheumatology visit for a severe SLE flare: worse joint pain with synovitis of wrists and knees, malar rash back, fatigue; anti-dsDNA positive, C3 and C4 low; CBC normal, creatinine normal, urinalysis without protein or blood. **Prednisone 60 mg daily started** (A6). The practice had arranged the urgent visit after a portal message on 2026-04-13 (P) |
| 2026-04-27 | outside | emergency department (age 38): for two days she has heard voices, which warn that someone is tailing her and that harm is coming to her relatives, along with paranoid delusions; no psychiatric illness before; on high-dose prednisone for the flare (the ED's home medication list leaves it out, R11); by her account nothing stressful has happened lately, and she uses no street drugs and does not misuse alcohol (R8). Vitals within normal limits. Restless and agitated, she seems to be attending to voices no one else hears; the neurological exam is nonfocal. Na 140, K 4.1, Cl 102, bicarbonate 24, BUN 18, creatinine 0.9, glucose 98, calcium 9.2; CBC within normal limits; urine toxicology negative. **Medication-induced (corticosteroid) psychosis** (R9). Psychiatry consult in the ED; **olanzapine 5 mg nightly started**; rheumatology reached by phone and the **prednisone taper begins** (A7) |
| 2026-04-28 | outside | ED observation discharge home with family; follow-up with the practice and rheumatology (A7) |
| 2026-05-04 | practice | post-ED visit (P): voices and paranoia gone since about 2026-04-30; calm, good insight; taper on schedule |
| 2026-06-01 | practice | follow-up: no psychotic symptoms for a month; **olanzapine stopped** (A7) |
| 2026-06-17 | outside | hematology surveillance letter: PML-RARA negative by PCR; second complete remission continues; next PCR in 3 months (A5) |
| 2026-07-06 | outside | rheumatology letter: flare settled; **prednisone off** (A7); hydroxychloroquine continues |

Medications at today (2026-08-27): hydroxychloroquine 200 mg daily, folic acid 1 mg daily,
vitamin D3 2000 units daily.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Every note before the hematology admission calls her APL a history in complete
  remission, while the admission confirms APL by marrow and starts treatment, and the ED
  note lists both "APL" and "history of APL in complete remission". Resolution: APL was
  diagnosed in 2018 and treated before she joined; the October 2025 admission is a
  relapse, seven years later; by the ED visit she is in a second complete remission
  (A5). The ED's two APL entries collapse into one: relapsed APL, in second complete
  remission. Its "(ATRA therapy mechanism)" label is not carried.
- **R2.** The profile and every list carry SLE and hydroxychloroquine from the first visit,
  but the rheumatology encounter is the diagnostic one (six months of symptoms, rash,
  synovitis, cytopenias, ANA, dsDNA, low complements). Resolution: SLE is diagnosed on
  2023-04-11. Nothing before that date mentions it.
- **R3.** The profile lists hypergonadotropic hypogonadism from the start; the gynecology
  visit is where low AMH and low-normal estradiol are found, with regular 28-day cycles.
  Resolution: the problem is diagnosed on 2024-01-10 as diminished ovarian reserve with an
  elevated FSH (A3), and the problem list names it hypergonadotropic hypogonadism, as the
  source does. Her cycles stay regular; no hormone therapy.
- **R4.** Hydroxychloroquine is on the list before the visit that diagnoses SLE.
  Resolution: rheumatology starts it on 2023-04-11. The rheumatology letter's medication
  list shows it as started that day.
- **R5.** The rheumatology note ends in a quiz about a three-drug SLE trial and SLEDAI
  confidence intervals. Resolution: dropped. She is in no trial and no SLEDAI is carried.
  The letter's plan is the hydroxychloroquine start and follow-up above.
- **R6.** The gynecology note stops at its finding. Resolution: the letter closes with the
  diagnosis and a discussion of the findings, with no treatment and no further referral.
- **R7.** "A peripheral smear is shown". Resolution: the smear is reported only as
  consistent with the marrow diagnosis, with no further descriptors.
- **R8.** The ED records that she denies recent stressors six months after a leukemia
  relapse. Resolution: kept as her report to the ED, without comment.
- **R9.** The source's diagnosis is "medication-induced psychosis (dopaminergic therapy)",
  yet she takes no dopaminergic drug and the note blames high-dose prednisone for a
  severe flare. Resolution: medication-induced psychosis from corticosteroids. ATRA had
  stopped 40 days earlier (2026-03-18, almost six weeks) and is not implicated.
- **R10.** The profile carries a family history of hereditary hemorrhagic telangiectasia,
  which no note mentions and which names no relative. Resolution: kept as a family history
  she reports at joining, with the relative not specified; it adds no family member. It
  sits in the practice's family history from 2020-09-22 and may appear in the family-history
  section of the 2025 hematology admission record. No note examines or tests her for it.
- **R11.** The ED note says prednisone was recently started at high dose for a severe flare,
  but its home medication list shows only hydroxychloroquine, folic acid and vitamin D3.
  Resolution: the ED's list is a stale copy of an older outside list, as outside lists
  are; the ED record keeps it as written, and its history carries the prednisone. The
  practice's list carries prednisone 60 mg daily from 2026-04-16 (A6) until it is off by
  2026-07-06 (A7).
- **Left as the source has it:** the 2023 cytopenias in a woman with prior APL are put down
  to lupus with no hematology referral or marrow. The relapse admission names no
  coagulation studies and no therapy besides ATRA. The gynecology letter offers no
  fertility treatment, and no note returns to fertility. The ED does no brain imaging or
  lumbar puncture, and no one weighs lupus itself as a cause of the psychosis. The severe
  flare gets no steroid-sparing drug. No one evaluates the reported family history of
  hereditary hemorrhagic telangiectasia or weighs it against her bruising and gum bleeding.
  No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** The 2020-09-22 new-patient visit (the source opens with an established patient
  carrying a past APL, so she was already in care).
- **A2.** Folic acid 1 mg starts at the July 2022 physical, when she starts trying to
  conceive; that is 18 months before the gynecology visit.
- **A3.** One early-follicular FSH, 14.8 mIU/mL, drawn by gynecology with the other
  labs, to support the source's "hypergonadotropic" diagnosis.
- **A4.** The 2025-10-10 practice visit that finds the pancytopenia and sends her in.
- **A5.** The relapse course after the source names ATRA and before the ED calls her in
  remission: tretinoin 45 mg/m² per day in two divided doses; negative pregnancy test on
  admission and counselling to avoid pregnancy on ATRA and for a month after (no method
  named); discharged 2025-11-07 on ATRA with counts recovering; marrow complete remission
  by the 2025-12-10 letter; consolidation with ATRA through early 2026; on 2026-03-18
  PCR negative for PML-RARA, second complete remission, ATRA stopped. Surveillance PCR
  2026-06-17 negative.
- **A6.** Prednisone, which the ED says was "recently started", starts at a rheumatology
  flare visit 11 days before the ED, at 60 mg daily. The flare findings repeat the
  manifestations the source already gives (rash, wrist and knee synovitis, dsDNA,
  complements) and add no organ.
- **A7.** The ED course: psychiatry consult, olanzapine 5 mg nightly, overnight
  observation and discharge home on 2026-04-28; prednisone tapered by rheumatology from
  60 mg to off by 2026-07-06; olanzapine stopped 2026-06-01. No psychotic symptoms
  return.
- **A8.** SLE between source events: on hydroxychloroquine her counts recover and the disease
  is quiet from mid-2023 until the April 2026 flare. Rheumatology letters every 3 to 6
  months carry a normal CBC, normal C3 and C4, normal creatinine and a urinalysis without
  protein or blood; anti-dsDNA is not repeated until the flare. Thrombocytopenia is marked
  resolved by mid-2023 and recurs only with the 2025 relapse.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart and lung sounds, abdomen, neurological exam, liver and kidney function). An abnormal
  finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries (a marrow's cellularity in remission, a procedure's
  access site), but nothing that adds a territory, a severity grade or a symptom, or
  changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (a normal cervical co-test in 2019).
- Yearly physicals (2021, 2022, 2023, 2024, 2025), with a CBC and metabolic panel each year,
  normal except as the timeline gives; lipids once or twice; depression screens at
  physicals, negative.
- Rheumatology letters about every 3 to 6 months from 2023 (A8); a baseline
  hydroxychloroquine retinal exam letter from ophthalmology in 2023, normal; hematology
  surveillance letters after the relapse (A5).
- Practice referral and follow-up visits as the timeline shows: the rheumatology and
  gynecology referrals, the post-discharge visit in November 2025, the post-ED visits.
- Prevention: cervical co-test 2024, normal; HIV and hepatitis C screening once, negative;
  Tdap if due; influenza each fall; COVID vaccines; pneumococcal conjugate vaccine and the
  recombinant zoster series in 2026 after consolidation, as indicated for her leukemia and
  immunosuppression. No breast, colorectal or lung screening (not due).
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, school
  work notes for her absence during the admission and the ED stay, a missed visit.
- Booked visit after 2026-08-27: annual physical on 2026-09-22, with the influenza vaccine.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
