# synthetic_hospital patient 1976

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, African American, Commercial auto insurance (occupational) and employer-sponsored health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-07-29 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2020-01-27 (Mon) | outpatient | Cardiology |
| 2 | 2021-01-15 | 2020-07-30 (Thu) | outpatient | Cardiology |
| 3 | 2022-01-15 | 2021-07-27 (Tue) | outpatient | Cardiology |
| 4 | 2023-01-15 | 2022-07-25 (Mon) | outpatient | Pulmonology |
| 5 | 2024-01-15 | 2023-08-02 (Wed) | outpatient | Gastroenterology |
| 6 | 2025-01-15 | 2024-07-30 (Tue) | outpatient | Neurosurgery |
| 7 | 2027-01-15 | 2026-08-04 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial auto insurance (occupational) and employer-sponsored health insurance",
 "occupation": "Long-haul truck driver",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea",
  "Mother: Hypertension, Hyperlipidemia",
  "Paternal uncle: Coronary artery disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 8 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Obstructive sleep apnea", "Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus", "Cervical spondylotic myelopathy", "Achalasia", "Vitreous hemorrhage"]
Comorbidities: ["Metabolic syndrome", "Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2019-07-29, outpatient, Endocrinology

Chief complaint: Routine health check-up for Type 2 diabetes mellitus with increased fatigue over past few months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up for Type 2 diabetes mellitus with increased fatigue over past few months

HISTORY OF PRESENT ILLNESS:
He reports no acute complaints but mentions that he has been feeling more fatigued over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension
- Hyperlipidemia

He has a history of hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes which is currently managed with lisinopril.

Home Medications:
- Metformin 1000 mg twice daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for type 2 diabetes mellitus in his mother.

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Long-haul truck driver.

REVIEW OF SYSTEMS:
He denies any chest pain, palpitations, polyuria, polydipsia, or unintentional weight loss.

VITAL SIGNS:
His BMI is calculated to be 28.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for a blood pressure of 138/86 mmHg.

LABORATORY DATA:
Laboratory results are as follows:

- Fasting blood glucose: 132 mg/dL
- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL
- Serum creatinine: 1.0 mg/dL

## Encounter 1: source 2020-07-15, practice 2020-01-27, outpatient, Cardiology

Chief complaint: Routine follow-up for essential hypertension and medication compliance verification

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for essential hypertension and medication compliance verification

HISTORY OF PRESENT ILLNESS:
The patient, a 55-year-old male with a 15-year history of Type 2 Diabetes Mellitus, Hypertension, and Hyperlipidemia, returns to cardiology for routine hypertension follow-up following his endocrinology evaluation six months ago for fatigue. He reports compliance with his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes which include metformin and atorvastatin.

Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea
- Mother: Hypertension, Hyperlipidemia
- Paternal uncle: Coronary artery disease

SOCIAL HISTORY:
However, he admits to struggling with dietary recommendations and has not been exercising regularly due to a demanding work schedule.

VITAL SIGNS:
His blood pressure today is 146/92 mmHg, which has been consistently elevated over the past three visits. His BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for obesity.

LABORATORY DATA:
Laboratory values are as follows:

- Fasting glucose: 138 mg/dL
- HbA1c: 7.2%
- Total cholesterol: 190 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 2: source 2021-01-15, practice 2020-07-30, outpatient, Cardiology

Chief complaint: Follow-up visit for persistently elevated home blood pressure readings despite medication adherence

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for persistently elevated home blood pressure readings despite medication adherence

HISTORY OF PRESENT ILLNESS:
This 56-year-old with type 2 diabetes and hypertension returns for follow-up of elevated home blood pressure following prior evaluation. He reports compliance with his current antihypertensive regimen, which includes hydrochlorothiazide and amlodipine, but notes that his home blood pressure readings have been consistently above the target range. He denies any chest pain, palpitations, or shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin and the aforementioned antihypertensives.

Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea
- Mother: Hypertension, Hyperlipidemia
- Paternal uncle: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Long-haul truck driver.

VITAL SIGNS:
On examination, his blood pressure is 152/94 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

LABORATORY DATA:
Recent laboratory results are as follows:

- Fasting plasma glucose: 158 mg/dL
- HbA1c: 7.4%
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 300 mg/dL

## Encounter 3: source 2022-01-15, practice 2021-07-27, outpatient, Cardiology

Chief complaint: Routine cardiovascular risk assessment and discussion of adjunctive lipid-lowering therapy

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cardiovascular risk assessment and discussion of adjunctive lipid-lowering therapy

HISTORY OF PRESENT ILLNESS:
The patient with diabetes, hypertension, and hyperlipidemia returns for cardiovascular risk assessment, last seen one year ago for elevated blood pressure despite adherence. He reports no new symptoms and feels generally well. The patient is interested in starting a new medication to further reduce his cardiovascular risk. He mentions reading about a recent clinical trial that showed a significant reduction in heart attacks among patients taking a new drug.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He has been adherent to his medication regimen, which includes metformin and lisinopril.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea
- Mother: Hypertension, Hyperlipidemia
- Paternal uncle: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Long-haul truck driver.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 158 mg/dL
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 200 mg/dL

ASSESSMENT AND PLAN:
However, you are aware that the trial did not adjust for patients' smoking status, and the majority of the patients in the treatment group were non-smokers.

## Encounter 4: source 2023-01-15, practice 2022-07-25, outpatient, Pulmonology

Chief complaint: Excessive daytime sleepiness, loud snoring, and witnessed apneas reported by wife

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Washington, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Excessive daytime sleepiness, loud snoring, and witnessed apneas reported by wife

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with longstanding Type 2 Diabetes Mellitus, Hypertension, and Hyperlipidemia, established with Endocrinology and Cardiology, presents to Pulmonology with new sleep complaints. His wife reports that he snores loudly and occasionally stops breathing at night, which wakes her up. The patient also mentions waking up with a dry mouth and sometimes a choking sensation. He denies any chest pain, palpitations, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Hyperlipidemia (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea
- Mother: Hypertension, Hyperlipidemia
- Paternal uncle: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Long-haul truck driver.

VITAL SIGNS:
Blood pressure is 138/86 mmHg, heart rate is 88 bpm, and BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
The oropharyngeal examination reveals enlarged tonsils and a crowded posterior pharynx. Neck circumference is 43 cm. The remainder of the examination is unremarkable.

LABORATORY DATA:
- Fasting Blood Glucose: 146 mg/dL
- HbA1c: 7.2%
- Total Cholesterol: 190 mg/dL
- HDL: 40 mg/dL
- LDL: 120 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 5: source 2024-01-15, practice 2023-08-02, outpatient, Gastroenterology

Chief complaint: Six-month history of progressive dysphagia to solids and liquids with occasional regurgitation

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive dysphagia to solids and liquids with occasional regurgitation

HISTORY OF PRESENT ILLNESS:
The patient has Type 2 diabetes, hypertension, hyperlipidemia, and sleep apnea and presents with dysphagia. He reports occasional regurgitation of undigested food and a sensation of food sticking in his mid-chest after eating. He denies any significant weight loss, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Hyperlipidemia (diagnosed 2022-01-15)
- Obstructive sleep apnea (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea
- Mother: Hypertension, Hyperlipidemia
- Paternal uncle: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Long-haul truck driver.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
No evidence of anemia or lymphadenopathy. His cardiovascular and abdominal examinations are unremarkable.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 14.2 g/dL
- White Blood Cell Count: 7,000/uL
- Fasting Blood Glucose: 158 mg/dL
- HbA1c: 7.1%

PATHOLOGY:
Biopsies show no evidence of malignancy or eosinophilic infiltration.

OTHER STUDIES:
An upper endoscopy reveals a dilated esophagus with retained food and a tight lower esophageal sphincter that does not relax with swallowing.

## Encounter 6: source 2025-01-15, practice 2024-07-30, outpatient, Neurosurgery

Chief complaint: Progressive gait difficulty, hand numbness, and occasional urinary incontinence over six months

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Kim, MD
Department: Neurosurgery
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive gait difficulty, hand numbness, and occasional urinary incontinence over six months

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with Type 2 Diabetes Mellitus, hypertension, and hyperlipidemia was evaluated last year for dysphagia. He presents to Neurosurgery with a 6-month history of progressive difficulty in walking, numbness in his hands, and occasional urinary incontinence. He also reports sharp, shooting pains radiating down his arms when he flexes his neck.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Hyperlipidemia (diagnosed 2022-01-15)
- Obstructive sleep apnea (diagnosed 2023-01-15)
- Achalasia (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea
- Mother: Hypertension, Hyperlipidemia
- Paternal uncle: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Long-haul truck driver.

PHYSICAL EXAMINATION:
On physical examination, you note hyperreflexia in the lower extremities, bilateral Hoffmann's signs, and an unsteady gait.

LABORATORY DATA:
Laboratory values:
- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 156 mg/dL
- Serum creatinine: 1.0 mg/dL
- Total cholesterol: 210 mg/dL
- Blood urea nitrogen (BUN): 14 mg/dL

IMAGING:
MRI of the cervical spine reveals multilevel degenerative disc disease with evidence of spinal cord compression.

## Encounter 7: source 2027-01-15, practice 2026-08-04, ed, Emergency Medicine

Chief complaint: Sudden painless vision loss in right eye with floaters noted earlier today

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden painless vision loss in right eye with floaters noted earlier today

HISTORY OF PRESENT ILLNESS:
This 62-year-old male with Type 2 Diabetes Mellitus presents to the ED. He describes seeing "dark spots" floating in his vision. His diabetes has been poorly controlled, with recent hemoglobin A1c of 9.2%.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Hyperlipidemia (diagnosed 2022-01-15)
- Obstructive sleep apnea (diagnosed 2023-01-15)
- Achalasia (diagnosed 2024-01-15)
- Cervical spondylotic myelopathy (diagnosed 2025-01-15)
- Type 2 Diabetes Mellitus (15-year history, diagnosed age 40)
- Hypertension

His other medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension, Obstructive Sleep Apnea
- Mother: Hypertension, Hyperlipidemia
- Paternal uncle: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Long-haul truck driver.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Ophthalmologic examination reveals loss of red reflex and dark streaks in the vitreous humor of the right eye. There is no evidence of retinal detachment or retinal tear on examination. Fundoscopy of the left eye is unremarkable.
