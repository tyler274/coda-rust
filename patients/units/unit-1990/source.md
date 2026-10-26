# synthetic_hospital patient 1990

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-19 (Fri) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-08-12 (Thu) | outpatient | Cardiology |
| 2 | 2021-07-15 | 2022-08-24 (Wed) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2023-08-21 (Mon) | telehealth | Internal Medicine |
| 4 | 2023-01-15 | 2024-02-12 (Mon) | outpatient | Internal Medicine |
| 5 | 2024-01-15 | 2025-02-11 (Tue) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-02-18 (Tue) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-02-12 (Thu) | outpatient | Otolaryngology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Hydrochlorothiazide (cutaneous rash)",
  "Simvastatin (drug-associated myopathy)"
 ],
 "insurance": "Private PPO",
 "occupation": "Business executive",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (40 pack-years), quit 5 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Glipizide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)",
  "Incisional biopsy of tongue lesion (benign result, 1 year prior)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease",
  "Obesity (BMI 32 kg/m\u00b2)"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Poorly controlled diabetes mellitus", "Type 2 diabetes mellitus with established cardiovascular disease and obesity", "Statin-associated muscle symptoms", "ST-segment elevation myocardial infarction, inferior wall", "Oral cavity squamous cell carcinoma", "Stable Angina Pectoris (Medical Management)", "Dressler syndrome", "Hydrochlorothiazide adverse reaction"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Chronic Kidney Disease due to Diabetic Nephropathy"]

## Encounter 0: source 2020-01-15, practice 2021-02-19, outpatient, Endocrinology

Chief complaint: Routine follow-up for diabetes management with recent weight gain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for diabetes management with recent weight gain

HISTORY OF PRESENT ILLNESS:
He reports a recent increase in polyuria and polydipsia. His diabetes has been managed with metformin and a sulfonylurea, but his last HbA1c was 8.2%. He has a BMI of 32 kg/m^2 and expresses a desire to lose weight.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
His other medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker (40 pack-years), quit 5 years ago. Alcohol: None. Occupation: Business executive.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his heart rate is 78 bpm.

LABORATORY DATA:
Recent laboratory results are as follows:

- Fasting plasma glucose: 172 mg/dL
- HbA1c: 8.4%
- Serum creatinine: 1.1 mg/dL
- Urinary albumin-to-creatinine ratio: 45 mg/g
- LDL cholesterol: 98 mg/dL
- HDL cholesterol: 36 mg/dL
- Triglycerides: 210 mg/dL

ASSESSMENT AND PLAN:
Given the patient's history and desire for weight loss,

## Encounter 1: source 2020-07-15, practice 2021-08-12, outpatient, Cardiology

Chief complaint: Chest discomfort during morning walks

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Chest discomfort during morning walks

HISTORY OF PRESENT ILLNESS:
The patient with established cardiovascular disease returns six months after his Endocrinology visit for diabetes management. He describes the discomfort as a pressure-like sensation in the center of his chest that occurs during his morning walks and resolves with rest. He denies symptoms at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with established cardiovascular disease and obesity (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
His father had a myocardial infarction at age 60.

SOCIAL HISTORY:
He has a 10-pack-year smoking history but quit 5 years ago.

VITAL SIGNS:
On physical examination, his blood pressure is 138/86 mmHg, heart rate is 78 bpm, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
His cardiovascular examination is unremarkable, and there are no signs of heart failure.

OTHER STUDIES:
An ECG performed in the office shows no abnormalities.

## Encounter 2: source 2021-07-15, practice 2022-08-24, outpatient, Internal Medicine

Chief complaint: Routine follow-up with increased thirst, frequent urination, and blurry vision

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with increased thirst, frequent urination, and blurry vision

HISTORY OF PRESENT ILLNESS:
The patient returns for routine diabetes follow-up with a history of type 2 diabetes mellitus, coronary artery disease, and obesity, previously evaluated for stable angina and weight management. He reports that he has been feeling generally unwell for the past two months, with increased thirst, frequent urination, and blurry vision. He admits to occasionally missing doses of his medications due to a hectic work schedule.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with established cardiovascular disease and obesity (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Glipizide 10 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker (40 pack-years), quit 5 years ago. Alcohol: None. Occupation: Business executive.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 158/96 mmHg, and his body mass index (BMI) is 32 kg/m^2.

LABORATORY DATA:
Recent laboratory results are as follows:

- Hemoglobin A1c: 9.2%
- Fasting plasma glucose: 198 mg/dL
- Serum creatinine: 1.4 mg/dL
- Urinary albumin-to-creatinine ratio: 300 mg/g

## Encounter 3: source 2022-07-15, practice 2023-08-21, telehealth, Internal Medicine

Chief complaint: Sudden red, itchy rash after starting new blood pressure medication

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: telehealth

CHIEF COMPLAINT:
Sudden red, itchy rash after starting new blood pressure medication

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with type 2 diabetes mellitus, hypertension, hyperlipidemia, coronary artery disease, and obesity presenting via telehealth for evaluation of a new rash. He is followed longitudinally for chronic disease management with recent optimization of his diabetes and cardiovascular care. He states that his home readings have also been consistently elevated. To improve blood pressure control, a second antihypertensive agent is added to his regimen. A few days later, while on vacation in Miami, he calls the clinic reporting the sudden appearance of a red, itchy rash and serous discharge from his nipples. He says the rash is limited to areas exposed to the sun, including his face, neck, forearms, and hands. He denies fever, recent illness, or use of new soaps or foods. He also says he hasn't used sunscreen.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with established cardiovascular disease and obesity (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-07-15)
- Poorly controlled diabetes mellitus (diagnosed 2021-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

He has a history of hypertension and hypercholesterolemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
He currently takes simvastatin and lisinopril.

Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker (40 pack-years), quit 5 years ago. Alcohol: None. Occupation: Business executive.

VITAL SIGNS:
His blood pressure in the office is 160/100 mm Hg

PHYSICAL EXAMINATION:
On video consultation, an erythematous maculopapular rash with mild scaling is noted in sun-exposed areas. The rest of his skin is unaffected.

## Encounter 4: source 2023-01-15, practice 2024-02-12, outpatient, Internal Medicine

Chief complaint: Diffuse muscle discomfort in thighs and shoulders for the past week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Diffuse muscle discomfort in thighs and shoulders for the past week

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of diffuse muscle discomfort in the context of longitudinal management for coronary artery disease and diabetes mellitus, with prior history of medication intolerance. He describes a constant, dull pain in both thighs and shoulders that is aggravated by activity and does not improve with rest. He has no recent history of trauma, infection, or new exercise routines.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with established cardiovascular disease and obesity (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-07-15)
- Poorly controlled diabetes mellitus (diagnosed 2021-07-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2022-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

His past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
His current medications are enalapril, aspirin, and a recently prescribed simvastatin, which was started 2 weeks ago after his last lipid profile review.

Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
He does not use alcohol, illicit drugs, or herbal supplements.

REVIEW OF SYSTEMS:
He denies fever, rash, joint swelling, or sensory changes.

PHYSICAL EXAMINATION:
Physical examination is notable only for mild tenderness to palpation of proximal muscles without swelling, redness, or warmth. Neurologic examination is normal, and there are no focal deficits.

LABORATORY DATA:
Laboratory testing shows elevated creatine kinase levels, with normal renal function and electrolytes.

## Encounter 5: source 2024-01-15, practice 2025-02-11, ed, Emergency Medicine

Chief complaint: Severe crushing chest pain radiating to left arm and jaw

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe crushing chest pain radiating to left arm and jaw

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with coronary artery disease, Type 2 diabetes mellitus, hypertension, hyperlipidemia, and obesity, previously evaluated for exertional chest discomfort in 2020, presents to the emergency department with severe chest pain that started 45 minutes ago while he was mowing the lawn. The pain is described as a heavy pressure in the center of his chest, radiating to his left arm and jaw. He also reports associated shortness of breath, nausea, and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with established cardiovascular disease and obesity (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-07-15)
- Poorly controlled diabetes mellitus (diagnosed 2021-07-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2022-07-15)
- Statin-associated muscle symptoms (diagnosed 2023-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker (40 pack-years), quit 5 years ago. Alcohol: None. Occupation: Business executive.

VITAL SIGNS:
His blood pressure is 150/90 mmHg, heart rate is 98 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On examination, he is diaphoretic, anxious, and in obvious distress. Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation bilaterally.

LABORATORY DATA:
His cardiac biomarkers are as follows:
- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 12 ng/mL (normal 0-5 ng/mL)
- Myoglobin: 90 ng/mL (normal 28-72 ng/mL)

OTHER STUDIES:
The ECG shows ST-segment elevations in leads II, III, and aVF.

## Encounter 6: source 2024-01-15, practice 2025-02-18, ed, Emergency Medicine

Chief complaint: Sharp chest pain worsening with deep breaths and lying flat

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Marcus Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sharp chest pain worsening with deep breaths and lying flat

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with Type 2 diabetes mellitus, hypertension, hyperlipidemia, coronary artery disease, and obesity returns to the emergency department three days following discharge from hospitalization for myocardial infarction. He reports that the pain somewhat improves when he sits up and leans forward.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with established cardiovascular disease and obesity (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-07-15)
- Poorly controlled diabetes mellitus (diagnosed 2021-07-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2022-07-15)
- Statin-associated muscle symptoms (diagnosed 2023-01-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2024-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

His past medical history is significant for hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker (40 pack-years), quit 5 years ago. Alcohol: None. Occupation: Business executive.

VITAL SIGNS:
His blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
On physical examination, you note a friction rub at the left lower sternal border.

OTHER STUDIES:
An ECG is performed and shows diffuse ST-segment elevations.

## Encounter 7: source 2025-01-15, practice 2026-02-12, outpatient, Otolaryngology

Chief complaint: Persistent non-healing sore on tongue for two months

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Singh, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent non-healing sore on tongue for two months

HISTORY OF PRESENT ILLNESS:
Patient with CAD and diabetes presents with persistent oral lesion. He denies any recent trauma to the area. He reports that a similar lesion was biopsied one year ago and was benign.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with established cardiovascular disease and obesity (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-07-15)
- Poorly controlled diabetes mellitus (diagnosed 2021-07-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2022-07-15)
- Statin-associated muscle symptoms (diagnosed 2023-01-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2024-01-15)
- Dressler syndrome (diagnosed 2024-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Obesity (BMI 32 kg/m²)

His medical history is significant for hypertension, controlled with lisinopril, and hyperlipidemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Incisional biopsy of tongue lesion (benign result, 1 year prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly

ALLERGIES:
Hydrochlorothiazide (cutaneous rash), Simvastatin (drug-associated myopathy)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On examination, you note a 1.5 cm white, thickened, non-removable plaque on the lateral border of his tongue. The lesion is irregular in shape and has a slightly erythematous base. There is no cervical lymphadenopathy.
