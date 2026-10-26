# Build requirements: unit-2279 (Wayne Alden, MRN00002279)

Source: synthetic_hospital patient 2279, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a hole;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or any quiz framing.

## Who he is (S)

Wayne Alden, born 1962-11-27, male, White, not Hispanic or Latino, English, employer
private insurance. Regional sales manager who travels for work (conventions, hotels).
Current cigarette smoker, 40 pack-years. Alcohol occasional, 1 to 2 drinks a week.
No known drug allergies. Surgical history: laparoscopic cholecystectomy in 2010 (R1).
Family history: father had type 2 diabetes with diabetic nephropathy and died at 68 of
a myocardial infarction; mother has hypertension and chronic kidney disease stage 3;
an older brother has coronary artery disease.

Joins the practice on 2025-03-11 (A1, new-patient visit), moving his care from a prior
primary-care office. He brings: type 2 diabetes on insulin, chronic kidney disease
stage 3b, hypertension, dyslipidemia, nicotine dependence (R2). Medications on joining,
all continued unchanged: insulin glargine 24 units subcutaneously at bedtime; insulin
lispro by sliding scale with meals; lisinopril 10 mg daily; amlodipine 5 mg daily;
atorvastatin 40 mg daily; aspirin 81 mg daily.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2025-03-11 | practice | new-patient visit (A1); history as above; labs and prevention per P; low-dose CT ordered (A2) |
| 2025-03-24 | outside records | prior primary-care office records received (P): problem and medication list as above, colonoscopy 2021 normal, Tdap 2019 |
| 2025-04-08 | outside imaging | low-dose CT chest for lung cancer screening: no nodules, Lung-RADS 1 (A2) |
| 2025-05-06 | outside records | optometry diabetic eye exam letter: no retinopathy (P) |
| 2025-06-10 | practice | diabetes and blood pressure follow-up (P); second shingles dose |
| 2025-07-08 | outside ED, then admitted | ED: two days of fever, watery diarrhea and fatigue, then dry cough and mild shortness of breath; returned recently from a 4-day hotel stay for a business convention. Temp 39.1 °C (102.4 °F), BP 126/76, pulse 98, SpO2 94% on room air. Crackles right lower lung field. Sodium 126, AST 68, ALT 70, WBC 15,400 with neutrophil predominance (neutrophils 88%, R3). Chest x-ray: patchy infiltrates right middle and lower lobes. Sputum Gram stain: many segmented neutrophils, no organisms seen. Legionella urine antigen positive (A3). **Legionnaires' disease diagnosed**; admitted |
| 2025-07-09 to 07-13 | outside hospital | (A4) hypotension on day 2: central venous catheter (right internal jugular) and norepinephrine in the ICU, off pressors by day 4, to the medical floor on day 6. Broad-spectrum antibiotics narrowed to levofloxacin, renally dosed, 14 days total (through 07-21). Lisinopril and amlodipine held; insulin continued |
| 2025-07-17 | outside hospital (critical care) | day 10: new fever 38.9 °C (102 °F), tachycardia and hypotension despite initial improvement; exam without a new focal source. WBC 14,000, hemoglobin 10.2, platelets 100,000, creatinine 2.1 (hospital's stated baseline 1.6, R4). Blood cultures drawn; within 48 hours all bottles grow Candida albicans (resulted 07-19). **Candidemia diagnosed** |
| 2025-07-19 to 07-29 | outside hospital | (A5) micafungin from 07-19; central line removed 07-19; repeat blood cultures 07-20 and 07-21 no growth; isolate fluconazole-susceptible; ophthalmology dilated exam 07-22 no candidal eye involvement; transthoracic echo 07-22 normal valves, no vegetation, LVEF 60%; step-down to **fluconazole 200 mg daily** (renally adjusted) 07-25 through 08-03 (14 days after the first negative culture). Discharge 07-29: WBC 8.6, hemoglobin 10.6, platelets 172, sodium 136, creatinine 1.94. Amlodipine resumed at discharge; **lisinopril held** pending kidney recheck |
| 2025-07-31 | practice | nurse transitional-care call (P) |
| 2025-08-05 | practice | post-discharge visit (P): afebrile, finishing fluconazole; hemoglobin 11.5, creatinine 1.86, sodium 139, liver enzymes normal (A6). Lisinopril stays held; recheck in 2 weeks; follow-up chest x-ray ordered (A7). Legionnaires' disease and candidemia marked resolved after fluconazole ends 08-03 |
| 2025-08-19 | practice lab | BMP: creatinine 1.80 (A6); **lisinopril 10 mg daily restarted** by phone 08-20 (A6) |
| 2025-09-09 | outside imaging | chest x-ray 2 views: clear lungs, earlier right-sided infiltrates resolved (A7) |
| 2025-09-23 | practice | follow-up visit, no-show (P) |
| 2025-10-24 | practice | **last seen.** Annual physical and diabetes follow-up (P); influenza and COVID vaccines |
| 2026-01-06 | outside ED, then admitted | two days of fever, productive cough and progressive shortness of breath. Temp 100 °F (37.8 °C), BP 120/80, HR 90, RR 20. Bronchial breath sounds and dullness to percussion over the right lower lung field. Chest x-ray: right lower lobe consolidation. Community-acquired pneumonia of the right lower lobe (R5); sputum and blood cultures and Legionella urine antigen sent; admitted |
| 2026-01-09 | outside hospital | (A8) discharge: treated with ceftriaxone and azithromycin; sputum culture normal respiratory flora, blood cultures no growth, Legionella urine antigen negative; discharged on cefpodoxime 200 mg twice daily and azithromycin to finish 7 days total; asked to see primary care in 1 to 2 weeks. Summary received 2026-01-12 and filed; practice lists unchanged |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source dates the cholecystectomy 2005 against its own 2020 timeline; the
  re-dated timeline moves source years about five and a half years later, so the chart
  says 2010.
- **R2.** The source gives no onset for the chronic conditions. The chart dates them
  approximately on joining: hypertension 2010, type 2 diabetes 2012 (insulin since
  2018), dyslipidemia 2012, chronic kidney disease stage 3b 2022. Diabetes is coded
  "Type 2 diabetes mellitus with diabetic chronic kidney disease" by the usual "with"
  convention; dyslipidemia as Hyperlipidemia; tobacco use disorder as Nicotine
  dependence, cigarettes.
- **R3.** "Neutrophil predominance" is carried as neutrophils 88% on the ED differential. The ED panel's chloride 92 and bicarbonate 23 go with the sodium of 126.
- **R4.** The ICU note gives a baseline creatinine of 1.6, which gives an eGFR near 48
  (stage 3a) at his age, against stage 3b. Resolution: the practice's own creatinine
  runs about 1.78 (eGFR about 42 to 44, stage 3b) before and after the admission; the
  hospital's "baseline 1.6" stays in the hospital's note as the hospital wrote it.
- **R5.** The source's comorbidity "opportunistic pulmonary infection requiring
  diagnostic sampling" is not carried as a label. The January 2026 ED visit is named
  community-acquired pneumonia of the right lower lobe, and the sampling is the sputum
  and blood cultures and urine antigen sent in the ED (results in A8).
- **R6.** The source lists Legionnaires' disease and candidemia on the July 2025 lists
  before their encounters and on every later list. Resolution: Legionnaires' disease
  starts 2025-07-08, candidemia 2025-07-17; both resolve after treatment and stay as
  history. The January 2026 ED record may list them as past history.
- **R7.** The source copies the full home list into the ICU and January ED notes. The
  ICU note shows the home list as held/continued in hospital; the January 2026 records
  copy the practice list (lisinopril back on).
- **Left as the source has it:** he keeps smoking; he declines cessation medication
  each time it is offered and no one presses it. The January 2026 discharge asks for a
  primary-care visit within 1 to 2 weeks; it never happens and no one chases it beyond
  one reply to his message. Atorvastatin stays 40 mg.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2025-03-11 (the source starts at an ED, and any patient needs
  a practice to be ours).
- **A2.** Low-dose CT lung screening for a 62-year-old current 40-pack-year smoker,
  done 2025-04-08: no nodules, Lung-RADS 1, annual repeat advised.
- **A3.** Positive Legionella urine antigen, the evidence for the source's
  Legionnaires' disease.
- **A4.** The hospital course the ICU note implies (catheter, vasopressor, antibiotics):
  norepinephrine day 2 to 4, levofloxacin 14 days, home blood-pressure drugs held.
- **A5.** Standard candidemia care: micafungin, line removal, clearance cultures, eye
  exam, echo, fluconazole step-down 200 mg daily through 2025-08-03, with the discharge
  values listed in the timeline.
- **A6.** Post-discharge kidney and blood count recheck (values in the timeline) and
  lisinopril restarted 2025-08-20 at 10 mg daily once creatinine is back near his
  baseline.
- **A7.** Follow-up chest x-ray about 8 weeks after the pneumonia in a smoker over 50:
  clear.
- **A8.** The January 2026 hospital course and results listed in the timeline, so the
  ED encounter reaches an end.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Levels: height 180 cm; weight 96.0 kg on joining, 91.5 kg after the hospital (August),
  93.8 kg in October. Office BP about 132/80 on his drugs. A1c 7.8 (March), 7.5 (June),
  7.7 (October). LDL about 84 on atorvastatin. Creatinine about 1.78 (R4) outside the
  admission. Urine albumin/creatinine normal, about 14 mg/g. Glucose on panels about 148 (insulin-treated diabetic). PSA about 1.3 (normal). PHQ-2 0 at each screen; AUDIT-C 2.
- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (colonoscopy 2021 normal, repeat in 10 years; Tdap 2019).
- Diabetes and blood pressure visits; A1c, metabolic panel, CBC, lipids, urine
  albumin/creatinine; foot exam with normal monofilament sensation; yearly optometry
  letter without retinopathy; hepatitis C and HIV screening once, nonreactive; PSA once
  after discussion, normal.
- Vaccines: pneumococcal PCV20 and Shingrix dose 1 on 2025-03-11, Shingrix dose 2 on
  2025-06-10, influenza (trivalent) and updated COVID on 2025-10-24.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, the one no-show, reminders after the last visit.
- Nothing booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002279 on 2025-10-24. Practice contacts after that day are
only messages, refills and records filed as received, without changing the practice's
lists. Nothing is booked after today.
