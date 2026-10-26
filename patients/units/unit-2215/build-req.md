# Build requirements: unit-2215 (Roger Wright, MRN00002215)

Source: synthetic_hospital patient 2215, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its copied "6 months prior" surgical line.

## Who he is (S)

Roger Wright, born 1951-11-21, male, White, not Hispanic or Latino, English, Medicare,
Hadley MA 01035. Retired postal worker. Former smoker, 40 pack-years, quit in 2022
(R3). No alcohol. Allergy: sulfonamide antibiotics (rash). Surgical history: one
coronary stent in 2022 at an outside hospital (R3); one left femoral catheterization
for diagnostic coronary angiography on 2024-09-30 (R2). Family history: father
myocardial infarction at 64; mother ischemic stroke at 79; a sibling with type 2
diabetes and end-stage renal disease. Marital status in the record: divorced (P, a
demographic field only; no spouse or partner appears in the chart).

Joined the practice on 2023-09-12 (A1, new patient visit). He brought:

- Problems: essential hypertension; type 2 diabetes on insulin with diabetic chronic
  kidney disease; chronic kidney disease stage 3a (creatinine about 1.6); hyperlipidemia;
  coronary artery disease with a coronary stent (2022); peripheral arterial disease;
  former smoker.
- Medications: insulin glargine 24 units subcutaneously at bedtime; insulin aspart by
  correction scale with meals; lisinopril 10 mg daily; amlodipine 5 mg daily;
  atorvastatin 40 mg at bedtime; aspirin 81 mg daily.

## Timeline (dates fixed; S unless marked)

Records texture (seed): gaps. Each outside contact below says what reaches the practice.

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | **new patient visit** (A1): history and medicines as above; labs drawn (levels under P) |
| 2023-09-27 | practice | prior practice records arrive (P): colonoscopy 2018 without polyps; the 2022 stent on its list |
| 2023-10-16 | outside | **ankle-brachial index 0.84 right, 0.78 left** (A2); abdominal aorta ultrasound, normal caliber (A4) |
| 2023-10-26 | outside | low-dose CT lung screening, Lung-RADS 1 (A3) |
| 2023-11-14 | outside | cardiology yearly letter (A5): no angina, stable, continue current medicines |
| 2024-09-27 to 2024-10-01 | outside | admission at an outside hospital for **unstable angina**; on 2024-09-30 **diagnostic coronary angiography through the left femoral artery**; no stent placed; medicines unchanged (R2). The hospital never sends records: the practice learns of it from the patient's portal message on 2024-10-03, requests records on 2024-10-04, and nothing arrives |
| 2024-10-08 | practice | post-hospital visit (P): no chest pain since; left groin site without hematoma; feet warm; creatinine about 1.6; unstable angina recorded as history |
| 2024-10-20 | outside | ED (encounter 0, three weeks after the catheterization): 2 days of severe sharp constant left foot pain, worse with elevating the leg; recent transient visual disturbances; abdominal pain after eating. Left foot cool, a cyanotic toe, mottled net-like discoloration over the lower legs; fundoscopy with bright refractile plaques at retinal arteriole branch points. **Creatinine 2.8 (baseline 1.6), BUN 44, CBC normal, CRP 46 mg/L, ESR 58 mm/h** (R6). **Atheroembolism (cholesterol embolism syndrome)** with **acute kidney injury**. Admitted 2024-10-20 to 2024-10-23 (A6). The ED record reaches the practice 2024-10-25; the discharge summary arrives late, 2024-11-19 |
| 2024-10-28 | practice | post-discharge visit (P; A6 course): toe still dusky, pain easing; creatinine 2.3; atheroembolism and acute kidney injury added to the list here (the list lags until this visit) |
| 2024-11-18 | practice | lab recheck: creatinine 1.9 (A6) |
| 2024-12-11 | outside | cardiology letter (A5): no angina since the angiography; aware of the atheroembolism; continue aspirin and atorvastatin 40 mg |
| 2025-01-13 | practice | follow-up: toe color normal, no tissue loss; creatinine back to 1.6 (A6, R5); **acute kidney injury resolved**; unstable angina, quiet since September, closed on the list |
| 2025-04-25 to 2025-04-29 | outside | ED (encounter 1): 6 hours of severe constant epigastric pain radiating to the back with nausea and vomiting; no chest pain, dyspnea or bowel change. **BP 165/90, HR 102, T 38.2 °C**; distended abdomen, epigastric tenderness, decreased bowel sounds. **WBC 16.0, hemoglobin 13.2, BUN 31, creatinine 2.1 (baseline 1.6), amylase 900 U/L, lipase 1100 U/L, glucose 210.** CT abdomen: peripancreatic inflammation and edema. **Acute pancreatitis**, with **acute kidney injury** again. Admitted (A7). The ED record never reaches the practice; the patient's portal message on 2025-05-01 is how the practice learns; the discharge summary (carrying the ED numbers and CT) arrives late, 2025-05-27 |
| 2025-05-05 | practice | post-discharge visit (A7 course): eating normally, pain gone; lipase normal; creatinine 1.9 |
| 2025-06-17 | practice | wellness visit: **acute pancreatitis and acute kidney injury resolved**; creatinine still 1.9, **chronic kidney disease restaged 3b** (R5) |
| 2025-10-26 to 2025-10-31 | outside | ICU admission (encounter 2): brought by ambulance with altered mental status, **severe hypoglycemia** in the setting of **urosepsis**. **BP 95/60, HR 102, RR 22, T 38.3 °C.** Lethargic but arousable; skin warm and dry. **Glucose 48, creatinine 2.8 (baseline 1.9), BUN 44, potassium 4.5, sodium 135; arterial gas pH 7.32, PaCO2 32, HCO3 18.** Acute kidney injury again. Course and discharge as A8. The practice first learns of it from the pharmacy on 2025-11-01 (a new glargine 18-unit sig and a cefpodoxime fill); the discharge summary arrives 2025-11-07 |
| 2025-11-04 | practice | post-discharge visit (A8, A9): **practice list changes glargine 24 to 18 units** (until now it lagged at 24); **glucagon nasal powder prescribed**; creatinine 2.1; no hypoglycemia since discharge |
| 2025-12-10 | practice | lab recheck: creatinine 1.9; **acute kidney injury, urinary infection, sepsis and the hypoglycemic event resolved** |
| 2025-12 | outside | cardiology yearly visit (A5): the letter never comes; at the 2026-02 visit he reports nothing changed |
| 2026-08-24 | practice | routine labs for the booked September visit, resulted and not yet commented on |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's first encounter is an ED visit with a full problem list already
  in place, so he was an established patient by then: he joined on 2023-09-12 (A1).
  Conditions the encounters diagnose (atheroembolism, acute pancreatitis, severe
  hypoglycemia, urosepsis, acute kidney injury, the stage 3b restaging) start at their
  encounters, not at joining.
- **R2.** The source has "cardiac catheterization 3 weeks ago due to unstable angina"
  (encounter 0) and "left femoral artery catheterization for diagnostic angiography
  (6 months prior)" copied unchanged into encounters 1 and 2. Resolution: one
  procedure, a diagnostic coronary angiography through the left femoral artery on
  2024-09-30, during an outside admission for unstable angina (2024-09-27 to
  2024-10-01), three weeks before encounter 0. No stent placed and no medicine
  changed (the source's home list is the same before and after). The later "6 months
  prior" lines are stale copies and are not carried. The hospital never sends a
  report, so the chart has no angiography findings.
- **R3.** "Coronary artery stent placement (2022)" is dated after the source's own
  encounters. Resolution: one stent placed in calendar 2022 at an outside hospital,
  before he joined; the stent's details and any antiplatelet course after it are not
  in the chart. "Quit 2 years ago", repeated unchanged at all three encounters, places
  the quit in 2022 (about the time of the stent); the chart says quit 2022 throughout.
- **R4.** The encounters move drugs between the home list and prose (encounter 1 puts
  lisinopril and amlodipine in prose, encounter 2 puts the insulins there). Resolution:
  the same six drugs at the same doses throughout until A8; nothing starts or stops.
- **R5.** Baselines: creatinine 1.6 at encounters 0 and 1, 1.9 at encounter 2; profile
  "CKD stage 3" and comorbidity "stage 3b". Resolution: baseline about 1.6 (eGFR about
  45, stage 3a) from joining; after the atheroembolic injury it recovers to 1.6 by
  January 2025, so the April 2025 ED's 1.6 baseline is right; after the pancreatitis
  injury it settles at 1.9 (eGFR in the high 30s) and the practice restages to 3b in
  June 2025, so the October 2025 baseline of 1.9 is right.
- **R6.** CRP and ESR are "elevated" with no values. Resolution: CRP 46 mg/L and ESR
  58 mm/h at encounter 0.
- **R7.** Encounter 2 is filed under Internal Medicine but is an ICU stay: outside care.
  The arterial gas belongs to the ICU and is kept.
- **R8.** The source's "(diagnosed ...)" stamps and its later problem lists keeping
  atheroembolism and pancreatitis as active: the chart keeps atheroembolism as a
  lasting history problem and resolves pancreatitis in June 2025.
- **Left as the source has it:** atorvastatin stays 40 mg and aspirin stays the only
  antiplatelet after the unstable angina and the atheroembolism; lisinopril and
  amlodipine continue unchanged through each kidney injury; no metformin, SGLT2
  inhibitor or GLP-1 drug is raised; no carotid imaging or eye referral follows the
  retinal plaques and visual symptoms; the post-meal abdominal pain gets no workup
  beyond the ED; no nephrology referral; insulin aspart stays a correction scale. No
  note comments on any of these.

## Additions (A): physician to confirm

- **A1.** The 2023-09-12 new-patient visit (three years with the practice; the source
  begins established).
- **A2.** One ankle-brachial index, 0.84 right and 0.78 left, to support the listed
  peripheral arterial disease. No claudication is recorded either way beyond "denies
  leg pain with walking" at routine visits.
- **A3.** Yearly low-dose CT lung screening (40 pack-years, quit 2022): 2023-10-26,
  2024-12-19, 2025-12-18, each Lung-RADS 1.
- **A4.** One abdominal aorta ultrasound for aneurysm screening (man 65 to 75 who
  smoked) on 2023-10-16: normal caliber, no aneurysm.
- **A5.** An outside cardiology group follows the 2022 stent yearly: letters
  2023-11-14 (stable, no angina) and 2024-12-11 (as in the timeline); the December 2025
  visit sends no letter. Each letter: regular rhythm, no murmur, no edema, ECG sinus
  rhythm without acute change, continue current medicines.
- **A6.** Encounter 0 admission 2024-10-20 to 2024-10-23: supportive care, aspirin and
  atorvastatin continued, no anticoagulation, no new drug; creatinine 2.5 at discharge.
  Course: creatinine 2.3 on 2024-10-28, 1.9 on 2024-11-18, 1.6 on 2025-01-13; the toe
  recovers its color with no tissue loss by January 2025.
- **A7.** Encounter 1 admission 2025-04-25 to 2025-04-29: IV fluids, bowel rest, IV
  pain control, diet advanced; right upper quadrant ultrasound with no gallstones and
  no bile duct dilation; triglycerides 172; no alcohol; the summary names no cause.
  Creatinine 1.9 at discharge. Lipase normal on 2025-05-05.
- **A8.** Encounter 2 admission 2025-10-26 to 2025-10-31 (ICU first two days): IV
  dextrose, IV fluids, IV ceftriaxone; urine culture Escherichia coli susceptible to
  ceftriaxone; blood cultures no growth; insulin held, then glargine resumed at 18
  units at bedtime with the aspart correction scale; discharged on cefpodoxime 200 mg
  twice daily to finish 10 days of antibiotics; creatinine 2.3 at discharge, 2.1 on
  2025-11-04, 1.9 on 2025-12-10.
- **A9.** Glucagon 3 mg nasal powder for severe hypoglycemia, prescribed 2025-11-04.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Levels between the source's numbers: height 175 cm, weight about 91 kg; office blood
  pressure about 138/78 on lisinopril and amlodipine; A1c about 7.7 until the glargine
  reduction, about 7.9 after; LDL about 76, HDL about 42, triglycerides about 160 on
  atorvastatin 40 mg; hemoglobin about 13.2; potassium about 4.6; BUN about 26 at a
  creatinine of 1.6 and about 30 at 1.9; urine albumin/creatinine normal.
- Diabetes, blood pressure and kidney visits every 3 to 4 months, with vitals, A1c,
  metabolic panels, lipids and urine albumin/creatinine yearly, foot exams (pulses
  diminished in keeping with the PAD, sensation intact, skin intact apart from the
  2024 toe).
- Yearly diabetic eye exam letters from an outside optometrist: no diabetic
  retinopathy (the letters after October 2024 note no retinal emboli seen).
- Prevention: colonoscopy 2018 in prior records without polyps; hepatitis C screen
  once (nonreactive); high-dose influenza each fall; updated COVID vaccine each fall;
  PCV20 once (2023); Shingrix two doses (2024); RSV once (2024); Tdap (2025); PHQ-2 at
  wellness visits, negative; Medicare wellness visits yearly.
- Refills, insulin pen supply and pharmacy traffic, portal messages about results and
  scheduling, a records request that gets no answer, a missed visit in spring 2026.
- Booked visit after 2026-08-27: routine diabetes and kidney follow-up on 2026-09-08.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002215 on 2026-04-28. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
