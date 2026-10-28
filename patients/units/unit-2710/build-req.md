# Build requirements: unit-2710 (Paula Delaney, MRN00002710)

Source: synthetic_hospital patient 2710, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz labels.

## Who she is (S)

Paula Delaney, born 1972-03-31, female, White, not Hispanic or Latino, English,
Belchertown MA 01007, commercial PPO. International sales manager (she travels a great
deal, long-haul flights and long drives). Never smoker; alcohol occasional, 1 to 2
glasses of wine a week. No known drug allergies. Surgical history: laparoscopic
cholecystectomy in 2015. Family history: mother had a pulmonary embolism at 68; father
has hypertension and hyperlipidemia; a sister had a deep vein thrombosis (R2).

Joined the practice on 2020-07-14 (A1), age 48, from a prior primary-care office,
bringing: essential hypertension on **hydrochlorothiazide 25 mg daily**, and a history
of a remote deep vein thrombosis, treated years before and no longer on any
anticoagulant (no further detail in her records). Records from the prior office arrive
after the first visit.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-07-14 | practice | new patient visit (A1): history as above; BP controlled on HCTZ; labs drawn (P) |
| 2020-07 to 2024-04 | practice | primary-care layer (P): hypertension visits, physicals, prevention |
| 2024-04-19 | practice | **last practice visit** (seed last_seen): routine hypertension follow-up, BP at goal on HCTZ 25 mg, BMP drawn, no change. No practice visit after this day |
| 2024-06-11 | outside ED | sudden shortness of breath and pleuritic chest pain beginning 2 hours before arrival; returned 3 days earlier from a 12-hour flight. On HCTZ; NKDA; does not smoke. BP 135/85, HR 110, RR 22, SpO2 94% on room air. Lungs clear; no leg swelling or tenderness. D-dimer within normal limits. Chest x-ray unremarkable. ECG sinus tachycardia, no ST changes. High pretest probability, so CT pulmonary angiography despite the normal D-dimer (R1, A2): **pulmonary embolism diagnosed**. **Apixaban started** (10 mg twice daily for 7 days, then 5 mg twice daily) for a 3-month course (R3); discharged from the ED with advice to follow up with primary care. Record received 2024-06-13 |
| 2024-09 | patient | finishes the 3-month apixaban course (R3); **apixaban stops** |
| 2025-02-20 to 2025-02-25 | outside hospital, Pulmonary & Critical Care (ICU) | age 52; sudden shortness of breath and sharp pleuritic chest pain beginning 2 hours before arrival, after a car trip of more than 8 hours without breaks; recurrence after the June 2024 PE. Home medication HCTZ only; NKDA; does not smoke. BP 135/85, HR 110, RR 22, SpO2 94% on room air. JVP not elevated; lungs without wheezes, crackles or rhonchi; no signs of DVT in the legs. D-dimer 0.8 mg/L (normal <0.5). Chest x-ray clear without consolidation, small right pleural effusion. ECG sinus tachycardia, no ST changes. CT pulmonary angiography (A3): extensive bilateral emboli in both main pulmonary arteries. Discharge diagnosis **massive pulmonary embolism** (R4). Heparin infusion in the ICU, then **apixaban** 10 mg twice daily for 7 days then 5 mg twice daily, planned for 6 months with pulmonary clinic follow-up (A4). Discharge summary received 2025-03-04 |
| 2025-04-10 | outside pulmonology | clinic follow-up letter (A4): breathing back to baseline, taking apixaban 5 mg twice daily, plan 6 months then reassess |
| 2025-08-21 | outside pulmonology | clinic letter (A5): 6 months completed; second event judged travel-provoked; **apixaban stopped**; travel precautions (walk every 1 to 2 hours, hydrate, compression stockings) |
| 2025-12-05 to 2025-12-09 | outside hospital, Pulmonary & Critical Care (ICU) | age 53; sudden shortness of breath and left-sided chest pain worse with deep breaths, like the June 2024 and February 2025 episodes; returned from a transatlantic flight 2 days earlier. Home medication HCTZ 25 mg daily; NKDA; family history as above; never smoker, occasional wine, international sales manager. BP 135/85, SpO2 92% on room air; anxious, tachypneic RR 24, tachycardic HR 110; decreased breath sounds at the left base with dullness to percussion. D-dimer elevated; troponin normal; BNP normal; arterial blood gas showing hypoxemia (no values). CT pulmonary angiography: filling defect in a peripheral branch of the left pulmonary artery, with a small left pleural effusion (R5). **Recurrent pulmonary embolism**; **apixaban restarted** (10 mg twice daily for 7 days, then 5 mg twice daily) **indefinitely** (A6). Discharge summary received 2025-12-16 |
| 2026-02-12 | outside pulmonology | clinic letter (A6): no dyspnea at rest, back to work, apixaban 5 mg twice daily to continue indefinitely; follow up in a year |

Records are complete: every outside letter and summary above reaches the practice and
is filed as received. Because the practice last saw her on 2024-04-19, its problem and
medication lists stay as they were that day (hypertension, history of DVT,
hydrochlorothiazide); the PE diagnoses and apixaban appear only in the outside records.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The first ED encounter has a normal D-dimer and an unremarkable chest x-ray,
  yet every later list says PE was diagnosed that day. Resolution: with heart rate 110,
  a prior DVT and no better explanation, the ED judged pretest probability high, where
  a D-dimer cannot exclude PE, and obtained a CT pulmonary angiogram (A2) that showed the
  embolism. The normal D-dimer stays in the record as the source has it. The quiz-derived
  comorbidity "Delayed management of life-threatening respiratory failure" is not carried.
- **R2.** The first two source encounters say there is no family history of clotting,
  cardiac or thromboembolic disease; the profile and the third give mother PE, sister
  DVT, father hypertension and hyperlipidemia. Resolution: the full family history is
  true and is recorded at the practice's new-patient visit. The two outside notes simply
  do not state a family history; no record carries the negative statements.
- **R3.** The source never lists an anticoagulant, though each encounter is a PE.
  Resolution: the ED prescribed apixaban for 3 months (the event read as provoked by the
  long flight); she completed it about September 2024. That is why the February 2025 home
  list carries HCTZ alone. Nobody reconsidered the duration in light of the remote DVT
  (she had stopped seeing the practice), and no note remarks on it.
- **R4.** "Massive" pulmonary embolism usually means hemodynamic instability, but her
  pressure was 135/85. Resolution: the outside team's discharge diagnosis stays
  "massive pulmonary embolism", resting on the clot burden in both main pulmonary
  arteries (A3); she stayed normotensive and received no thrombolysis. The label is left
  as that team wrote it.
- **R5.** The third encounter's left-base dullness and decreased breath sounds have no
  imaging correlate in the source. Resolution: the CT angiogram also shows a small left
  pleural effusion. No thoracentesis.
- **Source years** "January and September 2020" follow the re-dated timeline: June 2024
  and February 2025.
- **Left as the source has it:** no echocardiogram, no thrombophilia testing and no
  hematology referral are recorded despite three VTE events and a strong family history;
  no note comments on it. The practice never sees her after April 2024, so no
  post-discharge practice visit happens.

## Additions (A): physician to confirm

- **A1.** The 2020-07-14 new-patient visit (the seed gives about 4 years with the
  practice before the 2024-04-19 last visit).
- **A2.** CT pulmonary angiography at the June 2024 ED visit: filling defects in
  segmental branches of the right lower lobe pulmonary artery; no right heart strain
  described; no effusion. Needed for the source's PE diagnosis.
- **A3.** CT pulmonary angiography in February 2025: extensive bilateral emboli in both
  main pulmonary arteries extending into lobar branches; small right pleural effusion (as
  on the source's x-ray). Needed for the source's massive PE diagnosis.
- **A4.** Apixaban after the February 2025 admission, planned 6 months, and the April
  2025 pulmonary clinic letter. Standard treatment of the source's PE.
- **A5.** The August 2025 pulmonary letter stopping apixaban after 6 months, so the
  December 2025 home list (HCTZ only) is true.
- **A6.** Indefinite apixaban after the third event, and the February 2026 pulmonary
  letter. Standard ongoing care after recurrent PE.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise.
All of it falls between 2020-07-14 and 2024-04-19, except messages, refills and filed
records afterward.

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice may mention routine screening done there, with normal
  results.
- Height 165 cm; weight about 70 to 73 kg across the years (BMI about 26). Office BP on
  HCTZ about 124 to 134 / 78 to 86, at goal.
- Hypertension visits about every 6 months, yearly physicals; BMP yearly (sodium and
  potassium normal on HCTZ), CBC, lipids (normal for her age), A1c once (normal), TSH
  once (normal). Hepatitis C and HIV screening once, negative. PHQ-2 negative.
- Prevention for a woman 48 to 52: screening mammograms every 1 to 2 years, normal;
  cervical cytology with HPV co-testing once (2021), normal; screening colonoscopy at 50
  (2022), normal, repeat in 10 years; influenza vaccine each fall; COVID-19 vaccines from
  2021; Shingrix series at 50; Tdap once.
- Travel counseling for long flights given her DVT history: walk, hydrate, compression
  stockings; no drug prophylaxis.
- Refills of HCTZ, pharmacy traffic, portal messages about results and scheduling, one
  missed visit, recall letters after the last visit. After 2024-04-19 she sends portal
  messages (after the ED visit and the admissions) and receives HCTZ refills; she does
  not book a visit. Nothing is booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002710 on 2024-04-19. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. No visit is booked after today.
