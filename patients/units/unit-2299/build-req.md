# Build requirements: unit-2299 (Howard Finn, MRN00002299)

Source: synthetic_hospital patient 2299, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the anesthesia paragraph in
encounter 0).

## Who he is (S)

Howard Finn, born 1955-05-16, male, White, not Hispanic or Latino, English, Leeds MA
01053, Medicare with supplemental private insurance. Retired construction foreman.
Married; his wife is involved in his care (S, encounter 4). Tobacco: 40 pack-years;
still smoking when he joins, quits in March 2022 (R1). Alcohol: occasional, 1-2 drinks
a week. Allergies: no known drug allergies. Family history: father lung cancer, mother
COPD, brother DVT.

Surgical history: port-a-cath placed for chemotherapy (before joining, A1). The open
cholecystectomy (May 2023), open radical prostatectomy (September 2023) and right hip
replacement (September 2025) happen during the timeline (R2, R3, R7).

**Joins the practice 2021-08-24** (A1, new-patient visit; five years with the practice
by today). He brings: COPD on long-term home oxygen 2 L/min continuous by nasal cannula
(chronic hypoxemic respiratory failure), followed by an outside pulmonology group;
metastatic non-small cell lung cancer, under an outside oncology group (A1); history
of DVT with post-thrombotic syndrome, on long-term warfarin; hypertension. Drugs on
joining: tiotropium 18 mcg inhaled daily, albuterol 90 mcg inhaler every 4-6 hours as
needed, warfarin 5 mg daily (INR goal 2.0-3.0), amlodipine 10 mg daily, prednisone 5 mg
daily (chronic maintenance, from pulmonology), home oxygen 2 L/min continuous.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-08-24 | practice | new patient visit (A1): history as above; current smoker; counseled about smoking with home oxygen. Warfarin management (INR checks) taken on by the practice (P) |
| 2022-02-15 | practice | ready to quit: **nicotine patch 21 mg/24 h started** (A2) |
| 2022-03 | practice | quits smoking in March 2022 (R1); patch tapered off by June 2022 |
| 2023-03-29 | practice | (encounter 0) both legs swollen; he wonders if a medication is the cause. Exam: bilateral lower-leg edema. **Calcium channel blocker-induced peripheral edema diagnosed.** **Amlodipine 10 mg reduced to 5 mg daily** (R4). Elective gallbladder surgery already booked with an outside general surgery group (R2) |
| 2023-05-23 | outside | open cholecystectomy (begun laparoscopically, completed open, R2). Warfarin held by the surgical team before and resumed after (A3) |
| 2023-05-30 | outside ED | (encounter 1) age 68 (R8); 7 days after the open cholecystectomy, sudden severe abdominal pain and a feeling of something giving way after lifting something heavy. Afebrile, BP 130/85, HR 110, RR 22. Abdomen distended, tender non-reducible bulge at the incision, no erythema or purulent drainage, bowel sounds diminished. Hb 13.2, WBC 11.0, Cr 1.0, BUN 18, electrolytes normal. CT abdomen: bowel loops protruding through a fascial defect of the abdominal wall. **Deep fascial wound dehiscence.** Taken to the operating room for **closure of the dehiscence** (A3) |
| 2023-06-13 | practice | post-operative follow-up (P); leg swelling improved on amlodipine 5 mg; **CCB edema resolved** (R4) |
| 2023-09-11 | outside | open radical prostatectomy at an outside urology group; discharged 2023-09-14 (R3). Warfarin held around surgery by the surgical team (A4) |
| 2023-09-17 | outside ED, admitted | (encounter 2) 6 days after the prostatectomy, home 3 days, limited mobility: sudden dyspnea and pleuritic chest pain. T 37.2 C (99.0 F), BP 92/64, HR 115, RR 28, SpO2 88% on room air. JVD present, lungs clear, loud P2. ABG pH 7.47, PaCO2 29, PaO2 62. CT pulmonary angiogram: multiple bilateral segmental filling defects. **Pulmonary embolism diagnosed.** Treated with heparin and **warfarin resumed** (A4) |
| 2023-09-26 | practice | post-hospital visit (P) |
| 2024-03-19 | outside ED, admitted | (encounter 3) 3 days of more dyspnea, productive cough, less able to do daily activities, sleeping more. Denies chest pain, fever, chills. RR 22, HR 110, BP 145/90, SpO2 88% on room air. Somnolent but arousable; diffuse wheezing, decreased breath sounds. ABG pH 7.28, PaCO2 82, PaO2 60, HCO3 30. **COPD exacerbation with acute hypercapnic respiratory failure without altered mental status** (R5). Noninvasive ventilation, nebulized bronchodilators, systemic steroids and an antibiotic (A5); discharged 2024-03-24 on a **prednisone 40 mg 5-day burst** then back to 5 mg, and **doxycycline 100 mg twice daily for 5 days** (A5) |
| 2024-03-28 | practice | post-discharge visit (P) |
| 2024-09-20 | outside ED, admitted | (encounter 4) age 69 (R8); six months after the prior exacerbation; 3 days of low-grade fever and fatigue, 24 hours of worsening dyspnea and unable to lie flat; wife reports more cough with yellow-green sputum. Alert, exhausted, using accessory muscles. RR 32, HR 112, T 37.8 C (100.0 F), SpO2 85% on room air. Bilateral expiratory wheezes, decreased breath sounds at the bases, no JVD, no peripheral edema. Arterial gas reported abnormal without values (R6). Chest x-ray: hyperinflated lungs, no focal infiltrate. **Placed promptly on noninvasive positive pressure ventilation.** COPD exacerbation. Discharged 2024-09-25 with a **prednisone 40 mg 5-day burst** and **azithromycin 250 mg, 5-day course** (A6). Discharge summary never reaches the practice (Records) |
| 2024-09-30 | practice | post-admission visit (P); learns details from the patient and the pharmacy fill |
| 2025-03-30 | outside ICU | (encounter 5) age 69; COPD on oxygen and prior PE; had missed warfarin for the past week; 2 hours of sudden dyspnea and pleuritic chest pain, lightheaded with a near-syncopal episode earlier that day. BP 85/50, HR 120, RR 28, SpO2 88% on room air. JVP elevated; decreased breath sounds without wheezes or crackles. Troponin I elevated, BNP elevated, D-dimer elevated (no values), INR 1.2. Bedside echo: RV dilated and hypokinetic. ECG: sinus tachycardia with right bundle branch block. **Massive pulmonary embolism.** **Systemic thrombolysis with alteplase**, then heparin, then **warfarin resumed 5 mg** (A7); discharged 2025-04-04 |
| 2025-04-08 | practice | post-ICU visit (P) |
| 2025-09 (early) | outside | **right hip replacement** at an outside orthopedic group (R7); warfarin held, **enoxaparin started postoperatively** by the surgical team (S, encounter 6) |
| 2025-09-20 | outside ICU | (encounter 6) age 70; six months after the massive PE. A week of productive cough with yellow sputum he took for a COPD flare; then sudden dyspnea, pleuritic chest pain and hematemesis. RR 28, SpO2 89% on room air, HR 110, BP 135/85. Right leg swollen and tender along the deep veins. Hb 7.8, Hct 23.4, platelets 255, INR 1.1, aPTT 30. Chest x-ray: no pneumonia or pneumothorax. Right leg ultrasound: proximal DVT. CT pulmonary angiogram: large saddle pulmonary embolus. **Saddle pulmonary embolus with active GI bleeding; acute right proximal DVT.** Anticoagulation stopped; **IVC filter placed**, red cell transfusion, upper endoscopy (A8). **Enoxaparin stopped**; **warfarin restarted** before discharge (A8). INR checks move to the hospital's anticoagulation clinic (A8) |
| 2025-10-07 | practice | post-hospital visit (P); Hb 10.4 (A8) |
| 2025-11-24 | practice | **last visit**: follow-up; Hb 11.6 (A8); no other change |
| 2026-04-02 | outside ICU | (encounter 7) age 70; metastatic lung cancer, severe COPD on home oxygen, recent massive PE and GI bleeding; collapsed at home, drowsy and confused on arrival. BP 80/50, HR 118, RR 24. JVD, muffled heart sounds, weak thready pulse, cool extremities, delayed capillary refill, inspiratory fall in systolic pressure of 15 mm Hg; lungs clear; no peripheral edema. **Cardiac tamponade with cardiogenic shock.** Echo shows a pericardial effusion with tamponade physiology; **emergent pericardiocentesis** (A9). Practice hears first from his wife by portal (2026-04-06) and receives the cardiology discharge summary 2026-05-12; lists not updated |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Every encounter from 2023 to 2026 says he quit "1 year ago". Resolution: he
  quits in March 2022, one year before encounter 0; the later records copy the phrase
  forward. He is a current smoker when he joins in August 2021 (40 pack-years by then).
- **R2.** Encounter 0 lists an open cholecystectomy "complicated by deep fascial wound
  dehiscence" before encounter 1 describes that very operation and complication; its
  plan names an elective laparoscopic colectomy for recurrent diverticulitis; the
  comorbidities say "status post laparoscopic cholecystectomy". Resolution: the elective
  operation booked at encounter 0 is the gallbladder operation, done 2023-05-23, begun
  laparoscopically and completed open. The colectomy and diverticulitis are not carried.
  The general surgery consult and operative notes never reach the practice (Records), so
  the chart does not state the operation's indication. Before 2023-05-23 no record lists
  the cholecystectomy or the dehiscence.
- **R3.** The prostatectomy is on every list from encounter 0, but encounter 2 says it
  was done 6 days earlier. Resolution: open radical prostatectomy 2023-09-11 at an
  outside urology group, arranged without the practice; the urology records never
  arrive, so its indication is not in the chart. The practice learns of it from the ED
  record and the patient.
- **R4.** Encounter 0 diagnoses calcium channel blocker edema, but every later outside
  list carries amlodipine 10 mg and encounters 4 and 7 find no peripheral edema.
  Resolution: the practice cuts amlodipine to 5 mg on 2023-03-29; the edema settles and
  the diagnosis is resolved 2023-06-13. Outside lists keep copying 10 mg, as outside
  lists do; the practice list says 5 mg from 2023-03-29. Practice BPs run about 138/84
  on 5 mg.
- **R5.** Encounter 3 is labeled "without altered mental status" yet finds him somnolent
  but arousable. Both stay as the ED wrote them; no note reconciles them.
- **R6.** Encounter 4 says only that the arterial gas showed an acid-base disturbance.
  The chart gives no values: the record states the gas was abnormal.
- **R7.** Encounter 6 lists "recent right hip replacement" as its only surgical history;
  other lists never carry it. Resolution: right hip replacement in early September 2025
  at an outside orthopedic group, arranged without the practice; warfarin held and
  enoxaparin given after surgery by that team, which explains the INR of 1.1. The
  orthopedic records never arrive; the indication is not in the chart.
- **R8.** Stated ages follow the birth date: 67 at encounter 0, 68 at encounter 1 (the
  source says 67), 69 at encounter 4 (source 68), 69 at encounter 5, 70 at encounters 6
  and 7.
- **R9.** The home-medication lists drop tiotropium, albuterol, prednisone or warfarin at
  some encounters while the prose names them. Resolution: all continue throughout;
  oxygen 2 L/min continuous throughout. Encounter 6's "salbutamol" is albuterol.
- **R10.** Comorbidity "sinus tachycardia secondary to burns" is not carried (no burns
  anywhere). "Chronic hypoxemic respiratory failure" is the COPD on home oxygen, listed
  from joining. "Cardiogenic shock" is carried with the tamponade.
- **Left as the source has it:** no drug changes for the COPD between exacerbations
  (tiotropium alone with chronic prednisone, never stepped up); amlodipine 5 mg remains
  despite the edema history; no one revisits warfarin versus another anticoagulant after
  the March 2025 missed doses. No note comments on these.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2021-08-24 (encounter 0 reads as an established patient with
  a full list; the seed gives five years with the practice). Metastatic non-small cell
  lung cancer was diagnosed November 2020 at an outside oncology group, before joining;
  port-a-cath placed then; carboplatin, pemetrexed and pembrolizumab, then pembrolizumab
  maintenance every 3 weeks, completed after two years in November 2022; surveillance CT
  every 3 months after that, read as stable disease with no new lesions on every scan
  through 2025. Oncology letters reach the practice irregularly. Prior DVT in 2019, on
  warfarin since. No metastatic site is named in the chart.
- **A2.** Nicotine patch 21 mg/24 h started 2022-02-15 for the quit attempt; tapered
  and stopped June 2022.
- **A3.** Warfarin held around the cholecystectomy and the dehiscence closure, resumed
  after; operative closure of the abdominal wall dehiscence 2023-05-30 (S implies
  surgery for a non-reducible eviscerating defect).
- **A4.** Warfarin held around the prostatectomy and resumed with heparin overlap after
  the September 2023 PE.
- **A5.** Treatment of the March 2024 exacerbation: noninvasive ventilation, nebulized
  albuterol/ipratropium, IV methylprednisolone, then prednisone 40 mg daily for 5 days
  and doxycycline 100 mg twice daily for 5 days; back to prednisone 5 mg after.
  Discharged 2024-03-24.
- **A6.** Treatment of the September 2024 exacerbation, known to the practice from the
  pharmacy fill and the patient: prednisone 40 mg daily for 5 days and azithromycin
  250 mg (500 mg day 1, then 250 mg days 2-5). Discharged 2024-09-25.
- **A7.** Massive PE with hypotension: systemic alteplase, heparin, warfarin resumed
  5 mg daily; discharged 2025-04-04 with INR 2.1; INR checks back at the practice.
- **A8.** Saddle PE with active bleeding (the Hb of 7.8 is recorded as acute blood-loss anemia, still being followed at the last visit): anticoagulation held, IVC filter, 2 units of
  red cells, upper endoscopy (report not received; the practice knows only that the
  bleeding was treated at endoscopy); warfarin restarted before discharge 2025-09-27 once
  bleeding had stopped; enoxaparin stopped; omeprazole 40 mg daily started at discharge;
  INR managed by the hospital anticoagulation clinic after discharge. Practice Hb 10.4
  on 2025-10-07 and 11.6 on 2025-11-24.
- **A9.** Echo with pericardial effusion and tamponade physiology and emergent
  pericardiocentesis 2026-04-02 (standard treatment of the source's tamponade). The
  summary received gives no cause of the effusion.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there with normal
  results (a normal colonoscopy in 2017).
- Chronic-care visits every 2 to 4 months; INR checks at nurse visits every 4 to 8 weeks
  while the practice manages warfarin (INR mostly 2.0 to 3.0, an occasional value just
  out of range with a small dose adjustment of the weekly total that returns to 5 mg
  daily); CBC and comprehensive metabolic panel every 6 months; lipids yearly; Medicare
  annual wellness visits; compression stockings for post-thrombotic syndrome; oxygen
  supplier paperwork.
- Vaccines: influenza each fall (high-dose), COVID-19 boosters, PCV20, Shingrix series,
  Tdap, RSV once. Hepatitis C and HIV screening once (negative). Colorectal screening
  not continued given his cancer (the 2017 colonoscopy is the last). PHQ-2 at wellness
  visits, negative.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  missed or cancelled visit, records requests.
- Records: gaps (seed). The general surgery, urology and orthopedic records never
  arrive; the September 2024 discharge summary never arrives; the September 2025 stay
  reaches the practice only as the admission record and the patient's account; some
  oncology letters arrive late. The practice learns of the hip surgery and the September
  2024 treatment from the patient and pharmacy fills.
- No visit booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002299 on 2025-11-24. Practice contacts after that day do
not happen; outside care after it still does (the April 2026 tamponade), and its records
are filed as received without changing the practice's lists. Nothing is booked.
