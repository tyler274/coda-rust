# synthetic_hospital patient 2207

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-23 (Mon) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2021-02-16 (Tue) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2022-05-24 (Tue) | ed | Emergency Medicine |
| 3 | 2023-01-15 | 2023-11-15 (Wed) | ed | Emergency Medicine |
| 4 | 2025-01-15 | 2025-11-21 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired landscape architect",
 "alcohol_use": "Rare, less than 1 drink per month",
 "family_history": [
  "Bicuspid aortic valve in brother",
  "Myocardial infarction in father at age 62",
  "Type 2 diabetes mellitus in mother"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "7.5 mg daily (variable per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "100 mg daily",
   "name": "Metoprolol succinate"
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
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Mechanical aortic valve replacement (2 years prior)",
  "Coronary angiography with percutaneous intervention (following inferior wall STEMI)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Atrial fibrillation",
  "Coronary artery disease with history of myocardial infarction",
  "Status post mechanical aortic valve replacement for bicuspid aortic stenosis"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Atrial fibrillation", "Anticoagulation-related hemorrhage", "Right Ventricular Myocardial Infarction", "ST-segment elevation myocardial infarction, inferior wall"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-11-23, outpatient, Cardiology

Chief complaint: Routine follow-up for chronic atrial fibrillation with intermittent palpitations and mild dyspnea on exertion over the past two months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic atrial fibrillation with intermittent palpitations and mild dyspnea on exertion over the past two months.

HISTORY OF PRESENT ILLNESS:
He reports experiencing intermittent palpitations and mild dyspnea on exertion for the past two months. He denies chest pain, dizziness, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Atrial fibrillation
- Coronary artery disease with history of myocardial infarction
- Status post mechanical aortic valve replacement for bicuspid aortic stenosis

PAST SURGICAL HISTORY:
- Mechanical aortic valve replacement (2 years prior)
- Coronary angiography with percutaneous intervention (following inferior wall STEMI)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Warfarin 7.5 mg daily (variable per INR)
- Metoprolol succinate 100 mg daily
- Amlodipine 5 mg daily
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Bicuspid aortic valve in brother
- Myocardial infarction in father at age 62
- Type 2 diabetes mellitus in mother

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Rare, less than 1 drink per month. Occupation: Retired landscape architect.

VITAL SIGNS:
On physical examination, his blood pressure is 142/88 mmHg, heart rate is irregularly irregular at 88 bpm, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
Auscultation of the heart reveals an absence of S1 consistency and a variable intensity of S2. No murmurs or gallops are noted.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 14.2 g/dL
- Platelet count: 250,000/mm^3
- INR: 1.0
- Creatinine: 1.1 mg/dL
- Estimated Glomerular Filtration Rate (eGFR): 60 mL/min/1.73m^2

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with a ventricular rate of 86 bpm.

ASSESSMENT AND PLAN:
His CHA2DS2-VASc score is calculated to be 3 (1 point each for age >65, hypertension, and diabetes).

## Encounter 1: source 2020-04-15, practice 2021-02-16, ed, Emergency Medicine

Chief complaint: Acute onset of palpitations and shortness of breath beginning 2 hours ago while gardening, without chest pain or syncope.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of palpitations and shortness of breath beginning 2 hours ago while gardening, without chest pain or syncope.

HISTORY OF PRESENT ILLNESS:
This patient with atrial fibrillation and mechanical aortic valve replacement, seen in Cardiology 3 months ago, presents to the emergency department with palpitations and shortness of breath starting 2 hours ago while gardening. He denies chest pain, dizziness, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary artery disease with history of myocardial infarction
- Status post mechanical aortic valve replacement for bicuspid aortic stenosis

PAST SURGICAL HISTORY:
- Mechanical aortic valve replacement (2 years prior)
- Coronary angiography with percutaneous intervention (following inferior wall STEMI)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Warfarin 7.5 mg daily (variable per INR)
- Metoprolol succinate 100 mg daily
- Amlodipine 5 mg daily
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Bicuspid aortic valve in brother
- Myocardial infarction in father at age 62
- Type 2 diabetes mellitus in mother

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Rare, less than 1 drink per month. Occupation: Retired landscape architect.

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, heart rate is irregularly irregular at 130 beats per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals no crackles, and there is no leg edema.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 14.2 g/dL
- White blood cell count: 7.2 x 10^3/uL
- Platelets: 230 x 10^3/uL
- Serum creatinine: 1.1 mg/dL
- Blood urea nitrogen: 18 mg/dL
- Fasting blood glucose: 160 mg/dL
- Thyroid-stimulating hormone: 2.5 mIU/L

OTHER STUDIES:
An electrocardiogram (ECG) shows no P waves, and a variable ventricular response with a rapid ventricular rate.

## Encounter 2: source 2021-07-15, practice 2022-05-24, ed, Emergency Medicine

Chief complaint: Acute bleeding complication while receiving rifampin-based tuberculosis therapy following recent hospitalization for pulmonary tuberculosis.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute bleeding complication while receiving rifampin-based tuberculosis therapy following recent hospitalization for pulmonary tuberculosis.

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with mechanical aortic valve replacement on chronic warfarin and history of atrial fibrillation who presents to the emergency department. Three weeks ago, he was diagnosed with TB after presenting with hemoptysis, weight loss, and upper lobe infiltrates on chest imaging. He is currently receiving rifampin, isoniazid, pyrazinamide, and ethambutol. He reports no recent bleeding episodes or changes in diet but says his warfarin dose was increased last week due to a previously low INR of 1.2.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary artery disease with history of myocardial infarction
- Status post mechanical aortic valve replacement for bicuspid aortic stenosis

PAST SURGICAL HISTORY:
- Mechanical aortic valve replacement (2 years prior)
- Coronary angiography with percutaneous intervention (following inferior wall STEMI)

MEDICATIONS:
Medications include warfarin, metoprolol, insulin glargine, and amlodipine.

Home Medications:
- Metoprolol succinate 100 mg daily
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Bicuspid aortic valve in brother
- Myocardial infarction in father at age 62
- Type 2 diabetes mellitus in mother

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Rare, less than 1 drink per month. Occupation: Retired landscape architect.

REVIEW OF SYSTEMS:
He denies chest pain, shortness of breath, or melena.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Today, his INR is 1.7, and he is still below the therapeutic range.

## Encounter 3: source 2023-01-15, practice 2023-11-15, ed, Emergency Medicine

Chief complaint: Acute substernal pressure-like chest pain radiating to the left arm with associated shortness of breath and diaphoresis.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute substernal pressure-like chest pain radiating to the left arm with associated shortness of breath and diaphoresis.

HISTORY OF PRESENT ILLNESS:
The patient with prior MI returns with chest pain. The pain is substernal, pressure-like, and radiates to his left arm. He also reports shortness of breath and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Anticoagulation-related hemorrhage (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary artery disease with history of myocardial infarction
- Status post mechanical aortic valve replacement for bicuspid aortic stenosis

PAST SURGICAL HISTORY:
- Mechanical aortic valve replacement (2 years prior)
- Coronary angiography with percutaneous intervention (following inferior wall STEMI)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Warfarin 7.5 mg daily (variable per INR)
- Metoprolol succinate 100 mg daily
- Amlodipine 5 mg daily
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Bicuspid aortic valve in brother
- Myocardial infarction in father at age 62
- Type 2 diabetes mellitus in mother

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Rare, less than 1 drink per month. Occupation: Retired landscape architect.

VITAL SIGNS:
On examination, his blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation.

LABORATORY DATA:
His initial laboratory results are as follows:

- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatinine: 1.2 mg/dL (normal 0.6-1.3 mg/dL)
- Hemoglobin A1c: 7.8% (normal <5.7%)
- Total cholesterol: 185 mg/dL (normal <200 mg/dL)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in leads II, III, and aVF.

ASSESSMENT AND PLAN:
Given the patient's presentation and test results,

## Encounter 4: source 2025-01-15, practice 2025-11-21, ed, Emergency Medicine

Chief complaint: Acute pressure-like chest pain radiating to the left arm with shortness of breath and diaphoresis beginning 30 minutes ago while mowing the lawn.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Priya Nair, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute pressure-like chest pain radiating to the left arm with shortness of breath and diaphoresis beginning 30 minutes ago while mowing the lawn.

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with coronary artery disease status post myocardial infarction in 2023 and mechanical aortic valve replacement who presents with recurrent chest pain. The pain is described as a pressure-like sensation in the center of his chest, radiating to his left arm, and began while he was mowing the lawn 30 minutes ago. He also reports shortness of breath and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Anticoagulation-related hemorrhage (diagnosed 2021-07-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary artery disease with history of myocardial infarction
- Status post mechanical aortic valve replacement for bicuspid aortic stenosis

PAST SURGICAL HISTORY:
- Mechanical aortic valve replacement (2 years prior)
- Coronary angiography with percutaneous intervention (following inferior wall STEMI)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Warfarin 7.5 mg daily (variable per INR)
- Metoprolol succinate 100 mg daily
- Amlodipine 5 mg daily
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Bicuspid aortic valve in brother
- Myocardial infarction in father at age 62
- Type 2 diabetes mellitus in mother

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Rare, less than 1 drink per month. Occupation: Retired landscape architect.

VITAL SIGNS:
On examination, his blood pressure is 160/90 mmHg, heart rate is 98 bpm, and respiratory rate is 22 breaths per minute.

LABORATORY DATA:
Laboratory values:
- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatinine: 1.2 mg/dL (normal 0.6-1.2 mg/dL)
- Hemoglobin A1c: 7.2% (normal <5.7% for non-diabetics)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in leads II, III, and aVF.

PLAN:
The patient is given aspirin, nitroglycerin, and morphine for pain management. The cardiology team is consulted, and they recommend immediate percutaneous coronary intervention (PCI). The nurse verbally acknowledges the order and leaves to prepare the patient for transfer. Fifteen minutes later, the patient's condition has not changed, and there is no sign of the transfer taking place. Upon re-evaluation, it is discovered that the nurse misinterpreted the verbal order due to a noisy environment and thought the patient was to receive a repeat ECG instead.
