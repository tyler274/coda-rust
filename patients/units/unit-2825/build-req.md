# Build requirements: unit-2825 (Frances Mayhew, MRN00002825)

Source: synthetic_hospital patient 2825, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plans.

## Who she is (S)

Frances Mayhew, born 1948-02-21, female, White, not Hispanic or Latino, English,
Medicare, lives in her own home at 118 Laurel Ridge Road in the seed's town (01060),
phone +1-978-555-0134, frances.mayhew@example.test. Widowed (A0: the seed gives no
marital status; the source mentions only a daughter and a son). Retired elementary
school teacher. Former smoker, quit about 20 years ago (around 2004); no alcohol.
Allergies: no known drug allergies. Surgical history: no major surgery before the
2025 hip repair (R7). Family history: mother type 2 diabetes and breast cancer
(breast cancer from the GI note, S), father coronary artery disease, sister
Alzheimer's disease. Her daughter is the main contact at visits and becomes health
care proxy; her son also helps (he brings her medication list to the 2025 admission).

Joined the practice on **2020-09-22** (A1: new-patient visit, with her daughter).
She brought: type 2 diabetes, hypertension, COPD, chronic kidney disease stage 3a,
hyperlipidemia, osteoporosis, coronary artery disease and gastroparesis, and these
drugs: metformin 1000 mg twice daily, insulin glargine 24 units at bedtime, lisinopril
20 mg daily, hydrochlorothiazide 25 mg daily, atorvastatin 40 mg daily, aspirin 81 mg
daily, tiotropium 18 mcg inhaled daily, metoclopramide 10 mg three times daily before
meals. Alzheimer's disease is not known at joining (R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-09-22 | practice | new-patient visit (A1); history and drugs as above; daughter mentions mild forgetfulness she puts down to age; height 160 cm, weight about 82 kg |
| 2020-10 | outside | prior-practice records arrive (A2): CAD from an abnormal stress test in 2016, managed medically; spirometry 2018 FEV1/FVC 0.64, FEV1 66% predicted; DXA 2019 lowest T-score -2.6 at the left femoral neck; gastroparesis on a gastric emptying study in 2019, metoclopramide started then; colonoscopy 2016 normal; last Pap 2012 normal; mammogram 2019 normal |
| 2021-11-23 | practice | daughter reports memory clearly worse over about two years: forgets appointments and conversations, more withdrawn, agitated in the evenings, recently says people are stealing from her. Labs (R2): glucose 140, sodium 142, creatinine 1.0, calcium 9.2, TSH 2.5; vitamin B12 normal (P). **Neurology referral** (P) |
| 2021-12-24 | outside | neurology consult letter: alert but apathetic; oriented to person only; MMSE 18/30; neurological exam nonfocal; reviews the PCP labs above. **Alzheimer's disease diagnosed.** **Donepezil 5 mg nightly started** (A3); MRI brain ordered (A4) |
| 2022-01 | outside | MRI brain without contrast (A4): generalized volume loss with disproportionate bilateral hippocampal atrophy; no infarct, mass or hemorrhage |
| 2022-03 | outside | neurology follow-up letter: tolerating donepezil; **donepezil increased to 10 mg nightly** (A3) |
| 2022-05-31 | practice | about a year of worsening pelvic heaviness and a vaginal bulge, weak urine stream and incomplete emptying; no fecal incontinence; A1c 7.8 (R2). **Gynecology referral** (P) |
| 2022-06-28 | outside | gynecology consult letter (age 74): BP 145/90; BMI 32; not sexually active; with straining the cervix and vaginal walls protrude beyond the introitus; cervix atrophic, no ulceration; most recent Pap normal; reviews the A1c 7.8. **Severe symptomatic pelvic organ prolapse diagnosed.** Strongly prefers to avoid surgery because she fears anesthesia and has many conditions; pessary offered (R3) |
| 2022-07 | outside | gynecology: **ring pessary with support fitted** (A5); retained, voiding better; checks and cleaning in the gynecology office about every 4 months after that |
| 2022-11-22 | practice | three months of intermittent dull upper abdominal pain going through to the back; no weight, appetite or bowel change; epigastric tenderness, no jaundice or mass. Hepatic panel normal, amylase normal (R4); **abdominal ultrasound ordered** (R4) |
| 2022-11-29 | practice (imaging) | abdominal ultrasound: 4 cm cystic lesion in the pancreatic head with a thickened, irregular wall and calcifications; no biliary ductal dilatation; main pancreatic duct not seen. **Gastroenterology referral** (P) |
| 2022-12-19 | outside | gastroenterology consult letter: history and exam as above; normal liver tests and amylase; the ultrasound above. **Pancreatic cystic neoplasm with high-risk features diagnosed.** Plan: MRI/MRCP, EUS with fine-needle aspiration, CA 19-9 (R5) |
| 2023-01 | outside | MRI abdomen with MRCP (A6): 4.0 cm cystic lesion of the pancreatic head with a thick enhancing wall and peripheral calcification; main pancreatic duct 4 mm; no solid mass, no biliary dilatation, no liver lesions. CA 19-9 22 U/mL |
| 2023-02 | outside | EUS with FNA (A6): thick-walled cyst in the head as on MRI; viscous fluid; cyst fluid CEA 412 ng/mL; cytology mucinous material, no malignant cells |
| 2023-03 | outside | pancreatic surgery consult (A6): pancreaticoduodenectomy offered given the high-risk features; she and her daughter decline (fear of anesthesia, dementia, comorbidities); surveillance MRI every 6 to 12 months with gastroenterology. Her abdominal pain becomes infrequent over 2023 (A6) |
| 2023-09 | outside | surveillance MRI (A6): lesion unchanged at 4.0 cm; gastroenterology follow-up letter |
| 2023-12-12 | practice | about a month of resting tremor in the hands, stiff limbs, slowness with cooking and buttons; daughter notes a blank stare and less arm swing. **Neurology referral** (P) |
| 2023-12-29 | outside | neurology letter (age 75): expressionless with reduced blink, mildly stooped; cogwheel rigidity both arms, reduced arm swing, 4-Hz resting tremor right hand, slow shuffling gait; strength, reflexes and sensation normal; no falls, hallucinations or mood change; cognition at her baseline per daughter (R6). **Drug-induced parkinsonism due to metoclopramide diagnosed; metoclopramide stopped.** Gastroparesis: small, frequent low-fat meals; no replacement drug. Letter reaches the practice late (2024-01-25) |
| 2024-04 | outside | neurology follow-up letter (A7): tremor and rigidity resolved off metoclopramide; parkinsonism resolved; donepezil 10 mg continued |
| 2024-06 | outside | surveillance MRI (A6): lesion stable at 4.1 cm, no new features; gastroenterology letter: yearly MRI from now |
| 2024-12-29 | outside (ICU) | brought in by her daughter: two days of increasing sleepiness and disorientation, drinking water constantly and urinating hourly; no head trauma, recent illness or alcohol. T 36.5 C, BP 105/68, pulse 112; disoriented to time and place, dry mucous membranes, poor turgor. Na 127, K 3.7, Cl 94, HCO3 22, BUN 26, glucose 790, creatinine 1.6. **Hyperosmolar hyperglycemic state** (age 76): IV fluids and insulin infusion in the ICU; discharged 2025-01-03 on her home regimen unchanged (R8). The discharge summary arrives late (2025-02-10) |
| 2025-01-08 | practice | post-hospital visit, with daughter: back to baseline; creatinine 1.4; from now the daughter gives the bedtime insulin herself |
| 2025-06-23 | outside (admission) | community-acquired pneumonia with acute kidney injury; son brings a medication list but is unsure of doses; she names metformin, lisinopril and a "water pill". BP 145/90, HR 88, RR 22, SpO2 94% on room air; crackles right lower lung field. Creatinine 2.1 (baseline 1.5), BUN 36, potassium 5.2, A1c 7.8. Chest X-ray: right lower lobe infiltrate. IV ceftriaxone and azithromycin (R9); **lisinopril, hydrochlorothiazide and metformin held** (R9); discharged 2025-06-27 on **cefpodoxime** to finish the course. No discharge summary ever reaches the practice; it learns from her son's call and the pharmacy fill (R9) |
| 2025-07-08 | practice | post-hospital visit: cough nearly gone, lungs clear; creatinine back to 1.5, potassium 4.6; **lisinopril, hydrochlorothiazide and metformin resumed** at prior doses (R9); records requested |
| 2025-07-22 | outside | the hospital faxes lab pages and the chest X-ray report only (no summary) |
| 2025-12-25 | outside (ED, admission) | mechanical fall at home, slipped on a wet floor onto her right side; severe right hip pain, cannot bear weight (age 77). Vitals stable. Right leg shortened and externally rotated, tender over the right hip, pain with hip movement, neurovascularly intact. Calcium 9.2, A1c 7.8, vitamin D 20. X-ray: displaced fracture of the right femoral neck (R7). **Right hip hemiarthroplasty 2025-12-26** (A8); discharged 2025-12-30 to a rehabilitation facility on **enoxaparin 40 mg daily for 28 days**, **oxycodone-acetaminophen 5/325 as needed** (short) and **cholecalciferol 2000 units daily** (A8); home 2026-01-16 |
| 2026-01-21 | practice | post-hospital visit: walking with a walker, incision healed; creatinine 1.5, calcium 9.3; osteoporosis treatment discussed (A9) |
| 2026-03-20 | practice | **last visit**: vitamin D 34, calcium 9.4, creatinine 1.5; **denosumab 60 mg subcutaneously every 6 months, first dose given** (A9) |
| 2026-06-13 | outside (ED, admission) | about a week of feeling unwell she put down to her COPD; acute shortness of breath and fatigue, no chest pain. BP 160/90, HR 102, RR 22, SpO2 94% on room air; mild bilateral wheeze; regular rhythm, no murmur, rub or gallop. ECG: ST elevation in II, III and aVF. **Inferior STEMI** (age 78) (A10): high-sensitivity troponin I 1,860 ng/L; emergency catheterization: mid right coronary artery 100% thrombotic occlusion, one drug-eluting stent, TIMI 3 flow; LAD 40%, no other obstructive disease; echo LVEF 50% with inferior hypokinesis. Discharged 2026-06-17 on aspirin 81 mg, **clopidogrel 75 mg daily**, **atorvastatin increased to 80 mg**, **metoprolol succinate 25 mg daily**, other drugs unchanged; cardiac rehabilitation referral. Discharge summary received 2026-06-24 |
| 2026-07-14 | outside | cardiology clinic letter (A10), received 2026-07-21: no angina, breathing back to her usual; BP 132/76, HR 64; continue all |
| 2026-09-08 | practice (booked) | follow-up after the June hospitalization |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists Alzheimer's disease from the start, but neurology diagnoses
  it on 2021-12-24. Resolution: it is not on any list before that day; the 2020 and
  early-2021 notes record only the daughter's sense of mild forgetfulness.
- **R2.** The neurology note's labs (glucose 140, sodium 142, creatinine 1.0, calcium
  9.2, TSH 2.5) and the gynecology note's A1c 7.8 are carried as practice draws the
  consult letters cite (2021-11-23 and 2022-05-31). The gynecology letter's own BP
  145/90 and BMI 32 are measured there.
- **R3.** The gynecology plan stops at her wish to avoid surgery. Resolution: a
  pessary is the conservative choice (A5); no surgery is ever done.
- **R4.** The GI note's "normal liver function tests and amylase" and ultrasound are
  drawn and ordered by the practice in November 2022 before the referral. The GI
  plan, cut off mid-sentence, becomes the workup in R5 and A6.
- **R5.** Source years follow the re-dated timeline: "neurology in January 2020" is
  December 2021, "prolapse in July 2020" is June 2022, "parkinsonism in 2022" is
  December 2023. Ages follow the birth date (74 at gynecology, not 73).
- **R6.** The 2023 neurology note says she denies cognitive changes, calls mental status
  unremarkable, and says there is no personal or family history of neurodegenerative
  disease. Resolution: those lines are not carried; the letter says cognition is at
  her known baseline, and the family history stays as listed (sister with
  Alzheimer's). Its "simvastatin" is dropped: atorvastatin throughout. The 2026 ED
  note's "simvastatin" is likewise dropped.
- **R7.** The diagnosis list says "left femoral neck", but the history, exam and X-ray
  all describe the right hip. Resolution: right femoral neck, displaced, repaired by
  right hemiarthroplasty (A8). The 2026 ED note's "no prior major surgeries" is a stale
  copied history, as outside histories are.
- **R8.** The ICU note lists the full home regimen, and every later list keeps glargine
  24 units and metformin 1000 mg twice daily. Resolution: the regimen is unchanged
  after HHS. A1c drifts up to about 8.4 to 8.6 in the half-year before HHS (the ICU
  note calls her diabetes poorly controlled) and settles to about 7.8 once her
  daughter gives the insulin herself in 2025, matching the 7.8 of the June and December
  2025 hospital records.
- **R9.** The 2025 admission says only "appropriate antibiotics". Resolution: IV
  ceftriaxone and azithromycin in hospital, oral cefpodoxime 200 mg twice daily to
  finish. With creatinine 2.1 and potassium 5.2, lisinopril, hydrochlorothiazide and
  metformin are held in hospital and resumed at the 2025-07-08 visit, so later lists
  carry them as the source does. Creatinine runs about 1.0 to 1.1 from 2020 to 2024
  (CKD 3a), about 1.4 to 1.5 after the HHS admission (the "baseline 1.5"; CKD 3b
  recorded in April 2025).
- **R10.** Metoclopramide is stopped by neurology on 2023-12-29, yet the 2024, 2025 and
  2026 hospital notes still list it. Resolution: those are stale outside lists; the
  practice list drops it when the practice learns of the stop (2024-01-18).
- **Left as the source has it:** metformin stays 1000 mg twice daily with eGFR in the
  mid-30s from 2025; nobody adjusts it or comments. Osteoporosis is on her list from
  joining with no treatment and no repeat bone density until the hip fracture. No
  diabetes regimen change after HHS. No replacement drug for gastroparesis. The
  November 2022 visit says nothing about pancreatic cancer risk before the ultrasound.

## Additions (A): physician to confirm

- **A0.** Marital status widowed (a required identity field; nothing in the source).
- **A1.** 2020-09-22 new-patient visit (six years with the practice; the source opens
  with established conditions and drugs).
- **A2.** Prior-practice records supporting the listed CAD, COPD, osteoporosis and
  gastroparesis, with routine screening done there (numbers as in the timeline).
- **A3.** Donepezil 5 mg then 10 mg nightly: standard treatment for the diagnosed
  Alzheimer's disease, run by neurology.
- **A4.** One MRI brain supporting the diagnosis (findings as in the timeline).
- **A5.** Ring pessary with support, fitted and followed by gynecology.
- **A6.** The pancreatic cyst workup and surveillance: MRI/MRCP, CA 19-9 22 U/mL, EUS
  with cyst fluid CEA 412 ng/mL and cytology without malignant cells, surgery offered
  and declined, MRIs stable at 4.0 cm (2023-09) and 4.1 cm (2024-06), then yearly. The
  2025 yearly MRI is not done before the June admission and has not been rescheduled
  by the last visit. Pain infrequent from 2023.
- **A7.** Neurology follow-up confirming resolution off metoclopramide.
- **A8.** Right hip hemiarthroplasty and its discharge drugs (enoxaparin 40 mg daily
  for 28 days, oxycodone 5 mg/acetaminophen 325 mg 1 tablet every 6 hours as needed,
  short; cholecalciferol 2000 units daily, ongoing); rehabilitation stay 2025-12-30 to
  2026-01-16. A rehab discharge note arrives 2026-01-20. Home PT after that; an orthopedic
  follow-up in February 2026 sends no letter (the daughter reports it went well).
- **A9.** After a hip fracture with osteoporosis and CKD 3b, denosumab rather than a
  bisphosphonate: discussed 2026-01-21, first dose 2026-03-20 with vitamin D 34 and
  calcium 9.4.
- **A10.** The STEMI's usual hospital course and first cardiology follow-up (numbers as
  in the timeline). These arrive after the last visit: they are filed as received and
  the practice's lists are not updated.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Chronic-care visits about every 3 to 5 months, mostly with her daughter, who sets out
  her pills weekly from early 2022 and takes the car keys in late 2021; yearly annual
  wellness visits with PHQ-2/PHQ-9 (negative), fall-risk and advance care planning
  (daughter named health care proxy in 2023). Weight about 80 to 83 kg; BP mostly 134
  to 146 over 78 to 88 on her regimen.
- Labs: A1c every 3 to 6 months (about 7.5 to 7.9 through 2023, as in R8 after); BMP
  or CMP each time (creatinine as in R9, potassium 4.2 to 4.8); lipids yearly on
  atorvastatin 40 (LDL about 75 to 90); urine microalbumin/creatinine yearly, normal;
  CBC yearly, normal. Hepatitis C antibody once, negative (2020).
- Yearly diabetic eye exam letters from an optometrist: no retinopathy. Foot exams with
  intact monofilament sensation.
- Prevention: screening mammograms 2020 and 2022, normal, then stopped by age; no further
  colorectal or cervical screening; no lung screening (quit more than 15 years ago).
  Tdap 2020; high-dose influenza each fall; Shingrix 2021 (two doses); PCV20 2021; COVID
  primary series 2021 at a pharmacy, then yearly updated doses; RSV 2023.
- Refills, pharmacy and prior-authorization traffic (denosumab), portal messages from
  the daughter about results, refills and scheduling, one missed visit (2024-07).
- Booked visit after 2026-08-27: follow-up after the June hospitalization (2026-09-08).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002825 on 2026-03-20. Practice contacts after that day are
only messages, refills and records; outside care after it still happens, and its records
are filed as received without changing the practice's lists. A visit is booked after today.
