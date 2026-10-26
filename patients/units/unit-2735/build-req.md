# Build requirements: unit-2735 (Sandra Wojcik, MRN00002735)

Source: synthetic_hospital patient 2735, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style phrasing.

## Who she is (S)

Sandra Wojcik, born 1970-07-20, female, White, not Hispanic or Latino, English,
Northampton MA 01060, employer-sponsored PPO. Administrative coordinator at a school
district. Former smoker, quit 2015, about 8 pack-years. Alcohol: 1 to 2 glasses of wine
on weekends. No known drug allergies.

Surgical history: tonsillectomy and adenoidectomy (1978), cesarean section (1994).

Family history: mother with type 2 diabetes and gallstones (cholecystectomy at 52);
father with hypertension and hyperlipidemia; one older sister with gallstones.

Joined the practice on **2024-09-10** (A1), records complete (outside letters, reports
and summaries arrive and are filed; lists kept current). She brought:

- Type 2 diabetes mellitus since 2018; hemoglobin A1c 7.2% (the establish-care draw).
- Cholelithiasis with recurrent biliary colic (episodes every few months after rich meals).
- Obesity, BMI 33 kg/m² (height 162 cm, weight about 86.6 kg).
- Essential hypertension, controlled on lisinopril.
- Medications: metformin 1000 mg twice daily, glipizide 5 mg daily, lisinopril 10 mg
  daily, atorvastatin 20 mg at bedtime.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | establish-care visit (A1): history as above; BMI 33; BP controlled; A1c 7.2; biliary colic every few months, last episode in July; surgical referral offered and she defers (A2) |
| 2024-09-23 | outside records | prior practice records arrive: 2023 right upper quadrant ultrasound with gallstones, normal duct caliber (A3); routine screening per P |
| 2025-04-14 | practice (portal) | a biliary colic episode after a restaurant meal, a few hours, settled on its own, no fever or yellowing (S: recurrent biliary colic); surgical referral offered again, she plans to call after the school year; it is never placed (left as the source has it) |
| 2025-12-01 to 2025-12-05 | outside (ED, admission) | 2 days of right upper quadrant pain, fever and jaundice. T 38.9 °C (102 °F), BP 90/60, HR 110. Confused. RUQ tender without rebound or guarding. WBC 18,000, Hgb 12.5, platelets 250,000, bilirubin total/direct 3.0/2.5, AST 80, ALT 75, alkaline phosphatase 300, BUN 18, creatinine 1.0, lactate 4.5, anion gap 20 (R1). RUQ ultrasound: dilated common bile duct with a stone at its distal end; gallstones in the gallbladder. **Choledocholithiasis with cholangitis diagnosed.** Course per A4. Records reach the practice 2025-12-08 |
| 2025-12-11 | practice | post-discharge (transitional care) visit (P); cholangitis resolved; surgery consult already booked by the hospital |
| 2025-12-22 | outside | general surgery consult letter: interval laparoscopic cholecystectomy planned for 2026-02-06; asks the PCP for preoperative evaluation with an ECG (A5) |
| 2026-01-27 | practice | preoperative evaluation (A5): preoperative ECG normal sinus rhythm, no conduction abnormality (S) |
| 2026-02-06 to 2026-02-07 | outside (surgery) | laparoscopic cholecystectomy under general endotracheal anesthesia. About 30 minutes in, during CO2 insufflation, heart rate falls to 40 with BP 110/70, otherwise hemodynamically stable; no surgical difficulty. **Vagal bradycardia secondary to pneumoperitoneum.** Management per A6. Pathology per A6. Overnight observation, discharged 2026-02-07. **Cholelithiasis resolved.** Records reach the practice 2026-02-10 |
| 2026-02-12 | practice | hoarse voice and scratchy throat since the surgery (R2): **acute laryngitis diagnosed**; voice rest, fluids, humidified air. Vagal bradycardia recorded as resolved (transient, intraoperative) |
| 2026-02-20 | outside | surgery post-op letter: incisions healed, eating normally, pathology reviewed, discharged from surgical follow-up |
| 2026-02-26 | practice (portal) | voice back to normal: **acute laryngitis resolved** |
| 2026-03-07 to 2026-03-13 | outside (ED, ICU, admission) | one month after the cholecystectomy: severe right upper quadrant pain, fever, jaundice, nausea and vomiting, becoming more confused and lethargic. T 38.9 °C (102 °F), BP 90/60, HR 110, RR 22. Jaundiced, in distress, RUQ tenderness (R3). WBC 18,000, Hgb 12.5, platelets 250,000, bilirubin total 5.2, direct 3.8, alkaline phosphatase 210, AST 80, ALT 75, amylase 300, lipase 900 (R4). Abdominal ultrasound: common bile duct stone with dilated intrahepatic ducts, gallbladder surgically absent (R3). **Choledocholithiasis with cholangitis diagnosed again; septic shock diagnosed** (R5). Course per A7. Records reach the practice 2026-03-16 |
| 2026-03-19 | practice | post-discharge (transitional care) visit (P); lisinopril restarted (A8); septic shock and cholangitis resolved |
| 2026-04-21 | outside | gastroenterology clinic letter (A7 follow-up): no symptoms, liver tests normal, no further ERCP planned unless symptoms return |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source gives an anion gap of 20 without its electrolytes. Resolution: the
  December ED panel carries sodium 136, chloride 100 and bicarbonate 16 so the gap of 20
  adds up with the lactate of 4.5. No other December ED numbers are added.
- **R2.** The source lists acute laryngitis as a primary diagnosis but gives it no
  encounter. Resolution: it is the hoarseness that follows the endotracheal intubation of
  2026-02-06, seen at the practice on 2026-02-12 and gone by 2026-02-26. No laryngoscopy
  and no drug.
- **R3.** The March 2026 encounter comes one month after cholecystectomy, yet the source
  gives a positive Murphy sign and "cholelithiasis" on ultrasound. Resolution: the
  gallbladder is absent; the exam carries right upper quadrant tenderness and the
  ultrasound carries the common bile duct stone and dilated intrahepatic ducts with an
  empty gallbladder fossa. "Murphy sign" and "cholelithiasis" are not carried for March.
- **R4.** The source gives amylase 300 and lipase 900 in March and names no pancreatitis.
  Resolution: the values are carried exactly; the ICU summary reads them with the stone
  impacted at the distal duct and names no separate pancreatic diagnosis. Lipase is
  rechecked at the 2026-03-19 practice visit (normal). Physician to confirm that no
  pancreatitis diagnosis is added.
- **R5.** The source lists septic shock as a comorbidity without saying when. Resolution:
  septic shock belongs to the March 2026 ICU admission (hypotension needing norepinephrine
  after fluids). The December 2025 hypotension and lactate of 4.5 corrected with IV fluids
  on the floor, and that admission carries no shock or sepsis diagnosis.
- **R6.** The source copies "cholelithiasis with recurrent biliary colic" and the
  intraoperative bradycardia onto the March list. Resolution: cholelithiasis resolves with
  the cholecystectomy; the bradycardia is a transient intraoperative event, resolved on
  2026-02-12. The hospital's March letter may still carry a stale history line, as outside
  lists do; the practice's own list does not.
- **R7.** The source's March HPI repeats "Type 2 diabetes" with an A1c of 7.2% stamped
  2018. Resolution: A1c 7.2 is the 2024-09-10 draw; later draws scatter around it.
- **Left as the source has it:** symptomatic cholelithiasis with recurrent colic goes
  without a surgical referral until the cholangitis, because she defers and then does not
  follow up; no one remarks on it after the fact. Atorvastatin stays 20 mg.

## Additions (A): physician to confirm

- **A1.** The 2024-09-10 establish-care visit (the source starts with her already carrying
  four chronic conditions and four drugs). Labs: A1c, CMP, lipid panel, CBC, urine
  albumin/creatinine, hepatitis C and HIV screens.
- **A2.** Surgical referral offered at establish care and declined for now (the source's
  recurrent biliary colic predates the cholangitis by an unknown time).
- **A3.** One prior-practice right upper quadrant ultrasound (2023): multiple gallstones,
  no gallbladder wall thickening, common bile duct normal caliber. Supports the listed
  cholelithiasis.
- **A4.** December 2025 hospital course: blood cultures (no growth), IV fluid bolus with
  BP recovering, IV piperacillin-tazobactam, glipizide and metformin held in hospital with
  correctional insulin; ERCP on 2025-12-02 with biliary sphincterotomy and extraction of
  the distal common bile duct stone, duct clear on final cholangiogram; liver tests
  falling; discharged 2025-12-05 on **amoxicillin-clavulanate 875/125 mg twice daily to
  finish 7 days of antibiotics** (short course), home medications resumed, referral to
  general surgery for interval cholecystectomy.
- **A5.** The general surgery consult and the practice preoperative evaluation (the
  source's preoperative ECG must come from somewhere). Preop CBC and BMP normal, A1c on
  the draw. Instructions: hold metformin, glipizide and lisinopril the morning of surgery.
- **A6.** Bradycardia management and the rest of the surgical record: insufflation
  released, atropine 0.5 mg IV, heart rate back to the 70s within about a minute,
  reinsufflation at a lower pressure without recurrence, procedure completed. Overnight
  telemetry without recurrence. Pathology: gallbladder with multiple stones, negative for
  dysplasia and malignancy. Discharge drugs: **oxycodone-acetaminophen 5/325 mg, 1 tablet
  every 6 hours as needed, 10 tablets** and acetaminophen as needed (short courses).
- **A7.** March 2026 course: IV fluids, norepinephrine for about 24 hours, IV
  piperacillin-tazobactam, blood cultures (no growth), ICU 2026-03-07 to 2026-03-10; ERCP
  on 2026-03-08 with balloon sweep removing a retained distal common bile duct stone,
  duct clear; off norepinephrine 2026-03-09; liver tests and lipase falling; discharged
  2026-03-13 on **amoxicillin-clavulanate 875/125 mg twice daily to finish the course**
  (short); **lisinopril held at discharge** because of the shock; follow-up with the
  gastroenterology clinic.
- **A8.** Lisinopril 10 mg daily restarted at the 2026-03-19 visit (BP about 138/86 off
  it), BP back to about 128/80 afterward.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries (how a duct was measured, a procedure's access
  site), but nothing that adds a territory, a severity grade or a symptom, or changes the
  diagnosis or plan.
- Records from the prior practice may mention routine screening done there with normal
  results: screening colonoscopy 2021 without polyps, Pap with HPV co-test 2022 negative,
  mammogram 2023 normal, Tdap 2019, a diabetic eye exam in 2023 without retinopathy,
  COVID primary series 2021.
- Diabetes and blood pressure visits about every 3 to 5 months, an annual physical each
  fall, with vitals, A1c, metabolic panels, lipids yearly, urine albumin/creatinine yearly
  (normal), foot exams (normal sensation and pulses).
- Yearly diabetic eye exam letters from an optometrist: no retinopathy.
- Screening mammograms each fall, normal. Depression screens (PHQ-2/PHQ-9) negative.
- Influenza vaccine each fall, updated COVID vaccines, Shingrix two doses, PCV20 once
  (diabetes). Hepatitis C and HIV screening once, nonreactive.
- Post-discharge transition calls by the nurse, refills, pharmacy traffic, portal
  messages about results, refills and scheduling, a cancelled visit.
- Booked visit after 2026-08-27: annual physical in late September 2026.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
