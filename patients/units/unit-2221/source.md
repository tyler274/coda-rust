# synthetic_hospital patient 2221

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Plan G.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-25 (Fri) | outpatient | Ophthalmology |
| 1 | 2020-04-15 | 2021-09-13 (Mon) | outpatient | Dermatology |
| 2 | 2020-09-15 | 2022-02-15 (Tue) | outpatient | Internal Medicine |
| 3 | 2021-05-15 | 2022-10-17 (Mon) | outpatient | Gastroenterology |
| 4 | 2022-03-15 | 2023-08-23 (Wed) | outpatient | Dermatology |
| 5 | 2023-01-15 | 2024-06-20 (Thu) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-06-13 (Fri) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-06-16 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Plan G",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Occasional (1-2 glasses of wine per week)",
 "family_history": [
  "Father: Myocardial infarction at age 67",
  "Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "1 drop in both eyes at bedtime",
   "name": "Latanoprost"
  }
 ],
 "surgical_history": [
  "Colonoscopy (age 63, normal)",
  "Cardiac catheterization with drug-eluting stent placement",
  "Tonsillectomy (age 10)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy",
  "Primary Open-Angle Glaucoma",
  "Hyperlipidemia",
  "Multiple benign melanocytic nevi"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Chronic Kidney Disease due to Diabetic Nephropathy", "Contrast-induced nephropathy", "Pancreatic carcinoma", "Primary Open-Angle Glaucoma", "Multiple melanocytic nevi", "Bullous Pemphigoid", "Acute pulmonary edema", "Acute diverticulitis of sigmoid colon"]
Comorbidities: ["Chronic kidney disease stage 3b", "Stress hyperglycemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-06-25, outpatient, Ophthalmology

Chief complaint: Difficulty with night driving and occasionally bumping into objects at the periphery of vision during routine follow-up.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty with night driving and occasionally bumping into objects at the periphery of vision during routine follow-up.

HISTORY OF PRESENT ILLNESS:
He mentions that he has been experiencing difficulty with night driving and occasionally bumps into objects at the periphery of his vision. He denies any eye pain or redness.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Primary Open-Angle Glaucoma
- Hyperlipidemia
- Multiple benign melanocytic nevi

His medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for a brother with glaucoma.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, visual acuity is 20/25 in both eyes. Intraocular pressure measures 22 mmHg in the right eye and 24 mmHg in the left eye. Fundoscopic examination reveals an increased cup-to-disc ratio of 0.7 in both eyes.

OTHER STUDIES:
Visual field testing shows bilateral peripheral vision loss.

## Encounter 1: source 2020-04-15, practice 2021-09-13, outpatient, Dermatology

Chief complaint: Concern about a new slowly growing lesion on his back that he first noticed several months ago, denying pain or bleeding.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Concern about a new slowly growing lesion on his back that he first noticed several months ago, denying pain or bleeding.

HISTORY OF PRESENT ILLNESS:
The patient with multiple melanocytic nevi returns for dermatology follow-up following ophthalmology evaluation. He first noticed the lesion several months ago, and it has slowly increased in size. He denies any pain, itching, or bleeding from the lesion.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Open-Angle Glaucoma (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Hyperlipidemia
- Multiple benign melanocytic nevi

His medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
He denies any recent weight loss, fever, or night sweats

PHYSICAL EXAMINATION:
On physical examination, you note a well-demarcated, round, tan lesion with a "stuck-on" appearance on his upper back. The lesion measures 1.5 cm in diameter and has a slightly rough surface. No other similar lesions are noted on full skin examination.

## Encounter 2: source 2020-09-15, practice 2022-02-15, outpatient, Internal Medicine

Chief complaint: Increased urinary frequency and thirst during routine follow-up for diabetes management.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Patterson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increased urinary frequency and thirst during routine follow-up for diabetes management.

HISTORY OF PRESENT ILLNESS:
He returns for diabetes management following ophthalmology and dermatology visits. He reports feeling generally well but notes increased urinary frequency and thirst over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Open-Angle Glaucoma (diagnosed 2020-01-15)
- Multiple melanocytic nevi (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Hyperlipidemia
- Multiple benign melanocytic nevi

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
He takes his medications regularly, which include metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma
- Sister: Hypertension

SOCIAL HISTORY:
He follows a strict diet

VITAL SIGNS:
His blood pressure is 132/86 mmHg,

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Fasting plasma glucose: 146 mg/dL
- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.8 mg/dL (elevated from 1.2 mg/dL six months ago)
- Urine albumin-to-creatinine ratio: 300 mg/g

## Encounter 3: source 2021-05-15, practice 2022-10-17, outpatient, Gastroenterology

Chief complaint: Progressive nausea, vomiting, early satiety, and 10-pound weight loss over 3 months with unusually high blood glucose levels.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive nausea, vomiting, early satiety, and 10-pound weight loss over 3 months with unusually high blood glucose levels.

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with Type 2 Diabetes Mellitus, Chronic Kidney Disease Stage 3, and Primary Open-Angle Glaucoma, with established care in Ophthalmology, Dermatology, and Internal Medicine, presents to Gastroenterology with a 3-month history of progressive nausea, vomiting, early satiety, and a 10-pound weight loss. He reports that the vomiting occurs after meals and consists of undigested food. However, he notes that his blood glucose levels have been unusually high in the past few weeks despite no changes in diet or medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Open-Angle Glaucoma (diagnosed 2020-01-15)
- Multiple melanocytic nevi (diagnosed 2020-04-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Hyperlipidemia
- Multiple benign melanocytic nevi

His past medical history is significant for type 2 diabetes mellitus, which has been well-controlled with metformin.

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
He takes metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
Physical examination reveals mild epigastric tenderness and a palpable mass in the upper abdomen.

LABORATORY DATA:
Laboratory tests show:

- Hemoglobin: 12.1 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)
- Fasting glucose: 180 mg/dL (Normal: 70-99 mg/dL)
- CA 19-9: 120 U/mL (Normal: 0-37 U/mL)

IMAGING:
An upper gastrointestinal series demonstrates delayed gastric emptying and a narrowing at the distal stomach.

## Encounter 4: source 2022-03-15, practice 2023-08-23, outpatient, Dermatology

Chief complaint: Two-month history of itchy red patches progressing to large tense blisters over abdomen, arms, and thighs.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of itchy red patches progressing to large tense blisters over abdomen, arms, and thighs.

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with Type 2 Diabetes Mellitus, pancreatic carcinoma, and prior dermatology evaluation for melanocytic nevi presents with a new skin complaint. He initially noticed red, raised patches on his trunk that he attributed to an allergic reaction. Over the past two weeks, he has developed large, tense blisters over his abdomen, arms, and thighs. He denies any new medications, soaps, or detergents.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Open-Angle Glaucoma (diagnosed 2020-01-15)
- Multiple melanocytic nevi (diagnosed 2020-04-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-09-15)
- Pancreatic carcinoma (diagnosed 2021-05-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Hyperlipidemia
- Multiple benign melanocytic nevi

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, you note multiple 1-3 cm tense bullae on an erythematous base, some with surrounding urticarial plaques. There are no mucosal lesions.

PATHOLOGY:
A skin biopsy is performed, and direct immunofluorescence shows linear IgG and C3 deposition along the basement membrane zone.

## Encounter 5: source 2023-01-15, practice 2024-06-20, ed, Emergency Medicine

Chief complaint: Three days of worsening constant dull left lower quadrant abdominal pain with low-grade fever, nausea, and decreased appetite.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three days of worsening constant dull left lower quadrant abdominal pain with low-grade fever, nausea, and decreased appetite.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with a history of pancreatic carcinoma diagnosed in 2021 and chronic kidney disease stage 3 secondary to diabetic nephropathy presents with abdominal pain. He describes the pain as a constant, dull ache localized to the lower left quadrant of his abdomen. He also reports decreased appetite, nausea, and occasional loose stools over the past 2 days. He denies vomiting, hematochezia, or melena. He has had similar, less intense episodes in the past that resolved without treatment, but this time the pain has been more severe and persistent.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Open-Angle Glaucoma (diagnosed 2020-01-15)
- Multiple melanocytic nevi (diagnosed 2020-04-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-09-15)
- Pancreatic carcinoma (diagnosed 2021-05-15)
- Bullous Pemphigoid (diagnosed 2022-03-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Hyperlipidemia
- Multiple benign melanocytic nevi

His medical history includes type 2 diabetes mellitus and hypertension. He had a normal colonoscopy 5 years ago.

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma
- Sister: Hypertension

SOCIAL HISTORY:
He does not smoke and drinks alcohol occasionally.

VITAL SIGNS:
On exam, his temperature is 38.2°C (100.8°F), blood pressure is 128/74 mmHg, and pulse is 94/min.

PHYSICAL EXAMINATION:
Abdominal exam reveals tenderness to palpation in the left lower quadrant without rebound or guarding. Bowel sounds are slightly decreased. Rectal exam is unremarkable.

LABORATORY DATA:
Laboratory studies show leukocytosis. A urinalysis is negative for infection or hematuria.

IMAGING:
An abdominal CT scan with contrast demonstrates pericolic fat stranding and localized bowel wall thickening of the sigmoid colon without abscess or free air.

## Encounter 6: source 2024-01-15, practice 2025-06-13, ed, Emergency Medicine

Chief complaint: Acute onset of chest pain radiating to left arm associated with shortness of breath and diaphoresis starting 2 hours ago.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of chest pain radiating to left arm associated with shortness of breath and diaphoresis starting 2 hours ago.

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with diabetes and CKD presents with chest pain. The pain started 2 hours ago and is associated with shortness of breath and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Open-Angle Glaucoma (diagnosed 2020-01-15)
- Multiple melanocytic nevi (diagnosed 2020-04-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-09-15)
- Pancreatic carcinoma (diagnosed 2021-05-15)
- Bullous Pemphigoid (diagnosed 2022-03-15)
- Acute diverticulitis of sigmoid colon (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Hyperlipidemia
- Multiple benign melanocytic nevi

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, and respiratory rate is 22 breaths per minute.

LABORATORY DATA:
His baseline serum creatinine is 1.8 mg/dL, and estimated glomerular filtration rate (eGFR) is 40 mL/min/1.73m^2.

OTHER STUDIES:
An electrocardiogram shows ST-segment elevation in leads II, III, and aVF.

ASSESSMENT AND PLAN:
The patient is diagnosed with an ST-elevation myocardial infarction (STEMI)

He is scheduled for an urgent coronary angiography.

## Encounter 7: source 2025-01-15, practice 2026-06-16, ed, Emergency Medicine

Chief complaint: Sudden onset of severe dyspnea and productive cough with pink, frothy sputum while watching television at home.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Jennifer Adams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe dyspnea and productive cough with pink, frothy sputum while watching television at home.

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with Type 2 Diabetes Mellitus, Chronic Kidney Disease Stage 3, and prior contrast-induced nephropathy presents to the Emergency Department. He reports that his symptoms began suddenly while he was watching television.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Open-Angle Glaucoma (diagnosed 2020-01-15)
- Multiple melanocytic nevi (diagnosed 2020-04-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-09-15)
- Pancreatic carcinoma (diagnosed 2021-05-15)
- Bullous Pemphigoid (diagnosed 2022-03-15)
- Acute diverticulitis of sigmoid colon (diagnosed 2023-01-15)
- Contrast-induced nephropathy (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3 secondary to diabetic nephropathy
- Hyperlipidemia
- Multiple benign melanocytic nevi

PAST SURGICAL HISTORY:
- Colonoscopy (age 63, normal)
- Cardiac catheterization with drug-eluting stent placement
- Tonsillectomy (age 10)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO daily
- Latanoprost 1 drop in both eyes at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 Diabetes Mellitus and Primary Open-Angle Glaucoma
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, he is tachypneic with a respiratory rate of 30 breaths per minute, blood pressure is 180/100 mmHg, heart rate is 110 beats per minute, and oxygen saturation is 85% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral rales halfway up both lung fields. The patient's jugular venous pressure is elevated, and there is 2+ pitting edema in both lower extremities.

LABORATORY DATA:
The patient's serum creatinine is 1.2 mg/dL, and B-type natriuretic peptide (BNP) is significantly elevated.

IMAGING:
Chest X-ray demonstrates pulmonary edema with cephalization of blood flow and Kerley B lines.

OTHER STUDIES:
An electrocardiogram shows ST-segment elevations in leads II, III, and aVF.
