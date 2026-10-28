# Build requirements: unit-2207 (William Howland, MRN00002207)

Source: synthetic_hospital patient 2207, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, its CHA2DS2-VASc line or its quiz text about the verbal order.

## Who he is (S)

William Howland, born 1952-05-02, male, White, not Hispanic or Latino, English, Medicare,
Northampton 01060. Retired landscape architect. Former smoker, 30 pack-years, quit in
2012 (R1). Alcohol rare, under one drink a month. No known drug allergies.

Surgical history: mechanical aortic valve replacement for bicuspid aortic stenosis in
early 2018 (R2); coronary angiography with PCI (a stent to the right coronary artery)
after an inferior-wall STEMI in 2012 (R1).

Family history: brother with a bicuspid aortic valve; father MI at 62; mother type 2
diabetes.

**Joined the practice 2018-09-18** (A1), age 66, from another primary-care office, already
followed by an outside cardiology group whose anticoagulation clinic runs his warfarin
(R3). He brought: essential hypertension, type 2 diabetes on insulin, atrial
fibrillation, coronary artery disease with old MI and a coronary stent, mechanical aortic
valve (bicuspid aortic stenosis), hyperlipidemia, long-term anticoagulation, history of
tobacco dependence. Drugs on joining: warfarin 7.5 mg daily (dose per INR), metoprolol
succinate 100 mg daily, lisinopril 20 mg daily, amlodipine 5 mg daily, metformin 1000 mg
twice daily, insulin glargine 24 units at bedtime, atorvastatin 40 mg daily.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2018-09-18 | practice | new-patient visit (A1): history as above; exam irregularly irregular rhythm with crisp mechanical valve click; ECG atrial fibrillation, controlled rate; joining labs; records requested |
| 2018-10 | outside records | prior primary-care office records: the problems and drugs above; screening colonoscopy 2015 normal (P); the 2018 valve operation and 2012 PCI summarized (S) |
| 2020-11-23 | outside | **cardiology follow-up letter (E0)**: two months of intermittent palpitations and mild exertional breathlessness; no chest pain, dizziness or syncope. BP 142/88, HR 88 irregularly irregular, SpO2 97% on room air. Heart: S1 inconsistent, S2 varies in intensity, no murmur or gallop (valve click as P). Hgb 14.2, platelets 250, Cr 1.1 (eGFR as the lab computes it, R4), INR 2.8 (R5). ECG atrial fibrillation, ventricular rate 86. Plan: same drugs, anticoagulation clinic continues, cardiology in 6 months (R6). Letter received 2020-12-02 |
| 2021-02-16 | outside | **ED visit (E1)**: palpitations and breathlessness for 2 hours that began while potting seedlings indoors (R7); no chest pain or syncope. BP 150/90, HR 130 irregularly irregular, SpO2 96% on room air; no crackles, no leg edema. Hgb 14.2, WBC 7.2, platelets 230, Cr 1.1, BUN 18, glucose 160, TSH 2.5. ECG: no P waves, variable rapid ventricular response. AF with rapid ventricular response; IV metoprolol, rate under 100, discharged the same day, drugs unchanged (R6). The practice learns from the patient the next day; the ED record arrives 2021-03-24 |
| 2021-02-24 | practice | post-ED visit (P): feels back to baseline; rate controlled; no changes |
| 2022-04-29 to 2022-05-06 | outside | **hospital admission (S, dated by R8)**: hemoptysis, weight loss (87 kg in February to 81 kg), upper lobe infiltrates on chest imaging; sputum AFB smear positive and NAAT positive for M. tuberculosis complex (A2). **Pulmonary tuberculosis diagnosed**; **rifampin 600 mg, isoniazid 300 mg, pyrazinamide 2000 mg and ethambutol 1600 mg daily started 2022-05-03** (A3), directly observed by a regional TB clinic. The practice hears of it by the patient's call on 2022-05-09; the discharge summary arrives 2022-06-14, after the ED visit |
| 2022-05-17 | outside | anticoagulation clinic: INR 1.2; **warfarin increased to 10 mg daily** (S "increased last week"; dose A4). No note reaches the practice; the patient reports it |
| 2022-05-24 | outside | **ED visit (E2)**: blood-streaked sputum since the evening before (R9); no chest pain, breathlessness or melena; vitals normal; exam unremarkable. INR 1.7, still below goal. ED impression: anticoagulation-related bleeding while rifampin lowers the warfarin effect; warfarin continued at 10 mg, anticoagulation clinic recheck in 3 days, TB clinic informed. ED note received 2022-05-27 |
| 2022-06-02 | practice | post-ED visit (P): no further blood in sputum; weight 82 kg; **anticoagulation-related hemorrhage** recorded; TB drugs and warfarin 10 mg entered as outside-managed |
| 2022-07-05 | outside | (A5) TB clinic ends the intensive phase: **pyrazinamide and ethambutol stopped**; sputum cultures converted negative by month two; isolate pan-susceptible. No letter is sent; the practice list keeps all four drugs until the 2022-08-18 visit, when the patient's bottles correct it |
| 2022-11-03 | outside | (A5) TB clinic completion letter (received 2022-11-17): six months completed, **rifampin and isoniazid stopped**, chest x-ray improved with residual upper lobe scarring. Anticoagulation clinic brings **warfarin back to 7.5 mg daily** over the following weeks (A4) |
| 2022-12-07 | practice | follow-up (P): **tuberculosis resolved, personal history of tuberculosis** recorded; anticoagulation-related hemorrhage resolved; warfarin 7.5 mg on the list |
| 2023-11-15 to 2023-11-18 | outside | **ED and admission (E3)**: substernal pressure radiating to the left arm with breathlessness and sweating. BP 160/90, HR 102, RR 22, SpO2 94% on room air. Regular rhythm, no murmur, rub or gallop (R10); lungs clear. Troponin I 0.5 ng/mL (ref <0.04), Cr 1.2, A1c 7.8, total cholesterol 185. ECG ST elevation in II, III and aVF. **Inferior STEMI**; emergent angiography with a drug-eluting stent to the mid right coronary artery (R11, A6); **clopidogrel 75 mg daily for 12 months** added to warfarin, aspirin in hospital only (A7); other drugs unchanged. The patient messages the practice 2023-11-20; clopidogrel shows up as a pharmacy refill request; the discharge summary arrives 2024-01-09; the catheterization report never arrives |
| 2023-11-28 | practice | post-hospital visit (P): from the patient's account and his bottles; STEMI and clopidogrel added |
| 2024-02-20 | outside | cardiology follow-up letter (A8): no angina; clopidogrel until November 2024, then warfarin alone; no other change |
| 2024-12-12 | practice | follow-up: **clopidogrel stopped** at 12 months per the cardiology plan (A7) |
| 2025-09-12 | practice | **last seen**: annual wellness and diabetes visit; labs drawn and resulted; LDCT for lung screening ordered for October and never done (P) |
| 2025-11-21 to 2025-11-25 | outside | **ED and admission (E4)**: chest pressure radiating to the left arm with breathlessness and sweating starting 30 minutes earlier while mulching leaves with his mower (R12). BP 160/90, HR 98, RR 22. Troponin I 0.5 ng/mL, Cr 1.2, A1c 7.2. ECG ST elevation in II, III and aVF; right-sided leads with ST elevation in V4R (A9). Aspirin, nitroglycerin and morphine given. Cardiology advised immediate PCI; the move to the catheterization lab was held up about 15 minutes when the order was taken as a repeat ECG, and went ahead once clarified (R13). Proximal right coronary artery occlusion, drug-eluting stent (A9). **Inferior STEMI with right ventricular infarction**. Discharged on warfarin 7.5 mg plus **clopidogrel 75 mg daily for 12 months** (A10), other drugs unchanged. After last seen: records filed, lists not updated |
| 2025-12 to 2026-02 | outside records | ED record received 2025-12-03; pharmacy refill request for clopidogrel 2025-12-16, refilled; discharge summary received 2026-01-20; cardiology follow-up letter 2026-02-10 (A10): no angina, clopidogrel to November 2026 |
| 2026-09-16 | practice | booked visit: overdue follow-up after the November hospitalization and diabetes |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists a prior inferior STEMI with PCI from 2020, yet stamps the
  inferior STEMI "2023" and has another in 2025. Resolution: three inferior events. The
  first, in 2012, was treated with a right coronary stent; that year he quit smoking
  ("quit 8 years ago" at the 2020 visit, held at 2012 throughout, not re-aged each note).
  The 2023 and 2025 events are new.
- **R2.** "Mechanical aortic valve replacement (2 years prior)" is copied unchanged into
  every note. Resolution: the operation was in early 2018, before he joined.
- **R3.** The source does not say who doses warfarin. Resolution: the outside cardiology
  group's anticoagulation clinic, goal INR 2.5 to 3.5 (mechanical aortic valve with atrial
  fibrillation). The practice sees INR reports only now and then.
- **R4.** eGFR 60 with creatinine 1.1 at 68 does not match any lab formula. Resolution: the
  creatinine is carried; eGFR is what the lab computes (about 70). eGFR 60 is not carried.
- **R5.** INR 1.0 on warfarin 7.5 mg for a mechanical valve, with no response, cannot be
  true. Resolution: the November 2020 INR is 2.8; 1.0 is not carried.
- **R6.** Encounter 0 stops at a score and encounter 1 gives no treatment or disposition.
  Resolution: E0's plan is no change with cardiology in 6 months; the CHA2DS2-VASc line is
  not carried (anticoagulation is set by the mechanical valve). E1 is treated with IV
  metoprolol and discharged the same day on unchanged drugs.
- **R7.** "While gardening" falls in February in western Massachusetts. Resolution: he was
  potting seedlings indoors.
- **R8.** "Diagnosed with TB three weeks ago ... following recent hospitalization" has no
  dates. Resolution: admission 2022-04-29 to 2022-05-06, diagnosis and drug start 2022-05-03.
- **R9.** E2 is a "bleeding complication" with anticoagulation-related hemorrhage as its
  diagnosis, yet says no recent bleeding and no melena. Resolution: the bleed is a small
  amount of blood-streaked sputum; "no recent bleeding" is read as no other bleeding. The ED
  still labels it anticoagulation-related at an INR of 1.7, and nobody questions that.
- **R10.** Chronic atrial fibrillation, yet a regular rhythm in November 2023. Resolution:
  the atrial fibrillation is not permanent (recorded as atrial fibrillation, unspecified);
  he was in a regular rhythm that day. Practice exams describe an irregular rhythm at some
  visits and a regular one at others.
- **R11.** E3's assessment stops mid-sentence. Resolution: inferior STEMI, emergent
  angiography and stenting, three hospital days.
- **R12.** "Mowing the lawn" on 21 November. Resolution: mulching fallen leaves with the
  mower.
- **R13.** The source explains the transfer delay by a misheard order in a noisy room (quiz
  text). Resolution: the ED record says the transfer was delayed about 15 minutes because
  the order was taken as a repeat ECG, and that it was clarified; no cause, no lesson.
- Source medication lists swap drugs between "Medications" and "Home Medications" from note
  to note; all seven drugs run throughout at the same doses except warfarin (A4).
- **Left as the source has it:** atorvastatin stays 40 mg after the 2023 and 2025 STEMIs,
  with LDL around 100 to 115, and no note comments on it. No bridging is given when the INR
  falls to 1.2 on rifampin. No echocardiogram appears in the chart. Insulin glargine stays
  24 units with A1c in the 7s.

## Additions (A): physician to confirm

- **A1.** The 2018 new-patient visit (E0 is a "routine follow-up" with an established
  history; eight years with the practice).
- **A2.** Sputum AFB smear and NAAT positive, culture pan-susceptible: the evidence for the
  source's TB diagnosis and for a six-month regimen.
- **A3.** TB drug doses for 81 kg (the source names the four drugs only).
- **A4.** Warfarin 10 mg daily from 2022-05-17 to early December 2022, then 7.5 mg again
  after rifampin stops (the source's later lists say 7.5 mg).
- **A5.** The TB course to completion: intensive phase to 2022-07-05, isoniazid and
  rifampin to 2022-11-03, completion chest x-ray with improved upper lobe opacities and
  residual scarring.
- **A6.** Mid right coronary stent in 2023 (the source gives an inferior STEMI and PCI).
- **A7.** Clopidogrel 75 mg daily 2023-11-18 to 2024-12 (standard after a stent on
  warfarin); aspirin only in hospital.
- **A8.** The February 2024 cardiology follow-up letter.
- **A9.** Right-sided ECG ST elevation in V4R and a proximal right coronary occlusion with
  a stent in 2025: the evidence for the source's right ventricular infarction.
- **A10.** Clopidogrel 75 mg daily again from 2025-11-25 for 12 months, and the February
  2026 cardiology letter.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  A mechanical valve's crisp closing click may be described on exam. An abnormal finding
  appears only as the source or this file gives it; a report may carry the descriptors its
  kind always carries, but nothing that adds a territory, a severity grade or a symptom.
- Records from the prior office may mention routine screening there (colonoscopy 2015,
  normal).
- Diabetes and blood pressure visits about every 3 to 4 months (telehealth during 2020),
  yearly annual wellness visits, ECG at joining. Levels: weight about 88 kg (height 178 cm)
  until 2022, 81 to 82 kg in May to June 2022, back to about 86 kg from 2023; office BP
  mostly 128 to 138 / 76 to 84 on his drugs; heart rate 60s to 80s; A1c 7.1 to 7.9 (lowest
  in mid-2022 after the weight loss); LDL 100 to 115, HDL about 42, triglycerides 140 to
  180; creatinine 1.0 to 1.2; urine albumin/creatinine normal; CBC normal; TSH normal.
- INR reports from the anticoagulation clinic, now and then, between 2.5 and 3.5 except as
  the timeline gives.
- Diabetic foot exams normal; yearly diabetic eye exam letters from an optometrist with no
  retinopathy (some years none arrives).
- Lung cancer screening (30 pack-years, quit 2012): LDCT Lung-RADS 1 in 2018, 2019, 2020
  and 2021; none in 2022 (on TB treatment); 2023 and 2024 Lung-RADS 2 with upper lobe
  scarring from treated tuberculosis and no nodules; 2025 ordered, not done.
- One AAA screening ultrasound (male smoker 65 to 75): normal aorta.
- Hepatitis C and HIV screening once, negative. PHQ-2 yearly, negative. FIT in 2025 (ten
  years after the colonoscopy), negative.
- Vaccines: influenza each fall (2025 at a pharmacy); PCV13 2018 and PPSV23 2019; Shingrix
  two doses 2019; Tdap 2019; COVID primary series early 2021 at a community clinic and
  boosters each fall from 2021; RSV once in 2024.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders.
- Booked visit after 2026-08-27: follow-up after the November 2025 hospitalization.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002207 on 2025-09-12. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
