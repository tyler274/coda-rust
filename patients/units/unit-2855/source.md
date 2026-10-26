# synthetic_hospital patient 2855

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 75 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-12 (Wed) | outpatient | Orthopedic Surgery |
| 1 | 2020-07-15 | 2023-04-17 (Mon) | outpatient | Cardiology |
| 2 | 2020-10-15 | 2023-07-10 (Mon) | outpatient | Endocrinology |
| 3 | 2021-04-15 | 2024-01-04 (Thu) | inpatient | Internal Medicine |
| 4 | 2022-01-15 | 2024-10-04 (Fri) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2025-10-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired",
 "alcohol_use": "None",
 "family_history": [
  "Type 2 diabetes mellitus in mother and sister",
  "Cerebrovascular accident in father",
  "Alzheimer disease in maternal grandmother",
  "Hypertension in both parents"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale subcutaneously before meals",
   "name": "Insulin aspart"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg orally daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Left total knee arthroplasty 9 months prior to initial presentation"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Vascular dementia",
  "Cerebrovascular accident with residual right-sided hemiparesis",
  "Osteoarthritis",
  "Status post left total knee arthroplasty"
 ],
 "age_at_first_encounter": 75
}
```

Primary diagnoses: ["Sinus bradycardia with non-conducted premature atrial contractions", "Stage III pressure ulcer of sacrum", "Pressure ulcer of sacrum, stage 1", "Prosthetic joint infection due to Staphylococcus epidermidis", "Severe hypoglycemia", "Adult physical abuse"]
Comorbidities: ["Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-10-12, outpatient, Orthopedic Surgery

Chief complaint: Increasing pain, swelling, and stiffness in left knee nine months after total knee arthroplasty

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing pain, swelling, and stiffness in left knee nine months after total knee arthroplasty

HISTORY OF PRESENT ILLNESS:
Which was replaced with a prosthetic joint 9 months ago. She reports that the knee had been healing well after surgery until about 3 weeks ago, when she began noticing increasing stiffness and mild warmth. She denies fever, chills, or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Vascular dementia
- Cerebrovascular accident with residual right-sided hemiparesis
- Osteoarthritis
- Status post left total knee arthroplasty

Her past medical history includes type 2 diabetes mellitus and osteoarthritis.

PAST SURGICAL HISTORY:
- Left total knee arthroplasty 9 months prior to initial presentation

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin aspart Sliding scale subcutaneously before meals
- Lisinopril 10 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother and sister
- Cerebrovascular accident in father
- Alzheimer disease in maternal grandmother
- Hypertension in both parents

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired.

PHYSICAL EXAMINATION:
Physical examination reveals mild warmth and effusion of the left knee with decreased range of motion. There is no erythema or overlying skin breakdown. The remainder of her exam is unremarkable.

LABORATORY DATA:
Knee arthrocentesis yields cloudy synovial fluid. Gram stain shows Staph epidermidis .

## Encounter 1: source 2020-07-15, practice 2023-04-17, outpatient, Cardiology

Chief complaint: Six-month history of worsening fatigue, lightheadedness on standing, and two episodes of near-syncope

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of worsening fatigue, lightheadedness on standing, and two episodes of near-syncope

HISTORY OF PRESENT ILLNESS:
This 75-year-old female with diabetes, hypertension, vascular dementia, right hemiparesis, and recent left knee prosthetic infection presents to the clinic with a 6-month history of increasing fatigue and episodes of lightheadedness, particularly when she stands up from a sitting position. She also reports two instances of near-syncope over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Prosthetic joint infection due to Staphylococcus epidermidis (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Vascular dementia
- Cerebrovascular accident with residual right-sided hemiparesis
- Osteoarthritis
- Status post left total knee arthroplasty

Her past medical history is significant for hypertension, which is controlled with hydrochlorothiazide, and type 2 diabetes mellitus managed with metformin.

PAST SURGICAL HISTORY:
- Left total knee arthroplasty 9 months prior to initial presentation

MEDICATIONS:
She takes hydrochlorothiazide, metformin.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin aspart Sliding scale subcutaneously before meals
- Lisinopril 10 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother and sister
- Cerebrovascular accident in father
- Alzheimer disease in maternal grandmother
- Hypertension in both parents

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired.

REVIEW OF SYSTEMS:
She denies chest pain, palpitations, or dyspnea.

VITAL SIGNS:
On physical examination, her blood pressure is 130/80 mmHg, heart rate is 48 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Her cardiovascular examination is notable for a regular, slow rhythm without murmurs, rubs, or gallops. The remainder of the physical examination is unremarkable.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows sinus bradycardia with occasional dropped P waves.

## Encounter 2: source 2020-10-15, practice 2023-07-10, outpatient, Endocrinology

Chief complaint: Episodes of morning confusion, abnormal behavior, and diaphoresis before breakfast

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Aisha Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of morning confusion, abnormal behavior, and diaphoresis before breakfast

HISTORY OF PRESENT ILLNESS:
The patient presents to Endocrinology in the context of Type 2 diabetes mellitus, vascular dementia, and prior cerebrovascular accident, following recent Cardiology evaluation for sinus bradycardia and prior prosthetic joint infection. She admits to occasional mild tremors and sweating during these episodes but does not recall confusion. She denies recent changes in diet, medication, or activity level. A capillary blood glucose level measured in clinic during an episode is 50 mg/dL.

PAST MEDICAL HISTORY:
Active Problem List:
- Prosthetic joint infection due to Staphylococcus epidermidis (diagnosed 2020-01-15)
- Sinus bradycardia with non-conducted premature atrial contractions (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Vascular dementia
- Cerebrovascular accident with residual right-sided hemiparesis
- Osteoarthritis
- Status post left total knee arthroplasty

Her last HbA1c was 6.8%.

PAST SURGICAL HISTORY:
- Left total knee arthroplasty 9 months prior to initial presentation

MEDICATIONS:
Her diabetes has been managed with insulin glargine and mealtime insulin aspart.

Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 10 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother and sister
- Cerebrovascular accident in father
- Alzheimer disease in maternal grandmother
- Hypertension in both parents

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she is alert and oriented with no focal neurological deficits.

LABORATORY DATA:
Laboratory values:
- Capillary blood glucose: 50 mg/dL
- HbA1c: 6.8%

## Encounter 3: source 2021-04-15, practice 2024-01-04, inpatient, Internal Medicine

Chief complaint: Urinary tract infection with acute weakness, bedridden status, and new sacral skin discoloration

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Urinary tract infection with acute weakness, bedridden status, and new sacral skin discoloration

HISTORY OF PRESENT ILLNESS:
Following prior endocrinologic and orthopedic complications, she has been bedridden for the past week due to weakness and has a Foley catheter in place.

PAST MEDICAL HISTORY:
Active Problem List:
- Prosthetic joint infection due to Staphylococcus epidermidis (diagnosed 2020-01-15)
- Sinus bradycardia with non-conducted premature atrial contractions (diagnosed 2020-07-15)
- Severe hypoglycemia (diagnosed 2020-10-15)
- Type 2 diabetes mellitus
- Hypertension
- Vascular dementia
- Cerebrovascular accident with residual right-sided hemiparesis
- Osteoarthritis
- Status post left total knee arthroplasty

PAST SURGICAL HISTORY:
- Left total knee arthroplasty 9 months prior to initial presentation

MEDICATIONS:
She takes metformin, aspirin, and a course of antibiotics for her urinary tract infection.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin aspart Sliding scale subcutaneously before meals
- Lisinopril 10 mg orally daily
- Hydrochlorothiazide 25 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother and sister
- Cerebrovascular accident in father
- Alzheimer disease in maternal grandmother
- Hypertension in both parents

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired.

PHYSICAL EXAMINATION:
On examination, she is alert but oriented only to person. Her skin is dry with a reddened, non-blanchable area over the sacrum. She has decreased sensation to light touch on her right side.

LABORATORY DATA:
- Hemoglobin: 12.1 g/dL
- Albumin: 3.2 g/dL
- Blood urea nitrogen (BUN): 21 mg/dL
- Creatinine: 1.0 mg/dL
- HbA1c: 7.8%

## Encounter 4: source 2022-01-15, practice 2024-10-04, ed, Emergency Medicine

Chief complaint: Caregiver reports decreased activity, poor oral intake, and chronic sacral wound with increased drainage

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Caregiver reports decreased activity, poor oral intake, and chronic sacral wound with increased drainage

HISTORY OF PRESENT ILLNESS:
The patient with prior Stage I sacral ulcer presents with increased drainage. She is wheelchair-bound and incontinent of urine for two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Prosthetic joint infection due to Staphylococcus epidermidis (diagnosed 2020-01-15)
- Sinus bradycardia with non-conducted premature atrial contractions (diagnosed 2020-07-15)
- Severe hypoglycemia (diagnosed 2020-10-15)
- Pressure ulcer of sacrum, stage 1 (diagnosed 2021-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Vascular dementia
- Cerebrovascular accident with residual right-sided hemiparesis
- Osteoarthritis
- Status post left total knee arthroplasty

PAST SURGICAL HISTORY:
- Left total knee arthroplasty 9 months prior to initial presentation

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin aspart Sliding scale subcutaneously before meals
- Lisinopril 10 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother and sister
- Cerebrovascular accident in father
- Alzheimer disease in maternal grandmother
- Hypertension in both parents

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
The patient is afebrile, blood pressure is 130/85 mmHg, pulse is 88/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
She is oriented to person but not to time or place. A stage III pressure ulcer is noted on her sacrum, with full-thickness skin loss and visible subcutaneous fat, but no exposed muscle or bone.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 12.1 g/dL
- White blood cell count: 14,000/mm³
- Serum albumin: 2.9 g/dL
- Blood urea nitrogen: 40 mg/dL
- Creatinine: 1.2 mg/dL
- HbA1c: 8.2%

## Encounter 5: source 2023-01-15, practice 2025-10-05, ed, Emergency Medicine

Chief complaint: Generalized weakness, right arm pain, and unexplained bruising in the setting of advanced dementia

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized weakness, right arm pain, and unexplained bruising in the setting of advanced dementia

HISTORY OF PRESENT ILLNESS:
The patient is a 78-year-old female with vascular dementia, residual right-sided hemiparesis, and a history of recurrent pressure ulcers including Stage III sacral involvement documented in prior encounters. She returns to the emergency department accompanied by her son with complaints of generalized weakness and pain in her right arm. She is reluctant to explain how she got these bruises and looks toward her son before answering any questions. Her son reports that she is often clumsy and falls because she refuses to use her walker.

PAST MEDICAL HISTORY:
Active Problem List:
- Prosthetic joint infection due to Staphylococcus epidermidis (diagnosed 2020-01-15)
- Sinus bradycardia with non-conducted premature atrial contractions (diagnosed 2020-07-15)
- Severe hypoglycemia (diagnosed 2020-10-15)
- Pressure ulcer of sacrum, stage 1 (diagnosed 2021-04-15)
- Stage III pressure ulcer of sacrum (diagnosed 2022-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Vascular dementia
- Cerebrovascular accident with residual right-sided hemiparesis
- Osteoarthritis
- Status post left total knee arthroplasty

She has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left total knee arthroplasty 9 months prior to initial presentation

MEDICATIONS:
Her medications include metformin and lisinopril.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin aspart Sliding scale subcutaneously before meals
- Hydrochlorothiazide 25 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother and sister
- Cerebrovascular accident in father
- Alzheimer disease in maternal grandmother
- Hypertension in both parents

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
She appears disheveled and is noted to have poor dental hygiene. On examination, she has a bruise on her right forearm and another on her left thigh. Both bruises are of different colors, suggesting different stages of healing.

LABORATORY DATA:
- Hemoglobin: 12.1 g/dL
- White blood cell count: 7,200/mm^3
- Platelet count: 250,000/mm^3
- Serum glucose: 180 mg/dL
- Serum creatinine: 1.2 mg/dL
- Calcium: 9.2 mg/dL
- Alanine aminotransferase (ALT): 25 U/L
- Aspartate aminotransferase (AST): 28 U/L
