# synthetic_hospital patient 2039

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-08 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-01-05 (Wed) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2022-07-01 (Fri) | outpatient | Pulmonology |
| 3 | 2021-07-15 | 2023-01-03 (Tue) | outpatient | Endocrinology |
| 4 | 2022-07-15 | 2024-01-08 (Mon) | outpatient | Cardiology |
| 5 | 2023-07-15 | 2025-01-06 (Mon) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2026-01-09 (Fri) | follow_up | Cardiology |
| 7 | 2025-01-15 | 2026-07-11 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Simvastatin (interstitial lung disease)",
  "Niacin (severe flushing and pruritus)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Agricultural equipment operator",
 "alcohol_use": "Occasional social use",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 Diabetes Mellitus",
  "Brother: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day (30 pack-year history)",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "75 mg daily",
   "name": "Clopidogrel"
  },
  {
   "dose": "40 mg nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "20 mg daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)",
  "Colonoscopy (age 54, normal)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Coronary Artery Disease",
  "Gastroesophageal Reflux Disease",
  "Diabetic Autonomic Neuropathy"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Right Ventricular Myocardial Infarction", "Gastroesophageal reflux disease", "Organophosphate poisoning", "Hyperlipidemia", "Statin-induced interstitial lung disease", "Stable Angina Pectoris (Medical Management)", "Diabetic autonomic neuropathy", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall"]
Comorbidities: ["Status post percutaneous coronary intervention", "Type 2 diabetes mellitus", "Tobacco use disorder", "Alcohol use", "Hyperlipidemia", "Vagal bradycardia secondary to pneumoperitoneum", "Coronary Artery Disease (Antiplatelet Therapy)", "Hypotension"]

## Encounter 0: source 2020-01-15, practice 2021-07-08, outpatient, Internal Medicine

Chief complaint: Routine follow-up appointment for hyperlipidemia management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up appointment for hyperlipidemia management

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease
- Gastroesophageal Reflux Disease
- Diabetic Autonomic Neuropathy

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Colonoscopy (age 54, normal)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Clopidogrel 75 mg daily
- Atorvastatin 40 mg nightly
- Metoprolol succinate 50 mg daily
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-year history). Alcohol: Occasional social use. Occupation: Agricultural equipment operator.

LABORATORY DATA:
Fasting LDL cholesterol measurements collected annually over the past seven years: 112 mg/dL, 129 mg/dL, 136 mg/dL, 129 mg/dL, 144 mg/dL, 129 mg/dL, and 136 mg/dL

## Encounter 1: source 2020-07-15, practice 2022-01-05, outpatient, Gastroenterology

Chief complaint: Six-month history of heartburn and regurgitation occurring mainly at night

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of heartburn and regurgitation occurring mainly at night

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male with a history of gastroesophageal reflux disease and diabetic autonomic neuropathy who presents to the gastroenterology clinic six months following his prior internal medicine appointment. He reports a 6-month history of heartburn and regurgitation that occurs mainly at night. He states that his symptoms have progressively worsened despite over-the-counter antacid use.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Gastroesophageal Reflux Disease
- Diabetic Autonomic Neuropathy

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Colonoscopy (age 54, normal)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Clopidogrel 75 mg daily
- Atorvastatin 40 mg nightly
- Metoprolol succinate 50 mg daily
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
He has a 30-pack-year smoking history and drinks alcohol socially.

REVIEW OF SYSTEMS:
He denies dysphagia, odynophagia, weight loss, anemia, GI bleeding, or recurrent vomiting.

VITAL SIGNS:
His body mass index is 29 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
His complete blood count, basic metabolic panel, and liver function tests are within normal limits.

ASSESSMENT AND PLAN:
Given the patient's chronic symptoms and risk factors,

## Encounter 2: source 2021-01-15, practice 2022-07-01, outpatient, Pulmonology

Chief complaint: Episodes of facial flushing and itching occurring 30 minutes after taking medication

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Johnson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of facial flushing and itching occurring 30 minutes after taking medication

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with coronary artery disease, type 2 diabetes mellitus, and gastroesophageal reflux disease who was last evaluated by Gastroenterology six months prior for nocturnal reflux symptoms and by Internal Medicine one year prior for hyperlipidemia management. He reports that these symptoms have been occurring daily for the past two weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Diabetic Autonomic Neuropathy

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Colonoscopy (age 54, normal)

MEDICATIONS:
His current medications include simvastatin, metformin, and a new medication that was added recently to improve his lipid profile.

Home Medications:
- Aspirin 81 mg daily
- Clopidogrel 75 mg daily
- Atorvastatin 40 mg nightly
- Metoprolol succinate 50 mg daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-year history). Alcohol: Occasional social use. Occupation: Agricultural equipment operator.

PHYSICAL EXAMINATION:
On physical examination, his skin is warm and flushed, particularly over the cheeks and neck.

LABORATORY DATA:
His fasting lipid panel from one week ago showed:

- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 3: source 2021-07-15, practice 2023-01-03, outpatient, Endocrinology

Chief complaint: Four months of progressively infrequent, difficult bowel movements with bloating

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Four months of progressively infrequent, difficult bowel movements with bloating

HISTORY OF PRESENT ILLNESS:
The patient returns to the Endocrinology clinic for follow-up of his longitudinal diabetes care, having previously presented to Gastroenterology in 2020 for nocturnal heartburn and regurgitation and to Pulmonology in early 2021 for episodic facial flushing and itching. He now reports four months of progressively infrequent, difficult bowel movements with associated bloating. He states that stools are small, dry, and require straining despite increased water intake. He denies rectal bleeding, weight loss, or nocturnal pain. He eats irregularly because he often feels full after just a few bites and sometimes experiences mild nausea after meals. He notices large glucose swings hours after dinner, even when he eats very little. He denies recent travel or antibiotic exposure. He reports that fiber supplements worsened his distension without improving stool frequency. He denies abdominal pain localized to the right lower quadrant or epigastrium. He denies greasy stools or oily residue in the toilet.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-07-15)
- Statin-induced interstitial lung disease (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Diabetic Autonomic Neuropathy

He reports long-standing diabetes complicated by numbness in his feet, erectile difficulties, and occasional lightheadedness when standing quickly. Colon cancer screening last year was normal.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Colonoscopy (age 54, normal)

MEDICATIONS:
He takes metformin and basal-bolus insulin; he denies opioid or anticholinergic use.

Home Medications:
- Aspirin 81 mg daily
- Clopidogrel 75 mg daily
- Atorvastatin 40 mg nightly
- Metoprolol succinate 50 mg daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-year history). Alcohol: Occasional social use. Occupation: Agricultural equipment operator.

VITAL SIGNS:
Orthostatic vitals show a small drop in systolic pressure without compensatory tachycardia.

PHYSICAL EXAMINATION:
Physical exam shows dry mucous membranes and a mildly distended, tympanic abdomen with decreased bowel sounds but no tenderness. Digital rectal exam shows normal tone and no palpable masses or fissures. There is diminished pinprick sensation in a stocking distribution of both feet. Thyroid examination is normal.

IMAGING:
A plain abdominal film shows moderate stool burden without obstruction.

## Encounter 4: source 2022-07-15, practice 2024-01-08, outpatient, Cardiology

Chief complaint: Follow-up evaluation of exertional chest discomfort despite medical therapy

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Park, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up evaluation of exertional chest discomfort despite medical therapy

HISTORY OF PRESENT ILLNESS:
The patient returns for cardiology follow-up with CAD and prior ILD. Despite adherence to a regimen that includes aspirin, statin therapy, and a beta-blocker, he continues to experience occasional episodes of exertional chest discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-07-15)
- Statin-induced interstitial lung disease (diagnosed 2021-01-15)
- Diabetic autonomic neuropathy (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus
- Coronary Artery Disease

PAST SURGICAL HISTORY:
He underwent stent placement six months ago

MEDICATIONS:
He takes aspirin, statin therapy, and a beta-blocker.

Home Medications:
- Clopidogrel 75 mg daily
- Atorvastatin 40 mg nightly
- Metoprolol succinate 50 mg daily
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-year history). Alcohol: Occasional social use. Occupation: Agricultural equipment operator.

PHYSICAL EXAMINATION:
There is no jugular venous distension, no pedal edema, and no signs of heart failure.

PLAN:
He is commenced on isosorbide dinitrate.

## Encounter 5: source 2023-07-15, practice 2025-01-06, ed, Emergency Medicine

Chief complaint: Acute onset of excessive salivation, lacrimation, and difficulty breathing after spraying insecticides

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of excessive salivation, lacrimation, and difficulty breathing after spraying insecticides

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with coronary artery disease, diabetic autonomic neuropathy, and GERD (prior angina and constipation evaluations) presents to the emergency department with his wife, who reports that he has been experiencing excessive salivation, lacrimation, and difficulty breathing over the past hour. He was spraying insecticides in the field earlier in the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-07-15)
- Statin-induced interstitial lung disease (diagnosed 2021-01-15)
- Diabetic autonomic neuropathy (diagnosed 2021-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Colonoscopy (age 54, normal)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Clopidogrel 75 mg daily
- Atorvastatin 40 mg nightly
- Metoprolol succinate 50 mg daily
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-year history). Alcohol: Occasional social use. Occupation: Agricultural equipment operator.

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 55 bpm, respiratory rate 28 breaths/min, and oxygen saturation 88% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is anxious, diaphoretic, and has miosis. Auscultation of the lungs reveals bilateral wheezes. The patient also exhibits muscle fasciculations and weakness.

## Encounter 6: source 2024-07-15, practice 2026-01-09, follow_up, Cardiology

Chief complaint: Post-discharge follow-up two weeks after hospitalization for acute inferior wall STEMI

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. David Park, MD
Department: Cardiology
Visit Type: follow_up

CHIEF COMPLAINT:
Post-discharge follow-up two weeks after hospitalization for acute inferior wall STEMI

HISTORY OF PRESENT ILLNESS:
The patient with prior stable angina returns for follow-up two weeks after inferior STEMI. He underwent successful percutaneous coronary intervention (PCI) to his left anterior descending artery. He reports adherence to his medication regimen and has been participating in a cardiac rehabilitation program.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-07-15)
- Statin-induced interstitial lung disease (diagnosed 2021-01-15)
- Diabetic autonomic neuropathy (diagnosed 2021-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-07-15)
- Organophosphate poisoning (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Colonoscopy (age 54, normal)

MEDICATIONS:
His current medications include aspirin, clopidogrel, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg daily
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-year history). Alcohol: Occasional social use. Occupation: Agricultural equipment operator.

REVIEW OF SYSTEMS:
He denies any chest pain, dyspnea, or palpitations.

VITAL SIGNS:
His blood pressure is 128/82 mmHg, heart rate is 64 beats per minute

PHYSICAL EXAMINATION:
Physical examination is unremarkable

## Encounter 7: source 2025-01-15, practice 2026-07-11, ed, Emergency Medicine

Chief complaint: Severe, pressure-like substernal chest pain radiating to the left shoulder

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, pressure-like substernal chest pain radiating to the left shoulder

HISTORY OF PRESENT ILLNESS:
This 60-year-old male has Coronary Artery Disease and prior inferior wall STEMI in July 2024. He presents to the emergency department with 90 minutes of severe, pressure-like substernal chest pain radiating to his left shoulder.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-07-15)
- Statin-induced interstitial lung disease (diagnosed 2021-01-15)
- Diabetic autonomic neuropathy (diagnosed 2021-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-07-15)
- Organophosphate poisoning (diagnosed 2023-07-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2024-07-15)
- Type 2 Diabetes Mellitus
- Coronary Artery Disease

He has a history of hyperlipidemia and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Colonoscopy (age 54, normal)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Clopidogrel 75 mg daily
- Atorvastatin 40 mg nightly
- Metoprolol succinate 50 mg daily
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Omeprazole 20 mg daily

ALLERGIES:
Simvastatin (interstitial lung disease), Niacin (severe flushing and pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
He smokes one pack of cigarettes daily and drinks alcohol occasionally.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 92/60 mm Hg.

PHYSICAL EXAMINATION:
He appears diaphoretic and mildly short of breath. Cardiopulmonary exam reveals clear lung fields and bradycardia. Cardiac auscultation reveals normal S1 and S2 with no murmurs.

LABORATORY DATA:
Troponin I is elevated.

OTHER STUDIES:
ECG findings are shown below.
