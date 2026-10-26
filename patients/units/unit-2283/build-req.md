# Build requirements: unit-2283 (Vernon Simard, MRN00002283)

Source: synthetic_hospital patient 2283, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("is performed and shown above").

Records are **sparse** (seed chart_texture): most outside care reaches the practice as
the patient's or his wife's account, a short letter or ED note, or a pharmacy fill,
with few numbers. Each outside contact below says what reaches the chart.

## Who he is (S)

Vernon Simard, born 1956-01-19, male, White, not Hispanic or Latino, English, lives in
Leeds MA 01053, Medicare and Medicaid. Married; his wife comes to visits and calls
for him (the source's "his wife notes"). Retired long-haul truck driver.

- **Tobacco:** about 1 pack a day for about 40 years (40 pack-years); current smoker
  when he joins; quits in late May 2022 (R2).
- **Alcohol:** about 6 standard drinks a day for about 30 years; still drinking when he
  joins; stops in early February 2021 and stays abstinent through today (R2).
- **Allergies:** no known drug allergies.
- **Surgical history:** distal pancreatectomy with splenectomy for pancreatic
  adenocarcinoma of the tail (2013, before joining, R1); percutaneous coronary
  intervention with a drug-eluting stent (2016, before joining, R6); esophagoscopy with
  diverticulotomy for Zenker diverticulum (2022-03-24, R3); completion total
  pancreatectomy (2023-01-03, R1/A4).
- **Family history:** father with alcohol use disorder and liver cirrhosis; mother with
  type 2 diabetes; older brother with coronary artery disease and a myocardial
  infarction at 62.

**Joins the practice 2019-06-11** (A1, new-patient visit; the first internal-medicine
source encounter is a return visit). He brings: alcoholic cirrhosis of the liver,
compensated; chronic pancreatitis due to alcohol; coronary artery disease with a
stent; benign prostatic hyperplasia; history of pancreatic adenocarcinoma after distal
pancreatectomy and splenectomy (so asplenic); alcohol use disorder (active); daily
cigarette smoking. No diabetes, no Zenker diverticulum yet.
Medications on joining: tamsulosin 0.4 mg daily; aspirin 81 mg daily; atorvastatin
40 mg daily; pancrelipase 36,000 lipase units with meals; multivitamin 1 tablet daily;
lactulose 15 mL twice daily as needed for constipation.

Height 175 cm. Weight 82 kg on joining (see the weight course in A13).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-06-11 | practice | **establish care** (A1): history above; AUDIT-C high; counseled on alcohol and tobacco, not ready to quit either; prior records requested; baseline labs (P) |
| 2019-07 | outside records | prior-practice records arrive (P): 2013 distal pancreatectomy with splenectomy for tail adenocarcinoma, oncology surveillance completed 2018 with no recurrence; 2016 PCI with drug-eluting stent; post-splenectomy vaccines given in 2013 (PCV13, PPSV23, MenACWY, Hib); colonoscopy 2018 normal |
| 2019 to 2021 | practice | routine care of cirrhosis, pancreatitis, CAD, BPH, alcohol and tobacco (P); yearly low-dose CT lung screening (A2); liver ultrasound surveillance (P) |
| 2021-02 | practice | reports he **stopped drinking in early February 2021** (R2); stays abstinent; **alcohol use disorder in remission** recorded from mid-2021 |
| 2021-08 | (history) | trouble swallowing begins (about 6 months before the GI visit) |
| 2022-01-18 | practice | about 5 months of progressive trouble swallowing solids and liquids, coughing at meals, bad breath, regurgitating undigested food at night; wife hears him wake choking; neck fullness that gurgles when pressed; about 5 kg weight loss; referred to GI (P) |
| 2022-02-25 | outside GI | consult: dysphagia to solids and liquids for 6 months, cough with meals, halitosis, nocturnal regurgitation of undigested food, wife reports choking at night, 5 kg weight loss, gurgling neck fullness; exam unremarkable; barium esophagram: posterior outpouching just above the upper esophageal sphincter. **Zenker diverticulum diagnosed.** Plan: endoscopic diverticulotomy (R3). A short GI letter reaches the practice |
| 2022-03-24 | outside GI | **flexible esophagoscopy with diverticulotomy** (R3); the practice learns of it from the patient; swallowing better afterward |
| 2022-05-16 | practice | returns about 3 months after the Zenker diagnosis: fatigue, poor appetite, **20 lb weight loss over 2 months**, new jaundice (wife: skin more pale yellow over the past week), occasional nausea, vague upper abdominal discomfort not changed by food or bowel movements; no fever, vomiting or change in bowel habits. Smokes 1 pack a day for 40 years: **tobacco use disorder (nicotine dependence, cigarettes) diagnosed**; declines cessation medicine. Alcohol: abstinent since February 2021 (R2). Exam: mild scleral icterus, epigastric fullness without tenderness or a palpable mass. Labs: mild cholestatic liver enzyme elevation (A13 levels). **Unintentional weight loss** recorded. CT ordered |
| 2022-05-19 | practice-ordered CT | CT abdomen and pelvis with contrast: **mass in the head of the (remnant) pancreas**, 2.6 x 2.2 cm, with mild biliary duct dilation; postsurgical absence of the pancreatic tail and spleen; liver with nodular contour of the known cirrhosis and no focal lesion; no ascites; no enlarged lymph nodes; normal aortic caliber (A3). **Bile duct obstruction** recorded. Urgent GI referral for EUS |
| 2022-05-27 | outside GI | **EUS-guided biopsy of the pancreatic head mass**: gland-forming malignant cells with mucin production, pancreatic adenocarcinoma. The pathology report reaches the practice 2022-06-01: **adenocarcinoma of the head of the pancreas diagnosed**; oncology referral |
| late May 2022 | (history) | **quits smoking** after the diagnosis (R2) |
| 2022-08-03 | practice | follow-up: still losing weight; fasting labs: **fasting glucose 176 mg/dL, HbA1c 7.8%**; bilirubin and alkaline phosphatase higher than in May (A13) |
| 2022-08-05 | practice (phone) | **type 2 diabetes mellitus diagnosed**; **insulin glargine 10 units at bedtime started** (A5); glucose meter and supplies |
| 2022-08-18 | outside oncology | follow-up (he had a first oncology visit in June, which sent nothing): persistent fatigue, **15 lb weight loss over 3 months**, recent new type 2 diabetes; no change in diet or exercise. Former smoker, 40 pack-years; former heavy drinker, abstinent since early 2021. Exam: jaundiced, mild epigastric tenderness. Fasting glucose 176, HbA1c 7.8% (the practice's values). Liver tests: elevated bilirubin and alkaline phosphatase. Abdominal ultrasound: dilated common bile duct, no gallstones. Coagulation tests consistent with **chronic disseminated intravascular coagulation** (A6). Assessment: **mucin-producing pancreatic adenocarcinoma with chronic DIC**. Plan: biliary stent, then chemotherapy before surgery (A4). A short oncology letter reaches the practice |
| 2022-08-25 | outside GI | **ERCP with biliary stent** (A4); patient's account only |
| 2022-09 to 2022-11 | outside oncology | **neoadjuvant gemcitabine and nab-paclitaxel** (A4), infused at the cancer center; patient's account only |
| 2022-12 | outside oncology | restaging CT: tumor smaller, no spread (A4); patient's account only |
| 2023-01-03 | outside surgery | **completion total pancreatectomy** for the head tumor (A4), hospital stay to 2023-01-13; discharged on **insulin glargine 24 units at bedtime and insulin lispro sliding scale with meals** (glargine 10 units stopped) (A5); pancrelipase unchanged. Short discharge summary reaches the practice. **Bile duct obstruction resolved** |
| 2023-01-19 | practice | post-discharge visit (P): healing midline incision, no jaundice; weight 60 kg |
| 2023-02-13 | outside endocrinology | about 6 weeks after surgery: shakiness, anxiety and palpitations several times a week for a month, a few hours after meals, relieved by eating; wife sees him sweaty, irritable and confused, better quickly with juice or a snack; on daily insulin injections; hungrier between meals; **lost 5 lb**; adherent. Denies fever, chest pain or recent illness. Exam unremarkable except a healing midline abdominal scar; no focal neurologic deficit; heart, lungs and abdomen normal. **Post-pancreatectomy hypoglycemia due to glucagon deficiency diagnosed.** Plan (A7): smaller, more frequent meals with protein; lower lispro correction scale; glucose tablets; **glucagon nasal powder** for rescue; glargine 24 units continued. A short letter reaches the practice |
| 2023-03 | practice | post-operative labs: DIC tests normalized, platelets back to normal (A6): **DIC resolved** |
| 2023-09 | practice | weight regained to about 61 kg: **unintentional weight loss resolved** (P) |
| 2023 to 2026 | outside oncology | surveillance after surgery, no recurrence (A8); patient's account only |
| 2024-02-12 | practice (portal) | wife writes that he is more tired and bruising easily; GI visit already booked (P) |
| 2024-02-20 | outside GI | fatigue and increased bruising. Takes a daily multivitamin, no anticoagulant (aspirin continues). Exam: scleral icterus, distended abdomen with shifting dullness (**ascites**), spider angiomata on the chest, multiple ecchymoses on arms and legs, no jugular venous distension, normal neurologic exam and mental status (R4: no spleen). Labs ordered. Assessment: **decompensation of the alcoholic cirrhosis** with ascites and coagulopathy (R5). Results (A9). Plan: 2 g sodium diet, continued abstinence (left as the source has it: no diuretic). A short GI letter reaches the practice |
| 2024-03 to 2024-12 | practice | **recurrent urinary symptoms (dysuria, frequency, low back pain) treated three times with short antibiotic courses** (A10), each improving for a while; referred to urology in December |
| 2025-02-19 | outside urology | 3 months of dysuria, frequency and low back pain; similar episodes several times over the past year, each better for a while with antibiotics; no fever or chills. Afebrile, vitals normal. DRE: tender, boggy prostate. Urinalysis: pyuria and bacteriuria. Urine culture: a gram-negative rod. **Chronic bacterial prostatitis diagnosed.** Plan: **ciprofloxacin 500 mg twice daily for 4 weeks** (A11). A short letter reaches the practice |
| 2025-08-23 | outside ED (Saturday) | 2 days of fever, chills, painful urination, perineal and low back pain. Temperature 38.7 C (101.7 F). DRE: markedly tender, swollen prostate. WBC 14,000/uL. Urinalysis: pyuria, bacteriuria. Urine culture pending (never reaches the practice). **Acute bacterial prostatitis.** Treated (A12): ceftriaxone in the ED, discharged on **trimethoprim-sulfamethoxazole DS twice daily for 4 weeks**. A short ED note reaches the practice |
| 2025-08-26 | practice | post-ED visit (P): afebrile, improving |
| 2026-02-19 | outside ICU | **found unresponsive at home** (by his wife), intubated for airway protection, IV fluids and broad-spectrum antibiotics for **presumed aspiration pneumonia**; low-dose vasopressors; heart rate in the 90s; afebrile. Over 10 days the infection improves but he stays lethargic and ventilator dependent. Nutrition consult: mildly underweight. Encephalopathy workup: low serum albumin; low total T3, normal T4, normal TSH. Exam: minimal spontaneous movement, eyes open only to deep sternal rub, no response to voice, withdraws from pain, pupils equal and reactive, slow deep tendon reflexes, no goiter, exophthalmos or tremor. No pituitary or thyroid history. **Euthyroid sick syndrome.** Course after the source (A14). Cause of being found down not established. A short discharge summary reaches the practice |
| 2026-03-12 | outside | discharged from the hospital to a skilled nursing facility (A14) |
| 2026-04-02 | (history) | home from the skilled nursing facility (A14) |
| 2026-04 | practice | transitional care visit: weak, mildly underweight, eating better; repeat thyroid tests normal later in April (A14): **euthyroid sick syndrome resolved** |
| 2026-05 to 2026-08 | practice | recovery follow-up; weight creeping up (A13) |
| 2026-09-22 | practice | **booked** routine follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1. Two pancreatic cancers.** The profile lists "history of pancreatic
  adenocarcinoma, status post partial pancreatectomy" and a distal pancreatectomy with
  splenectomy from the first encounter, and the May 2022 note already speaks of a prior
  pancreatectomy, yet in May 2022 a new adenocarcinoma is found in the pancreatic head,
  and in February 2023 he is six weeks out from "a surgical procedure" with a midline
  scar and glucagon-deficiency hypoglycemia. A distal pancreatectomy cannot remove a
  head tumor. Resolution: the first cancer was in the pancreatic tail, treated by distal
  pancreatectomy with splenectomy in 2013, with surveillance completed in 2018 (history
  at joining). The 2022 head tumor is a new adenocarcinoma in the remnant, treated with
  completion total pancreatectomy on 2023-01-03 (A4), which explains the glucagon
  deficiency. The source's "Pancreatic adenocarcinoma (mucin-producing) with chronic
  DIC (diagnosed at oncology)" is the head tumor; the chart records the tumor when the
  biopsy result arrives (2022-06-01) and DIC at the oncology letter.
- **R2. Tobacco and alcohol.** GI (Feb 2022) says he denies smoking and alcohol; the May
  2022 practice note says 1 pack a day for 40 years and social weekend drinking; every
  later note says former smoker of 40 pack-years who quit "about a year ago" and former
  heavy drinker abstinent "18 months". Resolution: he smoked about 1 pack a day until
  late May 2022 and quit after the cancer diagnosis (40 pack-years); tobacco use
  disorder is diagnosed 2022-05-16 and resolved in the fall of 2022, then recorded as
  history of tobacco dependence. He drank about 6 drinks a day until early February
  2021 (18 months before the August 2022 oncology visit) and has been abstinent since.
  The GI letter records no current alcohol and leaves smoking out; the "social weekend
  drinking" is not carried; "quit about a year ago" and "18 months" are not repeated
  in later notes, which give the quit dates instead.
- **R3. Zenker diverticulum.** The profile lists the Zenker diverticulum and its
  diverticulotomy from the start, but GI diagnoses it in February 2022. Resolution:
  diagnosed 2022-02-25; flexible endoscopic diverticulotomy 2022-03-24. Zenker
  diverticulum stays on the problem list afterward, as the source keeps it.
- **R4. Spleen.** The 2024 GI exam finds a spleen palpable below the left costal margin
  in a man whose spleen was removed in 2013. Resolution: the spleen finding is not
  carried; the GI letter and the ultrasound note the absent spleen.
- **R5. Cirrhosis.** The profile and early notes carry alcoholic cirrhosis from the start,
  the GI encounter of 2024 "diagnoses" alcoholic cirrhosis of liver, and that encounter
  says he had not seen a physician in several years. Resolution: one condition,
  alcoholic cirrhosis of the liver, known since before he joined and compensated until
  2024; the February 2024 GI visit documents its decompensation (ascites, jaundice,
  spider angiomata, bruising). The "several years without a physician" line is not
  carried.
- **R6. Coronary procedure.** The profile gives a PCI with drug-eluting stent; the 2026
  ICU note says coronary artery bypass grafting 3 years earlier. Resolution: PCI with a
  drug-eluting stent in 2016, before joining; no bypass surgery is carried.
- **R7. Diabetes.** The profile lists type 2 diabetes and insulin from the start, but the
  August 2022 oncology visit calls the diabetes a recent new diagnosis. Resolution:
  diabetes is diagnosed 2022-08-05 from the practice's fasting glucose 176 and HbA1c
  7.8%; insulin starts then (A5). The source's label "type 2 diabetes mellitus" is
  kept after the total pancreatectomy.
- **R8. Weight.** 5 kg lost before February 2022, 20 lb over the 2 months to May 2022,
  15 lb over the 3 months to August 2022 and 5 lb more by February 2023 are carried as
  consecutive losses (A13 gives the levels).
- **R9. Lists.** Outside letters from 2023 on copy the source's full home list (glargine 24 units,
  lispro scale, tamsulosin, aspirin, atorvastatin, pancrelipase, multivitamin,
  lactulose). The practice list follows the course here: no insulin before
  2022-08-05, glargine 10 units until 2023-01-13, then glargine 24 units with lispro.
  The 2024 GI letter's list omits the multivitamin in its list while mentioning it in
  the text; the practice list keeps it.
- **R10. Quiz and unfinished text.** "A barium esophagram is performed and shown above"
  becomes the esophagram finding in the GI letter. The 2024 GI plan ("laboratory
  testing is ordered") is completed by A9. The 2026 note's "no evidence of pituitary
  abnormality" is carried as "no known pituitary or thyroid disease".
- **Left as the source has it:** no diuretic, paracentesis or variceal screening after
  ascites appears in 2024; aspirin continues through the bruising and the cirrhosis;
  pancrelipase stays at 36,000 lipase units after the total pancreatectomy; glargine
  stays 24 units after the hypoglycemia diagnosis; the practice treats the 2024
  urinary episodes without urine cultures; the ED culture result never reaches the
  practice; the cause of the February 2026 unresponsive episode is never established.
  No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2019-06-11 new-patient visit (the first internal-medicine source visit is a
  return visit; seed: 7 years with the practice).
- **A2.** Low-dose CT lung cancer screening in 2019, 2020 and 2021 (current smoker, 40
  pack-years): Lung-RADS 1 each year (no nodules). Stopped once
  oncology CT surveillance began in 2022.
- **A3.** The 2022-05-19 CT report (the source's "abdominal CT ... mass in the head of the
  pancreas causing mild biliary duct dilation"), ordered by the practice, with the mass
  measured at 2.6 x 2.2 cm and ordinary normal findings otherwise.
- **A4.** Treatment of the head tumor (R1): ERCP with a plastic biliary stent
  2022-08-25; neoadjuvant gemcitabine and nab-paclitaxel September to November 2022;
  restaging CT December 2022 with a smaller tumor and no spread; completion total
  pancreatectomy 2023-01-03, discharged 2023-01-13. The practice knows the chemotherapy
  and restaging only from the patient; the discharge summary is short.
- **A5.** Insulin: glargine 10 units at bedtime from 2022-08-05; at surgical discharge
  2023-01-13, glargine 24 units at bedtime plus lispro sliding scale with meals (the
  source's home regimen).
- **A6.** Chronic DIC evidence at oncology, August 2022: platelets low (about 115), low
  fibrinogen, raised D-dimer, INR about 1.4; the oncology letter states these in words.
  After surgery (March 2023 practice draw) platelets about 230 and INR about 1.2; DIC
  resolved.
- **A7.** Endocrinology plan for the hypoglycemia: meal pattern, lower lispro scale,
  glucose tablets, glucagon 3 mg nasal powder as needed for severe hypoglycemia.
- **A8.** Oncology surveillance after surgery with no recurrence through today (patient's
  account).
- **A9.** Results of the 2024 GI workup, also seen on the practice draw of 2024-03:
  total bilirubin about 3.1, albumin about 2.8, INR about 1.7, platelets about 132,
  sodium about 133, AST about 60, ALT about 35, alkaline phosphatase about 160,
  creatinine normal. Abdominal ultrasound (GI, 2024-02): nodular cirrhotic liver, no
  focal lesion, ascites, no biliary dilation, spleen surgically absent. Ascites is no
  longer evident on exam from November 2024 on, with the sodium restriction and
  continued abstinence.
- **A10.** The "several times over the past year ... improved with antibiotics" episodes:
  2024-04-09 office visit (dipstick leukocyte esterase and nitrite positive,
  trimethoprim-sulfamethoxazole DS twice daily for 7 days); 2024-07-23 telephone
  (same 7-day course); 2024-10-29 office visit (ciprofloxacin 500 mg twice daily for
  7 days); 2024-12-10 office visit (symptoms back since mid-November; urology referral).
  Each episode recorded as a urinary tract infection and resolved.
- **A11.** Ciprofloxacin 500 mg twice daily for 4 weeks for the chronic bacterial
  prostatitis (urology, 2025-02-19).
- **A12.** ED treatment of the acute prostatitis: ceftriaxone 1 g IV once; discharged on
  trimethoprim-sulfamethoxazole DS twice daily for 4 weeks.
- **A13.** Levels. Weight: 82 kg (2019 to mid-2021); 77 kg (Feb 2022); 68 kg (May
  2022); 61.5 kg (Aug 2022); 60 kg (Jan 2023); 57.7 kg (Feb 2023); about 61 kg by late
  2023; about 63 kg with the ascites (Feb to Oct 2024); about 62 kg in 2025; 55.5 kg
  after the 2026 admission (BMI about 18, the "mildly underweight"); about 57 kg by
  August 2026. Liver: 2019 to early 2021 while drinking, AST about 68, ALT about 40,
  GGT about 210, bilirubin about 1.2, albumin about 3.7, platelets about 205, INR about
  1.1; after he stops drinking, AST about 42, ALT about 30, GGT about 70. May 2022:
  alkaline phosphatase about 290, total bilirubin about 3.0, direct about 2.2, AST
  about 62, ALT about 66. August 2022: alkaline phosphatase about 340, bilirubin about
  3.6, platelets about 118, INR about 1.4. After surgery (2023): alkaline phosphatase
  about 130, bilirubin about 1.1, albumin about 3.4, AST about 40, ALT about 30. May 2022 albumin about 3.6.
  Random glucose about 165 from the diabetes diagnosis on. 2025: bilirubin about 2.2, albumin
  about 3.1, INR about 1.5, platelets about 145. After the 2026 admission: albumin
  about 2.7, bilirubin about 2.0, rising to albumin about 3.0 by August 2026. HbA1c 7.8%
  at diagnosis; about 7.4 in late 2022; 7.2 to 7.6 through 2023 to 2025; about 7.8 in
  2026. LDL about 75 on atorvastatin 40 mg. Urine microalbumin/creatinine normal (about 12 mg/g).
  Creatinine normal throughout. Blood
  pressure normal throughout (about 112/68).
- **A14.** After the source ends (2026-02-19 admission): the encephalopathy clears over
  the following week; extubated 2026-03-03; out of the ICU 2026-03-05; discharged to a
  skilled nursing facility 2026-03-12 on his home regimen; home 2026-04-02. Repeat
  thyroid tests on 2026-04-28 are normal (TSH about 1.8, free T4 about 1.1, free T3
  about 3.0), and euthyroid sick
  syndrome is resolved. Aspiration pneumonia and the encephalopathy are recorded at the
  discharge summary and resolved at the transitional care visit.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart sounds, lungs, no edema). An abnormal finding appears only as the source or
  this file gives it. A report may carry the descriptors its kind always carries (how a
  size was measured, a benign nodule's size and lobe within its Lung-RADS category, a
  procedure's access site), but nothing that adds a territory, a severity grade or a
  symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (the 2018 colonoscopy).
- Visits for cirrhosis, pancreatitis, CAD, BPH and, from August 2022, diabetes about
  every 3 to 6 months, with CMP, CBC, INR, HbA1c, lipids yearly, urine
  microalbumin/creatinine yearly from 2023, foot exams. Medicare annual wellness visits.
  A resting ECG at joining, normal sinus rhythm without acute change.
- Liver ultrasound surveillance for the cirrhosis about yearly (2019, 2020, 2021, 2025,
  2026): nodular liver, no focal lesion; no ascites except as A9 gives it.
- Yearly diabetic eye exam letters from an optometrist from 2023: no retinopathy.
- Prevention: hepatitis C, hepatitis B surface antigen and HIV screening once (2019,
  negative); hepatitis A and B vaccine series (Twinrix) 2019; MenACWY booster 2019 and
  2024 and MenB series 2019 (asplenia); Tdap 2019; Shingrix series 2020; influenza
  each fall (high-dose from age 65); COVID vaccines from 2021; RSV vaccine 2024.
  Depression screens (PHQ-2/PHQ-9) with low scores; AUDIT-C high while drinking, 0
  after.
- Refills, pharmacy and prior-authorization traffic (insulin, pancrelipase, pen
  needles, glucagon), portal messages from him or his wife about results, refills and
  scheduling, a missed visit, records requests, home health after the SNF stay.
- Booked visit after 2026-08-27: routine follow-up 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
