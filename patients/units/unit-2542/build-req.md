# Build requirements: unit-2542 (Malik Hughes, MRN00002542)

Source: synthetic_hospital patient 2542, in `source.md` beside this file. This file and
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

## Who he is (S)

Malik Hughes, born 1990-04-13, male, Black, not Hispanic, English-speaking, Medicaid,
lives in Holyoke MA 01040. Logistics coordinator. Never smoker; social alcohol (1-2
drinks a week); denies any injection drug use. No known drug allergies. Surgical
history: neonatal circumcision. Family history: mother HIV positive (the source of his
perinatal infection); father's medical history unknown; maternal grandmother type 2
diabetes. Sexually active with multiple partners; condom use is described as
consistent in early 2023 and inconsistent from 2025 on.

Joined the practice 2021-06-08 (A1). He brought:
- HIV infection, perinatally (vertically) acquired, followed at an outside infectious
  disease clinic, on bictegravir/emtricitabine/tenofovir alafenamide (Biktarvy)
  50/200/25 mg one tablet daily;
- genital herpes simplex virus type 2, on valacyclovir 500 mg daily suppression.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-06-08 | practice | new patient visit (A1): history as above; exam normal; PCV20 given; MenACWY dose 1; baseline labs (P) |
| 2021-2022 | outside | infectious disease follow-ups about every 6 months, letters received: viral load not detected (<20 copies/mL), CD4 in the 600s-700s (A4) |
| 2023-03-06 | outside | infectious disease visit, routine check: no complaints; multiple partners, says he uses condoms consistently but had unprotected sex with a new partner about two months earlier; no IV drug use; vitals normal; exam unremarkable. HIV-1 antibody positive, **HIV-1 RNA 55,000 copies/mL**, **CD4 600 cells/mm3**, CBC normal, hepatitis panel negative (hepatitis B surface antigen and hepatitis C antibody non-reactive). He admits missed Biktarvy doses over recent months (R1); adherence counseling, Biktarvy continued, repeat viral load in 3 months. Letter received 2023-03-09 |
| 2023-08-21 | practice | about 2 weeks of easy bruising and a petechial rash on both lower legs; still missing doses; no mucosal bleeding reported; exam: petechiae over both shins, scattered bruises, no lymphadenopathy, no splenomegaly. Same-day CBC: **platelets 31** (A2). Urgent hematology referral; avoid NSAIDs |
| 2023-08-23 | outside | hematology consult (age 33, R4): two-week history of easy bruising and lower-leg petechiae; no recent infection and no new medication (R5). No hepatosplenomegaly, no lymphadenopathy. **Platelets 25 x10^9/L, hemoglobin 14 g/dL, WBC 6.5 x10^9/L**; smear: normal morphology, isolated thrombocytopenia; PT/INR and aPTT normal; hepatitis C antibody negative; HIV-1/2 antigen/antibody reactive with HIV-1 RNA detected (R2). Impression: **isolated thrombocytopenia in the setting of non-suppressed HIV** (R3); plan: daily ART adherence, bleeding precautions, no NSAIDs, serial CBCs. No steroid or IVIG (left as source). Letter received 2023-08-25 |
| 2023-08-30 | practice lab | CBC (hematology's order, drawn at the practice): **platelets 44** (A3) |
| 2023-10-04 | practice lab | CBC: **platelets 118** (A3) |
| 2023-11-29 | outside | infectious disease follow-up: taking Biktarvy daily; **HIV-1 RNA not detected (<20)**, CD4 640 (A4). Same week hematology follow-up letter: **platelets 176**, released from hematology clinic (A3) |
| 2024-02-13 | practice | annual physical; late influenza vaccine given (the "one year ago" vaccine of 2025, P); thrombocytopenia **resolved** on the problem list; CBC normal |
| 2024-2025 | outside | infectious disease follow-ups about every 6 months: viral load not detected, CD4 in the 600s-700s (A4) |
| 2025-02-24 | practice | routine follow-up, age 34 (R6): earlier bruising episode noted; adherent; no complaints. Last influenza vaccine about a year ago (Feb 2024; none in fall 2024); cannot recall chickenpox or shingles; does not know his hepatitis A or B status. Multiple partners, condoms inconsistently. Exam unremarkable. **Tdap given** (the source's "tetanus prophylaxis", R7); influenza vaccine given; hepatitis B surface antibody, hepatitis A antibody and varicella IgG ordered (A5); safer-sex counseling |
| 2025-02-27 | practice | results (A5): hepatitis B surface antibody reactive (immune), hepatitis A total antibody non-reactive, varicella IgG positive. Hepatitis A vaccine series advised |
| 2025-03-25 | practice | nurse visit: hepatitis A vaccine dose 1 (A5) |
| 2025-09-03 | outside | infectious disease visit (moved from 2025-09-01, Labor Day, R8): painless penile ulcer for 3 weeks, unchanged in size, no discharge; mild groin node swelling; multiple partners, condoms inconsistently. Exam: one indurated, non-tender ulcer on the penile shaft; bilateral non-tender inguinal lymphadenopathy. **RPR non-reactive, TP-PA reactive. Primary syphilis diagnosed**; **benzathine penicillin G 2.4 million units IM once given** (A6); partner notification through the health department; viral load not detected on routine draw (A4). Letter received 2025-09-08 |
| 2025-10-21 | practice | nurse visit: hepatitis A vaccine dose 2 (A5), influenza and updated COVID vaccines (P) |
| 2025-12-09 | outside | infectious disease follow-up: ulcer healed; **RPR non-reactive** (A6); viral load not detected, CD4 in the 700s (A4). Primary syphilis treated (resolved) |
| 2026-02-23 | outside | ED visit: 2 days of severe left testicular pain and swelling, dysuria and a milky penile discharge; pain eases lying down with the scrotum elevated. **T 38.3 C, BP 130/80, HR 88.** Exam: swollen, tender left testis, retracted upward; pain relieved with elevation (positive Prehn sign); cremasteric reflex intact. **Urinalysis: pyuria, leukocyte esterase positive. Urine NAAT positive for Chlamydia trachomatis** (gonorrhea NAAT not detected, P). **Chlamydial epididymitis**; **ceftriaxone 500 mg IM once and doxycycline 100 mg twice daily for 10 days** (A7); discharged home. Record received 2026-02-25 |
| 2026-03-03 | practice | post-ED visit (A8): pain and swelling much improved, dysuria gone, still taking doxycycline; exam: testes non-tender, no swelling, no discharge. Repeat chlamydia test in 3 months; partners to be treated |
| 2026-05-01 | practice | **last practice visit** (annual physical): well; finished doxycycline; no symptoms. Chlamydia NAAT retest **negative** (resulted 2026-05-04, A8); epididymitis resolved; CMP, CBC, lipids normal. Follow-up booked |
| 2026-06-09 | outside | infectious disease follow-up letter, filed as received: viral load not detected; no change to lists (A4) |
| 2026-09-15 | practice | booked: routine follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** A long-standing, treated perinatal HIV with a viral load of 55,000 on Biktarvy
  needs a cause. Resolution: he had been missing doses for months before March 2023
  and kept missing them through August 2023; viral load was not detected on all
  infectious disease draws before 2023 and from November 2023 on. No resistance
  testing appears in the chart and no regimen change is made.
- **R2.** Hematology's plan "orders" HIV and hepatitis C serology whose results it
  already reports, as if HIV were new. Resolution: hematology sends them as part of its
  standard workup for isolated thrombocytopenia; the reactive HIV antigen/antibody and
  detected HIV-1 RNA confirm the known, non-suppressed infection. The source's
  "positive HIV-1 RNA PCR" carries no number, and the chart gives none.
- **R3.** The source's problem lists add "HIV-related opportunistic skin infection"
  dated to the hematology visit, but that visit finds only petechiae from a platelet
  count of 25 and no infection. Resolution: the chart records **Thrombocytopenia**
  (HIV-associated, at hematology), not a skin infection; no skin infection appears
  anywhere. "Maternal HIV infection" is the same perinatal HIV infection, not a second
  problem; his mother's HIV is family history.
- **R4.** The hematology note calls him 32; on 2023-08-23 he is 33. Ages follow the
  birth date.
- **R5.** Hematology says he "denies medications or significant history". Resolution:
  it means no new medications or recent infections; the letter lists his HIV, HSV-2,
  Biktarvy and valacyclovir.
- **R6.** Encounter 2 calls itself a "follow-up visit in October" in February.
  Resolution: a routine February follow-up; "October" is dropped.
- **R7.** "Tetanus prophylaxis (diagnosed 2022-01-15)" maps to a Tdap vaccine given at
  the 2025-02-24 practice visit. It is an immunization, not a problem.
- **R8.** The 2025-09-01 infectious disease date falls on Labor Day. Resolution: the
  visit is 2025-09-03.
- The 2023 infectious disease note calls the family history non-contributory while
  later notes list it. Resolution: that letter simply does not detail it; family
  history is as above throughout.
- "High risk sexual behavior" is carried as sexual history in notes, not as a coded
  problem.
- **Left as the source has it:** no gonorrhea/chlamydia or syphilis testing at the
  March 2023 visit despite a recent unprotected exposure; no steroid or IVIG for a
  platelet count of 25 with petechiae; no HSV testing of the penile ulcer; no scrotal
  ultrasound in the ED despite a retracted testis; hepatitis A, B and varicella immune
  status not checked until 2025. Nobody comments on any of these.

## Additions (A): physician to confirm

- **A1.** The 2021-06-08 new-patient visit (the seed gives about 5 years with the
  practice; the source's first visit is a routine check of a long-treated patient).
- **A2.** The 2023-08-21 practice visit and CBC (platelets 31) that sent him to
  hematology.
- **A3.** Platelet recovery after ART adherence resumes: 44 on 2023-08-30, 118 on
  2023-10-04, 176 on 2023-11-29, normal on every later CBC; a hematology follow-up letter
  in late November 2023 releases him. Thrombocytopenia resolved 2024-02-13.
- **A4.** Viral load not detected (<20 copies/mL) on every infectious disease draw
  except March and August 2023; CD4 runs about 600 to 760 cells/mm3 (600 exactly in
  March 2023).
- **A5.** 2025 immunity checks the source raises: hepatitis B surface antibody
  reactive, hepatitis A total antibody non-reactive, varicella IgG positive; hepatitis A
  vaccine 2 doses (2025-03-25, 2025-10-21).
- **A6.** Standard treatment of the primary syphilis: benzathine penicillin G 2.4
  million units IM once at the infectious disease visit; RPR still non-reactive at
  the December 2025 follow-up.
- **A7.** Standard ED treatment of epididymitis likely from a sexually transmitted
  infection: ceftriaxone 500 mg IM once plus doxycycline 100 mg twice daily for 10
  days.
- **A8.** Post-ED practice visit 2026-03-03 and a chlamydia retest at 3 months
  (2026-05-01), negative.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal
  results (including COVID primary vaccination in spring 2021 at a pharmacy).
- Practice visits about yearly plus the timeline's visits; annual physicals 2022-05-17,
  2024-02-13 and 2026-05-01 with CMP, CBC and lipids (normal); urinalysis normal at
  joining. Weight steady around 82 kg, height 179 cm, BP normal throughout.
- HIV care stays at the outside infectious disease clinic; its letters reach the
  practice. Infectious disease draws may include routine gonorrhea/chlamydia and RPR
  screens, negative, except as the timeline gives.
- Prevention: PCV20 (2021-06-08), MenACWY 2 doses (2021-06-08, 2021-08-10), influenza
  in fall 2021 and fall 2022, then February 2024 and February 2025, and fall 2025 (none
  in fall 2023 or fall 2024), COVID boosters fall 2021, fall 2022 and fall 2025, Tdap
  2025. PHQ-2 negative at physicals.
- Refills of Biktarvy (prescribed by infectious disease) and valacyclovir (prescribed by
  the practice), Medicaid prior-authorization and pharmacy traffic, portal messages
  about results, refills and scheduling, a missed flu-clinic visit in fall 2024.
- Booked visit after 2026-08-27: routine follow-up 2026-09-15.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002542 on 2026-05-01. Practice contacts after that day do not
happen except messages, refills and that visit's results; outside care after it still
happens, and its records are filed as received without changing the practice's lists. A
visit is booked after today.
