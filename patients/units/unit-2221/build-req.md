# Build requirements: unit-2221 (Thomas Casey, MRN00002221)

Source: synthetic_hospital patient 2221, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps or its quiz framing ("you note ...").

## Who he is (S)

Thomas Casey, born 1953-01-09, male, White, not Hispanic or Latino, English-speaking,
Medicare with supplemental Plan G. Retired high school mathematics teacher. Never
smoker. Alcohol occasional, 1 to 2 glasses of wine a week. No known drug allergies.
Surgical history: tonsillectomy at age 10 (1963); screening colonoscopy at age 63
(2016), normal. Family history: father myocardial infarction at 67; mother type 2
diabetes and primary open-angle glaucoma; sister hypertension.

Joined the practice in September 2020 (A1: new-patient visit 2020-09-10). He brought:
type 2 diabetes, hypertension, hyperlipidemia, and primary open-angle glaucoma followed
by an outside ophthalmologist; metformin 1000 mg twice daily, lisinopril 20 mg daily,
hydrochlorothiazide 12.5 mg daily, atorvastatin 40 mg daily, latanoprost 1 drop both
eyes at bedtime. Kidney function normal for age at joining (creatinine about 1.1).
Not known at joining: kidney disease, nevi, pancreatic cancer, bullous pemphigoid,
diverticulitis, coronary disease (each starts at its encounter below).

Records: gaps (seed). Some outside letters arrive late or only after a request; one
never arrives and the practice learns of it from the patient; a list lags behind.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-09-10 | practice | new-patient visit (A1); history and medications as above; height 176 cm, weight about 86 kg; BP normal on his regimen; labs: CMP, CBC, lipids, A1c 6.9 |
| 2021-06-25 | outside, ophthalmology | glaucoma follow-up letter: trouble with night driving and sometimes bumping into things at the edge of his vision; no eye pain or redness; visual acuity 20/25 both eyes; IOP 22 mmHg right, 24 mmHg left; cup-to-disc ratio 0.7 both eyes; visual fields show bilateral peripheral loss. Plan: continue latanoprost, drop technique and adherence reviewed, repeat fields (R3). Family history in the letter: mother with glaucoma (R2) |
| 2021-08-17 | practice | diabetes follow-up; mentions a slowly enlarging spot on his back noticed some months ago; **creatinine 1.2** (S: "1.2 six months ago"); referred to dermatology (A2) |
| 2021-09-13 | outside, dermatology | consult letter: lesion on upper back noticed several months ago, slowly growing, no pain, itching or bleeding; no weight loss, fevers or night sweats; well-demarcated round tan lesion, stuck-on appearance, 1.5 cm, slightly rough surface; no other lesion like it on full skin exam; multiple benign-appearing melanocytic nevi elsewhere. Benign, reassurance, no biopsy, return if it changes (R4). **Multiple melanocytic nevi** diagnosed. Letter arrives late, 2021-11-30, after a records request |
| 2022-02-15 | practice | diabetes follow-up: a few months of more frequent urination and thirst; otherwise well; takes medications regularly; follows a strict diet; BP 132/86; exam unremarkable. Labs drawn 2022-02-08: fasting glucose 146, **A1c 7.2**, **creatinine 1.8** (up from 1.2 six months earlier), urine albumin/creatinine 300 mg/g. **Chronic kidney disease due to diabetic nephropathy, stage 3b** diagnosed (R1); repeat in 3 months. No medication change (see Left as the source has it) |
| 2022-05-17 | practice | kidney recheck: creatinine 1.7 to 1.8 range, urine albumin/creatinine 280; stage 3b confirmed (P) |
| 2022-09-20 | practice | about 2 months of nausea, vomiting after meals, early satiety; weight down about 7 lb; home sugars running high; urgent GI referral (A2) |
| 2022-10-17 | outside, gastroenterology | consult letter: 3 months of progressive nausea, vomiting of undigested food after meals, early satiety, 10 lb weight loss; blood sugars unusually high for a few weeks without diet or drug change; on metformin; mild epigastric tenderness and a palpable upper abdominal mass; hemoglobin 12.1, WBC 7.0, platelets 250, fasting glucose 180, CA 19-9 120 U/mL; upper GI series with delayed gastric emptying and narrowing at the distal stomach. Suspected pancreatic malignancy; CT and endoscopic ultrasound arranged (R5, A3) |
| 2022-10-26 | outside, imaging | CT abdomen and pelvis with contrast, pancreas protocol, IV hydration given for his kidney function (A3): 3.4 cm hypoattenuating mass in the pancreatic head compressing the duodenum with gastric outlet narrowing; no arterial contact; less than 180 degrees contact with the superior mesenteric vein; no liver lesions, no ascites, no enlarged nodes. Resectable |
| 2022-11-03 | outside, gastroenterology | endoscopic ultrasound with fine-needle aspiration of the pancreatic head mass (A3): adenocarcinoma. **Pancreatic carcinoma (adenocarcinoma of the pancreatic head)** diagnosed |
| 2022-11-10 | practice | PCP visit to go over the diagnosis and surgical plan (P) |
| 2022-12-06 to 2022-12-14 | outside, surgical oncology admission | pancreaticoduodenectomy (A3); pathology: 3.3 cm moderately differentiated ductal adenocarcinoma, margins negative, 0 of 21 lymph nodes, pT2 N0; uneventful recovery; **pancrelipase with meals started** (A3); metformin continued. Discharge summary received 2022-12-19 |
| 2022-12-21 | practice | post-discharge visit (P) |
| 2023-01-17 | outside, medical oncology | consult letter: CA 19-9 after surgery 28; **adjuvant gemcitabine** every 4 weeks (days 1, 8, 15) for 6 cycles, dose adjusted for kidney function (A3), 2023-01-24 to 2023-07-05 |
| 2023-07-18 | outside, medical oncology | end-of-treatment letter: CT chest and MRI abdomen with no evidence of recurrence; CA 19-9 19; **gemcitabine completed**; surveillance every 6 months (A3) |
| 2023-08-23 | outside, dermatology | two months of itchy red raised patches on the trunk (he thought an allergy) then over two weeks large tense blisters on the abdomen, arms and thighs; no new drugs, soaps or detergents; multiple 1 to 3 cm tense bullae on a red base, some with urticarial plaques around them; no mucosal lesions; skin biopsy with direct immunofluorescence: linear IgG and C3 along the basement membrane zone. **Bullous pemphigoid** diagnosed; **clobetasol 0.05% ointment** to affected skin twice daily, taper as it settles (A4). Patient messages the practice 2023-08-25; letter arrives late, 2023-10-03 |
| 2023-12 (letter never sent) | outside, dermatology | follow-up: the practice learns from the patient at 2024-02-20 that the blisters have cleared and he has tapered off clobetasol (A4) |
| 2024-01-16 | outside, medical oncology | surveillance: no evidence of recurrence, CA 19-9 17 (A3) |
| 2024-06-20 | outside, emergency department | three days of worsening constant dull left lower quadrant pain, low-grade fever, nausea, poor appetite, loose stools two days; no vomiting, blood in stool or black stool; prior milder episodes that settled on their own; temperature 38.2 C (100.8 F), BP 128/74, pulse 94; LLQ tenderness without rebound or guarding; bowel sounds slightly decreased; rectal exam unremarkable; leukocytosis; urinalysis negative for infection and blood; CT abdomen with contrast: pericolic fat stranding and localized wall thickening of the sigmoid colon without abscess or free air. **Acute diverticulitis of sigmoid colon**; discharged home on **amoxicillin-clavulanate 875/125 mg twice daily for 10 days** (A5). Normal colonoscopy noted as 2016 (R6). Practice learns from the patient's call 2024-06-21; ED record arrives 2024-07-15 after a request |
| 2024-06-27 | practice | post-ED visit; improving; colonoscopy referral after the episode (A5) |
| 2024-08-13 | outside, medical oncology | surveillance: no evidence of recurrence, CA 19-9 21 (A3) |
| 2024-08-29 | outside, gastroenterology | colonoscopy (A5): sigmoid diverticulosis; no mass, no polyps; no repeat needed for screening at his age unless symptoms. Diverticulitis resolved |
| 2025-02-11 | outside, medical oncology | surveillance: no evidence of recurrence, CA 19-9 16 (A3) |
| 2025-06-13 to 2025-06-17 | outside, ED and cardiology admission | chest pain to the left arm with shortness of breath and sweating starting 2 hours before arrival; BP 160/90, HR 102, RR 22; baseline creatinine 1.8, eGFR 40; ECG ST elevation in II, III and aVF. **ST-elevation myocardial infarction, inferior**; urgent coronary angiography: occluded right coronary artery, **one drug-eluting stent** (A6). After the contrast, creatinine 2.6 on day 2 and 2.2 at discharge: **contrast-induced nephropathy** (S). Echocardiogram: ejection fraction 50%, inferior wall hypokinesis (A6). Metformin held 48 hours then resumed. **Aspirin 81 mg daily, clopidogrel 75 mg daily, metoprolol succinate 25 mg daily started** (A6); atorvastatin left at 40 mg (see Left as the source has it). Glucose high on arrival is read as his diabetes under stress, not a separate diagnosis (R8). Cardiac rehabilitation referral. Patient calls 2025-06-18; discharge summary arrives late, 2025-07-09 |
| 2025-06-24 | practice | post-hospital visit (P); creatinine 2.0; coronary artery disease with stent and contrast-induced nephropathy added to the list; new cardiac drugs added |
| 2025-08-12 | practice | follow-up; creatinine back to 1.8; contrast-induced nephropathy resolved (A6) |
| 2025-08-26 | outside, cardiology | follow-up letter: no angina, doing cardiac rehab, continue current regimen (A6) |
| 2025-09-09 | outside, medical oncology | surveillance: no evidence of recurrence, CA 19-9 20 (A3) |
| 2025-12-03 | practice | **last practice visit**: diabetes and kidney follow-up; stable |
| 2026-03-10 | outside, medical oncology | surveillance: no evidence of recurrence, CA 19-9 18 (A3); letter never sent (records gap) |
| 2026-06-16 to 2026-06-19 | outside, ED and admission | sudden severe shortness of breath and cough with pink frothy sputum while watching television; RR 30, BP 180/100, HR 110, SpO2 85% on room air; rales halfway up both lung fields, raised jugular venous pressure, 2+ pitting edema both legs; creatinine 1.8 (R7); BNP markedly elevated (no number); chest x-ray: pulmonary edema with cephalization and septal lines; ECG ST elevations in II, III and aVF, unchanged from his 2025 post-infarction tracing (R9). **Acute pulmonary edema** with severe hypertension; treated with nitroglycerin, IV furosemide and noninvasive ventilation; repeat echocardiogram ejection fraction 50%, unchanged; **furosemide 20 mg daily added at discharge** (A7). Patient messages 2026-06-22; discharge summary arrives 2026-07-21 after a request. Filed as received; practice lists not updated (after last seen) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists "CKD stage 3 secondary to diabetic nephropathy" from the first
  encounter, but the internal medicine encounter gives creatinine 1.2 six months earlier
  and diagnoses the nephropathy there. Resolution: kidney function is normal for age at
  joining (creatinine about 1.1, 1.2 in August 2021), and chronic kidney disease due to
  diabetic nephropathy, stage 3b (creatinine 1.8, eGFR about 39, albumin/creatinine 300),
  is diagnosed 2022-02-15. The profile's two CKD labels are one condition. No
  albumin/creatinine ratio is measured before 2022.
- **R2.** The ophthalmology encounter names a brother with glaucoma; every other family
  history has no brother and gives glaucoma to the mother. Resolution: the brother is not
  carried; the mother's glaucoma is.
- **R3.** Glaucoma is stamped "diagnosed" at the 2021 ophthalmology visit, but that visit is
  a routine follow-up on latanoprost. Resolution: glaucoma was diagnosed by ophthalmology
  before he joined and he brought it; the 2021 letter records the findings above. The
  source gives the visit no plan; the letter continues latanoprost and repeats fields.
- **R4.** The dermatology encounter describes a stuck-on lesion with no others like it, yet
  diagnoses multiple melanocytic nevi. Resolution: the letter carries the back lesion as
  examined, calls it benign without biopsy, and records multiple benign-appearing nevi
  elsewhere on full skin exam. No other name is given to the back lesion. The profile's
  "multiple benign melanocytic nevi" and the "multiple melanocytic nevi" diagnosis are one
  condition, diagnosed 2021-09-13.
- **R5.** The source gives a pancreatic carcinoma with a palpable mass and gastric outlet
  narrowing, then four more years of life with no mention of treatment. Resolution: a
  resectable pancreatic head adenocarcinoma, removed with clear margins and treated with
  adjuvant chemotherapy, then surveillance without recurrence (A3). Later outside lists
  carry it as a history.
- **R6.** Colonoscopy "at age 63" (2016) and "a normal colonoscopy 5 years ago" at the 2024
  ED visit disagree. Resolution: one normal colonoscopy in 2016; the ED record says 2016.
- **R7.** The 2026 ED gives creatinine 1.2, against a stable baseline of 1.8 (eGFR 40) in
  2025 and stage 3b disease since 2022. Resolution: the 1.2 is not carried; the 2026
  ED record reports creatinine 1.8.
- **R8.** "Stress hyperglycemia" is listed as a comorbidity in a known diabetic. Resolution:
  not a separate diagnosis; the 2025 admission notes a high glucose on arrival.
- **R9.** The 2026 ED ECG repeats the inferior ST elevations of 2025 with no plan (the
  source note stops after the ECG). Resolution: the ED reads them as unchanged from his
  2025 post-infarction tracing; no new infarction, no catheterization.
- **R10.** The surgical history lists "cardiac catheterization with drug-eluting stent"
  from 2021, years before the 2025 infarction that calls for it, with no antiplatelet drug
  anywhere. Resolution: the catheterization and stent are the 2025 STEMI procedure; earlier
  records never mention them. Atorvastatin 40, metformin, lisinopril and
  hydrochlorothiazide are on every list throughout, whatever drugs a given source note
  names in its prose.
- **Left as the source has it:** metformin stays at 1000 mg twice daily with an eGFR near
  40; no SGLT2 inhibitor is started for diabetic kidney disease with heavy albuminuria;
  atorvastatin stays 40 mg after the infarction; the glaucoma regimen stays latanoprost
  alone despite pressures of 22 and 24 with field loss. No note comments on these.

## Additions (A): physician to confirm

- **A1.** 2020-09-10 new-patient visit (the first source encounter is a routine follow-up;
  seed says about 6 years with the practice).
- **A2.** Referral visits: 2021-08-17 to dermatology for the back lesion; 2022-09-20 urgent
  GI referral for vomiting and weight loss.
- **A3.** The pancreatic cancer course (R5): CT 2022-10-26 and EUS-FNA 2022-11-03 as in the
  timeline; pancreaticoduodenectomy 2022-12-06 with the pathology given; pancrelipase
  (lipase 24,000 units) 2 capsules with meals and 1 with snacks from discharge onward;
  adjuvant gemcitabine for 6 cycles 2023-01-24 to 2023-07-05; surveillance every 6 months
  with CT chest and MRI abdomen without recurrence and CA 19-9 of 28, 19, 17, 21, 16, 20, 18
  on the dates above.
- **A4.** Bullous pemphigoid treatment: clobetasol 0.05% ointment twice daily, tapered to
  off by early 2024 with the skin clear.
- **A5.** Diverticulitis care: amoxicillin-clavulanate 875/125 mg twice daily for 10 days
  from the ED; colonoscopy 2024-08-29 showing sigmoid diverticulosis without mass or
  polyps; diverticulitis resolved.
- **A6.** STEMI course: right coronary artery occlusion, one drug-eluting stent; creatinine
  2.6 day 2, 2.2 at discharge, 2.0 on 2025-06-24, 1.8 on 2025-08-12 (contrast-induced
  nephropathy resolved); echocardiogram ejection fraction 50% with inferior hypokinesis;
  aspirin 81 mg, clopidogrel 75 mg and metoprolol succinate 25 mg daily; metformin held 48
  hours after contrast; cardiac rehabilitation; cardiology follow-up 2025-08-26 without
  angina.
- **A7.** 2026 admission course: nitroglycerin, IV furosemide, noninvasive ventilation;
  repeat echocardiogram ejection fraction 50%; discharged 2026-06-19 on furosemide 20 mg
  daily with cardiology follow-up. Appears only in the discharge summary and a pharmacy
  request, not on the practice's lists.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from his prior practice: tetanus-diphtheria-pertussis 2017, pneumococcal
  conjugate 2018 and polysaccharide 2019, the 2016 colonoscopy; hepatitis C screening done
  once at the practice (negative).
- Diabetes, blood pressure and kidney visits every 3 to 6 months through 2025-12-03, with
  vitals, A1c, metabolic panel, lipids yearly, albumin/creatinine yearly from 2022, CBC,
  foot exams; annual wellness visits; depression screens negative.
- Yearly ophthalmology glaucoma letters with dilated exams and no diabetic retinopathy.
- Influenza each fall, COVID vaccines, shingles series 2021, RSV 2023.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders, records requests.
- No visit is booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002221 on 2025-12-03. Practice contacts after that day are
only messages, refills and records filed as received; outside care after it still happens,
and its records do not change the practice's lists. Nothing is booked.
