# Build requirements: unit-2110 (Christopher Hebert, MRN00002110)

Source: synthetic_hospital patient 2110, in `source.md` beside this file. This file and
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

## Who he is (S)

Christopher Hebert, born 1976-07-20, male, White, not Hispanic, English, Medicaid.
Unemployed; former warehouse stock clerk. Current smoker, half a pack a day for about
15 years (as of 2022). Alcohol 1 to 2 drinks a month. No known drug allergies.
Surgical history: dental extraction at age 20; incision and drainage of a right thigh
abscess at age 40. Family history: father type 2 diabetes diagnosed at 50; mother
essential hypertension; brother ankylosing spondylitis; paternal grandfather abdominal
aortic aneurysm.

Joined the practice on **2022-03-22** (new-patient visit, A1). He brought:

- Type 2 diabetes, poorly controlled, on **metformin 1000 mg twice daily** and
  **insulin glargine 30 units at bedtime**; **lisinopril 10 mg daily**.
- Chronic low back pain, on **naproxen 250 mg twice daily as needed**.
- History of recurrent anterior uveitis (no active flare at any point in the chart) and
  recurrent oral aphthous ulcers.
- History of intravenous drug use disorder, in remission (remote; no use during the
  chart, R2).
- Syphilis diagnosed about 20 years earlier (about 2002) and never treated.

Records texture (seed): **gaps**. Some outside letters never arrive or arrive late, the
practice learns of some care from the patient, a home-infusion pharmacy or a fill, and
the practice list lags at times. Each outside contact below says what reaches the chart.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-03-22 | practice | new-patient visit (A1): history as above; a few months of exertional breathlessness and fatigue; diastolic murmur at the left sternal border and a wide pulse pressure (BP about 144/56, HR about 104). Cardiology referral (A1). Labs: A1c, CMP, CBC, lipids, urine albumin/creatinine, HIV, hepatitis C, hepatitis B surface antigen and antibody, **RPR and treponemal antibody (A2)** |
| 2022-03-24 | practice | results: **RPR reactive 1:8, treponemal antibody reactive (A2)**; A1c 9.4 (P level); HIV, hepatitis C, hepatitis B surface antigen nonreactive; hepatitis B surface antibody negative. Infectious disease referral placed (A3) |
| 2022-05-09 | outside | cardiology consult (letter received 2022-05-31): six months of exertional dyspnea and fatigue, occasional palpitations and a racing heart at night; BP 145/55, pulse 110, collapsing; high-pitched blowing early diastolic murmur at the left sternal border, third intercostal space, best sitting forward at end-expiration. Chest X-ray: dilated ascending aorta. Echo: aortic root dilatation, aortic regurgitation, **mitral valve prolapse (R3)**. **Syphilitic aortitis and aortic regurgitation diagnosed; mitral valve prolapse recorded.** **Carvedilol 6.25 mg twice daily and aspirin 81 mg daily started by cardiology (R4)** |
| 2022-06-06 | outside | infectious disease consult (letter received 2022-06-10) (A3): late syphilis with cardiovascular involvement; neurologic exam normal; lumbar puncture with normal CSF cell count and protein and nonreactive CSF VDRL; **benzathine penicillin G 2.4 million units IM weekly for 3 doses (06-06, 06-13, 06-20)**; repeat RPR at 6, 12 and 24 months |
| 2022-10-24 | practice | back-pain follow-up: low back pain better with exercise, not with rest; recent orthopnea. Exam: decreased lumbar range of motion, tenderness at the Achilles insertion, diastolic decrescendo murmur at the left sternal border. ECG in office: normal sinus rhythm, no conduction abnormality. Lumbar spine X-ray (R5): bilateral sacroiliitis. Labs: HLA-B27, ESR, CRP, CBC. **Furosemide 40 mg daily started for orthopnea (R4)** |
| 2022-10-26 | practice | results: HLA-B27 positive; ESR elevated (38 mm/hr) and CRP elevated (16 mg/L) (R6); CBC normal. **Ankylosing spondylitis diagnosed**; chronic low back pain closed into it. Rheumatology referral (A4) |
| 2022-12 | outside | rheumatology visit (A4): **no letter ever arrives**; at the 2023-01-24 visit he reports rheumatology agreed with the diagnosis and told him to keep the naproxen and do a daily stretching program |
| 2022-12-12 | practice lab | RPR 1:4 (A3) |
| 2023-06-13 | practice | annual physical; RPR 1:2 (A3) |
| 2023-11-07 | outside | cardiology follow-up (letter arrives late, 2024-01-09, after a records request): six months of exertional dyspnea and fatigue and a pounding heartbeat at night, worst lying on his left side; BP 155/45, pulse 110, collapsing; high-pitched blowing diastolic murmur along the left sternal border. Chest X-ray: cardiomegaly, prominent ascending aorta. Echo: dilated left ventricle, normal systolic function, **severe aortic regurgitation**. Plan in the letter: same medicines, repeat echo in a year (R7) |
| 2024-06-18 | practice | annual physical; RPR 1:2 (A3) |
| 2024-10-22 to 2024-10-31 | outside | admission (discharge summary received 2024-11-01): non-healing ulcer on the sole of the right foot with two days of fever and spreading redness and swelling; 4-cm ulcer with purulent drainage and surrounding cellulitis. Blood cultures: Staphylococcus aureus, mecA not detected (MSSA), beta-lactamase producing: resistant to penicillin and ampicillin; susceptible to oxacillin, cefazolin, piperacillin-tazobactam, vancomycin and linezolid (R8). Foot X-ray: no osteomyelitis (P). **Transesophageal echo: no vegetation (A5).** Vancomycin and piperacillin-tazobactam in hospital, then **cefazolin 2 g IV every 8 hours by PICC to complete 4 weeks from the first negative culture (A5)**. **Diagnosed: diabetic foot ulcer of the right sole, cellulitis of the right foot, MSSA bacteremia** |
| 2024-11-06 | practice | post-discharge visit (P); ulcer smaller, wound care continues |
| 2024-11-20 | outside | cefazolin finished, PICC out (home-infusion pharmacy note, P) |
| 2025-01-14 | practice | ulcer healed: **foot ulcer, cellulitis and bacteremia resolved** |
| 2025-02 | dental office | routine dental cleaning, no antibiotics beforehand (S; known only from the April 2025 hospital record and his account) |
| 2025-04-21 to 2025-05-01 | outside | admission (discharge summary arrives late, 2025-06-03; the practice first hears from the home-infusion pharmacy on 2025-05-02): six weeks of fatigue and intermittent low-grade fevers; dental cleaning two months earlier without antibiotics. T 37.8 C, HR 90, BP 130/85. New apical regurgitant murmur; splinter hemorrhages. WBC 11.0, Hgb 12.5, ESR 45; blood cultures pending at admission, **later growing MSSA with the same susceptibilities as October 2024 (R9)**. Echo: vegetations on the mitral valve. **Infective endocarditis of the mitral valve diagnosed.** **Cefazolin 2 g IV every 8 hours for 6 weeks by PICC (A6)**; **amoxicillin 2 g before dental procedures (A7)** |
| 2025-05-07 | practice | post-discharge visit (P) |
| 2025-06-03 | outside | cefazolin course finished, PICC out (P) |
| 2025-06-23 to 2025-06-27 | outside | ED visit (ED record received 2025-06-30; **no discharge summary ever arrives**): two weeks of fever and fatigue with a new regurgitant murmur; no recent dental work. T 38.5 C, BP 110/70, HR 105, RR 18. Janeway lesions on palms and soles, Osler nodes on fingertips; murmur of mitral regurgitation. WBC 12.0, Hgb 13.2, platelets 250, ESR 45, CRP 20 mg/L. Echo: vegetations on the mitral valve. Suspected infective endocarditis; admitted. From his account (R10): blood cultures did not grow anything, and he went home on **2 more weeks of IV cefazolin**, finished about 2025-07-07 |
| 2025-07-20 to 2025-08-01 | outside | ED visit and admission (ED record and discharge summary received 2025-08-05): two weeks of fever and malaise and a new murmur since finishing the last course; no chest pain, palpitations or syncope. T 38.5 C, BP 110/70, HR 98, RR 20. Diastolic murmur at the right second intercostal space. WBC 14.0, Hgb 12.5, platelets 250, ESR 85; blood cultures Staphylococcus aureus (MSSA). ED echo: vegetation on the aortic valve with mild aortic regurgitation (R11). ECG: **new first-degree AV block**. In hospital (A8): transesophageal echo showed the aortic valve vegetation, severe aortic regurgitation and no perivalvular abscess; mitral leaflets without residual vegetation. **Infective endocarditis of the aortic valve with first-degree AV block.** **Cefazolin 2 g IV every 8 hours for 6 weeks by PICC**; cardiac surgery consulted, valve surgery to follow the antibiotic course |
| 2025-08-06 | practice | post-discharge visit (P) |
| 2025-09-02 | outside | cefazolin finished, PICC out (P) |
| 2025-09-17 | outside | cardiac surgery clinic letter (A8): planned aortic root and valve replacement, tentatively in December 2025 |
| 2025-10-27 to 2025-11-03 | outside | ED visit and admission (ED record and discharge summary received 2025-11-04): two weeks of worsening breathlessness and fatigue with a **4-kg weight gain**; no chest pain, fever or cough. BP 145/55, HR 110, RR 22, SpO2 96% on room air. Water-hammer pulse; blowing holosystolic murmur at the left lower sternal border; bilateral lower-leg pitting edema; crackles at both bases. Hgb 13.5, WBC 7.0, platelets 250, creatinine 1.0, BNP 550 pg/mL. Chest X-ray: cardiomegaly, cephalization, Kerley B lines. Echo: dilated right ventricle, normal left ventricular ejection fraction, a large tricuspid regurgitant jet (R12). **Heart failure with preserved ejection fraction and tricuspid regurgitation diagnosed.** IV diuresis, home furosemide 40 mg resumed; surgery moved up |
| 2025-11-18 to 2025-11-26 | outside | cardiac surgery admission (A9; summary and pathology received 2025-12-01): **aortic root and ascending aorta replacement with a 25 mm bioprosthetic valved conduit, and tricuspid annuloplasty**. Pathology: aortic wall with chronic aortitis and medial scarring, consistent with the known syphilitic aortitis; valve leaflets with healed endocarditis, cultures no growth. Post-operative echo: well-seated bioprosthesis without paravalvular leak, trace tricuspid regurgitation, normal LV function. First-degree AV block persists. Cardiac rehabilitation referral |
| 2025-12-03 | practice | post-operative visit: **aortic regurgitation, tricuspid regurgitation, infective endocarditis and MSSA infection resolved** (presence of prosthetic heart valve is recorded at the surgery) |
| 2026-01-21 | outside | cardiology post-operative follow-up (A9): well; echo LVEF 55 to 60%, normally functioning bioprosthesis, trace tricuspid regurgitation; same medicines |
| 2026-03 | outside | cardiac rehabilitation completed (patient report, P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists every condition from the start. Resolution: at joining he
  has diabetes, chronic low back pain, uveitis and aphthous ulcer history, remote
  intravenous drug use and untreated syphilis. Syphilitic aortitis, aortic
  regurgitation and mitral valve prolapse start at the May 2022 cardiology echo;
  ankylosing spondylitis at the October 2022 results; MSSA, foot ulcer, endocarditis,
  AV block, heart failure and tricuspid regurgitation at their encounters.
- **R2.** The social history reads "history of" IV drug use in most encounters and
  "intravenous drug use" in the last ED note. Resolution: remote, in remission
  throughout; no relapse, no drug screen, no addiction treatment in the chart.
- **R3.** Mitral valve prolapse appears only in the profile. Resolution: it is first
  documented on the May 2022 cardiology echo.
- **R4.** The source lists carvedilol, aspirin and furosemide from the first encounter
  with no reason before the cardiac diagnoses. Resolution: cardiology starts carvedilol
  6.25 mg twice daily and aspirin 81 mg in May 2022; the practice starts furosemide
  40 mg daily for orthopnea on 2022-10-24. All doses as the source gives them,
  unchanged to today.
- **R5.** The October 2022 "chest X-ray" showing sacroiliitis cannot show the
  sacroiliac joints. Resolution: a lumbar spine series whose AP view includes both
  sacroiliac joints shows bilateral sacroiliitis.
- **R6.** ESR and CRP are "elevated" without numbers. Resolution: ESR 38 mm/hr,
  CRP 16 mg/L.
- **R7.** The November 2023 cardiology note gives findings and no plan. Resolution: the
  letter continues the same medicines and plans a repeat echo in a year.
- **R8.** The October 2024 isolate is called MSSA (no mecA) yet resistant to cephalexin
  and ceftriaxone and susceptible to piperacillin-tazobactam. A mecA-negative isolate
  resistant to cephalosporins is not coherent with the source's own diagnosis
  ("penicillin-resistant via beta-lactamase production, MSSA"). Resolution: resistant to
  penicillin and ampicillin through beta-lactamase, susceptible to oxacillin,
  cefazolin, piperacillin-tazobactam, vancomycin and linezolid. The cephalexin and
  ceftriaxone resistance is not carried.
- **R9.** The April 2025 blood cultures are "pending". Resolution: they grow MSSA with
  the October 2024 pattern (the source's endocarditis organism).
- **R10.** The June 2025 ED record ends at "suspected infective endocarditis", two
  months after the April episode, and the July record speaks of "recent treatment".
  Resolution: he is admitted, cultures show no growth, and he completes two more weeks
  of IV cefazolin (about 2025-06-24 to 2025-07-07). No summary reaches the practice;
  this comes from his account.
- **R11.** The July 2025 ED echo calls the aortic regurgitation mild, two years after
  severe aortic regurgitation with a dilated ventricle. Resolution: the ED record keeps
  its bedside read as written; the in-hospital transesophageal echo reports severe
  regurgitation. The difference stays without comment.
- **R12.** The October 2025 echo reads "a large right-to-left shunt via a tricuspid
  regurgitant jet", which is not a shunt. Resolution: a large tricuspid regurgitant jet
  with a dilated right ventricle; no shunt is carried. The ED's diagnosis is heart
  failure with preserved ejection fraction, with tricuspid regurgitation.
- **R13.** The source lists "Syphilitic aortitis" twice and "Aortic regurgitation"
  twice. One problem each.
- **Left as the source has it:** diabetes stays poorly controlled (A1c 8.6 to 9.8) on
  the same metformin and glargine doses throughout; no one changes them. No statin in
  a diabetic. Naproxen continues with lisinopril and furosemide, and after the heart
  failure admission. Carvedilol continues in severe aortic regurgitation. No surgical
  referral after the November 2023 echo. None of this is commented on.

## Additions (A): physician to confirm

- **A1.** The 2022-03-22 new-patient visit and its cardiology referral (the first
  source encounter is a cardiology consult and the second a practice "follow-up").
- **A2.** RPR reactive 1:8 and treponemal antibody reactive in March 2022, to support
  the source's syphilis and aortitis.
- **A3.** Infectious disease consult and standard treatment for late syphilis with
  cardiovascular involvement (source asserts it, never treats it): CSF normal, CSF
  VDRL nonreactive; benzathine penicillin G 2.4 million units IM weekly x3; RPR 1:4
  (2022-12), 1:2 (2023-06), 1:2 (2024-06).
- **A4.** Rheumatology referral for the new ankylosing spondylitis; no letter; naproxen
  continued and stretching program (by his account).
- **A5.** For MSSA bacteremia on known valve disease: transesophageal echo with no
  vegetation, and 4 weeks of IV cefazolin.
- **A6.** For MSSA mitral endocarditis: 6 weeks of IV cefazolin.
- **A7.** Amoxicillin 2 g (four 500 mg capsules) an hour before dental procedures,
  from the April 2025 discharge, given the source's dental-cleaning history.
- **A8.** July 2025: transesophageal echo (aortic vegetation, severe aortic
  regurgitation, no abscess, mitral leaflets clear), 6 weeks of IV cefazolin, cardiac
  surgery consult and the September 2025 surgery clinic letter.
- **A9.** The course after the source ends: aortic root and valve replacement with a
  25 mm bioprosthetic valved conduit and tricuspid annuloplasty on 2025-11-18;
  pathology and post-op echo as in the timeline; cardiology follow-up 2026-01-21 (LVEF
  55 to 60%); cardiac rehabilitation. Aspirin 81 mg continues alone (no warfarin).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal
  results.
- Diabetes visits every 3 to 5 months with vitals, A1c (8.6 to 9.8), basic or
  comprehensive metabolic panel, lipids yearly (LDL about 112, untreated),
  urine albumin/creatinine yearly (normal), foot exams. Creatinine about 0.9.
  Height 178 cm, weight about 84 kg (88 at the October 2025 ED visit, about 81 after
  surgery). BP otherwise as the cardiology readings show (wide pulse pressure until
  surgery, about 128/74 after).
- Yearly diabetic eye exam letters: no retinopathy, quiet anterior chambers.
- Prevention: FIT yearly 2022 to 2024, negative (2025 kit not returned); influenza each
  fall; COVID updated vaccines in 2023 and 2024; PCV20 once; Tdap once; hepatitis B
  vaccine (Heplisav-B, 2 doses, 2022) after negative serologies; HIV and hepatitis C
  screening once (March 2022, nonreactive); PHQ-2/PHQ-9 at physicals (negative);
  smoking counseling at visits (he is not ready to quit). Not eligible for lung cancer
  screening (about 7.5 pack-years).
- Refills, Medicaid prior authorization for glargine pens, home-infusion pharmacy
  calls, portal messages about results and scheduling, a missed visit, records
  requests.
- Booked visit after 2026-08-27: a diabetes follow-up on 2026-09-15.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002110 on 2026-06-16. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
