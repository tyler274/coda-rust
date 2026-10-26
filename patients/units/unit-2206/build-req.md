# Build requirements: unit-2206 (Carl Clark, MRN00002206)

Source: synthetic_hospital patient 2206, in `source.md` beside this file. This file and
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
"(diagnosed 2025-09-15)" stamps, its "shown below" images or its unfinished sentences.

Records texture (seed): **gaps.** Some outside care never sends its letter or sends it
late; the practice learns of it from the patient, his wife or a pharmacy fill; the
practice medication list lags once (after the 2026 admission). Each outside contact
below says what reaches the practice.

## Who he is (S)

Carl Clark, born 1952-09-28, male, White, not Hispanic or Latino, English, Medicare,
Westfield MA 01085. Retired postal worker. Never smoker, no alcohol, no hepatotoxic
drugs. Married; his wife is the only family member who appears in the story (she
gives history at the 2025 ED visit). No known drug allergies.

- Surgical history: tonsillectomy at age 8 (about 1961); knee arthroscopy at age 55
  (about 2008). Nothing else (R8).
- Family history: father had colon cancer diagnosed at 70 and is deceased; mother had
  type 2 diabetes and hypertension.
- **Joined the practice 2017-09-12** (A1, new patient visit, age 64), from a prior
  practice. He brought type 2 diabetes (on metformin, onset about 2011 by his
  account), hypertension (onset about 2005), hyperlipidemia (no drug), and these
  drugs: **metformin 1000 mg twice daily, lisinopril 20 mg daily, amlodipine 5 mg
  daily (R1), aspirin 81 mg daily.** Height 178 cm, weight about 74 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2017-09-12 | practice | new patient visit (A1); history as above; BP elevated on two drugs; A1c 8.1 (P level). Colonoscopy recommended given his father's colon cancer; he defers (R7) |
| 2019-02-19 | practice | fatigue and reduced exercise tolerance for a few months; BP 158/94; basic labs (BMP, CBC, TSH, A1c) unremarkable apart from the A1c in its usual range; **cardiology referral** (A2) |
| 2019-03-18 | outside | cardiology consult (letter received 2019-03-25): fatigue and reduced exercise tolerance for a few months; history of poorly controlled hypertension and diabetes; on lisinopril, metformin, amlodipine and aspirin 81; BP 160/96, pulse 72 and regular; displaced apical impulse and an S4 gallop, no murmurs, no signs of heart failure; renal function and electrolytes normal; ECG sinus rhythm 72 with voltage criteria for left ventricular hypertrophy (R3). **Left ventricular hypertrophy** diagnosed. Plan: echocardiogram (A3) and tighter blood pressure control, suggesting amlodipine 10 mg (R2) |
| 2019-03-28 | outside | transthoracic echocardiogram (A3), report received 2019-04-02: concentric LVH, interventricular septum 1.3 cm, posterior wall 1.3 cm, LVEF 60%, normal wall motion, normal valves, no pericardial effusion |
| 2019-04-02 | practice | telephone: **amlodipine 5 mg stopped, amlodipine 10 mg daily started** (R2) |
| 2024-10-29 | practice | (A4) since about July: fatigue, unintentional weight loss of about 10 lb, early satiety; for a few weeks mild right upper quadrant discomfort; intermittent constipation; no nausea, vomiting or visible blood. Thin; liver edge palpable. Weight 69.5 kg. CMP and CBC: AST, ALT, alkaline phosphatase, total bilirubin and albumin in the same range as the GI values below (code scatters them around those levels); Hgb 13.2. **Urgent GI referral** |
| 2024-11-12 | outside | GI consult (letter received 2024-11-18): 4 months of progressive fatigue and unintentional weight loss; early satiety, mild RUQ discomfort, intermittent constipation; denies nausea, vomiting, visible blood; no known viral hepatitis or liver disease; **has never had cancer screening** (R7); no hepatotoxic drugs. Thin, no acute distress, anicteric; soft abdomen with a firm, non-tender liver edge 3 cm below the right costal margin. AST 51, ALT 44, ALP 189, total bilirubin 1.4, albumin 3.4, INR 1.1, AFP within normal limits. CT abdomen with contrast the same day (R4). Impression: **colorectal neoplasia** with liver lesions suspicious for metastases. Plan: colonoscopy and core biopsy of a liver lesion |
| 2024-11-21 | outside | colonoscopy with biopsy (A5), report and pathology received 2024-12-02: ulcerated mass in the ascending colon, biopsied; no other lesion to the cecum; pathology invasive adenocarcinoma, moderately differentiated. Recorded in the chart as **malignant neoplasm of colon** (onset 2024-11-12) |
| 2024-12-10 | practice | office visit to talk through the diagnosis; oncology appointment set by GI; liver biopsy scheduled (P) |
| 2024-12-19 | outside | ultrasound-guided core biopsy of a liver lesion (A5); its pathology reaches the practice only inside the oncology letter: metastatic adenocarcinoma, CK20 positive, CDX2 positive, CK7 negative, consistent with a colorectal primary |
| 2025-01-06 | outside | oncology consult (letter arrives late, 2025-01-27): age 72 (R6); three months of intermittent RUQ pain (R5) and a 10-lb unintentional weight loss; recently evaluated for colorectal neoplasia; on metformin, lisinopril and aspirin (R1); vitals normal; firm, enlarged liver without jaundice; Hgb 12.8, WBC 7,000, platelets 250,000, AST 68, ALT 55, ALP 210, total bilirubin 1.2; liver biopsy reviewed. **Secondary malignant neoplasm of liver** diagnosed. CEA 46.2 ng/mL (A6). Plan: staging CT of chest, abdomen and pelvis with contrast (R4); surgical referral for right hemicolectomy, then systemic chemotherapy (A7) |
| 2025-01-14 | outside | staging CT (A7), known to the practice only through later oncology and hospital letters: chest without nodules or other metastatic disease; liver lesions and the ascending colon mass unchanged from November; no ascites or peritoneal nodularity |
| 2025-02 | outside | surgical consultation; no letter ever reaches the practice; the patient tells the practice by phone that a right hemicolectomy is planned for March (P) |
| 2025-03-01 | outside | ED (record received with the discharge summary on 2025-03-20): age 72; 2 days of severe abdominal pain, fever and confusion; his wife reports no bowel movement for 3 days and that he has refused food because of the pain; T 38.9 °C, BP 90/60, pulse 110, RR 22; abdomen diffusely tender without rebound or guarding; leukocytosis with a left shift (no values reach the practice); blood cultures drawn, broad-spectrum antibiotics started (piperacillin-tazobactam and vancomycin, A8); CT abdomen and pelvis with no diverticulitis or bowel obstruction. Admitted. Two days later (2025-03-03) blood cultures grow gram-positive rods, identified as Clostridium septicum (A8); recorded as **sepsis** (R9), resolved by discharge |
| 2025-03-05 | outside | (same admission) **open right hemicolectomy** (R10). Intraoperative atrial fibrillation with rapid ventricular response, treated with metoprolol; stable afterward and moved to the medical floor; the surgical team mentions the episode only briefly at handoff. Surgical pathology (A9): moderately differentiated adenocarcinoma of the ascending colon through the muscularis propria into pericolic fat (pT3), 2 of 18 lymph nodes positive (pN1b), margins negative |
| 2025-03-06 | outside | (same admission, POD 1; R10) overnight asymptomatic; vitals BP 130/85, HR 78, RR 16, T 36.8 °C, SpO2 97% room air (R11); in the morning palpitations and mild shortness of breath; irregularly irregular pulse, BP 120/80; ECG atrial fibrillation, ventricular rate 110. **Postoperative atrial fibrillation with rapid ventricular response.** Inpatient drugs: enoxaparin VTE prophylaxis, insulin sliding scale, metformin and lisinopril continued, aspirin 81. Oral metoprolol tartrate for rate control; back in sinus rhythm 2025-03-07 and stays there; metoprolol not continued at discharge (A10) |
| 2025-03-09 | outside | (same admission, POD 4) abdominal distension, no flatus or bowel movement since surgery; T 37.2 °C, BP 135/85, HR 88; abdomen distended and tympanic, not tender, sparse bowel sounds; Hgb 12.5, WBC 9,000, Na 140, K 4.2, Cl 102, HCO3 24, BUN 18, Cr 1.0; abdominal x-ray with diffuse bowel gas and no free air under the diaphragm. **Postoperative ileus**: bowel rest, nasogastric decompression, IV fluids, walking (A11); flatus on POD 6, bowel movement POD 7, diet advanced |
| 2025-03-14 | outside | discharge (summary, operative note and pathology received 2025-03-20). Discharge drugs: home drugs unchanged, plus **amoxicillin-clavulanate 875/125 mg twice daily through 2025-03-21** to complete the bacteremia course (A8). Atrial fibrillation and ileus resolved |
| 2025-03-18 | practice | transitional care visit (P): recovering, eating, incision healing |
| 2025-04-05 | outside | ED (the practice learns of it on 2025-04-07 from his phone call and the pharmacy fill; the ED record arrives 2025-04-28 after a records request): 2 days of progressive right-sided facial swelling and pain; since the surgery chewing has been painful and his intake poor (R12); T 38.6 °C, BP 135/85, pulse 98, RR 16; right parotid tender, red and swollen with pus from Stensen's duct on massage; WBC 14,000, neutrophils 80%, glucose 180, Na 140, K 4.2, Cr 1.1. **Acute bacterial parotitis.** IV ampicillin-sulbactam and IV fluids in the ED, home on **amoxicillin-clavulanate 875/125 mg twice daily for 10 days**, warm compresses, sour candies, fluids (A12) |
| 2025-04-08 | practice | office follow-up (P): swelling settling, afebrile, drinking well |
| 2025-04-29 | outside | oncology visit (letter received 2025-05-06): plan FOLFOX every 2 weeks for 12 cycles through a port, ondansetron as needed (A13) |
| 2025-05-06 | outside | implanted port placement (A13; the practice hears of it from the patient) |
| 2025-05-13 | outside | **FOLFOX** cycle 1 (A13); cycles every 2 weeks; cycle 12 on 2025-10-14; tolerated without dose changes |
| 2025-08-12 | outside | restaging CT (A13): partial response, liver lesions smaller, largest 2.9 cm (from 4.6 cm), no new disease; CEA 11.4. No letter reaches the practice; the patient reports "the spots are shrinking" (P) |
| 2025-10-28 | outside | restaging CT (A13), letter received 2025-11-10: liver lesions smaller again, largest 2.3 cm, no new disease; CEA 5.9; **oxaliplatin stopped; maintenance fluorouracil and leucovorin every 2 weeks from 2025-11-11** |
| 2026-03-07 | outside | ED and admission (the practice learns of it from his wife's call on 2026-03-09; the discharge summary arrives late, 2026-03-24): severe substernal chest pain for 2 hours, worsening shortness of breath; T 37.5 °C, pulse 104, RR 28, BP 100/78, SpO2 88% on room air; JVP elevated, bilateral crackles throughout, S3 and S4 gallop; chest x-ray with bilateral perihilar infiltrates and a mildly enlarged cardiac silhouette; ECG (R3) sinus tachycardia 104, LVH voltage, horizontal ST depression V3–V6 (A14). High-sensitivity troponin I 1,850 ng/L, peak 16,400 ng/L; NT-proBNP 4,820 pg/mL (A14). **Acute myocardial infarction (non-ST elevation)** with pulmonary edema: noninvasive ventilation, IV furosemide, aspirin load, heparin (A14) |
| 2026-03-09 | outside | (same admission) coronary angiography (A15): proximal LAD 90% stenosis treated with one drug-eluting stent; circumflex 30%, right coronary 40%. **Coronary artery disease; coronary stent.** Transthoracic echocardiogram: concentric LVH, LVEF 50%, anterior wall hypokinesis, normal valves, no pericardial effusion (A15) |
| 2026-03-12 | outside | discharge (A16): **clopidogrel 75 mg daily (12 months), atorvastatin 80 mg daily, metoprolol succinate 25 mg daily, nitroglycerin 0.4 mg sublingual as needed started; furosemide 20 mg daily for 5 days; amlodipine stopped;** aspirin 81, lisinopril 20 and metformin continued (metformin resumed 48 h after contrast); cardiac rehabilitation referral. Oncology holds maintenance fluorouracil (A17) |
| 2026-03-17 | practice | transitional care visit (P), before the summary arrives: medications reconciled from his printed discharge list; the practice's own list still showed amlodipine until this visit (the one list lag) |
| 2026-04-14 | outside | cardiology follow-up; no letter reaches the practice; he reports it went well and rehab started (P) |
| 2026-05-12 | outside | oncology surveillance CT (A17), letter received 2026-05-26: liver lesions stable, largest 2.3 cm, no new disease; CEA 6.4; **maintenance fluorouracil and leucovorin stopped**; treatment break with CT every 3 months |
| 2026-05 to 2026-08 | practice | follow-up visits; fasting lipids on atorvastatin 80: LDL 62 (A16) |
| 2026-09-22 | practice | booked: diabetes and coronary follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Amlodipine appears only on the 2019 cardiology list; every later list (GI,
  oncology, both ED visits, surgery) shows metformin, lisinopril and aspirin.
  Resolution: he arrives on amlodipine 5 mg (2017) and takes it until the 2026 MI
  discharge. The outside letters from 2024 to 2026 leave it off their copied lists, as
  outside lists do; the practice list keeps it.
- **R2.** "Poorly controlled hypertension" (BP 160/96 on lisinopril and amlodipine)
  against normal pressures from 2025 on. Resolution: BP runs about 146–152/88–94 in
  2017–2018 (the practice asks for home readings and makes no change, an ordinary
  decision), 158/94 in February 2019, 160/96 at cardiology; the practice raises
  amlodipine to 10 mg on 2019-04-02 after the letter, and BP then runs about
  130–136/78–84. With the 2024–2025 weight loss it runs about 122–128/74–80; the
  fatigue of early 2019 fades by mid-2019.
- **R3.** "ECG findings are shown below" (2019 and 2026). Resolution: the 2019 ECG
  reads sinus rhythm 72 with LVH voltage criteria; the 2026 ECG as in A14.
- **R4.** The GI note says a CT "is performed," with no result; oncology later "orders"
  a CT of the abdomen. Resolution: the GI CT abdomen with contrast (2024-11-12) shows
  multiple hypoattenuating lesions in both lobes of the liver, the largest 4.6 cm in
  the right lobe, and irregular mass-like wall thickening of the ascending colon; no
  biliary dilation, ascites or bowel obstruction; gallbladder, pancreas, spleen and
  kidneys unremarkable. The oncology order becomes the staging CT of chest, abdomen
  and pelvis (A7). The source's "representative gross pathology specimen" becomes the
  liver core biopsy report (A5).
- **R5.** GI: weight loss over 4 months (November); oncology: RUQ pain for 3 months and
  a 10-lb loss (January). Resolution: weight loss began about July 2024 and reached
  10 lb by November, then held; RUQ discomfort began about October 2024. Weight about
  74 kg before, 69.5 kg in November 2024 and January 2025, about 67 kg after the
  surgery, 68–70 kg during and after chemotherapy.
- **R6.** Oncology calls him 71; he is 72 on 2025-01-06. All ages follow the birth date.
  The source's "2020 cardiology evaluation" is the 2019 consult.
- **R7.** "He has never undergone cancer screening" after seven years with the
  practice and a father with colon cancer. Resolution: the practice recommends a
  colonoscopy at the 2017 new-patient visit and at most yearly visits; he defers
  each time, and a FIT kit mailed in 2021 is never returned. No PSA, no other cancer
  screening. No note explains or dwells on the deferrals.
- **R8.** The comorbidity "Status post laparoscopic cholecystectomy" appears in no
  surgical history and is dropped; the gallbladder is present on imaging.
- **R9.** Fever, hypotension, tachycardia, confusion and bacteremia at the 2025 ED visit
  are named sepsis (due to Clostridium septicum, an anaerobe) in the hospital
  record; it resolves by discharge.
- **R10.** The source dates the AF encounter and the ileus encounter the same day, calls
  the ileus day "postoperative day 4," and puts the AF "the next morning" after
  surgery. Resolution: the hemicolectomy is on 2025-03-05 during the sepsis
  admission (planned surgery brought forward); AF on POD 1 (2025-03-06); ileus on
  POD 4 (2025-03-09). The surgery note calls it open; the procedure is open.
- **R11.** AF note: heart rate 78 in the vital signs, then AF at 110. Resolution: the 78
  is the overnight set before symptoms; the morning exam has the irregular pulse,
  BP 120/80 and the ECG rate of 110.
- **R12.** The parotitis ED note says the hemicolectomy was "5 days ago"; it was a month
  earlier. Not carried: the ED record says "last month."
- **R13.** The 2026 ED says hypertension is "treated with a diuretic"; no diuretic is
  ever listed. Dropped: the ED record names lisinopril.
- **Left as the source has it:** no statin for hyperlipidemia (LDL about 140) in a
  diabetic until the MI; no anticoagulation decision or rhythm follow-up after the
  postoperative AF; metformin continued through the sepsis admission; metformin
  remains the only diabetes drug, with no second agent after the MI. No note
  comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2017-09-12 new-patient visit: nine years with the practice; the source opens
  with an established patient referred to cardiology.
- **A2.** 2019-02-19 practice visit that sends him to cardiology.
- **A3.** 2019 echocardiogram (findings above) to confirm the LVH any cardiologist
  would image.
- **A4.** 2024-10-29 practice visit that sends him to GI, with the labs a PCP would
  draw (Hgb 13.2; liver tests in the GI range).
- **A5.** Colonoscopy with biopsy (2024-11-21) and liver core biopsy (2024-12-19),
  findings above, the steps the GI plan names.
- **A6.** CEA 46.2 ng/mL at the oncology consult, then 11.4 (August 2025), 5.9 (October
  2025), 6.4 (May 2026).
- **A7.** Staging CT (2025-01-14) and the oncology plan: resect the primary, then
  chemotherapy; surgical consult February 2025.
- **A8.** Antibiotics for the bacteremia (piperacillin-tazobactam and vancomycin in the
  ED, then piperacillin-tazobactam alone once the rods are named; oral
  amoxicillin-clavulanate after discharge through 2025-03-21) and the organism
  (Clostridium septicum); repeat blood cultures on 2025-03-04 negative.
- **A9.** Surgical pathology of the hemicolectomy (above). With the liver metastases it
  is stage IV.
- **A10.** Rate control and the return to sinus rhythm after the postoperative AF.
- **A11.** Ileus management and recovery (above).
- **A12.** Parotitis treatment (above).
- **A13.** Systemic therapy for metastatic colon cancer, run by oncology: port, FOLFOX
  (oxaliplatin, leucovorin, fluorouracil) every 2 weeks for 12 cycles from
  2025-05-13 to 2025-10-14, ondansetron 8 mg as needed for nausea, restaging CTs
  (findings above), then maintenance fluorouracil and leucovorin from 2025-11-11.
- **A14.** The 2026 ECG, troponin, NT-proBNP and the MI's naming as non-ST elevation, with
  ED treatment.
- **A15.** Angiography with one drug-eluting stent to the proximal LAD and the inpatient
  echocardiogram (findings above).
- **A16.** Post-MI drugs and cardiac rehabilitation (attends April to June 2026); LDL 62
  on atorvastatin 80 in May 2026.
- **A17.** Oncology holds maintenance chemotherapy after the MI and moves to a treatment
  break with surveillance after the stable May 2026 CT. The next CT is due in August
  2026; its result is not in the chart.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine care there (no colonoscopy ever).
- Diabetes and blood pressure visits about every 3 to 6 months; yearly Medicare
  wellness visits from 2018; foot exams normal; yearly urine albumin/creatinine under
  30 mg/g; creatinine about 0.9–1.0 with normal electrolytes.
- Levels: A1c 8.1 at joining, about 7.4–7.8 from 2018 to mid-2024, about 7.0 in late
  2024 and 6.6–6.9 through 2025 and 2026 with the weight loss. Untreated lipids: LDL
  about 140, total cholesterol about 215, HDL about 44, triglycerides about 170.
  On atorvastatin 80 (2026): total cholesterol about 125, triglycerides about 115,
  HDL about 44. Fasting glucose on practice draws about 145–155 until 2024 and
  about 110–125 from late 2024. Hemoglobin about 14.4 until 2024; Hgb about 12.4
  after the surgery and through chemotherapy, about 13.0 in 2026. Liver tests at practice draws in 2025–2026: AST and ALT normal, alkaline
  phosphatase about 130 by late 2025. BP per R2; after the MI about 116–124/70–76 on
  metoprolol, pulse about 64.
- Yearly diabetic eye exam letters from an optometrist: no retinopathy.
- Vaccines: high-dose influenza each fall 2017–2025; PCV13 2017, PPSV23 2018;
  Shingrix two doses 2018–2019; Tdap 2020; COVID-19 primary series 2021 and boosters
  each fall 2021–2025; RSV once in 2023. Hepatitis C and HIV screening once (2017),
  nonreactive. PHQ-2 at wellness visits, negative.
- Colonoscopy recommendations deferred per R7 until the 2024 workup.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, records requests.
- Booked visit after 2026-08-27: 2026-09-22, diabetes and coronary follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
