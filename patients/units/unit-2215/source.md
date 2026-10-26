# synthetic_hospital patient 2215

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-20 (Sun) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-04-25 (Fri) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-10-26 (Sun) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa antibiotics (rash)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 64",
  "Mother: Ischemic stroke at age 79",
  "Sibling: Type 2 Diabetes Mellitus with end-stage renal disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 2 years ago",
 "home_medications": [
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale with meals per protocol",
   "name": "Insulin aspart"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg orally at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Coronary artery stent placement (2022)",
  "Left femoral artery catheterization for diagnostic angiography (6 months prior)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Chronic Kidney Disease, Stage 3",
  "Hyperlipidemia",
  "Atherosclerotic Cardiovascular Disease",
  "Peripheral Artery Disease"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Severe hypoglycemia", "Acute pancreatitis", "Atheroembolism (Cholesterol Embolism Syndrome)"]
Comorbidities: ["Acute kidney injury", "Chronic kidney disease stage 3b", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-10-20, ed, Emergency Medicine

Chief complaint: Severe, sharp pain in left foot for 2 days with transient visual disturbances and post-prandial abdominal pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, sharp pain in left foot for 2 days with transient visual disturbances and post-prandial abdominal pain

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 2-day history of severe, sharp pain in his left foot. He describes the pain as constant and exacerbated by elevation of the limb. He also reports a recent history of transient visual disturbances and abdominal pain after eating.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease, Stage 3
- Hyperlipidemia
- Atherosclerotic Cardiovascular Disease
- Peripheral Artery Disease

His medical history includes hypertension, hyperlipidemia, and chronic kidney disease.

PAST SURGICAL HISTORY:
He underwent cardiac catheterization 3 weeks ago due to unstable angina.

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin aspart Sliding scale with meals per protocol
- Lisinopril 10 mg orally daily
- Amlodipine 5 mg orally daily
- Atorvastatin 40 mg orally at bedtime
- Aspirin 81 mg orally daily

ALLERGIES:
Sulfa antibiotics (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Ischemic stroke at age 79
- Sibling: Type 2 Diabetes Mellitus with end-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: None. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On physical examination, the left foot is cool to touch, with a cyanotic toe and a mottled, net-like pattern of discoloration over the lower extremities. A fundoscopic exam reveals bright, refractile plaques at the bifurcations of retinal arterioles.

LABORATORY DATA:
Laboratory values are as follows:

- Serum creatinine: 2.8 mg/dL (baseline 1.6 mg/dL)
- Blood urea nitrogen (BUN): 44 mg/dL
- Complete blood count (CBC): Normal
- C-reactive protein (CRP): Elevated
- Erythrocyte sedimentation rate (ESR): Elevated

## Encounter 1: source 2020-07-15, practice 2025-04-25, ed, Emergency Medicine

Chief complaint: Severe epigastric abdominal pain radiating to the back with vomiting for 6 hours

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe epigastric abdominal pain radiating to the back with vomiting for 6 hours

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, diabetes, and CKD returns six months after prior atheroembolism. He describes the pain as constant, radiating to the back, and associated with nausea. He denies chest pain, shortness of breath, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Atheroembolism (Cholesterol Embolism Syndrome) (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease, Stage 3
- Hyperlipidemia
- Atherosclerotic Cardiovascular Disease
- Peripheral Artery Disease

PAST SURGICAL HISTORY:
- Coronary artery stent placement (2022)
- Left femoral artery catheterization for diagnostic angiography (6 months prior)

MEDICATIONS:
His medications include lisinopril and amlodipine.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin aspart Sliding scale with meals per protocol
- Atorvastatin 40 mg orally at bedtime
- Aspirin 81 mg orally daily

ALLERGIES:
Sulfa antibiotics (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Ischemic stroke at age 79
- Sibling: Type 2 Diabetes Mellitus with end-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 165/90 mmHg, heart rate is 102 beats per minute, and temperature is 38.2°C (100.8°F).

PHYSICAL EXAMINATION:
His abdomen is distended, with epigastric tenderness and decreased bowel sounds.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 16,000/uL
- Hemoglobin: 13.2 g/dL
- Blood urea nitrogen: 31 mg/dL
- Serum creatinine: 2.1 mg/dL (baseline 1.6 mg/dL)
- Serum amylase: 900 U/L
- Serum lipase: 1100 U/L
- Serum glucose: 210 mg/dL

IMAGING:
An abdominal CT scan shows inflammation and edema surrounding the pancreas.

## Encounter 2: source 2021-01-15, practice 2025-10-26, icu, Internal Medicine

Chief complaint: Altered mental status with severe hypoglycemia in the setting of urosepsis

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Altered mental status with severe hypoglycemia in the setting of urosepsis

PAST MEDICAL HISTORY:
Active Problem List:
- Atheroembolism (Cholesterol Embolism Syndrome) (diagnosed 2020-01-15)
- Acute pancreatitis (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease, Stage 3
- Hyperlipidemia
- Atherosclerotic Cardiovascular Disease
- Peripheral Artery Disease

PAST SURGICAL HISTORY:
- Coronary artery stent placement (2022)
- Left femoral artery catheterization for diagnostic angiography (6 months prior)

MEDICATIONS:
He is currently on insulin glargine and insulin aspart for his diabetes.

Home Medications:
- Lisinopril 10 mg orally daily
- Amlodipine 5 mg orally daily
- Atorvastatin 40 mg orally at bedtime
- Aspirin 81 mg orally daily

ALLERGIES:
Sulfa antibiotics (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Ischemic stroke at age 79
- Sibling: Type 2 Diabetes Mellitus with end-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure of 95/60 mmHg, heart rate of 102 beats per minute, respiratory rate of 22 breaths per minute, and temperature of 38.3°C (100.9°F)

PHYSICAL EXAMINATION:
He is lethargic but arousable. His skin is warm and dry.

LABORATORY DATA:
Laboratory results are as follows:

- Blood glucose: 48 mg/dL
- Serum creatinine: 2.8 mg/dL (baseline 1.9)
- Blood urea nitrogen (BUN): 44 mg/dL
- Serum potassium: 4.5 mEq/L
- Serum sodium: 135 mEq/L
- Arterial blood gas (ABG): pH 7.32, PaCO2 32 mmHg, HCO3- 18 mEq/L
