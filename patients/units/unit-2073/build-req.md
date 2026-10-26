# Build requirements: unit-2073 (Rohan Chowdhury, MRN00002073)

Source: synthetic_hospital patient 2073, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its quiz-style phrasing. Brand names are not needed:
the chart says pancrelipase.

## Who he is (S)

Rohan Chowdhury, born 1972-04-30, male, Asian (South Asian), English-speaking, Medicaid,
lives in South Hadley MA 01075 with his brother's family (R8), single. Construction
laborer. Immigrated to the US in 2019 (the source's "5 years ago" at the 2024 admission);
the chart says only that he moved to the US then, without naming the country.

- Tobacco: former smoker, 20 pack-years, quit in January 2022 (R1). At joining he still smoked.
- Alcohol: severe alcohol use disorder, about 6 beers daily (at least 72 g ethanol) for
  20 years, several earlier withdrawal episodes (before he joined; not documented
  anywhere the practice can see), and sober stretches of 1 to 3 months at a time.
- Allergies: no known drug allergies.
- Surgical history: none.
- Family history: father alcohol use disorder and cirrhosis; mother type 2 diabetes;
  brother hypertension.

Joined the practice 2021-09-14 (new patient visit, A1). He brought severe alcohol use
disorder and no medications. Alcoholic liver disease is established in his first month
(A2). Everything else starts at its encounter below.

## Timeline (dates fixed; S unless marked)

Records texture (seed: gaps): some outside letters arrive late or not at all; the
practice learns of care from the patient, a family call or a pharmacy prior
authorization; the problem and medication lists lag the outside events by days to weeks.

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new patient visit (A1). Weight 61.0 kg, height 168 cm, BP normal. Drinking about 6 beers daily; current smoker. Labs: AST 68, ALT 47, GGT 290, alkaline phosphatase 104, total bilirubin 1.0, albumin 3.9, hemoglobin 13.6, MCV 99, platelets 178; hepatitis C antibody nonreactive; hepatitis B surface antigen and surface antibody both negative. **Thiamine 100 mg daily, folic acid 1 mg daily and a multivitamin daily started** (A4). RUQ ultrasound ordered |
| 2021-09-28 | outside imaging | RUQ ultrasound (A2): diffusely increased liver echogenicity consistent with steatosis, normal liver size, no focal lesion, normal gallbladder and ducts, spleen normal size, no ascites, patent portal vein. **Alcoholic liver disease** added to the problem list |
| 2022-01 | patient report | quits smoking (R1); recorded as former smoker from the March 2022 visit |
| 2022-12-13 | practice | about 6 months of weight loss and chronic diarrhea with bulky, greasy, foul-smelling stools and epigastric pain going through to the back; sober since early December 2022 after a stretch of daily drinking. Weight 55.0 kg. **Gastroenterology referral** (A3) |
| 2023-01-05 | outside, GI | GI consult (enc 0): history as above; sober for the last month. Cachectic, mildly jaundiced, epigastric tenderness without rebound or guarding, normal bowel sounds. Amylase 160 U/L, lipase 300 U/L, fecal elastase <100 mcg/g, total cholesterol 145, triglycerides 88. **Chronic pancreatitis with exocrine pancreatic insufficiency** and **protein-calorie malnutrition** diagnosed; **pancrelipase 36,000 lipase units with meals and 18,000 with snacks started** (R2). Records: the practice learns of the start from the Medicaid prior authorization request on 2023-01-10; the GI letter arrives 2023-02-16 and the diagnoses reach the problem list then |
| 2023-05-16 | practice | relapse to daily drinking in March and April, stopped again about two weeks ago (R3); balance getting worse since about January, holds furniture at home. **Neurology referral** (A5) |
| 2023-07-06 | outside, neurology | neurology consult (enc 1; date moved, R4): 6 months of progressive gait difficulty and imbalance, holds furniture moving around the house; no weakness, numbness or limb pain; sober about 2 months. Wide-based unsteady gait, poor tandem walking, finger-to-nose accurate with eyes closed, normal speech, no nystagmus, strength, sensation and reflexes normal. Labs from the neurology draw: CBC normal, AST and ALT mildly elevated, B12 normal, TSH normal, serum thiamine low. **Alcoholic cerebellar degeneration** and **thiamine deficiency** diagnosed; continue thiamine. Letter arrives 2023-07-20 |
| 2024-01-15 | outside, neurology | neurology follow-up (enc 2): gait worse over 6 months with frequent falls, no head injury or loss of consciousness; family says he is more forgetful and struggles with tasks needing coordination. Wide-based unsteady gait, dysdiadochokinesia, intention tremor reaching for objects, reduced tone in all four limbs, normal reflexes, sensation intact. AST 72, ALT 55, alkaline phosphatase 130, GGT 450. No plan beyond abstinence (R5). Records: the letter is not sent; the patient describes the visit on 2024-02-20 and the practice requests it; it arrives 2024-04-03 |
| 2024-09-07 to 2024-09-16 | outside, hospital | admitted through the ED (enc 3, infectious disease consult 2024-09-08): deep constant mid-back pain worse at night, no help from over-the-counter analgesics, night sweats, fatigue, 6-kg weight loss over two months; had not been in to see anyone for months (R6); no known HIV. T 37.8 C, BP 124/70, pulse 88, RR 16. Moderate tenderness over T9–T10, no visible deformity, normal neurological exam. WBC 7.8, hemoglobin 10.6, ESR 96, CRP 8.1 mg/dL. Chest x-ray apical scarring. MRI spine: anterior vertebral body destruction at T9 and T10 with disc collapse and a prevertebral fluid collection. CT-guided aspiration of the collection 2024-09-10 (A6): AFB smear positive, tuberculosis NAAT positive with rifampin resistance not detected. HIV antigen/antibody nonreactive (A6). **Tuberculous spondylitis (Pott disease)** and **anemia of chronic disease** diagnosed. **Isoniazid 300 mg, rifampin 600 mg, pyrazinamide 1000 mg, ethambutol 800 mg and pyridoxine 50 mg daily started 2024-09-11**, directly observed by the health department TB clinic (A7). Discharge summary arrives 2024-09-19 |
| 2024-10-24 | outside, TB clinic | TB clinic letter (A6): culture final grows tuberculosis complex, susceptible to isoniazid, rifampin, pyrazinamide and ethambutol; on DOT, tolerating |
| 2024-11-12 | outside, TB clinic | **pyrazinamide and ethambutol stopped** after the 2-month intensive phase; isoniazid, rifampin and pyridoxine continue (A7). The practice learns of it at the 2024-12-10 visit; the letter arrives 2024-12-18 |
| 2024-12-10 | practice | follow-up: back pain easing, weight 55.0 kg; ESR 41, hemoglobin 11.6 (A7) |
| 2025-03-11 | outside, TB clinic | **isoniazid, rifampin and pyridoxine stopped**, 6 months completed (A7). End-of-treatment MRI thoracic spine 2025-03-04: interval healing at T9–T10, prevertebral collection resolved, no new bone destruction. Completion letter arrives 2025-03-20 |
| 2025-03-25 | practice | ESR 18, hemoglobin 12.9; anemia of chronic disease resolved; Pott disease resolved, treatment completed (A7) |
| 2025-05-10 to 2025-05-15 | outside, ED then hospital | ED (enc 4): 2 weeks of worsening abdominal pain, jaundice and poor appetite; drinking about six beers a day again (R3). Age 53 (R7). T 38.3 C, icteric sclerae, tender enlarged liver. WBC 12.0, hemoglobin 13.2, platelets 95, AST 180, ALT 60, total bilirubin 6.2, direct bilirubin 4.1, alkaline phosphatase 120, INR 1.6. **Alcoholic hepatitis** diagnosed. Admitted (A8): blood cultures no growth, urinalysis negative, chest x-ray clear; RUQ ultrasound enlarged echogenic liver, no biliary dilation, patent portal vein, no ascites, normal spleen; supportive care, nutrition, abstinence. Discharge bilirubin 4.8 (direct 3.1), INR 1.4. Discharge summary arrives 2025-05-19 |
| 2025-05-20 | practice | post-discharge visit (A8): bilirubin 3.9 (direct 2.4), INR 1.3 |
| 2025-06-17 | practice | labs (A8): bilirubin 1.8 (direct 0.9), INR 1.2, platelets 128 |
| 2025-08-12 | practice | follow-up (A8): bilirubin 1.1 (direct 0.4), AST 61, ALT 39, platelets 141; alcoholic hepatitis resolved |
| 2025-11-02 to 2025-11-04 | outside, ED and observation | ED (enc 5), brought by family: confusion and agitation, last drank heavily 2 days before. BP 160/100, HR 120, RR 22, T 38.2 C. Tremulous, diaphoretic, anxious, oriented to person only, no focal deficits. Na 140, K 4.0, Cl 100, BUN 18, creatinine 1.0, AST 72, ALT 50, GGT 300, MCV 96. ECG sinus tachycardia, otherwise normal. **Alcohol withdrawal** (coded uncomplicated, R9). Symptom-triggered lorazepam and IV thiamine in observation; clear by the next morning (A9). The practice learns of it from his brother's call 2025-11-03; the ED record arrives 2025-11-10 |
| 2025-11-13 | practice | post-ED visit (P) |
| 2026-01-11 to 2026-01-17 | outside, ED then ICU | ED then ICU (enc 6), about two months after the November visit: 12 hours of rising anxiety, hand tremor and drenching sweats; stopped drinking abruptly, last drink 36 hours before arrival; had been drinking about a pint of vodka daily in the months before (R10). BP 160/100, HR 120, RR 22, T 37.8 C. Agitated, diaphoretic, tremulous, disoriented to time and place, no focal deficits. Na 140, K 4.0, Cl 100, BUN 18, creatinine 1.1, glucose 90, magnesium 1.8, calcium 9.2, AST 72, ALT 50, alkaline phosphatase 90, CBC with platelets normal. **Delirium tremens**; ICU care with benzodiazepines, phenobarbital and IV thiamine, no intubation (A10). Records: no summary sent; the patient tells the practice at the 2026-01-27 visit; records requested; a brief ICU discharge summary with these labs arrives 2026-03-09 |
| 2026-07-14 to 2026-07-20 | outside, hospital | admitted for **community-acquired pneumonia** (enc 7), last drink about 24 hours before admission; IV ceftriaxone and oral azithromycin. Hospital day 2 (2026-07-15): increasingly agitated and tremulous, feels jittery; no headache, chest pain or shortness of breath. T 37.8 C, BP 160/100, pulse 110, RR 18. Diaphoretic, coarse tremor of outstretched hands, anxious. Electrolytes, renal function and liver enzymes normal; blood alcohol undetectable. **Alcohol withdrawal seizures** that day (A11): two brief generalized seizures, IV lorazepam, CT head without acute abnormality; no antiseizure drug at discharge; antibiotics completed (5 days) in hospital. Discharge summary arrives 2026-07-23 and recommends a follow-up chest x-ray in 6 to 8 weeks |
| 2026-07-28 | practice | post-discharge visit (P): **follow-up chest x-ray ordered, not yet done today** |
| 2026-09-15 | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** "Former smoker, quit 1 year ago" repeats unchanged from 2023 to 2026. Resolution:
  he quit in January 2022, a year before the first source encounter; every later note says
  former smoker, 20 pack-years, without restating "1 year ago".
- **R2.** The profile lists pancrelipase, thiamine, folic acid and a multivitamin before
  the GI visit that diagnoses the pancreatitis. Resolution: thiamine, folic acid and the
  multivitamin start at joining (A4); pancrelipase starts at the GI consult. The 18,000-unit
  snack dose is dispensed as three 6,000-unit capsules and the meal dose as one 36,000-unit
  capsule (two orders).
- **R3.** "Sober for the last month" in January 2023, "sober for the past two months" in July
  2023, and six beers a day in May 2025. Resolution: sober from early December 2022, relapse in
  March and April 2023, sober again from early May 2023 through the July neurology visit, then
  intermittent daily drinking returns in 2024 and continues around each later admission.
- **R4.** Encounter 1's practice date 2023-07-04 is Independence Day. Resolution: the
  neurology visit is 2023-07-06 (Thursday).
- **R5.** The 2024 neurology note stops at its labs and gives no plan. Resolution: the letter
  advises abstinence and continuing thiamine. The "forgetful" report gets no workup or
  diagnosis.
- **R6.** "Has not sought medical care recently" at the TB admission, though he is a practice
  patient. Resolution: his last practice contact before it was the February 2024 visit; he
  no-showed a May 2024 visit and did not reschedule.
- **R7.** "52-year-old" in May 2025: by his birth date he is 53. The chart says 53.
- **R8.** "His family" (2024 neurology, 2025 ED) is his brother, with whom he lives. No other
  family member is added.
- **R9.** The ED labels the November 2025 episode uncomplicated withdrawal though he is
  disoriented and febrile. Resolution: the ED's label is kept as written (no seizures, no
  hallucinations, cleared within a day); the physician reading the chart can weigh it. The
  practice problem list carries "Alcohol withdrawal, uncomplicated", resolved at the
  2025-11-13 visit.
- **R10.** January 2026 says "a decade of approximately one pint of vodka daily" against the
  profile's 6 beers daily for 20 years, and "two days ago" against "last drink 36 hours ago".
  Resolution: beer is his long-term pattern; in the months before January 2026 he was
  drinking about a pint of vodka daily; "a decade" is not carried; the last drink was 36 hours
  before arrival.
- **R11.** "Alcohol use disorder" appears twice (chronic condition and comorbidity), and
  "Alcoholic cerebellar degeneration" and "Chronic pancreatitis" twice each. One problem each.
- **R12.** Source TSH, B12, CBC and enzyme results given only as "normal", "mildly elevated" or
  "low" are carried as words in the letters that report them, with no numbers.
- **Left as the source has it:** no medication for alcohol use disorder is ever prescribed;
  the PCP offers counseling and outpatient addiction program information, which he does not
  take up, and no note comments on the absence of a drug. No pancreatic imaging is done for
  the chronic pancreatitis. No brain imaging or physical therapy follows the cerebellar
  findings or the falls. The alcoholic hepatitis summary does not discuss corticosteroids.

## Additions (A): physician to confirm

- **A1.** The 2021-09-14 new patient visit (five years with the practice; the first source
  encounter already carries his problems), with the joining labs given above.
- **A2.** One RUQ ultrasound (findings above) supporting the source's alcoholic liver disease.
- **A3.** The December 2022 visit that refers him to gastroenterology.
- **A4.** Thiamine 100 mg, folic acid 1 mg and a multivitamin daily from joining (the source
  lists them as home medications from the first encounter).
- **A5.** The May 2023 visit that refers him to neurology.
- **A6.** Confirmation of the tuberculous spondylitis: CT-guided aspiration with AFB smear,
  NAAT and pan-susceptible culture; HIV test nonreactive.
- **A7.** Standard treatment for drug-susceptible spinal TB, run by the health department TB
  clinic by DOT: 2 months of isoniazid, rifampin, pyrazinamide and ethambutol (doses above,
  weight about 52 kg), then 4 months of isoniazid and rifampin, with pyridoxine throughout;
  6 months total, completed 2025-03-11; end MRI, ESR and hemoglobin as above. Anemia of chronic
  disease resolves with it.
- **A8.** Admission and recovery course of the alcoholic hepatitis, numbers as above.
- **A9.** Observation course for the November 2025 withdrawal.
- **A10.** ICU course for delirium tremens.
- **A11.** Seizure details, CT head and the follow-up chest x-ray recommendation for the July
  2026 admission.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention. An
  abnormal finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries, but nothing that adds a territory, a severity grade or
  a symptom, or changes the diagnosis or plan.
- Weight: 61 kg at joining, about 59 in mid-2022, 55 in December 2022, about 53 to 54 at the
  GI visit, back to about 57 by late 2023 on pancrelipase, about 58 in mid-2024, 52 at the TB
  admission, about 55 to 56 in 2025, about 54 after each 2026 admission. Height 168 cm.
- Visits every 2 to 4 months for alcohol use, liver, pancreas and nutrition, with some
  no-shows; CMP or hepatic panel, CBC, INR, magnesium and GGT a few times a year; AUDIT-C and
  PHQ-9 screens (PHQ-9 low); counseling about drinking each visit. Platelets run about 150 to
  185 outside the hepatitis episode. Albumin about 3.4 to 3.9. Prothrombin time follows each
  INR given above (about 18.6 s at INR 1.6, 16.4 at 1.4, then normalizing).
- Prevention: yearly FIT (negative in 2021, 2023 and 2025; the 2022 and 2024 kits never
  returned); hepatitis B vaccine series in 2022 (liver disease, non-immune); PCV20 once; Tdap;
  Shingrix two doses; influenza each fall; COVID updated vaccines. Lung cancer screening
  offered in 2023 and deferred by the patient; never done.
- Refills, Medicaid prior authorization for pancrelipase, pharmacy calls, portal messages
  (some from his brother on his behalf), records requests, missed visits.
- Booked visit after 2026-08-27: 2026-09-15 follow-up; the chest x-ray order stays open.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002073 on 2026-07-28. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
